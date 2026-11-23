//! Certified ℓ-volcano levels for ℓ = 5 and 7 on the census-fp curves, from
//! rational ℓ-isogenies that ecdb computes itself (no stored graph needed).
//!
//! `volcano.rs` derives levels from the stored Vélu graphs (ℓ = 2, 3, 5); a
//! class it refuses has no level, and there is no stored 7-isogeny graph at
//! all. The curve-trait pipeline can also leave a level as an upper bound
//! (`level_at_most`) when its division-polynomial test is capped. This
//! importer settles ℓ = 5 and 7 exactly for every ordinary census-fp curve
//! (p < 2^10, p ≠ ℓ) and writes a row only when every check below passes.
//!
//! # Mathematics
//!
//! Write t^2 - 4p = v^2 D_K with D_K fundamental; the ℓ-volcano of the class
//! has depth d = v_ℓ(v) and a curve's level is v_ℓ of the conductor of
//! End(E) (Kohel): 0 on the crater, d on the floor.
//!
//! **Rational ℓ-isogenies.** An F_p-rational cyclic subgroup C of order ℓ has
//! kernel polynomial h_C(x) = ∏ (x - x(P)) over one of each ±P ∈ C \ {0}, of
//! degree m = (ℓ - 1)/2, dividing ψ_ℓ and with coefficients in F_p. ecdb
//! builds ψ_ℓ for y^2 = x^3 + a x + b (ψ_5 = 16 Y^2 f_4 - f_3^3,
//! ψ_7 = ψ_5 f_3^3 - 16 Y^2 f_4^3, Y = x^3 + a x + b), checks it is
//! squarefree, splits off its irreducible factors of degree ≤ m (distinct-
//! and equal-degree factorisation, Cantor–Zassenhaus), and for each factor g
//! takes X = x mod g in F_p[x]/(g) as x(P) and forms ∏_{k=1..m} (T - x([k]P))
//! with the multiplication-by-k maps (x([2]P) = (X^4 - 2aX^2 - 8bX + a^2)/4Y,
//! x([3]P) = X - 4 Y f_4 / f_3^2). The subgroup ⟨P⟩ is F_p-rational exactly
//! when every coefficient lies in F_p; those polynomials, deduplicated, are
//! the rational ℓ-isogeny kernels. Each one is re-checked: monic of degree m,
//! divides ψ_ℓ, its Vélu/Kohel codomain (A, B) = (a - 5t, b - 7w) is
//! nonsingular and has exactly #E(F_p) points by an exhaustive count (an
//! isogeny over F_p preserves the point count; the count is independent of
//! the kernel computation).
//!
//! **Levels.** With d ≥ 1 the number N of rational ℓ-isogenies is 1 on the
//! floor and ℓ + 1 above it (Kohel); with d = 0 it is 1 + (D_K/ℓ). Any other
//! N is a contradiction and refuses the class. On the floor the level is d.
//! Otherwise every rational first step is followed by a non-backtracking walk
//! (never back to the j-invariant just left) of at most d steps in total,
//! stopping at the first curve with N = 1. A descending first step exists
//! (at most 2 horizontal and 1 ascending among ℓ + 1 ≥ 6), and a walk that
//! starts by descending must keep descending: the only non-descending edge
//! at a level-k curve below the crater goes back up to level k - 1, i.e. to
//! the j-invariant just left. So the shortest such walk to the floor has
//! length exactly d - level (no walk is shorter, since one edge changes the
//! level by at most 1), and level = d - (that length). d = 0 gives level 0.
//!
//! # Cross-checks (a failure refuses the class and ℓ, reported, never written)
//!
//! - the stored invariants are consistent: |t| < 2√p, p ∤ t, and
//!   t^2 - 4p = v^2 D_K with D_K a negative fundamental discriminant (the depth
//!   and (D_K/ℓ) are read off v and D_K; a failure refuses the class at both ℓ
//!   and is counted as `levels57_classes_refused_invariants`);
//! - the stored trace equals the exhaustive count of the census model;
//! - N ∈ {1, ℓ + 1} (d ≥ 1) or N = 1 + (D_K/ℓ) (d = 0) at every curve and
//!   every walk step;
//! - every codomain j-invariant is a census curve of the same class (the
//!   census holds every curve over F_p), the curves of one j get one level,
//!   and across every computed ℓ-isogeny edge the levels differ by at most 1;
//! - for D_K ∉ {-3, -4} and d ≥ 1, the edge counts of the volcano: a crater
//!   curve has 1 + (D_K/ℓ) crater neighbours and ℓ - (D_K/ℓ) at level 1, a
//!   curve at 0 < k < d has one neighbour at k - 1 and ℓ at k + 1, a floor
//!   curve one at d - 1;
//! - across every stored ℓ = 5 Vélu edge, levels differ by at most 1;
//! - an exact level already stored (volcano.rs from the ℓ = 5 graph, or
//!   end_ring.rs from Kohel's torsion test) must be equal, a stored
//!   `level_at_most` must not be below the certified level, and a stored
//!   `depth` must equal v_ℓ(v);
//! - where End(E) is stored with status proved, level = v_ℓ(conductor_f).
//!
//! A class that fails any check at ℓ is refused whole at that ℓ.
//!
//! # Writes (insert-or-update, never delete)
//!
//! `endomorphism_level` (status `proved`): inserted where missing; a row that
//! has only `level_at_most` (level NULL) and a NULL or equal depth gets the
//! certified level, keeping its bound and naming the bound's earlier method
//! in `method`; an exact row is never touched (it was cross-checked above). End(E)
//! itself is not written here: `end_ring.rs` (Kohel's l^k-torsion criterion)
//! stores it for every ordinary census curve, and this importer runs after it
//! so that its levels at ℓ = 5, 7 are checked against an independent method.
//!
//! # Report
//!
//! `classes_ok_l5`, `classes_ok_l7`, `classes_refused_l5`, `classes_refused_l7`
//! (each refusal also a problem), `levels57_kernels_checked`,
//! `levels57_edges_checked`, `levels57_walk_steps`,
//! `levels57_agree_stored_level`, `levels57_agree_end_conductor`,
//! `levels57_stored_l5_edges_checked`, `levels57_classes_refused_invariants`,
//! `endomorphism_levels_l5_l7_written`.
//!
//! ℓ = p (p = 5 at ℓ = 5, p = 7 at ℓ = 7) has no volcano and is skipped;
//! supersingular classes are skipped.

use super::volcano::kronecker;
use super::{bulk, Ctx, Report};
use anyhow::Result;
use num_bigint::BigInt;
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

pub const ELLS: [u64; 2] = [5, 7];

/// The largest p handled (exhaustive point counts are O(p)).
pub const MAX_P: u64 = 1 << 16;

// ------------------------------------------------------------- F_p polynomials

/// Ascending coefficients mod p, no trailing zeros (the zero polynomial is empty).
pub type Poly = Vec<u64>;

#[derive(Clone, Copy, Debug)]
pub struct Fp {
    pub p: u64,
}

