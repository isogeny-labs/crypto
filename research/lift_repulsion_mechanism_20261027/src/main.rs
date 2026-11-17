//! Mechanism of lift-set repulsion (study `lift_repulsion_mechanism_20261027`).
//!
//! Parent study: `research/lift_dense_factor_base_20261027/src/main.rs`.
//! Copied from it (same semantics, same seed constant, same population
//! generators): `Fq` (log/exp field arithmetic, trace), `Rng`, `Bits`,
//! `lift_set`, `members`, `random_basis`, `rank`, the count-mode subspace DFS
//! (`Search`, here depth-capped at 4 with the last level counted without
//! building child lists), and `expect` (from its `summarize` crate).
//!
//! For each set L (curve lift set or null) in F_q^*, q = 2^N, u = q - 1:
//!   psi(x) = +1 (x in L), -1 (x != 0 not in L), psi(0) = 0,
//!   W = Walsh transform of psi (exact integers),
//!   exact all-lift subspace counts A_2, A_3, A_4,
//!   centred correlations D_line, D_plane of e = 1_L - p (p = |L|/u),
//!   the first-order prediction of A_k - E_k from D_line, D_plane.
//! Std only. `cargo run --release -- run <outdir> [--quick] [--threads T]`.

use std::fmt::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

const SEED: u64 = 0x11F7_2026_1027_C;

// ---------------------------------------------------------------- F_q ----

struct Fq {
    n: u32,
    q1: usize,
    exp: Vec<u32>,
    log: Vec<u32>,
    tr_mask: u32,
}

fn clmul_reduce(a: u32, b: u32, n: u32, poly: u32) -> u32 {
    let mut r: u64 = 0;
    let (a, mut b, mut i) = (a as u64, b, 0);
    while b != 0 {
        if b & 1 == 1 {
            r ^= a << i;
        }
        b >>= 1;
        i += 1;
    }
    for k in (n..2 * n).rev() {
        if (r >> k) & 1 == 1 {
            r ^= (poly as u64) << (k - n);
        }
    }
    r as u32
}

impl Fq {
    fn new(n: u32, poly: u32) -> Fq {
        assert!((poly >> n) == 1 && n <= 24);
        let q1 = (1usize << n) - 1;
        let mut exp = vec![0u32; 2 * q1];
        let mut log = vec![0u32; q1 + 1];
        let mut found = false;
        'search: for g in 2u32..64 {
            let mut e = 1u32;
            for i in 0..q1 {
                if i > 0 && e == 1 {
                    continue 'search;
                }
                exp[i] = e;
                log[e as usize] = i as u32;
                e = clmul_reduce(e, g, n, poly);
            }
            if e != 1 {
                continue;
            }
            found = true;
            break;
        }
        assert!(found, "no generator: poly {poly:#x} not irreducible for n = {n}");
        for i in 0..q1 {
            exp[q1 + i] = exp[i];
        }
        let mut f = Fq { n, q1, exp, log, tr_mask: 0 };
        for i in 0..n {
            let e = 1u32 << i;
            let (mut t, mut z) = (0u32, e);
            for _ in 0..n {
                t ^= z;
                z = f.sq(z);
            }
            assert!(t <= 1);
            f.tr_mask |= t << i;
        }
        f
    }
    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            return 0;
        }
        self.exp[(self.log[a as usize] + self.log[b as usize]) as usize]
    }
    #[inline]
    fn sq(&self, a: u32) -> u32 {
        self.mul(a, a)
    }
    #[inline]
    fn inv(&self, a: u32) -> u32 {
        assert!(a != 0);
        self.exp[(self.q1 - self.log[a as usize] as usize) % self.q1]
    }
    #[inline]
    fn tr(&self, a: u32) -> u32 {
        (a & self.tr_mask).count_ones() & 1
    }
}

const FIELDS: &[(u32, u32)] = &[
    (9, (1 << 9) | (1 << 4) | 1),
    (11, (1 << 11) | (1 << 2) | 1),
    (12, (1 << 12) | (1 << 3) | 1),
    (13, (1 << 13) | (1 << 4) | (1 << 3) | (1 << 1) | 1),
];

// ---------------------------------------------------------------- rng ----

#[derive(Clone)]
struct Rng(u64);
impl Rng {
    fn new(tag: &str, n: u32) -> Rng {
        let mut h: u64 = 0xcbf29ce484222325 ^ SEED ^ ((n as u64) << 40);
        for b in tag.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        let mut r = Rng(h);
        r.next();
        r
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: u64) -> u64 {
        self.next() % m
    }
}

// ------------------------------------------------------------ bitsets ----

#[derive(Clone)]
struct Bits(Vec<u64>);
impl Bits {
    fn new(n: u32) -> Bits {
        Bits(vec![0; ((1usize << n) + 63) / 64])
    }
    #[inline]
    fn get(&self, x: u32) -> bool {
        (self.0[(x >> 6) as usize] >> (x & 63)) & 1 == 1
    }
    #[inline]
    fn set(&mut self, x: u32) {
        self.0[(x >> 6) as usize] |= 1 << (x & 63);
    }
    #[inline]
    fn clear(&mut self, x: u32) {
        self.0[(x >> 6) as usize] &= !(1 << (x & 63));
    }
}

fn lift_set(f: &Fq, a: u32, b: u32) -> Bits {
    let mut s = Bits::new(f.n);
    for x in 1..=(f.q1 as u32) {
        let w = x ^ a ^ f.mul(b, f.inv(f.sq(x)));
        if f.tr(w) == 0 {
            s.set(x);
        }
    }
    s
}

fn members(n: u32, s: &Bits) -> Vec<u32> {
    (1..(1u32 << n)).filter(|&x| s.get(x)).collect()
}

fn rank(vs: &[u32]) -> usize {
    let mut piv: Vec<u32> = vec![];
    for &v in vs {
        let mut x = v;
        for &p in &piv {
            x = x.min(x ^ p);
        }
        if x != 0 {
            piv.push(x);
            piv.sort_unstable_by(|a, b| b.cmp(a));
        }
    }
    piv.len()
}

fn random_basis(n: u32, l: usize, rng: &mut Rng) -> Vec<u32> {
    loop {
        let b: Vec<u32> = (0..l).map(|_| 1 + rng.below((1u64 << n) - 1) as u32).collect();
        if rank(&b) == l {
            return b;
        }
    }
}

// ------------------------------------------------ count-mode subspace DFS ----

/// Exact counts of k-dim subspaces V with V \ {0} inside the set, k <= cap.
/// Same reduced-echelon enumeration as the parent study's `Search::dfs` in
/// count mode; the last level only counts valid extensions.
struct Search {
    levels: Vec<Bits>,
    cap: usize,
    cur: usize,
    counts: Vec<u64>,
}

impl Search {
    fn new(n: u32, cap: usize) -> Search {
        Search {
            levels: (0..=cap).map(|_| Bits::new(n)).collect(),
            cap,
            cur: 0,
            counts: vec![0; cap + 1],
        }
    }
    fn child(&mut self, cand: &[u32], lv: usize, v: u32) -> Vec<u32> {
        let mut out = Vec::new();
        for &w in cand {
            if self.levels[lv].get(w ^ v) {
                out.push(w);
            }
        }
        for &w in &out {
            self.levels[lv + 1].set(w);
        }
        out
    }
    fn release(&mut self, c: &[u32], lv: usize) {
        for &w in c {
            self.levels[lv].clear(w);
        }
    }
    fn dfs(&mut self, cand: &[u32], lv: usize, last: u32, pivmask: u32) {
        self.counts[self.cur] += 1;
        if self.cur + 2 == self.cap {
            // Two levels from the cap: for each valid v count the valid
            // extensions without building the child list (cand is sorted).
            for i in 0..cand.len() {
                let v = cand[i];
                if v <= last || v & pivmask != 0 {
                    continue;
                }
                self.counts[self.cur + 1] += 1;
                let np = pivmask | (1u32 << (31 - v.leading_zeros()));
                let mut c = 0u64;
                for &w in &cand[i + 1..] {
                    if w & np == 0 && self.levels[lv].get(w ^ v) {
                        c += 1;
                    }
                }
                self.counts[self.cap] += c;
            }
            return;
        }
        if self.cur + 1 == self.cap {
            let mut c = 0u64;
            for &v in cand {
                if v > last && v & pivmask == 0 {
                    c += 1;
                }
            }
            self.counts[self.cap] += c;
            return;
        }
        for i in 0..cand.len() {
            let v = cand[i];
            if v <= last || v & pivmask != 0 {
                continue;
            }
            let c = self.child(cand, lv, v);
            self.cur += 1;
            let top = 1u32 << (31 - v.leading_zeros());
            self.dfs(&c, lv + 1, v, pivmask | top);
            self.cur -= 1;
            self.release(&c, lv + 1);
        }
    }
}

