//! `rho`: Pollard rho with the negation map on <G> (prime order n). Walk version 2.
//!
//! The lab's `rho.negation` (src/cryptanalysis/ic_wide.rs `rho_negation`,
//! generic over `CountedGroup`) could not be linked: `crypto_lib` does not
//! resolve its dependencies offline here (aws-credential-types is not in the
//! crates cache), and it is a 900k-line crate. This is a re-implementation of
//! the same walk family with the differences listed in the README.
//!
//! Walk version 2 (r = 32 additions; see README "Walk version 2" for the defect of v1):
//!   * canonical representative of {P, -P}: the one with y <= (p-1)/2; key = x.
//!   * table per (run, curve), built once and shared by the T targets of that curve in that
//!     run: jumps M_j = u_j G, j < 32, and the stride part dG, with u_j, d uniform in [1, n)
//!     (full size). Its group operations are charged to the
//!     targets as an equal share, ceil(table ops / T) each.
//!     index = (mix64(x ^ salt) >> 40) mod 32 with a per-target salt: the jump set is shared,
//!     the walk function is not, so a DP-free cycle (or two fruitless cycles whose escapes
//!     feed each other) of one target's function does not trap the other targets.
//!   * step: P <- canon(P + M_j), tracking (a, b) with P = aG + bQ (mod n); a += u_j;
//!     canonicalisation flips (a, b) -> (-a, -b). b only changes sign inside a walk.
//!   * fruitless cycles (Bernstein-Lange-Schwabe 2011): keep the last 16 canonical points of
//!     the walk; when the new point's x equals one of them, the cycle is the window tail from
//!     that point; the walk continues from canon(m + M_s + M_t), m the member with the smallest
//!     x (the same point for every walker that enters this cycle), s, t two further 5-bit fields
//!     of mix64(x(m) ^ salt) (t != s). The escape depends on m: with one fixed escape point E,
//!     R = -(m + E) and R + E = -m make the escape an involution, and a 2-cycle {R, R'} with
//!     minimum R sent the walk back to m (ping-pong, about 1 escape in 300 at 2^20, each a capped
//!     walk). Doubling m would avoid it but doubles b, and walks' b = +-(1 + i) would then meet.
//!     If the next escape of the same walk picks the same minimum, the walk is abandoned (counted).
//!   * distinguished points: mix64(x) & (2^dp - 1) == 0, dp = floor(bitlen(p) / 4), min 1:
//!     one value for every curve over the same prime (v1 took it from n, see README).
//!     Walk i starts at S_i = Q + i D, D = dG + Q (one addition per target, then one per
//!     walk), so walk i has b = +-(1 + i): two walks never have the same b and every DP
//!     collision between different walks is useful. Length cap 20*2^dp + 64.
//!   * collision on a DP with b != b': k = (a - a')/(b' - b) mod n, verified [k]G = Q.
//! Every non-trivial group addition/doubling is counted.
use crate::curves::{load, CurveRec};
use crate::ec::*;
use crate::util::*;
use std::collections::{HashMap, VecDeque};

pub const JUMPS: usize = 32;
pub const WALK_VERSION: u32 = 2;
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

/// The per-(run, curve) walk table: jumps u_j G and the stride part dG.
#[derive(Clone, Debug)]
pub struct Table {
    pub jumps: Vec<((u64, u64), u64)>,
    pub dg: ((u64, u64), u64),
    pub ops: u64,
}

/// DP bits from the field size, so all curves over one prime share it. v1 used
/// floor(log2 n)/4, which flips for curves with n >= 2^k when p is just below 2^k.
pub fn dp_bits_for(p: u64) -> u32 {
    ((64 - p.leading_zeros()) / 4).max(1)
}

/// Full-size coefficients: no short integer relation among the jump logarithms
/// (v1's u_j, v_j < 16 gave many, hence fruitless cycles longer than the window).
pub fn build_table(c: &Curve, g: (u64, u64), n: u64, seed: u64) -> Table {
    let mut rng = Rng::new(seed);
    let mut ops = 0u64;
    let mut draw = |ops: &mut u64| -> ((u64, u64), u64) {
        loop {
            let u = 1 + rng.below(n - 1);
            if let Some(m) = c.mul(ops, u, Some(g)) {
                return (m, u);
            }
        }
    };
    let mut jumps = Vec::with_capacity(JUMPS);
    while jumps.len() < JUMPS {
        let j = draw(&mut ops);
        if jumps.iter().all(|e: &((u64, u64), u64)| e.1 != j.1 && e.1 != n - j.1) {
            jumps.push(j);
        }
    }
    let dg = draw(&mut ops);
    Table { jumps, dg, ops }
}

fn canon(c: &Curve, pt: (u64, u64), a: u64, b: u64, n: u64) -> ((u64, u64), u64, u64) {
    let (x, y) = pt;
    if y > c.p - y {
        ((x, c.p - y), (n - a) % n, (n - b) % n)
    } else {
        (pt, a, b)
    }
}

