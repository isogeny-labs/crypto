//! `Φ_ℓ(X, j₀)` over `ℤ` for a class-number-one CM `j₀`, by CRT over word-size
//! primes, with the `ℓ + 1` neighbours of each prime found by Vélu on curves
//! whose whole `ℓ`-torsion is rational.
//!
//! # Why this exists
//!
//! The neighbours of the crater `j₀ = j(O_K)` of an `ℓ`-isogeny volcano are the
//! roots of `Φ_ℓ(X, j₀)`.  The repository's earlier binary-field walk could only
//! take that step for the two tabulated levels `ℓ = 2, 3`, and PARI's
//! `polmodular` accepts an evaluation point in the prime subfield only in
//! characteristic 2 and does not finish for `ℓ` in the thousands.  The
//! `q`-expansion route costs `O(ℓ⁴)` and the class polynomial by floating point
//! needs `~10⁵`-bit precision.  This module does neither.
//!
//! # Algorithm (Bröker–Lauter–Sutherland style, specialised)
//!
//! `Φ_ℓ(X, j₀)` is a monic integer polynomial of degree `ℓ + 1`, so it is fixed
//! by its residues modulo enough primes.  For one prime `q`:
//!
//! 1. Take `π₀ = 1 + ℓγ ∈ O_K` of prime norm `q`, `γ = u + vω`.
//! 2. The CM curve `E/F_q` with `j(E) = j₀` has Frobenius `±π₀` or its
//!    conjugate up to units; the twist with `#E = N(π₀ − 1) = q + 1 − tr π₀`
//!    has `π ≡ 1 (mod ℓ)`, hence **all of `E[ℓ]` rational**.  The right twist
//!    is the one for which `[#E]P = O` on random points.
//! 3. `E[ℓ] = ⟨R₁, R₂⟩`.  The `ℓ + 1` cyclic subgroups are `⟨R₁⟩` and
//!    `⟨R₂ + sR₁⟩`.  Vélu's formulas give each codomain `j'` from the sums
//!    `t = Σ (6x² + 2A)`, `w = Σ (4y² + x(6x² + 2A))` over one point of every
//!    `±` pair; the subgroups are advanced in lockstep with batched inversions.
//! 4. `∏ (X − j')` mod `q` is `Φ_ℓ(X, j₀) mod q`.
//!
//! The coefficients are reconstructed by Garner's algorithm.  There is no a
//! priori height bound in use: reconstruction stops when the symmetric lift of
//! the accumulated residues already reproduces **every** coefficient modulo a
//! whole batch of fresh primes (at least 8 × 62 bits of independent
//! confirmation), which a truncated lift passes with probability about `2⁻⁴⁹⁶`.
//!
//! For a split `ℓ` the horizontal isogenies return to `j₀`, so
//! `Φ_ℓ(X, j₀) = (X − j₀)^{1 + (D/ℓ)} · H_{D ℓ²}(X)`; for inert `ℓ` it equals
//! the Hilbert class polynomial `H_{D ℓ²}`.
//!
//! Cost per prime is `≈ 12 · ℓ · (ℓ − 1)/2` Montgomery multiplications.

use super::arith::*;
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};
use rayon::prelude::*;
use std::collections::HashSet;
use std::time::Instant;

/// Fundamental discriminants of class number one and their `j`-invariants.
pub const CLASS_ONE: &[(i64, i64)] = &[
    (-3, 0),
    (-4, 1728),
    (-7, -3375),
    (-8, 8000),
    (-11, -32768),
    (-19, -884736),
    (-43, -884736000),
    (-67, -147197952000),
    (-163, -262537412640768000),
];

pub fn class_one_j(d: i64) -> Option<i64> {
    CLASS_ONE.iter().find(|(dd, _)| *dd == d).map(|(_, j)| *j)
}

/// `π₀ = x + yω` of norm `q`, `ω = (D + √D)/2`, `x ≡ 1 (mod ℓ)`, `y ≡ 0 (mod ℓ)`.
#[derive(Clone, Copy, Debug)]
pub struct Frob {
    pub q: u64,
    pub x: i128,
    pub y: i128,
}

