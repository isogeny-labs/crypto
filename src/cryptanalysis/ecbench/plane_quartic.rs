//! Jacobians of smooth plane quartics over `F_p`: the non-hyperelliptic
//! curves of genus 3, as ecbench workloads.
//!
//! # The curve and its section spaces
//!
//! `C : F(X, Y, Z) = 0`, `F` a quartic form with the coefficient of `X⁴`
//! scaled to 1, smooth over `F̄_p` (checked by resultants; see
//! [`is_smooth`]).  A smooth plane quartic is its own canonical model, so
//! the line sections are the canonical class `K` and the sections of `nK`
//! are the forms of degree `n` modulo `F`:
//!
//! ```text
//!   S_n = H⁰(C, nK) = F_p[X, Y, Z]_n / F·F_p[X, Y, Z]_{n−4},   dim S_n = 4n − 2  (n ≥ 2)
//! ```
//!
//! with the normal-form basis of monomials whose `X`-degree is at most 3
//! (reduce `X⁴ ↦ X⁴ − F`).
//!
//! # The group law: linear algebra on Riemann–Roch spaces
//!
//! Following Khuri-Makdisi (*Linear algebra algorithms for divisors on an
//! algebraic curve*, Math. Comp. 73 (2004)), an effective divisor `D` is
//! handled through the subspace `H⁰(nK − D) ⊂ S_n` of forms vanishing on
//! it, and only three operations are needed:
//!
//! - **product**: `H⁰(L₁ − D)·H⁰(L₂ − E) = H⁰(L₁ + L₂ − D − E)` when the
//!   two bundles have degrees `≥ 2g` and `≥ 2g + 1` (`g = 3`);
//! - **colon**: `{s ∈ S_a : s·H⁰(M) ⊂ H⁰(aK + M − B)} = H⁰(aK − B)` when
//!   `M` is base-point free (degree `≥ 2g`);
//! - **flip**: for `f ∈ H⁰(nK − D)`, `div f = D + D′` with `D′ ~ nK − D`.
//!
//! Multiplicities (repeated points, points shared between two divisors)
//! are carried by the products and colons, never by evaluation, so
//! doubling and degenerate sums need no special case.
//!
//! A class `x ∈ Pic⁰(C)` is represented by an effective divisor `E` of
//! degree 3 with `E ~ D∞ + x`, where `D∞` is three fixed non-collinear
//! rational points.  When `h⁰(D∞ + x) = 1` (all but about `1/p` of the
//! classes) `E` is unique; otherwise `|E|` is a pencil (a `g¹₃`, residual
//! to a point in the line sections through it) and the canonical member is
//! the one through a fixed fourth rational point `P₁`.  The element stored
//! is the annihilator of `H⁰(3K − E) ⊂ S_3` in reduced row-echelon form: a
//! function of the class alone, so equality of classes is equality of
//! representations and [`CountedGroup::key`] is a hash of it.
//!
//! Addition `x + y` (two flips, `E_x + E_y ↦ A ↦ G`):
//!
//! ```text
//!   H⁰(6K − E_x − E_y) = H⁰(3K − E_x)·H⁰(3K − E_y)           (product)
//!   f ∈ it, div f = E_x + E_y + A                               (flip, deg A = 18)
//!   T  = H⁰(3K − E_x − E_y)  = {t ∈ S_3 : t·S_3 ⊂ H⁰(6K − E_x − E_y)}
//!   Q  = H⁰(6K − A − D∞)     = {h ∈ S_6 : h·T ⊂ f·H⁰(3K − D∞)}      (colon)
//!   h ∈ Q (Q ∩ {G ∋ P₁} if dim Q = 2), div h = A + D∞ + G      (flip, G ~ E_x + E_y − D∞)
//!   U  = H⁰(7K − A − D∞)     = {u ∈ S_7 : u·T ⊂ f·H⁰(4K − D∞)}
//!   H⁰(3K − G)                = {t ∈ S_3 : t·U ⊂ h·S_4}
//! ```
//!
//! Negation is one flip against a fixed divisor `B` cut by a fixed `c ∈
//! H⁰(6K − 2D∞)`: `div c = 2D∞ + B`, `f ∈ H⁰(6K − E_x − B)`, `div f = E_x +
//! B + G`, `G ~ 2D∞ − E_x`.  It costs about as much as an addition, so the
//! negation map is **not** free on this group, and the methods that use it
//! for nothing (`rho.negation`, `bsgs.negation`) are refused here.
//!
//! # The group order
//!
//! `#J(F_p) = L(1)` for `L(T) = 1 + a₁T + a₂T² + a₃T³ + p a₂T⁴ + p² a₁T⁵ +
//! p³T⁶`.  `a₁, a₂` come from point counts over `F_p` and `F_{p²}`; `a₃`
//! (`|a₃| ≤ 20 p^{3/2}`) is found by baby-step giant-step on element orders
//! in that window, and accepted only when the least common multiple of the
//! orders of several random classes leaves exactly one candidate.  The unit
//! tests check the order against `L(1)` from a third point count over
//! `F_{p³}` on small fields, where that count is affordable.

use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::cryptanalysis::ecbench::canonical::{sha256_hex, splitmix64};
use crate::cryptanalysis::ecbench::jacobian::prime_part;
use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
use crate::cryptanalysis::residual_walk::is_prime_u64;

/// Smallest and largest primes the searched family accepts.  Below 29 a
/// quartic can have too few rational points for `D∞` and `P₁`; above
/// `2^11` the point count over `F_{p²}` (`p²` abscissae, each a root
/// count) dominates a search.
pub const QUARTIC_MIN_P: u64 = 29;
pub const QUARTIC_MAX_P: u64 = 1 << 11;

/// The largest prime the group law itself accepts (a curve whose points
/// are supplied, [`QuarticJacobian::with_points`]): products of two
/// residues fit a `u64`, and a class packs into 32-bit entries.
pub const QUARTIC_LAW_MAX_P: u64 = 1 << 31;

// ── F_p ──────────────────────────────────────────────────────────────

thread_local! {
    static MULS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// `F_p` multiplications this thread has performed in this module (an
/// inversion is its Fermat exponentiation's multiplications).  A transfer
/// reads differences of it to price Jacobian work in the source curve's
/// field-multiplication unit.
pub fn field_muls() -> u64 {
    MULS.with(|m| m.get())
}

#[inline]
pub(crate) fn mulm(a: u64, b: u64, p: u64) -> u64 {
    MULS.with(|m| m.set(m.get() + 1));
    a * b % p
}

fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}

#[inline]
pub(crate) fn invm(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
}

// ── Linear algebra over F_p ──────────────────────────────────────────

/// Reduced row-echelon form of `rows` (length-`n` vectors), zero rows
/// dropped.
fn rref(mut rows: Vec<Vec<u64>>, n: usize, p: u64) -> Vec<Vec<u64>> {
    let mut r = 0;
    for col in 0..n {
        let Some(piv) = (r..rows.len()).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(r, piv);
        let inv = invm(rows[r][col], p);
        for v in rows[r].iter_mut() {
            *v = mulm(*v, inv, p);
        }
        let pivot_row = rows[r].clone();
        for (i, row) in rows.iter_mut().enumerate() {
            if i != r && row[col] != 0 {
                let c = row[col];
                for (x, &y) in row.iter_mut().zip(&pivot_row) {
                    *x = (*x + p - mulm(c, y, p)) % p;
                }
            }
        }
        r += 1;
        if r == rows.len() {
            break;
        }
    }
    rows.truncate(r);
    rows
}

