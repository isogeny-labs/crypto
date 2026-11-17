//! Solver and oracle work units and their calibration keys (2026-11-12).
//!
//! Pure data and one pure function, with no other dependency, so that
//! `ecbench/band.rs` can use them both in this crate and in
//! `tools/backend-leaderboard`, which compiles `band.rs` by path and
//! includes this file the same way.  Re-exported as
//! `ic_boundary::PINNABLE_NATIVE` and `ic_framework::solvers::{solver_unit_key,
//! SolverUnitKey}`.

/// The calibration key that would price one solver unit by count, and why
/// a pin of it must still be refused, if it must.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolverUnitKey {
    /// A field of `ic_boundary::Calibration` and a key of
    /// `docs/ic/calibration.json` (`ic_boundary::SOLVER_UNIT_KEYS`).
    pub key: &'static str,
    /// `Some(reason)` when the unit's count does not cover the engine's
    /// whole run, so a pinned ratio would charge only part of the run (the
    /// module comment's rule).  The run is then priced as if no key were
    /// pinned, and the reason is recorded in the solver block.
    pub refused: Option<&'static str>,
}

/// The calibration key of a solver's `op_unit`, for every unit except the
/// `word XORs (…)` family (`ns_per_word_xor`, matched in
/// `ic_framework::is_word_xor_op_unit`) and `F_q multiplications`.  A field
/// multiplication unit that carries the curve's own multiplications per
/// addition (`extra["fp_muls_per_add"]`, the Gaudry oracles) is priced by
/// that figure before this key is consulted.  `None` for a unit with no key
/// (`m4gb-f2`'s wall-time-only unit, a test engine's relabelled unit).
///
/// A key's ratio is defined as the solver call's whole wall time per
/// counted unit (`docs/ic/calibration-solver-units.md`), which is why a
/// count that is a whole-run proxy may be charged at it.  A count is
/// refused when the unit itself says it is partial or modelled: it does not
/// grow with all of the run's work, so no constant ratio prices the run.
pub fn solver_unit_key(op_unit: &str) -> Option<SolverUnitKey> {
    let ok = |key| Some(SolverUnitKey { key, refused: None });
    let refuse = |key, why| {
        Some(SolverUnitKey {
            key,
            refused: Some(why),
        })
    };
    match op_unit {
        "conflicts" => ok("ns_per_conflict"),
        "monomial operations" => ok("ns_per_monomial_op"),
        "monomial operations (modelled)" => refuse(
            "ns_per_monomial_op",
            "xl-f2 reports a modelled count (terms × n²), not the work it did, so the count does not cover the engine's whole run",
        ),
        "monomial tests" => ok("ns_per_monomial_test"),
        // The line oracles (`line-resultant`, `line-s4-macaulay`,
        // `fghr-d3`, `y-line-s3-macaulay`) report bare `F_p
        // multiplications` without the curve's multiplications per
        // addition; a unit that carries that figure is priced by it first
        // (`counted`) and never reaches this key.
        "F_p multiplications" | "F_p multiplications (grid)" => ok("ns_per_fp_mul"),
        u if u.starts_with("word operations (partial") => refuse(
            "ns_per_word_op",
            "crossbred-f2's count covers extraction, transform and filter only; the per-point linear solves are counted apart (solve_row_ops), so the count does not cover the engine's whole run",
        ),
        u if u.starts_with("vector XORs (Gray-code steps, 8 lanes of 32 bits") => {
            ok("ns_per_vector_xor_8x32")
        }
        u if u.starts_with("vector XORs (Gray-code steps, 16 lanes of 32 bits") => {
            ok("ns_per_vector_xor_16x32")
        }
        u if u.starts_with("F_p multiply-adds") => ok("ns_per_f4_muladd"),
        _ => None,
    }
}

/// Native counters that a phase charges only when their key is pinned
/// (`ic_boundary::Calibration::pinned_unit_ns`); unpinned, `ic_boundary::price_phase` leaves them
/// out exactly as before and `ecbench` lists the `*_uncharged` ones as
/// unpriced.  The names are frozen (records carry them), so a pinned
/// `pkm_*_uncharged` counter is charged under its old name and dropped
/// from `unpriced`.  `pkm_crosscheck_fp_mults` (the opt-in cross-check) is
/// not here: it is a diagnostic, not part of the solve.
pub const PINNABLE_NATIVE: &[(&str, &str)] = &[
    ("hashed_tensor_steps", "ns_per_hashed_tensor_step"),
    ("pkm_system_fp_mults_uncharged", "ns_per_fp_mul"),
    ("pkm_grid_fp_mults_uncharged", "ns_per_fp_mul"),
    ("pkm_shared_poly_fp_mults_uncharged", "ns_per_fp_mul"),
    ("pkm_trace_learn_spec_fp_mults_uncharged", "ns_per_fp_mul"),
    ("pkm_t0_base_map_fp_mults_uncharged", "ns_per_fp_mul"),
    ("pkm_base_check_fp_mults_uncharged", "ns_per_fp_mul"),
    ("pkm_t0_target_fp_mults_uncharged", "ns_per_fp_mul"),
];
