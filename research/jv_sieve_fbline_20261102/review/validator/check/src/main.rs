//! Independent validator check for `research/jv_sieve_fbline_20261102/`
//! (factor-base line search on the JV sieve route).  Written from the frozen
//! PROTOCOL.md and the record schema only; it shares no code with the
//! producer's `fit/` tool (own bootstrap, own RNG, own quantile rule, own
//! normal-maximum quadrature).
//!
//! Usage: fbline_validator_check <repo_root> <replay_dir> <out_json>
//! Prints a plain-text report on stdout; writes the headline numbers as JSON.

use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::fs;

const PRIMES: [u64; 3] = [1009, 1511, 1823];
const ARMS: [&str; 4] = ["P", "H", "T", "U"];
const SEEDS: std::ops::RangeInclusive<u64> = 17..=32;
const B: usize = 20000;
const GAE: f64 = 331.0;

// ---------------------------------------------------------------- helpers
fn load(path: &str) -> Vec<Value> {
    let t = fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    match serde_json::from_str::<Value>(&t).unwrap_or_else(|e| panic!("{path}: {e}")) {
        Value::Array(v) => v,
        v => vec![v],
    }
}
fn u(v: &Value, k: &str) -> u64 {
    v.get(k)
        .and_then(|x| x.as_u64())
        .unwrap_or_else(|| panic!("missing u64 {k}"))
}
fn fl(v: &Value, k: &str) -> f64 {
    v.get(k)
        .and_then(|x| x.as_f64())
        .unwrap_or_else(|| panic!("missing f64 {k}"))
}
fn bo(v: &Value, k: &str) -> bool {
    v.get(k)
        .and_then(|x| x.as_bool())
        .unwrap_or_else(|| panic!("missing bool {k}"))
}
fn fb(v: &Value, k: &str) -> u64 {
    v.get("fb_line").map_or(0, |f| u(f, k))
}
fn rel(a: f64, b: f64) -> f64 {
    ((a - b) / b).abs()
}
/// Keys whose values differ between two records (wall_ms excluded), and the
/// keys present in only one.
fn diff_keys(a: &Value, b: &Value) -> Vec<String> {
    let (ma, mb) = (a.as_object().unwrap(), b.as_object().unwrap());
    let mut out = Vec::new();
    for (k, va) in ma {
        if k == "wall_ms" {
            continue;
        }
        match mb.get(k) {
            Some(vb) if vb == va => {}
            Some(_) => out.push(k.clone()),
            None => out.push(format!("{k}(only left)")),
        }
    }
    for k in mb.keys() {
        if k != "wall_ms" && !ma.contains_key(k) {
            out.push(format!("{k}(only right)"));
        }
    }
    out
}

