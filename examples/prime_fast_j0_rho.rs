//! Automorphism-folded (x6) single-target rho on j=0 curves, native, vs rho_parallel (negation only).
//! usage: prime_fast_j0_rho <out.jsonl> <bits> <targets>
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

fn folded_rho(
    c: &FastCurve,
    g: Pt,
    q: Pt,
    lam: u64,
    dp_bits: u32,
    seed: u64,
    cyc: bool,
) -> Option<(u64, u64)> {
    let f = c.f;
    let n = c.n;
    let beta = beta_of(c, g, lam);
    let beta2 = f.mul(beta, beta);
    let lam2 = (lam as u128 * lam as u128 % n as u128) as u64;
    let lams = [1u64, lam, lam2];
    let mulm = |a: u64, b: u64| (a as u128 * b as u128 % n as u128) as u64;
    let mut rng = StdRng::seed_from_u64(seed);
    const R: usize = 32;
    let mut tb: Vec<(Pt, u64, u64)> = Vec::new();
    for _ in 0..R {
        let (u, v) = (rng.gen_range(1..n), rng.gen_range(1..n));
        if let Some(t) = c.combine(g, q, u, v) {
            tb.push((t, u, v));
        }
    }
    let mut dps: std::collections::HashMap<u64, (u64, u64)> = std::collections::HashMap::new();
    let mut steps = 0u64;
    let mask = (1u64 << dp_bits) - 1;
    let cap = 40u64 << dp_bits;
    loop {
        let (a0, b0) = (rng.gen_range(1..n), rng.gen_range(1..n));
        let Some(p0) = c.combine(g, q, a0, b0) else {
            continue;
        };
        let (mut p, s0, neg0) = canon_pt(c, beta, beta2, p0);
        let mut mu = lams[s0];
        if neg0 {
            mu = (n - mu) % n;
        }
        let (mut a, mut b) = (mulm(a0, mu), mulm(b0, mu));
        let mut len = 0u64;
        let mut ring = [u64::MAX; 8];
        let mut ri = 0usize;
        loop {
            if f.from_mont(p.x) & mask == 0 {
                let key = f.from_mont(p.x) ^ (f.from_mont(p.y) << 32);
                if let Some(&(a2, b2)) = dps.get(&key) {
                    if b2 != b {
                        let num = (a + n - a2) % n;
                        let den = (b2 + n - b) % n;
                        let k = mulm(num, inv_mod_u64(den, n));
                        if c.mul(Some(g), k) == Some(q) {
                            return Some((k, steps));
                        }
                    }
                } else {
                    dps.insert(key, (a, b));
                }
                break;
            }
            let h = (f.from_mont(p.x) as usize) % R;
            let (mut t, mut u, mut v) = tb[h];
            let Some(mut p1) = c.add(Some(p), Some(t)) else {
                break;
            };
            let (mut p2, mut s, mut neg) = canon_pt(c, beta, beta2, p1);
            if cyc && ring.contains(&p2.x) {
                // fruitless short cycle: leave it through a different jump
                (t, u, v) = tb[(h + 1) % R];
                let Some(q1) = c.add(Some(p), Some(t)) else {
                    break;
                };
                p1 = q1;
                (p2, s, neg) = canon_pt(c, beta, beta2, p1);
            }
            ring[ri & 7] = p2.x;
            ri += 1;
            let mut mu = lams[s];
            if neg {
                mu = (n - mu) % n;
            }
            a = mulm((a + u) % n, mu);
            b = mulm((b + v) % n, mu);
            p = p2;
            steps += 1;
            len += 1;
            if len > cap {
                break;
            }
        }
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let bits: u32 = a[2].parse().unwrap();
    let nt: u64 = a[3].parse().unwrap();
    let mut out = std::fs::File::create(&a[1]).unwrap();
    let cs: u64 = std::env::var("CSKIP")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    for curve_idx in cs..cs + 2 {
        let (c, g, _n, lam) = find_j0_curve(bits, curve_idx);
        let dp_bits = (bits / 2).saturating_sub(10).max(2);
        let (mut fold_ms, mut fold_steps, mut neg_ms) = (Vec::new(), Vec::new(), Vec::new());
        let mut ok = 0;
        for t in 0..nt {
            let mut rng = StdRng::seed_from_u64(0xFEED ^ t ^ c.n);
            let k = rng.gen_range(2..c.n);
            let q = c.mul(Some(g), k).unwrap();
            let t0 = Instant::now();
            if let Some((x, st)) = folded_rho(
                &c,
                g,
                q,
                lam,
                dp_bits,
                t + 1,
                std::env::var_os("CYCFIX").is_some(),
            ) {
                if x == k {
                    ok += 1;
                }
                fold_ms.push(t0.elapsed().as_secs_f64() * 1e3);
                fold_steps.push(st as f64);
            }
            let mut cfg = RhoConfig::for_bits(bits, true);
            cfg.threads = 1;
            cfg.seed = t + 1;
            let t1 = Instant::now();
            if rho_parallel(&c, g, q, &cfg).is_some() {
                neg_ms.push(t1.elapsed().as_secs_f64() * 1e3);
            }
        }
        let med = |v: &mut Vec<f64>| {
            v.sort_by(|x, y| x.partial_cmp(y).unwrap());
            v.get(v.len() / 2).copied()
        };
        let row = json!({"bits": bits, "curve": c.name, "targets": nt, "ok": ok, "dp_bits": dp_bits,
            "folded_median_ms": med(&mut fold_ms), "folded_median_steps": med(&mut fold_steps),
            "rho_parallel_negation_median_ms": med(&mut neg_ms)});
        writeln!(out, "{row}").unwrap();
        println!("{row}");
    }
}
