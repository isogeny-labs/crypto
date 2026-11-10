//! Multiplicative-subgroup (power-map) factor bases for the algebraic
//! index-calculus system, written in the elementary symmetric polynomials.
//!
//! This is the membership `Membership::Subgroup` of the exponent study
//! (`research/curve25519_exponent_study_20261108`), kept in its own module so
//! that no path of `pkm_semaev` or `f4_fp_tower` changes.  It is *not* a tower
//! of `SquareRule`s: the tower engine's monomials are bit masks (exponent 0 or
//! 1 per variable), which cannot hold the odd-degree levels `y^3 = y'`,
//! `y^7 = y'`, ... of a subgroup of odd order.  The system is therefore handed
//! to the generic degree-bounded F4 of [`crate::cryptanalysis::f4_fp`], with
//! the power rules as ordinary generators.  Its counted unit is that engine's
//! `field_ops` (`F_p` multiplications of the row reduction), not the tower
//! engine's `f4_muladds`; [`Base::Bits`] builds the bit-interval base in the
//! same presentation so that the two units can be compared on one system.
//!
//! # The system
//!
//! Base `V = lo + g * mu_d` (`mu_d` the subgroup of order `d | p - 1`,
//! `d = e_0 e_1 ... e_{L-1}`, primes ascending, repeated with multiplicity).
//! For each summand `i = 1..m` the chain variables `y_{i,0..L-1}` with
//!
//! ```text
//!   x_i = lo + g * y_{i,0},
//!   y_{i,j}^{e_j} = y_{i,j+1}   (j < L-1),      y_{i,L-1}^{e_{L-1}} = 1,
//! ```
//!
//! whose zero set is exactly `y_{i,0} in mu_d` (`d` points).  The summation
//! polynomial `S_{m+1}(x_1, .., x_m, X)` (target abscissa `X` a constant) is
//! symmetric in the `x_i`; [`Form::Sym`] writes it as
//! `S~(e_1, .., e_m)` in the elementary symmetric polynomials and adds the
//! definitions `e_k - sigma_k(x_1..x_m) = 0` as generators, so the variables
//! are the chain variables followed by `e_1..e_m`.  [`Form::Plain`] substitutes
//! `e_k = sigma_k(x)` and has the chain variables only (the ablation).
//!
//! `S~` is obtained by interpolation: `S_{m+1}` is evaluated numerically
//! (closed forms for `m = 2, 3`; `Res_U(S_4(x_1,x_2,x_3,U), S_3(x_4,X,U))` for
//! `m = 4`) at random points and fitted on the monomials `e^a` with
//! `sum a_k <= 2^(m-1)` (the degree of `S_{m+1}` in one variable); extra
//! points check the fit, which also certifies the symmetry.

use std::collections::BTreeMap;

use crate::cryptanalysis::f4_fp::{self, F4Options, F4Report, Ordering, Poly};

// ── modular helpers (p < 2^32) ───────────────────────────────────────

#[inline]
fn mulm(a: u64, b: u64, p: u64) -> u64 {
    a * b % p
}
#[inline]
fn addm(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
#[inline]
fn subm(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b % p) % p
}
fn powm(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, b, p);
        }
        b = mulm(b, b, p);
        e >>= 1;
    }
    r
}
fn invm(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
}

/// A small deterministic generator (splitmix64) for the interpolation points.
pub struct Rng(pub u64);
impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

// ── numeric summation polynomials ────────────────────────────────────

/// `S_3(A, B, U) = alpha U^2 + beta U + gamma`, `(alpha, beta, gamma)`.
fn s3_abg(x1: u64, x2: u64, a: u64, b: u64, p: u64) -> [u64; 3] {
    let d = subm(x1, x2, p);
    let s = addm(x1, x2, p);
    let pr = mulm(x1, x2, p);
    let beta = subm(
        0,
        mulm(2, addm(mulm(s, addm(pr, a, p), p), mulm(2, b, p), p), p),
        p,
    );
    let q = subm(pr, a, p);
    let gamma = subm(mulm(q, q, p), mulm(mulm(4, b, p), s, p), p);
    [mulm(d, d, p), beta, gamma]
}

