//! Exact scalar arithmetic and sampling for the shared counted rho walk.
//! The BigUint implementation is a correctness backend; operation counts are
//! API-level counts and do not price allocations or internal word arithmetic.
use super::exact_scalar::ExactScalar;
use crate::cryptanalysis::koblitz_strong_rho::RhoScalar;
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::cell::Cell;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScalarWork {
    pub modular_adds: u64,
    pub modular_subs: u64,
    pub modular_muls: u64,
    pub modular_inverses: u64,
    pub wide_sample_draws: u64,
}
thread_local! { static WORK: Cell<ScalarWork> = Cell::new(ScalarWork::default()); }
fn count(update: impl FnOnce(&mut ScalarWork)) {
    WORK.with(|cell| {
        let mut work = cell.get();
        update(&mut work);
        cell.set(work);
    });
}
pub fn take_work() -> ScalarWork {
    WORK.with(|cell| cell.replace(ScalarWork::default()))
}
fn exact(value: BigUint) -> ExactScalar {
    ExactScalar::from_biguint(&value).expect("reduced scalar fits the declared 256-bit modulus")
}

#[derive(Clone, Debug)]
pub struct UniformExactScalar(UniformKind);
#[derive(Clone, Debug)]
enum UniformKind {
    Word(Uniform<u64>),
    DoubleWord(Uniform<u128>),
    Wide {
        low: BigUint,
        span: BigUint,
        mask: BigUint,
    },
}
impl UniformExactScalar {
    fn bounds(low: ExactScalar, high: ExactScalar, inclusive: bool) -> Self {
        assert!(
            if inclusive { low <= high } else { low < high },
            "empty scalar range"
        );
        if let (Some(low), Some(high)) = (low.checked_u64(), high.checked_u64()) {
            return Self(UniformKind::Word(if inclusive {
                Uniform::new_inclusive(low, high)
            } else {
                Uniform::new(low, high)
            }));
        }
        if let (Some(low), Some(high)) = (low.checked_u128(), high.checked_u128()) {
            return Self(UniformKind::DoubleWord(if inclusive {
                Uniform::new_inclusive(low, high)
            } else {
                Uniform::new(low, high)
            }));
        }
        let low = low.to_biguint();
        let span = high.to_biguint() - &low + u8::from(inclusive);
        let bits = (&span - 1u8).bits();
        let mask = (BigUint::from(1u8) << bits as usize) - 1u8;
        Self(UniformKind::Wide { low, span, mask })
    }
}
impl SampleUniform for ExactScalar {
    type Sampler = UniformExactScalar;
}
impl UniformSampler for UniformExactScalar {
    type X = ExactScalar;
    fn new<B1: SampleBorrow<ExactScalar> + Sized, B2: SampleBorrow<ExactScalar> + Sized>(
        low: B1,
        high: B2,
    ) -> Self {
        Self::bounds(*low.borrow(), *high.borrow(), false)
    }
    fn new_inclusive<
        B1: SampleBorrow<ExactScalar> + Sized,
        B2: SampleBorrow<ExactScalar> + Sized,
    >(
        low: B1,
        high: B2,
    ) -> Self {
        Self::bounds(*low.borrow(), *high.borrow(), true)
    }
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExactScalar {
        match &self.0 {
            UniformKind::Word(dist) => dist.sample(rng).into(),
            UniformKind::DoubleWord(dist) => dist.sample(rng).into(),
            UniformKind::Wide { low, span, mask } => loop {
                let mut bytes = [0u8; 32];
                rng.fill_bytes(&mut bytes);
                count(|w| w.wide_sample_draws += 1);
                let draw = BigUint::from_bytes_le(&bytes) & mask;
                if &draw < span {
                    return exact(low + draw);
                }
            },
        }
    }
    // rand documents that sample_single may differ from a cached Uniform.
    // Delegate both narrow forms exactly to preserve legacy seeded draws.
    fn sample_single<
        R: Rng + ?Sized,
        B1: SampleBorrow<ExactScalar> + Sized,
        B2: SampleBorrow<ExactScalar> + Sized,
    >(
        low: B1,
        high: B2,
        rng: &mut R,
    ) -> ExactScalar {
        let (low, high) = (*low.borrow(), *high.borrow());
        if let (Some(l), Some(h)) = (low.checked_u64(), high.checked_u64()) {
            return rng.gen_range(l..h).into();
        }
        if let (Some(l), Some(h)) = (low.checked_u128(), high.checked_u128()) {
            return rng.gen_range(l..h).into();
        }
        Self::bounds(low, high, false).sample(rng)
    }
    fn sample_single_inclusive<
        R: Rng + ?Sized,
        B1: SampleBorrow<ExactScalar> + Sized,
        B2: SampleBorrow<ExactScalar> + Sized,
    >(
        low: B1,
        high: B2,
        rng: &mut R,
    ) -> ExactScalar {
        let (low, high) = (*low.borrow(), *high.borrow());
        if let (Some(l), Some(h)) = (low.checked_u64(), high.checked_u64()) {
            return rng.gen_range(l..=h).into();
        }
        if let (Some(l), Some(h)) = (low.checked_u128(), high.checked_u128()) {
            return rng.gen_range(l..=h).into();
        }
        Self::bounds(low, high, true).sample(rng)
    }
}

