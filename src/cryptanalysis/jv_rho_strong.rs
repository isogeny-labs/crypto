//! A fair Pollard rho reference for the Joux–Vitse rows on `E(F_{p⁶})`.
//!
//! The reference of the cover ledger (`jv_cover::rho_e`, `rho_e_dp`) is an
//! r-adding walk with no negation map, one affine inversion per step and
//! four runs per size.  This module is the same walk as a 2×2 factorial of
//! the two standard accelerations, so each one is measured on its own:
//!
//! * **negation map**: the walk lives on `±P` (canonical `y`), with 2-cycle
//!   detection and a doubling escape (Bos–Kleinjung–Lenstra);
//! * **batched inversion**: `k` walkers step in lockstep and share one
//!   inversion (Montgomery's trick, `3(k−1)` products per round).
//!
//! plus distinguished points for storage.  Every run is single-threaded and
//! deterministic in `(p, seed, run, arm)`; runs are parallelised by the
//! caller.  The arithmetic is native (`u64` Barrett reductions) so that
//! `2·10⁹`-step runs finish; the **cost is not wall time**: each run counts
//! `F_{q³}` products, squarings and inversions (and group additions,
//! `F_p` additions and native operations) and the counts are priced
//!
//! * in the **harness unit**: per-operation `F_p` multiplications measured
//!   from `jv_cover::Fq3` itself (55, 55, 166 → one affine addition 331);
//! * in a **tuned unit** (Karatsuba products, Chung–Hasan squarings, the
//!   adjugate inversion): a bound that credits rho and nobody else.
//!
//! Counts, not formulas, are what a run emits; a step that took a different
//! path (doubling, restart) is counted as what it did.

use std::collections::HashMap;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

use super::gaudry_cubic::mm;
use super::jv_cover::{generate_spec, unit_costs, Ctx, EllE, Fld, Fq3, PtE6, Spec, E2, E6};
use super::residual_walk::{inv_mod, mix64};

type X6 = [u64; 6];

/// Barrett reduction for `x < 2^40`.
#[derive(Clone, Copy)]
struct Red {
    p: u64,
    m: u64,
}
impl Red {
    fn new(p: u64) -> Red {
        Red {
            p,
            m: u64::MAX / p + 1,
        }
    }
    #[inline(always)]
    fn r(&self, x: u64) -> u64 {
        let q = ((x as u128 * self.m as u128) >> 64) as u64;
        let mut r = x as i64 - (q * self.p) as i64;
        if r < 0 {
            r += self.p as i64;
        }
        if r >= self.p as i64 {
            r -= self.p as i64;
        }
        r as u64
    }
}

/// Counts of what a run did.
#[derive(Clone, Debug, Default, Serialize)]
pub struct OpCounts {
    pub mul: u64,
    pub sq: u64,
    pub inv: u64,
    /// group additions in the walk, doublings, and in setup (multipliers and
    /// restarts), each a full group operation
    pub walk_adds: u64,
    pub dbl_adds: u64,
    pub setup_adds: u64,
    /// `F_p` additions/subtractions inside the group law
    pub fp_adds: u64,
    /// native operations a harness price leaves out
    pub canon_cmp: u64,
    pub hash: u64,
    pub dp_check: u64,
    pub scalar_ops: u64,
    pub table_ops: u64,
    pub cycle_cmp: u64,
}

impl OpCounts {
    pub fn native(&self) -> u64 {
        self.canon_cmp
            + self.hash
            + self.dp_check
            + self.scalar_ops
            + self.table_ops
            + self.cycle_cmp
    }
}

/// `F_p` multiplications per `F_{q³}` product, squaring and inversion.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct PriceVector {
    pub mul: f64,
    pub sq: f64,
    pub inv: f64,
}

/// The harness's own prices, measured by calling its `Fq3`.
pub fn harness_prices(p: u64) -> PriceVector {
    let f = Fq3::new(p);
    let mut rng = StdRng::seed_from_u64(5);
    let a = f.random(&mut rng);
    let b = f.random(&mut rng);
    f.reset_muls();
    let _ = f.mul(&a, &b);
    let mul = f.muls() as f64;
    f.reset_muls();
    let _ = f.sq(&a);
    let sq = f.muls() as f64;
    f.reset_muls();
    let _ = f.inv(&a);
    let inv = f.muls() as f64;
    PriceVector { mul, sq, inv }
}

