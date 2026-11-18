//! Radical (closed-form) low-degree root finding over F_p, 5 <= p <= 2^61-1.
//!
//! Quadratic (Tonelli-Shanks), cubic (Cardano) and quartic (Ferrari) distinct
//! F_p-roots. The crux primitive is a modular cube root: bijective power for
//! p = 2 mod 3, Adleman-Manders-Miller (AMM) for p = 1 mod 3, and AMM over
//! F_{p^2} = F_p[i]/(i^2 - n) for the casus irreducibilis (three F_p roots whose
//! Cardano radicals live in F_{p^2}). No brute-force fallback anywhere.
//!
//! Honest limit: a quartic of Frobenius cycle type (1,3) (one rational root,
//! irreducible cubic cofactor) has no rational Ferrari pairing; its root needs
//! F_{p^3} radicals. That class (about 1/3 of random quartics) is delegated to
//! the distinct-degree gcd(X^p - X, f) step of `prime_root_trace::solve` and is
//! NOT accelerated (`RadicalRoots::frobenius_fallback`).
//!
//! Prototype evidence (counted field multiplications, validation vs brute force
//! and Cantor-Zassenhaus): research/radical_quartic_20261117/. Off the
//! `prime_fast` subtraction hot path. [pcc:unverified]
use super::prime_root_trace::{solve, RootLimits, RootStop, MAX_PRIME, MAX_WORK};
use super::residual_walk::is_prime_u64;

/// Distinct roots plus whether the (1,3) Frobenius fallback was used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadicalRoots {
    pub roots: Vec<u64>,
    pub frobenius_fallback: bool,
}

fn frobenius_roots(p: u64, poly: &[u64]) -> Result<Vec<u64>, String> {
    let limits = RootLimits {
        field_work: MAX_WORK,
        split_attempts: 1000,
        events: 100_000,
        degree: 8,
    };
    let r = solve(poly, p, 0x5eed, limits, &mut |_| Ok(()))?;
    if r.stop != RootStop::Complete {
        return Err(format!("frobenius fallback incomplete: {:?}", r.stop));
    }
    Ok(r.roots)
}

#[derive(Clone, Copy)]
pub(crate) struct Fp {
    pub p: u64,
}
impl Fp {
    pub fn new(p: u64) -> Self {
        Fp { p }
    }
    #[inline]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        // Radical arithmetic normally operates on canonical residues.
        // For a,b < p <= 2^61-1, a+b fits in u64 and needs one correction.
        // Keep the former wide-modulus behavior for noncanonical callers.
        if a < self.p && b < self.p {
            let sum = a + b;
            if sum >= self.p { sum - self.p } else { sum }
        } else {
            ((a as u128 + b as u128) % self.p as u128) as u64
        }
    }
    #[inline]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        if a < self.p && b < self.p {
            if a >= b { a - b } else { a + self.p - b }
        } else {
            ((a as u128 + self.p as u128 - b as u128) % self.p as u128) as u64
        }
    }
    #[inline]
    pub fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    #[inline]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        if self.p == MAX_PRIME && a < MAX_PRIME && b < MAX_PRIME {
            // p = 2^61 - 1: 2^61 == 1 mod p. The 122-bit product
            // reduces with two end-around-carry folds, no wide division.
            let product = a as u128 * b as u128;
            let folded = ((product as u64) & MAX_PRIME) + (product >> 61) as u64;
            let folded = (folded & MAX_PRIME) + (folded >> 61);
            if folded >= MAX_PRIME { folded - MAX_PRIME } else { folded }
        } else {
            ((a as u128 * b as u128) % self.p as u128) as u64
        }
    }
    #[inline]
    pub fn sqr(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    pub fn powm(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64 % self.p;
        a %= self.p;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.sqr(a);
            e >>= 1;
        }
        r
    }
    pub fn inv(&self, a: u64) -> u64 {
        // Extended Euclid replaces ~log(p) Fermat squarings/multiplies on
        // public root-finder inputs. The modulus is prime and < 2^61;
        // Euclidean coefficients and q*t stay within signed i64 bounds.
        // Variable time: NEVER use for secret-dependent field operations.
        let mut r0 = self.p;
        let mut r1 = if a < self.p { a } else { a % self.p };
        if r1 == 0 {
            return 0; // preserve powm(0, p-2)
        }
        let (mut t0, mut t1) = (0i64, 1i64);
        while r1 != 0 {
            let q = r0 / r1;
            let r2 = r0 - q * r1;
            let t2 = t0 - (q as i64) * t1;
            r0 = r1;
            r1 = r2;
            t0 = t1;
            t1 = t2;
        }
        if t0 < 0 { (t0 + self.p as i64) as u64 } else { t0 as u64 }
    }
}

