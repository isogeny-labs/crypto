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

// ------------------------------------------------------------ field ----

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn addm(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
fn subm(a: u64, b: u64, p: u64) -> u64 {
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
fn invm(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
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
                let s4 = s4_in_u(nodes[i1], nodes[i2], nodes[i3], a, b, p);
                for i4 in 0..K {
                    let g = s3_in_u(nodes[i4], x_r, a, b, p);
                    vals[((i1 * K + i2) * K + i3) * K + i4] = sylvester(&s4, 4, &g, 2, p);
                }
            }
        }
    }
    // Inverse Vandermonde on the nodes: coefficient vector = V^{-1} · values.
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
    // S_{m+1}(g w_1..g w_m, x_R) in e_k(w): substitute x_i = g w_i first.
    let s = summation_poly(m, x_r, inst.a, inst.b, p);
    let mut sw = s.clone();
    for i in 0..m {
        sw = sw.subst(i, &Pol::var(m, i).scale(inst.g, p), p);
    }
    let se = to_elementary(&sw, m, p);
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
