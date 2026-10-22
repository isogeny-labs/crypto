//! Driver for `research/cheon_p224_shape_20261022/` (see its `PROTOCOL.md`).
//!
//! ```text
//! cheon_p224_shape gen   --out curves.json
//! cheon_p224_shape check --curves curves.json
//! cheon_p224_shape spec  --curves curves.json --out ecbench-spec.json
//! cheon_p224_shape run   --curves curves.json --out runs.jsonl [--family minus|plus]
//!                        [--bits 32,36] [--targets 6] [--timeout 1800]
//! cheon_p224_shape xcheck --curves curves.json      (existing cheon_attack.rs)
//! cheon_p224_shape analyse --curves curves.json --runs runs.jsonl --rho rho.jsonl --out DIR
//! ```
//!
//! Native Rust only.  The Cheon arms are `cryptanalysis::cheon_counted`;
//! the matched rho arm is `ecbench`'s `rho.negation`, run by `ecbench run`
//! on the spec this tool writes.  Workloads (curve, target, planted
//! logarithm) are rebuilt here with the same `Workload::build` so the two
//! see identical targets.

use crypto_lib::cryptanalysis::cheon_counted::{
    cheon_pminus1, cheon_pplus1, distinct_prime_factors, find_shaped_curve, leaked_powers,
    next_prime, scalar_mul_plain, shape_divisor, Side,
};
use crypto_lib::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind, Workload};
use crypto_lib::cryptanalysis::ic_boundary::{GroupOps, PrimeCurve, PrimePoint};
use serde_json::{json, Value};
use std::io::Write;
use std::time::{Duration, Instant};

const RHO_MINUS: f64 = 28.991_298_55 / 224.000_000_17;
const RHO_PLUS: f64 = 47.278_406_86 / 224.000_000_17;
const TOL: f64 = 0.015;
const SIZES: [u32; 7] = [32, 36, 40, 44, 48, 52, 56];
const TARGET_SEED: u64 = 20261022;

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn side_of(f: &str) -> Side {
    if f == "minus" {
        Side::Minus
    } else {
        Side::Plus
    }
}

fn load_curves(path: &str) -> Vec<Value> {
    let v: Value = serde_json::from_str(&std::fs::read_to_string(path).expect("read curves"))
        .expect("parse curves");
    v["curves"].as_array().expect("curves array").clone()
}

fn u(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap_or_else(|| panic!("field {k}"))
}

fn explicit_spec(c: &Value) -> CurveSpec {
    CurveSpec::PrimeExplicit {
        p: u(c, "p"),
        a: u(c, "a"),
        b: u(c, "b"),
        group_order: u(c, "n"),
        r: u(c, "n"),
        gx: u(c, "gx"),
        gy: u(c, "gy"),
    }
}

