#![cfg(feature = "experimental-subnlogn")]

//! Independent boundary and integration checks for the opt-in components.
//! These are correctness checks, not timing or asymptotic evidence.

use isogeny_algos::bigint::Big;
use isogeny_algos::field::{Field, Prepared, Rng, Zp};
use isogeny_algos::fpm::FpM;
use isogeny_algos::gf2n::GF2n;
use isogeny_algos::poly;
use isogeny_algos::subnlogn::{
    multiply_exact, Algorithm, Config, Error, ExperimentalField, MAX_INPUT_LIMBS,
};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Word-based reference, independent of the candidate's byte transforms.
/// Each inner sum is at most (2^64-1)^2 + 2(2^64-1) = 2^128-1.
fn word_product(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0; a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        let mut carry = 0u128;
        for (j, &y) in b.iter().enumerate() {
            let value = x as u128 * y as u128 + out[i + j] as u128 + carry;
            out[i + j] = value as u64;
            carry = value >> 64;
        }
        out[i + b.len()] = carry as u64;
    }
    while out.last() == Some(&0) {
        out.pop();
    }
    out
}

fn direct_convolution<F: Field>(f: &F, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut out = vec![f.zero(); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = f.add(out[i + j], f.mul(x, y));
        }
    }
    out
}

#[test]
fn integer_reference_has_independent_known_answers() {
    assert_eq!(word_product(&[u64::MAX], &[u64::MAX]), [1, u64::MAX - 1]);
    assert_eq!(
        word_product(&[u64::MAX; 2], &[u64::MAX; 2]),
        [1, 0, u64::MAX - 1, u64::MAX]
    );
    assert_eq!(word_product(&[0, 1], &[0, 0, 1]), [0, 0, 0, 1]);
    assert_eq!(word_product(&[], &[1]), []);
    assert_eq!(word_product(&[1], &[]), []);
}

#[test]
fn exact_products_match_word_reference_at_carry_and_limb_boundaries() {
    let mut rng = Rng::new(0x5355_424e_4c4f_474e);
    for na in 1..=4 {
        for nb in 1..=4 {
            let fixtures = [
                (vec![u64::MAX; na], vec![u64::MAX; nb]),
                (
                    vec![0xaaaa_aaaa_aaaa_aaaa; na],
                    vec![0x5555_5555_5555_5555; nb],
                ),
                (
                    (0..na).map(|_| rng.next()).collect(),
                    (0..nb).map(|_| rng.next()).collect(),
                ),
            ];
            for (a, b) in fixtures {
                let product = multiply_exact(&a, &b, Algorithm::SyntheticReference, 64).unwrap();
                assert_eq!(product.limbs, word_product(&a, &b), "lengths {na}, {nb}");
                assert_eq!(product.backend, Algorithm::SyntheticReference);
                assert!(product.transform_points.is_power_of_two());
                assert!(product.transform_points <= 64);
                assert_ne!(product.limbs.last(), Some(&0));
            }
        }
    }
}

#[test]
fn exact_integer_zero_padding_and_endianness_are_explicit() {
    for (a, b) in [
        (vec![], vec![]),
        (vec![], vec![u64::MAX]),
        (vec![u64::MAX], vec![]),
        (vec![0, 0, 0], vec![1, 2]),
    ] {
        let product = multiply_exact(&a, &b, Algorithm::SyntheticReference, 64).unwrap();
        assert!(product.limbs.is_empty());
        assert_eq!(product.transform_points, 0);
    }
    for (a, b) in [
        (vec![1, 0, 0], vec![7, 0]),
        (vec![0, 1], vec![0, 0, 1]),
        (vec![1 << 56], vec![1 << 8]),
        (vec![0xff00_ff00_ff00_ff00], vec![0x00ff_00ff_00ff_00ff]),
    ] {
        let product = multiply_exact(&a, &b, Algorithm::SyntheticReference, 64).unwrap();
        assert_eq!(product.limbs, word_product(&a, &b));
    }
}

#[test]
fn strict_integer_modes_and_resource_limits_do_not_silently_fall_back() {
    for (a, b) in [(&[][..], &[][..]), (&[1][..], &[1][..])] {
        assert!(matches!(
            multiply_exact(a, b, Algorithm::PublishedSubNlogN, 64),
            Err(Error::FullAlgorithmUnavailable)
        ));
    }
    let too_long = vec![0; MAX_INPUT_LIMBS + 1];
    // The input cap applies before normalization and also when the product is zero.
    for (a, b) in [(&too_long[..], &[1][..]), (&[][..], &too_long[..])] {
        assert!(matches!(
            multiply_exact(a, b, Algorithm::SyntheticReference, 64),
            Err(Error::InputLimit { .. })
        ));
    }
    assert!(multiply_exact(&[u64::MAX], &[u64::MAX], Algorithm::SyntheticReference, 16).is_ok());
    assert!(multiply_exact(&[u64::MAX], &[u64::MAX], Algorithm::SyntheticReference, 15).is_err());
}

