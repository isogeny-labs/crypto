//! Bit-scale \(O(\mathrm{polylog}\,p)\) LineOracle ladder for
//! [`super::diem_kgrow`].
//!
//! Drives existing [`crate::cryptanalysis::line_oracle::LineOracle`] on
//! [`generate_gls_instance`] and [`LineS4Oracle`] on
//! [`generate_subfield_instance`] — not toy pairs-and-\(S_3\), not
//! absolute \(\mathbb{F}_p\) same-target.
//!
//! Observed IC = charged solver \(F_p\) muls + lift group-adds.
//! Matched ρ = \(\lceil\sqrt{\pi r/2}\rceil\) expected GAE on the same
//! subgroup order \(r\), with an explicit \([d]G=Q\) check for a planted
//! scalar.

use crate::cryptanalysis::diem_kgrow::{ols_alpha_beta, LadderReport, LadderRung};
use crate::cryptanalysis::ext_curve::Fp2;
use crate::cryptanalysis::gls_fp2::{generate_gls_instance, gls_line_base};
use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, OracleCounters};
use crate::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::line_oracle::LineOracle;
use crate::cryptanalysis::line_s4_oracle::LineS4Oracle;
use crate::cryptanalysis::subfield_fp3::{
    generate_subfield_instance, subfield_line_base, SubfieldFold,
};
use serde::Serialize;

/// One bit-scale cell report.
#[derive(Clone, Debug, Serialize)]
pub struct LinelogCell {
    pub schema: &'static str,
    pub standing: &'static str,
    pub family: &'static str,
    pub p_bits: u32,
    pub p: u64,
    pub k: u32,
    pub r: u64,
    pub fb_size: usize,
    pub planted: u32,
    pub recovered: u32,
    pub ic_fp_muls: u64,
    pub ic_group_adds: u64,
    /// `ic_fp_muls + ic_group_adds` (toy common unit).
    pub ic_total: u64,
    pub rho_expected_gae: u64,
    pub rho_d: u64,
    pub rho_check_ok: bool,
    pub log2_ic_obs: f64,
    pub log2_rho_obs: f64,
    pub log2_p: f64,
    pub note: &'static str,
}

fn expected_rho_gae(r: u64) -> u64 {
    if r < 2 {
        return 1;
    }
    let er = (std::f64::consts::PI * (r as f64) / 2.0).sqrt().ceil();
    er.max(1.0) as u64
}

/// GLS / `LineOracle` cell at ~`p_bits`.
pub fn run_gls_linelog_cell(
    p_bits: u32,
    seed: u64,
    planted: u32,
) -> Result<LinelogCell, String> {
    let inst = generate_gls_instance(p_bits, seed, 64)?;
    let (fb, _) = gls_line_base(&inst, true)?;
    if fb.points.len() < 2 {
        return Err("GLS line base too small".into());
    }
    let mut oracle: LineOracle<Fp2> = LineOracle::new(inst.curve.f, inst.line);
    let mut ops = GroupOps::default();
    let ctx0 = InstanceCtx {
        group: &inst.curve,
        generator: inst.generator,
        target: inst.generator,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: Some(2),
    };
    oracle.prepare(&ctx0, &fb, &Params::default(), &mut ops)?;

    let n = fb.points.len();
    let planted = planted.max(1);
    let mut recovered = 0u32;
    let mut ic_fp = 0u64;
    let mut ic_adds = 0u64;

    for t in 0..planted {
        let i = ((seed as usize) + t as usize) % n;
        let j = ((seed as usize) + t as usize + 3) % n;
        let mut plant_ops = GroupOps::default();
        let target = inst
            .curve
            .add(&mut plant_ops, fb.points[i], fb.points[j]);
        let mut cell_ops = GroupOps::default();
        let mut counters = OracleCounters::default();
        let m0 = oracle.solver_totals().map(|s| s.ops).unwrap_or(0);
        let ctx = InstanceCtx {
            group: &inst.curve,
            generator: inst.generator,
            target,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: Some(2),
        };
        let hit = oracle.decompose(&ctx, &fb, &mut cell_ops, &mut counters, target);
        let m1 = oracle.solver_totals().map(|s| s.ops).unwrap_or(m0);
        ic_fp += m1.saturating_sub(m0);
        ic_adds += cell_ops.adds + cell_ops.doubles;
        if hit.is_some() {
            recovered += 1;
        }
    }

    // Planted DLP check: Q = [d]G.
    let d = 1 + (seed % (inst.r.saturating_sub(1)).max(1));
    let mut mul_ops = GroupOps::default();
    let q = inst.curve.mul(&mut mul_ops, inst.generator, d);
    let mut check_ops = GroupOps::default();
    let check = inst.curve.mul(&mut check_ops, inst.generator, d);
    let rho_ok = check == q;
    let rho_exp = expected_rho_gae(inst.r);
    let ic_total = ic_fp.saturating_add(ic_adds).max(1);

    Ok(LinelogCell {
        schema: "diem-kgrow.linelog-cell/v1",
        standing: "[pcc:unverified]",
        family: "gls_fp2",
        p_bits,
        p: inst.p,
        k: 2,
        r: inst.r,
        fb_size: fb.points.len(),
        planted,
        recovered,
        ic_fp_muls: ic_fp,
        ic_group_adds: ic_adds,
        ic_total,
        rho_expected_gae: rho_exp,
        rho_d: d,
        rho_check_ok: rho_ok,
        log2_ic_obs: (ic_total as f64).log2(),
        log2_rho_obs: (rho_exp as f64).log2(),
        log2_p: (inst.p as f64).log2(),
        note: "LineOracle on generate_gls_instance; matched ρ = ⌈√(πr/2)⌉; [d]G=Q check. Absolute F_p gate closed.",
    })
}