/// A basis of `{v : M v = 0}` for `M` with rows of length `n`, from the
/// RREF of `M`: one vector per free column.
fn nullspace(m: Vec<Vec<u64>>, n: usize, p: u64) -> Vec<Vec<u64>> {
    let e = rref(m, n, p);
    let pivots: Vec<usize> = e
        .iter()
        .map(|row| row.iter().position(|&v| v != 0).unwrap())
        .collect();
    let mut out = Vec::new();
    for free in (0..n).filter(|c| !pivots.contains(c)) {
        let mut v = vec![0u64; n];
        v[free] = 1;
        for (row, &pc) in e.iter().zip(&pivots) {
            v[pc] = (p - row[free]) % p;
        }
        out.push(v);
    }
    out
}

// ── Monomials and normal forms ───────────────────────────────────────

/// The exponents `(a, b, c)` of `X^a Y^b Z^c`, `a + b + c = n`, in a fixed
/// order: `a` descending, then `b` descending.
pub fn monomials(n: usize) -> Vec<(u8, u8, u8)> {
    let mut v = Vec::new();
    for a in (0..=n).rev() {
        for b in (0..=n - a).rev() {
            v.push((a as u8, b as u8, (n - a - b) as u8));
        }
    }
    v
}

/// The normal-form basis of `S_n`: monomials of degree `n`, `X`-degree ≤ 3.
pub fn basis(n: usize) -> Vec<(u8, u8, u8)> {
    monomials(n).into_iter().filter(|m| m.0 <= 3).collect()
}

pub fn dim(n: usize) -> usize {
    basis(n).len()
}

/// The highest degree any operation of the group law reaches.
const MAX_DEG: usize = 10;

/// Normal forms of every monomial of degree ≤ [`MAX_DEG`], and the index
/// of each normal-form basis monomial.
struct Forms {
    p: u64,
    /// `basis[n]`: the basis of `S_n`.
    basis: Vec<Vec<(u8, u8, u8)>>,
    /// `nf[n][(a, b, c)]`: the normal form of any monomial of degree `n`,
    /// as `(basis index, coefficient)` pairs.
    nf: Vec<HashMap<(u8, u8, u8), Vec<(usize, u64)>>>,
}

impl Forms {
    fn new(p: u64, f: &[u64; 15]) -> Forms {
        let mons4 = monomials(4);
        let mut basis_v = Vec::new();
        let mut index: Vec<HashMap<(u8, u8, u8), usize>> = Vec::new();
        for n in 0..=MAX_DEG {
            let b = basis(n);
            index.push(b.iter().enumerate().map(|(i, &m)| (m, i)).collect());
            basis_v.push(b);
        }
        let mut nf: Vec<HashMap<(u8, u8, u8), Vec<(usize, u64)>>> = Vec::new();
        for n in 0..=MAX_DEG {
            let mut table: HashMap<(u8, u8, u8), Vec<(usize, u64)>> = HashMap::new();
            // Increasing X-degree: X^a·rest reduces to terms of X-degree < a.
            let mut ms = monomials(n);
            ms.sort_by_key(|m| m.0);
            for m in ms {
                let v = if m.0 <= 3 {
                    vec![(index[n][&m], 1)]
                } else {
                    // X^a Y^b Z^c = X^{a−4} Y^b Z^c · X⁴, X⁴ ≡ −(F − X⁴).
                    let mut acc: HashMap<usize, u64> = HashMap::new();
                    for (k, &mm) in mons4.iter().enumerate() {
                        if mm == (4, 0, 0) || f[k] == 0 {
                            continue;
                        }
                        let prod = (m.0 - 4 + mm.0, m.1 + mm.1, m.2 + mm.2);
                        let c = (p - f[k]) % p;
                        for &(i, v) in &table[&prod] {
                            let e = acc.entry(i).or_insert(0);
                            *e = (*e + mulm(c, v, p)) % p;
                        }
                    }
                    let mut v: Vec<(usize, u64)> = acc.into_iter().filter(|e| e.1 != 0).collect();
                    v.sort_unstable();
                    v
                };
                table.insert(m, v);
            }
            nf.push(table);
        }
        Forms {
            p,
            basis: basis_v,
            nf,
        }
    }

    fn dim(&self, n: usize) -> usize {
        self.basis[n].len()
    }

    /// `u·v` for `u ∈ S_a`, `v ∈ S_b`, in `S_{a+b}`.
    fn mul(&self, u: &[u64], a: usize, v: &[u64], b: usize) -> Vec<u64> {
        let p = self.p;
        let n = a + b;
        let mut out = vec![0u64; self.dim(n)];
        for (i, &ui) in u.iter().enumerate() {
            if ui == 0 {
                continue;
            }
            let mi = self.basis[a][i];
            for (j, &vj) in v.iter().enumerate() {
                if vj == 0 {
                    continue;
                }
                let mj = self.basis[b][j];
                let c = mulm(ui, vj, p);
                for &(k, w) in &self.nf[n][&(mi.0 + mj.0, mi.1 + mj.1, mi.2 + mj.2)] {
                    out[k] = (out[k] + mulm(c, w, p)) % p;
                }
            }
        }
        out
    }

    /// The span of all products, reduced.
    fn product(&self, us: &[Vec<u64>], a: usize, vs: &[Vec<u64>], b: usize) -> Vec<Vec<u64>> {
        let mut rows = Vec::with_capacity(us.len() * vs.len());
        for u in us {
            for v in vs {
                rows.push(self.mul(u, a, v, b));
            }
        }
        rref(rows, self.dim(a + b), self.p)
    }

    /// `{t ∈ S_a : ann·(t·u) = 0 ∀ u ∈ us}`, `ann` the annihilator rows of
    /// a subspace of `S_{a+b}` and `us ⊂ S_b`.
    fn colon(&self, ann: &[Vec<u64>], us: &[Vec<u64>], a: usize, b: usize) -> Vec<Vec<u64>> {
        let p = self.p;
        let da = self.dim(a);
        let mut rows = Vec::with_capacity(ann.len() * us.len());
        let unit = |i: usize| -> Vec<u64> {
            let mut e = vec![0u64; da];
            e[i] = 1;
            e
        };
        for u in us {
            // Column i: the product of the i-th basis monomial with u.
            let cols: Vec<Vec<u64>> = (0..da).map(|i| self.mul(&unit(i), a, u, b)).collect();
            for arow in ann {
                let row: Vec<u64> = cols
                    .iter()
                    .map(|c| {
                        c.iter()
                            .zip(arow)
                            .fold(0u64, |s, (&x, &y)| (s + mulm(x, y, p)) % p)
                    })
                    .collect();
                rows.push(row);
            }
        }
        nullspace(rows, da, p)
    }

    /// The annihilator rows of a subspace of `S_n`.
    fn ann(&self, rows: &[Vec<u64>], n: usize) -> Vec<Vec<u64>> {
        nullspace(rows.to_vec(), self.dim(n), self.p)
    }

    /// The evaluation functional of `S_n` at the projective point `pt`.
    fn eval_row(&self, pt: [u64; 3], n: usize) -> Vec<u64> {
        let p = self.p;
        self.basis[n]
            .iter()
            .map(|&(a, b, c)| {
                mulm(
                    mulm(powm(pt[0], a.into(), p), powm(pt[1], b.into(), p), p),
                    powm(pt[2], c.into(), p),
                    p,
                )
            })
            .collect()
    }

