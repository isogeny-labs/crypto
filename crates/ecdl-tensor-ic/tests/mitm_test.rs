//! Meet-in-the-middle relation search: the same relation network contracted
//! from both ends. Checks that it finds a witness exactly when the network has
//! one, that the witness is valid, and that it costs about sqrt(2r) point
//! additions instead of r.

use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use ecdl_tensor_ic::{
    mitm_find_relation, random_prime_order_curve, relation_network, Counter, Point,
    SparseContractor, INFINITY,
};

fn valid(
    c: &ecdl_tensor_ic::Curve,
    fb: &ecdl_tensor_ic::FactorBases,
    r: Point,
    pts: &[Point],
) -> bool {
    if pts.len() != fb.sets.len() {
        return false;
    }
    let mut sum = INFINITY;
    for (i, pt) in pts.iter().enumerate() {
        if !c.on_curve(*pt) || !fb.member(i, *pt) {
            return false;
        }
        sum = c.add(sum, *pt);
    }
    sum == r
}

// mitm finds a decomposition exactly when the network counts one, and its
// witness is valid.
#[test]
fn mitm_agrees_with_the_network_count() {
    let mut rng = StdRng::seed_from_u64(101);
    for bits in [7usize, 9] {
        for m in [2usize, 3, 4] {
            let c = random_prime_order_curve(&mut rng, bits).expect("curve");
            let fb = ecdl_tensor_ic::relation::new_affine_factor_bases(&c, m, &mut rng);
            let mut found = 0;
            for _ in 0..20 {
                let k = 1 + rng.gen_range(0..c.order - 1);
                let r = c.mul(k, c.g);
                let (net, _) = relation_network(&c, &fb, r);
                let count = SparseContractor::default()
                    .count(&net, &HashMap::new())
                    .expect("count");
                match mitm_find_relation(&c, &fb, r) {
                    Some(pts) => {
                        assert!(
                            count > 0,
                            "m={}: mitm found a witness but the count is 0",
                            m
                        );
                        assert!(valid(&c, &fb, r, &pts), "m={}: invalid witness", m);
                        found += 1;
                    }
                    None => assert_eq!(count, 0, "m={}: count {} but no witness", m, count),
                }
            }
            assert!(found > 0, "m={}: no target had a decomposition", m);
        }
    }
}

// The cost evidence: one attempt costs about sqrt(2r) additions instead of
// the forward contraction's |F_1| x ... x |F_m|.
#[test]
fn mitm_costs_far_fewer_additions_than_forward_contraction() {
    let mut rng = StdRng::seed_from_u64(103);
    let c = random_prime_order_curve(&mut rng, 16).expect("curve");
    let m = 2;
    let fb = ecdl_tensor_ic::relation::new_affine_factor_bases(&c, m, &mut rng);
    let k = 1 + rng.gen_range(0..c.order - 1);
    let r = c.mul(k, c.g);

    ecdl_tensor_ic::opcount::start();
    ecdl_tensor_ic::opcount::set_phase("relation_search");
    let (net, _) = relation_network(&c, &fb, r);
    let _ = SparseContractor::default()
        .count(&net, &HashMap::new())
        .expect("count");
    let forward = ecdl_tensor_ic::opcount::take()
        .get("relation_search.adds")
        .copied()
        .unwrap_or(0);

    ecdl_tensor_ic::opcount::start();
    ecdl_tensor_ic::opcount::set_phase("relation_search");
    let _ = mitm_find_relation(&c, &fb, r);
    let mitm = ecdl_tensor_ic::opcount::take()
        .get("relation_search.adds")
        .copied()
        .unwrap_or(0);

    assert!(forward > 0 && mitm > 0, "no additions were counted");
    assert!(
        mitm * 4 < forward,
        "mitm {} additions vs forward contraction {}: expected at least 4x fewer",
        mitm,
        forward
    );
    let root = (2.0 * c.order as f64).sqrt();
    assert!(
        (mitm as f64) < 8.0 * root,
        "mitm did {} additions, more than 8 sqrt(2r) = {}",
        mitm,
        8.0 * root
    );
}

// The solver path is end-to-end: with mitm on, solve still recovers and
// verifies the secret.
#[test]
fn solver_with_mitm_recovers_the_secret() {
    let mut rng = StdRng::seed_from_u64(107);
    for m in [2usize, 3] {
        let c = random_prime_order_curve(&mut rng, 10).expect("curve");
        let xi = rng.gen_range(0..c.order);
        let q = c.mul(xi, c.g);
        let mut s = ecdl_tensor_ic::Solver::new(
            c,
            m,
            Box::new(SparseContractor::default()),
            StdRng::seed_from_u64(11),
        );
        s.mitm = true;
        let (got, _) = s.solve(q).expect("solve");
        assert_eq!(got, xi);
    }
}

