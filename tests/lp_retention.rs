//! Tests for `large_prime_graph::retention` (port of workspace#144 T3).

use crypto_lib::cryptanalysis::large_prime_filter::PartialRelation;
use crypto_lib::cryptanalysis::large_prime_graph::eval::{synthetic_stream, SyntheticParams};
use crypto_lib::cryptanalysis::large_prime_graph::retention::{
    compare_policies, cycle_count, fp_toy, retain, split_by_arity, useful_rank, RetentionError,
    RetentionPolicy,
};
use crypto_lib::cryptanalysis::large_prime_graph::{dense_rank, LpIndex};

fn pool(seed: u64, rows: usize, lp: u64, unit: bool, modulus: u64) -> Vec<PartialRelation> {
    let p = SyntheticParams {
        seed,
        rows,
        lp_columns: lp,
        arity_weights: vec![0, 3, 5],
        unit_coefficients: unit,
        ..SyntheticParams::default()
    };
    split_by_arity(&synthetic_stream(&p, modulus), modulus)
        .unwrap()
        .0
}

/// Rows whose LP coefficients cancel mod ℓ are complete, not partials.
#[test]
fn cancelling_rows_split_as_complete() {
    let mk = |large: Vec<(u64, u64)>| PartialRelation {
        id: 0,
        small: vec![],
        large,
        rhs: 0,
    };
    let rels = vec![
        mk(vec![(5, 1), (5, 2)]),
        mk(vec![(5, 1), (5, 1)]),
        mk(vec![(5, 1), (7, 1)]),
        mk(vec![]),
    ];
    let (partials, complete, wide) = split_by_arity(&rels, 3).unwrap();
    assert_eq!((partials.len(), complete.len(), wide), (2, 2, 0));
}

/// Every policy's GainDsu left nullity equals the dense-rank count, on 120
/// random batches over four prime fields, unit and random coefficients.
#[test]
fn useful_rank_matches_dense_rank_for_every_policy() {
    let mut checked = 0;
    for &ell in &[3u64, 7, 101, 1_000_003] {
        for seed in 0..15u64 {
            for unit in [false, true] {
                let rels = pool(seed, 60 + 7 * seed as usize, 40, unit, ell);
                let budget = rels.len() / 3 + seed as usize % 5;
                for policy in RetentionPolicy::ALL {
                    let kept = retain(&rels, ell, budget, policy, seed).unwrap();
                    let snap = useful_rank(&rels, ell, &kept).unwrap();
                    let subset: Vec<_> = kept.iter().map(|&i| rels[i].clone()).collect();
                    let idx = LpIndex::build(&subset, ell).unwrap();
                    let dense = dense_rank(&idx.rows, idx.num_columns(), ell).unwrap();
                    assert_eq!(
                        snap.rank, dense,
                        "ell={ell} seed={seed} unit={unit} {policy:?}"
                    );
                    assert_eq!(snap.left_nullity, kept.len() - dense);
                    let b1 = cycle_count(&rels, ell, &kept).unwrap();
                    assert!(snap.left_nullity <= b1, "useful rank above the cycle count");
                    checked += 1;
                }
            }
        }
    }
    assert_eq!(checked, 4 * 15 * 2 * RetentionPolicy::ALL.len());
}

#[test]
fn budget_is_exact_sorted_distinct_and_deterministic() {
    let ell = 1_000_003;
    let rels = pool(3, 500, 300, false, ell);
    for policy in RetentionPolicy::ALL {
        for budget in [0, 1, 137, 499, 500, 10_000] {
            let a = retain(&rels, ell, budget, policy, 9).unwrap();
            assert_eq!(a.len(), budget.min(rels.len()));
            assert!(a.windows(2).all(|w| w[0] < w[1]));
            assert_eq!(a, retain(&rels, ell, budget, policy, 9).unwrap());
        }
    }
    // The seed matters for the seeded policies only.
    let r1 = retain(&rels, ell, 100, RetentionPolicy::Random, 1).unwrap();
    let r2 = retain(&rels, ell, 100, RetentionPolicy::Random, 2).unwrap();
    assert_ne!(r1, r2);
    let d1 = retain(&rels, ell, 100, RetentionPolicy::Degree, 1).unwrap();
    assert_eq!(
        d1,
        retain(&rels, ell, 100, RetentionPolicy::Degree, 2).unwrap()
    );
}

