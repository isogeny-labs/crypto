//! Item 20 of the 2026-10-18 blind-spot campaign: classical pre- and
//! post-processing for quantum ECDLP (research/blindspots_20261018/
//! 20-quantum-classical-processing/PROTOCOL.md).
//!
//! Standalone, std only; the resource model is the dependency-free
//! `prime_shor` module of `src/cryptanalysis/quantum_estimator.rs`, included
//! below with `#[path]` because the crate itself does not build at the branch
//! base.  Build:
//!
//! ```text
//! rustc --edition 2021 -C opt-level=3 -C codegen-units=1 \
//!     -o target/blindspots-20/b20 examples/blindspots_20_quantum_classical.rs
//! ```
//!
//! Modes: `selftest`, `single`, `lattice`, `model`.  Every record is one JSON
//! line on stdout.

#[path = "../src/cryptanalysis/quantum_estimator.rs"]
#[allow(dead_code)]
mod quantum_estimator;

use quantum_estimator::prime_shor as ps;
use std::collections::HashMap;

// ───────────────────────────── small utilities ─────────────────────────────

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed ^ 0x9E37_79B9_7F4A_7C15)
    }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn below(&mut self, n: u64) -> u64 {
        // n is far below 2^64, so the modulo bias is negligible (< 2^-40)
        self.next_u64() % n
    }
}

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

/// Inverse of `a` modulo `m` (`gcd = 1`), as a value in `[0, m)`.
fn inv_mod(a: u64, m: u64) -> Option<u64> {
    let (mut r0, mut r1) = (m as i128, (a % m) as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    if r0 != 1 {
        return None;
    }
    Some(t0.rem_euclid(m as i128) as u64)
}

fn largest_prime_below_pow2(m: u32) -> u64 {
    let mut n = (1u64 << m) - 1;
    while !is_prime(n) {
        n -= 1;
    }
    n
}

fn smallest_prime_above_half(m: u32) -> u64 {
    let mut n = (1u64 << (m - 1)) + 1;
    while !is_prime(n) {
        n += 1;
    }
    n
}

fn wilson(k: u64, n: u64) -> (f64, f64) {
    if n == 0 {
        return (0.0, 1.0);
    }
    let z = 1.96f64;
    let nf = n as f64;
    let p = k as f64 / nf;
    let denom = 1.0 + z * z / nf;
    let center = (p + z * z / (2.0 * nf)) / denom;
    let half = z * (p * (1.0 - p) / nf + z * z / (4.0 * nf * nf)).sqrt() / denom;
    ((center - half).max(0.0), (center + half).min(1.0))
}

fn jf(x: f64) -> String {
    if x.is_finite() {
        format!("{x}")
    } else {
        "null".to_string()
    }
}

fn lg(x: f64) -> f64 {
    if x > 0.0 {
        x.log2()
    } else {
        f64::NEG_INFINITY
    }
}

fn parse_list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split(',')
        .filter(|t| !t.is_empty())
        .map(|t| t.trim().parse::<T>().unwrap_or_else(|_| panic!("bad list item {t:?}")))
        .collect()
}

struct Args(HashMap<String, String>);

impl Args {
    fn from_env() -> Self {
        let v: Vec<String> = std::env::args().skip(1).collect();
        let mut m = HashMap::new();
        let mut i = 0;
        while i < v.len() {
            let k = v[i].trim_start_matches("--").to_string();
            let val = v.get(i + 1).cloned().unwrap_or_default();
            m.insert(k, val);
            i += 2;
        }
        Args(m)
    }
    fn str(&self, k: &str, d: &str) -> String {
        self.0.get(k).cloned().unwrap_or_else(|| d.to_string())
    }
    fn u64(&self, k: &str, d: u64) -> u64 {
        self.0.get(k).map(|s| s.parse().expect("integer argument")).unwrap_or(d)
    }
}

// ─────────────────────────────── complex and FFT ───────────────────────────

#[derive(Clone, Copy, Debug)]
struct C {
    re: f64,
    im: f64,
}

impl C {
    fn mul(self, o: C) -> C {
        C { re: self.re * o.re - self.im * o.im, im: self.re * o.im + self.im * o.re }
    }
    fn cis(theta: f64) -> C {
        C { re: theta.cos(), im: theta.sin() }
    }
}

fn fft(a: &mut [C]) {
    let n = a.len();
    assert!(n.is_power_of_two());
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let ang = 2.0 * std::f64::consts::PI / len as f64;
        let wl = C::cis(ang);
        for start in (0..n).step_by(len) {
            let mut w = C { re: 1.0, im: 0.0 };
            for k in 0..len / 2 {
                let u = a[start + k];
                let v = a[start + k + len / 2].mul(w);
                a[start + k] = C { re: u.re + v.re, im: u.im + v.im };
                a[start + k + len / 2] = C { re: u.re - v.re, im: u.im - v.im };
                // re-seed the twiddle from the exact angle every 64 steps to
                // stop drift
                w = if (k + 1) % 64 == 0 {
                    C::cis(ang * (k + 1) as f64)
                } else {
                    w.mul(wl)
                };
            }
        }
        len <<= 1;
    }
}

// ─────────────────────── exact quantum-measurement sampler ──────────────────

/// Parameters of one run: group order `r` (odd prime), exponent registers of
/// `m + sigma` and `ell` qubits.
#[derive(Clone, Debug)]
struct Params {
    m: u32,
    sigma: u32,
    ell: u32,
    r: u64,
    mm: u64,   // M = 2^(m+sigma)
    ll: u64,   // L = 2^ell
    rho: u64,  // M mod r
    t_lo: u64, // floor(M / r)
    rinv: u64, // r^-1 mod M
}

impl Params {
    fn new(m: u32, sigma: u32, ell: u32, r: u64) -> Params {
        assert!(r % 2 == 1 && ell >= 1 && ell <= m + sigma && m + sigma <= 26);
        let mm = 1u64 << (m + sigma);
        Params {
            m,
            sigma,
            ell,
            r,
            mm,
            ll: 1u64 << ell,
            rho: mm % r,
            t_lo: mm / r,
            rinv: inv_mod(r % mm, mm).expect("r odd"),
        }
    }
}

/// `F_T(u) = sin^2(pi T u / M) / sin^2(pi u / M)`, `T^2` at `u = 0`.
fn f_t(t: u64, u: u64, mm: u64) -> f64 {
    if u == 0 {
        return (t as f64) * (t as f64);
    }
    let x = ((t as u128 * u as u128) % (2 * mm as u128)) as f64;
    let num = (std::f64::consts::PI * x / mm as f64).sin();
    let den = (std::f64::consts::PI * u as f64 / mm as f64).sin();
    (num * num) / (den * den)
}

/// Complex geometric sum `sum_{t<T} exp(2 pi i u t / M)` up to the phase that
/// is carried by `a0 * j`: `exp(i pi u (T-1) / M) sin(pi T u / M)/sin(pi u/M)`.
fn s_t(t: u64, u: u64, mm: u64) -> C {
    if u == 0 {
        return C { re: t as f64, im: 0.0 };
    }
    let x = ((t as u128 * u as u128) % (2 * mm as u128)) as f64;
    let amp = (std::f64::consts::PI * x / mm as f64).sin()
        / (std::f64::consts::PI * u as f64 / mm as f64).sin();
    let ph = ((u as u128 * (t - 1) as u128) % (2 * mm as u128)) as f64;
    let c = C::cis(std::f64::consts::PI * ph / mm as f64);
    C { re: c.re * amp, im: c.im * amp }
}