/// `S_4(x1, x2, x3, x4) = Res_V(S_3(x1, x2, V), S_3(x3, x4, V))`.
fn s4_eval(x1: u64, x2: u64, x3: u64, x4: u64, a: u64, b: u64, p: u64) -> u64 {
    let [al, be, ga] = s3_abg(x1, x2, a, b, p);
    let [al2, be2, ga2] = s3_abg(x3, x4, a, b, p);
    let ag = subm(mulm(al, ga2, p), mulm(al2, ga, p), p);
    let ab = subm(mulm(al, be2, p), mulm(al2, be, p), p);
    let bg = subm(mulm(be, ga2, p), mulm(be2, ga, p), p);
    subm(mulm(ag, ag, p), mulm(ab, bg, p), p)
}

/// Determinant of a square matrix mod `p` (destroys it).
fn det_mod(m: &mut [Vec<u64>], p: u64) -> u64 {
    let n = m.len();
    let mut det = 1u64;
    for c in 0..n {
        let Some(piv) = (c..n).find(|&r| m[r][c] != 0) else {
            return 0;
        };
        if piv != c {
            m.swap(piv, c);
            det = subm(0, det, p);
        }
        det = mulm(det, m[c][c], p);
        let inv = invm(m[c][c], p);
        for r in c + 1..n {
            if m[r][c] != 0 {
                let f = mulm(m[r][c], inv, p);
                for k in c..n {
                    let t = mulm(f, m[c][k], p);
                    m[r][k] = subm(m[r][k], t, p);
                }
            }
        }
    }
    det
}

/// Coefficients (low to high) of the degree-<=4 polynomial through the
/// values at `u = 0..=4`, mod `p` (`p > 4`).
fn interp5(vals: [u64; 5], p: u64) -> [u64; 5] {
    // Solve the 5x5 Vandermonde system by Gaussian elimination.
    let mut m: Vec<Vec<u64>> = (0..5u64)
        .map(|u| {
            let mut row: Vec<u64> = (0..5).map(|j| powm(u, j, p)).collect();
            row.push(vals[u as usize] % p);
            row
        })
        .collect();
    for c in 0..5 {
        let piv = (c..5).find(|&r| m[r][c] != 0).expect("Vandermonde");
        m.swap(piv, c);
        let inv = invm(m[c][c], p);
        for k in c..6 {
            m[c][k] = mulm(m[c][k], inv, p);
        }
        for r in 0..5 {
            if r != c && m[r][c] != 0 {
                let f = m[r][c];
                for k in c..6 {
                    let t = mulm(f, m[c][k], p);
                    m[r][k] = subm(m[r][k], t, p);
                }
            }
        }
    }
    [m[0][5], m[1][5], m[2][5], m[3][5], m[4][5]]
}

/// `S_5(x1, x2, x3, x4, X) = Res_U(S_4(x1, x2, x3, U), S_3(x4, X, U))`,
/// the Sylvester determinant at the formal degrees 4 and 2.
fn s5_eval(x: &[u64; 4], xt: u64, a: u64, b: u64, p: u64) -> u64 {
    let mut v = [0u64; 5];
    for (u, slot) in v.iter_mut().enumerate() {
        *slot = s4_eval(x[0], x[1], x[2], u as u64, a, b, p);
    }
    let f = interp5(v, p); // f[i] = coefficient of U^i
    let [al, be, ga] = s3_abg(x[3], xt, a, b, p);
    // g = al U^2 + be U + ga; rows high-to-low coefficients.
    let fh = [f[4], f[3], f[2], f[1], f[0]];
    let gh = [al, be, ga];
    let mut mat = vec![vec![0u64; 6]; 6];
    for k in 0..4 {
        for (j, &c) in gh.iter().enumerate() {
            mat[k][k + j] = c;
        }
    }
    for k in 0..2 {
        for (j, &c) in fh.iter().enumerate() {
            mat[4 + k][k + j] = c;
        }
    }
    det_mod(&mut mat, p)
}