// splitmix64, validator's own stream (seed unrelated to the producer's)
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        // rejection-free multiply-shift; bias < 2^-50 for n = 16
        ((self.next() as u128 * n as u128) >> 64) as usize
    }
}
/// Type-7 (linear interpolation) quantile of a sorted slice.
fn quant(s: &[f64], q: f64) -> f64 {
    let h = (s.len() - 1) as f64 * q;
    let lo = h.floor() as usize;
    let hi = (lo + 1).min(s.len() - 1);
    s[lo] + (h - lo as f64) * (s[hi] - s[lo])
}
fn ci(mut v: Vec<f64>) -> (f64, f64) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (quant(&v, 0.025), quant(&v, 0.975))
}
fn geo(x: &[f64]) -> f64 {
    (x.iter().map(|v| v.ln()).sum::<f64>() / x.len() as f64).exp()
}
fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}
/// Geometric mean of per-pair ratios per prime and pooled (strata = primes),
/// with percentile bootstrap intervals.  `r[i]` = 16 ratios of prime i.
fn boot_geo(r: &[Vec<f64>; 3], seed: u64) -> ([(f64, f64, f64); 3], (f64, f64, f64)) {
    let logs: Vec<Vec<f64>> = r.iter().map(|v| v.iter().map(|x| x.ln()).collect()).collect();
    let mut rng = Rng(seed);
    let mut per: [Vec<f64>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut pooled = Vec::with_capacity(B);
    for _ in 0..B {
        let mut tot = 0.0;
        let mut cnt = 0usize;
        for (i, l) in logs.iter().enumerate() {
            let n = l.len();
            let mut s = 0.0;
            for _ in 0..n {
                s += l[rng.below(n)];
            }
            per[i].push((s / n as f64).exp());
            tot += s;
            cnt += n;
        }
        pooled.push((tot / cnt as f64).exp());
    }
    let mut pp = [(0.0, 0.0, 0.0); 3];
    for i in 0..3 {
        let (a, b) = ci(per[i].clone());
        pp[i] = (geo(&r[i]), a, b);
    }
    let all: Vec<f64> = r.iter().flatten().copied().collect();
    let (a, b) = ci(pooled);
    (pp, (geo(&all), a, b))
}

// ------------------------------------------------- normal maximum k(n)
fn erfc(z: f64) -> f64 {
    if z < 0.0 {
        return 2.0 - erfc(-z);
    }
    if z < 2.5 {
        // erf by its Taylor series
        let mut term = z;
        let mut s = z;
        let mut n = 0.0;
        loop {
            n += 1.0;
            term *= -z * z / n;
            let add = term / (2.0 * n + 1.0);
            s += add;
            if add.abs() <= 1e-17 * s.abs() {
                break;
            }
        }
        1.0 - s * 2.0 / std::f64::consts::PI.sqrt()
    } else {
        // continued fraction (modified Lentz) for erfc
        let tiny = 1e-300;
        let mut f = z;
        let mut c = z;
        let mut d = 0.0;
        for k in 1..400 {
            let a = k as f64 / 2.0;
            d = z + a * d;
            if d.abs() < tiny {
                d = tiny;
            }
            c = z + a / c;
            if c.abs() < tiny {
                c = tiny;
            }
            d = 1.0 / d;
            let delta = c * d;
            f *= delta;
            if (delta - 1.0).abs() < 1e-16 {
                break;
            }
        }
        (-z * z).exp() / f / std::f64::consts::PI.sqrt()
    }
}
/// Upper tail Q(x) = 1 - Phi(x).
fn qtail(x: f64) -> f64 {
    0.5 * erfc(x / std::f64::consts::SQRT_2)
}
/// E[max of n iid N(0,1)] = int_0^inf (1 - Phi^n) - int_-inf^0 Phi^n, Simpson.
fn kmax(n: f64) -> f64 {
    if n <= 1.0 {
        return 0.0;
    }
    let h = 1e-3;
    let simpson = |a: f64, b: f64, g: &dyn Fn(f64) -> f64| {
        let m = ((b - a) / h).round() as usize;
        let m = m + m % 2;
        let hh = (b - a) / m as f64;
        let mut s = g(a) + g(b);
        for i in 1..m {
            let x = a + i as f64 * hh;
            s += if i % 2 == 1 { 4.0 } else { 2.0 } * g(x);
        }
        s * hh / 3.0
    };
    let pos = |x: f64| -> f64 {
        let q = qtail(x);
        -(n * (-q).ln_1p()).exp_m1()
    };
    let neg = |x: f64| -> f64 {
        let q = qtail(-x); // Phi(x) for x < 0 is Q(-x)
        (n * q.ln()).exp()
    };
    simpson(0.0, 14.0, &pos) - simpson(-14.0, 0.0, &neg)
}

// ------------------------------------------------------------------ main
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let root = a.get(1).expect("repo root").clone();
    let replay = a.get(2).expect("replay dir").clone();
    let outp = a.get(3).expect("out json").clone();
    let d = format!("{root}/research/jv_sieve_fbline_20261102/results");
    let v2 = format!("{root}/research/jv_sieve_strong_rho_v2_20261020/raw");
    let mut fails: Vec<String> = Vec::new();
    let mut head = Map::new();
    let check = |ok: bool, what: String, fails: &mut Vec<String>| {
        println!("  [{}] {}", if ok { "ok" } else { "FAIL" }, what);
        if !ok {
            fails.push(what);
        }
    };

    // ---- load
    let mut rec: BTreeMap<(String, u64), Vec<Value>> = BTreeMap::new();
    for p in PRIMES {
        for arm in ARMS.iter().chain(["c2h", "c2t", "c3"].iter()) {
            let v = load(&format!("{d}/{arm}_{p}.json"));
            let log = fs::read_to_string(format!("{d}/{arm}_{p}.log")).unwrap();
            let nlog = log.lines().filter(|l| l.starts_with("p=")).count();
            let want = if arm.len() == 1 { 16 } else { 1 };
            let seeds: Vec<u64> = v.iter().map(|r| u(r, "seed")).collect();
            let want_seeds: Vec<u64> = if want == 16 { SEEDS.collect() } else { vec![17] };
            let ps = v.iter().all(|r| u(r, "p") == p);
            check(
                v.len() == want && nlog == want && seeds == want_seeds && ps,
                format!("R0 {arm}_{p}: {} records, {} log lines, seeds {:?}..{:?}, p ok {}", v.len(), nlog, seeds.first(), seeds.last(), ps),
                &mut fails,
            );
            rec.insert((arm.to_string(), p), v);
        }
    }
    let mut v2r: BTreeMap<u64, Vec<Value>> = BTreeMap::new();
    for p in PRIMES {
        v2r.insert(p, load(&format!("{v2}/ic_{p}_sparse.json")));
    }

    // ---- receipts
    println!("\n# Receipts (every run of every arm and control)");
    let mut nrun = 0;
    let mut bad_c4 = 0;
    let mut bad_sum = 0;
    let mut bad_s = 0;
    let mut timed_out = 0u64;
    let mut incomplete = 0u64;
    for ((arm, p), v) in &rec {
        for r in v {
            nrun += 1;
            let c4 = bo(r, "solved") && bo(r, "correct") && !bo(r, "exhausted") && u(r, "rels_failed_verify") == 0
                && r["lin_mode"] == "sparse";
            if !c4 {
                bad_c4 += 1;
                println!("    C4 fails: {arm} {p} seed {}", u(r, "seed"));
            }
            timed_out += u(r, "descent_timed_out");
            incomplete += u(r, "descent_incomplete");
            let rel_m = u(r, "table_muls") + u(r, "enum_muls") + u(r, "sieve_muls") + u(r, "extract_muls") + u(r, "verify_muls");
            let des = u(r, "descent_weil_muls") + u(r, "descent_f4_muls") + u(r, "descent_lin_muls")
                + u(r, "descent_post_muls") + u(r, "descent_verify_muls") + u(r, "descent_stream_muls");
            let tot = u(r, "setup_muls") + u(r, "base_muls") + u(r, "relation_muls") + u(r, "descent_muls") + u(r, "la_muls");
            let plus = u(r, "total_muls") + u(r, "sieve_adds") + u(r, "sieve_lookups") + u(r, "enum_adds")
                + fb(r, "adds") + fb(r, "lookups");
            let ok = rel_m == u(r, "relation_muls") && des == u(r, "descent_muls") && tot == u(r, "total_muls")
                && u(r, "la_muls") == 16 * u(r, "la_ops") && plus == u(r, "total_plus")
                && u(r, "setup_muls") >= fb(r, "muls");
            if !ok {
                bad_sum += 1;
                println!("    phase sum fails: {arm} {p} seed {}", u(r, "seed"));
            }
            let sl = (u(r, "l") as f64).sqrt();
            let c = fl(r, "c_add_e");
            if rel(u(r, "total_muls") as f64 / (c * sl), fl(r, "s")) > 1e-12
                || rel(u(r, "total_plus") as f64 / (c * sl), fl(r, "s_plus")) > 1e-12
                || c != GAE
            {
                bad_s += 1;
            }
        }
    }
    check(bad_c4 == 0, format!("C4: {nrun} runs solved, correct, not exhausted, 0 failed verifications, lin_mode sparse ({bad_c4} bad)"), &mut fails);
    check(bad_sum == 0, format!("phase sums: relation = table+enum+sieve+extract+verify; descent = weil+f4+lin+post+verify+stream; total = setup+base+relation+descent+la; la = 16*la_ops; total_plus = total + sieve_adds + sieve_lookups + enum_adds + search adds + search lookups; setup >= search muls ({bad_sum} bad of {nrun})"), &mut fails);
    check(bad_s == 0, format!("s = total_muls/(c_add_e sqrt l), s_plus = total_plus/(c_add_e sqrt l) to 1e-12, c_add_e = 331 ({bad_s} bad)"), &mut fails);
    println!("  (info) descent_timed_out summed over all runs {timed_out}; descent_incomplete {incomplete}");

    // ---- pairing identity
    let mut bad_pair = 0;
    for p in PRIMES {
        let pr = &rec[&("P".into(), p)];
        for arm in ["H", "T", "U"] {
            for (x, y) in rec[&(arm.into(), p)].iter().zip(pr) {
                if u(x, "l") != u(y, "l") || x["c_add_e"] != y["c_add_e"] || u(x, "seed") != u(y, "seed") {
                    bad_pair += 1;
                }
            }
        }
    }
    check(bad_pair == 0, format!("pair identity: l, c_add_e, seed of every arm row equal P's ({bad_pair} bad of 144)"), &mut fails);

    // ---- C1: P vs V2
    println!("\n# Controls");
    let mut c1_eq = 0;
    for p in PRIMES {
        for (x, y) in rec[&("P".into(), p)].iter().zip(&v2r[&p]) {
            let dk = diff_keys(x, y);
            if dk.is_empty() {
                c1_eq += 1;
            } else {
                println!("    C1 diff p={p} seed {}: {:?}", u(x, "seed"), dk);
            }
        }
    }
    check(c1_eq == 48, format!("C1: arm P equals V2 sparse record in every field but wall_ms (key sets equal too): {c1_eq}/48"), &mut fails);

    // ---- C2
    for p in PRIMES {
        let p0 = &rec[&("P".into(), p)][0];
        for (lab, table) in [("c2h", false), ("c2t", true)] {
            let c = &rec[&(lab.into(), p)][0];
            let f = &c["fb_line"];
            let same = ["relations", "descent_digest", "descent_residuals", "base", "hits", "lines"]
                .iter()
                .all(|k| c[*k] == p0[*k]);
            let dm = u(c, "total_muls") - u(p0, "total_muls");
            let dp = u(c, "total_plus") - u(p0, "total_plus");
            let q = p * p;
            let want_m = (p - 1) / 2 + 5 + if table { 38 * q } else { 17 * p };
            let want_a = if table { 29 * q + 2 * p } else { 15 * p };
            let want_l = if table { q + p } else { p };
            let ok = same && dm == u(f, "muls") && dp == u(f, "muls") + u(f, "adds") + u(f, "lookups")
                && u(f, "best_index") == 0 && u(f, "muls") == want_m && u(f, "adds") == want_a && u(f, "lookups") == want_l;
            let dk = diff_keys(c, p0);
            check(ok, format!("{lab} p={p}: route fields equal P; d(total_muls) = {dm} = search muls (formula {want_m}); d(total_plus) = {dp} = muls+adds+lookups; best_index 0; fields differing from P: {dk:?}"), &mut fails);
        }
        // ---- C3
        let c3 = &rec[&("c3".into(), p)][0];
        let h0 = &rec[&("H".into(), p)][0];
        let (f3, fh) = (&c3["fb_line"], &h0["fb_line"]);
        let ok = ["b_plain", "b_best", "best_index", "candidates"].iter().all(|k| f3[*k] == fh[*k])
            && c3["relations"] == h0["relations"] && c3["descent_digest"] == h0["descent_digest"]
            && u(c3, "total_muls") - u(f3, "muls") == u(h0, "total_muls") - u(fh, "muls");
        let dk = diff_keys(c3, h0);
        check(ok, format!("C3 p={p}: table n=4096 = Horner n=4096 on seed 17: B {}->{} idx {}; route muls {} equal; differing fields {dk:?}", u(f3, "b_plain"), u(f3, "b_best"), u(f3, "best_index"), u(c3, "total_muls") - u(f3, "muls")), &mut fails);
    }
    // ---- C5 and the search counters
    let mut bad_c5 = 0;
    let mut bad_ctr = 0;
    let mut resid: BTreeMap<(String, u64), Vec<i64>> = BTreeMap::new();
    for p in PRIMES {
        let q = p * p;
        let lcls = p.div_ceil(2) * p;
        for arm in ["H", "T", "U"] {
            for (x, y) in rec[&(arm.into(), p)].iter().zip(&rec[&("P".into(), p)]) {
                let f = &x["fb_line"];
                if u(f, "b_plain") as u64 != u(y, "base") || u(f, "b_best") != u(x, "base") {
                    bad_c5 += 1;
                }
                let (mode, n, exh) = match arm {
                    "H" => ("horner", 4096u64, false),
                    "T" => ("table", 1 + lcls, true),
                    _ => ("table", 32768, false),
                };
                let table = mode == "table";
                let walked = n; // lines walked (plain line included)
                let want_a = if table { 29 * q + 2 * p * walked } else { 15 * p * walked };
                let want_l = if table { q + p * walked } else { p * walked };
                let want_t = if table { 38 * q } else { 0 };
                let ok = f["mode"] == mode && bo(f, "exhaustive") == exh && u(f, "candidates") == n
                    && u(f, "adds") == want_a && u(f, "lookups") == want_l && u(f, "points") == want_l
                    && u(f, "table_muls") == want_t;
                if !ok {
                    bad_ctr += 1;
                    println!("    counter mismatch {arm} p={p} seed {}", u(x, "seed"));
                }
                // muls beyond the counted floor: qr table, t^2 per candidate, per-point charge
                let floor = (p - 1) / 2
                    + if exh { 5 + 5 * (p + 1) } else { 5 * n }
                    + if table { 38 * q } else { 17 * p * n };
                resid.entry((arm.into(), p)).or_default().push(u(f, "muls") as i64 - floor as i64);
            }
        }
    }
    check(bad_c5 == 0, format!("C5: fb_line.b_plain = P base and b_best = arm base on all 144 pairs ({bad_c5} bad)"), &mut fails);
    check(bad_ctr == 0, format!("search counters exact on all 144 arm rows: Horner adds 15pn, lookups pn; table adds 29q+2pn, lookups q+pn, table_muls 38q; T exhaustive with n = 1 + p(p+1)/2 ({bad_ctr} bad)"), &mut fails);
    println!("  search muls above the counted floor [(p-1)/2 + 5 per candidate t^2 + 17pn (Horner) | 38q (table)]:");
    for ((arm, p), r) in &resid {
        let (mn, mx) = (r.iter().min().unwrap(), r.iter().max().unwrap());
        let per = if arm == "H" { format!(" = {:.2} per extra candidate (line_poly) + move", *mn as f64 / 4095.0) } else { String::new() };
        println!("    {arm} p={p}: min {mn} max {mx}{per}");
    }

    // ---- replays
    println!("\n# Replays (frozen main binary vs committed records, every field but wall_ms)");
    let mut replays = Vec::new();
    for (arm, seed) in [("H", 27u64), ("U", 21), ("P", 32), ("T", 18)] {
        let path = format!("{replay}/{arm}_1511_s{seed}.json");
        if !std::path::Path::new(&path).exists() {
            println!("  (no replay file {path})");
            continue;
        }
        let rr = load(&path);
        let orig = rec[&(arm.into(), 1511)].iter().find(|r| u(r, "seed") == seed).unwrap();
        let dk = diff_keys(&rr[0], orig);
        let mut extra = String::new();
        if arm == "P" {
            let v = v2r[&1511].iter().find(|r| u(r, "seed") == seed).unwrap();
            extra = format!("; vs V2 record: differing {:?}", diff_keys(&rr[0], v));
        }
        let ok = rr.len() == 1 && dk.is_empty();
        check(ok, format!("replay {arm} p=1511 seed {seed}: differing fields {dk:?}{extra}; total_muls {} total_plus {} digest {}", u(&rr[0], "total_muls"), u(&rr[0], "total_plus"), u(&rr[0], "descent_digest")), &mut fails);
        replays.push(json!({"arm": arm, "p": 1511, "seed": seed, "identical_except_wall_ms": ok,
            "total_muls": u(&rr[0], "total_muls"), "total_plus": u(&rr[0], "total_plus"),
            "descent_digest": u(&rr[0], "descent_digest"), "replay_wall_ms": fl(&rr[0], "wall_ms"), "original_wall_ms": fl(orig, "wall_ms")}));
    }

    // ---- metrics
    println!("\n# Paired ratios arm/P (geometric mean over seeds; 95% percentile bootstrap, {B} resamples, validator RNG)");
    let mut seedc = 0x0A11_DA7E_2026_1103u64;
    for arm in ["H", "T", "U"] {
        let mut am = Map::new();
        let metrics: Vec<(String, Box<dyn Fn(&Value, &Value) -> f64>)> = vec![
            ("S".into(), Box::new(move |x: &Value, y: &Value| s_of(x, false) / s_of(y, false))),
            ("S+".into(), Box::new(move |x: &Value, y: &Value| s_of(x, true) / s_of(y, true))),
            ("band_w0".into(), Box::new(move |x: &Value, y: &Value| cost(x, 0.0) / cost(y, 0.0))),
            ("band_w0.1".into(), Box::new(move |x: &Value, y: &Value| cost(x, 0.1) / cost(y, 0.1))),
            ("band_w1".into(), Box::new(move |x: &Value, y: &Value| cost(x, 1.0) / cost(y, 1.0))),
        ];
        for (name, m) in &metrics {
            let r: [Vec<f64>; 3] = PRIMES.map(|p| {
                rec[&(arm.into(), p)].iter().zip(&rec[&("P".into(), p)]).map(|(x, y)| m(x, y)).collect()
            });
            seedc = seedc.wrapping_add(0x1234_5678_9ABC);
            let (pp, po) = boot_geo(&r, seedc);
            println!(
                "  {arm} {name:<9} p=1009 {:.4} [{:.4}, {:.4}]  p=1511 {:.4} [{:.4}, {:.4}]  p=1823 {:.4} [{:.4}, {:.4}]  pooled {:.4} [{:.4}, {:.4}]  (x{:.3})",
                pp[0].0, pp[0].1, pp[0].2, pp[1].0, pp[1].1, pp[1].2, pp[2].0, pp[2].1, pp[2].2, po.0, po.1, po.2, 1.0 / po.0
            );
            am.insert(name.clone(), json!({"p1009": [pp[0].0, pp[0].1, pp[0].2], "p1511": [pp[1].0, pp[1].1, pp[1].2],
                "p1823": [pp[2].0, pp[2].1, pp[2].2], "pooled": [po.0, po.1, po.2]}));
        }
        head.insert(arm.into(), Value::Object(am));
    }

    println!("\n# Absolute arm S and S+ per prime (arithmetic mean | geometric mean over 16 seeds)");
    let mut absm = Map::new();
    for arm in ARMS {
        for p in PRIMES {
            let v = &rec[&(arm.into(), p)];
            let s: Vec<f64> = v.iter().map(|r| s_of(r, false)).collect();
            let sp: Vec<f64> = v.iter().map(|r| s_of(r, true)).collect();
            println!("  {arm} p={p}: S {:.5} | {:.5}   S+ {:.5} | {:.5}", mean(&s), geo(&s), mean(&sp), geo(&sp));
            absm.insert(format!("{arm}_{p}"), json!({"S_mean": mean(&s), "S_geo": geo(&s), "Splus_mean": mean(&sp), "Splus_geo": geo(&sp)}));
        }
    }
    head.insert("absolute".into(), Value::Object(absm));

    // ---- sensitivity: unpriced table index arithmetic (x0*p + x1 per line-walk lookup: 1 integer mul + 1 add)
    println!("\n# Sensitivity (validator, not in the protocol): table arms with 2 extra native ops per line-walk lookup (index arithmetic)");
    for arm in ["T", "U"] {
        let mut line = format!("  {arm}:");
        for w in [1.0 / GAE, 0.1, 1.0] {
            let r: [Vec<f64>; 3] = PRIMES.map(|p| {
                rec[&(arm.into(), p)].iter().zip(&rec[&("P".into(), p)]).map(|(x, y)| {
                    let walk = (fb(x, "lookups") - p * p) as f64;
                    (cost(x, w) + w * fl(x, "c_add_e") * 2.0 * walk) / cost(y, w)
                }).collect()
            });
            let (pp, po) = boot_geo(&r, 0x5E45_0000 + (w * 10.0) as u64 + arm.as_bytes()[0] as u64);
            line += &format!("  w={}: pooled {:.4} [{:.4}, {:.4}] per prime {:.3}/{:.3}/{:.3}", if w < 0.01 { "1/331 (=S+)".to_string() } else { w.to_string() }, po.0, po.1, po.2, pp[0].0, pp[1].0, pp[2].0);
        }
        println!("{line}");
    }
    // ---- estimate: uncharged moved-model line_poly inside every Ctx built (341 F_p mults each)
    {
        let mut worst: f64 = 0.0;
        for arm in ["H", "T", "U"] {
            for p in PRIMES {
                for r in &rec[&(arm.into(), p)] {
                    let chunk = (4_000_000 / p).max(64) as f64;
                    let ctxs = u(r, "sieve_steps") as f64 / (chunk * p as f64) + 4.0 + u(r, "descent_residuals") as f64 + 1.0;
                    worst = worst.max(ctxs * 341.0 / u(r, "total_muls") as f64);
                }
            }
        }
        println!("  estimate (model): uncharged line_poly of the moved model per Ctx (341 muls) x [sieve batches ~ sieve_steps/(chunk p) + 4 + one per descent residual + 1] <= {:.2e} of total_muls on every arm run", worst);
    }

    println!("\n# Section 5a counter list (mean per run): emitted native counters of every arm; F4/descent additions have no counter on any arm; byte counters: none");
    for arm in ARMS {
        for p in PRIMES {
            let v = &rec[&(arm.into(), p)];
            let m = |f: &dyn Fn(&Value) -> u64| mean(&v.iter().map(|r| f(r) as f64).collect::<Vec<_>>());
            println!("  {arm} p={p}: sieve_adds {:.3e} sieve_lookups {:.3e} enum_adds {:.3e} search_adds {:.3e} search_lookups {:.3e}",
                m(&|r| u(r, "sieve_adds")), m(&|r| u(r, "sieve_lookups")), m(&|r| u(r, "enum_adds")), m(&|r| fb(r, "adds")), m(&|r| fb(r, "lookups")));
        }
    }

    println!("\n# Phase shares of total_muls (mean over seeds of per-run share), and search share of total_plus");
    let mut shares = Map::new();
    for arm in ARMS {
        for p in PRIMES {
            let v = &rec[&(arm.into(), p)];
            let sh = |k: &dyn Fn(&Value) -> f64| -> f64 { mean(&v.iter().map(|r| k(r) / u(r, "total_muls") as f64).collect::<Vec<_>>()) * 100.0 };
            let e = sh(&|r| u(r, "enum_muls") as f64);
            let s = sh(&|r| u(r, "sieve_muls") as f64);
            let de = sh(&|r| u(r, "descent_muls") as f64);
            let la = sh(&|r| u(r, "la_muls") as f64);
            let se = sh(&|r| fb(r, "muls") as f64);
            let other = 100.0 - e - s - de - la - se;
            let nat_sh = mean(&v.iter().map(|r| (fb(r, "adds") + fb(r, "lookups")) as f64 / u(r, "total_plus") as f64).collect::<Vec<_>>()) * 100.0;
            let all_sh = mean(&v.iter().map(|r| (fb(r, "muls") + fb(r, "adds") + fb(r, "lookups")) as f64 / u(r, "total_plus") as f64).collect::<Vec<_>>()) * 100.0;
            println!("  {arm} p={p}: enum {e:.1}% sieve {s:.1}% descent {de:.1}% la {la:.1}% search {se:.2}% other {other:.2}% | search native/total_plus {nat_sh:.2}% (search muls+native {all_sh:.2}%)");
            shares.insert(format!("{arm}_{p}"), json!([e, s, de, la, se, other, nat_sh, all_sh]));
        }
    }
    head.insert("phase_shares_pct".into(), Value::Object(shares));

    println!("\n# B model check: route-only ratio (total_muls - search muls)/P total_muls vs (B_plain/B_best)^5");
    let mut bm = Map::new();
    for arm in ["H", "T", "U"] {
        let mut lr = Vec::new();
        let mut xy: [Vec<(f64, f64)>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for (i, p) in PRIMES.iter().enumerate() {
            for (x, y) in rec[&(arm.into(), *p)].iter().zip(&rec[&("P".into(), *p)]) {
                let route = (u(x, "total_muls") - fb(x, "muls")) as f64 / u(y, "total_muls") as f64;
                let bx = (fb(x, "b_plain") as f64 / fb(x, "b_best") as f64).ln();
                lr.push(route.ln() - 5.0 * bx);
                xy[i].push((bx, route.ln()));
            }
        }
        let m = mean(&lr);
        let sd = (lr.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (lr.len() - 1) as f64).sqrt();
        let gam = |s: &[(f64, f64)]| s.iter().map(|(x, y)| x * y).sum::<f64>() / s.iter().map(|(x, _)| x * x).sum::<f64>();
        let all: Vec<(f64, f64)> = xy.iter().flatten().copied().collect();
        let g = gam(&all);
        let mut rng = Rng(0xB0D3_1000 + arm.as_bytes()[0] as u64);
        let mut bs = Vec::with_capacity(B);
        for _ in 0..B {
            let mut s = Vec::with_capacity(48);
            for st in &xy {
                for _ in 0..st.len() {
                    s.push(st[rng.below(st.len())]);
                }
            }
            bs.push(gam(&s));
        }
        let (lo, hi) = ci(bs);
        println!("  {arm}: geo-mean measured/model {:.4}, sd of log {:.4}; gamma_hat {:.3} [{:.3}, {:.3}]", m.exp(), sd, g, lo, hi);
        bm.insert(arm.into(), json!({"geo_measured_over_model": m.exp(), "sd_log": sd, "gamma_hat": [g, lo, hi]}));
    }
    head.insert("b_model".into(), Value::Object(bm));

    // ---- the frozen n rule, recomputed
    println!("\n# n-selection rule recomputed (model; gamma 5, GAE 331, C_P from V2 sparse means)");
    let mut mp = [0.0; 3];
    let mut np = [0.0; 3];
    for (i, p) in PRIMES.iter().enumerate() {
        let v = &v2r[p];
        mp[i] = mean(&v.iter().map(|r| u(r, "total_muls") as f64).collect::<Vec<_>>());
        np[i] = mean(&v.iter().map(|r| (u(r, "sieve_adds") + u(r, "sieve_lookups") + u(r, "enum_adds")) as f64).collect::<Vec<_>>());
        println!("  p={p}: M_P {:.4e}  N_P {:.4e}", mp[i], np[i]);
    }
    for n in [1.0, 16.0, 256.0, 4096.0, 65536.0, 1e6] {
        print!("  k({n})={:.3}", kmax(n));
    }
    println!();
    let jcalc = |table: bool, n: Option<u64>, w: f64, kcap: bool| -> (f64, [f64; 3]) {
        let mut js = [0.0; 3];
        for (i, &p) in PRIMES.iter().enumerate() {
            let pf = p as f64;
            let q = pf * pf;
            let lcls = (p.div_ceil(2) * p) as f64;
            let nn = n.map_or(lcls, |x| x as f64);
            let keff = if table && kcap { nn.min(lcls) } else { nn };
            let cp = mp[i] + w * GAE * np[i];
            let route = cp * (1.0 + kmax(keff) / pf.sqrt()).powi(-5);
            let search = if table {
                38.0 * q + w * GAE * (30.0 * q + 3.0 * nn.min(lcls) * pf)
            } else {
                nn * pf * (17.0 + w * GAE * 16.0)
            };
            js[i] = (route + search) / cp;
        }
        (geo(&js), js)
    };
    let mut rule = Map::new();
    for w in [0.0, 0.1, 1.0] {
        for table in [false, true] {
            let mut best = (f64::MAX, String::new(), [0.0; 3]);
            let mut grid: Vec<Option<u64>> = (0..=20).map(|e| Some(1u64 << e)).collect();
            if table {
                grid.push(None);
            }
            for n in grid {
                let (j, js) = jcalc(table, n, w, true);
                if j < best.0 {
                    best = (j, n.map_or("exhaustive".into(), |x| x.to_string()), js);
                }
            }
            let lab = if table { "T (table)" } else { "H (Horner)" };
            println!("  w = {w}: {lab}: n = {}  J = {:.4} [p=1009 {:.4}, p=1511 {:.4}, p=1823 {:.4}]", best.1, best.0, best.2[0], best.2[1], best.2[2]);
            rule.insert(format!("w{w}_{}", if table { "T" } else { "H" }), json!({"n": best.1, "J": best.0}));
        }
    }
    let (j19a, _) = jcalc(true, Some(1 << 19), 0.0, true);
    let (j19b, _) = jcalc(true, Some(1 << 19), 0.0, false);
    let (j20a, _) = jcalc(true, Some(1 << 20), 0.0, true);
    let (j20b, _) = jcalc(true, Some(1 << 20), 0.0, false);
    println!("  table w=0 at n=2^19: {j19a:.4} (k capped at L) / {j19b:.4} (k uncapped); n=2^20: {j20a:.4} / {j20b:.4}");
    head.insert("rule".into(), Value::Object(rule));
    head.insert("replays".into(), Value::Array(replays));
    // ---- the producer's analysis.txt numbers against the recomputation
    println!("\n# Producer analysis/analysis.txt vs validator recomputation (points: rounding tolerance; intervals: Monte Carlo)");
    {
        let txt = fs::read_to_string(format!("{root}/research/jv_sieve_fbline_20261102/analysis/analysis.txt")).unwrap();
        let num = |s: &str| -> f64 { s.trim_matches(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-')).parse::<f64>().unwrap() };
        let mut gate = Vec::new();
        let (mut worst_pt, mut worst_ci): (f64, f64) = (0.0, 0.0);
        let mut npt = 0;
        for arm in ["H", "T", "U"] {
            let start = txt.find(&format!("### Arm {arm} against P")).unwrap();
            let sec = &txt[start..];
            let sec = &sec[..sec[4..].find("###").map_or(sec.len(), |i| i + 4)];
            for (prefix, metric) in [("  S  (muls", "S"), ("  S+ (muls", "S+"), ("    w = 0:", "band_w0"), ("    w = 0.1:", "band_w0.1"), ("    w = 1:", "band_w1")] {
                let line = sec.lines().find(|l| l.starts_with(prefix)).unwrap();
                let t: Vec<&str> = line.split_whitespace().collect();
                let mine = &head[arm][metric];
                let mut pairs: Vec<(String, f64, f64, f64, f64)> = Vec::new(); // (where, claimed pt, lo, hi, decimals)
                if metric == "S" || metric == "S+" {
                    for (i, tok) in t.iter().enumerate() {
                        for (w, key) in [("p=1009", "p1009"), ("p=1511", "p1511"), ("p=1823", "p1823"), ("pooled", "pooled")] {
                            if *tok == w {
                                pairs.push((key.into(), num(t[i + 1]), num(t[i + 2]), num(t[i + 3]), 4.0));
                            }
                        }
                    }
                } else {
                    let i = t.iter().position(|x| *x == "pooled").unwrap();
                    pairs.push(("pooled".into(), num(t[i + 1]), num(t[i + 2]), num(t[i + 3]), 4.0));
                    let j = t.iter().position(|x| *x == "prime").unwrap();
                    for (k, key) in ["p1009", "p1511", "p1823"].iter().enumerate() {
                        let a = t[j + 1 + 2 * k];
                        let b = t[j + 2 + 2 * k];
                        let (lo, hi) = b.trim_matches(|c| c == '[' || c == ']').split_once(',').unwrap();
                        pairs.push((key.to_string(), num(a), num(lo), num(hi), 3.0));
                    }
                }
                for (key, pt, lo, hi, dec) in pairs {
                    let m = mine[&key].as_array().unwrap();
                    let (mp, ml, mh) = (m[0].as_f64().unwrap(), m[1].as_f64().unwrap(), m[2].as_f64().unwrap());
                    let tol = 0.5 * 10f64.powf(-dec) + 1e-12;
                    worst_pt = worst_pt.max((mp - pt).abs() / tol);
                    worst_ci = worst_ci.max((ml - lo).abs()).max((mh - hi).abs());
                    npt += 1;
                    gate.push(json!({"name": format!("{arm} {metric} {key} (analysis.txt, {dec} decimals)"), "claimed": pt, "recomputed": mp, "rel_tol": tol / pt}));
                }
            }
        }
        let ok = worst_pt <= 1.0;
        println!("  [{}] {npt} point estimates within half a unit of the last printed decimal (worst {worst_pt:.3} of the tolerance); interval endpoints differ by at most {worst_ci:.4} (independent bootstrap streams)", if ok { "ok" } else { "FAIL" });
        if !ok {
            fails.push("analysis.txt point estimates differ".into());
        }
        head.insert("gate_headline".into(), Value::Array(gate));
    }
    head.insert("failures".into(), json!(fails));
    println!("\n# Failures: {}", if fails.is_empty() { "none".to_string() } else { fails.join(" | ") });
    fs::write(&outp, serde_json::to_string_pretty(&Value::Object(head)).unwrap()).unwrap();
}

/// S (or S+) of a record: total_muls (total_plus) / (c_add_e sqrt l).
fn s_of(r: &Value, plus: bool) -> f64 {
    let n = if plus { u(r, "total_plus") } else { u(r, "total_muls") };
    n as f64 / (fl(r, "c_add_e") * (u(r, "l") as f64).sqrt())
}
/// Emitted native operations of a record: route sieve adds, sieve lookups,
/// enumeration adds, and the search's adds and lookups.
fn native(r: &Value) -> f64 {
    (u(r, "sieve_adds") + u(r, "sieve_lookups") + u(r, "enum_adds") + fb(r, "adds") + fb(r, "lookups")) as f64
}
/// Section 5a cost at w GAE per native operation, in F_p multiplications.
fn cost(r: &Value, w: f64) -> f64 {
    u(r, "total_muls") as f64 + w * fl(r, "c_add_e") * native(r)
}