impl RhoScalar for ExactScalar {
    type MulCtx = ExactScalar;
    type Fixed = ExactScalar;
    fn zero() -> Self {
        0u64.into()
    }
    fn one() -> Self {
        1u64.into()
    }
    fn from_biguint(value: &BigUint) -> Option<Self> {
        ExactScalar::from_biguint(value)
    }
    fn to_biguint(self) -> BigUint {
        ExactScalar::to_biguint(self)
    }
    fn checked_u128(self) -> Option<u128> {
        ExactScalar::checked_u128(self)
    }
    fn to_f64(self) -> f64 {
        ExactScalar::to_f64(self)
    }
    fn bit_len(self) -> u32 {
        self.bits()
    }
    fn bit(self, i: u32) -> bool {
        ExactScalar::bit(self, i)
    }
    fn mul_ctx(m: Self) -> Self {
        assert!(m > Self::one(), "modulus must exceed one");
        m
    }
    fn mul_in(m: &Self, a: Self, b: Self) -> Self {
        Self::mul_mod(a, b, *m)
    }
    fn fixed(m: &Self, w: Self) -> Self {
        exact(w.to_biguint() % m.to_biguint())
    }
    fn mul_fixed(m: &Self, a: Self, w: Self) -> Self {
        Self::mul_mod(a, w, *m)
    }
    fn mul_pair_in(m: &Self, a: Self, b: Self, multiplier: Self) -> (Self, Self) {
        (
            Self::mul_mod(a, multiplier, *m),
            Self::mul_mod(b, multiplier, *m),
        )
    }
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        count(|w| w.modular_muls += 1);
        exact(a.to_biguint() * b.to_biguint() % m.to_biguint())
    }
    fn add_mod(a: Self, b: Self, m: Self) -> Self {
        count(|w| w.modular_adds += 1);
        exact((a.to_biguint() + b.to_biguint()) % m.to_biguint())
    }
    fn sub_mod(a: Self, b: Self, m: Self) -> Self {
        count(|w| w.modular_subs += 1);
        let modulus = m.to_biguint();
        exact((a.to_biguint() % &modulus + &modulus - b.to_biguint() % &modulus) % modulus)
    }
    fn inverse_mod(value: Self, m: Self) -> Option<Self> {
        count(|w| w.modular_inverses += 1);
        if m <= Self::one() {
            return None;
        }
        let modulus = BigInt::from(m.to_biguint());
        let egcd = BigInt::from(value.to_biguint()).extended_gcd(&modulus);
        if egcd.gcd != BigInt::from(1u8) {
            return None;
        }
        Some(exact(egcd.x.mod_floor(&modulus).to_biguint()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};
    #[test]
    fn single_and_cached_narrow_draws_match_the_legacy_rng_streams() {
        for high in [5u128, u64::MAX as u128, (1u128 << 90) + 17, u128::MAX] {
            let mut expected = StdRng::seed_from_u64(7);
            let mut actual = expected.clone();
            for _ in 0..128 {
                let want = if high <= u64::MAX as u128 {
                    expected.gen_range(1..high as u64) as u128
                } else {
                    expected.gen_range(1..high)
                };
                assert_eq!(
                    actual
                        .gen_range(ExactScalar::one()..ExactScalar::from(high))
                        .checked_u128(),
                    Some(want)
                );
            }
            let mut expected = StdRng::seed_from_u64(9);
            let mut actual = expected.clone();
            let exact = Uniform::new_inclusive(ExactScalar::one(), ExactScalar::from(high));
            for _ in 0..128 {
                let want = if high <= u64::MAX as u128 {
                    Uniform::new_inclusive(1, high as u64).sample(&mut expected) as u128
                } else {
                    Uniform::new_inclusive(1u128, high).sample(&mut expected)
                };
                assert_eq!(exact.sample(&mut actual).checked_u128(), Some(want));
            }
        }
    }
    #[test]
    fn wide_draws_cover_high_bits_and_full_inclusive_endpoint() {
        let max = exact((BigUint::from(1u8) << 256usize) - 1u8);
        let high: ExactScalar = crate::cryptanalysis::curve25519_wide::L25519
            .parse()
            .unwrap();
        let mut rng = StdRng::seed_from_u64(13);
        let mut wide = false;
        for _ in 0..128 {
            let value = rng.gen_range(ExactScalar::one()..high);
            assert!(value >= ExactScalar::one() && value < high);
            wide |= value.checked_u128().is_none();
        }
        assert!(wide);
        assert_eq!(rng.gen_range(max..=max), max);
        for _ in 0..8 {
            assert!(rng.gen_range(ExactScalar::zero()..=max) <= max);
        }
    }
    #[test]
    fn full_width_modular_identities_include_inverse_failure() {
        let m: ExactScalar = crate::cryptanalysis::curve25519_wide::L25519
            .parse()
            .unwrap();
        let a = exact(m.to_biguint() - 1u8);
        let b = exact((BigUint::from(1u8) << 200usize) + 17u8);
        assert_eq!(ExactScalar::mul_mod(a, a, m), ExactScalar::one());
        assert_eq!(
            ExactScalar::add_mod(a, ExactScalar::one(), m),
            ExactScalar::zero()
        );
        assert_eq!(
            ExactScalar::sub_mod(ExactScalar::zero(), b, m),
            exact(m.to_biguint() - b.to_biguint())
        );
        let inverse = ExactScalar::inverse_mod(b, m).unwrap();
        assert_eq!(ExactScalar::mul_mod(b, inverse, m), ExactScalar::one());
        assert!(ExactScalar::inverse_mod(3u64.into(), 15u64.into()).is_none());
        assert!(ExactScalar::inverse_mod(ExactScalar::zero(), m).is_none());
        let ctx = ExactScalar::mul_ctx(m);
        assert_eq!(
            ExactScalar::mul_pair_in(&ctx, a, b, b),
            (ExactScalar::mul_mod(a, b, m), ExactScalar::mul_mod(b, b, m))
        );
        assert_eq!(
            ExactScalar::mul_fixed(&ctx, a, ExactScalar::fixed(&ctx, b)),
            ExactScalar::mul_mod(a, b, m)
        );
    }
}
