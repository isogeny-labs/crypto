//! Bit-packed polynomials over `F_2` with the factoring needed to split
//! `Φ_ℓ(X, j₀) mod 2` into Frobenius orbits.
//!
//! If `Φ mod 2` has all its roots in `F_{2^n}`, each irreducible factor over
//! `F_2` has degree dividing `n` and its roots are one Frobenius orbit of
//! neighbours.  Factoring over `F_2` costs word operations; factoring the same
//! polynomial over `F_{2^n}` would cost field multiplications.

use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct F2Poly {
    w: Vec<u64>,
}

impl F2Poly {
    pub fn zero() -> Self {
        F2Poly { w: vec![] }
    }
    pub fn one() -> Self {
        F2Poly { w: vec![1] }
    }
    pub fn x() -> Self {
        F2Poly { w: vec![2] }
    }
    pub fn from_bits(bits: &[bool]) -> Self {
        let mut w = vec![0u64; bits.len().div_ceil(64)];
        for (i, &b) in bits.iter().enumerate() {
            if b {
                w[i / 64] |= 1u64 << (i % 64);
            }
        }
        let mut p = F2Poly { w };
        p.trim();
        p
    }
    pub fn monomial(k: usize) -> Self {
        let mut w = vec![0u64; k / 64 + 1];
        w[k / 64] = 1u64 << (k % 64);
        F2Poly { w }
    }
    fn trim(&mut self) {
        while let Some(&l) = self.w.last() {
            if l == 0 {
                self.w.pop();
            } else {
                break;
            }
        }
    }
    pub fn is_zero(&self) -> bool {
        self.w.is_empty()
    }
    pub fn is_one(&self) -> bool {
        self.w.len() == 1 && self.w[0] == 1
    }
    /// Degree, `None` for the zero polynomial.
    pub fn deg(&self) -> Option<usize> {
        self.w
            .last()
            .map(|&l| (self.w.len() - 1) * 64 + 63 - l.leading_zeros() as usize)
    }
    pub fn coeff(&self, i: usize) -> bool {
        self.w
            .get(i / 64)
            .is_some_and(|&v| (v >> (i % 64)) & 1 == 1)
    }
    pub fn bits(&self) -> Vec<bool> {
        match self.deg() {
            None => vec![],
            Some(d) => (0..=d).map(|i| self.coeff(i)).collect(),
        }
    }
    pub fn add(&self, o: &Self) -> Self {
        let n = self.w.len().max(o.w.len());
        let mut w = vec![0u64; n];
        for (i, v) in w.iter_mut().enumerate() {
            *v = self.w.get(i).copied().unwrap_or(0) ^ o.w.get(i).copied().unwrap_or(0);
        }
        let mut p = F2Poly { w };
        p.trim();
        p
    }
    fn xor_shifted(&mut self, o: &Self, sh: usize) {
        let (ws, bs) = (sh / 64, sh % 64);
        let need = o.w.len() + ws + 1;
        if self.w.len() < need {
            self.w.resize(need, 0);
        }
        for (i, &v) in o.w.iter().enumerate() {
            self.w[i + ws] ^= v << bs;
            if bs != 0 {
                self.w[i + ws + 1] ^= v >> (64 - bs);
            }
        }
    }
    pub fn shl(&self, sh: usize) -> Self {
        let mut r = F2Poly { w: vec![] };
        r.xor_shifted(self, sh);
        r.trim();
        r
    }
    pub fn mul(&self, o: &Self) -> Self {
        if self.is_zero() || o.is_zero() {
            return Self::zero();
        }
        let (a, b) = if self.deg() >= o.deg() {
            (self, o)
        } else {
            (o, self)
        };
        let mut r = F2Poly {
            w: vec![0; a.w.len() + b.w.len() + 1],
        };
        for (wi, &word) in b.w.iter().enumerate() {
            let mut v = word;
            while v != 0 {
                let t = v.trailing_zeros() as usize;
                r.xor_shifted(a, wi * 64 + t);
                v &= v - 1;
            }
        }
        r.trim();
        r
    }
    pub fn square(&self) -> Self {
        let mut w = vec![0u64; self.w.len() * 2];
        for (i, &v) in self.w.iter().enumerate() {
            w[2 * i] = spread(v as u32 as u64);
            w[2 * i + 1] = spread(v >> 32);
        }
        let mut p = F2Poly { w };
        p.trim();
        p
    }
    /// `(quotient, remainder)` of division by a nonzero polynomial.
    pub fn divrem(&self, f: &Self) -> (Self, Self) {
        let df = f.deg().expect("division by zero polynomial");
        let mut r = self.clone();
        let mut q = F2Poly {
            w: vec![0; self.w.len() + 1],
        };
        while let Some(dr) = r.deg() {
            if dr < df {
                break;
            }
            let sh = dr - df;
            r.xor_shifted(f, sh);
            r.trim();
            q.w[sh / 64] |= 1u64 << (sh % 64);
        }
        q.trim();
        (q, r)
    }
    pub fn rem(&self, f: &Self) -> Self {
        let df = f.deg().expect("division by zero polynomial");
        let mut r = self.clone();
        while let Some(dr) = r.deg() {
            if dr < df {
                break;
            }
            r.xor_shifted(f, dr - df);
            r.trim();
        }
        r
    }
    pub fn gcd(&self, o: &Self) -> Self {
        let (mut a, mut b) = (self.clone(), o.clone());
        while !b.is_zero() {
            let r = a.rem(&b);
            a = b;
            b = r;
        }
        a
    }
    pub fn derivative(&self) -> Self {
        let mut w = vec![0u64; self.w.len()];
        for (i, v) in w.iter_mut().enumerate() {
            // keep odd-position coefficients, shifted down by one
            let odd = self.w[i] & 0xAAAA_AAAA_AAAA_AAAA;
            *v = odd >> 1;
        }
        let mut p = F2Poly { w };
        p.trim();
        p
    }
    /// If every exponent is even, the polynomial `g` with `g(x)² = self`.
    pub fn sqrt_if_square(&self) -> Option<Self> {
        let d = self.deg()?;
        let mut bits = vec![false; d / 2 + 1];
        for i in 0..=d {
            if self.coeff(i) {
                if i % 2 == 1 {
                    return None;
                }
                bits[i / 2] = true;
            }
        }
        Some(Self::from_bits(&bits))
    }
    pub fn cmp_key(&self, o: &Self) -> Ordering {
        (self.deg(), &self.w).cmp(&(o.deg(), &o.w))
    }
}