impl Fp {
    pub fn new(p: u64) -> Fp {
        assert!(p > 3 && p < MAX_P, "Fp: 3 < p < 2^16");
        Fp { p }
    }
    fn red(&self, x: i64) -> u64 {
        x.rem_euclid(self.p as i64) as u64
    }
    fn add(&self, a: u64, b: u64) -> u64 {
        (a + b) % self.p
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        (a + self.p - b) % self.p
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        a * b % self.p
    }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        debug_assert!(!a.is_multiple_of(self.p));
        self.pow(a, self.p - 2)
    }

    fn trim(mut a: Poly) -> Poly {
        while a.last() == Some(&0) {
            a.pop();
        }
        a
    }
    fn poly(&self, c: &[i64]) -> Poly {
        Fp::trim(c.iter().map(|&x| self.red(x)).collect())
    }
    fn padd(&self, a: &[u64], b: &[u64]) -> Poly {
        let n = a.len().max(b.len());
        Fp::trim((0..n).map(|i| self.add(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0))).collect())
    }
    fn psub(&self, a: &[u64], b: &[u64]) -> Poly {
        let n = a.len().max(b.len());
        Fp::trim((0..n).map(|i| self.sub(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0))).collect())
    }
    fn pscale(&self, a: &[u64], c: u64) -> Poly {
        Fp::trim(a.iter().map(|&x| self.mul(x, c)).collect())
    }
    fn pmul(&self, a: &[u64], b: &[u64]) -> Poly {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        let mut r = vec![0u64; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            if x == 0 {
                continue;
            }
            for (j, &y) in b.iter().enumerate() {
                r[i + j] = (r[i + j] + x * y) % self.p;
            }
        }
        Fp::trim(r)
    }
    /// (quotient, remainder); `b` must be nonzero.
    fn pdivrem(&self, a: &[u64], b: &[u64]) -> (Poly, Poly) {
        assert!(!b.is_empty(), "division by the zero polynomial");
        let mut r: Poly = a.to_vec();
        if r.len() < b.len() {
            return (Vec::new(), Fp::trim(r));
        }
        let lead_inv = self.inv(*b.last().unwrap());
        let mut q = vec![0u64; r.len() - b.len() + 1];
        for i in (0..q.len()).rev() {
            let c = self.mul(r[i + b.len() - 1], lead_inv);
            q[i] = c;
            if c != 0 {
                for (j, &y) in b.iter().enumerate() {
                    r[i + j] = self.sub(r[i + j], self.mul(c, y));
                }
            }
        }
        r.truncate(b.len() - 1);
        (Fp::trim(q), Fp::trim(r))
    }
    fn pmod(&self, a: &[u64], m: &[u64]) -> Poly {
        self.pdivrem(a, m).1
    }
    fn monic(&self, a: &[u64]) -> Poly {
        match a.last() {
            None => Vec::new(),
            Some(&l) => self.pscale(a, self.inv(l)),
        }
    }
    fn pgcd(&self, a: &[u64], b: &[u64]) -> Poly {
        let (mut x, mut y) = (a.to_vec(), b.to_vec());
        while !y.is_empty() {
            let r = self.pmod(&x, &y);
            x = y;
            y = r;
        }
        self.monic(&x)
    }
    fn mulmod(&self, a: &[u64], b: &[u64], m: &[u64]) -> Poly {
        self.pmod(&self.pmul(a, b), m)
    }
    fn powmod(&self, a: &[u64], mut e: u64, m: &[u64]) -> Poly {
        let mut r = self.pmod(&[1], m);
        let mut b = self.pmod(a, m);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mulmod(&r, &b, m);
            }
            b = self.mulmod(&b, &b, m);
            e >>= 1;
        }
        r
    }
    /// a^-1 mod m, or None when gcd(a, m) ≠ 1.
    fn invmod(&self, a: &[u64], m: &[u64]) -> Option<Poly> {
        let (mut r0, mut r1) = (m.to_vec(), self.pmod(a, m));
        let (mut s0, mut s1): (Poly, Poly) = (Vec::new(), vec![1]);
        while !r1.is_empty() {
            let (q, r) = self.pdivrem(&r0, &r1);
            let s = self.psub(&s0, &self.pmul(&q, &s1));
            (r0, r1) = (r1, r);
            (s0, s1) = (s1, s);
        }
        if r0.len() != 1 {
            return None;
        }
        Some(self.pmod(&self.pscale(&s0, self.inv(r0[0])), m))
    }
    #[cfg(test)]
    fn eval(&self, a: &[u64], x: u64) -> u64 {
        a.iter().rev().fold(0, |acc, &c| self.add(self.mul(acc, x), c))
    }
    /// a(x) for x an element of F_p[X]/(m).
    fn eval_mod(&self, a: &[u64], x: &[u64], m: &[u64]) -> Poly {
        a.iter().rev().fold(Vec::new(), |acc, &c| self.padd(&self.mulmod(&acc, x, m), &[c]))
    }
    fn deriv(&self, a: &[u64]) -> Poly {
        Fp::trim(a.iter().enumerate().skip(1).map(|(i, &c)| self.mul(c, i as u64 % self.p)).collect())
    }
}

// ------------------------------------------------------- division polynomials

/// Y = x^3 + a x + b.
fn cubic(f: &Fp, a: u64, b: u64) -> Poly {
    f.poly(&[b as i64, a as i64, 0, 1])
}

/// f_3 = ψ_3 = 3x^4 + 6a x^2 + 12b x - a^2.
fn f3(f: &Fp, a: u64, b: u64) -> Poly {
    let (a, b) = (a as i64, b as i64);
    f.poly(&[-(a * a), 12 * b, 6 * a, 0, 3])
}

/// f_4 = ψ_4 / ψ_2 = 2(x^6 + 5a x^4 + 20b x^3 - 5a^2 x^2 - 4ab x - 8b^2 - a^3).
fn f4(f: &Fp, a: u64, b: u64) -> Poly {
    let (a, b) = (a as i64, b as i64);
    f.poly(&[2 * (-8 * b * b - a * a * a), -8 * a * b, -10 * a * a, 40 * b, 10 * a, 0, 2])
}

/// ψ_ℓ in x of y^2 = x^3 + a x + b, for ℓ ∈ {3, 5, 7} (leading coefficient ℓ).
pub fn division_polynomial(f: &Fp, a: u64, b: u64, l: u64) -> Poly {
    let y = cubic(f, a, b);
    let y2 = f.pmul(&y, &y);
    let t3 = f3(f, a, b);
    let t4 = f4(f, a, b);
    let t3c = f.pmul(&t3, &f.pmul(&t3, &t3));
    let psi5 = f.psub(&f.pscale(&f.pmul(&y2, &t4), 16), &t3c);
    match l {
        3 => t3,
        5 => psi5,
        7 => {
            let t4c = f.pmul(&t4, &f.pmul(&t4, &t4));
            f.psub(&f.pmul(&psi5, &t3c), &f.pscale(&f.pmul(&y2, &t4c), 16))
        }
        _ => panic!("division_polynomial: l must be 3, 5 or 7"),
    }
}

// ------------------------------------------------------------- factorisation

/// Deterministic xorshift for Cantor–Zassenhaus (correctness never depends on it).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

