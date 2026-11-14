//! Multi-target BSGS on j=0 curves, folded by the order-6 automorphism group (x -> beta x, y -> -y), batched giant steps across targets.
//! usage: prime_fast_j0_bsgs_multi <out.jsonl> <bits> <T> <instances>   (env MC = table-size scale, default 1.0)
//!
//! Curve y^2 = x^3 + b over F_p, p = 1 mod 3, prime order n.  phi(x,y) = (beta x, y) = [lambda]P.
use crypto_lib::cryptanalysis::challenge_ladder::is_prime_u64;
use crypto_lib::cryptanalysis::prime_fast::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;
use std::io::Write;
use std::time::Instant;

fn find_j0_curve(bits: u32, skip: u64) -> (FastCurve, Pt, u64, u64) {
    let mut p = (1u64 << bits) - 1;
    let mut skipped = 0;
    loop {
        while !(p % 3 == 1 && is_prime_u64(p)) {
            p -= 1;
        }
        let f = Fp64::new(p);
        for b in 1u64..2000 {
            let probe = FastCurve::new("probe", p, 0, b, 1);
            let Some(g) = (1u64..10_000).find_map(|x| probe.lift_x(x)) else {
                continue;
            };
            let Some(order) = point_order_hasse(&probe, g) else {
                continue;
            };
            if !is_prime_u64(order) || order % 3 != 1 {
                continue;
            }
            if skipped < skip {
                skipped += 1;
                continue;
            }
            let c = FastCurve::new(&format!("j0-{bits}-b{b}"), p, 0, b, order);
            let g = c.lift_x(probe.canonical(g).0).unwrap();
            // beta: cube root of unity mod p (Montgomery).
            let mut beta = f.one;
            for t in 2u64..100 {
                let r = f.pow(f.to_mont(t), (p - 1) / 3);
                if r != f.one {
                    beta = r;
                    break;
                }
            }
            assert!(beta != f.one);
            // lambda mod n with phi(G) = [lambda]G.
            let fnum = c.scalar_field();
            let mut lam = 0u64;
            for h in 2u64..100 {
                let r = fnum.from_mont(fnum.pow(fnum.to_mont(h), (order - 1) / 3));
                if r != 1 {
                    let phi = Pt {
                        x: f.mul(beta, g.x),
                        y: g.y,
                    };
                    for cand in [r, (r as u128 * r as u128 % order as u128) as u64] {
                        if c.mul(Some(g), cand) == Some(phi) {
                            lam = cand;
                        }
                    }
                    break;
                }
            }
            if lam == 0 {
                // try the other beta
                continue;
            }
            return (c, g, order, lam);
        }
        p -= 1;
    }
}

fn beta_of(c: &FastCurve, g: Pt, lam: u64) -> u64 {
    let f = c.f;
    let target = c.mul(Some(g), lam).unwrap();
    let want = f.mul(target.x, f.inv(g.x));
    assert_eq!(target.y, g.y);
    want
}

fn canon_pt(c: &FastCurve, beta: u64, beta2: u64, p: Pt) -> (Pt, usize, bool) {
    // returns canonical point, rotation s (canon = phi^s(p) up to sign), negated flag
    let f = c.f;
    let xs = [p.x, f.mul(beta, p.x), f.mul(beta2, p.x)];
    let mut s = 0;
    for k in 1..3 {
        if xs[k] < xs[s] {
            s = k;
        }
    }
    let q = Pt { x: xs[s], y: p.y };
    let (q2, neg) = c.canonical_sign(q);
    (q2, s, neg)
}

struct BTab {
    keys: Vec<u64>,
    vals: Vec<(u32, u8)>,
    mask: usize,
}
impl BTab {
    fn new(cap: usize) -> Self {
        let n = (cap * 2).next_power_of_two();
        BTab {
            keys: vec![u64::MAX; n],
            vals: vec![(0, 0); n],
            mask: n - 1,
        }
    }
    #[inline(always)]
    fn h(&self, k: u64) -> usize {
        (k.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 20) as usize & self.mask
    }
    fn insert(&mut self, k: u64, v: (u32, u8)) {
        let mut i = self.h(k);
        while self.keys[i] != u64::MAX {
            if self.keys[i] == k {
                return;
            }
            i = (i + 1) & self.mask;
        }
        self.keys[i] = k;
        self.vals[i] = v;
    }
    #[inline(always)]
    fn get(&self, k: u64) -> Option<(u32, u8)> {
        let mut i = self.h(k);
        loop {
            let kk = self.keys[i];
            if kk == k {
                return Some(self.vals[i]);
            }
            if kk == u64::MAX {
                return None;
            }
            i = (i + 1) & self.mask;
        }
    }
}

fn pkey(c: &FastCurve, p: Pt) -> u64 {
    let f = c.f;
    f.from_mont(p.x) ^ (f.from_mont(p.y) << 32)
}

