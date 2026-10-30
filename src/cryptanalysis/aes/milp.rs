//! MILP-based trail search for AES (Mouha, Wang, Gu, Preneel — CRYPTO 2011).
//!
//! Mixed-Integer Linear Programming (MILP) became a major tool for
//! symmetric cryptanalysis when Mouha et al. observed that the
//! **number of active S-boxes** in a differential (or linear) trail
//! can be encoded as a system of inequalities and solved with a
//! generic MILP solver (Gurobi, CPLEX, CBC). The same approach is
//! now used for related-key attacks, division-property propagation,
//! and impossible-differential trail search.
//!
//! # Encoding for AES
//!
//! Variables (per round `r`, per byte position `(c, i) ∈ [0, 4) × [0, 4)`):
//!
//! - `a_(r, c, i) ∈ {0, 1}` — "byte is active" indicator at the input
//!   of the S-box layer of round `r`.
//! - `d_(r, c)` — "any byte of column `c` is active" after MixColumns
//!   of round `r` (auxiliary).
//!
//! Constraints:
//!
//! 1. **SubBytes / ShiftRows preserve activity.** Activity at input
//!    of round `r+1` is a permutation of activity at output of MC of
//!    round `r`. (Linear equality constraints.)
//! 2. **MixColumns branch number 5.** For each column `c` and round
//!    `r`: `sum_i a_(r+1, c', i) + sum_i a_(r, c, i) ≥ 5 · d_(r, c)`
//!    where `c' = (c - i) mod 4` (the SR-mapped destination column).
//! 3. **Indicator coupling.** `d_(r, c) ≥ a_(r+1, c', i)` for each `i`,
//!    and `d_(r, c) ≥ a_(r, c, i)` for each `i` (a column is "any-active"
//!    iff at least one byte is active).
//! 4. **Non-triviality.** `sum_{r, c, i} a_(r, 0, c, i) ≥ 1` (the trail
//!    starts with at least one active byte).
//!
//! Objective: `minimise sum_{r, c, i} a_(r, c, i)` — the total number
//! of active S-boxes over the trail.
//!
//! For 4 rounds the optimum is **25**: this is the Daemen-Rijmen
//! classical bound. MILP recovers this value automatically; with
//! AES's max DP per S-box of `2⁻⁶`, the probability bound becomes
//! `2⁻¹⁵⁰`.
//!
//! # What this module ships
//!
//! Implementing a general LP/MILP solver from scratch is a large
//! undertaking. Instead we provide:
//!
//! - [`active_sboxes_for_trail`] — given an explicit activity pattern
//!   per round (a candidate "trail"), verify that it satisfies the
//!   branch-number constraints and return its active S-box count.
//! - [`enumerate_low_weight_trails`] — a legacy accessor for stored
//!   classical lower bounds over 1–4 rounds. It does not perform
//!   enumeration or produce a trail certificate.
//!
//! For higher round counts, point your favourite MILP solver at the
//! constraint set above.

use super::reduced::RoundOps;

/// Activity pattern: 16 booleans, one per byte position (column-major).
pub type ActivityPattern = [bool; 16];

/// Convert a count of bytes per column to per-byte activity by setting
/// the first `k` rows of each column active. Useful for building
/// canonical trails.
pub fn canonical_pattern(active_per_column: [usize; 4]) -> ActivityPattern {
    let mut p = [false; 16];
    for c in 0..4 {
        for r in 0..active_per_column[c] {
            p[4 * c + r] = true;
        }
    }
    p
}

/// Number of active bytes.
pub fn weight(p: &ActivityPattern) -> u32 {
    p.iter().filter(|&&b| b).count() as u32
}

/// Apply ShiftRows to an activity pattern (deterministic permutation).
pub fn apply_shiftrows(p: &ActivityPattern) -> ActivityPattern {
    // Use the RoundOps SR by encoding activity as 0xff-vs-0x00 in a
    // dummy state.
    let mut state: [[u8; 4]; 4] = [[0; 4]; 4];
    for c in 0..4 {
        for r in 0..4 {
            state[c][r] = if p[4 * c + r] { 0xff } else { 0 };
        }
    }
    RoundOps::shift_rows(&mut state);
    let mut q = [false; 16];
    for c in 0..4 {
        for r in 0..4 {
            q[4 * c + r] = state[c][r] != 0;
        }
    }
    q
}

/// Per-column active byte count.
pub fn column_weights(p: &ActivityPattern) -> [u32; 4] {
    let mut w = [0u32; 4];
    for c in 0..4 {
        for r in 0..4 {
            if p[4 * c + r] {
                w[c] += 1;
            }
        }
    }
    w
}

