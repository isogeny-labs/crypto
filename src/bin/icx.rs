//! # `icx` — index calculus across every standardized curve.
//!
//! One command that accepts any standardized elliptic curve by name and, per
//! `docs/ic/ENGINE_AUDIT_AND_DESIGN.md`:
//!
//! - `icx list` — enumerate the catalog (filter by `--family`);
//! - `icx inspect <curve>` — verify the curve's parameters;
//! - `icx estimate <curve>` — the generic-attack cost and the honest IC
//!   picture for this size, with heuristics named;
//! - `icx <curve>` — shorthand for `inspect`.
//!
//! `icx run` (execute the pipeline on an in-envelope curve or a same-family
//! analogue) is layered on top of this in the same crate and lands with the
//! solver wiring; this binary is the catalog-and-estimate front door and never
//! claims to solve what it has not.
//!
//! Every subcommand supports `--json` for a machine-readable report carrying a
//! provenance block (binary hash, git commit, host, dispatched field backend).

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use clap::{Args, Parser, Subcommand};
use serde_json::{json, Value};

#[path = "icx/parameters.rs"]
mod parameters;
#[path = "icx/state.rs"]
mod state;
use state::RunState;

use crypto_lib::cryptanalysis::curve_catalog::{self, CatalogCurve, Family};
use crypto_lib::cryptanalysis::ic_engine;

#[derive(Parser)]
#[command(
    name = "icx",
    version,
    about = "Index calculus across every standardized elliptic curve",
    long_about = "Inspect, estimate and (where feasible) run index calculus on \
                  any standardized curve — prime, binary, Koblitz, extension \
                  field, or characteristic three — selected by name. See \
                  `icx list` for the catalog and \
                  docs/ic/ENGINE_AUDIT_AND_DESIGN.md for the honest scope."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Action>,
    /// Shorthand: inspect this curve. Use `icx list` for names.
    #[arg(value_name = "CURVE")]
    curve: Option<String>,
    /// Emit a machine-readable JSON report.
    #[arg(long, global = true)]
    json: bool,
    /// Stream stage progress on stderr, including with --json.
    #[arg(long, global = true)]
    verbose: bool,
    /// Seconds between live run heartbeats (implies --verbose).
    #[arg(long, global = true, value_parser = clap::value_parser!(u64).range(1..))]
    progress_every: Option<u64>,
}

#[derive(Subcommand)]
enum Action {
    /// List catalog curves, optionally filtered by field family.
    List(ListArgs),
    /// Verify a curve's parameters (generator on curve, [n]G = O, Hasse).
    Inspect(CurveArg),
    /// Check public point sums and S3 on the named curve's exact parameters.
    Validate(CurveArg),
    /// Report the generic-attack cost and the IC picture for this curve.
    Estimate(EstimateArgs),
    /// Run the index-calculus pipeline on the curve (or a same-family analogue).
    Run(RunArgs),
    /// Solve a quadratic Boolean system with the FES kernel (CPU or GPU worker).
    Fes(FesArgs),
}

#[derive(Args)]
struct FesArgs {
    /// Variables in the generated quadratic system.
    #[arg(long, default_value_t = 18)]
    n: usize,
    /// Equations in the generated quadratic system.
    #[arg(long, default_value_t = 20)]
    m: usize,
    #[arg(long, default_value_t = 1)]
    seed: u64,
    /// Backend: `cpu` (in-process), `gpu` (an external CUDA/Metal/host worker),
    /// or `auto` (a discovered worker if present, else cpu).
    #[arg(long, default_value = "auto")]
    backend: String,
    /// Explicit worker executable (overrides discovery); it must speak the
    /// gpu/fes file contract.
    #[arg(long)]
    worker: Option<String>,
}

