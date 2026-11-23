//! `transfer.isogeny`: move a prime-curve discrete logarithm across a
//! rational `ℓ`-isogeny, solve it on the isogenous curve, and check the
//! answer on the original.
//!
//! For `E : y² = x³ + ax + b` over `F_p` and a rational kernel
//! polynomial `h(x) = x − x₀` (`ℓ = 2`: `x₀` a root of `x³ + ax + b`;
//! `ℓ = 3`: a root of the 3-division polynomial `ψ₃ = 3x⁴ + 6ax² + 12bx −
//! a²`), Vélu's formulas (Washington, *Elliptic Curves*, Thm. 12.16) with
//! `g^x = 3x₀² + a`, `v = g^x` (`ℓ = 2`) or `2g^x` (`ℓ = 3`), `u = 4y₀² =
//! 4(x₀³ + ax₀ + b)` give
//!
//! ```text
//!   E′ : y² = x³ + (a − 5v)x + (b − 7(u + x₀v))
//!   φ(x, y) = ( x + v/(x − x₀) + u/(x − x₀)²,  y·(1 − 2u/(x − x₀)³ − v/(x − x₀)²) )
//! ```
//!
//! all over `F_p` even when the kernel points themselves are not (their
//! ordinates enter only through `y₀² ∈ F_p`).  `φ` is a homomorphism of
//! degree `ℓ`, so for `ℓ ≠ r` it maps `⟨G⟩` isomorphically onto the order-`r`
//! subgroup of `E′` and `Q = [k]G` becomes `φ(Q) = [k]φ(G)`.
//!
//! What is charged.  The isogenous curve is a different curve, but over
//! the same field and with the same group law, so its group operations
//! are GAE in the workload's own unit: the inner solve on `E′` is charged
//! as it would be on any curve, and the final check `[k]G = Q` on `E` is
//! one scalar multiplication.  Finding the kernel (`x^p mod ψ`, a gcd, a
//! root split) and evaluating `φ` twice are field operations, recorded as
//! native work in a `transfer` phase and left unpriced (AGENTS.md §5a
//! prices them across the band); they are `O(log p)` multiplications
//! against the solve's `O(√r)` group operations.

use crate::cryptanalysis::ic_boundary::{PrimeCurve, PrimePoint};

/// Field operations the transfer spends outside the group law.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FieldTally {
    pub muls: u64,
    pub invs: u64,
}

struct Fp<'t> {
    p: u64,
    tally: &'t mut FieldTally,
}

