//! `ecdlp-nist` — the ECDLP across every NIST curve.
//!
//! ```text
//! ecdlp-nist list                         # the fifteen curves with their audit
//! ecdlp-nist audit K-233                  # one curve's structural checks
//! ecdlp-nist solve P-256 --secret 123456789 --interval-bits 32
//! ecdlp-nist solve B-571 --random-bits 24 --method kangaroo --threads 4
//! ecdlp-nist solve toy-k23a1 --random-bits 21          # whole-group rho, Frobenius folded
//! ecdlp-nist solve toy-k23a1 --random-bits 21 --no-fold
//! ecdlp-nist smart --bits 256 --seed 7                 # construct an anomalous curve and break it
//! ecdlp-nist bench P-521                               # additions per second on this host
//! ```
//!
//! Every subcommand takes `--json` for a machine-readable report.  See
//! `docs/ECDLP_NIST.md`.

use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;

use clap::{Args, Parser, Subcommand};
use num_bigint::{BigUint, RandBigInt};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::json;

use crypto_lib::binary_ecc::curve::BinaryCurve;

use crypto_lib::cryptanalysis::ecdlp_nist::{
    self as nist, anomalous, audit, curve_by_name, curve_names, nist_curves, solve, Audit, Curve,
    EcdlpGroup, Group, Method, SolveOptions,
};

#[derive(Parser)]
#[command(
    name = "ecdlp-nist",
    version,
    about = "ECDLP solvers and structural audit across NIST and Certicom curves",
    long_about = "Solve Q = [k]G on the NIST P, K and B curves and Certicom prime/binary \
                  challenges by the \
                  branch the curve calls for (Smart on anomalous prime curves, \
                  Pohlig–Hellman on composite orders, BSGS/kangaroo on an interval, \
                  folded parallel rho otherwise), and audit each curve for the \
                  structural conditions those attacks need.  Bounded instances run \
                  on the real curves; whole-group searches are refused as infeasible \
                  unless forced, and no result here is a statement about deployed keys."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    /// Emit a machine-readable JSON report.
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Command {
    /// List the NIST and Certicom curves with their audit summary.
    List,
    /// Structural audit of one curve.
    Audit(CurveArg),
    /// Plant (or accept) a target point and solve for its scalar.
    Solve(SolveArgs),
    /// Construct an anomalous curve by CM and break it with Smart's attack.
    Smart(SmartArgs),
    /// Measure group additions per second on a curve.
    Bench(BenchArgs),
}

#[derive(Args)]
struct CurveArg {
    /// Curve name: P-256, K-163, B-571, secp256r1, toy-k23a1, anomalous-128-3, …
    curve: String,
}

#[derive(Args)]
struct SolveArgs {
    /// Curve name (see `list`).
    curve: String,
    /// Plant Q = [k]G with this scalar (decimal or 0x-hex).
    #[arg(long, conflicts_with_all = ["random_bits", "point", "point_file", "challenge"])]
    secret: Option<String>,
    /// Plant Q = [k]G with a random k of this many bits (seeded).
    #[arg(long, conflicts_with_all = ["point", "point_file", "challenge"])]
    random_bits: Option<u32>,
    /// Solve an externally supplied point `x,y` (hex).  Needs an interval
    /// unless the curve is a toy.
    #[arg(long, value_name = "X,Y", conflicts_with_all = ["point_file", "challenge"])]
    point: Option<String>,
    /// Checked JSON target artifact, local or store:NAME/sha256:DIGEST.
    #[arg(long, conflicts_with_all = ["secret", "random_bits", "point", "challenge"])]
    point_file: Option<std::path::PathBuf>,
    /// Use the published Certicom challenge point Q for this curve.
    #[arg(long)]
    challenge: bool,
    /// Interval lower bound `lo` (decimal or 0x-hex); default 0.
    #[arg(long)]
    interval_lo: Option<String>,
    /// Interval width as a bit size: k ∈ [lo, lo + 2^bits).
    #[arg(long)]
    interval_bits: Option<u32>,
    /// Interval width as a number (decimal or 0x-hex).
    #[arg(long, conflicts_with = "interval_bits")]
    interval_width: Option<String>,
    /// auto, smart, pohlig-hellman, bsgs, kangaroo, rho.
    #[arg(long, default_value = "auto")]
    method: String,
    #[arg(long, default_value_t = 1)]
    threads: usize,
    #[arg(long, default_value_t = 0x5EED)]
    seed: u64,
    /// Iteration budget for rho / kangaroo.
    #[arg(long)]
    max_iterations: Option<u64>,
    /// Distinguished-point bits (default: derived from the expected cost).
    #[arg(long)]
    dp_bits: Option<u32>,
    /// Disable negation / Frobenius folding in the rho (control arm).
    #[arg(long)]
    no_fold: bool,
    /// Start a whole-group rho even when the audit calls it infeasible.
    #[arg(long)]
    force: bool,
}