/// Split `g`, a product of distinct monic irreducibles of degree r, into them.
fn equal_degree(f: &Fp, g: &[u64], r: usize, rng: &mut Rng, out: &mut Vec<Poly>) {
    let n = g.len() - 1;
    if n == r {
        out.push(g.to_vec());
        return;
    }
    loop {
        let a: Poly = Fp::trim((0..n).map(|_| rng.next() % f.p).collect());
        if a.len() < 2 {
            continue;
        }
        // a^((p^r - 1)/2) = (a^(1 + p + ... + p^(r-1)))^((p - 1)/2)
        let mut c = a.clone();
        let mut norm = a.clone();
        for _ in 1..r {
            c = f.powmod(&c, f.p, g);
            norm = f.mulmod(&norm, &c, g);
        }
        let b = f.powmod(&norm, (f.p - 1) / 2, g);
        let d = f.pgcd(&f.psub(&b, &[1]), g);
        if d.len() > 1 && d.len() < g.len() {
            let (q, _) = f.pdivrem(g, &d);
            let q = f.monic(&q);
            equal_degree(f, &d, r, rng, out);
            equal_degree(f, &q, r, rng, out);
            return;
        }
    }
}

/// The monic irreducible factors of degree ≤ `max_deg` of the squarefree monic `psi`.
fn small_factors(f: &Fp, psi: &[u64], max_deg: usize) -> Vec<Poly> {
    let x: Poly = vec![0, 1];
    let mut rest = psi.to_vec();
    let mut h = f.pmod(&x, psi);
    let mut out = Vec::new();
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15 ^ f.p);
    for r in 1..=max_deg {
        if rest.len() <= 1 {
            break;
        }
        h = f.powmod(&h, f.p, psi);
        let g = f.pgcd(&f.psub(&h, &x), &rest);
        if g.len() > 1 {
            rest = f.monic(&f.pdivrem(&rest, &g).0);
            equal_degree(f, &g, r, &mut rng, &mut out);
        }
    }
    out
}

// --------------------------------------------------------------- isogenies

/// One rational ℓ-isogeny of y^2 = x^3 + a x + b: its kernel polynomial (monic,
/// ascending) and the Vélu/Kohel codomain y^2 = x^3 + A x + B with j-invariant j.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Neighbour {
    pub kernel: Poly,
    pub a: u64,
    pub b: u64,
    pub j: u64,
}

/// j(y^2 = x^3 + a x + b), or None when singular.
pub fn j_invariant(f: &Fp, a: u64, b: u64) -> Option<u64> {
    let a3 = f.mul(4, f.mul(a, f.mul(a, a)));
    let den = f.add(a3, f.mul(27, f.mul(b, b)));
    if den == 0 {
        return None;
    }
    Some(f.mul(f.mul(1728 % f.p, a3), f.inv(den)))
}

/// #E(F_p) for y^2 = x^3 + a x + b by an exhaustive count with Euler's criterion table.
pub fn point_count(f: &Fp, chi: &[i8], a: u64, b: u64) -> u64 {
    let mut s: i64 = 0;
    for x in 0..f.p {
        let v = f.add(f.mul(f.mul(x, x), x), f.add(f.mul(a, x), b));
        s += chi[v as usize] as i64;
    }
    (f.p as i64 + 1 + s) as u64
}

/// Quadratic character table of F_p (chi[0] = 0).
pub fn chi_table(p: u64) -> Vec<i8> {
    let mut chi = vec![-1i8; p as usize];
    chi[0] = 0;
    for x in 1..p {
        chi[(x * x % p) as usize] = 1;
    }
    chi
}

/// The Vélu/Kohel codomain of the kernel polynomial h (monic, degree m = (ℓ-1)/2).
pub fn kohel_codomain(f: &Fp, a: u64, b: u64, h: &[u64]) -> (u64, u64) {
    let m = h.len() - 1;
    let c = |i: isize| if i < 0 { 0 } else { h[i as usize] };
    let mi = m as isize;
    let s1 = f.sub(0, c(mi - 1));
    let s2 = c(mi - 2);
    let s3 = f.sub(0, c(mi - 3));
    let p1 = s1;
    let p2 = f.sub(f.mul(s1, s1), f.mul(2, s2));
    let p3 = f.add(f.sub(f.mul(s1, f.mul(s1, s1)), f.mul(3, f.mul(s1, s2))), f.mul(3, s3));
    let mm = m as u64 % f.p;
    let t = f.add(f.mul(6, p2), f.mul(2, f.mul(a, mm)));
    let w = f.add(f.add(f.mul(10, p3), f.mul(6, f.mul(a, p1))), f.mul(4, f.mul(b, mm)));
    (f.sub(a, f.mul(5, t)), f.sub(b, f.mul(7, w)))
}

/// The x-coordinates of [k]P, k = 1..m, in F_p[X]/(g) with x(P) = X.
fn multiples(f: &Fp, a: u64, b: u64, g: &[u64], m: usize) -> Option<Vec<Poly>> {
    let x = f.pmod(&[0, 1], g);
    let y = f.eval_mod(&cubic(f, a, b), &x, g);
    let mut xs = vec![x.clone()];
    if m >= 2 {
        let (aa, bb) = (a as i64, b as i64);
        let num = f.eval_mod(&f.poly(&[aa * aa, -8 * bb, -2 * aa, 0, 1]), &x, g);
        let den = f.invmod(&f.pscale(&y, 4), g)?;
        xs.push(f.mulmod(&num, &den, g));
    }
    if m >= 3 {
        let t3 = f.eval_mod(&f3(f, a, b), &x, g);
        let t4 = f.eval_mod(&f4(f, a, b), &x, g);
        let den = f.invmod(&f.mulmod(&t3, &t3, g), g)?;
        let corr = f.mulmod(&f.pscale(&f.mulmod(&y, &t4, g), 4), &den, g);
        xs.push(f.psub(&x, &corr));
    }
    if m >= 4 {
        return None;
    }
    Some(xs)
}