/// Returns (ks, baby entries, giant steps).
fn bsgs_multi(
    c: &FastCurve,
    g: Pt,
    lam: u64,
    qs: &[Pt],
    m: usize,
) -> Option<(Vec<u64>, usize, u64)> {
    let f = c.f;
    let n = c.n;
    let t_count = qs.len();
    let beta = beta_of(c, g, lam);
    let beta2 = f.mul(beta, beta);
    let lam2 = (lam as u128 * lam as u128 % n as u128) as u64;
    let lams = [1u64, lam, lam2];
    let mulm = |a: u64, b: u64| (a as u128 * b as u128 % n as u128) as u64;
    let mu_of = |s: usize, neg: bool| -> u64 {
        let mu = lams[s];
        if neg {
            (n - mu) % n
        } else {
            mu
        }
    };
    // baby steps: j = 1..m, lanes of consecutive j's
    let mut tab = BTab::new(m);
    let lanes = 256usize.min(m);
    let chunk = m.div_ceil(lanes);
    let mut pts: Vec<Option<Pt>> = (0..lanes)
        .map(|l| c.mul(Some(g), (1 + l * chunk) as u64))
        .collect();
    let mut d = vec![0u64; lanes];
    let mut scratch = Vec::new();
    for step in 0..chunk {
        for l in 0..lanes {
            let j = 1 + l * chunk + step;
            if j > m {
                continue;
            }
            if let Some(p) = pts[l] {
                let (cp, s, neg) = canon_pt(c, beta, beta2, p);
                let code = (s as u8) | if neg { 4 } else { 0 };
                tab.insert(pkey(c, cp), (j as u32, code));
            }
        }
        for l in 0..lanes {
            let dd = pts[l].map(|p| f.sub(g.x, p.x)).unwrap_or(f.one);
            d[l] = if dd == 0 { f.one } else { dd };
        }
        f.batch_inv(&mut d, &mut scratch);
        for l in 0..lanes {
            if let Some(p) = pts[l] {
                pts[l] = if f.sub(g.x, p.x) == 0 {
                    c.add(Some(p), Some(g))
                } else {
                    Some(c.add_with_inv(p, g, d[l]))
                };
            }
        }
    }
    // giant steps: P_i = Q_t - i*S, S = stride*G with stride = 2m + 1
    let stride = 2 * m as u64 + 1;
    let s_pt = c.mul(Some(g), stride)?;
    let neg_s = c.neg(s_pt);
    let mut cur: Vec<Option<Pt>> = qs.iter().map(|q| Some(*q)).collect();
    let mut solved: Vec<Option<u64>> = vec![None; t_count];
    let mut active: Vec<usize> = (0..t_count).collect();
    let mut giant = 0u64;
    let mut round = 0u64;
    let max_rounds = (n / stride) * 8 + 16;
    let mut dd = vec![0u64; t_count];
    while !active.is_empty() && round <= max_rounds {
        // lookup current points
        let mut still = Vec::with_capacity(active.len());
        for &t in &active {
            let Some(p) = cur[t] else {
                // P = O: Q_t = round*stride*G
                let k = mulm(round % n, stride % n);
                if c.mul(Some(g), k) == Some(qs[t]) {
                    solved[t] = Some(k);
                    continue;
                }
                still.push(t);
                continue;
            };
            let (cp, s, neg) = canon_pt(c, beta, beta2, p);
            giant += 1;
            if let Some((j, code)) = tab.get(pkey(c, cp)) {
                let (sj, nj) = ((code & 3) as usize, code & 4 != 0);
                let muj = mu_of(sj, nj);
                let mup = mu_of(s, neg);
                // P' = (muj / mup) * j * G ; k = round*stride + that
                let ratio = mulm(muj, inv_mod_u64(mup, n));
                let k = (mulm(round % n, stride % n) + mulm(ratio, j as u64)) % n;
                if c.mul(Some(g), k) == Some(qs[t]) {
                    solved[t] = Some(k);
                    continue;
                }
            }
            still.push(t);
        }
        active = still;
        if active.is_empty() {
            break;
        }
        // step all active: P <- P - S
        for (i, &t) in active.iter().enumerate() {
            let dx = cur[t].map(|p| f.sub(neg_s.x, p.x)).unwrap_or(f.one);
            dd[i] = if dx == 0 { f.one } else { dx };
        }
        f.batch_inv(&mut dd[..active.len()], &mut scratch);
        for (i, &t) in active.iter().enumerate() {
            cur[t] = match cur[t] {
                None => Some(neg_s),
                Some(p) => {
                    if f.sub(neg_s.x, p.x) == 0 {
                        c.add(Some(p), Some(neg_s))
                    } else {
                        Some(c.add_with_inv(p, neg_s, dd[i]))
                    }
                }
            };
        }
        round += 1;
    }
    if solved.iter().all(|x| x.is_some()) {
        Some((solved.into_iter().map(|x| x.unwrap()).collect(), m, giant))
    } else {
        None
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bits: u32 = a[2].parse().unwrap();
    let t_count: usize = a[3].parse().unwrap();
    let inst: u64 = a[4].parse().unwrap();
    let mc: f64 = std::env::var("MC")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1.0);
    let mut out = std::fs::File::create(&a[1]).unwrap();
    let cs: u64 = std::env::var("CSKIP")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    for curve_idx in cs..cs + 2 {
        let (c, g, _n, lam) = find_j0_curve(bits, curve_idx);
        let m = ((mc * ((t_count as f64) * (c.n as f64) / 6.0).sqrt()).round() as usize).max(16);
        for ins in 0..inst {
            let mut rng = StdRng::seed_from_u64(0xC0DE ^ ins ^ c.n ^ t_count as u64);
            let ks: Vec<u64> = (0..t_count).map(|_| rng.gen_range(2..c.n)).collect();
            let qs: Vec<Pt> = ks.iter().map(|&k| c.mul(Some(g), k).unwrap()).collect();
            let t0 = Instant::now();
            let res = bsgs_multi(&c, g, lam, &qs, m);
            let ms = t0.elapsed().as_secs_f64() * 1e3;
            let (ok, giant) = match &res {
                Some((got, _, gi)) => (got.iter().zip(&ks).filter(|(x, k)| x == k).count(), *gi),
                None => (0, 0),
            };
            let row = json!({"bits": bits, "curve": c.name, "T": t_count, "inst": ins, "m": m, "mc": mc,
                "ok": ok, "giant_steps": giant, "folded_bsgs_ms": ms});
            writeln!(out, "{row}").unwrap();
            println!("{row}");
        }
    }
}