// ---------- generic finite-field ring for the AMM cube root ----------
pub(crate) trait Ring {
    type E: Copy + PartialEq + std::fmt::Debug;
    fn zero(&self) -> Self::E;
    fn one(&self) -> Self::E;
    fn mul(&self, a: Self::E, b: Self::E) -> Self::E;
    fn order(&self) -> u128; // |K^*|
    fn cand(&self, k: u64) -> Self::E; // k-th nonzero candidate (for non-cube search)
}
impl Ring for Fp {
    type E = u64;
    fn zero(&self) -> u64 {
        0
    }
    fn one(&self) -> u64 {
        1
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        Fp::mul(self, a, b)
    }
    fn order(&self) -> u128 {
        (self.p - 1) as u128
    }
    fn cand(&self, k: u64) -> u64 {
        (k + 2) % self.p
    }
}
// F_{p^2} = F_p[i]/(i^2 - n), n a fixed quadratic non-residue.
#[derive(Clone, Copy)]
pub(crate) struct Fq2 {
    pub f: Fp,
    pub n: u64,
}
impl Ring for Fq2 {
    type E = (u64, u64);
    fn zero(&self) -> (u64, u64) {
        (0, 0)
    }
    fn one(&self) -> (u64, u64) {
        (1, 0)
    }
    fn mul(&self, x: (u64, u64), y: (u64, u64)) -> (u64, u64) {
        let f = &self.f;
        let ac = f.mul(x.0, y.0);
        let bd = f.mul(x.1, y.1);
        // The first quadratic non-residue is commonly 2 or 3.
        // In these cases, one or two cheap modular additions replace a
        // fourth full base-field multiplication (particularly costly for
        // generic primes using 128-bit remainder).
        let nbd = match self.n {
            2 => f.add(bd, bd),
            3 => f.add(f.add(bd, bd), bd),
            _ => f.mul(self.n, bd),
        };
        let s = f.mul(f.add(x.0, x.1), f.add(y.0, y.1));
        (f.add(ac, nbd), f.sub(f.sub(s, ac), bd))
    }
    fn order(&self) -> u128 {
        (self.f.p as u128) * (self.f.p as u128) - 1
    }
    fn cand(&self, k: u64) -> (u64, u64) {
        (k % self.f.p, 1)
    }
}
pub(crate) fn rpow<R: Ring>(r: &R, a: R::E, mut e: u128) -> R::E {
    let mut acc = r.one();
    let mut b = a;
    while e > 0 {
        if e & 1 == 1 {
            acc = r.mul(acc, b);
        }
        e >>= 1;
        if e > 0 {
            b = r.mul(b, b);
        }
    }
    acc
}