/// Cumulative marginal over `u = r j mod M` (not normalised); the last entry
/// is the total.
fn build_marginal(p: &Params) -> Vec<f64> {
    let mut cdf = Vec::with_capacity(p.mm as usize);
    let mut acc = 0.0f64;
    let (t_lo, t_hi) = (p.t_lo, p.t_lo + 1);
    let mmf = p.mm as f64;
    for u in 0..p.mm {
        let v = if u == 0 {
            p.rho as f64 * (t_hi as f64).powi(2) + (p.r - p.rho) as f64 * (t_lo as f64).powi(2)
        } else {
            let den = (std::f64::consts::PI * u as f64 / mmf).sin();
            let den2 = den * den;
            let xh = ((t_hi as u128 * u as u128) % (2 * p.mm as u128)) as f64;
            let xl = ((t_lo as u128 * u as u128) % (2 * p.mm as u128)) as f64;
            let nh = (std::f64::consts::PI * xh / mmf).sin();
            let nl = (std::f64::consts::PI * xl / mmf).sin();
            (p.rho as f64 * nh * nh + (p.r - p.rho) as f64 * nl * nl) / den2
        };
        acc += v;
        cdf.push(acc);
    }
    cdf
}

/// Normalised marginal probabilities `P(u)` (selftest only).
fn marginal_probs(p: &Params) -> Vec<f64> {
    let cdf = build_marginal(p);
    let tot = *cdf.last().unwrap();
    let mut out = Vec::with_capacity(cdf.len());
    let mut prev = 0.0;
    for c in cdf {
        out.push((c - prev) / tot);
        prev = c;
    }
    out
}

/// For a log `d`: `n_hi(e) = #{b < L : (e + b d) mod r < rho}`.
struct Inst {
    d: u64,
    nhi: Vec<u32>,
}

fn make_inst(p: &Params, d: u64) -> Inst {
    let r = p.r as usize;
    let mut diff = vec![0i32; r + 1];
    let mut bd = 0u64; // b*d mod r
    for _b in 0..p.ll {
        let s0 = ((p.r - bd) % p.r) as usize;
        let end = s0 + p.rho as usize;
        if end <= r {
            diff[s0] += 1;
            diff[end] -= 1;
        } else {
            diff[s0] += 1;
            diff[r] -= 1;
            diff[0] += 1;
            diff[end - r] -= 1;
        }
        bd = (bd + d) % p.r;
    }
    let mut nhi = Vec::with_capacity(r);
    let mut run = 0i32;
    for item in diff.iter().take(r) {
        run += item;
        nhi.push(run as u32);
    }
    Inst { d, nhi }
}

/// `P(e | j)` for the third-register value (normalised).
fn cond_e_probs(p: &Params, inst: &Inst, u: u64) -> Vec<f64> {
    let (fh, fl) = (f_t(p.t_lo + 1, u, p.mm), f_t(p.t_lo, u, p.mm));
    let l = p.ll as f64;
    let mut w: Vec<f64> =
        inst.nhi.iter().map(|&n| n as f64 * fh + (l - n as f64) * fl).collect();
    let tot: f64 = w.iter().sum();
    for x in w.iter_mut() {
        *x /= tot;
    }
    w
}

/// `P(k | e, j)`: squared magnitude of the FFT over `b`, normalised.
fn cond_k_probs(p: &Params, inst: &Inst, e: u64, u: u64, j: u64) -> Vec<f64> {
    let sc = [s_t(p.t_lo, u, p.mm), s_t(p.t_lo + 1, u, p.mm)];
    let mut v = Vec::with_capacity(p.ll as usize);
    let mut a0 = e % p.r;
    let two_pi = 2.0 * std::f64::consts::PI;
    for _b in 0..p.ll {
        let hi = (a0 < p.rho) as usize;
        let ph = ((a0 as u128 * j as u128) % p.mm as u128) as f64;
        let w = C::cis(two_pi * ph / p.mm as f64);
        v.push(w.mul(sc[hi]));
        a0 = (a0 + inst.d) % p.r;
    }
    // An all-zero vector cannot occur: e has at least one preimage.
    fft(&mut v);
    let mut pr: Vec<f64> = v.iter().map(|c| c.re * c.re + c.im * c.im).collect();
    let tot: f64 = pr.iter().sum();
    if tot <= 0.0 {
        return vec![1.0 / p.ll as f64; p.ll as usize];
    }
    for x in pr.iter_mut() {
        *x /= tot;
    }
    pr
}

fn sample_index(probs: &[f64], rng: &mut Rng) -> usize {
    let x = rng.f64();
    let mut acc = 0.0;
    for (i, &q) in probs.iter().enumerate() {
        acc += q;
        if x < acc {
            return i;
        }
    }
    probs.len() - 1
}

fn sample_run(p: &Params, marg: &[f64], inst: &Inst, rng: &mut Rng) -> (u64, u64) {
    let tot = *marg.last().unwrap();
    let x = rng.f64() * tot;
    let u = marg.partition_point(|&c| c <= x).min(marg.len() - 1) as u64;
    let j = mulmod(u, p.rinv, p.mm);
    let pe = cond_e_probs(p, inst, u);
    let e = sample_index(&pe, rng) as u64;
    let pk = cond_k_probs(p, inst, e, u, j);
    let k = sample_index(&pk, rng) as u64;
    (j, k)
}

// ───────────────────────────── counted toy group ───────────────────────────

struct Cg {
    r: u64,
    adds: u64,
    native: u64,
}

impl Cg {
    fn new(r: u64) -> Cg {
        Cg { r, adds: 0, native: 0 }
    }
    fn add(&mut self, a: u64, b: u64) -> u64 {
        self.adds += 1;
        let s = a + b;
        if s >= self.r {
            s - self.r
        } else {
            s
        }
    }
    fn neg(&self, a: u64) -> u64 {
        (self.r - a) % self.r
    }
    /// Left-to-right double-and-add; every doubling and addition is counted.
    fn mul(&mut self, k: u64, a: u64) -> u64 {
        if k == 0 {
            return 0;
        }
        let top = 63 - k.leading_zeros();
        let mut res = a;
        for i in (0..top).rev() {
            res = self.add(res, res);
            if (k >> i) & 1 == 1 {
                res = self.add(res, a);
            }
        }
        res
    }
}

/// Baby-step giant-step context for `t in [-bt, bt]` in base `g`.
struct Bsgs {
    q: u64,
    table: HashMap<u64, u64>,
    gneg_q: u64,
    gbt: u64,
    bt: u64,
}

fn bsgs_build(cg: &mut Cg, g: u64, bt: u64) -> Bsgs {
    let w = 2 * bt + 1;
    let q = ((w as f64).sqrt().ceil() as u64).max(1);
    let mut table = HashMap::with_capacity(q as usize);
    let mut cur = 0u64;
    table.insert(0u64, 0u64);
    cg.native += 1;
    for i in 1..q {
        cur = cg.add(cur, g);
        table.entry(cur).or_insert(i);
        cg.native += 1;
    }
    let gq = cg.add(cur, g);
    let gneg_q = cg.neg(gq);
    let gbt = cg.mul(bt, g);
    Bsgs { q, table, gneg_q, gbt, bt }
}

fn bsgs_find(cg: &mut Cg, ctx: &Bsgs, target: u64) -> Option<i64> {
    let w = 2 * ctx.bt + 1;
    let giants = w.div_ceil(ctx.q);
    let mut cur = cg.add(target, ctx.gbt); // [t + bt] g
    for a in 0..giants {
        cg.native += 1;
        if let Some(&i) = ctx.table.get(&cur) {
            let tp = a * ctx.q + i;
            if tp < w {
                return Some(tp as i64 - ctx.bt as i64);
            }
        }
        cur = cg.add(cur, ctx.gneg_q);
    }
    None
}

fn nearest_div(num: u128, den: u128) -> u128 {
    (2 * num + den) / (2 * den)
}

/// `(z, kappa)` of a measured pair: `z = round(r j / M)`, `kappa = round(r k / L)`.
fn zk(p: &Params, j: u64, k: u64) -> (i128, i128) {
    let z = nearest_div(p.r as u128 * j as u128, p.mm as u128) as i128;
    let kap = nearest_div(p.r as u128 * k as u128, p.ll as u128) as i128;
    (z, kap)
}

