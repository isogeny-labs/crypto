//! Growing-arity prime-field Kummer/Semaev *schedule* slice.
//!
//! Implements the smallest charged discriminating experiment from
//! `GROWING_ARITY_ORACLE_TARGET.md`: arity `m = m(L)` grows with
//! `L = ln r`, factor-base size follows `B ≤ C m exp(L/m)` (capped for
//! the existing [`KSumOracle`] table), and the lab's `ic_framework`
//! pipeline (`prime-abscissa` + `ksum:m`) runs with matched ρ (`S_ρ =
//! √(π/2)`), planted `[d]G=Q` verification, and ABBA seed pairing.
//!
//! This is **not** absolute-\(\mathbb{F}_p\) extension-IC, and **not** a
//! claim that a proved coverage law exists — it measures whether this
//! concrete growing-\(m\) ksum class clears \(\alpha < 1/2\) vs \(\sqrt{r}\).

use crate::cryptanalysis::diem_kgrow::{ols_alpha_beta, LadderReport, LadderRung};
use crate::cryptanalysis::ic_boundary::{
    prime_instance_for, Calibration, CountedGroup, GroupOps, PrimeInstance,
};
use crate::cryptanalysis::ic_framework::plugins::{KSumOracle, PrimeAbscissaBase};
use crate::cryptanalysis::ic_framework::stages::{InstanceCtx, Targets};
use crate::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec, RunReport};
use serde::Serialize;

/// Schedule exponent `β` in `m = Θ((L/ln L)^{1/(1+β)})` (target doc).
pub const BETA: f64 = 0.5;

/// Leading constant in `B ≤ C m exp(L/m)`.
pub const B_CONST: f64 = 1.0;

/// `m(L)` under `β = 1/2`, clamped to the ksum window `[3, 7]`.
pub fn m_of_l(l: f64) -> u32 {
    if l <= std::f64::consts::E {
        return 3;
    }
    let ln_l = l.ln();
    if ln_l <= 0.0 {
        return 3;
    }
    let exp = 1.0 / (1.0 + BETA);
    let m = (l / ln_l).powf(exp).round() as i64;
    m.clamp(3, 7) as u32
}

/// Uncapped schedule base size `⌈C m exp(L/m)⌉`.
pub fn b_schedule(l: f64, m: u32) -> u64 {
    let mf = f64::from(m).max(1.0);
    let raw = B_CONST * mf * (l / mf).exp();
    raw.ceil().max(2.0) as u64
}

/// KSumOracle base-size caps (from the oracle's packed-index help / tests).
pub fn ksum_base_cap(m: u32) -> u64 {
    match m {
        3 => 64,
        4 => 24,
        5 => 12,
        6 => 10,
        7 => 10,
        8 => 8,
        9 => 6,
        _ => 8,
    }
}

/// Feasible `B` for this rung: schedule size capped by ksum.
pub fn b_feasible(l: f64, m: u32) -> u64 {
    b_schedule(l, m).min(ksum_base_cap(m))
}

#[derive(Clone, Debug, Serialize)]
pub struct GrowingArityCell {
    pub schema: &'static str,
    pub standing: &'static str,
    pub bits: u32,
    pub p: u64,
    pub r: u64,
    pub l: f64,
    pub m: u32,
    pub b_schedule: u64,
    pub b_used: u64,
    pub capped: bool,
    pub verified: bool,
    pub exhausted: bool,
    pub total_gae: f64,
    pub s: f64,
    pub rho_s: f64,
    pub s_over_rho: f64,
    pub recovered: Option<u64>,
    pub planted: u64,
    pub note: &'static str,
}

fn rho_s_ref() -> f64 {
    (std::f64::consts::PI / 2.0).sqrt()
}

fn ctx_planted(inst: &PrimeInstance) -> (InstanceCtx<'_, crate::cryptanalysis::ic_boundary::PrimeCurve>, u64) {
    let g = inst.generator_point();
    let mut ops = GroupOps::default();
    let planted = 30_011 % inst.r;
    let q = CountedGroup::mul(&inst.curve, &mut ops, g, planted);
    (
        InstanceCtx {
            group: &inst.curve,
            generator: g,
            target: q,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: None,
        },
        planted,
    )
}

/// One growing-arity ksum cell on a `bits`-bit prime-order curve.
pub fn run_growing_arity_cell(bits: u32, seed: u64) -> Result<GrowingArityCell, String> {
    if !(12..=40).contains(&bits) {
        return Err(format!("bits={bits} outside 12..=40 for this slice"));
    }
    let inst = prime_instance_for(bits, seed);
    let l = (inst.r as f64).ln();
    let m = m_of_l(l);
    let b_sched = b_schedule(l, m);
    let b_used = b_feasible(l, m);
    let capped = b_used < b_sched;

    let (ctx, planted) = ctx_planted(&inst);
    let mut spec = PipelineSpec {
        factor_base: "prime-abscissa".into(),
        oracle: "ksum".into(),
        targets: Targets::Walk,
        linalg: "incremental-gauss".into(),
        max_trials: 20_000_000,
        seed,
        ..Default::default()
    };
    spec.factor_base_params.set("size", b_used.to_string());
    spec.oracle_params.set("m", m.to_string());

    let mut oracle = KSumOracle::new(m);
    let report: RunReport = run_pipeline(
        &ctx,
        &spec,
        &PrimeAbscissaBase { instance: &inst },
        &mut oracle,
        planted,
        &Calibration::default(),
        Some(rho_s_ref()),
    )?;

    // Exact [d]G=Q already enforced by the pipeline when verified.
    Ok(GrowingArityCell {
        schema: "growing-arity.cell/v1",
        standing: "[pcc:unverified]",
        bits,
        p: inst.curve.p,
        r: inst.r,
        l,
        m,
        b_schedule: b_sched,
        b_used,
        capped,
        verified: report.verified,
        exhausted: report.exhausted,
        total_gae: report.total_gae,
        s: report.s,
        rho_s: rho_s_ref(),
        s_over_rho: report.s_over_rho.unwrap_or(f64::INFINITY),
        recovered: report.recovered,
        planted,
        note: "Growing-m ksum on prime-abscissa; B capped to KSumOracle; matched S_ρ=√(π/2). Not a proved coverage law.",
    })
}

