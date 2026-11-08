//! NC-1 Stage 0: exact slope-coverage figure of merit for oblivious point sequences at K = 1.
//!
//! LABEL: BSGS-class boundary result, outside the IC ratchet. This tool is not index calculus.
//! It performs zero group operations. Record: research/curve25519_autolab_20261104/ideas/
//! NONCOLLISION-20261108.md, idea NC-1.
//!
//! Model. d is uniform in Z_r^*. An oblivious (d-independent) sequence of vectors
//! v_t = (a_t, b_t) in Z_r^2 gives points P_t = (a_t + b_t d) G, keyed by x (negation fold, so the
//! key is the value up to sign). A pair (s, t) collides iff
//!   (-) (a_s - a_t) + (b_s - b_t) d = 0   (equal points), or
//!   (+) (a_s + a_t) + (b_s + b_t) d = 0   (opposite points).
//! A relation with a zero coefficient of d never fires (it has no solution, or holds for all d
//! when its constant is zero too: a degenerate pair v_s = +-v_t, counted and ignored). Each
//! relation with nonzero coefficient fires at exactly one slope d = -(const)/(coef). Events for
//! distinct slopes are mutually exclusive in d, so
//!   P(first collision within T points) = |Sigma(T)| / (r - 1)   (slope 0 is excluded),
//! with Sigma(T) the set of distinct nonzero slopes of the first T points. With tau the number of
//! points generated when the first collision occurs, E[tau] = sum_{T >= 0} P(tau > T) and the
//! reported E = that sum divided by sqrt(r) (terms T = 0 and T = 1 are included; each is 1).
//! The sum is truncated at T_cap points; the residual 1 - |Sigma(T_cap)|/(r - 1) is reported and a
//! row with residual above 1e-4 is not ranked.
//!
//! DESIGN TABLE (written before any result was looked at; parameters are fixed here)
//! ---------------------------------------------------------------------------------
//! Common: T_cap = 3.2 sqrt(r) with the negation fold (survival of an i.i.d. design there is
//! about 4e-5), 5.0 sqrt(r) in the no-negation mode. Early stop once |Sigma| = r - 1.
//! Cost rule: a scalar coefficient c > 1 costs 1.5 log2(c) group operations (double-and-add);
//! c <= 1 costs 0; one addition per extra term of a start or stride vector. One point costs one
//! addition and is counted inside E, not in the set-up. Set-up / sqrt(r) is added to E for the
//! figure of merit E + set-up/sqrt(r). Verification ([d]G = Q) is taken as 30 operations.
//!
//! IID   control: every (a_t, b_t) uniform in Z_r^2. Must give E = sqrt(pi)/2 = 0.8862.
//!       Not a realisable algorithm (no set-up charged: reported as 0).
//! D0    random 16-jump walk (control). Start (a_0, b_0) uniform, 16 jump vectors uniform,
//!       jump index per step from the seeded PRNG (oblivious, never from x), v_t = v_{t-1} + J.
//!       Set-up: 34 full scalar multiplications (start 2, jumps 32). Must give E in
//!       [0.85, 0.92], otherwise STOP and report, no tuning.
//!       PRE-REGISTERED AMENDMENT (coordinator, 2026-11-08, made after the D0 k=20 value 0.9211
//!       was seen and before any D3 run): the D0 interval is [0.85, 0.93]. Reason: the i.i.d.
//!       control matches 0.887, so the engine is right; a 16-jump walk shows the same 3 to 4%
//!       excess over i.i.d. in NC-2 Stage 0 (nc2/RESULTS.md), so 0.92 was too tight.
//! D1    bsgs.negation layout (control), mirroring src/cryptanalysis/ecbench/generic.rs:186-239:
//!       m = ceil(ceil(sqrt(r)) / 2); baby points (j, 0), j = 0..=m first, then giants
//!       (-i s, 1), i = 0.., s = 2m + 1. Set-up: one scalar multiplication of s. Expected about 1.0
//!       in [0.95, 1.05].
//! D2    Bernstein-Lange two grumpy giants and a baby, NO negation (key is the point itself, only
//!       relation (-) counts). RECONSTRUCTION FROM THE ABSTRACT, body unread, parameters recalled
//!       not read: n baby points (j, 0), then alternating giants X_i = (-2 i n, 1) and
//!       Y_i = (2 i n + 2 n - 1, -1); X covers [2in, 2in+n), Y covers [2in+n, 2in+2n) against the
//!       babies, and X-Y pairs add one extra slope each. n = alpha sqrt(r), alpha in
//!       {0.4, 0.5, 0.6, 0.75, 0.9}. Checked at T = 1.5 sqrt(r) against the abstract's success
//!       probabilities 0.5625 (BSGS), 0.6753 (random walk), 0.71875 (grumpy giants); a miss on
//!       0.71875 is reported as an approximation. Set-up: one scalar multiplication of 2n.
//!       No-negation references run here: i.i.d. points and BSGS with n = 0.75 sqrt(r).
//! D3    multi-class stride tiling, c in {2, 3, 4, 8} classes, negation fold on. Class k emits
//!       points (o_k + i stride_k, beta_k), i = 0, 1, ...; the classes are interleaved round robin
//!       with w_k points per round. Scale-free parameters (so a design transfers across r and k):
//!         sigma = round(rho sqrt(r)), tau = round(phi sqrt(r)), rho, phi in [0.05, 4]
//!         stride_k = m_k sigma + s_k,  m_k in [-8, 8], s_k in [-2, 2]
//!         o_k      = u_k tau + v_k,    u_k in [-8, 8], v_k in [-4, 4]
//!         beta_k in [-8, 8],           w_k in [1, 3]
//!       Set-up: sigma and tau are the only full-size multiplications (at most 2); everything
//!       else is a small multiple priced by the cost rule. Search: hill climb on |Sigma| at
//!       T = ceil(sqrt(r)), mean over the TRAIN r values, single-parameter mutations (rho, phi
//!       times 1 +- {0.01, 0.05, 0.2}; +-1 step or redraw of one class field), accept if not worse,
//!       R restarts of N evaluations; restart 0 of c = 2 starts from the BSGS layout, all other
//!       restarts are random. Budgets (N x R): k = 20 1000 x 4; k = 24 300 x 4; k = 28 50 x 4;
//!       k = 32 only if the gate passes. Train r = siblings minus confirm; HELD-OUT r = the
//!       confirm curves' r (lists below). Only held-out E counts. The headline per k is the c with
//!       the best TRAIN score (best-by-held-out is shown but labelled optimistic).
//!       Gate for k = 32: held-out D3 E < 0.886 at both k = 20 and k = 28.
//! RB    randomised-b controls on the D3 design: per point (b_t replaced by uniform) and per class
//!       (beta_k replaced by a uniform full-size value). E must return to the Poisson value.
//! Conversion (ASSUMED from the record's band line, not measured): at k = 20,
//!       S(w) = E + set-up/sqrt(r) + 30/sqrt(r) + w E, rho-neg S = 1.447 / 1.547 / 2.448 at
//!       w = 0 / 0.1 / 1; implied ratio = S(w) / rho-neg S(w).