fn centered(x: i128, r: i128) -> i128 {
    let v = x.rem_euclid(r);
    if v > r / 2 {
        v - r
    } else {
        v
    }
}

/// `|t_eta|` for `eta in -2..=2` using the true `d` (infinite when `z + eta`
/// is not invertible).  Index `eta + 2`.
fn t_abs(p: &Params, d: u64, j: u64, k: u64) -> [f64; 5] {
    let (z, kap) = zk(p, j, k);
    let r = p.r as i128;
    let mut out = [f64::INFINITY; 5];
    for (idx, eta) in (-2i128..=2).enumerate() {
        let c = (z + eta).rem_euclid(r);
        if c == 0 {
            continue;
        }
        let t = centered(d as i128 * c + kap, r);
        out[idx] = t.unsigned_abs() as f64;
    }
    out
}

fn criterion(tabs: &[f64; 5], b_eta: usize, bt: f64) -> bool {
    (2 - b_eta..=2 + b_eta).any(|i| tabs[i] <= bt)
}

fn bt_of(p: &Params, b_delta: u64) -> u64 {
    // floor((B_Delta + 1/2) r / 2^ell)
    ((2 * b_delta + 1) as u128 * p.r as u128 / (2 * p.ll as u128)) as u64
}

/// The actual limited search: eta in 0, +1, -1, ..., BSGS for t, candidate
/// accepted only if `[d] g = x`.  Returns the recovered `d`.
fn search_single(
    cg: &mut Cg,
    ctx: &Bsgs,
    p: &Params,
    g: u64,
    x: u64,
    j: u64,
    k: u64,
    b_eta: usize,
) -> Option<u64> {
    let (z, kap) = zk(p, j, k);
    let r = p.r as i128;
    let kap_m = kap.rem_euclid(r) as u64;
    let mut etas = vec![0i128];
    for e in 1..=b_eta as i128 {
        etas.push(e);
        etas.push(-e);
    }
    for eta in etas {
        let c = (z + eta).rem_euclid(r) as u64;
        if c == 0 {
            continue;
        }
        let xc = cg.mul(c, x);
        let kg = cg.mul(kap_m, g);
        let target = cg.add(xc, kg);
        if let Some(t) = bsgs_find(cg, ctx, target) {
            cg.native += 1; // modular inverse
            let cinv = inv_mod(c, p.r).unwrap();
            let num = (t as i128 - kap).rem_euclid(r) as u64;
            let d = mulmod(num, cinv, p.r);
            let chk = cg.mul(d, g);
            if chk == x {
                return Some(d);
            }
        }
    }
    None
}

// ───────────────────────────────── lattice ─────────────────────────────────