#[test]
fn existing_polynomial_calls_reach_candidate_and_bounded_fallback() {
    let base = Zp::new((1 << 31) - 1);
    let a: Vec<_> = (0..33).map(|i| (17 * i + 3) as u64).collect();
    let b: Vec<_> = (0..35).map(|i| (i * i + 7) as u64).collect();
    let expected = direct_convolution(&base, &a, &b);
    let candidate = ExperimentalField::new(
        base,
        Config {
            max_points: 128,
            min_len: 32,
        },
    );
    assert_eq!(poly::mul_raw(&candidate, &a, &b), expected);
    assert_eq!(candidate.stats().attempts, 1);
    assert_eq!(candidate.stats().synthetic, 1);
    assert_eq!(candidate.stats().fallback, 0);

    let capped = ExperimentalField::new(
        base,
        Config {
            max_points: 32,
            min_len: 32,
        },
    );
    assert_eq!(poly::mul_raw(&capped, &a, &b), expected);
    assert!(capped.stats().attempts > 0);
    assert_eq!(capped.stats().synthetic, 0);
    assert!(capped.stats().fallback > 0);
    assert!(capped.try_convolve(&a, &b).is_err());
}

#[test]
fn square_route_and_shared_counters_are_exercised() {
    let f = ExperimentalField::new(
        Zp::new(101),
        Config {
            max_points: 128,
            min_len: 32,
        },
    );
    let a: Vec<_> = (0..33).map(|i| ((3 * i + 1) % 101) as u64).collect();
    let copy = f.clone();
    assert_eq!(
        poly::mul_raw(&copy, &a, &a),
        direct_convolution(f.inner(), &a, &a)
    );
    assert_eq!(f.stats().synthetic, 1);
    assert_eq!(copy.stats(), f.stats());
}

#[test]
fn binary_field_rejection_does_not_disable_existing_polynomial_arithmetic() {
    let base = GF2n::new(8);
    let a: Vec<_> = (0..33).map(|i| (7 * i) as u64 & 255).collect();
    let b: Vec<_> = (0..35).map(|i| (11 * i + 1) as u64 & 255).collect();
    let expected = direct_convolution(&base, &a, &b);
    let wrapped = ExperimentalField::new(base, Config::default());
    assert!(wrapped.try_convolve(&a, &b).is_err());
    assert_eq!(poly::mul_raw(&wrapped, &a, &b), expected);
    assert_eq!(wrapped.stats().synthetic, 0);
    assert!(wrapped.stats().fallback > 0);
}

#[test]
fn p224_low_characteristic_limb_is_not_mistaken_for_characteristic_one() {
    let p = Big::from_u64(1)
        .shl(224)
        .sub(&Big::from_u64(1).shl(96))
        .add_small(1);
    let base = FpM::<4>::new(&p);
    assert_eq!(base.char(), 1);
    let a: Vec<_> = (0..9).map(|i| base.from_u64(3 * i + 1)).collect();
    let b: Vec<_> = (0..11)
        .map(|i| base.neg(base.from_u64(i * i + 1)))
        .collect();
    let expected = direct_convolution(&base, &a, &b);
    let wrapped = ExperimentalField::new(
        base,
        Config {
            max_points: 32,
            min_len: 1,
        },
    );
    assert!(wrapped.characteristic_exceeds(u64::MAX));
    assert_eq!(wrapped.q(), p);
    assert_eq!(wrapped.try_convolve(&a, &b).unwrap(), expected);
}

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum Hook {
    Sq,
    Pow,
    PowBig,
    ConvSq,
    Trunc,
    Prepare,
    Prepared,
    Cyclic,
    Rows,
    Batch,
    Long,
    Sqrt,
}
const HOOKS: usize = 12;

#[derive(Clone)]
struct OptimizedHooks {
    base: Zp,
    calls: Arc<[AtomicUsize; HOOKS]>,
}
impl OptimizedHooks {
    fn new() -> Self {
        Self {
            base: Zp::new(101),
            calls: Arc::new(std::array::from_fn(|_| AtomicUsize::new(0))),
        }
    }
    fn hit(&self, hook: Hook) {
        self.calls[hook as usize].fetch_add(1, Ordering::Relaxed);
    }
    fn assert_called_once(&self, hook: Hook) {
        assert_eq!(
            self.calls[hook as usize].load(Ordering::Relaxed),
            1,
            "lost optimized hook {hook:?}"
        );
    }
}

