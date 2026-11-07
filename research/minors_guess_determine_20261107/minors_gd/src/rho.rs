//! Matched baseline: batched (Montgomery-trick) r-adding Pollard rho with the
//! negation map (canonical y <= p/2), short-ring fruitless-cycle detection with a
//! counted doubling escape, distinguished points, restarts by counted scalar
//! multiplications. Every F_p multiplication and inversion goes through the
//! counted field. Partition hashing, DP tests, canonicalisation comparisons
//! and mod-n scalar bookkeeping are integer work that is NOT charged (also true
//! for the minors arms' hashing); they are tallied in `uncharged`.
use crate::curve::*;
use crate::rng::*;
use std::collections::HashMap;

pub const R: usize = 64;
pub const W: usize = 32;
const RING: usize = 8;

#[derive(Default, Debug, Clone)]
pub struct RhoOut {
    pub k: Option<u64>,
    pub steps: u64,
    pub dps: u64,
    pub restarts: u64,
    pub escapes: u64,
    pub special_adds: u64,
    pub useless_collisions: u64,
    pub cap_hit: bool,
}

struct Walker { x: u64, y: u64, a: u64, b: u64, ring: [u64; RING], ri: usize, since: u64 }

#[inline] fn part(x: u64) -> usize { (x.wrapping_mul(0x9E3779B97F4A7C15) >> 58) as usize }
#[inline] fn is_dp(x: u64, bits: u32) -> bool { bits == 0 || (x.wrapping_mul(0xC2B2AE3D27D4EB4F) >> (64 - bits)) == 0 }

fn canon(c: &Curve, w: &mut Walker, n: u64) {
    let half = c.f.p / 2;
    if w.y > half {
        w.y = c.f.p - w.y;
        w.a = (n - w.a) % n;
        w.b = (n - w.b) % n;
    }
}

fn start(c: &Curve, g: (u64, u64), q: (u64, u64), n: u64, rng: &mut Rng) -> Walker {
    loop {
        let a = rng.below(n); let b = rng.below(n);
        let x = c.add(c.mul_scalar(a, Some(g)), c.mul_scalar(b, Some(q)));
        if let Some((x, y)) = x {
            let mut w = Walker { x, y, a, b, ring: [u64::MAX; RING], ri: 0, since: 0 };
            canon(c, &mut w, n);
            return w;
        }
    }
}

pub fn solve(c: &Curve, g: (u64, u64), q: (u64, u64), n: u64, bits: u32, rng: &mut Rng, cap: f64) -> RhoOut {
    let f = &c.f;
    let invw = crate::field::inv_weight(f.p);
    let cost = || f.muls.get() as f64 + invw * f.invs.get() as f64;
    let dp_bits: u32 = if bits / 2 > 5 { bits / 2 - 5 } else { 1 };
    let mut out = RhoOut::default();
    // jump table
    let mut jx = vec![0u64; R]; let mut jy = vec![0u64; R]; let mut ju = vec![0u64; R]; let mut jv = vec![0u64; R];
    for j in 0..R {
        loop {
            let u = rng.below(n); let v = rng.below(n);
            if let Some((x, y)) = c.add(c.mul_scalar(u, Some(g)), c.mul_scalar(v, Some(q))) {
                jx[j] = x; jy[j] = y; ju[j] = u; jv[j] = v; break;
            }
        }
    }
    let mut ws: Vec<Walker> = (0..W).map(|_| start(c, g, q, n, rng)).collect();
    let mut table: HashMap<(u64, u64), (u64, u64)> = HashMap::new();
    let mut den = vec![0u64; W];
    let mut idx = vec![0usize; W];
    let mut js = vec![0usize; W];
    loop {
        if cost() > cap { out.cap_hit = true; return out; }
        // gather denominators
        let mut m = 0;
        for (wi, w) in ws.iter().enumerate() {
            let j = part(w.x);
            js[wi] = j;
            let d = f.sub(w.x, jx[j]);
            if d != 0 { den[m] = d; idx[m] = wi; m += 1; }
        }
        f.batch_inv(&mut den[..m]);
        // new points
        let mut nxt: Vec<Option<(u64, u64)>> = vec![None; W];
        let mut have = vec![false; W];
        for t in 0..m {
            let wi = idx[t];
            let w = &ws[wi]; let j = js[wi];
            let lam = f.mul(f.sub(w.y, jy[j]), den[t]);
            let x3 = f.sub(f.sub(f.mul(lam, lam), w.x), jx[j]);
            let y3 = f.sub(f.mul(lam, f.sub(w.x, x3)), w.y);
            nxt[wi] = Some((x3, y3)); have[wi] = true;
        }
        for wi in 0..W {
            let j = js[wi];
            let (x3, y3) = if have[wi] { nxt[wi].unwrap() } else {
                out.special_adds += 1;
                match c.add(Some((ws[wi].x, ws[wi].y)), Some((jx[j], jy[j]))) {
                    Some(v) => v,
                    None => { ws[wi] = start(c, g, q, n, rng); out.restarts += 1; continue; }
                }
            };
            let w = &mut ws[wi];
            w.x = x3; w.y = y3;
            w.a = (w.a + ju[j]) % n; w.b = (w.b + jv[j]) % n;
            canon(c, w, n);
            w.since += 1; out.steps += 1;
            // fruitless cycle look-back
            if w.ring.contains(&w.x) {
                out.escapes += 1;
                if let Some((x4, y4)) = c.dbl(Some((w.x, w.y))) {
                    w.x = x4; w.y = y4; w.a = (2 * w.a) % n; w.b = (2 * w.b) % n; canon(c, w, n);
                } else { *w = start(c, g, q, n, rng); out.restarts += 1; continue; }
                w.ring = [u64::MAX; RING]; w.ri = 0;
            }
            w.ring[w.ri] = w.x; w.ri = (w.ri + 1) % RING;
            let limit = 40u64 << dp_bits;
            if is_dp(w.x, dp_bits) {
                out.dps += 1;
                let key = (w.x, w.y);
                match table.get(&key) {
                    Some(&(a2, b2)) if (a2, b2) != (w.a, w.b) => {
                        let db = (b2 + n - w.b) % n;
                        if db != 0 {
                            let da = (w.a + n - a2) % n;
                            let k = (da as u128 * modinv(db, n) as u128 % n as u128) as u64;
                            out.k = Some(k);
                            return out;
                        } else { out.useless_collisions += 1; }
                    }
                    Some(_) => { out.useless_collisions += 1; }
                    None => { table.insert(key, (w.a, w.b)); }
                }
                *w = start(c, g, q, n, rng); out.restarts += 1;
            } else if w.since > limit {
                *w = start(c, g, q, n, rng); out.restarts += 1;
            }
        }
    }
}

pub fn modinv(a: u64, n: u64) -> u64 {
    let (mut r0, mut r1) = (n as i128, (a % n) as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 { let q = r0 / r1; let r2 = r0 - q * r1; r0 = r1; r1 = r2; let t2 = t0 - q * t1; t0 = t1; t1 = t2; }
    (((t0 % n as i128) + n as i128) % n as i128) as u64
}