#[derive(Args)]
struct RunArgs {
    /// Curve name or alias.
    curve: String,
    /// Require the actual named curve; fail if only an analogue is available.
    #[arg(long)]
    require_named_curve: bool,
    /// Factor base plugin: `name` or `name:k=v,...` (default per family).
    #[arg(long)]
    factor_base: Option<String>,
    /// Decomposition oracle (subtract | mitm | mitm-frobenius | descent-algebraic).
    #[arg(long)]
    oracle: Option<String>,
    /// Polynomial solver for descent-algebraic (f4-f2 | matrix-f5 | xl-f2 |
    /// crossbred-f2 | fes-f2 | fes-f2-wide | sat-cdcl | buchberger-f2 | exhaustive).
    #[arg(long)]
    solver: Option<String>,
    /// Relation matrix (incremental-gauss | structured-gauss).
    #[arg(long, default_value = "incremental-gauss")]
    linalg: String,
    /// Analogue field degree for binary/Koblitz families.
    #[arg(long)]
    degree: Option<u32>,
    /// Analogue prime size in bits (7|10|12|14|16|18|20).
    #[arg(long)]
    bits: Option<u32>,
    #[arg(long, default_value_t = 20_260_922)]
    seed: u64,
    #[arg(long, default_value_t = 2_000_000)]
    max_trials: u64,
    #[arg(long, default_value_t = 1)]
    repeats: usize,
    /// Counted Pollard-rho reference runs to average (0 to skip).
    #[arg(long, default_value_t = 8)]
    rho_runs: usize,
    /// End-to-end envelope in subgroup-order bits.
    #[arg(long, default_value_t = ic_engine::ATTACK_ENVELOPE_BITS)]
    envelope: u64,
    /// Per-solver-call budget in seconds (0 for none).
    #[arg(long, default_value_t = 60)]
    solver_budget_seconds: u64,
    /// Dedicated directory for completed-repeat checkpoints and reports.
    #[arg(long)]
    state_dir: Option<PathBuf>,
    /// Continue saved repeats from --state-dir or restore them from S3.
    #[arg(long)]
    resume: bool,
    /// Mirror the state directory to this S3 prefix with the AWS CLI.
    #[arg(long)]
    s3_uri: Option<String>,
}

#[derive(Args)]
struct ListArgs {
    /// Restrict to one family: prime | binary | koblitz | extension | char3.
    #[arg(long)]
    family: Option<String>,
}

#[derive(Args)]
struct CurveArg {
    /// Curve name or alias.
    curve: String,
}

#[derive(Args)]
struct EstimateArgs {
    /// Curve name or alias.
    curve: String,
    /// End-to-end envelope in subgroup-order bits (default: demonstrated max).
    #[arg(long, default_value_t = ic_engine::ATTACK_ENVELOPE_BITS)]
    envelope: u64,
}

fn provenance() -> Value {
    json!({
        "version": env!("CARGO_PKG_VERSION"),
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "field_backend": field_backend_label(),
    })
}

/// Which carry-less-multiply backend this build will use at runtime, for the
/// provenance block. (The dispatch itself lands with the field-backend work;
/// this reports what the host supports.)
fn field_backend_label() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    {
        if std::arch::is_x86_feature_detected!("vpclmulqdq") {
            return "x86_64:vpclmulqdq";
        }
        if std::arch::is_x86_feature_detected!("pclmulqdq") {
            return "x86_64:pclmulqdq";
        }
        return "x86_64:portable";
    }
    #[cfg(target_arch = "aarch64")]
    {
        if std::arch::is_aarch64_feature_detected!("aes") {
            return "aarch64:pmull";
        }
        return "aarch64:portable";
    }
    #[allow(unreachable_code)]
    "portable"
}

fn family_of(curve: &CatalogCurve) -> &'static str {
    curve.family.tag()
}