use std::collections::BTreeSet;
use std::env;
use std::process::exit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;

const LABEL: &str = "BSGS-class boundary result, outside the IC ratchet";

/// Held-out r values (the confirm curves) and train r values (siblings minus confirm), from
/// research/curve25519_autolab_20261104/c1/{confirm-curves,siblings-k*-10}.json.
const HELD: [(u32, &[u64]); 4] = [
    (20, &[131111, 131149, 131171, 131101, 131129]),
    (24, &[2097523, 2097401, 2097169, 2097427, 2097421]),
    (28, &[33558059, 33556559, 33554839, 33556961, 33554831]),
    (32, &[536880611, 536875303, 536876363, 536883349, 536871367]),
];
const TRAIN: [(u32, &[u64]); 4] = [
    (20, &[131203, 131317, 131113, 131143]),
    (24, &[2097349, 2097559, 2098051, 2097229]),
    (28, &[33558163, 33554761, 33554581, 33555289]),
    (32, &[536885407, 536875813, 536882483, 536877853, 536882917]),
];

fn lookup(tab: &[(u32, &'static [u64])], k: u32) -> &'static [u64] {
    tab.iter()
        .find(|(kk, _)| *kk == k)
        .map(|x| x.1)
        .unwrap_or(&[])
}

// ---------------------------------------------------------------- PRNG

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
        ((self.next() as u128 * n as u128) >> 64) as u64
    }
    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + self.below((hi - lo + 1) as u64) as i64
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

// ---------------------------------------------------------------- modular helpers

fn modinv(a: u64, r: u64) -> u64 {
    let (mut t, mut nt) = (0i64, 1i64);
    let (mut x, mut nx) = (r as i64, a as i64);
    while nx != 0 {
        let q = x / nx;
        (t, nt) = (nt, t - q * nt);
        (x, nx) = (nx, x - q * nx);
    }
    debug_assert_eq!(x, 1);
    t.rem_euclid(r as i64) as u64
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 1;
    }
    true
}

fn isqrt_ceil(n: u64) -> u64 {
    let mut s = (n as f64).sqrt() as u64;
    while s * s < n {
        s += 1;
    }
    while s > 0 && (s - 1) * (s - 1) >= n {
        s -= 1;
    }
    s
}

/// out[i] = dens[i]^-1 mod r, or 0 where dens[i] = 0 (Montgomery batch inversion).
fn batch_inv(dens: &[u32], out: &mut Vec<u32>, pref: &mut Vec<u32>, r: u64) {
    out.clear();
    out.resize(dens.len(), 0);
    pref.clear();
    pref.resize(dens.len(), 0);
    let mut run = 1u64;
    for (i, &d) in dens.iter().enumerate() {
        pref[i] = run as u32;
        if d != 0 {
            run = run * d as u64 % r;
        }
    }
    let mut inv = modinv(run, r);
    for i in (0..dens.len()).rev() {
        let d = dens[i];
        if d != 0 {
            out[i] = (inv * pref[i] as u64 % r) as u32;
            inv = inv * d as u64 % r;
        }
    }
}

// ---------------------------------------------------------------- coverage

#[derive(Clone, Copy, Debug)]
struct Pt {
    a: u32,
    b: u32,
}

#[derive(Clone, Debug, Default)]
struct Curve {
    e_sum: f64,    // sum_{T=0}^{T_end} (1 - |Sigma(T)|/(r-1)), in points
    resid: f64,    // 1 - |Sigma|/(r-1) after the last point
    sigma: u64,    // |Sigma| after the last point
    points: usize, // points processed
    degenerate: u64,
    sigma_at: Vec<(usize, u64)>, // requested checkpoints
}

/// Process points in order and return the exact survival sum. `neg` = negation fold (both
/// relations), otherwise only the equal-point relation. `checks` are T values at which |Sigma| is
/// recorded.
fn coverage(v: &[Pt], r: u64, neg: bool, checks: &[usize]) -> Curve {
    let mut bits = vec![0u64; (r as usize).div_ceil(64) + 1];
    let denom = (r - 1) as f64;
    let mut sigma = 0u64;
    let mut cur = Curve {
        e_sum: 1.0, // T = 0
        ..Default::default()
    };
    let mut den1: Vec<u32> = Vec::new();
    let mut den2: Vec<u32> = Vec::new();
    let mut inv1: Vec<u32> = Vec::new();
    let mut inv2: Vec<u32> = Vec::new();
    let mut pref: Vec<u32> = Vec::new();
    let mut done = false;
    for t in 0..v.len() {
        let (at, bt) = (v[t].a as u64, v[t].b as u64);
        if t > 0 && !done {
            den1.clear();
            den2.clear();
            for p in &v[..t] {
                den1.push(((p.b as u64 + r - bt) % r) as u32);
                if neg {
                    den2.push(((p.b as u64 + bt) % r) as u32);
                }
            }
            batch_inv(&den1, &mut inv1, &mut pref, r);
            if neg {
                batch_inv(&den2, &mut inv2, &mut pref, r);
            }
            for s in 0..t {
                let a_s = v[s].a as u64;
                // (-): d = (a_t - a_s) / (b_s - b_t)
                let num = (at + r - a_s) % r;
                if den1[s] == 0 {
                    if num == 0 {
                        cur.degenerate += 1;
                    }
                } else {
                    let d = (num * inv1[s] as u64 % r) as usize;
                    if d != 0 && bits[d >> 6] >> (d & 63) & 1 == 0 {
                        bits[d >> 6] |= 1 << (d & 63);
                        sigma += 1;
                    }
                }
                if neg {
                    // (+): d = -(a_s + a_t) / (b_s + b_t)
                    let num = (2 * r - a_s - at) % r;
                    if den2[s] == 0 {
                        if num == 0 {
                            cur.degenerate += 1;
                        }
                    } else {
                        let d = (num * inv2[s] as u64 % r) as usize;
                        if d != 0 && bits[d >> 6] >> (d & 63) & 1 == 0 {
                            bits[d >> 6] |= 1 << (d & 63);
                            sigma += 1;
                        }
                    }
                }
            }
            if sigma == r - 1 {
                done = true;
            }
        }
        let big_t = t + 1;
        cur.e_sum += 1.0 - sigma as f64 / denom;
        cur.points = big_t;
        if checks.contains(&big_t) {
            cur.sigma_at.push((big_t, sigma));
        }
        if done {
            break;
        }
    }
    cur.sigma = sigma;
    cur.resid = 1.0 - sigma as f64 / denom;
    cur
}

// ---------------------------------------------------------------- designs

fn modr(x: i64, r: u64) -> u32 {
    x.rem_euclid(r as i64) as u32
}

fn gen_iid(r: u64, n: usize, rng: &mut Rng) -> Vec<Pt> {
    (0..n)
        .map(|_| Pt {
            a: rng.below(r) as u32,
            b: rng.below(r) as u32,
        })
        .collect()
}

const D0_JUMPS: usize = 16;

fn gen_d0(r: u64, n: usize, rng: &mut Rng) -> Vec<Pt> {
    let jumps: Vec<(u64, u64)> = (0..D0_JUMPS)
        .map(|_| (rng.below(r), rng.below(r)))
        .collect();
    let (mut a, mut b) = (rng.below(r), rng.below(r));
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(Pt {
            a: a as u32,
            b: b as u32,
        });
        let j = jumps[rng.below(D0_JUMPS as u64) as usize];
        a = (a + j.0) % r;
        b = (b + j.1) % r;
    }
    out
}

