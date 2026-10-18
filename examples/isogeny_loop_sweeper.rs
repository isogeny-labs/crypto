//! Native isogeny-loop sweeper (port of `scripts/isogeny_loop_sweeper.py`,
//! `curve` mode on frozen curve records), with the class group computed by
//! `crypto_lib::isogeny::class_group_subexp` instead of PARI's `bnfinit`.
//!
//! ```text
//! cargo run --release --example isogeny_loop_sweeper -- curve \
//!     [--groups FILE ...] [--only NAME ...] [--cost-model projective|affine|xonly] \
//!     [--doubling-cost 8] [--prime-bound 100] [--radius-factor 1] [--top 6] \
//!     [--max-disc-bits 140] [--fb-factor 1] [--seed N] [--node-cap N] \
//!     [--certify-minimum] [--certify-node-cap N] [--json OUT]
//! cargo run --release --example isogeny_loop_sweeper -- disc D [D ...]   # class group only
//! ```
//!
//! For each record `results[]` of the frozen JSON (fields `curve.{name, family,
//! q, trace, r, n}` and `frobenius.{DK, conductor_f}`) it re-verifies
//! `t² − 4q = f² D_K`, computes `Cl(O_K)` (GRH), the relation lattice of the
//! split primes `ℓ ≤ L`, the cheapest loops under the chosen cost model, their
//! generators `α = (A + B√D_K)/2` (verified), and the eigenvalue
//! `λ = (A + B s)/2 mod r` with `s = (2 − t) f⁻¹ mod r` (so that
//! `π = (t + f√D_K)/2` acts as 1).  In characteristic 2 the prime above 2 is
//! oriented so that `+1` is the prime containing `π` (Frobenius `F`, 2 M) and
//! `−1` its conjugate (Verschiebung `V`, `c_dbl + 2` M).

use crypto_lib::isogeny::class_group_subexp::{compute_class_group, SubexpParams};
use crypto_lib::isogeny::loop_lattice::{
    in_ideal, loop_generator, loop_primes, weighted_shortest, CostModel, LoopPrime,
};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use serde_json::{json, Value};
use std::str::FromStr;
use std::time::Instant;

const DEFAULT_GROUPS: [&str; 4] = [
    "research/isogeny_loop_glv_20261010/group_a_koblitz_and_known.json",
    "research/isogeny_loop_glv_20261010/group_b_binary131.json",
    "research/isogeny_loop_glv_20261010/group_c_certicom_mid.json",
    "research/isogeny_loop_glv_20261010/group_d_certicom_small.json",
];

struct Args {
    mode: String,
    positional: Vec<String>,
    groups: Vec<String>,
    only: Vec<String>,
    models: Vec<CostModel>,
    doubling_cost: f64,
    scan_from: i64,
    scan_to: i64,
    bits: u64,
    char2: bool,
    frozen_scan: Option<String>,
    prime_bound: u64,
    radius_factor: f64,
    top: usize,
    max_disc_bits: u64,
    fb_factor: f64,
    markowitz: usize,
    seed: u64,
    node_cap: u64,
    certify: bool,
    certify_node_cap: u64,
    json: Option<String>,
    verbose: bool,
}

