//! CT204: complete public toy DLPs using the *implemented* genus-two pullback.
//!
//! This tests full scalar recovery through a Jacobian, rather than a
//! pull/push identity alone. Both solvers use the same BSGS algorithm.
//! Neither solver is index calculus; there is no speedup claim.
//! [pcc:unverified]

use crypto_lib::prime_hyperelliptic::{
    BalancedDivisorP, PrimeEllipticPointP, PrimeQuadraticCoverP,
};
use num_bigint::BigUint;

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

fn point(x: u64, y: u64) -> PrimeEllipticPointP {
    PrimeEllipticPointP::Affine { x: big(x), y: big(y) }
}

#[derive(Debug)]
struct CompleteBsgs {
    log: u64,
    additions: u64,
    baby_entries: usize,
}

/// Deterministic BSGS, no target-specific scalar or table supplied by caller.
/// Counts every group addition used to build both tables and giant steps.
/// Group operations have different real prices in E and Jac(C).
fn solve_bsgs<T, A, N>(
    identity: T,
    generator: &T,
    target: &T,
    order: u64,
    mut add: A,
    mut neg: N,
) -> CompleteBsgs
where
    T: Clone + PartialEq,
    A: FnMut(&T, &T) -> T,
    N: FnMut(&T) -> T,
{
    assert!(order > 1);
    let mut m = 1u64;
    while m * m < order {
        m += 1;
    }
    let mut operations = 0u64;
    let mut baby = Vec::with_capacity(m as usize);
    let mut running = identity.clone();
    for j in 0..m {
        baby.push((running.clone(), j));
        if j + 1 < m {
            running = add(&running, generator);
            operations += 1;
        }
    }

    let mut step = identity;
    for _ in 0..m {
        step = add(&step, generator);
        operations += 1;
    }
    let neg_step = neg(&step);
    let mut giant = target.clone();
    for i in 0..=m {
        for (entry, j) in &baby {
            if *entry == giant {
                let k = i * m + *j;
                if k < order {
                    return CompleteBsgs {
                        log: k,
                        additions: operations,
                        baby_entries: baby.len(),
                    };
                }
            }
        }
        if i < m {
            giant = add(&giant, &neg_step);
            operations += 1;
        }
    }
    panic!("BSGS did not recover a scalar in the declared subgroup");
}

#[test]
fn all_43_public_logs_recovered_through_real_balanced_jacobian() {
    // Independently enumerated in a separate integer-field oracle:
    // #E(F_31)=43, E:y^2=x^3+3, (1,2) has exact order 43.
    // No source-group order assertion is used as an oracle for the log.
    let cover = PrimeQuadraticCoverP::try_new(big(31), big(0), big(3), big(0))
        .expect("genuine nonsingular prime-field quadratic cover");
    let source = cover.source_curve();
    let g = point(1, 2);
    assert_eq!(
        cover.checked_target_scalar_mul(&g, &big(43)).unwrap(),
        PrimeEllipticPointP::Infinity
    );
    for j in 1..43u64 {
        assert_ne!(
            cover.checked_target_scalar_mul(&g, &big(j)).unwrap(),
            PrimeEllipticPointP::Infinity
        );
    }
    let mut number_of_rational_points = 1usize; // infinity
    for x in 0..31 {
        for y in 0..31 {
            if cover.validate_target_point(&point(x, y)).is_ok() {
                number_of_rational_points += 1;
            }
        }
    }
    assert_eq!(number_of_rational_points, 43);

    let mut direct_adds = 0u64;
    let mut jacobian_adds = 0u64;

    for secret_for_fixture_only in 0..43u64 {
        let q = cover
            .checked_target_scalar_mul(&g, &big(secret_for_fixture_only))
            .unwrap();

        // The solvers receive only public (G,Q,r), not the fixture scalar.
        let direct = solve_bsgs(
            PrimeEllipticPointP::Infinity,
            &g,
            &q,
            43,
            |left, right| cover.checked_target_add(left, right).unwrap(),
            |value| cover.checked_target_neg(value).unwrap(),
        );
        // Recreate even the generator image for each public target rather
        // than borrowing it from an earlier target's solve.
        let image_g = cover.pullback_point_class(&g).unwrap();
        assert!(!image_g.is_identity());
        assert!(image_g.checked_scalar_mul(&big(43), source).unwrap().is_identity());
        let image_q = cover.pullback_point_class(&q).unwrap();
        let transferred = solve_bsgs(
            BalancedDivisorP::try_identity(source).unwrap(),
            &image_g,
            &image_q,
            43,
            |left, right| left.checked_add(right, source).unwrap(),
            |value| value.checked_neg(source).unwrap(),
        );
        assert_eq!(direct.log, secret_for_fixture_only);
        assert_eq!(transferred.log, secret_for_fixture_only);
        assert_eq!(direct.log, transferred.log);
        assert_eq!(direct.baby_entries, transferred.baby_entries);
        // Check the actual Jacobian solution, not only the recovered scalar.
        let reconstructed_image = image_g
            .checked_scalar_mul(&big(transferred.log), source)
            .unwrap();
        assert!(reconstructed_image.checked_eq(&image_q, source).unwrap());

        // Physical norm and recovery are part of the transfer path.
        let norm_target = cover.pushforward_arbitrary_balanced_class(&image_q).unwrap();
        let expected_norm = cover.checked_target_add(&q, &q).unwrap();
        assert_eq!(norm_target, expected_norm);
        let reconstructed_target = cover
            .checked_target_scalar_mul(&g, &big(transferred.log))
            .unwrap();
        assert_eq!(reconstructed_target, q);

        direct_adds += direct.additions;
        jacobian_adds += transferred.additions;
    }
    // Group-operation COUNTS, not comparable primitive or wall-clock costs.
    // Same BSGS roster and order means equal addition counts in both groups.
    assert_eq!(direct_adds, jacobian_adds);
    assert_eq!(direct_adds, 670);
    eprintln!(
        "CT204: 43/43 actual toy logarithms; direct E additions={direct_adds}; \
         Jacobian additions={jacobian_adds}; 43 pullbacks and norms \
         plus 43 generator pullbacks; no complete-cost speedup inferred \
         [pcc:unverified]"
    );
}

#[test]
fn wrong_scalar_is_rejected_on_both_groups() {
    let cover = PrimeQuadraticCoverP::try_new(big(31), big(0), big(3), big(0)).unwrap();
    let source = cover.source_curve();
    let g = point(1, 2);
    let q = cover.checked_target_scalar_mul(&g, &big(17)).unwrap();
    let image_g = cover.pullback_point_class(&g).unwrap();
    let image_q = cover.pullback_point_class(&q).unwrap();

    let false_k = big(18);
    assert_ne!(cover.checked_target_scalar_mul(&g, &false_k).unwrap(), q);
    assert!(!image_g
        .checked_scalar_mul(&false_k, source)
        .unwrap()
        .checked_eq(&image_q, source)
        .unwrap());
}
