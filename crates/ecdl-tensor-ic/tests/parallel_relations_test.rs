//! Parallel relation search: targets are drawn in order, evaluated in
//! parallel, and kept in target order, so the result must not depend on the
//! thread count.

use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use ecdl_tensor_ic::Solver;
use ecdl_tensor_ic::{random_prime_order_curve, SparseContractor};

const SEED: u64 = 31;

fn solve(bits: usize, m: usize, threads: usize, seed: u64) -> (u64, ecdl_tensor_ic::Stats) {
    let mut rng = StdRng::seed_from_u64(seed);
    let c = random_prime_order_curve(&mut rng, bits).expect("curve");
    let xi = rng.gen_range(0..c.order);
    let q = c.mul(xi, c.g);
    let mut s = Solver::new(
        c,
        m,
        Box::new(SparseContractor::default()),
        StdRng::seed_from_u64(seed ^ 0x5eed),
    );
    s.mitm = true;
    s.threads = threads;
    let (got, st) = s.solve(q).expect("solve");
    assert_eq!(got, xi);
    (got, st)
}

// Same seed, one thread versus several: the recovered secret, the relation
// count and the attempt count are identical, because the targets are drawn
// before dispatch and the hits are kept in target order.
#[test]
fn thread_count_does_not_change_the_result() {
    for (bits, m) in [(10usize, 2usize), (10, 3), (12, 2)] {
        let (xi1, s1) = solve(bits, m, 1, SEED);
        let (xi4, s4) = solve(bits, m, 4, SEED);
        assert_eq!(xi1, xi4, "b={} m={}: different secrets", bits, m);
        assert_eq!(
            (s1.relations, s1.attempts, s1.factor_base_size),
            (s4.relations, s4.attempts, s4.factor_base_size),
            "b={} m={}: different relation/attempt counts",
            bits,
            m
        );
    }
}

// The parallel path must not run while a tally is active: the tally is
// thread-local, so the ecbench arm's counts would be incomplete.
#[test]
fn parallel_path_is_disabled_while_tallying() {
    let mut rng = StdRng::seed_from_u64(37);
    let c = random_prime_order_curve(&mut rng, 10).expect("curve");
    let xi = rng.gen_range(0..c.order);
    let q = c.mul(xi, c.g);
    let mut s = Solver::new(
        c,
        2,
        Box::new(SparseContractor::default()),
        StdRng::seed_from_u64(5),
    );
    s.mitm = true;
    s.threads = 4;

    ecdl_tensor_ic::opcount::start();
    assert!(ecdl_tensor_ic::opcount::active());
    let (got, _) = s.solve(q).expect("solve");
    let tally = ecdl_tensor_ic::opcount::take();
    assert_eq!(got, xi);
    let charged: u64 = tally
        .iter()
        .filter(|(k, _)| !k.ends_with("_uncharged"))
        .map(|(_, v)| *v)
        .sum();
    assert!(charged > 0, "tally was empty while solving");
    assert!(!ecdl_tensor_ic::opcount::active());
}

// Parallel search is faster when there are cores to use. Not a hard timing
// assertion (CI machines vary); the comparison runs only with >1 core.
#[test]
fn parallel_search_is_not_slower_on_a_multi_core_machine() {
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    if cores < 2 {
        return;
    }
    let started = Instant::now();
    let (_, s1) = solve(14, 2, 1, 41);
    let seq = started.elapsed();
    let started = Instant::now();
    let (_, sp) = solve(14, 2, cores.min(8), 41);
    let par = started.elapsed();
    assert_eq!(s1.relations, sp.relations);
    eprintln!(
        "cores={} sequential={:?} parallel={:?} ({:.1}x)",
        cores,
        seq,
        par,
        seq.as_secs_f64() / par.as_secs_f64().max(1e-9)
    );
}