/// `S_{m+1}(x_1, .., x_m, X)` for `y^2 = x^3 + a x + b` over `F_p`,
/// `m in {2, 3, 4}`.
pub fn semaev_eval(xs: &[u64], xt: u64, a: u64, b: u64, p: u64) -> u64 {
    match xs.len() {
        2 => {
            let [al, be, ga] = s3_abg(xs[0], xs[1], a, b, p);
            addm(
                addm(mulm(mulm(al, xt, p), xt, p), mulm(be, xt, p), p),
                ga,
                p,
            )
        }
        3 => s4_eval(xs[0], xs[1], xs[2], xt, a, b, p),
        4 => s5_eval(&[xs[0], xs[1], xs[2], xs[3]], xt, a, b, p),
        m => panic!("subgroup_semaev: m = {m}, only 2, 3, 4"),
    }
}

/// Elementary symmetric polynomials `e_1..e_m` of `xs`.
pub fn elementary(xs: &[u64], p: u64) -> Vec<u64> {
    let m = xs.len();
    let mut e = vec![0u64; m + 1];
    e[0] = 1;
    for &x in xs {
        for k in (1..=m).rev() {
            e[k] = addm(e[k], mulm(e[k - 1], x, p), p);
        }
    }
    e[1..].to_vec()
}

// ── the symmetrised polynomial S~(e_1..e_m; X) by interpolation ──────

fn exponent_vectors(m: usize, total: u32) -> Vec<Vec<u32>> {
    fn rec(m: usize, left: u32, cur: &mut Vec<u32>, out: &mut Vec<Vec<u32>>) {
        if cur.len() == m {
            out.push(cur.clone());
            return;
        }
        for a in 0..=left {
            cur.push(a);
            rec(m, left - a, cur, out);
            cur.pop();
        }
    }
    let mut out = Vec::new();
    rec(m, total, &mut Vec::new(), &mut out);
    out
}

/// `S~` as terms `(exponents of e_1..e_m, coefficient)`; errors if the fit
/// fails on the check points (so the polynomial is not symmetric of the
/// assumed degree).
pub fn symmetrise(
    m: usize,
    xt: u64,
    a: u64,
    b: u64,
    p: u64,
    seed: u64,
) -> Result<Vec<(Vec<u32>, u64)>, String> {
    let deg = 1u32 << (m - 1);
    let mons = exponent_vectors(m, deg);
    let n = mons.len();
    let extra = 12;
    let mut rng = Rng(seed ^ 0x5eed_5eed);
    let mut rows: Vec<Vec<u64>> = Vec::with_capacity(n + extra);
    for _ in 0..n + extra {
        let xs: Vec<u64> = (0..m).map(|_| rng.next_u64() % p).collect();
        let e = elementary(&xs, p);
        let mut row: Vec<u64> = mons
            .iter()
            .map(|mono| {
                let mut t = 1u64;
                for (k, &ak) in mono.iter().enumerate() {
                    if ak > 0 {
                        t = mulm(t, powm(e[k], u64::from(ak), p), p);
                    }
                }
                t
            })
            .collect();
        row.push(semaev_eval(&xs, xt, a, b, p));
        rows.push(row);
    }
    // Gaussian elimination on the augmented system.
    for (r0, c) in (0..n).enumerate() {
        let Some(piv) = (r0..rows.len()).find(|&r| rows[r][c] != 0) else {
            return Err(format!("symmetrise: column {c} has no pivot"));
        };
        rows.swap(piv, r0);
        let inv = invm(rows[r0][c], p);
        for k in c..=n {
            rows[r0][k] = mulm(rows[r0][k], inv, p);
        }
        for r in 0..rows.len() {
            if r != r0 && rows[r][c] != 0 {
                let f = rows[r][c];
                for k in c..=n {
                    let t = mulm(f, rows[r0][k], p);
                    rows[r][k] = subm(rows[r][k], t, p);
                }
            }
        }
    }
    for r in n..rows.len() {
        if rows[r][n] != 0 {
            return Err("symmetrise: check point disagrees with the fit".into());
        }
    }
    let mut out = Vec::new();
    for (i, mono) in mons.iter().enumerate() {
        let c = rows[i][n];
        if c != 0 {
            out.push((mono.clone(), c));
        }
    }
    Ok(out)
}

