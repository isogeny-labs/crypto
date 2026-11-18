//! CT213: paired same-runner single-target *toy* cold-state cost probe.
//!
//! Every arm starts with fresh BSGS tables and independently verifies its
//! recovered logarithm. Jacobian arm also pays cover creation, pullbacks,
//! norm, and elliptic recovery. Input target construction is outside timing.
//! This is F_31 / order 43 only, NOT an IC/rho frontier, blinded ECDLP
//! attack, isolated hardware benchmark, memory-complete cost, or PCC result.
//! [pcc:unverified]
use crypto_lib::prime_hyperelliptic::{
    BalancedDivisorP, PrimeEllipticPointP, PrimeQuadraticCoverP,
};
use num_bigint::BigUint;
use std::time::Instant;

fn big(n: u64) -> BigUint { BigUint::from(n) }
fn point(x: u64, y: u64) -> PrimeEllipticPointP {
    PrimeEllipticPointP::Affine { x: big(x), y: big(y) }
}

struct Bsgs { log: u64, additions: u64, baby_entries: usize }
fn bsgs<T, A, N>(
    identity: T, generator: &T, target: &T, order: u64,
    mut add: A, mut neg: N,
) -> Bsgs
where
    T: Clone + PartialEq,
    A: FnMut(&T, &T) -> T,
    N: FnMut(&T) -> T,
{
    assert!(order > 1);
    let mut m = 1u64;
    while m * m < order { m += 1; }
    let mut baby = Vec::with_capacity(m as usize);
    let mut running = identity.clone();
    let mut additions = 0u64;
    for j in 0..m {
        baby.push((running.clone(), j));
        if j + 1 < m {
            running = add(&running, generator);
            additions += 1;
        }
    }
    let mut step = identity;
    for _ in 0..m {
        step = add(&step, generator);
        additions += 1;
    }
    let minus_step = neg(&step);
    let mut giant = target.clone();
    for i in 0..=m {
        for (entry, j) in &baby {
            if *entry == giant {
                let k = i * m + *j;
                if k < order { return Bsgs { log: k, additions, baby_entries: baby.len() }; }
            }
        }
        if i < m {
            giant = add(&giant, &minus_step);
            additions += 1;
        }
    }
    panic!("declared-order BSGS failed");
}

struct Observation { elapsed_ns: u128, additions: u64, baby_entries: usize }

fn direct(
    curve: &PrimeQuadraticCoverP, g: &PrimeEllipticPointP, q: &PrimeEllipticPointP,
) -> Observation {
    // Public curve representation is shared by both arms; no target-specific
    // precomputation is shared. The direct solver does not need a genus-2 cover.
    let start = Instant::now();
    let answer = bsgs(PrimeEllipticPointP::Infinity, g, q, 43,
        |a, b| curve.checked_target_add(a, b).unwrap(),
        |a| curve.checked_target_neg(a).unwrap());
    assert_eq!(curve.checked_target_scalar_mul(g, &big(answer.log)).unwrap(), *q);
    Observation { elapsed_ns: start.elapsed().as_nanos(),
                  additions: answer.additions, baby_entries: answer.baby_entries }
}

fn transferred(g: &PrimeEllipticPointP, q: &PrimeEllipticPointP) -> Observation {
    let start = Instant::now();
    // Fresh setup, rather than an amortized/preloaded Jacobian group.
    let cover = PrimeQuadraticCoverP::try_new(big(31), big(0), big(3), big(0)).unwrap();
    let source = cover.source_curve();
    let image_g = cover.pullback_point_class(g).unwrap();
    let image_q = cover.pullback_point_class(q).unwrap();
    assert!(image_g.checked_scalar_mul(&big(43), source).unwrap().is_identity());
    let answer = bsgs(BalancedDivisorP::try_identity(source).unwrap(),
        &image_g, &image_q, 43,
        |a, b| a.checked_add(b, source).unwrap(),
        |a| a.checked_neg(source).unwrap());
    assert!(image_g.checked_scalar_mul(&big(answer.log), source)
        .unwrap().checked_eq(&image_q, source).unwrap());
    let norm = cover.pushforward_arbitrary_balanced_class(&image_q).unwrap();
    assert_eq!(norm, cover.checked_target_add(q, q).unwrap());
    assert_eq!(cover.checked_target_scalar_mul(g, &big(answer.log)).unwrap(), *q);
    Observation { elapsed_ns: start.elapsed().as_nanos(),
                  additions: answer.additions, baby_entries: answer.baby_entries }
}

