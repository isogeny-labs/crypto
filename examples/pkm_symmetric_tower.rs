//! **The symmetric PKM tower** (item 1 of the 2026-10-26 triage): does
//! writing the Kummer factor-base membership and the point sum in the
//! elementary symmetric functions of the summands, before any expansion,
//! lower the F4 solving degree `D` of the decomposition system?
//!
//! Kummer base `V = g·μ_{2^t}`.  For `m` summands `x_i = g·w_i`, the plain
//! PKM system (`research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`)
//! has `t` tower unknowns per summand, `y_{i,0} = w_i`,
//! `y_{i,j+1} = y_{i,j}²`, `y_{i,t−1}² = 1`, and the summation part.
//!
//! The symmetric system replaces the `m` towers by one: with
//! `Q(X) = ∏ (X − w_i) = X^m − e_1 X^{m−1} + … + (−1)^m e_m`, distinct
//! `w_i ∈ μ_{2^t}` is `Q | X^{2^t} − 1`, i.e. `X^{2^t} ≡ 1 mod Q`.  The
//! unknowns are `e_1..e_m` and the coefficients of `R_j ≡ X^{2^j} mod Q`
//! for `j = 1..t−1` (`m` each), so `N = m·t`, the plain system's count.  The
//! equations are `R_j = R_{j−1}² mod Q` and `R_{t−1}² ≡ 1 mod Q`; the
//! summation polynomial `S_{m+1}(g w_1, …, g w_m, x_R)` is symmetric and is
//! rewritten in the `e_k`.  It sees unordered summand sets only: its
//! solution count is the plain one divided by `m!` (distinct summands).
//!
//! Arms, every one solved by `f4_fp` (grevlex, degree bound `n + d + 6` as
//! the pilot), on the same prime, curve, `g`, `ζ` and targets:
//!
//! - `plain-chain`: the pilot's `tower` control (`S₃` at `m = 2`, the `S₃`
//!   chain with partial-sum unknowns at `m ≥ 3`): the recorded baseline;
//! - `plain-smp1`: the plain towers with one `S_{m+1}` (`m = 3`: `S_4`, by
//!   resultant), isolating "chain vs one summation polynomial";
//! - `symmetric`: the symmetric tower with `S_{m+1}` in the `e_k`.
//!
//! Every system is first checked on a planted decomposition (a random pair
//! or triple of base elements and an `x_R` that solves `S_{m+1}`), so a
//! construction bug stops the run instead of reading as a degree.  Measured
//! rows use random `x_R` (the pilot's `--planted 0 --random 2`), mostly
//! inconsistent systems, as the recorded `D` were.
//!
//! `cargo run --release --example pkm_symmetric_tower -- [--p P] [--m 2,3]
//!  [--t-max-m2 9] [--t-max-m3 5] [--random 2] [--budget 600]`
//!
//! Output: JSON lines (one per arm, cell and target).  Stage diagnostic of
//! the solver axis (AGENTS.md §2): no `S`, no pipeline, no claim.

use std::collections::HashMap;
use std::time::Duration;

use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly};

// ------------------------------------------------------ counters ----
//
// Counted unit for the pre-F4 stages (2026-11-15 pricing study,
// `research/pkm_priced_20261115/`).  Every field operation executed by this
// file's own arithmetic (`mulm`, `addm`, `subm`, `invm`) is booked to the
// current *stage* of the current thread.  Kinds:
//   0  mulmod outside inversions      1  add / sub
//   2  inversion calls                3  mulmods inside Fermat inversions
// The mulmod total of a stage is kind 0 + kind 3.  The counters never change a
// value; the existing arms' rows are unchanged.  The F4 stage keeps its own
// counter (`F4Report::field_ops`, row multiply-subtracts) and is not booked here.
mod cnt {
    use std::cell::Cell;

    pub const NST: usize = 11;
    pub const STAGES: [&str; NST] = [
        "other",
        "s4_eval",
        "s3_eval",
        "sylvester_det",
        "vandermonde_inv",
        "grid_interp",
        "subst_gw",
        "to_elementary",
        "param_xr_interp",
        "param_eval",
        "planted_check",
    ];
    pub const OTHER: usize = 0;
    pub const S4_EVAL: usize = 1;
    pub const S3_EVAL: usize = 2;
    pub const DET: usize = 3;
    pub const VINV: usize = 4;
    pub const GRID_INTERP: usize = 5;
    pub const SUBST: usize = 6;
    pub const ELEM: usize = 7;
    pub const PARAM_INTERP: usize = 8;
    pub const PARAM_EVAL: usize = 9;
    pub const CHECK: usize = 10;
    /// Stages that make up one as-built (grid) S5 build.
    pub const GRID_BUILD: [usize; 7] = [S4_EVAL, S3_EVAL, DET, VINV, GRID_INTERP, SUBST, ELEM];

    pub type Snap = [[u64; 4]; NST];

    thread_local! {
        static STAGE: Cell<usize> = const { Cell::new(0) };
        static C: [[Cell<u64>; 4]; NST] =
            const { [const { [const { Cell::new(0) }; 4] }; NST] };
    }

    #[inline]
    pub fn bump(kind: usize) {
        add(kind, 1);
    }
    #[inline]
    pub fn add(kind: usize, n: u64) {
        let s = STAGE.with(|s| s.get());
        C.with(|c| c[s][kind].set(c[s][kind].get() + n));
    }
    #[inline]
    pub fn sub(kind: usize, n: u64) {
        let s = STAGE.with(|s| s.get());
        C.with(|c| c[s][kind].set(c[s][kind].get() - n));
    }
    #[inline]
    pub fn get(kind: usize) -> u64 {
        let s = STAGE.with(|s| s.get());
        C.with(|c| c[s][kind].get())
    }
    pub fn snapshot() -> Snap {
        let mut out = [[0u64; 4]; NST];
        C.with(|c| {
            for (s, row) in c.iter().enumerate() {
                for (k, v) in row.iter().enumerate() {
                    out[s][k] = v.get();
                }
            }
        });
        out
    }
    pub fn diff(a: &Snap, b: &Snap) -> Snap {
        let mut out = [[0u64; 4]; NST];
        for s in 0..NST {
            for k in 0..4 {
                out[s][k] = a[s][k] - b[s][k];
            }
        }
        out
    }
    /// Mulmods (kind 0 + kind 3) summed over the listed stages.
    pub fn mulmods(d: &Snap, stages: &[usize]) -> u64 {
        stages.iter().map(|&s| d[s][0] + d[s][3]).sum()
    }
    pub fn addsubs(d: &Snap, stages: &[usize]) -> u64 {
        stages.iter().map(|&s| d[s][1]).sum()
    }
    pub fn invs(d: &Snap, stages: &[usize]) -> u64 {
        stages.iter().map(|&s| d[s][2]).sum()
    }

    /// RAII stage marker: books everything inside its scope to `stage`.
    pub struct Guard(usize);
    impl Guard {
        pub fn enter(stage: usize) -> Guard {
            Guard(STAGE.with(|s| s.replace(stage)))
        }
    }
    impl Drop for Guard {
        fn drop(&mut self) {
            STAGE.with(|s| s.set(self.0));
        }
    }
}

// ------------------------------------------------------------ field ----

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    cnt::bump(0);
    ((a as u128 * b as u128) % p as u128) as u64
}
fn addm(a: u64, b: u64, p: u64) -> u64 {
    cnt::bump(1);
    (a + b) % p
}
fn subm(a: u64, b: u64, p: u64) -> u64 {
    cnt::bump(1);
    (a + p - b) % p
}
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
/// Fermat inversion.  Its multiplications are booked under kind 3
/// ("inside inversions"), not kind 0, and the call under kind 2.
fn invm(a: u64, p: u64) -> u64 {
    let before = cnt::get(0);
    let r = powm(a, p - 2, p);
    let used = cnt::get(0) - before;
    cnt::sub(0, used);
    cnt::add(3, used);
    cnt::bump(2);
    r
}
/// Tonelli–Shanks; `None` for a non-residue.
fn sqrtm(a: u64, p: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }
    if powm(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    let (mut q, mut s) = (p - 1, 0);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while powm(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let (mut m, mut c, mut t, mut r) = (s, powm(z, q, p), powm(a, q, p), powm(a, q.div_ceil(2), p));
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = mulm(tt, tt, p);
            i += 1;
        }
        let b = powm(c, 1 << (m - i - 1), p);
        m = i;
        c = mulm(b, b, p);
        t = mulm(t, c, p);
        r = mulm(r, b, p);
    }
    Some(r)
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

// ------------------------------------------------------ polynomials ----

#[derive(Clone, Debug)]
struct Pol {
    n: usize,
    t: HashMap<Vec<u32>, u64>,
}