impl Fp<'_> {
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.tally.muls += 1;
        // Root-polynomial arithmetic uses canonical residues. Keep the
        // original wide remainder for external/noncanonical operands.
        if a < self.p && b < self.p {
            if self.p <= u16::MAX as u64 {
                return ((a as u32 * b as u32) % self.p as u32) as u64;
            }
            if self.p <= u32::MAX as u64 {
                return (a * b) % self.p;
            }
            // Fast reduction for p=2^61-1 (two end-around-carry limbs).
            if self.p == (1u64 << 61) - 1 {
                let product = a as u128 * b as u128;
                let folded = ((product as u64) & self.p) + (product >> 61) as u64;
                return if folded >= self.p {
                    folded - self.p
                } else {
                    folded
                };
            }
        }
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    fn add(&self, a: u64, b: u64) -> u64 {
        // p<=2^63-1 and canonical operands imply the sum fits in u64.
        if self.p <= u64::MAX / 2 && a < self.p && b < self.p {
            let sum = a + b;
            return if sum >= self.p { sum - self.p } else { sum };
        }
        ((a as u128 + b as u128) % self.p as u128) as u64
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        if self.p <= u64::MAX / 2 && a < self.p && b < self.p {
            return if a >= b { a - b } else { a + self.p - b };
        }
        ((a as u128 + self.p as u128 - b as u128) % self.p as u128) as u64
    }
    fn small(&self, c: u64) -> u64 {
        c % self.p
    }
    fn pow(&mut self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1 % self.p;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            e >>= 1;
            // Do not square the base after consuming the final exponent bit.
            if e != 0 {
                a = self.mul(a, a);
            }
        }
        r
    }
    fn inv(&mut self, a: u64) -> u64 {
        self.tally.invs += 1;
        // Fermat, counted as one inversion (not as its multiplications).
        let mut scratch = FieldTally::default();
        let mut f = Fp {
            p: self.p,
            tally: &mut scratch,
        };
        f.pow(a, self.p - 2)
    }

    // Polynomials, lowest degree first.
    fn trim(v: &mut Vec<u64>) {
        while v.last() == Some(&0) {
            v.pop();
        }
    }

    // Root extraction repeatedly reduces modulo the same polynomial.
    // Convert it to monic once so each subsequent remainder avoids a
    // full Fermat inversion of the leading coefficient. The zero set is
    // unchanged because the scaling factor is nonzero in F_p.
    fn make_monic(&mut self, polynomial: &mut Vec<u64>) {
        if polynomial.len() <= 1 || polynomial.last() == Some(&1) {
            return;
        }
        let degree = polynomial.len() - 1;
        let inverse = self.inv(polynomial[degree]);
        for coefficient in &mut polynomial[..degree] {
            *coefficient = self.mul(*coefficient, inverse);
        }
        polynomial[degree] = 1;
    }

    fn rem(&mut self, a: &[u64], m: &[u64]) -> Vec<u64> {
        self.rem_owned(a.to_vec(), m)
    }

    // Polynomial multiplication already owns its coefficient buffer. Reduce
    // it in-place rather than cloning a second Vec for each powmod step.
    fn rem_owned(&mut self, mut r: Vec<u64>, m: &[u64]) -> Vec<u64> {
        Self::trim(&mut r);
        let dm = m.len() - 1;
        // Monic moduli dominate x^p mod f and Cantor-Zassenhaus splitting.
        // Their leading inverse is 1: avoid a Fermat inversion per remainder.
        let monic = m[dm] == 1;
        let inv = if monic { 1 } else { self.inv(m[dm]) };
        while r.len() > dm {
            let top = *r.last().unwrap();
            let c = if monic && top < self.p {
                top
            } else {
                self.mul(top, inv)
            };
            let s = r.len() - 1 - dm;
            // The leading coefficient cancels exactly, so do not multiply
            // and subtract it. Skip zero terms in sparse divisors.
            for (i, &mi) in m[..dm].iter().enumerate() {
                if mi != 0 {
                    let t = self.mul(c, mi);
                    r[s + i] = self.sub(r[s + i], t);
                }
            }
            r[s + dm] = 0;
            Self::trim(&mut r);
        }
        r
    }
    fn mulmod(&mut self, a: &[u64], b: &[u64], m: &[u64]) -> Vec<u64> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut t = vec![0u64; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                let v = self.mul(x, y);
                t[i + j] = self.add(t[i + j], v);
            }
        }
        self.rem_owned(t, m)
    }
    fn powmod(&mut self, base: &[u64], mut e: u64, m: &[u64]) -> Vec<u64> {
        let mut r = vec![1];
        let mut b = self.rem(base, m);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mulmod(&r, &b, m);
            }
            e >>= 1;
            // An unused final polynomial square is especially expensive.
            if e != 0 {
                b = self.mulmod(&b, &b, m);
            }
        }
        r
    }
    fn gcd(&mut self, a: &[u64], b: &[u64]) -> Vec<u64> {
        let mut a = a.to_vec();
        let mut b = b.to_vec();
        Self::trim(&mut a);
        Self::trim(&mut b);
        while !b.is_empty() {
            let r = self.rem(&a, &b);
            a = b;
            b = r;
        }
        a
    }

    /// Every root in `F_p` of `f`, ascending: `g = gcd(f, x^p − x)`, then
    /// equal-degree splitting with `(x + δ)^((p−1)/2) − 1` for `δ = 0, 1, …`
    /// (deterministic).
    fn roots(&mut self, f: &[u64]) -> Vec<u64> {
        let p = self.p;
        let mut f = f.to_vec();
        Self::trim(&mut f);
        if f.len() <= 1 {
            return vec![];
        }
        self.make_monic(&mut f);
        let xp = self.powmod(&[0, 1], p, &f);
        let mut h = xp;
        if h.len() < 2 {
            h.resize(2, 0);
        }
        h[1] = self.sub(h[1], 1);
        Self::trim(&mut h);
        let mut g = if h.is_empty() {
            f.clone()
        } else {
            self.gcd(&f, &h)
        };
        // Euclidean gcd does not necessarily return a monic polynomial.
        // Normalize before all subsequent equal-degree splitting.
        self.make_monic(&mut g);
        let mut out = Vec::new();
        self.split(g, &mut out, 0);
        out.sort_unstable();
        out.dedup();
        out
    }
    fn split(&mut self, g: Vec<u64>, out: &mut Vec<u64>, mut delta: u64) {
        let p = self.p;
        match g.len() {
            0 | 1 => {}
            2 => {
                // Every factor emitted by roots() is monic. Preserve a
                // fallback for nonmonic callers of this private helper.
                let neg = self.sub(0, g[0]);
                let root = if g[1] == 1 {
                    neg
                } else {
                    let inv = self.inv(g[1]);
                    self.mul(neg, inv)
                };
                out.push(root);
            }
            _ => loop {
                let shift = self.small(delta);
                let e = self.powmod(&[shift, 1], (p - 1) / 2, &g);
                let mut e = e;
                if e.is_empty() {
                    e = vec![0];
                }
                e[0] = self.sub(e[0], 1);
                let mut d = self.gcd(&g, &e);
                delta += 1;
                if d.len() > 1 && d.len() < g.len() {
                    // GCD factors are only defined up to a field scalar.
                    // Make them monic before division and recursion: this
                    // eliminates repeated inversions within both branches.
                    self.make_monic(&mut d);
                    let other = {
                        // Exact division g / d, where d is now monic.
                        let mut q = vec![0u64; g.len() - d.len() + 1];
                        let mut r = g.clone();
                        while r.len() >= d.len() && !r.is_empty() {
                            let c = *r.last().unwrap();
                            let s = r.len() - d.len();
                            q[s] = c;
                            for (i, &di) in d[..d.len() - 1].iter().enumerate() {
                                if di != 0 {
                                    let t = self.mul(c, di);
                                    r[s + i] = self.sub(r[s + i], t);
                                }
                            }
                            r[s + d.len() - 1] = 0;
                            Self::trim(&mut r);
                        }
                        q
                    };
                    self.split(d, out, delta);
                    self.split(other, out, delta);
                    return;
                }
                if delta > 4 * 64 + p.min(1 << 20) {
                    return;
                }
            },
        }
    }
}

