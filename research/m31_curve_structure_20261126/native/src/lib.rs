//! Single-target ECDLP over GF(2^31 - 1): equal-engineering rho arms that differ
//! only in the equivalence classes the walk runs on.
//!
//!   plain : classes {P}            (the PR #5184 `rho_batched.rs` policy, re-implemented)
//!   neg   : classes {+-P}          (negation map, Wiener-Zuccherato look-ahead + restart on fruitless cycles)
//!   aut6  : classes {+-zeta^i P}   (j = 0 only: order-6 automorphism group, Duursma-Gaudry-Morain)
//!
//! Shared engineering in every arm: W parallel walks, one Montgomery batch
//! inversion per step, Mersenne reduction, 32-jump r-adding walk on the class
//! representative, distinguished points (low DP bits of the canonical x), the
//! same start-point schedule (S_k = (k+1)X, X = uG + vQ), the same restart rule.
//! Every field multiplication (squarings included) and inversion is counted.
//! Each recovered log is verified with an uncounted double-and-add (d*G == Q).
//!
//! Library: `solve(&Job) -> Solve` (one JSON line via `Solve::json`).

use std::collections::HashMap;
use std::time::Instant;

pub const P: u64 = (1u64 << 31) - 1;
const JUMPS: usize = 32;

#[derive(Default, Clone, Copy)]
pub struct Counts {
    pub mul: u64, // field multiplications, squarings included (squarings also tallied in sqr)
    pub sqr: u64,
    pub inv: u64,       // inversions (each also costs ~61 counted multiplications inside `mul`)
    pub canon_mul: u64, // multiplications spent on class canonicalisation (subset of mul)
}

pub struct F {
    pub c: Counts,
}
impl F {
    #[inline(always)]
    pub fn red(x: u64) -> u64 {
        let x = (x & P) + (x >> 31);
        let x = (x & P) + (x >> 31);
        if x >= P { x - P } else { x }
    }
    #[inline(always)]
    pub fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.c.mul += 1;
        Self::red(a * b)
    }
    #[inline(always)]
    pub fn sqr(&mut self, a: u64) -> u64 {
        self.c.sqr += 1;
        self.mul(a, a)
    }
    #[inline(always)]
    pub fn sub(a: u64, b: u64) -> u64 {
        if a >= b { a - b } else { a + P - b }
    }
    #[inline(always)]
    pub fn add(a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= P { s - P } else { s }
    }
    #[inline(always)]
    pub fn neg(a: u64) -> u64 {
        if a == 0 { 0 } else { P - a }
    }
    pub fn inv(&mut self, x: u64) -> u64 {
        assert!(x != 0);
        self.c.inv += 1;
        let (mut o, mut b, mut e) = (1u64, x, P - 2);
        while e > 0 {
            if e & 1 != 0 {
                o = self.mul(o, b);
            }
            b = self.sqr(b);
            e >>= 1;
        }
        o
    }
}

pub type Pt = Option<(u64, u64)>;

/// Counted affine addition with its own inversion (setup and restarts only).
pub fn add_counted(f: &mut F, a_coef: u64, p1: Pt, p2: Pt) -> Pt {
    let (a, b) = match (p1, p2) {
        (None, q) => return q,
        (q, None) => return q,
        (Some(a), Some(b)) => (a, b),
    };
    let l = if a.0 == b.0 {
        if F::add(a.1, b.1) == 0 {
            return None;
        }
        let x2 = f.sqr(a.0);
        let num = F::add(F::add(F::add(x2, x2), x2), a_coef);
        let d = f.inv(F::add(a.1, a.1));
        f.mul(num, d)
    } else {
        let d = f.inv(F::sub(b.0, a.0));
        f.mul(F::sub(b.1, a.1), d)
    };
    let x3 = F::sub(F::sub(f.sqr(l), a.0), b.0);
    let y3 = F::sub(f.mul(l, F::sub(a.0, x3)), a.1);
    Some((x3, y3))
}
pub fn scalar_counted(f: &mut F, a_coef: u64, mut k: u64, mut pt: Pt) -> Pt {
    let mut r = None;
    while k > 0 {
        if k & 1 != 0 {
            r = add_counted(f, a_coef, r, pt);
        }
        pt = add_counted(f, a_coef, pt, pt);
        k >>= 1;
    }
    r
}
/// Uncounted reference arithmetic for verification only.
pub fn scalar_ref(a_coef: u64, k: u64, pt: Pt) -> Pt {
    let mut f = F { c: Counts::default() };
    scalar_counted(&mut f, a_coef, k, pt)
}