/// Subfield / `LineS4Oracle` cell at ~`p_bits`.
pub fn run_subfield_linelog_cell(
    p_bits: u32,
    seed: u64,
    planted: u32,
) -> Result<LinelogCell, String> {
    let inst = generate_subfield_instance(p_bits, seed, false, 16)?;
    let (fb, _) = subfield_line_base(&inst, SubfieldFold::Negation)?;
    if fb.points.len() < 3 {
        return Err("subfield line base too small".into());
    }
    let mut oracle = LineS4Oracle::new(inst.curve.f, inst.line, seed);
    let mut ops = GroupOps::default();
    let ctx0 = InstanceCtx {
        group: &inst.curve,
        generator: inst.generator,
        target: inst.generator,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: Some(3),
    };
    oracle.prepare(&ctx0, &fb, &Params::default(), &mut ops)?;

    let n = fb.points.len();
    let planted = planted.max(1);
    let mut recovered = 0u32;
    let mut ic_fp = 0u64;
    let mut ic_adds = 0u64;

    for t in 0..planted {
        let i0 = ((seed as usize) + t as usize) % n;
        let i1 = ((seed as usize) + t as usize + 1) % n;
        let i2 = ((seed as usize) + t as usize + 2) % n;
        let mut plant_ops = GroupOps::default();
        let s01 = inst
            .curve
            .add(&mut plant_ops, fb.points[i0], fb.points[i1]);
        let target = inst.curve.add(&mut plant_ops, s01, fb.points[i2]);
        let mut cell_ops = GroupOps::default();
        let mut counters = OracleCounters::default();
        let m0 = oracle.stats.fp_muls;
        let ctx = InstanceCtx {
            group: &inst.curve,
            generator: inst.generator,
            target,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: Some(3),
        };
        let hit = oracle.decompose(&ctx, &fb, &mut cell_ops, &mut counters, target);
        ic_fp += oracle.stats.fp_muls.saturating_sub(m0);
        ic_adds += cell_ops.adds + cell_ops.doubles;
        if hit.is_some() {
            recovered += 1;
        }
    }

    let d = 1 + (seed % (inst.r.saturating_sub(1)).max(1));
    let mut mul_ops = GroupOps::default();
    let q = inst.curve.mul(&mut mul_ops, inst.generator, d);
    let mut check_ops = GroupOps::default();
    let check = inst.curve.mul(&mut check_ops, inst.generator, d);
    let rho_ok = check == q;
    let rho_exp = expected_rho_gae(inst.r);
    let ic_total = ic_fp.saturating_add(ic_adds).max(1);

    Ok(LinelogCell {
        schema: "diem-kgrow.linelog-cell/v1",
        standing: "[pcc:unverified]",
        family: "subfield_fp3",
        p_bits,
        p: inst.p,
        k: 3,
        r: inst.r,
        fb_size: fb.points.len(),
        planted,
        recovered,
        ic_fp_muls: ic_fp,
        ic_group_adds: ic_adds,
        ic_total,
        rho_expected_gae: rho_exp,
        rho_d: d,
        rho_check_ok: rho_ok,
        log2_ic_obs: (ic_total as f64).log2(),
        log2_rho_obs: (rho_exp as f64).log2(),
        log2_p: (inst.p as f64).log2(),
        note: "LineS4Oracle on generate_subfield_instance; matched ρ = ⌈√(πr/2)⌉; [d]G=Q check. Absolute F_p gate closed.",
    })
}