#[derive(Args)]
struct SmartArgs {
    /// Size of the anomalous prime.
    #[arg(long, default_value_t = 128)]
    bits: u32,
    /// Selects the CM discriminant and the prime.
    #[arg(long, default_value_t = 0)]
    seed: u64,
    /// The planted scalar (default: random, seeded).
    #[arg(long)]
    secret: Option<String>,
}

#[derive(Args)]
struct BenchArgs {
    curve: String,
    /// Additions to time.
    #[arg(long, default_value_t = 2000)]
    ops: u64,
}

fn parse_uint(s: &str) -> Result<BigUint, String> {
    let t = s.trim();
    if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        BigUint::parse_bytes(h.as_bytes(), 16)
    } else {
        BigUint::parse_bytes(t.as_bytes(), 10)
    }
    .ok_or_else(|| format!("not a number: {s:?}"))
}

fn random_span(bits: u32, available: &BigUint) -> Result<BigUint, String> {
    if bits == 0 {
        return Err("--random-bits must be positive".into());
    }
    if u64::from(bits) >= available.bits() {
        Ok(available.clone())
    } else {
        Ok((BigUint::from(1u8) << bits).min(available.clone()))
    }
}

fn solve_bounds(
    args: &SolveArgs,
    order: &BigUint,
    method: Method,
) -> Result<(BigUint, Option<BigUint>), String> {
    let lo = match &args.interval_lo {
        Some(s) => parse_uint(s)?,
        None => BigUint::from(0u8),
    };
    if &lo >= order {
        return Err("interval lower bound must be below the subgroup order".into());
    }
    let available = order - &lo;
    let infer_from_random = args.random_bits.is_some()
        && (args.interval_lo.is_some()
            || matches!(method, Method::Bsgs | Method::Kangaroo)
            || (method == Method::Auto && order.bits() > 40));
    let width = match (&args.interval_bits, &args.interval_width) {
        (Some(bits), _) => {
            if u64::from(*bits) > order.bits() {
                return Err("interval bit width exceeds the subgroup order".into());
            }
            Some(BigUint::from(1u8) << *bits)
        }
        (None, Some(value)) => Some(parse_uint(value)?),
        (None, None) if infer_from_random => Some(random_span(
            args.random_bits.expect("inferred from random bits"),
            &available,
        )?),
        (None, None) => None,
    };
    if args.interval_lo.is_some() && width.is_none() {
        return Err("--interval-lo requires an interval width or --random-bits".into());
    }
    if let Some(width) = &width {
        if width == &BigUint::from(0u8) || width > &available {
            return Err(
                "interval width must be positive and stay within the subgroup order".into(),
            );
        }
    }
    if let Some(bits) = args.random_bits {
        random_span(bits, &available)?;
    }
    Ok((lo, width))
}