impl Pol {
    fn zero(n: usize) -> Self {
        Pol {
            n,
            t: HashMap::new(),
        }
    }
    fn c(n: usize, c: u64, p: u64) -> Self {
        let mut f = Self::zero(n);
        if !c.is_multiple_of(p) {
            f.t.insert(vec![0; n], c % p);
        }
        f
    }
    fn var(n: usize, i: usize) -> Self {
        let mut e = vec![0; n];
        e[i] = 1;
        let mut f = Self::zero(n);
        f.t.insert(e, 1);
        f
    }
    fn add(&self, o: &Pol, p: u64) -> Pol {
        let mut f = self.clone();
        for (e, &c) in &o.t {
            let v = f.t.entry(e.clone()).or_insert(0);
            *v = addm(*v, c, p);
        }
        f.t.retain(|_, c| *c != 0);
        f
    }
    fn scale(&self, k: u64, p: u64) -> Pol {
        let mut f = Pol::zero(self.n);
        for (e, &c) in &self.t {
            let v = mulm(c, k % p, p);
            if v != 0 {
                f.t.insert(e.clone(), v);
            }
        }
        f
    }
    fn sub(&self, o: &Pol, p: u64) -> Pol {
        self.add(&o.scale(p - 1, p), p)
    }
    fn mul(&self, o: &Pol, p: u64) -> Pol {
        let mut f = Pol::zero(self.n);
        for (ea, &ca) in &self.t {
            for (eb, &cb) in &o.t {
                let e: Vec<u32> = ea.iter().zip(eb).map(|(a, b)| a + b).collect();
                let v = f.t.entry(e).or_insert(0);
                *v = addm(*v, mulm(ca, cb, p), p);
            }
        }
        f.t.retain(|_, c| *c != 0);
        f
    }
    fn eval(&self, x: &[u64], p: u64) -> u64 {
        let mut s = 0;
        for (e, &c) in &self.t {
            let mut v = c;
            for (i, &k) in e.iter().enumerate() {
                v = mulm(v, powm(x[i], k as u64, p), p);
            }
            s = addm(s, v, p);
        }
        s
    }
    fn degree(&self) -> u32 {
        self.t.keys().map(|e| e.iter().sum()).max().unwrap_or(0)
    }
    /// Substitute variable `i` by the polynomial `g` (in the same ring).
    fn subst(&self, i: usize, g: &Pol, p: u64) -> Pol {
        let mut out = Pol::zero(self.n);
        let maxk = self.t.keys().map(|e| e[i]).max().unwrap_or(0);
        let mut pw = vec![Pol::c(self.n, 1, p)];
        for k in 1..=maxk as usize {
            let nx = pw[k - 1].mul(g, p);
            pw.push(nx);
        }
        for (e, &c) in &self.t {
            let mut mono = e.clone();
            let k = mono[i] as usize;
            mono[i] = 0;
            let mut m = Pol::zero(self.n);
            m.t.insert(mono, c);
            out = out.add(&m.mul(&pw[k], p), p);
        }
        out
    }
    /// Re-index into a ring of `n2` variables by `map[i]`.
    fn remap(&self, n2: usize, map: &[usize]) -> Pol {
        let mut f = Pol::zero(n2);
        for (e, &c) in &self.t {
            let mut e2 = vec![0u32; n2];
            for (i, &k) in e.iter().enumerate() {
                if k > 0 {
                    e2[map[i]] += k;
                }
            }
            f.t.insert(e2, c);
        }
        f
    }
    fn to_poly(&self, p: u64) -> Poly {
        let terms: Vec<(Vec<u32>, u64)> = self.t.iter().map(|(e, &c)| (e.clone(), c)).collect();
        f4_fp::normalise(&terms, p, Ordering::Grevlex)
    }
}

fn s3(x1: &Pol, x2: &Pol, x3: &Pol, a: u64, b: u64, p: u64) -> Pol {
    let n = x1.n;
    let d = x1.sub(x2, p);
    let t1 = d.mul(&d, p).mul(x3, p).mul(x3, p);
    let s = x1.add(x2, p);
    let pr = x1.mul(x2, p);
    let inner = s
        .mul(&pr.add(&Pol::c(n, a, p), p), p)
        .add(&Pol::c(n, mulm(2, b, p), p), p);
    let t2 = inner.mul(x3, p).scale(p - 2, p);
    let q = pr.sub(&Pol::c(n, a, p), p);
    let t3 = q.mul(&q, p).sub(&s.scale(mulm(4, b, p), p), p);
    t1.add(&t2, p).add(&t3, p)
}

/// `S_{m+1}(x_1..x_m, x_R)` in an `m`-variable ring (`x_i` = variable `i`),
/// by resultant in the partial sum `u`: `S_3` for `m = 2`,
/// `S_4 = Res_u(S_3(x_1, x_2, u), S_3(u, x_3, x_R))` for `m = 3`.
fn summation_poly(m: usize, x_r: u64, a: u64, b: u64, p: u64) -> Pol {
    if m == 2 {
        let n = 2;
        return s3(
            &Pol::var(n, 0),
            &Pol::var(n, 1),
            &Pol::c(n, x_r, p),
            a,
            b,
            p,
        );
    }
    if m == 4 {
        return summation_poly_5(x_r, a, b, p);
    }
    assert_eq!(m, 3);
    // Work in variables (x1, x2, x3, u), then eliminate u (degree 2 in each).
    let n = 4;
    let f = s3(&Pol::var(n, 0), &Pol::var(n, 1), &Pol::var(n, 3), a, b, p);
    let g = s3(
        &Pol::var(n, 3),
        &Pol::var(n, 2),
        &Pol::c(n, x_r, p),
        a,
        b,
        p,
    );
    let coeffs = |h: &Pol| -> [Pol; 3] {
        let mut c = [Pol::zero(n), Pol::zero(n), Pol::zero(n)];
        for (e, &v) in &h.t {
            let mut e2 = e.clone();
            let k = e2[3] as usize;
            e2[3] = 0;
            let mut mono = Pol::zero(n);
            mono.t.insert(e2, v);
            c[k] = c[k].add(&mono, p);
        }
        c
    };
    let [f0, f1, f2] = coeffs(&f);
    let [g0, g1, g2] = coeffs(&g);
    // Res of a2 u² + a1 u + a0 and b2 u² + b1 u + b0:
    // (a2 b0 − a0 b2)² − (a2 b1 − a1 b2)(a1 b0 − a0 b1).
    let t1 = f2.mul(&g0, p).sub(&f0.mul(&g2, p), p);
    let t2 = f2.mul(&g1, p).sub(&f1.mul(&g2, p), p);
    let t3 = f1.mul(&g0, p).sub(&f0.mul(&g1, p), p);
    let r = t1.mul(&t1, p).sub(&t2.mul(&t3, p), p);
    r.remap(3, &[0, 1, 2, 0])
}

/// Univariate polynomials over `F_p`, constant term first.
fn upoly_mul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = addm(c[i + j], mulm(x, y, p), p);
        }
    }
    c
}
fn upoly_add(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut c = vec![0u64; a.len().max(b.len())];
    for (i, &x) in a.iter().enumerate() {
        c[i] = addm(c[i], x, p);
    }
    for (i, &x) in b.iter().enumerate() {
        c[i] = addm(c[i], x, p);
    }
    c
}
fn upoly_scale(a: &[u64], k: u64, p: u64) -> Vec<u64> {
    a.iter().map(|&x| mulm(x, k, p)).collect()
}

/// `S_3(v, c, u)` as a quadratic in `v` whose coefficients are polynomials in
/// `u` (each of degree ≤ 2), constant term first.
fn s3_in_v_over_u(c: u64, a: u64, b: u64, p: u64) -> [Vec<u64>; 3] {
    let f = s3(&Pol::var(2, 0), &Pol::c(2, c, p), &Pol::var(2, 1), a, b, p);
    let mut out = [vec![0u64; 3], vec![0u64; 3], vec![0u64; 3]];
    for (e, &v) in &f.t {
        out[e[0] as usize][e[1] as usize] = addm(out[e[0] as usize][e[1] as usize], v, p);
    }
    out
}

/// `S_3(u, c, d)` as a quadratic in `u`, constant term first.
fn s3_in_u(c: u64, d: u64, a: u64, b: u64, p: u64) -> Vec<u64> {
    let f = s3(&Pol::var(1, 0), &Pol::c(1, c, p), &Pol::c(1, d, p), a, b, p);
    let mut out = vec![0u64; 3];
    for (e, &v) in &f.t {
        out[e[0] as usize] = addm(out[e[0] as usize], v, p);
    }
    out
}

/// `S_4(x1, x2, x3, u)` as a polynomial in `u` (formal degree 4):
/// `Res_v(S_3(x1, x2, v), S_3(v, x3, u))`.
fn s4_in_u(x1: u64, x2: u64, x3: u64, a: u64, b: u64, p: u64) -> Vec<u64> {
    let fa = s3_in_u(x1, x2, a, b, p); // S_3(v, x1, x2) = S_3(x1, x2, v), symmetric
    let [b0, b1, b2] = s3_in_v_over_u(x3, a, b, p);
    let (a0, a1, a2) = (vec![fa[0]], vec![fa[1]], vec![fa[2]]);
    let neg = |v: &[u64]| upoly_scale(v, p - 1, p);
    let t1 = upoly_add(&upoly_mul(&a2, &b0, p), &neg(&upoly_mul(&a0, &b2, p)), p);
    let t2 = upoly_add(&upoly_mul(&a2, &b1, p), &neg(&upoly_mul(&a1, &b2, p)), p);
    let t3 = upoly_add(&upoly_mul(&a1, &b0, p), &neg(&upoly_mul(&a0, &b1, p)), p);
    let mut r = upoly_add(&upoly_mul(&t1, &t1, p), &neg(&upoly_mul(&t2, &t3, p)), p);
    r.resize(5, 0);
    r
}

