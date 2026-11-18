//! **Encoding control for the subgroup factor base** (`research/enc_control_20261118/`).
//!
//! `research/rand_vs_struct_fb_20261118/` found that the `2^k`-subgroup factor
//! base written as the Kummer tower chain has a flat F4 solving degree (4–5),
//! while a random base written as one membership polynomial `∏(x − f)` is
//! floored at `|F| + 2`. It could not separate "the subgroup is intrinsically
//! easier" from "the subgroup has a cheap encoding". This example builds the
//! missing control: the *same* tower-shaped encoding for an arbitrary set.
//!
//! For a set `F ⊂ F_p` of size `2^k`, one block of unknowns per summand:
//!
//! - **tower** encoding: `u_0, …, u_{k−1}` with `u_j² − u_{j+1}` (`j < k − 1`)
//!   and the closing relation `u_{k−1}² − R_F(u)`, where
//!   `R_F(x) = x^{2^k} mod ∏_{f∈F}(x − f)` rewritten through
//!   `x^n = u^{bits(n)}` (multilinear, degree `≤ k`). `x = u_0`. For the
//!   subgroup `H = μ_{2^k}`, `R_F = 1`; for a coset `gH`, `R_F = g^{2^k}`.
//! - **bits** encoding: `x, b_0, …, b_{k−1}` with `b_j² − b_j` and
//!   `x − P_F(b)`, `P_F` the multilinear interpolant `P_F(v) = f_v`.
//! - **single** encoding (the earlier `randbase`): `x` with `∏_{f∈F}(x − f)`.
//!
//! The system at `m = 2` is `S₃(x_1, x_2, x_R)` plus both blocks. Every arm
//! is measured by the repository's F4 (`f4_fp`, grevlex): solving degree
//! `D = solving_degree_max`, degree reached, counted `F_p` multiplications
//! (`field_ops`), the per-step degree profile and the Hilbert first-fall
//! diagnostic. Where the closing relation is a constant (subgroup, coset) the
//! HKY closure engine (`hky_fp_tower`) gives the last and first fall degree
//! as a cross-check.
//!
//! Every encoding is verified before anything is measured (`--verify`): all
//! `2^k` points of `F` satisfy the block, the block's quotient has dimension
//! exactly `2^k` (F4 staircase), and the univariate eliminant has exactly
//! `2^k` roots in `F_p`; a deliberately wrong closing relation is the
//! negative control and must change the count of `F`-points it admits.
//!
//! Stage diagnostic only: no `S`, no rho comparison, no claim. `[pcc:unverified]`.

use std::collections::BTreeMap;
use std::io::Write;
use std::time::{Duration, Instant};

use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly};
use crypto_lib::cryptanalysis::f4_fp_tower::SquareRule;
use crypto_lib::cryptanalysis::hky_fp_tower::{self, HkyLimits, HkyRing};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

/// The prime of the task: `15·2^27 + 1`; 31 generates `F_p^*`.
const P1: u64 = 2013265921;
const GEN: u64 = 31;

// ── Field ──────────────────────────────────────────────────────────

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn addm(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
fn subm(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b % p) % p
}
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
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
fn invm(a: u64, p: u64) -> u64 {
    assert!(a % p != 0, "inverse of 0");
    powm(a, p - 2, p)
}
fn legendre(a: u64, p: u64) -> i32 {
    if a % p == 0 {
        return 0;
    }
    if powm(a, (p - 1) / 2, p) == 1 {
        1
    } else {
        -1
    }
}
/// Tonelli–Shanks square root of a quadratic residue.
fn sqrtm(a: u64, p: u64) -> u64 {
    assert_eq!(legendre(a, p), 1);
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while legendre(z, p) != -1 {
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
        let b = powm(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mulm(b, b, p);
        t = mulm(t, c, p);
        r = mulm(r, b, p);
    }
    r
}

/// Seed derivation: SplitMix64 over the master seed and the cell's labels.
fn derive(master: u64, parts: &[u64]) -> u64 {
    let mut h = master ^ 0x9e37_79b9_7f4a_7c15;
    for &x in parts {
        h = h.wrapping_add(x).wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = h;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        h = z ^ (z >> 31);
    }
    h
}
fn tag(s: &str) -> u64 {
    s.bytes()
        .fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3))
}

// ── Polynomials (sparse, exponent vectors) ─────────────────────────

#[derive(Clone, Debug)]
struct Pol {
    n: usize,
    terms: BTreeMap<Vec<u32>, u64>,
}

