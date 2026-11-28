//! A/B harness: static occurrence-ranked variable relabelling for the
//! inherited-F4 splitting tree (lane research/ic_binary_unsat_cost_20261119).
//!
//! Candidate: before solving a decomposition system, rename its variables so
//! that the variable occurring in the most monomials has the highest index
//! (arm `freq_hi`: `SplitRule::HighestFree` then splits on it first), or the
//! lowest (arm `freq_lo`, exploratory).  The system, the solver, the node
//! budget and the split rule are untouched; roots are mapped back through the
//! inverse permutation, so verdicts and root sets must match arm `base`
//! exactly (hard gate: the harness aborts otherwise).
//!
//! ```bash
//! RAYON_NUM_THREADS=1 cargo run --release --example ic_unsat_relabel -- \
//!     --a 1 --n 17 --dim 6 --m 3 --targets 24 --reps 3 --seed 1 --budget 200000
//! ```
//!
//! Prints one JSON line per (target, rep, arm).  Arm order within a
//! (target, rep) block is a seeded shuffle.  Arms: `base`, `base2` (A/A),
//! `freq_hi`, `freq_lo`.  Cost per attempt: wall ms, counted word ops (the
//! engine's own word XORs plus the `bitop/v1` tally), reductions, splits.

use crypto_lib::cryptanalysis::bitop_tally;
use crypto_lib::cryptanalysis::koblitz_groebner::{
    f4_word_ops_thread, permute_poly, solve_boolean_system, split_rule_default, FieldStructure,
    SolveOptions, SolverEngine,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_standard_subspace_factor_base, decomposition_solver_equations, KoblitzCurve,
};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::time::Instant;