// ── bases ────────────────────────────────────────────────────────────

/// The factor base: the chain presentation of `V`.
#[derive(Clone, Debug)]
pub enum Base {
    /// `V = lo + g * mu_d`, `d = prod factors` (primes ascending, bottom
    /// first); `zeta` is unused by the system (the roots come from the
    /// rules) and kept only for [`Base::abscissae`].
    Subgroup {
        factors: Vec<u32>,
        g: u64,
        lo: u64,
        zeta: u64,
    },
    /// `V = [lo, lo + 2^t)`, `x = lo + sum 2^j b_j`, `b_j^2 = b_j`.
    Bits { t: u32, lo: u64 },
}

impl Base {
    /// Chain variables per summand.
    pub fn levels(&self) -> usize {
        match self {
            Base::Subgroup { factors, .. } => factors.len(),
            Base::Bits { t, .. } => *t as usize,
        }
    }
    /// `|V|`.
    pub fn size(&self) -> u64 {
        match self {
            Base::Subgroup { factors, .. } => factors.iter().map(|&e| u64::from(e)).product(),
            Base::Bits { t, .. } => 1u64 << t,
        }
    }
    /// `x_i = c0 + sum coef_j y_{i,j}`.
    fn linear_form(&self, p: u64) -> (u64, Vec<(usize, u64)>) {
        match self {
            Base::Subgroup { g, lo, .. } => (*lo % p, vec![(0, *g % p)]),
            Base::Bits { t, lo } => (
                *lo % p,
                (0..*t as usize).map(|j| (j, (1u64 << j) % p)).collect(),
            ),
        }
    }
    /// The `d` abscissae of `V`.
    pub fn abscissae(&self, p: u64) -> Vec<u64> {
        match self {
            Base::Subgroup {
                factors,
                g,
                lo,
                zeta,
            } => {
                let d: u64 = factors.iter().map(|&e| u64::from(e)).product();
                let mut z = 1u64;
                (0..d)
                    .map(|_| {
                        let x = addm(*lo, mulm(*g, z, p), p);
                        z = mulm(z, *zeta, p);
                        x
                    })
                    .collect()
            }
            Base::Bits { t, lo } => (0..1u64 << t).map(|v| addm(*lo, v, p)).collect(),
        }
    }
}

/// A primitive `d`-th root of unity mod `p` (`d | p - 1`).
pub fn primitive_root_of_unity(p: u64, d: u64) -> Option<u64> {
    if d == 0 || !(p - 1).is_multiple_of(d) {
        return None;
    }
    let mut fs = Vec::new();
    let mut n = d;
    let mut q = 2u64;
    while q * q <= n {
        if n.is_multiple_of(q) {
            fs.push(q);
            while n.is_multiple_of(q) {
                n /= q;
            }
        }
        q += 1;
    }
    if n > 1 {
        fs.push(n);
    }
    (2..p).find_map(|h| {
        let z = powm(h, (p - 1) / d, p);
        (z != 1 || d == 1)
            .then_some(z)
            .filter(|&z| fs.iter().all(|&q| powm(z, d / q, p) != 1))
    })
}