impl Pol {
    fn zero(n: usize) -> Self {
        Pol { n, terms: BTreeMap::new() }
    }
    fn constant(n: usize, c: u64, p: u64) -> Self {
        let mut f = Pol::zero(n);
        if c % p != 0 {
            f.terms.insert(vec![0; n], c % p);
        }
        f
    }
    fn monomial(e: Vec<u32>, c: u64, p: u64) -> Self {
        let mut f = Pol::zero(e.len());
        if c % p != 0 {
            f.terms.insert(e, c % p);
        }
        f
    }
    fn var(n: usize, i: usize) -> Self {
        let mut e = vec![0; n];
        e[i] = 1;
        Pol::monomial(e, 1, u64::MAX)
    }
    fn add(&self, o: &Pol, p: u64) -> Pol {
        let mut f = self.clone();
        for (e, c) in &o.terms {
            let v = f.terms.entry(e.clone()).or_insert(0);
            *v = addm(*v, *c, p);
        }
        f.terms.retain(|_, c| *c != 0);
        f
    }
    fn scale(&self, k: u64, p: u64) -> Pol {
        let mut f = Pol::zero(self.n);
        for (e, c) in &self.terms {
            let v = mulm(*c, k, p);
            if v != 0 {
                f.terms.insert(e.clone(), v);
            }
        }
        f
    }
    fn sub(&self, o: &Pol, p: u64) -> Pol {
        self.add(&o.scale(p - 1, p), p)
    }
    fn mul(&self, o: &Pol, p: u64) -> Pol {
        let mut f = Pol::zero(self.n);
        for (e1, c1) in &self.terms {
            for (e2, c2) in &o.terms {
                let e: Vec<u32> = e1.iter().zip(e2).map(|(a, b)| a + b).collect();
                let v = f.terms.entry(e).or_insert(0);
                *v = addm(*v, mulm(*c1, *c2, p), p);
            }
        }
        f.terms.retain(|_, c| *c != 0);
        f
    }
    fn eval(&self, x: &[u64], p: u64) -> u64 {
        let mut acc = 0u64;
        for (e, c) in &self.terms {
            let mut t = *c;
            for (i, k) in e.iter().enumerate() {
                if *k > 0 {
                    t = mulm(t, powm(x[i], *k as u64, p), p);
                }
            }
            acc = addm(acc, t, p);
        }
        acc
    }
    fn degree(&self) -> u32 {
        self.terms.keys().map(|e| e.iter().sum::<u32>()).max().unwrap_or(0)
    }
    fn raw(&self) -> Vec<(Vec<u32>, u64)> {
        self.terms.iter().map(|(e, c)| (e.clone(), *c)).collect()
    }
    fn to_poly(&self, p: u64) -> Poly {
        f4_fp::normalise(&self.raw(), p, Ordering::Grevlex)
    }
}

/// Semaev's `S₃` for `y² = x³ + ax + b` (same formula as `pkm_tower_pilot`).
fn s3(x1: &Pol, x2: &Pol, x3: &Pol, a: u64, b: u64, p: u64) -> Pol {
    let n = x1.n;
    let d = x1.sub(x2, p);
    let t1 = d.mul(&d, p).mul(x3, p).mul(x3, p);
    let s = x1.add(x2, p);
    let pr = x1.mul(x2, p);
    let inner = s
        .mul(&pr.add(&Pol::constant(n, a, p), p), p)
        .add(&Pol::constant(n, mulm(2, b, p), p), p);
    let t2 = inner.mul(x3, p).scale(p - 2, p);
    let q = pr.sub(&Pol::constant(n, a, p), p);
    let t3 = q.mul(&q, p).sub(&s.scale(mulm(4, b, p), p), p);
    t1.add(&t2, p).add(&t3, p)
}

// ── Univariate helpers (coefficients low to high) ──────────────────

