//! Port of the Go `8d17e6c` window tests (package `ecdl`): the coordinate
//! window x < p >> k, its gadget membership test, window-derived candidate
//! generation by modular square root, and the build-cost evidence that
//! replaces Go's `BenchmarkFactorBaseBuild`.

use std::collections::{HashMap, HashSet};

use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use ecdl_tensor_ic::field::{mul_mod, sqrt_mod};
use ecdl_tensor_ic::relation::{new_affine_factor_bases, new_windowed_factor_bases, FactorBases};
use ecdl_tensor_ic::{
    gadget_relation_network, random_prime_order_curve, relation_network, Arith, Builder, Counter,
    DFSCounter, FieldPoint, Network, Point, SparseContractor,
};

/// Go: countWith(t, n, nil, nil) in gadget_test.go.
fn count_all(n: &Network) -> u64 {
    DFSCounter.count(n, &HashMap::new()).expect("count")
}

fn bi(x: u64) -> BigUint {
    BigUint::from(x)
}

// Go: TestSqrtMod
#[test]
fn sqrt_mod_matches_square_table() {
    for &p in &[11u64, 13, 17, 29, 97, 101, 241] {
        let mut sq = HashSet::new();
        for y in 0..p {
            sq.insert(mul_mod(y, y, p));
        }
        for a in 0..p {
            match sqrt_mod(a, p) {
                Some(y) => {
                    assert!(sq.contains(&a), "p={} a={}: reported a residue", p, a);
                    assert_eq!(mul_mod(y, y, p), a, "p={} a={}: {}^2 != a", p, a, y);
                }
                None => assert!(!sq.contains(&a), "p={} a={}: missed a residue", p, a),
            }
        }
    }
}

// Go: TestWindowedMembershipExhaustive
//
// The coordinate window composes with the hash: the gadget must accept
// exactly the points with x < x_bound that pass the hash test, and the window
// must shrink F_i without changing its target size.
#[test]
fn windowed_membership_exhaustive() {
    let mut rng = StdRng::seed_from_u64(19);
    let mut found: Option<(ecdl_tensor_ic::Curve, FactorBases)> = None;
    for _ in 0..50 {
        let c = random_prime_order_curve(&mut rng, 4).expect("curve");
        let fb = new_windowed_factor_bases(&c, 2, &mut rng, 1);
        if !fb.x_bound.is_empty() {
            found = Some((c, fb));
            break;
        }
    }
    let (c, fb) = found.expect("window was always dropped");
    let ar = Arith::new(&bi(c.p));
    for i in 0..fb.sets.len() {
        for pt in &fb.sets[i] {
            assert!(
                pt.x < fb.x_bound[i],
                "F_{} contains {:?} outside the window",
                i,
                pt
            );
        }
        let mut b = Builder::new();
        b.exhaustive = true;
        let px = b.field_var(&ar, "x");
        let py = b.field_var(&ar, "y");
        b.hint(&[], &[px.clone(), py.clone()], |_| Vec::new());
        let pt = FieldPoint {
            x: px,
            y: py,
            inf: None,
        };
        b.validity_check(&ar, &c, &pt);
        b.membership_check(&ar, &fb, i, &pt);
        let n = b.finalize();
        assert_eq!(
            count_all(&n),
            fb.sets[i].len() as u64,
            "F_{}: gadget accepts a different number of encodings than |F_i|",
            i
        );
    }
}

// Go: TestWindowCandidates
#[test]
fn window_candidates_match_predicate() {
    let mut rng = StdRng::seed_from_u64(29);
    let c = random_prime_order_curve(&mut rng, 7).expect("curve");
    let fb = new_windowed_factor_bases(&c, 3, &mut rng, 1);
    assert!(!fb.x_bound.is_empty(), "window was dropped");
    for i in 0..fb.sets.len() {
        let got = fb.window_candidates(&c, i);
        let want: HashSet<Point> = fb.sets[i].iter().copied().collect();
        assert_eq!(got.len(), want.len(), "F_{}: point count differs", i);
        let mut seen = HashSet::new();
        for pt in &got {
            assert!(want.contains(pt), "F_{}: {:?} not in F_i", i, pt);
            assert!(seen.insert(*pt), "F_{}: duplicate {:?}", i, pt);
        }
    }
}

