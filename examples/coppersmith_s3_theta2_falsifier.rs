//! Toy falsifier for the `θ₂` ceiling of
//! `research/notes/ecdlp-general/RESEARCH_LATTICE_ECDLP_ROUTES.md` §6–§7.
//!
//! **Question.** Can a Coppersmith-type lattice on Semaev's `S₃` recover a
//! planted two-summand decomposition `R = P₁ + P₂` with `x(P_i) ≤ B = p^β`
//! beyond `β = 1/4` (root region `∏X_i = p^{2β} > p^{1/2}`)? The note derives
//! the Jochemsz–May basic-strategy ceiling `θ₂ = 1/3` (`β = 1/6` per
//! variable) in the limit, and at multiplicity `ℓ` the determinant condition
//! gives `β_det(ℓ) = 1/18, 1/10, 5/42` for `ℓ = 1, 2, 3`.
//!
//! **Pipeline (the whole thing, not a proxy).** For each trial: a random
//! Weierstrass curve over `F_p`, two random points with `x ≤ B`, the target
//! `z = x(P₁ + P₂)`; `f(x, y) = S₃(x, y, z)`, monic in `x²y²`; the JM basic
//! lattice of the shifts `x^{i−2k} y^{j−2k} f^k p^{ℓ−k}` over the box
//! `[0, 2ℓ]²`, scaled by `B^{i+j}`; high-precision LLL (`δ = 0.99`); the first
//! `ROWS` reduced rows read back as integer polynomials `h_r(x, y)`; for each
//! pair, the resultant `Res_y(h_r, h_s)(x)` computed modulo three 61-bit
//! primes by evaluation and interpolation, its roots in `[0, B]` sieved modulo
//! those primes, each surviving `x₀` lifted to a `y₀ ∈ [0, B]` that is an exact
//! integer root of both polynomials. A trial **recovers** when the planted
//! `{x(P₁), x(P₂)}` is among the pairs so found. The root is never used to
//! choose rows; it is used only for the two diagnostics also recorded
//! (Howgrave-Graham on the first two rows; how many rows vanish at it over Z).
//!
//! **Controls.** Positive: the smallest `β` on the grid with `B ≥ 4` at each
//! size and `ℓ` must recover. Negative: the same pipeline on `z = x(R)` for a
//! uniformly random `R` (no planted small decomposition) at `β = 0.30` must
//! report nothing, unless what it reports is a genuine decomposition, which
//! is checked and counted separately.
//!
//! Everything here is native Rust; Python only plots the recorded JSON.
//!
//! ```bash
//! cargo run --release --example coppersmith_s3_theta2_falsifier -- \
//!     --bits 24,32,40,56 --ell 1,2,3 --beta 0.04:0.34:0.02 --trials 10 \
//!     --neg-trials 10 --neg-beta 0.30 --seed 20261023 --out <dir>
//! ```

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use crypto_lib::cryptanalysis::lattice::lll_reduce_hp;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

// ───────────────────────────── F_p (p < 2^62) ─────────────────────────────

#[derive(Clone, Copy, Debug)]
struct Fp {
    p: u64,
}

impl Fp {
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64 % self.p;
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
        assert!(a != 0, "inverse of zero");
        self.pow(a, self.p - 2)
    }
    fn c(&self, v: i64) -> u64 {
        let p = self.p as i128;
        (((v as i128 % p) + p) % p) as u64
    }
    fn big(&self, c: &BigInt) -> u64 {
        c.mod_floor(&BigInt::from(self.p)).to_u64().unwrap()
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n.is_multiple_of(q) {
            return n == q;
        }
    }
    let f = Fp { p: n };
    let mut d = n - 1;
    let mut s = 0;
    while d.is_multiple_of(2) {
        d /= 2;
        s += 1;
    }
    'outer: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = f.pow(a, d);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = f.mul(x, x);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn largest_prime_below(n: u64) -> u64 {
    let mut p = n - 1;
    loop {
        if p % 2 == 1 && is_prime(p) {
            return p;
        }
        p -= 1;
    }
}