/// `∏_{f∈F}(x − f)`, monic of degree `|F|`.
fn prod_linear(f: &[u64], p: u64) -> Vec<u64> {
    let mut c = vec![1u64];
    for &r in f {
        let mut nc = vec![0u64; c.len() + 1];
        for (i, &ci) in c.iter().enumerate() {
            nc[i + 1] = addm(nc[i + 1], ci, p);
            nc[i] = subm(nc[i], mulm(ci, r, p), p);
        }
        c = nc;
    }
    c
}
fn trim(v: &mut Vec<u64>) {
    while v.len() > 1 && *v.last().unwrap() == 0 {
        v.pop();
    }
}
fn umod(a: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    let mut r = a.to_vec();
    trim(&mut r);
    let dm = m.len() - 1;
    if dm == 0 {
        return vec![0];
    }
    let lc_inv = invm(m[dm], p);
    while r.len() > dm && !(r.len() == 1 && r[0] == 0) {
        let dr = r.len() - 1;
        let q = mulm(r[dr], lc_inv, p);
        if q != 0 {
            for i in 0..=dm {
                r[dr - dm + i] = subm(r[dr - dm + i], mulm(q, m[i], p), p);
            }
        }
        r.pop();
        trim(&mut r);
    }
    r
}
fn umulmod(a: &[u64], b: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = addm(c[i + j], mulm(x, y, p), p);
        }
    }
    umod(&c, m, p)
}
fn ugcd(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    trim(&mut a);
    trim(&mut b);
    while !(b.len() == 1 && b[0] == 0) {
        let r = umod(&a, &b, p);
        a = b;
        b = r;
    }
    a
}
/// Number of distinct roots in `F_p` of `m` (degree of `gcd(x^p − x, m)`).
fn roots_in_fp(m: &[u64], p: u64) -> usize {
    let mut m = m.to_vec();
    trim(&mut m);
    if m.len() <= 1 {
        return 0;
    }
    // x^p mod m by square-and-multiply.
    let mut r = vec![1u64];
    let mut base = umod(&[0, 1], &m, p);
    let mut e = p;
    while e > 0 {
        if e & 1 == 1 {
            r = umulmod(&r, &base, &m, p);
        }
        base = umulmod(&base, &base, &m, p);
        e >>= 1;
    }
    // x^p − x
    if r.len() < 2 {
        r.resize(2, 0);
    }
    r[1] = subm(r[1], 1, p);
    trim(&mut r);
    let g = ugcd(&m, &r, p);
    g.len() - 1
}

// ── Curve ──────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
}
impl Curve {
    fn random(p: u64, rng: &mut StdRng) -> Self {
        loop {
            let a = rng.gen_range(1..p);
            let b = rng.gen_range(1..p);
            let disc = addm(mulm(4, powm(a, 3, p), p), mulm(27, mulm(b, b, p), p), p);
            if disc != 0 {
                return Curve { p, a, b };
            }
        }
    }
    fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        addm(addm(powm(x, 3, p), mulm(self.a, x, p), p), self.b, p)
    }
    fn is_abscissa(&self, x: u64) -> bool {
        legendre(self.rhs(x), self.p) == 1
    }
    /// `x(P_1 + P_2)` for points with abscissae `x1 ≠ x2` and given ordinates.
    fn add_x(&self, x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
        let p = self.p;
        let l = mulm(subm(y2, y1, p), invm(subm(x2, x1, p), p), p);
        subm(subm(mulm(l, l, p), x1, p), x2, p)
    }
    fn random_abscissa(&self, rng: &mut StdRng, avoid: &[u64]) -> u64 {
        loop {
            let x = rng.gen_range(1..self.p);
            if self.is_abscissa(x) && !avoid.contains(&x) {
                return x;
            }
        }
    }
}

// ── Sets and encodings ─────────────────────────────────────────────

fn subgroup(k: usize, p: u64) -> Vec<u64> {
    let zeta = powm(GEN, (p - 1) >> k, p);
    if k > 0 {
        assert_eq!(powm(zeta, 1u64 << (k - 1), p), p - 1, "zeta has order 2^k");
    }
    (0..1u64 << k).map(|v| powm(zeta, v, p)).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Enc {
    Tower,
    Bits,
    Single,
}
impl Enc {
    fn block_size(self, k: usize) -> usize {
        match self {
            Enc::Tower => k,
            Enc::Bits => k + 1,
            Enc::Single => 1,
        }
    }
    fn name(self) -> &'static str {
        match self {
            Enc::Tower => "tower",
            Enc::Bits => "bits",
            Enc::Single => "single",
        }
    }
}

/// `R_F(x) = x^{2^k} − ∏(x − f)`: the coefficients `r_0..r_{2^k−1}`.
fn closing_coeffs(f: &[u64], p: u64) -> Vec<u64> {
    let pi = prod_linear(f, p);
    let d = f.len();
    (0..d).map(|i| subm(0, pi[i], p)).collect()
}

/// Möbius transform: `c_S` with `Σ_{S ⊆ v} c_S = f_v`.
fn multilinear_interpolant(f: &[u64], k: usize, p: u64) -> Vec<u64> {
    let mut c = f.to_vec();
    for j in 0..k {
        for s in 0..1usize << k {
            if s & (1 << j) != 0 {
                c[s] = subm(c[s], c[s ^ (1 << j)], p);
            }
        }
    }
    c
}