fn gen(args: &[String]) {
    let out = arg(args, "--out").expect("--out");
    let mut rows = Vec::new();
    for family in ["minus", "plus"] {
        for &bits in &SIZES {
            let f = if family == "minus" { 0u64 } else { 1 };
            let seed = (0x4368_6561u64 << 32) ^ ((bits as u64) << 8) ^ f;
            let p = next_prime(3u64 << (bits - 2));
            let (side, ratio) = if family == "minus" {
                (Side::Minus, RHO_MINUS)
            } else {
                (Side::Plus, RHO_PLUS)
            };
            let t0 = Instant::now();
            let s = find_shaped_curve(p, seed, side, ratio, TOL);
            let m = if family == "minus" { s.n - 1 } else { s.n + 1 };
            let log2n = (s.n as f64).log2();
            eprintln!(
                "{family} {bits}: n={} ({log2n:.2} bits) d={} ({:.2} bits, ratio {:.4}) tried {} in {:.1}s",
                s.n,
                s.d,
                (s.d as f64).log2(),
                (s.d as f64).log2() / log2n,
                s.tried,
                t0.elapsed().as_secs_f64()
            );
            rows.push(json!({
                "family": family, "bits": bits, "stream_seed": seed,
                "p": s.p, "a": s.a, "b": s.b, "n": s.n, "d": s.d, "q": m / s.d,
                "gx": s.gx, "gy": s.gy, "tried": s.tried,
                "log2_n": log2n, "log2_d": (s.d as f64).log2(),
                "ratio": (s.d as f64).log2() / log2n,
            }));
        }
    }
    let doc = json!({
        "schema": "cheon-p224-shape.curves/v1",
        "rho_minus": RHO_MINUS, "rho_plus": RHO_PLUS, "tolerance": TOL,
        "target_seed": TARGET_SEED,
        "construction": "random (a,b) from SplitMix64 stream, #E by Hasse-interval BSGS, first curve with prime n != p and n -/+ 1 = d*q (q prime), |log d/log n - rho| <= tol",
        "curves": rows,
    });
    std::fs::write(&out, serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap();
}

fn check(args: &[String]) {
    let curves = load_curves(&arg(args, "--curves").expect("--curves"));
    let mut bad = 0;
    for c in &curves {
        let (p, a, b, n, d) = (u(c, "p"), u(c, "a"), u(c, "b"), u(c, "n"), u(c, "d"));
        let (gx, gy) = (u(c, "gx"), u(c, "gy"));
        let fam = c["family"].as_str().unwrap();
        let curve = PrimeCurve { p, a, b };
        let mut errs = Vec::new();
        let prime = crypto_lib::cryptanalysis::challenge_ladder::is_prime_u64;
        if !prime(p) {
            errs.push("p not prime");
        }
        if !prime(n) || n == p {
            errs.push("n not prime or anomalous");
        }
        let g = PrimePoint::affine(gx, gy);
        if !curve.is_on_curve(g) {
            errs.push("G not on curve");
        }
        let mut ops = GroupOps::default();
        if !scalar_mul_plain(&curve, &mut ops, n, g).infinity {
            errs.push("[n]G != O");
        }
        let t = (4 * p).isqrt();
        if n < p + 1 - t || n > p + 1 + t {
            errs.push("n outside Hasse interval");
        }
        let m = if fam == "minus" { n - 1 } else { n + 1 };
        if m % d != 0 || !prime(m / d) {
            errs.push("n -/+ 1 != d * prime");
        }
        let ratio = (d as f64).log2() / (n as f64).log2();
        let target = if fam == "minus" { RHO_MINUS } else { RHO_PLUS };
        if (ratio - target).abs() > TOL + 1e-12 {
            errs.push("shape ratio outside tolerance");
        }
        if shape_divisor(n, side_of(fam), target, TOL).is_none() {
            errs.push("shape_divisor finds none");
        }
        if distinct_prime_factors(m).is_none() {
            errs.push("n -/+ 1 not factored");
        }
        if explicit_spec(c).build().is_err() {
            errs.push("ecbench rejects the curve");
        }
        println!(
            "{fam:5} {:2} bits  n={n} d={d} ratio={ratio:.4}  {}",
            c["bits"],
            if errs.is_empty() {
                "OK".to_string()
            } else {
                format!("FAIL {errs:?}")
            }
        );
        bad += !errs.is_empty() as u32;
    }
    if bad > 0 {
        std::process::exit(1);
    }
    println!("all {} curves verified", curves.len());
}

fn spec(args: &[String]) {
    let curves = load_curves(&arg(args, "--curves").expect("--curves"));
    let out = arg(args, "--out").expect("--out");
    let targets: u64 = arg(args, "--targets").map_or(6, |s| s.parse().unwrap());
    let only = arg(args, "--bits");
    let family = arg(args, "--family");
    let list: Vec<Value> = curves
        .iter()
        .filter(|c| family.as_deref().is_none_or(|f| c["family"] == f))
        .filter(|c| {
            only.as_deref().is_none_or(|s| {
                s.split(',')
                    .any(|b| b.parse::<u64>().ok() == c["bits"].as_u64())
            })
        })
        .map(|c| {
            json!({"kind": "prime_explicit", "p": u(c,"p"), "a": u(c,"a"), "b": u(c,"b"),
                   "group_order": u(c,"n"), "r": u(c,"n"), "gx": u(c,"gx"), "gy": u(c,"gy")})
        })
        .collect();
    let spec = json!({
        "schema": "ecbench.spec/v1",
        "label": "cheon-p224-shape: matched rho.negation on the frozen shaped curves",
        "description": "Matched rho reference for research/cheon_p224_shape_20261022/PROTOCOL.md. Operation counts only; wall time is descriptive.",
        "workloads": {"curves": list, "targets_per_curve": targets, "target_seed": TARGET_SEED},
        "arms": [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}],
        "measurement": {"rounds": 1, "warmup": 0, "order": "alternate", "seed": 1,
                        "isolation_required": "L0", "timeout_seconds": 1800}
    });
    std::fs::write(&out, serde_json::to_string_pretty(&spec).unwrap() + "\n").unwrap();
}