/// Square root mod p (Tonelli–Shanks), `None` for non-residues.
fn sqrt_mod(f: Fp, a: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }
    let p = f.p;
    if f.pow(a, (p - 1) / 2) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(f.pow(a, (p + 1) / 4));
    }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2u64;
    while f.pow(z, (p - 1) / 2) != p - 1 {
        z += 1;
    }
    let (mut m, mut c, mut t, mut r) = (s, f.pow(z, q), f.pow(a, q), f.pow(a, q.div_ceil(2)));
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = f.mul(tt, tt);
            i += 1;
        }
        let b = f.pow(c, 1u64 << (m - i - 1));
        m = i;
        c = f.mul(b, b);
        t = f.mul(t, c);
        r = f.mul(r, b);
    }
    Some(r)
}

// ───────────────────────────── the curve ──────────────────────────────────

#[derive(Clone, Copy, Debug)]
struct Weier {
    f: Fp,
    a: u64,
    b: u64,
}

impl Weier {
    /// `S₃(x₁, x₂, x₃)` evaluated (Semaev's closed form).
    fn s3(&self, x1: u64, x2: u64, x3: u64) -> u64 {
        let f = self.f;
        let (a, b) = (self.a, self.b);
        let uv = f.mul(x1, x2);
        let upv = f.add(x1, x2);
        let d = f.sub(x1, x2);
        let c2 = f.mul(d, d);
        let inner = f.add(f.mul(upv, f.add(uv, a)), f.mul(f.c(2), b));
        let c1 = f.neg(f.mul(f.c(2), inner));
        let t = f.sub(uv, a);
        let c0 = f.sub(f.mul(t, t), f.mul(f.mul(f.c(4), b), upv));
        f.add(f.add(f.mul(c2, f.mul(x3, x3)), f.mul(c1, x3)), c0)
    }
    fn on_curve(&self, x: u64, y: u64) -> bool {
        let f = self.f;
        let rhs = f.add(f.add(f.mul(f.mul(x, x), x), f.mul(self.a, x)), self.b);
        f.mul(y, y) == rhs
    }
}

fn pt_add(w: &Weier, p1: Option<(u64, u64)>, p2: Option<(u64, u64)>) -> Option<(u64, u64)> {
    let f = w.f;
    match (p1, p2) {
        (None, q) | (q, None) => q,
        (Some((x1, y1)), Some((x2, y2))) => {
            let lam = if x1 == x2 {
                if f.add(y1, y2) == 0 {
                    return None;
                }
                let num = f.add(f.mul(f.c(3), f.mul(x1, x1)), w.a);
                f.mul(num, f.inv(f.add(y1, y1)))
            } else {
                f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1)))
            };
            let x3 = f.sub(f.sub(f.mul(lam, lam), x1), x2);
            let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
            Some((x3, y3))
        }
    }
}

/// A random affine point with `1 ≤ x ≤ bound`.
fn random_point(w: &Weier, rng: &mut StdRng, bound: u64) -> (u64, u64) {
    loop {
        let x = rng.gen_range(1..=bound.min(w.f.p - 1));
        let rhs =
            w.f.add(w.f.add(w.f.mul(w.f.mul(x, x), x), w.f.mul(w.a, x)), w.b);
        if let Some(y) = sqrt_mod(w.f, rhs) {
            let y = if rng.gen_bool(0.5) { y } else { w.f.neg(y) };
            return (x, y);
        }
    }
}

fn random_curve(p: u64, rng: &mut StdRng) -> Weier {
    let f = Fp { p };
    loop {
        let a = rng.gen_range(1..p);
        let b = rng.gen_range(1..p);
        // 4a³ + 27b² ≠ 0
        let disc = f.add(
            f.mul(f.c(4), f.mul(f.mul(a, a), a)),
            f.mul(f.c(27), f.mul(b, b)),
        );
        if disc != 0 {
            return Weier { f, a, b };
        }
    }
}

// ───────────────── bivariate integer polynomials, dense ───────────────────

/// Dense `Z[x, y]` polynomial on the box `[0, side)²`: `c[i * side + j]` is the
/// coefficient of `x^i y^j`.
#[derive(Clone, Debug)]
struct Poly {
    side: usize,
    c: Vec<BigInt>,
}

