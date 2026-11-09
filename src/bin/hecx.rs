//! # `hecx` — a hyperelliptic discrete-log front door.
//!
//! `hecx` accepts an arbitrary hyperelliptic curve over a prime or binary
//! field, verifies it, estimates the direct index-calculus cost, and (when
//! the instance is small enough) solves a known-answer discrete logarithm
//! with the repository's Jacobian engines.
//!
//! ```text
//! hecx list
//! hecx inspect --field prime --p 1019 --genus 2 --f 1,1,2,3,0,1
//! hecx estimate --field prime --p 1019 --genus 2 --f 1,1,2,3,0,1
//! hecx run --field prime --p 1019 --genus 2 --f 1,1,2,3,0,1 --secret 7
//! hecx run --field binary --m 8 --modulus 0,1,3,4 --genus 2 --h 0,1 \
//!           --f 1,0,1,0,3,1 --secret 37
//! ```
//!
//! The estimator is explicitly a heuristic for the *direct degree-one
//! factor-base* index calculus.  It is not a security claim and it does not
//! model Weil descent, summation polynomials, or any specialized family
//! attack.

use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};

use crypto_lib::binary_ecc::f2m::{F2mElement, IrreduciblePoly};
use crypto_lib::binary_ecc::hyperelliptic::{HyperellipticCurve, MumfordDivisor};
use crypto_lib::binary_ecc::poly_f2m::F2mPoly;
use crypto_lib::cryptanalysis::binary_hyperelliptic_index_calculus::{
    build_factor_base as build_binary_factor_base, default_factor_base_size as binary_default_base,
    divisor_order_bsgs as binary_order_bsgs, BhecFactorBase,
};
use crypto_lib::cryptanalysis::hec_dlp::{
    factor_order, pohlig_hellman, HecGroup, HecPrimeCall, HecSolveConfig, HecSolveReport,
};
use crypto_lib::cryptanalysis::hyperelliptic_index_calculus::{
    build_factor_base as build_prime_factor_base, divisor_order_bsgs as prime_order_bsgs,
    HecFactorBase,
};
use crypto_lib::prime_hyperelliptic::{FpPoly, HyperellipticCurveP, MumfordDivisorP};

#[derive(Parser)]
#[command(
    name = "hecx",
    version,
    about = "Hyperelliptic discrete logarithms: inspect, estimate, solve",
    long_about = "A front door for the repository's prime-field and binary-field \
                  hyperelliptic Jacobian engines. It accepts arbitrary genus, \
                  coefficients and subgroup orders; solves with index calculus \
                  in large prime-order components and baby-step giant-step in \
                  small ones, under a Pohlig-Hellman decomposition."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    /// Emit one JSON object (the default for this tool).
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Command {
    /// List the supported field families and engine capabilities.
    List,
    /// Parse and validate a curve, and report its basic invariants.
    Inspect(CurveArgs),
    /// Heuristic direct-IC versus rho cost estimate for the curve.
    Estimate(EstimateArgs),
    /// Run a known-answer or planted discrete logarithm.
    Run(RunArgs),
    /// Run a built-in small instance end to end.
    Demo(DemoArgs),
}

#[derive(Args, Clone, Debug)]
struct CurveArgs {
    /// `prime` or `binary`.
    #[arg(long, default_value = "prime")]
    field: String,
    /// Prime modulus `p` (decimal or 0x-hex), for `--field prime`.
    #[arg(long)]
    p: Option<String>,
    /// Extension degree `m`, for `--field binary`.
    #[arg(long)]
    m: Option<u32>,
    /// Irreducible modulus low terms, e.g. `0,1,3,4` for z^8+z^4+z^3+z+1.
    #[arg(long)]
    modulus: Option<String>,
    /// Genus.
    #[arg(long)]
    genus: u32,
    /// `f` coefficients, lowest degree first, decimal or 0x-hex.
    #[arg(long)]
    f: String,
    /// `h` coefficients (binary field only), lowest degree first.
    #[arg(long)]
    h: Option<String>,
}

#[derive(Args, Debug)]
struct EstimateArgs {
    #[command(flatten)]
    curve: CurveArgs,
    /// Factor-base size; omit for the engine's heuristic choice.
    #[arg(long)]
    fb_size: Option<usize>,
    /// Stated subgroup order, if known (used only to report bit length).
    #[arg(long)]
    order: Option<String>,
}