fn parse_args() -> Args {
    let mut it = std::env::args().skip(1);
    let mut a = Args {
        mode: it.next().unwrap_or_else(|| "curve".into()),
        positional: Vec::new(),
        groups: Vec::new(),
        only: Vec::new(),
        models: Vec::new(),
        doubling_cost: 8.0,
        scan_from: 3,
        scan_to: 3000,
        bits: 128,
        char2: false,
        frozen_scan: None,
        prime_bound: 100,
        radius_factor: 1.0,
        top: 6,
        max_disc_bits: 140,
        fb_factor: 1.0,
        markowitz: 20_000,
        seed: 0x5eed_c1a5_5600_0001,
        node_cap: 2_000_000_000,
        certify: false,
        certify_node_cap: 20_000_000_000,
        json: None,
        verbose: false,
    };
    while let Some(x) = it.next() {
        let mut val = || it.next().unwrap_or_else(|| panic!("{x} needs a value"));
        match x.as_str() {
            "--groups" => a.groups.push(val()),
            "--only" => a.only.push(val()),
            "--cost-model" => {
                for m in val().split(',') {
                    a.models
                        .push(CostModel::parse(m.trim()).expect("cost model: projective|affine|xonly"));
                }
            }
            "--scan-from" => a.scan_from = val().parse().unwrap(),
            "--scan-to" => a.scan_to = val().parse().unwrap(),
            "--bits" => a.bits = val().parse().unwrap(),
            "--char2" => a.char2 = true,
            "--frozen-scan" => a.frozen_scan = Some(val()),
            "--doubling-cost" => a.doubling_cost = val().parse().unwrap(),
            "--prime-bound" => a.prime_bound = val().parse().unwrap(),
            "--radius-factor" => a.radius_factor = val().parse().unwrap(),
            "--top" => a.top = val().parse().unwrap(),
            "--max-disc-bits" => a.max_disc_bits = val().parse().unwrap(),
            "--fb-factor" => a.fb_factor = val().parse().unwrap(),
            "--markowitz" => a.markowitz = val().parse().unwrap(),
            "--seed" => a.seed = val().parse().unwrap(),
            "--node-cap" => a.node_cap = val().parse().unwrap(),
            "--certify-minimum" => a.certify = true,
            "--certify-node-cap" => a.certify_node_cap = val().parse().unwrap(),
            "--json" => a.json = Some(val()),
            "--verbose" => a.verbose = true,
            _ => a.positional.push(x.clone()),
        }
    }
    if a.groups.is_empty() {
        a.groups = DEFAULT_GROUPS.iter().map(|s| s.to_string()).collect();
    }
    if a.models.is_empty() {
        a.models.push(CostModel::Projective);
    }
    a
}

fn model_names(args: &Args) -> String {
    args.models.iter().map(|m| m.name()).collect::<Vec<_>>().join(",")
}

fn big(v: &Value) -> BigInt {
    match v {
        Value::String(s) => BigInt::from_str(s).expect("integer string"),
        Value::Number(n) => BigInt::from_str(&n.to_string()).expect("integer"),
        _ => panic!("not an integer: {v}"),
    }
}

fn modinv(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let e = a.mod_floor(m).extended_gcd(m);
    if e.gcd.is_one() {
        Some(e.x.mod_floor(m))
    } else {
        None
    }
}

fn half(x: u64) -> String {
    if x % 2 == 0 {
        format!("{}.0", x / 2)
    } else {
        format!("{}.5", x / 2)
    }
}

fn half_f(x: u64) -> f64 {
    x as f64 / 2.0
}

fn exps_map(primes: &[LoopPrime], e: &[i64]) -> Value {
    let mut m = serde_json::Map::new();
    for (p, &x) in primes.iter().zip(e) {
        if x != 0 {
            m.insert(p.ell.to_string(), json!(x));
        }
    }
    Value::Object(m)
}

fn exps_str(primes: &[LoopPrime], e: &[i64]) -> String {
    let parts: Vec<String> = primes
        .iter()
        .zip(e)
        .filter(|(_, &x)| x != 0)
        .map(|(p, x)| format!("{}: {}", p.ell, x))
        .collect();
    format!("{{{}}}", parts.join(", "))
}

fn short(s: &str) -> String {
    if s.len() > 24 {
        format!("{}…({} digits)", &s[..12], s.trim_start_matches('-').len())
    } else {
        s.to_string()
    }
}

fn frobenius_power(char2: bool, n: u64, primes: &[LoopPrime], e: &[i64]) -> Option<i64> {
    if !char2 {
        return None;
    }
    let nz: Vec<(u64, i64)> = primes
        .iter()
        .zip(e)
        .filter(|(_, &x)| x != 0)
        .map(|(p, &x)| (p.ell, x))
        .collect();
    if nz.len() == 1 && nz[0].0 == 2 && nz[0].1 % n as i64 == 0 {
        Some(nz[0].1 / n as i64)
    } else {
        None
    }
}

