//! 131-bit prime field in Montgomery form (three 64-bit limbs, p < 2^192).
//! One process-wide modulus; `muls()` counts multiplications on the calling thread.

use std::cell::Cell;
use std::sync::OnceLock;

struct Mctx {
    p: [u64; 3],
    ninv: u64,
    r2: [u64; 3],
    one: [u64; 3],
    pm2: [u64; 3],
    sqrt_e: [u64; 3],
    leg_e: [u64; 3],
}

static MCTX: OnceLock<Mctx> = OnceLock::new();

thread_local! {
    static MULS: Cell<u64> = const { Cell::new(0) };
}

pub fn muls() -> u64 {
    MULS.with(|c| c.get())
}

fn ctx() -> &'static Mctx {
    MCTX.get().expect("modulus not set")
}

fn ge(a: &[u64; 3], b: &[u64; 3]) -> bool {
    for i in (0..3).rev() {
        if a[i] != b[i] {
            return a[i] > b[i];
        }
    }
    true
}

fn sub_raw(a: &[u64; 3], b: &[u64; 3]) -> ([u64; 3], bool) {
    let mut r = [0u64; 3];
    let mut borrow = 0u64;
    for i in 0..3 {
        let (x, b1) = a[i].overflowing_sub(b[i]);
        let (y, b2) = x.overflowing_sub(borrow);
        r[i] = y;
        borrow = (b1 | b2) as u64;
    }
    (r, borrow != 0)
}

fn add_raw(a: &[u64; 3], b: &[u64; 3]) -> ([u64; 3], bool) {
    let mut r = [0u64; 3];
    let mut carry = 0u64;
    for i in 0..3 {
        let (x, c1) = a[i].overflowing_add(b[i]);
        let (y, c2) = x.overflowing_add(carry);
        r[i] = y;
        carry = (c1 | c2) as u64;
    }
    (r, carry != 0)
}

fn shr(a: &[u64; 3], k: u32) -> [u64; 3] {
    let mut r = *a;
    for _ in 0..k {
        let mut carry = 0u64;
        for i in (0..3).rev() {
            let nc = r[i] & 1;
            r[i] = (r[i] >> 1) | (carry << 63);
            carry = nc;
        }
    }
    r
}

pub fn parse_hex_raw(s: &str) -> [u64; 3] {
    let s = s.trim_start_matches("0x");
    let mut r = [0u64; 3];
    for ch in s.chars() {
        let d = ch.to_digit(16).expect("hex digit") as u64;
        let mut carry = d;
        for limb in r.iter_mut() {
            let v = ((*limb as u128) << 4) | carry as u128;
            *limb = v as u64;
            carry = (v >> 64) as u64;
        }
        assert_eq!(carry, 0, "hex literal too wide");
    }
    r
}

/// Set the modulus (odd, < 2^191).  Idempotent for the same value.
pub fn set_modulus(hex: &str) {
    let p = parse_hex_raw(hex);
    let mut inv = p[0];
    for _ in 0..6 {
        inv = inv.wrapping_mul(2u64.wrapping_sub(p[0].wrapping_mul(inv)));
    }
    let ninv = inv.wrapping_neg();
    // R mod p, R = 2^192: double 1 192 times, then continue to R^2 (384 doublings).
    let mut x = [1u64, 0, 0];
    let mut r1 = [0u64; 3];
    for i in 0..384 {
        let (s, c) = add_raw(&x, &x);
        x = if c || ge(&s, &p) { sub_raw(&s, &p).0 } else { s };
        if i == 191 {
            r1 = x;
        }
    }
    let r2 = x;
    let (pm2, _) = sub_raw(&p, &[2, 0, 0]);
    let (pp1, _) = add_raw(&p, &[1, 0, 0]);
    let sqrt_e = shr(&pp1, 2);
    let (pm1, _) = sub_raw(&p, &[1, 0, 0]);
    let leg_e = shr(&pm1, 1);
    let _ = MCTX.set(Mctx { p, ninv, r2, one: r1, pm2, sqrt_e, leg_e });
    assert_eq!(p[0] & 3, 3, "p must be 3 mod 4 for the (p+1)/4 square root");
}

pub fn modulus_hex() -> String {
    raw_hex(&ctx().p)
}

fn raw_hex(r: &[u64; 3]) -> String {
    let s = format!("{:016x}{:016x}{:016x}", r[2], r[1], r[0]);
    let t = s.trim_start_matches('0');
    format!("0x{}", if t.is_empty() { "0" } else { t })
}