/// The F_p-rational ℓ-isogeny kernels of y^2 = x^3 + a x + b (ℓ ∈ {3, 5, 7}),
/// each re-checked and with its codomain; `order` is #E(F_p).
pub fn rational_isogenies(f: &Fp, chi: &[i8], a: u64, b: u64, l: u64, order: u64) -> Result<Vec<Neighbour>, String> {
    if f.p == l {
        return Err(format!("l = p = {l}"));
    }
    if j_invariant(f, a, b).is_none() {
        return Err("singular model".into());
    }
    let psi = f.monic(&division_polynomial(f, a, b, l));
    if f.pgcd(&psi, &f.deriv(&psi)).len() != 1 {
        return Err(format!("psi_{l} is not squarefree"));
    }
    let m = ((l - 1) / 2) as usize;
    let mut kernels: BTreeSet<Poly> = BTreeSet::new();
    for g in small_factors(f, &psi, m) {
        let Some(xs) = multiples(f, a, b, &g, m) else {
            return Err(format!("a factor of psi_{l} meets the 2- or 3-torsion"));
        };
        // ∏ (T - x_k) with coefficients in F_p[X]/(g); index = power of T
        let mut h: Vec<Poly> = vec![vec![1]];
        for xk in &xs {
            let mut next: Vec<Poly> = vec![Vec::new(); h.len() + 1];
            for (i, c) in h.iter().enumerate() {
                next[i + 1] = f.padd(&next[i + 1], c);
                next[i] = f.psub(&next[i], &f.mulmod(c, xk, &g));
            }
            h = next;
        }
        if h.iter().all(|c| c.len() <= 1) {
            kernels.insert(h.iter().map(|c| c.first().copied().unwrap_or(0)).collect());
        }
    }
    let mut out = Vec::new();
    for k in kernels {
        if k.len() != m + 1 || k[m] != 1 {
            return Err(format!("kernel {k:?} is not monic of degree {m}"));
        }
        if !f.pmod(&psi, &k).is_empty() {
            return Err(format!("kernel {k:?} does not divide psi_{l}"));
        }
        let (ca, cb) = kohel_codomain(f, a, b, &k);
        let Some(j) = j_invariant(f, ca, cb) else {
            return Err(format!("kernel {k:?}: singular codomain"));
        };
        let n = point_count(f, chi, ca, cb);
        if n != order {
            return Err(format!("kernel {k:?}: codomain has {n} points, E has {order}"));
        }
        out.push(Neighbour { kernel: k, a: ca, b: cb, j });
    }
    if !(out.len() <= 2 || out.len() == l as usize + 1) {
        return Err(format!("{} rational {l}-isogenies: neither 0, 1, 2 nor l + 1", out.len()));
    }
    Ok(out)
}

/// The level of E (depth `depth` ≥ 1) by non-backtracking walks to the floor;
/// `first` are E's rational ℓ-isogenies. Returns (level, steps to the floor).
pub fn level_by_walks(
    f: &Fp,
    chi: &[i8],
    j0: u64,
    first: &[Neighbour],
    l: u64,
    depth: u32,
    order: u64,
) -> Result<(u32, u32), String> {
    if first.len() == 1 {
        return Ok((depth, 0));
    }
    if first.len() != l as usize + 1 {
        return Err(format!("depth {depth} but {} rational {l}-isogenies (want 1 or {})", first.len(), l + 1));
    }
    let mut best: Option<u32> = None;
    for start in first {
        let (mut cur, mut prev_j, mut steps) = (start.clone(), j0, 1u32);
        loop {
            if best.is_some_and(|b| steps >= b) {
                break;
            }
            let nb = rational_isogenies(f, chi, cur.a, cur.b, l, order)?;
            if nb.len() == 1 {
                best = Some(steps);
                break;
            }
            if nb.len() != l as usize + 1 {
                return Err(format!(
                    "walk step at j = {}: {} rational {l}-isogenies (want 1 or {})",
                    cur.j,
                    nb.len(),
                    l + 1
                ));
            }
            if steps >= depth {
                break;
            }
            let Some(next) = nb.into_iter().find(|n| n.j != prev_j) else { break };
            prev_j = cur.j;
            cur = next;
            steps += 1;
        }
        if best == Some(1) {
            break;
        }
    }
    match best {
        Some(h) => Ok((depth - h, h)),
        None => Err(format!("no non-backtracking walk reaches the floor within depth {depth}")),
    }
}

/// p-adic-free valuation v_ℓ(n), n ≠ 0.
pub fn val(mut n: i64, l: i64) -> u32 {
    let mut k = 0;
    while n != 0 && n % l == 0 {
        n /= l;
        k += 1;
    }
    k
}

/// Checks the stored Frobenius invariants of an ordinary class over F_p:
/// |t| < 2√p, p ∤ t, t^2 - 4p = v^2 D_K with v ≥ 1 and D_K a negative
/// fundamental discriminant. The depth and (D_K/ℓ) are read off v and D_K,
/// so a class whose stored v or D_K does not match its trace is refused.
pub fn check_invariants(p: u64, t: i64, v: i64, d_k: i64) -> Result<(), String> {
    let pi = p as i64;
    let disc = t * t - 4 * pi;
    if disc >= 0 {
        return Err(format!("t = {t} violates the Hasse bound over F_{p}"));
    }
    if t % pi == 0 {
        return Err(format!("t = {t} is divisible by p = {p}: supersingular"));
    }
    if v < 1 || v.checked_mul(v).and_then(|w| w.checked_mul(d_k)) != Some(disc) {
        return Err(format!("t^2 - 4p = {disc} != v^2 D_K = {v}^2 * {d_k}"));
    }
    let squarefree = |n: i64| (2..).take_while(|q: &i64| q * q <= n).all(|q| n % (q * q) != 0);
    let fundamental = match d_k.rem_euclid(4) {
        1 => squarefree(-d_k),
        0 => matches!((d_k / 4).rem_euclid(4), 2 | 3) && squarefree(-d_k / 4),
        _ => false,
    };
    if !fundamental {
        return Err(format!("D_K = {d_k} is not a negative fundamental discriminant"));
    }
    Ok(())
}

/// A stored `endomorphism_level` row: (level, level_at_most, depth).
pub type StoredLevel = (Option<i32>, Option<i32>, Option<i32>);

/// A stored `endomorphism_level` row (level, level_at_most, depth) against the
/// certified `level` at depth `depth`: Ok(true) when it already holds the same
/// exact level, Ok(false) when it may be filled, Err on any conflict (a
/// different level or depth, or a bound below the level).
pub fn compare_stored(id: i64, l: u64, depth: u32, level: u32, stored: StoredLevel) -> Result<bool, String> {
    let (old, at_most, old_depth) = stored;
    if old_depth.is_some_and(|d| i64::from(d) != i64::from(depth)) {
        return Err(format!("curve {id}: stored depth {old_depth:?}, computed v_{l}(v) = {depth}"));
    }
    match old {
        Some(o) if i64::from(o) != i64::from(level) => Err(format!("curve {id}: stored level {o}, certified {level}")),
        Some(_) => Ok(true),
        None if at_most.is_some_and(|m| i64::from(m) < i64::from(level)) => {
            Err(format!("curve {id}: stored level_at_most {at_most:?} < certified {level}"))
        }
        None => Ok(false),
    }
}

/// What one class gave at one ℓ.
#[derive(Debug, Default)]
pub struct ClassLevels {
    pub levels: BTreeMap<i64, u32>,
    pub kernels: u64,
    pub walk_steps: u64,
    pub edges: u64,
}