/// Adleman-Manders-Miller cube root in a finite field K with group order N = |K^*|.
/// Per-field precomputation (non-cube, Sylow generator) is done once in `new`.
pub(crate) struct Cube<R: Ring> {
    pub r: R,
    n: u128,
    s: u32,
    e3: u128,
    c: R::E,
    pub w: R::E,
    p3s: u128,
}
impl<R: Ring> Cube<R> {
    pub fn new(r: R) -> Self {
        let n = r.order();
        let (mut t, mut s) = (n, 0u32);
        while t % 3 == 0 {
            t /= 3;
            s += 1;
        }
        if s == 0 {
            // cubing is a bijection: root = a^(3^{-1} mod N)
            let e3 = if n % 3 == 1 {
                (2 * n + 1) / 3
            } else {
                (n + 1) / 3
            };
            let one = r.one();
            return Cube {
                r,
                n,
                s,
                e3,
                c: one,
                w: one,
                p3s: 1,
            };
        }
        let mut k = 0u64;
        let g = loop {
            let g = r.cand(k);
            if rpow(&r, g, n / 3) != r.one() {
                break g;
            }
            k += 1;
        };
        let c = rpow(&r, g, t); // generator of the 3-Sylow, order 3^s
        let p3s = 3u128.pow(s);
        let w = rpow(&r, c, p3s / 3); // primitive cube root of unity
        let e3 = if t % 3 == 1 {
            (2 * t + 1) / 3
        } else {
            (t + 1) / 3
        }; // 3*e3 = 1 mod t
        Cube {
            r,
            n,
            s,
            e3,
            c,
            w,
            p3s,
        }
    }
    pub fn bijective(&self) -> bool {
        self.s == 0
    }
    /// One cube root of `a`, or None if `a` is not a cube.
    pub fn root(&self, a: R::E) -> Option<R::E> {
        let r = &self.r;
        if a == r.zero() {
            return Some(a);
        }
        if self.s == 0 {
            return Some(rpow(r, a, self.e3));
        }
        if rpow(r, a, self.n / 3) != r.one() {
            return None;
        }
        let x0 = rpow(r, a, self.e3); // x0^3 = a * h, h in the 3-Sylow
        let h = rpow(r, a, 3 * self.e3 - 1);
        let w2 = r.mul(self.w, self.w);
        let mut l: u128 = 0; // h = c^l, l found digit by digit
        for j in 0..self.s {
            let corr = rpow(r, self.c, (self.p3s - l) % self.p3s); // c^{-l}
            let x = r.mul(h, corr);
            let z = rpow(r, x, 3u128.pow(self.s - 1 - j)); // w^digit
            let d = if z == r.one() {
                0
            } else if z == self.w {
                1
            } else if z == w2 {
                2
            } else {
                return None;
            };
            l += d * 3u128.pow(j);
        }
        if !l.is_multiple_of(3) {
            return None;
        }
        let y_inv = rpow(r, self.c, (self.p3s - l / 3) % self.p3s); // c^{-l/3}
        Some(r.mul(x0, y_inv))
    }
}