fn count_subspaces(n: u32, all: &[u32], cap: usize) -> Vec<u64> {
    let mut sc = Search::new(n, cap);
    for &w in all {
        sc.levels[0].set(w);
    }
    sc.dfs(all, 0, 0, 0);
    sc.counts
}

// ----------------------------------------------------------- expectations ----

/// Parent study (`summarize`): expected number of k-dim subspaces inside a
/// uniform random `size`-subset of F_q^*.
fn expect(n: u32, k: u32, size: u64) -> f64 {
    let mut g = 1.0f64;
    for i in 0..k {
        g *= (2f64.powi((n - i) as i32) - 1.0) / (2f64.powi((k - i) as i32) - 1.0);
    }
    let tot = 2f64.powi(n as i32) - 1.0;
    for i in 0..(1u64 << k) - 1 {
        g *= (size as f64 - i as f64).max(0.0) / (tot - i as f64);
    }
    g
}

fn gauss(n: i32, k: i32) -> f64 {
    if k < 0 || k > n {
        return 0.0;
    }
    let mut g = 1.0f64;
    for i in 0..k {
        g *= (2f64.powi(n - i) - 1.0) / (2f64.powi(k - i) - 1.0);
    }
    g
}

/// E[prod_{x in T} (1_L(x) - p)] for |T| = t, L a uniform m-subset of u points.
fn eps(t: u32, m: f64, u: f64, p: f64) -> f64 {
    let mut tot = 0.0;
    let mut binom = 1.0f64; // C(t, j)
    let mut ffr = 1.0f64; // (m)_j / (u)_j
    for j in 0..=t {
        tot += binom * (-p).powi((t - j) as i32) * ffr;
        // advance j -> j+1
        binom = binom * (t - j) as f64 / (j + 1) as f64;
        ffr *= (m - j as f64) / (u - j as f64);
    }
    tot
}

// ----------------------------------------------------------- Walsh --------

fn wht_i64(v: &mut [i64]) {
    let n = v.len();
    let mut h = 1;
    while h < n {
        for i in (0..n).step_by(2 * h) {
            for j in i..i + h {
                let (a, b) = (v[j], v[j + h]);
                v[j] = a + b;
                v[j + h] = a - b;
            }
        }
        h *= 2;
    }
}

fn wht_f64(v: &mut [f64]) {
    let n = v.len();
    let mut h = 1;
    while h < n {
        for i in (0..n).step_by(2 * h) {
            for j in i..i + h {
                let (a, b) = (v[j], v[j + h]);
                v[j] = a + b;
                v[j + h] = a - b;
            }
        }
        h *= 2;
    }
}

// ----------------------------------------------------------- JSON ---------

enum J {
    N(f64),
    I(i128),
    S(String),
    B(bool),
    A(Vec<J>),
    O(Vec<(String, J)>),
    Null,
}

fn o(v: Vec<(&str, J)>) -> J {
    J::O(v.into_iter().map(|(k, x)| (k.to_string(), x)).collect())
}
fn s(x: &str) -> J {
    J::S(x.to_string())
}
fn fa(v: &[f64]) -> J {
    J::A(v.iter().map(|&x| J::N(x)).collect())
}

impl J {
    fn w(&self, out: &mut String) {
        match self {
            J::N(x) => {
                if x.is_finite() {
                    let _ = write!(out, "{:e}", x);
                } else {
                    out.push_str("null");
                }
            }
            J::I(x) => {
                let _ = write!(out, "{x}");
            }
            J::S(x) => {
                out.push('"');
                for c in x.chars() {
                    match c {
                        '"' => out.push_str("\\\""),
                        '\\' => out.push_str("\\\\"),
                        '\n' => out.push_str("\\n"),
                        c => out.push(c),
                    }
                }
                out.push('"');
            }
            J::B(x) => out.push_str(if *x { "true" } else { "false" }),
            J::Null => out.push_str("null"),
            J::A(v) => {
                out.push('[');
                for (i, x) in v.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    x.w(out);
                }
                out.push(']');
            }
            J::O(v) => {
                out.push('{');
                for (i, (k, x)) in v.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    let _ = write!(out, "\"{k}\":");
                    x.w(out);
                }
                out.push('}');
            }
        }
    }
    fn to_s(&self) -> String {
        let mut s = String::new();
        self.w(&mut s);
        s
    }
}

// ----------------------------------------------------------- statistics ----

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn sd(v: &[f64]) -> f64 {
    if v.len() < 2 {
        return f64::NAN;
    }
    let m = mean(v);
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() - 1) as f64).sqrt()
}
fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
    let mut r = vec![0.0; v.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
            j += 1;
        }
        let avg = (i + j) as f64 / 2.0 + 1.0;
        for t in i..=j {
            r[idx[t]] = avg;
        }
        i = j + 1;
    }
    r
}
fn pearson(x: &[f64], y: &[f64]) -> f64 {
    let (mx, my) = (mean(x), mean(y));
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for i in 0..x.len() {
        sxy += (x[i] - mx) * (y[i] - my);
        sxx += (x[i] - mx) * (x[i] - mx);
        syy += (y[i] - my) * (y[i] - my);
    }
    if sxx == 0.0 || syy == 0.0 {
        return f64::NAN;
    }
    sxy / (sxx * syy).sqrt()
}
fn spearman(x: &[f64], y: &[f64]) -> f64 {
    pearson(&ranks(x), &ranks(y))
}
/// One-sided permutation p for H1: rho > 0.
fn spearman_perm_p(x: &[f64], y: &[f64], b: usize, rng: &mut Rng) -> f64 {
    let obs = spearman(x, y);
    if !obs.is_finite() {
        return f64::NAN;
    }
    let rx = ranks(x);
    let mut ry = ranks(y);
    let mut ge = 0usize;
    for _ in 0..b {
        for i in (1..ry.len()).rev() {
            let j = rng.below(i as u64 + 1) as usize;
            ry.swap(i, j);
        }
        if pearson(&rx, &ry) >= obs - 1e-12 {
            ge += 1;
        }
    }
    (1 + ge) as f64 / (1 + b) as f64
}
/// Fisher-z 95% interval for a correlation r from n points (Spearman: SE inflated by 1.06).
fn fisher_ci(r: f64, n: usize, spear: bool) -> (f64, f64) {
    if !r.is_finite() || n < 4 {
        return (f64::NAN, f64::NAN);
    }
    let z = r.clamp(-0.999999, 0.999999).atanh();
    let se = if spear { 1.06 } else { 1.0 } / ((n - 3) as f64).sqrt();
    ((z - 1.96 * se).tanh(), (z + 1.96 * se).tanh())
}
/// One-sided permutation p for H1: mean(a) > mean(b).
fn mean_diff_perm_p(a: &[f64], b: &[f64], bn: usize, rng: &mut Rng) -> f64 {
    let obs = mean(a) - mean(b);
    let mut pool: Vec<f64> = a.iter().chain(b.iter()).copied().collect();
    let (na, tot) = (a.len(), a.len() + b.len());
    let mut ge = 0usize;
    for _ in 0..bn {
        for i in (1..tot).rev() {
            let j = rng.below(i as u64 + 1) as usize;
            pool.swap(i, j);
        }
        let d = mean(&pool[..na]) - mean(&pool[na..]);
        if d >= obs - 1e-12 {
            ge += 1;
        }
    }
    (1 + ge) as f64 / (1 + bn) as f64
}
fn boot_diff_ci(a: &[f64], b: &[f64], bn: usize, rng: &mut Rng) -> (f64, f64) {
    let mut ds = Vec::with_capacity(bn);
    for _ in 0..bn {
        let ma = (0..a.len()).map(|_| a[rng.below(a.len() as u64) as usize]).sum::<f64>() / a.len() as f64;
        let mb = (0..b.len()).map(|_| b[rng.below(b.len() as u64) as usize]).sum::<f64>() / b.len() as f64;
        ds.push(ma - mb);
    }
    ds.sort_by(|x, y| x.partial_cmp(y).unwrap());
    (ds[(bn as f64 * 0.025) as usize], ds[(bn as f64 * 0.975) as usize])
}
fn boot_mean_ci(a: &[f64], bn: usize, rng: &mut Rng) -> (f64, f64) {
    let mut ds = Vec::with_capacity(bn);
    for _ in 0..bn {
        ds.push((0..a.len()).map(|_| a[rng.below(a.len() as u64) as usize]).sum::<f64>() / a.len() as f64);
    }
    ds.sort_by(|x, y| x.partial_cmp(y).unwrap());
    (ds[(bn as f64 * 0.025) as usize], ds[(bn as f64 * 0.975) as usize])
}
fn cohen_d(a: &[f64], b: &[f64]) -> f64 {
    let (na, nb) = (a.len() as f64, b.len() as f64);
    let sp = (((na - 1.0) * sd(a).powi(2) + (nb - 1.0) * sd(b).powi(2)) / (na + nb - 2.0)).sqrt();
    (mean(a) - mean(b)) / sp
}
fn cliff_delta(a: &[f64], b: &[f64]) -> f64 {
    let mut c = 0.0;
    for x in a {
        for y in b {
            if x > y {
                c += 1.0;
            } else if x < y {
                c -= 1.0;
            }
        }
    }
    c / (a.len() * b.len()) as f64
}
/// OLS y = a + b x -> (slope, intercept, r2)
fn ols(x: &[f64], y: &[f64]) -> (f64, f64, f64) {
    let (mx, my) = (mean(x), mean(y));
    let (mut sxy, mut sxx) = (0.0, 0.0);
    for i in 0..x.len() {
        sxy += (x[i] - mx) * (y[i] - my);
        sxx += (x[i] - mx) * (x[i] - mx);
    }
    let b = sxy / sxx;
    let r = pearson(x, y);
    (b, my - b * mx, r * r)
}

