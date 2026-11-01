// Construction helpers derived from pkm_symmetric_complete.rs at PR2868
// dc7f22a94551dd550fa054fa807ec15cadf707fb, itself derived from PR2650
// adde64fdc22d51dc180fe0b9f30a2d28adf992fe. Historical drivers stay unchanged.
#![allow(dead_code)]
use crypto_lib::cryptanalysis::f4_fp::{self, Ordering, Poly};
use std::cell::Cell;
use std::collections::HashMap;

#[derive(Clone, Copy, Default, Debug, serde::Serialize, PartialEq, Eq)]
pub(super) struct Arithmetic {
    pub additions: u64,
    pub subtractions: u64,
    pub multiplications: u64,
    pub inverse_calls: u64,
    pub sqrt_calls: u64,
}
thread_local! { static COUNTS: Cell<Arithmetic> = const { Cell::new(Arithmetic { additions: 0, subtractions: 0, multiplications: 0, inverse_calls: 0, sqrt_calls: 0 }) }; }
pub(super) fn counts() -> Arithmetic {
    COUNTS.with(Cell::get)
}
fn tick(f: impl FnOnce(&mut Arithmetic)) {
    COUNTS.with(|c| {
        let mut v = c.get();
        f(&mut v);
        c.set(v);
    });
}
impl Arithmetic {
    pub fn since(self, old: Self) -> Self {
        Self {
            additions: self.additions - old.additions,
            subtractions: self.subtractions - old.subtractions,
            multiplications: self.multiplications - old.multiplications,
            inverse_calls: self.inverse_calls - old.inverse_calls,
            sqrt_calls: self.sqrt_calls - old.sqrt_calls,
        }
    }
}
pub(super) fn mulm(a: u64, b: u64, p: u64) -> u64 {
    tick(|c| c.multiplications += 1);
    ((a as u128 * b as u128) % p as u128) as u64
}
pub(super) fn addm(a: u64, b: u64, p: u64) -> u64 {
    tick(|c| c.additions += 1);
    (a + b) % p
}
pub(super) fn subm(a: u64, b: u64, p: u64) -> u64 {
    tick(|c| c.subtractions += 1);
    (a + p - b) % p
}
pub(super) fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
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
pub(super) fn invm(a: u64, p: u64) -> u64 {
    tick(|c| c.inverse_calls += 1);
    powm(a, p - 2, p)
}
/// Tonelli–Shanks; `None` for a non-residue.
pub(super) fn sqrtm(a: u64, p: u64) -> Option<u64> {
    tick(|c| c.sqrt_calls += 1);
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

#[derive(Clone, Debug)]
pub(super) struct Pol {
    pub n: usize,
    pub t: HashMap<Vec<u32>, u64>,
}

impl Pol {
    pub(super) fn zero(n: usize) -> Self {
        Pol {
            n,
            t: HashMap::new(),
        }
    }
    pub(super) fn c(n: usize, c: u64, p: u64) -> Self {
        let mut f = Self::zero(n);
        if !c.is_multiple_of(p) {
            f.t.insert(vec![0; n], c % p);
        }
        f
    }
    pub(super) fn var(n: usize, i: usize) -> Self {
        let mut e = vec![0; n];
        e[i] = 1;
        let mut f = Self::zero(n);
        f.t.insert(e, 1);
        f
    }
    pub(super) fn add(&self, o: &Pol, p: u64) -> Pol {
        let mut f = self.clone();
        for (e, &c) in &o.t {
            let v = f.t.entry(e.clone()).or_insert(0);
            *v = addm(*v, c, p);
        }
        f.t.retain(|_, c| *c != 0);
        f
    }
    pub(super) fn scale(&self, k: u64, p: u64) -> Pol {
        let mut f = Pol::zero(self.n);
        for (e, &c) in &self.t {
            let v = mulm(c, k % p, p);
            if v != 0 {
                f.t.insert(e.clone(), v);
            }
        }
        f
    }
    pub(super) fn sub(&self, o: &Pol, p: u64) -> Pol {
        self.add(&o.scale(p - 1, p), p)
    }
    pub(super) fn mul(&self, o: &Pol, p: u64) -> Pol {
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
    pub(super) fn eval(&self, x: &[u64], p: u64) -> u64 {
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
    pub(super) fn degree(&self) -> u32 {
        self.t.keys().map(|e| e.iter().sum()).max().unwrap_or(0)
    }
    /// Substitute variable `i` by the polynomial `g` (in the same ring).
    pub(super) fn subst(&self, i: usize, g: &Pol, p: u64) -> Pol {
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
    pub(super) fn remap(&self, n2: usize, map: &[usize]) -> Pol {
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
    pub(super) fn to_poly(&self, p: u64) -> Poly {
        let terms: Vec<(Vec<u32>, u64)> = self.t.iter().map(|(e, &c)| (e.clone(), c)).collect();
        f4_fp::normalise(&terms, p, Ordering::Grevlex)
    }
}

pub(super) fn s3(x1: &Pol, x2: &Pol, x3: &Pol, a: u64, b: u64, p: u64) -> Pol {
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
pub(super) fn summation_poly(m: usize, x_r: u64, a: u64, b: u64, p: u64) -> Pol {
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
pub(super) fn upoly_mul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = addm(c[i + j], mulm(x, y, p), p);
        }
    }
    c
}
pub(super) fn upoly_add(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut c = vec![0u64; a.len().max(b.len())];
    for (i, &x) in a.iter().enumerate() {
        c[i] = addm(c[i], x, p);
    }
    for (i, &x) in b.iter().enumerate() {
        c[i] = addm(c[i], x, p);
    }
    c
}
pub(super) fn upoly_scale(a: &[u64], k: u64, p: u64) -> Vec<u64> {
    a.iter().map(|&x| mulm(x, k, p)).collect()
}

/// `S_3(v, c, u)` as a quadratic in `v` whose coefficients are polynomials in
/// `u` (each of degree ≤ 2), constant term first.
pub(super) fn s3_in_v_over_u(c: u64, a: u64, b: u64, p: u64) -> [Vec<u64>; 3] {
    let f = s3(&Pol::var(2, 0), &Pol::c(2, c, p), &Pol::var(2, 1), a, b, p);
    let mut out = [vec![0u64; 3], vec![0u64; 3], vec![0u64; 3]];
    for (e, &v) in &f.t {
        out[e[0] as usize][e[1] as usize] = addm(out[e[0] as usize][e[1] as usize], v, p);
    }
    out
}

/// `S_3(u, c, d)` as a quadratic in `u`, constant term first.
pub(super) fn s3_in_u(c: u64, d: u64, a: u64, b: u64, p: u64) -> Vec<u64> {
    let f = s3(&Pol::var(1, 0), &Pol::c(1, c, p), &Pol::c(1, d, p), a, b, p);
    let mut out = vec![0u64; 3];
    for (e, &v) in &f.t {
        out[e[0] as usize] = addm(out[e[0] as usize], v, p);
    }
    out
}

/// `S_4(x1, x2, x3, u)` as a polynomial in `u` (formal degree 4):
/// `Res_v(S_3(x1, x2, v), S_3(v, x3, u))`.
pub(super) fn s4_in_u(x1: u64, x2: u64, x3: u64, a: u64, b: u64, p: u64) -> Vec<u64> {
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
pub(super) fn det_mod(mut m: Vec<Vec<u64>>, p: u64) -> u64 {
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
pub(super) fn sylvester(f: &[u64], df: usize, g: &[u64], dg: usize, p: u64) -> u64 {
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
pub(super) fn summation_poly_5(x_r: u64, a: u64, b: u64, p: u64) -> Pol {
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
pub(super) fn to_elementary(f: &Pol, m: usize, p: u64) -> Pol {
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

pub(super) struct Instance {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub g: u64,
    pub zeta: u64,
    pub t: usize,
}

impl Instance {
    pub(super) fn base(&self) -> Vec<u64> {
        (0..1u64 << self.t)
            .map(|k| mulm(self.g, powm(self.zeta, k, self.p), self.p))
            .collect()
    }
}

/// Plain towers: variables `y_{i,j}` at `i·t + j`, then the chain's
/// partial sums (`m ≥ 3`, chain arm only).
pub(super) fn plain_system(inst: &Instance, m: usize, x_r: u64, chain: bool) -> (Vec<Pol>, usize) {
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
pub(super) fn symmetric_system(inst: &Instance, m: usize, x_r: u64) -> (Vec<Pol>, usize) {
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

pub(super) fn validate_parameters(inst: &Instance, m: usize) -> Result<(), String> {
    if !(2..=4).contains(&m)
        || inst.t == 0
        || inst.t > 8
        || inst.p <= 11
        || inst.p >= 1 << 32
        || inst.a >= inst.p
        || inst.b >= inst.p
        || !crypto_lib::cryptanalysis::residual_walk::is_prime_u64(inst.p)
        || inst.g == 0
        || inst.g >= inst.p
        || inst.zeta >= inst.p
        || powm(inst.zeta, 1 << inst.t, inst.p) != 1
        || powm(inst.zeta, 1 << (inst.t - 1), inst.p) == 1
    {
        return Err("invalid bounded prime/tower parameters".into());
    }
    let p = u128::from(inst.p);
    if (4 * u128::from(inst.a).pow(3) + 27 * u128::from(inst.b).pow(2)) % p == 0 {
        return Err("singular curve".into());
    }
    Ok(())
}
