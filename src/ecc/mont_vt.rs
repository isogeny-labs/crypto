//! Variable-time fixed-width Montgomery arithmetic mod an odd `p < 2^(64·N)`,
//! for the public-input Jacobian ladder of [`Point::scalar_mul`].
//!
//! [`FieldElement`]'s operations run on `num-bigint`: every one allocates,
//! multiplies in a general loop and reduces by a long division, and an
//! inversion is a 3-multiplications-a-bit exponentiation.  Here an element
//! is `N` words in Montgomery form (`a·R mod p`, `R = 2^(64·N)`), the
//! product is one CIOS pass, and every value is kept in `[0, p)`, so the
//! Montgomery form is canonical: zero tests and equality tests are the ones
//! on the integers.  Nothing here depends on `p` being prime: the formulas
//! are ring operations, and the inversion is the Fermat power `a^(p−2)`
//! that [`FieldElement::inv`] computes, so results equal the `BigUint`
//! path's for every odd modulus.  Variable time: for public values only.
//!
//! [`Point::scalar_mul`]: super::point::Point::scalar_mul
//! [`FieldElement`]: super::field::FieldElement
//! [`FieldElement::inv`]: super::field::FieldElement::inv

use num_bigint::BigUint;

/// The constants of an odd modulus `p` of at most `N` words.
#[derive(Clone, Copy)]
pub(crate) struct Mont<const N: usize> {
    p: [u64; N],
    /// `-p⁻¹ mod 2^64`.
    n0: u64,
    /// `R² mod p`.
    r2: [u64; N],
    /// `R mod p`: one, in Montgomery form.
    one: [u64; N],
}

fn limbs<const N: usize>(v: &BigUint) -> Option<[u64; N]> {
    let mut out = [0u64; N];
    let mut it = v.iter_u64_digits();
    for o in out.iter_mut() {
        match it.next() {
            Some(d) => *o = d,
            None => return Some(out),
        }
    }
    it.next().is_none().then_some(out)
}

fn to_big<const N: usize>(a: &[u64; N]) -> BigUint {
    BigUint::from_slice(
        &a.iter()
            .flat_map(|w| [*w as u32, (*w >> 32) as u32])
            .collect::<Vec<u32>>(),
    )
}

/// `a ≥ b`.
#[inline(always)]
fn geq<const N: usize>(a: &[u64; N], b: &[u64; N]) -> bool {
    for i in (0..N).rev() {
        if a[i] != b[i] {
            return a[i] > b[i];
        }
    }
    true
}

/// `a − b` in place, returning the borrow.
#[inline(always)]
fn sub_in_place<const N: usize>(a: &mut [u64; N], b: &[u64; N]) -> bool {
    let mut borrow = false;
    for i in 0..N {
        let (d, b1) = a[i].overflowing_sub(b[i]);
        let (d, b2) = d.overflowing_sub(borrow as u64);
        a[i] = d;
        borrow = b1 | b2;
    }
    borrow
}