fn list(args: &ListArgs) -> Result<Value, String> {
    let filter = match &args.family {
        Some(f) => Some(Family::parse(f).ok_or_else(|| {
            format!("unknown family '{f}'; use prime|binary|koblitz|extension|char3")
        })?),
        None => None,
    };
    let mut rows = Vec::new();
    for c in curve_catalog::all() {
        if let Some(f) = filter {
            if c.family != f {
                continue;
            }
        }
        let field = c.field();
        let est = ic_engine::estimate(&c, ic_engine::ATTACK_ENVELOPE_BITS);
        rows.push(json!({
            "name": c.name,
            "aliases": c.aliases,
            "family": family_of(&c),
            "standard": c.standard,
            "field": field.label,
            "field_bits": field.bits,
            "order_bits": c.order_bits(),
            "rho_security_bits": round1(c.rho_security_bits()),
            "exact_parameters": parameters::exact_parameters(&c),
            "ic_relevant": est.ic_relevant,
            "regime": est.regime.tag(),
        }));
    }
    Ok(json!({
        "schema_version": 1,
        "operation": "list",
        "status": "complete",
        "count": rows.len(),
        "curves": rows,
    }))
}

fn inspect(name: &str) -> Result<Value, String> {
    let c = curve_catalog::by_name(name)
        .ok_or_else(|| format!("unknown curve '{name}'; try `icx list`"))?;
    let field = c.field();
    let checks = c.verify();
    let all_passed = checks.iter().all(|k| k.passed);
    let checks_json: Vec<Value> = checks
        .iter()
        .map(|k| json!({"name": k.name, "status": if k.passed {"pass"} else {"fail"}, "detail": k.detail}))
        .collect();
    Ok(json!({
        "schema_version": 1,
        "operation": "inspect",
        "status": if all_passed {"checks_passed"} else {"checks_failed"},
        "curve": {
            "name": c.name,
            "aliases": c.aliases,
            "family": family_of(&c),
            "standard": c.standard,
            "field": {"kind": field.kind, "label": field.label, "bits": field.bits},
            "order_bits": c.order_bits(),
            "subgroup_order": c.subgroup_order().to_string(),
            "cofactor": c.cofactor().to_string(),
            "group_order": c.group_order().to_string(),
            "rho_security_bits": round1(c.rho_security_bits()),
            "exact_parameters": parameters::exact_parameters(&c),
        },
        "checks": checks_json,
        "verified": all_passed,
    }))
}

fn estimate(args: &EstimateArgs) -> Result<Value, String> {
    let c = curve_catalog::by_name(&args.curve)
        .ok_or_else(|| format!("unknown curve '{}'; try `icx list`", args.curve))?;
    let est = ic_engine::estimate(&c, args.envelope);
    Ok(json!({
        "schema_version": 1,
        "operation": "estimate",
        "status": "complete",
        "curve": est.curve,
        "family": est.family,
        "field": est.field_label,
        "field_bits": est.field_bits,
        "order_bits": est.order_bits,
        "generic_attack": {
            "algorithm": "pollard-rho",
            "log2_operations": round2(est.rho_log2_ops),
            "security_bits": round1(est.rho_security_bits),
        },
        "index_calculus": {
            "relevant": est.ic_relevant,
            "regime": est.regime.tag(),
        },
        "heuristic_index_calculus": est.heuristic.as_ref().map(|h| heuristic_json(h, est.rho_log2_ops)),
        "notes": est.notes,
    }))
}

/// The per-`ω` heuristic table: a model on the first-fall-degree
/// assumption, never a measurement.
fn heuristic_json(h: &ic_engine::HeuristicIc, rho_log2_ops: f64) -> Value {
    let n = h.field_degree as f64;
    let rows: Vec<Value> = h
        .rows
        .iter()
        .map(|r| {
            json!({
                "omega_bound": r.bound.id,
                "omega": r.bound.value,
                "relation": r.bound.relation(),
                "construction": r.bound.construction.tag(),
                "proof": r.bound.proof.tag(),
                "characteristic_2": r.applicability.tag(),
                "log2_cost": round2(r.log2_cost),
                "below_generic": r.log2_cost < n / 2.0,
                "log2_cost_minus_rho": round2(r.log2_cost - rho_log2_ops),
                "turning_point_n": r.turning_point,
                "exponent_ratio_to_best_established": (r.exponent_ratio_to_best_established * 1e4).round() / 1e4,
                "source": r.bound.source,
            })
        })
        .collect();
    json!({
        "kind": "model",
        "heuristic": h.heuristic.id(),
        "formula": h.heuristic.formula(),
        "assumption": "first-fall-degree assumption (D_reg ~ D_ff), contested by Kosters-Yeo 2015 and Huang-Kosters-Yeo 2015",
        "field_degree": h.field_degree,
        "generic_log2": n / 2.0,
        "rows": rows,
    })
}