/// A block's equations in `n` variables from `base`, its `x` variable, and
/// the wrong-closing perturbation `delta` (0 for the correct encoding):
/// tower `R_F + delta`, bits `P_F + delta`, single `∏ + delta`.
fn block(enc: Enc, f: &[u64], k: usize, n: usize, base: usize, delta: u64, p: u64) -> (Vec<Pol>, usize) {
    assert_eq!(f.len(), 1 << k);
    match enc {
        Enc::Tower => {
            let mut eqs = Vec::new();
            for j in 0..k - 1 {
                let uj = Pol::var(n, base + j);
                eqs.push(uj.mul(&uj, p).sub(&Pol::var(n, base + j + 1), p));
            }
            let mut r = closing_coeffs(f, p);
            r[0] = addm(r[0], delta, p);
            let mut rhs = Pol::zero(n);
            for (m, &c) in r.iter().enumerate() {
                if c == 0 {
                    continue;
                }
                let mut e = vec![0u32; n];
                for j in 0..k {
                    if m >> j & 1 == 1 {
                        e[base + j] = 1;
                    }
                }
                rhs = rhs.add(&Pol::monomial(e, c, p), p);
            }
            let top = Pol::var(n, base + k - 1);
            eqs.push(top.mul(&top, p).sub(&rhs, p));
            (eqs, base)
        }
        Enc::Bits => {
            let mut eqs = Vec::new();
            for j in 0..k {
                let bj = Pol::var(n, base + 1 + j);
                eqs.push(bj.mul(&bj, p).sub(&bj, p));
            }
            let mut c = multilinear_interpolant(f, k, p);
            c[0] = addm(c[0], delta, p);
            let mut pf = Pol::zero(n);
            for (s, &cs) in c.iter().enumerate() {
                if cs == 0 {
                    continue;
                }
                let mut e = vec![0u32; n];
                for j in 0..k {
                    if s >> j & 1 == 1 {
                        e[base + 1 + j] = 1;
                    }
                }
                pf = pf.add(&Pol::monomial(e, cs, p), p);
            }
            eqs.push(Pol::var(n, base).sub(&pf, p));
            (eqs, base)
        }
        Enc::Single => {
            let mut pi = prod_linear(f, p);
            pi[0] = addm(pi[0], delta, p);
            let mut g = Pol::zero(n);
            for (i, &c) in pi.iter().enumerate() {
                let mut e = vec![0u32; n];
                e[base] = i as u32;
                g = g.add(&Pol::monomial(e, c, p), p);
            }
            (vec![g], base)
        }
    }
}

/// The value of a block's variables at the point `f_v` (`v` its label).
fn block_point(enc: Enc, fv: u64, v: usize, k: usize, p: u64) -> Vec<u64> {
    match enc {
        Enc::Tower => (0..k).map(|j| powm(fv, 1u64 << j, p)).collect(),
        Enc::Bits => {
            let mut out = vec![fv];
            out.extend((0..k).map(|j| (v >> j & 1) as u64));
            out
        }
        Enc::Single => vec![fv],
    }
}

// ── Arms ───────────────────────────────────────────────────────────

/// The factor base of an arm, in label order, and any notes on its draw.
fn arm_set(arm: &str, k: usize, r: usize, curve: &Curve, rng: &mut StdRng) -> (Vec<u64>, Value) {
    let p = curve.p;
    let h = subgroup(k, p);
    match arm {
        "tower-H" | "bits-H" => (h, json!({})),
        "coset" => {
            let g = loop {
                let g = rng.gen_range(2..p);
                if powm(g, 1u64 << k, p) != 1 {
                    break g;
                }
            };
            (h.iter().map(|&z| mulm(g, z, p)).collect(), json!({"g": g, "g_pow_2k": powm(g, 1u64 << k, p)}))
        }
        "tower-rand" | "bits-rand" | "randbase" => {
            let mut f: Vec<u64> = Vec::new();
            while f.len() < 1 << k {
                let x = curve.random_abscissa(rng, &f);
                f.push(x);
            }
            (f, json!({}))
        }
        "dose" => {
            assert!(r <= 1 << k, "r ≤ 2^k");
            let mut f = h.clone();
            // r distinct positions of H, chosen at random.
            let mut pos: Vec<usize> = (0..1 << k).collect();
            for i in 0..pos.len() {
                let j = rng.gen_range(i..pos.len());
                pos.swap(i, j);
            }
            let replaced: Vec<usize> = pos[..r].to_vec();
            for &i in &replaced {
                let mut avoid = f.clone();
                avoid.extend_from_slice(&h);
                f[i] = curve.random_abscissa(rng, &avoid);
            }
            (f, json!({"replaced_positions": replaced}))
        }
        _ => panic!("unknown arm {arm}"),
    }
}
fn arm_enc(arm: &str) -> Enc {
    match arm {
        "bits-H" | "bits-rand" => Enc::Bits,
        "randbase" => Enc::Single,
        _ => Enc::Tower,
    }
}

// ── Measurement ────────────────────────────────────────────────────

