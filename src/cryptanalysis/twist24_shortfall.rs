//! **E41 of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`: where the order-24
//! walk's step shortfall comes from (Stage 0).**
//!
//! E34 (§9.39) measured the order-24 `⟨±ψ⟩` walk on E18's `j = 0` curves at
//! `1.701×` fewer steps than the order-6 walk, pooled, against the `√4 = 2`
//! an ideal walk on `n/24` classes would take.  This module is an
//! instrumented copy of E34's walk loop ([`super::twist24_rep::rho_j0_counted`],
//! which it does not modify): the same walk, the same counted unit and the
//! same native-operation counters, plus instrument counters that are not
//! part of the walk's cost ([`Shortfall`]): look-ahead redirects, fruitless
//! revisits by cycle length, representative ties, the useful collision's
//! tail and cycle, and the partition occupancy.  The walk has exhaustive
//! storage, so it has no distinguished-point losses by construction.

use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

use super::gaudry_quartic::{Curve4, Pt4};
use super::quartic_folds::{RhoFold, RhoRun4F, Twist24Instance};
use super::residual_walk::inv_mod;
use super::twist24_rep::{part, rep, visit_digest, CountedWalk, NativeOps, RepArm, RepTables};

fn mulm(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

/// Fruitless-cycle length classes: `1`–`8`, and longer.
pub const CYCLE_BINS: usize = 9;

/// The instrument (not charged to the walk).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Shortfall {
    /// Class size `m` (`6` or `24`) and the number of classes `(n − 1)/m`.
    pub class_size: u64,
    pub classes: u64,
    /// `√(π·classes/2)`, the expected steps to the first repeat of a random
    /// mapping on the classes.
    pub ideal_steps: f64,
    /// Distinct classes stored when the walk stopped.
    pub stored: u64,
    /// Steps whose first successor fell in the step's own partition, so the
    /// look-ahead took the next partition.
    pub redirects: u64,
    /// Revisits with `Δb = 0` (fruitless cycles), each escaped by doubling.
    pub fruitless: u64,
    /// Those revisits by cycle length (steps since the revisited class was
    /// stored); index `k − 1` for length `k ≤ 8`, index `8` for longer.
    pub fruitless_len: [u64; CYCLE_BINS],
    /// The sum of those cycle lengths.
    pub fruitless_cycle_steps: u64,
    /// Escapes whose doubled representative was already stored.
    pub escape_rehits: u64,
    /// Representative calls on an input with `x₀ = 0` (a tie on coordinate 0).
    pub tie_inputs: u64,
    /// The useful collision: the revisited class was stored at step `tail + 1`
    /// and met again `cycle` steps later.
    pub final_tail: u64,
    pub final_cycle: u64,
    /// Stored classes by partition.
    pub part_hist: Vec<u64>,
}