    /// `H⁰(nK − Σ P_i)` for distinct rational points.
    fn vanishing(&self, pts: &[[u64; 3]], n: usize) -> Vec<Vec<u64>> {
        let rows = pts.iter().map(|&q| self.eval_row(q, n)).collect();
        nullspace(rows, self.dim(n), self.p)
    }

    fn full(&self, n: usize) -> Vec<Vec<u64>> {
        (0..self.dim(n))
            .map(|i| {
                let mut e = vec![0u64; self.dim(n)];
                e[i] = 1;
                e
            })
            .collect()
    }
}

// ── The quartic, its points and smoothness ───────────────────────────

/// Evaluate a quartic form (coefficients in [`monomials`]`(4)` order).
fn eval_form(f: &[u64], n: usize, pt: [u64; 3], p: u64) -> u64 {
    monomials(n).iter().zip(f).fold(0, |s, (&(a, b, c), &k)| {
        if k == 0 {
            return s;
        }
        let t = mulm(
            mulm(powm(pt[0], a.into(), p), powm(pt[1], b.into(), p), p),
            powm(pt[2], c.into(), p),
            p,
        );
        (s + mulm(k, t, p)) % p
    })
}

/// The partial derivatives of a quartic form, as cubic forms.
fn partials(f: &[u64; 15], p: u64) -> [Vec<u64>; 3] {
    let m4 = monomials(4);
    let m3 = monomials(3);
    let idx3: HashMap<(u8, u8, u8), usize> = m3.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    let mut out = [vec![0u64; 10], vec![0u64; 10], vec![0u64; 10]];
    for (k, &(a, b, c)) in m4.iter().enumerate() {
        let e = [a, b, c];
        for v in 0..3 {
            if e[v] == 0 {
                continue;
            }
            let mut d = e;
            d[v] -= 1;
            let i = idx3[&(d[0], d[1], d[2])];
            out[v][i] = (out[v][i] + mulm(f[k], u64::from(e[v]), p)) % p;
        }
    }
    out
}

/// Polynomials over F_p, lowest degree first.
fn ptrim(mut a: Vec<u64>) -> Vec<u64> {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}

fn pgcd(mut a: Vec<u64>, mut b: Vec<u64>, p: u64) -> Vec<u64> {
    a = ptrim(a);
    b = ptrim(b);
    while !b.is_empty() {
        let r = prem(&a, &b, p);
        a = b;
        b = r;
    }
    a
}

fn prem(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut r = a.to_vec();
    let db = b.len() - 1;
    let inv = invm(b[db], p);
    while r.len() > db && !r.is_empty() {
        let c = mulm(*r.last().unwrap(), inv, p);
        let s = r.len() - 1 - db;
        for (i, &bi) in b.iter().enumerate() {
            r[s + i] = (r[s + i] + p - mulm(c, bi, p)) % p;
        }
        r = ptrim(r);
    }
    r
}

fn det(mut m: Vec<Vec<u64>>, p: u64) -> u64 {
    let n = m.len();
    let mut d = 1u64;
    for col in 0..n {
        let Some(piv) = (col..n).find(|&i| m[i][col] != 0) else {
            return 0;
        };
        if piv != col {
            m.swap(piv, col);
            d = (p - d) % p;
        }
        d = mulm(d, m[col][col], p);
        let inv = invm(m[col][col], p);
        for i in col + 1..n {
            let c = mulm(m[i][col], inv, p);
            if c != 0 {
                for j in col..n {
                    m[i][j] = (m[i][j] + p - mulm(c, m[col][j], p)) % p;
                }
            }
        }
    }
    d
}

/// The Sylvester resultant of two polynomials in `y` (lowest first).
fn resultant(a: &[u64], b: &[u64], p: u64) -> u64 {
    let (m, n) = (a.len() - 1, b.len() - 1);
    let size = m + n;
    let mut s = vec![vec![0u64; size]; size];
    for i in 0..n {
        for (j, &c) in a.iter().rev().enumerate() {
            s[i][i + j] = c;
        }
    }
    for i in 0..m {
        for (j, &c) in b.iter().rev().enumerate() {
            s[n + i][i + j] = c;
        }
    }
    det(s, p)
}

/// Lagrange interpolation through `(x_i, y_i)`.
fn interpolate(xs: &[u64], ys: &[u64], p: u64) -> Vec<u64> {
    let n = xs.len();
    let mut out = vec![0u64; n];
    for i in 0..n {
        let mut num = vec![1u64];
        let mut den = 1u64;
        for j in 0..n {
            if j == i {
                continue;
            }
            let mut next = vec![0u64; num.len() + 1];
            for (k, &c) in num.iter().enumerate() {
                next[k + 1] = (next[k + 1] + c) % p;
                next[k] = (next[k] + mulm(c, (p - xs[j]) % p, p)) % p;
            }
            num = next;
            den = mulm(den, (xs[i] + p - xs[j]) % p, p);
        }
        let c = mulm(ys[i], invm(den, p), p);
        for (k, &v) in num.iter().enumerate() {
            out[k] = (out[k] + mulm(c, v, p)) % p;
        }
    }
    ptrim(out)
}

/// A cubic form's restriction to `Z = 1` as a polynomial in `y` whose
/// coefficients are evaluated at `x`.
fn cubic_in_y(g: &[u64], x: u64, p: u64) -> Vec<u64> {
    let mut out = vec![0u64; 4];
    for (k, &(a, b, _)) in monomials(3).iter().enumerate() {
        out[b as usize] = (out[b as usize] + mulm(g[k], powm(x, a.into(), p), p)) % p;
    }
    out
}

/// Whether `F = 0` is smooth over `F̄_p`, conservatively: a singular point
/// is a common zero of the three partials (Euler, `p ∤ 4`).  On `Z = 1`
/// the abscissae of common zeros divide `gcd(Res_y(F_X, F_Y), Res_y(F_X,
/// F_Z))`, which has degree ≤ 9; the `y³` coefficients of the partials are
/// constants and must be nonzero so that the resultants specialise.  On
/// `Z = 0` the binary cubics must have no common zero.  Any common root of
/// the resultants is treated as singular, so a smooth curve may be
/// refused, never a singular one accepted.
pub fn is_smooth(f: &[u64; 15], p: u64) -> bool {
    let g = partials(f, p);
    let y3 = monomials(3).iter().position(|&m| m == (0, 3, 0)).unwrap();
    if g.iter().any(|gi| gi[y3] == 0) {
        return false;
    }
    let xs: Vec<u64> = (0..13u64).collect();
    let r = |a: &[u64], b: &[u64]| -> Vec<u64> {
        let ys: Vec<u64> = xs
            .iter()
            .map(|&x| resultant(&cubic_in_y(a, x, p), &cubic_in_y(b, x, p), p))
            .collect();
        interpolate(&xs, &ys, p)
    };
    let r12 = r(&g[0], &g[1]);
    let r13 = r(&g[0], &g[2]);
    if r12.is_empty() || r13.is_empty() || pgcd(r12, r13, p).len() > 1 {
        return false;
    }
    // Z = 0: (1 : 0 : 0) and (x : 1 : 0).
    let at = |gi: &[u64], pt: [u64; 3]| eval_form(gi, 3, pt, p);
    if g.iter().all(|gi| at(gi, [1, 0, 0]) == 0) {
        return false;
    }
    let inf = |gi: &[u64]| -> Vec<u64> {
        let mut out = vec![0u64; 4];
        for (k, &(a, _, c)) in monomials(3).iter().enumerate() {
            if c == 0 {
                out[a as usize] = (out[a as usize] + gi[k]) % p;
            }
        }
        ptrim(out)
    };
    let common = pgcd(pgcd(inf(&g[0]), inf(&g[1]), p), inf(&g[2]), p);
    common.len() <= 1
}

