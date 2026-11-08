//! `rho`: Pollard rho with the negation map on <G> (prime order n).
//!
//! The lab's `rho.negation` (src/cryptanalysis/ic_wide.rs `rho_negation`,
//! generic over `CountedGroup`) could not be linked: `crypto_lib` does not
//! resolve its dependencies offline here (aws-credential-types is not in the
//! crates cache), and it is a 900k-line crate. This is a re-implementation of
//! the same walk family with the differences listed in the README.
//!
//! Walk (r = 32 additions):
//!   * canonical representative of {P, -P}: the one with y <= (p-1)/2; key = x.
//!   * jump table M_j = u_j G + v_j Q, 32 distinct pairs (u_j, v_j) in [0,16)^2
//!     \ {(0,0)}, built from the tables iG, iQ (i < 16); index = (mix64(x)>>40) mod 32.
//!   * step: P <- canon(P + M_j), tracking (a, b) with P = aG + bQ (mod n);
//!     canonicalisation flips (a, b) -> (-a, -b).
//!   * fruitless cycles (Bernstein-Lange-Schwabe 2011): keep the last 16
//!     canonical points of the walk; when the new point's x equals one of them,
//!     the cycle is the window tail from that point; double the cycle member with
//!     the smallest x (the same point for every walker that enters this cycle)
//!     and continue from it. If the next escape from the same walk picks the same
//!     minimum, the walk is abandoned (counted). 
//!   * distinguished points: mix64(x) & (2^dp - 1) == 0, dp = floor(log2 n / 4), min 1.
//!     Walks start at S_0 + i*D (one addition each, S_0 = aG + bQ and D = dG + eQ
//!     uniform with full-size coefficients), end at a DP or at the length cap 20*2^dp + 64.
//!   * collision on a DP with b != b': k = (a - a')/(b' - b) mod n, verified [k]G = Q.
//! Every non-trivial group addition/doubling is counted.
use crate::curves::{load, CurveRec};
use crate::ec::*;
use crate::util::*;
use std::collections::{HashMap, VecDeque};

pub const JUMPS: usize = 32;
const WINDOW: usize = 16;

#[derive(Clone, Debug, Default)]
pub struct Out {
    pub ops_total: u64,
    pub ops_setup: u64,
    pub ops_start: u64,
    pub ops_walk: u64,
    pub ops_verify: u64,
    pub escapes: u64,
    pub abandoned: u64,
    pub capped: u64,
    pub walks: u64,
    pub dps: u64,
    pub useless: u64,
    pub verified: bool,
    pub recovered_matches: bool,
}

pub fn dp_bits_for(n: u64) -> u32 {
    (((63 - n.leading_zeros()) / 4).max(1)) as u32
}

fn canon(c: &Curve, pt: (u64, u64), a: u64, b: u64, n: u64) -> ((u64, u64), u64, u64) {
    let (x, y) = pt;
    if y > c.p - y {
        ((x, c.p - y), (n - a) % n, (n - b) % n)
    } else {
        (pt, a, b)
    }
}