fn dot_f(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn gram_schmidt(b: &[Vec<i128>], flops: &mut u64) -> (Vec<f64>, Vec<Vec<f64>>) {
    let d = b.len();
    let bf: Vec<Vec<f64>> = b.iter().map(|r| r.iter().map(|&x| x as f64).collect()).collect();
    let mut bs: Vec<Vec<f64>> = Vec::with_capacity(d);
    let mut norm = vec![0.0; d];
    let mut mu = vec![vec![0.0; d]; d];
    for i in 0..d {
        let mut v = bf[i].clone();
        for j in 0..i {
            mu[i][j] = dot_f(&bf[i], &bs[j]) / norm[j];
            for (vv, sv) in v.iter_mut().zip(&bs[j]) {
                *vv -= mu[i][j] * sv;
            }
        }
        norm[i] = dot_f(&v, &v);
        bs.push(v);
    }
    *flops += (d * d * d) as u64;
    (norm, mu)
}

/// LLL with delta = 0.99 on an integer row basis; GS is recomputed after each
/// change (dimension is at most 7).
fn lll(b: &mut [Vec<i128>], flops: &mut u64, reductions: &mut u64) {
    let d = b.len();
    let delta = 0.99;
    let (mut norm, mut mu) = gram_schmidt(b, flops);
    let mut k = 1;
    let mut guard = 0u32;
    while k < d && guard < 200_000 {
        guard += 1;
        for j in (0..k).rev() {
            let q = mu[k][j].round() as i128;
            if q != 0 {
                let bj = b[j].clone();
                for (x, y) in b[k].iter_mut().zip(&bj) {
                    *x -= q * y;
                }
                (norm, mu) = gram_schmidt(b, flops);
            }
        }
        if norm[k] >= (delta - mu[k][k - 1] * mu[k][k - 1]) * norm[k - 1] {
            k += 1;
        } else {
            b.swap(k, k - 1);
            (norm, mu) = gram_schmidt(b, flops);
            k = (k - 1).max(1);
        }
    }
    *reductions += 1;
}

/// Babai nearest plane on the (reduced) basis; returns the lattice point
/// closest to `v` found, as `v - residual`.
fn babai(b: &[Vec<i128>], v: &[i128], flops: &mut u64) -> Vec<i128> {
    let d = b.len();
    let (norm, _mu) = gram_schmidt(b, flops);
    // recompute b* as vectors for projections
    let bf: Vec<Vec<f64>> = b.iter().map(|r| r.iter().map(|&x| x as f64).collect()).collect();
    let mut bs: Vec<Vec<f64>> = Vec::with_capacity(d);
    for i in 0..d {
        let mut w = bf[i].clone();
        for j in 0..i {
            let m = dot_f(&bf[i], &bs[j]) / norm[j];
            for (x, y) in w.iter_mut().zip(&bs[j]) {
                *x -= m * y;
            }
        }
        bs.push(w);
    }
    let mut t: Vec<i128> = v.to_vec();
    for i in (0..d).rev() {
        let tf: Vec<f64> = t.iter().map(|&x| x as f64).collect();
        let c = (dot_f(&tf, &bs[i]) / norm[i]).round() as i128;
        if c != 0 {
            for (x, y) in t.iter_mut().zip(&b[i]) {
                *x -= c * y;
            }
        }
    }
    v.iter().zip(&t).map(|(a, r)| a - r).collect()
}

struct LatCount {
    reductions: u64,
    flops: u64,
    candidates: u64,
    group_adds: u64,
}

/// Lattice attempt on a set of runs `(z_i, kappa_i)`.  Returns `d` when a
/// candidate passes the consistency filter and `[d] g = x`.
fn lattice_attempt(
    runs: &[(i128, i128)],
    r: u64,
    ell: u32,
    g: u64,
    x: u64,
    cnt: &mut LatCount,
) -> Option<u64> {
    let n = runs.len();
    let d = n + 1;
    let rr = r as i128;
    let w = (r as u128).div_ceil(1u128 << ell).max(1) as i128;
    let mut basis = vec![vec![0i128; d]; d];
    for i in 0..n {
        basis[i][i] = rr * rr;
    }
    for (i, &(z, _)) in runs.iter().enumerate() {
        basis[n][i] = z.rem_euclid(rr) * rr;
    }
    basis[n][n] = w;
    let mut v = vec![0i128; d];
    for (i, &(_, kap)) in runs.iter().enumerate() {
        v[i] = -(kap.rem_euclid(rr)) * rr;
    }
    lll(&mut basis, &mut cnt.flops, &mut cnt.reductions);
    let center = babai(&basis, &v, &mut cnt.flops);
    let tol = 6 * w + 3;
    // offsets in {-1,0,1}^d, the zero offset first
    let total = 3usize.pow(d as u32);
    let zero_idx = total / 2;
    for idx in 0..total {
        let id = if idx == 0 {
            zero_idx
        } else if idx == zero_idx {
            0
        } else {
            idx
        };
        let mut offs = vec![0i128; d];
        let mut t2 = id;
        for o in offs.iter_mut() {
            *o = (t2 % 3) as i128 - 1;
            t2 /= 3;
        }
        let mut p = center.clone();
        for (i, &c) in offs.iter().enumerate() {
            if c != 0 {
                for (pp, bb) in p.iter_mut().zip(&basis[i]) {
                    *pp += c * bb;
                }
            }
        }
        cnt.flops += d as u64;
        cnt.candidates += 1;
        let last = p[n];
        if last <= 0 || last % w != 0 {
            continue;
        }
        let dd = last / w;
        if dd <= 0 || dd >= rr {
            continue;
        }
        let ok = runs.iter().all(|&(z, kap)| {
            centered(dd * z.rem_euclid(rr) + kap, rr).abs() <= tol
        });
        if !ok {
            continue;
        }
        let mut cg = Cg::new(r);
        let chk = cg.mul(dd as u64, g);
        cnt.group_adds += cg.adds;
        if chk == x {
            return Some(dd as u64);
        }
    }
    None
}

// ───────────────────────────── single-run driver ───────────────────────────

fn rshape_prime(m: u32, shape: &str) -> u64 {
    if shape == "bottom" {
        smallest_prime_above_half(m)
    } else {
        largest_prime_below_pow2(m)
    }
}

struct SingleResult {
    n: u64,
    /// success counts per (b_eta index, b_delta index)
    succ: Vec<Vec<u64>>,
    null: Vec<Vec<u64>>,
    p: Params,
}

const B_ETAS: [usize; 3] = [0, 1, 2];

#[allow(clippy::too_many_arguments)]
fn run_single_cell(
    m: u32,
    sigma: u32,
    ell: u32,
    r: u64,
    samples: u64,
    b_deltas: &[u64],
    rng: &mut Rng,
) -> SingleResult {
    let p = Params::new(m, sigma, ell, r);
    let marg = build_marginal(&p);
    let bts: Vec<f64> = b_deltas.iter().map(|&bd| bt_of(&p, bd) as f64).collect();
    let mut succ = vec![vec![0u64; b_deltas.len()]; B_ETAS.len()];
    let mut null = vec![vec![0u64; b_deltas.len()]; B_ETAS.len()];
    let mut inst = make_inst(&p, 1 + rng.below(r - 1));
    for i in 0..samples {
        if i % 20 == 0 && i > 0 {
            inst = make_inst(&p, 1 + rng.below(r - 1));
        }
        let (j, k) = sample_run(&p, &marg, &inst, rng);
        let tabs = t_abs(&p, inst.d, j, k);
        let (jn, kn) = (rng.below(p.mm), rng.below(p.ll));
        let tnull = t_abs(&p, inst.d, jn, kn);
        for (ie, &be) in B_ETAS.iter().enumerate() {
            for (id, &bt) in bts.iter().enumerate() {
                if criterion(&tabs, be, bt) {
                    succ[ie][id] += 1;
                }
                if criterion(&tnull, be, bt) {
                    null[ie][id] += 1;
                }
            }
        }
    }
    SingleResult { n: samples, succ, null, p }
}

/// Counted cost of the actual search on `verify` runs at one box; returns
/// (mean adds, mean native, successes, mismatches vs the closed criterion,
/// mean formula adds, mean formula native).
fn verify_cost(
    p: &Params,
    marg: &[f64],
    b_eta: usize,
    b_delta: u64,
    verify: u64,
    rng: &mut Rng,
) -> (f64, f64, u64, u64, f64, f64) {
    let bt = bt_of(p, b_delta);
    let (fg, fnat) = ps::single_run_search_work(p.m as usize, bt as f64, b_eta);
    let mut adds = 0u64;
    let mut native = 0u64;
    let mut succ = 0u64;
    let mut mism = 0u64;
    for _ in 0..verify {
        let d = 1 + rng.below(p.r - 1);
        let g = 1 + rng.below(p.r - 1);
        let mut setup = Cg::new(p.r);
        let x = setup.mul(d, g);
        let inst = make_inst(p, d);
        let (j, k) = sample_run(p, marg, &inst, rng);
        let tabs = t_abs(p, d, j, k);
        let crit = criterion(&tabs, b_eta, bt as f64);
        let mut cg = Cg::new(p.r);
        let ctx = bsgs_build(&mut cg, g, bt);
        let got = search_single(&mut cg, &ctx, p, g, x, j, k, b_eta);
        adds += cg.adds;
        native += cg.native;
        let ok = got == Some(d);
        if ok {
            succ += 1;
        }
        if ok != crit {
            mism += 1;
        }
    }
    let v = verify.max(1) as f64;
    (adds as f64 / v, native as f64 / v, succ, mism, fg, fnat)
}

fn mode_single(a: &Args) {
    let m = a.u64("m", 12) as u32;
    let sigmas: Vec<u32> = parse_list(&a.str("sigma", "0,7"));
    let nus: Vec<i64> = parse_list(&a.str("nu", "0"));
    let samples = a.u64("samples", 1000);
    let verify = a.u64("verify", 100);
    let seed = a.u64("seed", 1);
    let shape = a.str("rshape", "top");
    let b_deltas: Vec<u64> = parse_list(&a.str("beta-delta", "0,1,2,4,10,30,100"));
    let r = rshape_prime(m, &shape);
    let mut rng = Rng::new(seed ^ ((m as u64) << 32));
    let mut null_gate_ok = true;
    // (sigma, nu) -> success at (b_eta=0, each b_delta) for the monotone gate
    let mut table: Vec<(u32, i64, Vec<(u64, u64)>)> = Vec::new();
    for &nu in &nus {
        for &sigma in &sigmas {
            let ell = (m as i64 + nu) as u32;
            let t0 = std::time::Instant::now();
            let res = run_single_cell(m, sigma, ell, r, samples, &b_deltas, &mut rng);
            let p = &res.p;
            for (ie, &be) in B_ETAS.iter().enumerate() {
                for (id, &bd) in b_deltas.iter().enumerate() {
                    let bt = bt_of(p, bd);
                    let k = res.succ[ie][id];
                    let (lo, hi) = wilson(k, res.n);
                    let chance =
                        (((2 * be + 1) as f64) * ((2 * bt + 1) as f64) / r as f64).min(1.0);
                    let nk = res.null[ie][id];
                    let nrate = nk as f64 / res.n as f64;
                    let se = (chance * (1.0 - chance) / res.n as f64)
                        .sqrt()
                        .max(1.0 / res.n as f64);
                    let null_ok = (nrate - chance).abs() <= 4.0 * se;
                    if !null_ok {
                        null_gate_ok = false;
                    }
                    println!(
                        "{{\"kind\":\"single_cell\",\"m\":{m},\"r\":{r},\"rshape\":\"{shape}\",\"sigma\":{sigma},\"nu\":{nu},\"ell\":{ell},\"b_eta\":{be},\"b_delta\":{bd},\"bt\":{bt},\"runs\":{},\"success\":{k},\"rate\":{},\"ci_lo\":{},\"ci_hi\":{},\"null_success\":{nk},\"null_rate\":{},\"chance\":{},\"null_ok\":{null_ok},\"quantum_ops\":{}}}",
                        res.n,
                        jf(k as f64 / res.n as f64),
                        jf(lo),
                        jf(hi),
                        jf(nrate),
                        jf(chance),
                        m + sigma + ell
                    );
                    if be == 0 {
                        if let Some(row) = table.iter_mut().find(|t| t.0 == sigma && t.1 == nu) {
                            row.2.push((k, res.n));
                        } else {
                            table.push((sigma, nu, vec![(k, res.n)]));
                        }
                    }
                }
            }
            // cost of the real search
            if verify > 0 {
                let marg = build_marginal(p);
                for &(be, bd) in &[(0usize, 100u64), (1usize, 30u64)] {
                    if bd > (p.ll / 2).saturating_sub(1) {
                        continue;
                    }
                    let (ma, mn, vs, mism, fg, fnat) =
                        verify_cost(p, &marg, be, bd, verify, &mut rng);
                    println!(
                        "{{\"kind\":\"single_cost\",\"m\":{m},\"sigma\":{sigma},\"nu\":{nu},\"ell\":{ell},\"b_eta\":{be},\"b_delta\":{bd},\"bt\":{},\"verified_runs\":{verify},\"search_successes\":{vs},\"criterion_mismatches\":{mism},\"mean_group_adds\":{},\"mean_native\":{},\"formula_group_adds\":{},\"formula_native\":{},\"ratio_adds\":{},\"ratio_native\":{}}}",
                        bt_of(p, bd),
                        jf(ma),
                        jf(mn),
                        jf(fg),
                        jf(fnat),
                        jf(ma / fg),
                        jf(mn / fnat)
                    );
                }
            }
            eprintln!(
                "single m={m} sigma={sigma} nu={nu} samples={samples} {:.1}s",
                t0.elapsed().as_secs_f64()
            );
        }
    }
    // monotone gate in sigma (fixed nu, b_delta) and in b_delta (fixed sigma, nu)
    let mut mono_ok = true;
    for &nu in &nus {
        let mut rows: Vec<&(u32, i64, Vec<(u64, u64)>)> =
            table.iter().filter(|t| t.1 == nu).collect();
        rows.sort_by_key(|t| t.0);
        for w in rows.windows(2) {
            for (a, b) in w[0].2.iter().zip(&w[1].2) {
                let (pa, pb) = (a.0 as f64 / a.1 as f64, b.0 as f64 / b.1 as f64);
                let se = (0.25 / a.1 as f64 + 0.25 / b.1 as f64).sqrt();
                if pb + 2.0 * se < pa {
                    mono_ok = false;
                }
            }
        }
        for row in &rows {
            for w in row.2.windows(2) {
                let (pa, pb) = (w[0].0 as f64 / w[0].1 as f64, w[1].0 as f64 / w[1].1 as f64);
                let se = (0.25 / w[0].1 as f64 + 0.25 / w[1].1 as f64).sqrt();
                if pb + 2.0 * se < pa {
                    mono_ok = false;
                }
            }
        }
    }
    // positive-control thresholds (only meaningful at m = 16, nu = 0)
    let mut pc = String::from("\"na\"");
    if m == 16 && shape == "top" {
        let get = |sg: u32, ix: usize| -> Option<(u64, u64)> {
            table
                .iter()
                .find(|t| t.0 == sg && t.1 == 0)
                .and_then(|t| t.2.get(ix).copied())
        };
        let i100 = b_deltas.iter().position(|&x| x == 100);
        let i0 = b_deltas.iter().position(|&x| x == 0);
        if let (Some(i100), Some(i0)) = (i100, i0) {
            if let (Some(hi), Some(lo)) = (get(7, i100), get(0, i0)) {
                let top_ok = wilson(hi.0, hi.1).0 >= 0.97;
                let sr = lo.0 as f64 / lo.1 as f64;
                let shor_ok = (0.40..=0.90).contains(&sr);
                pc = format!("{}", top_ok && shor_ok);
            }
        }
    }
    println!(
        "{{\"kind\":\"single_gates\",\"m\":{m},\"rshape\":\"{shape}\",\"null_gate_ok\":{null_gate_ok},\"monotone_gate_ok\":{mono_ok},\"positive_control_ok\":{pc}}}"
    );
}

// ───────────────────────────── lattice driver ──────────────────────────────

struct LatOutcome {
    cum_success: Vec<(u64, u64)>, // for n = s..=nmax: (successes, trials)
    mean_flops: f64,
    mean_reductions: f64,
    mean_candidates: f64,
    mean_group_adds: f64,
    mean_runs: f64,
}

fn run_lattice(
    m: u32,
    sigma: u32,
    s: u32,
    extra: u32,
    trials: u64,
    shape: &str,
    rng: &mut Rng,
) -> (LatOutcome, Params) {
    let r = rshape_prime(m, shape);
    let ell = m.div_ceil(s);
    let p = Params::new(m, sigma, ell, r);
    let marg = build_marginal(&p);
    let nmax = (s + extra) as usize;
    let mut cum = vec![0u64; (extra + 1) as usize];
    let (mut fl, mut red, mut cand, mut gadd, mut runs_used) = (0u64, 0u64, 0u64, 0u64, 0u64);
    for _ in 0..trials {
        let d = 1 + rng.below(r - 1);
        let g = 1 + rng.below(r - 1);
        let mut setup = Cg::new(r);
        let x = setup.mul(d, g);
        let inst = make_inst(&p, d);
        let mut runs: Vec<(i128, i128)> = Vec::new();
        let mut solved_at: Option<usize> = None;
        let mut cnt = LatCount { reductions: 0, flops: 0, candidates: 0, group_adds: 0 };
        for n in 1..=nmax {
            let (j, k) = sample_run(&p, &marg, &inst, rng);
            runs.push(zk(&p, j, k));
            if n < s as usize || solved_at.is_some() {
                continue;
            }
            // all subsets containing the newest run, of size >= s
            let prev = n - 1;
            'subsets: for mask in 0u32..(1u32 << prev) {
                let size = mask.count_ones() as usize + 1;
                if size < s as usize {
                    continue;
                }
                let mut sub = Vec::with_capacity(size);
                for (i, run) in runs.iter().enumerate().take(prev) {
                    if (mask >> i) & 1 == 1 {
                        sub.push(*run);
                    }
                }
                sub.push(runs[prev]);
                if lattice_attempt(&sub, r, ell, g, x, &mut cnt) == Some(d) {
                    solved_at = Some(n);
                    break 'subsets;
                }
            }
        }
        for n in s as usize..=nmax {
            if solved_at.is_some_and(|sa| sa <= n) {
                cum[n - s as usize] += 1;
            }
        }
        runs_used += solved_at.unwrap_or(nmax) as u64;
        fl += cnt.flops;
        red += cnt.reductions;
        cand += cnt.candidates;
        gadd += cnt.group_adds;
    }
    let t = trials.max(1) as f64;
    (
        LatOutcome {
            cum_success: cum.into_iter().map(|c| (c, trials)).collect(),
            mean_flops: fl as f64 / t,
            mean_reductions: red as f64 / t,
            mean_candidates: cand as f64 / t,
            mean_group_adds: gadd as f64 / t,
            mean_runs: runs_used as f64 / t,
        },
        p,
    )
}