fn analyse(rec: &Value, args: &Args, md: &mut Vec<String>) -> Value {
    let t_curve = Instant::now();
    let cv = &rec["curve"];
    let name = cv["name"].as_str().unwrap_or("?").to_string();
    let family = cv["family"].as_str().unwrap_or("?").to_string();
    let char2 = family == "binary" || family == "koblitz";
    let q = big(&cv["q"]);
    let t = big(&cv["trace"]);
    let r = big(&cv["r"]);
    let n_ext: u64 = cv["n"].as_u64().unwrap_or(1);
    let mut out = json!({"curve": {"name": name, "family": family, "q": q.to_string(),
        "trace": t.to_string(), "r": r.to_string(), "n": n_ext, "char2": char2}});
    md.push(format!("### {name}"));
    md.push(String::new());
    let Some(fr) = rec.get("frobenius").filter(|v| !v.is_null()) else {
        md.push("- no frozen Frobenius record; skipped".into());
        out["error"] = json!("no frobenius record");
        return out;
    };
    let dk = big(&fr["DK"]);
    let f = big(&fr["conductor_f"]);
    let delta: BigInt = &t * &t - &q * 4;
    let ok = delta == &f * &f * &dk;
    out["frobenius"] = json!({"delta": delta.to_string(), "DK": dk.to_string(),
        "DK_bits": dk.abs().bits(), "conductor_f": f.to_string(), "delta_eq_f2_DK": ok});
    md.push(format!(
        "- t² − 4q = f²·D_K re-verified: {ok}; D_K = {dk} ({} bits), f = {}",
        dk.abs().bits(),
        short(&f.to_string())
    ));
    if !ok {
        out["error"] = json!("t^2 - 4q != f^2 DK");
        return out;
    }
    let bits_r = r.bits();
    let lprime = bits_r.div_ceil(2);
    let dbl_half = (args.doubling_cost * 2.0).round() as u64;
    let saving_half = dbl_half * lprime;
    out["budget"] = json!({"log2_r": bits_r, "half_bits": lprime,
        "doublings_saved_cost_M": half_f(saving_half), "cost_models": model_names(args),
        "doubling_cost_M": args.doubling_cost});
    // frozen evidence
    let frozen = rec["orders"].get(0).cloned().unwrap_or(Value::Null);
    let frozen_h = frozen["class_group"]["h"].as_str().map(|s| s.to_string());
    let frozen_cyc: Option<Vec<String>> = frozen["class_group"]["cyc_K"]
        .as_array()
        .map(|a| a.iter().map(|x| x.as_str().unwrap_or("").to_string()).collect());
    let frozen_cost = frozen["loops"].get(0).and_then(|l| l["cost_M"].as_f64());
    if dk.abs().bits() > args.max_disc_bits {
        md.push(format!(
            "- skipped: |D_K| has {} bits > --max-disc-bits {}",
            dk.abs().bits(),
            args.max_disc_bits
        ));
        out["skipped"] = json!(format!("|DK| > {} bits", args.max_disc_bits));
        return out;
    }
    let params = SubexpParams {
        fb_factor: args.fb_factor,
        markowitz_limit: args.markowitz,
        min_fb_bound: args.prime_bound,
        seed: args.seed,
        verbose: args.verbose,
        ..Default::default()
    };
    eprintln!("[{name}] class group of D_K ({} bits) ...", dk.abs().bits());
    let cg = match compute_class_group(&dk, &params) {
        Ok(c) => c,
        Err(e) => {
            md.push(format!("- class group FAILED: {e}"));
            out["error"] = json!(format!("class group: {e}"));
            return out;
        }
    };
    let cyc: Vec<String> = cg.invariants.iter().map(|x| x.to_string()).collect();
    let h_match = frozen_h.as_ref().map(|h| *h == cg.h.to_string());
    let cyc_match = frozen_cyc.as_ref().map(|c| *c == cyc);
    eprintln!(
        "[{name}] h = {} cyc {:?} in {:.1}s (frozen h {:?})",
        cg.h, cyc, cg.seconds, frozen_h
    );
    out["class_group"] = json!({
        "h": cg.h.to_string(), "cyc_K": cyc, "seconds": cg.seconds,
        "seconds_relations": cg.seconds_relations, "seconds_linalg": cg.seconds_linalg,
        "fb_bound": cg.fb_bound, "bach_bound": cg.bach_bound, "fb_size": cg.fb.len(),
        "relations_used": cg.relations_used, "full_relations_found": cg.full_relations_found,
        "combined_large_prime_relations": cg.combined_large_prime_relations,
        "trials": cg.trials, "dense_dim": cg.dense_dim, "eliminated": cg.eliminated,
        "h_history": cg.h_history.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
        "euler_bound": cg.euler_bound, "log_h_approx": cg.log_h_approx,
        "log_h_over_h_approx": cg.log_ratio, "analytic_ok": cg.analytic_ok,
        "heldout_checked": cg.heldout_checked, "heldout_ok": cg.heldout_ok,
        "assumption": "GRH (Bach 6 log^2|D| generation bound); analytic tolerance |log(h/h_approx)| < log sqrt 2 (not a proven bound)",
        "frozen_pari_h": frozen_h, "frozen_pari_cyc_K": frozen_cyc,
        "h_matches_frozen": h_match, "cyc_matches_frozen": cyc_match,
    });
    md.push(format!(
        "- Cl(O_K): h = {}, cyc = {:?}; {:.2} s ({} relations, FB bound {} = Bach·{}, dense block {}), log(h/h_approx) = {:.4} (Euler product to {}), held-out relations {}/{} ok",
        cg.h, cyc, cg.seconds, cg.relations_used, cg.fb_bound, args.fb_factor, cg.dense_dim,
        cg.log_ratio, cg.euler_bound, if cg.heldout_ok { cg.heldout_checked } else { 0 }, cg.heldout_checked
    ));
    md.push(format!(
        "- frozen PARI: h = {}, cyc = {:?} → h match {:?}, cyc match {:?}",
        frozen_h.clone().unwrap_or_default(),
        frozen_cyc.clone().unwrap_or_default(),
        h_match,
        cyc_match
    ));
    // eigenvalue bookkeeping
    let s = if (&f).mod_floor(&r).is_zero() {
        None
    } else {
        modinv(&f, &r).map(|fi| ((BigInt::from(2) - &t) * fi).mod_floor(&r))
    };
    let s = s.filter(|s| (s * s - &dk).mod_floor(&r).is_zero());
    out["sqrtDK_mod_r"] = json!(s.as_ref().map(|x| x.to_string()));
    let mut models_out = serde_json::Map::new();
    for &model in &args.models {
        let mut om = json!({});
        if args.models.len() > 1 {
            md.push(String::new());
            md.push(format!("#### cost model {}", model.name()));
            md.push(String::new());
        }
    // loop primes; char 2: orient l = 2 so that +1 is the prime containing pi
    let two = BigInt::from(2);
    let primes = loop_primes(&cg, args.prime_bound, char2, model, dbl_half, |fp| {
        char2 && fp.p == 2 && !in_ideal(&t, &f, &fp.b, &two)
    });
    if char2 {
        if let Some(p2) = primes.iter().find(|p| p.ell == 2) {
            assert!(in_ideal(&t, &f, &p2.b, &two), "pi not in the oriented prime above 2");
        }
    }
    if primes.is_empty() {
        om["error"] = json!("no split primes below the bound");
        {
            models_out.insert(model.name().to_string(), om);
            continue;
        }
    }
    let radius_cap = (args.radius_factor * saving_half as f64).floor() as u64;
    let ts = Instant::now();
    let mut res = weighted_shortest(&primes, &cg.invariants, Some(radius_cap), args.node_cap);
    let mut certified = None;
    if args.certify && res.lll_best > res.radius {
        let tc = Instant::now();
        let r2 = weighted_shortest(&primes, &cg.invariants, None, args.certify_node_cap);
        certified = Some(json!({"radius_M": half_f(r2.radius), "exhaustive": r2.exhaustive,
            "nodes": r2.nodes, "seconds": tc.elapsed().as_secs_f64(),
            "minimum_M": r2.ranked.first().map(|x| half_f(x.0))}));
        if r2.exhaustive {
            // merge: everything found is valid; the minimum is now certified
            res.ranked = r2.ranked;
        }
    }
    let search_s = ts.elapsed().as_secs_f64();
    let within = res.ranked.iter().filter(|x| x.0 <= res.radius).count();
    om["relation_search"] = json!({"rank": primes.len(), "exhaustive_within_radius": res.exhaustive,
        "radius_M": half_f(res.radius), "lll_upper_bound_M": half_f(res.lll_best),
        "loops_within_radius": within, "candidates": res.ranked.len(), "nodes": res.nodes,
        "seconds": search_s, "certify_minimum": certified});
    om["primes"] = Value::Array(
        primes
            .iter()
            .map(|p| {
                json!({"ell": p.ell, "cost_forward_M": half_f(p.wf), "cost_backward_M": half_f(p.wb),
                "b": p.b.to_string(), "dlog": p.dlog.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                "order_in_ClK": cg.order_of(p.fb_index).to_string()})
            })
            .collect(),
    );
    let mut loops = Vec::new();
    md.push(format!(
        "- split primes ℓ ≤ {}: {} ; cost model {} (c_dbl = {} M), GLV saving {} M; radius {} M, LLL upper bound {} M, exhaustive within radius: {}, {} loops within radius",
        args.prime_bound,
        primes.iter().map(|p| p.ell.to_string()).collect::<Vec<_>>().join(","),
        model.name(), args.doubling_cost, half(saving_half), half(res.radius),
        half(res.lll_best), res.exhaustive, within
    ));
    md.push(String::new());
    md.push("| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |".into());
    md.push("|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|".into());
    for (rank, (c, e)) in res.ranked.iter().take(args.top).enumerate() {
        let g = match loop_generator(&dk, &primes, e) {
            Ok(g) => g,
            Err(err) => {
                loops.push(json!({"cost_M": half_f(*c), "exponents": exps_map(&primes, e), "error": err}));
                continue;
            }
        };
        let fpow = frobenius_power(char2, n_ext, &primes, e);
        let (a2, b2) = (&g.a, &g.b);
        let in_zpi = b2.mod_floor(&f).is_zero() && {
            let d = b2 / &f;
            (a2 - &d * &t).mod_floor(&two).is_zero()
        };
        let acts = if in_zpi {
            let d = b2 / &f;
            Some((a2 - &d * &t) / 2 + &d)
        } else {
            None
        };
        let lam = s.as_ref().map(|s| {
            let inv2 = modinv(&two, &r).unwrap();
            ((a2 + b2 * s) * &inv2).mod_floor(&r)
        });
        let mut lam_ok = None;
        if let Some(l) = &lam {
            let tr = a2.clone();
            let chk = (l * l - &tr * l + &g.norm).mod_floor(&r).is_zero();
            let int_ok = acts.as_ref().is_none_or(|x: &BigInt| (x - l).mod_floor(&r).is_zero());
            lam_ok = Some(chk && int_ok);
        }
        let chain: u64 = e.iter().map(|x| x.unsigned_abs()).sum();
        let nb = g.norm.bits();
        let alpha = format!("({} + {}*sqrt({}))/2", a2, b2, dk);
        loops.push(json!({
            "cost_M": half_f(*c), "ratio_to_saving": half_f(*c) / half_f(saving_half),
            "exponents": exps_map(&primes, e), "chain_length": chain,
            "degree": g.norm.to_string(), "degree_bits": nb,
            "alpha": alpha, "alpha_A": a2.to_string(), "alpha_B": b2.to_string(),
            "trace": a2.to_string(), "in_Z_pi": in_zpi, "Z_pi_is_OK": f.is_one(),
            "acts_as_integer": acts.as_ref().map(|x| x.to_string()),
            "frobenius_power": fpow, "lambda": lam.as_ref().map(|x| x.to_string()),
            "lambda_checks": lam_ok, "lambda_is_pm1": lam.as_ref().map(|l| l.is_one() || *l == &r - 1u32),
            "within_radius": *c <= res.radius, "generator_verified": true,
        }));
        let fp = match fpow {
            Some(m) if m > 0 => format!("π^{m}"),
            Some(m) => format!("π̄^{}", -m),
            None => "—".into(),
        };
        let zp = if f.is_one() {
            "Z[π] = O_K".to_string()
        } else if in_zpi {
            format!("yes ({})", short(&acts.as_ref().unwrap().to_string()))
        } else {
            "no".into()
        };
        md.push(format!(
            "| {} | {} | {:.3} | {} | {} | {} | ({} + {}√D)/2 | {} | {} | {} |",
            rank + 1,
            half(*c),
            half_f(*c) / half_f(saving_half),
            exps_str(&primes, e),
            chain,
            nb,
            short(&a2.to_string()),
            short(&b2.to_string()),
            fp,
            zp,
            lam.as_ref().map(|l| short(&l.to_string())).unwrap_or_default()
        ));
    }
    om["loops"] = Value::Array(loops);
    let genuine = res
        .ranked
        .iter()
        .find(|(_, e)| frobenius_power(char2, n_ext, &primes, e).is_none())
        .map(|(c, e)| {
            json!({"cost_M": half_f(*c), "ratio_to_saving": half_f(*c) / half_f(saving_half),
            "exponents": exps_map(&primes, e), "chain_length": e.iter().map(|x| x.unsigned_abs()).sum::<u64>(),
            "within_radius": *c <= res.radius})
        });
    om["cheapest_non_frobenius_loop"] = genuine.clone().unwrap_or(Value::Null);
    let cheapest = res.ranked.first().map(|x| half_f(x.0));
    // the frozen (round 1, PARI) costs are projective-model costs
    let cost_match = match (frozen_cost, cheapest) {
        (Some(a), Some(b)) if model == CostModel::Projective => Some((a - b).abs() < 1e-9),
        _ => None,
    };
    om["frozen_comparison"] = json!({"frozen_loops0_cost_M": frozen_cost, "native_loops0_cost_M": cheapest,
        "cost_equal": cost_match, "h_equal": h_match, "cyc_equal": cyc_match});
    md.push(String::new());
    if let Some(g) = &genuine {
        md.push(format!(
            "- cheapest loop that is not a Frobenius/Verschiebung power: {} M ({:.3}× saving), {}, chain {}{}",
            g["cost_M"], g["ratio_to_saving"].as_f64().unwrap_or(0.0), g["exponents"], g["chain_length"],
            if g["within_radius"].as_bool() == Some(true) { "" } else { " — above the radius, from the LLL basis (upper bound)" }
        ));
    }
    if model == CostModel::Projective {
        md.push(format!(
            "- frozen PARI loops[0].cost_M = {:?}, native = {:?} → equal: {:?}",
            frozen_cost, cheapest, cost_match
        ));
    }
        models_out.insert(model.name().to_string(), om);
    }
    // the first model's results also sit at the top level (round-1 layout)
    if let Some(Value::Object(first)) = models_out.get(args.models[0].name()) {
        for (k, v) in first {
            out[k.as_str()] = v.clone();
        }
    }
    out["models"] = Value::Object(models_out);
    let secs = t_curve.elapsed().as_secs_f64();
    out["seconds"] = json!(secs);
    md.push(format!("- wall time {:.2} s", secs));
    md.push(String::new());
    out
}