/// Default discriminating ladder: 16, 20, 24, 28-bit prime instances.
pub fn default_growing_arity_bits() -> Vec<u32> {
    vec![16, 20, 24, 28]
}

/// ABBA ladder: for each bit size, seed and seed+1.
pub fn run_growing_arity_ladder(bits: &[u32], seed: u64) -> Result<LadderReport, String> {
    let mut out = Vec::with_capacity(bits.len());
    for &b in bits {
        let ab = run_growing_arity_cell(b, seed)?;
        let ba = run_growing_arity_cell(b, seed.wrapping_add(1))?;
        let ic_mean = ((ab.total_gae + ba.total_gae) / 2.0).max(1.0);
        let rho_gae_mean = {
            // Matched ρ GAE ≈ √(π r / 2) = S_ρ · √r
            let ra = ab.rho_s * (ab.r as f64).sqrt();
            let rb = ba.rho_s * (ba.r as f64).sqrt();
            ((ra + rb) / 2.0).max(1.0)
        };
        let log2_ic = ic_mean.log2();
        let log2_rho = rho_gae_mean.log2();
        let p_mean = ((ab.p as f64) + (ba.p as f64)) / 2.0;
        out.push(LadderRung {
            p: ab.p,
            k: ab.m, // arity m rides in the `k` field for JSON reuse
            ic_adds_ab: ab.total_gae.round() as u64,
            ic_adds_ba: ba.total_gae.round() as u64,
            rho_adds_ab: (ab.rho_s * (ab.r as f64).sqrt()).round() as u64,
            rho_adds_ba: (ba.rho_s * (ba.r as f64).sqrt()).round() as u64,
            log2_ic_obs_mean: log2_ic,
            log2_rho_obs_mean: log2_rho,
            log2_ic_pred: (ab.l / f64::from(ab.m)).log2() + ab.m as f64, // rough
            log2_rho_pred: log2_rho,
            alpha_proxy: if p_mean.log2() > 0.0 {
                log2_ic / p_mean.log2()
            } else {
                f64::INFINITY
            },
            beats_sqrt_r_obs: ab.s_over_rho < 1.0 && ba.s_over_rho < 1.0,
            recovered_ok: ab.verified && ba.verified,
        });
    }

    let pts: Vec<(f64, f64)> = out
        .iter()
        .filter(|r| r.recovered_ok)
        .map(|r| ((r.p as f64).log2(), r.log2_ic_obs_mean))
        .collect();
    let (alpha_ols, beta_ols) = ols_alpha_beta(&pts);
    let largest = out.last().expect("nonempty");
    let verdict = if pts.len() < 2 {
        "INCONCLUSIVE: <2 verified rungs for α fit".into()
    } else if alpha_ols.unwrap_or(f64::INFINITY) < 0.5 && largest.beats_sqrt_r_obs {
        format!(
            "SURVIVES_SLICE: OLS α={:.3} < 1/2 and largest rung S < S_ρ (not a verified L_p claim; B may be capped)",
            alpha_ols.unwrap()
        )
    } else {
        format!(
            "FAIL_FOR_GOAL_α: OLS α={} ; largest beats_ρ={} — growing-arity ksum class falsified at this scale; open next successor",
            alpha_ols
                .map(|a| format!("{a:.3}"))
                .unwrap_or_else(|| "n/a".into()),
            largest.beats_sqrt_r_obs
        )
    };

    Ok(LadderReport {
        schema: "growing-arity.ladder/v1",
        standing: "[pcc:unverified]",
        order: "ABBA+BAAB (seed-paired prime_instance_for + ksum:m(L))",
        rungs: out,
        alpha_ols,
        beta_ols,
        acceptance: "≥2 verified rungs; OLS α<1/2 AND largest-rung S<S_ρ to survive; else falsify growing-arity ksum class → named successor",
        verdict,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m_of_l_grows_and_clamps() {
        assert_eq!(m_of_l(8.0), 3);
        let m_big = m_of_l(40.0);
        assert!((3..=7).contains(&m_big));
    }

    #[test]
    fn growing_arity_cell_16bit_runs() {
        let cell = run_growing_arity_cell(16, 5).expect("16-bit cell");
        assert!(cell.m >= 3);
        assert!(cell.b_used >= 2);
        // May or may not verify depending on yield; just ensure it returns.
        assert!(cell.total_gae >= 0.0);
    }
}