fn mont_mul(a: &[u64; 3], b: &[u64; 3]) -> [u64; 3] {
    let c = ctx();
    let p = &c.p;
    let mut t = [0u64; 5];
    for i in 0..3 {
        let mut carry = 0u128;
        for j in 0..3 {
            let s = t[j] as u128 + (a[j] as u128) * (b[i] as u128) + carry;
            t[j] = s as u64;
            carry = s >> 64;
        }
        let s = t[3] as u128 + carry;
        t[3] = s as u64;
        t[4] = (s >> 64) as u64;
        let m = t[0].wrapping_mul(c.ninv);
        let s = t[0] as u128 + (m as u128) * (p[0] as u128);
        let mut carry = s >> 64;
        for j in 1..3 {
            let s = t[j] as u128 + (m as u128) * (p[j] as u128) + carry;
            t[j - 1] = s as u64;
            carry = s >> 64;
        }
        let s = t[3] as u128 + carry;
        t[2] = s as u64;
        let carry = s >> 64;
        t[3] = t[4] + carry as u64;
        t[4] = 0;
    }
    let r = [t[0], t[1], t[2]];
    if t[3] != 0 || ge(&r, p) {
        sub_raw(&r, p).0
    } else {
        r
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Fp(pub [u64; 3]);

impl Fp {
    pub const ZERO: Fp = Fp([0; 3]);
    pub fn one() -> Fp {
        Fp(ctx().one)
    }
    pub fn from_u64(x: u64) -> Fp {
        Fp(mont_mul(&[x, 0, 0], &ctx().r2))
    }
    pub fn from_raw(r: [u64; 3]) -> Fp {
        assert!(!ge(&r, &ctx().p), "value not reduced");
        Fp(mont_mul(&r, &ctx().r2))
    }
    pub fn from_hex(s: &str) -> Fp {
        Fp::from_raw(parse_hex_raw(s))
    }
    pub fn to_raw(self) -> [u64; 3] {
        mont_mul(&self.0, &[1, 0, 0])
    }
    pub fn to_hex(self) -> String {
        raw_hex(&self.to_raw())
    }
    pub fn is_zero(&self) -> bool {
        self.0 == [0; 3]
    }
    pub fn add(self, o: Fp) -> Fp {
        let (s, c) = add_raw(&self.0, &o.0);
        Fp(if c || ge(&s, &ctx().p) { sub_raw(&s, &ctx().p).0 } else { s })
    }
    pub fn sub(self, o: Fp) -> Fp {
        let (s, b) = sub_raw(&self.0, &o.0);
        Fp(if b { add_raw(&s, &ctx().p).0 } else { s })
    }
    pub fn neg(self) -> Fp {
        if self.is_zero() {
            self
        } else {
            Fp(sub_raw(&ctx().p, &self.0).0)
        }
    }
    pub fn mul(self, o: Fp) -> Fp {
        MULS.with(|c| c.set(c.get() + 1));
        Fp(mont_mul(&self.0, &o.0))
    }
    pub fn sq(self) -> Fp {
        self.mul(self)
    }
    pub fn pow_limbs(self, e: &[u64; 3]) -> Fp {
        let mut r = Fp::one();
        for i in (0..192).rev() {
            r = r.sq();
            if (e[i / 64] >> (i % 64)) & 1 == 1 {
                r = r.mul(self);
            }
        }
        r
    }
    pub fn inv(self) -> Fp {
        assert!(!self.is_zero(), "inverse of zero");
        self.pow_limbs(&ctx().pm2)
    }
    pub fn is_square(self) -> bool {
        self.is_zero() || self.pow_limbs(&ctx().leg_e) == Fp::one()
    }
    /// A square root when `self` is a square (p = 3 mod 4).
    pub fn sqrt(self) -> Option<Fp> {
        let r = self.pow_limbs(&ctx().sqrt_e);
        if r.sq() == self {
            Some(r)
        } else {
            None
        }
    }
    pub fn small(x: i64) -> Fp {
        if x >= 0 {
            Fp::from_u64(x as u64)
        } else {
            Fp::from_u64((-x) as u64).neg()
        }
    }
    /// Reduce a big-endian byte string mod p.
    pub fn from_bytes_mod(bytes: &[u8]) -> Fp {
        let k = Fp::from_u64(256);
        let mut acc = Fp::ZERO;
        for &b in bytes {
            acc = acc.mul(k).add(Fp::from_u64(b as u64));
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const P: &str = "0x48e1d43f293469e33194c43186b3abc0b";

    fn slow_mul(a: Fp, b: Fp) -> Fp {
        // double-and-add over the bits of b's raw value, using only add
        let braw = b.to_raw();
        let mut r = Fp::ZERO;
        for i in (0..192).rev() {
            r = r.add(r);
            if (braw[i / 64] >> (i % 64)) & 1 == 1 {
                r = r.add(a);
            }
        }
        r
    }

    #[test]
    fn field_laws() {
        set_modulus(P);
        let mut x = Fp::from_hex("0x1234567890abcdef1234567890abcdef1");
        let mut y = Fp::from_hex("0x2f74f717e8dec90991e5ea9b2ff03da58");
        for i in 0..200 {
            let z = x.mul(y);
            assert_eq!(z, slow_mul(x, y), "mul {i}");
            assert_eq!(z.mul(y.inv()), x);
            assert_eq!(x.add(y).sub(y), x);
            assert_eq!(Fp::from_raw(x.to_raw()), x);
            if let Some(r) = x.sq().sqrt() {
                assert!(r == x || r == x.neg());
            }
            x = z.add(Fp::from_u64(i + 7));
            y = y.sq().add(x);
        }
        assert_eq!(Fp::from_u64(5).to_hex(), "0x5");
        assert_eq!(Fp::from_u64(0).to_hex(), "0x0");
        assert!(Fp::from_u64(4).is_square());
    }
}
