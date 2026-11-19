//! Floating types for LLL Gram-Schmidt data: a common trait implemented by
//! dd::F (double-double, ~106-bit mantissa) and D (f64 mantissa, i64 exponent).
use crate::dd::F;
use num_bigint::{BigInt, Sign};
use num_traits::Zero;

/// Top bits of |x| as (u128 with up to 128 significant bits, shift, negative).
#[inline]
pub fn big_top(x: &BigInt) -> (u128, i64, bool) {
    let neg = x.sign() == Sign::Minus;
    let it = x.iter_u64_digits();
    let n = it.len();
    if n == 0 { return (0, 0, false); }
    let mut it = x.iter_u64_digits().rev();
    let d1 = it.next().unwrap() as u128;
    let d0 = if n >= 2 { it.next().unwrap() as u128 } else { 0 };
    if n >= 2 {
        let v = (d1 << 64) | d0;
        // keep at most 126 significant bits so the value fits an i128
        if v.leading_zeros() < 2 { (v >> 2, 64 * (n as i64 - 2) + 2, neg) } else { (v, 64 * (n as i64 - 2), neg) }
    } else { (d1, 0, neg) }
}

pub trait Fl: Copy + std::fmt::Debug {
    fn zero() -> Self;
    fn from_f64(x: f64) -> Self;
    fn from_big(x: &BigInt) -> Self;
    fn add(self, o: Self) -> Self;
    fn sub(self, o: Self) -> Self;
    fn mul(self, o: Self) -> Self;
    fn div(self, o: Self) -> Self;
    fn is_zero(&self) -> bool;
    fn abs_le_half(&self) -> bool;
    fn abs_le_pow2(&self, k: i64) -> bool;
    fn round_big(&self) -> BigInt;
    fn gt(self, o: Self) -> bool;
    fn log2(&self) -> f64;
}

impl Fl for F {
    fn zero() -> Self { F::ZERO }
    fn from_f64(x: f64) -> Self { F::from_f64(x) }
    fn from_big(x: &BigInt) -> Self { F::from_big(x) }
    fn add(self, o: Self) -> Self { F::add(self, o) }
    fn sub(self, o: Self) -> Self { F::sub(self, o) }
    fn mul(self, o: Self) -> Self { F::mul(self, o) }
    fn div(self, o: Self) -> Self { F::div(self, o) }
    fn is_zero(&self) -> bool { F::is_zero(self) }
    fn abs_le_half(&self) -> bool { F::abs_le_half(self) }
    fn abs_le_pow2(&self, k: i64) -> bool { F::abs_le_pow2(self, k) }
    fn round_big(&self) -> BigInt { F::round_big(self) }
    fn gt(self, o: Self) -> bool { F::gt(self, o) }
    fn log2(&self) -> f64 { F::log2(self) }
}

#[derive(Clone, Copy, Debug)]
pub struct D { m: f64, e: i64 }

#[inline]
fn pow2(k: i64) -> f64 { f64::from_bits(((1023 + k) as u64) << 52) }
#[inline]
fn expo(h: f64) -> i64 { (((h.to_bits() >> 52) & 0x7ff) as i64) - 1022 }

impl D {
    #[inline]
    fn norm(m: f64, e: i64) -> D {
        if m == 0.0 { return D { m: 0.0, e: 0 }; }
        let k = expo(m);
        D { m: m * pow2(-k), e: e + k }
    }
}

impl Fl for D {
    #[inline] fn zero() -> Self { D { m: 0.0, e: 0 } }
    fn from_f64(x: f64) -> Self { D::norm(x, 0) }
    #[inline]
    fn from_big(x: &BigInt) -> Self {
        if x.is_zero() { return D::zero(); }
        let (t, sh, neg) = big_top(x);
        let v = t as f64;
        D::norm(if neg { -v } else { v }, sh)
    }
    #[inline]
    fn add(self, o: Self) -> Self {
        if self.m == 0.0 { return o; }
        if o.m == 0.0 { return self; }
        let (a, b) = if self.e >= o.e { (self, o) } else { (o, self) };
        let d = a.e - b.e;
        if d > 60 { return a; }
        D::norm(a.m + b.m * pow2(-d), a.e)
    }
    #[inline] fn sub(self, o: Self) -> Self { self.add(D { m: -o.m, e: o.e }) }
    #[inline] fn mul(self, o: Self) -> Self { if self.m == 0.0 || o.m == 0.0 { D::zero() } else { D::norm(self.m * o.m, self.e + o.e) } }
    #[inline] fn div(self, o: Self) -> Self { assert!(o.m != 0.0, "division by zero"); if self.m == 0.0 { D::zero() } else { D::norm(self.m / o.m, self.e - o.e) } }
    #[inline] fn is_zero(&self) -> bool { self.m == 0.0 }
    #[inline] fn abs_le_half(&self) -> bool { self.m == 0.0 || self.e < 0 || (self.e == 0 && self.m.abs() <= 0.51) }
    #[inline] fn abs_le_pow2(&self, k: i64) -> bool { self.m == 0.0 || self.e <= k }
    fn round_big(&self) -> BigInt {
        if self.m == 0.0 || self.e < 0 { return BigInt::zero(); }
        if self.e <= 62 { BigInt::from((self.m * pow2(self.e)).round() as i64) }
        else { BigInt::from((self.m * pow2(53)) as i64) << ((self.e - 53) as usize) }
    }
    #[inline] fn gt(self, o: Self) -> bool { self.sub(o).m > 0.0 }
    fn log2(&self) -> f64 { if self.m == 0.0 { f64::NEG_INFINITY } else { self.e as f64 + self.m.abs().log2() } }
}