/// A rational `ℓ`-isogeny `φ : E → E′` from a degree-1 kernel polynomial.
#[derive(Clone, Debug)]
pub struct Isogeny {
    pub ell: u64,
    pub x0: u64,
    pub u: u64,
    pub v: u64,
    pub source: PrimeCurve,
    pub target: PrimeCurve,
}

/// The rational `ℓ`-isogeny (`ℓ ∈ {2, 3}`) with the smallest kernel
/// abscissa, or `None` when `E` has none.
pub fn find_isogeny(e: &PrimeCurve, ell: u64, tally: &mut FieldTally) -> Option<Isogeny> {
    let p = e.p;
    let mut f = Fp { p, tally };
    let (a, b) = (e.a % p, e.b % p);
    let psi: Vec<u64> = match ell {
        2 => vec![b, a, 0, 1],
        3 => {
            let a2 = f.mul(a, a);
            let c12 = f.small(12);
            let c6 = f.small(6);
            let t1 = f.mul(c12, b);
            let t2 = f.mul(c6, a);
            vec![f.sub(0, a2), t1, t2, 0, f.small(3)]
        }
        _ => return None,
    };
    let x0 = *f.roots(&psi).first()?;
    let x02 = f.mul(x0, x0);
    let c3 = f.small(3);
    let t3 = f.mul(c3, x02);
    let gx = f.add(t3, a);
    let v = if ell == 2 { gx } else { f.mul(2, gx) };
    let y0sq = {
        let t = f.mul(x02, x0);
        let ax = f.mul(a, x0);
        f.add(f.add(t, ax), b)
    };
    let u = if ell == 2 { 0 } else { f.mul(4, y0sq) };
    let v5 = f.mul(5 % p, v);
    let a1 = f.sub(a, v5);
    let x0v = f.mul(x0, v);
    let w = f.add(u, x0v);
    let w7 = f.mul(7 % p, w);
    let b1 = f.sub(b, w7);
    let target = PrimeCurve { p, a: a1, b: b1 };
    // Singular target: the polynomial was not a kernel (cannot happen for
    // a genuine root, but the transfer refuses rather than trusts).
    let disc = {
        let a1sq = f.mul(a1, a1);
        let a3 = f.mul(a1sq, a1);
        let b2 = f.mul(b1, b1);
        let l = f.mul(4 % p, a3);
        let r = f.mul(27 % p, b2);
        f.add(l, r)
    };
    if disc == 0 {
        return None;
    }
    Some(Isogeny {
        ell,
        x0,
        u,
        v,
        source: e.clone(),
        target,
    })
}

