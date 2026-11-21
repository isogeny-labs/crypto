//! AUX-tower factor-base supply + algebraic rootfind decomp slice.
//!
//! Smallest charged discriminating experiment from
//! `SUCCESSOR_H_AUX_TOWER_SOLVER.md`:
//!
//! 1. Build `prime-isogeny:t` (AUX search charged in the factor-base phase).
//! 2. Decompose with `rootfind-mitm:m=3` (algebraic — **not** ksum tables).
//!    `pkm-semaev` F4 at `t≥4` exceeded lab memory/time; rootfind is the
//!    lab's charged algebraic path already used with `prime-isogeny`.
//! 3. Compare with analytic \(S_\rho=\sqrt{\pi/2}\), verify planted
//!    \([d]G=Q\), pair seeds. Absolute-\(\mathbb{F}_p\) extension-IC closed.

use crate::cryptanalysis::diem_kgrow::ols_alpha_beta;
use crate::cryptanalysis::growing_arity::{
    b_schedule, m_of_l, GrowingArityLadderReport, GrowingArityLadderRung,
};
use crate::cryptanalysis::ic_boundary::{
    prime_instance_for, Calibration, CountedGroup, GroupOps, PrimeInstance,
};
use crate::cryptanalysis::ic_framework::prime_tower::{isogeny_tower, PrebuiltPrimeIsogenyBase};
use crate::cryptanalysis::ic_framework::rootfind::RootfindMitmOracle;
use crate::cryptanalysis::ic_framework::stages::{InstanceCtx, Targets};
use crate::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec, RunReport};
use serde::Serialize;
use std::time::Instant;

/// Cap tower height so AUX search + rootfind stay in-process (`B=2^t`).
/// Existing ecbench charge test uses `t=7` on a 20-bit curve.
pub const T_MAX: u32 = 7;

/// Floor tower height (need a nontrivial AUX tower).
pub const T_MIN: u32 = 3;

/// Algebraic arity for this slice (rootfind-mitm supports 2|3).
pub const M: u32 = 3;

fn rho_s_ref() -> f64 {
    (std::f64::consts::PI / 2.0).sqrt()
}

/// `t` from schedule `B ≈ C m exp(L/m)`, clamped to `[T_MIN, T_MAX]`.
pub fn t_of_schedule(l: f64, m: u32) -> u32 {
    let b = b_schedule(l, m).max(2);
    let t = (b as f64).log2().ceil() as i64;
    t.clamp(T_MIN as i64, T_MAX as i64) as u32
}