fn load_target_artifact(
    curve: &Curve,
    input: &Path,
) -> Result<(nist::AnyPoint, serde_json::Value), String> {
    let resolved = crypto_lib::store::resolve_path(input)?;
    if std::fs::metadata(&resolved)
        .map_err(|e| e.to_string())?
        .len()
        > 1024 * 1024
    {
        return Err("ECDLP target artifact exceeds 1 MiB".into());
    }
    let bytes = std::fs::read(&resolved).map_err(|e| e.to_string())?;
    if bytes.len() > 1024 * 1024 {
        return Err("ECDLP target artifact exceeds 1 MiB".into());
    }
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let accepted_envelope = (value["schema_version"] == 1
        && value.get("operation").is_none()
        && value.as_object().is_some_and(|obj| {
            obj.keys()
                .all(|key| matches!(key.as_str(), "schema_version" | "curve" | "target"))
        }))
        || (value["operation"] == "solve" && value.get("schema_version").is_none());
    if !accepted_envelope {
        return Err("ECDLP target artifact must be a v1 target envelope or solve report".into());
    }
    if value.get("curve") != Some(&curve.describe()) {
        return Err("ECDLP target artifact curve identity differs from the selected curve".into());
    }
    let x = value["target"]["x"]
        .as_str()
        .ok_or("ECDLP target artifact lacks hex x coordinate")?;
    let y = value["target"]["y"]
        .as_str()
        .ok_or("ECDLP target artifact lacks hex y coordinate")?;
    let point = curve.parse_point(x, y)?;
    Ok((
        point,
        json!({
            "source": input.display().to_string(),
            "sha256": hex::encode(crypto_lib::hash::sha256(&bytes)),
        }),
    ))
}

fn audit_row(a: &Audit) -> String {
    format!(
        "{:<6} {:<8} {:>5} {:>5} {:>3}  {:<5} {:<5} {:<5}  {:>6}  {:>7.1}  {:>7.1}  {}",
        a.name,
        a.family.tag(),
        a.field_bits,
        a.order_bits,
        a.cofactor,
        if a.anomalous { "YES" } else { "no" },
        if a.order_is_prime { "yes" } else { "NO" },
        a.embedding_degree
            .map(|k| k.to_string())
            .unwrap_or_else(|| format!(">{}", a.embedding_degree_bound)),
        a.rho_class_size,
        a.rho_log2_expected_unfolded,
        a.rho_log2_expected,
        a.recommended
    )
}

fn audit_header() -> String {
    format!(
        "{:<6} {:<8} {:>5} {:>5} {:>3}  {:<5} {:<5} {:<5}  {:>6}  {:>7}  {:>7}  {}",
        "curve",
        "family",
        "q",
        "n",
        "h",
        "anom",
        "prime",
        "embed",
        "class",
        "log2ρ",
        "folded",
        "method"
    )
}

fn cmd_list(json: bool) -> Result<(), String> {
    let mut curves = nist_curves();
    curves.extend(nist::certicom_curves());
    let audits: Vec<Audit> = curves.iter().map(audit).collect();
    if json {
        let rows: Vec<serde_json::Value> = curves
            .iter()
            .zip(&audits)
            .map(|(c, a)| json!({"curve": c.describe(), "audit": a}))
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "operation": "list",
                "count": rows.len(),
                "curves": rows,
                "other_names": curve_names(),
            }))
            .unwrap()
        );
        return Ok(());
    }
    println!("{}", audit_header());
    for a in &audits {
        println!("{}", audit_row(a));
    }
    println!();
    println!("q, n: bits of the field and of the generator's order; h: cofactor; anom: #E(F_q) = q (Smart's attack);");
    println!("prime: generator order prime (else Pohlig–Hellman); embed: embedding degree if ≤ {} (MOV);", nist::EMBEDDING_DEGREE_BOUND);
    println!(
        "class: automorphisms folded by the rho (2 = negation, 2m = negation × Frobenius on K-m);"
    );
    println!(
        "log2ρ: log2 of expected rho additions unfolded √(πn/2); folded: with the class folded."
    );
    println!(
        "Also accepted: {}",
        curve_names()
            .into_iter()
            .filter(|n| n.starts_with("toy") || n.starts_with("anom"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    Ok(())
}

fn cmd_audit(name: &str, json: bool) -> Result<(), String> {
    let c = curve_by_name(name)?;
    let a = audit(&c);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(
                &json!({"operation": "audit", "curve": c.describe(), "audit": a})
            )
            .unwrap()
        );
        return Ok(());
    }
    println!("{}", audit_header());
    println!("{}", audit_row(&a));
    println!();
    println!("standard:      {}", a.standard);
    println!("trace:         {}", a.trace);
    println!("group order:   {}", c.group_order);
    println!("subgroup n:    {}", c.order());
    if let Some(l) = &a.frobenius_eigenvalue {
        println!("Frobenius λ:   {l}");
    }
    if let Some(p) = a.extension_degree_prime {
        println!("m prime:       {p} (composite m would open Weil descent)");
    }
    if let Some(k) = a.koblitz_order_consistent {
        println!(
            "Lucas order:   {}",
            if k { "matches h·n" } else { "MISMATCH" }
        );
    }
    if !a.order_factors.is_empty() {
        println!("order factors: {:?}", a.order_factors);
    }
    for n in &a.notes {
        println!("note:          {n}");
    }
    Ok(())
}

