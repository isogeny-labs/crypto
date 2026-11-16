//! E5a of research/generator_effect_ic_20261019/PLAN.md: x-coordinate
//! statistics of the first `2^24` multiples of the standard generator on
//! P-192 and P-224, against random-scalar points and random-abscissa points.
//!
//! `cargo run --release --example generator_effect_e5a -- OUT.json`
//!
//! The tests, bins, seeds and the Holm correction are frozen in
//! `research/generator_effect_ic_20261019/e5/PROTOCOL.md`; this program
//! implements that text and nothing else.  No ECDLP run, no security claim.
//!
//! Arithmetic: 4-limb Montgomery field elements (`ct_bignum::Uint<4>`),
//! Jacobian points with `a = −3` doubling and mixed addition, one batched
//! inversion per chunk of `2^16` points.  A fixed-base comb (8-bit windows)
//! gives `[s]G`.  The BigUint arithmetic of `src/ecc` checks the results.
use crypto_lib::ct_bignum::{MontgomeryContext, Uint};
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::point::Point;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use rayon::prelude::*;
use serde_json::{json, Value};
use std::time::Instant;

type U = Uint<4>;

const CHUNK_BITS: u32 = 16;
const CHUNK: usize = 1 << CHUNK_BITS;
const LOG2_B: [u32; 2] = [20, 24];
const B_MAX_CHUNKS: usize = 1 << (24 - CHUNK_BITS);
const SEED: u64 = 20261019;
const ALPHA: f64 = 0.05;
const FAMILY: usize = 20;

// ── field ────────────────────────────────────────────────────────────────

struct Fp {
    ctx: MontgomeryContext<4>,
    p: U,
    one: U,
    bits: u32,
}

impl Fp {
    fn new(p: &BigUint) -> Self {
        let pu = U::from_biguint(p);
        let ctx = MontgomeryContext::new(pu).expect("odd prime");
        let one = ctx.r_mod_n;
        Fp {
            ctx,
            p: pu,
            one,
            bits: p.bits() as u32,
        }
    }
    fn mul(&self, a: &U, b: &U) -> U {
        self.ctx.mont_mul(a, b)
    }
    fn sqr(&self, a: &U) -> U {
        self.ctx.mont_sqr(a)
    }
    fn add(&self, a: &U, b: &U) -> U {
        a.add_mod(b, &self.p)
    }
    fn sub(&self, a: &U, b: &U) -> U {
        a.sub_mod(b, &self.p)
    }
    fn dbl(&self, a: &U) -> U {
        self.add(a, a)
    }
    fn to_m(&self, a: &U) -> U {
        self.ctx.to_montgomery(a)
    }
    fn canon(&self, a: &U) -> U {
        self.ctx.from_montgomery(a)
    }
    /// Variable-time `a^e` in Montgomery form; `e` canonical.
    fn pow(&self, a: &U, e: &U) -> U {
        let mut acc = self.one;
        for bit in (0..256).rev() {
            acc = self.sqr(&acc);
            if (e.0[bit / 64] >> (bit % 64)) & 1 == 1 {
                acc = self.mul(&acc, a);
            }
        }
        acc
    }
    fn inv(&self, a: &U) -> U {
        let pm2 = U::from_biguint(&(self.p.to_biguint() - 2u32));
        self.pow(a, &pm2)
    }
}

fn is_zero(a: &U) -> bool {
    a.0.iter().all(|&l| l == 0)
}

fn lt(a: &U, b: &U) -> bool {
    for k in (0..4).rev() {
        if a.0[k] != b.0[k] {
            return a.0[k] < b.0[k];
        }
    }
    false
}

fn bitlen(a: &U) -> u32 {
    for k in (0..4).rev() {
        if a.0[k] != 0 {
            return 64 * k as u32 + 64 - a.0[k].leading_zeros();
        }
    }
    0
}

fn popcount(a: &U) -> u32 {
    a.0.iter().map(|l| l.count_ones()).sum()
}

/// Uniform in `[lo, bound)` from `bitlen(bound)` random bits, by rejection.
fn draw_below(rng: &mut StdRng, bound: &U, bits: u32, lo: &U) -> U {
    loop {
        let mut v = U::ZERO;
        for k in 0..4 {
            let take = bits.saturating_sub(64 * k as u32).min(64);
            if take > 0 {
                let w = rng.next_u64();
                v.0[k] = if take == 64 {
                    w
                } else {
                    w & ((1u64 << take) - 1)
                };
            }
        }
        if lt(&v, bound) && !lt(&v, lo) {
            return v;
        }
    }
}