/// Determinant mod `p` by Gaussian elimination.
fn det_mod(mut m: Vec<Vec<u64>>, p: u64) -> u64 {
    let n = m.len();
    let mut det = 1u64;
    for c in 0..n {
        let Some(r) = (c..n).find(|&r| m[r][c] != 0) else {
            return 0;
        };
        if r != c {
            m.swap(r, c);
            det = mulm(det, p - 1, p);
        }
        det = mulm(det, m[c][c], p);
        let inv = invm(m[c][c], p);
        for r in c + 1..n {
            let f = mulm(m[r][c], inv, p);
            if f == 0 {
                continue;
            }
            for k in c..n {
                m[r][k] = subm(m[r][k], mulm(f, m[c][k], p), p);
            }
        }
    }
    det
}

/// Sylvester resultant of `f` (formal degree `df`) and `g` (formal degree `dg`).
fn sylvester(f: &[u64], df: usize, g: &[u64], dg: usize, p: u64) -> u64 {
    let n = df + dg;
    let mut m = vec![vec![0u64; n]; n];
    for i in 0..dg {
        for k in 0..=df {
            m[i][i + k] = f[df - k];
        }
    }
    for i in 0..df {
        for k in 0..=dg {
            m[dg + i][i + k] = g[dg - k];
        }
    }
    det_mod(m, p)
}

/// Inverse of the Vandermonde matrix on `nodes` (Gauss-Jordan, mod `p`):
/// coefficient vector = `V^{-1}` times the values.
fn vandermonde_inverse(nodes: &[u64], p: u64) -> Vec<Vec<u64>> {
    #[allow(non_snake_case)]
    let K = nodes.len();
    let mut v: Vec<Vec<u64>> = (0..K)
        .map(|i| (0..K).map(|j| powm(nodes[i], j as u64, p)).collect())
        .collect();
    let mut inv: Vec<Vec<u64>> = (0..K)
        .map(|i| (0..K).map(|j| (i == j) as u64).collect())
        .collect();
    for c in 0..K {
        let r = (c..K)
            .find(|&r| v[r][c] != 0)
            .expect("Vandermonde invertible");
        v.swap(r, c);
        inv.swap(r, c);
        let ic = invm(v[c][c], p);
        for k in 0..K {
            v[c][k] = mulm(v[c][k], ic, p);
            inv[c][k] = mulm(inv[c][k], ic, p);
        }
        for r in 0..K {
            if r != c && v[r][c] != 0 {
                let f = v[r][c];
                for k in 0..K {
                    v[r][k] = subm(v[r][k], mulm(f, v[c][k], p), p);
                    inv[r][k] = subm(inv[r][k], mulm(f, inv[c][k], p), p);
                }
            }
        }
    }
    inv
}

/// `S_5(x1, x2, x3, x4, x_R)` with `x_R` fixed, in four variables:
/// `Res_u(S_4(x1, x2, x3, u), S_3(u, x4, x_R))`, degree ≤ 8 in each variable,
/// recovered exactly by evaluation on a 9⁴ grid and tensor Lagrange
/// interpolation.
fn summation_poly_5(x_r: u64, a: u64, b: u64, p: u64) -> Pol {
    const K: usize = 9;
    let nodes: Vec<u64> = (0..K as u64).collect();
    let mut vals = vec![0u64; K * K * K * K];
    for i1 in 0..K {
        for i2 in 0..K {
            for i3 in 0..K {
                let s4 = {
                    let _g = cnt::Guard::enter(cnt::S4_EVAL);
                    s4_in_u(nodes[i1], nodes[i2], nodes[i3], a, b, p)
                };
                for i4 in 0..K {
                    let g = {
                        let _g = cnt::Guard::enter(cnt::S3_EVAL);
                        s3_in_u(nodes[i4], x_r, a, b, p)
                    };
                    let _g = cnt::Guard::enter(cnt::DET);
                    vals[((i1 * K + i2) * K + i3) * K + i4] = sylvester(&s4, 4, &g, 2, p);
                }
            }
        }
    }
    let inv = {
        let _g = cnt::Guard::enter(cnt::VINV);
        vandermonde_inverse(&nodes, p)
    };
    let _g = cnt::Guard::enter(cnt::GRID_INTERP);
    // Apply V^{-1} along each axis.
    let idx = |a: [usize; 4]| ((a[0] * K + a[1]) * K + a[2]) * K + a[3];
    for axis in 0..4 {
        let mut out = vec![0u64; vals.len()];
        for flat in 0..vals.len() {
            let mut a = [
                flat / (K * K * K),
                flat / (K * K) % K,
                flat / K % K,
                flat % K,
            ];
            let deg = a[axis];
            let mut acc = 0u64;
            for node in 0..K {
                a[axis] = node;
                acc = addm(acc, mulm(inv[deg][node], vals[idx(a)], p), p);
            }
            out[flat] = acc;
        }
        vals = out;
    }
    let mut f = Pol::zero(4);
    for (flat, &c) in vals.iter().enumerate() {
        if c != 0 {
            let e = vec![
                (flat / (K * K * K)) as u32,
                (flat / (K * K) % K) as u32,
                (flat / K % K) as u32,
                (flat % K) as u32,
            ];
            f.t.insert(e, c);
        }
    }
    f
}

/// Rewrite a symmetric polynomial in `m` variables as a polynomial in the
/// elementary symmetric functions (`m` variables `e_1..e_m`), by the
/// leading-term algorithm (lex).  Panics if the input is not symmetric.
fn to_elementary(f: &Pol, m: usize, p: u64) -> Pol {
    let mut e_x: Vec<Pol> = Vec::new();
    for k in 1..=m {
        // e_k(x_1..x_m)
        let mut s = Pol::zero(m);
        for mask in 0u32..(1 << m) {
            if mask.count_ones() as usize == k {
                let mut e = vec![0u32; m];
                for (i, ei) in e.iter_mut().enumerate() {
                    if mask >> i & 1 == 1 {
                        *ei = 1;
                    }
                }
                s.t.insert(e, 1);
            }
        }
        e_x.push(s);
    }
    let mut rest = f.clone();
    let mut out = Pol::zero(m);
    while !rest.t.is_empty() {
        let (lead, &c) = rest.t.iter().max_by(|a, b| a.0.cmp(b.0)).unwrap();
        let lead = lead.clone();
        for i in 1..m {
            assert!(
                lead[i - 1] >= lead[i],
                "not symmetric: leading exponent {lead:?}"
            );
        }
        let mut pw = vec![0u32; m];
        let mut term = Pol::c(m, c, p);
        for k in 0..m {
            let ek = lead[k] - if k + 1 < m { lead[k + 1] } else { 0 };
            pw[k] = ek;
            for _ in 0..ek {
                term = term.mul(&e_x[k], p);
            }
        }
        rest = rest.sub(&term, p);
        let mut mono = Pol::zero(m);
        mono.t.insert(pw, c);
        out = out.add(&mono, p);
    }
    out
}

// --------------------------------------------------------- systems ----

struct Instance {
    p: u64,
    a: u64,
    b: u64,
    g: u64,
    zeta: u64,
    t: usize,
}

impl Instance {
    fn base(&self) -> Vec<u64> {
        (0..1u64 << self.t)
            .map(|k| mulm(self.g, powm(self.zeta, k, self.p), self.p))
            .collect()
    }
}

/// Plain towers: variables `y_{i,j}` at `i·t + j`, then the chain's
/// partial sums (`m ≥ 3`, chain arm only).
fn plain_system(inst: &Instance, m: usize, x_r: u64, chain: bool) -> (Vec<Pol>, usize) {
    let (p, t) = (inst.p, inst.t);
    let n = m * t + if chain && m >= 3 { m - 2 } else { 0 };
    let x = |i: usize| Pol::var(n, i * t).scale(inst.g, p);
    let mut eqs = Vec::new();
    if m == 2 {
        eqs.push(s3(&x(0), &x(1), &Pol::c(n, x_r, p), inst.a, inst.b, p));
    } else if chain {
        let u = |k: usize| Pol::var(n, m * t + k);
        eqs.push(s3(&x(0), &x(1), &u(0), inst.a, inst.b, p));
        for k in 1..m - 2 {
            eqs.push(s3(&u(k - 1), &x(k + 1), &u(k), inst.a, inst.b, p));
        }
        eqs.push(s3(
            &u(m - 3),
            &x(m - 1),
            &Pol::c(n, x_r, p),
            inst.a,
            inst.b,
            p,
        ));
    } else {
        let s = summation_poly(m, x_r, inst.a, inst.b, p);
        let mut f = s.remap(n, &(0..m).map(|i| i * t).collect::<Vec<_>>());
        for i in 0..m {
            f = f.subst(i * t, &Pol::var(n, i * t).scale(inst.g, p), p);
        }
        eqs.push(f);
    }
    for i in 0..m {
        for j in 0..t {
            let y = Pol::var(n, i * t + j);
            let next = if j + 1 < t {
                Pol::var(n, i * t + j + 1)
            } else {
                Pol::c(n, 1, p)
            };
            eqs.push(next.sub(&y.mul(&y, p), p));
        }
    }
    (eqs, n)
}

