//! Tests of `cryptanalysis::cheon_counted` (the counted Cheon `n - 1` and
//! `n + 1` algorithms and the Mestre point counter of
//! `research/cheon_p224_shape_20261022/`).  Integration tests, so they link
//! the release library only.

use crypto_lib::cryptanalysis::challenge_ladder::is_prime_u64;
use crypto_lib::cryptanalysis::cheon_counted::{
    cheon_pminus1, cheon_pplus1, find_shaped_curve, leaked_powers, next_prime, prime_order_bsgs,
    scalar_mul_plain, shape_divisor, Shaped, Side,
};
use crypto_lib::cryptanalysis::ic_boundary::{GroupOps, PrimeCurve, PrimePoint};

fn powm(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u128;
    let mut b = a as u128 % m as u128;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % m as u128;
        }
        b = b * b % m as u128;
        e >>= 1;
    }
    a = r as u64;
    a
}

fn small(side: Side, ratio: f64) -> (PrimeCurve, Shaped) {
    let p = next_prime(3 << 17);
    let s = find_shaped_curve(p, 77, side, ratio, 0.06);
    (
        PrimeCurve {
            p: s.p,
            a: s.a,
            b: s.b,
        },
        s,
    )
}

#[test]
fn bsgs_count_matches_brute_force() {
    let c = PrimeCurve {
        p: 100003,
        a: 5,
        b: 7,
    };
    let brute = c.point_count();
    if is_prime_u64(brute) {
        assert_eq!(prime_order_bsgs(&c), Some(brute));
    } else {
        assert_eq!(prime_order_bsgs(&c), None);
    }
    let (c, s) = small(Side::Minus, 0.2);
    assert_eq!(c.point_count(), s.n);
    assert!(is_prime_u64(s.n));
    assert!(shape_divisor(s.n, Side::Minus, 0.2, 0.06).is_some());
}

#[test]
fn pminus1_recovers_alpha_comb_and_naive() {
    let (c, s) = small(Side::Minus, 0.2);
    let g0 = PrimePoint::affine(s.gx, s.gy);
    let mut ops = GroupOps::default();
    for alpha in [2u64, 3, 12345, s.n - 2, s.n / 2 + 1] {
        let ad = powm(alpha, s.d, s.n);
        let ag = scalar_mul_plain(&c, &mut ops, alpha, g0);
        let adg = scalar_mul_plain(&c, &mut ops, ad, g0);
        for comb in [true, false] {
            let o = cheon_pminus1(&c, s.n, s.d, g0, ag, adg, comb, None);
            assert_eq!(o.alpha, Some(alpha), "comb={comb} {:?}", o.error);
            assert_eq!(o.ops, o.phases.values().sum::<u64>());
            assert_eq!(o.ops, o.adds + o.doubles);
        }
    }
}

/// Negative control: the auxiliary input is used, so a wrong `[alpha^d]G`
/// does not return the planted logarithm.
#[test]
fn pminus1_needs_the_auxiliary_input() {
    let (c, s) = small(Side::Minus, 0.2);
    let g0 = PrimePoint::affine(s.gx, s.gy);
    let mut ops = GroupOps::default();
    let alpha = 777u64;
    let ag = scalar_mul_plain(&c, &mut ops, alpha, g0);
    let bogus = scalar_mul_plain(&c, &mut ops, 5, g0);
    let o = cheon_pminus1(&c, s.n, s.d, g0, ag, bogus, true, None);
    assert_ne!(o.alpha, Some(alpha));
}

#[test]
fn pminus1_rejects_a_non_divisor() {
    let (c, s) = small(Side::Minus, 0.2);
    let g0 = PrimePoint::affine(s.gx, s.gy);
    let o = cheon_pminus1(&c, s.n, s.n, g0, g0, g0, true, None);
    assert!(o.alpha.is_none() && o.error.is_some());
}

#[test]
fn pplus1_recovers_alpha() {
    let p = next_prime(3 << 15);
    let s = find_shaped_curve(p, 91, Side::Plus, 0.2, 0.08);
    let c = PrimeCurve {
        p: s.p,
        a: s.a,
        b: s.b,
    };
    let g0 = PrimePoint::affine(s.gx, s.gy);
    for alpha in [2u64, 99, 31337, s.n - 3] {
        let pows = leaked_powers(&c, s.n, g0, alpha, 2 * s.d);
        let o = cheon_pplus1(&c, s.n, s.d, &pows, None);
        assert_eq!(o.alpha, Some(alpha), "{:?}", o.error);
    }
}

/// Negative control for the `n + 1` variant: powers of a different secret in
/// the high slots do not return the planted logarithm.
#[test]
fn pplus1_wrong_powers_do_not_recover() {
    let p = next_prime(3 << 15);
    let s = find_shaped_curve(p, 91, Side::Plus, 0.2, 0.08);
    let c = PrimeCurve {
        p: s.p,
        a: s.a,
        b: s.b,
    };
    let g0 = PrimePoint::affine(s.gx, s.gy);
    let mut pows = leaked_powers(&c, s.n, g0, 4242, 2 * s.d);
    let other = leaked_powers(&c, s.n, g0, 4243, 2 * s.d);
    pows[2..].copy_from_slice(&other[2..]);
    let o = cheon_pplus1(&c, s.n, s.d, &pows, None);
    assert_ne!(o.alpha, Some(4242));
}