// ----------------------------------------------------------- instances ----

#[derive(Clone)]
struct Inst {
    arm: String,
    pop: &'static str,
    label: String,
    a: u32,
    b: u32,
    s: Bits,
    planted_l: u32,
}

fn make_null(n: u32, q1: u32, tag: &str, size: usize) -> Bits {
    let mut r = Rng::new(tag, n);
    let mut perm: Vec<u32> = (1..=q1).collect();
    for i in (1..perm.len()).rev() {
        let j = r.below(i as u64 + 1) as usize;
        perm.swap(i, j);
    }
    let mut s = Bits::new(n);
    for &x in &perm[..size] {
        s.set(x);
    }
    s
}

/// Union of cosets of a random ell-dim subspace H (H itself excluded):
/// the positive control with a planted large fourth moment.
fn coset_union(n: u32, q1: u32, target: usize, ell: u32, tag: &str) -> Bits {
    let mut r = Rng::new(tag, n);
    let basis = random_basis(n, ell as usize, &mut r);
    let mut h = vec![0u32];
    for &b in &basis {
        let l = h.len();
        for i in 0..l {
            h.push(h[i] ^ b);
        }
    }
    let mut perm: Vec<u32> = (1..=q1).collect();
    for i in (1..perm.len()).rev() {
        let j = r.below(i as u64 + 1) as usize;
        perm.swap(i, j);
    }
    let mut taken = Bits::new(n);
    for &x in &h {
        taken.set(x);
    }
    let mut s = Bits::new(n);
    let mut size = 0usize;
    for &x in &perm {
        if size + (1usize << ell) > target {
            break;
        }
        if taken.get(x) {
            continue;
        }
        for &y in &h {
            taken.set(x ^ y);
            s.set(x ^ y);
        }
        size += 1usize << ell;
    }
    s
}

fn build_population(f: &Fq, n: u32, quick: bool, with_extra: bool) -> Vec<Inst> {
    let q1 = f.q1 as u32;
    let mut out: Vec<Inst> = vec![];
    // ---- orig: identical generator to the parent study (same seed, tags) ----
    let mut rng = Rng::new("curves", n);
    let mut curves: Vec<Inst> = vec![];
    let mk = |arm: &str, pop: &'static str, label: String, a: u32, b: u32, f: &Fq| Inst {
        arm: arm.into(),
        pop,
        label,
        a,
        b,
        s: lift_set(f, a, b),
        planted_l: 0,
    };
    curves.push(mk("curve", "orig", "koblitz_a0_b1".into(), 0, 1, f));
    curves.push(mk("curve", "orig", "koblitz_a1_b1".into(), 1, 1, f));
    let extra = if n <= 11 && !quick { 24 } else { 4 };
    for i in 0..extra {
        let a = (i % 2) as u32;
        let b = 2 + rng.below(f.q1 as u64 - 1) as u32;
        curves.push(mk("curve", "orig", format!("random_{i}"), a, b, f));
    }
    // ---- rep: fresh random curves, new stream ----
    let mut rng2 = Rng::new("curves_rep", n);
    let mut reps: Vec<Inst> = vec![];
    {
        for i in 0..extra {
            let a = (i % 2) as u32;
            let b = 2 + rng2.below(f.q1 as u64 - 1) as u32;
            reps.push(mk("curve_r", "rep", format!("rand_rep_{i}"), a, b, f));
        }
    }
    // interleaved: curve_i, paired null_i
    for (i, c) in curves.iter().enumerate() {
        let size = members(n, &c.s).len();
        out.push(c.clone());
        out.push(Inst {
            arm: "null".into(),
            pop: "orig",
            label: format!("null_{i}"),
            a: 0,
            b: 0,
            s: make_null(n, q1, &format!("null_{i}"), size),
            planted_l: 0,
        });
    }
    for (i, c) in reps.iter().enumerate() {
        let size = members(n, &c.s).len();
        out.push(c.clone());
        out.push(Inst {
            arm: "null_r".into(),
            pop: "rep",
            label: format!("nullr_{i}"),
            a: 0,
            b: 0,
            s: make_null(n, q1, &format!("nullr_{i}"), size),
            planted_l: 0,
        });
    }
    let all_curves: Vec<&Inst> = curves.iter().chain(reps.iter()).collect();
    if with_extra {
        let nx = if n <= 11 { 3 * all_curves.len() } else { 4 * all_curves.len() };
        for j in 0..nx {
            let size = members(n, &all_curves[j % all_curves.len()].s).len();
            out.push(Inst {
                arm: "null_x".into(),
                pop: "extra",
                label: format!("nullx_{j}"),
                a: 0,
                b: 0,
                s: make_null(n, q1, &format!("nullx_{j}"), size),
                planted_l: 0,
            });
        }
        // planted positive control (gated): a rank-one Walsh bias. L = {x != 0 :
        // l(x) = 0} for a random functional l, each membership flipped with
        // probability eta; density stays 1/2, the Walsh spike is (1 - 2 eta) q.
        for (tag, eta_pm) in [("e46", 460u64), ("e42", 420), ("e38", 380), ("e34", 340), ("e30", 300)] {
            for j in 0..4 {
                let lab = format!("bias_{tag}_{j}");
                let mut r = Rng::new(&lab, n);
                let w0 = random_basis(n, 1, &mut r)[0];
                let mut s = Bits::new(n);
                for x in 1..=q1 {
                    let base = (x & w0).count_ones() & 1 == 0;
                    let flip = r.below(1000) < eta_pm;
                    if base != flip {
                        s.set(x);
                    }
                }
                out.push(Inst { arm: format!("bias_{tag}"), pop: "planted", label: lab, a: 0, b: 0, s, planted_l: 0 });
            }
        }
        // exploratory: unions of cosets (not gated), density of the first Koblitz curve
        let base = members(n, &curves[0].s).len();
        for ell in 1..=3u32 {
            for j in 0..4 {
                let tag = format!("cos{ell}_{j}");
                out.push(Inst {
                    arm: format!("planted_l{ell}"),
                    pop: "planted",
                    label: tag.clone(),
                    a: 0,
                    b: 0,
                    s: coset_union(n, q1, base, ell, &tag),
                    planted_l: ell,
                });
            }
        }
    }
    out
}