fn spread(mut x: u64) -> u64 {
    x = (x | (x << 16)) & 0x0000_FFFF_0000_FFFF;
    x = (x | (x << 8)) & 0x00FF_00FF_00FF_00FF;
    x = (x | (x << 4)) & 0x0F0F_0F0F_0F0F_0F0F;
    x = (x | (x << 2)) & 0x3333_3333_3333_3333;
    x = (x | (x << 1)) & 0x5555_5555_5555_5555;
    x
}

/// Squarefree decomposition: `f = ∏ g_i^{e_i}` with each `g_i` squarefree
/// (not necessarily irreducible), returned as `(g_i, e_i)`.
pub fn squarefree(f: &F2Poly) -> Vec<(F2Poly, u64)> {
    fn rec(f: &F2Poly, mult: u64, out: &mut Vec<(F2Poly, u64)>) {
        if f.deg().unwrap_or(0) == 0 {
            return;
        }
        let d = f.derivative();
        if d.is_zero() {
            let g = f
                .sqrt_if_square()
                .expect("zero derivative implies a square in characteristic 2");
            rec(&g, mult * 2, out);
            return;
        }
        let mut c = f.gcd(&d);
        let mut w = f.divrem(&c).0;
        let mut i = 1u64;
        while w.deg().unwrap_or(0) > 0 {
            let y = w.gcd(&c);
            let z = w.divrem(&y).0;
            if z.deg().unwrap_or(0) > 0 {
                out.push((z, i * mult));
            }
            w = y.clone();
            c = c.divrem(&y).0;
            i += 1;
        }
        if c.deg().unwrap_or(0) > 0 {
            rec(
                &c.sqrt_if_square().expect("remaining part is a square"),
                mult * 2,
                out,
            );
        }
    }
    let mut out = Vec::new();
    rec(f, 1, &mut out);
    out
}

/// Distinct-degree factorisation of a squarefree polynomial: `(product of the
/// irreducible factors of degree d, d)` for every `d ≤ max_deg`; whatever is
/// left (factors of degree `> max_deg`) is returned last with degree 0.
pub fn ddf(f: &F2Poly, max_deg: usize) -> Vec<(F2Poly, usize)> {
    let mut out = Vec::new();
    let mut rest = f.clone();
    let mut h = F2Poly::x();
    let mut d = 0usize;
    while let Some(dr) = rest.deg() {
        if dr == 0 {
            break;
        }
        d += 1;
        if d > max_deg || 2 * d > dr {
            break;
        }
        h = h.square().rem(&rest);
        let g = rest.gcd(&h.add(&F2Poly::x()));
        if g.deg().unwrap_or(0) > 0 {
            rest = rest.divrem(&g).0;
            h = h.rem(&rest);
            out.push((g, d));
        }
    }
    if let Some(dr) = rest.deg() {
        if dr > 0 {
            // everything left is irreducible of degree dr if 2*d > dr was the stop reason
            if 2 * d > dr && d <= max_deg + 1 {
                out.push((rest.clone(), dr));
            } else {
                out.push((rest, 0));
            }
        }
    }
    out
}