fn norm_form(d: i64, x: i128, y: i128) -> i128 {
    let d = d as i128;
    x * x + d * x * y + ((d * d - d) / 4) * y * y
}

/// Deterministic stream of primes `q ∈ [2^61, 2^62)` of the form `N(1 + ℓγ)`.
pub struct PrimeGen {
    d: i64,
    ell: i128,
    v: i128,
    side: u8,
    x: i128,
    x_end: i128,
    seen: HashSet<u64>,
}

const Q_LO: i128 = 1 << 61;
const Q_HI: i128 = 1 << 62;

impl PrimeGen {
    pub fn new(d: i64, ell: u64) -> Self {
        let mut g = PrimeGen {
            d,
            ell: ell as i128,
            v: 0,
            side: 1,
            x: 1,
            x_end: 0,
            seen: HashSet::new(),
        };
        g.advance_range();
        g
    }
    /// Move to the next (v, side) window of x values.
    fn advance_range(&mut self) {
        loop {
            if self.side == 1 {
                self.v += 1;
                self.side = 0;
            } else {
                self.side = 1;
            }
            let y = self.ell * self.v;
            let dabs = (-self.d) as i128;
            let xs = dabs * y / 2; // vertex of the quadratic in x (floor)
            if norm_form(self.d, xs, y) >= Q_HI {
                // y too large for any q below 2^62: restart from small v (cannot happen for sane ell)
                panic!("prime generator exhausted: ell too large for 62-bit primes");
            }
            // monotone on each side of xs: binary search the window with q in [Q_LO, Q_HI)
            let (lo, hi) = if self.side == 0 {
                // right side: x >= xs, q increasing
                (self.search(xs, true, Q_LO), self.search(xs, true, Q_HI))
            } else {
                // left side: x <= xs, q decreasing in x ... mirror
                (self.search(xs, false, Q_HI), self.search(xs, false, Q_LO))
            };
            // right: x in [lo, hi); left: x in [lo, hi) with lo<hi after mirroring
            let (a, b) = if self.side == 0 { (lo, hi) } else { (lo, hi) };
            if a >= b {
                continue;
            }
            // first x >= a with x ≡ 1 (mod ell)
            let r = (1 - a).rem_euclid(self.ell);
            self.x = a + r;
            self.x_end = b;
            return;
        }
    }
    /// Smallest x on the given side of `xs` with q(x) >= target (right side), or
    /// for the left side the smallest x with q(x) < target.
    fn search(&self, xs: i128, right: bool, target: i128) -> i128 {
        let y = self.ell * self.v;
        let (mut lo, mut hi) = (0i128, 1i128 << 40);
        if right {
            while lo < hi {
                let mid = (lo + hi) / 2;
                if norm_form(self.d, xs + mid, y) >= target {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            xs + lo
        } else {
            // largest s with q(xs - s) < target  ->  smallest x = xs - s_max
            let (mut lo, mut hi) = (0i128, 1i128 << 40);
            while lo < hi {
                let mid = (lo + hi + 1) / 2;
                if norm_form(self.d, xs - mid, y) < target {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            xs - lo
        }
    }
    pub fn next_prime(&mut self) -> Frob {
        loop {
            while self.x >= self.x_end {
                self.advance_range();
            }
            let x = self.x;
            self.x += self.ell;
            let y = self.ell * self.v;
            let q = norm_form(self.d, x, y);
            if !(Q_LO..Q_HI).contains(&q) {
                continue;
            }
            let q = q as u64;
            if is_prime_u64(q) && self.seen.insert(q) {
                return Frob { q, x, y };
            }
        }
    }
}

/// A short-Weierstrass curve over `F_q` (`q = f.n`) with `j`-invariant `j0 mod q` whose group
/// order is `n_ord`, found by testing `[n_ord]P = O` on random points of each twist candidate
/// (the quadratic twist; for `j = 0, 1728` the sextic/quartic twists `B = 1, 2, …` / `A = 1, 2, …`).
pub fn curve_with_order(f: &Mont, j0: i128, n_ord: u64, rng: &mut Rng) -> Result<Ec, String> {
    let q = f.n;
    let f = *f;
    let jm = f.from_i128(j0);
    let c1728 = f.to_mont(1728);
    let mut cands: Vec<(u64, u64)> = Vec::new();
    if jm != 0 && jm != c1728 {
        let u = f.sub(c1728, jm);
        let a = f.mul(f.mul(f.to_mont(3), jm), u);
        let b = f.mul(f.mul(f.to_mont(2), jm), f.mul(u, u));
        cands.push((a, b));
        let mut dd = f.to_mont(2);
        while f.pow(dd, (q - 1) / 2) == f.one() {
            dd = f.add(dd, f.one());
        }
        let d2 = f.mul(dd, dd);
        let d3 = f.mul(d2, dd);
        cands.push((f.mul(a, d2), f.mul(b, d3)));
    } else if jm == 0 {
        for s in 1..400u64 {
            cands.push((0, f.to_mont(s)));
        }
    } else {
        for s in 1..400u64 {
            cands.push((f.to_mont(s), 0));
        }
    }
    for (a, b) in cands {
        let ec = Ec { f, a, b };
        let mut ok = true;
        for _ in 0..4 {
            let p = ec.random_point(rng);
            if ec.mul(n_ord, p).is_some() {
                ok = false;
                break;
            }
        }
        if ok {
            return Ok(ec);
        }
    }
    Err("no twist with the expected order".into())
}

/// Residues of the non-leading coefficients of `Φ_ℓ(X, j₀) mod q`, low degree first.
pub fn eval_at_prime(d: i64, j0: i64, ell: u64, fr: Frob, seed: u64) -> Result<Vec<u64>, String> {
    let q = fr.q;
    let f = Mont::new(q);
    let t = 2 * fr.x + (d as i128) * fr.y;
    let n_ord = (q as i128 + 1 - t) as u64;
    if n_ord % (ell * ell) != 0 {
        return Err(format!("order {} not divisible by ell^2", n_ord));
    }
    let mut rng = Rng(seed ^ q ^ 0x9E37_79B9_7F4A_7C15);
    // --- curve with the right twist ---
    let ec = curve_with_order(&f, j0 as i128, n_ord, &mut rng)?;
    // --- E[ell] basis ---
    let mut e = 0u32;
    let mut cof = n_ord;
    while cof % ell == 0 {
        cof /= ell;
        e += 1;
    }
    let _ = e;
    let order_ell_point = |rng: &mut Rng| -> (u64, u64) {
        loop {
            let p = ec.random_point(rng);
            let p1 = ec.mul(cof, p);
            let mut cur = match p1 {
                None => continue,
                Some(_) => p1,
            };
            loop {
                let nxt = ec.mul(ell, cur);
                if nxt.is_none() {
                    return cur.unwrap();
                }
                cur = nxt;
            }
        }
    };
    let r1 = order_ell_point(&mut rng);
    let half = ((ell - 1) / 2) as usize;
    let mut xs1: HashSet<u64> = HashSet::with_capacity(half * 2);
    {
        let mut cur = Some(r1);
        for _ in 0..half {
            xs1.insert(cur.unwrap().0);
            cur = ec.add(cur, Some(r1));
        }
    }
    let r2 = loop {
        let c = order_ell_point(&mut rng);
        if !xs1.contains(&c.0) {
            break c;
        }
    };
    let nsub = (ell + 1) as usize;
    let mut gx = vec![0u64; nsub];
    let mut gy = vec![0u64; nsub];
    gx[0] = r1.0;
    gy[0] = r1.1;
    {
        let mut cur = Some(r2);
        for s in 0..ell as usize {
            let (x, y) = cur.ok_or("degenerate subgroup generator")?;
            gx[1 + s] = x;
            gy[1 + s] = y;
            cur = ec.add(cur, Some(r1));
        }
    }
    // --- lockstep Vélu ---
    let a = ec.a;
    let b = ec.b;
    let mut tacc = vec![0u64; nsub];
    let mut wacc = vec![0u64; nsub];
    let mut cx = gx.clone();
    let mut cy = gy.clone();
    let mut scratch: Vec<u64> = Vec::with_capacity(nsub);
    let mut den = vec![0u64; nsub];
    let contribute = |x: u64, y: u64, t: &mut u64, w: &mut u64| {
        let xx = f.mul(x, x);
        let tq = f.add(f.add(f.add(xx, xx), xx), a); // 3x^2 + A
        let v = f.add(tq, tq); // 6x^2 + 2A
        let yy = f.mul(y, y);
        let u = f.add(f.add(yy, yy), f.add(yy, yy)); // 4y^2
        *t = f.add(*t, v);
        *w = f.add(*w, f.add(u, f.mul(x, v)));
    };
    for s in 0..nsub {
        contribute(cx[s], cy[s], &mut tacc[s], &mut wacc[s]);
    }
    if half >= 2 {
        // 2Q by doubling (batch inversion of 2y)
        for s in 0..nsub {
            den[s] = f.add(cy[s], cy[s]);
        }
        f.batch_inv(&mut den, &mut scratch);
        for s in 0..nsub {
            let xx = f.mul(cx[s], cx[s]);
            let num = f.add(f.add(f.add(xx, xx), xx), a);
            let lam = f.mul(num, den[s]);
            let x3 = f.sub(f.mul(lam, lam), f.add(cx[s], cx[s]));
            let y3 = f.sub(f.mul(lam, f.sub(cx[s], x3)), cy[s]);
            cx[s] = x3;
            cy[s] = y3;
            contribute(x3, y3, &mut tacc[s], &mut wacc[s]);
        }
        for _i in 3..=half {
            for s in 0..nsub {
                den[s] = f.sub(cx[s], gx[s]);
            }
            f.batch_inv(&mut den, &mut scratch);
            for s in 0..nsub {
                let lam = f.mul(f.sub(cy[s], gy[s]), den[s]);
                let x3 = f.sub(f.sub(f.mul(lam, lam), cx[s]), gx[s]);
                let y3 = f.sub(f.mul(lam, f.sub(gx[s], x3)), gy[s]);
                cx[s] = x3;
                cy[s] = y3;
                contribute(x3, y3, &mut tacc[s], &mut wacc[s]);
            }
        }
    }
    // --- codomain j' ---
    let c5 = f.to_mont(5);
    let c7 = f.to_mont(7);
    let c27 = f.to_mont(27);
    let c4 = f.to_mont(4);
    let c1728m = f.to_mont(1728);
    let mut num = vec![0u64; nsub];
    for s in 0..nsub {
        let a2 = f.sub(a, f.mul(c5, tacc[s]));
        let b2 = f.sub(b, f.mul(c7, wacc[s]));
        let a3 = f.mul(f.mul(a2, a2), a2);
        let fa = f.mul(c4, a3);
        num[s] = f.mul(c1728m, fa);
        den[s] = f.add(fa, f.mul(c27, f.mul(b2, b2)));
    }
    if den.iter().any(|&v| v == 0) {
        return Err("singular codomain".into());
    }
    f.batch_inv(&mut den, &mut scratch);
    let roots: Vec<u64> = (0..nsub).map(|s| f.mul(num[s], den[s])).collect();
    // --- product of (X - j') ---
    let mut c = vec![0u64; nsub + 1];
    c[0] = f.one();
    let mut deg = 0usize;
    for &r in &roots {
        c[deg + 1] = 0;
        for k in (1..=deg + 1).rev() {
            c[k] = f.sub(c[k - 1], f.mul(r, c[k]));
        }
        c[0] = f.neg(f.mul(r, c[0]));
        deg += 1;
    }
    // c[nsub] must be 1 (monic)
    debug_assert_eq!(c[nsub], f.one());
    Ok(c[..nsub].iter().map(|&v| f.from_mont(v)).collect())
}

#[derive(Clone, Debug, Default)]
pub struct Level1Report {
    pub d: i64,
    pub ell: u64,
    pub primes_used: usize,
    pub primes_confirming: usize,
    pub max_coeff_bits: u64,
    pub seconds: f64,
}

fn sym_residue(val: &BigUint, m: &BigUint, q: u64) -> u64 {
    // symmetric lift of val mod m, reduced mod q
    let two_val = val << 1u32;
    if two_val <= *m {
        (val % q).to_u64().unwrap()
    } else {
        let neg = (m - val) % q;
        let nv = neg.to_u64().unwrap();
        if nv == 0 {
            0
        } else {
            q - nv
        }
    }
}

/// `Φ_ℓ(X, j₀)` with `j₀ = j(O_K)`, coefficients low degree first, length `ℓ + 2`
/// (monic).  `batch` primes are evaluated in parallel per round.
pub fn phi_at_crater(
    d: i64,
    ell: u64,
    batch: usize,
    progress: bool,
) -> Result<(Vec<BigInt>, Level1Report), String> {
    let j0 = class_one_j(d).ok_or("D must be a class-number-one fundamental discriminant")?;
    if ell < 3 || ell % 2 == 0 || !is_prime_u64(ell) {
        return Err("ell must be an odd prime".into());
    }
    let start = Instant::now();
    let ncoef = (ell + 1) as usize;
    let mut gen = PrimeGen::new(d, ell);
    let mut m = BigUint::one();
    let mut vals: Vec<BigUint> = vec![BigUint::zero(); ncoef];
    let mut used = 0usize;
    let mut rounds = 0usize;
    let batch = batch.max(8);
    loop {
        rounds += 1;
        let frobs: Vec<Frob> = (0..batch).map(|_| gen.next_prime()).collect();
        let results: Vec<Result<Vec<u64>, String>> = frobs
            .par_iter()
            .map(|&fr| eval_at_prime(d, j0, ell, fr, 0xA5A5_0000 + rounds as u64))
            .collect();
        let mut res: Vec<(u64, Vec<u64>)> = Vec::new();
        for (fr, r) in frobs.iter().zip(results) {
            match r {
                Ok(v) => res.push((fr.q, v)),
                Err(e) => return Err(format!("prime {}: {}", fr.q, e)),
            }
        }
        // convergence test against the whole fresh batch
        if used > 0 {
            let all_ok = (0..ncoef).into_par_iter().all(|k| {
                res.iter()
                    .all(|(q, r)| sym_residue(&vals[k], &m, *q) == r[k])
            });
            if all_ok {
                let mut out: Vec<BigInt> = Vec::with_capacity(ncoef + 1);
                for v in &vals {
                    if (v << 1u32) <= m {
                        out.push(BigInt::from(v.clone()));
                    } else {
                        out.push(-BigInt::from(&m - v));
                    }
                }
                out.push(BigInt::one());
                let max_bits = out.iter().map(|c| c.bits()).max().unwrap_or(0);
                return Ok((
                    out,
                    Level1Report {
                        d,
                        ell,
                        primes_used: used,
                        primes_confirming: res.len(),
                        max_coeff_bits: max_bits,
                        seconds: start.elapsed().as_secs_f64(),
                    },
                ));
            }
        }
        for (q, r) in &res {
            let q = *q;
            let m_mod = (&m % q).to_u64().unwrap();
            let minv = {
                // inverse of m_mod modulo q (q prime)
                let mut e = q - 2;
                let mut b = m_mod as u128;
                let mut acc = 1u128;
                let qq = q as u128;
                while e > 0 {
                    if e & 1 == 1 {
                        acc = acc * b % qq;
                    }
                    b = b * b % qq;
                    e >>= 1;
                }
                acc
            };
            let mref = &m;
            vals.par_iter_mut().enumerate().for_each(|(k, v)| {
                let vm = (&*v % q).to_u64().unwrap();
                let diff = ((r[k] + q - vm) % q) as u128;
                let delta = (diff * minv % q as u128) as u64;
                if delta != 0 {
                    *v += mref * delta;
                }
            });
            m *= q;
            used += 1;
        }
        if progress {
            eprintln!(
                "[level1] ell={} primes={} modulus_bits={} elapsed={:.1}s",
                ell,
                used,
                m.bits(),
                start.elapsed().as_secs_f64()
            );
        }
        if used > 20000 {
            return Err("did not converge in 20000 primes".into());
        }
    }
}

/// Coefficient parity vector of `Φ_ℓ(X, j₀) mod 2`, low degree first.
pub fn mod2(coeffs: &[BigInt]) -> Vec<bool> {
    coeffs.iter().map(|c| c.is_odd()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_generator_yields_valid_norms() {
        for (d, ell) in [(-7i64, 5u64), (-3, 7), (-4, 13), (-163, 11)] {
            let mut g = PrimeGen::new(d, ell);
            for _ in 0..20 {
                let fr = g.next_prime();
                assert!(is_prime_u64(fr.q));
                assert_eq!(norm_form(d, fr.x, fr.y), fr.q as i128);
                assert_eq!((fr.x - 1).rem_euclid(ell as i128), 0);
                assert_eq!(fr.y.rem_euclid(ell as i128), 0);
                let t = 2 * fr.x + d as i128 * fr.y;
                let n = fr.q as i128 + 1 - t;
                assert_eq!(n % (ell * ell) as i128, 0);
            }
        }
    }

    /// `Φ_3(X, j₀)` from the repository's own tabulated polynomial, as a check that is
    /// independent of this module.
    fn phi3_at(j0: i64) -> Vec<BigInt> {
        let phi = crate::cryptanalysis::modular_polynomial::phi_3();
        let y = BigInt::from(j0);
        (0..=4u32)
            .map(|i| {
                let mut acc = BigInt::zero();
                let mut yp = BigInt::one();
                for j in 0..=4u32 {
                    acc += phi.coeff(i, j) * &yp;
                    yp *= &y;
                }
                acc
            })
            .collect()
    }

    #[test]
    fn phi3_over_z_matches_the_repository_table() {
        for d in [-7i64, -3, -4, -8, -11, -19] {
            let j0 = class_one_j(d).unwrap();
            let (got, rep) = phi_at_crater(d, 3, 8, false).unwrap();
            assert_eq!(got, phi3_at(j0), "Φ_3(X, j(D={})) differs", d);
            assert!(rep.primes_used >= 1);
        }
    }

    /// Independent native cross-check: the repository's own `Φ_ℓ mod p` (computed from the
    /// `q`-expansion of `j`, `isogeny_walk::modpoly`) evaluated at `j₀ mod p` must equal our
    /// integer polynomial reduced mod `p`.  Covers the special craters `j = 0` and `j = 1728`.
    #[test]
    fn matches_the_q_expansion_modular_polynomial_mod_p() {
        use crate::cryptanalysis::isogeny_walk::field::Field;
        use crate::cryptanalysis::isogeny_walk::modpoly::modular_polynomial;
        let primes: [u64; 3] = [1_000_003, 998_244_353, 4_611_686_018_427_387_847];
        let cases: [(i64, &[u64]); 5] = [
            (-7, &[5, 7, 11, 13, 17, 19]),
            (-3, &[5, 7, 11]),
            (-4, &[5, 7, 13]),
            (-8, &[5, 11]),
            (-163, &[5, 7]),
        ];
        for (d, ells) in cases {
            let j0 = class_one_j(d).unwrap();
            for &ell in ells {
                let (ours, _) = phi_at_crater(d, ell, 8, false).unwrap();
                for &p in &primes {
                    let f = Field::new(&BigUint::from(p)).unwrap();
                    let phi = modular_polynomial(&f, ell).unwrap();
                    let theirs = phi.at_x(&f, &f.from_i64(j0));
                    assert_eq!(theirs.len(), ours.len(), "degree, D={d} ell={ell} p={p}");
                    for (k, c) in ours.iter().enumerate() {
                        let want = c.mod_floor(&BigInt::from(p)).to_biguint().unwrap();
                        assert_eq!(
                            f.to_big(&theirs[k]),
                            want,
                            "coefficient {k}, D={d} ell={ell} p={p}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn class_polynomial_degree_and_monic() {
        let (got, _) = phi_at_crater(-7, 5, 8, false).unwrap();
        assert_eq!(got.len(), 7);
        assert_eq!(got[6], BigInt::one());
    }
}