/// Karatsuba (6 `F_q` products and 2 by `s`), Chung–Hasan squaring, the
/// adjugate inversion; `F_q` product 3, squaring 2, multiplication by the
/// small constant `ω` free, `F_q` inversion 20 (the harness's 16 for the
/// `F_p` inversion).
pub const TUNED: PriceVector = PriceVector {
    mul: 24.0,
    sq: 18.0,
    inv: 62.0,
};

impl OpCounts {
    pub fn muls(&self, v: &PriceVector) -> f64 {
        self.mul as f64 * v.mul + self.sq as f64 * v.sq + self.inv as f64 * v.inv
    }
}

#[derive(Clone, Copy)]
struct Pt {
    x: X6,
    y: X6,
}

struct Arith {
    r: Red,
    p: u64,
    w: u64,
    s: [u64; 2],
    a2: X6,
    a4: X6,
    c: OpCounts,
}

impl Arith {
    #[inline(always)]
    fn fq_mul(&self, a: &[u64], b: &[u64]) -> [u64; 2] {
        [
            self.r.r(a[0] * b[0] + self.w * (a[1] * b[1])),
            self.r.r(a[0] * b[1] + a[1] * b[0]),
        ]
    }
    /// Unreduced `F_q` product (components < 2^35).
    #[inline(always)]
    fn fq_raw(&self, a: &[u64], b: &[u64]) -> [u64; 2] {
        [
            a[0] * b[0] + self.w * (a[1] * b[1]),
            a[0] * b[1] + a[1] * b[0],
        ]
    }
    fn mul(&mut self, a: &X6, b: &X6) -> X6 {
        self.c.mul += 1;
        self.mul_raw(a, b)
    }
    fn sq(&mut self, a: &X6) -> X6 {
        self.c.sq += 1;
        self.mul_raw(a, a)
    }
    #[inline(always)]
    fn mul_raw(&self, a: &X6, b: &X6) -> X6 {
        let mut d = [[0u64; 2]; 5];
        for i in 0..3 {
            for j in 0..3 {
                let t = self.fq_raw(&a[2 * i..2 * i + 2], &b[2 * j..2 * j + 2]);
                d[i + j][0] += t[0];
                d[i + j][1] += t[1];
            }
        }
        let d: [[u64; 2]; 5] = core::array::from_fn(|k| [self.r.r(d[k][0]), self.r.r(d[k][1])]);
        let s3 = self.fq_mul(&self.s, &d[3]);
        let s4 = self.fq_mul(&self.s, &d[4]);
        [
            self.addp(d[0][0], s3[0]),
            self.addp(d[0][1], s3[1]),
            self.addp(d[1][0], s4[0]),
            self.addp(d[1][1], s4[1]),
            d[2][0],
            d[2][1],
        ]
    }
    #[inline(always)]
    fn addp(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline(always)]
    fn subp(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    fn sub(&self, a: &X6, b: &X6) -> X6 {
        core::array::from_fn(|i| self.subp(a[i], b[i]))
    }
    fn add(&self, a: &X6, b: &X6) -> X6 {
        core::array::from_fn(|i| self.addp(a[i], b[i]))
    }
    fn neg(&self, a: &X6) -> X6 {
        core::array::from_fn(|i| if a[i] == 0 { 0 } else { self.p - a[i] })
    }
    fn fq_inv(&self, a: &[u64]) -> [u64; 2] {
        let n = self.subp(
            self.r.r(a[0] * a[0]),
            self.r.r(self.w * self.r.r(a[1] * a[1])),
        );
        let ni = inv_mod(n, self.p);
        [self.r.r(a[0] * ni), self.r.r((self.p - a[1]) % self.p * ni)]
    }
    /// Adjugate inversion in `F_q[θ]/(θ³ − s)`.
    fn inv(&mut self, a: &X6) -> X6 {
        self.c.inv += 1;
        let (a0, a1, a2) = (&a[0..2], &a[2..4], &a[4..6]);
        let sub2 = |u: [u64; 2], v: [u64; 2]| [self.subp(u[0], v[0]), self.subp(u[1], v[1])];
        let add2 = |u: [u64; 2], v: [u64; 2]| [self.addp(u[0], v[0]), self.addp(u[1], v[1])];
        let b0 = sub2(
            self.fq_mul(a0, a0),
            self.fq_mul(&self.s, &self.fq_mul(a1, a2)),
        );
        let b1 = sub2(
            self.fq_mul(&self.s, &self.fq_mul(a2, a2)),
            self.fq_mul(a0, a1),
        );
        let b2 = sub2(self.fq_mul(a1, a1), self.fq_mul(a0, a2));
        let n = add2(
            self.fq_mul(a0, &b0),
            self.fq_mul(&self.s, &add2(self.fq_mul(a2, &b1), self.fq_mul(a1, &b2))),
        );
        let ni = self.fq_inv(&n);
        let (c0, c1, c2) = (
            self.fq_mul(&b0, &ni),
            self.fq_mul(&b1, &ni),
            self.fq_mul(&b2, &ni),
        );
        [c0[0], c0[1], c1[0], c1[1], c2[0], c2[1]]
    }
    /// Addition given the inverse of `x₂ − x₁` (1 product for λ, one
    /// squaring, one product).
    fn add_with_inv(&mut self, p1: &Pt, p2: &Pt, dxi: &X6) -> Pt {
        self.c.fp_adds += 42;
        let dy = self.sub(&p2.y, &p1.y);
        let lam = self.mul(&dy, dxi);
        let l2 = self.sq(&lam);
        let x3 = self.sub(&self.sub(&self.sub(&l2, &self.a2), &p1.x), &p2.x);
        let t = self.sub(&p1.x, &x3);
        let lt = self.mul(&lam, &t);
        let y3 = self.sub(&lt, &p1.y);
        Pt { x: x3, y: y3 }
    }
    /// Doubling (`None` for a point of order 2).
    fn dbl(&mut self, p1: &Pt) -> Option<Pt> {
        self.c.dbl_adds += 1;
        self.c.fp_adds += 66;
        if p1.y == [0; 6] {
            return None;
        }
        let x2 = self.sq(&p1.x);
        let a2x = self.mul(&self.add(&self.a2, &self.a2), &p1.x);
        let num = self.add(
            &self.add(&self.add(&x2, &self.add(&x2, &x2)), &a2x),
            &self.a4,
        );
        let den = self.inv(&self.add(&p1.y, &p1.y));
        let lam = self.mul(&num, &den);
        let l2 = self.sq(&lam);
        let x3 = self.sub(&self.sub(&self.sub(&l2, &self.a2), &p1.x), &p1.x);
        let t = self.sub(&p1.x, &x3);
        let lt = self.mul(&lam, &t);
        let y3 = self.sub(&lt, &p1.y);
        Some(Pt { x: x3, y: y3 })
    }
    /// Full addition with its own inversion; `None` for the point at infinity.
    fn add_full(&mut self, p1: &Pt, p2: &Pt) -> Option<Pt> {
        if p1.x == p2.x {
            return if p1.y == p2.y { self.dbl(p1) } else { None };
        }
        let dx = self.sub(&p2.x, &p1.x);
        let dxi = self.inv(&dx);
        Some(self.add_with_inv(p1, p2, &dxi))
    }
    fn mul_scalar(&mut self, pt: &Pt, mut k: u64) -> Option<Pt> {
        let mut acc: Option<Pt> = None;
        let mut base = Some(*pt);
        while k > 0 {
            if k & 1 == 1 {
                acc = match (acc, base) {
                    (None, b) => b,
                    (a, None) => a,
                    (Some(a), Some(b)) => self.add_full(&a, &b),
                };
            }
            k >>= 1;
            if k > 0 {
                base = base.and_then(|b| self.dbl(&b));
            }
        }
        acc
    }
}

fn to_x6(e: &E6) -> X6 {
    core::array::from_fn(|i| e.0[i / 2].0[i % 2])
}
fn to_pt(p: &PtE6) -> Pt {
    Pt {
        x: to_x6(&p.x),
        y: to_x6(&p.y),
    }
}
#[allow(dead_code)]
fn from_x6(x: &X6) -> E6 {
    E6([E2([x[0], x[1]]), E2([x[2], x[3]]), E2([x[4], x[5]])])
}

#[inline(always)]
fn hash_x(x: &X6) -> u64 {
    let mut h = 0u64;
    for (i, &v) in x.iter().enumerate() {
        h ^= v
            .wrapping_mul(0x9E37_79B9_7F4A_7C15 >> (i * 3))
            .rotate_left(11 * i as u32);
    }
    mix64(h)
}

/// Options of one arm.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct StrongOpts {
    pub negation: bool,
    /// walkers per batched inversion; `1` = one inversion per addition
    pub batch: usize,
    pub dp_bits: u32,
    /// multipliers of the walk
    pub r: usize,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct StrongRun {
    pub p: u64,
    pub seed: u64,
    pub run: u64,
    pub l: u64,
    pub negation: bool,
    pub batch: usize,
    pub dp_bits: u32,
    pub r: usize,
    pub c_add_e: f64,
    /// walk steps (group additions made by walkers, doublings included)
    pub steps: u64,
    pub ops: OpCounts,
    pub muls_harness: f64,
    pub muls_tuned: f64,
    /// `F_p` multiplications / (`c_add_e` · √ℓ), i.e. in units of √ℓ group
    /// additions of the harness
    pub s_harness: f64,
    pub s_tuned: f64,
    /// `S⁺`: every `F_p` addition and native operation charged one
    /// multiplication
    pub s_plus_harness: f64,
    pub s_plus_tuned: f64,
    pub restarts: u64,
    pub cycles_found: u64,
    pub self_collisions: u64,
    pub dps: u64,
    pub correct: bool,
    pub capped: bool,
    pub wall_ms: f64,
}

struct Walker {
    pt: Pt,
    a: u64,
    b: u64,
    tail: u64,
    // history for 2-cycle detection (negation arms)
    hist: std::collections::VecDeque<(u64, Pt, u64, u64)>,
}
const CYCLE_WINDOW: usize = 8;

/// Recover from equal x coordinates, accounting for the two possible y signs.
/// A zero denominator is an uninformative collision, not a recovered scalar.
fn collision_scalar(n: u64, same_y: bool, a: u64, b: u64, a2: u64, b2: u64) -> Option<u64> {
    let (num, den) = if same_y {
        (
            ((a2 as u128 + n as u128 - a as u128) % n as u128) as u64,
            ((b as u128 + n as u128 - b2 as u128) % n as u128) as u64,
        )
    } else {
        let sum = ((a as u128 + a2 as u128) % n as u128) as u64;
        ((n - sum) % n, ((b as u128 + b2 as u128) % n as u128) as u64)
    };
    (den != 0).then(|| mm(num, inv_mod(den, n), n))
}

/// One run of an arm on the instance `(p, instance_seed)`.
pub fn strong_rho(spec: &Spec, seed: u64, run: u64, o: StrongOpts) -> StrongRun {
    let start = Instant::now();
    let p = spec.p;
    let n = spec.l;
    let f = Fq3::new(p);
    let ec = EllE::new(&f, &spec.alpha);
    let c_e = {
        let ctx = Ctx::new(spec);
        unit_costs(spec, &ctx).0
    };
    let mut ar = Arith {
        r: Red::new(p),
        p,
        w: f.f.w,
        s: f.s.0,
        a2: to_x6(&ec.a2),
        a4: to_x6(&ec.a4),
        c: OpCounts::default(),
    };
    let g = to_pt(&spec.g);
    let q = to_pt(&spec.q);
    let mut rng = StdRng::seed_from_u64(
        seed.wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ run.wrapping_mul(0xD1B5_4A32_D192_ED03)
            ^ 0x57_20,
    );
    // setup counted into setup_adds: take the delta of group operations
    let mut setup_ops = 0u64;
    let lin_comb = |ar: &mut Arith, a: u64, b: u64, setup_ops: &mut u64| -> Option<Pt> {
        let before = ar.c.dbl_adds;
        let ga = ar.mul_scalar(&g, a);
        let qb = ar.mul_scalar(&q, b);
        // group operations of the scalar multiplications: count add_full calls
        // through the muls they used is not needed; count by doubling + adds
        let r = match (ga, qb) {
            (Some(x), Some(y)) => ar.add_full(&x, &y),
            (x, None) | (None, x) => x,
        };
        *setup_ops += (ar.c.dbl_adds - before) + (a.count_ones() + b.count_ones()) as u64 + 1;
        r
    };
    let mults: Vec<(u64, u64, Pt)> = (0..o.r)
        .map(|_| loop {
            let (al, be) = (rng.gen_range(0..n), rng.gen_range(1..n));
            if let Some(pt) = lin_comb(&mut ar, al, be, &mut setup_ops) {
                break (al, be, pt);
            }
        })
        .collect();
    let canon = |ar: &mut Arith, pt: Pt, a: u64, b: u64| -> (Pt, u64, u64) {
        if !o.negation {
            return (pt, a, b);
        }
        ar.c.canon_cmp += 1;
        let ny = ar.neg(&pt.y);
        if pt.y <= ny {
            (pt, a, b)
        } else {
            ar.c.fp_adds += 6;
            ar.c.scalar_ops += 2;
            (Pt { x: pt.x, y: ny }, (n - a) % n, (n - b) % n)
        }
    };
    let mask = (1u64 << o.dp_bits) - 1;
    let tail_cap = 20u64 << o.dp_bits;
    let cap = 64 * (n as f64).sqrt() as u64 + 1_000_000;
    let mut table: HashMap<X6, (X6, u64, u64)> = HashMap::new();
    let mut found: Option<u64> = None;
    let mut steps = 0u64;
    let (mut restarts, mut cycles_found, mut self_coll) = (0u64, 0u64, 0u64);
    let mut capped = false;

    let new_walker = |ar: &mut Arith, rng: &mut StdRng, setup_ops: &mut u64| -> Walker {
        loop {
            let (a, b) = (rng.gen_range(0..n), rng.gen_range(1..n));
            if let Some(pt) = lin_comb(ar, a, b, setup_ops) {
                let (pt, a, b) = canon(ar, pt, a, b);
                return Walker {
                    pt,
                    a,
                    b,
                    tail: 0,
                    hist: Default::default(),
                };
            }
        }
    };
    let mut ws: Vec<Walker> = (0..o.batch)
        .map(|_| new_walker(&mut ar, &mut rng, &mut setup_ops))
        .collect();
    'main: loop {
        // choose multipliers; collect the batch of x-differences
        let k = ws.len();
        let js: Vec<usize> = ws
            .iter()
            .map(|w| (hash_x(&w.pt.x) % o.r as u64) as usize)
            .collect();
        ar.c.hash += k as u64;
        let dxs: Vec<X6> = (0..k)
            .map(|i| ar.sub(&mults[js[i]].2.x, &ws[i].pt.x))
            .collect();
        let special: Vec<bool> = dxs.iter().map(|d| *d == [0; 6]).collect();
        // Montgomery's trick over the non-special walkers
        let idx: Vec<usize> = (0..k).filter(|&i| !special[i]).collect();
        let mut invs: Vec<X6> = vec![[0; 6]; k];
        if o.batch == 1 {
            for &i in &idx {
                invs[i] = ar.inv(&dxs[i]);
            }
        } else if !idx.is_empty() {
            let mut pref: Vec<X6> = Vec::with_capacity(idx.len());
            let mut acc = dxs[idx[0]];
            pref.push(acc);
            for &i in &idx[1..] {
                acc = ar.mul(&acc, &dxs[i]);
                pref.push(acc);
            }
            let mut inv_acc = ar.inv(&acc);
            for t in (1..idx.len()).rev() {
                let i = idx[t];
                invs[i] = ar.mul(&inv_acc, &pref[t - 1]);
                inv_acc = ar.mul(&inv_acc, &dxs[i]);
            }
            invs[idx[0]] = inv_acc;
        }
        for i in 0..k {
            let j = js[i];
            let (aj, bj, rj) = &mults[j];
            let next = if special[i] {
                ar.c.walk_adds += 1;
                ar.add_full(&ws[i].pt, rj)
            } else {
                ar.c.walk_adds += 1;
                Some(ar.add_with_inv(&ws[i].pt, rj, &invs[i]))
            };
            steps += 1;
            ar.c.scalar_ops += 2;
            let mut a = ws[i].a + aj;
            if a >= n {
                a -= n;
            }
            let mut b = ws[i].b + bj;
            if b >= n {
                b -= n;
            }
            let Some(npt) = next else {
                // infinity: restart this walker
                restarts += 1;
                ws[i] = new_walker(&mut ar, &mut rng, &mut setup_ops);
                continue;
            };
            let (mut npt, mut a, mut b) = canon(&mut ar, npt, a, b);
            // fruitless cycles under the negation map: the new point against
            // the last CYCLE_WINDOW points of this walker; a cycle is left by
            // doubling its point of smallest x (Bos–Kleinjung–Lenstra)
            if o.negation {
                let hn = hash_x(&npt.x);
                ar.c.hash += 1;
                ar.c.cycle_cmp += ws[i].hist.len() as u64;
                // hist[0] is the newest; distance d means P_{t+1} = P_{t+1-d}
                let hit = ws[i]
                    .hist
                    .iter()
                    .enumerate()
                    .skip(1)
                    .find(|(_, h)| h.0 == hn && h.1.x == npt.x)
                    .map(|(d, _)| d);
                if let Some(d) = hit {
                    cycles_found += 1;
                    let mut best = (npt, a, b);
                    for h in ws[i].hist.iter().take(d) {
                        if h.1.x < best.0.x {
                            best = (h.1, h.2, h.3);
                        }
                    }
                    if let Some(dd) = ar.dbl(&best.0) {
                        ar.c.scalar_ops += 2;
                        let (dd, da, db) = canon(&mut ar, dd, (2 * best.1) % n, (2 * best.2) % n);
                        npt = dd;
                        a = da;
                        b = db;
                    }
                    ws[i].hist.clear();
                }
                ar.c.hash += 1;
                ws[i].hist.push_front((hash_x(&npt.x), npt, a, b));
                ws[i].hist.truncate(CYCLE_WINDOW);
            }
            ws[i].pt = npt;
            ws[i].a = a;
            ws[i].b = b;
            ws[i].tail += 1;
            ar.c.dp_check += 1;
            let h = hash_x(&npt.x);
            ar.c.hash += 1;
            if (h >> 20) & mask == 0 {
                ar.c.table_ops += 1;
                if let Some(&(y2, a2, b2)) = table.get(&npt.x) {
                    if let Some(scalar) = collision_scalar(n, npt.y == y2, ws[i].a, ws[i].b, a2, b2)
                    {
                        found = Some(scalar);
                        break 'main;
                    }
                    self_coll += 1;
                    restarts += 1;
                    ws[i] = new_walker(&mut ar, &mut rng, &mut setup_ops);
                    continue;
                }
                table.insert(npt.x, (npt.y, ws[i].a, ws[i].b));
                ws[i].tail = 0;
            } else if ws[i].tail > tail_cap {
                restarts += 1;
                ws[i] = new_walker(&mut ar, &mut rng, &mut setup_ops);
            }
        }
        if steps > cap {
            capped = true;
            break;
        }
    }
    ar.c.setup_adds = setup_ops;
    let sqrt_l = (n as f64).sqrt();
    let ph = harness_prices(p);
    let mh = ar.c.muls(&ph);
    let mt = ar.c.muls(&TUNED);
    let extra = (ar.c.fp_adds + ar.c.native()) as f64;
    StrongRun {
        p,
        seed,
        run,
        l: n,
        negation: o.negation,
        batch: o.batch,
        dp_bits: o.dp_bits,
        r: o.r,
        c_add_e: c_e,
        steps,
        muls_harness: mh,
        muls_tuned: mt,
        s_harness: mh / (c_e * sqrt_l),
        s_tuned: mt / (c_e * sqrt_l),
        s_plus_harness: (mh + extra) / (c_e * sqrt_l),
        s_plus_tuned: (mt + extra) / (c_e * sqrt_l),
        restarts,
        cycles_found,
        self_collisions: self_coll,
        dps: table.len() as u64,
        correct: found == Some(spec.d),
        capped,
        ops: ar.c,
        wall_ms: start.elapsed().as_secs_f64() * 1e3,
    }
}