fn mode_lattice(a: &Args) {
    let m = a.u64("m", 12) as u32;
    let sigmas: Vec<u32> = parse_list(&a.str("sigma", "4,7"));
    let ss: Vec<u32> = parse_list(&a.str("s", "2,3"));
    let extra = a.u64("extra", 3) as u32;
    let trials = a.u64("trials", 200);
    let seed = a.u64("seed", 1);
    let shape = a.str("rshape", "top");
    let mut rng = Rng::new(seed ^ ((m as u64) << 32) ^ 0xA5A5);
    for &s in &ss {
        for &sigma in &sigmas {
            let t0 = std::time::Instant::now();
            let (out, p) = run_lattice(m, sigma, s, extra, trials, &shape, &mut rng);
            for (i, &(k, n)) in out.cum_success.iter().enumerate() {
                let runs = s as usize + i;
                let (lo, hi) = wilson(k, n);
                println!(
                    "{{\"kind\":\"lattice_cell\",\"m\":{m},\"r\":{},\"sigma\":{sigma},\"s\":{s},\"ell\":{},\"runs\":{runs},\"trials\":{n},\"success\":{k},\"rate\":{},\"ci_lo\":{lo},\"ci_hi\":{hi},\"quantum_ops_total\":{},\"mean_flops_to_stop\":{},\"mean_reductions_to_stop\":{},\"mean_candidates_to_stop\":{},\"mean_group_adds_to_stop\":{},\"mean_runs_to_stop\":{}}}",
                    p.r,
                    p.ell,
                    jf(k as f64 / n as f64),
                    runs as u32 * (m + sigma + p.ell),
                    jf(out.mean_flops),
                    jf(out.mean_reductions),
                    jf(out.mean_candidates),
                    jf(out.mean_group_adds),
                    jf(out.mean_runs),
                );
            }
            eprintln!("lattice m={m} s={s} sigma={sigma} trials={trials} {:.1}s", t0.elapsed().as_secs_f64());
        }
    }
}