impl Field for OptimizedHooks {
    type E = u64;
    fn zero(&self) -> u64 {
        self.base.zero()
    }
    fn one(&self) -> u64 {
        self.base.one()
    }
    fn add(&self, a: u64, b: u64) -> u64 {
        self.base.add(a, b)
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        self.base.sub(a, b)
    }
    fn neg(&self, a: u64) -> u64 {
        self.base.neg(a)
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.base.mul(a, b)
    }
    fn inv(&self, a: u64) -> u64 {
        self.base.inv(a)
    }
    fn from_u64(&self, n: u64) -> u64 {
        self.base.from_u64(n)
    }
    fn char(&self) -> u64 {
        self.base.char()
    }
    fn size(&self) -> u128 {
        self.base.size()
    }
    fn random(&self, rng: &mut Rng) -> u64 {
        self.base.random(rng)
    }
    fn karatsuba_threshold(&self) -> usize {
        17
    }
    fn sqr_threshold(&self) -> usize {
        19
    }
    fn powmod_table_max(&self) -> usize {
        23
    }
    fn sq(&self, a: u64) -> u64 {
        self.hit(Hook::Sq);
        self.base.sq(a)
    }
    fn pow(&self, a: u64, e: u128) -> u64 {
        self.hit(Hook::Pow);
        self.base.pow(a, e)
    }
    fn pow_big(&self, a: u64, e: &Big) -> u64 {
        self.hit(Hook::PowBig);
        self.base.pow_big(a, e)
    }
    fn sqrt(&self, a: u64) -> Option<u64> {
        self.hit(Hook::Sqrt);
        self.base.sqrt(a)
    }
    fn conv_sq(&self, a: &[u64]) -> Vec<u64> {
        self.hit(Hook::ConvSq);
        self.base.conv_sq(a)
    }
    fn conv_trunc(&self, a: &[u64], b: &[u64], n: usize) -> Vec<u64> {
        self.hit(Hook::Trunc);
        self.base.conv_trunc(a, b, n)
    }
    fn prepare(&self, b: &[u64], max_len: usize, cyc: usize) -> Option<Prepared> {
        self.hit(Hook::Prepare);
        Some(Rc::new((b.to_vec(), max_len, cyc)))
    }
    fn conv_prepared(&self, a: &[u64], b: &Prepared) -> Option<Vec<u64>> {
        self.hit(Hook::Prepared);
        let (b, max_len, cyc) = b.downcast_ref::<(Vec<u64>, usize, usize)>()?;
        if a.len() > *max_len || *cyc != usize::MAX {
            return None;
        }
        Some(direct_convolution(&self.base, a, b))
    }
    fn conv_cyclic(&self, a: &[u64], b: &[u64], n: usize) -> Option<Vec<u64>> {
        self.hit(Hook::Cyclic);
        if n == 0 {
            return None;
        }
        let mut out = vec![0; n];
        for (i, c) in direct_convolution(&self.base, a, b).into_iter().enumerate() {
            out[i % n] = self.base.add(out[i % n], c);
        }
        Some(out)
    }
    fn dot_rev_rows(&self, a: &[u64], rows: &[&[u64]]) -> Vec<u64> {
        self.hit(Hook::Rows);
        self.base.dot_rev_rows(a, rows)
    }
    fn batch_inv(&self, xs: &[u64]) -> Vec<u64> {
        self.hit(Hook::Batch);
        self.base.batch_inv(xs)
    }
    fn conv_long(&self, a: &[u64], b: &[u64]) -> Option<Vec<u64>> {
        self.hit(Hook::Long);
        Some(direct_convolution(&self.base, a, b))
    }
}

#[test]
fn wrapper_retains_optimized_hooks_and_opaque_prepared_operands() {
    let base = OptimizedHooks::new();
    let f = ExperimentalField::new(
        base.clone(),
        Config {
            max_points: 32,
            min_len: usize::MAX,
        },
    );
    let (a, b) = ([3, 5, 7], [11, 13, 17]);
    let expected = direct_convolution(&base.base, &a, &b);
    assert_eq!(f.karatsuba_threshold(), 17);
    assert_eq!(f.sqr_threshold(), 19);
    assert_eq!(f.powmod_table_max(), 23);
    assert_eq!(f.sq(7), 49);
    assert_eq!(f.pow(7, 13), base.base.pow(7, 13));
    assert_eq!(f.pow_big(7, &Big::from_u64(13)), base.base.pow(7, 13));
    assert_eq!(f.sqrt(4), base.base.sqrt(4));
    assert_eq!(f.conv_sq(&a), direct_convolution(&base.base, &a, &a));
    assert_eq!(f.conv_trunc(&a, &b, 4), expected[..4]);
    let prepared = f.prepare(&b, 3, usize::MAX).unwrap();
    assert_eq!(f.conv_prepared(&a, &prepared).unwrap(), expected);
    let mut cyclic = vec![0; 4];
    for (i, &c) in expected.iter().enumerate() {
        cyclic[i % 4] = base.base.add(cyclic[i % 4], c);
    }
    assert_eq!(f.conv_cyclic(&a, &b, 4).unwrap(), cyclic);
    assert_eq!(
        f.dot_rev_rows(&a, &[&a, &b]),
        base.base.dot_rev_rows(&a, &[&a, &b])
    );
    assert_eq!(f.batch_inv(&a), base.base.batch_inv(&a));
    assert_eq!(f.conv_long(&a, &b).unwrap(), expected);
    for hook in [
        Hook::Sq,
        Hook::Pow,
        Hook::PowBig,
        Hook::ConvSq,
        Hook::Trunc,
        Hook::Prepare,
        Hook::Prepared,
        Hook::Cyclic,
        Hook::Rows,
        Hook::Batch,
        Hook::Long,
        Hook::Sqrt,
    ] {
        base.assert_called_once(hook);
    }
    assert_eq!(f.stats().attempts, 0);
    assert_eq!(f.stats().fallback, 1);
}