/// Standard monomials of a zero-dimensional leading ideal (`None` when some
/// variable has no pure power among the leading monomials, or above `cap`).
fn staircase(lms: &[Vec<u32>], n: usize, cap: usize) -> Option<usize> {
    let mut bound = vec![0u32; n];
    for (k, b) in bound.iter_mut().enumerate() {
        *b = lms
            .iter()
            .filter(|m| m.iter().enumerate().all(|(i, &e)| (i == k) == (e > 0)))
            .map(|m| m[k])
            .min()?;
    }
    let mut count = 0usize;
    let mut mono = vec![0u32; n];
    fn walk(i: usize, mono: &mut Vec<u32>, bound: &[u32], lms: &[Vec<u32>], count: &mut usize, cap: usize) -> bool {
        if lms.iter().any(|l| l.iter().zip(mono.iter()).all(|(a, b)| a <= b)) {
            return true;
        }
        if i == bound.len() {
            *count += 1;
            return *count <= cap;
        }
        for e in 0..bound[i] {
            mono[i] = e;
            if !walk(i + 1, mono, bound, lms, count, cap) {
                mono[i] = 0;
                return false;
            }
        }
        mono[i] = 0;
        true
    }
    if walk(0, &mut mono, &bound, lms, &mut count, cap) {
        Some(count)
    } else {
        None
    }
}

fn run_f4(eqs: &[Pol], n: usize, p: u64, sol: Option<&[u64]>, max_degree: u32, budget: Duration, runlog: bool) -> Value {
    if let Some(s) = sol {
        for (i, e) in eqs.iter().enumerate() {
            assert_eq!(e.eval(s, p), 0, "equation {i} does not vanish at the planted solution");
        }
    }
    let input: Vec<Poly> = eqs.iter().map(|e| e.to_poly(p)).filter(|f| !f.is_empty()).collect();
    let mut opts = F4Options::new(Ordering::Grevlex, max_degree)
        .with_budget(budget)
        .recording_degree_falls();
    if runlog {
        opts = opts.capturing_run_log();
    }
    let _ = f4_fp::take_run_log();
    let t0 = Instant::now();
    let r = f4_fp::f4(&input, n, p, &opts);
    let wall = t0.elapsed().as_secs_f64();
    let log = f4_fp::take_run_log();
    let profile: Vec<Value> = log
        .steps
        .iter()
        .map(|s| json!([s.degree, s.pairs, s.all_rows, s.new_polys, s.ops]))
        .collect();
    let planted_ok = sol.map(|s| !r.inconsistent && r.basis.iter().all(|f| f4_fp::eval(f, s, p) == 0));
    json!({
        "n_vars": n,
        "n_equations": input.len(),
        "input_degrees": eqs.iter().map(|e| e.degree()).collect::<Vec<_>>(),
        "input_terms": eqs.iter().map(|e| e.terms.len()).collect::<Vec<_>>(),
        "max_degree_bound": max_degree,
        "inconsistent": r.inconsistent,
        "D": r.solving_degree_max,
        "last_productive_degree": r.solving_degree,
        "degree_reached": r.degree_reached,
        "field_ops": r.field_ops,
        "steps": r.steps,
        "steps_to_solution": r.steps_to_solution,
        "max_rows": r.max_rows,
        "max_cols": r.max_cols,
        "max_cols_to_solution": r.max_cols_to_solution,
        "basis_len": r.basis.len(),
        "timed_out": r.timed_out,
        "pairs_above_bound": r.pairs_above_bound,
        "hilbert_first_fall": r.first_fall_degree,
        "hilbert_last_fall": r.last_fall_degree,
        "dreg_observed": r.degree_of_regularity_observed,
        "step_profile_deg_pairs_rows_new_ops": profile,
        "planted_ok": planted_ok,
        "ms": r.ms,
        "wall_s": wall,
    })
}

/// HKY closure over the tower quotient with a constant closing relation
/// `u_{k−1}² = c` in both blocks (subgroup and coset only).
fn run_hky(s3eq: &Pol, k: usize, closing: u64, p: u64, c_max: u32, budget: Duration) -> Value {
    let mut rules = Vec::new();
    for _blk in 0..2 {
        let base = rules.len();
        for j in 0..k {
            if j + 1 < k {
                rules.push(SquareRule { next: Some((base + j + 1) as u8), a: 0, b: 1, c: 0, d: 0 });
            } else {
                rules.push(SquareRule { next: None, a: 0, b: 0, c: 0, d: closing });
            }
        }
    }
    let ring = HkyRing::new(p, rules, 0);
    let g = ring.from_raw(&s3eq.raw());
    let gens = vec![g];
    let c_min = gens.iter().map(HkyRing::degree).min().unwrap_or(1);
    let limits = HkyLimits { budget, ..HkyLimits::default() };
    let t0 = Instant::now();
    let fd = hky_fp_tower::fall_degrees(&ring, &gens, c_min, c_max, &limits);
    json!({
        "hky_last_fall": fd.last_fall,
        "hky_first_fall": fd.first_fall,
        "hky_falls": fd.falls,
        "hky_censored": fd.closures.last().and_then(|c| c.censored.clone()),
        "hky_muladds": fd.closures.iter().map(|c| c.muladds).sum::<u64>(),
        "hky_c_min": c_min,
        "hky_wall_s": t0.elapsed().as_secs_f64(),
    })
}

