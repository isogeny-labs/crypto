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