// ── Point counting over F_p and F_{p²} ───────────────────────────────

/// `F_{p^k}` for `k ≤ 3` as `F_p[t]/(m(t))`, `m` monic of degree `k`
/// without roots in `F_p` (irreducible for `k ≤ 3`).
#[derive(Clone)]
struct Ext {
    p: u64,
    k: usize,
    /// `t^k = Σ red[i] t^i`.
    red: Vec<u64>,
}

type E = [u64; 3];

impl Ext {
    fn new(p: u64, k: usize) -> Ext {
        if k == 1 {
            return Ext { p, k, red: vec![0] };
        }
        for seed in 1u64.. {
            let mut m = vec![0u64; k + 1];
            let mut s = seed;
            for c in m.iter_mut().take(k) {
                s = splitmix64(s);
                *c = s % p;
            }
            m[k] = 1;
            let has_root = (0..p).any(|x| {
                let mut v = 0u64;
                for &c in m.iter().rev() {
                    v = (mulm(v, x, p) + c) % p;
                }
                v == 0
            });
            if !has_root {
                return Ext {
                    p,
                    k,
                    red: m[..k].iter().map(|&c| (p - c) % p).collect(),
                };
            }
        }
        unreachable!()
    }
    fn q(&self) -> u64 {
        self.p.pow(self.k as u32)
    }
    fn element(&self, mut i: u64) -> E {
        let mut e = [0u64; 3];
        for c in e.iter_mut().take(self.k) {
            *c = i % self.p;
            i /= self.p;
        }
        e
    }
    fn add(&self, a: E, b: E) -> E {
        let p = self.p;
        [(a[0] + b[0]) % p, (a[1] + b[1]) % p, (a[2] + b[2]) % p]
    }
    fn mul(&self, a: E, b: E) -> E {
        let p = self.p;
        let k = self.k;
        let mut t = [0u64; 5];
        for i in 0..k {
            for j in 0..k {
                t[i + j] = (t[i + j] + mulm(a[i], b[j], p)) % p;
            }
        }
        for d in (k..2 * k - 1).rev() {
            let c = t[d];
            t[d] = 0;
            for (i, &r) in self.red.iter().enumerate() {
                t[d - k + i] = (t[d - k + i] + mulm(c, r, p)) % p;
            }
        }
        [t[0], t[1], t[2]]
    }
    fn is_zero(&self, a: E) -> bool {
        a == [0, 0, 0]
    }
    fn inv(&self, a: E) -> E {
        // a^(q − 2).
        let mut r = self.one();
        let mut b = a;
        let mut e = self.q() - 2;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    fn one(&self) -> E {
        [1, 0, 0]
    }
    fn scalar(&self, c: u64) -> E {
        [c % self.p, 0, 0]
    }
    fn neg(&self, a: E) -> E {
        let p = self.p;
        [(p - a[0]) % p, (p - a[1]) % p, (p - a[2]) % p]
    }
}

/// Polynomials over `Ext`, lowest first.
fn ext_trim(e: &Ext, mut a: Vec<E>) -> Vec<E> {
    while a.last().is_some_and(|&c| e.is_zero(c)) {
        a.pop();
    }
    a
}

fn ext_mulmod(e: &Ext, a: &[E], b: &[E], m: &[E]) -> Vec<E> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut t = vec![[0u64; 3]; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            t[i + j] = e.add(t[i + j], e.mul(x, y));
        }
    }
    ext_rem(e, t, m)
}

fn ext_rem(e: &Ext, a: Vec<E>, m: &[E]) -> Vec<E> {
    let mut r = ext_trim(e, a);
    let dm = m.len() - 1;
    let inv = if m[dm] == e.one() {
        e.one()
    } else {
        e.inv(m[dm])
    };
    while r.len() > dm {
        let c = e.mul(*r.last().unwrap(), inv);
        let s = r.len() - 1 - dm;
        for (i, &mi) in m.iter().enumerate() {
            r[s + i] = e.add(r[s + i], e.neg(e.mul(c, mi)));
        }
        r = ext_trim(e, r);
    }
    r
}

/// Number of distinct roots in `F_q` of `g` (lowest first, nonzero):
/// `deg gcd(g, y^q − y)`.
fn ext_root_count(e: &Ext, g: Vec<E>) -> u64 {
    let g = ext_trim(e, g);
    if g.len() <= 1 {
        return 0;
    }
    // Monic, so that every reduction below divides by 1.
    let lead = e.inv(*g.last().unwrap());
    let g: Vec<E> = g.into_iter().map(|c| e.mul(c, lead)).collect();
    // y^q mod g by repeated squaring.
    let mut acc = vec![e.one()];
    let mut base = ext_rem(e, vec![[0, 0, 0], e.one()], &g);
    let mut q = e.q();
    while q > 0 {
        if q & 1 == 1 {
            acc = ext_mulmod(e, &acc, &base, &g);
        }
        base = ext_mulmod(e, &base, &base, &g);
        q >>= 1;
    }
    // acc − y
    let mut h = acc;
    if h.len() < 2 {
        h.resize(2, [0, 0, 0]);
    }
    h[1] = e.add(h[1], e.neg(e.one()));
    let h = ext_trim(e, h);
    if h.is_empty() {
        return (g.len() - 1) as u64;
    }
    // gcd(g, h)
    let mut a = g;
    let mut b = h;
    while !b.is_empty() {
        let r = ext_rem(e, a, &b);
        a = b;
        b = r;
    }
    (a.len() - 1) as u64
}

/// `#C(F_{p^k})`, `k ≤ 3`: for each `x ∈ F_q` the distinct roots `y` of
/// `F(x, y, 1)`, plus the points `(x : 1 : 0)` (`(1 : 0 : 0)` is never on
/// `C`: the `X⁴` coefficient is 1).
pub fn count_points(f: &[u64; 15], p: u64, k: usize) -> u64 {
    let e = Ext::new(p, k);
    let m4 = monomials(4);
    let q = e.q();
    let pow = |x: E, n: u8| -> E {
        let mut r = e.one();
        for _ in 0..n {
            r = e.mul(r, x);
        }
        r
    };
    let mut total = 0u64;
    for xi in 0..q {
        let x = e.element(xi);
        let mut g = vec![[0u64; 3]; 5];
        for (kk, &(a, b, _)) in m4.iter().enumerate() {
            if f[kk] != 0 {
                g[b as usize] = e.add(g[b as usize], e.mul(e.scalar(f[kk]), pow(x, a)));
            }
        }
        let g = ext_trim(&e, g);
        if g.is_empty() {
            total += q;
        } else {
            total += ext_root_count(&e, g);
        }
    }
    // Z = 0, Y = 1: F(x, 1, 0).
    let mut g = vec![[0u64; 3]; 5];
    for (kk, &(a, _, c)) in m4.iter().enumerate() {
        if c == 0 && f[kk] != 0 {
            g[a as usize] = e.add(g[a as usize], e.scalar(f[kk]));
        }
    }
    total + ext_root_count(&e, g)
}

// ── The Jacobian ─────────────────────────────────────────────────────