#[derive(Args, Debug)]
struct RunArgs {
    #[command(flatten)]
    curve: CurveArgs,
    /// Base divisor point coordinates `x,y`; for binary fields these are
    /// field-element bitmasks. Omit to auto-pick a factor-base place.
    #[arg(long = "P", value_name = "X,Y")]
    p_point: Option<String>,
    /// Target divisor point coordinates. Required unless `--secret` is given.
    #[arg(long = "Q", value_name = "X,Y")]
    q_point: Option<String>,
    /// Plant a target `Q = secret * P`; ignored when `--Q` is given.
    #[arg(long)]
    secret: Option<String>,
    /// Known subgroup order. Omit to compute it with BSGS.
    #[arg(long)]
    order: Option<String>,
    /// Factor-base size; `0` selects the engine default.
    #[arg(long, default_value_t = 0)]
    fb_size: usize,
    #[arg(long, default_value_t = 200_000)]
    max_trials: usize,
    #[arg(long, default_value_t = 1)]
    seed: u64,
    #[arg(long, default_value_t = 8)]
    extra_relations: usize,
    #[arg(long, default_value_t = 200_000)]
    bsgs_max_steps: usize,
    /// Known factorisation of the subgroup order: `q1^e1,q2^e2,...`.
    #[arg(long)]
    factors: Option<String>,
    #[arg(long, default_value_t = 1 << 16)]
    trial_factor_bound: u64,
    #[arg(long, default_value_t = 1 << 20)]
    small_prime_bound: u64,
    /// Relation-collection threads for the binary engine.
    #[arg(long, default_value_t = 1)]
    threads: usize,
}

#[derive(Args, Debug)]
struct DemoArgs {
    /// `prime` or `binary`.
    #[arg(long, default_value = "prime")]
    field: String,
    /// Genus 2 or 3.
    #[arg(long, default_value_t = 2)]
    genus: u32,
    #[arg(long, default_value_t = 7)]
    secret: u64,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match &cli.command {
        Command::List => Ok(list()),
        Command::Inspect(args) => inspect(args),
        Command::Estimate(args) => estimate(args),
        Command::Run(args) => run(args),
        Command::Demo(args) => demo(args),
    };
    match result {
        Ok(value) => {
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&value).unwrap());
            } else {
                println!("{}", serde_json::to_string_pretty(&value).unwrap());
            }
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("hecx: {message}");
            ExitCode::FAILURE
        }
    }
}

fn list() -> Value {
    json!({
        "schema": "hecx/v1",
        "operation": "list",
        "capabilities": {
            "prime": {
                "field": "F_p",
                "models": "y^2 = f(x), deg f = 2g+1, f squarefree",
                "sizes": "deterministic checked path p <= 2^64; wider primes use cached Miller-Rabin and a sampled reduced base",
                "genus": "any positive genus the field supports",
                "solvers": ["Gaudry/ADH degree-one IC", "BSGS", "Pohlig-Hellman"]
            },
            "binary": {
                "field": "F_2^m",
                "models": "y^2 + h(x)y = f(x), deg f = 2g+1, deg h <= g",
                "sizes": "field arithmetic to m = 4096; full factor base practical to m ~ 20",
                "genus": "any positive genus the field supports",
                "solvers": ["trace Cantor-Zassenhaus smoothness", "degree-one IC", "BSGS", "Pohlig-Hellman"]
            }
        },
        "commands": ["list", "inspect", "estimate", "run", "demo"],
        "claim_boundary": "Toy and known-answer instances. Not a key-recovery claim; the estimator is a heuristic for direct degree-one IC only."
    })
}