impl<const N: usize> Mont<N> {
    /// The context for `modulus`, or `None` if it is even, below 3, or
    /// wider than `N` words.
    pub(crate) fn new(modulus: &BigUint) -> Option<Self> {
        let p = limbs::<N>(modulus)?;
        if p[0] & 1 == 0 || *modulus < BigUint::from(3u32) {
            return None;
        }
        // n0 = -p⁻¹ mod 2^64 by Newton iteration.
        let mut inv = 1u64;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(p[0].wrapping_mul(inv)));
        }
        let r = BigUint::from(1u8) << (64 * N);
        let one = limbs::<N>(&(&r % modulus))?;
        let r2 = limbs::<N>(&((&r * &r) % modulus))?;
        Some(Mont {
            p,
            n0: inv.wrapping_neg(),
            r2,
            one,
        })
    }

    /// `a·b·R⁻¹ mod p` for `a, b < p` (CIOS), in `[0, p)`.
    #[inline(always)]
    pub(crate) fn mul(&self, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        let mut t = [0u64; N];
        let mut t_n: u64 = 0;
        for i in 0..N {
            let bi = b[i] as u128;
            let mut carry: u128 = 0;
            for j in 0..N {
                let s = t[j] as u128 + a[j] as u128 * bi + carry;
                t[j] = s as u64;
                carry = s >> 64;
            }
            let s = t_n as u128 + carry;
            t_n = s as u64;
            let t_n1 = (s >> 64) as u64;

            let m = t[0].wrapping_mul(self.n0) as u128;
            let s = t[0] as u128 + m * self.p[0] as u128;
            let mut carry = s >> 64;
            for j in 1..N {
                let s = t[j] as u128 + m * self.p[j] as u128 + carry;
                t[j - 1] = s as u64;
                carry = s >> 64;
            }
            let s = t_n as u128 + carry;
            t[N - 1] = s as u64;
            t_n = t_n1 + (s >> 64) as u64;
        }
        if t_n != 0 || geq(&t, &self.p) {
            sub_in_place(&mut t, &self.p);
        }
        t
    }

    #[inline(always)]
    pub(crate) fn add(&self, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        let mut s = [0u64; N];
        let mut carry = false;
        for i in 0..N {
            let (d, c1) = a[i].overflowing_add(b[i]);
            let (d, c2) = d.overflowing_add(carry as u64);
            s[i] = d;
            carry = c1 | c2;
        }
        if carry || geq(&s, &self.p) {
            sub_in_place(&mut s, &self.p);
        }
        s
    }

    #[inline(always)]
    pub(crate) fn sub(&self, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        let mut d = *a;
        if sub_in_place(&mut d, b) {
            let mut carry = false;
            for i in 0..N {
                let (v, c1) = d[i].overflowing_add(self.p[i]);
                let (v, c2) = v.overflowing_add(carry as u64);
                d[i] = v;
                carry = c1 | c2;
            }
        }
        d
    }

    /// Canonical integer `v < p` to Montgomery form.
    pub(crate) fn enter(&self, v: &BigUint) -> Option<[u64; N]> {
        let w = limbs::<N>(v)?;
        if !geq(&w, &self.p) {
            Some(self.mul(&w, &self.r2))
        } else {
            None
        }
    }

    pub(crate) fn one(&self) -> [u64; N] {
        self.one
    }

    /// Montgomery form back to the integer in `[0, p)`.
    pub(crate) fn leave(&self, a: &[u64; N]) -> BigUint {
        let mut unit = [0u64; N];
        unit[0] = 1;
        to_big(&self.mul(a, &unit))
    }

    /// `a^e` for a Montgomery-form `a` and an integer exponent, square and
    /// multiply from the top bit (as `FieldElement::pow` computes the same
    /// power); the result is in Montgomery form.
    pub(crate) fn pow(&self, a: &[u64; N], e: &BigUint) -> [u64; N] {
        let mut acc = self.one;
        for i in (0..e.bits()).rev() {
            acc = self.mul(&acc, &acc);
            if e.bit(i) {
                acc = self.mul(&acc, a);
            }
        }
        acc
    }
}

/// A Jacobian point `(X : Y : Z)` in Montgomery form, `Z ≠ 0` not assumed.
#[derive(Clone, Copy)]
pub(crate) struct Jac<const N: usize> {
    pub x: [u64; N],
    pub y: [u64; N],
    pub z: [u64; N],
}

impl<const N: usize> Mont<N> {
    #[inline(always)]
    fn is_zero(a: &[u64; N]) -> bool {
        a.iter().all(|&w| w == 0)
    }

    /// `2P` as `Jacobian::double` of `point.rs` (dbl-2007-bl): `None` when
    /// `Y = 0`.
    pub(crate) fn double(&self, j: &Jac<N>, a: &[u64; N]) -> Option<Jac<N>> {
        if Self::is_zero(&j.y) {
            return None;
        }
        let xx = self.mul(&j.x, &j.x);
        let yy = self.mul(&j.y, &j.y);
        let yyyy = self.mul(&yy, &yy);
        let zz = self.mul(&j.z, &j.z);
        let t = self.add(&j.x, &yy);
        let s = self.sub(&self.sub(&self.mul(&t, &t), &xx), &yyyy);
        let s = self.add(&s, &s);
        let m = self.add(&self.add(&self.add(&xx, &xx), &xx), &self.mul(a, &self.mul(&zz, &zz)));
        let x3 = self.sub(&self.sub(&self.mul(&m, &m), &s), &s);
        let yyyy8 = {
            let d = self.add(&yyyy, &yyyy);
            let q = self.add(&d, &d);
            self.add(&q, &q)
        };
        let y3 = self.sub(&self.mul(&m, &self.sub(&s, &x3)), &yyyy8);
        let yz = self.add(&j.y, &j.z);
        let z3 = self.sub(&self.sub(&self.mul(&yz, &yz), &yy), &zz);
        Some(Jac {
            x: x3,
            y: y3,
            z: z3,
        })
    }