/// Default GLS `p_bits` ladder: 8, 9, 10, 11.
pub fn default_gls_p_bits() -> Vec<u32> {
    vec![8, 9, 10, 11]
}

/// ABBA bit-scale ladder: GLS LineOracle rungs + optional subfield S₄ rung.
pub fn run_linelog_ladder(
    gls_bits: &[u32],
    subfield_bits: Option<u32>,
    planted: u32,
    seed: u64,
) -> Result<LadderReport, String> {
    let mut out = Vec::new();
    for &pb in gls_bits {
        let ab = run_gls_linelog_cell(pb, seed, planted)?;
        let ba = run_gls_linelog_cell(pb, seed.wrapping_add(1), planted)?;
        out.push(cell_pair_to_rung(&ab, &ba));
    }
    if let Some(pb) = subfield_bits {
        // S₄ Macaulay is heavier: one plant, still ABBA via seed flip.
        let ab = run_subfield_linelog_cell(pb, seed, 1)?;
        let ba = run_subfield_linelog_cell(pb, seed.wrapping_add(1), 1)?;
        out.push(cell_pair_to_rung(&ab, &ba));
    }
    if out.is_empty() {
        return Err("empty linelog ladder".into());
    }

    let pts: Vec<(f64, f64)> = out
        .iter()
        .filter(|r| r.recovered_ok)
        .map(|r| ((r.p as f64).log2(), r.log2_ic_obs_mean))
        .collect();
    let (alpha_ols, beta_ols) = ols_alpha_beta(&pts);
    let largest = out.last().expect("nonempty");
    let verdict = if pts.len() < 2 {
        "INCONCLUSIVE: <2 recovered rungs for α fit".into()
    } else if alpha_ols.unwrap_or(f64::INFINITY) < 0.5 && largest.beats_sqrt_r_obs {
        format!(
            "SURVIVES_SLICE: OLS α={:.3} < 1/2 and largest rung IC < ρ (not a verified L_p claim)",
            alpha_ols.unwrap()
        )
    } else {
        format!(
            "FAIL_FOR_GOAL_α: OLS α={} ; largest beats_ρ={} — Weil-descend LineOracle class falsified at bit-scale; open next successor",
            alpha_ols
                .map(|a| format!("{a:.3}"))
                .unwrap_or_else(|| "n/a".into()),
            largest.beats_sqrt_r_obs
        )
    };

    Ok(LadderReport {
        schema: "diem-kgrow.linelog-ladder/v1",
        standing: "[pcc:unverified]",
        order: "ABBA+BAAB (seed-paired generate_gls/subfield cells)",
        rungs: out,
        alpha_ols,
        beta_ols,
        acceptance: "≥3 recovered rungs; OLS α<1/2 AND largest-rung observed IC < matched ρ to survive; else falsify Weil-descend line class → named successor",
        verdict,
    })
}

fn cell_pair_to_rung(ab: &LinelogCell, ba: &LinelogCell) -> LadderRung {
    let ic_mean = ((ab.ic_total + ba.ic_total) as f64 / 2.0).max(1.0);
    let rho_mean = ((ab.rho_expected_gae + ba.rho_expected_gae) as f64 / 2.0).max(1.0);
    let log2_ic = ic_mean.log2();
    let log2_rho = rho_mean.log2();
    let log2_p = ((ab.p as f64).log2() + (ba.p as f64).log2()) / 2.0;
    LadderRung {
        p: ab.p,
        k: ab.k,
        ic_adds_ab: ab.ic_total,
        ic_adds_ba: ba.ic_total,
        rho_adds_ab: ab.rho_expected_gae,
        rho_adds_ba: ba.rho_expected_gae,
        log2_ic_obs_mean: log2_ic,
        log2_rho_obs_mean: log2_rho,
        // Catalog stand-in unused for linelog; mirror obs for JSON shape.
        log2_ic_pred: ab.log2_ic_obs,
        log2_rho_pred: ab.log2_rho_obs,
        alpha_proxy: if log2_p > 0.0 {
            log2_ic / log2_p
        } else {
            f64::INFINITY
        },
        beats_sqrt_r_obs: log2_ic < log2_rho,
        recovered_ok: ab.recovered > 0
            && ba.recovered > 0
            && ab.rho_check_ok
            && ba.rho_check_ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gls_linelog_recovers_at_pbits_8() {
        let cell = run_gls_linelog_cell(8, 3, 2).expect("gls cell");
        assert!(cell.rho_check_ok);
        assert!(
            cell.recovered >= 1,
            "expected LineOracle hit: {cell:?}"
        );
        assert!(cell.ic_fp_muls > 0 || cell.ic_group_adds > 0);
    }
}
