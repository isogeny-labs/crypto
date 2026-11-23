//! Double-double floating point (about 106-bit mantissa) with an unbounded
//! binary exponent: value = (hi + lo) * 2^e, |hi| in [0.5, 1) when nonzero.
//! Used only for the Gram-Schmidt data inside LLL; the basis stays exact.
use num_bigint::BigInt;
use num_traits::{Signed, Zero};

#[derive(Clone, Copy, Debug)]
pub struct F { hi: f64, lo: f64, e: i64 }

#[inline]
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let bb = s - a;
    (s, (a - (s - bb)) + (b - bb))
}
#[inline]
fn quick_two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    (s, b - (s - a))
}
#[inline]
fn pow2(k: i64) -> f64 {
    debug_assert!(k > -1022 && k < 1023);
    f64::from_bits(((1023 + k) as u64) << 52)
}
#[inline]
fn expo(h: f64) -> i64 {
    // h normal, nonzero: returns k with |h| / 2^k in [0.5, 1)
    (((h.to_bits() >> 52) & 0x7ff) as i64) - 1022
}

impl F {
    pub const ZERO: F = F { hi: 0.0, lo: 0.0, e: 0 };
    #[inline]
    fn norm(hi: f64, lo: f64, e: i64) -> F {
        let (h, l) = quick_two_sum(hi, lo);
        if h == 0.0 { return F::ZERO; }
        let k = expo(h);
        let s = pow2(-k);
        F { hi: h * s, lo: l * s, e: e + k }
    }
    pub fn from_f64(x: f64) -> F { if x == 0.0 { F::ZERO } else { F::norm(x, 0.0, 0) } }
    pub fn is_zero(&self) -> bool { self.hi == 0.0 }
    pub fn from_big(x: &BigInt) -> F {
        if x.is_zero() { return F::ZERO; }
        let (t, sh, neg) = crate::fl::big_top(x);
        let hi = t as f64;
        let lo = (t as i128 - hi as u128 as i128) as f64;
        let (hi, lo) = if neg { (-hi, -lo) } else { (hi, lo) };
        F::norm(hi, lo, sh)
    }
    pub fn neg(self) -> F { F { hi: -self.hi, lo: -self.lo, e: self.e } }
    pub fn abs(self) -> F { if self.hi < 0.0 { self.neg() } else { self } }
    pub fn sign(&self) -> i32 { if self.hi > 0.0 { 1 } else if self.hi < 0.0 { -1 } else { 0 } }
    pub fn add(self, o: F) -> F {
        if self.hi == 0.0 { return o; }
        if o.hi == 0.0 { return self; }
        let (a, b) = if self.e >= o.e { (self, o) } else { (o, self) };
        let d = a.e - b.e;
        if d > 110 { return a; }
        let s = pow2(-d);
        let (bh, bl) = (b.hi * s, b.lo * s);
        let (s1, s2) = two_sum(a.hi, bh);
        let (t1, t2) = two_sum(a.lo, bl);
        let s2 = s2 + t1;
        let (s1, s2) = quick_two_sum(s1, s2);
        let s2 = s2 + t2;
        F::norm(s1, s2, a.e)
    }
    pub fn sub(self, o: F) -> F { self.add(o.neg()) }
    pub fn mul(self, o: F) -> F {
        if self.hi == 0.0 || o.hi == 0.0 { return F::ZERO; }
        let p1 = self.hi * o.hi;
        let p2 = self.hi.mul_add(o.hi, -p1);
        let p2 = p2 + (self.hi * o.lo + self.lo * o.hi);
        F::norm(p1, p2, self.e + o.e)
    }
    pub fn div(self, o: F) -> F {
        assert!(o.hi != 0.0, "dd division by zero");
        if self.hi == 0.0 { return F::ZERO; }
        // mantissa division, exponent separately
        let a = F { hi: self.hi, lo: self.lo, e: 0 };
        let b = F { hi: o.hi, lo: o.lo, e: 0 };
        let q1 = a.hi / b.hi;
        let r = a.sub(b.mul(F::from_f64(q1)));
        let q2 = r.to_f64() / b.hi;
        let r = r.sub(b.mul(F::from_f64(q2)));
        let q3 = r.to_f64() / b.hi;
        let q = F::from_f64(q1).add(F::from_f64(q2)).add(F::from_f64(q3));
        F { hi: q.hi, lo: q.lo, e: q.e + self.e - o.e }
    }
    pub fn to_f64(&self) -> f64 {
        if self.hi == 0.0 { return 0.0; }
        if self.e > 1000 { return f64::INFINITY * self.hi.signum(); }
        if self.e < -1000 { return 0.0; }
        (self.hi + self.lo) * pow2(self.e)
    }
    /// log2 |x| (approximate).
    pub fn log2(&self) -> f64 { if self.hi == 0.0 { f64::NEG_INFINITY } else { self.e as f64 + self.hi.abs().log2() } }
    pub fn gt(self, o: F) -> bool { self.sub(o).sign() > 0 }
    /// Nearest integer as BigInt (exact up to the 106-bit precision).
    pub fn round_big(&self) -> BigInt {
        if self.hi == 0.0 || self.e < 0 { return BigInt::zero(); }
        if self.e <= 100 {
            let s = pow2(self.e);
            let (h, l) = (self.hi * s, self.lo * s);
            let fh = h.floor();
            let r = fh as i128 + ((h - fh) + l).round() as i128;
            BigInt::from(r)
        } else {
            let s = pow2(100);
            let (h, l) = (self.hi * s, self.lo * s);
            let fh = h.floor();
            let r = fh as i128 + ((h - fh) + l).round() as i128;
            BigInt::from(r) << ((self.e - 100) as usize)
        }
    }
    /// |x| <= 2^k ?
    pub fn abs_le_pow2(&self, k: i64) -> bool { self.hi == 0.0 || self.e <= k }
    pub fn abs_le_half(&self) -> bool {
        // |x| <= 0.51 (eta)
        self.hi == 0.0 || self.abs().to_f64() <= 0.51
    }
}

#[allow(dead_code)]
pub fn big_to_log2(x: &BigInt) -> f64 {
    if x.is_zero() { return f64::NEG_INFINITY; }
    F::from_big(&x.abs()).log2()
}
