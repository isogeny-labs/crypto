// Matched rho on E0(K): N4's plain walk (overflow-safe mod-ell adds) and a negation-map walk on
// classes {P, -P} with look-ahead, snapshot cycle checks and deterministic doubling escape.
#![allow(dead_code)]
use crate::common::*;
use crate::ghs::*;
use crate::ic2::{la2, lm2, ls2};
use std::collections::HashMap;

pub struct RhoOut {
    pub ok: bool,
    pub steps: u64,
    pub adds: u64,
    pub dps: u64,
    pub restarts: u64,
    pub lookahead: u64,
    pub cycles: u64,
    pub cycle_adds: u64,
    pub cnt: Cnt,
    pub wall: f64,
}

#[inline(always)]
fn hx(x: &K) -> u64 {
    x[0].wrapping_mul(0x9E3779B97F4A7C15) ^ x[1].wrapping_mul(0xC2B2AE3D27D4EB4F) ^ x[2]
}

/// N4's solver::rho, with la2 (identical walk for ell < 2^62).
pub fn rho_plain(inst: &Inst, k_true: u64, seed: u64) -> RhoOut {
    let ell = inst.ell;
    let ec = &inst.ec;
    let mut rng = Rng::new(seed ^ 0xABCDEF);
    let b0 = snap();
    let t0 = std::time::Instant::now();
    let target = ec.mul(&inst.g, k_true as u128);
    const R: usize = 16;
    let mut ma = [0u64; R];
    let mut mb = [0u64; R];
    let mut mp = [EP::Inf; R];
    for j in 0..R {
        if j % 2 == 0 {
            ma[j] = 1 + rng.below(ell - 1);
            mp[j] = ec.mul(&inst.g, ma[j] as u128);
        } else {
            mb[j] = 1 + rng.below(ell - 1);
            mp[j] = ec.mul(&target, mb[j] as u128);
        }
    }
    let dbits = {
        let s = (ell as f64).sqrt() / 64.0;
        (s.max(4.0).log2().floor() as u32).max(1)
    };
    let mask = (1u64 << dbits) - 1;
    let mut table: HashMap<[u64; 6], (u64, u64)> = HashMap::new();
    let (mut steps, mut dps, mut restarts) = (0u64, 0u64, 0u64);
    let maxlen = 20u64 << dbits;
    let mk = |ok: bool, steps: u64, dps: u64, restarts: u64| RhoOut { ok, steps, adds: steps, dps, restarts, lookahead: 0, cycles: 0, cycle_adds: 0, cnt: snap().since(&b0), wall: t0.elapsed().as_secs_f64() };
    loop {
        let (mut a, mut b) = (rng.below(ell), rng.below(ell));
        let mut p = ec.add(&ec.mul(&inst.g, a as u128), &ec.mul(&target, b as u128));
        let mut since_dp = 0u64;
        loop {
            let (x, y) = match &p {
                EP::Inf => break,
                EP::A(x, y) => (*x, *y),
            };
            bump_cmp(1);
            let h = hx(&x);
            if (h >> 20) & mask == 0 {
                dps += 1;
                bump_hash(1);
                let key = [x[0], x[1], x[2], y[0], y[1], y[2]];
                if let Some(&(a2, b2)) = table.get(&key) {
                    if b2 != b {
                        let num = ls2(a, a2, ell);
                        let den = ls2(b2, b, ell);
                        let k = lm2(num, inv_mod_u64(den, ell), ell);
                        let ok = ec.mul(&inst.g, k as u128) == target;
                        return mk(ok, steps, dps, restarts);
                    }
                } else {
                    table.insert(key, (a, b));
                }
                since_dp = 0;
            }
            let j = (h as usize) % R;
            p = ec.add(&p, &mp[j]);
            a = la2(a, ma[j], ell);
            b = la2(b, mb[j], ell);
            steps += 1;
            since_dp += 1;
            if since_dp > maxlen {
                restarts += 1;
                break;
            }
        }
        if steps > 60 * (ell as f64).sqrt() as u64 + 200000 {
            return mk(false, steps, dps, restarts);
        }
    }
}

/// canonical representative of {P, -P}: the y whose first nonzero coordinate is < q/2.
#[inline(always)]
fn canon(kf: &Kf, p: EP) -> (EP, bool) {
    bump_cmp(1);
    match p {
        EP::Inf => (p, false),
        EP::A(x, y) => {
            let half = kf.q() / 2;
            let first = if y[0] != 0 { y[0] } else if y[1] != 0 { y[1] } else { y[2] };
            if first > half {
                (EP::A(x, kf.neg(&y)), true)
            } else {
                (p, false)
            }
        }
    }
}
fn ptkey(p: &EP) -> [u64; 6] {
    match p {
        EP::Inf => [u64::MAX; 6],
        EP::A(x, y) => [x[2], x[1], x[0], y[2], y[1], y[0]],
    }
}

