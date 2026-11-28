//! Matched reference: negation-map Pollard rho, counted through the same
//! [`CF`] as the IC arms.
//!
//! Shape follows crypto `ic_boundary::RhoWalk::negation()` (the ledger's
//! matched reference for generic binary curves, A = 2): r-adding walk on
//! classes `{P, −P}` with `rho_jumps_for(ℓ)` jumps of
//! `jump_coefficient_bits` bits, distinguished points of `⌊bits(ℓ)/4⌋`
//! bits, the Wiener–Zuccherato look-ahead, a 16-point cycle window with
//! escape by doubling the cycle's least member, and a walk cap of
//! `20·2^dp + 64` steps.  Added for equal engineering with the IC arms:
//! `W` walks advance in lock step with one batched inversion per round
//! (crypto `rho_walk_lanes`, `rho.negation_lanes_v1`); `W` is tuned per
//! rung on pilot curves before the frozen run.
use crate::field::{batch_add, Curve, Ops, Pt, CF};
use crate::seeds::Sm64;
use std::collections::HashMap;

pub fn rho_jumps_for(ell: u64) -> usize {
    match 64 - ell.leading_zeros() {
        0..=15 => 4,
        16..=22 => 8,
        _ => 16,
    }
}
fn jump_coefficient_bits(jumps: usize) -> u32 {
    let log = usize::BITS - 1 - jumps.max(2).leading_zeros();
    (3 * log).clamp(6, 12)
}
#[inline]
fn hidx(x: u64, r: usize) -> usize {
    ((x.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 40) as usize) % r
}
#[inline]
fn is_dp(x: u64, bits: u32) -> bool {
    bits == 0 || (x.wrapping_mul(0xD6E8_FEB8_6659_FD93) >> (64 - bits)) == 0
}

#[derive(Clone)]
struct Lane {
    p: Pt,
    a: u64,
    b: u64,
    steps: u64,
    ring: Vec<(u64, Pt, u64, u64)>,
}