fn d0_setup_ops(r: u64) -> f64 {
    34.0 * 1.5 * (r as f64).log2()
}

fn gen_d1(r: u64, n: usize) -> (Vec<Pt>, u64) {
    let m = isqrt_ceil(r).div_ceil(2).max(1);
    let s = 2 * m + 1;
    let mut out = Vec::with_capacity(n);
    for j in 0..=m {
        if out.len() >= n {
            break;
        }
        out.push(Pt { a: j as u32, b: 0 });
    }
    let mut i = 0u64;
    while out.len() < n {
        out.push(Pt {
            a: modr(-((i * s % r) as i64), r),
            b: 1,
        });
        i += 1;
    }
    (out, s)
}

fn gen_d2(r: u64, n: usize, alpha: f64) -> (Vec<Pt>, u64) {
    let nb = ((alpha * (r as f64).sqrt()).round() as u64).max(1);
    let mut out = Vec::with_capacity(n);
    for j in 0..nb {
        if out.len() >= n {
            break;
        }
        out.push(Pt { a: j as u32, b: 0 });
    }
    let mut i = 0u64;
    while out.len() < n {
        out.push(Pt {
            a: modr(-((2 * i * nb % r) as i64), r),
            b: 1,
        });
        if out.len() < n {
            out.push(Pt {
                a: modr((2 * i * nb + 2 * nb - 1) as i64 % r as i64, r),
                b: modr(-1, r),
            });
        }
        i += 1;
    }
    (out, 2 * nb)
}

