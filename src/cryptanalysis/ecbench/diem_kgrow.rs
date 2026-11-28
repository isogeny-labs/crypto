//! `ic.diem_kgrow`: charged selectable-`k` Diem extension-IC cell.
//!
//! Geometry is **self-hosted** from method params `(p, k)` (extension
//! target `E(F_{p^k})`), not from the workload curve.  Absolute
//! \(\mathbb{F}_p\) same-target remains gated; this arm is for growing
//! `(p, k(p))` ladders only.
//!
//! Semaev-line decomp (`line_oracle` / `line_s4_oracle`) plus matched ρ
//! are charged into `SolveReport` counters.  Full `CountedGroup` wiring
//! on Vec-backed `Pt` remains a follow-up.

use super::exact_scalar::ExactScalar;
use super::methods::{param_u64, PhaseRecord, ResolvedMethod, SolveReport};
use crate::cryptanalysis::diem_kgrow::{
    k_of_p, run_cell_charged, CellRequest, ChargedCellReport,
};
use serde_json::json;
use std::collections::BTreeMap;

/// Build a charged cell request from method params.
pub fn request_from_method(m: &ResolvedMethod, seed: u64) -> Result<CellRequest, String> {
    let p = param_u64(m, "p")?;
    let k_opt = m.params.get("k").map(|s| s.as_str());
    let k = match k_opt {
        None | Some("schedule") | Some("") => None,
        Some(s) => Some(
            s.parse::<u32>()
                .map_err(|_| format!("ic.diem_kgrow: bad k={s}"))?,
        ),
    };
    let k_min = m
        .params
        .get("k_min")
        .and_then(|s| s.parse().ok())
        .unwrap_or(2);
    let k_max = m
        .params
        .get("k_max")
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);
    let trial_budget = m
        .params
        .get("trial_budget")
        .and_then(|s| s.parse().ok())
        .unwrap_or(2_000_000);
    let planted = m
        .params
        .get("planted")
        .and_then(|s| s.parse().ok())
        .unwrap_or(2);
    let oracle = m
        .params
        .get("oracle")
        .cloned()
        .unwrap_or_else(|| "semaev".into());
    Ok(CellRequest {
        p,
        k,
        k_min,
        k_max,
        trial_budget,
        planted_decompositions: planted,
        seed,
        oracle,
    })
}

/// Resolve schedule `k` when the caller left `k` unset.
pub fn resolved_k(req: &CellRequest) -> u32 {
    match req.k {
        Some(k) => k,
        None => k_of_p(req.p, req.k_min, req.k_max),
    }
}

/// Run one charged cell and pack a [`SolveReport`].
pub fn solve(m: &ResolvedMethod, seed: u64) -> Result<SolveReport, String> {
    let req = request_from_method(m, seed)?;
    let cell = run_cell_charged(&req)?;
    Ok(report_from_cell(&cell))
}

fn report_from_cell(cell: &ChargedCellReport) -> SolveReport {
    let recovered = cell.rho_d.map(ExactScalar::from);
    let exhausted = cell.recovered == 0;
    let mut counters = BTreeMap::new();
    counters.insert("ic_group_adds".into(), cell.ic_group_adds);
    counters.insert("ic_trials".into(), cell.ic_trials);
    counters.insert("rho_group_adds".into(), cell.rho_group_adds);
    counters.insert("fb_size".into(), cell.fb_size as u64);
    counters.insert("planted".into(), u64::from(cell.planted));
    counters.insert("recovered_decomps".into(), u64::from(cell.recovered));
    counters.insert("p".into(), cell.p);
    counters.insert("k".into(), u64::from(cell.k));
    counters.insert(
        "rho_check_ok".into(),
        u64::from(cell.rho_check_ok),
    );

    let phases = vec![
        PhaseRecord {
            name: "ic_k_decomp".into(),
            adds: cell.ic_group_adds,
            doubles: 0,
            scalar_mults: 0,
            gae: cell.ic_group_adds as f64,
            wall_ns: None,
            native: BTreeMap::new(),
            work_native: BTreeMap::new(),
        },
        PhaseRecord {
            name: "matched_rho".into(),
            adds: cell.rho_group_adds,
            doubles: 0,
            scalar_mults: 0,
            gae: cell.rho_group_adds as f64,
            wall_ns: None,
            native: BTreeMap::new(),
            work_native: BTreeMap::new(),
        },
    ];
    let total_gae = phases.iter().map(|p| p.gae).sum();

    SolveReport {
        recovered,
        exhausted,
        phases,
        total_gae,
        automorphisms_used: 1,
        counters,
        unpriced: vec![],
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: 0,
        factor_base: None,
        detail: json!({
            "schema": cell.schema,
            "standing": cell.standing,
            "p": cell.p,
            "k": cell.k,
            "k_from_schedule": cell.k_from_schedule,
            "schedule_k_of_p": cell.schedule_k_of_p,
            "irr_const": cell.irr_const,
            "log2_extension_ic_pred": cell.log2_extension_ic_pred,
            "log2_rho_pred": cell.log2_rho_pred,
            "log2_ic_obs": cell.log2_ic_obs,
            "log2_rho_obs": cell.log2_rho_obs,
            "rho_check_ok": cell.rho_check_ok,
            "note": cell.note,
        }),
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    }
}