/// A divisor class, stored as the annihilator of `H⁰(3K − E) ⊂ S_3` in
/// reduced row-echelon form (3 rows of 10): a function of the class.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QClass(pub [u32; 30]);

/// The Jacobian of a smooth plane quartic, with everything the group law
/// precomputes: `D∞`, `P₁`, `H⁰(3K − D∞)`, `H⁰(4K − D∞)`, `H⁰(3K − D∞ −
/// P₁)`, `H⁰(3K − 2D∞)` and the fixed `c ∈ H⁰(6K − 2D∞)`.
pub struct QuarticJacobian {
    pub p: u64,
    pub f: [u64; 15],
    forms: Forms,
    pub d_inf: [[u64; 3]; 3],
    pub p1: [u64; 3],
    pub points: Vec<[u64; 3]>,
    r3: Vec<Vec<u64>>,
    r4: Vec<Vec<u64>>,
    r3_p1: Vec<Vec<u64>>,
    t2: Vec<Vec<u64>>,
    c: Vec<u64>,
    s3: Vec<Vec<u64>>,
    s4: Vec<Vec<u64>>,
    zero: QClass,
}

impl QuarticJacobian {
    /// Build from a smooth quartic with `X⁴` coefficient 1 and at least four
    /// rational points, three of them not collinear.
    pub fn new(p: u64, f: [u64; 15]) -> Result<QuarticJacobian, String> {
        if !(QUARTIC_MIN_P..QUARTIC_MAX_P).contains(&p) || !is_prime_u64(p) {
            return Err(format!(
                "plane quartics take a prime {QUARTIC_MIN_P} ≤ p < {QUARTIC_MAX_P}, not {p}"
            ));
        }
        Self::with_points(p, f, rational_points(&f, p))
    }

    /// Build from a smooth quartic and a list of its rational points (at
    /// least four, three of them not collinear), for a field too large to
    /// enumerate.  Every point is checked to lie on the curve.
    pub fn with_points(
        p: u64,
        f: [u64; 15],
        points: Vec<[u64; 3]>,
    ) -> Result<QuarticJacobian, String> {
        if !(QUARTIC_MIN_P..QUARTIC_LAW_MAX_P).contains(&p) || !is_prime_u64(p) {
            return Err(format!(
                "the plane-quartic group law takes a prime {QUARTIC_MIN_P} ≤ p < 2^31, not {p}"
            ));
        }
        if f[0] != 1 || f.iter().any(|&c| c >= p) {
            return Err("the quartic must have X⁴ coefficient 1 and reduced coefficients".into());
        }
        if !is_smooth(&f, p) {
            return Err("the quartic is singular, or the smoothness test cannot certify it".into());
        }
        if points
            .iter()
            .any(|&q| eval_form(&f, 4, q, p) != 0 || q == [0, 0, 0])
        {
            return Err("a supplied point is not on the curve".into());
        }
        let collinear = |a: [u64; 3], b: [u64; 3], c: [u64; 3]| {
            det(vec![a.to_vec(), b.to_vec(), c.to_vec()], p) == 0
        };
        let mut d_inf = None;
        'outer: for i in 0..points.len() {
            for j in i + 1..points.len() {
                for k in j + 1..points.len() {
                    if !collinear(points[i], points[j], points[k]) {
                        d_inf = Some([points[i], points[j], points[k]]);
                        break 'outer;
                    }
                }
            }
        }
        let d_inf = d_inf.ok_or("fewer than three non-collinear rational points")?;
        let p1 = *points
            .iter()
            .find(|q| !d_inf.contains(q))
            .ok_or("fewer than four rational points")?;
        let forms = Forms::new(p, &f);
        let r3 = forms.vanishing(&d_inf, 3);
        let r4 = forms.vanishing(&d_inf, 4);
        let mut d1 = d_inf.to_vec();
        d1.push(p1);
        let r3_p1 = forms.vanishing(&d1, 3);
        // H⁰(6K − 2D∞) = H⁰(3K − D∞)², and H⁰(3K − 2D∞) its colon by S_3.
        let r6x2 = forms.product(&r3, 3, &r3, 3);
        let ann6 = forms.ann(&r6x2, 6);
        let s3 = forms.full(3);
        let t2 = forms.colon(&ann6, &s3, 3, 3);
        let c = r6x2[0].clone();
        let s4 = forms.full(4);
        let zero_ann = rref(d_inf.iter().map(|&q| forms.eval_row(q, 3)).collect(), 10, p);
        let mut jac = QuarticJacobian {
            p,
            f,
            forms,
            d_inf,
            p1,
            points,
            r3,
            r4,
            r3_p1,
            t2,
            c,
            s3,
            s4,
            zero: QClass([0; 30]),
        };
        jac.zero = jac.pack(&zero_ann)?;
        jac.check_dims()?;
        Ok(jac)
    }

    fn check_dims(&self) -> Result<(), String> {
        let want = [
            (self.r3.len(), 7),
            (self.r4.len(), 11),
            (self.r3_p1.len(), 6),
            (self.t2.len(), 4),
        ];
        if want.iter().any(|(a, b)| a != b) {
            return Err(format!(
                "Riemann–Roch dimensions {want:?} are wrong: the curve is not a smooth quartic"
            ));
        }
        Ok(())
    }

    fn pack(&self, ann: &[Vec<u64>]) -> Result<QClass, String> {
        if ann.len() != 3 {
            return Err(format!("a class needs codimension 3, got {}", ann.len()));
        }
        let mut out = [0u32; 30];
        for (i, row) in ann.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                out[10 * i + j] = v as u32;
            }
        }
        Ok(QClass(out))
    }

    fn unpack(&self, x: &QClass) -> Vec<Vec<u64>> {
        (0..3)
            .map(|i| (0..10).map(|j| u64::from(x.0[10 * i + j])).collect())
            .collect()
    }

    /// `H⁰(3K − E)` for the class's divisor `E`.
    fn w3(&self, x: &QClass) -> Vec<Vec<u64>> {
        nullspace(self.unpack(x), 10, self.p)
    }

    pub fn zero(&self) -> QClass {
        self.zero
    }

    /// The evaluation functional of `S_3` at a point (for transfers that
    /// build a divisor's conditions themselves).
    pub fn eval_row3(&self, pt: [u64; 3]) -> Vec<u64> {
        self.forms.eval_row(pt, 3)
    }

    /// The class of `E − D∞` for an effective degree-3 divisor `E` given by
    /// three linear conditions on `S_3` cutting out `H⁰(3K − E)` (for
    /// example evaluations at its points, or the two `F_p`-conditions of a
    /// conjugate pair).  Refused unless the conditions have rank 3.
    pub fn class_from_conditions(&self, rows: Vec<Vec<u64>>) -> Result<QClass, String> {
        let ann = rref(rows, 10, self.p);
        let raw = self.pack(&ann)?;
        Ok(self.add_raw(&raw, &self.zero))
    }

    /// The class of `P + Q + R − D∞` for three distinct rational points.
    pub fn from_points(&self, pts: [[u64; 3]; 3]) -> Result<QClass, String> {
        if pts[0] == pts[1] || pts[0] == pts[2] || pts[1] == pts[2] {
            return Err("three distinct points".into());
        }
        let ann = rref(
            pts.iter().map(|&q| self.forms.eval_row(q, 3)).collect(),
            10,
            self.p,
        );
        let raw = self.pack(&ann)?;
        // Canonicalise: the sum with the identity is the canonical member.
        Ok(self.add_raw(&raw, &self.zero))
    }

    /// `x + y`; see the module documentation for the steps.
    pub fn add_raw(&self, x: &QClass, y: &QClass) -> QClass {
        let fm = &self.forms;
        let p = self.p;
        let wx = self.w3(x);
        let wy = self.w3(y);
        let p6 = fm.product(&wx, 3, &wy, 3);
        assert_eq!(p6.len(), 16, "H⁰(6K − E_x − E_y) has dimension 16");
        let f = p6[0].clone();
        let ann6 = fm.ann(&p6, 6);
        let t = fm.colon(&ann6, &self.s3, 3, 3);
        assert_eq!(t.len(), 4, "H⁰(3K − E_x − E_y) has dimension 4");
        let ann_fr3 = fm.ann(&fm.product(std::slice::from_ref(&f), 6, &self.r3, 3), 9);
        let mut q = fm.colon(&ann_fr3, &t, 6, 3);
        if q.len() >= 2 {
            let ann_fr3p1 = fm.ann(&fm.product(std::slice::from_ref(&f), 6, &self.r3_p1, 3), 9);
            q = fm.colon(&ann_fr3p1, &t, 6, 3);
        }
        assert_eq!(q.len(), 1, "the canonical member is unique");
        let h = q[0].clone();
        let ann_fr4 = fm.ann(&fm.product(&[f], 6, &self.r4, 4), 10);
        let u = fm.colon(&ann_fr4, &t, 7, 3);
        assert_eq!(u.len(), 5, "H⁰(7K − A − D∞) has dimension 5");
        let ann_hs4 = fm.ann(&fm.product(&[h], 6, &self.s4, 4), 10);
        let wg = fm.colon(&ann_hs4, &u, 3, 7);
        assert_eq!(wg.len(), 7, "H⁰(3K − G) has dimension 7");
        let ann = rref(nullspace(wg, 10, p), 10, p);
        self.pack(&ann).expect("codimension 3")
    }

    /// `−x`: one flip against the fixed divisor `B` (`div c = 2D∞ + B`).
    pub fn neg_raw(&self, x: &QClass) -> QClass {
        let fm = &self.forms;
        let p = self.p;
        let wx = self.w3(x);
        // H⁰(6K − E_x − B) = {f ∈ S_6 : f·H⁰(3K − 2D∞) ⊂ c·H⁰(3K − E_x)}.
        let ann_cw = fm.ann(&fm.product(std::slice::from_ref(&self.c), 6, &wx, 3), 9);
        let n = fm.colon(&ann_cw, &self.t2, 6, 3);
        assert!(!n.is_empty(), "H⁰(6K − E_x − B) is not zero");
        let f = n[0].clone();
        // H⁰(4K − E_x) = {t ∈ S_4 : t·S_3 ⊂ H⁰(3K − E_x)·S_4}.
        let w7 = fm.product(&wx, 3, &self.s4, 4);
        let w4 = fm.colon(&fm.ann(&w7, 7), &self.s3, 4, 3);
        // H⁰(7K − E_x − B) = {u ∈ S_7 : u·H⁰(3K − 2D∞) ⊂ c·H⁰(4K − E_x)}.
        let ann_cw4 = fm.ann(&fm.product(std::slice::from_ref(&self.c), 6, &w4, 4), 10);
        let u = fm.colon(&ann_cw4, &self.t2, 7, 3);
        // H⁰(3K − G) = {t ∈ S_3 : t·U ⊂ f·S_4}.
        let ann_fs4 = fm.ann(&fm.product(&[f], 6, &self.s4, 4), 10);
        let wg = fm.colon(&ann_fs4, &u, 3, 7);
        assert_eq!(wg.len(), 7, "H⁰(3K − G) has dimension 7");
        let ann = rref(nullspace(wg, 10, p), 10, p);
        let g = self.pack(&ann).expect("codimension 3");
        if n.len() >= 2 {
            // h⁰ = 2: G is a member of the pencil, not the canonical one.
            self.add_raw(&g, &self.zero)
        } else {
            g
        }
    }

    /// `[k]x` by double-and-add, uncounted.
    pub fn mul_raw(&self, x: &QClass, k: u64) -> QClass {
        let mut ops = GroupOps::default();
        CountedGroup::mul(&QuarticGroup(self), &mut ops, *x, k)
    }

    /// A random class: three distinct random rational points.
    pub fn random<R: Rng>(&self, rng: &mut R) -> QClass {
        let n = self.points.len();
        loop {
            let (i, j, k) = (
                rng.gen_range(0..n),
                rng.gen_range(0..n),
                rng.gen_range(0..n),
            );
            if i != j && i != k && j != k {
                return self
                    .from_points([self.points[i], self.points[j], self.points[k]])
                    .expect("distinct points");
            }
        }
    }

    /// The model's identity: `JCV1:pq:fp<p>:<15 coefficients>`.
    pub fn model(&self) -> String {
        let c: Vec<String> = self.f.iter().map(u64::to_string).collect();
        format!("JCV1:pq:fp{}:{}", self.p, c.join(","))
    }
}