fn inspect(args: &CurveArgs) -> Result<Value, String> {
    match args.field.as_str() {
        "prime" => {
            let curve = parse_prime_curve(args)?;
            let bits = curve.p.bits();
            let (lo, hi) =
                crypto_lib::cryptanalysis::hyperelliptic_index_calculus::hasse_interval(&curve)
                    .map(|(l, h)| (l.to_string(), h.to_string()))
                    .unwrap_or_else(|| ("unknown".into(), "unknown".into()));
            let full = (bits <= 20).then(|| {
                build_prime_factor_base(&curve, usize::MAX)
                    .len()
                    .to_string()
            });
            Ok(json!({
                "schema": "hecx/v1",
                "operation": "inspect",
                "status": "ok",
                "field": "prime",
                "curve": prime_curve_json(&curve),
                "field_bits": bits,
                "hasse_interval": {"lo": lo, "hi": hi},
                "full_factor_base": full,
                "engine": "prime-field Mumford/Cantor + degree-one IC",
            }))
        }
        "binary" => {
            let curve = parse_binary_curve(args)?;
            let (lo, hi) =
                crypto_lib::cryptanalysis::binary_hyperelliptic_index_calculus::hasse_interval(
                    &curve,
                );
            Ok(json!({
                "schema": "hecx/v1",
                "operation": "inspect",
                "status": "ok",
                "field": "binary",
                "curve": binary_curve_json(&curve),
                "field_bits": curve.m,
                "hasse_interval": {"lo": lo.to_string(), "hi": hi.to_string()},
                "full_factor_base": (curve.m <= 16).then(|| {
                    build_binary_factor_base(&curve, usize::MAX, 0).len().to_string()
                }),
                "max_degree_one_columns": (curve.m <= 40).then(|| (1u64 << curve.m).to_string()),
                "engine": "characteristic-two Mumford/Cantor + trace Cantor-Zassenhaus + degree-one IC",
            }))
        }
        other => Err(format!("unknown --field '{other}'; use prime|binary")),
    }
}

