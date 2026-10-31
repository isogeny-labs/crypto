//! Rust-only regression tests for the two places where the port deliberately
//! differs from the Go code (PR review findings): duplicate support rows and
//! empty SHA-256 factor bases.

use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::SeedableRng;

use ecdl_tensor_ic::relation::new_factor_bases;
use ecdl_tensor_ic::{
    random_prime_order_curve, BruteForceCounter, Counter, DFSCounter, Network, SparseContractor,
    SupportVertex, Vertex,
};

#[test]
fn duplicate_support_rows_count_once() {
    let v = SupportVertex::new(vec![0], vec![vec![false], vec![false], vec![true]]);
    assert_eq!(v.rows, vec![vec![false], vec![true]]);
    let n = Network {
        num_edges: 1,
        vertices: vec![Vertex::Support(v)],
        ..Default::default()
    };
    let none = HashMap::new();
    assert_eq!(BruteForceCounter.count(&n, &none).unwrap(), 2);
    assert_eq!(SparseContractor::default().count(&n, &none).unwrap(), 2);
    assert_eq!(DFSCounter.count(&n, &none).unwrap(), 2);
}

#[test]
fn sha_factor_bases_are_never_empty() {
    // 6-bit curves with m = 4 give |F_i| of about 3 out of r < 64 points, so
    // an empty F_i is a likely draw without the redraw.
    let mut rng = StdRng::seed_from_u64(5);
    for _ in 0..20 {
        let c = random_prime_order_curve(&mut rng, 6).unwrap();
        for _ in 0..50 {
            let fb = new_factor_bases(&c, 4, &mut rng);
            assert!(fb.sets.iter().all(|s| !s.is_empty()));
        }
    }
}

#[test]
fn opcount_tally_is_deterministic_and_off_by_default() {
    use ecdl_tensor_ic::field::rand_below;
    use ecdl_tensor_ic::opcount;
    use ecdl_tensor_ic::Solver;

    let run = |tally: bool| {
        let mut rng = StdRng::seed_from_u64(11);
        let c = random_prime_order_curve(&mut rng, 9).unwrap();
        let xi = rand_below(&mut rng, c.order);
        let q = c.mul(xi, c.g);
        if tally {
            opcount::start();
        }
        let mut s = Solver::new(c, 2, Box::new(SparseContractor::default()), rng);
        let (got, _) = s.solve(q).unwrap();
        assert_eq!(got, xi);
        opcount::take()
    };
    assert!(run(false).is_empty());
    let a = run(true);
    assert_eq!(a, run(true));
    for key in [
        "factor_base.fb_points_listed_uncharged",
        "relation_target.adds",
        "relation_target.doubles",
        "relation_search.adds",
        "relation_search.contraction_rows_uncharged",
        "linear_algebra.linalg_mulmod_uncharged",
        "recovery_check.doubles",
    ] {
        assert!(a.get(key).copied().unwrap_or(0) > 0, "missing {key}: {a:?}");
    }
}