impl Poly {
    fn zero(side: usize) -> Self {
        Poly {
            side,
            c: vec![BigInt::zero(); side * side],
        }
    }
    fn get(&self, i: usize, j: usize) -> &BigInt {
        &self.c[i * self.side + j]
    }
    fn set(&mut self, i: usize, j: usize, v: BigInt) {
        self.c[i * self.side + j] = v;
    }
    fn is_zero(&self) -> bool {
        self.c.iter().all(|v| v.is_zero())
    }
    fn deg_x(&self) -> usize {
        (0..self.side)
            .rev()
            .find(|&i| (0..self.side).any(|j| !self.get(i, j).is_zero()))
            .unwrap_or(0)
    }
    fn deg_y(&self) -> usize {
        (0..self.side)
            .rev()
            .find(|&j| (0..self.side).any(|i| !self.get(i, j).is_zero()))
            .unwrap_or(0)
    }
    fn mul(&self, o: &Poly, side: usize) -> Poly {
        let mut r = Poly::zero(side);
        for i in 0..self.side {
            for j in 0..self.side {
                let a = self.get(i, j);
                if a.is_zero() {
                    continue;
                }
                for k in 0..o.side {
                    for l in 0..o.side {
                        let b = o.get(k, l);
                        if b.is_zero() {
                            continue;
                        }
                        let (ii, jj) = (i + k, j + l);
                        assert!(ii < side && jj < side, "product leaves the box");
                        let cur = r.get(ii, jj).clone();
                        r.set(ii, jj, cur + a * b);
                    }
                }
            }
        }
        r
    }
    fn eval(&self, x: &BigInt, y: &BigInt) -> BigInt {
        // Horner in x of Horner in y.
        let mut acc = BigInt::zero();
        for i in (0..self.side).rev() {
            let mut row = BigInt::zero();
            for j in (0..self.side).rev() {
                row = row * y + self.get(i, j);
            }
            acc = acc * x + row;
        }
        acc
    }
    /// `h(x₀, y)` as a univariate integer polynomial in `y` (low → high).
    fn at_x(&self, x0: &BigInt) -> Vec<BigInt> {
        let mut out = vec![BigInt::zero(); self.side];
        for j in 0..self.side {
            let mut acc = BigInt::zero();
            for i in (0..self.side).rev() {
                acc = acc * x0 + self.get(i, j);
            }
            out[j] = acc;
        }
        out
    }
}

/// `f(x, y) = S₃(x, y, z)` over `Z`, coefficients in `[0, p)`, monic in `x²y²`.
fn s3_poly(w: &Weier, z: u64) -> Poly {
    let f = w.f;
    let (a, b) = (w.a, w.b);
    let z2 = f.mul(z, z);
    let two = f.c(2);
    let four = f.c(4);
    let mut pz = Poly::zero(3);
    let big = |v: u64| BigInt::from(v);
    pz.set(2, 2, big(1));
    pz.set(2, 1, big(f.neg(f.mul(two, z))));
    pz.set(1, 2, big(f.neg(f.mul(two, z))));
    pz.set(2, 0, big(z2));
    pz.set(0, 2, big(z2));
    pz.set(1, 1, big(f.neg(f.add(f.mul(two, z2), f.mul(two, a)))));
    let lin = f.neg(f.add(f.mul(two, f.mul(z, a)), f.mul(four, b)));
    pz.set(1, 0, big(lin));
    pz.set(0, 1, big(lin));
    pz.set(0, 0, big(f.sub(f.mul(a, a), f.mul(four, f.mul(b, z)))));
    pz
}

// ───────────────── univariate polynomials over F_q (q < 2^62) ──────────────

/// Sylvester resultant of `a` (formal degree `a.len() − 1`) and `b` over `F_q`.
fn sylvester_det(f: Fp, a: &[u64], b: &[u64]) -> u64 {
    let (da, db) = (a.len() - 1, b.len() - 1);
    let n = da + db;
    if n == 0 {
        return 1;
    }
    let mut m = vec![vec![0u64; n]; n];
    for r in 0..db {
        for (k, &c) in a.iter().enumerate() {
            m[r][r + da - k] = c; // high → low along the row
        }
    }
    for r in 0..da {
        for (k, &c) in b.iter().enumerate() {
            m[db + r][r + db - k] = c;
        }
    }
    // Gaussian elimination mod q.
    let mut det = 1u64;
    for col in 0..n {
        let piv = (col..n).find(|&r| m[r][col] != 0);
        let Some(piv) = piv else {
            return 0;
        };
        if piv != col {
            m.swap(piv, col);
            det = f.neg(det);
        }
        let inv = f.inv(m[col][col]);
        det = f.mul(det, m[col][col]);
        for r in col + 1..n {
            if m[r][col] == 0 {
                continue;
            }
            let factor = f.mul(m[r][col], inv);
            for k in col..n {
                let t = f.mul(factor, m[col][k]);
                m[r][k] = f.sub(m[r][k], t);
            }
        }
    }
    det
}