/// Levels at ℓ of the census curves `(id, a, b)` of one ordinary class over F_p
/// with #E = `order`, depth `depth`, χ = (D_K/ℓ); `special` when D_K ∈ {-3, -4}.
#[allow(clippy::too_many_arguments)]
pub fn class_levels(
    p: u64,
    chi_tab: &[i8],
    l: u64,
    depth: u32,
    chi: i64,
    special: bool,
    order: u64,
    curves: &[(i64, u64, u64)],
) -> Result<ClassLevels, String> {
    let f = Fp::new(p);
    let mut out = ClassLevels::default();
    let mut by_j: BTreeMap<u64, u32> = BTreeMap::new();
    let mut nbs: Vec<(i64, u64, Vec<Neighbour>)> = Vec::new();
    for &(id, a, b) in curves {
        let j = j_invariant(&f, a, b).ok_or_else(|| format!("curve {id} is singular"))?;
        let n = point_count(&f, chi_tab, a, b);
        if n != order {
            return Err(format!("curve {id}: {n} points, the class says {order}"));
        }
        let nb = rational_isogenies(&f, chi_tab, a, b, l, order).map_err(|e| format!("curve {id}: {e}"))?;
        out.kernels += nb.len() as u64;
        let lv = if depth == 0 {
            if nb.len() as i64 != 1 + chi {
                return Err(format!("curve {id}: depth 0 but {} rational {l}-isogenies != 1 + (D_K/{l})", nb.len()));
            }
            0
        } else if let Some(&lv) = by_j.get(&j) {
            lv
        } else {
            let (lv, h) =
                level_by_walks(&f, chi_tab, j, &nb, l, depth, order).map_err(|e| format!("curve {id}: {e}"))?;
            out.walk_steps += h as u64;
            lv
        };
        if let Some(&old) = by_j.get(&j) {
            if old != lv {
                return Err(format!("j = {j} has levels {old} and {lv}"));
            }
        }
        by_j.insert(j, lv);
        out.levels.insert(id, lv);
        nbs.push((id, j, nb));
    }
    // every computed edge: codomain in the class, |level difference| <= 1, volcano edge counts
    for (id, j, nb) in &nbs {
        let lv = by_j[j];
        let mut at = BTreeMap::<u32, i64>::new();
        for n in nb {
            let Some(&lt) = by_j.get(&n.j) else {
                return Err(format!("curve {id}: codomain j = {} is not a census curve of the class", n.j));
            };
            if lt.abs_diff(lv) > 1 {
                return Err(format!("curve {id}: an {l}-isogeny joins levels {lv} and {lt}"));
            }
            *at.entry(lt).or_default() += 1;
            out.edges += 1;
        }
        if depth >= 1 && !special {
            let li = l as i64;
            let get = |k: u32| at.get(&k).copied().unwrap_or(0);
            let ok = if lv == 0 {
                get(0) == 1 + chi && get(1) == li - chi
            } else if lv < depth {
                get(lv - 1) == 1 && get(lv + 1) == li && get(lv) == 0
            } else {
                get(lv - 1) == 1 && nb.len() == 1
            };
            if !ok {
                return Err(format!("curve {id} at level {lv}: neighbour levels {at:?} break the volcano shape"));
            }
        }
    }
    Ok(out)
}

// ------------------------------------------------------------------- import

const CURVES_SQL: &str = "
SELECT c.id, ic.id, ic.p::text, c.coefficients->>'a', c.coefficients->>'b', fg.trace::text,
       fg.conductor_v::text, fg.cm_discriminant::text, coalesce(fg.is_supersingular, false)
FROM ec.curve c
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
JOIN ec.frobenius_genus1 fg ON fg.isogeny_class_id = ic.id
WHERE ic.p < 1024 AND c.model = 'short_weierstrass' AND c.genus = 1
  AND fg.conductor_v IS NOT NULL AND fg.cm_discriminant IS NOT NULL
  AND (c.family = 'census-fp'
       OR EXISTS (SELECT 1 FROM ec.isogeny i WHERE i.source_curve_id = c.id AND i.formula = 'velu'))";

const EXISTING_SQL: &str = "
SELECT l.curve_id, l.ell::int8, l.level, l.level_at_most, l.depth
FROM ec.endomorphism_level l WHERE l.ell IN (2, 3, 5, 7)";

const END_EXISTING_SQL: &str = "
SELECT e.curve_id, e.conductor_f::text FROM ec.endomorphism_ring e
WHERE e.status = 'proved' AND e.conductor_f IS NOT NULL";

const EDGES_SQL: &str = "
SELECT i.source_curve_id, i.target_curve_id
FROM ec.isogeny i
JOIN ec.abelian_variety av ON av.curve_id = i.source_curve_id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
WHERE ic.p < 1024 AND i.formula = 'velu' AND i.ell = 5";

const LEVEL_SQL: &str = "
INSERT INTO ec.endomorphism_level AS el (curve_id, ell, depth, level, level_at_most, method, status, provenance_id)
SELECT r.curve_id, r.ell, r.depth, r.level, NULL, r.method, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, ell numeric, depth int, level int, method text)
ON CONFLICT (curve_id, ell) DO UPDATE SET depth = EXCLUDED.depth, level = EXCLUDED.level,
       method = CASE WHEN el.method IS NULL OR el.level_at_most IS NULL THEN EXCLUDED.method
                     ELSE EXCLUDED.method || '; level_at_most kept from: ' || el.method END,
       status = 'proved', provenance_id = EXCLUDED.provenance_id
WHERE el.level IS NULL AND (el.depth IS NULL OR el.depth = EXCLUDED.depth)";

struct Class {
    p: u64,
    t: i64,
    order: u64,
    v: i64,
    d_k: i64,
    curves: Vec<(i64, u64, u64)>,
}

struct Job {
    class_id: i64,
    l: u64,
    depth: u32,
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let mut report = Report {
        input: "census-fp curves (rational 5- and 7-isogenies computed by ecdb)".into(),
        ..Default::default()
    };
    let client = ctx.client;
    let src = ctx
        .source(
            "ecdb/src/import/levels57.rs",
            &crate::math::sha256_hex(include_str!("levels57.rs")),
            "l = 5, 7 volcano levels certified by ecdb from computed rational isogenies",
        )
        .await?;
    let prov = ctx
        .provenance(
            src,
            "rational l-isogeny kernels from psi_l (Cantor-Zassenhaus, Kohel codomain, point-count check); \
             level = depth - shortest non-backtracking walk to the floor; edge and End cross-checks",
            None,
        )
        .await?;

    let mut classes: BTreeMap<i64, Class> = BTreeMap::new();
    for r in client.query(CURVES_SQL, &[]).await? {
        if r.get::<_, bool>(8) {
            report.count("levels57_supersingular_skipped", 1);
            continue;
        }
        let id: i64 = r.get(0);
        let class_id: i64 = r.get(1);
        let parsed = (|| -> Option<(u64, u64, u64, i64, i64, i64)> {
            let p: u64 = r.get::<_, String>(2).parse().ok()?;
            let a: u64 = r.get::<_, Option<String>>(3)?.parse().ok()?;
            let b: u64 = r.get::<_, Option<String>>(4)?.parse().ok()?;
            let t: i64 = r.get::<_, String>(5).parse().ok()?;
            let v: i64 = r.get::<_, String>(6).parse().ok()?;
            let d_k: i64 = r.get::<_, String>(7).parse().ok()?;
            Some((p, a % p, b % p, t, v, d_k))
        })();
        let Some((p, a, b, t, v, d_k)) = parsed else {
            report.problem(format!("curve {id}: unreadable p, a, b, trace, v or D_K"));
            continue;
        };
        if p <= 3 || v == 0 {
            report.count("levels57_unsupported_skipped", 1);
            continue;
        }
        let order = (p as i64 + 1 - t) as u64;
        let c = classes.entry(class_id).or_insert_with(|| Class { p, t, order, v, d_k, curves: Vec::new() });
        c.curves.push((id, a, b));
    }
    report.count("levels57_classes", classes.len() as u64);
    report.count("levels57_curves", classes.values().map(|c| c.curves.len() as u64).sum());