fn estimate(args: &EstimateArgs) -> Result<Value, String> {
    let (q, full_base, field_label, genus, model) = match args.curve.field.as_str() {
        "prime" => {
            let p = require_int(&args.curve.p, "--p")?;
            if p < BigUint::from(5u32) {
                return Err("--p must be an odd prime greater than 3".into());
            }
            let f = parse_list(&args.curve.f)?;
            let deg = f.len().saturating_sub(1);
            let expected = 2 * args.curve.genus as usize + 1;
            if deg != expected {
                return Err(format!("--f has degree {deg}, expected 2g+1 = {expected}"));
            }
            // Number of ±-identified degree-one places is about p/2.
            let full = big_log2(&p) - 1.0;
            (
                p.clone(),
                full,
                format!("F_p ({} bits)", p.bits()),
                args.curve.genus,
                format!("y^2 = f(x), deg f = {deg}"),
            )
        }
        "binary" => {
            let m = args.curve.m.ok_or("--m is required for --field binary")?;
            let f = parse_list(&args.curve.f)?;
            let deg = f.len().saturating_sub(1);
            let expected = 2 * args.curve.genus as usize + 1;
            if deg != expected {
                return Err(format!("--f has degree {deg}, expected 2g+1 = {expected}"));
            }
            let q = BigUint::one() << m as usize;
            (
                q.clone(),
                big_log2(&q),
                format!("F_2^{m} ({m} bits)"),
                args.curve.genus,
                format!("y^2 + h(x)y = f(x), deg f = {deg}"),
            )
        }
        other => return Err(format!("unknown --field '{other}'; use prime|binary")),
    };

    let g = genus as f64;
    let q_log2 = big_log2(&q);
    let order = args
        .order
        .as_deref()
        .map(parse_int)
        .transpose()?
        .unwrap_or_else(|| q.pow(genus));
    let order_bits = order.bits() as f64;
    let rho_log2 = 0.5 * order_bits + (std::f64::consts::PI / 4.0).sqrt().log2();

    // Work in log2 throughout: the estimate must stay finite for 4096-bit
    // fields, where raw factor-base sizes overflow an f64.
    let default_b_log2 = (2.0 / (g + 1.0) * q_log2).max(3.0).min(full_base);
    let b_log2 = args
        .fb_size
        .map(|v| (v.max(1) as f64).log2())
        .unwrap_or(default_b_log2)
        .min(full_base);
    let factorial_log2: f64 = (1..=genus as u64).map(|v| (v as f64).log2()).sum();
    let coverage_log2 = if b_log2 >= full_base {
        0.0
    } else {
        g * (b_log2 - full_base)
    };
    let log_smooth_prob = coverage_log2 - factorial_log2;
    let log_trials = b_log2.max(1.0) - log_smooth_prob;

    // One walked group operation per trial, plus base construction.
    let log_group_ops = logsumexp2(log_trials, b_log2);
    // Smoothness oracle: ~g^2 log q field operations per trial.
    let log_oracle_field_ops = log_trials + 2.0 * g.log2() + q_log2.max(1.0).log2();
    // Dense and sparse linear algebra, in field operations.
    let log_dense_la = 3.0 * b_log2 + 3.0f64.log2();
    let log_sparse_la = b_log2 + 2.0 * g.log2() + 4.0;
    // Convert field operations to group-operation equivalents by log q.
    let log_oracle_group = log_oracle_field_ops - q_log2.max(1.0).log2();
    let log_ic_dense = logsumexp3(log_group_ops, log_dense_la, log_oracle_group);
    let log_ic_sparse = logsumexp3(log_group_ops, log_sparse_la, log_oracle_group);

    let smooth_prob = 2f64.powf(log_smooth_prob);
    let expected_trials = 2f64.powf(log_trials);
    let count_json = |log2_value: f64| -> Value {
        if log2_value < 53.0 {
            json!(2f64.powf(log2_value))
        } else {
            json!(format!("2^{log2_value:.1}"))
        }
    };

    let mut notes = vec![
        "Heuristic H1: a random reduced divisor splits over a full degree-one base with probability about 1/g!.".to_string(),
        "A truncated base of size B scales that by (B/full)^g; this is Gaudry's reduced-base model.".to_string(),
        "The estimate is for direct degree-one index calculus only. It does not model Weil descent, summation polynomials or family-specific attacks.".to_string(),
    ];
    if args.curve.field == "prime" {
        notes.push(
            "IC on prime-field Jacobians is not a sub-exponential threat; this row is for scaling studies."
                .to_string(),
        );
    } else {
        notes.push(
            "This is the direct Jacobian IC cost, not the Weil-descent / summation-polynomial attack on the associated elliptic curve."
                .to_string(),
        );
    }
    notes.push("No security claim is made and no deployed curve is attacked.".to_string());

    Ok(json!({
        "schema": "hecx/v1",
        "operation": "estimate",
        "status": "ok",
        "field": args.curve.field,
        "field_label": field_label,
        "model": model,
        "genus": genus,
        "field_size_log2": q_log2,
        "group_order_bits": order.bits(),
        "stated_order": order.to_string(),
        "full_factor_base": count_json(full_base),
        "factor_base_size": count_json(b_log2),
        "smooth_probability_heuristic": if log_smooth_prob > -1000.0 {
            json!(smooth_prob)
        } else {
            json!(format!("2^{log_smooth_prob:.1}"))
        },
        "expected_trials_heuristic": if log_trials < 60.0 {
            json!(expected_trials)
        } else {
            json!(format!("2^{log_trials:.1}"))
        },
        "index_calculus": {
            "group_ops_log2": log_group_ops,
            "oracle_field_ops_log2": log_oracle_field_ops,
            "dense_la_ops_log2": log_dense_la,
            "sparse_la_ops_log2": log_sparse_la,
            "total_dense_log2": log_ic_dense,
            "total_sparse_log2": log_ic_sparse,
        },
        "rho": {
            "ops_log2": rho_log2,
            "ops": format!("2^{rho_log2:.1}"),
        },
        "ratio_sparse_ic_over_rho_log2": log_ic_sparse - rho_log2,
        "notes": notes,
        "claim_boundary": "heuristic_estimate"
    }))
}

fn run(args: &RunArgs) -> Result<Value, String> {
    match args.curve.field.as_str() {
        "prime" => run_prime(args),
        "binary" => run_binary(args),
        other => Err(format!("unknown --field '{other}'; use prime|binary")),
    }
}