// The left table depends only on the factor base: one table reused across
// many targets returns exactly the witness (same points, same order) that the
// per-call search `mitm_find_relation` returns, which rebuilds it each time.
#[test]
fn mitm_table_reuse_matches_per_call_search() {
    let mut rng = StdRng::seed_from_u64(109);
    for bits in [7usize, 9, 11] {
        for m in [2usize, 3, 4] {
            let c = random_prime_order_curve(&mut rng, bits).expect("curve");
            let fb = if m == 3 {
                ecdl_tensor_ic::relation::new_factor_bases(&c, m, &mut rng)
            } else {
                ecdl_tensor_ic::relation::new_affine_factor_bases(&c, m, &mut rng)
            };
            let table = ecdl_tensor_ic::MitmTable::new(&c, &fb);
            assert!(!table.is_empty());
            let mut hits = 0;
            for _ in 0..40 {
                let k = rng.gen_range(0..c.order);
                let r = c.mul(k, c.g);
                let reused = table.find(r);
                assert_eq!(reused, mitm_find_relation(&c, &fb, r), "b={bits} m={m}");
                if let Some(pts) = reused {
                    assert!(valid(&c, &fb, r, &pts));
                    hits += 1;
                }
            }
            assert!(hits > 0, "b={bits} m={m}: no hits");
        }
    }
}

// Accounting: n searches with one table cost the table's build once plus the
// n right-half searches; the per-call path costs the build n times. Every
// addition actually executed is still charged.
#[test]
fn mitm_table_charges_its_build_once() {
    let adds = |f: &mut dyn FnMut()| {
        ecdl_tensor_ic::opcount::start();
        ecdl_tensor_ic::opcount::set_phase("relation_search");
        f();
        let t = ecdl_tensor_ic::opcount::take();
        t.get("relation_search.adds").copied().unwrap_or(0)
            + t.get("relation_search.doubles").copied().unwrap_or(0)
    };
    let mut rng = StdRng::seed_from_u64(113);
    let c = random_prime_order_curve(&mut rng, 12).expect("curve");
    // m = 4, so the left half sums two points: with m <= 3 the left table is
    // O + A_1, which costs no counted addition (only hash-map work).
    let fb = ecdl_tensor_ic::relation::new_affine_factor_bases(&c, 4, &mut rng);
    let targets: Vec<Point> = (0..25)
        .map(|_| c.mul(rng.gen_range(0..c.order), c.g))
        .collect();

    let build = adds(&mut || {
        let _ = ecdl_tensor_ic::MitmTable::new(&c, &fb);
    });
    let table = ecdl_tensor_ic::MitmTable::new(&c, &fb);
    let reused = adds(&mut || {
        let _ = ecdl_tensor_ic::MitmTable::new(&c, &fb);
        for &r in &targets {
            let _ = table.find(r);
        }
    });
    let per_call = adds(&mut || {
        for &r in &targets {
            let _ = mitm_find_relation(&c, &fb, r);
        }
    });
    assert!(build > 0);
    assert_eq!(per_call - reused, (targets.len() as u64 - 1) * build);
}

// Solver-level pin: with the table built once per factor base, the mitm solve
// returns the same secret, relation count and attempt count as before the
// change (values recorded from the per-call implementation), on both the
// parallel path and the sequential path taken while tallying.
#[test]
fn solver_mitm_results_are_unchanged_by_table_reuse() {
    // (bits, m, tally) -> (xi, relations, attempts, restarts, N, old
    // relation_search.adds of the per-call path, which rebuilt the table on
    // every attempt).
    type Case = (usize, usize, bool);
    type Pinned = (u64, usize, usize, usize, usize, Option<u64>);
    let pins: [(Case, Pinned); 6] = [
        ((10, 2, false), (448, 351, 368, 0, 91, None)),
        ((10, 3, false), (666, 146, 160, 0, 39, None)),
        ((10, 2, true), (448, 92, 95, 0, 91, Some(1691))),
        ((11, 3, true), (240, 35, 57, 0, 34, Some(4595))),
        ((12, 4, false), (2202, 121, 140, 0, 34, None)),
        ((12, 4, true), (2202, 35, 42, 0, 34, Some(5278))),
    ];
    for ((bits, m, tally), (xi_pin, rels, attempts, restarts, n, old_adds)) in pins {
        let mut rng = StdRng::seed_from_u64(127 + bits as u64 * 10 + m as u64);
        let c = random_prime_order_curve(&mut rng, bits).expect("curve");
        let xi = rng.gen_range(0..c.order);
        let q = c.mul(xi, c.g);
        let mut s = ecdl_tensor_ic::Solver::new(
            c,
            m,
            Box::new(SparseContractor::default()),
            StdRng::seed_from_u64(131),
        );
        s.mitm = true;
        if tally {
            ecdl_tensor_ic::opcount::start();
        }
        let (got, st) = s.solve(q).expect("solve");
        let t = ecdl_tensor_ic::opcount::take();
        assert_eq!(got, xi);
        assert_eq!(
            (
                got,
                st.relations,
                st.attempts,
                st.restarts,
                st.factor_base_size
            ),
            (xi_pin, rels, attempts, restarts, n),
            "b={bits} m={m} tally={tally}"
        );
        let adds = t.get("relation_search.adds").copied();
        eprintln!("b={bits} m={m}: relation_search.adds {adds:?} (per-call path: {old_adds:?})");
        match (adds, old_adds) {
            // m <= 3: the left table is O + A_1, no counted addition, so the
            // tally is unchanged; m >= 4: the per-attempt rebuilds are gone.
            (Some(new), Some(old)) if m <= 3 => assert_eq!(new, old, "b={bits} m={m}"),
            (Some(new), Some(old)) => assert!(new < old, "b={bits} m={m}: {new} !< {old}"),
            (None, None) => {}
            other => panic!("b={bits} m={m}: unexpected tally {other:?}"),
        }
    }
}