// ───────────────────────────────── model mode ──────────────────────────────

#[derive(Clone)]
struct Arm {
    name: String,
    family: &'static str,
    ops_a: usize,
    ops_b: usize,
    windowed_base: bool,
    e_runs: f64,
    p_used: f64,
    nu: i64,
    classical_g: f64,
    classical_native: f64,
    promise: bool,
    note: String,
}

#[derive(Clone, Copy)]
struct Eval {
    qubits: f64,
    toffoli_run: f64,
    toffoli_expected: f64,
    window: usize,
    table_additions: f64,
}

fn eval_arm(n: usize, arm: &Arm, band: ps::Band) -> Eval {
    let c = if arm.windowed_base {
        ps::best_window(n, arm.ops_a, arm.ops_b, band)
    } else {
        ps::plain_cost(n, arm.ops_a, arm.ops_b, band.f)
    };
    Eval {
        qubits: c.qubits,
        toffoli_run: c.toffoli,
        toffoli_expected: c.toffoli * arm.e_runs,
        window: c.window,
        table_additions: c.table_additions,
    }
}

const CAP_LOG2: f64 = 40.0;

fn verdict(prod: &[f64], tof: &[f64], qub: &[f64], capped: bool) -> &'static str {
    let max_p = prod.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_p = prod.iter().cloned().fold(f64::INFINITY, f64::min);
    if max_p <= 0.95 {
        return if capped { "TRADES" } else { "ADVANCES" };
    }
    if min_p >= 1.05 {
        return "REGRESSES";
    }
    if prod.iter().all(|&x| x > 0.95 && x < 1.05) {
        return "MATCHES";
    }
    let tof_lt = tof.iter().all(|&x| x < 1.0);
    let tof_gt = tof.iter().all(|&x| x > 1.0);
    let q_lt = qub.iter().all(|&x| x < 1.0);
    let q_gt = qub.iter().all(|&x| x > 1.0);
    if (tof_lt && q_gt) || (tof_gt && q_lt) {
        return "TRADES";
    }
    "INCONCLUSIVE"
}