/// Prime factors of `d` ascending with multiplicity.
pub fn factor_ascending(mut d: u64) -> Vec<u32> {
    let mut out = Vec::new();
    let mut q = 2u64;
    while q * q <= d {
        while d.is_multiple_of(q) {
            out.push(q as u32);
            d /= q;
        }
        q += 1;
    }
    if d > 1 {
        out.push(d as u32);
    }
    out
}

// ── building the polynomial system ───────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Form {
    /// Chain variables and `e_1..e_m`, with `e_k - sigma_k(x)` generators.
    Sym,
    /// Chain variables only; `e_k` substituted by `sigma_k(x)`.
    Plain,
}

/// A system ready for [`f4_fp::f4`].
pub struct System {
    pub polys: Vec<Poly>,
    pub n_vars: usize,
    pub m: usize,
    /// Terms of `S~` (in `e`), the equation the target enters.
    pub sym_terms: usize,
    /// Terms of the generators in all.
    pub input_terms: usize,
    /// Highest total degree of a generator.
    pub input_degree: u32,
}

type Mpoly = BTreeMap<Vec<u32>, u64>;

fn mp_add(f: &mut Mpoly, e: Vec<u32>, c: u64, p: u64) {
    let v = f.entry(e).or_insert(0);
    *v = addm(*v, c, p);
}

fn mp_mul(f: &Mpoly, g: &Mpoly, p: u64) -> Mpoly {
    let mut out = Mpoly::new();
    for (e1, c1) in f {
        for (e2, c2) in g {
            let e: Vec<u32> = e1.iter().zip(e2).map(|(a, b)| a + b).collect();
            mp_add(&mut out, e, mulm(*c1, *c2, p), p);
        }
    }
    out.retain(|_, c| *c != 0);
    out
}

fn to_poly(f: &Mpoly, p: u64) -> Poly {
    let terms: Vec<(Vec<u32>, u64)> = f.iter().map(|(e, c)| (e.clone(), *c)).collect();
    f4_fp::normalise(&terms, p, Ordering::Grevlex)
}

