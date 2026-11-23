//! EXP-PKM-IDEAS-001 (`research/pkm_ideas_20261115/PROTOCOL.md`): five follow-up ideas on PKM
//! smooth-map factor bases, one mode each. Counted metrics only (`F_p` multiplications, exact counts).
//!
//! ```text
//! pkm_ideas dense    --p P --S S --m M --vkind random|coset [--draws K] [--seed S] [--budget SECS] [--dmax D]
//! pkm_ideas replay   --p P --degrees D --m M [--draws K] [--seed S] [--replay-seed S2] [--replay-p P2]
//! pkm_ideas cosets   --p P --degrees D [--draws K] [--seed S]           (F4 count, shared vs per-point coset)
//! pkm_ideas coverage --p P --S S [--seed S]                              (exact sums coverage, m = 2)
//! pkm_ideas aut      --p P --S S [--targets K] [--seed S]                (j = 0 automorphism check)
//! ```
//! One JSON row per system or check on stdout.

use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly};
use crypto_lib::cryptanalysis::pkm_smooth_tower::{
    addm, enumerate_decompositions, invm, is_prime, kummer_tower, mulm, poly_mul, powm, s3, sqrtm,
    subm, Curve, Rng,
};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

type Terms = Vec<(Vec<u32>, u64)>;

/// `S_3(slot0, slot1, slot2)` over `n` variables; a slot is a variable or a constant.
fn s3_terms(e: &Curve, slots: [Result<usize, u64>; 3], n: usize) -> Terms {
    let p = e.p;
    let m2 = p - 2;
    let base: [([u32; 3], u64); 13] = [
        ([2, 0, 2], 1),
        ([0, 2, 2], 1),
        ([1, 1, 2], m2),
        ([2, 1, 1], m2),
        ([1, 2, 1], m2),
        ([1, 0, 1], mulm(m2, e.a, p)),
        ([0, 1, 1], mulm(m2, e.a, p)),
        ([0, 0, 1], mulm(p - 4, e.b, p)),
        ([2, 2, 0], 1),
        ([1, 1, 0], mulm(m2, e.a, p)),
        ([0, 0, 0], mulm(e.a, e.a, p)),
        ([1, 0, 0], mulm(p - 4, e.b, p)),
        ([0, 1, 0], mulm(p - 4, e.b, p)),
    ];
    let mut acc: HashMap<Vec<u32>, u64> = HashMap::new();
    for (ex, c) in base {
        let mut mono = vec![0u32; n];
        let mut coef = c;
        for k in 0..3 {
            match slots[k] {
                Ok(v) => mono[v] += ex[k],
                Err(cst) => coef = mulm(coef, powm(cst, ex[k] as u64, p), p),
            }
        }
        let t = acc.entry(mono).or_insert(0);
        *t = addm(*t, coef, p);
    }
    acc.into_iter().filter(|&(_, c)| c != 0).collect()
}

/// The summation chain on point variables `xs[i]` (first variable of each point) and helpers `us`.
fn chain(e: &Curve, xs: &[usize], us: &[usize], xr: u64, n: usize) -> Vec<Terms> {
    let m = xs.len();
    if m == 2 {
        return vec![s3_terms(e, [Ok(xs[0]), Ok(xs[1]), Err(xr)], n)];
    }
    let mut out = vec![s3_terms(e, [Ok(xs[0]), Ok(xs[1]), Ok(us[0])], n)];
    for k in 2..m {
        let right = if k == m - 1 { Err(xr) } else { Ok(us[k - 1]) };
        out.push(s3_terms(e, [Ok(us[k - 2]), Ok(xs[k]), right], n));
    }
    out
}

