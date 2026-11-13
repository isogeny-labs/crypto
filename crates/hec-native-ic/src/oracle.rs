//! Smoothness oracle: does a monic `u` (degree `≤ 4`) split into linear
//! factors over `F_p`, and if so, at which roots and with which multiplicity?
//!
//! The same decision procedure as the BigUint engine's `SmoothnessTest::Gcd`
//! with round six's parity filter:
//!
//! - degree 1: the root;
//! - degree 2: one Legendre symbol on the discriminant, then the quadratic
//!   formula;
//! - degree ≥ 3: if `disc(u)` is a non-square, Frobenius acts as an odd
//!   permutation on the roots and `u` cannot split (rejects half of all
//!   candidates for one resultant and one Euler exponentiation).  Otherwise
//!   `w = gcd(u, x^p − x)`, the product of the distinct linear factors; its
//!   roots by equal-degree splitting, and multiplicities by division.

use crate::fp::Fp;
use crate::poly::Poly;

/// Roots with multiplicity; at most 4 distinct.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Roots {
    pub r: [(u64, u32); 4],
    pub len: usize,
}

impl Roots {
    #[inline]
    fn push(&mut self, x: u64, m: u32) {
        self.r[self.len] = (x, m);
        self.len += 1;
    }

    pub fn as_slice(&self) -> &[(u64, u32)] {
        &self.r[..self.len]
    }

    fn sort(&mut self) {
        self.r[..self.len].sort_unstable();
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct OracleStats {
    pub calls: u64,
    pub parity_rejects: u64,
    pub frobenius: u64,
    pub smooth: u64,
}

/// `Some(roots)` iff `u` splits completely (sorted by root).
pub fn split(u: &Poly, fp: &Fp, stats: &mut OracleStats) -> Option<Roots> {
    stats.calls += 1;
    debug_assert_eq!(u.lc(), 1);
    let d = u.deg();
    let mut out = Roots::default();
    match d {
        d if d <= 0 => {
            stats.smooth += 1;
            return Some(out);
        }
        1 => {
            out.push(fp.neg(u.c[0]), 1);
            stats.smooth += 1;
            return Some(out);
        }
        2 => {
            let (b, c) = (u.c[1], u.c[0]);
            let disc = fp.sub(fp.mul(b, b), fp.mul(4, c));
            let half = fp.inv(2);
            if disc == 0 {
                out.push(fp.mul(fp.neg(b), half), 2);
            } else {
                let s = fp.sqrt(disc)?;
                let r1 = fp.mul(fp.sub(s, b), half);
                let r2 = fp.mul(fp.sub(fp.neg(s), b), half);
                out.push(r1, 1);
                out.push(r2, 1);
                out.sort();
            }
            stats.smooth += 1;
            return Some(out);
        }
        _ => {}
    }
    // Degree 3 or 4.
    let disc = u.disc_monic(fp);
    if disc != 0 && fp.legendre(disc) == -1 {
        stats.parity_rejects += 1;
        return None;
    }
    stats.frobenius += 1;
    let xp = Poly::x().powmod(fp.p, u, fp);
    let w = Poly::gcd(u, &xp.sub(&Poly::x(), fp), fp);
    if disc != 0 && w.deg() < d {
        return None;
    }
    if w.deg() <= 0 {
        return None;
    }
    // Distinct roots of w, then multiplicities in u.
    let mut roots = [0u64; 4];
    let mut nr = 0usize;
    edf(&w, fp, &mut roots, &mut nr);
    let mut rest = *u;
    let mut total = 0isize;
    for &x0 in &roots[..nr] {
        let lin = Poly::x_minus(x0, fp);
        let mut m = 0u32;
        loop {
            let (q, r) = rest.divrem(&lin, fp);
            if !r.is_zero() {
                break;
            }
            rest = q;
            m += 1;
        }
        out.push(x0, m);
        total += m as isize;
    }
    if total != d {
        return None;
    }
    out.sort();
    stats.smooth += 1;
    Some(out)
}

/// Equal-degree splitting of a monic product of distinct linear factors.
fn edf(w: &Poly, fp: &Fp, roots: &mut [u64; 4], nr: &mut usize) {
    match w.deg() {
        d if d <= 0 => {}
        1 => {
            roots[*nr] = fp.neg(w.c[0]);
            *nr += 1;
        }
        2 => {
            let (b, c) = (w.c[1], w.c[0]);
            let disc = fp.sub(fp.mul(b, b), fp.mul(4, c));
            let s = fp.sqrt(disc).expect("edf: split quadratic has square disc");
            let half = fp.inv(2);
            roots[*nr] = fp.mul(fp.sub(s, b), half);
            roots[*nr + 1] = fp.mul(fp.sub(fp.neg(s), b), half);
            *nr += 2;
        }
        _ => {
            // gcd(w, (x + a)^((p−1)/2) − 1) for a = 1, 2, …: each a splits
            // w nontrivially with probability about 1 − 2^{1−deg w}.
            let e = (fp.p - 1) / 2;
            let mut a = 1u64;
            loop {
                let base = Poly::from_coeffs(&[a % fp.p, 1]);
                let h = base.powmod(e, w, fp).sub(&Poly::one(), fp);
                let g = Poly::gcd(w, &h, fp);
                if g.deg() > 0 && g.deg() < w.deg() {
                    let q = w.div_exact(&g, fp);
                    edf(&g, fp, roots, nr);
                    edf(&q, fp, roots, nr);
                    return;
                }
                a += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// G2: exhaustive against brute-force evaluation, every monic
    /// polynomial of degree 1..=4 (degree 4 sampled at p = 17), at
    /// p ≡ 1 and p ≡ 3 (mod 4).
    #[test]
    fn oracle_matches_bruteforce() {
        for &p in &[13u64, 11, 17, 19] {
            let fp = Fp::new(p);
            for d in 1..=4usize {
                let total = p.pow(d as u32);
                let step = if total > 50_000 { 7 } else { 1 };
                let mut idx = 0u64;
                while idx < total {
                    let mut cs = vec![0u64; d + 1];
                    let mut t = idx;
                    for c in cs.iter_mut().take(d) {
                        *c = t % p;
                        t /= p;
                    }
                    cs[d] = 1;
                    let u = Poly::from_coeffs(&cs);
                    // Brute force: roots with multiplicity.
                    let mut rest = u;
                    let mut bf: Vec<(u64, u32)> = Vec::new();
                    for x in 0..p {
                        let lin = Poly::x_minus(x, &fp);
                        let mut m = 0;
                        loop {
                            let (q, r) = rest.divrem(&lin, &fp);
                            if !r.is_zero() {
                                break;
                            }
                            rest = q;
                            m += 1;
                        }
                        if m > 0 {
                            bf.push((x, m));
                        }
                    }
                    let splits = rest.deg() == 0;
                    let mut st = OracleStats::default();
                    let got = split(&u, &fp, &mut st);
                    assert_eq!(got.is_some(), splits, "p={p} u={:?}", &cs);
                    if let Some(r) = got {
                        assert_eq!(r.as_slice(), &bf[..], "p={p} u={:?}", &cs);
                    }
                    idx += step;
                }
            }
        }
    }
}