fn cmd_solve(args: &SolveArgs, json: bool) -> Result<(), String> {
    let curve = curve_by_name(&args.curve)?;
    let method: Method = args.method.parse()?;
    let mut rng = StdRng::seed_from_u64(args.seed ^ 0xC0FFEE);
    let (lo, width) = solve_bounds(args, curve.order(), method)?;
    // Target: planted or supplied.
    let (target, planted, target_artifact): (
        nist::AnyPoint,
        Option<BigUint>,
        Option<serde_json::Value>,
    ) = if args.challenge {
        if curve.name.eq_ignore_ascii_case("ECCp-131") {
            (
                curve.parse_point(
                    "03AA6F004FC62E2DA1ED0BFB62C3FFB568",
                    "009C21C284BA8A445BB2701BF55E3A67ED",
                )?,
                None,
                None,
            )
        } else {
            let (_, target) = BinaryCurve::certicom_challenge(&curve.name)
                .ok_or_else(|| format!("{} is not a supported Certicom challenge", curve.name))?;
            (nist::AnyPoint::Binary(target), None, None)
        }
    } else if let Some(path) = &args.point_file {
        let (point, receipt) = load_target_artifact(&curve, path)?;
        (point, None, Some(receipt))
    } else if let Some(pt) = &args.point {
        let (x, y) = pt
            .split_once(',')
            .ok_or_else(|| "--point wants X,Y in hex".to_string())?;
        (curve.parse_point(x, y)?, None, None)
    } else {
        let k = match (&args.secret, args.random_bits) {
            (Some(s), _) => {
                let k = parse_uint(s)?;
                if &k >= curve.order() {
                    return Err("planted scalar must be below the subgroup order".into());
                }
                if width.as_ref().is_some_and(|w| k < lo || k >= &lo + w) {
                    return Err("planted scalar is outside the declared interval".into());
                }
                k
            }
            (None, Some(bits)) => {
                // Keep the sampled scalar in both the declared interval and
                // the subgroup's canonical [0, n) range, without wraparound.
                let available = curve.order() - &lo;
                let mut span = random_span(bits, &available)?;
                if let Some(w) = &width {
                    span = span.min(w.clone());
                }
                &lo + rng.gen_biguint_below(&span)
            }
            (None, None) => {
                return Err(
                    "give --secret K, --random-bits B, --point X,Y, --point-file PATH, or --challenge"
                        .into(),
                );
            }
        };
        (curve.plant(&k), Some(k), None)
    };
    let interval = width.map(|w| (lo.clone(), w));
    let opts = SolveOptions {
        method,
        interval,
        threads: args.threads.max(1),
        seed: args.seed,
        max_iterations: args.max_iterations,
        dp_bits: args.dp_bits,
        fold_automorphisms: !args.no_fold,
        force: args.force,
        ..SolveOptions::default()
    };
    let report = solve(&curve, &target, &opts);
    let (tx, ty) = curve.point_hex(&target).unwrap_or_default();
    let planted_ok = planted
        .as_ref()
        .map(|k| report.scalar_value.as_ref() == Some(k));
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "operation": "solve",
                "curve": curve.describe(),
                "target": {"x": tx, "y": ty},
                "target_artifact": target_artifact,
                "planted_scalar": planted.as_ref().map(|k| k.to_string()),
                "planted_recovered": planted_ok,
                "report": report,
            }))
            .unwrap()
        );
    } else {
        println!("curve:     {} ({})", report.curve, report.family.tag());
        println!("target:    ({tx}, {ty})");
        if let Some(artifact) = &target_artifact {
            println!(
                "artifact:  {} (SHA-256 {})",
                artifact["source"].as_str().unwrap_or("?"),
                artifact["sha256"].as_str().unwrap_or("?")
            );
        }
        println!("method:    {}", report.method);
        match &report.scalar {
            Some(k) => println!(
                "scalar:    {k}  (0x{})  verified={}",
                report.scalar_hex.as_deref().unwrap_or(""),
                report.verified
            ),
            None => println!("scalar:    not found"),
        }
        if let Some(k) = &planted {
            println!("planted:   {k}  recovered={}", planted_ok.unwrap_or(false));
        }
        println!(
            "cost:      {} group additions in {} ms on {} thread(s); expected ≈ {:.3e}",
            report.group_ops, report.elapsed_ms, report.threads, report.expected_ops
        );
        if report.table_size > 0 {
            println!("table:     {} entries", report.table_size);
        }
        for n in &report.notes {
            println!("note:      {n}");
        }
        if let Some(f) = &report.failure {
            println!("failure:   {f}");
        }
    }
    if report.failure.is_some() && report.scalar.is_none() {
        return Err(String::new());
    }
    Ok(())
}