    /// `P + (x₂, y₂)` as `Jacobian::add_affine` (madd-2007-bl).
    pub(crate) fn add_affine(
        &self,
        j: &Jac<N>,
        x2: &[u64; N],
        y2: &[u64; N],
        a: &[u64; N],
    ) -> Option<Jac<N>> {
        let z1z1 = self.mul(&j.z, &j.z);
        let u2 = self.mul(x2, &z1z1);
        let s2 = self.mul(&self.mul(y2, &j.z), &z1z1);
        let h = self.sub(&u2, &j.x);
        let r = self.sub(&s2, &j.y);
        if Self::is_zero(&h) {
            return if Self::is_zero(&r) {
                self.double(j, a)
            } else {
                None
            };
        }
        let r = self.add(&r, &r);
        let hh = self.mul(&h, &h);
        let i = self.add(&hh, &hh);
        let i = self.add(&i, &i);
        let jj = self.mul(&h, &i);
        let v = self.mul(&j.x, &i);
        let x3 = self.sub(&self.sub(&self.sub(&self.mul(&r, &r), &jj), &v), &v);
        let y1j = self.mul(&j.y, &jj);
        let y3 = self.sub(&self.sub(&self.mul(&r, &self.sub(&v, &x3)), &y1j), &y1j);
        let zh = self.add(&j.z, &h);
        let z3 = self.sub(&self.sub(&self.mul(&zh, &zh), &z1z1), &hh);
        Some(Jac {
            x: x3,
            y: y3,
            z: z3,
        })
    }
}

/// Constant-time Montgomery product of four-word residues: `a·b·R⁻¹ mod p`
/// for `a, b < p`, `R = 2^256`, `n0 = -p⁻¹ mod 2^64`, in `[0, p)` — the
/// value of [`Uint::mont_mul`](crate::ct_bignum::Uint::mont_mul) for every
/// such input, by one interleaved (CIOS) pass instead of a full product
/// followed by a reduction, with every loop bound fixed and the final
/// subtraction selected by a mask (no branch and no table index depends
/// on a value), as in `ct_bignum`.
#[inline(always)]
pub(crate) fn ct_mont_mul4(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4], n0: u64) -> [u64; 4] {
    let mut t = [0u64; 4];
    let mut t4: u64 = 0;
    for i in 0..4 {
        let bi = b[i] as u128;
        let mut carry: u128 = 0;
        for j in 0..4 {
            let s = t[j] as u128 + a[j] as u128 * bi + carry;
            t[j] = s as u64;
            carry = s >> 64;
        }
        let s = t4 as u128 + carry;
        t4 = s as u64;
        let t5 = (s >> 64) as u64;

        let m = t[0].wrapping_mul(n0) as u128;
        let s = t[0] as u128 + m * p[0] as u128;
        let mut carry = s >> 64;
        for j in 1..4 {
            let s = t[j] as u128 + m * p[j] as u128 + carry;
            t[j - 1] = s as u64;
            carry = s >> 64;
        }
        let s = t4 as u128 + carry;
        t[3] = s as u64;
        t4 = t5 + (s >> 64) as u64;
    }
    // t4·2^256 + t < 2p: subtract p when t4 = 1 or t >= p.
    let mut d = [0u64; 4];
    let mut borrow = 0u64;
    for i in 0..4 {
        let (v, b1) = t[i].overflowing_sub(p[i]);
        let (v, b2) = v.overflowing_sub(borrow);
        d[i] = v;
        borrow = (b1 | b2) as u64;
    }
    let need = t4 | (borrow ^ 1);
    let mask = 0u64.wrapping_sub(need & 1);
    [
        (t[0] & !mask) | (d[0] & mask),
        (t[1] & !mask) | (d[1] & mask),
        (t[2] & !mask) | (d[2] & mask),
        (t[3] & !mask) | (d[3] & mask),
    ]
}