/// One rho solve of Q = [k]G with table `tb`; `setup_share` is this target's share of the
/// table's operations (charged to ops_setup); `salt` selects this target's partition
/// (a fresh walk function per target at no group-operation cost). `cap_ops` bounds total
/// group operations.
pub fn rho_one(c: &Curve, g: (u64, u64), q: (u64, u64), n: u64, k_true: u64, tb: &Table, setup_share: u64, salt: u64, cap_ops: u64) -> Out {
    let mut o = Out::default();
    let (mut s_setup, mut s_start, mut s_walk, mut s_ver) = (setup_share, 0u64, 0u64, 0u64);
    // S_0 = Q (a = 0, b = 1); D = dG + Q (a += d, b += 1 per walk).
    let du = tb.dg.1;
    let stride = c.add(&mut s_setup, Some(tb.dg.0), Some(q));
    let mut s_pt: Pt = Some(q);
    let (mut sa, mut sb) = (0u64, 1u64);
    let dp_bits = dp_bits_for(c.p);
    let dp_mask = (1u64 << dp_bits) - 1;
    let walk_cap = (1u64 << dp_bits) * 20 + 64;
    let mut table: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut walks = 0u64;
    let total = |a: u64, b: u64, d: u64, e: u64| a + b + d + e;
    'outer: while total(s_setup, s_start, s_walk, s_ver) < cap_ops {
        if walks > 0 {
            s_pt = c.add(&mut s_start, s_pt, stride);
            sa = (sa + du) % n;
            sb = (sb + 1) % n;
        }
        walks += 1;
        let Some(start) = s_pt else { continue };
        let (mut pt, mut a, mut b) = canon(c, start, sa, sb, n);
        let mut win: VecDeque<(u64, (u64, u64), u64, u64)> = VecDeque::with_capacity(WINDOW + 1);
        let mut last_escape: Option<u64> = None;
        let mut len = 0u64;
        let end: Option<((u64, u64), u64, u64)> = loop {
            let j = ((mix64(pt.0 ^ salt) >> 40) as usize) & (JUMPS - 1);
            let (m, uj) = tb.jumps[j];
            let sum = c.add(&mut s_walk, Some(pt), Some(m));
            let Some(sp) = sum else { break None };
            let (np, na, nb) = canon(c, sp, (a + uj) % n, b, n);
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
                let hm = mix64(mp.0 ^ salt);
                let js = ((hm >> 45) as usize) & (JUMPS - 1);
                let jt = (js + 1 + (((hm >> 50) as usize) % (JUMPS - 1))) & (JUMPS - 1);
                let e1 = c.add(&mut s_walk, Some(mp), Some(tb.jumps[js].0));
                let e2 = c.add(&mut s_walk, e1, Some(tb.jumps[jt].0));
                let Some(e2) = e2 else { break None };
                let (ep, ea, eb) = canon(c, e2, (ma + tb.jumps[js].1 + tb.jumps[jt].1) % n, mb, n);
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
        let tb = build_table(&c, g, n, derive(seed, &[run_tag, idx, 0x7AB1E]));
        let share = tb.ops.div_ceil(targets.max(1));
        let (mut sum, mut sumsq, mut sum_walk, mut sum_setup, mut esc) = (0f64, 0f64, 0f64, 0f64, 0u64);
        let (mut ver, mut mism, mut cap_hits, mut aband) = (0u64, 0u64, 0u64, 0u64);
        for ti in 0..targets {
            let tseed = derive(seed, &[run_tag, idx, ti]);
            let mut r = Rng::new(derive(tseed, &[1]));
            let k = 1 + r.below(n - 1);
            let mut z = 0;
            let q = c.mul(&mut z, k, Some(g)).expect("Q");
            let o = rho_one(&c, g, q, n, k, &tb, share, derive(tseed, &[2]), cap);
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
            "{{\"kind\":\"rho\",\"run\":\"{}\",\"idx\":{},\"n\":{},\"T\":{},\"verified\":{},\"mismatch\":{},\"unsolved\":{},\"mean_ops\":{:.4},\"sd_ops\":{:.4},\"mean_over_sqrt_n\":{:.6},\"se_over_sqrt_n\":{:.6},\"mean_walk_ops\":{:.4},\"mean_setup_ops\":{:.4},\"escapes_per_target\":{:.4},\"abandoned_total\":{},\"walk_version\":{}}}",
            run, idx, n, targets, ver, mism, cap_hits, mean, var.sqrt(), mean / sq, (var / tn).sqrt() / sq,
            sum_walk / tn, sum_setup / tn, esc as f64 / tn, aband, WALK_VERSION
        ));
    }
    sink.flush();
    let el = t0.elapsed().as_secs_f64();
    eprintln!("rho run {run}: {} curves x {} targets, {} group ops in {:.2}s = {:.3e} ops/s (one core)", curves.len(), targets, total_ops, el, total_ops as f64 / el);
}