fn run_prime(args: &RunArgs) -> Result<Value, String> {
    let curve = parse_prime_curve(&args.curve)?;
    if args.p_point.is_none() && curve.p.bits() > 20 {
        return Err(
            "prime fields above 2^20 need --P (and normally --order); the full-base auto-pick \
             enumerates F_p"
                .into(),
        );
    }
    let cfg = solve_config(args);
    let (base, order) = if let Some(point) = &args.p_point {
        let (x, y) = parse_point(point)?;
        let base =
            MumfordDivisorP::from_point(&curve, &x, &y).ok_or("--P is not a point on the curve")?;
        let order = match &args.order {
            Some(s) => parse_int(s)?,
            None => prime_order_bsgs(&curve, &base, cfg.bsgs_max_steps)
                .ok_or("could not find the order of --P with the BSGS budget; pass --order")?,
        };
        (base, order)
    } else {
        let fb = build_prime_factor_base(
            &curve,
            if cfg.fb_size == 0 {
                usize::MAX
            } else {
                cfg.fb_size
            },
        );
        let mut found = None;
        for entry in fb.entries.iter().take(64) {
            if let Some(order) = prime_order_bsgs(&curve, &entry.divisor, cfg.bsgs_max_steps) {
                if order > BigUint::from(2u32) {
                    found = Some((entry.divisor.clone(), order));
                    break;
                }
            }
        }
        let (base, derived) = found.ok_or("no usable factor-base place; pass --P")?;
        let order = match &args.order {
            Some(s) => parse_int(s)?,
            None => derived,
        };
        if !curve_is_identity(&base.scalar_mul_fast(&order, &curve)) {
            return Err("--order does not annihilate the auto-picked base; pass --P".into());
        }
        (base, order)
    };

    let target = target_for(&curve, &base, &order, args)?;
    let factors = parse_factors(args)?;
    let (answer, report) = pohlig_hellman(&curve, &base, &target, &order, factors, &cfg);
    Ok(run_json(
        "prime",
        prime_curve_json(&curve),
        &curve,
        &base,
        &target,
        &order,
        answer,
        report,
    ))
}

fn run_binary(args: &RunArgs) -> Result<Value, String> {
    let curve = parse_binary_curve(&args.curve)?;
    let cfg = solve_config(args);
    let (base, order) = if let Some(point) = &args.p_point {
        let (x, y) = parse_point(point)?;
        let x = F2mElement::from_biguint(&x, curve.m);
        let y = F2mElement::from_biguint(&y, curve.m);
        if !curve.is_on_curve(&x, &y) {
            return Err("--P is not a point on the curve".into());
        }
        let base = MumfordDivisor::from_point(&curve, &x, &y)
            .ok_or("--P could not be converted to a divisor")?;
        let order = match &args.order {
            Some(s) => parse_int(s)?,
            None => binary_order_bsgs(&curve, &base, cfg.bsgs_max_steps)
                .ok_or("could not find the order of --P with the BSGS budget; pass --order")?,
        };
        (base, order)
    } else {
        let size = if cfg.fb_size == 0 {
            binary_default_base(&curve)
        } else {
            cfg.fb_size
        };
        let fb = build_binary_factor_base(&curve, size, cfg.seed);
        let mut found = None;
        for entry in fb.entries.iter() {
            if let Some(order) = binary_order_bsgs(&curve, &entry.divisor, cfg.bsgs_max_steps) {
                if order > BigUint::from(2u32) {
                    found = Some((entry.divisor.clone(), order));
                    break;
                }
            }
        }
        let (base, derived) = found.ok_or("no usable factor-base place; pass --P")?;
        let order = match &args.order {
            Some(s) => parse_int(s)?,
            None => derived,
        };
        if !base.scalar_mul_fast(&order, &curve).is_identity() {
            return Err("--order does not annihilate the auto-picked base; pass --P".into());
        }
        (base, order)
    };

    let target = binary_target_for(&curve, &base, &order, args)?;
    let factors = parse_factors(args)?;
    let (answer, report) = pohlig_hellman(&curve, &base, &target, &order, factors, &cfg);
    Ok(run_json(
        "binary",
        binary_curve_json(&curve),
        &curve,
        &base,
        &target,
        &order,
        answer,
        report,
    ))
}

fn solve_config(args: &RunArgs) -> HecSolveConfig {
    HecSolveConfig {
        fb_size: args.fb_size,
        max_trials: args.max_trials,
        extra_relations: args.extra_relations,
        seed: args.seed,
        trial_factor_bound: args.trial_factor_bound,
        bsgs_max_steps: args.bsgs_max_steps,
        small_prime_bound: args.small_prime_bound,
        threads: args.threads,
    }
}