pub struct RhoOut {
    pub d: Option<u64>,
    pub verified: bool,
    pub steps: u64,
    pub lookahead: u64,
    pub cycles: u64,
    pub capped: u64,
    pub fruitless_collisions: u64,
    pub phases: Vec<(&'static str, Ops)>,
    pub wall_ns: u128,
}

pub fn rho(c: &Curve, cf: &CF, g: Pt, q: Pt, w: usize, rng: &mut Sm64, max_steps: u64) -> RhoOut {
    let t0 = std::time::Instant::now();
    let ell = c.ell;
    let r = rho_jumps_for(ell);
    let cb = jump_coefficient_bits(r);
    let dpb = (64 - ell.leading_zeros()) / 4;
    let cap = 20 * (1u64 << dpb) + 64;
    let o0 = cf.ops();
    // jumps M_t = [c_t]G + [e_t]Q, small coefficients
    let mut jumps = vec![];
    for _ in 0..r {
        let (a, b) = (rng.bits(cb).max(1) % ell, rng.bits(cb).max(1) % ell);
        jumps.push((c.joint(cf, a, g, b, q), a, b));
    }
    let (a0, b0) = (rng.below(ell), rng.below(ell));
    let (as_, bs) = (rng.below(ell), rng.below(ell));
    let mut next_start = c.joint(cf, a0, g, b0, q);
    let stride = c.joint(cf, as_, g, bs, q);
    let mut ns_ab = (a0, b0);
    let o_setup = cf.ops();
    let canon_ab = |p: Pt, a: u64, b: u64| -> (Pt, u64, u64) {
        let (cp, flip) = c.canon(p);
        if flip {
            (cp, (ell - a) % ell, (ell - b) % ell)
        } else {
            (cp, a, b)
        }
    };
    let mut new_lane = |cf: &CF| -> Lane {
        let (p, a, b) = (next_start, ns_ab.0, ns_ab.1);
        next_start = c.add(cf, next_start, stride);
        ns_ab = ((ns_ab.0 + as_) % ell, (ns_ab.1 + bs) % ell);
        let (p, a, b) = canon_ab(p, a, b);
        Lane { p, a, b, steps: 0, ring: vec![] }
    };
    let mut lanes: Vec<Lane> = (0..w).map(|_| new_lane(cf)).collect();
    let mut dps: HashMap<u64, (u64, u64)> = HashMap::new();
    let (mut steps, mut lookahead, mut cycles, mut capped, mut fruitless) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut scr = (vec![], vec![]);
    let mut out = vec![];
    let mut found: Option<u64> = None;
    // per lane: current offset t of the look-ahead
    let mut tries: Vec<usize> = vec![0; w];
    let mut base: Vec<usize> = vec![0; w];
    let mut done: Vec<bool> = vec![false; w];
    let mut res: Vec<(Pt, u64, u64)> = vec![(Pt::Inf, 0, 0); w];
    'outer: while found.is_none() && steps < max_steps {
        for k in 0..w {
            base[k] = match lanes[k].p {
                Pt::A(x, _) => hidx(x, r),
                Pt::Inf => 0,
            };
            tries[k] = 0;
            done[k] = false;
        }
        // look-ahead rounds, batched over the lanes still searching
        loop {
            let act: Vec<usize> = (0..w).filter(|&k| !done[k]).collect();
            if act.is_empty() {
                break;
            }
            let ps: Vec<Pt> = act.iter().map(|&k| lanes[k].p).collect();
            let qs: Vec<Pt> = act.iter().map(|&k| jumps[(base[k] + tries[k]) % r].0).collect();
            batch_add(c, cf, &ps, &qs, &mut out, &mut scr);
            for (t, &k) in act.iter().enumerate() {
                let j = (base[k] + tries[k]) % r;
                let (np, na, nb) = canon_ab(out[t], (lanes[k].a + jumps[j].1) % ell, (lanes[k].b + jumps[j].2) % ell);
                let nj = match np {
                    Pt::A(x, _) => hidx(x, r),
                    Pt::Inf => usize::MAX,
                };
                if nj == j && tries[k] + 1 < r {
                    tries[k] += 1;
                    lookahead += 1;
                    continue;
                }
                if nj == j {
                    // every jump would 2-cycle: double.
                    let p = c.dbl(cf, lanes[k].p);
                    res[k] = canon_ab(p, 2 * lanes[k].a % ell, 2 * lanes[k].b % ell);
                } else {
                    res[k] = (np, na, nb);
                }
                done[k] = true;
            }
        }
        for k in 0..w {
            steps += 1;
            let (np, na, nb) = res[k];
            let ln = &mut lanes[k];
            ln.steps += 1;
            let x = np.x();
            // cycle window
            if let Some(pos) = ln.ring.iter().position(|e| e.0 == x) {
                cycles += 1;
                let mut best = ln.ring[pos];
                for e in &ln.ring[pos..] {
                    if e.0 < best.0 {
                        best = *e;
                    }
                }
                let p2 = c.dbl(cf, best.1);
                let (p2, a2, b2) = canon_ab(p2, 2 * best.2 % ell, 2 * best.3 % ell);
                ln.p = p2;
                ln.a = a2;
                ln.b = b2;
                ln.ring.clear();
                continue;
            }
            if ln.ring.len() == 16 {
                ln.ring.remove(0);
            }
            ln.ring.push((x, np, na, nb));
            ln.p = np;
            ln.a = na;
            ln.b = nb;
            if np == Pt::Inf {
                *ln = new_lane(cf);
                continue;
            }
            if is_dp(x, dpb) {
                if let Some(&(a2, b2)) = dps.get(&x) {
                    // same canonical class ⇒ same point: a + bd = a2 + b2 d
                    let db = (b2 + ell - nb) % ell;
                    if db == 0 {
                        fruitless += 1;
                    } else {
                        cf.zmul(2 * (64 - ell.leading_zeros()) as u64 + 2);
                        let inv = crate::gen::powmod(db, ell - 2, ell);
                        let d = (((na + ell - a2) % ell) as u128 * inv as u128 % ell as u128) as u64;
                        found = Some(d);
                        break 'outer;
                    }
                } else {
                    cf.hash();
                    dps.insert(x, (na, nb));
                }
                *ln = new_lane(cf);
                continue;
            }
            if ln.steps > cap {
                capped += 1;
                *ln = new_lane(cf);
            }
        }
    }
    let o_walk = cf.ops();
    let verified = match found {
        Some(d) => c.mul(cf, d, g) == q,
        None => false,
    };
    let o_ver = cf.ops();
    RhoOut {
        d: found,
        verified,
        steps,
        lookahead,
        cycles,
        capped,
        fruitless_collisions: fruitless,
        phases: vec![("setup", o_setup.sub(&o0)), ("walk", o_walk.sub(&o_setup)), ("verify", o_ver.sub(&o_walk))],
        wall_ns: t0.elapsed().as_nanos(),
    }
}