#[test]
fn complete_and_wide_rows_are_refused() {
    let ell = 101;
    let mk = |large: Vec<(u64, u64)>| PartialRelation {
        id: 0,
        small: vec![],
        large,
        rhs: 0,
    };
    let e = retain(
        &[mk(vec![(5, 1)]), mk(vec![])],
        ell,
        1,
        RetentionPolicy::Degree,
        0,
    );
    assert_eq!(
        e,
        Err(RetentionError::UnsupportedArity { row: 1, arity: 0 })
    );
    let e = retain(
        &[mk(vec![(5, 1), (7, 1), (9, 1)])],
        ell,
        1,
        RetentionPolicy::Degree,
        0,
    );
    assert_eq!(
        e,
        Err(RetentionError::UnsupportedArity { row: 0, arity: 3 })
    );
}

/// Hand-checked graph: triangle a-b-c (odd, unit coefficients: unbalanced, so
/// it pins the component and contributes no relation by itself), plus a
/// square d-e-f-g (even: balanced, one relation), plus two singletons on h.
#[test]
fn hand_graph_counts() {
    let ell = 1_000_003;
    let mk = |id, large: Vec<u64>| PartialRelation {
        id,
        small: vec![],
        large: large.into_iter().map(|q| (q, 1)).collect(),
        rhs: 0,
    };
    let rels = vec![
        mk(0, vec![1, 2]),
        mk(1, vec![2, 3]),
        mk(2, vec![3, 1]),
        mk(3, vec![4, 5]),
        mk(4, vec![5, 6]),
        mk(5, vec![6, 7]),
        mk(6, vec![7, 4]),
        mk(7, vec![8]),
        mk(8, vec![8]),
    ];
    let all: Vec<usize> = (0..rels.len()).collect();
    let snap = useful_rank(&rels, ell, &all).unwrap();
    // rank: triangle 3, square 3, h 1  -> 7; left nullity 9 - 7 = 2
    assert_eq!((snap.rank, snap.left_nullity), (7, 2));
    // b1 = E - V + C = 9 - (8 + virtual) + 3 = 3: the odd triangle counts topologically.
    assert_eq!(cycle_count(&rels, ell, &all).unwrap(), 3);
}

#[test]
fn fp_toy_collects_replays_and_ranks() {
    let p = fp_toy::Instance::next_prime(1 << 31);
    assert_eq!(p, 2_147_483_659);
    let inst = fp_toy::Instance::new(p, 1001);
    assert!(fp_toy::is_prime(inst.ell) && (p - 1) % inst.ell == 0);
    let col = fp_toy::collect(&inst, 4096, 65536, 20_000, 0);
    assert!(col.partials.len() > 3000 && col.complete > 1000);
    assert_eq!(fp_toy::replay(&col), Ok(col.partials.len()));
    // A tampered relation must fail replay.
    let mut bad = col.clone();
    bad.exponents[0] += 1;
    assert_eq!(fp_toy::replay(&bad), Err(0));
    let n = col.partials.len();
    let out = compare_policies(&col.partials, inst.ell, n / 4, &RetentionPolicy::ALL, 0).unwrap();
    assert!(out
        .iter()
        .all(|o| o.kept == n / 4 && o.rank.left_nullity <= o.cycle_count));
}

/// SinglesThenSpectral keeps every one-LP row before any two-LP row.
#[test]
fn singles_then_spectral_takes_singletons_first() {
    let ell = 1_000_003;
    let rels = pool(5, 400, 200, false, ell);
    let idx = LpIndex::build(&rels, ell).unwrap();
    let singles = idx.rows.iter().filter(|r| r.len() == 1).count();
    assert!(singles > 20 && singles < rels.len());
    let kept = retain(&rels, ell, singles, RetentionPolicy::SinglesThenSpectral, 0).unwrap();
    assert!(kept.iter().all(|&i| idx.rows[i].len() == 1));
}