fn round1(x: f64) -> f64 {
    (x * 10.0).round() / 10.0
}
fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn display(report: &Value) {
    match report["operation"].as_str() {
        Some("list") => {
            // Column header: literal names are the clearest form for a table
            // header, so print_literal is not helpful here.
            #[allow(clippy::print_literal)]
            {
                println!(
                    "{:<22} {:<10} {:<9} {:>10} {:>9}  {}",
                    "curve", "family", "field", "order_bits", "rho_bits", "regime"
                );
            }
            for c in report["curves"].as_array().into_iter().flatten() {
                println!(
                    "{:<22} {:<10} {:<9} {:>10} {:>9}  {}",
                    c["name"].as_str().unwrap_or("?"),
                    c["family"].as_str().unwrap_or("?"),
                    c["field"].as_str().unwrap_or("?"),
                    c["order_bits"].as_u64().unwrap_or(0),
                    format!("{:.1}", c["rho_security_bits"].as_f64().unwrap_or(0.0)),
                    c["regime"].as_str().unwrap_or("?"),
                );
            }
            println!("\n{} curves.", report["count"]);
        }
        Some("inspect") => {
            let cu = &report["curve"];
            println!(
                "Curve: {} [{}]",
                cu["name"].as_str().unwrap_or("?"),
                cu["family"].as_str().unwrap_or("?")
            );
            println!("Standard: {}", cu["standard"].as_str().unwrap_or("?"));
            println!(
                "Field: {} ({} bits); subgroup order {} bits; cofactor {}",
                cu["field"]["label"].as_str().unwrap_or("?"),
                cu["field"]["bits"],
                cu["order_bits"],
                cu["cofactor"].as_str().unwrap_or("?"),
            );
            println!("Generic (rho) security: {} bits", cu["rho_security_bits"]);
            for k in report["checks"].as_array().into_iter().flatten() {
                println!(
                    "  {:<22} {:<5} {}",
                    k["name"].as_str().unwrap_or("?"),
                    k["status"].as_str().unwrap_or("?"),
                    k["detail"].as_str().unwrap_or("")
                );
            }
            println!(
                "Verified: {}",
                if report["verified"] == true {
                    "yes"
                } else {
                    "NO"
                }
            );
        }
        Some("estimate") => {
            println!(
                "Curve: {} [{}]  field {} ({} bits)  order {} bits",
                report["curve"].as_str().unwrap_or("?"),
                report["family"].as_str().unwrap_or("?"),
                report["field"].as_str().unwrap_or("?"),
                report["field_bits"],
                report["order_bits"],
            );
            let ga = &report["generic_attack"];
            println!(
                "Generic attack (rho): ~2^{} operations, {} security bits",
                ga["log2_operations"], ga["security_bits"]
            );
            println!(
                "Index calculus: relevant={}  regime={}",
                report["index_calculus"]["relevant"],
                report["index_calculus"]["regime"].as_str().unwrap_or("?"),
            );
            let h = &report["heuristic_index_calculus"];
            if !h.is_null() {
                println!(
                    "Heuristic IC ({}, a model, not a measurement): {}",
                    h["heuristic"].as_str().unwrap_or("?"),
                    h["formula"].as_str().unwrap_or("?"),
                );
                println!(
                    "  {:<24} {:>10} {:>15} {:>13} {:>9} {:>8}",
                    "omega bound", "omega", "construction", "char 2", "log2 T", "turn n"
                );
                for r in h["rows"].as_array().into_iter().flatten() {
                    println!(
                        "  {:<24} {:>2}{:>8.4} {:>15} {:>13} {:>9.2} {:>8}",
                        r["omega_bound"].as_str().unwrap_or("?"),
                        r["relation"].as_str().unwrap_or("?"),
                        r["omega"].as_f64().unwrap_or(f64::NAN),
                        r["construction"].as_str().unwrap_or("?"),
                        r["characteristic_2"].as_str().unwrap_or("?"),
                        r["log2_cost"].as_f64().unwrap_or(f64::NAN),
                        r["turning_point_n"].as_u64().unwrap_or(0),
                    );
                }
                println!("  generic 2^(n/2) = 2^{}", h["generic_log2"]);
            }
            for n in report["notes"].as_array().into_iter().flatten() {
                println!("  - {}", n.as_str().unwrap_or(""));
            }
        }
        Some("run") => {
            let r = &report["result"];
            let an = &r["analogue"];
            println!(
                "Run: {} [{}]  regime={}  ic_relevant={}",
                r["curve"].as_str().unwrap_or("?"),
                an["family"].as_str().unwrap_or("?"),
                r["regime"].as_str().unwrap_or("?"),
                r["ic_relevant"],
            );
            println!(
                "Instance run: {} (order {} bits){}",
                an["instance"].as_str().unwrap_or("?"),
                an["run_order_bits"],
                if an["is_named_curve"] == true {
                    " — the named curve"
                } else {
                    " — scaled analogue"
                },
            );
            println!("Config: {}", r["config_label"].as_str().unwrap_or("?"));
            for (i, rep) in r["reports"].as_array().into_iter().flatten().enumerate() {
                let s = rep["s"].as_f64().unwrap_or(0.0);
                let sr = rep.get("s_over_rho").and_then(|v| v.as_f64());
                println!(
                    "  config {}: verified={}  S={:.3e}{}",
                    i + 1,
                    rep["verified"],
                    s,
                    sr.map(|x| format!("  ({x:.2}x rho)")).unwrap_or_default(),
                );
            }
            if let Some(rho) = r.get("rho").filter(|v| !v.is_null()) {
                println!(
                    "Rho reference: {} runs, mean S={:.3e}, all verified={}",
                    rho["runs"],
                    rho["mean_s"].as_f64().unwrap_or(0.0),
                    rho["all_verified"],
                );
            }
            if let Some(note) = r["extrapolation_note"].as_str() {
                println!("\nScope: {note}");
            }
        }
        Some("fes") => {
            let sy = &report["system"];
            println!(
                "FES: {}-var, {}-eq quadratic system (seed {})",
                sy["n"], sy["m"], sy["seed"]
            );
            println!("Backend: {}", report["backend"].as_str().unwrap_or("?"));
            println!(
                "Solutions: {} verified of {} proposed",
                report["verified"], report["proposed"]
            );
            println!(
                "Planted solution {}: {}",
                report["planted"],
                if report["planted_found"] == true {
                    "found"
                } else {
                    "NOT found"
                }
            );
        }
        Some("error") => {
            eprintln!("icx: {}", report["message"].as_str().unwrap_or("failed"));
        }
        _ => {
            println!("{}", serde_json::to_string_pretty(report).unwrap());
        }
    }
}