/// Build the decomposition system for target abscissa `xt`.
pub fn build_system(
    p: u64,
    a: u64,
    b: u64,
    base: &Base,
    m: usize,
    xt: u64,
    form: Form,
    seed: u64,
) -> Result<System, String> {
    let l = base.levels();
    let chain = m * l;
    let n_vars = match form {
        Form::Sym => chain + m,
        Form::Plain => chain,
    };
    let unit = |idx: usize, k: u32| -> Vec<u32> {
        let mut e = vec![0u32; n_vars];
        e[idx] = k;
        e
    };
    let mut polys: Vec<Poly> = Vec::new();
    // chain rules
    for i in 0..m {
        match base {
            Base::Subgroup { factors, .. } => {
                for j in 0..l {
                    let mut f = Mpoly::new();
                    mp_add(&mut f, unit(i * l + j, factors[j]), 1, p);
                    if j + 1 < l {
                        mp_add(&mut f, unit(i * l + j + 1, 1), p - 1, p);
                    } else {
                        mp_add(&mut f, vec![0; n_vars], p - 1, p);
                    }
                    polys.push(to_poly(&f, p));
                }
            }
            Base::Bits { .. } => {
                for j in 0..l {
                    let mut f = Mpoly::new();
                    mp_add(&mut f, unit(i * l + j, 2), 1, p);
                    mp_add(&mut f, unit(i * l + j, 1), p - 1, p);
                    polys.push(to_poly(&f, p));
                }
            }
        }
    }
    // x_i as a polynomial
    let (c0, lin) = base.linear_form(p);
    let xpoly = |i: usize| -> Mpoly {
        let mut f = Mpoly::new();
        mp_add(&mut f, vec![0; n_vars], c0, p);
        for &(j, c) in &lin {
            mp_add(&mut f, unit(i * l + j, 1), c, p);
        }
        f
    };
    // sigma_k(x_1..x_m) as polynomials
    let mut sig: Vec<Mpoly> = vec![Mpoly::new(); m + 1];
    let mut one = Mpoly::new();
    one.insert(vec![0; n_vars], 1);
    sig[0] = one;
    for i in 0..m {
        let xi = xpoly(i);
        for k in (1..=m).rev() {
            let t = mp_mul(&sig[k - 1], &xi, p);
            for (e, c) in t {
                mp_add(&mut sig[k], e, c, p);
            }
            sig[k].retain(|_, c| *c != 0);
        }
    }
    let sym = symmetrise(m, xt, a, b, p, seed)?;
    let sym_terms = sym.len();
    match form {
        Form::Sym => {
            for k in 1..=m {
                let mut f = sig[k].clone();
                for c in f.values_mut() {
                    *c = subm(0, *c, p);
                }
                mp_add(&mut f, unit(chain + k - 1, 1), 1, p);
                f.retain(|_, c| *c != 0);
                polys.push(to_poly(&f, p));
            }
            let mut s = Mpoly::new();
            for (ex, c) in &sym {
                let mut e = vec![0u32; n_vars];
                for (k, &ak) in ex.iter().enumerate() {
                    e[chain + k] = ak;
                }
                mp_add(&mut s, e, *c, p);
            }
            polys.push(to_poly(&s, p));
        }
        Form::Plain => {
            // powers of sigma_k, cached
            let mut pw: Vec<Vec<Mpoly>> = Vec::new();
            for k in 1..=m {
                let mut v = vec![{
                    let mut o = Mpoly::new();
                    o.insert(vec![0; n_vars], 1);
                    o
                }];
                for _ in 0..(1u32 << (m - 1)) {
                    let nx = mp_mul(v.last().unwrap(), &sig[k], p);
                    v.push(nx);
                }
                pw.push(v);
            }
            let mut s = Mpoly::new();
            for (ex, c) in &sym {
                let mut t = {
                    let mut o = Mpoly::new();
                    o.insert(vec![0; n_vars], *c);
                    o
                };
                for (k, &ak) in ex.iter().enumerate() {
                    if ak > 0 {
                        t = mp_mul(&t, &pw[k][ak as usize], p);
                    }
                }
                for (e, cc) in t {
                    mp_add(&mut s, e, cc, p);
                }
            }
            s.retain(|_, c| *c != 0);
            polys.push(to_poly(&s, p));
        }
    }
    let input_terms = polys.iter().map(|f| f.len()).sum();
    let input_degree = polys
        .iter()
        .flat_map(|f| f.iter().map(|(e, _)| e.iter().sum::<u32>()))
        .max()
        .unwrap_or(0);
    Ok(System {
        polys,
        n_vars,
        m,
        sym_terms,
        input_terms,
        input_degree,
    })
}

/// Run F4 on a system with a wall-clock cap (a censoring rule only; the
/// reported quantities are counts).
pub fn solve_system(sys: &System, p: u64, cap_s: u64, max_degree: u32) -> F4Report {
    let mut opts = F4Options::new(Ordering::Grevlex, max_degree);
    opts = opts.with_budget(std::time::Duration::from_secs(cap_s));
    f4_fp::f4(&sys.polys, sys.n_vars, p, &opts)
}

#[cfg(test)]
mod tests {
    use super::*;

    // y^2 = x^3 + a x + b over F_p, affine arithmetic for the tests.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    struct Pt(Option<(u64, u64)>);

