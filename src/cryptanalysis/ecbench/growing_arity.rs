//! `ic.growing_arity`: self-hosted growing-\(m\) ksum cell on a
//! `bits`-bit prime-order curve (`prime_instance_for` + `KSumOracle`).

use super::exact_scalar::ExactScalar;
use super::methods::{param_u64, PhaseRecord, ResolvedMethod, SolveReport};
use crate::cryptanalysis::growing_arity::run_growing_arity_cell;
use serde_json::json;
use std::collections::BTreeMap;

pub fn solve(m: &ResolvedMethod, seed: u64) -> Result<SolveReport, String> {
    let bits = u32::try_from(param_u64(m, "bits")?).map_err(|_| "bits exceeds u32")?;
    let cell = run_growing_arity_cell(bits, seed)?;
    let recovered = cell.recovered.map(ExactScalar::from);
    let mut counters = BTreeMap::new();
    counters.insert("bits".into(), u64::from(cell.bits));
    counters.insert("m".into(), u64::from(cell.m));
    counters.insert("b_used".into(), cell.b_used);
    counters.insert("b_schedule".into(), cell.b_schedule);
    counters.insert("verified".into(), u64::from(cell.verified));
    counters.insert("exhausted".into(), u64::from(cell.exhausted));
    counters.insert("total_gae_u64".into(), cell.total_gae.round() as u64);
    Ok(SolveReport {
        recovered,
        exhausted: cell.exhausted || !cell.verified,
        phases: vec![PhaseRecord {
            name: "growing_arity_ksum".into(),
            adds: cell.total_gae.round() as u64,
            doubles: 0,
            scalar_mults: 0,
            gae: cell.total_gae,
            wall_ns: None,
            native: BTreeMap::new(),
            work_native: BTreeMap::new(),
        }],
        total_gae: cell.total_gae,
        automorphisms_used: 2,
        counters,
        unpriced: vec![],
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: 0,
        factor_base: None,
        detail: json!(cell),
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    })
}