struct Heartbeat {
    stop: mpsc::Sender<()>,
    worker: Option<JoinHandle<()>>,
}

impl Heartbeat {
    fn start(seconds: u64) -> Self {
        let (stop, receiver) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let started = Instant::now();
            while receiver
                .recv_timeout(Duration::from_secs(seconds))
                .is_err_and(|e| e == mpsc::RecvTimeoutError::Timeout)
            {
                eprintln!(
                    "Info:Index calculus: running  elapsed {:.1}s",
                    started.elapsed().as_secs_f64()
                );
            }
        });
        Self {
            stop,
            worker: Some(worker),
        }
    }
}

impl Drop for Heartbeat {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn run_cmd(
    args: &RunArgs,
    json: bool,
    verbose: bool,
    progress_every: Option<u64>,
) -> Result<Value, String> {
    use crypto_lib::cryptanalysis::ic_framework::RunReport;
    use crypto_lib::cryptanalysis::ic_progress::ProgressReporter;
    use crypto_lib::cryptanalysis::ic_run::{run_curve_from_repeat, RhoRef, RunConfig};

    let curve = curve_catalog::by_name(&args.curve)
        .ok_or_else(|| format!("unknown curve '{}'; try `icx list`", args.curve))?;
    if args.require_named_curve {
        return Err(format!(
            "full-parameter IC solving is unavailable for {}; the current run path uses \
             an analogue. Use `icx validate {}` for bounded checks on the actual parameters",
            curve.name, curve.name
        ));
    }
    let cfg = RunConfig {
        factor_base: args.factor_base.clone(),
        oracle: args.oracle.clone(),
        solver: args.solver.clone(),
        linalg: args.linalg.clone(),
        degree: args.degree,
        bits: args.bits,
        seed: args.seed,
        max_trials: args.max_trials,
        repeats: args.repeats,
        rho_runs: args.rho_runs,
        envelope_bits: args.envelope,
        solver_budget_seconds: args.solver_budget_seconds,
    };
    if args.repeats == 0 {
        return Err("repeats must be greater than zero".into());
    }
    if args.state_dir.is_none() && (args.resume || args.s3_uri.is_some()) {
        return Err("--resume and --s3-uri require --state-dir".into());
    }
    // Progress uses stderr. Keep stdout as one final JSON document even when
    // requested stage lines are live in JSON report mode.
    let verbose = verbose || progress_every.is_some();
    let manifest = json!({
        "schema_version": 1,
        "curve": curve.name,
        "factor_base": args.factor_base,
        "oracle": args.oracle,
        "solver": args.solver,
        "linalg": args.linalg,
        "degree": args.degree,
        "bits": args.bits,
        "seed": args.seed,
        "max_trials": args.max_trials,
        "repeats": args.repeats,
        "rho_runs": args.rho_runs,
        "envelope": args.envelope,
        "solver_budget_seconds": args.solver_budget_seconds,
        "binary_blake3": if args.state_dir.is_some() {
            Some(state::binary_digest()?)
        } else {
            None
        },
    });
    let mut state = args
        .state_dir
        .as_ref()
        .map(|dir| {
            RunState::prepare(
                dir.clone(),
                args.resume,
                args.s3_uri.clone(),
                verbose,
                manifest,
            )
        })
        .transpose()?;
    let completed = state.as_ref().map_or(0, RunState::completed);
    if completed > cfg.repeats {
        return Err("ICX checkpoint has more repeats than requested".into());
    }
    let current_host = state::host_name();
    if let Some(saved) = &state {
        if completed > 0 && cfg.rho_runs > 0 {
            if saved.snapshot.rho.is_none() {
                return Err("ICX checkpoint is missing its rho reference".into());
            }
            if saved.snapshot.origin_host != current_host {
                return Err(
                    "rho reference is host-specific; cross-host resume needs --rho-runs 0".into(),
                );
            }
        }
    }
    let prior_rho = state.as_ref().and_then(|s| s.snapshot.rho.clone());
    let prior_reports = state
        .as_ref()
        .map(|s| s.snapshot.reports.clone())
        .unwrap_or_default();
    let mut progress = ProgressReporter::new(false, json && !verbose);
    if completed > 0 {
        progress.info(&format!("resuming after {completed} complete repeat(s)"));
    }
    let heartbeat = verbose.then(|| Heartbeat::start(progress_every.unwrap_or(5)));
    let managed = state.is_some();
    let mut on_repeat = |rep: usize, report: &RunReport, rho: Option<&RhoRef>, label: &str| {
        state
            .as_mut()
            .expect("managed repeat callback")
            .repeat(rep, report, rho, label)
    };
    let result = run_curve_from_repeat(
        &curve,
        &cfg,
        &mut progress,
        completed,
        prior_rho,
        managed.then_some(&mut on_repeat),
    )?;
    drop(on_repeat);
    drop(heartbeat);
    let mut result_value = serde_json::to_value(&result).map_err(|e| e.to_string())?;
    let mut reports = prior_reports;
    let fresh = result_value["reports"]
        .as_array()
        .ok_or("invalid ICX report list")?;
    reports.extend(fresh.iter().cloned());
    if fresh.is_empty() {
        if let Some(saved) = &state {
            result_value["config_label"] = json!(saved.snapshot.config_label);
        }
    }
    let verified_count = reports.iter().filter(|r| r["verified"] == true).count();
    let verified_any = verified_count > 0;
    result_value["reports"] = Value::Array(reports);
    if !json {
        progress.summary(&format!(
            "{} — {} of {} configuration(s) verified",
            result.curve, verified_count, cfg.repeats
        ));
    }
    let mut envelope = json!({
        "schema_version": 1,
        "operation": "run",
        "status": if verified_any { "complete" } else { "incomplete" },
        "verified": verified_any,
        "result": result_value,
    });
    if args.resume {
        if let Some(saved) = &state {
            envelope["resume"] = json!({
                "completed_before_restart": completed,
                "origin_host": saved.snapshot.origin_host,
                "current_host": current_host,
                "cross_host": saved.snapshot.origin_host != current_host,
                "accounting": "per-repeat reports; restart setup and storage are excluded from those costs"
            });
        }
    }
    envelope["software"] = provenance();
    if let Some(saved) = state.as_mut() {
        saved.finish(&envelope)?;
    }
    Ok(envelope)
}

fn fes_cmd(args: &FesArgs) -> Result<Value, String> {
    use crypto_lib::cryptanalysis::fes_gpu::{self, FesBackend};

    let (sys, planted) = fes_gpu::random_system(args.n, args.m, args.seed);
    let backend = match (args.worker.as_deref(), args.backend.as_str()) {
        (Some(path), _) => FesBackend::Worker(std::path::PathBuf::from(path)),
        (None, "cpu") => FesBackend::Cpu,
        (None, "auto") => FesBackend::Auto,
        (None, "gpu") => match fes_gpu::discover_worker() {
            Some(w) => FesBackend::Worker(w),
            None => {
                return Err(
                    "no FES worker found; build one with `make -C gpu/fes host-worker` \
                            (or cuda / metal-worker) or set ICX_FES_WORKER, or use --backend cpu"
                        .into(),
                )
            }
        },
        (None, other) => return Err(format!("unknown backend '{other}'; use cpu|gpu|auto")),
    };
    let res = fes_gpu::solve(&sys, &backend)?;
    let planted_found = res.solutions.contains(&planted);
    Ok(json!({
        "schema_version": 1,
        "operation": "fes",
        "status": "complete",
        "system": {"n": args.n, "m": args.m, "seed": args.seed},
        "backend": res.backend,
        "proposed": res.proposed,
        "verified": res.verified,
        "solutions": res.solutions.len(),
        "planted": planted,
        "planted_found": planted_found,
    }))
}

fn execute(cli: &Cli) -> Result<Value, String> {
    match &cli.command {
        Some(Action::List(a)) => list(a),
        Some(Action::Inspect(a)) => inspect(&a.curve),
        Some(Action::Validate(a)) => {
            let curve = curve_catalog::by_name(&a.curve)
                .ok_or_else(|| format!("unknown curve '{}'; try `icx list`", a.curve))?;
            parameters::validate(&curve)
        }
        Some(Action::Estimate(a)) => estimate(a),
        Some(Action::Run(a)) => run_cmd(a, cli.json, cli.verbose, cli.progress_every),
        Some(Action::Fes(a)) => fes_cmd(a),
        None => {
            let name = cli
                .curve
                .as_deref()
                .ok_or("supply a curve name or a subcommand; see `icx --help`")?;
            inspect(name)
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut report = match execute(&cli) {
        Ok(v) => v,
        Err(message) => json!({
            "schema_version": 1, "operation": "error", "status": "error", "message": message
        }),
    };
    if report.get("operation").is_some() {
        report["software"] = provenance();
    }
    let ok = matches!(
        report["status"].as_str(),
        Some("complete" | "checks_passed")
    );
    if cli.json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        display(&report);
    }
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