/// The Jacobian as a counted group.  Addition and doubling are one group
/// operation each.  [`CountedGroup::neg`] is exact but costs about one
/// operation that this interface cannot charge; methods that rely on a
/// free negation are refused on this family (`methods::applicable`).
pub struct QuarticGroup<'a>(pub &'a QuarticJacobian);

impl CountedGroup for QuarticGroup<'_> {
    type Elt = QClass;
    fn identity(&self) -> QClass {
        self.0.zero
    }
    fn is_identity(&self, x: &QClass) -> bool {
        *x == self.0.zero
    }
    fn add(&self, ops: &mut GroupOps, x: QClass, y: QClass) -> QClass {
        ops.adds += 1;
        self.0.add_raw(&x, &y)
    }
    fn double(&self, ops: &mut GroupOps, x: QClass) -> QClass {
        ops.doubles += 1;
        self.0.add_raw(&x, &x)
    }
    fn neg(&self, x: QClass) -> QClass {
        if x == self.0.zero {
            x
        } else {
            self.0.neg_raw(&x)
        }
    }
    fn key(&self, x: &QClass) -> u64 {
        if *x == self.0.zero {
            return 0;
        }
        let mut h = 0x9e37_79b9_7f4a_7c15u64;
        for &v in &x.0 {
            h = splitmix64(h ^ u64::from(v));
        }
        h | 1
    }
}

/// Every rational point, `(x : y : 1)` then `(x : 1 : 0)`, in that order.
pub fn rational_points(f: &[u64; 15], p: u64) -> Vec<[u64; 3]> {
    let mut out = Vec::new();
    for x in 0..p {
        for y in 0..p {
            if eval_form(f, 4, [x, y, 1], p) == 0 {
                out.push([x, y, 1]);
            }
        }
    }
    for x in 0..p {
        if eval_form(f, 4, [x, 1, 0], p) == 0 {
            out.push([x, 1, 0]);
        }
    }
    out
}

/// `(a₁, a₂)` of the `L`-polynomial from `N₁ = #C(F_p)`, `N₂ = #C(F_{p²})`.
pub fn l_coefficients(p: u64, n1: u64, n2: u64) -> (i128, i128) {
    let s1 = p as i128 + 1 - n1 as i128;
    let s2 = (p as i128).pow(2) + 1 - n2 as i128;
    let a1 = -s1;
    let a2 = -(s2 + a1 * s1) / 2;
    (a1, a2)
}