fn target_for(
    curve: &HyperellipticCurveP,
    base: &MumfordDivisorP,
    order: &BigUint,
    args: &RunArgs,
) -> Result<MumfordDivisorP, String> {
    if let Some(point) = &args.q_point {
        let (x, y) = parse_point(point)?;
        let q =
            MumfordDivisorP::from_point(curve, &x, &y).ok_or("--Q is not a point on the curve")?;
        if !q.scalar_mul_fast(order, curve).is_identity() {
            return Err("--Q is not in the order-`--order` subgroup".into());
        }
        return Ok(q);
    }
    let secret = require_int(&args.secret, "--secret or --Q")?;
    Ok(base.scalar_mul_fast(&(&secret % order), curve))
}

fn binary_target_for(
    curve: &HyperellipticCurve,
    base: &MumfordDivisor,
    order: &BigUint,
    args: &RunArgs,
) -> Result<MumfordDivisor, String> {
    if let Some(point) = &args.q_point {
        let (x, y) = parse_point(point)?;
        let x = F2mElement::from_biguint(&x, curve.m);
        let y = F2mElement::from_biguint(&y, curve.m);
        if !curve.is_on_curve(&x, &y) {
            return Err("--Q is not a point on the curve".into());
        }
        let q = MumfordDivisor::from_point(curve, &x, &y)
            .ok_or("--Q could not be converted to a divisor")?;
        if !q.scalar_mul_fast(order, curve).is_identity() {
            return Err("--Q is not in the order-`--order` subgroup".into());
        }
        return Ok(q);
    }
    let secret = require_int(&args.secret, "--secret or --Q")?;
    Ok(base.scalar_mul_fast(&(&secret % order), curve))
}

fn run_json<G: HecGroup>(
    field: &'static str,
    curve_json: Value,
    group: &G,
    base: &G::Elem,
    target: &G::Elem,
    order: &BigUint,
    answer: Option<BigUint>,
    report: HecSolveReport,
) -> Value {
    let verified = report.verified;
    let status = if verified { "ok" } else { "unsolved" };
    json!({
        "schema": "hecx/v1",
        "operation": "run",
        "status": status,
        "field": field,
        "engine": group.describe(),
        "curve": curve_json,
        "order": order.to_string(),
        "order_bits": order.bits(),
        "P": group.elem_json(base),
        "Q": group.elem_json(target),
        "k": answer.map(|k| k.to_string()),
        "verified": verified,
        "factors": report.factors.iter().map(|(q, e)| json!([q.to_string(), e])).collect::<Vec<_>>(),
        "factorisation_complete": report.factorisation_complete,
        "residues": report.residues.iter().map(|(m, r)| json!([m.to_string(), r.to_string()])).collect::<Vec<_>>(),
        "prime_calls": report.prime_calls.iter().map(prime_call_json).collect::<Vec<_>>(),
        "notes": report.notes,
        "claim_boundary": "known-answer_or_planted_instance; verification is [k]P = Q"
    })
}

fn prime_call_json(call: &HecPrimeCall) -> Value {
    let mut value = json!({
        "order": call.order.to_string(),
        "method": call.method,
        "scalar": call.scalar.as_ref().map(|k| k.to_string()),
        "bsgs_steps": call.bsgs_steps,
    });
    if let Some(report) = &call.index_calculus {
        value["index_calculus"] = json!({
            "factor_base_size": report.factor_base_size,
            "trials": report.trials,
            "smooth_trials": report.smooth_trials,
            "relations": report.relations,
            "jacobian_ops": report.jacobian_ops,
            "precompute_ops": report.precompute_ops,
            "solve_row_ops": report.solve_row_ops,
            "solved": report.solved,
            "walk_ms": report.walk_wall_ns as f64 / 1e6,
            "smoothness_ms": report.smoothness_wall_ns as f64 / 1e6,
            "solve_ms": report.solve_wall_ns as f64 / 1e6,
        });
    }
    value
}