pub fn rho_neg(inst: &Inst, k_true: u64, seed: u64) -> RhoOut {
    let ell = inst.ell;
    let ec = &inst.ec;
    let kf = &ec.kf;
    let mut rng = Rng::new(seed ^ 0x4E4547);
    let b0 = snap();
    let t0 = std::time::Instant::now();
    let target = ec.mul(&inst.g, k_true as u128);
    const R: usize = 32;
    const SNAP: u64 = 128;
    let mut ma = [0u64; R];
    let mut mb = [0u64; R];
    let mut mp = [EP::Inf; R];
    for j in 0..R {
        ma[j] = 1 + rng.below(ell - 1);
        mb[j] = 1 + rng.below(ell - 1);
        mp[j] = ec.add(&ec.mul(&inst.g, ma[j] as u128), &ec.mul(&target, mb[j] as u128));
    }
    let dbits = {
        let s = (ell as f64).sqrt() / 64.0;
        (s.max(4.0).log2().floor() as u32).max(1)
    };
    let mask = (1u64 << dbits) - 1;
    let mut table: HashMap<[u64; 6], (u64, u64)> = HashMap::new();
    let mut o = RhoOut { ok: false, steps: 0, adds: 0, dps: 0, restarts: 0, lookahead: 0, cycles: 0, cycle_adds: 0, cnt: Cnt::default(), wall: 0.0 };
    let maxlen = 20u64 << dbits;
    let neg = |v: u64| if v == 0 { 0 } else { ell - v };
    // one deterministic walk step from (p, a, b): returns (p', a', b', adds used)
    let step = |p: &EP, a: u64, b: u64, o_adds: &mut u64, o_la: &mut u64| -> (EP, u64, u64) {
        let x = match p {
            EP::A(x, _) => *x,
            EP::Inf => return (EP::Inf, a, b),
        };
        let mut j = (hx(&x) as usize) % R;
        let mut res = (EP::Inf, false);
        for t in 0..R {
            let s = ec.add(p, &mp[j]);
            *o_adds += 1;
            res = canon(kf, s);
            let jn = match &res.0 {
                EP::A(xn, _) => (hx(xn) as usize) % R,
                EP::Inf => usize::MAX,
            };
            if jn != j || t == R - 1 {
                break;
            }
            *o_la += 1;
            j = (j + 1) % R;
        }
        let (na, nb) = (la2(a, ma[j], ell), la2(b, mb[j], ell));
        if res.1 {
            (res.0, neg(na), neg(nb))
        } else {
            (res.0, na, nb)
        }
    };
    loop {
        let (a0, b0_) = (rng.below(ell), rng.below(ell));
        let p0 = ec.add(&ec.mul(&inst.g, a0 as u128), &ec.mul(&target, b0_ as u128));
        let (mut p, s0) = canon(kf, p0);
        let (mut a, mut b) = if s0 { (neg(a0), neg(b0_)) } else { (a0, b0_) };
        let mut since_dp = 0u64;
        let mut snapk: Option<[u64; 6]> = None;
        let mut snap_step = 0u64;
        let mut local = 0u64;
        loop {
            let (x, y) = match &p {
                EP::Inf => break,
                EP::A(x, y) => (*x, *y),
            };
            bump_cmp(1);
            let h = hx(&x);
            if (h >> 20) & mask == 0 {
                o.dps += 1;
                bump_hash(1);
                let key = [x[0], x[1], x[2], y[0], y[1], y[2]];
                if let Some(&(a2, b2)) = table.get(&key) {
                    if b2 != b {
                        let num = ls2(a, a2, ell);
                        let den = ls2(b2, b, ell);
                        let k = lm2(num, inv_mod_u64(den, ell), ell);
                        o.ok = ec.mul(&inst.g, k as u128) == target;
                        o.cnt = snap().since(&b0);
                        o.wall = t0.elapsed().as_secs_f64();
                        return o;
                    }
                } else {
                    table.insert(key, (a, b));
                }
                since_dp = 0;
            }
            let (np, na, nb) = step(&p, a, b, &mut o.adds, &mut o.lookahead);
            p = np;
            a = na;
            b = nb;
            o.steps += 1;
            local += 1;
            since_dp += 1;
            // fruitless-cycle check against the snapshot
            let k = ptkey(&p);
            bump_cmp(1);
            if snapk == Some(k) {
                o.cycles += 1;
                let len = local - snap_step;
                // walk the cycle once, keep its minimal point, escape by doubling it
                let (mut cp, mut ca, mut cb) = (p, a, b);
                let (mut mpnt, mut mina, mut minb) = (p, a, b);
                let mut cadds = 0u64;
                let mut cla = 0u64;
                for _ in 0..len {
                    let r = step(&cp, ca, cb, &mut cadds, &mut cla);
                    cp = r.0;
                    ca = r.1;
                    cb = r.2;
                    bump_cmp(1);
                    if ptkey(&cp) < ptkey(&mpnt) {
                        mpnt = cp;
                        mina = ca;
                        minb = cb;
                    }
                }
                o.cycle_adds += cadds;
                let d = ec.add(&mpnt, &mpnt);
                o.cycle_adds += 1;
                let (dp, sg) = canon(kf, d);
                let (da, db) = (la2(mina, mina, ell), la2(minb, minb, ell));
                p = dp;
                if sg {
                    a = neg(da);
                    b = neg(db);
                } else {
                    a = da;
                    b = db;
                }
                snapk = None;
                snap_step = local;
            }
            if local % SNAP == 0 {
                snapk = Some(ptkey(&p));
                snap_step = local;
            }
            if since_dp > maxlen {
                o.restarts += 1;
                break;
            }
        }
        if o.steps > 60 * (ell as f64).sqrt() as u64 + 200000 {
            o.cnt = snap().since(&b0);
            o.wall = t0.elapsed().as_secs_f64();
            return o;
        }
    }
}