/// Symmetric tower: `e_1..e_m` at `0..m`, `R_j` coefficients (constant
/// term first) at `m·j .. m·j + m` for `j = 1..t−1`.
fn symmetric_system(inst: &Instance, m: usize, x_r: u64) -> (Vec<Pol>, usize) {
    let se = s_in_e_grid(inst, m, x_r);
    symmetric_system_from(inst, m, &se)
}

/// `S_{m+1}(g w_1..g w_m, x_R)` written in `e_k(w)`, as built by the
/// original instrument: grid evaluation + interpolation (m = 4), then the
/// substitution `x_i = g w_i` and the elementary-symmetric rewrite.
fn s_in_e_grid(inst: &Instance, m: usize, x_r: u64) -> Pol {
    let p = inst.p;
    let s = summation_poly(m, x_r, inst.a, inst.b, p);
    let mut sw = s.clone();
    {
        let _g = cnt::Guard::enter(cnt::SUBST);
        for i in 0..m {
            sw = sw.subst(i, &Pol::var(m, i).scale(inst.g, p), p);
        }
    }
    let _g = cnt::Guard::enter(cnt::ELEM);
    to_elementary(&sw, m, p)
}

/// The symmetric tower system around a given `S_{m+1}(e)` polynomial `se`.
fn symmetric_system_from(inst: &Instance, m: usize, se: &Pol) -> (Vec<Pol>, usize) {
    let (p, t) = (inst.p, inst.t);
    let n = m * t;
    let e = |k: usize| Pol::var(n, k - 1);
    // X^m ≡ Σ_{k=1}^m (−1)^{k+1} e_k X^{m−k}.
    let reduce = |coef: Vec<Pol>| -> Vec<Pol> {
        let mut c = coef;
        while c.len() > m {
            let top = c.pop().unwrap();
            let d = c.len(); // top was the coefficient of X^d, d ≥ m
            for k in 1..=m {
                let term = top.mul(&e(k), p);
                let idx = d - k;
                c[idx] = if k % 2 == 1 {
                    c[idx].add(&term, p)
                } else {
                    c[idx].sub(&term, p)
                };
            }
        }
        while c.len() < m {
            c.push(Pol::zero(n));
        }
        c
    };
    let square = |r: &[Pol]| -> Vec<Pol> {
        let mut c = vec![Pol::zero(n); 2 * r.len() - 1];
        for i in 0..r.len() {
            for j in 0..r.len() {
                c[i + j] = c[i + j].add(&r[i].mul(&r[j], p), p);
            }
        }
        reduce(c)
    };
    let mut eqs = Vec::new();
    // R_0 = X.
    let mut r: Vec<Pol> = (0..m)
        .map(|k| {
            if k == 1 {
                Pol::c(n, 1, p)
            } else {
                Pol::zero(n)
            }
        })
        .collect();
    if m == 1 {
        r = reduce(vec![Pol::zero(n), Pol::c(n, 1, p)]);
    }
    for j in 1..t {
        let next = square(&r);
        let vars: Vec<Pol> = (0..m).map(|k| Pol::var(n, m * j + k)).collect();
        for k in 0..m {
            eqs.push(vars[k].sub(&next[k], p));
        }
        r = vars;
    }
    let last = square(&r);
    for (k, c) in last.iter().enumerate() {
        let target = if k == 0 {
            Pol::c(n, 1, p)
        } else {
            Pol::zero(n)
        };
        eqs.push(c.sub(&target, p));
    }
    // S_{m+1}(g w_1..g w_m, x_R) in e_k(w), supplied by the caller.
    eqs.insert(0, se.remap(n, &(0..m).collect::<Vec<_>>()));
    (eqs, n)
}

/// The symmetric system's planted values from summand roots `w`.
fn symmetric_solution(inst: &Instance, m: usize, w: &[u64]) -> Vec<u64> {
    let (p, t) = (inst.p, inst.t);
    // Q coefficients: e_k.
    let mut ek = vec![0u64; m + 1];
    ek[0] = 1;
    for &wi in w {
        for k in (1..=m).rev() {
            ek[k] = addm(ek[k], mulm(ek[k - 1], wi, p), p);
        }
    }
    let mut sol = vec![0u64; m * t];
    sol[..m].copy_from_slice(&ek[1..=m]);
    // R_j = X^{2^j} mod Q, numerically.
    let mulmod = |a: &[u64], b: &[u64]| -> Vec<u64> {
        let mut c = vec![0u64; 2 * m - 1];
        for i in 0..m {
            for j in 0..m {
                c[i + j] = addm(c[i + j], mulm(a[i], b[j], p), p);
            }
        }
        while c.len() > m {
            let top = c.pop().unwrap();
            let d = c.len();
            for k in 1..=m {
                let term = mulm(top, ek[k], p);
                c[d - k] = if k % 2 == 1 {
                    addm(c[d - k], term, p)
                } else {
                    subm(c[d - k], term, p)
                };
            }
        }
        c
    };
    let mut r: Vec<u64> = (0..m).map(|k| (k == 1) as u64).collect();
    for j in 1..t {
        r = mulmod(&r, &r);
        sol[m * j..m * j + m].copy_from_slice(&r);
    }
    sol
}

fn plain_solution(inst: &Instance, w: &[u64], partial: &[u64]) -> Vec<u64> {
    let (p, t) = (inst.p, inst.t);
    let mut sol = Vec::new();
    for &wi in w {
        let mut y = wi;
        for _ in 0..t {
            sol.push(y);
            y = mulm(y, y, p);
        }
    }
    sol.extend_from_slice(partial);
    sol
}

/// A root `x` of the quadratic `S_3(x1, x2, X)`.
fn s3_root(x1: u64, x2: u64, a: u64, b: u64, p: u64) -> Option<u64> {
    let f = s3(
        &Pol::c(1, x1, p),
        &Pol::c(1, x2, p),
        &Pol::var(1, 0),
        a,
        b,
        p,
    );
    let co = |k: u32| f.t.get(&vec![k]).copied().unwrap_or(0);
    let (c2, c1, c0) = (co(2), co(1), co(0));
    if c2 == 0 {
        return None;
    }
    let disc = subm(mulm(c1, c1, p), mulm(4, mulm(c2, c0, p), p), p);
    let s = sqrtm(disc, p)?;
    Some(mulm(subm(s, c1, p), invm(mulm(2, c2, p), p), p))
}

fn measure(eqs: &[Pol], n: usize, p: u64, budget: Duration) -> f4_fp::F4Report {
    let d = eqs.iter().map(|e| e.degree()).max().unwrap_or(1);
    let input: Vec<Poly> = eqs
        .iter()
        .map(|e| e.to_poly(p))
        .filter(|f| !f.is_empty())
        .collect();
    let opts = F4Options::new(Ordering::Grevlex, n as u32 + d + 6).with_budget(budget);
    f4_fp::f4(&input, n, p, &opts)
}

// ============================================ pricing study (2026-11-15) ====
//
// `research/pkm_priced_20261115/`: the per-target S5 build, counted
// (`cnt`), and a per-curve parametric alternative.  `S_{m+1}` has degree
// <= 2^{m-1} in `x_R`, so for m = 4 the polynomial `S_5(e; x_R)` is, per
// monomial in the `e_k`, a polynomial of degree <= 8 in `x_R`.  The
// parametric build computes it at 9 nodes `x_R = 0..8` ONCE per curve (the
// grid build, substitution and `e` rewrite included), interpolates each
// monomial coefficient in `x_R`, and then per target only evaluates those
// degree-8 coefficient polynomials (Horner).  Both builds must agree
// coefficient by coefficient; the check is exact, mod p.

const XR_NODES: usize = 9; // 2^{m-1} + 1 for m = 4

/// `S_5(e; x_R)` as `monomial -> coefficients in x_R` (constant term first).
struct ParamS5 {
    p: u64,
    terms: Vec<(Vec<u32>, [u64; XR_NODES])>,
}

impl ParamS5 {
    /// `node_builder(x_R)` must return the e-form `S_5` at that `x_R`
    /// (the grid build in production, a synthetic family in unit tests).
    fn build(p: u64, mut node_builder: impl FnMut(u64) -> Pol) -> ParamS5 {
        let nodes: Vec<u64> = (0..XR_NODES as u64).collect();
        let at_nodes: Vec<Pol> = nodes.iter().map(|&x| node_builder(x)).collect();
        let _g = cnt::Guard::enter(cnt::PARAM_INTERP);
        let inv = vandermonde_inverse(&nodes, p);
        let mut keys: Vec<Vec<u32>> = at_nodes
            .iter()
            .flat_map(|f| f.t.keys().cloned())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        keys.sort();
        let terms = keys
            .into_iter()
            .map(|e| {
                let mut c = [0u64; XR_NODES];
                for (j, cj) in c.iter_mut().enumerate() {
                    let mut acc = 0u64;
                    for (k, f) in at_nodes.iter().enumerate() {
                        let v = f.t.get(&e).copied().unwrap_or(0);
                        acc = addm(acc, mulm(inv[j][k], v, p), p);
                    }
                    *cj = acc;
                }
                (e, c)
            })
            .collect();
        ParamS5 { p, terms }
    }