/// Equal-degree splitting of `g`, a product of irreducibles all of degree `d`
/// (Cantor–Zassenhaus with the trace map; `rng` is a plain counter-based mixer).
pub fn edf(g: &F2Poly, d: usize, rng: &mut u64) -> Vec<F2Poly> {
    let dg = g.deg().unwrap_or(0);
    if dg == d {
        return vec![g.clone()];
    }
    loop {
        // random a of degree < dg
        let mut bits = vec![false; dg];
        for b in bits.iter_mut() {
            *rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *b = (*rng >> 33) & 1 == 1;
        }
        let a = F2Poly::from_bits(&bits).rem(g);
        let mut t = a.clone();
        let mut cur = a;
        for _ in 1..d {
            cur = cur.square().rem(g);
            t = t.add(&cur);
        }
        let h = g.gcd(&t);
        let dh = h.deg().unwrap_or(0);
        if dh > 0 && dh < dg {
            let other = g.divrem(&h).0;
            let mut v = edf(&h, d, rng);
            v.extend(edf(&other, d, rng));
            return v;
        }
    }
}

/// All irreducible factors over `F_2` of `f` with degree dividing `n`, with
/// multiplicities; `leftover_deg` is the total degree (with multiplicity) of
/// factors whose degree does not divide `n` (their roots are not in `F_{2^n}`).
pub struct RationalFactors {
    pub factors: Vec<(F2Poly, usize, u64)>,
    pub leftover_deg: usize,
}

pub fn rational_factors(f: &F2Poly, n: usize) -> RationalFactors {
    let mut rng = 0x1234_5678_9ABC_DEF1u64;
    let mut factors = Vec::new();
    let mut leftover = 0usize;
    for (g, mult) in squarefree(f) {
        for (prod, d) in ddf(&g, n) {
            if d != 0 && n % d == 0 {
                for irr in edf(&prod, d, &mut rng) {
                    factors.push((irr, d, mult));
                }
            } else {
                leftover += prod.deg().unwrap_or(0) * mult as usize;
            }
        }
    }
    factors.sort_by(|a, b| a.0.cmp_key(&b.0));
    RationalFactors {
        factors,
        leftover_deg: leftover,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(exps: &[usize]) -> F2Poly {
        let mut r = F2Poly::zero();
        for &e in exps {
            r = r.add(&F2Poly::monomial(e));
        }
        r
    }

    #[test]
    fn mul_div_roundtrip() {
        let a = p(&[0, 1, 5, 70, 130]);
        let b = p(&[0, 3, 64, 90]);
        let ab = a.mul(&b);
        let (q, r) = ab.divrem(&b);
        assert!(r.is_zero());
        assert_eq!(q, a);
        assert_eq!(a.square(), a.mul(&a));
    }

    #[test]
    fn gcd_and_derivative() {
        let a = p(&[0, 1, 2]); // x^2+x+1, irreducible
        let b = p(&[0, 1, 3]); // x^3+x+1, irreducible
        let ab = a.mul(&b);
        assert_eq!(ab.gcd(&a), a);
        assert!(a.gcd(&b).is_one());
        // d/dx (x^3 + x) = x^2 + 1 over F_2
        assert_eq!(p(&[1, 3]).derivative(), p(&[0, 2]));
    }

    #[test]
    fn squarefree_and_factor_with_multiplicity() {
        // (x+1)^2 (x^2+x+1) (x^3+x+1)^3, plus a degree-4 irreducible x^4+x+1
        let l = p(&[0, 1]);
        let a = p(&[0, 1, 2]);
        let b = p(&[0, 1, 3]);
        let c = p(&[0, 1, 4]);
        let f = l.mul(&l).mul(&a).mul(&b).mul(&b).mul(&b).mul(&c);
        let rf = rational_factors(&f, 12);
        let mut got: Vec<(usize, u64)> = rf.factors.iter().map(|(_, d, m)| (*d, *m)).collect();
        got.sort();
        assert_eq!(got, vec![(1, 2), (2, 1), (3, 3), (4, 1)]);
        assert_eq!(rf.leftover_deg, 0);
        // product reconstructs f
        let mut prod = F2Poly::one();
        for (g, _, m) in &rf.factors {
            for _ in 0..*m {
                prod = prod.mul(g);
            }
        }
        assert_eq!(prod, f);
    }

    #[test]
    fn non_dividing_degrees_are_reported_as_leftover() {
        let b = p(&[0, 1, 3]); // degree 3: not rational over F_{2^4}
        let c = p(&[0, 1, 4]); // degree 4
        let rf = rational_factors(&b.mul(&c), 4);
        assert_eq!(rf.factors.len(), 1);
        assert_eq!(rf.leftover_deg, 3);
    }
}