/// Per-prime context: one-time precomputation (non-residues, 3-Sylow generators, constants).
pub struct RadicalRootCtx {
    pub(crate) f: Fp,
    inv2: u64,
    inv3: u64,
    inv4: u64,
    inv27: u64,
    inv_n: u64,
    n: u64, // quadratic non-residue
    s2: u32,
    t2: u64,
    zc: u64, // Tonelli-Shanks: p-1 = 2^s2 t2, zc = n^t2
    cube_p: Cube<Fp>,
    cube_q: Cube<Fq2>,
    w2: (u64, u64), // primitive cube root of unity in F_{p^2}
}
impl RadicalRootCtx {
    pub fn new(p: u64) -> Result<Self, String> {
        if !(5..=MAX_PRIME).contains(&p) || !is_prime_u64(p) {
            return Err("radical root finder needs a prime 5 <= p <= 2^61-1".into());
        }
        let f = Fp::new(p);
        let n = {
            let mut z = 2u64;
            while f.powm(z, (p - 1) / 2) == 1 {
                z += 1;
            }
            z
        };
        let (mut t2, mut s2) = (p - 1, 0u32);
        while t2 & 1 == 0 {
            t2 >>= 1;
            s2 += 1;
        }
        let zc = f.powm(n, t2);
        let cube_p = Cube::new(f);
        let cube_q = Cube::new(Fq2 { f, n });
        let w2 = cube_q.w;
        let i4 = f.inv(4 % p);
        let i27 = f.inv(27 % p);
        Ok(RadicalRootCtx {
            f,
            inv2: f.inv(2),
            inv3: f.inv(3),
            inv4: i4,
            inv27: i27,
            inv_n: f.inv(n),
            n,
            s2,
            t2,
            zc,
            cube_p,
            cube_q,
            w2,
        })
    }
    /// Square root in F_p (p%4==3: one pow; else Tonelli-Shanks), None for non-residues.
    pub fn sqrt(&self, a: u64) -> Option<u64> {
        let f = &self.f;
        let p = f.p;
        if a == 0 {
            return Some(0);
        }
        if p % 4 == 3 {
            let r = f.powm(a, (p + 1) / 4);
            return if f.sqr(r) == a { Some(r) } else { None };
        }
        if f.powm(a, (p - 1) / 2) != 1 {
            return None;
        }
        let mut m = self.s2;
        let mut c = self.zc;
        let mut t = f.powm(a, self.t2);
        let mut r = f.powm(a, self.t2.div_ceil(2));
        loop {
            if t == 1 {
                return Some(r);
            }
            let mut i = 0u32;
            let mut t2 = t;
            while t2 != 1 && i < m {
                t2 = f.sqr(t2);
                i += 1;
            }
            if i == m {
                return None;
            }
            let b = f.powm(c, 1u64 << (m - i - 1));
            m = i;
            c = f.sqr(b);
            t = f.mul(t, c);
            r = f.mul(r, b);
        }
    }
    /// Distinct roots of monic y^2 + b y + c.
    pub fn quad_monic(&self, b: u64, c: u64) -> Vec<u64> {
        let f = &self.f;
        let disc = f.sub(f.sqr(b), f.mul(4, c));
        match self.sqrt(disc) {
            None => vec![],
            Some(0) => vec![f.mul(f.neg(b), self.inv2)],
            Some(s) => {
                let nb = f.neg(b);
                vec![
                    f.mul(f.add(nb, s), self.inv2),
                    f.mul(f.sub(nb, s), self.inv2),
                ]
            }
        }
    }
    /// General quadratic a x^2 + b x + c (a != 0), sorted distinct roots.
    pub fn quad_roots(&self, a: u64, b: u64, c: u64) -> Vec<u64> {
        let f = &self.f;
        let ia = f.inv(a);
        let mut v = self.quad_monic(f.mul(b, ia), f.mul(c, ia));
        v.sort_unstable();
        v.dedup();
        v
    }
    /// F_p-roots of the depressed cubic t^3 + P t + Q = 0 (Cardano; F_{p^2} when D is a non-residue).
    pub fn cubic_depressed(&self, pp: u64, q: u64) -> Vec<u64> {
        let f = &self.f;
        if pp == 0 {
            if q == 0 {
                return vec![0];
            }
            return match self.cube_p.root(f.neg(q)) {
                None => vec![],
                Some(u) => {
                    if self.cube_p.bijective() {
                        vec![u]
                    } else {
                        let w = self.cube_p.w;
                        let uw = f.mul(u, w);
                        vec![u, uw, f.mul(uw, w)]
                    }
                }
            };
        }
        let q2 = f.mul(q, self.inv2);
        let p2 = f.sqr(pp);
        let d = f.add(f.mul(q2, q2), f.mul(f.mul(p2, pp), self.inv27)); // q^2/4 + P^3/27
        if d == 0 {
            // double root -3Q/(2P), simple root 3Q/P
            let ip = f.inv(pp);
            let r1 = f.mul(f.mul(3, q), ip);
            let r2 = f.mul(f.neg(r1), self.inv2);
            return vec![r1, r2];
        }
        let nq2 = f.neg(q2);
        let m = f.neg(f.mul(pp, self.inv3)); // -P/3
        match self.sqrt(d) {
            Some(sg) => {
                let u3 = f.add(nq2, sg);
                match self.cube_p.root(u3) {
                    None => vec![],
                    Some(u) => {
                        let v = f.mul(m, f.inv(u));
                        if self.cube_p.bijective() {
                            vec![f.add(u, v)]
                        } else {
                            let w = self.cube_p.w;
                            let w_2 = f.sqr(w);
                            let (mut uk, mut vk) = (u, v);
                            let mut out = vec![f.add(u, v)];
                            for _ in 0..2 {
                                uk = f.mul(uk, w);
                                vk = f.mul(vk, w_2);
                                out.push(f.add(uk, vk));
                            }
                            out
                        }
                    }
                }
            }
            None => {
                // casus irreducibilis route: sqrt(D) = i * sqrt(D/n) in F_{p^2}
                let sg = self
                    .sqrt(f.mul(d, self.inv_n))
                    .expect("D/n must be a residue");
                let u3 = (nq2, sg);
                let k = &self.cube_q;
                let u = match k.root(u3) {
                    None => return vec![],
                    Some(u) => u,
                };
                // 1/u = conj(u)/N(u)
                let nrm = f.sub(f.sqr(u.0), f.mul(self.n, f.sqr(u.1)));
                let ni = f.inv(nrm);
                let uinv = (f.mul(u.0, ni), f.mul(f.neg(u.1), ni));
                let v = (f.mul(m, uinv.0), f.mul(m, uinv.1));
                let w = self.w2;
                let w_2 = k.r.mul(w, w);
                let mut out = vec![];
                let (mut uk, mut vk) = (u, v);
                for kk in 0..3 {
                    if kk > 0 {
                        uk = k.r.mul(uk, w);
                        vk = k.r.mul(vk, w_2);
                    }
                    let im = f.add(uk.1, vk.1);
                    if im == 0 {
                        out.push(f.add(uk.0, vk.0));
                    }
                }
                out
            }
        }
    }
    /// F_p-roots of the monic cubic x^3 + c2 x^2 + c1 x + c0.
    pub fn cubic_monic(&self, c2: u64, c1: u64, c0: u64) -> Vec<u64> {
        let f = &self.f;
        let c2sq = f.sqr(c2);
        let pp = f.sub(c1, f.mul(c2sq, self.inv3));
        // q = 2 c2^3/27 - c1 c2/3 + c0
        let t1 = f.mul(f.mul(c2sq, c2), f.mul(2, self.inv27));
        let t2 = f.mul(f.mul(c1, c2), self.inv3);
        let q = f.add(f.sub(t1, t2), c0);
        let sh = f.mul(c2, self.inv3);
        self.cubic_depressed(pp, q)
            .into_iter()
            .map(|t| f.sub(t, sh))
            .collect()
    }
    /// Distinct F_p roots of a x^4 + b x^3 + c x^2 + d x + e (a != 0). Returns (sorted roots, used_fallback).
    pub(crate) fn quartic(
        &self,
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
    ) -> Result<(Vec<u64>, bool), String> {
        let f = &self.f;
        // monic
        let (b1, c1, d1, e1) = if a == 1 {
            (b, c, d, e)
        } else {
            let ia = f.inv(a);
            (f.mul(b, ia), f.mul(c, ia), f.mul(d, ia), f.mul(e, ia))
        };
        // x = y - B, B = b1/4
        let bb = f.mul(b1, self.inv4);
        let b2 = f.sqr(bb);
        let pp = f.sub(c1, f.mul(6, b2));
        let b3 = f.mul(b2, bb);
        let qq = f.add(f.sub(d1, f.mul(f.mul(2, bb), c1)), f.mul(8, b3));
        let rr = f.sub(
            f.add(f.sub(e1, f.mul(bb, d1)), f.mul(b2, c1)),
            f.mul(3, f.sqr(b2)),
        );
        let mut ys: Vec<u64> = vec![];
        let mut fallback = false;
        if qq == 0 {
            for z in self.quad_monic(pp, rr) {
                match self.sqrt(z) {
                    None => {}
                    Some(0) => ys.push(0),
                    Some(s) => {
                        ys.push(s);
                        ys.push(f.neg(s));
                    }
                }
            }
        } else {
            // resolvent in z = (r1+r2)^2:  z^3 + 2P z^2 + (P^2 - 4R) z - Q^2 = 0
            let zs = self.cubic_monic(
                f.mul(2, pp),
                f.sub(f.sqr(pp), f.mul(4, rr)),
                f.neg(f.sqr(qq)),
            );
            if zs.is_empty() {
                // Frobenius type (1,3): no rational pairing; the unique root needs F_{p^3} radicals.
                // Use the distinct-degree (CZ) path on the depressed quartic - NOT radical.
                fallback = true;
                ys = frobenius_roots(f.p, &[rr, qq, pp, 0, 1])?;
            } else {
                for &z in &zs {
                    if let Some(al) = self.sqrt(z) {
                        // quartic = (y^2 + al y + be)(y^2 - al y + ga)
                        let qa = f.mul(qq, f.inv(al));
                        let s = f.add(pp, z);
                        let be = f.mul(f.sub(s, qa), self.inv2);
                        let ga = f.mul(f.add(s, qa), self.inv2);
                        ys.extend(self.quad_monic(al, be));
                        ys.extend(self.quad_monic(f.neg(al), ga));
                        break;
                    }
                }
            }
        }
        let mut xs: Vec<u64> = ys.into_iter().map(|y| f.sub(y, bb)).collect();
        xs.sort_unstable();
        xs.dedup();
        Ok((xs, fallback))
    }
}