    /// Per-target substitution: Horner in `x_R` for every monomial.
    fn eval(&self, x_r: u64) -> Pol {
        let _g = cnt::Guard::enter(cnt::PARAM_EVAL);
        let p = self.p;
        let n = self.terms.first().map(|(e, _)| e.len()).unwrap_or(0);
        let mut f = Pol::zero(n);
        for (e, c) in &self.terms {
            let mut acc = c[XR_NODES - 1];
            for j in (0..XR_NODES - 1).rev() {
                acc = addm(mulm(acc, x_r, p), c[j], p);
            }
            if acc != 0 {
                f.t.insert(e.clone(), acc);
            }
        }
        f
    }
}

/// Coefficient-by-coefficient equality of two polynomials (exact, mod p).
/// Returns `(number of coefficients compared, number of mismatches)` over the
/// union of supports (an absent term compares as 0).
fn compare_coeffs(a: &Pol, b: &Pol) -> (usize, usize) {
    let keys: std::collections::BTreeSet<&Vec<u32>> = a.t.keys().chain(b.t.keys()).collect();
    let bad = keys
        .iter()
        .filter(|e| a.t.get(**e).copied().unwrap_or(0) != b.t.get(**e).copied().unwrap_or(0))
        .count();
    (keys.len(), bad)
}

fn stage_json(d: &cnt::Snap) -> serde_json::Value {
    let mut m = serde_json::Map::new();
    for (s, name) in cnt::STAGES.iter().enumerate() {
        if d[s].iter().any(|&v| v != 0) {
            m.insert(
                name.to_string(),
                serde_json::json!({"mulmod": d[s][0] + d[s][3], "mul_outside_inv": d[s][0],
                    "addsub": d[s][1], "inv_calls": d[s][2], "mul_inside_inv": d[s][3]}),
            );
        }
    }
    serde_json::Value::Object(m)
}

/// Deterministic `x_R` of target `i`: independent of shard layout.
fn target_xr(seed: u64, t: usize, i: u64, p: u64) -> u64 {
    let mut r = Rng(seed ^ ((t as u64) << 48) ^ i.wrapping_mul(0xD1B5_4A32_D192_ED03));
    r.next();
    r.below(p)
}

/// The instrument's own instance for `(p, m, t)`: the same draws, in the same
/// order, as the legacy `main` (same prime, curve, `g`, `zeta` as the
/// committed rows).  Returns the generator state after the draws.
fn legacy_instance(p: u64, m: usize, t: usize) -> (Instance, Rng) {
    let mut rng = Rng(0x504B_4D53 ^ ((m as u64) << 32) ^ t as u64);
    assert!((p - 1).is_multiple_of(1u64 << t), "2^t must divide p - 1");
    let zeta = loop {
        let h = 2 + rng.below(p - 2);
        if powm(h, (p - 1) / 2, p) == p - 1 {
            break powm(h, (p - 1) >> t, p);
        }
    };
    let inst = Instance {
        p,
        a: rng.below(p),
        b: 1 + rng.below(p - 1),
        g: 2 + rng.below(p - 2),
        zeta,
        t,
    };
    (inst, rng)
}

/// A planted decomposition (distinct summands of `V`, chained `x_R`), or `None`.
fn find_planted(inst: &Instance, m: usize, rng: &mut Rng) -> Option<(Vec<u64>, Vec<u64>, u64)> {
    let (p, base) = (inst.p, inst.base());
    if base.len() < m {
        return None;
    }
    for _ in 0..200_000u32 {
        let mut idx: Vec<usize> = Vec::new();
        while idx.len() < m {
            let k = rng.below(base.len() as u64) as usize;
            if !idx.contains(&k) {
                idx.push(k);
            }
        }
        let xs: Vec<u64> = idx.iter().map(|&k| base[k]).collect();
        let w: Vec<u64> = idx.iter().map(|&k| powm(inst.zeta, k as u64, p)).collect();
        let mut chain = Vec::new();
        let mut acc = s3_root(xs[0], xs[1], inst.a, inst.b, p);
        for &x in &xs[2..] {
            let Some(u) = acc else { break };
            chain.push(u);
            acc = s3_root(u, x, inst.a, inst.b, p);
        }
        if let Some(xr) = acc {
            if chain.len() == m - 2 {
                return Some((w, chain, xr));
            }
        }
    }
    None
}

// ------------------------------------------------ the curve, its order ----

type Pt = Option<(u64, u64)>;

fn ec_add(p1: Pt, p2: Pt, a: u64, p: u64) -> Pt {
    let ((x1, y1), (x2, y2)) = match (p1, p2) {
        (None, q) | (q, None) => return q,
        (Some(u), Some(v)) => (u, v),
    };
    let lam = if x1 == x2 {
        if addm(y1, y2, p) == 0 {
            return None;
        }
        mulm(
            addm(mulm(3, mulm(x1, x1, p), p), a, p),
            invm(mulm(2, y1, p), p),
            p,
        )
    } else {
        mulm(subm(y2, y1, p), invm(subm(x2, x1, p), p), p)
    };
    let x3 = subm(subm(mulm(lam, lam, p), x1, p), x2, p);
    let y3 = subm(mulm(lam, subm(x1, x3, p), p), y1, p);
    Some((x3, y3))
}

fn ec_mul(mut k: u64, mut q: Pt, a: u64, p: u64) -> Pt {
    let mut r = None;
    while k > 0 {
        if k & 1 == 1 {
            r = ec_add(r, q, a, p);
        }
        q = ec_add(q, q, a, p);
        k >>= 1;
    }
    r
}