/// Exhaustive: does some tuple of `vs[i]` satisfy the chain (helper over the closure)?
fn decomposable(e: &Curve, vs: &[Vec<u64>], xr: u64) -> bool {
    let p = e.p;
    let quad = |a: u64, b: u64| -> [u64; 3] {
        // S_3(a, b, X) as c0 + c1 X + c2 X^2
        let d = subm(a, b, p);
        let s = addm(a, b, p);
        let pr = mulm(a, b, p);
        let c2 = mulm(d, d, p);
        let c1 = subm(
            0,
            mulm(2, addm(mulm(s, addm(pr, e.a, p), p), mulm(2, e.b, p), p), p),
            p,
        );
        let pa = subm(pr, e.a, p);
        [
            subm(mulm(pa, pa, p), mulm(4, mulm(e.b, s, p), p), p),
            c1,
            c2,
        ]
    };
    let res = |a: [u64; 3], b: [u64; 3]| {
        let d = |x: u64, y: u64, z: u64, w: u64| subm(mulm(x, y, p), mulm(z, w, p), p);
        let t1 = d(a[2], b[0], a[0], b[2]);
        let t2 = d(a[2], b[1], a[1], b[2]);
        let t3 = d(a[1], b[0], a[0], b[1]);
        subm(mulm(t1, t1, p), mulm(t2, t3, p), p)
    };
    if vs.len() == 2 {
        return vs[0]
            .iter()
            .any(|&a| vs[1].iter().any(|&b| s3(e, a, b, xr) == 0));
    }
    let second: Vec<[u64; 3]> = vs[2].iter().map(|&c| quad(c, xr)).collect();
    for &a in &vs[0] {
        for &b in &vs[1] {
            let q1 = quad(a, b);
            if q1 == [0, 0, 0] || second.iter().any(|q2| res(q1, *q2) == 0) {
                return true;
            }
        }
    }
    false
}

fn curve_from(seed: u64, p: u64) -> Curve {
    let mut r = Rng(seed ^ 0xe11);
    loop {
        let c = Curve {
            p,
            a: r.below(p),
            b: r.below(p),
        };
        if !c.is_singular() {
            return c;
        }
    }
}

fn arg(o: &HashMap<String, String>, k: &str, d: &str) -> String {
    o.get(k).cloned().unwrap_or_else(|| d.to_string())
}

fn f4_once(input: &[Poly], n: usize, p: u64, budget: u64, dmax: u32) -> f4_fp::F4Report {
    let mut opts = F4Options::new(Ordering::Grevlex, dmax);
    opts.deadline = Some(Instant::now() + Duration::from_secs(budget));
    f4_fp::f4(input, n, p, &opts)
}

fn norm(t: &[Terms], p: u64) -> Vec<Poly> {
    t.iter()
        .map(|x| f4_fp::normalise(x, p, Ordering::Grevlex))
        .collect()
}

/// Tower membership equations for one point: vars `base..base+n'` (Kummer, per-point constant `c`).
fn tower_eqs(degrees: &[u64], base: usize, c: u64, n: usize, p: u64) -> Vec<Terms> {
    let k = degrees.len();
    (0..k)
        .map(|j| {
            let mut a = vec![0u32; n];
            a[base + j] = degrees[j] as u32;
            if j + 1 < k {
                let mut b = vec![0u32; n];
                b[base + j + 1] = 1;
                vec![(a, 1), (b, p - 1)]
            } else {
                vec![(a, 1), (vec![0u32; n], p - c % p)]
            }
        })
        .collect()
}