// ── points ───────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct Jac {
    x: U,
    y: U,
    z: U,
}

#[derive(Clone, Copy)]
struct Aff {
    x: U,
    y: U,
}

struct Curve {
    f: Fp,
    b: U,
    three: U,
    n: U,
    n_bits: u32,
    g: Aff,
}

impl Curve {
    fn new(c: &CurveParams) -> Self {
        let f = Fp::new(&c.p);
        let b = f.to_m(&U::from_biguint(&c.b));
        let three = f.to_m(&U::from_biguint(&BigUint::from(3u32)));
        let g = Aff {
            x: f.to_m(&U::from_biguint(&c.gx)),
            y: f.to_m(&U::from_biguint(&c.gy)),
        };
        Curve {
            f,
            b,
            three,
            n: U::from_biguint(&c.n),
            n_bits: c.n.bits() as u32,
            g,
        }
    }

    fn inf(&self) -> Jac {
        Jac {
            x: self.f.one,
            y: self.f.one,
            z: U::ZERO,
        }
    }

    /// dbl-2001-b, `a = −3`.
    fn double(&self, p: &Jac) -> Jac {
        let f = &self.f;
        if is_zero(&p.z) || is_zero(&p.y) {
            return self.inf();
        }
        let delta = f.sqr(&p.z);
        let gamma = f.sqr(&p.y);
        let beta = f.mul(&p.x, &gamma);
        let alpha = f.mul(
            &self.three,
            &f.mul(&f.sub(&p.x, &delta), &f.add(&p.x, &delta)),
        );
        let beta4 = f.dbl(&f.dbl(&beta));
        let x3 = f.sub(&f.sqr(&alpha), &f.dbl(&beta4));
        let yz = f.add(&p.y, &p.z);
        let z3 = f.sub(&f.sub(&f.sqr(&yz), &gamma), &delta);
        let g2 = f.sqr(&gamma);
        let g8 = f.dbl(&f.dbl(&f.dbl(&g2)));
        let y3 = f.sub(&f.mul(&alpha, &f.sub(&beta4, &x3)), &g8);
        Jac {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// madd-2007-bl, with the `P = ±Q` cases handled.
    fn madd(&self, p: &Jac, q: &Aff) -> Jac {
        let f = &self.f;
        if is_zero(&p.z) {
            return Jac {
                x: q.x,
                y: q.y,
                z: f.one,
            };
        }
        let z1z1 = f.sqr(&p.z);
        let u2 = f.mul(&q.x, &z1z1);
        let s2 = f.mul(&q.y, &f.mul(&p.z, &z1z1));
        let h = f.sub(&u2, &p.x);
        let r = f.dbl(&f.sub(&s2, &p.y));
        if is_zero(&h) {
            return if is_zero(&r) {
                self.double(p)
            } else {
                self.inf()
            };
        }
        let hh = f.sqr(&h);
        let i = f.dbl(&f.dbl(&hh));
        let j = f.mul(&h, &i);
        let v = f.mul(&p.x, &i);
        let x3 = f.sub(&f.sub(&f.sqr(&r), &j), &f.dbl(&v));
        let y3 = f.sub(&f.mul(&r, &f.sub(&v, &x3)), &f.dbl(&f.mul(&p.y, &j)));
        let zh = f.add(&p.z, &h);
        let z3 = f.sub(&f.sub(&f.sqr(&zh), &z1z1), &hh);
        Jac {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    /// Batched normalisation to affine (Montgomery form); no point may be O.
    fn normalize(&self, pts: &[Jac]) -> Vec<Aff> {
        let f = &self.f;
        let mut prefix = Vec::with_capacity(pts.len());
        let mut acc = f.one;
        for p in pts {
            assert!(!is_zero(&p.z), "point at infinity in a batch");
            prefix.push(acc);
            acc = f.mul(&acc, &p.z);
        }
        let mut inv = f.inv(&acc);
        let mut out = vec![
            Aff {
                x: U::ZERO,
                y: U::ZERO
            };
            pts.len()
        ];
        for k in (0..pts.len()).rev() {
            let zi = f.mul(&inv, &prefix[k]);
            inv = f.mul(&inv, &pts[k].z);
            let zi2 = f.sqr(&zi);
            out[k] = Aff {
                x: f.mul(&pts[k].x, &zi2),
                y: f.mul(&pts[k].y, &f.mul(&zi2, &zi)),
            };
        }
        out
    }

    /// Fixed-base comb: `table[j][d − 1] = [d·256^j]G`.
    fn comb(&self) -> Vec<Vec<Aff>> {
        let windows = self.n_bits.div_ceil(8) as usize;
        let mut base = self.g;
        let mut table = Vec::with_capacity(windows);
        for _ in 0..windows {
            let mut chain = Vec::with_capacity(255);
            let mut acc = Jac {
                x: base.x,
                y: base.y,
                z: self.f.one,
            };
            chain.push(acc);
            for _ in 1..255 {
                acc = self.madd(&acc, &base);
                chain.push(acc);
            }
            let row = self.normalize(&chain);
            let mut b = Jac {
                x: base.x,
                y: base.y,
                z: self.f.one,
            };
            for _ in 0..8 {
                b = self.double(&b);
            }
            base = self.normalize(&[b])[0];
            table.push(row);
        }
        table
    }

    fn mul_comb(&self, table: &[Vec<Aff>], s: &U) -> Jac {
        let mut acc = self.inf();
        for (j, row) in table.iter().enumerate() {
            let d = ((s.0[j / 8] >> (8 * (j % 8))) & 0xff) as usize;
            if d != 0 {
                acc = self.madd(&acc, &row[d - 1]);
            }
        }
        acc
    }

    /// `x³ − 3x + b` is a square (or zero) in `F_p`; `x` canonical.
    fn x_on_curve(&self, x: &U) -> bool {
        let f = &self.f;
        let xm = f.to_m(x);
        let rhs = f.add(
            &f.sub(&f.mul(&f.sqr(&xm), &xm), &f.mul(&self.three, &xm)),
            &self.b,
        );
        if is_zero(&rhs) {
            return true;
        }
        let e = U::from_biguint(&((f.p.to_biguint() - 1u32) >> 1));
        f.canon(&f.pow(&rhs, &e)).0 == U::ONE.0
    }
}

// ── statistics ───────────────────────────────────────────────────────────

#[derive(Clone)]
struct Stats {
    lz: Vec<u64>,
    hw: Vec<u64>,
    hw_sum: u64,
    /// record lows inside the chunk: (1-based global index, x)
    records: Vec<(u64, U)>,
}

impl Stats {
    fn new() -> Self {
        Stats {
            lz: vec![0; 257],
            hw: vec![0; 257],
            hw_sum: 0,
            records: Vec::new(),
        }
    }
    fn push(&mut self, idx: u64, x: &U, ell: u32) {
        self.lz[(ell - bitlen(x)) as usize] += 1;
        let w = popcount(x);
        self.hw[w as usize] += 1;
        self.hw_sum += w as u64;
        if self.records.last().is_none_or(|(_, m)| lt(x, m)) {
            self.records.push((idx, *x));
        }
    }
    fn merge(chunks: &[Stats]) -> Stats {
        let mut out = Stats::new();
        for c in chunks {
            for k in 0..257 {
                out.lz[k] += c.lz[k];
                out.hw[k] += c.hw[k];
            }
            out.hw_sum += c.hw_sum;
            for (i, x) in &c.records {
                if out.records.last().is_none_or(|(_, m)| lt(x, m)) {
                    out.records.push((*i, *x));
                }
            }
        }
        out
    }
}

struct Chunk {
    stats: Stats,
    /// (global index, scalar or 0, canonical x) of the first 64 elements and of checkpoints
    samples: Vec<(u64, U, U)>,
}

const CHECKPOINTS: [u64; 8] = [1, 2, 3, 1000, 1 << 16, (1 << 16) + 1, 1 << 20, 1 << 24];

fn chunk_rng(curve_id: u64, set_id: u64, c: usize) -> StdRng {
    StdRng::seed_from_u64((SEED << 20) | (curve_id << 12) | (set_id << 8) | c as u64)
}

fn run_set(cv: &Curve, table: &[Vec<Aff>], curve_id: u64, set: &str) -> Vec<Chunk> {
    let ell = cv.f.bits;
    let p = cv.f.p;
    (0..B_MAX_CHUNKS)
        .into_par_iter()
        .map(|c| {
            let first = (c * CHUNK) as u64 + 1;
            let mut st = Stats::new();
            let mut samples = Vec::new();
            match set {
                "gmul" | "rscalar" => {
                    let mut scalars = Vec::with_capacity(CHUNK);
                    let mut pts = Vec::with_capacity(CHUNK);
                    if set == "gmul" {
                        let s0 = U::from_biguint(&BigUint::from(first));
                        let mut acc = cv.mul_comb(table, &s0);
                        for k in 0..CHUNK {
                            if k > 0 {
                                acc = cv.madd(&acc, &cv.g);
                            }
                            pts.push(acc);
                            scalars.push(U::from_biguint(&BigUint::from(first + k as u64)));
                        }
                    } else {
                        let mut rng = chunk_rng(curve_id, 1, c);
                        for _ in 0..CHUNK {
                            let s = draw_below(&mut rng, &cv.n, cv.n_bits, &U::ONE);
                            pts.push(cv.mul_comb(table, &s));
                            scalars.push(s);
                        }
                    }
                    let aff = cv.normalize(&pts);
                    for (k, a) in aff.iter().enumerate() {
                        let idx = first + k as u64;
                        let x = cv.f.canon(&a.x);
                        st.push(idx, &x, ell);
                        if (c == 0 && k < 64) || CHECKPOINTS.contains(&idx) {
                            samples.push((idx, scalars[k], x));
                        }
                    }
                }
                "rx" => {
                    let mut rng = chunk_rng(curve_id, 2, c);
                    for k in 0..CHUNK {
                        let x = loop {
                            let x = draw_below(&mut rng, &p, ell, &U::ZERO);
                            if cv.x_on_curve(&x) {
                                break x;
                            }
                        };
                        let idx = first + k as u64;
                        st.push(idx, &x, ell);
                        if c == 0 && k < 64 {
                            samples.push((idx, U::ZERO, x));
                        }
                    }
                }
                _ => unreachable!(),
            }
            Chunk { stats: st, samples }
        })
        .collect()
}

// ── tests ────────────────────────────────────────────────────────────────

fn ln_gamma(x: f64) -> f64 {
    // Lanczos, g = 7, n = 9.
    const C: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        let pi = std::f64::consts::PI;
        return (pi / (pi * x).sin()).ln() - ln_gamma(1.0 - x);
    }
    let x = x - 1.0;
    let mut a = C[0];
    let t = x + 7.5;
    for (i, c) in C.iter().enumerate().skip(1) {
        a += c / (x + i as f64);
    }
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
}

/// Regularised upper incomplete gamma `Q(a, x)`.
fn gamma_q(a: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 1.0;
    }
    let gln = ln_gamma(a);
    if x < a + 1.0 {
        let (mut sum, mut del, mut ap) = (1.0 / a, 1.0 / a, a);
        for _ in 0..10_000 {
            ap += 1.0;
            del *= x / ap;
            sum += del;
            if del.abs() < sum.abs() * 1e-15 {
                break;
            }
        }
        1.0 - sum * (-x + a * x.ln() - gln).exp()
    } else {
        let tiny = 1e-300;
        let mut b = x + 1.0 - a;
        let mut c = 1.0 / tiny;
        let mut d = 1.0 / b;
        let mut h = d;
        for i in 1..10_000 {
            let an = -(i as f64) * (i as f64 - a);
            b += 2.0;
            d = an * d + b;
            if d.abs() < tiny {
                d = tiny;
            }
            c = b + an / c;
            if c.abs() < tiny {
                c = tiny;
            }
            d = 1.0 / d;
            let del = d * c;
            h *= del;
            if (del - 1.0).abs() < 1e-15 {
                break;
            }
        }
        (-x + a * x.ln() - gln).exp() * h
    }
}

fn chi2_p(stat: f64, df: usize) -> f64 {
    gamma_q(df as f64 / 2.0, stat / 2.0)
}

/// `lz` bins `0..K−1` and `≥ K`.
fn lz_bins(lz: &[u64], k: usize) -> Vec<u64> {
    let mut v: Vec<u64> = lz[..k].to_vec();
    v.push(lz[k..].iter().sum());
    v
}

/// Hamming-weight bins: one per integer in `[lo, hi]`, tails pooled.
fn hw_range(ell: u32) -> (usize, usize) {
    let mid = ell as f64 / 2.0;
    let sigma = (ell as f64).sqrt() / 2.0;
    (
        (mid - 3.0 * sigma).floor() as usize,
        (mid + 3.0 * sigma).ceil() as usize,
    )
}

fn hw_bins(hw: &[u64], ell: u32) -> Vec<u64> {
    let (lo, hi) = hw_range(ell);
    let mut v = vec![hw[..lo].iter().sum()];
    v.extend_from_slice(&hw[lo..=hi]);
    v.push(hw[hi + 1..].iter().sum());
    v
}

/// Two-sample chi-square homogeneity; returns (stat, df, p).
fn chi2_two_sample(a: &[u64], b: &[u64]) -> (f64, usize, f64) {
    let (na, nb) = (a.iter().sum::<u64>() as f64, b.iter().sum::<u64>() as f64);
    let (ka, kb) = ((nb / na).sqrt(), (na / nb).sqrt());
    let mut stat = 0.0;
    let mut used = 0usize;
    for (x, y) in a.iter().zip(b) {
        let s = (*x + *y) as f64;
        if s > 0.0 {
            let d = ka * *x as f64 - kb * *y as f64;
            stat += d * d / s;
            used += 1;
        }
    }
    let df = used.saturating_sub(1);
    (stat, df, chi2_p(stat, df))
}

fn chi2_gof(obs: &[u64], probs: &[f64]) -> (f64, usize, f64) {
    let n = obs.iter().sum::<u64>() as f64;
    let stat: f64 = obs
        .iter()
        .zip(probs)
        .map(|(o, q)| {
            let e = n * q;
            (*o as f64 - e).powi(2) / e
        })
        .sum();
    let df = obs.len() - 1;
    (stat, df, chi2_p(stat, df))
}

fn lz_probs(p: &BigUint, ell: u32, k: usize) -> Vec<f64> {
    // Uniform on [0, p): bit length ℓ−j for j ≥ 1 has 2^{ℓ−j−1} values.
    let pf = p.to_string().parse::<f64>().unwrap();
    let two = |e: i32| 2f64.powi(e);
    let mut v = vec![(pf - two(ell as i32 - 1)) / pf];
    for j in 1..k {
        v.push(two(ell as i32 - j as i32 - 1) / pf);
    }
    v.push(two(ell as i32 - k as i32) / pf);
    v
}

fn hw_probs(ell: u32) -> Vec<f64> {
    // Hamming weight of a uniform ℓ-bit integer: Binomial(ℓ, 1/2).
    let ln_choose = |k: usize| {
        ln_gamma(ell as f64 + 1.0)
            - ln_gamma(k as f64 + 1.0)
            - ln_gamma((ell as usize - k) as f64 + 1.0)
    };
    let pk = |k: usize| (ln_choose(k) - ell as f64 * 2f64.ln()).exp();
    let (lo, hi) = hw_range(ell);
    let mut v = vec![(0..lo).map(pk).sum()];
    v.extend((lo..=hi).map(pk));
    v.push((hi + 1..=ell as usize).map(pk).sum());
    v
}

fn u_to_f64(a: &U) -> f64 {
    a.to_biguint().to_string().parse::<f64>().unwrap()
}

fn min_test(xmin: &U, p: &U, b: u64) -> Value {
    let pf = u_to_f64(p);
    let xf = u_to_f64(xmin);
    let u = b as f64 * xf / pf;
    let cdf = -(-u).exp_m1();
    let pval = (2.0 * cdf.min(1.0 - cdf)).min(1.0);
    let expect = pf / (b as f64 + 1.0);
    json!({
        "u": u,
        "cdf": cdf,
        "p": pval,
        "log2_xmin": if xf > 0.0 { xf.log2() } else { f64::NEG_INFINITY },
        "log2_expectation": expect.log2(),
        "log2_xmin_over_expectation": if xf > 0.0 { (xf / expect).log2() } else { f64::NEG_INFINITY },
    })
}

fn hex(a: &U) -> String {
    format!("0x{}", a.to_biguint().to_str_radix(16))
}

// ── main ─────────────────────────────────────────────────────────────────

fn main() {
    let out = std::env::args()
        .nth(1)
        .expect("usage: generator_effect_e5a OUT.json");
    let curves = [(1u64, CurveParams::p192()), (2u64, CurveParams::p224())];
    let mut curve_out = Vec::new();
    let mut family: Vec<(String, f64)> = Vec::new();
    let mut controls = Vec::new();
    let mut all_checks_pass = true;

    for (cid, cp) in &curves {
        let t0 = Instant::now();
        let cv = Curve::new(cp);
        let ell = cv.f.bits;
        let a_fe = cp.a_fe();
        let g = cp.generator();

        // Check 1: G on the curve and [n]G = O.
        let g_on = cp.is_on_curve(&g);
        let ng_inf = matches!(g.scalar_mul_vartime(&cp.n, &a_fe), Point::Infinity);

        let table = cv.comb();
        let sets = ["gmul", "rscalar", "rx"];
        let mut runs = Vec::new();
        for set in sets {
            let ts = Instant::now();
            let chunks = run_set(&cv, &table, *cid, set);
            eprintln!("{} {set}: {:.1} s", cp.name, ts.elapsed().as_secs_f64());
            runs.push((set, chunks, ts.elapsed().as_secs_f64()));
        }

        // Check 2: fast arithmetic against BigUint.
        let mut mism = 0usize;
        let mut checked = 0usize;
        for (set, chunks, _) in &runs {
            for ch in chunks {
                for (idx, s, x) in &ch.samples {
                    checked += 1;
                    let ok = match *set {
                        "gmul" | "rscalar" => {
                            let k = if *set == "gmul" {
                                BigUint::from(*idx)
                            } else {
                                s.to_biguint()
                            };
                            match g.scalar_mul_vartime(&k, &a_fe) {
                                Point::Affine { x: px, .. } => px.value == x.to_biguint(),
                                Point::Infinity => false,
                            }
                        }
                        _ => {
                            let xb = x.to_biguint();
                            let rhs = (&xb * &xb * &xb + &cp.a * &xb + &cp.b) % &cp.p;
                            let e = (&cp.p - 1u32) >> 1;
                            let l = rhs.modpow(&e, &cp.p);
                            l == BigUint::from(1u32) || rhs == BigUint::from(0u32)
                        }
                    };
                    if !ok {
                        mism += 1;
                    }
                }
            }
        }
        let checkpoints_seen: Vec<u64> = runs[0]
            .1
            .iter()
            .flat_map(|c| c.samples.iter().map(|s| s.0))
            .filter(|i| CHECKPOINTS.contains(i))
            .collect();
        let checks_pass =
            g_on && ng_inf && mism == 0 && checkpoints_seen.len() == CHECKPOINTS.len();
        all_checks_pass &= checks_pass;

        // First valid abscissa (positive control).
        let mut xf = U::ZERO;
        while !cv.x_on_curve(&xf) {
            xf = xf.add_mod(&U::ONE, &cv.f.p);
        }

        let mut by_b = Vec::new();
        for lb in LOG2_B {
            let nchunks = 1usize << (lb - CHUNK_BITS);
            let b = 1u64 << lb;
            let k = (lb - 6) as usize;
            let merged: Vec<(String, Stats)> = runs
                .iter()
                .map(|(s, ch, _)| {
                    let st: Vec<Stats> = ch[..nchunks].iter().map(|c| c.stats.clone()).collect();
                    (s.to_string(), Stats::merge(&st))
                })
                .collect();
            let gm = &merged[0].1;
            let mut set_out = serde_json::Map::new();
            for (name, st) in &merged {
                let (xmin_idx, xmin) = *st.records.last().unwrap();
                let lzb = lz_bins(&st.lz, k);
                let hwb = hw_bins(&st.hw, ell);
                let (lz_s, lz_df, lz_p) = chi2_gof(&lzb, &lz_probs(&cp.p, ell, k));
                let (hw_s, hw_df, hw_p) = chi2_gof(&hwb, &hw_probs(ell));
                let records: Vec<Value> = st
                    .records
                    .iter()
                    .map(|(i, x)| json!({"index": i, "log2_x": u_to_f64(x).log2()}))
                    .collect();
                let max_lz = st.lz.iter().rposition(|&c| c > 0).unwrap_or(0);
                set_out.insert(
                    name.clone(),
                    json!({
                        "n": b,
                        "lz_hist": st.lz[..=max_lz.max(k)].to_vec(),
                        "bitlen_min": ell as usize - max_lz,
                        "hw_hist": st.hw[..=ell as usize].to_vec(),
                        "hw_mean": st.hw_sum as f64 / b as f64,
                        "x_min": hex(&xmin),
                        "x_min_index": xmin_idx,
                        "min_test": min_test(&xmin, &cv.f.p, b),
                        "records": records,
                        "calibration": {
                            "lz_vs_analytic": {"stat": lz_s, "df": lz_df, "p": lz_p},
                            "hw_vs_binomial": {"stat": hw_s, "df": hw_df, "p": hw_p},
                        },
                    }),
                );
            }
            // The family.
            let mut tests = Vec::new();
            for (name, st) in merged.iter().skip(1) {
                let (s1, d1, p1) = chi2_two_sample(&lz_bins(&gm.lz, k), &lz_bins(&st.lz, k));
                let (s2, d2, p2) = chi2_two_sample(&hw_bins(&gm.hw, ell), &hw_bins(&st.hw, ell));
                let id1 = format!("T-LZ/{}/2^{lb}/gmul-vs-{name}", cp.name);
                let id2 = format!("T-HW/{}/2^{lb}/gmul-vs-{name}", cp.name);
                family.push((id1.clone(), p1));
                family.push((id2.clone(), p2));
                tests.push(json!({"id": id1, "stat": s1, "df": d1, "p": p1}));
                tests.push(json!({"id": id2, "stat": s2, "df": d2, "p": p2}));
            }
            let gmin = gm.records.last().unwrap().1;
            let mt = min_test(&gmin, &cv.f.p, b);
            let id3 = format!("T-MIN/{}/2^{lb}/gmul", cp.name);
            family.push((id3.clone(), mt["p"].as_f64().unwrap()));
            tests.push(json!({"id": id3, "p": mt["p"]}));

            // Positive control: rx with element B/2 replaced by the first valid abscissa.
            let rx = &merged[2].1;
            let rmin = rx.records.last().unwrap().1;
            let planted_min = if lt(&xf, &rmin) { xf } else { rmin };
            let pc = min_test(&planted_min, &cv.f.p, b);
            let pc_p = pc["p"].as_f64().unwrap();
            controls.push(json!({
                "id": format!("planted-min/{}/2^{lb}/rx", cp.name),
                "planted_index": b / 2,
                "planted_x": hex(&xf),
                "p": pc_p,
                "threshold": ALPHA / FAMILY as f64,
                "detected": pc_p <= ALPHA / FAMILY as f64,
            }));
            by_b.push(json!({
                "log2_B": lb,
                "lz_K": k,
                "hw_range": hw_range(ell),
                "sets": set_out,
                "tests": tests,
            }));
        }
        curve_out.push(json!({
            "curve": cp.name,
            "p": hex(&cv.f.p),
            "n": hex(&cv.n),
            "ell": ell,
            "first_valid_x": hex(&xf),
            "checks": {
                "g_on_curve": g_on,
                "n_g_is_infinity": ng_inf,
                "biguint_crosschecks": checked,
                "biguint_mismatches": mism,
                "checkpoints": checkpoints_seen,
                "pass": checks_pass,
            },
            "wall_s": runs.iter().map(|(s, _, t)| json!({"set": s, "wall_s": t})).collect::<Vec<_>>(),
            "total_wall_s": t0.elapsed().as_secs_f64(),
            "by_B": by_b,
        }));
    }

    // Holm–Bonferroni.
    assert_eq!(family.len(), FAMILY, "family size is frozen at 20");
    let mut order: Vec<usize> = (0..family.len()).collect();
    order.sort_by(|&i, &j| family[i].1.partial_cmp(&family[j].1).unwrap());
    let mut reject = vec![false; family.len()];
    for (rank, &i) in order.iter().enumerate() {
        if family[i].1 <= ALPHA / (FAMILY - rank) as f64 {
            reject[i] = true;
        } else {
            break;
        }
    }
    let holm: Vec<Value> = order
        .iter()
        .enumerate()
        .map(|(rank, &i)| {
            json!({
                "id": family[i].0,
                "p": family[i].1,
                "holm_threshold": ALPHA / (FAMILY - rank) as f64,
                "reject": reject[i],
            })
        })
        .collect();
    let n_reject = reject.iter().filter(|&&r| r).count();
    let doc = json!({
        "schema": "generator_effect_e5a/v1",
        "protocol": "research/generator_effect_ic_20261019/e5/PROTOCOL.md",
        "seed": SEED,
        "alpha": ALPHA,
        "family_size": FAMILY,
        "checks_pass": all_checks_pass,
        "curves": curve_out,
        "holm": holm,
        "rejections": n_reject,
        "prediction_holds": n_reject == 0,
        "positive_controls": controls,
    });
    std::fs::write(&out, serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap();
    eprintln!("checks_pass={all_checks_pass} rejections={n_reject} -> {out}");
}