/// Lagrange interpolation over `F_q` through `(i, ys[i])`, `i = 0, 1, …`.
/// Returns coefficients low → high.
fn interpolate(f: Fp, ys: &[u64]) -> Vec<u64> {
    let n = ys.len();
    let mut coeffs = vec![0u64; n];
    for i in 0..n {
        // basis polynomial ∏_{j≠i} (x − j) / (i − j), built incrementally
        let mut num = vec![0u64; n];
        num[0] = 1;
        let mut deg = 0usize;
        let mut denom = 1u64;
        for j in 0..n {
            if j == i {
                continue;
            }
            let mj = f.neg(f.c(j as i64));
            // new[k] = old[k−1] + (−j)·old[k]
            let mut new = vec![0u64; n];
            for k in 0..=deg + 1 {
                let from_shift = if k >= 1 { num[k - 1] } else { 0 };
                let from_scale = if k <= deg { f.mul(mj, num[k]) } else { 0 };
                new[k] = f.add(from_shift, from_scale);
            }
            num = new;
            deg += 1;
            denom = f.mul(denom, f.c(i as i64 - j as i64));
        }
        let scale = f.mul(ys[i], f.inv(denom));
        for k in 0..n {
            coeffs[k] = f.add(coeffs[k], f.mul(scale, num[k]));
        }
    }
    coeffs
}

fn horner(f: Fp, c: &[u64], x: u64) -> u64 {
    let mut acc = 0u64;
    for &ci in c.iter().rev() {
        acc = f.add(f.mul(acc, x), ci);
    }
    acc
}

fn reduce_poly_mod(h: &Poly, f: Fp) -> Vec<Vec<u64>> {
    (0..h.side)
        .map(|i| (0..h.side).map(|j| f.big(h.get(i, j))).collect())
        .collect()
}

/// `Res_y(h₁, h₂)` as a polynomial in `x` over `F_q`, with the formal
/// `y`-degrees of `h₁`, `h₂`. `None` when the resultant is identically zero.
fn resultant_mod(h1: &Poly, h2: &Poly, f: Fp) -> Option<Vec<u64>> {
    let (dx1, dy1, dx2, dy2) = (h1.deg_x(), h1.deg_y(), h2.deg_x(), h2.deg_y());
    let dbound = dx1 * dy2 + dx2 * dy1;
    let r1 = reduce_poly_mod(h1, f);
    let r2 = reduce_poly_mod(h2, f);
    let at = |r: &Vec<Vec<u64>>, dy: usize, x0: u64| -> Vec<u64> {
        (0..=dy)
            .map(|j| {
                let mut acc = 0u64;
                for i in (0..r.len()).rev() {
                    acc = f.add(f.mul(acc, x0), r[i][j]);
                }
                acc
            })
            .collect()
    };
    let ys: Vec<u64> = (0..=dbound as u64)
        .map(|x0| {
            let a = at(&r1, dy1, x0);
            let b = at(&r2, dy2, x0);
            if dy1 == 0 {
                f.pow(a[0], dy2 as u64)
            } else if dy2 == 0 {
                f.pow(b[0], dy1 as u64)
            } else {
                sylvester_det(f, &a, &b)
            }
        })
        .collect();
    let coeffs = interpolate(f, &ys);
    if coeffs.iter().all(|&c| c == 0) {
        None
    } else {
        Some(coeffs)
    }
}

/// Integer roots of the univariate `g` (low → high, over Z) in `[0, bound]`,
/// sieved modulo the primes in `qs` then checked exactly.
fn int_roots(g: &[BigInt], bound: u64, qs: &[Fp]) -> Vec<u64> {
    let reduced: Vec<Vec<u64>> = qs
        .iter()
        .map(|q| g.iter().map(|c| q.big(c)).collect())
        .collect();
    let mut out = Vec::new();
    'cand: for y0 in 0..=bound {
        for (q, r) in qs.iter().zip(&reduced) {
            if horner(*q, r, y0) != 0 {
                continue 'cand;
            }
        }
        // exact
        let y = BigInt::from(y0);
        let mut acc = BigInt::zero();
        for c in g.iter().rev() {
            acc = acc * &y + c;
        }
        if acc.is_zero() {
            out.push(y0);
        }
    }
    out
}