/// One rho solve of Q = [k]G. `cap_ops` bounds total group operations.
pub fn rho_one(c: &Curve, g: (u64, u64), q: (u64, u64), n: u64, k_true: u64, seed: u64, cap_ops: u64) -> Out {
    let mut rng = Rng::new(seed);
    let mut o = Out::default();
    let (mut s_setup, mut s_start, mut s_walk, mut s_ver) = (0u64, 0u64, 0u64, 0u64);
    // multiples iG, iQ, i < 16
    let mut tg: Vec<Pt> = vec![None, Some(g)];
    let mut tq: Vec<Pt> = vec![None, Some(q)];
    for i in 2..16 {
        let a = c.add(&mut s_setup, tg[i - 1], Some(g));
        tg.push(a);
        let b = c.add(&mut s_setup, tq[i - 1], Some(q));
        tq.push(b);
    }
    let mut jumps: Vec<((u64, u64), u64, u64)> = vec![];
    let mut seen = std::collections::HashSet::new();
    while jumps.len() < JUMPS {
        let (u, v) = (rng.below(16), rng.below(16));
        if (u == 0 && v == 0) || !seen.insert((u, v)) {
            continue;
        }
        if let Some(m) = c.add(&mut s_setup, tg[u as usize], tq[v as usize]) {
            jumps.push((m, u, v));
        }
    }
    // S_0 = a0 G + b0 Q uniform; stride D = u G + v Q with small u, v >= 1.
    let (a0, b0) = (rng.below(n), rng.below(n));
    let m1 = c.mul(&mut s_setup, a0, Some(g));
    let m2 = c.mul(&mut s_setup, b0, Some(q));
    let mut s_pt = c.add(&mut s_setup, m1, m2);
    let (mut sa, mut sb) = (a0, b0);
    // Stride D = du G + dv Q with FULL-size random (du, dv). Small (du, dv) in the
    // lattice of the jump coefficients made different walks reach the same point
    // with identical (a, b) (useless collisions, first draft of this file).
    let (du, dv) = (rng.below(n), rng.below(n));
    let m3 = c.mul(&mut s_setup, du, Some(g));
    let m4 = c.mul(&mut s_setup, dv, Some(q));
    let stride = c.add(&mut s_setup, m3, m4);
    let dp_bits = dp_bits_for(n);
    let dp_mask = (1u64 << dp_bits) - 1;
    let walk_cap = (1u64 << dp_bits) * 20 + 64;
    let mut table: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut walks = 0u64;
    let total = |a: u64, b: u64, d: u64, e: u64| a + b + d + e;
    'outer: while total(s_setup, s_start, s_walk, s_ver) < cap_ops {
        if walks > 0 {
            s_pt = c.add(&mut s_start, s_pt, stride);
            sa = (sa + du) % n;
            sb = (sb + dv) % n;
        }
        walks += 1;
        let Some(start) = s_pt else { continue };
        let (mut pt, mut a, mut b) = canon(c, start, sa, sb, n);
        let mut win: VecDeque<(u64, (u64, u64), u64, u64)> = VecDeque::with_capacity(WINDOW + 1);
        let mut last_escape: Option<u64> = None;
        let mut len = 0u64;
        let end: Option<((u64, u64), u64, u64)> = loop {
            let j = ((mix64(pt.0) >> 40) as usize) & (JUMPS - 1);
            let (m, uj, vj) = jumps[j];
            let sum = c.add(&mut s_walk, Some(pt), Some(m));
            let Some(sp) = sum else { break None };
            let (np, na, nb) = canon(c, sp, (a + uj) % n, (b + vj) % n, n);
            pt = np;
            a = na;
            b = nb;
            len += 1;
            if let Some(pos) = win.iter().position(|e| e.0 == pt.0) {
                // fruitless cycle: win[pos..] are its members
                let (min_key, mp, ma, mb) = win.iter().skip(pos).copied().min_by_key(|e| e.0).unwrap();
                if last_escape == Some(min_key) {
                    o.abandoned += 1;
                    break None;
                }
                last_escape = Some(min_key);
                o.escapes += 1;
                let d2 = c.dbl(&mut s_walk, Some(mp));
                let Some(d2) = d2 else { break None };
                let (ep, ea, eb) = canon(c, d2, (2 * ma) % n, (2 * mb) % n, n);
                pt = ep;
                a = ea;
                b = eb;
                win.clear();
            }
            win.push_back((pt.0, pt, a, b));
            if win.len() > WINDOW {
                win.pop_front();
            }
            if mix64(pt.0) & dp_mask == 0 {
                break Some((pt, a, b));
            }
            if len > walk_cap {
                o.capped += 1;
                break None;
            }
            if total(s_setup, s_start, s_walk, s_ver) >= cap_ops {
                break None;
            }
        };
        let Some((dp, a, b)) = end else { continue };
        o.dps += 1;
        match table.get(&dp.0) {
            Some(&(a2, b2)) => {
                if b2 != b {
                    let num = (a + n - a2) % n;
                    let den = (b2 + n - b) % n;
                    if let Some(iv) = invmod(den, n) {
                        let d = mulmod128(num, iv, n);
                        let chk = c.mul(&mut s_ver, d, Some(g));
                        if chk == Some(q) {
                            o.verified = true;
                            o.recovered_matches = d == k_true;
                            break 'outer;
                        }
                    }
                }
                o.useless += 1;
            }
            None => {
                table.insert(dp.0, (a, b));
            }
        }
    }
    o.walks = walks;
    o.ops_setup = s_setup;
    o.ops_start = s_start;
    o.ops_walk = s_walk;
    o.ops_verify = s_ver;
    o.ops_total = s_setup + s_start + s_walk + s_ver;
    o
}