fn demo(args: &DemoArgs) -> Result<Value, String> {
    match args.field.as_str() {
        "prime" => {
            // A small deterministic genus-2 family; the first c whose
            // model passes the checked constructor is used.
            let p = BigUint::from(1019u32);
            let mut curve = None;
            for c in 1u64..64 {
                let f = FpPoly::from_coeffs(
                    vec![
                        BigUint::from(c),
                        BigUint::one(),
                        BigUint::from(2u32),
                        BigUint::from(3u32),
                        BigUint::zero(),
                        BigUint::one(),
                    ],
                    p.clone(),
                );
                if let Ok(candidate) = HyperellipticCurveP::try_new(p.clone(), f, 2) {
                    curve = Some(candidate);
                    break;
                }
            }
            let curve = curve.ok_or("no smooth prime demo model")?;
            let run_args = RunArgs {
                curve: CurveArgs {
                    field: "prime".into(),
                    p: Some(p.to_string()),
                    m: None,
                    modulus: None,
                    genus: 2,
                    f: curve
                        .f
                        .coeffs
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                    h: None,
                },
                p_point: None,
                q_point: None,
                secret: Some(args.secret.to_string()),
                order: None,
                fb_size: 0,
                max_trials: 200_000,
                seed: 1,
                extra_relations: 8,
                bsgs_max_steps: 200_000,
                factors: None,
                trial_factor_bound: 1 << 16,
                small_prime_bound: 1 << 20,
                threads: 1,
            };
            run_prime(&run_args)
        }
        "binary" => {
            let m = 8u32;
            let irr = IrreduciblePoly::deg_8();
            let mut curve = None;
            for c in 1u64..64 {
                let h = F2mPoly::from_coeffs(vec![F2mElement::zero(m), F2mElement::one(m)], m);
                let mut fc = vec![F2mElement::zero(m); 6];
                fc[0] = F2mElement::one(m);
                fc[2] = F2mElement::one(m);
                fc[4] = F2mElement::from_biguint(&BigUint::from(c), m);
                fc[5] = F2mElement::one(m);
                let f = F2mPoly::from_coeffs(fc, m);
                if let Ok(candidate) = HyperellipticCurve::try_new(m, irr.clone(), h, f, 2) {
                    curve = Some(candidate);
                    break;
                }
            }
            let curve = curve.ok_or("no smooth binary demo model")?;
            let run_args = RunArgs {
                curve: CurveArgs {
                    field: "binary".into(),
                    p: None,
                    m: Some(m),
                    modulus: Some("0,1,3,4".into()),
                    genus: 2,
                    f: curve
                        .f
                        .coeffs
                        .iter()
                        .map(|c| format!("{:x}", c.to_biguint()))
                        .collect::<Vec<_>>()
                        .join(","),
                    h: Some(
                        curve
                            .h
                            .coeffs
                            .iter()
                            .map(|c| format!("{:x}", c.to_biguint()))
                            .collect::<Vec<_>>()
                            .join(","),
                    ),
                },
                p_point: None,
                q_point: None,
                secret: Some(args.secret.to_string()),
                order: None,
                fb_size: 0,
                max_trials: 200_000,
                seed: 1,
                extra_relations: 8,
                bsgs_max_steps: 200_000,
                factors: None,
                trial_factor_bound: 1 << 16,
                small_prime_bound: 1 << 20,
                threads: 1,
            };
            run_binary(&run_args)
        }
        other => Err(format!("unknown --field '{other}'; use prime|binary")),
    }
}

// ── Parsing / formatting ───────────────────────────────────────────────

fn parse_prime_curve(args: &CurveArgs) -> Result<HyperellipticCurveP, String> {
    let p = require_int(&args.p, "--p")?;
    let f = parse_list(&args.f)?;
    let f = FpPoly::from_coeffs(f, p.clone());
    HyperellipticCurveP::try_new(p, f, args.genus)
        .map_err(|e| format!("invalid prime hyperelliptic curve: {e}"))
}

fn parse_binary_curve(args: &CurveArgs) -> Result<HyperellipticCurve, String> {
    let m = args.m.ok_or("--m is required for --field binary")?;
    let modulus = parse_list(
        args.modulus
            .as_deref()
            .ok_or("--modulus is required for --field binary")?,
    )?;
    let low_terms: Vec<u32> = modulus
        .iter()
        .map(|v| {
            if v.is_zero() {
                return Ok(0u32);
            }
            v.to_u32_digits()
                .first()
                .copied()
                .filter(|_| v.to_u32_digits().len() <= 1)
                .ok_or("--modulus terms must fit in u32")
        })
        .collect::<Result<_, _>>()?;
    let irr = IrreduciblePoly {
        degree: m,
        low_terms,
    };
    let h = parse_list(
        args.h
            .as_deref()
            .ok_or("--h is required for --field binary")?,
    )?;
    let h = F2mPoly::from_coeffs(
        h.into_iter()
            .map(|v| F2mElement::from_biguint(&v, m))
            .collect(),
        m,
    );
    let f = parse_list(&args.f)?;
    let f = F2mPoly::from_coeffs(
        f.into_iter()
            .map(|v| F2mElement::from_biguint(&v, m))
            .collect(),
        m,
    );
    HyperellipticCurve::try_new(m, irr, h, f, args.genus)
        .map_err(|e| format!("invalid binary hyperelliptic curve: {e}"))
}