fn median(mut values: Vec<u128>) -> u128 {
    values.sort_unstable();
    values[values.len()/2]
}

#[test]
fn paired_43_distinct_public_targets_no_complete_cost_claim() {
    const ROUNDS: usize = 7;
    let fixture = PrimeQuadraticCoverP::try_new(big(31), big(0), big(3), big(0)).unwrap();
    let g = point(1, 2);
    // Producing public Q is not part of either DLP algorithm. This known-answer
    // corpus is *not* blinded; Q itself (not k) is handed to the timed solvers.
    let targets: Vec<_> = (0..43u64).map(|k| {
        (k, fixture.checked_target_scalar_mul(&g, &big(k)).unwrap())
    }).collect();
    for i in 0..targets.len() {
        for j in (i+1)..targets.len() {
            assert_ne!(targets[i].1, targets[j].1, "targets not distinct");
        }
    }
    let mut e_times = Vec::with_capacity(ROUNDS * 43);
    let mut jac_times = Vec::with_capacity(ROUNDS * 43);
    let mut e_add = 0u64;
    let mut jac_add = 0u64;
    for round in 0..ROUNDS {
        // All 43 targets are covered once per round. Alternate AB/BA to
        // reduce systematic thermal and cache order effects, not eliminate them.
        for offset in 0..43 {
            let i = (offset + round * 17) % 43;
            let (known_answer_for_validation, q) = &targets[i];
            let (e, j) = if (i + round) % 2 == 0 {
                (direct(&fixture, &g, q), transferred(&g, q))
            } else {
                let j = transferred(&g, q);
                (direct(&fixture, &g, q), j)
            };
            // The implementations already verify [log]G == Q independently;
            // this assertion checks the public known-answer oracle as well.
            let public_check = fixture.checked_target_scalar_mul(&g, &big(*known_answer_for_validation)).unwrap();
            assert_eq!(&public_check, q);
            assert_eq!(e.baby_entries, j.baby_entries);
            // One raw paired record per public target, before aggregate statistics.
            // Timing is diagnostic only; target k is a public known-answer fixture.
            let order = if (i + round) % 2 == 0 { "EJ" } else { "JE" };
            eprintln!(
                "CT214_PAIR_JSON {{\"round\":{},\"offset\":{},\"target_index\":{},\"arm_order\":\"{}\",\"direct_ns\":{},\"jacobian_ns\":{},\"direct_additions\":{},\"jacobian_additions\":{},\"direct_baby_entries\":{},\"jacobian_baby_entries\":{}}}",
                round, offset, i, order, e.elapsed_ns, j.elapsed_ns,
                e.additions, j.additions, e.baby_entries, j.baby_entries
            );
            e_add += e.additions;
            jac_add += j.additions;
            e_times.push(e.elapsed_ns);
            jac_times.push(j.elapsed_ns);
        }
    }
    assert_eq!(e_add, jac_add);
    assert_eq!(e_add, 670 * ROUNDS as u64);
    let e_total: u128 = e_times.iter().sum();
    let jac_total: u128 = jac_times.iter().sum();
    assert!(e_total > 0 && jac_total > 0);
    let e_median = median(e_times);
    let j_median = median(jac_times);
    // JSON record without a claimed speed advantage. Process/runner are not
    // isolated, no independent PCC, no measured memory; ratio is diagnostic.
    eprintln!(
        "CT213_TOY_COST_JSON {{\"field\":31,\"order\":43,\"rounds\":{},\"pairs\":{},\"direct_total_ns\":{},\"jacobian_total_ns\":{},\"direct_median_ns\":{},\"jacobian_median_ns\":{},\"direct_bsgs_additions\":{},\"jacobian_bsgs_additions\":{},\"ratio_direct_over_jacobian\":{:.9},\"memory\":\"unmeasured\",\"pcc\":\"unverified\",\"status\":\"toy_only_not_full_reference\"}}",
        ROUNDS, ROUNDS * 43, e_total, jac_total, e_median, j_median, e_add, jac_add,
        (e_total as f64)/(jac_total as f64)
    );
}
