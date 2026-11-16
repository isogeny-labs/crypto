//! Actual-root comparison worker. Execution policy is ROOT_WORKER_CONTRACT.md.
//! Single-call clocks are diagnostics, never a paired speedup conclusion.
use certified_relations::System;
use crypto_lib::cryptanalysis::f4_fp::{self, certified, normalise, F4Options, Ordering, Poly};
use serde_json::{json, Value};
use std::io::{self, Read};
use std::time::{Duration, Instant};

const MAX_INPUT_BYTES: u64 = 4 << 20;

#[derive(Clone, Copy)]
enum Engine {
    F4,
    Certified,
    Compact,
    CompactAllVertices,
}
struct Options {
    engine: Engine,
    degree_cap: u32,
    deadline_ms: u64,
}
fn options() -> Result<Options, String> {
    let mut args = std::env::args().skip(1);
    let engine = match args.next().as_deref() {
        Some("f4") => Engine::F4,
        Some("certified") => Engine::Certified,
        Some("compact") => Engine::Compact,
        Some("compact-all-vertices") => Engine::CompactAllVertices,
        _ => return Err("usage: native-root-worker f4|certified|compact|compact-all-vertices [--degree-cap 12] [--deadline-ms 10000]; System JSON on stdin".into()),
    };
    let mut opts = Options {
        engine,
        degree_cap: 12,
        deadline_ms: 10000,
    };
    while let Some(arg) = args.next() {
        let value = args.next().ok_or("missing option value")?;
        match arg.as_str() {
            "--degree-cap" => opts.degree_cap = value.parse().map_err(|_| "invalid degree cap")?,
            "--deadline-ms" => opts.deadline_ms = value.parse().map_err(|_| "invalid deadline")?,
            _ => return Err(format!("unknown option {arg}")),
        }
    }
    if opts.degree_cap > 32 || opts.deadline_ms == 0 || opts.deadline_ms > 30000 {
        return Err("degree cap must be <=32 and F4 deadline 1..30000 ms".into());
    }
    Ok(opts)
}
fn canonical_basis(basis: &mut Vec<Poly>, prime: u64) {
    for f in basis.iter_mut() {
        *f = normalise(f, prime, Ordering::Grevlex);
    }
    basis.sort();
    basis.dedup();
}
fn run(start: Instant) -> Result<Value, String> {
    let opts = options()?;
    let mut bytes = Vec::new();
    io::stdin()
        .lock()
        .take(MAX_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_INPUT_BYTES {
        return Err("input exceeds 4 MiB".into());
    }
    let system: System = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let n = system.variables.len();
    if !(3..=certified_relations::quotient::MAX_VARIABLES).contains(&n)
        || system.polynomials.is_empty()
        || system.polynomials.len() > 2000
        || system.polynomials.iter().any(|f| {
            f.len() > 5000
                || f.iter().any(|t| {
                    t.exponents.len() != n
                        || t.exponents
                            .iter()
                            .try_fold(0usize, |s, e| s.checked_add(*e))
                            .is_none_or(|d| d > 16)
                })
        })
    {
        return Err("input shape/term/degree budget".into());
    }
    let decoded = system.decode()?;
    let p = system.prime;
    let input: Vec<Poly> = decoded
        .into_iter()
        .map(|f| {
            let f: Poly = f
                .into_iter()
                .map(|(m, c)| (m.into_iter().map(|e| e as u32).collect(), c))
                .collect();
            normalise(&f, p, Ordering::Grevlex)
        })
        .collect();
    let admission_ns = start.elapsed().as_nanos();
    let engine = match opts.engine {
        Engine::F4 => "f4",
        Engine::Certified => "certified",
        Engine::Compact => "compact",
        Engine::CompactAllVertices => "compact-all-vertices",
    };
    if matches!(opts.engine, Engine::F4) && p >= 1u64 << 32 {
        return Ok(json!({"status":"unsupported", "engine":engine, "prime":p,
            "variables":n, "basis":null, "observed_solving_degree":null,
            "reason":"regular F4 control is documented only for p < 2^32"}));
    }
    let solve_start = Instant::now();
    let (mut basis, complete, f4_observations, quotient_info, observed_degree) = match opts.engine {
        Engine::F4 => {
            let mut options = F4Options::new(Ordering::Grevlex, opts.degree_cap);
            options.deadline = Some(Instant::now() + Duration::from_millis(opts.deadline_ms));
            let report = f4_fp::f4(&input, n, p, &options);
            let complete = !report.timed_out
                && report.pairs_above_bound == 0
                && report.staircase_at_stop.is_none()
                && !report.trace_mismatch;
            let degree = if complete {
                json!(report.solving_degree_max)
            } else {
                Value::Null
            };
            let observations = json!({
                "highest_processed_pair_degree":report.degree_reached,
                "highest_productive_degree_raw":report.solving_degree_max,
                "timed_out":report.timed_out,
                "pairs_above_bound":report.pairs_above_bound,
                "staircase_at_stop":report.staircase_at_stop,
                "trace_mismatch":report.trace_mismatch,
                "steps":report.steps,"max_rows":report.max_rows,"max_columns":report.max_cols
            });
            (report.basis, complete, observations, Value::Null, degree)
        }
        Engine::Certified => match certified::try_quotient_basis(&input, n, p) {
            Ok(report) => (
                report.basis,
                true,
                Value::Null,
                serde_json::to_value(report.quotient).map_err(|e| e.to_string())?,
                Value::Null,
            ),
            Err(reason) => {
                return Ok(json!({"status":"constructor_rejected", "engine":engine,
                    "prime":p,"variables":n,"basis":null,
                    "observed_solving_degree":null,"reason":reason}));
            }
        },
        Engine::Compact | Engine::CompactAllVertices => {
            match if matches!(opts.engine, Engine::Compact) {
                certified::try_compact_quotient_basis(&input, n, p)
            } else {
                certified::try_compact_quotient_basis_all_vertices(&input, n, p)
            } {
                Ok(report) => (
                    report.basis,
                    true,
                    Value::Null,
                    serde_json::to_value(report.info).map_err(|e| e.to_string())?,
                    Value::Null,
                ),
                Err(reason) => {
                    return Ok(json!({"status":"constructor_rejected", "engine":engine,
                    "prime":p,"variables":n,"basis":null,
                    "observed_solving_degree":null,"reason":reason}));
                }
            }
        }
    };
    let construction_and_internal_checks_ns = solve_start.elapsed().as_nanos();
    let output_start = Instant::now();
    canonical_basis(&mut basis, p);
    // Partial bases are deliberately not exposed as interchangeable answers.
    let basis = if complete { json!(basis) } else { Value::Null };
    let canonical_output_ns = output_start.elapsed().as_nanos();
    Ok(
        json!({"status":if complete {"complete"} else {"incomplete"},
        "engine":engine,"prime":p,"variables":n,"basis":basis,
        "observed_solving_degree":observed_degree,
        "f4_observations":f4_observations,"quotient_info":quotient_info,
        "clock_ns":{"input_parse_admission_conversion":admission_ns,
            "construction_and_internal_checks":construction_and_internal_checks_ns,
            "canonical_output":canonical_output_ns},
        "requires_matched_external_basis_check":true,
        "clock_scope":"worker call; excludes process launch and printing; not a speed ratio"}),
    )
}
fn main() {
    let start = Instant::now();
    let mut output = match run(start) {
        Ok(value) => value,
        Err(reason) => json!({"status":"input_error","basis":null,
            "observed_solving_degree":null,"reason":reason}),
    };
    output["schema"] = json!("relation-root-worker/v1");
    output["worker_wall_before_print_ns"] = json!(start.elapsed().as_nanos());
    let success = output["status"] == "complete";
    println!("{output}");
    if !success {
        std::process::exit(2);
    }
}