pub fn splitmix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
pub fn mulmod(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
pub fn powmod(mut a: u64, mut e: u64, n: u64) -> u64 {
    let mut o = 1;
    while e > 0 {
        if e & 1 != 0 {
            o = mulmod(o, a, n);
        }
        a = mulmod(a, a, n);
        e >>= 1;
    }
    o
}
/// Hot-loop scalar arithmetic mod the group order without division (inputs < n < 2^32).
#[inline(always)]
fn addn(a: u64, b: u64, n: u64) -> u64 {
    let s = a + b;
    if s >= n { s - n } else { s }
}
#[inline(always)]
fn negn(a: u64, n: u64) -> u64 {
    if a == 0 { 0 } else { n - a }
}

#[inline(always)]
fn jump_of(x: u64) -> usize {
    ((x.wrapping_mul(0x9e37_79b9_7f4a_7c15)) >> 59) as usize
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Arm {
    Plain,
    Neg,
    Aut6,
}

#[derive(Clone, Copy)]
struct Walk {
    x: u64,
    y: u64,
    a: u64,
    b: u64,
    off: usize, // look-ahead offset (neg / aut6)
    since_dp: u64,
}

struct Ctx {
    arm: Arm,
    n: u64,
    beta: u64,
    beta2: u64,
    lambda: u64,
    lambda2: u64,
    lams: [u64; 3],
    barrett_m: u64,
}
impl Ctx {
    /// x mod n for x < 2^62, n < 2^32 (Barrett, one 64x64->128 multiply).
    #[inline(always)]
    fn barrett(&self, x: u64) -> u64 {
        let q = ((x as u128 * self.barrett_m as u128) >> 64) as u64;
        let mut r = x - q * self.n;
        while r >= self.n {
            r -= self.n;
        }
        r
    }
}

impl Ctx {
    /// Canonical class representative; coefficients follow the automorphism applied.
    #[inline(always)]
    fn canon(&self, f: &mut F, w: &mut Walk) {
        match self.arm {
            Arm::Plain => {}
            Arm::Neg => {
                // branch-free select (the sign is a fair coin, so a branch mispredicts half the time)
                let mask = 0u64.wrapping_sub((w.y > P / 2) as u64);
                w.y ^= (w.y ^ (P - w.y)) & mask;
                w.a ^= (w.a ^ negn(w.a, self.n)) & mask;
                w.b ^= (w.b ^ negn(w.b, self.n)) & mask;
            }
            Arm::Aut6 => {
                // orbit of x under zeta: {x, beta x, beta^2 x}, beta^2 x = -x - beta x (one mult);
                // branch-free selection, Barrett reduction for the coefficient scaling.
                let bx = f.mul(self.beta, w.x);
                f.c.canon_mul += 1;
                let b2x = F::neg(F::add(w.x, bx));
                let c1 = (bx < w.x) as usize;
                let m01 = if c1 == 1 { bx } else { w.x };
                let c2 = (b2x < m01) as usize;
                let mu = c2 * 2 + (1 - c2) * c1;
                let mx = if c2 == 1 { b2x } else { m01 };
                let lam = self.lams[mu];
                w.a = self.barrett(w.a * lam);
                w.b = self.barrett(w.b * lam);
                w.x = mx;
                let mask = 0u64.wrapping_sub((w.y > P / 2) as u64);
                w.y ^= (w.y ^ (P - w.y)) & mask;
                w.a ^= (w.a ^ negn(w.a, self.n)) & mask;
                w.b ^= (w.b ^ negn(w.b, self.n)) & mask;
                let _ = (self.beta2, self.lambda, self.lambda2);
            }
        }
    }
}

/// The walk function applied to one walk with its own inversion (cycle replay only; counted).
fn step_single(ctx: &Ctx, f: &mut F, acoef: u64, jumps: &[((u64, u64), u64, u64)], w: Walk) -> Walk {
    let n = ctx.n;
    let mut off = 0usize;
    loop {
        let j = (jump_of(w.x) + off) % JUMPS;
        let (jp, c, d) = jumps[j];
        let s = add_counted(f, acoef, Some((w.x, w.y)), Some(jp));
        let (x3, y3) = match s {
            Some(v) => v,
            None => return w, // cannot occur inside a fruitless cycle; caller's confirmation fails
        };
        let mut nw = Walk { x: x3, y: y3, a: addn(w.a, c, n), b: addn(w.b, d, n), off: 0, since_dp: w.since_dp + 1 };
        ctx.canon(f, &mut nw);
        if ctx.arm != Arm::Plain && off + 1 < JUMPS && jump_of(nw.x) == j {
            off += 1;
            continue;
        }
        return nw;
    }
}

pub fn parse_arm(s: &str) -> Arm {
    match s {
        "plain" => Arm::Plain,
        "neg" => Arm::Neg,
        "aut6" => Arm::Aut6,
        _ => panic!("arm"),
    }
}

/// One single-target solve request (public data only).
#[derive(Clone)]
pub struct Job {
    pub arm: Arm,
    pub a: u64,
    pub b: u64,
    pub n: u64,
    pub g: (u64, u64),
    pub q: (u64, u64),
    pub seed: u64,
    pub walks: usize,
    pub dp_bits: u32,
    pub beta: u64,
    pub lambda: u64,
}

pub struct Solve {
    pub recovered: u64,
    pub verified: bool,
    pub steps: u64,
    pub json: String,
}

pub fn solve(job: &Job) -> Solve {
    let arm = job.arm;
    let (acoef, bcoef, n) = (job.a, job.b, job.n);
    let g: Pt = Some(job.g);
    let q: Pt = Some(job.q);
    let seed = job.seed;
    let w_n = job.walks;
    let dp_bits = job.dp_bits;
    let (beta, lambda) = (job.beta, job.lambda);
    let on_curve = |p: Pt| {
        let (x, y) = p.unwrap();
        let r = F::red;
        r(y * y) == F::add(F::add(r(r(x * x) * x), r(acoef * x)), bcoef)
    };
    assert!(on_curve(g) && on_curve(q), "inputs must lie on the curve");
    assert!(scalar_ref(acoef, n, g).is_none(), "G must have order n");
    let ctx = Ctx {
        arm,
        n,
        beta,
        beta2: F::red(beta * beta),
        lambda,
        lambda2: mulmod(lambda, lambda, n),
        lams: [1, lambda % n.max(1), mulmod(lambda, lambda, n.max(1))],
        barrett_m: (u64::MAX / n),
    };
    if arm == Arm::Aut6 {
        let (gx, gy) = g.unwrap();
        assert!(bcoef != 0 && acoef == 0, "aut6 needs j = 0");
        assert_eq!(scalar_ref(acoef, lambda, g), Some((F::red(beta * gx), gy)), "lambda must match beta on G");
    }
    let cold = Instant::now();
    let mut f = F { c: Counts::default() };
    let mut s = seed;
    let mut setup = F { c: Counts::default() };
    // Jump table R_j = c_j G + d_j Q (counted as setup).
    let mut jumps = Vec::with_capacity(JUMPS);
    for _ in 0..JUMPS {
        let (c, d) = (splitmix(&mut s) % n, splitmix(&mut s) % n);
        let u = scalar_counted(&mut setup, acoef, c, g);
        let w = scalar_counted(&mut setup, acoef, d, q);
        let pt = add_counted(&mut setup, acoef, u, w).unwrap();
        jumps.push((pt, c, d));
    }
    // Start schedule S_k = (k+1) X, X = uG + vQ.
    let (xu, xv) = (splitmix(&mut s) % n, splitmix(&mut s) % n);
    let xpt = {
        let a1 = scalar_counted(&mut setup, acoef, xu, g);
        let a2 = scalar_counted(&mut setup, acoef, xv, q);
        add_counted(&mut setup, acoef, a1, a2).unwrap()
    };
    let mut cur: (Pt, u64, u64) = (None, 0, 0);
    let mut next_start = |setup: &mut F| -> Walk {
        loop {
            cur.0 = add_counted(setup, acoef, cur.0, Some(xpt));
            cur.1 = (cur.1 + xu) % n;
            cur.2 = (cur.2 + xv) % n;
            if let Some((x, y)) = cur.0 {
                return Walk { x, y, a: cur.1, b: cur.2, off: 0, since_dp: 0 };
            }
        }
    };
    let mut walks: Vec<Walk> = Vec::with_capacity(w_n);
    for _ in 0..w_n {
        let mut w = next_start(&mut setup);
        ctx.canon(&mut setup, &mut w);
        walks.push(w);
    }
    let setup_counts = setup.c;
    let mut restart_f = F { c: Counts::default() };
    let max_len: u64 = 16u64 << dp_bits;
    let dp_mask = (1u64 << dp_bits) - 1;
    let mut dps: HashMap<(u64, u64), (u64, u64)> = HashMap::new();
    let (mut steps, mut accepted, mut lookahead_rejects) = (0u64, 0u64, 0u64);
    let (mut restarts_cycle, mut restarts_selfhit, mut restarts_special) = (0u64, 0u64, 0u64);
    let mut dens = vec![0u64; w_n];
    let mut pre = vec![0u64; w_n];
    let mut jidx = vec![0usize; w_n];
    let use_la = arm != Arm::Plain;
    const RING: usize = 12;
    const FULL_EVERY: u64 = 16;
    let mut false_cycle_alarms = 0u64;
    let mut ring_pos = vec![0usize; w_n];
    let mut rx: Vec<[u64; RING]> = vec![[u64::MAX; RING]; w_n];
    let mut cycle_escapes = 0u64;
    let mut cycle_len_hist = [0u64; RING + 1];
    let mut ring_compares = 0u64;
    let walk_t0 = Instant::now();
    let recovered = 'outer: loop {
        for k in 0..w_n {
            let w = walks[k];
            let j = (jump_of(w.x) + w.off) % JUMPS;
            jidx[k] = j;
            let d = F::sub(jumps[j].0 .0, w.x);
            dens[k] = if d == 0 { 1 } else { d };
        }
        let mut acc = dens[0];
        pre[0] = acc;
        for k in 1..w_n {
            acc = f.mul(acc, dens[k]);
            pre[k] = acc;
        }
        let mut iv = f.inv(acc);
        for k in (1..w_n).rev() {
            let dk = dens[k];
            dens[k] = f.mul(iv, pre[k - 1]);
            iv = f.mul(iv, dk);
        }
        dens[0] = iv;
        for k in 0..w_n {
            let w = walks[k];
            let j = jidx[k];
            let (jp, c, d) = jumps[j];
            steps += 1;
            if jp.0 == w.x {
                restarts_special += 1;
                let mut nw = next_start(&mut restart_f);
                ctx.canon(&mut restart_f, &mut nw);
                walks[k] = nw;
                continue;
            }
            let l = f.mul(F::sub(jp.1, w.y), dens[k]);
            let x3 = F::sub(F::sub(f.sqr(l), w.x), jp.0);
            let y3 = F::sub(f.mul(l, F::sub(w.x, x3)), w.y);
            let mut nw = Walk { x: x3, y: y3, a: addn(w.a, c, n), b: addn(w.b, d, n), off: 0, since_dp: w.since_dp + 1 };
            ctx.canon(&mut f, &mut nw);
            if use_la && w.off + 1 < JUMPS && jump_of(nw.x) == j {
                // Wiener-Zuccherato look-ahead: reject, retry from w with the next jump.
                lookahead_rejects += 1;
                walks[k].off += 1;
                walks[k].since_dp += 1;
                continue;
            }
            accepted += 1;
            if use_la {
                // x-only ring of the last RING accepted x per walk. Every accepted step compares
                // with the x of 2 and 4 steps ago (the common fruitless 2- and 4-cycles); every
                // FULL_EVERY accepted steps the whole ring is scanned (longer cycles). On a hit at
                // distance L the cycle is replayed with the same walk function (unbatched, counted)
                // and left deterministically by doubling its minimum-x point.
                let pos = ring_pos[k];
                let at = |d: usize| rx[k][(pos + RING - d) % RING];
                let mut found = 0usize;
                ring_compares += 2;
                if at(2) == nw.x {
                    found = 2;
                } else if at(4) == nw.x {
                    found = 4;
                } else if accepted % FULL_EVERY == 0 {
                    ring_compares += RING as u64;
                    for dist in 1..=RING {
                        if at(dist) == nw.x {
                            found = dist;
                            break;
                        }
                    }
                }
                if found > 0 {
                    // confirm the cycle by replay: L steps from nw must return to nw
                    let mut cyc = vec![nw];
                    let mut cur_w = nw;
                    for _ in 0..found {
                        cur_w = step_single(&ctx, &mut restart_f, acoef, &jumps, cur_w);
                        cyc.push(cur_w);
                    }
                    if cur_w.x == nw.x && cur_w.y == nw.y {
                        cycle_escapes += 1;
                        cycle_len_hist[found] += 1;
                        let m = *cyc[..found].iter().min_by_key(|e| e.x).unwrap();
                        let d2 = add_counted(&mut restart_f, acoef, Some((m.x, m.y)), Some((m.x, m.y)));
                        let mut esc = match d2 {
                            Some((x, y)) => Walk { x, y, a: addn(m.a, m.a, n), b: addn(m.b, m.b, n), off: 0, since_dp: nw.since_dp },
                            None => next_start(&mut restart_f),
                        };
                        ctx.canon(&mut restart_f, &mut esc);
                        rx[k] = [u64::MAX; RING];
                        walks[k] = esc;
                        continue;
                    }
                    false_cycle_alarms += 1;
                }
                rx[k][pos] = nw.x;
                ring_pos[k] = if pos + 1 == RING { 0 } else { pos + 1 };
            }
            if nw.since_dp > max_len {
                // walk trapped in a fruitless cycle (or pathologically long): restart, charged.
                restarts_cycle += 1;
                let mut r = next_start(&mut restart_f);
                ctx.canon(&mut restart_f, &mut r);
                walks[k] = r;
                continue;
            }
            if nw.x & dp_mask == 0 {
                nw.since_dp = 0;
                let key = (nw.x, nw.y);
                if let Some(&(a2, b2)) = dps.get(&key) {
                    if b2 != nw.b {
                        let num = (a2 + n - nw.a) % n;
                        let den = (nw.b + n - b2) % n;
                        break 'outer mulmod(num, powmod(den, n - 2, n), n);
                    }
                    restarts_selfhit += 1;
                    let mut r = next_start(&mut restart_f);
                    ctx.canon(&mut restart_f, &mut r);
                    walks[k] = r;
                    continue;
                }
                dps.insert(key, (nw.a, nw.b));
            }
            walks[k] = nw;
        }
    };
    let walk_wall_ms = walk_t0.elapsed().as_secs_f64() * 1e3;
    let wall_ms = cold.elapsed().as_secs_f64() * 1e3;
    let verified = scalar_ref(acoef, recovered, g) == q;
    let arm_s = match arm {
        Arm::Plain => "plain",
        Arm::Neg => "neg",
        Arm::Aut6 => "aut6",
    };
    let wc = f.c;
    let rc = restart_f.c;
    let json = format!(
        "{{\"arm\":\"{}\",\"seed\":\"{:#018x}\",\"W\":{},\"dp_bits\":{},\"steps\":{},\"accepted_steps\":{},\"lookahead_rejects\":{},\
\"restarts_cycle\":{},\"cycle_escapes\":{},\"cycle_len_hist\":{:?},\"ring_compares\":{},\"false_cycle_alarms\":{},\"restarts_selfhit\":{},\"restarts_special\":{},\"distinguished_points\":{},\
\"walk_mul\":{},\"walk_sqr\":{},\"walk_inv\":{},\"canon_mul\":{},\"restart_mul\":{},\"restart_inv\":{},\
\"setup_mul\":{},\"setup_inv\":{},\"total_mul\":{},\"recovered\":{},\"verified\":{},\"wall_ms\":{:.3},\"walk_wall_ms\":{:.3}}}",
        arm_s, seed, w_n, dp_bits, steps, accepted, lookahead_rejects, restarts_cycle, cycle_escapes, cycle_len_hist, ring_compares, false_cycle_alarms, restarts_selfhit, restarts_special,
        dps.len(), wc.mul, wc.sqr, wc.inv, wc.canon_mul, rc.mul, rc.inv, setup_counts.mul, setup_counts.inv,
        wc.mul + rc.mul + setup_counts.mul, recovered, verified, wall_ms, walk_wall_ms
    );
    Solve { recovered, verified, steps, json }
}

pub mod structure;
pub mod kdf;