fn mode_model(a: &Args) {
    let seed = a.u64("seed", 20);
    let mtoy = a.u64("toy-m", 16) as u32;
    let runs_toy = a.u64("toy-runs", 2000);
    let lat_trials = a.u64("toy-trials", 300);
    let mut rng = Rng::new(seed);
    let rt = largest_prime_below_pow2(mtoy);

    // measured success probabilities (Wilson lower bounds)
    let measure = |sigma: u32, nu: i64, b_delta: u64, rng: &mut Rng| -> (f64, f64, f64, u64) {
        let ell = (mtoy as i64 + nu) as u32;
        let res = run_single_cell(mtoy, sigma, ell, rt, runs_toy, &[b_delta], rng);
        let k = res.succ[0][0];
        let (lo, hi) = wilson(k, res.n);
        (k as f64 / res.n as f64, lo, hi, k)
    };
    let (p0_pt, p0_lo, p0_hi, k0) = measure(0, 0, 0, &mut rng);
    println!(
        "{{\"kind\":\"measured_p\",\"which\":\"p0_shor\",\"m\":{mtoy},\"sigma\":0,\"nu\":0,\"b_eta\":0,\"b_delta\":0,\"runs\":{runs_toy},\"success\":{k0},\"point\":{},\"wilson_lower\":{},\"wilson_upper\":{}}}",
        jf(p0_pt), jf(p0_lo), jf(p0_hi)
    );
    let mut p1_lo = 1.0f64;
    for &nu in &[0i64, -2, -4] {
        let (pt, lo, _hi, k) = measure(7, nu, 100, &mut rng);
        p1_lo = p1_lo.min(lo);
        println!(
            "{{\"kind\":\"measured_p\",\"which\":\"p1_ekera\",\"m\":{mtoy},\"sigma\":7,\"nu\":{nu},\"b_eta\":0,\"b_delta\":100,\"runs\":{runs_toy},\"success\":{k},\"point\":{},\"wilson_lower\":{}}}",
            jf(pt), jf(lo)
        );
    }
    // lattice cumulative successes and measured native work
    struct Lat {
        s: usize,
        cum_lo: Vec<f64>,
        flops: f64,
        gadd: f64,
    }
    let mut lats: Vec<Lat> = Vec::new();
    for &s in &[2u32, 3] {
        let (out, p) = run_lattice(mtoy, 7, s, 3, lat_trials, "top", &mut rng);
        let cum_lo: Vec<f64> = out.cum_success.iter().map(|&(k, n)| wilson(k, n).0).collect();
        for (i, &(k, n)) in out.cum_success.iter().enumerate() {
            println!(
                "{{\"kind\":\"measured_lattice\",\"m\":{mtoy},\"s\":{s},\"sigma\":7,\"ell\":{},\"runs\":{},\"trials\":{n},\"success\":{k},\"point\":{},\"wilson_lower\":{}}}",
                p.ell, s as usize + i, jf(k as f64 / n as f64), jf(cum_lo[i])
            );
        }
        lats.push(Lat { s: s as usize, cum_lo, flops: out.mean_flops, gadd: out.mean_group_adds });
    }

    println!(
        "{{\"kind\":\"anchor\",\"rnsl_qubits_256\":{},\"table_qubits_256\":2330,\"rnsl_toffoli_256\":{},\"table_toffoli_256\":1.26e11,\"formula_over_table\":{},\"haner_low_w_qubits\":2124,\"haner_low_t_qubits\":2619,\"provisional_2026_qubits\":\"1200-1450\",\"provisional_2026_toffoli\":\"7e7-9e7\"}}",
        ps::rnsl_qubits(256),
        jf(ps::rnsl_toffoli(256)),
        jf(ps::rnsl_toffoli(256) / 1.26e11)
    );

    for &n in &[256usize, 384, 521] {
        let m = n;
        let labels = if n == 256 { "P-256,secp256k1" } else { "scaling-context" };
        let mut arms: Vec<Arm> = Vec::new();
        let base = |name: &str, family: &'static str, ops_a: usize, ops_b: usize, e: f64, p: f64, nu: i64, g: f64, nat: f64, note: &str| Arm {
            name: name.to_string(),
            family,
            ops_a,
            ops_b,
            windowed_base: false,
            e_runs: e,
            p_used: p,
            nu,
            classical_g: g,
            classical_native: nat,
            promise: false,
            note: note.to_string(),
        };
        arms.push(base("A0", "baseline", m, m, 1.0, 1.0, 0, 0.0, 0.0, "committed baseline: Shor 2m additions, success taken as 1"));
        arms.push(base("A0p", "baseline-measured-success", m, m, 1.0 / p0_hi, p0_hi, 0, 0.0, 0.0, "A0 with the single-run success measured at toy m (Wilson UPPER bound, the side that favours the baseline)"));
        // E1-0
        let ell0 = m;
        let bt0 = ps::search_half_width(m as f64, ell0, 100.0);
        let (g0, n0) = ps::single_run_search_work(m, bt0, 0);
        arms.push(base("E1-0", "ekera-single", m + 7, ell0, 1.0 / p1_lo, p1_lo, 0, g0, n0, "sigma=7, B_eta=0, B_Delta=100, ell=m"));
        let mut tier_arms: Vec<Arm> = Vec::new();
        for &tier in &[20.0f64, 30.0, 40.0, 50.0, 58.0] {
            let nu = ps::tier_nu(m, 100.0, 0, tier);
            let (ra, rb) = ps::ekera_registers(m, 7, 1, nu);
            let bt = ps::search_half_width(m as f64, rb, 100.0);
            let (g, nat) = ps::single_run_search_work(m, bt, 0);
            let name = format!("E1-2^{}", tier as u32);
            println!(
                "{{\"kind\":\"tier\",\"n\":{n},\"tier_log2\":{tier},\"nu\":{nu},\"ell\":{rb},\"quantum_ops\":{},\"saved_vs_2m\":{},\"classical_g_log2\":{},\"classical_native_log2\":{},\"classical_s1_log2\":{},\"bt_log2\":{}}}",
                ra + rb, 2 * m as i64 - (ra + rb) as i64, jf(lg(g)), jf(lg(nat)), jf(lg(g + nat)), jf(lg(bt))
            );
            let a = base(&name, "ekera-single-tier", ra, rb, 1.0 / p1_lo, p1_lo, nu, g, nat, "sigma=7, B_eta=0, B_Delta=100, ell=m+nu(tier)");
            tier_arms.push(a);
        }
        arms.extend(tier_arms.iter().cloned());
        for l in &lats {
            let (ra, rb) = ps::ekera_registers(m, 7, l.s, 0);
            let (e, cyc) = ps::expected_runs_sequential(l.s, &l.cum_lo);
            let note = format!("lattice over subsets, toy-measured cumulative success (cycle success {cyc:.3}); native work is the toy mean to stop");
            arms.push(base(&format!("E2-s{}", l.s), "ekera-lattice", ra, rb, e, cyc, 0, l.gadd, l.flops, &note));
        }
        // windowed variants
        let to_windowed = |src: &Arm| -> Arm {
            let mut w = src.clone();
            w.name = format!("W({})", src.name);
            w.family = "windowed";
            w.windowed_base = true;
            w
        };
        let a0 = arms[0].clone();
        let e10 = arms[2].clone();
        let e140 = tier_arms[2].clone();
        arms.push(to_windowed(&a0));
        arms.push(to_windowed(&e10));
        arms.push(to_windowed(&e140));
        // promise problem rows
        for &md in &[128usize, 160, 192] {
            let mut x = base(&format!("EH-{md}"), "ekera-hastad-promise", md, md / 2, 1.0 / 0.99, 0.99, 0, 0.0, 0.0, "short-logarithm promise problem; success >= 0.99 cited from Ekera-Hastad, not measured; classical lattice not measured");
            x.promise = true;
            arms.push(x);
        }

        // evaluation
        let band = ps::band_points();
        let plain_band = [ps::Band { kappa: 1.0, mu: 3.0, f: 1.0 }];
        for arm in &arms {
            let pts: &[ps::Band] = if arm.windowed_base { &band } else { &plain_band };
            // baselines at the same band points
            let (mut prod, mut tof, mut qub) = (Vec::new(), Vec::new(), Vec::new());
            let (mut prod2, mut tof2, mut qub2) = (Vec::new(), Vec::new(), Vec::new());
            let (mut wmin, mut wmax) = (usize::MAX, 0usize);
            let mut tab_max = 0.0f64;
            let mut central = None;
            for b in pts {
                let ev = eval_arm(n, arm, *b);
                let a0e = eval_arm(n, &arms[0], *b);
                let a0pe = eval_arm(n, &arms[1], *b);
                prod.push(ev.qubits * ev.toffoli_expected / (a0e.qubits * a0e.toffoli_expected));
                tof.push(ev.toffoli_expected / a0e.toffoli_expected);
                qub.push(ev.qubits / a0e.qubits);
                prod2.push(ev.qubits * ev.toffoli_expected / (a0pe.qubits * a0pe.toffoli_expected));
                tof2.push(ev.toffoli_expected / a0pe.toffoli_expected);
                qub2.push(ev.qubits / a0pe.qubits);
                if arm.windowed_base {
                    wmin = wmin.min(ev.window);
                    wmax = wmax.max(ev.window);
                    tab_max = tab_max.max(ev.table_additions);
                }
                if (b.kappa - 1.0).abs() < 1e-12 && (b.mu - 3.0).abs() < 1e-12 && (b.f - 1.0).abs() < 1e-12 {
                    central = Some(ev);
                }
            }
            let ce = central.unwrap_or_else(|| eval_arm(n, arm, plain_band[0]));
            let s0 = arm.classical_g + arm.classical_native * 0.0 + tab_max;
            let s01 = arm.classical_g + arm.classical_native * 0.1 + tab_max;
            let s1 = arm.classical_g + arm.classical_native + tab_max;
            let capped = lg(s1) > CAP_LOG2;
            let (v1, v2) = if arm.promise {
                ("N/A-promise-problem", "N/A-promise-problem")
            } else if arm.name == "A0" {
                ("BASELINE", "BASELINE")
            } else if arm.name == "A0p" {
                ("BASELINE-ADJUSTED", "BASELINE")
            } else {
                (verdict(&prod, &tof, &qub, capped), verdict(&prod2, &tof2, &qub2, capped))
            };
            let mm = |v: &[f64]| (v.iter().cloned().fold(f64::INFINITY, f64::min), v.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
            let (pmin, pmax) = mm(&prod);
            let (tmin, tmax) = mm(&tof);
            let (qmin, qmax) = mm(&qub);
            let (p2min, p2max) = mm(&prod2);
            println!(
                "{{\"kind\":\"arm\",\"n\":{n},\"labels\":\"{labels}\",\"arm\":\"{}\",\"family\":\"{}\",\"nu\":{},\"ops_a\":{},\"ops_b\":{},\"ops_per_run\":{},\"expected_runs\":{},\"p_used\":{},\"band_points\":{},\"central_qubits\":{},\"central_toffoli_per_run\":{},\"central_toffoli_expected\":{},\"central_product\":{},\"ratio_product_min\":{},\"ratio_product_max\":{},\"ratio_toffoli_min\":{},\"ratio_toffoli_max\":{},\"ratio_qubits_min\":{},\"ratio_qubits_max\":{},\"ratio_product_vs_A0p_min\":{},\"ratio_product_vs_A0p_max\":{},\"window_min\":{},\"window_max\":{},\"table_additions_log2_max\":{},\"classical_g_log2\":{},\"classical_native_log2\":{},\"s_c0_log2\":{},\"s_c01_log2\":{},\"s_c1_log2\":{},\"rho_distance_bits_c1\":{},\"capped\":{capped},\"verdict\":\"{v1}\",\"verdict_vs_A0p\":\"{v2}\",\"level\":\"constant\",\"note\":\"{}\"}}",
                arm.name, arm.family, arm.nu, arm.ops_a, arm.ops_b, arm.ops_a + arm.ops_b,
                jf(arm.e_runs), jf(arm.p_used), pts.len(),
                jf(ce.qubits), jf(ce.toffoli_run), jf(ce.toffoli_expected), jf(ce.qubits * ce.toffoli_expected),
                jf(pmin), jf(pmax), jf(tmin), jf(tmax), jf(qmin), jf(qmax), jf(p2min), jf(p2max),
                if arm.windowed_base { wmin as i64 } else { 0 },
                wmax,
                jf(lg(tab_max)),
                jf(lg(arm.classical_g)), jf(lg(arm.classical_native)),
                jf(lg(s0)), jf(lg(s01)), jf(lg(s1)),
                jf(ps::matched_rho_log2(m) - lg(s1)),
                arm.note
            );
        }
    }
    println!("{{\"kind\":\"model_done\",\"cap_log2\":{CAP_LOG2}}}");
}

// ──────────────────────────────── selftest ─────────────────────────────────

fn gate(name: &str, pass: bool, detail: String) -> bool {
    println!("{{\"kind\":\"selftest\",\"gate\":\"{name}\",\"pass\":{pass},\"detail\":\"{detail}\"}}");
    pass
}

/// Brute-force joint distribution of (j, k) for a tiny instance.
fn brute_joint(p: &Params, d: u64) -> Vec<Vec<f64>> {
    let (mm, ll, r) = (p.mm as usize, p.ll as usize, p.r as usize);
    let two_pi = 2.0 * std::f64::consts::PI;
    let mut out = vec![vec![0.0f64; ll]; mm];
    for e in 0..r {
        let mut members: Vec<(usize, usize)> = Vec::new();
        for a in 0..mm {
            for b in 0..ll {
                if (a + r * ll * 4 - (b * d as usize) % r) % r == e {
                    members.push((a, b));
                }
            }
        }
        for j in 0..mm {
            for k in 0..ll {
                let (mut re, mut im) = (0.0f64, 0.0f64);
                for &(a, b) in &members {
                    let th = two_pi * ((a * j) % mm) as f64 / mm as f64
                        + two_pi * ((b * k) % ll) as f64 / ll as f64;
                    re += th.cos();
                    im += th.sin();
                }
                out[j][k] += (re * re + im * im) / ((mm * ll) as f64).powi(2);
            }
        }
    }
    out
}