// ----------------------------------------------------------- per-set row ----

struct Row {
    n: u32,
    arm: String,
    pop: &'static str,
    label: String,
    a: u32,
    b: u32,
    planted_l: u32,
    m: u64,
    counts: Vec<u64>,
    sw1: i64,
    sw2: i128,
    sw3: i128,
    sw4: i128,
    kurt: f64,
    w4_q3: f64,
    c3: i128,
    a2_exact: i128,
    a2_exact_ok: bool,
    w4_curve_formula: i128,
    w3_curve_formula: i128,
    d_line: f64,
    d_plane: f64,
    e: [f64; 3],     // E_2, E_3, E_4
    ratio: [f64; 3], // A_k / E_k
    pred: [f64; 3],  // 1 + hat Delta_k / E_k
    comp_line: [f64; 3],
    comp_plane: [f64; 3],
    pred4_a3: f64,   // exploratory: A_3-aware k=4 prediction
    z_line: f64,
    z_plane: f64,
    a2_resid: f64,   // A_2 - E_2 - D_line
    struct_f4: i64,  // all-lift F_4-subspaces of F_4-dim 2 (N = 12 only), -1 if n/a
    struct_f16: i64,
    secs: f64,
}

fn all_lift_f4_subspaces(f: &Fq, s: &Bits, d: u32) -> (i64, i64) {
    // returns (#F_{2^d}-subspaces of F_2-dim 4, #F_16-lines) for d = 2
    let q1 = f.q1;
    assert!(q1 % 15 == 0 && q1 % 3 == 0);
    let w = f.exp[q1 / 3]; // primitive cube root of unity
    let w2 = f.mul(w, w);
    let _ = d;
    // good F_4-points: representatives with log < q1/3 whose 3 multiples lie in L
    let mut good: Vec<u32> = vec![];
    for l in 0..q1 / 3 {
        let x = f.exp[l];
        if s.get(x) && s.get(f.mul(x, w)) && s.get(f.mul(x, w2)) {
            good.push(x);
        }
    }
    let mut pairs = 0i64;
    for i in 0..good.len() {
        for j in i + 1..good.len() {
            let (r1, r2) = (good[i], good[j]);
            let mut ok = true;
            'o: for al in [0u32, 1, w, w2] {
                for be in [0u32, 1, w, w2] {
                    if al == 0 && be == 0 {
                        continue;
                    }
                    let y = f.mul(al, r1) ^ f.mul(be, r2);
                    if !s.get(y) {
                        ok = false;
                        break 'o;
                    }
                }
            }
            if ok {
                pairs += 1;
            }
        }
    }
    // F_16-lines: representatives with log < q1/15, all 15 multiples in L
    let g16 = f.exp[q1 / 15];
    let mut lines = 0i64;
    for l in 0..q1 / 15 {
        let x = f.exp[l];
        let mut z = x;
        let mut ok = true;
        for _ in 0..15 {
            if !s.get(z) {
                ok = false;
                break;
            }
            z = f.mul(z, g16);
        }
        if ok {
            lines += 1;
        }
    }
    assert!(pairs % 10 == 0, "pairs {pairs}");
    (pairs / 10, lines)
}

fn compute_row(f: &Fq, inst: &Inst) -> Row {
    let t0 = Instant::now();
    let n = f.n;
    let q = 1usize << n;
    let (qf, uf) = (q as f64, (q - 1) as f64);
    let (qi, ui) = (q as i128, (q - 1) as i128);
    let all = members(n, &inst.s);
    let m = all.len() as u64;
    let counts = count_subspaces(n, &all, 4);
    assert!(counts[0] == 1 && counts[1] == m);
    // Walsh transform of psi
    let mut w: Vec<i64> = (0..q as u32)
        .map(|x| if x == 0 { 0 } else if inst.s.get(x) { 1 } else { -1 })
        .collect();
    wht_i64(&mut w);
    let (mut sw2, mut sw3, mut sw4) = (0i128, 0i128, 0i128);
    for &x in &w {
        let x = x as i128;
        sw2 += x * x;
        sw3 += x * x * x;
        sw4 += x * x * x * x;
    }
    assert_eq!(sw2, qi * ui, "Parseval");
    let sw1 = w[0];
    assert_eq!(2 * m as i64 - ui as i64, sw1);
    assert!(sw3 % qi == 0);
    let c3 = sw3 / qi;
    // exact A_2 identity: 8T = u(u-1) + 3 s (u-1) + 3 (s^2 - u) + C3, A_2 = T/6
    let sv = sw1 as i128;
    let t8 = ui * (ui - 1) + 3 * sv * (ui - 1) + 3 * (sv * sv - ui) + c3;
    let a2_exact_ok = t8 % 48 == 0 && counts[2] as i128 == t8 / 48;
    let a2_exact = t8 / 48;
    // centred correlations
    let p = m as f64 / uf;
    let mut ev: Vec<f64> = (0..q as u32)
        .map(|x| if x == 0 { 0.0 } else if inst.s.get(x) { 1.0 - p } else { -p })
        .collect();
    let p2: f64 = ev.iter().map(|x| x * x).sum();
    let p4: f64 = ev.iter().map(|x| x.powi(4)).sum();
    wht_f64(&mut ev);
    let c3e: f64 = ev.iter().map(|x| x * x * x).sum::<f64>() / qf;
    let r4e: f64 = ev.iter().map(|x| x.powi(4)).sum::<f64>() / qf;
    let nl = uf * (uf - 1.0) / 6.0;
    let np = uf * (uf - 1.0) * (uf - 3.0) / 24.0;
    let mf = m as f64;
    let d_line = c3e / 6.0 - nl * eps(3, mf, uf, p);
    let d_plane = (r4e - 3.0 * p2 * p2 + 2.0 * p4) / 24.0 - np * eps(4, mf, uf, p);
    let mut e = [0.0; 3];
    let mut ratio = [0.0; 3];
    let mut pred = [0.0; 3];
    let mut comp_line = [0.0; 3];
    let mut comp_plane = [0.0; 3];
    for (i, k) in [2i32, 3, 4].iter().enumerate() {
        let k = *k;
        e[i] = expect(n, k as u32, m);
        ratio[i] = counts[k as usize] as f64 / e[i];
        let nk = (1i32 << k) - 1;
        let nn = n as i32;
        let mut dl = 0.0;
        if nk >= 3 {
            dl += p.powi(nk - 3) * d_line * (gauss(nn - 2, k - 2) - gauss(nn - 3, k - 3));
        }
        if nk >= 4 {
            dl += p.powi(nk - 4) * d_plane * (gauss(nn - 3, k - 3) - gauss(nn - 4, k - 4));
        }
        pred[i] = 1.0 + dl / e[i];
        comp_line[i] = if nk >= 3 { p.powi(nk - 3) * d_line * (gauss(nn - 2, k - 2) - gauss(nn - 3, k - 3)) / e[i] } else { 0.0 };
        comp_plane[i] = if nk >= 4 { p.powi(nk - 4) * d_plane * (gauss(nn - 3, k - 3) - gauss(nn - 4, k - 4)) / e[i] } else { 0.0 };
    }
    // exploratory second level: replace the plane-only rank-3 block by the measured A_3 deviation
    let nn = n as i32;
    let a3_dev = counts[3] as f64 - e[1];
    let s3p = a3_dev - p.powi(4) * d_line * (gauss(nn - 2, 1) - 1.0);
    let d4 = p.powi(12) * d_line * (gauss(nn - 2, 2) - gauss(nn - 3, 1))
        + gauss(nn - 3, 1) * p.powi(8) * s3p
        - p.powi(11) * d_plane;
    let pred4_a3 = 1.0 + d4 / e[2];
    let z_line = d_line / ((nl).sqrt() * (p * (1.0 - p)).powf(1.5));
    let z_plane = d_plane / ((np).sqrt() * (p * (1.0 - p)).powi(2));
    let a2_resid = counts[2] as f64 - e[0] - d_line;
    let (sf4, sf16) = if n == 12 { all_lift_f4_subspaces(f, &inst.s, 2) } else { (-1, -1) };
    Row {
        n,
        arm: inst.arm.clone(),
        pop: inst.pop,
        label: inst.label.clone(),
        a: inst.a,
        b: inst.b,
        planted_l: inst.planted_l,
        m,
        counts,
        sw1,
        sw2,
        sw3,
        sw4,
        kurt: sw4 as f64 / (qf * uf * uf),
        w4_q3: sw4 as f64 / (qf * qf * qf),
        c3,
        a2_exact,
        a2_exact_ok,
        w4_curve_formula: 2 * qi * qi * qi - 2 * qi * qi - 3 * qi,
        w3_curve_formula: {
            let eps_a: i128 = if f.tr(inst.a) == 1 { -1 } else { 1 };
            if n % 2 == 1 { -eps_a * qi * (qi - 2) } else { eps_a * qi * (qi + 2) }
        },
        d_line,
        d_plane,
        e,
        ratio,
        pred,
        comp_line,
        comp_plane,
        pred4_a3,
        z_line,
        z_plane,
        a2_resid,
        struct_f4: sf4,
        struct_f16: sf16,
        secs: t0.elapsed().as_secs_f64(),
    }
}