fn main() {
    let args = parse_args();
    let t0 = Instant::now();
    let start = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string();
    if args.mode == "disc" {
        let mut results = Vec::new();
        for d in &args.positional {
            let dk = BigInt::from_str(d).expect("discriminant");
            let params = SubexpParams {
                fb_factor: args.fb_factor,
                markowitz_limit: args.markowitz,
                min_fb_bound: args.prime_bound,
                seed: args.seed,
                verbose: args.verbose,
                ..Default::default()
            };
            match compute_class_group(&dk, &params) {
                Ok(cg) => {
                    println!(
                        "D = {} ({} bits): h = {} cyc {:?} in {:.2} s (rel {:.2} s, linalg {:.2} s), FB {} primes ≤ {}, relations {}, dense {}, log(h/h_approx) {:.4}, held-out ok {}",
                        dk, dk.abs().bits(), cg.h,
                        cg.invariants.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                        cg.seconds, cg.seconds_relations, cg.seconds_linalg, cg.fb.len(), cg.fb_bound,
                        cg.relations_used, cg.dense_dim, cg.log_ratio, cg.heldout_ok
                    );
                    results.push(json!({"D": dk.to_string(), "h": cg.h.to_string(),
                        "cyc": cg.invariants.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                        "seconds": cg.seconds, "fb_bound": cg.fb_bound, "relations": cg.relations_used,
                        "log_ratio": cg.log_ratio}));
                }
                Err(e) => println!("D = {dk}: FAILED {e}"),
            }
        }
        if let Some(p) = &args.json {
            std::fs::write(p, serde_json::to_string_pretty(&json!({"mode": "disc", "results": results})).unwrap())
                .unwrap();
        }
        return;
    }
    if args.mode == "scan" {
        scan(&args, &start, t0);
        return;
    }
    let mut md = Vec::new();
    md.push(format!(
        "## Native isogeny-loop sweep — cost model(s) {}, doubling {} M, primes ℓ ≤ {}, radius factor {}",
        model_names(&args),
        args.doubling_cost,
        args.prime_bound,
        args.radius_factor
    ));
    md.push(String::new());
    md.push(format!("start (UTC): {start}"));
    md.push(String::new());
    let mut results = Vec::new();
    for g in &args.groups {
        let txt = std::fs::read_to_string(g).unwrap_or_else(|e| panic!("{g}: {e}"));
        let v: Value = serde_json::from_str(&txt).expect("json");
        for rec in v["results"].as_array().cloned().unwrap_or_default() {
            let name = rec["curve"]["name"].as_str().unwrap_or("").to_string();
            if !args.only.is_empty() && !args.only.contains(&name) {
                continue;
            }
            let mut out = analyse(&rec, &args, &mut md);
            out["source"] = json!(g);
            // flush per curve so a crash keeps finished results
            println!("{}", md.join("\n"));
            md.clear();
            results.push(out);
            if let Some(p) = &args.json {
                let payload = json!({"mode": "curve", "start_utc": start,
                    "cost_models": model_names(&args), "doubling_cost": args.doubling_cost,
                    "prime_bound": args.prime_bound, "radius_factor": args.radius_factor,
                    "fb_factor": args.fb_factor, "seed": args.seed, "node_cap": args.node_cap,
                    "certify_minimum": args.certify,
                    "seconds": t0.elapsed().as_secs_f64(), "results": results});
                std::fs::write(p, serde_json::to_string_pretty(&payload).unwrap()).unwrap();
            }
        }
    }
    eprintln!("total {:.1} s", t0.elapsed().as_secs_f64());
}