// Go: TestWindowedListMatchesFull
//
// Building the sets from the window must agree with the membership predicate
// evaluated over the whole group, including the column index.
#[test]
fn windowed_list_matches_full_predicate() {
    let mut rng = StdRng::seed_from_u64(31);
    let c = random_prime_order_curve(&mut rng, 7).expect("curve");
    let fb = new_windowed_factor_bases(&c, 3, &mut rng, 1);
    assert!(!fb.x_bound.is_empty(), "window was dropped");

    let all = c.points();
    let mut full_sets: Vec<Vec<Point>> = vec![Vec::new(); fb.sets.len()];
    let mut full_points: Vec<Point> = Vec::new();
    let mut full_index: HashMap<Point, usize> = HashMap::new();
    for pt in &all {
        for i in 0..fb.sets.len() {
            if fb.member(i, *pt) {
                full_sets[i].push(*pt);
                if !full_index.contains_key(pt) {
                    full_index.insert(*pt, full_points.len());
                    full_points.push(*pt);
                }
            }
        }
    }
    for i in 0..fb.sets.len() {
        assert_eq!(fb.sets[i].len(), full_sets[i].len(), "F_{}: size", i);
        assert_eq!(fb.sets[i], full_sets[i], "F_{}: order differs", i);
    }
    assert_eq!(fb.points.len(), full_points.len());
    assert_eq!(fb.index.len(), full_index.len());
    for (pt, col) in &full_index {
        assert_eq!(fb.index.get(pt), Some(col), "column of {:?} differs", pt);
    }
}

// Go: TestWindowedRelationCounts
//
// Windowed gadget networks count what the whole-point network counts.
#[test]
fn windowed_relation_counts_match_whole_point() {
    let mut rng = StdRng::seed_from_u64(23);
    let c = random_prime_order_curve(&mut rng, 7).expect("curve");
    let ar = Arith::new(&bi(c.p));
    let fb = new_windowed_factor_bases(&c, 2, &mut rng, 1);
    assert!(!fb.x_bound.is_empty(), "window was dropped");
    for ti in 0..4 {
        let k = 1 + rng.gen_range(0..c.order - 1);
        let r = c.mul(k, c.g);
        let (whole, _) = relation_network(&c, &fb, r);
        let want = SparseContractor::default()
            .count(&whole, &HashMap::new())
            .expect("count");
        let (n, _) = gadget_relation_network(&c, &ar, &fb, r);
        assert_eq!(count_all(&n), want, "target {}: counts differ", ti);
    }
}

// Go: BenchmarkFactorBaseBuild, as deterministic evidence instead of wall
// time: the window must replace whole-group listing with window-only listing.
#[test]
fn window_build_avoids_whole_group_listing() {
    let mut rng = StdRng::seed_from_u64(37);
    let c = random_prime_order_curve(&mut rng, 16).expect("curve");
    let mut evals = Vec::new();
    for k in [0usize, 1, 2, 4] {
        ecdl_tensor_ic::opcount::start();
        let fb = new_windowed_factor_bases(&c, 2, &mut rng, k);
        let tally = ecdl_tensor_ic::opcount::take();
        let n = tally
            .get("setup.fb_membership_evals_uncharged")
            .copied()
            .unwrap_or(0);
        assert!(
            !fb.sets.iter().any(|s| s.is_empty()),
            "empty F_i at k={}",
            k
        );
        if k > 0 {
            assert!(!fb.x_bound.is_empty(), "window dropped at k={}", k);
        }
        evals.push((k, n));
    }
    let full = evals[0].1;
    assert!(full > 0, "whole-group listing evaluated no hashes");
    for pair in evals.windows(2) {
        assert!(
            pair[1].1 < pair[0].1,
            "k={} evaluated {} hashes, k={} evaluated {}: not monotone",
            pair[1].0,
            pair[1].1,
            pair[0].0,
            pair[0].1
        );
    }
    // The window visits p >> k of the p abscissae, so k = 1 only halves the
    // work; k >= 2 must beat the whole group by at least that factor.
    let slack = |n: u64| n * 10;
    for &(k, n) in &evals[1..] {
        assert!(
            slack(n) <= full * if k >= 2 { 4 } else { 11 },
            "k={} evaluated {} hashes, whole group {}: window is not cheaper",
            k,
            n,
            full
        );
    }
}

// The port's default path must be unchanged: k = 0 draws the same factor
// bases as `new_affine_factor_bases`, which now delegates here.
#[test]
fn k_zero_matches_affine_constructor() {
    let mut curve_rng = StdRng::seed_from_u64(40);
    let c = random_prime_order_curve(&mut curve_rng, 8).expect("curve");
    let mut a = StdRng::seed_from_u64(41);
    let mut b = StdRng::seed_from_u64(41);
    let plain = new_affine_factor_bases(&c, 3, &mut b);
    let windowed = new_windowed_factor_bases(&c, 3, &mut a, 0);
    assert!(windowed.x_bound.is_empty());
    assert_eq!(plain.sets, windowed.sets);
    assert_eq!(plain.points, windowed.points);
    assert_eq!(plain.alpha, windowed.alpha);
    assert_eq!(plain.bound, windowed.bound);
}