fn row_json(r: &Row) -> J {
    let is_curve = r.arm.starts_with("curve");
    o(vec![
        ("kind", s("set")),
        ("n", J::I(r.n as i128)),
        ("arm", s(&r.arm)),
        ("pop", s(r.pop)),
        ("label", s(&r.label)),
        ("a", J::I(r.a as i128)),
        ("b", J::I(r.b as i128)),
        ("planted_l", J::I(r.planted_l as i128)),
        ("size", J::I(r.m as i128)),
        ("density", J::N(r.m as f64 / ((1u64 << r.n) - 1) as f64)),
        ("counts_A0_A4", J::A(r.counts.iter().map(|&c| J::I(c as i128)).collect())),
        ("walsh_sum1", J::I(r.sw1 as i128)),
        ("walsh_sum2", J::I(r.sw2)),
        ("walsh_sum3", J::I(r.sw3)),
        ("walsh_sum4", J::I(r.sw4)),
        ("walsh4_over_q3", J::N(r.w4_q3)),
        ("kurtosis", J::N(r.kurt)),
        ("c3_three_point", J::I(r.c3)),
        ("a2_from_identity", J::I(r.a2_exact)),
        ("a2_identity_exact", J::B(r.a2_exact_ok)),
        ("curve_w4_formula", if is_curve { J::I(r.w4_curve_formula) } else { J::Null }),
        ("curve_w4_matches", if is_curve { J::B(r.sw4 == r.w4_curve_formula) } else { J::Null }),
        ("curve_w3_formula", if is_curve { J::I(r.w3_curve_formula) } else { J::Null }),
        ("curve_w3_matches", if is_curve { J::B(r.sw3 == r.w3_curve_formula) } else { J::Null }),
        ("d_line", J::N(r.d_line)),
        ("d_plane", J::N(r.d_plane)),
        ("E2_E3_E4", fa(&r.e)),
        ("ratio_A2_A3_A4", fa(&r.ratio)),
        ("pred_ratio_k2_k3_k4", fa(&r.pred)),
        ("a2_minus_E2_minus_Dline", J::N(r.a2_resid)),
        ("comp_line_k2_k3_k4", fa(&r.comp_line)),
        ("comp_plane_k2_k3_k4", fa(&r.comp_plane)),
        ("explor_pred_ratio_k4_with_measured_A3", J::N(r.pred4_a3)),
        ("z_line", J::N(r.z_line)),
        ("z_plane", J::N(r.z_plane)),
        ("f4_subspaces_dim2", J::I(r.struct_f4 as i128)),
        ("f16_lines", J::I(r.struct_f16 as i128)),
        ("secs", J::N(r.secs)),
    ])
}

// ----------------------------------------------------------- analysis ----

fn col<F: Fn(&Row) -> f64>(rows: &[&Row], g: F) -> Vec<f64> {
    rows.iter().map(|r| g(r)).collect()
}

fn desc(v: &[f64], rng: &mut Rng) -> J {
    let (lo, hi) = if v.len() >= 2 { boot_mean_ci(v, 10000, rng) } else { (f64::NAN, f64::NAN) };
    o(vec![
        ("n", J::I(v.len() as i128)),
        ("mean", J::N(mean(v))),
        ("sd", J::N(sd(v))),
        ("mean_ci95_boot", fa(&[lo, hi])),
        ("min", J::N(v.iter().cloned().fold(f64::INFINITY, f64::min))),
        ("max", J::N(v.iter().cloned().fold(f64::NEG_INFINITY, f64::max))),
    ])
}

fn corr_block(x: &[f64], y: &[f64], rng: &mut Rng) -> J {
    let rs = spearman(x, y);
    let rp = pearson(x, y);
    let p = spearman_perm_p(x, y, 100000, rng);
    let (sl, sh) = fisher_ci(rs, x.len(), true);
    let (pl, ph) = fisher_ci(rp, x.len(), false);
    o(vec![
        ("n", J::I(x.len() as i128)),
        ("spearman", J::N(rs)),
        ("spearman_ci95_fisher", fa(&[sl, sh])),
        ("spearman_perm_p_one_sided_pos", J::N(p)),
        ("pearson", J::N(rp)),
        ("pearson_ci95_fisher", fa(&[pl, ph])),
    ])
}

fn getf(j: &J, key: &str) -> f64 {
    if let J::O(v) = j {
        for (k, x) in v {
            if k == key {
                if let J::N(z) = x {
                    return *z;
                }
            }
        }
    }
    f64::NAN
}

struct Verd {
    p1: bool,
    p2: bool,
    p3a: bool,
    p3b: bool,
    pc: bool,
    i1: bool,
}