/// `base^exp mod p` for a canonical `base < p` and a 256-bit `exp`, by the
/// fixed 256-iteration square-and-always-multiply ladder of
/// [`mont_pow_ct`](crate::ct_bignum::mont_pow_ct) (the product is always
/// computed and kept or dropped by a mask) on [`ct_mont_mul4`]; `r2` is
/// `R² mod p` and `one` is `R mod p`.  The result is the canonical power.
pub(crate) fn ct_pow4(
    base: &[u64; 4],
    exp: &[u64; 4],
    p: &[u64; 4],
    n0: u64,
    r2: &[u64; 4],
    one: &[u64; 4],
) -> [u64; 4] {
    let base_m = ct_mont_mul4(base, r2, p, n0);
    let mut acc = *one;
    for i in (0..256).rev() {
        acc = ct_mont_mul4(&acc, &acc, p, n0);
        let prod = ct_mont_mul4(&acc, &base_m, p, n0);
        let mask = 0u64.wrapping_sub((exp[i / 64] >> (i % 64)) & 1);
        for k in 0..4 {
            acc[k] = (acc[k] & !mask) | (prod[k] & mask);
        }
    }
    ct_mont_mul4(&acc, &[1, 0, 0, 0], p, n0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ct_bignum::{compute_minv64, Uint, U256};
    use num_bigint::RandBigInt;
    use rand::{rngs::StdRng, SeedableRng};

    fn moduli() -> Vec<BigUint> {
        let two = |e: u32| BigUint::from(1u8) << e;
        vec![
            crate::ecc::curve::CurveParams::p256().p.clone(),
            crate::ecc::curve::CurveParams::secp256k1().p.clone(),
            two(256) - 189u32,
            two(256) - 1u32,
            two(255) + 95u32,
            two(192) + 133u32,
            BigUint::from(3u32),
        ]
    }

    /// `ct_mont_mul4` is `Uint::mont_mul`, on random and edge operands.
    #[test]
    fn ct_mont_mul4_matches_ct_bignum() {
        let mut rng = StdRng::seed_from_u64(0x6d75_6c34);
        for m in moduli() {
            let p = U256::from_biguint(&m);
            let n0 = compute_minv64(p.0[0]);
            let mut vals = vec![BigUint::from(0u8), BigUint::from(1u8), &m - 1u32, &m - 2u32];
            for _ in 0..200 {
                vals.push(rng.gen_biguint_below(&m));
            }
            for a in &vals {
                for b in vals.iter().take(24) {
                    let (ua, ub) = (U256::from_biguint(a), U256::from_biguint(b));
                    assert_eq!(
                        ct_mont_mul4(&ua.0, &ub.0, &p.0, n0),
                        Uint::mont_mul(&ua, &ub, &p, n0).0,
                        "m={m} a={a} b={b}"
                    );
                }
            }
        }
    }

    /// `ct_pow4` is `mont_pow_ct`.
    #[test]
    fn ct_pow4_matches_mont_pow_ct() {
        use crate::ct_bignum::{mont_pow_ct, MontgomeryContext};
        let mut rng = StdRng::seed_from_u64(0x706f_7734);
        for m in moduli().into_iter().take(4) {
            let ctx = MontgomeryContext::new(U256::from_biguint(&m)).unwrap();
            for _ in 0..24 {
                let b = U256::from_biguint(&rng.gen_biguint_below(&m));
                let e = U256::from_biguint(&rng.gen_biguint(256));
                assert_eq!(
                    ct_pow4(&b.0, &e.0, &ctx.n.0, ctx.n_inv_low, &ctx.r2_mod_n.0, &ctx.r_mod_n.0),
                    mont_pow_ct(&b, &e, &ctx).0
                );
            }
        }
    }

    /// The generic variable-time product against `BigUint`, every width.
    fn generic_mul_matches<const N: usize>(m: &BigUint, rng: &mut StdRng) {
        let ctx = Mont::<N>::new(m).unwrap();
        for _ in 0..200 {
            let (a, b) = (rng.gen_biguint_below(m), rng.gen_biguint_below(m));
            let (am, bm) = (ctx.enter(&a).unwrap(), ctx.enter(&b).unwrap());
            assert_eq!(ctx.leave(&ctx.mul(&am, &bm)), (&a * &b) % m);
            assert_eq!(ctx.leave(&ctx.add(&am, &bm)), (&a + &b) % m);
            assert_eq!(ctx.leave(&ctx.sub(&am, &bm)), (m + &a - &b) % m);
        }
    }

    #[test]
    fn generic_arithmetic_matches_biguint() {
        let mut rng = StdRng::seed_from_u64(0x6765_6e31);
        let two = |e: u32| BigUint::from(1u8) << e;
        generic_mul_matches::<1>(&BigUint::from(u64::MAX), &mut rng);
        generic_mul_matches::<1>(&BigUint::from(97u32), &mut rng);
        generic_mul_matches::<2>(&(two(128) - 159u32), &mut rng);
        generic_mul_matches::<4>(&(two(256) - 189u32), &mut rng);
        generic_mul_matches::<4>(&(two(200) + 235u32), &mut rng);
        generic_mul_matches::<9>(&(two(521) - 1u32), &mut rng);
        generic_mul_matches::<9>(&(two(576) - 3u32), &mut rng);
    }
}