fn cmd_smart(args: &SmartArgs, json: bool) -> Result<(), String> {
    let t0 = Instant::now();
    let cp = anomalous::generate(args.bits, args.seed)?;
    let construct_ms = t0.elapsed().as_millis();
    let curve = curve_by_name(&format!("anomalous-{}-{}", args.bits, args.seed))?;
    let mut rng = StdRng::seed_from_u64(args.seed ^ 0x5A4D);
    let k = match &args.secret {
        Some(s) => parse_uint(s)? % curve.order(),
        None => rng.gen_biguint_below(curve.order()),
    };
    let target = curve.plant(&k);
    let report = solve(&curve, &target, &SolveOptions::default());
    let ok = report.scalar_value.as_ref() == Some(&k);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "operation": "smart",
                "curve": {
                    "name": cp.name, "p": cp.p.to_string(), "a": cp.a.to_string(), "b": cp.b.to_string(),
                    "gx": cp.gx.to_string(), "gy": cp.gy.to_string(), "order": cp.n.to_string(),
                    "bits": cp.p.bits(), "construct_ms": construct_ms,
                },
                "planted_scalar": k.to_string(),
                "planted_recovered": ok,
                "report": report,
            }))
            .unwrap()
        );
    } else {
        println!(
            "anomalous curve ({} bits, {}): y² = x³ + {}x + {} over F_p",
            cp.p.bits(),
            cp.name,
            cp.a,
            cp.b
        );
        println!("p = #E(F_p) = {}", cp.p);
        println!("constructed in {construct_ms} ms; planted k = {k}");
        println!("method:  {}", report.method);
        match &report.scalar {
            Some(s) => println!(
                "recovered k = {s}  (match={ok}) in {} ms",
                report.elapsed_ms
            ),
            None => println!(
                "not recovered: {}",
                report.failure.as_deref().unwrap_or("?")
            ),
        }
        for n in &report.notes {
            println!("note:    {n}");
        }
    }
    if ok {
        Ok(())
    } else {
        Err(String::new())
    }
}

fn bench_group<G: EcdlpGroup>(g: &G, ops: u64) -> (f64, u64) {
    let gen = g.generator();
    let mut p = g.double(&gen);
    let t0 = Instant::now();
    for _ in 0..ops {
        p = g.add(&p, &gen);
    }
    let dt = t0.elapsed().as_secs_f64();
    (ops as f64 / dt, g.key(&p))
}