pub fn run_strong_rho(p: u64, seed: u64, run: u64, o: StrongOpts) -> StrongRun {
    let spec = generate_spec(p, seed);
    strong_rho(&spec, seed, run, o)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_points_recover_the_same_scalar() {
        // With d=7 modulo 101, (1,1) is 8G and (79,2) is -8G.
        // Treating them as equal points would return 23 instead of 7.
        assert_eq!(collision_scalar(101, false, 1, 1, 79, 2), Some(7));
        assert_eq!(collision_scalar(101, true, 1, 1, 95, 2), Some(7));
        assert_eq!(collision_scalar(101, true, 1, 1, 1, 1), None);
        assert_eq!(collision_scalar(101, false, 1, 1, 100, 100), None);
        for a in 0..101 {
            for b in 1..101 {
                let a2 = 13;
                // Independent brute-force point-coefficient oracle.
                for same in [false, true] {
                    let point = (a + 7 * b) % 101;
                    let other = if same { point } else { (101 - point) % 101 };
                    let b2 = (0..101).find(|&v| (a2 + 7 * v) % 101 == other).unwrap();
                    if let Some(d) = collision_scalar(101, same, a, b, a2, b2) {
                        assert_eq!(d, 7);
                    }
                }
            }
        }
    }

    #[test]
    fn native_arithmetic_matches_the_harness() {
        let p = 101;
        let spec = generate_spec(p, 1);
        let f = Fq3::new(p);
        let ec = EllE::new(&f, &spec.alpha);
        let mut rng = StdRng::seed_from_u64(3);
        let mut ar = Arith {
            r: Red::new(p),
            p,
            w: f.f.w,
            s: f.s.0,
            a2: to_x6(&ec.a2),
            a4: to_x6(&ec.a4),
            c: OpCounts::default(),
        };
        for _ in 0..200 {
            let a = f.random(&mut rng);
            let b = f.random(&mut rng);
            assert_eq!(ar.mul(&to_x6(&a), &to_x6(&b)), to_x6(&f.mul(&a, &b)));
            if a != E6::ZERO {
                assert_eq!(ar.inv(&to_x6(&a)), to_x6(&f.inv(&a)));
            }
        }
        // group law against the harness, additions and doublings
        let g = to_pt(&spec.g);
        let q = to_pt(&spec.q);
        for k in [2u64, 3, 17, 1000, 123_456_789] {
            let h = ec.mul(&spec.g, k);
            assert_eq!(ar.mul_scalar(&g, k).unwrap().x, to_x6(&h.x));
            let h2 = ec.add(&ec.mul(&spec.g, k), &spec.q);
            let n = ar.mul_scalar(&g, k).unwrap();
            assert_eq!(ar.add_full(&n, &q).unwrap().y, to_x6(&h2.y));
        }
    }

    #[test]
    fn harness_price_of_the_weak_walk_is_331() {
        let v = harness_prices(101);
        assert_eq!(2.0 * v.mul + v.sq + v.inv, 331.0);
    }

    #[test]
    fn every_arm_recovers_the_planted_logarithm() {
        let p = 53;
        for (neg, batch) in [(false, 1), (false, 8), (true, 1), (true, 8)] {
            let mut ok = 0;
            for seed in 1..=3u64 {
                let r = run_strong_rho(
                    p,
                    seed,
                    0,
                    StrongOpts {
                        negation: neg,
                        batch,
                        dp_bits: 4,
                        r: 64,
                    },
                );
                ok += r.correct as usize;
            }
            assert_eq!(ok, 3, "neg {neg} batch {batch}");
        }
    }
}
