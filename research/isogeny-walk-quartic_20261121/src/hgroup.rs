//! Group-side tools on the ORIGINAL genus-3 Jacobian J_H(F_p) with Cantor arithmetic
//! (`prime_hyperelliptic`): cost snapshots, the unit GAE_H, a negation-map Pollard rho
//! (distinguished points, r-adding walk, two-cycle escape, restart on long walks; the same
//! algorithm as `run_rho` of qds_core, ported to Mumford divisors), and the BSGS window search
//! that turns `N_1, N_2` into the Jacobian order.

use crypto_lib::prime_hyperelliptic::ops as hops;
use crypto_lib::prime_hyperelliptic::{HyperellipticCurveP, MumfordDivisorP};
use num_bigint::BigUint;
use std::collections::HashMap;

/// (M = mul + sqr, I = inv, adds, native = hash probes etc. counted by the caller).
#[derive(Clone, Copy, Debug, Default)]
pub struct Cost {
    pub m: u64,
    pub i: u64,
    pub add: u64,
}

pub fn snap() -> Cost {
    let t = hops::snapshot().total();
    Cost {
        m: t.get(hops::Kind::FieldMul) + t.get(hops::Kind::FieldSqr),
        i: t.get(hops::Kind::FieldInv),
        add: t.get(hops::Kind::FieldAdd),
    }
}

pub fn since(a: Cost, b: Cost) -> Cost {
    Cost {
        m: a.m - b.m,
        i: a.i - b.i,
        add: a.add - b.add,
    }
}

pub type Key = [u64; 7];

/// Canonical key of a reduced divisor: degree of `u`, then `u` and `v` coefficients (low three).
pub fn key(d: &MumfordDivisorP) -> Key {
    let mut k = [0u64; 7];
    k[0] = d.u.degree().map_or(0, |x| x as u64);
    let lo = |c: &BigUint| -> u64 {
        let v = c.to_u64_digits();
        v.first().copied().unwrap_or(0)
    };
    for i in 0..3 {
        if i < d.u.coeffs.len() {
            k[1 + i] = lo(&d.u.coeffs[i]);
        }
        if i < d.v.coeffs.len() {
            k[4 + i] = lo(&d.v.coeffs[i]);
        }
    }
    k
}

fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub struct Rng(pub u64);
impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0xA076_1D64_78BD_642F)
    }
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        mix(self.0)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        ((self.next() as u128 * n as u128) >> 64) as u64
    }
}

pub fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

pub fn invmod(a: u64, m: u64) -> u64 {
    let (mut r0, mut r1) = (m as i128, a as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let qt = r0 / r1;
        (r0, r1) = (r1, r0 - qt * r1);
        (t0, t1) = (t1, t0 - qt * t1);
    }
    t0.rem_euclid(m as i128) as u64
}

fn smul(g: &MumfordDivisorP, k: u64, hc: &HyperellipticCurveP) -> MumfordDivisorP {
    g.scalar_mul_fast(&BigUint::from(k), hc)
}

#[derive(Clone, Debug, Default)]
pub struct RhoRes {
    pub found: bool,
    pub verified: bool,
    pub k: u64,
    pub steps: u64,
    pub restarts: u64,
    pub cycle_escapes: u64,
    pub cost: Cost,
    pub nat: u64,
    /// Cost of the 32 walk multipliers and the first start point (inside `cost`).
    pub setup: Cost,
}