/// `L(1)` from all three coefficients.
pub fn l_at_one(p: u64, a1: i128, a2: i128, a3: i128) -> i128 {
    let p = p as i128;
    1 + a1 + a2 + a3 + p * a2 + p * p * a1 + p * p * p
}

/// `a₃` from `N₃ = #C(F_{p³})` (tests, small `p`).
pub fn a3_from_counts(p: u64, n1: u64, n2: u64, n3: u64) -> i128 {
    let (a1, a2) = l_coefficients(p, n1, n2);
    let s1 = p as i128 + 1 - n1 as i128;
    let s2 = (p as i128).pow(2) + 1 - n2 as i128;
    let s3 = (p as i128).pow(3) + 1 - n3 as i128;
    -(s3 + a1 * s2 + a2 * s1) / 3
}

fn factor(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            out.push(d);
            while n.is_multiple_of(d) {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}

/// The order of `x`, given a multiple of it.
fn order_from_multiple(jac: &QuarticJacobian, x: &QClass, n: u64) -> u64 {
    let mut o = n;
    for q in factor(n) {
        while o.is_multiple_of(q) && jac.mul_raw(x, o / q) == jac.zero {
            o /= q;
        }
    }
    o
}

/// Some `n ∈ [lo, hi]` with `[n]x = 0`, by baby-step giant-step: baby
/// steps `[j]x`, `0 ≤ j < m`; giant steps `y_i = [lo + i·m]x`; a match
/// `y_i = [j]x` gives `n = lo + i·m − j`.
fn bsgs_in_window(jac: &QuarticJacobian, x: &QClass, lo: u64, hi: u64) -> Option<u64> {
    let g = QuarticGroup(jac);
    let mut ops = GroupOps::default();
    let width = hi - lo + 1;
    let m = (width as f64).sqrt().ceil() as u64 + 1;
    let mut baby: HashMap<QClass, u64> = HashMap::new();
    let mut cur = jac.zero;
    for j in 0..m {
        baby.entry(cur).or_insert(j);
        cur = g.add(&mut ops, cur, *x);
    }
    let mx = cur; // [m]x
    let mut y = jac.mul_raw(x, lo);
    let mut i = 0u64;
    while i * m <= width + m {
        if let Some(&j) = baby.get(&y) {
            if let Some(n) = (lo + i * m).checked_sub(j) {
                if (lo..=hi).contains(&n) && n > 0 {
                    return Some(n);
                }
            }
        }
        y = g.add(&mut ops, y, mx);
        i += 1;
    }
    None
}

/// `#J(F_p)` from `N₁`, `N₂` and element orders, or why not.
pub fn jacobian_order(
    jac: &QuarticJacobian,
    n1: u64,
    n2: u64,
    rng: &mut StdRng,
) -> Result<u64, String> {
    let p = jac.p;
    let (a1, a2) = l_coefficients(p, n1, n2);
    let bound = (20.0 * (p as f64).powf(1.5)).ceil() as i128;
    let base = l_at_one(p, a1, a2, 0);
    let lo = (base - bound).max(1) as u64;
    let hi = (base + bound) as u64;
    let mut lcm = 1u64;
    for _ in 0..24 {
        let x = jac.random(rng);
        if x == jac.zero {
            continue;
        }
        let n = bsgs_in_window(jac, &x, lo, hi)
            .ok_or("no multiple of an element order in the a₃ window")?;
        let o = order_from_multiple(jac, &x, n);
        lcm = lcm / gcd(lcm, o) * o;
        let first = lo.div_ceil(lcm) * lcm;
        if first <= hi && first + lcm > hi {
            return Ok(first);
        }
        if first > hi {
            return Err("element orders admit no group order in the window".into());
        }
    }
    Err("the Jacobian's exponent leaves several candidate orders".into())
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// A built plane-quartic workload: the Jacobian, `#J(F_p) = h·r` with `r`
/// prime, and a generator of the order-`r` subgroup.
pub struct QuarticInstance {
    pub jac: QuarticJacobian,
    pub group_order: u64,
    pub r: u64,
    pub cofactor: u64,
    pub generator: QClass,
}

impl QuarticInstance {
    /// Search: a prime of `p_bits` bits and a random smooth quartic, both
    /// from `seed`, until `#J(F_p) = h·r` with `h ≤ max_cofactor`.
    pub fn search(p_bits: u32, seed: u64, max_cofactor: u64) -> Result<QuarticInstance, String> {
        if !(5..=11).contains(&p_bits) {
            return Err(format!(
                "plane_quartic_search takes 5 ≤ p_bits ≤ 11, not {p_bits}"
            ));
        }
        let mut rng = StdRng::seed_from_u64(seed ^ (u64::from(p_bits) << 40) ^ 0x7151);
        for _ in 0..2000 {
            let p = loop {
                let c = (rng.gen::<u64>() & ((1 << p_bits) - 1)) | (1 << (p_bits - 1)) | 1;
                if is_prime_u64(c) && (QUARTIC_MIN_P..QUARTIC_MAX_P).contains(&c) {
                    break c;
                }
            };
            let mut f = [0u64; 15];
            f[0] = 1;
            for c in f.iter_mut().skip(1) {
                *c = rng.gen_range(0..p);
            }
            let Ok(jac) = QuarticJacobian::new(p, f) else {
                continue;
            };
            let n1 = jac.points.len() as u64;
            let n2 = count_points(&f, p, 2);
            let Ok(n) = jacobian_order(&jac, n1, n2, &mut rng) else {
                continue;
            };
            let Some((r, h)) = prime_part(n, max_cofactor) else {
                continue;
            };
            let mut g = None;
            for _ in 0..16 {
                let x = jac.mul_raw(&jac.random(&mut rng), h);
                if x != jac.zero {
                    g = Some(x);
                    break;
                }
            }
            let Some(g) = g else { continue };
            if jac.mul_raw(&g, r) != jac.zero {
                continue;
            }
            return Ok(QuarticInstance {
                jac,
                group_order: n,
                r,
                cofactor: h,
                generator: g,
            });
        }
        Err(format!(
            "no plane quartic with #J = h·r, h ≤ {max_cofactor}, for p_bits = {p_bits}, seed = {seed}"
        ))
    }

    /// The same Jacobian given outright; the order is trusted, `r` prime,
    /// `r | #J`, `G ≠ 0` and `[r]G = 0` are checked.
    pub fn explicit(
        p: u64,
        f: [u64; 15],
        group_order: u64,
        r: u64,
        g: &[String; 2],
    ) -> Result<QuarticInstance, String> {
        let jac = QuarticJacobian::new(p, f)?;
        if r < 5 || !is_prime_u64(r) || !group_order.is_multiple_of(r) {
            return Err(format!(
                "r = {r} is not a prime divisor of #J = {group_order}"
            ));
        }
        let generator = decode(&jac, g)?;
        if generator == jac.zero || jac.mul_raw(&generator, r) != jac.zero {
            return Err("the generator does not have order r".into());
        }
        Ok(QuarticInstance {
            jac,
            group_order,
            r,
            cofactor: group_order / r,
            generator,
        })
    }

    pub fn group(&self) -> QuarticGroup<'_> {
        QuarticGroup(&self.jac)
    }

    pub fn names(&self) -> (String, String) {
        let id = format!("{}:{}", self.jac.model(), self.group_order);
        let h = &sha256_hex(id.as_bytes())[..8];
        let bits = 64 - self.jac.p.leading_zeros();
        (id.clone(), format!("jcv1-pq-fp{bits}-{h}"))
    }

    pub fn encode(&self, x: &QClass) -> [String; 2] {
        encode(x)
    }

    pub fn mul_generator_hex(&self, k: u64) -> Option<[String; 2]> {
        let x = self.jac.mul_raw(&self.generator, k);
        (x != self.jac.zero).then(|| encode(&x))
    }

    /// A public target: three rational points chosen by the draw `h`
    /// (indices `h`, `h ≫ 21`, `h ≫ 42` modulo `#C(F_p)`), times the
    /// cofactor; `None` when they are not distinct or the class clears.
    pub fn hash_to_subgroup_hex(&self, h: u64) -> Option<[String; 2]> {
        let n = self.jac.points.len() as u64;
        let idx = [h % n, (h >> 21) % n, (h >> 42) % n];
        if idx[0] == idx[1] || idx[0] == idx[2] || idx[1] == idx[2] {
            return None;
        }
        let pts = idx.map(|i| self.jac.points[i as usize]);
        let x = self.jac.from_points(pts).ok()?;
        let q = self.jac.mul_raw(&x, self.cofactor);
        (q != self.jac.zero).then(|| encode(&q))
    }
}

/// Two hex strings of 15 four-digit entries each.
pub fn encode(x: &QClass) -> [String; 2] {
    let half = |s: &[u32]| -> String {
        let mut out = String::from("0x");
        for v in s {
            out.push_str(&format!("{v:08x}"));
        }
        out
    };
    [half(&x.0[..15]), half(&x.0[15..])]
}

pub fn decode(jac: &QuarticJacobian, t: &[String; 2]) -> Result<QClass, String> {
    let mut out = [0u32; 30];
    for (h, s) in t.iter().enumerate() {
        let digits = s.trim_start_matches("0x");
        if digits.len() != 120 {
            return Err(format!("`{s}` is not a plane-quartic class encoding"));
        }
        for i in 0..15 {
            out[15 * h + i] = u32::from_str_radix(&digits[8 * i..8 * i + 8], 16)
                .map_err(|_| format!("bad hex `{s}`"))?;
        }
    }
    let x = QClass(out);
    // A valid encoding is a reduced row-echelon 3×10 matrix over F_p that
    // the group law reproduces: canonicalise and compare.
    if out.iter().any(|&v| u64::from(v) >= jac.p) {
        return Err("an entry is not reduced modulo p".into());
    }
    let rows = jac.unpack(&x);
    if rref(rows.clone(), 10, jac.p) != rows {
        return Err("not a reduced row-echelon annihilator".into());
    }
    let canon = jac.add_raw(&x, &jac.zero);
    if canon != x {
        return Err("not the canonical representative of its class".into());
    }
    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_curve(p: u64, seed: u64) -> QuarticJacobian {
        let mut rng = StdRng::seed_from_u64(seed);
        loop {
            let mut f = [0u64; 15];
            f[0] = 1;
            for c in f.iter_mut().skip(1) {
                *c = rng.gen_range(0..p);
            }
            if let Ok(j) = QuarticJacobian::new(p, f) {
                return j;
            }
        }
    }

    #[test]
    fn the_section_spaces_have_their_riemann_roch_dimensions() {
        for n in 2..=MAX_DEG {
            assert_eq!(dim(n), 4 * n - 2, "S_{n}");
        }
        assert_eq!(dim(1), 3);
    }

    #[test]
    fn the_group_law_is_a_commutative_group_law() {
        let jac = small_curve(31, 5);
        let g = QuarticGroup(&jac);
        let mut ops = GroupOps::default();
        let mut rng = StdRng::seed_from_u64(9);
        for _ in 0..6 {
            let (x, y, z) = (
                jac.random(&mut rng),
                jac.random(&mut rng),
                jac.random(&mut rng),
            );
            assert_eq!(g.add(&mut ops, x, jac.zero), x, "identity");
            assert_eq!(g.add(&mut ops, x, y), g.add(&mut ops, y, x), "commutative");
            let xy = g.add(&mut ops, x, y);
            let l = g.add(&mut ops, xy, z);
            let yz = g.add(&mut ops, y, z);
            let r = g.add(&mut ops, x, yz);
            assert_eq!(l, r, "associative");
            assert_eq!(g.add(&mut ops, x, g.neg(x)), jac.zero, "inverse");
            assert_eq!(g.double(&mut ops, x), g.add(&mut ops, x, x), "doubling");
        }
    }

    /// The order the group law implies equals `L(1)` from three independent
    /// point counts, and annihilates random classes.
    #[test]
    fn the_jacobian_order_is_l_of_one() {
        let mut certified = 0;
        for (p, seed) in [(29u64, 1u64), (31, 2), (37, 3)] {
            let jac = small_curve(p, seed);
            let n1 = count_points(&jac.f, p, 1);
            assert_eq!(n1, jac.points.len() as u64);
            let n2 = count_points(&jac.f, p, 2);
            let n3 = count_points(&jac.f, p, 3);
            let (a1, a2) = l_coefficients(p, n1, n2);
            let a3 = a3_from_counts(p, n1, n2, n3);
            let lj = l_at_one(p, a1, a2, a3) as u64;
            let mut rng = StdRng::seed_from_u64(seed);
            for _ in 0..4 {
                let x = jac.random(&mut rng);
                assert_eq!(jac.mul_raw(&x, lj), jac.zero, "p = {p}: [L(1)]x = 0");
            }
            match jacobian_order(&jac, n1, n2, &mut rng) {
                Ok(n) => {
                    assert_eq!(n, lj, "p = {p}");
                    certified += 1;
                }
                Err(e) => assert!(e.contains("several"), "p = {p}: {e}"),
            }
        }
        assert!(
            certified >= 2,
            "the order search certified {certified} of 3"
        );
    }

    #[test]
    fn a_searched_instance_round_trips_through_its_explicit_form() {
        let i = QuarticInstance::search(6, 1, 64).unwrap();
        assert_eq!(i.jac.mul_raw(&i.generator, i.r), i.jac.zero);
        let g = encode(&i.generator);
        let again = QuarticInstance::explicit(i.jac.p, i.jac.f, i.group_order, i.r, &g).unwrap();
        assert_eq!(again.generator, i.generator);
        let mut bad = g.clone();
        bad[0] = format!("0x{}", "0".repeat(120));
        assert!(decode(&i.jac, &bad).is_err());
    }

    /// Wall time per group operation at the top of the family's range
    /// (diagnostic, printed; run with --nocapture).
    #[test]
    fn group_operation_cost() {
        let jac = small_curve(1021, 4);
        let g = QuarticGroup(&jac);
        let mut ops = GroupOps::default();
        let mut rng = StdRng::seed_from_u64(1);
        let mut x = jac.random(&mut rng);
        let y = jac.random(&mut rng);
        let t = std::time::Instant::now();
        for _ in 0..200 {
            x = g.add(&mut ops, x, y);
        }
        eprintln!("p = 1021: {:?} per addition", t.elapsed() / 200);
    }

    #[test]
    fn a_singular_quartic_is_refused() {
        // (X² − Y Z)² is a double conic: singular everywhere on it.
        let p = 31;
        let m4 = monomials(4);
        let mut f = [0u64; 15];
        for (k, &m) in m4.iter().enumerate() {
            f[k] = match m {
                (4, 0, 0) => 1,
                (2, 1, 1) => p - 2,
                (0, 2, 2) => 1,
                _ => 0,
            };
        }
        assert!(!is_smooth(&f, p));
        assert!(QuarticJacobian::new(p, f).is_err());
    }
}