    struct Cv {
        p: u64,
        a: u64,
        b: u64,
    }
    impl Cv {
        fn rhs(&self, x: u64) -> u64 {
            let p = self.p;
            addm(
                addm(mulm(mulm(x, x, p), x, p), mulm(self.a, x, p), p),
                self.b,
                p,
            )
        }
        fn sqrt(&self, v: u64) -> Option<u64> {
            (0..self.p).find(|&y| mulm(y, y, self.p) == v)
        }
        fn add(&self, s: Pt, t: Pt) -> Pt {
            let p = self.p;
            let (Some((x1, y1)), Some((x2, y2))) = (s.0, t.0) else {
                return if s.0.is_none() { t } else { s };
            };
            let lam = if x1 == x2 {
                if addm(y1, y2, p) == 0 {
                    return Pt(None);
                }
                mulm(
                    addm(mulm(3, mulm(x1, x1, p), p), self.a, p),
                    invm(addm(y1, y1, p), p),
                    p,
                )
            } else {
                mulm(subm(y2, y1, p), invm(subm(x2, x1, p), p), p)
            };
            let x3 = subm(subm(mulm(lam, lam, p), x1, p), x2, p);
            let y3 = subm(mulm(lam, subm(x1, x3, p), p), y1, p);
            Pt(Some((x3, y3)))
        }
        fn neg(&self, s: Pt) -> Pt {
            Pt(s.0.map(|(x, y)| (x, (self.p - y) % self.p)))
        }
    }

    fn all_points(c: &Cv) -> Vec<Pt> {
        let mut v = Vec::new();
        for x in 0..c.p {
            if let Some(y) = c.sqrt(c.rhs(x)) {
                v.push(Pt(Some((x, y))));
                if y != 0 {
                    v.push(Pt(Some((x, c.p - y))));
                }
            }
        }
        v
    }

    #[test]
    fn summation_polynomial_vanishes_on_sums() {
        let c = Cv {
            p: 1013,
            a: 3,
            b: 7,
        };
        let pts = all_points(&c);
        let mut rng = Rng(11);
        for m in 2..=4usize {
            for _ in 0..40 {
                let mut sum = Pt(None);
                let mut xs = Vec::new();
                for _ in 0..m {
                    let q = pts[(rng.next_u64() % pts.len() as u64) as usize];
                    xs.push(q.0.unwrap().0);
                    sum = c.add(sum, q);
                }
                let Some((xr, _)) = sum.0 else { continue };
                assert_eq!(semaev_eval(&xs, xr, c.a, c.b, c.p), 0, "m = {m}");
                // a wrong abscissa is not a root (check a few; at most m^.. roots)
                let nonroots = (0..c.p)
                    .filter(|&x| semaev_eval(&xs, x, c.a, c.b, c.p) != 0)
                    .count();
                assert!(nonroots as u64 > c.p - 40, "m = {m}");
            }
        }
    }

    #[test]
    fn symmetrised_polynomial_equals_the_original() {
        let (p, a, b) = (1013u64, 3u64, 7u64);
        let mut rng = Rng(5);
        for m in 2..=4usize {
            let xt = rng.next_u64() % p;
            let sym = symmetrise(m, xt, a, b, p, 99).expect("fit");
            for _ in 0..20 {
                let xs: Vec<u64> = (0..m).map(|_| rng.next_u64() % p).collect();
                let e = elementary(&xs, p);
                let mut v = 0u64;
                for (ex, cf) in &sym {
                    let mut t = *cf;
                    for (k, &ak) in ex.iter().enumerate() {
                        t = mulm(t, powm(e[k], u64::from(ak), p), p);
                    }
                    v = addm(v, t, p);
                }
                assert_eq!(v, semaev_eval(&xs, xt, a, b, p), "m = {m}");
            }
        }
    }

    #[test]
    fn roots_of_unity_and_factors() {
        let p = 211u64; // p - 1 = 2 * 3 * 5 * 7
        for d in [3u64, 5, 7, 15, 21, 35, 105] {
            let z = primitive_root_of_unity(p, d).unwrap();
            assert_eq!(powm(z, d, p), 1);
            assert!((1..d).all(|k| powm(z, k, p) != 1));
        }
        assert_eq!(factor_ascending(63), vec![3, 3, 7]);
        assert_eq!(factor_ascending(73), vec![73]);
    }