impl RadicalRootCtx {
    /// Distinct F_p roots of `f` (coefficients low to high, any trailing zeros allowed).
    /// Degrees 1..=4 are supported; the zero polynomial and constants return an error / no roots.
    pub fn roots(&self, f: &[u64]) -> Result<RadicalRoots, String> {
        let fp = &self.f;
        let mut c: Vec<u64> = f.iter().map(|&x| x % fp.p).collect();
        while c.last() == Some(&0) {
            c.pop();
        }
        let ok = |roots: Vec<u64>| {
            Ok(RadicalRoots {
                roots,
                frobenius_fallback: false,
            })
        };
        match c.len() {
            0 => Err("zero polynomial".into()),
            1 => ok(vec![]),
            2 => ok(vec![fp.mul(fp.neg(c[0]), fp.inv(c[1]))]),
            3 => ok(self.quad_roots(c[2], c[1], c[0])),
            4 => {
                let ia = fp.inv(c[3]);
                let mut v = self.cubic_monic(fp.mul(c[2], ia), fp.mul(c[1], ia), fp.mul(c[0], ia));
                v.sort_unstable();
                v.dedup();
                ok(v)
            }
            5 => {
                let (roots, fb) = self.quartic(c[4], c[3], c[2], c[1], c[0])?;
                Ok(RadicalRoots {
                    roots,
                    frobenius_fallback: fb,
                })
            }
            _ => Err("radical root finder supports degree <= 4".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimized_field_arithmetic_matches_wide_reference() {
        fn ref_mul(p: u64, a: u64, b: u64) -> u64 {
            ((a as u128 * b as u128) % p as u128) as u64
        }
        let mut seed = 0x7d3af0192ebc42u64;
        for &p in &[5u64, 7, 11, 13, 97, 65_537, 998_244_353,
                     1_000_000_007, 2_147_483_647, MAX_PRIME] {
            let f = Fp::new(p);
            let mut input = vec![0, 1, 2, p / 2, p - 2, p - 1];
            for _ in 0..4_096 {
                input.push(lcg(&mut seed) % p);
            }
            for (i, &a) in input.iter().enumerate() {
                let b = input[(i * 37 + 1) % input.len()];
                let expected_add = ((a as u128 + b as u128) % p as u128) as u64;
                let expected_sub = ((a as u128 + p as u128 - b as u128) % p as u128) as u64;
                assert_eq!(f.add(a, b), expected_add, "add p={p} a={a} b={b}");
                assert_eq!(f.sub(a, b), expected_sub, "sub p={p} a={a} b={b}");
                assert_eq!(f.mul(a, b), ref_mul(p, a, b), "mul p={p} a={a} b={b}");
                if i < 300 {
                    let inv = f.inv(a);
                    assert_eq!(inv, f.powm(a, p - 2), "inv p={p} a={a}");
                    if a != 0 { assert_eq!(ref_mul(p, a, inv), 1); }
                }
                let q = Fq2 { f, n: if i & 1 == 0 { 2 } else { 3 } };
                let x = (a, b);
                let y = (input[(i + 7) % input.len()], input[(i + 13) % input.len()]);
                let ac = ref_mul(p, x.0, y.0);
                let bd = ref_mul(p, x.1, y.1);
                let nbd = ref_mul(p, q.n, bd);
                let s = ref_mul(
                    p,
                    ((x.0 as u128 + x.1 as u128) % p as u128) as u64,
                    ((y.0 as u128 + y.1 as u128) % p as u128) as u64,
                );
                let real = ((ac as u128 + nbd as u128) % p as u128) as u64;
                let imag = ((s as u128 + 2 * p as u128 - ac as u128 - bd as u128)
                    % p as u128) as u64;
                assert_eq!(q.mul(x, y), (real, imag), "Fq2 p={p} x={x:?} y={y:?}");
            }
            // Public Fp entry points previously accepted noncanonical values.
            if p == MAX_PRIME {
                assert_eq!(f.mul(p + 1, p + 2), 2);
                assert_eq!(f.add(p + 1, p + 2), 3);
                assert_eq!(f.sub(p + 1, p + 2), p - 1);
                assert_eq!(f.inv(p + 2), f.inv(2));
            }
        }
    }

    fn eval(p: u64, poly: &[u64], x: u64) -> u64 {
        poly.iter().rev().fold(0u64, |a, &b| {
            (((a as u128 * x as u128) + b as u128) % p as u128) as u64
        })
    }
    fn brute(p: u64, poly: &[u64]) -> Vec<u64> {
        (0..p).filter(|&x| eval(p, poly, x) == 0).collect()
    }
    fn lcg(s: &mut u64) -> u64 {
        *s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *s >> 33
    }

    #[test]
    fn rejects_bad_primes() {
        for p in [0u64, 1, 2, 3, 4, 9, 15, MAX_PRIME + 2] {
            assert!(RadicalRootCtx::new(p).is_err(), "{p}");
        }
    }

    #[test]
    fn cube_root_exhaustive_fp_and_fp2_both_classes() {
        use std::collections::HashSet;
        for &p in &[5u64, 7, 11, 13, 17, 19, 23, 37, 53, 109, 163] {
            let f = Fp::new(p);
            let k = Cube::new(f);
            let cubes: HashSet<u64> = (0..p).map(|x| f.mul(x, f.mul(x, x))).collect();
            for a in 0..p {
                match (k.root(a), cubes.contains(&a)) {
                    (Some(r), true) => assert_eq!(f.mul(r, f.mul(r, r)), a),
                    (None, false) => {}
                    other => panic!("Fp p={p} a={a} {other:?}"),
                }
            }
            if p <= 163 {
                let n = (2..p).find(|&z| f.powm(z, (p - 1) / 2) != 1).unwrap();
                let r = Fq2 { f, n };
                let kq = Cube::new(r);
                let cs: HashSet<(u64, u64)> = (0..p)
                    .flat_map(|x| (0..p).map(move |y| (x, y)))
                    .map(|e| r.mul(e, r.mul(e, e)))
                    .collect();
                for x in 0..p {
                    for y in 0..p {
                        let e = (x, y);
                        match (kq.root(e), cs.contains(&e)) {
                            (Some(t), true) => assert_eq!(r.mul(t, r.mul(t, t)), e),
                            (None, false) => {}
                            other => panic!("Fq2 p={p} e={e:?} {other:?}"),
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn exhaustive_quadratics_cubics_quartics_small_primes() {
        for &p in &[5u64, 7, 11, 13, 17] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            for deg in 1..=4usize {
                let n = (p as usize).pow(deg as u32 + 1);
                for code in 0..n {
                    let mut c = vec![0u64; deg + 1];
                    let mut t = code;
                    for x in c.iter_mut() {
                        *x = (t % p as usize) as u64;
                        t /= p as usize;
                    }
                    if c[deg] == 0 {
                        continue;
                    }
                    let got = ctx.roots(&c).unwrap();
                    assert_eq!(got.roots, brute(p, &c), "p={p} poly={c:?}");
                }
            }
        }
    }

    #[test]
    fn matches_prime_root_trace_on_random_large_instances() {
        let limits = || RootLimits {
            field_work: MAX_WORK,
            split_attempts: 1000,
            events: 100_000,
            degree: 8,
        };
        let mut seed = 1u64;
        let (mut fb, mut classes) = (0, [0u32; 5]);
        for &p in &[
            1_000_003u64,
            1_000_000_007,
            998_244_353,
            2_147_483_647,
            MAX_PRIME,
        ] {
            let ctx = RadicalRootCtx::new(p).unwrap();
            for it in 0..60 {
                let poly: Vec<u64> = if it % 3 == 0 {
                    // product of four random linear factors
                    let mut pl = vec![1u64];
                    for _ in 0..4 {
                        let r = lcg(&mut seed) % p;
                        let mut np = vec![0u64; pl.len() + 1];
                        for (i, &cf) in pl.iter().enumerate() {
                            np[i + 1] = (np[i + 1] + cf) % p;
                            np[i] = ((np[i] as u128 + p as u128
                                - (cf as u128 * r as u128 % p as u128))
                                % p as u128) as u64;
                        }
                        pl = np;
                    }
                    pl
                } else {
                    (0..5)
                        .map(|i| {
                            if i == 4 {
                                1 + lcg(&mut seed) % (p - 1)
                            } else {
                                lcg(&mut seed) % p
                            }
                        })
                        .collect()
                };
                let want = solve(&poly, p, 3, limits(), &mut |_| Ok(())).unwrap();
                assert_eq!(want.stop, RootStop::Complete);
                let got = ctx.roots(&poly).unwrap();
                assert_eq!(got.roots, want.roots, "p={p} poly={poly:?}");
                if got.frobenius_fallback {
                    fb += 1;
                }
                classes[want.roots.len()] += 1;
            }
        }
        assert!(
            classes[0] > 0 && classes[1] > 0 && classes[2] > 0 && classes[4] > 0,
            "{classes:?}"
        );
        assert!(fb > 0, "the (1,3) class must be exercised");
    }
}