fn analyze(rows: &[Row], quick: bool) -> (J, Verd) {
    let mut per_n: Vec<(String, J)> = vec![];
    let mut v = Verd { p1: true, p2: true, p3a: true, p3b: true, pc: true, i1: true };
    let (mut p2_pooled_all, mut p2_within_count) = (true, 0);
    let mut ns: Vec<u32> = rows.iter().map(|r| r.n).collect();
    ns.sort();
    ns.dedup();
    for &n in &ns {
        let mut rng = Rng::new("stats", n);
        let rn: Vec<&Row> = rows.iter().filter(|r| r.n == n).collect();
        let curves: Vec<&Row> = rn.iter().copied().filter(|r| r.arm.starts_with("curve")).collect();
        let paired: Vec<&Row> = rn.iter().copied().filter(|r| r.arm == "null" || r.arm == "null_r").collect();
        let allnull: Vec<&Row> = rn.iter().copied().filter(|r| r.arm.starts_with("null")).collect();
        let curves_orig: Vec<&Row> = curves.iter().copied().filter(|r| r.pop == "orig").collect();
        let paired_orig: Vec<&Row> = paired.iter().copied().filter(|r| r.pop == "orig").collect();
        let primary = n != 12; // N = 12 is a descriptive contrast
        let mut blk: Vec<(&str, J)> = vec![];
        blk.push(("n_sets", J::I(rn.len() as i128)));
        // ---- instrument: identity I1 and exact k=2 identity ----
        let i1_ok = curves.iter().all(|r| r.sw4 == r.w4_curve_formula);
        let i2_ok = curves.iter().all(|r| r.sw3 == r.w3_curve_formula);
        let a2_ok = rn.iter().all(|r| r.a2_exact_ok);
        let a2_max_resid = rn.iter().map(|r| r.a2_resid.abs()).fold(0.0, f64::max);
        blk.push((
            "I1_curve_walsh4_equals_2q3_2q2_3q",
            o(vec![
                ("all_curves_match", J::B(i1_ok)),
                ("I2_all_curves_walsh3_match_pm_q_q_minus_2_or_q_plus_2", J::B(i2_ok)),
                ("n_curves", J::I(curves.len() as i128)),
                ("formula", J::I(curves[0].w4_curve_formula)),
                ("distinct_curve_sw4_values", J::I({
                    let mut x: Vec<i128> = curves.iter().map(|r| r.sw4).collect();
                    x.sort();
                    x.dedup();
                    x.len() as i128
                })),
            ]),
        ));
        // ---- P1 ----
        let kc = col(&curves, |r| r.kurt);
        let kp = col(&paired, |r| r.kurt);
        let ka = col(&allnull, |r| r.kurt);
        let w4c = col(&curves, |r| r.w4_q3);
        let w4a = col(&allnull, |r| r.w4_q3);
        let p1p = mean_diff_perm_p(&kp, &kc, 100000, &mut rng);
        let (dl, dh) = boot_diff_ci(&kp, &kc, 10000, &mut rng);
        let p1_n = p1p < 0.01 && (mean(&kp) - mean(&kc)) >= 0.5;
        blk.push((
            "P1",
            o(vec![
                ("kurtosis_curves", desc(&kc, &mut rng)),
                ("kurtosis_paired_nulls", desc(&kp, &mut rng)),
                ("kurtosis_all_nulls", desc(&ka, &mut rng)),
                ("walsh4_over_q3_curves", desc(&w4c, &mut rng)),
                ("walsh4_over_q3_nulls", desc(&w4a, &mut rng)),
                ("mean_diff_null_minus_curve", J::N(mean(&kp) - mean(&kc))),
                ("mean_diff_ci95_boot", fa(&[dl, dh])),
                ("cohen_d_null_minus_curve", J::N(cohen_d(&kp, &kc))),
                ("cliff_delta_null_vs_curve", J::N(cliff_delta(&kp, &kc))),
                ("perm_p_one_sided_null_gt_curve", J::N(p1p)),
                ("pass_this_n", J::B(p1_n)),
            ]),
        ));
        // ---- effect on ratio4 (curves vs paired nulls) ----
        let r4c = col(&curves, |r| r.ratio[2]);
        let r4p = col(&paired, |r| r.ratio[2]);
        let r4o_c = col(&curves_orig, |r| r.ratio[2]);
        let r4o_p = col(&paired_orig, |r| r.ratio[2]);
        let (el, eh) = boot_diff_ci(&r4p, &r4c, 10000, &mut rng);
        blk.push((
            "ratio_A4_curves_vs_nulls",
            o(vec![
                ("curves", desc(&r4c, &mut rng)),
                ("paired_nulls", desc(&r4p, &mut rng)),
                ("curves_orig_population", desc(&r4o_c, &mut rng)),
                ("paired_nulls_orig_population", desc(&r4o_p, &mut rng)),
                ("mean_diff_null_minus_curve", J::N(mean(&r4p) - mean(&r4c))),
                ("mean_diff_ci95_boot", fa(&[el, eh])),
                ("cohen_d_null_minus_curve", J::N(cohen_d(&r4p, &r4c))),
                ("perm_p_one_sided_null_gt_curve", J::N(mean_diff_perm_p(&r4p, &r4c, 100000, &mut rng))),
            ]),
        ));
        // ---- P2 ----
        let sets_p2: Vec<&Row> = curves.iter().chain(allnull.iter()).copied().collect();
        let (x_all, y_all) = (col(&sets_p2, |r| r.kurt), col(&sets_p2, |r| r.ratio[2]));
        let pooled = corr_block(&x_all, &y_all, &mut rng);
        let pooled_p = getf(&pooled, "spearman_perm_p_one_sided_pos");
        let pooled_rho = getf(&pooled, "spearman");
        let (xn, yn) = (col(&allnull, |r| r.kurt), col(&allnull, |r| r.ratio[2]));
        let within_null = corr_block(&xn, &yn, &mut rng);
        let wn_p = getf(&within_null, "spearman_perm_p_one_sided_pos");
        let wn_rho = getf(&within_null, "spearman");
        let (xc, yc) = (col(&curves, |r| r.kurt), col(&curves, |r| r.ratio[2]));
        let kurt_sd_curves = sd(&xc);
        // within-curve: kurtosis is constant (identity I1); the correlation is undefined.
        // Secondary: curves, ratio4 vs the first-order predictor pred4.
        let (xc2, yc2) = (col(&curves, |r| r.pred[2]), col(&curves, |r| r.ratio[2]));
        let within_curve_pred = corr_block(&xc2, &yc2, &mut rng);
        // secondary: nulls only, ratio4 vs D_plane
        let (xd, yd) = (col(&allnull, |r| r.d_plane), col(&allnull, |r| r.ratio[2]));
        let within_null_dplane = corr_block(&xd, &yd, &mut rng);
        if primary {
            if !(pooled_p < 0.01 && pooled_rho > 0.0) {
                p2_pooled_all = false;
            }
            if wn_p < 0.05 && wn_rho > 0.0 {
                p2_within_count += 1;
            }
        }
        blk.push((
            "P2",
            o(vec![
                ("pooled_curves_and_nulls", pooled),
                ("within_nulls_paired_plus_extra", within_null),
                ("within_nulls_ratio4_vs_Dplane_secondary", within_null_dplane),
                (
                    "within_curves_kurtosis",
                    o(vec![
                        ("kurtosis_sd", J::N(kurt_sd_curves)),
                        ("note", s("kurtosis is constant across curves (identity I1): correlation undefined")),
                        ("ratio4_sd_curves", J::N(sd(&yc))),
                    ]),
                ),
                ("within_curves_ratio4_vs_first_order_prediction_secondary", within_curve_pred),
            ]),
        ));
        // ---- P3 ----
        let obs_def_c = 1.0 - mean(&r4c);
        let pred_def_c = 1.0 - mean(&col(&curves, |r| r.pred[2]));
        let ratio_def = pred_def_c / obs_def_c;
        let sets_fit: Vec<&Row> = curves.iter().chain(allnull.iter()).copied().collect();
        let fx = col(&sets_fit, |r| r.pred[2] - 1.0);
        let fy = col(&sets_fit, |r| r.ratio[2] - 1.0);
        let (sl, ic, r2) = ols(&fx, &fy);
        let nx: Vec<f64> = allnull.iter().map(|r| r.pred[2] - 1.0).collect();
        let ny: Vec<f64> = allnull.iter().map(|r| r.ratio[2] - 1.0).collect();
        let (sln, icn, r2n) = ols(&nx, &ny);
        let cx: Vec<f64> = curves.iter().map(|r| r.pred[2] - 1.0).collect();
        let cy: Vec<f64> = curves.iter().map(|r| r.ratio[2] - 1.0).collect();
        let (slc, icc, r2c) = ols(&cx, &cy);
        let p3b_n = ratio_def >= 0.5 && ratio_def <= 2.0 && r2 >= 0.5 && sl >= 0.5 && sl <= 2.0;
        let k3_def_c = 1.0 - mean(&col(&curves, |r| r.ratio[1]));
        let k3_pred_c = 1.0 - mean(&col(&curves, |r| r.pred[1]));
        let k3x: Vec<f64> = sets_fit.iter().map(|r| r.pred[1] - 1.0).collect();
        let k3y: Vec<f64> = sets_fit.iter().map(|r| r.ratio[1] - 1.0).collect();
        let (sl3, ic3, r23) = ols(&k3x, &k3y);
        let k2_defs: Vec<f64> = curves.iter().map(|r| r.ratio[0] - 1.0).collect();
        if primary {
            if !(a2_ok && a2_max_resid <= 1e-4) {
                v.p3a = false;
            }
            if !p3b_n {
                v.p3b = false;
            }
            if !p1_n {
                v.p1 = false;
            }
            if !(i1_ok && i2_ok) {
                v.i1 = false;
            }
        }
        blk.push((
            "P3",
            o(vec![
                (
                    "k2_exact_identity",
                    o(vec![
                        ("integer_identity_holds_all_sets", J::B(a2_ok)),
                        ("max_abs_A2_minus_E2_minus_Dline", J::N(a2_max_resid)),
                        ("mean_ratio_A2_minus_1_curves", J::N(mean(&k2_defs))),
                    ]),
                ),
                (
                    "k4_first_order",
                    o(vec![
                        ("observed_deficit_curves_1_minus_ratio4", J::N(obs_def_c)),
                        ("predicted_deficit_curves", J::N(pred_def_c)),
                        ("predicted_over_observed", J::N(ratio_def)),
                        ("observed_deficit_times_q", J::N(obs_def_c * (1u64 << n) as f64)),
                        ("predicted_deficit_times_q", J::N(pred_def_c * (1u64 << n) as f64)),
                        ("residual_deficit_times_q", J::N((obs_def_c - pred_def_c) * (1u64 << n) as f64)),
                        ("residual_deficit_times_q2", J::N((obs_def_c - pred_def_c) * ((1u64 << n) as f64).powi(2))),
                        ("fit_all_sets_slope", J::N(sl)),
                        ("fit_all_sets_intercept", J::N(ic)),
                        ("fit_all_sets_r2", J::N(r2)),
                        ("fit_nulls_slope", J::N(sln)),
                        ("fit_nulls_intercept", J::N(icn)),
                        ("fit_nulls_r2", J::N(r2n)),
                        ("fit_curves_slope", J::N(slc)),
                        ("fit_curves_intercept", J::N(icc)),
                        ("fit_curves_r2", J::N(r2c)),
                        ("pass_this_n", J::B(p3b_n)),
                    ]),
                ),
                (
                    "k3_first_order_descriptive",
                    o(vec![
                        ("observed_deficit_curves", J::N(k3_def_c)),
                        ("predicted_deficit_curves", J::N(k3_pred_c)),
                        ("fit_all_sets_slope", J::N(sl3)),
                        ("fit_all_sets_intercept", J::N(ic3)),
                        ("fit_all_sets_r2", J::N(r23)),
                    ]),
                ),
            ]),
        ));
        // ---- terms: how large each correlation is, curves vs nulls ----
        blk.push((
            "correlations",
            o(vec![
                ("d_plane_curves", desc(&col(&curves, |r| r.d_plane), &mut rng)),
                ("d_plane_nulls", desc(&col(&allnull, |r| r.d_plane), &mut rng)),
                ("d_line_curves", desc(&col(&curves, |r| r.d_line), &mut rng)),
                ("d_line_nulls", desc(&col(&allnull, |r| r.d_line), &mut rng)),
                ("z_plane_curves", desc(&col(&curves, |r| r.z_plane), &mut rng)),
                ("z_plane_nulls", desc(&col(&allnull, |r| r.z_plane), &mut rng)),
                ("z_line_curves", desc(&col(&curves, |r| r.z_line), &mut rng)),
                ("z_line_nulls", desc(&col(&allnull, |r| r.z_line), &mut rng)),
                ("comp_plane4_curves", desc(&col(&curves, |r| r.comp_plane[2]), &mut rng)),
                ("comp_line4_curves", desc(&col(&curves, |r| r.comp_line[2]), &mut rng)),
                ("explor_pred4_with_measured_A3_deficit_curves", J::N(1.0 - mean(&col(&curves, |r| r.pred4_a3)))),
                ("explor_pred4_with_measured_A3_fit_all_sets_r2", J::N({
                    let x: Vec<f64> = sets_fit.iter().map(|r| r.pred4_a3 - 1.0).collect();
                    let y: Vec<f64> = sets_fit.iter().map(|r| r.ratio[2] - 1.0).collect();
                    ols(&x, &y).2
                })),
                ("c3_curves", desc(&col(&curves, |r| r.c3 as f64), &mut rng)),
                ("c3_nulls", desc(&col(&allnull, |r| r.c3 as f64), &mut rng)),
                ("density_curves", desc(&col(&curves, |r| r.m as f64 / ((1u64 << n) - 1) as f64), &mut rng)),
            ]),
        ));
        for (kk, name) in [(0usize, "ratio_A2"), (1, "ratio_A3")] {
            blk.push((
                if kk == 0 { "ratio_A2_summary" } else { "ratio_A3_summary" },
                o(vec![
                    ("curves", desc(&col(&curves, |r| r.ratio[kk]), &mut rng)),
                    ("nulls", desc(&col(&allnull, |r| r.ratio[kk]), &mut rng)),
                    ("name", s(name)),
                ]),
            ));
        }
        // ---- exploratory (not gated): what moves ratio4 inside each arm ----
        {
            let mut items: Vec<(String, J)> = vec![];
            for (name, set) in [("nulls", &allnull), ("curves", &curves)] {
                let y = col(set, |r| r.ratio[2]);
                let preds: Vec<(&str, Vec<f64>)> = vec![
                    ("ratio3", col(set, |r| r.ratio[1])),
                    ("ratio2", col(set, |r| r.ratio[0])),
                    ("d_plane", col(set, |r| r.d_plane)),
                    ("d_line", col(set, |r| r.d_line)),
                    ("density", col(set, |r| r.m as f64)),
                    ("pred4", col(set, |r| r.pred[2])),
                    ("kurtosis", col(set, |r| r.kurt)),
                    ("z_plane", col(set, |r| r.z_plane)),
                    ("z_line", col(set, |r| r.z_line)),
                    ("comp_plane4", col(set, |r| r.comp_plane[2])),
                    ("comp_line4", col(set, |r| r.comp_line[2])),
                    ("pred4_with_measured_A3", col(set, |r| r.pred4_a3)),
                ];
                let mut ent: Vec<(String, J)> = vec![("sd_ratio4".to_string(), J::N(sd(&y)))];
                for (vn, x) in preds {
                    let rho = spearman(&x, &y);
                    let (sl_, _ic, r2_) = ols(&x, &y);
                    ent.push((
                        vn.to_string(),
                        o(vec![("spearman", J::N(rho)), ("ols_slope", J::N(sl_)), ("r2", J::N(r2_))]),
                    ));
                }
                items.push((name.to_string(), J::O(ent)));
            }
            blk.push(("exploratory_within_arm_dependence_of_ratio4", J::O(items)));
        }
        // ---- positive control: planted rank-one Walsh bias (gated) ----
        let nm = mean(&col(&allnull, |r| r.kurt));
        let ns_ = sd(&col(&allnull, |r| r.kurt));
        let rm = mean(&col(&allnull, |r| r.ratio[2]));
        let rs_ = sd(&col(&allnull, |r| r.ratio[2]));
        {
            let mut lv: Vec<J> = vec![];
            let mut bias_all: Vec<&Row> = vec![];
            let mut top_ok = true;
            for e in ["e46", "e42", "e38", "e34", "e30"] {
                let arm = format!("bias_{e}");
                let pl: Vec<&Row> = rn.iter().copied().filter(|r| r.arm == arm).collect();
                if pl.is_empty() {
                    continue;
                }
                let kk = col(&pl, |r| r.kurt);
                let rr = col(&pl, |r| r.ratio[2]);
                let ok = kk.iter().all(|&x| x > nm + 3.0 * ns_) && rr.iter().all(|&x| x > rm + 3.0 * rs_);
                if e == "e30" {
                    top_ok = ok;
                }
                lv.push(o(vec![
                    ("arm", s(&arm)),
                    ("kurtosis_mean", J::N(mean(&kk))),
                    ("kurtosis_min", J::N(kk.iter().cloned().fold(f64::INFINITY, f64::min))),
                    ("ratio4_mean", J::N(mean(&rr))),
                    ("ratio4_min", J::N(rr.iter().cloned().fold(f64::INFINITY, f64::min))),
                    ("pred4_mean_first_order", J::N(mean(&col(&pl, |r| r.pred[2])))),
                    ("size_mean", J::N(mean(&col(&pl, |r| r.m as f64)))),
                    ("all_exceed_null_mean_plus_3sd", J::B(ok)),
                ]));
                bias_all.extend(pl);
            }
            if !bias_all.is_empty() {
                let corr = corr_block(&col(&bias_all, |r| r.kurt), &col(&bias_all, |r| r.ratio[2]), &mut rng);
                let c_ok = getf(&corr, "spearman") > 0.0 && getf(&corr, "spearman_perm_p_one_sided_pos") < 0.01;
                if primary {
                    v.pc &= top_ok && c_ok;
                }
                blk.push((
                    "positive_control_planted_bias",
                    o(vec![
                        ("null_kurtosis_mean_sd", fa(&[nm, ns_])),
                        ("null_ratio4_mean_sd", fa(&[rm, rs_])),
                        ("per_level", J::A(lv)),
                        ("strongest_level_exceeds_null_3sd", J::B(top_ok)),
                        ("planted_sets_corr_kurt_vs_ratio4", corr),
                        ("pass_this_n", J::B(top_ok && c_ok)),
                    ]),
                ));
            }
        }
        // ---- exploratory (not gated): unions of cosets ----
        {
            let mut pcs: Vec<J> = vec![];
            for ell in 1..=3u32 {
                let pl: Vec<&Row> = rn.iter().copied().filter(|r| r.planted_l == ell).collect();
                if pl.is_empty() {
                    continue;
                }
                pcs.push(o(vec![
                    ("ell", J::I(ell as i128)),
                    ("kurtosis_mean", J::N(mean(&col(&pl, |r| r.kurt)))),
                    ("ratio4", fa(&col(&pl, |r| r.ratio[2]))),
                    ("pred4_first_order", fa(&col(&pl, |r| r.pred[2]))),
                    ("size", fa(&col(&pl, |r| r.m as f64))),
                ]));
            }
            if !pcs.is_empty() {
                blk.push(("exploratory_coset_unions_not_gated", J::A(pcs)));
            }
        }
        // ---- N = 12 structured subspaces ----
        if n == 12 {
            let mut st: Vec<J> = vec![];
            for r in rn.iter().filter(|r| r.struct_f4 >= 0) {
                let mm = r.m as f64;
                let g4 = (4095.0 * 1023.0) / (15.0 * 3.0);
                let mut ff = 1.0;
                for i in 0..15 {
                    ff *= (mm - i as f64) / (4095.0 - i as f64);
                }
                let e_st = g4 * ff;
                let e4 = r.e[2];
                let resid = (r.counts[4] as f64 - r.struct_f4 as f64) / (e4 - e_st);
                st.push(o(vec![
                    ("arm", s(&r.arm)),
                    ("label", s(&r.label)),
                    ("A4", J::I(r.counts[4] as i128)),
                    ("f4_dim2_subspaces", J::I(r.struct_f4 as i128)),
                    ("f16_lines", J::I(r.struct_f16 as i128)),
                    ("E_f4_dim2_random", J::N(e_st)),
                    ("ratio_A4", J::N(r.ratio[2])),
                    ("pred_ratio_A4", J::N(r.pred[2])),
                    ("ratio_A4_without_f4_structured", J::N(resid)),
                ]));
            }
            blk.push(("n12_structured_subspaces", J::A(st)));
        }
        per_n.push((format!("{n}"), J::O(blk.into_iter().map(|(k, x)| (k.to_string(), x)).collect())));
    }
    let p2 = p2_pooled_all && p2_within_count >= 2;
    v.p2 = p2;
    let verdict = o(vec![
        ("I1_I2_curve_walsh3_and_walsh4_identities_all_curves_N9_11_13", J::B(v.i1)),
        ("P1_curves_lower_walsh4", J::B(v.p1)),
        ("P2_pooled_positive_all_N_and_within_null_at_least_2_of_3", J::B(v.p2)),
        ("P2_within_null_significant_count_of_3", J::I(p2_within_count as i128)),
        ("P3a_k2_exact_identity", J::B(v.p3a)),
        ("P3b_k4_first_order_within_tolerance", J::B(v.p3b)),
        ("P3_overall", J::B(v.p3a && v.p3b)),
        ("positive_control", J::B(v.pc)),
    ]);
    let top = o(vec![
        ("study", s("lift_repulsion_mechanism_20261027")),
        ("quick", J::B(quick)),
        ("verdicts", verdict),
        ("per_N", J::O(per_n)),
    ]);
    (top, v)
}

