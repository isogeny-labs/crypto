//! The coordinate window on the meet-in-the-middle path: it is a membership
//! constraint on the affine hash, so it works without gadgets, and it replaces
//! whole-group listing with window-only listing.

use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use ecdl_tensor_ic::relation::new_windowed_factor_bases;
use ecdl_tensor_ic::{random_prime_order_curve, Solver, SparseContractor};

// A solve with MITM and a window, no gadgets: the factor bases carry the
// window and the secret is still recovered and verified.
#[test]
fn mitm_solve_with_window_without_gadgets() {
    let mut rng = StdRng::seed_from_u64(301);
    let c = random_prime_order_curve(&mut rng, 12).expect("curve");
    let xi = rng.gen_range(0..c.order);
    let q = c.mul(xi, c.g);
    let mut s = Solver::new(
        c,
        2,
        Box::new(SparseContractor::default()),
        StdRng::seed_from_u64(7),
    );
    s.mitm = true;
    s.window_bits = 4;
    let (got, st) = s.solve(q).expect("solve");
    assert_eq!(got, xi);
    assert!(st.factor_base_size > 0);
}

// The window must reach the constructor from the solver, i.e. the solver's
// factor bases are windowed even when `gadgets` is false.
#[test]
fn solver_window_is_applied_without_gadgets() {
    let mut rng = StdRng::seed_from_u64(303);
    let c = random_prime_order_curve(&mut rng, 11).expect("curve");
    let xi = rng.gen_range(0..c.order);
    let q = c.mul(xi, c.g);
    let mut s = Solver::new(
        c,
        2,
        Box::new(SparseContractor::default()),
        StdRng::seed_from_u64(9),
    );
    s.mitm = true;
    s.window_bits = 3;
    let (got, st) = s.solve(q).expect("solve");
    assert_eq!(got, xi);
    // |F_i| stays near its target, so the window is not degenerate.
    assert!(st.factor_base_size >= 2, "N = {}", st.factor_base_size);
}

// Listing cost: the window visits p >> k abscissae, so at k = 8 it must be far
// cheaper than the whole group. Deterministic (membership hashes via opcount),
// not a wall-clock assertion; the same construction is also timed once and
// printed for the record.
#[test]
fn window_cuts_factor_base_listing_cost() {
    let mut rng = StdRng::seed_from_u64(307);
    let c = random_prime_order_curve(&mut rng, 16).expect("curve");
    let mut evals = Vec::new();
    for k in [0usize, 8] {
        ecdl_tensor_ic::opcount::start();
        let fb = new_windowed_factor_bases(&c, 2, &mut rng, k);
        let tally = ecdl_tensor_ic::opcount::take();
        let n = tally
            .get("setup.fb_membership_evals_uncharged")
            .copied()
            .unwrap_or(0);
        assert!(!fb.sets.iter().any(|s| s.is_empty()));
        evals.push((k, n));
    }
    let (k0, full) = evals[0];
    let (k8, windowed) = evals[1];
    assert!(full > 0 && windowed > 0);
    assert!(
        windowed * 32 <= full,
        "k={} evaluated {} hashes, k={} evaluated {}: expected at least 32x fewer",
        k8,
        windowed,
        k0,
        full
    );

    let started = Instant::now();
    let _ = new_windowed_factor_bases(&c, 2, &mut rng, 0);
    let t0 = started.elapsed();
    let started = Instant::now();
    let _ = new_windowed_factor_bases(&c, 2, &mut rng, 8);
    let t8 = started.elapsed();
    eprintln!("b=16 listing: k=0 {:?}, k=8 {:?}", t0, t8);
}