fn flag(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

fn occurrence(
    equations: &[crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolPoly],
    n: usize,
) -> Vec<u64> {
    let mut c = vec![0u64; n];
    for p in equations {
        for t in &p.terms {
            let mut m = t.mask;
            while m != 0 {
                c[m.trailing_zeros() as usize] += 1;
                m &= m - 1;
            }
        }
    }
    c
}

/// `perm[old] = new`.  `hi`: most frequent variable gets the highest index.
fn rank_perm(counts: &[u64], hi: bool) -> Vec<u32> {
    let mut order: Vec<usize> = (0..counts.len()).collect();
    // ascending by (count, old index) for hi; descending count for lo
    order.sort_by_key(|&v| {
        if hi {
            (counts[v], v as u64)
        } else {
            (u64::MAX - counts[v], v as u64)
        }
    });
    let mut perm = vec![0u32; counts.len()];
    for (new, &old) in order.iter().enumerate() {
        perm[old] = new as u32;
    }
    perm
}

fn gate_check(reference: &[u64], roots: &[u64], exhausted: bool, ti: usize, rep: usize, arm: &str) {
    if exhausted {
        assert!(
            roots.iter().all(|r| reference.contains(r)),
            "UNSOUND ROOT target {ti} rep {rep} arm {arm}"
        );
    } else {
        assert_eq!(
            reference, roots,
            "VERDICT/ROOT MISMATCH target {ti} rep {rep} arm {arm}"
        );
    }
}

fn unpermute(mask: u64, perm: &[u32]) -> u64 {
    // root bit `perm[old]` of the solved system is variable `old`
    let mut out = 0u64;
    for (old, &new) in perm.iter().enumerate() {
        if mask >> new & 1 == 1 {
            out |= 1u64 << old;
        }
    }
    out
}

fn main() {
    let a: u8 = flag("--a").and_then(|v| v.parse().ok()).unwrap_or(1);
    let n: u32 = flag("--n").and_then(|v| v.parse().ok()).unwrap_or(17);
    let dim: u32 = flag("--dim").and_then(|v| v.parse().ok()).unwrap_or(6);
    let m: usize = flag("--m").and_then(|v| v.parse().ok()).unwrap_or(3);
    let targets: usize = flag("--targets").and_then(|v| v.parse().ok()).unwrap_or(24);
    let reps: usize = flag("--reps").and_then(|v| v.parse().ok()).unwrap_or(3);
    let seed: u64 = flag("--seed").and_then(|v| v.parse().ok()).unwrap_or(1);
    let budget: usize = flag("--budget")
        .and_then(|v| v.parse().ok())
        .unwrap_or(200_000);
    let kc = match KoblitzCurve::new(a, n) {
        Some(k) => k,
        None => {
            eprintln!("K({a},{n}) has no usable subgroup");
            return;
        }
    };
    let fb = build_standard_subspace_factor_base(&kc, dim).expect("factor base");
    let st = FieldStructure::new(kc.n, &kc.curve.irreducible);
    let engine = SolverEngine::InheritedF4 { max_degree: 3 };
    let r = kc.subgroup_order.to_u64().expect("small order");
    let mut rng = Rng(seed ^ ((a as u64) << 56) ^ ((n as u64) << 40) ^ ((dim as u64) << 32));
    let arms = ["base", "base2", "freq_hi", "freq_lo"];
    let opts = SolveOptions {
        engine,
        max_solutions: usize::MAX,
        node_budget: budget,
        split_rule: split_rule_default(),
    };
    for ti in 0..targets {
        let k = 1 + rng.next() % (r - 1);
        let target = kc.mul(kc.generator(), &BigUint::from(k));
        let (eqs, nv) = match decomposition_solver_equations(&kc, &fb, &st, &target, m, engine) {
            Some(x) => x,
            None => continue,
        };
        let counts = occurrence(&eqs, nv);
        let perms: Vec<Option<Vec<u32>>> = vec![
            None,
            None,
            Some(rank_perm(&counts, true)),
            Some(rank_perm(&counts, false)),
        ];
        let mut reference: Option<(Vec<u64>, bool)> = None;
        let mut pending: Vec<(usize, Vec<u64>, bool, usize)> = Vec::new();
        for rep in 0..reps {
            // seeded shuffle of arm order for this block
            let mut order = [0usize, 1, 2, 3];
            for i in (1..4).rev() {
                let j = (rng.next() % (i as u64 + 1)) as usize;
                order.swap(i, j);
            }
            for &ai in &order {
                let sys: Vec<_> = match &perms[ai] {
                    None => eqs.clone(),
                    Some(p) => eqs.iter().map(|e| permute_poly(e, p)).collect(),
                };
                let w0 = f4_word_ops_thread();
                let t0 = bitop_tally::snapshot();
                let start = Instant::now();
                let (roots, stats) = solve_boolean_system(&sys, nv, &opts);
                let ms = start.elapsed().as_secs_f64() * 1000.0;
                let tally = bitop_tally::snapshot().since(t0);
                let native = f4_word_ops_thread().wrapping_sub(w0);
                let mut roots: Vec<u64> = match &perms[ai] {
                    None => roots.to_vec(),
                    Some(p) => roots.iter().map(|&x| unpermute(x, p)).collect(),
                };
                roots.sort_unstable();
                roots.dedup();
                // Hard gate (checked per attempt): `base` is the reference.  An
                // arm that finishes (not exhausted) must return exactly the
                // reference root set; an arm that exhausts its node budget is
                // censored and may only return roots that are in the reference.
                if ai == 0 && reference.is_none() {
                    assert!(
                        !stats.exhausted,
                        "base arm exhausted its budget: raise --budget"
                    );
                    reference = Some((roots.clone(), false));
                }
                if ai == 0 {
                    assert_eq!(reference.as_ref().unwrap().0, roots, "base not repeatable");
                } else if let Some(rf) = &reference {
                    gate_check(&rf.0, &roots, stats.exhausted, ti, rep, arms[ai]);
                } else {
                    pending.push((ai, roots.clone(), stats.exhausted, rep));
                }
                println!(
                    "{{\"a\":{a},\"n\":{n},\"dim\":{dim},\"m\":{m},\"nv\":{nv},\"target\":{ti},\"k\":{k},\"rep\":{rep},\"arm\":\"{}\",\"roots\":{},\"exhausted\":{},\"ms\":{ms:.4},\"word_ops\":{},\"native_word_ops\":{native},\"reductions\":{},\"splits\":{},\"propagations\":{},\"infeasible\":{}}}",
                    arms[ai],
                    roots.len(),
                    stats.exhausted,
                    native.wrapping_add(tally.word_ops),
                    stats.reductions,
                    stats.splits,
                    stats.propagations,
                    stats.infeasible_branches
                );
            }
        }
        let rf = reference.expect("base ran");
        for (ai, roots, exhausted, rep) in pending {
            gate_check(&rf.0, &roots, exhausted, ti, rep, arms[ai]);
        }
    }
}