fn rss_hwm_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1)?.parse().ok())
        })
        .unwrap_or(0)
}

/// One Cheon run on workload `t` of curve `c`: the outcome, the leaked-power
/// count, the planted logarithm and the workload id.
fn execute(
    c: &Value,
    t: u64,
    arm: &str,
    deadline: Option<Instant>,
) -> (
    crypto_lib::cryptanalysis::cheon_counted::Outcome,
    u64,
    u64,
    String,
) {
    let (p, a, bb, n, d) = (u(c, "p"), u(c, "a"), u(c, "b"), u(c, "n"), u(c, "d"));
    let curve = PrimeCurve { p, a, b: bb };
    let cspec = explicit_spec(c);
    let inst = cspec.build().expect("build curve");
    let w = Workload::on(&cspec, &inst, TARGET_SEED, t, TargetKind::Planted).expect("workload");
    let alpha = w.planted.expect("planted") as u64;
    let parse = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap();
    let q = PrimePoint::affine(parse(&w.target[0]), parse(&w.target[1]));
    let g0 = PrimePoint::affine(u(c, "gx"), u(c, "gy"));
    let mut o = GroupOps::default();
    assert_eq!(
        scalar_mul_plain(&curve, &mut o, alpha, g0),
        q,
        "workload target"
    );
    let (outcome, leaked) = match arm {
        "cheon.pminus1_comb" | "cheon.pminus1_naive" => {
            // [alpha^d]G: the leaked power (not charged)
            let mut e = 1u128;
            let mut base = alpha as u128;
            let mut k = d;
            while k > 0 {
                if k & 1 == 1 {
                    e = e * base % n as u128;
                }
                base = base * base % n as u128;
                k >>= 1;
            }
            let ad = scalar_mul_plain(&curve, &mut o, e as u64, g0);
            (
                cheon_pminus1(
                    &curve,
                    n,
                    d,
                    g0,
                    q,
                    ad,
                    arm == "cheon.pminus1_comb",
                    deadline,
                ),
                2u64,
            )
        }
        _ => {
            let pows = leaked_powers(&curve, n, g0, alpha, 2 * d);
            (cheon_pplus1(&curve, n, d, &pows, deadline), 2 * d)
        }
    };
    (outcome, leaked, alpha, w.workload_id)
}

fn arms_for(family: &str) -> Vec<&'static str> {
    if family == "minus" {
        vec!["cheon.pminus1_comb", "cheon.pminus1_naive"]
    } else {
        vec!["cheon.pplus1_comb"]
    }
}

fn run(args: &[String]) {
    let curves = load_curves(&arg(args, "--curves").expect("--curves"));
    let out = arg(args, "--out").expect("--out");
    let targets: u64 = arg(args, "--targets").map_or(6, |s| s.parse().unwrap());
    let timeout = Duration::from_secs(arg(args, "--timeout").map_or(1800, |s| s.parse().unwrap()));
    let fam = arg(args, "--family");
    let bits: Option<Vec<u64>> =
        arg(args, "--bits").map(|s| s.split(',').map(|b| b.parse().unwrap()).collect());
    let arms_only = arg(args, "--arms");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&out)
        .unwrap();
    let mut skip: std::collections::HashSet<(String, u64, String)> = Default::default();
    for c in &curves {
        let family = c["family"].as_str().unwrap().to_string();
        let b = c["bits"].as_u64().unwrap();
        if fam.as_deref().is_some_and(|f| f != family) {
            continue;
        }
        if bits.as_ref().is_some_and(|l| !l.contains(&b)) {
            continue;
        }
        let (n, d) = (u(c, "n"), u(c, "d"));
        for t in 0..targets {
            for arm in arms_for(&family) {
                if arms_only
                    .as_deref()
                    .is_some_and(|s| !s.split(',').any(|x| x == arm))
                {
                    continue;
                }
                if skip.contains(&(family.clone(), b, arm.to_string())) {
                    continue;
                }
                let t0 = Instant::now();
                let (outcome, leaked, alpha, wid) = execute(c, t, arm, Some(t0 + timeout));
                let wall = t0.elapsed();
                let timed_out = outcome
                    .error
                    .as_deref()
                    .is_some_and(|e| e.contains("deadline"))
                    && wall >= timeout;
                let correct = outcome.alpha == Some(alpha);
                let row = json!({
                    "family": family, "bits": b, "target": t, "workload_id": wid,
                    "arm": arm, "n": n, "d": d, "leaked_powers": leaked,
                    "ops": outcome.ops, "adds": outcome.adds, "doubles": outcome.doubles,
                    "exponentiations": outcome.exponentiations, "table_points": outcome.table_points,
                    "phases": outcome.phases, "uncharged": outcome.uncharged,
                    "correct": correct, "recovered": outcome.alpha, "planted": alpha,
                    "error": outcome.error, "timed_out": timed_out,
                    "wall_ms": wall.as_millis() as u64, "peak_rss_kb": rss_hwm_kb(),
                });
                writeln!(file, "{row}").unwrap();
                file.flush().unwrap();
                eprintln!(
                    "{family} {b}b t{t} {arm}: ops={} correct={correct} wall={:.1}s rss={}MB{}",
                    outcome.ops,
                    wall.as_secs_f64(),
                    rss_hwm_kb() / 1024,
                    if timed_out { " TIMEOUT" } else { "" }
                );
                if timed_out {
                    skip.insert((family.clone(), b, arm.to_string()));
                }
            }
        }
    }
}