/// Verifies one block's encoding of `F`, and the wrong-closing negative control.
fn verify(enc: Enc, f: &[u64], k: usize, p: u64, budget: Duration) -> Value {
    let n = enc.block_size(k);
    let check = |delta: u64| -> Value {
        let (eqs, _) = block(enc, f, k, n, 0, delta, p);
        let vanish = f
            .iter()
            .enumerate()
            .filter(|(v, &fv)| {
                let pt = block_point(enc, fv, *v, k, p);
                eqs.iter().all(|e| e.eval(&pt, p) == 0)
            })
            .count();
        let input: Vec<Poly> = eqs.iter().map(|e| e.to_poly(p)).collect();
        let opts = F4Options::new(Ordering::Grevlex, (2 * k + 4).max(1 << k.min(10)) as u32 + 2).with_budget(budget);
        let r = f4_fp::f4(&input, n, p, &opts);
        let lms: Vec<Vec<u32>> = r.basis.iter().filter(|g| !g.is_empty()).map(|g| g[0].0.clone()).collect();
        let quot = if r.inconsistent { Some(0) } else { staircase(&lms, n, 1 << 20) };
        // The univariate eliminant in x: ∏(x − f) (+ delta for tower and single;
        // for bits the variety is F + delta, so the F-point count is what moves).
        let mut pi = prod_linear(f, p);
        let fp_roots = match enc {
            Enc::Tower | Enc::Single => {
                pi[0] = addm(pi[0], delta, p);
                Some(roots_in_fp(&pi, p))
            }
            Enc::Bits => {
                let shifted: Vec<u64> = f.iter().map(|&x| addm(x, delta, p)).collect();
                Some(roots_in_fp(&prod_linear(&shifted, p), p))
            }
        };
        json!({"delta": delta, "F_points_satisfying": vanish, "quotient_dim": quot,
               "fp_points_of_variety": fp_roots, "timed_out": r.timed_out, "pairs_above_bound": r.pairs_above_bound})
    };
    let ok = check(0);
    let wrong = check(1);
    let size = 1usize << k;
    let pass_ok = ok["F_points_satisfying"] == json!(size) && ok["quotient_dim"] == json!(size) && ok["fp_points_of_variety"] == json!(size);
    let neg_detected = wrong["F_points_satisfying"] != json!(size);
    json!({"correct": ok, "wrong_closing": wrong, "encoding_exact": pass_ok, "negative_control_detected": neg_detected})
}

// ── CLI ────────────────────────────────────────────────────────────

struct Args {
    arm: String,
    k: usize,
    r: usize,
    draw: u64,
    seed: u64,
    random_targets: usize,
    planted: bool,
    budget: u64,
    hky: bool,
    hky_cmax: u32,
    runlog: bool,
    verify: bool,
    out: Option<String>,
}

fn parse() -> Args {
    let mut a = Args {
        arm: "tower-H".into(),
        k: 2,
        r: 0,
        draw: 0,
        seed: 20261118,
        random_targets: 2,
        planted: true,
        budget: 120,
        hky: false,
        hky_cmax: 14,
        runlog: true,
        verify: true,
        out: None,
    };
    let v: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < v.len() {
        let val = || v.get(i + 1).cloned().expect("flag needs a value");
        match v[i].as_str() {
            "--arm" => a.arm = val(),
            "--k" => a.k = val().parse().unwrap(),
            "--r" => a.r = val().parse().unwrap(),
            "--draw" => a.draw = val().parse().unwrap(),
            "--seed" => a.seed = val().parse().unwrap(),
            "--random-targets" => a.random_targets = val().parse().unwrap(),
            "--planted" => a.planted = val() == "1",
            "--budget" => a.budget = val().parse().unwrap(),
            "--hky" => a.hky = val() == "1",
            "--hky-cmax" => a.hky_cmax = val().parse().unwrap(),
            "--runlog" => a.runlog = val() == "1",
            "--verify" => a.verify = val() == "1",
            "--out" => a.out = Some(val()),
            f => panic!("unknown flag {f}"),
        }
        i += 2;
    }
    a
}