// ───────────────────────────── one trial ──────────────────────────────────

const ROWS: usize = 6;

#[derive(Debug)]
struct TrialOut {
    dim: usize,
    lll_ms: f64,
    extract_ms: f64,
    hg_first_two: bool,
    rows_vanish: usize,
    pairs_tried: usize,
    pairs_dependent: usize,
    candidates: usize,
    found: Vec<(u64, u64)>,
    found_genuine: usize,
    recovered: bool,
    recovered_hg: bool,
}

#[allow(clippy::too_many_arguments)]
fn run_trial(
    w: &Weier,
    z: u64,
    planted: Option<(u64, u64)>,
    ell: usize,
    bound: u64,
    qs: &[Fp],
) -> TrialOut {
    let f = w.f;
    let pz = BigInt::from(f.p);
    let fpoly = s3_poly(w, z);
    // sanity: f agrees with S₃ at a few points
    for (u, v) in [(1u64, 2u64), (3, 7), (11, 5)] {
        let want = w.s3(u, v, z);
        let got = f.big(&fpoly.eval(&BigInt::from(u), &BigInt::from(v)));
        assert_eq!(want, got, "S₃ expansion mismatch");
    }
    let side = 2 * ell + 1;
    let mut fpow: Vec<Poly> = vec![Poly::zero(side)];
    fpow[0].set(0, 0, BigInt::one());
    for k in 1..=ell {
        let next = fpow[k - 1].mul(&fpoly, side);
        fpow.push(next);
    }
    let dim = side * side;
    let bb = BigInt::from(bound);
    let mut basis: Vec<Vec<BigInt>> = Vec::with_capacity(dim);
    for i in 0..side {
        for j in 0..side {
            let k = (i / 2).min(j / 2).min(ell);
            let (si, sj) = (i - 2 * k, j - 2 * k);
            let pk = pz.pow((ell - k) as u32);
            let mut row = vec![BigInt::zero(); dim];
            let g = &fpow[k];
            for a in 0..g.side {
                for b in 0..g.side {
                    let c = g.get(a, b);
                    if c.is_zero() {
                        continue;
                    }
                    let (ii, jj) = (a + si, b + sj);
                    row[ii * side + jj] = c * &pk * bb.pow((ii + jj) as u32);
                }
            }
            basis.push(row);
        }
    }
    let t0 = Instant::now();
    lll_reduce_hp(&mut basis, 0.99).expect("LLL failed");
    let lll_ms = t0.elapsed().as_secs_f64() * 1e3;
    let t1 = Instant::now();
    // rows → integer polynomials
    let mut hs: Vec<Poly> = Vec::new();
    let mut hg_rows: Vec<bool> = Vec::new();
    let pm = pz.pow(ell as u32);
    let pm2 = &pm * &pm;
    let dim_big = BigInt::from(dim as u64);
    for row in basis.iter() {
        let mut h = Poly::zero(side);
        let mut n2 = BigInt::zero();
        for i in 0..side {
            for j in 0..side {
                let c = &row[i * side + j];
                n2 += c * c;
                if c.is_zero() {
                    continue;
                }
                let d = bb.pow((i + j) as u32);
                let (q, r) = c.div_rem(&d);
                assert!(r.is_zero(), "row entry not divisible by the scaling");
                h.set(i, j, q);
            }
        }
        // Howgrave-Graham: ‖v‖ < p^ℓ / √dim  ⇔  ‖v‖²·dim < p^{2ℓ}
        hg_rows.push(&n2 * &dim_big < pm2);
        hs.push(h);
    }
    let hg_first_two = hg_rows[0] && hg_rows[1];
    let rows_vanish = match planted {
        Some((x1, x2)) => hs
            .iter()
            .filter(|h| !h.is_zero() && h.eval(&BigInt::from(x1), &BigInt::from(x2)).is_zero())
            .count(),
        None => 0,
    };
    // root extraction from the first ROWS rows, pairwise
    let nrows = ROWS.min(hs.len());
    let mut found: Vec<(u64, u64)> = Vec::new();
    let mut found_hg: Vec<(u64, u64)> = Vec::new();
    let mut pairs_tried = 0usize;
    let mut pairs_dependent = 0usize;
    for r in 0..nrows {
        for s in r + 1..nrows {
            if hs[r].is_zero() || hs[s].is_zero() {
                continue;
            }
            pairs_tried += 1;
            // resultant mod each prime; dependent if zero mod all
            let mut res: Vec<Option<Vec<u64>>> = Vec::new();
            for q in qs {
                res.push(resultant_mod(&hs[r], &hs[s], *q));
            }
            if res.iter().all(|x| x.is_none()) {
                pairs_dependent += 1;
                continue;
            }
            let res: Vec<(Fp, Vec<u64>)> = qs
                .iter()
                .zip(res)
                .filter_map(|(q, r)| r.map(|r| (*q, r)))
                .collect();
            let mut xs: Vec<u64> = Vec::new();
            'cand: for x0 in 0..=bound {
                for (q, r) in &res {
                    if horner(*q, r, x0) != 0 {
                        continue 'cand;
                    }
                }
                xs.push(x0);
            }
            for x0 in xs {
                let xb = BigInt::from(x0);
                let g1 = hs[r].at_x(&xb);
                let g2 = hs[s].at_x(&xb);
                let (g, other) = if g1.iter().any(|c| !c.is_zero()) {
                    (g1, g2)
                } else if g2.iter().any(|c| !c.is_zero()) {
                    (g2, g1)
                } else {
                    continue;
                };
                for y0 in int_roots(&g, bound, qs) {
                    let yb = BigInt::from(y0);
                    let mut acc = BigInt::zero();
                    for c in other.iter().rev() {
                        acc = acc * &yb + c;
                    }
                    if acc.is_zero() {
                        let pair = if x0 <= y0 { (x0, y0) } else { (y0, x0) };
                        if !found.contains(&pair) {
                            found.push(pair);
                        }
                        if hg_rows[r] && hg_rows[s] && !found_hg.contains(&pair) {
                            found_hg.push(pair);
                        }
                    }
                }
            }
        }
    }
    let extract_ms = t1.elapsed().as_secs_f64() * 1e3;
    // A Coppersmith pipeline checks every candidate against the congruence
    // before reporting it: the short reduced polynomials at large bounds are
    // near-monomials whose common integer root is (0, 0), which is not a
    // root of f mod p. `candidates` counts the raw pairs, `found` the checked.
    let candidates = found.len();
    found.retain(|(x0, y0)| w.s3(*x0, *y0, z) == 0);
    found_hg.retain(|(x0, y0)| w.s3(*x0, *y0, z) == 0);
    let found_genuine = found.len();
    let (recovered, recovered_hg) = match planted {
        Some((x1, x2)) => {
            let key = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            (found.contains(&key), found_hg.contains(&key))
        }
        None => (false, false),
    };
    TrialOut {
        dim,
        lll_ms,
        extract_ms,
        hg_first_two,
        rows_vanish,
        pairs_tried,
        pairs_dependent,
        candidates,
        found,
        found_genuine,
        recovered,
        recovered_hg,
    }
}