pub fn cmd(args: &[String]) {
    let curves = load(&opt(args, "--curves").expect("--curves"));
    let targets = opt_u64(args, "--targets", None);
    let seed = opt_u64(args, "--seed", None);
    let run = opt(args, "--run").unwrap_or_else(|| "A".into());
    let run_tag: u64 = match run.as_str() {
        "A" => 0xA11,
        "B" => 0xB22,
        s => s.bytes().fold(7u64, |h, b| h.wrapping_mul(131).wrapping_add(b as u64)),
    };
    let mut sink = Sink::open(opt(args, "--out"));
    let mut tsink = opt(args, "--targets-out").map(|p| Sink::open(Some(p)));
    if let Some(t) = tsink.as_mut() {
        t.line("idx,target,k,ops_total,ops_setup,ops_walk,escapes,verified");
    }
    let t0 = std::time::Instant::now();
    let mut total_ops = 0u64;
    for cr in &curves {
        let CurveRec { idx, c, n, g, .. } = *cr;
        let cap = 60 * ((n as f64).sqrt() as u64) + 20000;
        let (mut sum, mut sumsq, mut sum_walk, mut sum_setup, mut esc) = (0f64, 0f64, 0f64, 0f64, 0u64);
        let (mut ver, mut mism, mut cap_hits, mut aband) = (0u64, 0u64, 0u64, 0u64);
        for ti in 0..targets {
            let tseed = derive(seed, &[run_tag, idx, ti]);
            let mut r = Rng::new(derive(tseed, &[1]));
            let k = 1 + r.below(n - 1);
            let mut z = 0;
            let q = c.mul(&mut z, k, Some(g)).expect("Q");
            let o = rho_one(&c, g, q, n, k, derive(tseed, &[2]), cap);
            if o.verified {
                ver += 1;
            } else {
                cap_hits += 1;
            }
            if o.verified && !o.recovered_matches {
                mism += 1;
            }
            let x = o.ops_total as f64;
            sum += x;
            sumsq += x * x;
            sum_walk += o.ops_walk as f64;
            sum_setup += o.ops_setup as f64;
            esc += o.escapes;
            aband += o.abandoned;
            total_ops += o.ops_total;
            if let Some(t) = tsink.as_mut() {
                t.line(&format!("{},{},{},{},{},{},{},{}", idx, ti, k, o.ops_total, o.ops_setup, o.ops_walk, o.escapes, o.verified as u8));
            }
        }
        let tn = targets as f64;
        let mean = sum / tn;
        let var = if targets > 1 { (sumsq - sum * sum / tn) / (tn - 1.0) } else { 0.0 };
        let sq = (n as f64).sqrt();
        sink.line(&format!(
            "{{\"kind\":\"rho\",\"run\":\"{}\",\"idx\":{},\"n\":{},\"T\":{},\"verified\":{},\"mismatch\":{},\"unsolved\":{},\"mean_ops\":{:.4},\"sd_ops\":{:.4},\"mean_over_sqrt_n\":{:.6},\"se_over_sqrt_n\":{:.6},\"mean_walk_ops\":{:.4},\"mean_setup_ops\":{:.4},\"escapes_per_target\":{:.4},\"abandoned_total\":{}}}",
            run, idx, n, targets, ver, mism, cap_hits, mean, var.sqrt(), mean / sq, (var / tn).sqrt() / sq,
            sum_walk / tn, sum_setup / tn, esc as f64 / tn, aband
        ));
    }
    sink.flush();
    let el = t0.elapsed().as_secs_f64();
    eprintln!("rho run {run}: {} curves x {} targets, {} group ops in {:.2}s = {:.3e} ops/s (one core)", curves.len(), targets, total_ops, el, total_ops as f64 / el);
}