fn cmd_bench(args: &BenchArgs, json: bool) -> Result<(), String> {
    let c: Curve = curve_by_name(&args.curve)?;
    let (rate, _) = match &c.group {
        Group::Prime(g) => bench_group(g.as_ref(), args.ops),
        Group::Binary(g) => bench_group(g.as_ref(), args.ops),
    };
    let a = audit(&c);
    let folded_secs = 2f64.powf(a.rho_log2_expected) / rate;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "operation": "bench",
                "curve": c.name,
                "additions_per_second": rate,
                "rho_log2_expected_folded": a.rho_log2_expected,
                "single_thread_rho_seconds_log2": folded_secs.log2(),
            }))
            .unwrap()
        );
    } else {
        println!(
            "{}: {:.0} affine additions/s (this host, one thread, BigUint/F2m arithmetic)",
            c.name, rate
        );
        println!(
            "whole-group rho would need 2^{:.1} additions ≈ 2^{:.1} s single-threaded here (≈ 2^{:.1} years)",
            a.rho_log2_expected,
            folded_secs.log2(),
            (folded_secs / 3.15e7).log2()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args() -> SolveArgs {
        SolveArgs {
            curve: "P-256".into(),
            secret: None,
            random_bits: None,
            point: None,
            point_file: None,
            challenge: false,
            interval_lo: None,
            interval_bits: None,
            interval_width: None,
            method: "auto".into(),
            threads: 1,
            seed: 1,
            max_iterations: None,
            dp_bits: None,
            no_fold: false,
            force: false,
        }
    }

    #[test]
    fn random_bits_infer_large_curve_interval_and_reject_invalid_bounds() {
        let order = curve_by_name("P-256").unwrap().order().clone();
        let mut input = args();
        input.random_bits = Some(8);
        assert_eq!(
            solve_bounds(&input, &order, Method::Auto).unwrap().1,
            Some(BigUint::from(256u32))
        );
        let small_order = (BigUint::from(1u32) << 23) - BigUint::from(1u32);
        assert!(solve_bounds(&input, &small_order, Method::Auto)
            .unwrap()
            .1
            .is_none());
        input.interval_width = Some("0".into());
        assert!(solve_bounds(&input, &order, Method::Auto)
            .unwrap_err()
            .contains("positive"));
        input.interval_width = None;
        input.interval_lo = Some((&order - BigUint::from(1u32)).to_string());
        input.interval_bits = Some(1);
        assert!(solve_bounds(&input, &order, Method::Auto)
            .unwrap_err()
            .contains("within"));
        input.interval_lo = None;
        input.interval_bits = None;
        input.random_bits = Some(0);
        assert!(solve_bounds(&input, &order, Method::Auto)
            .unwrap_err()
            .contains("positive"));
    }

    #[test]
    fn target_artifact_requires_exact_curve_and_valid_point() {
        let curve = curve_by_name("P-256").unwrap();
        let fixture = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/ic/fixtures/ecdlp_nist_p256_target_20261022.json"
        ));
        let (point, receipt) = load_target_artifact(&curve, fixture).unwrap();
        assert_eq!(point, curve.plant(&BigUint::from(38u32)));
        assert_eq!(
            receipt["sha256"],
            hex::encode(crypto_lib::hash::sha256(&std::fs::read(fixture).unwrap()))
        );
        let path = std::env::temp_dir().join(format!(
            "ecdlp-target-{}-{}.json",
            std::process::id(),
            line!()
        ));
        let mut value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(fixture).unwrap()).unwrap();
        value["curve"]["field_and_equation"]["p_hex"] = json!("dead");
        std::fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(load_target_artifact(&curve, &path)
            .err()
            .unwrap()
            .contains("curve identity"));
        value["curve"] = curve.describe();
        value["target"]["y"] = json!("0");
        std::fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(load_target_artifact(&curve, &path).is_err());
        value["target"]["y"] = json!(curve.point_hex(&point).unwrap().1);
        value.as_object_mut().unwrap().remove("schema_version");
        value["operation"] = json!("solve");
        std::fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
        assert_eq!(load_target_artifact(&curve, &path).unwrap().0, point);
        std::fs::remove_file(path).unwrap();
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let r = match &cli.command {
        Command::List => cmd_list(cli.json),
        Command::Audit(a) => cmd_audit(&a.curve, cli.json),
        Command::Solve(a) => cmd_solve(a, cli.json),
        Command::Smart(a) => cmd_smart(a, cli.json),
        Command::Bench(a) => cmd_bench(a, cli.json),
    };
    match r {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            if !e.is_empty() {
                eprintln!("error: {e}");
            }
            ExitCode::FAILURE
        }
    }
}
