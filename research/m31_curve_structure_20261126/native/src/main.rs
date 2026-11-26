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
//! Usage: m31-structure-rho <arm> <a> <b> <order> <gx> <gy> <qx> <qy> <seed_hex> <W> <dp_bits> [beta] [lambda]
//! Output: one JSON line. No dependencies.

use std::collections::HashMap;
use std::time::Instant;

const P: u64 = (1u64 << 31) - 1;
const JUMPS: usize = 32;

#[derive(Default, Clone, Copy)]
struct Counts {
    mul: u64, // field multiplications, squarings included (squarings also tallied in sqr)
    sqr: u64,
    inv: u64,       // inversions (each also costs ~61 counted multiplications inside `mul`)
    canon_mul: u64, // multiplications spent on class canonicalisation (subset of mul)
}

struct F {
    c: Counts,
}
impl F {
    #[inline(always)]
    fn red(x: u64) -> u64 {
        let x = (x & P) + (x >> 31);
        let x = (x & P) + (x >> 31);
        if x >= P { x - P } else { x }
    }
    #[inline(always)]
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.c.mul += 1;
        Self::red(a * b)
    }
    #[inline(always)]
    fn sqr(&mut self, a: u64) -> u64 {
        self.c.sqr += 1;
        self.mul(a, a)
    }
    #[inline(always)]
    fn sub(a: u64, b: u64) -> u64 {
        if a >= b { a - b } else { a + P - b }
    }
    #[inline(always)]
    fn add(a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= P { s - P } else { s }
    }
    #[inline(always)]
    fn neg(a: u64) -> u64 {
        if a == 0 { 0 } else { P - a }
    }
    fn inv(&mut self, x: u64) -> u64 {
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

type Pt = Option<(u64, u64)>;

/// Counted affine addition with its own inversion (setup and restarts only).
fn add_counted(f: &mut F, a_coef: u64, p1: Pt, p2: Pt) -> Pt {
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
fn scalar_counted(f: &mut F, a_coef: u64, mut k: u64, mut pt: Pt) -> Pt {
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
fn scalar_ref(a_coef: u64, k: u64, pt: Pt) -> Pt {
    let mut f = F { c: Counts::default() };
    scalar_counted(&mut f, a_coef, k, pt)
}

fn splitmix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
fn mulmod(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn powmod(mut a: u64, mut e: u64, n: u64) -> u64 {
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
#[inline(always)]
fn jump_of(x: u64) -> usize {
    ((x.wrapping_mul(0x9e37_79b9_7f4a_7c15)) >> 59) as usize
}

#[derive(Clone, Copy, PartialEq)]
enum Arm {
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
}

impl Ctx {
    /// Canonical class representative; coefficients follow the automorphism applied.
    #[inline(always)]
    fn canon(&self, f: &mut F, w: &mut Walk) {
        match self.arm {
            Arm::Plain => {}
            Arm::Neg => {
                if w.y > P / 2 {
                    w.y = P - w.y;
                    w.a = (self.n - w.a) % self.n;
                    w.b = (self.n - w.b) % self.n;
                }
            }
            Arm::Aut6 => {
                // orbit of x under zeta: {x, beta x, beta^2 x}, beta^2 x = -x - beta x (one mult)
                let bx = f.mul(self.beta, w.x);
                f.c.canon_mul += 1;
                let b2x = F::neg(F::add(w.x, bx));
                let (mut mx, mut mu) = (w.x, 0u8);
                if bx < mx {
                    mx = bx;
                    mu = 1;
                }
                if b2x < mx {
                    mx = b2x;
                    mu = 2;
                }
                // (beta x, y) = lambda * (x, y) on the order-n subgroup
                if mu == 1 {
                    w.a = mulmod(w.a, self.lambda, self.n);
                    w.b = mulmod(w.b, self.lambda, self.n);
                } else if mu == 2 {
                    w.a = mulmod(w.a, self.lambda2, self.n);
                    w.b = mulmod(w.b, self.lambda2, self.n);
                }
                w.x = mx;
                if w.y > P / 2 {
                    w.y = P - w.y;
                    w.a = (self.n - w.a) % self.n;
                    w.b = (self.n - w.b) % self.n;
                }
                let _ = self.beta2;
            }
        }
    }
}

fn arg(v: &[String], i: usize) -> u64 {
    let s = &v[i];
    if let Some(h) = s.strip_prefix("0x") {
        u64::from_str_radix(h, 16).unwrap()
    } else {
        s.parse().unwrap()
    }
}

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let arm = match v[1].as_str() {
        "plain" => Arm::Plain,
        "neg" => Arm::Neg,
        "aut6" => Arm::Aut6,
        _ => panic!("arm"),
    };
    let (acoef, bcoef, n) = (arg(&v, 2), arg(&v, 3), arg(&v, 4));
    let g: Pt = Some((arg(&v, 5), arg(&v, 6)));
    let q: Pt = Some((arg(&v, 7), arg(&v, 8)));
    let seed = arg(&v, 9);
    let w_n = arg(&v, 10) as usize;
    let dp_bits = arg(&v, 11) as u32;
    let (beta, lambda) = if v.len() > 13 { (arg(&v, 12), arg(&v, 13)) } else { (0, 0) };
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
    // Fruitless-cycle detection (neg / aut6): ring of the last RING accepted states per walk;
    // a repeat at distance L <= RING is a fruitless L-cycle, escaped deterministically by
    // doubling the cycle's minimum-x point (counted with its own inversion).
    const RING: usize = 12;
    let mut ring: Vec<[Walk; RING]> = vec![[Walk { x: u64::MAX, y: 0, a: 0, b: 0, off: 0, since_dp: 0 }; RING]; w_n];
    let mut ring_pos = vec![0usize; w_n];
    let mut cycle_escapes = 0u64;
    let mut cycle_len_hist = [0u64; RING + 1];
    let mut ring_compares = 0u64;
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
            let mut nw = Walk { x: x3, y: y3, a: (w.a + c) % n, b: (w.b + d) % n, off: 0, since_dp: w.since_dp + 1 };
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
                let pos = ring_pos[k];
                let mut found = 0usize;
                for dist in 1..=RING {
                    ring_compares += 1;
                    let e = &ring[k][(pos + RING - dist) % RING];
                    if e.x == nw.x && e.y == nw.y {
                        found = dist;
                        break;
                    }
                }
                if found > 0 {
                    cycle_escapes += 1;
                    cycle_len_hist[found] += 1;
                    let mut m = nw;
                    for dist in 1..found {
                        let e = ring[k][(pos + RING - dist) % RING];
                        if e.x < m.x {
                            m = e;
                        }
                    }
                    let d2 = add_counted(&mut restart_f, acoef, Some((m.x, m.y)), Some((m.x, m.y)));
                    let mut esc = match d2 {
                        Some((x, y)) => Walk { x, y, a: (m.a * 2) % n, b: (m.b * 2) % n, off: 0, since_dp: nw.since_dp },
                        None => next_start(&mut restart_f),
                    };
                    ctx.canon(&mut restart_f, &mut esc);
                    for e in ring[k].iter_mut() {
                        e.x = u64::MAX;
                    }
                    walks[k] = esc;
                    continue;
                }
                ring[k][pos] = nw;
                ring_pos[k] = (pos + 1) % RING;
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
    let wall_ms = cold.elapsed().as_secs_f64() * 1e3;
    let verified = scalar_ref(acoef, recovered, g) == q;
    let arm_s = match arm {
        Arm::Plain => "plain",
        Arm::Neg => "neg",
        Arm::Aut6 => "aut6",
    };
    let wc = f.c;
    let rc = restart_f.c;
    println!(
        "{{\"arm\":\"{}\",\"seed\":\"{:#018x}\",\"W\":{},\"dp_bits\":{},\"steps\":{},\"accepted_steps\":{},\"lookahead_rejects\":{},\
\"restarts_cycle\":{},\"cycle_escapes\":{},\"cycle_len_hist\":{:?},\"ring_compares\":{},\"restarts_selfhit\":{},\"restarts_special\":{},\"distinguished_points\":{},\
\"walk_mul\":{},\"walk_sqr\":{},\"walk_inv\":{},\"canon_mul\":{},\"restart_mul\":{},\"restart_inv\":{},\
\"setup_mul\":{},\"setup_inv\":{},\"total_mul\":{},\"recovered\":{},\"verified\":{},\"wall_ms\":{:.3}}}",
        arm_s, seed, w_n, dp_bits, steps, accepted, lookahead_rejects, restarts_cycle, cycle_escapes, cycle_len_hist, ring_compares, restarts_selfhit, restarts_special,
        dps.len(), wc.mul, wc.sqr, wc.inv, wc.canon_mul, rc.mul, rc.inv, setup_counts.mul, setup_counts.inv,
        wc.mul + rc.mul + setup_counts.mul, recovered, verified, wall_ms
    );
    assert!(verified, "recovered log must verify");
}