fn refute_targets(e: &Curve, vs: &[Vec<u64>], k: u64, seed: u64) -> Vec<u64> {
    let mut r = Rng(seed ^ 0x7a6e7);
    let mut out = vec![];
    while (out.len() as u64) < k {
        let pt = e.random_point(&mut r).unwrap();
        if !decomposable(e, vs, pt.0) {
            out.push(pt.0);
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().cloned().unwrap_or_default();
    let mut o: HashMap<String, String> = HashMap::new();
    let mut i = 1;
    while i + 1 < args.len() {
        o.insert(
            args[i].trim_start_matches("--").to_string(),
            args[i + 1].clone(),
        );
        i += 2;
    }
    let p: u64 = arg(&o, "p", "1399680001").parse().unwrap();
    let seed: u64 = arg(&o, "seed", "20261115").parse().unwrap();
    let budget: u64 = arg(&o, "budget", "1800").parse().unwrap();
    let draws: u64 = arg(&o, "draws", "3").parse().unwrap();
    // Amendment 1 (2026-11-15): --dmax, default the frozen 64.
    let dmax: u32 = arg(&o, "dmax", "64").parse().unwrap();
    let threads = std::env::var("RAYON_NUM_THREADS").unwrap_or_default();
    match mode.as_str() {
        // ---------------- idea 1: random V written as one dense polynomial
        "dense" => {
            let s: usize = arg(&o, "S", "16").parse().unwrap();
            let m: usize = arg(&o, "m", "2").parse().unwrap();
            let vkind = arg(&o, "vkind", "random");
            let mut r = Rng(seed ^ 0xd5);
            let v: Vec<u64> = if vkind == "random" {
                let mut set = HashSet::new();
                while set.len() < s {
                    set.insert(1 + r.below(p - 1));
                }
                let mut v: Vec<u64> = set.into_iter().collect();
                v.sort_unstable();
                v
            } else {
                assert_eq!((p - 1) % s as u64, 0, "coset needs S | p - 1");
                // S may be composite: build the coset g * mu_S directly.
                let g = 1 + r.below(p - 1);
                let z = loop {
                    let h = 2 + r.below(p - 3);
                    let z = powm(h, (p - 1) / s as u64, p);
                    let mut ok = true;
                    let mut q = 2u64;
                    let mut ss = s as u64;
                    while q <= ss {
                        if ss.is_multiple_of(q) {
                            if powm(z, s as u64 / q, p) == 1 {
                                ok = false;
                            }
                            while ss.is_multiple_of(q) {
                                ss /= q;
                            }
                        }
                        q += 1;
                    }
                    if ok {
                        break z;
                    }
                };
                (0..s).map(|k| mulm(g, powm(z, k as u64, p), p)).collect()
            };
            // M(x) = prod (x - v), low degree first
            let mut mpoly: Vec<u64> = vec![1];
            for &x in &v {
                mpoly = poly_mul(&mpoly, &vec![p - x, 1], p);
            }
            let n = m + (m - 2);
            let e = curve_from(seed, p);
            let vs: Vec<Vec<u64>> = vec![v.clone(); m];
            let targets = refute_targets(&e, &vs, draws, seed);
            for (draw, &xr) in targets.iter().enumerate() {
                let mut sys: Vec<Terms> = (0..m)
                    .map(|i| {
                        mpoly
                            .iter()
                            .enumerate()
                            .filter(|&(_, &c)| c != 0)
                            .map(|(k, &c)| {
                                let mut mono = vec![0u32; n];
                                mono[i] = k as u32;
                                (mono, c)
                            })
                            .collect()
                    })
                    .collect();
                let us: Vec<usize> = (m..n).collect();
                sys.extend(chain(&e, &(0..m).collect::<Vec<_>>(), &us, xr, n));
                let input = norm(&sys, p);
                let rep = f4_once(&input, n, p, budget, dmax);
                println!(
                    "{}",
                    json!({"schema": "pkm-ideas.dense/v1", "p": p, "S": s, "m": m, "vkind": vkind,
                    "draw": draw, "seed": seed, "target_x": xr, "n_vars": n, "nonzero_terms_membership": mpoly.iter().filter(|&&c| c != 0).count(),
                    "inconsistent": rep.inconsistent, "agree": !rep.timed_out && rep.inconsistent,
                    "censored": rep.timed_out || rep.pairs_above_bound > 0, "dmax": dmax, "field_ops": rep.field_ops,
                    "solving_degree_max": rep.solving_degree_max, "max_cols": rep.max_cols, "ms": rep.ms, "threads": threads})
                );
            }
        }
        // ---------------- ideas 3 and 4: record a trace once, replay it
        "replay" => {
            let degrees: Vec<u64> = arg(&o, "degrees", "2,2,2,2")
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            let m: usize = arg(&o, "m", "2").parse().unwrap();
            let rseed: u64 = arg(&o, "replay-seed", &seed.to_string()).parse().unwrap();
            let rp: u64 = arg(&o, "replay-p", &p.to_string()).parse().unwrap();
            let build = |pp: u64,
                         sd: u64,
                         xr: Option<u64>|
             -> (Vec<Poly>, usize, Vec<Vec<u64>>, Curve, u64) {
                let t = kummer_tower(pp, &degrees, None, sd).unwrap();
                let e = curve_from(sd, pp);
                let k = degrees.len();
                let n = m * k + (m - 2);
                let vs = vec![t.v.clone(); m];
                let x = xr.unwrap_or_else(|| refute_targets(&e, &vs, 1, sd)[0]);
                let mut sys = vec![];
                for i in 0..m {
                    sys.extend(tower_eqs(&degrees, i * k, t.c, n, pp));
                }
                let xs: Vec<usize> = (0..m).map(|i| i * k).collect();
                let us: Vec<usize> = (m * k..n).collect();
                sys.extend(chain(&e, &xs, &us, x, n));
                (norm(&sys, pp), n, vs, e, x)
            };
            let (rin, n, _, _, rx) = build(p, seed, None);
            let opts = F4Options::new(Ordering::Grevlex, dmax);
            let (rrep, trace) = f4_fp::f4_record(&rin, n, p, &opts);
            println!(
                "{}",
                json!({"schema": "pkm-ideas.record/v1", "p": p, "degrees": degrees, "m": m, "seed": seed,
                "target_x": rx, "field_ops": rrep.field_ops, "inconsistent": rrep.inconsistent,
                "trace_steps": trace.steps.len(), "threads": threads})
            );
            // replay targets on (rp, rseed)
            let t2 = kummer_tower(rp, &degrees, None, rseed).unwrap();
            let e2 = curve_from(rseed, rp);
            let vs2 = vec![t2.v.clone(); m];
            let xs = refute_targets(&e2, &vs2, draws + 1, rseed ^ 0x99);
            for (draw, &xr) in xs.iter().enumerate() {
                let (inp, n2, _, _, _) = build(rp, rseed, Some(xr));
                let fresh = f4_once(&inp, n2, rp, budget, dmax);
                let rep = f4_fp::f4_replay(&inp, n2, rp, &opts, &trace);
                println!(
                    "{}",
                    json!({"schema": "pkm-ideas.replay/v1", "record_p": p, "record_seed": seed, "p": rp,
                    "seed": rseed, "degrees": degrees, "m": m, "draw": draw, "target_x": xr,
                    "fresh_field_ops": fresh.field_ops, "fresh_inconsistent": fresh.inconsistent,
                    "replay_field_ops": rep.field_ops, "replay_inconsistent": rep.inconsistent,
                    "trace_mismatch": rep.trace_mismatch,
                    "agree": !rep.trace_mismatch && rep.inconsistent == fresh.inconsistent && fresh.inconsistent,
                    "ratio_replay_over_fresh": rep.field_ops as f64 / fresh.field_ops.max(1) as f64,
                    "fresh_ms": fresh.ms, "replay_ms": rep.ms, "threads": threads})
                );
            }
        }
        // ---------------- idea 5a: F4 work with a separate coset per point
        "cosets" => {
            let degrees: Vec<u64> = arg(&o, "degrees", "2,2,2,2")
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            let s: u64 = degrees.iter().product();
            let k = degrees.len();
            let n = 2 * k;
            let t = kummer_tower(p, &degrees, None, seed).unwrap();
            let e = curve_from(seed, p);
            // second coset: g2 = g * h with h not in mu_S (so V2 != V)
            let mut r = Rng(seed ^ 0xc05);
            // h outside mu_S, so V2 = h V is a different coset
            let h = loop {
                let h = 2 + r.below(p - 3);
                if powm(h, s, p) != 1 {
                    break h;
                }
            };
            let v2: Vec<u64> = t.v.iter().map(|&x| mulm(x, h, p)).collect();
            let c2 = mulm(t.c, powm(h, s, p), p);
            for (label, vs, cs) in [
                ("shared", vec![t.v.clone(), t.v.clone()], [t.c, t.c]),
                ("per_point", vec![t.v.clone(), v2.clone()], [t.c, c2]),
            ] {
                let targets = refute_targets(&e, &vs, draws, seed ^ 0x51);
                for (draw, &xr) in targets.iter().enumerate() {
                    let mut sys = tower_eqs(&degrees, 0, cs[0], n, p);
                    sys.extend(tower_eqs(&degrees, k, cs[1], n, p));
                    sys.extend(chain(&e, &[0, k], &[], xr, n));
                    let rep = f4_once(&norm(&sys, p), n, p, budget, dmax);
                    println!(
                        "{}",
                        json!({"schema": "pkm-ideas.cosets/v1", "p": p, "degrees": degrees, "S": s,
                        "layout": label, "draw": draw, "target_x": xr, "inconsistent": rep.inconsistent,
                        "agree": !rep.timed_out && rep.inconsistent, "field_ops": rep.field_ops,
                        "solving_degree_max": rep.solving_degree_max, "threads": threads})
                    );
                }
            }
        }
        // ---------------- idea 5b: exact coverage of m = 2 sums
        "coverage" => {
            let s: u64 = arg(&o, "S", "16").parse().unwrap();
            assert!(is_prime(p) && (p - 1).is_multiple_of(s));
            let mut r = Rng(seed ^ 0xc0e);
            // prime-order curve (exhaustive order, small p)
            let (e, n) = loop {
                let c = Curve {
                    p,
                    a: r.below(p),
                    b: r.below(p),
                };
                if c.is_singular() {
                    continue;
                }
                let n = c.order(&mut r).unwrap();
                if is_prime(n) {
                    break (c, n);
                }
            };
            let zeta = loop {
                let h = 2 + r.below(p - 3);
                let z = powm(h, (p - 1) / s, p);
                let mut ok = true;
                let mut ss = s;
                let mut q = 2;
                while q <= ss {
                    if ss.is_multiple_of(q) {
                        if powm(z, s / q, p) == 1 {
                            ok = false;
                        }
                        while ss.is_multiple_of(q) {
                            ss /= q;
                        }
                    }
                    q += 1;
                }
                if ok {
                    break z;
                }
            };
            let coset =
                |g: u64| -> Vec<u64> { (0..s).map(|k| mulm(g, powm(zeta, k, p), p)).collect() };
            let lift = |v: &[u64]| -> Vec<(u64, u64)> {
                let mut out = vec![];
                for &x in v {
                    if let Some(y) = sqrtm(e.rhs(x), p) {
                        out.push((x, y));
                        if y != 0 {
                            out.push((x, p - y));
                        }
                    }
                }
                out
            };
            let g1 = 1 + r.below(p - 1);
            let mut g2 = 1 + r.below(p - 1);
            while powm(mulm(g2, invm(g1, p), p), s, p) == 1 {
                g2 = 1 + r.below(p - 1);
            }
            let rand_v = |r: &mut Rng| -> Vec<u64> {
                let mut st = HashSet::new();
                while st.len() < s as usize {
                    st.insert(1 + r.below(p - 1));
                }
                st.into_iter().collect()
            };
            let sums = |a: &[(u64, u64)], b: &[(u64, u64)]| -> usize {
                let mut set: HashSet<Option<(u64, u64)>> = HashSet::new();
                for &x in a {
                    for &y in b {
                        set.insert(e.add(Some(x), Some(y)));
                    }
                }
                set.len()
            };
            let f1 = lift(&coset(g1));
            let f2 = lift(&coset(g2));
            let r1 = lift(&rand_v(&mut r));
            let r2 = lift(&rand_v(&mut r));
            println!(
                "{}",
                json!({"schema": "pkm-ideas.coverage/v1", "p": p, "n": n, "S": s,
                "F_shared": f1.len(), "F_other": f2.len(), "F_rand1": r1.len(), "F_rand2": r2.len(),
                "covered_shared": sums(&f1, &f1), "covered_per_point": sums(&f1, &f2),
                "covered_random_shared": sums(&r1, &r1), "covered_random_pair": sums(&r1, &r2)})
            );
        }
        // ---------------- idea 2: tower matched to the j = 0 automorphism
        "aut" => {
            let s: u64 = arg(&o, "S", "36").parse().unwrap();
            let tk: u64 = arg(&o, "targets", "2000").parse().unwrap();
            assert!(is_prime(p) && p % 3 == 1 && (p - 1).is_multiple_of(s) && s.is_multiple_of(3));
            let mut r = Rng(seed ^ 0xa07);
            let (e, n) = loop {
                let c = Curve {
                    p,
                    a: 0,
                    b: 1 + r.below(p - 1),
                };
                let n = c.order(&mut r).unwrap();
                if is_prime(n) {
                    break (c, n);
                }
            };
            // primitive cube root of unity mod p, and lambda mod n with omega(P) = [lambda] P
            let z3 = loop {
                let h = 2 + r.below(p - 3);
                let z = powm(h, (p - 1) / 3, p);
                if z != 1 {
                    break z;
                }
            };
            let omega = |pt: (u64, u64)| (mulm(z3, pt.0, p), pt.1);
            let gpt = e.random_point(&mut r).unwrap();
            let mut lam = None;
            // roots of l^2 + l + 1 mod n by search over cube roots of unity in Z/n (n prime, n = 1 mod 3)
            if n % 3 == 1 {
                let l = loop {
                    let h = 2 + r.below(n - 3);
                    let l = powm(h, (n - 1) / 3, n);
                    if l != 1 {
                        break l;
                    }
                };
                for cand in [l, mulm(l, l, n)] {
                    if e.mul(cand, Some(gpt)) == Some(omega(gpt)) {
                        lam = Some(cand);
                    }
                }
            }
            // V = g mu_S built directly (S may be composite)
            let zeta = loop {
                let h = 2 + r.below(p - 3);
                let z = powm(h, (p - 1) / s, p);
                let mut ok = true;
                let mut ss = s;
                let mut q = 2;
                while q <= ss {
                    if ss.is_multiple_of(q) {
                        if powm(z, s / q, p) == 1 {
                            ok = false;
                        }
                        while ss.is_multiple_of(q) {
                            ss /= q;
                        }
                    }
                    q += 1;
                }
                if ok {
                    break z;
                }
            };
            let g = 1 + r.below(p - 1);
            let v: Vec<u64> = (0..s).map(|k| mulm(g, powm(zeta, k, p), p)).collect();
            let vset: HashSet<u64> = v.iter().copied().collect();
            let mut f: Vec<(u64, u64)> = vec![];
            for &x in &v {
                if let Some(y) = sqrtm(e.rhs(x), p) {
                    f.push((x, y));
                    if y != 0 {
                        f.push((x, p - y));
                    }
                }
            }
            let fset: HashSet<(u64, u64)> = f.iter().copied().collect();
            let closed = f.iter().all(|&pt| fset.contains(&omega(pt)));
            let eigen = lam.map(|l| f.iter().all(|&pt| e.mul(l, Some(pt)) == Some(omega(pt))));
            // orbits under <omega, -1>
            let mut seen: HashSet<(u64, u64)> = HashSet::new();
            let mut orbits = 0;
            for &pt in &f {
                if seen.contains(&pt) {
                    continue;
                }
                orbits += 1;
                let mut q = pt;
                for _ in 0..3 {
                    seen.insert(q);
                    seen.insert((q.0, (p - q.1) % p));
                    q = omega(q);
                }
            }
            // decomposition counts are omega-invariant: d(R) == d(omega(R))
            let mut inv_ok = 0;
            let mut nonzero = 0;
            for k in 0..tk {
                // half planted (a sum of two factor-base points), half random
                let rpt = if k % 2 == 0 && f.len() >= 2 {
                    let a = f[r.below(f.len() as u64) as usize];
                    let b = f[r.below(f.len() as u64) as usize];
                    match e.add(Some(a), Some(b)) {
                        Some(q) => q,
                        None => e.random_point(&mut r).unwrap(),
                    }
                } else {
                    e.random_point(&mut r).unwrap()
                };
                let (a, _) = enumerate_decompositions(&e, &v, 2, rpt);
                let (b, _) = enumerate_decompositions(&e, &v, 2, omega(rpt));
                if a.len() == b.len() {
                    inv_ok += 1;
                }
                if !a.is_empty() {
                    nonzero += 1;
                }
            }
            // random-set control: not omega-closed
            let mut rset = HashSet::new();
            while rset.len() < s as usize {
                rset.insert(1 + r.below(p - 1));
            }
            let rclosed = rset
                .iter()
                .filter(|&&x| rset.contains(&mulm(z3, x, p)))
                .count();
            let _ = vset;
            println!(
                "{}",
                json!({"schema": "pkm-ideas.aut/v1", "p": p, "n": n, "S": s, "curve_b": e.b,
                "F": f.len(), "orbits_neg_omega": orbits, "columns_negation_only": f.len() / 2,
                "omega_closed": closed, "lambda": lam, "eigen_relation_holds": eigen,
                "targets": tk, "decomp_count_invariant": inv_ok, "targets_with_relation": nonzero,
                "random_set_omega_closed_elements": rclosed})
            );
        }
        _ => panic!("mode: dense | replay | cosets | coverage | aut"),
    }
}