/// Pollard rho for `q = k g` with `g` of prime order `ell` (odd); with `neg` the walk is on
/// classes modulo +-1.
pub fn rho(
    hc: &HyperellipticCurveP,
    g: &MumfordDivisorP,
    q: &MumfordDivisorP,
    ell: u64,
    neg: bool,
    seed: u64,
) -> RhoRes {
    let mut rng = Rng::new(seed);
    let c0 = snap();
    let mut nat = 0u64;
    let r = 32usize;
    let mut tj: Vec<(MumfordDivisorP, u64, u64)> = Vec::new();
    while tj.len() < r {
        let m = 1 + rng.below(1 << 16);
        let n = 1 + rng.below(1 << 16);
        let t = smul(g, m, hc).add_fast(&smul(q, n, hc), hc);
        tj.push((t, m, n));
    }
    let theta_bits = 5u32;
    let mask = (1u64 << theta_bits) - 1;
    let mut table: HashMap<Key, (u64, u64)> = HashMap::new();
    let max_steps = 60 * ((ell as f64).sqrt() as u64 + 100) + 100_000;
    let new_start = |rng: &mut Rng| -> (MumfordDivisorP, u64, u64) {
        let m = 1 + rng.below(ell - 1);
        let n = 1 + rng.below(ell - 1);
        (smul(g, m, hc).add_fast(&smul(q, n, hc), hc), m, n)
    };
    let (mut w, mut u, mut v) = new_start(&mut rng);
    let setup = since(snap(), c0);
    let mut steps = 0u64;
    let mut walk_len = 0u64;
    let mut restarts = 0u64;
    let mut escapes = 0u64;
    let mut hist: [Option<Key>; 2] = [None, None];
    let mut found = None;
    while steps < max_steps {
        if neg {
            let nw = w.neg_fast(hc);
            nat += 1;
            if key(&nw) < key(&w) {
                w = nw;
                u = (ell - u) % ell;
                v = (ell - v) % ell;
            }
            let kcur = key(&w);
            if hist[0] == Some(kcur) {
                let j = rng.below(r as u64) as usize;
                w = w.add_fast(&tj[j].0, hc);
                u = (u + tj[j].1) % ell;
                v = (v + tj[j].2) % ell;
                escapes += 1;
                hist = [None, None];
                continue;
            }
            hist[0] = hist[1];
            hist[1] = Some(kcur);
        }
        let kw = key(&w);
        let h = mix(kw[1] ^ mix(kw[2] ^ mix(kw[3] ^ mix(kw[4]))));
        nat += 2;
        if (h >> 20) & mask == 0 && walk_len > 0 {
            nat += 1;
            if let Some(&(u2, v2)) = table.get(&kw) {
                if (u2, v2) != (u, v) && v2 != v {
                    let num = (u + ell - u2) % ell;
                    let den = (v2 + ell - v) % ell;
                    if den != 0 {
                        found = Some(mulmod(num, invmod(den, ell), ell));
                        break;
                    }
                }
            } else {
                table.insert(kw, (u, v));
            }
            let j = rng.below(r as u64) as usize;
            w = w.add_fast(&tj[j].0, hc);
            u = (u + tj[j].1) % ell;
            v = (v + tj[j].2) % ell;
            restarts += 1;
            walk_len = 0;
            steps += 1;
            continue;
        }
        let j = (h % r as u64) as usize;
        w = w.add_fast(&tj[j].0, hc);
        u = (u + tj[j].1) % ell;
        v = (v + tj[j].2) % ell;
        steps += 1;
        walk_len += 1;
        if walk_len > 40 * (1u64 << theta_bits) {
            let (nw, nu, nv) = new_start(&mut rng);
            w = nw;
            u = nu;
            v = nv;
            walk_len = 0;
            restarts += 1;
        }
    }
    let mut res = RhoRes {
        found: found.is_some(),
        steps,
        restarts,
        cycle_escapes: escapes,
        nat,
        setup,
        ..Default::default()
    };
    if let Some(kk) = found {
        res.k = kk;
        res.verified = smul(g, kk, hc) == *q;
    }
    res.cost = since(snap(), c0);
    res
}

/// Candidates `N = nlo + j` in `0..=width` with `N g = 0` by BSGS; returns (hits as j, cost,
/// native operations = hash inserts and probes).
pub fn window_search(
    hc: &HyperellipticCurveP,
    g: &MumfordDivisorP,
    nlo: u64,
    width: u64,
) -> (Vec<u64>, Cost, u64) {
    let c0 = snap();
    let m = ((width + 1) as f64).sqrt().ceil() as u64 + 1;
    let mut nat = 0u64;
    let mut table: HashMap<Key, u64> = HashMap::new();
    let mut cur = MumfordDivisorP::identity(hc.p.clone());
    for i in 0..m {
        table.entry(key(&cur)).or_insert(i);
        nat += 1;
        cur = cur.add_fast(g, hc);
    }
    // cur = m g
    let step = cur;
    let t = smul(g, nlo, hc);
    let mut y = t;
    let mut hits = Vec::new();
    for kk in 0..=m {
        let ny = y.neg_fast(hc);
        nat += 1;
        if let Some(&i) = table.get(&key(&ny)) {
            // t + k m g = - i g  =>  j = k m + i
            let j = kk * m + i;
            if j <= width {
                hits.push(j);
            }
        }
        y = y.add_fast(&step, hc);
    }
    hits.sort_unstable();
    hits.dedup();
    (hits, since(snap(), c0), nat)
}