fn is_squarefree(mut m: i64) -> bool {
    let mut p = 2i64;
    while p * p <= m {
        if m % (p * p) == 0 {
            return false;
        }
        while m % p == 0 {
            m /= p;
        }
        p += 1;
    }
    true
}

/// Fundamental discriminant test for `D = -m < 0`.
fn is_fundamental_neg(m: i64) -> bool {
    if m <= 0 {
        return false;
    }
    if m % 4 == 3 {
        return is_squarefree(m);
    }
    if m % 4 == 0 {
        let k = m / 4;
        return (k % 4 == 1 || k % 4 == 2) && is_squarefree(k);
    }
    false
}

/// `scan` mode: every fundamental `D = -m`, `scan_from ≤ m ≤ scan_to`, as a
/// synthetic record (`t² − 4q = D`, `f = 1`, `r` of `--bits` bits; no curve),
/// through the same class-group and loop search as `curve` mode, one row per
/// discriminant with the cheapest loop under every cost model.  With
/// `--char2` only `D ≡ 1 (mod 8)` (2 splits) is scanned, the family is
/// binary, and the 2-steps are priced F = 2 M / V = c_dbl + 2 M.
fn scan(args: &Args, start: &str, t0: Instant) {
    let frozen: Option<Value> = args.frozen_scan.as_ref().map(|p| {
        serde_json::from_str(&std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{p}: {e}")))
            .expect("frozen scan json")
    });
    let frozen_rows: std::collections::HashMap<i64, Value> = frozen
        .as_ref()
        .and_then(|v| v["rows"].as_array().cloned())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|r| r["DK"].as_i64().map(|d| (d, r)))
        .collect();
    let r_fake: BigInt = (BigInt::one() << args.bits) - 1;
    println!(
        "## Native isogeny-loop scan — fundamental D in [-{}, -{}], cost model(s) {}, doubling {} M, primes ℓ ≤ {}, saving at {} bits{}",
        args.scan_to, args.scan_from, model_names(args), args.doubling_cost, args.prime_bound, args.bits,
        if args.char2 { ", characteristic 2 (D ≡ 1 mod 8 only; F = 2 M, V = c_dbl + 2 M)" } else { "" }
    );
    println!();
    println!("start (UTC): {start}");
    println!();
    let mut head = String::from("| D_K | h | Cl | ");
    let mut sep = String::from("|--:|--:|:--|");
    for m in &args.models {
        head.push_str(&format!("{} cost M | ratio | loop | degree | ", m.name()));
        sep.push_str("--:|--:|:--|--:|");
    }
    if frozen.is_some() {
        head.push_str("round-1 projective cost M | match |");
        sep.push_str("--:|:--|");
    }
    println!("{head}");
    println!("{sep}");
    let mut rows = Vec::new();
    let mut summary: Vec<(usize, usize, f64, i64)> = vec![(0, 0, f64::INFINITY, 0); args.models.len()];
    let (mut matched, mut compared) = (0usize, 0usize);
    let mut scanned = 0usize;
    for m in args.scan_from..=args.scan_to {
        if !is_fundamental_neg(m) {
            continue;
        }
        let d = -m;
        if args.char2 && d.rem_euclid(8) != 1 {
            continue;
        }
        scanned += 1;
        let t: i64 = if d.rem_euclid(2) == 0 { 0 } else { 1 };
        let q: BigInt = (BigInt::from(t) * t - d) / 4i64;
        let rec = json!({
            "curve": {"name": format!("D={d}"), "family": if args.char2 { "binary" } else { "prime" },
                "q": q.to_string(), "trace": t.to_string(), "r": r_fake.to_string(), "n": 1},
            "frobenius": {"DK": d.to_string(), "conductor_f": "1"},
        });
        let mut scratch = Vec::new();
        let out = analyse(&rec, args, &mut scratch);
        let h = out["class_group"]["h"].as_str().unwrap_or("?").to_string();
        let cyc = out["class_group"]["cyc_K"].clone();
        let mut line = format!("| {d} | {h} | {} | ", cyc);
        let mut row = json!({"DK": d, "h": h, "cyc": cyc, "models": {}});
        for (mi, model) in args.models.iter().enumerate() {
            let mo = &out["models"][model.name()];
            let l0 = &mo["loops"][0];
            let cost = l0["cost_M"].as_f64();
            let ratio = l0["ratio_to_saving"].as_f64();
            match (cost, ratio) {
                (Some(c), Some(r)) => {
                    line.push_str(&format!("{c} | {r:.3} | {} | {} | ", l0["exponents"], l0["degree"]));
                    let e = &mut summary[mi];
                    if r < 1.0 {
                        e.0 += 1;
                    }
                    if r < 0.5 {
                        e.1 += 1;
                    }
                    if r < e.2 {
                        e.2 = r;
                        e.3 = d;
                    }
                }
                _ => line.push_str(&format!("{} | | | | ", mo["error"].as_str().unwrap_or("—"))),
            }
            row["models"][model.name()] = json!({"cheapest": l0.clone(),
                "relation_search": mo["relation_search"].clone(),
                "cheapest_non_frobenius_loop": mo["cheapest_non_frobenius_loop"].clone()});
        }
        if frozen.is_some() {
            let fz = frozen_rows.get(&d).and_then(|r| r["best_cost_M"].as_f64());
            let native = out["models"]["projective"]["loops"][0]["cost_M"].as_f64();
            let eq = match (fz, native) {
                (Some(a), Some(b)) => {
                    compared += 1;
                    let ok = (a - b).abs() < 1e-9;
                    if ok {
                        matched += 1;
                    }
                    Some(ok)
                }
                _ => None,
            };
            line.push_str(&format!("{} | {} |", fz.map(|x| x.to_string()).unwrap_or_default(),
                eq.map(|x| x.to_string()).unwrap_or_default()));
            row["round1_projective_cost_M"] = json!(fz);
            row["round1_match"] = json!(eq);
        }
        println!("{line}");
        rows.push(row);
    }
    println!();
    println!("scanned {scanned} fundamental discriminants in {:.1} s", t0.elapsed().as_secs_f64());
    for (mi, model) in args.models.iter().enumerate() {
        let e = &summary[mi];
        println!(
            "- {}: {} discriminants with cheapest loop below the saving, {} below half of it; minimum ratio {:.3} at D = {}",
            model.name(), e.0, e.1, e.2, e.3
        );
    }
    if frozen.is_some() {
        println!("- round-1 projective cheapest-loop costs reproduced: {matched} / {compared}");
    }
    if let Some(p) = &args.json {
        let payload = json!({"mode": "scan", "start_utc": start, "from": args.scan_from, "to": args.scan_to,
            "bits": args.bits, "char2": args.char2, "cost_models": model_names(args),
            "doubling_cost": args.doubling_cost, "prime_bound": args.prime_bound,
            "seconds": t0.elapsed().as_secs_f64(), "scanned": scanned,
            "round1_matched": matched, "round1_compared": compared, "rows": rows});
        std::fs::write(p, serde_json::to_string_pretty(&payload).unwrap()).unwrap();
    }
}