fn ctx_planted(
    inst: &PrimeInstance,
) -> (
    InstanceCtx<'_, crate::cryptanalysis::ic_boundary::PrimeCurve>,
    u64,
) {
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

#[derive(Clone, Debug, Serialize)]
pub struct AuxTowerCell {
    pub schema: &'static str,
    pub standing: &'static str,
    pub bits: u32,
    pub p: u64,
    pub r: u64,
    pub l: f64,
    pub m: u32,
    pub t: u32,
    pub b_schedule: u64,
    pub b_tower: u64,
    pub verified: bool,
    pub exhausted: bool,
    pub total_gae: f64,
    pub cold_wall_seconds: f64,
    pub unpriced_hint: u64,
    pub s: f64,
    pub rho_s: f64,
    pub s_over_rho: f64,
    pub recovered: Option<u64>,
    pub planted: u64,
    pub note: &'static str,
}

/// One AUX-tower + `rootfind-mitm` cell on a `bits`-bit prime-order curve.
pub fn run_aux_tower_cell(bits: u32, seed: u64) -> Result<AuxTowerCell, String> {
    if !(14..=36).contains(&bits) {
        return Err(format!("bits={bits} outside 14..=36 for this slice"));
    }
    let inst = prime_instance_for(bits, seed);
    let l = (inst.r as f64).ln();
    let m_sched = m_of_l(l);
    let b_sched = b_schedule(l, m_sched);
    let t = t_of_schedule(l, m_sched);
    let b_tower = 1u64 << t;

    let (ctx, planted) = ctx_planted(&inst);
    let mut spec = PipelineSpec {
        factor_base: "prime-isogeny".into(),
        oracle: "rootfind-mitm".into(),
        targets: Targets::Walk,
        linalg: "incremental-gauss".into(),
        max_trials: 20_000_000,
        seed,
        ..Default::default()
    };
    spec.factor_base_params.set("t", t.to_string());
    spec.factor_base_params.set("seed", seed.to_string());
    spec.oracle_params.set("m", M.to_string());

    let cold_started = Instant::now();
    let tower_started = Instant::now();
    let mut tower_ops = GroupOps::default();
    let tower = isogeny_tower(inst.curve.p, t, seed, &mut tower_ops)?;
    let tower_wall_ns = tower_started.elapsed().as_nanos().min(u64::MAX as u128) as u64;
    let mut oracle = RootfindMitmOracle::new(M);
    let base = PrebuiltPrimeIsogenyBase {
        instance: &inst,
        tower: &tower,
        tower_ops,
        tower_wall_ns,
        seed,
    };
    let report: RunReport = run_pipeline(
        &ctx,
        &spec,
        &base,
        &mut oracle,
        planted,
        &Calibration::default(),
        Some(rho_s_ref()),
    )?;

    let unpriced_hint = report
        .decomposition
        .solver
        .as_ref()
        .filter(|s| s.priced_by == "unpriced")
        .map(|s| s.ops)
        .unwrap_or(0);
    let pricing_incomplete = unpriced_hint > 0 || report.cost_qualification.is_some();

    Ok(AuxTowerCell {
        schema: "aux-tower-solver.cell/v1",
        standing: "[pcc:unverified]",
        bits,
        p: inst.curve.p,
        r: inst.r,
        l,
        m: M,
        t,
        b_schedule: b_sched,
        b_tower,
        verified: report.verified,
        exhausted: report.exhausted,
        total_gae: report.total_gae,
        cold_wall_seconds: cold_started.elapsed().as_secs_f64(),
        unpriced_hint,
        s: report.s,
        rho_s: rho_s_ref(),
        s_over_rho: report.s_over_rho.unwrap_or(f64::INFINITY),
        recovered: report.recovered,
        planted,
        note: if pricing_incomplete {
            "prime-isogeny AUX + rootfind-mitm; analytic S_ρ=√(π/2). Incomplete pricing — S lower bound; cannot claim beat ρ."
        } else {
            "prime-isogeny AUX supply + rootfind-mitm algebraic decomp; analytic S_ρ=√(π/2). Not a proved β<1 T_dec."
        },
    })
}

/// Default ladder: 16, 20, 24-bit (t≤7; matches charged prime-isogeny test scale).
pub fn default_aux_tower_bits() -> Vec<u32> {
    vec![16, 20, 24]
}

/// Seed-paired ladder over bit sizes.
pub fn run_aux_tower_ladder(bits: &[u32], seed: u64) -> Result<GrowingArityLadderReport, String> {
    let mut out = Vec::with_capacity(bits.len());
    let mut notes_capped = false;
    for &b in bits {
        eprintln!("aux-tower cell bits={b} seed={seed} …");
        let ab = run_aux_tower_cell(b, seed)?;
        eprintln!(
            "  seed_a: verified={} exhausted={} t={} S/Sρ={:.3} gae={:.0} wall={:.2}s",
            ab.verified, ab.exhausted, ab.t, ab.s_over_rho, ab.total_gae, ab.cold_wall_seconds
        );
        eprintln!("aux-tower cell bits={b} seed={} …", seed.wrapping_add(1));
        let ba = run_aux_tower_cell(b, seed.wrapping_add(1))?;
        eprintln!(
            "  seed_b: verified={} exhausted={} t={} S/Sρ={:.3} gae={:.0} wall={:.2}s",
            ba.verified, ba.exhausted, ba.t, ba.s_over_rho, ba.total_gae, ba.cold_wall_seconds
        );
        let capped = ab.b_tower < ab.b_schedule || ba.b_tower < ba.b_schedule;
        if capped {
            notes_capped = true;
        }
        let ic_mean = ((ab.total_gae + ba.total_gae) / 2.0).max(1.0);
        let rho_gae_mean = {
            let ra = ab.rho_s * (ab.r as f64).sqrt();
            let rb = ba.rho_s * (ba.r as f64).sqrt();
            ((ra + rb) / 2.0).max(1.0)
        };
        let log2_ic = ic_mean.log2();
        let log2_rho = rho_gae_mean.log2();
        let pricing_complete = ab.unpriced_hint == 0 && ba.unpriced_hint == 0;
        let below = pricing_complete
            && ab.verified
            && ba.verified
            && ab.s_over_rho < 1.0
            && ba.s_over_rho < 1.0;
        out.push(GrowingArityLadderRung {
            p: ab.p,
            m: ab.m,
            b_schedule: ab.b_schedule.max(ba.b_schedule),
            b_used: ab.b_tower.min(ba.b_tower),
            capped,
            ic_adds_seed_a: ab.total_gae.round() as u64,
            ic_adds_seed_b: ba.total_gae.round() as u64,
            rho_expected_adds_seed_a: (ab.rho_s * (ab.r as f64).sqrt()).round() as u64,
            rho_expected_adds_seed_b: (ba.rho_s * (ba.r as f64).sqrt()).round() as u64,
            log2_ic_observed_mean: log2_ic,
            log2_rho_expected_mean: log2_rho,
            below_rho_expectation: below,
            recovered_ok: ab.verified && ba.verified,
        });
    }

    let pts: Vec<(f64, f64)> = out
        .iter()
        .filter(|r| r.recovered_ok)
        .map(|r| ((r.p as f64).log2(), r.log2_ic_observed_mean))
        .collect();
    let (slope, intercept) = ols_alpha_beta(&pts);
    let largest = out.last().expect("nonempty");
    let cap_note = if notes_capped {
        " (tower B=2^t capped below schedule)"
    } else {
        ""
    };
    let verdict = if pts.len() < 2 {
        format!("INCONCLUSIVE: <2 verified rungs for local power-law fit{cap_note}")
    } else if slope.unwrap_or(f64::INFINITY) < 0.5 && largest.below_rho_expectation {
        format!(
            "SURVIVES_SLICE: local OLS slope={:.3} < 1/2 and largest rung priced S < S_ρ{cap_note} (not a verified L_p claim)",
            slope.unwrap()
        )
    } else {
        format!(
            "FAIL_FOR_GOAL_α: local OLS slope={} ; largest below_ρ_expectation={}{cap_note} — AUX+rootfind class falsified at this scale; open next successor",
            slope
                .map(|a| format!("{a:.3}"))
                .unwrap_or_else(|| "n/a".into()),
            largest.below_rho_expectation
        )
    };

    Ok(GrowingArityLadderReport {
        schema: "aux-tower-solver.ladder/v1",
        standing: "[pcc:unverified]",
        comparison: "seed-paired prime-isogeny + rootfind-mitm vs analytic S_ρ=√(π/2)·√r",
        rungs: out,
        local_power_slope_ols: slope,
        local_power_intercept_ols: intercept,
        acceptance: "≥2 verified rungs; local OLS slope<1/2 AND largest priced S<S_ρ to survive; else falsify → named successor",
        verdict,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_of_schedule_clamps() {
        let t = t_of_schedule(20.0, 3);
        assert!((T_MIN..=T_MAX).contains(&t));
    }
}