fn gen_bsgs_noneg(r: u64, n: usize, alpha: f64) -> Vec<Pt> {
    let nb = ((alpha * (r as f64).sqrt()).round() as u64).max(1);
    let mut out = Vec::with_capacity(n);
    for j in 0..nb {
        if out.len() >= n {
            break;
        }
        out.push(Pt { a: j as u32, b: 0 });
    }
    let mut i = 0u64;
    while out.len() < n {
        out.push(Pt {
            a: modr(-((i * nb % r) as i64), r),
            b: 1,
        });
        i += 1;
    }
    out
}

#[derive(Clone, Debug, PartialEq)]
struct Class {
    beta: i64,
    m: i64,
    s: i64,
    u: i64,
    v: i64,
    w: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct D3 {
    rho: f64,
    phi: f64,
    classes: Vec<Class>,
}

impl D3 {
    fn show(&self) -> String {
        let cl: Vec<String> = self
            .classes
            .iter()
            .map(|c| format!("{},{},{},{},{},{}", c.beta, c.m, c.s, c.u, c.v, c.w))
            .collect();
        format!("rho={:.4};phi={:.4};{}", self.rho, self.phi, cl.join(";"))
    }
    fn parse(s: &str) -> Option<D3> {
        let mut it = s.split(';');
        let rho = it.next()?.strip_prefix("rho=")?.parse().ok()?;
        let phi = it.next()?.strip_prefix("phi=")?.parse().ok()?;
        let mut classes = Vec::new();
        for c in it {
            let f: Vec<i64> = c.split(',').map(|x| x.parse().unwrap_or(0)).collect();
            if f.len() != 6 {
                return None;
            }
            classes.push(Class {
                beta: f[0],
                m: f[1],
                s: f[2],
                u: f[3],
                v: f[4],
                w: f[5] as usize,
            });
        }
        Some(D3 { rho, phi, classes })
    }
    fn sigma(&self, r: u64) -> i64 {
        (self.rho * (r as f64).sqrt()).round() as i64
    }
    fn tau(&self, r: u64) -> i64 {
        (self.phi * (r as f64).sqrt()).round() as i64
    }
    /// Set-up group operations under the cost rule of the design table.
    fn setup_ops(&self, r: u64) -> f64 {
        let cc = |x: i64| {
            let x = x.unsigned_abs() as f64;
            if x > 1.0 {
                1.5 * x.log2()
            } else {
                0.0
            }
        };
        let mut ops = 0.0;
        if self.classes.iter().any(|c| c.m != 0) {
            ops += cc(self.sigma(r));
        }
        if self.classes.iter().any(|c| c.u != 0) {
            ops += cc(self.tau(r));
        }
        for c in &self.classes {
            ops += cc(c.m) + cc(c.u) + cc(c.beta);
            let st = (c.m != 0) as i64 + (c.s != 0) as i64;
            let sa = (c.u != 0) as i64 + (c.v != 0) as i64 + (c.beta != 0) as i64;
            ops += ((st - 1).max(0) + (sa - 1).max(0)) as f64;
        }
        ops
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Rb {
    No,
    PerPoint,
    PerClass,
}

fn gen_d3(d: &D3, r: u64, n: usize, rb: Rb, rng: &mut Rng) -> Vec<Pt> {
    let sg = d.sigma(r);
    let ta = d.tau(r);
    let betas: Vec<u32> = d
        .classes
        .iter()
        .map(|c| {
            if rb == Rb::PerClass {
                1 + rng.below(r - 1) as u32
            } else {
                modr(c.beta, r)
            }
        })
        .collect();
    let pattern: Vec<usize> = d
        .classes
        .iter()
        .enumerate()
        .flat_map(|(k, c)| std::iter::repeat_n(k, c.w))
        .collect();
    let mut count = vec![0i64; d.classes.len()];
    let mut out = Vec::with_capacity(n);
    let mut pi = 0;
    while out.len() < n {
        let k = pattern[pi % pattern.len()];
        pi += 1;
        let c = &d.classes[k];
        let stride = c.m * sg + c.s;
        let o = c.u * ta + c.v;
        let a = (o as i128 + count[k] as i128 * stride as i128).rem_euclid(r as i128) as u32;
        count[k] += 1;
        let b = if rb == Rb::PerPoint {
            1 + rng.below(r - 1) as u32
        } else {
            betas[k]
        };
        out.push(Pt { a, b });
    }
    out
}

fn random_class(rng: &mut Rng) -> Class {
    Class {
        beta: rng.range(-8, 8),
        m: rng.range(-8, 8),
        s: rng.range(-2, 2),
        u: rng.range(-8, 8),
        v: rng.range(-4, 4),
        w: rng.range(1, 3) as usize,
    }
}

fn mutate(d: &D3, rng: &mut Rng) -> D3 {
    let mut n = d.clone();
    let steps = [0.01, 0.05, 0.2];
    match rng.below(10) {
        0 => {
            let f = steps[rng.below(3) as usize];
            let sgn = if rng.below(2) == 0 { 1.0 } else { -1.0 };
            n.rho = (n.rho * (1.0 + sgn * f)).clamp(0.05, 4.0);
        }
        1 => {
            let f = steps[rng.below(3) as usize];
            let sgn = if rng.below(2) == 0 { 1.0 } else { -1.0 };
            n.phi = (n.phi * (1.0 + sgn * f)).clamp(0.05, 4.0);
        }
        _ => {
            let k = rng.below(n.classes.len() as u64) as usize;
            let c = &mut n.classes[k];
            let delta = if rng.below(2) == 0 { 1 } else { -1 };
            match rng.below(6) {
                0 => c.beta = (c.beta + delta).clamp(-8, 8),
                1 => c.m = (c.m + delta).clamp(-8, 8),
                2 => c.s = rng.range(-2, 2),
                3 => c.u = (c.u + delta).clamp(-8, 8),
                4 => c.v = rng.range(-4, 4),
                _ => c.w = rng.range(1, 3) as usize,
            }
        }
    }
    n
}

/// Training objective for one r: |Sigma| / (r - 1) at T = ceil(sqrt(r)).
fn train_score(d: &D3, r: u64) -> f64 {
    let t0 = isqrt_ceil(r) as usize;
    let mut rng = Rng(1);
    let v = gen_d3(d, r, t0, Rb::No, &mut rng);
    let c = coverage(&v, r, true, &[]);
    c.sigma as f64 / (r - 1) as f64
}

fn mean_train(d: &D3, rs: &[u64]) -> f64 {
    par_map(rs.to_vec(), |r| train_score(d, r))
        .iter()
        .sum::<f64>()
        / rs.len() as f64
}

/// Hill climb with a fixed budget. Returns (best design, best train score).
fn hill_climb(c: usize, rs: &[u64], n_eval: usize, restarts: usize, seed: u64) -> (D3, f64) {
    let mut best: Option<(D3, f64)> = None;
    for rs_i in 0..restarts {
        let mut rng = Rng(seed.wrapping_mul(1000003).wrapping_add(rs_i as u64));
        let mut cur = if c == 2 && rs_i == 0 {
            D3 {
                rho: 1.0,
                phi: 1.0,
                classes: vec![
                    Class {
                        beta: 0,
                        m: 0,
                        s: 1,
                        u: 0,
                        v: 0,
                        w: 1,
                    },
                    Class {
                        beta: 1,
                        m: -1,
                        s: 0,
                        u: 0,
                        v: 0,
                        w: 1,
                    },
                ],
            }
        } else {
            D3 {
                rho: 0.05 + 3.95 * rng.unit(),
                phi: 0.05 + 3.95 * rng.unit(),
                classes: (0..c).map(|_| random_class(&mut rng)).collect(),
            }
        };
        let mut cur_s = mean_train(&cur, rs);
        for _ in 0..n_eval {
            let cand = mutate(&cur, &mut rng);
            let s = mean_train(&cand, rs);
            if s >= cur_s {
                cur = cand;
                cur_s = s;
            }
        }
        if best.as_ref().map(|b| cur_s > b.1).unwrap_or(true) {
            best = Some((cur, cur_s));
        }
    }
    best.unwrap()
}

// ---------------------------------------------------------------- parallel helper

fn par_map<T: Send, U: Send, F: Fn(T) -> U + Sync>(items: Vec<T>, f: F) -> Vec<U> {
    let n = items.len();
    let items: Vec<Mutex<Option<T>>> = items.into_iter().map(|x| Mutex::new(Some(x))).collect();
    let results: Vec<Mutex<Option<U>>> = (0..n).map(|_| Mutex::new(None)).collect();
    let next = AtomicUsize::new(0);
    let workers = thread::available_parallelism()
        .map(|x| x.get())
        .unwrap_or(1)
        .min(n.max(1));
    thread::scope(|sc| {
        for _ in 0..workers {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= n {
                    break;
                }
                let x = items[i].lock().unwrap().take().unwrap();
                *results[i].lock().unwrap() = Some(f(x));
            });
        }
    });
    results
        .into_iter()
        .map(|m| m.into_inner().unwrap().unwrap())
        .collect()
}

// ---------------------------------------------------------------- reporting

struct Row {
    design: String,
    k: u32,
    r: u64,
    seed: u64,
    e: f64,
    resid: f64,
    setup_ops: f64,
    sigma_t1: f64, // |Sigma|/(r-1) at T = sqrt(r)
    p_15: f64,     // |Sigma|/(r-1) at T = 1.5 sqrt(r)
    degenerate: u64,
    extra: String,
}

fn run_row(
    design: &str,
    k: u32,
    r: u64,
    seed: u64,
    v: &[Pt],
    neg: bool,
    setup_ops: f64,
    extra: &str,
) -> Row {
    let sq = (r as f64).sqrt();
    let t1 = sq.round() as usize;
    let t15 = (1.5 * sq).round() as usize;
    let c = coverage(v, r, neg, &[t1, t15]);
    let at = |t: usize| {
        c.sigma_at
            .iter()
            .find(|x| x.0 == t)
            .map(|x| x.1)
            .unwrap_or(c.sigma) as f64
            / (r - 1) as f64
    };
    Row {
        design: design.to_string(),
        k,
        r,
        seed,
        e: c.e_sum / sq,
        resid: c.resid,
        setup_ops,
        sigma_t1: at(t1),
        p_15: at(t15),
        degenerate: c.degenerate,
        extra: extra.to_string(),
    }
}

fn print_row(x: &Row) {
    let sq = (x.r as f64).sqrt();
    println!(
        "ROW {{\"label\":\"{}\",\"design\":\"{}\",\"k\":{},\"r\":{},\"seed\":{},\"E\":{:.5},\"resid\":{:.3e},\"setup_ops\":{:.1},\"setup_over_sqrt_r\":{:.4},\"E_plus_setup\":{:.5},\"sigma_frac_T1\":{:.5},\"p_T1.5\":{:.5},\"degenerate_pairs\":{},\"note\":\"{}\"}}",
        LABEL, x.design, x.k, x.r, x.seed, x.e, x.resid, x.setup_ops, x.setup_ops / sq,
        x.e + x.setup_ops / sq, x.sigma_t1, x.p_15, x.degenerate, x.extra
    );
}

fn summarize(name: &str, rows: &[&Row]) {
    if rows.is_empty() {
        return;
    }
    let n = rows.len() as f64;
    let mean = |f: &dyn Fn(&Row) -> f64| rows.iter().map(|x| f(x)).sum::<f64>() / n;
    let e = mean(&|x| x.e);
    let sd = (rows.iter().map(|x| (x.e - e).powi(2)).sum::<f64>() / (n - 1.0).max(1.0)).sqrt();
    let sq = |x: &Row| (x.r as f64).sqrt();
    println!(
        "SUMMARY {} k={} n={} E={:.4} sd={:.4} setup/sqrt(r)={:.4} E+setup={:.4} p_T1.5={:.4} max_resid={:.2e}",
        name,
        rows[0].k,
        rows.len(),
        e,
        sd,
        mean(&|x| x.setup_ops / sq(x)),
        mean(&|x| x.e + x.setup_ops / sq(x)),
        mean(&|x| x.p_15),
        rows.iter().map(|x| x.resid).fold(0.0, f64::max)
    );
    if rows[0].k == 20 {
        let em = e;
        let su = mean(&|x| (x.setup_ops + 30.0) / sq(x));
        let rho = [1.447, 1.547, 2.448];
        let w = [0.0, 0.1, 1.0];
        let s: Vec<String> = (0..3)
            .map(|i| {
                let sv = em + su + w[i] * em;
                format!("w={}: S={:.3} ratio={:.3}", w[i], sv, sv / rho[i])
            })
            .collect();
        println!(
            "CONVERSION(assumed, not measured) {} k=20: {}",
            name,
            s.join(" | ")
        );
    }
}

// ---------------------------------------------------------------- commands

fn tcap(r: u64, neg: bool) -> usize {
    ((if neg { 3.2 } else { 5.0 }) * (r as f64).sqrt()).ceil() as usize
}

fn cmd_control(k: u32, seeds: u64) {
    let rs = lookup(&HELD, k);
    println!("# {} | control k={} seeds={}", LABEL, k, seeds);
    for r in rs {
        assert!(is_prime(*r), "r={} not prime", r);
    }
    // jobs: (design, r, seed)
    let mut jobs: Vec<(String, u64, u64)> = Vec::new();
    for &r in rs {
        for s in 0..seeds {
            jobs.push(("iid".into(), r, s));
            jobs.push(("D0".into(), r, s));
            jobs.push(("iid-noneg".into(), r, s));
        }
        jobs.push(("D1".into(), r, 0));
        jobs.push(("bsgs-noneg-0.75".into(), r, 0));
        for a in [0.4, 0.5, 0.6, 0.75, 0.9] {
            jobs.push((format!("D2-noneg-a{}", a), r, 0));
        }
    }
    let rows: Vec<Row> = par_map(jobs, |(name, r, seed)| {
        let mut rng = Rng(0xC0FFEE ^ (r << 8) ^ seed);
        let neg = !name.contains("noneg");
        let n = tcap(r, neg);
        let sq = (r as f64).sqrt();
        let (v, setup) = match name.as_str() {
            "iid" | "iid-noneg" => (gen_iid(r, n, &mut rng), 0.0),
            "D0" => (gen_d0(r, n, &mut rng), d0_setup_ops(r)),
            "D1" => {
                let (v, s) = gen_d1(r, n);
                (v, 1.5 * (s as f64).log2())
            }
            "bsgs-noneg-0.75" => (gen_bsgs_noneg(r, n, 0.75), 1.5 * (0.75 * sq).log2()),
            _ => {
                let a: f64 = name.rsplit('a').next().unwrap().parse().unwrap();
                let (v, s) = gen_d2(r, n, a);
                (v, 1.5 * (s as f64).log2())
            }
        };
        run_row(&name, k, r, seed, &v, neg, setup, "")
    });
    for x in &rows {
        print_row(x);
    }
    let names: BTreeSet<String> = rows.iter().map(|x| x.design.clone()).collect();
    for n in names {
        let sel: Vec<&Row> = rows.iter().filter(|x| x.design == n).collect();
        summarize(&n, &sel);
    }
}

fn cmd_d3(
    k: u32,
    cs: &[usize],
    n_eval: usize,
    restarts: usize,
    seed: u64,
    seeds: u64,
    fixed: Option<D3>,
) {
    let train = lookup(&TRAIN, k);
    let held = lookup(&HELD, k);
    println!(
        "# {} | d3 k={} budget={}x{} seed={} rb_seeds={}",
        LABEL, k, n_eval, restarts, seed, seeds
    );
    println!("# train r = {:?}; held-out r = {:?}", train, held);
    for r in train.iter().chain(held.iter()) {
        assert!(is_prime(*r), "r={} not prime", r);
    }
    let mut designs: Vec<(usize, D3, f64)> = Vec::new();
    if let Some(d) = fixed {
        let sc = mean_train(&d, train);
        designs.push((d.classes.len(), d, sc));
    } else {
        for &c in cs {
            let (d, sc) = hill_climb(c, train, n_eval, restarts, seed);
            println!(
                "TRAIN c={} train_sigma_frac_T1={:.5} design={}",
                c,
                sc,
                d.show()
            );
            designs.push((c, d, sc));
        }
    }
    for (c, d, sc) in &designs {
        let mut jobs: Vec<(&str, u64, u64)> = Vec::new();
        for &r in held {
            jobs.push(("D3", r, 0));
            for s in 0..seeds {
                jobs.push(("RB-point", r, s));
                jobs.push(("RB-class", r, s));
            }
        }
        let rows: Vec<Row> = par_map(jobs, |(name, r, s)| {
            let rb = match name {
                "RB-point" => Rb::PerPoint,
                "RB-class" => Rb::PerClass,
                _ => Rb::No,
            };
            let mut rng = Rng(0xFACE ^ (r << 8) ^ s);
            let v = gen_d3(d, r, tcap(r, true), rb, &mut rng);
            let nm = format!("{}-c{}", name, c);
            run_row(&nm, k, r, s, &v, true, d.setup_ops(r), &d.show())
        });
        for x in &rows {
            print_row(x);
        }
        println!("DESIGN c={} train_score={:.5} {}", c, sc, d.show());
        let names: BTreeSet<String> = rows.iter().map(|x| x.design.clone()).collect();
        for n in names {
            let sel: Vec<&Row> = rows.iter().filter(|x| x.design == n).collect();
            summarize(&n, &sel);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let get = |key: &str, def: &str| -> String {
        args.iter()
            .position(|a| a == key)
            .and_then(|i| args.get(i + 1))
            .cloned()
            .unwrap_or_else(|| def.to_string())
    };
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("");
    let k: u32 = get("--k", "20").parse().unwrap();
    match cmd {
        "control" => cmd_control(k, get("--seeds", "8").parse().unwrap()),
        "d3" => {
            let cs: Vec<usize> = get("--c", "2,3,4,8")
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            let fixed = args
                .iter()
                .position(|a| a == "--design")
                .and_then(|i| args.get(i + 1))
                .map(|s| D3::parse(s).expect("bad --design"));
            cmd_d3(
                k,
                &cs,
                get("--budget", "300").parse().unwrap(),
                get("--restarts", "4").parse().unwrap(),
                get("--seed", "1").parse().unwrap(),
                get("--seeds", "4").parse().unwrap(),
                fixed,
            )
        }
        _ => {
            eprintln!("usage: nc1_stage0 control|d3 --k K ...");
            exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Brute force: for every d in 1..r, the first T (1-indexed point count) at which two points
    /// that are not +-identical vectors collide; returns #{d : tau(d) <= T} for T = 0..=n.
    fn brute(v: &[Pt], r: u64, neg: bool) -> Vec<u64> {
        let n = v.len();
        let mut cnt = vec![0u64; n + 1];
        for d in 1..r {
            let val = |p: &Pt| (p.a as u64 + p.b as u64 * d) % r;
            let key = |u: u64| if neg { u.min((r - u) % r) } else { u };
            let mut tau = None;
            'outer: for t in 0..n {
                for s in 0..t {
                    let same = v[s].a == v[t].a && v[s].b == v[t].b;
                    let opp = neg
                        && (v[s].a as u64 + v[t].a as u64) % r == 0
                        && (v[s].b as u64 + v[t].b as u64) % r == 0;
                    if same || opp {
                        continue;
                    }
                    if key(val(&v[s])) == key(val(&v[t])) {
                        tau = Some(t + 1);
                        break 'outer;
                    }
                }
            }
            if let Some(t) = tau {
                for c in cnt.iter_mut().skip(t) {
                    *c += 1;
                }
            }
        }
        cnt
    }

    fn sigma_curve(v: &[Pt], r: u64, neg: bool) -> Vec<u64> {
        (0..=v.len())
            .map(|t| {
                if t == 0 {
                    0
                } else {
                    coverage(&v[..t], r, neg, &[]).sigma
                }
            })
            .collect()
    }

    fn check(v: &[Pt], r: u64) {
        for neg in [true, false] {
            let b = brute(v, r, neg);
            assert!(*b.last().unwrap() > 5, "test must exercise collisions");
            assert_eq!(b, sigma_curve(v, r, neg), "neg={}", neg);
        }
    }

    #[test]
    fn exact_formula_random_r101() {
        let r = 101;
        for seed in 0..6 {
            let mut rng = Rng(seed);
            let v = gen_iid(r, 40, &mut rng);
            check(&v, r);
        }
    }

    #[test]
    fn exact_formula_structured_r101() {
        let r = 101;
        let (v, _) = gen_d1(r, 40);
        check(&v, r);
        let mut rng = Rng(9);
        let (v, _) = gen_d2(r, 40, 0.9);
        check(&v, r);
        // b = 0, equal b, opposite b, small b classes
        let mut w = Vec::new();
        for i in 0..40u32 {
            let b = [0u32, 1, 100, 1, 0, 100, 5, 96][(rng.below(8)) as usize];
            w.push(Pt {
                a: rng.below(r) as u32 + i * 0,
                b,
            });
        }
        w.dedup_by(|x, y| x.a == y.a && x.b == y.b);
        check(&w, r);
        let d = D3 {
            rho: 1.0,
            phi: 0.5,
            classes: vec![
                Class {
                    beta: 0,
                    m: 0,
                    s: 1,
                    u: 0,
                    v: 0,
                    w: 1,
                },
                Class {
                    beta: 2,
                    m: -1,
                    s: 0,
                    u: 1,
                    v: 1,
                    w: 2,
                },
                Class {
                    beta: -3,
                    m: 2,
                    s: 1,
                    u: 0,
                    v: 0,
                    w: 1,
                },
            ],
        };
        let mut rng = Rng(3);
        let v = gen_d3(&d, r, 40, Rb::No, &mut rng);
        check(&v, r);
        let v = gen_d0(r, 40, &mut rng);
        check(&v, r);
    }

    #[test]
    fn batch_inverse_matches() {
        let r = 101u64;
        let dens: Vec<u32> = (0..101).collect();
        let (mut o, mut p) = (Vec::new(), Vec::new());
        batch_inv(&dens, &mut o, &mut p, r);
        assert_eq!(o[0], 0);
        for i in 1..101u64 {
            assert_eq!(o[i as usize] as u64 * i % r, 1);
        }
    }

    #[test]
    fn d3_roundtrip_and_determinism() {
        let mut rng = Rng(5);
        let d = D3 {
            rho: 1.25,
            phi: 0.75,
            classes: (0..3).map(|_| random_class(&mut rng)).collect(),
        };
        assert_eq!(D3::parse(&d.show()).unwrap().show(), d.show());
        let r = 1009;
        let a = gen_d3(&d, r, 50, Rb::No, &mut Rng(1));
        let b = gen_d3(&d, r, 50, Rb::No, &mut Rng(2));
        assert!(a.iter().zip(&b).all(|(x, y)| x.a == y.a && x.b == y.b));
    }

    #[test]
    fn bsgs_layout_covers_everything_exactly_once_per_giant() {
        // D1 at r = 101: after enough giants every nonzero d is covered.
        let r = 101;
        let (v, _) = gen_d1(r, 120);
        let c = coverage(&v, r, true, &[]);
        assert_eq!(c.sigma, r - 1);
    }
}