/// Independent re-execution of recorded runs in a fresh process: every
/// count must reproduce exactly and the planted logarithm must come back.
fn replay(args: &[String]) {
    let curves = load_curves(&arg(args, "--curves").expect("--curves"));
    let runs = std::fs::read_to_string(arg(args, "--runs").expect("--runs")).unwrap();
    let out = arg(args, "--out").expect("--out");
    let max_bits: u64 = arg(args, "--full-up-to").map_or(48, |s| s.parse().unwrap());
    let mut results = vec![];
    let mut all_ok = true;
    for line in runs.lines().filter(|l| !l.trim().is_empty()) {
        let r: Value = serde_json::from_str(line).unwrap();
        let bits = r["bits"].as_u64().unwrap();
        let t = r["target"].as_u64().unwrap();
        // every target up to --full-up-to bits, target 0 above
        if bits > max_bits && t != 0 {
            continue;
        }
        if !r["correct"].as_bool().unwrap_or(false) {
            continue;
        }
        let fam = r["family"].as_str().unwrap();
        let c = curves
            .iter()
            .find(|c| c["family"] == fam && c["bits"].as_u64() == Some(bits))
            .unwrap();
        let (o, _, alpha, wid) = execute(c, t, r["arm"].as_str().unwrap(), None);
        let same = o.ops == r["ops"].as_u64().unwrap()
            && o.adds == r["adds"].as_u64().unwrap()
            && o.doubles == r["doubles"].as_u64().unwrap()
            && o.alpha == Some(alpha)
            && Some(alpha) == r["planted"].as_u64()
            && wid == r["workload_id"].as_str().unwrap();
        all_ok &= same;
        eprintln!(
            "replay {fam} {bits}b t{t} {}: {}",
            r["arm"],
            if same { "identical" } else { "DIFFERENT" }
        );
        results.push(
            json!({"family": fam, "bits": bits, "target": t, "arm": r["arm"],
            "workload_id": wid, "ops_recorded": r["ops"], "ops_replayed": o.ops, "identical": same,
            "scalar_verified": o.alpha == Some(alpha)}),
        );
    }
    let doc = json!({
        "schema": "cheon-p224-shape.replay/v1",
        "status": if all_ok && !results.is_empty() { "verified" } else { "failed" },
        "independent": true,
        "independence": "fresh process re-executing the recorded (curve, target, arm) from curves.json and the ecbench target law; counts compared to the recorded run",
        "scalars_verified": all_ok,
        "scope": format!("every recorded run up to {max_bits} bits, target 0 above"),
        "runs": results,
    });
    std::fs::write(&out, serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap();
    if !all_ok {
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("gen") => gen(&args),
        Some("check") => check(&args),
        Some("spec") => spec(&args),
        Some("run") => run(&args),
        Some("replay") => replay(&args),
        _ => {
            eprintln!("usage: cheon_p224_shape gen|check|spec|run|xcheck|analyse ...");
            std::process::exit(2);
        }
    }
}