/// Check necessary MixColumns activity conditions for one column. A zero
/// input has zero output and vice versa; when both are nonzero, the
/// branch-number condition is `in + out >= 5`. Passing does not certify
/// a realizable pair of exact byte differences.
pub fn mc_branch_number_ok(input_active: u32, output_active: u32) -> bool {
    if input_active > 4 || output_active > 4 {
        return false;
    }
    if input_active == 0 || output_active == 0 {
        return input_active == 0 && output_active == 0;
    }
    input_active + output_active >= 5
}

/// Count active S-boxes for an explicit trail of activity patterns
/// `[round_0_input, round_1_input, …, round_n_input]`. Each pair of
/// adjacent patterns is connected by `SR ∘ MC`; the function verifies
/// necessary activity constraints at every transition and returns
/// `Some(total)` if none is violated. It does not certify an exact trail.
pub fn active_sboxes_for_trail(trail: &[ActivityPattern]) -> Option<u32> {
    if trail.is_empty() || !trail[0].iter().any(|&active| active) {
        return None;
    }
    let mut total = 0u32;
    for (i, p) in trail.iter().enumerate() {
        total += weight(p);
        if i + 1 == trail.len() {
            break;
        }
        // Apply SR to p to get post-SR activity.
        let post_sr = apply_shiftrows(p);
        // The next pattern is post_sr → post_MC → next.
        // Branch-number-5 per column: input is post_sr column, output
        // is next round's input column.
        let in_cw = column_weights(&post_sr);
        let out_cw = column_weights(&trail[i + 1]);
        for c in 0..4 {
            if !mc_branch_number_ok(in_cw[c], out_cw[c]) {
                return None;
            }
        }
    }
    Some(total)
}

/// Return stored classical lower bounds for 1–4 rounds. Despite its legacy
/// name, this function does not enumerate trails or certify a solution.
pub fn enumerate_low_weight_trails(rounds: usize) -> u32 {
    // For 1 round: minimum is 1 (the active starting byte).
    if rounds == 1 {
        return 1;
    }
    // For 2 rounds: start with 1 active byte, MC R1 forces ≥ 4 in
    // the output column; total = 1 + 4 = 5.
    if rounds == 2 {
        return 5;
    }
    // Legacy reference bound; this function does not construct a witness.
    if rounds == 3 {
        return 9;
    }
    // 4 rounds: 25 (Daemen-Rijmen).
    if rounds == 4 {
        return 25;
    }
    // Beyond: not implemented; would call out to a MILP solver.
    u32::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_rows_permutes_activity() {
        let p = canonical_pattern([4, 0, 0, 0]);
        let q = apply_shiftrows(&p);
        // Column 0 had all 4 rows active; after SR, the 4 active bytes
        // are at positions (0,0), (3,1), (2,2), (1,3) — one per column.
        let cw = column_weights(&q);
        assert_eq!(cw, [1, 1, 1, 1]);
    }

    #[test]
    fn branch_number_constraint() {
        assert!(mc_branch_number_ok(0, 0));
        assert!(!mc_branch_number_ok(0, 4));
        assert!(!mc_branch_number_ok(4, 0));
        assert!(!mc_branch_number_ok(1, 5));
        assert!(!mc_branch_number_ok(1, 1)); // 2 < 5
        assert!(!mc_branch_number_ok(2, 2)); // 4 < 5
        assert!(mc_branch_number_ok(1, 4));
        assert!(mc_branch_number_ok(2, 3));
        assert!(mc_branch_number_ok(4, 4));
    }

    /// A valid 2-round trail with 1+4 = 5 active S-boxes.
    #[test]
    fn two_round_trail_achieves_five() {
        let r0 = canonical_pattern([1, 0, 0, 0]);
        let r1 = canonical_pattern([4, 0, 0, 0]);
        let t = active_sboxes_for_trail(&[r0, r1]).expect("valid trail");
        assert_eq!(t, 5);
    }

    /// An invalid trail: 1 active → 1 active across MC fails branch
    /// number.
    #[test]
    fn invalid_trail_rejected() {
        let r0 = canonical_pattern([1, 0, 0, 0]);
        let r1 = canonical_pattern([1, 0, 0, 0]);
        assert!(active_sboxes_for_trail(&[r0, r1]).is_none());
        assert!(active_sboxes_for_trail(&[[false; 16], [false; 16]]).is_none());
        assert!(active_sboxes_for_trail(&[[false; 16], [true; 16]]).is_none());
    }

    #[test]
    fn classical_bounds() {
        assert_eq!(enumerate_low_weight_trails(1), 1);
        assert_eq!(enumerate_low_weight_trails(2), 5);
        assert_eq!(enumerate_low_weight_trails(3), 9);
        assert_eq!(enumerate_low_weight_trails(4), 25);
    }
}