impl Isogeny {
    /// `φ(P)`; the identity for the identity and for a kernel point.
    pub fn map(&self, pt: PrimePoint, tally: &mut FieldTally) -> PrimePoint {
        if pt.infinity {
            return pt;
        }
        let p = self.source.p;
        let mut f = Fp { p, tally };
        let d = f.sub(pt.x, self.x0);
        if d == 0 {
            return PrimePoint::INFINITY;
        }
        let di = f.inv(d);
        let di2 = f.mul(di, di);
        let di3 = f.mul(di2, di);
        let vdi = f.mul(self.v, di);
        let udi2 = f.mul(self.u, di2);
        let x = f.add(f.add(pt.x, vdi), udi2);
        let u2 = f.mul(2 % p, self.u);
        let a = f.mul(u2, di3);
        let bb = f.mul(self.v, di2);
        let t = f.add(a, bb);
        let one_t = f.sub(1, t);
        let y = f.mul(pt.y, one_t);
        PrimePoint::affine(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::{find_prime_order_curve, CountedGroup, GroupOps};

    #[test]
    fn round128_arithmetic_matches_wide_modulo() {
        let primes = [101u64, 65_521, 1_000_000_007, 4_294_967_291, (1u64 << 61) - 1];
        for p in primes {
            let mut tally = FieldTally::default();
            let mut f = Fp { p, tally: &mut tally };
            let mut state = p ^ 0x9e37_79b9_7f4a_7c15;
            for _ in 0..20_000 {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let a = state % p;
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let b = state % p;
                assert_eq!(f.mul(a, b), (a as u128 * b as u128 % p as u128) as u64);
                assert_eq!(f.add(a, b), ((a as u128 + b as u128) % p as u128) as u64);
                assert_eq!(f.sub(a, b), ((a as u128 + p as u128 - b as u128) % p as u128) as u64);
            }
        }
    }

    #[test]
    fn round128_remainders_and_powers_match_naive_division() {
        fn old_rem(f: &mut Fp<'_>, a: &[u64], m: &[u64]) -> Vec<u64> {
            let mut r = a.to_vec();
            Fp::trim(&mut r);
            let dm = m.len() - 1;
            let inv = f.inv(m[dm]);
            while r.len() > dm {
                let c = f.mul(*r.last().unwrap(), inv);
                let shift = r.len() - 1 - dm;
                for (i, &mi) in m.iter().enumerate() {
                    let t = f.mul(c, mi);
                    r[shift + i] = f.sub(r[shift + i], t);
                }
                Fp::trim(&mut r);
            }
            r
        }
        for p in [101u64, 65_521, 1_000_000_007, (1u64 << 61) - 1] {
            let mut tally = FieldTally::default();
            let mut f = Fp { p, tally: &mut tally };
            let mut state = p ^ 0xabc1_2500_0000_0028;
            for i in 0..250 {
                let mut dividend = Vec::new();
                for _ in 0..8 {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    dividend.push(state % p);
                }
                let modulus = match i % 4 {
                    0 => vec![3, 0, 0, 1],
                    1 => vec![3, 2, 4, 1],
                    2 => vec![3, 0, 0, 2],
                    _ => vec![3, 2, 4, 7],
                };
                let got = f.rem(&dividend, &modulus);
                let expected = old_rem(&mut f, &dividend, &modulus);
                assert_eq!(got, expected);
                if i < 15 {
                    let optimized = f.powmod(&[1, 1], p, &modulus);
                    // Independent square-and-multiply with the original reducer.
                    let mut exponent = p;
                    let mut acc = vec![1];
                    let mut base = old_rem(&mut f, &[1, 1], &modulus);
                    while exponent > 0 {
                        if exponent & 1 == 1 {
                            let product = f.mulmod(&acc, &base, &modulus);
                            acc = old_rem(&mut f, &product, &modulus);
                        }
                        exponent >>= 1;
                        if exponent != 0 {
                            let square = f.mulmod(&base, &base, &modulus);
                            base = old_rem(&mut f, &square, &modulus);
                        }
                    }
                    assert_eq!(optimized, acc);
                }
            }
        }
    }

    /// On prime-order curves with a rational 3-isogeny, φ lands on E′, is a
    /// homomorphism on ⟨G⟩, and keeps the order.
    #[test]
    fn a_three_isogeny_is_a_homomorphism_onto_the_target() {
        let mut found = 0;
        for seed in 0..60u64 {
            let inst = find_prime_order_curve(16, seed);
            let mut t = FieldTally::default();
            let Some(phi) = find_isogeny(&inst.curve, 3, &mut t) else {
                continue;
            };
            found += 1;
            assert!(t.muls > 0);
            let g = inst.generator_point();
            let mut ops = GroupOps::default();
            let e = &inst.curve;
            let e1 = &phi.target;
            let pg = phi.map(g, &mut t);
            assert!(e1.is_on_curve(pg), "seed {seed}: φ(G) is on E′");
            assert!(!pg.infinity);
            for k in [2u64, 3, 17, inst.r - 1, inst.r / 2 + 5] {
                let kg = e.mul(&mut ops, g, k);
                assert_eq!(
                    phi.map(kg, &mut t),
                    e1.mul(&mut ops, pg, k),
                    "seed {seed}, k {k}"
                );
            }
            assert!(e1.mul(&mut ops, pg, inst.r).infinity);
        }
        assert!(
            found >= 5,
            "only {found} curves of 60 had a rational 3-isogeny"
        );
    }

    #[test]
    fn roots_are_every_rational_root() {
        let p = 101u64;
        let mut t = FieldTally::default();
        let mut f = Fp { p, tally: &mut t };
        // (x − 3)(x − 50)(x² + 2) (x² + 2 irreducible mod 101: −2 is a non-residue).
        let poly = {
            let a = [p - 3, 1];
            let b = [p - 50, 1];
            let c = [2, 0, 1];
            let ab = f.mulmod(&a, &b, &[0, 0, 0, 0, 0, 0, 1]);
            f.mulmod(&ab, &c, &[0, 0, 0, 0, 0, 0, 1])
        };
        assert_eq!(f.roots(&poly), vec![3, 50]);
    }

    #[test]
    fn round129_monic_factor_roots_match_known_factors() {
        for p in [101u64, 65_521, 1_000_000_007, 4_294_967_291, (1u64 << 61) - 1] {
            let mut tally = FieldTally::default();
            let mut field = Fp { p, tally: &mut tally };
            for i in 0..24u64 {
                let factors = [(3 + 7 * i) % p, (13 + 7 * i) % p,
                    (29 + 7 * i) % p, (47 + 7 * i) % p];
                let mut polynomial = vec![1u64];
                for &root in &factors {
                    let mut next = vec![0u64; polynomial.len() + 1];
                    for (j, &coefficient) in polynomial.iter().enumerate() {
                        let term = field.mul(coefficient, root);
                        next[j] = field.sub(next[j], term);
                        next[j + 1] = field.add(next[j + 1], coefficient);
                    }
                    polynomial = next;
                }
                let mut expected = factors.to_vec();
                expected.sort_unstable();
                expected.dedup();
                assert_eq!(field.roots(&polynomial), expected, "monic p={p} i={i}");
                for scalar in [2u64, 3, 7] {
                    let scaled: Vec<u64> = polynomial.iter()
                        .map(|&coefficient| field.mul(coefficient, scalar))
                        .collect();
                    assert_eq!(field.roots(&scaled), expected, "scaled p={p} i={i}");
                }
            }
        }
    }

    #[test]
    fn round129_isogeny_psi3_roots_match_small_field_exhaustive_search() {
        let p = 101u64;
        let mut tally = FieldTally::default();
        let mut field = Fp { p, tally: &mut tally };
        for a in 0..12u64 {
            for b in 0..12u64 {
                // ψ3(x) = 3 x^4 + 6 a x^2 + 12 b x - a^2.
                let aa = field.mul(a, a);
                let poly = vec![field.sub(0, aa), field.mul(12, b),
                    field.mul(6, a), 0, 3];
                let roots = field.roots(&poly);
                let brute_force: Vec<u64> = (0..p).filter(|&x| {
                    poly.iter().rev().fold(0u64, |acc, &c| {
                        let prod = field.mul(acc, x);
                        field.add(prod, c)
                    }) == 0
                }).collect();
                assert_eq!(roots, brute_force, "ψ3 a={a} b={b}");
            }
        }
    }

}