// ----------------------------------------------------------- main ---------

fn explore(n: u32) {
    let poly = FIELDS.iter().find(|x| x.0 == n).unwrap().1;
    let f = Fq::new(n, poly);
    let q = 1i128 << n;
    println!("n={n} formula 2q^3-3q^2-6q = {}", 2 * q * q * q - 3 * q * q - 6 * q);
    let mut rng = Rng::new("explore", n);
    for (a, b) in [(0u32, 1u32), (1, 1), (0, 5), (1, 77), (0, 200 % ((1u32 << n) - 1) + 2), (1, 1 + rng.below(1000) as u32 + 1)] {
        let s = lift_set(&f, a, b);
        let mut w: Vec<i64> = (0..1usize << n)
            .map(|x| if x == 0 { 0 } else if s.get(x as u32) { 1 } else { -1 })
            .collect();
        wht_i64(&mut w);
        let (mut s2, mut s3, mut s4) = (0i128, 0i128, 0i128);
        for &x in &w {
            let x = x as i128;
            s2 += x * x;
            s3 += x * x * x;
            s4 += x * x * x * x;
        }
        println!("a={a} b={b} sw1={} sw2={s2} sw3={s3} sw4={s4}", w[0]);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|x| x.as_str()) {
        Some("explore") => explore(args[2].parse().unwrap()),
        Some("run") => {
            let out = std::path::PathBuf::from(&args[2]);
            let quick = args.iter().any(|a| a == "--quick");
            let threads: usize = args
                .iter()
                .position(|a| a == "--threads")
                .map(|i| args[i + 1].parse().unwrap())
                .unwrap_or(3);
            let ns: Vec<u32> = args
                .iter()
                .position(|a| a == "--ns")
                .map(|i| args[i + 1].split(',').map(|x| x.parse().unwrap()).collect())
                .unwrap_or(vec![9, 11, 13, 12]);
            std::fs::create_dir_all(&out).unwrap();
            let t0 = Instant::now();
            let mut rows_all: Vec<Row> = vec![];
            for &n in &ns {
                let poly = FIELDS.iter().find(|x| x.0 == n).expect("field").1;
                let f = Fq::new(n, poly);
                let insts = build_population(&f, n, quick, true);
                eprintln!("N = {n}: {} sets", insts.len());
                let next = AtomicUsize::new(0);
                let res: Mutex<Vec<(usize, Row)>> = Mutex::new(vec![]);
                std::thread::scope(|sc| {
                    for _ in 0..threads {
                        sc.spawn(|| loop {
                            let i = next.fetch_add(1, Ordering::SeqCst);
                            if i >= insts.len() {
                                break;
                            }
                            let r = compute_row(&f, &insts[i]);
                            eprintln!("  N={n} {} {} A4={} {:.1}s", r.arm, r.label, r.counts[4], r.secs);
                            res.lock().unwrap().push((i, r));
                        });
                    }
                });
                let mut v = res.into_inner().unwrap();
                v.sort_by_key(|x| x.0);
                rows_all.extend(v.into_iter().map(|x| x.1));
            }
            let mut rl = String::new();
            for r in &rows_all {
                r_push(&mut rl, row_json(r));
            }
            std::fs::write(out.join("rows.jsonl"), rl).unwrap();
            let (summary, v) = analyze(&rows_all, quick);
            std::fs::write(out.join("summary.json"), summary.to_s()).unwrap();
            eprintln!(
                "done {:.1}s: I1={} P1={} P2={} P3a={} P3b={} PC={}",
                t0.elapsed().as_secs_f64(),
                v.i1,
                v.p1,
                v.p2,
                v.p3a,
                v.p3b,
                v.pc
            );
        }
        _ => eprintln!("usage: explore <N> | run <outdir> [--quick] [--threads T] [--ns 9,11,13,12]"),
    }
}

fn r_push(buf: &mut String, j: J) {
    j.w(buf);
    buf.push('\n');
}