fn prime_curve_json(curve: &HyperellipticCurveP) -> Value {
    json!({
        "field": "prime",
        "p": curve.p.to_string(),
        "genus": curve.genus,
        "f": curve.f.coeffs.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
    })
}

fn binary_curve_json(curve: &HyperellipticCurve) -> Value {
    let hex = |p: &F2mPoly| {
        p.coeffs
            .iter()
            .map(|c| format!("{:x}", c.to_biguint()))
            .collect::<Vec<_>>()
    };
    json!({
        "field": "binary",
        "m": curve.m,
        "modulus": curve.irr.low_terms,
        "genus": curve.genus,
        "h": hex(&curve.h),
        "f": hex(&curve.f),
    })
}

fn parse_int(s: &str) -> Result<BigUint, String> {
    let s = s.trim();
    let (digits, radix) = if let Some(rest) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X"))
    {
        (rest, 16)
    } else {
        (s, 10)
    };
    BigUint::parse_bytes(digits.as_bytes(), radix).ok_or_else(|| format!("invalid integer '{s}'"))
}

fn parse_list(s: &str) -> Result<Vec<BigUint>, String> {
    s.split(',')
        .map(|part| parse_int(part))
        .collect::<Result<Vec<_>, _>>()
}

fn parse_point(s: &str) -> Result<(BigUint, BigUint), String> {
    let (x, y) = s
        .split_once(',')
        .ok_or_else(|| format!("expected X,Y in '{s}'"))?;
    Ok((parse_int(x)?, parse_int(y)?))
}

fn require_int(s: &Option<String>, name: &str) -> Result<BigUint, String> {
    parse_int(s.as_deref().ok_or_else(|| format!("{name} is required"))?)
}

fn parse_factors(args: &RunArgs) -> Result<Option<Vec<(BigUint, u32)>>, String> {
    let Some(raw) = &args.factors else {
        return Ok(None);
    };
    let mut out = Vec::new();
    for part in raw.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (q, e) = match part.split_once('^') {
            Some((q, e)) => (
                parse_int(q)?,
                e.parse::<u32>()
                    .map_err(|_| format!("bad exponent '{e}'"))?,
            ),
            None => (parse_int(part)?, 1),
        };
        out.push((q, e));
    }
    if out.is_empty() {
        return Err("--factors must name at least one prime power".into());
    }
    Ok(Some(out))
}

fn big_to_f64(n: &BigUint) -> f64 {
    n.to_u64_digits()
        .iter()
        .rev()
        .fold(0f64, |acc, &d| acc * 2f64.powi(64) + d as f64)
}

/// `log2(n)` without overflowing for `n > 2^1024`.
fn big_log2(n: &BigUint) -> f64 {
    let bits = n.bits();
    if bits == 0 {
        return f64::NEG_INFINITY;
    }
    let shift = bits.saturating_sub(53) as usize;
    let top = n >> shift;
    shift as f64 + big_to_f64(&top).log2()
}

fn logsumexp2(a: f64, b: f64) -> f64 {
    let m = a.max(b);
    if m == f64::NEG_INFINITY {
        return m;
    }
    m + ((a - m).exp() + (b - m).exp()).ln()
}

fn logsumexp3(a: f64, b: f64, c: f64) -> f64 {
    logsumexp2(logsumexp2(a, b), c)
}

fn curve_is_identity(d: &MumfordDivisorP) -> bool {
    d.is_identity()
}

// Keep the unused imports explicit: these are the engine entry points this
// binary is a front door for, and a later subcommand will call them.
#[allow(dead_code)]
fn _engine_names(
    _a: Option<HecFactorBase>,
    _b: Option<BhecFactorBase>,
    _c: Option<fn(&BigUint, u64, u64) -> (Vec<(BigUint, u32)>, bool)>,
) {
    let _ = factor_order;
}