/// `#E(F_p)` by a square table (O(p)).
fn curve_order(a: u64, b: u64, p: u64) -> u64 {
    let mut roots = vec![0u8; p as usize];
    for y in 0..p {
        let v = mulm(y, y, p) as usize;
        roots[v] = roots[v].saturating_add(1);
    }
    let mut n = 1u64;
    for x in 0..p {
        let rhs = addm(addm(mulm(mulm(x, x, p), x, p), mulm(a, x, p), p), b, p);
        n += roots[rhs as usize] as u64;
    }
    n
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut f = Vec::new();
    let mut d = 2;
    while d * d <= n {
        while n.is_multiple_of(d) {
            f.push(d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        f.push(n);
    }
    f
}

// ------------------------------------------------------------- modes ----

fn argv(args: &[String], k: &str, d: &str) -> String {
    args.iter()
        .position(|a| a == k)
        .map(|i| args[i + 1].clone())
        .unwrap_or_else(|| d.to_string())
}

/// `--mode curve`: the curve's order, subgroup, a generator, the liftable part
/// of `V`, the derived relation yield, and (with `--spec-out`) the ecbench spec
/// of the matched rho-neg arm on the same curve.
fn run_curve(args: &[String]) {
    let p: u64 = argv(args, "--p", "786433").parse().unwrap();
    let t: usize = argv(args, "--t", "3").parse().unwrap();
    let (inst, _) = legacy_instance(p, 4, t);
    let n = curve_order(inst.a, inst.b, p);
    let fs = prime_factors(n);
    let r = *fs.iter().max().unwrap();
    let h = n / r;
    let mut gen = None;
    for x in 1..p {
        let rhs = addm(
            addm(mulm(mulm(x, x, p), x, p), mulm(inst.a, x, p), p),
            inst.b,
            p,
        );
        if let Some(y) = sqrtm(rhs, p) {
            if y == 0 {
                continue;
            }
            let g = ec_mul(h, Some((x, y)), inst.a, p);
            if g.is_some() && ec_mul(r, g, inst.a, p).is_none() {
                gen = g;
                break;
            }
        }
    }
    let (gx, gy) = gen.expect("generator of order r");
    let base = inst.base();
    let on_curve = |x: u64| {
        let rhs = addm(
            addm(mulm(mulm(x, x, p), x, p), mulm(inst.a, x, p), p),
            inst.b,
            p,
        );
        sqrtm(rhs, p).is_some()
    };
    let liftable = base.iter().filter(|&&x| on_curve(x)).count();
    let c4 = |l: usize| {
        if l < 4 {
            0.0
        } else {
            (l * (l - 1) * (l - 2) * (l - 3)) as f64 / 24.0
        }
    };
    // Signed sums: 16 sign patterns over an unordered 4-subset, +-R giving one
    // x value, so 8 x values per subset; a random point hits one with
    // probability (8 C(L,4)) / ((#E - 1)/2) = 16 C(L,4) / #E.
    let yield_derived = 16.0 * c4(liftable) / n as f64;
    let spec = serde_json::json!({
        "schema": "ecbench.spec/v1",
        "label": format!("pkm_priced_20261115 matched rho-neg, p={p} t={t}"),
        "description": "Matched negation-map rho on the pkm_symmetric_lead1 curve (PROTOCOL.md). Public synthetic curve and targets; no security claim.",
        "workloads": {"curves": [{"kind": "prime_explicit", "p": p, "a": inst.a, "b": inst.b,
            "group_order": n, "r": r, "gx": gx, "gy": gy}],
            "targets_per_curve": 200, "target_seed": 2026111531u64 + t as u64, "target_kind": "public"},
        "arms": [
            {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
            {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}],
        "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": 2026111541u64 + t as u64,
            "isolation_required": "L0", "timeout_seconds": 1800}
    });
    if let Some(i) = args.iter().position(|a| a == "--spec-out") {
        std::fs::write(&args[i + 1], serde_json::to_string_pretty(&spec).unwrap()).unwrap();
    }
    println!(
        "{}",
        serde_json::json!({"kind": "curve", "p": p, "t": t, "N": 4 * t, "a": inst.a, "b": inst.b, "g": inst.g,
            "zeta": inst.zeta, "order": n, "factors": fs, "r": r, "cofactor": h, "gx": gx, "gy": gy,
            "v_size": base.len(), "v_liftable_x": liftable, "unordered_4subsets_of_liftable": c4(liftable),
            "relations_needed_derived": liftable + 1, "yield_per_target_derived": yield_derived,
            "yield_lead_formula_8C(|V|,4)/r": 8.0 * c4(base.len()) / n as f64,
            "label": "derived; [pcc:unverified]"})
    );
}

/// `--mode calibrate`: best-of-N wall-clock cost of one mulmod relative to one
/// affine group addition (Fermat inversion) on the curve (AGENTS.md 5a `w_cal`).
/// Best-of-N timing, not an isolated host: labelled as such.
fn run_calibrate(args: &[String]) {
    use std::hint::black_box;
    use std::time::Instant;
    let p: u64 = argv(args, "--p", "786433").parse().unwrap();
    let t: usize = argv(args, "--t", "3").parse().unwrap();
    let (inst, _) = legacy_instance(p, 4, t);
    let (mut best_mul, mut best_add) = (f64::MAX, f64::MAX);
    for _ in 0..7 {
        let n = 20_000_000u64;
        let mut x = black_box(12345u64);
        let s = Instant::now();
        for _ in 0..n {
            x = mulm(x, black_box(x | 1), p);
        }
        black_box(x);
        best_mul = best_mul.min(s.elapsed().as_nanos() as f64 / n as f64);
        let (mut q, g) = {
            let mut x = 1;
            loop {
                let rhs = addm(
                    addm(mulm(mulm(x, x, p), x, p), mulm(inst.a, x, p), p),
                    inst.b,
                    p,
                );
                if let Some(y) = sqrtm(rhs, p) {
                    if y != 0 {
                        break (Some((x, y)), Some((x, y)));
                    }
                }
                x += 1;
            }
        };
        let n = 200_000u64;
        let s = Instant::now();
        for _ in 0..n {
            q = ec_add(black_box(q), g, inst.a, p);
        }
        black_box(q);
        best_add = best_add.min(s.elapsed().as_nanos() as f64 / n as f64);
    }
    println!(
        "{}",
        serde_json::json!({"kind": "calibration", "p": p, "t": t, "ns_per_mulmod": best_mul,
            "ns_per_group_add": best_add, "w_cal": best_mul / best_add,
            "method": "best of 7 loops; group add = affine add with Fermat inversion, this file's arithmetic (cnt-counted code path, not the production mulmod); not an isolated host",
            "label": "measured on the executing host; [pcc:unverified]"})
    );
}

/// `--mode stage0`: per target, the counted grid build, the counted parametric
/// evaluation, the exact coefficient comparison, and (first `--f4-targets`
/// targets) the F4 stage on the system built from the grid polynomial.
fn run_stage0(args: &[String]) {
    let p: u64 = argv(args, "--p", "786433").parse().unwrap();
    let t: usize = argv(args, "--t", "3").parse().unwrap();
    let targets: u64 = argv(args, "--targets", "24").parse().unwrap();
    let lo: u64 = argv(args, "--target-lo", "0").parse().unwrap();
    let f4_targets: u64 = argv(args, "--f4-targets", "8").parse().unwrap();
    let seed: u64 = argv(args, "--seed", "2026111501").parse().unwrap();
    let budget = Duration::from_secs(argv(args, "--budget", "600").parse().unwrap());
    let skip_param = argv(args, "--param", "1") == "0";
    let m = 4usize;
    let (inst, mut rng) = legacy_instance(p, m, t);
    let base = inst.base();
    let row = |v: serde_json::Value| println!("{v}");
    row(
        serde_json::json!({"kind": "config", "p": p, "m": m, "t": t, "N": m * t, "a": inst.a, "b": inst.b,
        "g": inst.g, "zeta": inst.zeta, "seed": seed, "targets": targets, "target_lo": lo,
        "f4_targets": f4_targets, "xr_nodes": XR_NODES, "counted_unit":
        "mulmod = kind0 + kind3 (every executed field multiplication, inversions' included); f4 = f4_fp field_ops (row multiply-subtracts)"}),
    );

    // Parametric precompute, once per curve, its own snapshot.
    let mut mismatches_total = 0usize;
    let mut compared_total = 0usize;
    let pre0 = cnt::snapshot();
    let t0 = std::time::Instant::now();
    let param = if skip_param {
        None
    } else {
        Some(ParamS5::build(p, |x| s_in_e_grid(&inst, m, x)))
    };
    let pre_ms = t0.elapsed().as_secs_f64() * 1e3;
    let pre = cnt::diff(&cnt::snapshot(), &pre0);
    if let Some(ps) = &param {
        row(
            serde_json::json!({"kind": "param_precompute", "xr_nodes": XR_NODES,
            "e_monomials": ps.terms.len(),
            "mulmod": cnt::mulmods(&pre, &(0..cnt::NST).collect::<Vec<_>>()),
            "addsub": cnt::addsubs(&pre, &(0..cnt::NST).collect::<Vec<_>>()),
            "grid_builds_inside": XR_NODES, "stages": stage_json(&pre), "ms": pre_ms}),
        );
    }

    // Tower assembly (x_R independent): counted once, outside both builds.
    let a0 = cnt::snapshot();
    let dummy = Pol::zero(m);
    let _ = symmetric_system_from(&inst, m, &dummy);
    let asm = cnt::diff(&cnt::snapshot(), &a0);
    row(serde_json::json!({"kind": "tower_assembly",
        "mulmod": cnt::mulmods(&asm, &[cnt::OTHER]), "addsub": cnt::addsubs(&asm, &[cnt::OTHER]),
        "note": "x_R-independent, per curve; reported, not added to either build"}));

    // Planted target: both builds must reproduce the planted decomposition.
    match find_planted(&inst, m, &mut rng) {
        None => row(
            serde_json::json!({"kind": "planted", "status": "skipped: no planted decomposition"}),
        ),
        Some((w, _partial, xr)) => {
            let _g = cnt::Guard::enter(cnt::CHECK);
            let se = s_in_e_grid(&inst, m, xr);
            let sol = symmetric_solution(&inst, m, &w);
            let (eqs, _) = symmetric_system_from(&inst, m, &se);
            let ok_grid = eqs.iter().all(|e| e.eval(&sol, p) == 0);
            let (ok_param, cmp) = match &param {
                Some(ps) => {
                    let sp = ps.eval(xr);
                    let (eqs, _) = symmetric_system_from(&inst, m, &sp);
                    (
                        eqs.iter().all(|e| e.eval(&sol, p) == 0),
                        compare_coeffs(&se, &sp),
                    )
                }
                None => (true, (0, 0)),
            };
            compared_total += cmp.0;
            mismatches_total += cmp.1;
            row(
                serde_json::json!({"kind": "planted", "x_r": xr, "grid_vanishes_at_planted": ok_grid,
                "param_vanishes_at_planted": ok_param, "coeffs_compared": cmp.0, "mismatches": cmp.1}),
            );
            assert!(ok_grid && ok_param, "planted check failed");
        }
    }

    // Targets.
    let mut excluded = 0u64;
    for i in lo..lo + targets {
        let xr = target_xr(seed, t, i, p);
        if base.contains(&xr) {
            excluded += 1;
            row(
                serde_json::json!({"kind": "excluded", "target": i, "x_r": xr, "reason": "x_R in V"}),
            );
            continue;
        }
        let s0 = cnt::snapshot();
        let tm = std::time::Instant::now();
        let se = s_in_e_grid(&inst, m, xr);
        let grid_ms = tm.elapsed().as_secs_f64() * 1e3;
        let g = cnt::diff(&cnt::snapshot(), &s0);
        let mut rec = serde_json::json!({"kind": "target", "target": i, "x_r": xr,
            "e_monomials": se.t.len(),
            "grid": {"mulmod": cnt::mulmods(&g, &cnt::GRID_BUILD), "addsub": cnt::addsubs(&g, &cnt::GRID_BUILD),
                "inv_calls": cnt::invs(&g, &cnt::GRID_BUILD), "ms": grid_ms, "stages": stage_json(&g)}});
        if let Some(ps) = &param {
            let s1 = cnt::snapshot();
            let tm = std::time::Instant::now();
            let sp = ps.eval(xr);
            let par_ms = tm.elapsed().as_secs_f64() * 1e3;
            let d = cnt::diff(&cnt::snapshot(), &s1);
            let (cmp, bad) = compare_coeffs(&se, &sp);
            compared_total += cmp;
            mismatches_total += bad;
            rec["param"] = serde_json::json!({"mulmod": cnt::mulmods(&d, &[cnt::PARAM_EVAL]),
                "addsub": cnt::addsubs(&d, &[cnt::PARAM_EVAL]), "ms": par_ms,
                "coeffs_compared": cmp, "mismatches": bad});
        }
        if i - lo < f4_targets {
            let (eqs, n) = symmetric_system_from(&inst, m, &se);
            let rep = measure(&eqs, n, p, budget);
            rec["f4"] = serde_json::json!({"field_ops": rep.field_ops, "D": rep.solving_degree_max,
                "last_productive": rep.solving_degree, "max_cols": rep.max_cols,
                "inconsistent": rep.inconsistent, "timed_out": rep.timed_out, "ms": rep.ms});
            if let Some(gm) = rec["grid"]["mulmod"].as_u64() {
                rec["build_over_f4"] = serde_json::json!(gm as f64 / rep.field_ops.max(1) as f64);
            }
        }
        row(rec);
    }
    row(
        serde_json::json!({"kind": "stage0_verdict", "coeffs_compared": compared_total,
        "mismatches": mismatches_total, "targets_run": targets - excluded, "excluded_x_r_in_V": excluded,
        "param_checked": !skip_param, "coefficient_check_passed": !skip_param && mismatches_total == 0}),
    );
    if mismatches_total != 0 {
        std::process::exit(3);
    }
}

fn read_jsonl(path: &str) -> Vec<serde_json::Value> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

/// `--mode summary --stage0 a.jsonl[,b.jsonl] --curve c.json --band band.json
///  --calib k.json [--amortise 100]`: the end-to-end priced S of each build
/// mode against the matched rho-neg, at w = 0, 0.1, 1, `w_cal`, 10 `w_cal`
/// and the leaderboard's 1/13 convention.  Mixes derived and measured inputs
/// and says which is which.
fn run_summary(args: &[String]) {
    let rows: Vec<serde_json::Value> = argv(args, "--stage0", "")
        .split(',')
        .filter(|s| !s.is_empty())
        .flat_map(read_jsonl)
        .collect();
    let curve = read_jsonl(&argv(args, "--curve", "curve.json"))
        .pop()
        .unwrap_or_default();
    let calib = read_jsonl(&argv(args, "--calib", "calib.json"))
        .pop()
        .unwrap_or_default();
    let band: serde_json::Value = std::fs::read_to_string(argv(args, "--band", "band.json"))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    let amort: f64 = argv(args, "--amortise", "100").parse().unwrap();
    let mean = |key: &dyn Fn(&serde_json::Value) -> Option<f64>| -> Option<f64> {
        let v: Vec<f64> = rows
            .iter()
            .filter(|r| r["kind"] == "target")
            .filter_map(key)
            .collect();
        (!v.is_empty()).then(|| v.iter().sum::<f64>() / v.len() as f64)
    };
    let grid = mean(&|r| r["grid"]["mulmod"].as_f64());
    let par = mean(&|r| r["param"]["mulmod"].as_f64());
    let f4 = mean(&|r| r["f4"]["field_ops"].as_f64());
    let inv_share = mean(&|r| {
        let st = r["grid"]["stages"].as_object()?;
        let inv: f64 = st
            .values()
            .filter_map(|v| v["mul_inside_inv"].as_f64())
            .sum();
        let all: f64 = st.values().filter_map(|v| v["mulmod"].as_f64()).sum();
        Some(inv / all)
    });
    let pre = rows
        .iter()
        .find(|r| r["kind"] == "param_precompute")
        .and_then(|r| r["mulmod"].as_f64());
    let (n_order, r_sub) = (
        curve["order"].as_f64().unwrap_or(f64::NAN),
        curve["r"].as_f64().unwrap_or(f64::NAN),
    );
    let y = curve["yield_per_target_derived"]
        .as_f64()
        .unwrap_or(f64::NAN);
    let need = curve["relations_needed_derived"]
        .as_f64()
        .unwrap_or(f64::NAN);
    let targets_per_solve = need / y;
    // rho-neg S(w) = s0 + w u is linear in w: fit from the band's weights.
    let ws: Vec<f64> = band["weights"]
        .as_array()
        .map(|a| a.iter().filter_map(|x| x.as_f64()).collect())
        .unwrap_or_default();
    let rho = band["rows"]
        .as_array()
        .and_then(|rs| rs.iter().find(|r| r["arm"] == "rho-neg"))
        .cloned()
        .unwrap_or_default();
    let ms: Vec<f64> = rho["mean_s"]
        .as_array()
        .map(|a| a.iter().filter_map(|x| x.as_f64()).collect())
        .unwrap_or_default();
    let (rho_s0, rho_u, rho_lin_resid) = if ws.len() == 3 && ms.len() == 3 {
        let u = (ms[2] - ms[0]) / (ws[2] - ws[0]);
        (
            ms[0],
            u,
            (ms[1] - (ms[0] + u * ws[1])).abs() / ms[1].abs().max(1e-300),
        )
    } else {
        (f64::NAN, f64::NAN, f64::NAN)
    };
    let w_cal = calib["w_cal"].as_f64().unwrap_or(f64::NAN);
    let weights = [
        ("w0", 0.0),
        ("w0.1", 0.1),
        ("w1", 1.0),
        ("w_cal", w_cal),
        ("10w_cal", 10.0 * w_cal),
        ("legacy_1/13", 1.0 / 13.0),
    ];
    let sqrt_r = r_sub.sqrt();
    let arm = |name: &str, per_target_ops: Option<f64>, setup_ops: f64| -> serde_json::Value {
        let mut out = serde_json::Map::new();
        for (label, w) in weights {
            let s_alg = per_target_ops.map(|c| (targets_per_solve * c + setup_ops) * w / sqrt_r);
            let s_rho = rho_s0 + rho_u * w;
            out.insert(label.into(), serde_json::json!({"w": w, "S": s_alg, "S_rho_neg": s_rho,
                "S_over_rho_neg": if w == 0.0 { None } else { s_alg.map(|s| s / s_rho) },
                "note": if w == 0.0 { "undefined at w = 0: the algebraic arm has no counted group work" } else { "" }}));
        }
        serde_json::json!({"arm": name, "per_target_ops": per_target_ops, "setup_ops": setup_ops, "by_weight": out})
    };
    let tot = |b: Option<f64>| match (b, f4) {
        (Some(b), Some(f)) => Some(b + f),
        _ => None,
    };
    let summary = serde_json::json!({
        "kind": "priced_summary",
        "inputs": {"mean_grid_build_mulmod": grid, "mean_param_eval_mulmod": par, "mean_f4_field_ops": f4,
            "param_precompute_mulmod": pre, "targets_per_solve_derived": targets_per_solve,
            "relations_needed_derived": need, "yield_per_target_derived": y, "order": n_order, "r": r_sub,
            "w_cal": w_cal, "rho_neg_S_fit": {"s0": rho_s0, "u_per_w": rho_u, "linearity_residual_at_w0.1": rho_lin_resid}},
        "ratios_measured": {"grid_inversion_mulmod_share": inv_share,
            "grid_build_over_f4": grid.zip(f4).map(|(a, b)| a / b),
            "param_eval_over_f4": par.zip(f4).map(|(a, b)| a / b),
            "grid_over_param_per_target": grid.zip(par).map(|(a, b)| a / b),
            "precompute_over_grid_build": pre.zip(grid).map(|(a, b)| a / b),
            "breakeven_targets": pre.zip(grid.zip(par)).map(|(s, (g, q))| s / (g - q)),
            "param_eval_plus_amortised_precompute_over_grid_at_horizon": pre.zip(grid.zip(par)).map(|(s, (g, q))| (q + s / amort) / g),
            "amortisation_horizon": amort},
        "arms": [
            arm("f4_only (as quoted, unpriced build)", f4, 0.0),
            arm("symmetric_grid_build (as built)", tot(grid), 0.0),
            arm("symmetric_parametric_build", tot(par), pre.unwrap_or(f64::NAN)),
        ],
        "not_priced_lower_bound": ["root recovery of relations", "linear algebra over the relation matrix",
            "verification [d]G = Q", "x_R lift / point decoding", "tower assembly (per curve, reported separately)"],
        "labels": "ratios_measured: measured; yield, relations_needed, targets_per_solve: derived; S and S/rho: derived from the two; all [pcc:unverified]",
    });
    println!("{summary}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| -> String {
        args.iter()
            .position(|a| a == k)
            .map(|i| args[i + 1].clone())
            .unwrap_or_else(|| d.to_string())
    };
    let p: u64 = get("--p", "786433").parse().unwrap();
    let ms: Vec<usize> = get("--m", "2,3")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let tmax2: usize = get("--t-max-m2", "9").parse().unwrap();
    let tmax3: usize = get("--t-max-m3", "5").parse().unwrap();
    let tmax4: usize = get("--t-max-m4", "4").parse().unwrap();
    let tmin: usize = get("--t-min", "1").parse().unwrap();
    let randoms: usize = get("--random", "2").parse().unwrap();
    let budget = Duration::from_secs(get("--budget", "600").parse().unwrap());
    let arms_filter = get("--arms", "plain-chain,plain-smp1,symmetric");
    match get("--mode", "legacy").as_str() {
        "legacy" => {}
        "stage0" => return run_stage0(&args),
        "curve" => return run_curve(&args),
        "calibrate" => return run_calibrate(&args),
        "summary" => return run_summary(&args),
        other => panic!("unknown --mode {other}"),
    }
    for &m in &ms {
        let tmax = match m {
            2 => tmax2,
            3 => tmax3,
            _ => tmax4,
        };
        for t in tmin..=tmax {
            let mut rng = Rng(0x504B_4D53 ^ ((m as u64) << 32) ^ t as u64);
            assert!((p - 1).is_multiple_of(1u64 << t), "2^t must divide p − 1");
            let zeta = loop {
                let h = 2 + rng.below(p - 2);
                if powm(h, (p - 1) / 2, p) == p - 1 {
                    break powm(h, (p - 1) >> t, p);
                }
            };
            let inst = Instance {
                p,
                a: rng.below(p),
                b: 1 + rng.below(p - 1),
                g: 2 + rng.below(p - 2),
                zeta,
                t,
            };
            let base = inst.base();
            if base.len() < m {
                continue;
            }
            // Planted check: distinct summands from V, x_R solving S_{m+1}.
            if base.len() < m {
                eprintln!(
                    "m={m} t={t}: |V| = {} < m, no distinct summands; cell skipped",
                    base.len()
                );
                continue;
            }
            let mut tries = 0u32;
            let planted = loop {
                tries += 1;
                if tries > 200_000 {
                    break None;
                }
                let mut idx: Vec<usize> = Vec::new();
                while idx.len() < m {
                    let k = rng.below(base.len() as u64) as usize;
                    if !idx.contains(&k) {
                        idx.push(k);
                    }
                }
                let xs: Vec<u64> = idx.iter().map(|&k| base[k]).collect();
                let w: Vec<u64> = idx.iter().map(|&k| powm(zeta, k as u64, p)).collect();
                // Partial sums u_1 = x(P1+P2), u_k = x(u_{k−1} + P_{k+1}); the last is x_R.
                let mut chain = Vec::new();
                let mut acc = s3_root(xs[0], xs[1], inst.a, inst.b, p);
                for &x in &xs[2..] {
                    let Some(u) = acc else { break };
                    chain.push(u);
                    acc = s3_root(u, x, inst.a, inst.b, p);
                }
                if let Some(xr) = acc {
                    if chain.len() == m - 2 {
                        break Some((w, chain, xr));
                    }
                }
            };
            let Some((w, partial, xr_pl)) = planted else {
                eprintln!("m={m} t={t}: no planted decomposition in 200000 draws; cell skipped");
                continue;
            };
            let arms: Vec<&str> = arms_filter
                .split(',')
                .filter(|a| !(m == 2 && *a == "plain-smp1"))
                .collect();
            for arm in &arms {
                let (eqs, n, sol) = match *arm {
                    "plain-chain" => {
                        let (e, n) = plain_system(&inst, m, xr_pl, true);
                        (e, n, plain_solution(&inst, &w, &partial))
                    }
                    "plain-smp1" => {
                        let (e, n) = plain_system(&inst, m, xr_pl, false);
                        (e, n, plain_solution(&inst, &w, &[]))
                    }
                    _ => {
                        let (e, n) = symmetric_system(&inst, m, xr_pl);
                        (e, n, symmetric_solution(&inst, m, &w))
                    }
                };
                for (k, e) in eqs.iter().enumerate() {
                    assert_eq!(
                        e.eval(&sol, p),
                        0,
                        "{arm} m={m} t={t}: equation {k} fails at the planted point"
                    );
                }
                let _ = n;
            }
            // Measured: random targets.
            for r in 0..randoms {
                let xr = rng.below(p);
                for arm in &arms {
                    let (eqs, n) = match *arm {
                        "plain-chain" => plain_system(&inst, m, xr, true),
                        "plain-smp1" => plain_system(&inst, m, xr, false),
                        _ => symmetric_system(&inst, m, xr),
                    };
                    let maxdeg = eqs.iter().map(|e| e.degree()).max().unwrap_or(0);
                    let rep = measure(&eqs, n, p, budget);
                    println!(
                        "{{\"p\":{p},\"m\":{m},\"t\":{t},\"N\":{},\"arm\":\"{arm}\",\"target\":{r},\"n_vars\":{n},\
                         \"n_eqs\":{},\"max_input_degree\":{maxdeg},\"D\":{},\"last_productive\":{},\"degree_reached\":{},\
                         \"inconsistent\":{},\"timed_out\":{},\"pairs_above_bound\":{},\"max_cols\":{},\"max_cols_to_solution\":{},\
                         \"steps\":{},\"field_ops\":{},\"ms\":{:.1}}}",
                        m * t,
                        eqs.len(),
                        rep.solving_degree_max,
                        rep.solving_degree,
                        rep.degree_reached,
                        rep.inconsistent,
                        rep.timed_out,
                        rep.pairs_above_bound,
                        rep.max_cols,
                        rep.max_cols_to_solution,
                        rep.steps,
                        rep.field_ops,
                        rep.ms
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: u64 = 12289; // 3 * 2^12 + 1

    #[test]
    fn counters_book_to_the_current_stage() {
        let s0 = cnt::snapshot();
        {
            let _g = cnt::Guard::enter(cnt::S3_EVAL);
            let _ = mulm(3, 5, P);
            let _ = addm(3, 5, P);
            let _ = subm(3, 5, P);
        }
        let _ = mulm(3, 5, P);
        let d = cnt::diff(&cnt::snapshot(), &s0);
        assert_eq!(d[cnt::S3_EVAL][0], 1);
        assert_eq!(d[cnt::S3_EVAL][1], 2);
        assert_eq!(d[cnt::OTHER][0], 1);
    }

    #[test]
    fn inversion_muls_are_booked_apart() {
        let s0 = cnt::snapshot();
        let r = invm(7, P);
        assert_eq!(mulm(r, 7, P), 1);
        let d = cnt::diff(&cnt::snapshot(), &s0);
        assert_eq!(d[cnt::OTHER][2], 1);
        assert!(
            d[cnt::OTHER][3] >= 14,
            "Fermat inversion is about 2 log p mulmods"
        );
        assert_eq!(
            d[cnt::OTHER][0],
            1,
            "only the explicit mulm is outside the inversion"
        );
    }

    #[test]
    fn counting_does_not_change_values() {
        let (inst, _) = legacy_instance(P, 4, 3);
        let a = s4_in_u(1, 2, 3, inst.a, inst.b, P);
        let _g = cnt::Guard::enter(cnt::S4_EVAL);
        assert_eq!(a, s4_in_u(1, 2, 3, inst.a, inst.b, P));
    }

    /// A synthetic e-form family of exact degree 8 in `x_R`: the parametric
    /// machinery must reproduce it at points that are not nodes.
    #[test]
    fn param_is_exact_on_a_degree_8_family() {
        let fam = |x: u64| {
            let mut f = Pol::zero(4);
            for (k, e) in [[1, 0, 0, 0], [0, 1, 0, 0], [2, 1, 0, 1], [0, 0, 0, 0]]
                .iter()
                .enumerate()
            {
                let mut c = 0;
                for j in 0..=8u64 {
                    c = addm(c, mulm(1000 + 37 * j + 11 * k as u64, powm(x, j, P), P), P);
                }
                f.t.insert(e.to_vec(), c);
            }
            f
        };
        let ps = ParamS5::build(P, fam);
        for x in [9u64, 100, 4321, P - 1] {
            let (n, bad) = compare_coeffs(&fam(x), &ps.eval(x));
            assert_eq!((n, bad), (4, 0), "x_R = {x}");
        }
    }

    #[test]
    fn param_detects_a_degree_9_family() {
        let fam = |x: u64| {
            let mut f = Pol::zero(1);
            f.t.insert(vec![1], powm(x, 9, P));
            f
        };
        let ps = ParamS5::build(P, fam);
        let (_, bad) = compare_coeffs(&fam(5000), &ps.eval(5000));
        assert!(bad > 0, "degree 9 in x_R must not be reproduced by 9 nodes");
    }

    #[test]
    fn compare_coeffs_reports_a_mismatch() {
        let mut a = Pol::zero(2);
        a.t.insert(vec![1, 0], 5);
        let mut b = a.clone();
        assert_eq!(compare_coeffs(&a, &b), (1, 0));
        b.t.insert(vec![0, 1], 3);
        assert_eq!(compare_coeffs(&a, &b), (2, 1));
    }

    #[test]
    fn curve_order_matches_group_law() {
        let (inst, _) = legacy_instance(P, 4, 3);
        let n = curve_order(inst.a, inst.b, P);
        let mut x = 1;
        let pt = loop {
            let rhs = addm(
                addm(mulm(mulm(x, x, P), x, P), mulm(inst.a, x, P), P),
                inst.b,
                P,
            );
            if let Some(y) = sqrtm(rhs, P) {
                if y != 0 {
                    break Some((x, y));
                }
            }
            x += 1;
        };
        assert!(ec_mul(n, pt, inst.a, P).is_none());
    }

    /// The real thing at the smallest toy size (slow in debug:
    /// `cargo test --release --example pkm_symmetric_tower -- --ignored`).
    #[test]
    #[ignore]
    fn parametric_s5_equals_grid_s5_on_toy_curve() {
        let (inst, _) = legacy_instance(P, 4, 3);
        let ps = ParamS5::build(P, |x| s_in_e_grid(&inst, 4, x));
        for i in 0..3 {
            let xr = target_xr(7, 3, i, P);
            let (_, bad) = compare_coeffs(&s_in_e_grid(&inst, 4, xr), &ps.eval(xr));
            assert_eq!(bad, 0, "target {i}");
        }
    }
}