/// E34's walk ([`super::twist24_rep::rho_j0_counted`]) with the instrument.
pub fn rho_j0_instrumented(
    inst: &Twist24Instance,
    arm: RepArm,
    seed: u64,
    karatsuba: bool,
) -> (CountedWalk, Shortfall) {
    let curve = Curve4::with_params_arith(inst.core.p, inst.core.a, inst.core.b, karatsuba);
    let f = &curve.f;
    let n = inst.core.n;
    let mut rng = StdRng::seed_from_u64(seed ^ 0x8D04_F01D);
    let jumps = 32usize;
    let mut ops = NativeOps::default();
    let mut rep_ops = NativeOps::default();
    let (mut rep_calls, mut rep_fp_muls) = (0u64, 0u64);
    let class_size = if arm.is_order24() { 24 } else { 6 };
    let classes = (n - 1) / class_size;
    let mut sf = Shortfall {
        class_size,
        classes,
        ideal_steps: (std::f64::consts::PI * classes as f64 / 2.0).sqrt(),
        part_hist: vec![0; jumps],
        ..Default::default()
    };
    let mults: Vec<(u64, u64, Pt4)> = (0..jumps)
        .map(|_| {
            let a = rng.gen_range(0..n);
            let b = rng.gen_range(1..n);
            (
                a,
                b,
                curve.add(&curve.mul(&inst.core.g, a), &curve.mul(&inst.core.q, b)),
            )
        })
        .collect();
    let mut a = rng.gen_range(0..n);
    let mut b = rng.gen_range(1..n);
    let start = curve.add(&curve.mul(&inst.core.g, a), &curve.mul(&inst.core.q, b));
    let tables = RepTables::new(inst, f);
    let cheap = !matches!(arm, RepArm::Order6Built | RepArm::Order24Built);
    let table_fp_muls = if cheap { tables.setup_fp_muls } else { 0 };
    f.charge(table_fp_muls);
    ops.modn += if cheap { tables.setup_modn } else { 0 };
    let mut tie_inputs = 0u64;
    let mut canon = |pt: Pt4, ops: &mut NativeOps| -> (Pt4, u64) {
        if !pt.inf && pt.x.0[0] == 0 {
            tie_inputs += 1;
        }
        let (m0, c0, n0, d0) = (f.muls(), ops.cmp, ops.neg, ops.modn);
        let out = rep(inst, &curve, &tables, ops, arm, pt);
        rep_calls += 1;
        rep_fp_muls += f.muls() - m0;
        rep_ops.cmp += ops.cmp - c0;
        rep_ops.neg += ops.neg - n0;
        rep_ops.modn += ops.modn - d0;
        out
    };
    // The instrument's own record of the step at which each class was stored.
    let mut table: HashMap<Pt4, (u64, u64, u64)> = HashMap::new();
    let (mut steps, mut cycle_escapes) = (0u64, 0u64);
    let mut digest = 0u64;
    let mut found = None;
    let mut after_escape = false;
    let cap = 64 * (n as f64).sqrt() as u64 + 1_000_000;
    let (mut pt, mult) = canon(start, &mut ops);
    ops.modn += 2;
    a = mulm(a, mult, n);
    b = mulm(b, mult, n);
    while steps <= cap {
        steps += 1;
        digest = visit_digest(digest, &pt);
        ops.table += 1;
        if let Some(&(a2, b2, s0)) = table.get(&pt) {
            if after_escape {
                sf.escape_rehits += 1;
            }
            ops.modn += 1;
            let db = (b + n - b2) % n;
            if db != 0 {
                ops.modn += 3;
                found = Some(mulm((a2 + n - a) % n, inv_mod(db, n), n));
                sf.final_tail = s0 - 1;
                sf.final_cycle = steps - s0;
                break;
            }
            let len = steps - s0;
            sf.fruitless += 1;
            sf.fruitless_len[(len as usize).clamp(1, CYCLE_BINS) - 1] += 1;
            sf.fruitless_cycle_steps += len;
            cycle_escapes += 1;
            let (doubled, m) = canon(curve.add(&pt, &pt), &mut ops);
            pt = doubled;
            ops.modn += 4;
            a = mulm(mulm(2, a, n), m, n);
            b = mulm(mulm(2, b, n), m, n);
            after_escape = true;
            continue;
        }
        after_escape = false;
        ops.table += 1;
        table.insert(pt, (a, b, steps));
        ops.hash += 1;
        let mut j = part(&pt, jumps);
        sf.part_hist[j] += 1;
        let (mut next, mut m) = canon(curve.add(&pt, &mults[j].2), &mut ops);
        ops.hash += 1;
        if part(&next, jumps) == j {
            sf.redirects += 1;
            j = (j + 1) % jumps;
            (next, m) = canon(curve.add(&pt, &mults[j].2), &mut ops);
        }
        ops.modn += 4;
        a = mulm((a + mults[j].0) % n, m, n);
        b = mulm((b + mults[j].1) % n, m, n);
        pt = next;
    }
    sf.stored = table.len() as u64;
    sf.tie_inputs = tie_inputs;
    let (walk_muls, walk_ops) = (f.muls(), curve.ops());
    let correct = found == Some(inst.core.d)
        && found.is_some_and(|d| curve.mul(&inst.core.g, d) == inst.core.q);
    let walk = CountedWalk {
        arm,
        karatsuba,
        run: RhoRun4F {
            fold: if arm.is_order24() {
                RhoFold::Twist24
            } else {
                RhoFold::Psi
            },
            seed,
            steps,
            group_ops: curve.ops(),
            fp_muls: f.muls(),
            cycle_escapes,
            correct,
            recovered: found,
            visit_digest: digest,
            verify_fp_muls: f.muls() - walk_muls,
            verify_group_ops: curve.ops() - walk_ops,
        },
        ops,
        rep_calls,
        rep_fp_muls,
        rep_ops,
        table_fp_muls,
    };
    (walk, sf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::quartic_folds::construct_twist24;
    use crate::cryptanalysis::twist24_rep::rho_j0_counted;

    #[test]
    fn e41_instrument_is_e34s_walk() {
        let inst = construct_twist24(241, 3, 1).expect("E29's p = 241 model");
        for arm in [RepArm::Order6Cheap, RepArm::Order24Cheap] {
            for seed in [1000u64, 1001] {
                let e34 = rho_j0_counted(&inst, arm, seed, false);
                let (w, sf) = rho_j0_instrumented(&inst, arm, seed, false);
                assert_eq!(format!("{:?}", w), format!("{:?}", e34));
                assert_eq!(sf.fruitless, w.run.cycle_escapes);
                assert_eq!(sf.stored + sf.fruitless + 1, w.run.steps);
            }
        }
    }
}