fn main() {
    let a = parse();
    let p = P1;
    let k = a.k;
    assert!(k >= 2, "k ≥ 2");
    let budget = Duration::from_secs(a.budget);
    // One curve for the whole study; random targets shared by every arm at k.
    let curve = Curve::random(p, &mut StdRng::seed_from_u64(derive(a.seed, &[tag("curve")])));
    let mut set_rng = StdRng::seed_from_u64(derive(a.seed, &[tag("set"), tag(&a.arm), k as u64, a.r as u64, a.draw]));
    let (f, set_info) = arm_set(&a.arm, k, a.r, &curve, &mut set_rng);
    let enc = arm_enc(&a.arm);
    {
        let mut s = f.clone();
        s.sort_unstable();
        s.dedup();
        assert_eq!(s.len(), 1 << k, "F has 2^k distinct elements");
    }
    let closing = closing_coeffs(&f, p);
    let closing_nonzero = closing.iter().filter(|&&c| c != 0).count();
    let closing_const = closing_nonzero <= 1 && closing.iter().skip(1).all(|&c| c == 0);
    let head = json!({
        "schema": "enc_control.row/v1", "p": p, "curve": {"a": curve.a, "b": curve.b},
        "arm": a.arm, "encoding": enc.name(), "k": k, "size": 1usize << k, "r": a.r, "draw": a.draw,
        "seed": a.seed, "set": set_info, "closing_terms_nonzero": closing_nonzero,
        "closing_is_constant": closing_const,
        "abscissae_in_F": f.iter().filter(|&&x| curve.is_abscissa(x)).count(),
    });
    let mut out: Box<dyn Write> = match &a.out {
        Some(path) => Box::new(std::fs::OpenOptions::new().create(true).append(true).open(path).unwrap()),
        None => Box::new(std::io::stdout()),
    };
    let mut emit = |mut row: Value, extra: Value| {
        for (key, val) in head.as_object().unwrap() {
            row[key] = val.clone();
        }
        for (key, val) in extra.as_object().unwrap() {
            row[key] = val.clone();
        }
        writeln!(out, "{}", row).unwrap();
        out.flush().unwrap();
    };
    if a.verify {
        let v = verify(enc, &f, k, p, budget);
        eprintln!("verify {} k={} r={} draw={}: exact={} neg_detected={}", a.arm, k, a.r, a.draw, v["encoding_exact"], v["negative_control_detected"]);
        assert_eq!(v["encoding_exact"], json!(true), "encoding is not exact: {v}");
        assert_eq!(v["negative_control_detected"], json!(true), "negative control not detected: {v}");
        emit(json!({"kind": "verify"}), v);
    }
    let bs = enc.block_size(k);
    let n = 2 * bs;
    let max_degree: u32 = match enc {
        Enc::Single => (1u32 << k) + 8,
        _ => (4 * k as u32 + 8).max(16),
    };
    let blocks: Vec<(Vec<Pol>, usize)> = (0..2).map(|i| block(enc, &f, k, n, i * bs, 0, p)).collect();
    let system = |x_r: u64| -> Vec<Pol> {
        let mut eqs = vec![s3(&Pol::var(n, blocks[0].1), &Pol::var(n, blocks[1].1), &Pol::constant(n, x_r, p), curve.a, curve.b, p)];
        for (b, _) in &blocks {
            eqs.extend(b.iter().cloned());
        }
        eqs
    };
    let mut targets: Vec<(String, u64, Option<Vec<u64>>)> = Vec::new();
    if a.planted {
        let mut prng = StdRng::seed_from_u64(derive(a.seed, &[tag("plant"), tag(&a.arm), k as u64, a.r as u64, a.draw]));
        let abs: Vec<usize> = (0..f.len()).filter(|&i| curve.is_abscissa(f[i])).collect();
        if abs.len() >= 2 {
            let i1 = abs[prng.gen_range(0..abs.len())];
            let i2 = loop {
                let j = abs[prng.gen_range(0..abs.len())];
                if j != i1 {
                    break j;
                }
            };
            let (x1, x2) = (f[i1], f[i2]);
            let mut y1 = sqrtm(curve.rhs(x1), p);
            let mut y2 = sqrtm(curve.rhs(x2), p);
            if prng.gen::<bool>() {
                y1 = subm(0, y1, p);
            }
            if prng.gen::<bool>() {
                y2 = subm(0, y2, p);
            }
            let x_r = curve.add_x(x1, y1, x2, y2);
            let mut sol = block_point(enc, x1, i1, k, p);
            sol.extend(block_point(enc, x2, i2, k, p));
            targets.push(("planted".into(), x_r, Some(sol)));
        }
    }
    for t in 0..a.random_targets {
        let mut trng = StdRng::seed_from_u64(derive(a.seed, &[tag("target"), k as u64, t as u64]));
        targets.push((format!("random{t}"), curve.random_abscissa(&mut trng, &[]), None));
    }
    for (name, x_r, sol) in &targets {
        let eqs = system(*x_r);
        let la = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
        let mut row = run_f4(&eqs, n, p, sol.as_deref(), max_degree, budget, a.runlog);
        row["kind"] = json!("f4");
        if a.hky && sol.is_none() && closing_const && enc == Enc::Tower {
            let h = run_hky(&eqs[0], k, closing[0], p, a.hky_cmax, budget);
            for (key, val) in h.as_object().unwrap() {
                row[key] = val.clone();
            }
        }
        eprintln!(
            "{:10} k={} r={:3} draw={} {:8} D={:3} reached={:3} incons={} ops={:>14} hky={} {:.2}s",
            a.arm, k, a.r, a.draw, name, row["D"], row["degree_reached"], row["inconsistent"], row["field_ops"],
            row.get("hky_last_fall").cloned().unwrap_or(Value::Null), row["wall_s"].as_f64().unwrap_or(0.0)
        );
        emit(row, json!({"target": name, "x_r": x_r, "loadavg_before": la.split_whitespace().next().unwrap_or("")}));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const P: u64 = P1;

    #[test]
    fn sqrt_and_roots() {
        for a in [4u64, 9, 31 * 31, 123456789] {
            let a2 = mulm(a, a, P);
            let r = sqrtm(a2, P);
            assert_eq!(mulm(r, r, P), a2);
        }
        let f = [3u64, 5, 7, 11];
        assert_eq!(roots_in_fp(&prod_linear(&f, P), P), 4);
    }

    #[test]
    fn subgroup_closing_is_one_and_coset_is_constant() {
        for k in 2..6 {
            let h = subgroup(k, P);
            let r = closing_coeffs(&h, P);
            assert_eq!(r[0], 1);
            assert!(r[1..].iter().all(|&c| c == 0));
            let g = 12345u64;
            let gh: Vec<u64> = h.iter().map(|&z| mulm(g, z, P)).collect();
            let r = closing_coeffs(&gh, P);
            assert_eq!(r[0], powm(g, 1 << k, P));
            assert!(r[1..].iter().all(|&c| c == 0));
        }
    }

    #[test]
    fn encodings_vanish_on_f_and_not_wrong() {
        let mut rng = StdRng::seed_from_u64(7);
        let curve = Curve::random(P, &mut rng);
        for k in 2..5 {
            let (f, _) = arm_set("tower-rand", k, 0, &curve, &mut rng);
            for enc in [Enc::Tower, Enc::Bits, Enc::Single] {
                let n = enc.block_size(k);
                let (eqs, _) = block(enc, &f, k, n, 0, 0, P);
                let (bad, _) = block(enc, &f, k, n, 0, 1, P);
                for (v, &fv) in f.iter().enumerate() {
                    let pt = block_point(enc, fv, v, k, P);
                    assert!(eqs.iter().all(|e| e.eval(&pt, P) == 0), "{enc:?}");
                    assert!(!bad.iter().all(|e| e.eval(&pt, P) == 0), "{enc:?}");
                }
                let v = verify(enc, &f, k, P, Duration::from_secs(30));
                assert_eq!(v["encoding_exact"], json!(true), "{v}");
                assert_eq!(v["negative_control_detected"], json!(true), "{v}");
            }
        }
    }

    #[test]
    fn interpolant_reproduces_values() {
        let f = [10u64, 20, 33, 47, 52, 61, 70, 88];
        let c = multilinear_interpolant(&f, 3, P);
        for v in 0..8usize {
            let mut s = 0;
            for (sset, &cs) in c.iter().enumerate() {
                if sset & !v == 0 {
                    s = addm(s, cs, P);
                }
            }
            assert_eq!(s, f[v]);
        }
    }

    #[test]
    fn planted_sum_satisfies_s3() {
        let mut rng = StdRng::seed_from_u64(3);
        let curve = Curve::random(P, &mut rng);
        let x1 = curve.random_abscissa(&mut rng, &[]);
        let x2 = curve.random_abscissa(&mut rng, &[x1]);
        let (y1, y2) = (sqrtm(curve.rhs(x1), P), sqrtm(curve.rhs(x2), P));
        let x3 = curve.add_x(x1, y1, x2, y2);
        let e = s3(&Pol::var(3, 0), &Pol::var(3, 1), &Pol::var(3, 2), curve.a, curve.b, P);
        assert_eq!(e.eval(&[x1, x2, x3], P), 0);
    }
}