    let mut jobs = Vec::new();
    for (class_id, c) in &classes {
        if let Err(why) = check_invariants(c.p, c.t, c.v, c.d_k) {
            report.count("levels57_classes_refused_invariants", 1);
            report.problem(format!("class {class_id}: {why}"));
            continue;
        }
        for l in ELLS {
            if l == c.p {
                report.count(&format!("classes_skipped_l{l}_equals_p"), 1);
                continue;
            }
            jobs.push(Job { class_id: *class_id, l, depth: val(c.v, l as i64) });
        }
    }

    // exhaustive work, off the async runtime, on a few threads
    let chi_tabs: BTreeMap<u64, Vec<i8>> = classes.values().map(|c| (c.p, chi_table(c.p))).collect();
    let results: Vec<Mutex<Option<Result<ClassLevels, String>>>> = jobs.iter().map(|_| Mutex::new(None)).collect();
    {
        let next = AtomicUsize::new(0);
        let threads = std::thread::available_parallelism().map_or(2, |n| n.get()).clamp(1, 4);
        let (jobs, classes, chi_tabs, results) = (&jobs, &classes, &chi_tabs, &results);
        let next = &next;
        std::thread::scope(|s| {
            for _ in 0..threads {
                s.spawn(move || loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(job) = jobs.get(i) else { break };
                    let c = &classes[&job.class_id];
                    let special = c.d_k == -3 || c.d_k == -4;
                    let chi = kronecker(&BigInt::from(c.d_k), job.l);
                    let r = class_levels(c.p, &chi_tabs[&c.p], job.l, job.depth, chi, special, c.order, &c.curves);
                    *results[i].lock().unwrap() = Some(r);
                });
            }
        });
    }

    let mut existing: BTreeMap<(i64, u64), StoredLevel> = BTreeMap::new();
    for r in client.query(EXISTING_SQL, &[]).await? {
        existing.insert((r.get(0), r.get::<_, i64>(1) as u64), (r.get(2), r.get(3), r.get(4)));
    }
    let mut end_f: BTreeMap<i64, BigInt> = BTreeMap::new();
    for r in client.query(END_EXISTING_SQL, &[]).await? {
        if let Ok(f) = r.get::<_, String>(1).parse::<BigInt>() {
            end_f.insert(r.get(0), f);
        }
    }

    let mut certified: BTreeMap<(u64, i64), u32> = BTreeMap::new();
    let mut rows = Vec::new();
    let (mut agree_existing, mut agree_end) = (0u64, 0u64);
    for (job, res) in jobs.iter().zip(results) {
        let l = job.l;
        let res = res.into_inner().unwrap().expect("every job ran");
        let cl = match res {
            Ok(cl) => cl,
            Err(why) => {
                report.count(&format!("classes_refused_l{l}"), 1);
                report.problem(format!("class {}, l = {l}: {why}", job.class_id));
                continue;
            }
        };
        // cross-checks against what ecdb already holds
        let mut conflict = None;
        let (mut ok_existing, mut ok_end) = (0u64, 0u64);
        for (&id, &lv) in &cl.levels {
            if let Some(stored) = existing.get(&(id, l)) {
                match compare_stored(id, l, job.depth, lv, *stored) {
                    Ok(true) => ok_existing += 1,
                    Ok(false) => {}
                    Err(why) => conflict = Some(why),
                }
            }
            if let Some(f) = end_f.get(&id) {
                let mut f = f.clone();
                let mut e = 0u32;
                let lb = BigInt::from(l);
                while f != BigInt::from(0) && (&f % &lb) == BigInt::from(0) {
                    f /= &lb;
                    e += 1;
                }
                if e != lv {
                    conflict = Some(format!("curve {id}: v_{l}(f_End) = {e}, certified level {lv}"));
                } else {
                    ok_end += 1;
                }
            }
        }
        if let Some(why) = conflict {
            report.count(&format!("classes_refused_l{l}"), 1);
            report.problem(format!("class {}, l = {l}: {why}", job.class_id));
            continue;
        }
        agree_existing += ok_existing;
        agree_end += ok_end;
        report.count(&format!("classes_ok_l{l}"), 1);
        report.count("levels57_kernels_checked", cl.kernels);
        report.count("levels57_edges_checked", cl.edges);
        report.count("levels57_walk_steps", cl.walk_steps);
        let method = if job.depth == 0 {
            format!(
                "depth v_{l}(v) = 0: the crater; checked: 1 + (D_K/{l}) rational {l}-isogenies (kernels from psi_{l})"
            )
        } else {
            format!(
                "depth v_{l}(v) = {}; level = depth - shortest non-backtracking walk to the floor (floor: one rational \
                 {l}-isogeny), kernels from psi_{l}, codomain point counts, edge levels checked",
                job.depth
            )
        };
        for (&id, &lv) in &cl.levels {
            certified.insert((l, id), lv);
            rows.push(json!({ "curve_id": id, "ell": l, "depth": job.depth, "level": lv, "method": method }));
        }
    }
    report.count("levels57_agree_stored_level", agree_existing);
    report.count("levels57_agree_end_conductor", agree_end);

    // stored ℓ = 5 Vélu edges: levels differ by at most 1
    let mut bad_edges = 0u64;
    let mut stored_edges = 0u64;
    for r in client.query(EDGES_SQL, &[]).await? {
        let (s, t): (i64, i64) = (r.get(0), r.get(1));
        if let (Some(ls), Some(lt)) = (certified.get(&(5, s)), certified.get(&(5, t))) {
            stored_edges += 1;
            if ls.abs_diff(*lt) > 1 {
                bad_edges += 1;
                report.problem(format!("stored 5-isogeny {s} -> {t} joins certified levels {ls} and {lt}"));
            }
        }
    }
    report.count("levels57_stored_l5_edges_checked", stored_edges);
    if bad_edges > 0 {
        // a contradiction with the stored graph: write nothing at ℓ = 5
        report.count("levels57_stored_l5_edges_contradicting", bad_edges);
        rows.retain(|r| r["ell"].as_u64() != Some(5));
        certified.retain(|(l, _), _| *l != 5);
    }
    report.count("endomorphism_levels_l5_l7_written", bulk(client, LEVEL_SQL, &rows, prov).await?);

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn primes(lo: u64, hi: u64) -> Vec<u64> {
        (lo..hi).filter(|&n| n > 1 && (2..n).take_while(|d| d * d <= n).all(|d| n % d != 0)).collect()
    }

    /// Brute force: every monic degree-m polynomial over F_p that divides ψ_ℓ and
    /// whose root set is closed under x ↦ x([2]P) (2 generates (Z/ℓ)^*/±1 for ℓ = 5, 7).
    fn brute_kernels(f: &Fp, a: u64, b: u64, l: u64) -> BTreeSet<Poly> {
        let m = ((l - 1) / 2) as usize;
        let psi = f.monic(&division_polynomial(f, a, b, l));
        let (aa, bb) = (a as i64, b as i64);
        let num = f.poly(&[aa * aa, -8 * bb, -2 * aa, 0, 1]);
        let den = f.pscale(&cubic(f, a, b), 4);
        let mut out = BTreeSet::new();
        let total = f.p.pow(m as u32);
        for idx in 0..total {
            let mut h: Poly = (0..m).map(|i| (idx / f.p.pow(i as u32)) % f.p).collect();
            h.push(1);
            if !f.pmod(&psi, &h).is_empty() {
                continue;
            }
            // h(num/den) * den^m ≡ 0 mod h
            let n_ = f.pmod(&num, &h);
            let d_ = f.pmod(&den, &h);
            let mut acc: Poly = Vec::new();
            for (i, &c) in h.iter().enumerate() {
                let term = f.mulmod(&f.powmod(&n_, i as u64, &h), &f.powmod(&d_, (m - i) as u64, &h), &h);
                acc = f.padd(&acc, &f.pscale(&term, c));
            }
            if f.pmod(&acc, &h).is_empty() {
                out.insert(h);
            }
        }
        out
    }

    #[test]
    fn kernels_agree_with_brute_force_on_tiny_fields() {
        let mut seen = [0usize; 9];
        for p in primes(11, 48) {
            let f = Fp::new(p);
            let chi = chi_table(p);
            for a in 0..p {
                for b in 0..p {
                    if j_invariant(&f, a, b).is_none() {
                        continue;
                    }
                    let order = point_count(&f, &chi, a, b);
                    for l in [5u64, 7] {
                        // l = 7 above p = 29 (p^3 candidates each) only where 7^2 | t^2 - 4p,
                        // the classes that can have all 8 subgroups rational (p = 43, t = ±5)
                        let t = p as i64 + 1 - order as i64;
                        let deep7 = (t * t - 4 * p as i64) % 49 == 0;
                        if l == p || (l == 7 && p > 29 && !deep7) {
                            continue;
                        }
                        let got: BTreeSet<Poly> = rational_isogenies(&f, &chi, a, b, l, order)
                            .unwrap()
                            .into_iter()
                            .map(|n| n.kernel)
                            .collect();
                        let want = brute_kernels(&f, a, b, l);
                        assert_eq!(got, want, "p = {p}, a = {a}, b = {b}, l = {l}");
                        seen[got.len()] += 1;
                    }
                }
            }
        }
        // every possible count occurred: 0, 1, 2 and l + 1 (6 for l = 5, 8 for l = 7)
        assert!(seen[0] > 0 && seen[1] > 0 && seen[2] > 0 && seen[6] > 0 && seen[8] > 0, "{seen:?}");
    }

    #[test]
    fn multiplication_maps_match_point_arithmetic() {
        // x([2]P), x([3]P) against affine addition for every point of small curves
        let p = 43u64;
        let f = Fp::new(p);
        for (a, b) in [(1u64, 1u64), (2, 7), (5, 0), (0, 3)] {
            let add = |p1: Option<(u64, u64)>, p2: Option<(u64, u64)>| -> Option<(u64, u64)> {
                let (Some((x1, y1)), Some((x2, y2))) = (p1, p2) else { return p1.or(p2) };
                let lam = if x1 == x2 {
                    if f.add(y1, y2) == 0 {
                        return None;
                    }
                    f.mul(f.add(f.mul(3, f.mul(x1, x1)), a), f.inv(f.mul(2, y1)))
                } else {
                    f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1)))
                };
                let x3 = f.sub(f.sub(f.mul(lam, lam), x1), x2);
                Some((x3, f.sub(f.mul(lam, f.sub(x1, x3)), y1)))
            };
            for x in 0..p {
                for y in 1..p {
                    if f.mul(y, y) != f.eval(&cubic(&f, a, b), x) {
                        continue;
                    }
                    let pt = Some((x, y));
                    let p2 = add(pt, pt);
                    let p3 = add(p2, pt);
                    let g: Poly = vec![f.sub(0, x), 1];
                    if f.eval(&f3(&f, a, b), x) == 0 {
                        continue; // order 3: x([3]P) undefined
                    }
                    let xs = multiples(&f, a, b, &g, 3).unwrap();
                    assert_eq!(xs[1].first().copied().unwrap_or(0), p2.unwrap().0);
                    if let Some((x3, _)) = p3 {
                        assert_eq!(xs[2].first().copied().unwrap_or(0), x3);
                    }
                }
            }
        }
    }

    #[test]
    fn pinned_to_cremona_11a1_rational_5_torsion() {
        // 11a1 (LMFDB 11.a2): y^2 + y = x^3 - x^2 - 10x - 20 has rational 5-torsion
        // (5, 5), (16, 60); its short model y^2 = x^3 - 13392x - 1080432 (X = 36x - 12)
        // has the kernel polynomial (X - 168)(X - 564) for every good p.
        for p in [101u64, 103, 1009] {
            let f = Fp::new(p);
            let chi = chi_table(p);
            let (a, b) = (f.red(-13392), f.red(-1080432));
            let order = point_count(&f, &chi, a, b);
            assert_eq!(order % 5, 0, "rational 5-torsion divides #E");
            let want = f.pmul(&f.poly(&[-168, 1]), &f.poly(&[-564, 1]));
            let got = rational_isogenies(&f, &chi, a, b, 5, order).unwrap();
            assert!(got.iter().any(|n| n.kernel == want), "p = {p}: {got:?}");
        }
    }

    #[test]
    fn pinned_to_ecdb_velu_codomain_p101() {
        // generated.rs pins Sage's 3-isogeny of y^2 = x^3 + 2x + 3 over F_101: kernel x + 66 -> (97, 75)
        let f = Fp::new(101);
        assert_eq!(kohel_codomain(&f, 2, 3, &[66, 1]), (97, 75));
        assert!(f.pmod(&f.monic(&division_polynomial(&f, 2, 3, 3)), &[66, 1]).is_empty());
    }

    /// Levels by BFS from the floor in the complete brute-force graph of a class
    /// (the volcano.rs method), against the walks.
    #[test]
    fn walk_levels_match_graph_levels() {
        let mut checked = [0usize; 2];
        let mut deep = 0usize;
        // p = 499, t = 11: t^2 - 4p = 5^4 (-3), a depth-2 5-volcano (the deepest in the census:
        // depth 2 at l = 5 needs D_K in {-3, -4} below 2^10, and depth 1 is the most at l = 7)
        for p in primes(19, 140).into_iter().chain([499]) {
            let f = Fp::new(p);
            let chi = chi_table(p);
            // census: one curve per (j, order), a and b over the whole field
            let mut census: BTreeMap<(u64, u64), (u64, u64)> = BTreeMap::new();
            for a in 0..p {
                for b in 0..p {
                    if let Some(j) = j_invariant(&f, a, b) {
                        census.entry((point_count(&f, &chi, a, b), j)).or_insert((a, b));
                    }
                }
            }
            let mut by_order: BTreeMap<u64, Vec<(i64, u64, u64)>> = BTreeMap::new();
            for (i, (&(n, _), &(a, b))) in census.iter().enumerate() {
                by_order.entry(n).or_default().push((i as i64, a, b));
            }
            for (n, curves) in by_order {
                let t = p as i64 + 1 - n as i64;
                let disc = t * t - 4 * p as i64;
                if t % p as i64 == 0 || (p > 140 && disc % 625 != 0) {
                    continue;
                }
                // v^2 D_K = disc, D_K fundamental
                let mut v = 1i64;
                for q in 2..=((-disc) as f64).sqrt() as i64 + 1 {
                    while disc % (v * v * q * q) == 0 {
                        let d = disc / (v * v * q * q);
                        if d % 4 == 1 || d.rem_euclid(4) == 1 || (d % 4 == 0 && (d / 4).rem_euclid(4) >= 2) {
                            v *= q;
                        } else {
                            break;
                        }
                    }
                }
                let d_k = disc / (v * v);
                assert!(d_k.rem_euclid(4) == 1 || (d_k % 4 == 0 && (d_k / 4).rem_euclid(4) >= 2), "D_K {d_k}");
                for (k, l) in [5u64, 7].into_iter().enumerate() {
                    if l == p {
                        continue;
                    }
                    let depth = val(v, l as i64);
                    let chi_l = kronecker(&BigInt::from(d_k), l);
                    let special = d_k == -3 || d_k == -4;
                    let got = class_levels(p, &chi, l, depth, chi_l, special, n, &curves).unwrap();
                    // graph: BFS from the floor over j-invariants
                    let js: BTreeMap<i64, u64> =
                        curves.iter().map(|&(id, a, b)| (id, j_invariant(&f, a, b).unwrap())).collect();
                    let mut adj: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
                    for &(id, a, b) in &curves {
                        let nb = rational_isogenies(&f, &chi, a, b, l, n).unwrap();
                        adj.insert(js[&id], nb.iter().map(|x| x.j).collect());
                    }
                    let mut dist: BTreeMap<u64, u32> = BTreeMap::new();
                    let mut q = std::collections::VecDeque::new();
                    for (j, nb) in &adj {
                        if depth > 0 && nb.len() == 1 {
                            dist.insert(*j, 0);
                            q.push_back(*j);
                        }
                    }
                    while let Some(x) = q.pop_front() {
                        let dx = dist[&x];
                        for y in adj[&x].clone() {
                            if let std::collections::btree_map::Entry::Vacant(e) = dist.entry(y) {
                                e.insert(dx + 1);
                                q.push_back(y);
                            }
                        }
                    }
                    for (id, lv) in &got.levels {
                        let want = if depth == 0 { 0 } else { depth - dist[&js[id]] };
                        assert_eq!(*lv, want, "p = {p}, #E = {n}, l = {l}");
                    }
                    if depth >= 1 {
                        checked[k] += 1;
                    }
                    if depth >= 2 {
                        deep += 1;
                    }
                }
            }
        }
        assert!(checked[0] > 0 && checked[1] > 0 && deep > 0, "{checked:?}, depth >= 2: {deep}");
    }

    #[test]
    fn crosscheck_rejects_corrupted_input() {
        // a class whose stored order is wrong for one curve is refused
        let p = 101u64;
        let f = Fp::new(p);
        let chi = chi_table(p);
        let (a, b) = (f.red(-13392), f.red(-1080432));
        let n = point_count(&f, &chi, a, b);
        assert!(class_levels(p, &chi, 5, 0, 0, false, n + 1, &[(1, a, b)]).is_err());
        // a wrong depth contradicts the rational-isogeny count (11a1 has 1 or 2, not 6)
        let r = class_levels(p, &chi, 5, 2, 0, false, n, &[(1, a, b)]);
        assert!(r.is_err(), "{r:?}");
        // a codomain outside the given class (an incomplete census) is refused
        let nb = rational_isogenies(&f, &chi, a, b, 5, n).unwrap();
        let d_k_chi = nb.len() as i64 - 1;
        assert!(class_levels(p, &chi, 5, 0, d_k_chi, false, n, &[(1, a, b)]).err().is_some_and(|e| e
            .contains("not a census curve")
            || nb.iter().all(|x| x.j == j_invariant(&f, a, b).unwrap())));
        // a corrupted kernel fails the point-count identity: shift the codomain's b
        let k = &nb[0];
        let (ca, cb) = kohel_codomain(&f, a, b, &k.kernel);
        assert_eq!((ca, cb), (k.a, k.b));
        assert_ne!(point_count(&f, &chi, ca, f.add(cb, 1)), n);
    }

    #[test]
    fn valuation_and_counts() {
        assert_eq!(val(175, 5), 2);
        assert_eq!(val(49 * 3, 7), 2);
        assert_eq!(val(12, 7), 0);
        let f = Fp::new(13);
        assert_eq!(division_polynomial(&f, 1, 1, 5).len(), 13);
        assert_eq!(division_polynomial(&f, 1, 1, 7).len(), 25);
    }

    #[test]
    fn stored_invariants_must_match_the_trace() {
        // p = 499, t = 11: t^2 - 4p = -1875 = 25^2 * (-3) (the depth-2 5-volcano)
        assert!(check_invariants(499, 11, 25, -3).is_ok());
        // p = 101, t = 2: -400 = 10^2 * (-4)
        assert!(check_invariants(101, 2, 10, -4).is_ok());
        // a v with an extra factor 5 (a fake depth) or a D_K that is not fundamental is refused,
        // even when v^2 D_K still equals t^2 - 4p
        assert!(check_invariants(499, 11, 5, -75).is_err());
        assert!(check_invariants(101, 2, 5, -16).is_err());
        assert!(check_invariants(101, 2, 50, -4).is_err());
        assert!(check_invariants(101, 2, 10, -3).is_err());
        // supersingular, Hasse bound, v = 0
        assert!(check_invariants(101, 0, 2, -101).is_err());
        assert!(check_invariants(101, 21, 1, -3).is_err());
        assert!(check_invariants(101, 2, 0, -4).is_err());
    }

    #[test]
    fn stored_rows_agree_or_are_refused() {
        // nothing stored but a bound at or above the level, depth NULL or equal: may be filled
        assert_eq!(compare_stored(1, 5, 1, 1, (None, Some(1), Some(1))), Ok(false));
        assert_eq!(compare_stored(1, 5, 1, 0, (None, Some(1), None)), Ok(false));
        assert_eq!(compare_stored(1, 5, 1, 1, (None, None, None)), Ok(false));
        // the same exact level (depth NULL as walks.rs stores it, or equal)
        assert_eq!(compare_stored(1, 5, 2, 1, (Some(1), None, None)), Ok(true));
        assert_eq!(compare_stored(1, 5, 2, 1, (Some(1), None, Some(2))), Ok(true));
        // conflicts: a different level, a bound below the level, a different depth (regression:
        // a level_at_most row with a conflicting depth used to have its depth overwritten)
        assert!(compare_stored(1, 5, 2, 1, (Some(2), None, None)).is_err());
        assert!(compare_stored(1, 5, 2, 2, (None, Some(1), None)).is_err());
        assert!(compare_stored(1, 5, 1, 1, (None, Some(1), Some(0))).is_err());
        assert!(compare_stored(1, 5, 1, 1, (Some(1), None, Some(0))).is_err());
    }
}