// ───────────────────────────── driver ─────────────────────────────────────

fn parse_list_u64(s: &str) -> Vec<u64> {
    s.split(',')
        .map(|t| t.trim().parse().expect("integer list"))
        .collect()
}

fn parse_beta_grid(s: &str) -> Vec<f64> {
    let parts: Vec<f64> = s
        .split(':')
        .map(|t| t.trim().parse().expect("beta grid a:b:step"))
        .collect();
    assert_eq!(parts.len(), 3);
    let (a, b, st) = (parts[0], parts[1], parts[2]);
    let n = ((b - a) / st).round() as usize;
    (0..=n)
        .map(|i| ((a + st * i as f64) * 1e6).round() / 1e6)
        .collect()
}

fn beta_det(ell: usize) -> f64 {
    // per-variable determinant bound of the JM basic box lattice, side 2ℓ+1
    let side = 2 * ell + 1;
    let dim = (side * side) as f64;
    let s_x = dim * (side as f64 - 1.0);
    let mut s_p = 0.0;
    for k in 0..ell {
        let mk = ((2 * (ell - k) + 1) * (2 * (ell - k) + 1)) as f64;
        let mk1 = ((2 * (ell - k - 1) + 1) * (2 * (ell - k - 1) + 1)) as f64;
        s_p += (ell - k) as f64 * (mk - mk1);
    }
    (ell as f64 * dim - s_p) / s_x
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut bits_list = vec![24u64, 32, 40, 56];
    let mut ells = vec![1u64, 2, 3];
    let mut betas = parse_beta_grid("0.04:0.34:0.02");
    let mut trials = 10usize;
    let mut neg_trials = 10usize;
    let mut neg_beta = 0.30f64;
    let mut seed = 20261023u64;
    let mut out_dir = PathBuf::from("research/coppersmith_s3_theta2_20261023/scratch");
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--bits" => bits_list = parse_list_u64(&args[i + 1]),
            "--ell" => ells = parse_list_u64(&args[i + 1]),
            "--beta" => betas = parse_beta_grid(&args[i + 1]),
            "--trials" => trials = args[i + 1].parse().unwrap(),
            "--neg-trials" => neg_trials = args[i + 1].parse().unwrap(),
            "--neg-beta" => neg_beta = args[i + 1].parse().unwrap(),
            "--seed" => seed = args[i + 1].parse().unwrap(),
            "--out" => out_dir = PathBuf::from(&args[i + 1]),
            other => panic!("unknown argument {other}"),
        }
        i += 2;
    }
    fs::create_dir_all(&out_dir).unwrap();
    let mut rows_file = fs::File::create(out_dir.join("trials.jsonl")).unwrap();
    // three 61-bit primes for the resultant sieve
    let qs: Vec<Fp> = {
        let mut v = Vec::new();
        let mut n = (1u64 << 61) - 1; // 2^61 − 1 is prime
        while v.len() < 3 {
            if is_prime(n) {
                v.push(Fp { p: n });
            }
            n -= 2;
        }
        v
    };
    let started = Instant::now();
    let mut cells: Vec<serde_json::Value> = Vec::new();
    println!("| bits | p | ℓ | dim | β | B | recovered/N | recovered (HG rows)/N | HG first two/N | ≥2 rows vanish/N | mean LLL ms | mean extract ms |");
    println!("|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for &bits in &bits_list {
        let p = largest_prime_below(1u64 << bits);
        let mut crng = StdRng::seed_from_u64(seed ^ (bits << 8));
        let w = random_curve(p, &mut crng);
        for &ell in &ells {
            let ell = ell as usize;
            for (bi, &beta) in betas.iter().enumerate() {
                let bound = ((p as f64).powf(beta)).floor() as u64;
                let bound = bound.max(1).min(p - 1);
                let mut rec = 0usize;
                let mut rec_hg = 0usize;
                let mut hg2 = 0usize;
                let mut van2 = 0usize;
                let mut lll_sum = 0.0;
                let mut ext_sum = 0.0;
                let mut dim = 0;
                for t in 0..trials {
                    let s = seed.wrapping_mul(1_000_003).wrapping_add(
                        bits * 1_000_000 + (ell as u64) * 100_000 + (bi as u64) * 1000 + t as u64,
                    );
                    let mut rng = StdRng::seed_from_u64(s);
                    let (p1, p2, z) = loop {
                        let p1 = random_point(&w, &mut rng, bound);
                        let p2 = random_point(&w, &mut rng, bound);
                        if let Some((z, _)) = pt_add(&w, Some(p1), Some(p2)) {
                            break (p1, p2, z);
                        }
                    };
                    debug_assert!(w.on_curve(p1.0, p1.1) && w.on_curve(p2.0, p2.1));
                    assert_eq!(
                        w.s3(p1.0, p2.0, z),
                        0,
                        "planted decomposition is not a root of S₃"
                    );
                    let o = run_trial(&w, z, Some((p1.0, p2.0)), ell, bound, &qs);
                    dim = o.dim;
                    rec += o.recovered as usize;
                    rec_hg += o.recovered_hg as usize;
                    hg2 += o.hg_first_two as usize;
                    van2 += (o.rows_vanish >= 2) as usize;
                    lll_sum += o.lll_ms;
                    ext_sum += o.extract_ms;
                    let row = json!({
                        "arm": "planted", "bits": bits, "p": p, "a": w.a, "b": w.b, "ell": ell,
                        "beta": beta, "bound": bound, "trial": t, "seed": s,
                        "x1": p1.0, "x2": p2.0, "z": z, "dim": o.dim,
                        "recovered": o.recovered, "recovered_hg": o.recovered_hg,
                        "hg_first_two": o.hg_first_two, "rows_vanish": o.rows_vanish,
                        "pairs_tried": o.pairs_tried, "pairs_dependent": o.pairs_dependent,
                        "candidates": o.candidates, "found": o.found, "found_genuine": o.found_genuine,
                        "lll_ms": o.lll_ms, "extract_ms": o.extract_ms,
                    });
                    writeln!(rows_file, "{row}").unwrap();
                }
                let n = trials as f64;
                println!(
                    "| {bits} | {p} | {ell} | {dim} | {beta:.2} | {bound} | {rec}/{trials} | {rec_hg}/{trials} | {hg2}/{trials} | {van2}/{trials} | {:.1} | {:.1} |",
                    lll_sum / n, ext_sum / n
                );
                cells.push(json!({
                    "arm": "planted", "bits": bits, "p": p, "ell": ell, "dim": dim, "beta": beta, "bound": bound,
                    "trials": trials, "recovered": rec, "recovered_hg": rec_hg, "hg_first_two": hg2,
                    "vanish_ge2": van2, "mean_lll_ms": lll_sum / n, "mean_extract_ms": ext_sum / n,
                }));
            }
            // negative control: random target, no planted small decomposition
            let bound = ((p as f64).powf(neg_beta)).floor() as u64;
            let bound = bound.max(1).min(p - 1);
            let mut reported = 0usize;
            let mut genuine = 0usize;
            let mut lll_sum = 0.0;
            let mut ext_sum = 0.0;
            let mut dim = 0;
            for t in 0..neg_trials {
                let s = seed
                    .wrapping_mul(7_919)
                    .wrapping_add(bits * 1_000_000 + (ell as u64) * 100_000 + 99_000 + t as u64);
                let mut rng = StdRng::seed_from_u64(s);
                let r = random_point(&w, &mut rng, p - 1);
                let z = r.0;
                let o = run_trial(&w, z, None, ell, bound, &qs);
                dim = o.dim;
                reported += (o.candidates > 0) as usize;
                genuine += (o.found_genuine > 0) as usize;
                lll_sum += o.lll_ms;
                ext_sum += o.extract_ms;
                let row = json!({
                    "arm": "negative", "bits": bits, "p": p, "a": w.a, "b": w.b, "ell": ell,
                    "beta": neg_beta, "bound": bound, "trial": t, "seed": s, "z": z, "dim": o.dim,
                    "candidates": o.candidates, "found": o.found, "found_genuine": o.found_genuine,
                    "hg_first_two": o.hg_first_two, "pairs_tried": o.pairs_tried,
                    "pairs_dependent": o.pairs_dependent, "lll_ms": o.lll_ms, "extract_ms": o.extract_ms,
                });
                writeln!(rows_file, "{row}").unwrap();
            }
            let n = neg_trials.max(1) as f64;
            println!(
                "| {bits} | {p} | {ell} | {dim} | {neg_beta:.2} (negative control) | {bound} | genuine {genuine}/{neg_trials} (raw candidates in {reported}) | – | – | – | {:.1} | {:.1} |",
                lll_sum / n, ext_sum / n
            );
            cells.push(json!({
                "arm": "negative", "bits": bits, "p": p, "ell": ell, "dim": dim, "beta": neg_beta, "bound": bound,
                "trials": neg_trials, "raw_candidates": reported, "genuine": genuine,
                "mean_lll_ms": lll_sum / n, "mean_extract_ms": ext_sum / n,
            }));
        }
    }
    let summary = json!({
        "example": "coppersmith_s3_theta2_falsifier",
        "seed": seed, "bits": bits_list, "ell": ells, "betas": betas, "trials": trials,
        "neg_trials": neg_trials, "neg_beta": neg_beta, "rows_used": ROWS,
        "beta_det": ells.iter().map(|&e| json!({"ell": e, "beta_det": beta_det(e as usize)})).collect::<Vec<_>>(),
        "beta_asymptotic": 1.0 / 6.0, "beta_stop_rule": 0.25,
        "sieve_primes": qs.iter().map(|q| q.p).collect::<Vec<_>>(),
        "wall_s": started.elapsed().as_secs_f64(),
        "cells": cells,
    });
    fs::write(
        out_dir.join("run.json"),
        serde_json::to_string_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!(
        "\nwrote {} ({:.1} s)",
        out_dir.display(),
        started.elapsed().as_secs_f64()
    );
}