    /// The system is consistent exactly when `S(x_1..x_m, X) = 0` has a root
    /// in `V^m` (all roots are rational since `V` is in `F_p`); and a target
    /// that is a sum of `m` base points is consistent.  Both forms, tiny
    /// curve, every target abscissa.
    fn consistency(p: u64, a: u64, b: u64, d: u64, m: usize, forms: &[Form]) {
        let factors = factor_ascending(d);
        let zeta = primitive_root_of_unity(p, d).unwrap();
        let base = Base::Subgroup {
            factors,
            g: 1,
            lo: 0,
            zeta,
        };
        let v = base.abscissae(p);
        assert_eq!(
            v.iter().collect::<std::collections::BTreeSet<_>>().len() as u64,
            d
        );
        // numeric brute force over V^m (ordered tuples)
        let has_root = |xt: u64| -> bool {
            let mut idx = vec![0usize; m];
            loop {
                let xs: Vec<u64> = idx.iter().map(|&i| v[i]).collect();
                if semaev_eval(&xs, xt, a, b, p) == 0 {
                    return true;
                }
                let mut k = 0;
                while k < m {
                    idx[k] += 1;
                    if idx[k] < v.len() {
                        break;
                    }
                    idx[k] = 0;
                    k += 1;
                }
                if k == m {
                    return false;
                }
            }
        };
        let c = Cv { p, a, b };
        // targets that are sums of base points on the curve
        let base_pts: Vec<Pt> = v
            .iter()
            .filter_map(|&x| c.sqrt(c.rhs(x)).map(|y| Pt(Some((x, y)))))
            .flat_map(|q| [q, c.neg(q)])
            .collect();
        let mut sums = std::collections::BTreeSet::new();
        let mut cur = vec![Pt(None)];
        for _ in 0..m {
            let mut nx = Vec::new();
            for &s in &cur {
                for &q in &base_pts {
                    nx.push(c.add(s, q));
                }
            }
            nx.sort_by_key(|q| q.0);
            nx.dedup();
            cur = nx;
        }
        for s in cur {
            if let Some((x, _)) = s.0 {
                sums.insert(x);
            }
        }
        let mut n_consistent = 0;
        for xt in 0..p {
            let want = has_root(xt);
            if sums.contains(&xt) {
                assert!(want, "a sum of base points must be a root, X = {xt}");
            }
            for &form in forms {
                let sys = build_system(p, a, b, &base, m, xt, form, 3).unwrap();
                let rep = solve_system(&sys, p, 120, 200);
                assert!(!rep.timed_out && rep.pairs_above_bound == 0);
                assert_eq!(
                    !rep.inconsistent, want,
                    "X = {xt}, {form:?}, d = {d}, m = {m}"
                );
            }
            n_consistent += usize::from(want);
        }
        assert!(n_consistent > 0 && (n_consistent as u64) < p);
    }

    #[test]
    fn m2_system_consistent_iff_decomposable() {
        consistency(211, 5, 9, 7, 2, &[Form::Sym, Form::Plain]);
        consistency(211, 5, 9, 15, 2, &[Form::Sym, Form::Plain]);
    }

    #[test]
    fn m3_system_consistent_iff_decomposable() {
        consistency(211, 5, 9, 7, 3, &[Form::Sym, Form::Plain]);
    }

    /// The bit-interval base in the same presentation: a root exists exactly
    /// when the numeric brute force over `[lo, lo + 2^t)^m` finds one.
    #[test]
    fn bits_base_matches_brute_force() {
        let (p, a, b) = (211u64, 5u64, 9u64);
        let base = Base::Bits { t: 3, lo: 4 };
        let v = base.abscissae(p);
        for xt in 0..p {
            let want = v
                .iter()
                .any(|&x1| v.iter().any(|&x2| semaev_eval(&[x1, x2], xt, a, b, p) == 0));
            let sys = build_system(p, a, b, &base, 2, xt, Form::Sym, 1).unwrap();
            let rep = solve_system(&sys, p, 120, 200);
            assert_eq!(!rep.inconsistent, want, "X = {xt}");
        }
    }
}