fn factorised_joint(p: &Params, d: u64) -> Vec<Vec<f64>> {
    let inst = make_inst(p, d);
    let marg = marginal_probs(p);
    let mut out = vec![vec![0.0f64; p.ll as usize]; p.mm as usize];
    for j in 0..p.mm {
        let u = mulmod(j, p.r, p.mm);
        if marg[u as usize] == 0.0 {
            continue;
        }
        let pe = cond_e_probs(p, &inst, u);
        for (e, &pe_v) in pe.iter().enumerate() {
            if pe_v == 0.0 {
                continue;
            }
            let pk = cond_k_probs(p, &inst, e as u64, u, j);
            for (k, &pk_v) in pk.iter().enumerate() {
                out[j as usize][k] += marg[u as usize] * pe_v * pk_v;
            }
        }
    }
    out
}

fn mode_selftest() {
    let mut all = true;
    // (a) formulas
    let q = ps::rnsl_qubits(256);
    let ratio = ps::rnsl_toffoli(256) / 1.26e11;
    all &= gate(
        "rnsl_formulas",
        q == 2330 && (ratio - 1.0).abs() < 0.03,
        format!("qubits={q} toffoli_over_table={ratio:.4}"),
    );
    // (b) sampler vs brute force
    for &(r, sigma, ell, d) in &[(29u64, 1u32, 4u32, 11u64), (31, 2, 3, 7), (13, 0, 4, 5)] {
        let m = 64 - (r - 1).leading_zeros();
        let p = Params::new(m, sigma, ell, r);
        let br = brute_joint(&p, d);
        let fa = factorised_joint(&p, d);
        let (mut maxd, mut sum) = (0.0f64, 0.0f64);
        for j in 0..p.mm as usize {
            for k in 0..p.ll as usize {
                maxd = maxd.max((br[j][k] - fa[j][k]).abs());
                sum += fa[j][k];
            }
        }
        all &= gate(
            &format!("sampler_exact_r{r}_s{sigma}_l{ell}"),
            maxd < 1e-9 && (sum - 1.0).abs() < 1e-9,
            format!("max_abs_diff={maxd:.3e} sum={sum:.12}"),
        );
    }
    // (c) LLL / Babai
    {
        let mut rng = Rng::new(7);
        let mut ok = true;
        for _ in 0..50 {
            let dim = 4;
            let mut b: Vec<Vec<i128>> = (0..dim)
                .map(|i| {
                    (0..dim)
                        .map(|j| if i == j { 1000 } else { rng.below(2000) as i128 - 1000 })
                        .collect()
                })
                .collect();
            let det = |m: &Vec<Vec<i128>>| -> f64 {
                let mut f: Vec<Vec<f64>> =
                    m.iter().map(|r| r.iter().map(|&x| x as f64).collect()).collect();
                let mut d = 1.0;
                for c in 0..f.len() {
                    let piv = (c..f.len())
                        .max_by(|&x, &y| f[x][c].abs().partial_cmp(&f[y][c].abs()).unwrap())
                        .unwrap();
                    if f[piv][c].abs() < 1e-9 {
                        return 0.0;
                    }
                    if piv != c {
                        f.swap(piv, c);
                        d = -d;
                    }
                    d *= f[c][c];
                    for r2 in c + 1..f.len() {
                        let fac = f[r2][c] / f[c][c];
                        for k in c..f.len() {
                            let t = f[c][k];
                            f[r2][k] -= fac * t;
                        }
                    }
                }
                d.abs()
            };
            let d0 = det(&b);
            let (mut fl, mut rd) = (0u64, 0u64);
            lll(&mut b, &mut fl, &mut rd);
            let d1 = det(&b);
            if d0 > 0.0 && (d0 - d1).abs() / d0 > 1e-6 {
                ok = false;
            }
            // planted close vector: lattice point + small noise
            let coef: Vec<i128> = (0..dim).map(|_| rng.below(21) as i128 - 10).collect();
            let mut pt = vec![0i128; dim];
            for (c, row) in coef.iter().zip(&b) {
                for (x, y) in pt.iter_mut().zip(row) {
                    *x += c * y;
                }
            }
            let noisy: Vec<i128> = pt.iter().map(|&x| x + rng.below(7) as i128 - 3).collect();
            let got = babai(&b, &noisy, &mut fl);
            if got != pt {
                ok = false;
            }
        }
        all &= gate("lll_babai", ok, "determinant preserved, planted point recovered (50 bases)".into());
    }
    // (d) BSGS recovery and cost
    {
        let mut rng = Rng::new(11);
        let r = largest_prime_below_pow2(20);
        let mut ok = true;
        let (mut tot, mut cnt) = (0.0f64, 0u64);
        for _ in 0..200 {
            let g = 1 + rng.below(r - 1);
            let bt = 100u64;
            let t = rng.below(2 * bt + 1) as i64 - bt as i64;
            let mut cg = Cg::new(r);
            let pos = cg.mul(t.unsigned_abs(), g);
            let target = if t >= 0 { pos } else { cg.neg(pos) };
            let mut cg2 = Cg::new(r);
            let ctx = bsgs_build(&mut cg2, g, bt);
            let got = bsgs_find(&mut cg2, &ctx, target);
            if got != Some(t) {
                ok = false;
            }
            tot += cg2.adds as f64;
            cnt += 1;
        }
        let mean = tot / cnt as f64;
        // formula without the per-eta scalar multiplications (pure BSGS part)
        let w = 201.0f64;
        let q = w.sqrt().ceil();
        let model = (q - 1.0) + 1.0 + 1.5 * 7.0 + (w / q).ceil();
        let rr = mean / model;
        all &= gate("bsgs", ok && (0.5..=2.0).contains(&rr), format!("recovered_all={ok} mean_adds={mean:.1} model={model:.1} ratio={rr:.3}"));
    }
    // (e) window model against Haner et al.
    {
        let b = ps::Band { kappa: 1.0, mu: 3.0, f: 1.0 / 62.0 };
        let c = ps::best_window(256, 263, 256, b);
        let t7 = c.toffoli * 7.0;
        let lo = 1.08 * 2f64.powi(31);
        let hi = 1.72 * 2f64.powi(32);
        let pass = (15..=22).contains(&c.window) && t7 >= lo / 3.0 && t7 <= hi * 3.0;
        all &= gate("window_vs_haner", pass, format!("best_window={} model_T={:.3e} haner_range=[{:.3e},{:.3e}]", c.window, t7, lo, hi));
    }
    // (f) search/criterion agreement on a tiny cell
    {
        let mut rng = Rng::new(3);
        let r = largest_prime_below_pow2(10);
        let p = Params::new(10, 4, 10, r);
        let marg = build_marginal(&p);
        let (_ma, _mn, _vs, mism, _fg, _fn) = verify_cost(&p, &marg, 1, 30, 150, &mut rng);
        all &= gate("search_vs_criterion", mism == 0, format!("mismatches={mism}/150"));
    }
    println!("{{\"kind\":\"selftest_done\",\"all_pass\":{all}}}");
    if !all {
        std::process::exit(3);
    }
}

fn main() {
    let args = Args::from_env();
    match args.str("mode", "selftest").as_str() {
        "selftest" => mode_selftest(),
        "single" => mode_single(&args),
        "lattice" => mode_lattice(&args),
        "model" => mode_model(&args),
        other => {
            eprintln!("unknown mode {other}");
            std::process::exit(2);
        }
    }
}
