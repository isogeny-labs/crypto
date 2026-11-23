//! `ic oversample`: a paired measurement of relation oversampling in the
//! factor-base logarithm precompute.
//!
//! For each seed one relation pool is drawn with the exact collector,
//! batches and probe sequence of `ic logs`, continued past the point
//! where the coefficient rank first becomes full.  Every arm then solves
//! a prefix of that same pool: `full_rank + ceil(f · columns)` relations
//! for its oversampling fraction `f`, with the same filter (whose purge
//! may spend the surplus on the heaviest rows) and the same Krylov
//! solver.  Arms differ only in `f`, so per-seed differences isolate
//! what the surplus costs in collection against what it saves in linear
//! algebra.  An arm counts only if every column logarithm verifies in
//! the group (`[x]G == point`).
//!
//! Collection cost of a prefix is exact in trials; its seconds (and the
//! scanned summands, where the collector reports them) are pro-rated
//! within the collector's batch.  Linear algebra is reported in its own
//! counted unit (`row_ops`) and in seconds.  No unit conversion between
//! trials and row operations is applied here: that is the analysis'
//! decision (see the protocol).
use super::experiment::{
    curve, factor_base_json, ic_options, load_factor_base, materialize, sparse_options,
    validate_factor_size, with_linear_algebra, FilterMode, LinearAlgebraMode, Solver,
    SparseSolverMode, SpmvMode,
};
use clap::Args;
use crypto_lib::cryptanalysis::koblitz_factor_base_search::FactorBaseSpec;
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    collect_factor_base_log_pool, solve_factor_base_logs_from_relations, KoblitzCurve,
    LinearAlgebra, RelationPool, MAX_SUBFIELD_DEGREE,
};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::time::Instant;

/// Measure relation oversampling: collect one pool per seed past full
/// rank, solve prefixes at each surplus fraction, emit one row per
/// (seed, arm).
#[derive(Clone, Debug, Args)]
pub struct OversampleArgs {
    #[arg(long,default_value_t=9,value_parser=super::experiment::field_degree)]
    pub degree: u32,
    #[arg(long,default_value_t=0,value_parser=clap::value_parser!(u8).range(0..=255))]
    pub curve_a: u8,
    /// Degree k of the subfield the curve is defined over (q = 2^k); 1 is the Koblitz family.
    #[arg(long,default_value_t=1,value_parser=clap::value_parser!(u32).range(1..=MAX_SUBFIELD_DEGREE as i64))]
    pub subfield: u32,
    /// Coordinate of b in the subfield basis (1 for Koblitz curves); 1..2^k.
    #[arg(long,default_value_t=1,value_parser=clap::value_parser!(u64).range(1..=255))]
    pub curve_b: u64,
    /// Candidate index in the legacy degree-ord_n(2) factor-base family.
    #[arg(long, default_value_t = 0, conflicts_with = "factor_base")]
    pub factor_index: usize,
    /// Factor-base recipe saved by `ic search` (JSON); replaces --factor-index.
    #[arg(long)]
    pub factor_base: Option<PathBuf>,
    /// Factor-base points per relation (2, 3 or 4).
    #[arg(long,default_value_t=2,value_parser=clap::value_parser!(u8).range(2..=4))]
    pub summands: u8,
    /// Trial budget for the whole pool (full rank plus the largest surplus).
    #[arg(long,default_value_t=200_000,value_parser=clap::value_parser!(u32).range(1..=1_000_000))]
    pub max_trials: u32,
    #[arg(long,value_enum,default_value_t=Solver::PairTable)]
    pub solver: Solver,
    /// Base seed; seed i of --seeds is `seed + i`.
    #[arg(long, default_value_t = 0x4b_6f_62_6c_69_74_7a_00u64)]
    pub seed: u64,
    /// Number of seeds (one relation pool each).
    #[arg(long,default_value_t=3,value_parser=clap::value_parser!(u32).range(1..=10_000))]
    pub seeds: u32,
    /// Surplus fractions f: each arm solves full_rank + ceil(f · columns) relations.
    #[arg(long, value_delimiter = ',', default_value = "0,0.1,0.25,0.5")]
    pub fractions: Vec<f64>,
    /// Add an A/A control: a second, independently timed f = 0 arm.
    #[arg(long)]
    pub aa_control: bool,
    /// Add a reference arm that solves exactly the prefix `ic logs` would
    /// first attempt (the end of the batch in which rank became full).
    #[arg(long)]
    pub logs_arm: bool,
    /// Filter excess kept after the purge (rows − columns), the same in
    /// every arm; omitted keeps the filter strategy's default.
    #[arg(long)]
    pub target_excess: Option<usize>,
    /// Sparse Krylov solver used after filtering.
    #[arg(long, value_enum, default_value_t = SparseSolverMode::BlockWiedemann)]
    pub sparse_solver: SparseSolverMode,
    #[arg(long, value_enum, default_value_t = SpmvMode::Auto)]
    pub spmv: SpmvMode,
    #[arg(long, default_value_t = 0)]
    pub spmv_shards: usize,
    #[arg(long, value_enum, default_value_t = FilterMode::Cado)]
    pub filter_strategy: FilterMode,
    #[arg(long,default_value_t=4,value_parser=clap::value_parser!(u8).range(1..=64))]
    pub block_size: u8,
    /// Also write the rows here as JSON lines; an existing path is never overwritten.
    #[arg(long)]
    pub rows_out: Option<PathBuf>,
}

/// One arm of a seed: its label and surplus fraction.
#[derive(Clone, Debug, PartialEq)]
struct Arm {
    id: String,
    fraction: f64,
    role: &'static str,
    /// Solve the prefix `ic logs` would first attempt instead of a fraction.
    logs_default: bool,
}

fn arms(fractions: &[f64], aa: bool, logs: bool) -> Result<Vec<Arm>, String> {
    if fractions.is_empty() {
        return Err("--fractions needs at least one value".into());
    }
    let mut out: Vec<Arm> = Vec::new();
    for &f in fractions {
        if !f.is_finite() || !(0.0..=16.0).contains(&f) {
            return Err(format!("fraction {f} is outside [0, 16]"));
        }
        let id = format!("f{f}");
        if out.iter().any(|a| a.id == id) {
            return Err(format!("fraction {f} is listed twice"));
        }
        out.push(Arm {
            id,
            fraction: f,
            role: if f == 0.0 { "baseline" } else { "candidate" },
            logs_default: false,
        });
    }
    if aa {
        if !out.iter().any(|a| a.fraction == 0.0) {
            return Err("--aa-control repeats the f = 0 arm, so --fractions must include 0".into());
        }
        out.push(Arm {
            id: "f0_aa".into(),
            fraction: 0.0,
            role: "control_aa",
            logs_default: false,
        });
    }
    if logs {
        out.push(Arm {
            id: "logs_default".into(),
            fraction: f64::NAN,
            role: "reference",
            logs_default: true,
        });
    }
    Ok(out)
}

/// The arm order for seed index `i`: the arm list rotated by `i`.
fn rotated(arms: &[Arm], i: usize) -> Vec<Arm> {
    let mut v = arms.to_vec();
    if !v.is_empty() {
        let k = i % v.len();
        v.rotate_left(k);
    }
    v
}

fn spec_of(args: &OversampleArgs) -> Result<FactorBaseSpec, String> {
    match &args.factor_base {
        Some(path) => {
            let doc = load_factor_base(path)?;
            if doc.degree != args.degree
                || doc.curve_a != args.curve_a
                || doc.subfield != args.subfield
                || doc.curve_b != args.curve_b
            {
                return Err("factor-base recipe was found on a different curve".into());
            }
            Ok(doc.spec)
        }
        None => {
            validate_factor_size(args.degree, args.subfield)?;
            Ok(FactorBaseSpec::Factor {
                index: args.factor_index,
            })
        }
    }
}

/// Solve one arm's prefix of `pool` and describe it.
#[allow(clippy::too_many_arguments)]
fn run_arm(
    kc: &KoblitzCurve,
    fb: &crypto_lib::cryptanalysis::koblitz_index_calculus::FrobeniusFactorBase,
    opts: &crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzIcOptions,
    pool: &RelationPool,
    arm: &Arm,
    seed: u64,
    seed_index: usize,
    order: usize,
    instance: &str,
) -> Value {
    let columns = pool.columns;
    let base = json!({
        "arm": arm.id, "role": arm.role,
        "fraction": arm.fraction.is_finite().then_some(arm.fraction), "seed": seed,
        "seed_index": seed_index, "arm_order": order, "columns": columns,
        "full_rank_relations": pool.full_rank_relations,
        "logs_first_attempt_relations": pool.logs_first_attempt_relations,
    });
    let (Some(full), Some(logs_prefix)) =
        (pool.full_rank_relations, pool.logs_first_attempt_relations)
    else {
        return merge(
            base,
            json!({"status":"failed","verified":false,
            "reason":"coefficient rank never became full within the trial budget"}),
        );
    };
    let prefix = if arm.logs_default {
        logs_prefix
    } else {
        full + (arm.fraction * columns as f64).ceil() as usize
    };
    let base = merge(base, json!({"surplus_relations": prefix - full}));
    if prefix > pool.relations.len() {
        return merge(
            base,
            json!({"status":"failed","verified":false,"prefix_relations":prefix,
            "reason":"pool exhausted the trial budget before this prefix"}),
        );
    }
    let cost = pool.cumulative[prefix - 1];
    let begin = Instant::now();
    let solved = solve_factor_base_logs_from_relations(kc, fb, opts, &pool.relations[..prefix]);
    let arm_wall = begin.elapsed().as_secs_f64();
    let Some((table, report)) = solved else {
        return merge(
            base,
            json!({"status":"failed","verified":false,"prefix_relations":prefix,
            "reason":"degenerate factor base"}),
        );
    };
    // Independent re-check of every column, not only the solver's flag.
    let verified = report.verified && table.columns.len() == columns && table.verify(kc);
    let sparse = report.sparse_report.as_ref();
    let filter = sparse.map(|s| &s.filter);
    let row_ops = sparse.map(|s| s.row_ops);
    let core_dimension = sparse.map(|s| s.core_dimension);
    let core_nonzeros = sparse.map(|s| s.core_nonzeros);
    let krylov_proxy = sparse.map(|s| s.core_dimension as u128 * s.core_nonzeros as u128);
    let total_wall = cost.collection_seconds_prorated + report.linear_algebra_seconds;
    let status = if verified { "succeeded" } else { "failed" };
    let detail = json!({
        "status": status,
        "verified": verified,
        "prefix_relations": prefix,
        "collection": {
            "trials": cost.trials,
            "seconds_prorated": cost.collection_seconds_prorated,
            "summands_scanned_prorated": cost.summands_scanned_prorated,
        },
        "relation_reverification_seconds": report.collection_seconds,
        "duplicate_relations": report.duplicate_relations,
        "rejected_relations": report.rejected_relations,
        "linear_algebra": {
            "mode": if report.sparse { "sparse" } else { "dense" },
            "attempts": report.solve_attempts,
            "seconds": report.linear_algebra_seconds,
            "row_ops": row_ops,
            "filter": filter,
            "core_dimension": core_dimension,
            "core_nonzeros": core_nonzeros,
            "krylov_proxy_columns_x_nonzeros": krylov_proxy.map(|v| v.to_string()),
        },
        "total_wall_seconds": total_wall,
        "arm_process_seconds": arm_wall,
    });
    let export_run = json!({
        "run_id": format!("{instance}/s{seed_index}/{}", arm.id),
        "arm": arm.id, "instance_id": instance, "seed": seed, "replicate": seed_index,
        "warmup": false, "status": status,
        "outcome_class": if verified { "completed" } else { "not_completed" },
        "attempts": 1, "resumed": 0, "checkpoint": null,
        "measurements": {
            "counted": {
                "collection_trials": cost.trials,
                "la_row_ops": row_ops,
            },
            "wall_seconds": total_wall,
            // Host isolation is recorded by the queue that runs this, not here.
            "wall_valid": false,
            "cpu_user_seconds": null, "cpu_sys_seconds": null, "max_rss_bytes": null,
            "counters": {
                "prefix_relations": prefix,
                "collection_seconds_prorated": cost.collection_seconds_prorated,
                "summands_scanned_prorated": cost.summands_scanned_prorated,
                "la_seconds": report.linear_algebra_seconds,
                "core_dimension": core_dimension,
                "core_nonzeros": core_nonzeros,
                "filter_rows_out": filter.map(|f| f.rows_out),
                "filter_nonzeros_out": filter.map(|f| f.nonzeros_out),
            },
        },
        "verification": {"every_column_verified": verified, "columns": columns},
    });
    merge(merge(base, detail), json!({"export_run": export_run}))
}

fn merge(mut a: Value, b: Value) -> Value {
    if let (Some(a), Value::Object(b)) = (a.as_object_mut(), b) {
        a.extend(b);
    }
    a
}

/// Paired differences against the seed's f = 0 baseline arm.
fn add_deltas(rows: &mut [Value]) {
    let baseline = rows
        .iter()
        .find(|r| r["role"] == "baseline" && r["verified"] == true)
        .cloned();
    let Some(b) = baseline else { return };
    for row in rows.iter_mut() {
        if row["verified"] != true {
            continue;
        }
        let d = |path: &[&str]| -> Option<f64> {
            let get = |v: &Value| path.iter().try_fold(v, |v, k| v.get(*k))?.as_f64();
            Some(get(row)? - get(&b)?)
        };
        row["paired_vs_f0"] = json!({
            "collection_trials": d(&["collection", "trials"]),
            "collection_seconds_prorated": d(&["collection", "seconds_prorated"]),
            "la_row_ops": d(&["linear_algebra", "row_ops"]),
            "la_seconds": d(&["linear_algebra", "seconds"]),
            "total_wall_seconds": d(&["total_wall_seconds"]),
        });
    }
}

pub fn run(args: OversampleArgs, quiet: bool) -> Result<Value, String> {
    let begin = Instant::now();
    if let Some(path) = &args.rows_out {
        if std::fs::symlink_metadata(path).is_ok() {
            return Err(format!("rows output already exists: {}", path.display()));
        }
    }
    let arm_list = arms(&args.fractions, args.aa_control, args.logs_arm)?;
    let f_max = arm_list
        .iter()
        .filter(|a| a.fraction.is_finite())
        .map(|a| a.fraction)
        .fold(0.0, f64::max);
    let spec = spec_of(&args)?;
    let c = curve(args.degree, args.curve_a, args.subfield, args.curve_b)?;
    let fb = materialize(&c, &spec)?;
    let mut sparse = sparse_options(
        usize::from(args.block_size),
        args.sparse_solver,
        args.spmv,
        args.spmv_shards,
        args.filter_strategy,
    );
    if let Some(excess) = args.target_excess {
        sparse.filter.target_excess = excess;
    }
    let instance = format!(
        "{}/m{}/{}",
        c.label(),
        args.summands,
        serde_json::to_string(&spec).unwrap_or_default()
    );
    if !quiet {
        println!(
            "ic — oversampling measurement on {}; {} seeds; arms {:?}",
            c.label(),
            args.seeds,
            arm_list.iter().map(|a| a.id.as_str()).collect::<Vec<_>>()
        );
    }
    let mut rows: Vec<Value> = Vec::new();
    let mut pools: Vec<Value> = Vec::new();
    let mut failures = 0usize;
    for i in 0..args.seeds as usize {
        let seed = args.seed.wrapping_add(i as u64);
        let opts = with_linear_algebra(
            ic_options(args.solver, args.summands, args.max_trials, seed),
            LinearAlgebraMode::Sparse,
            sparse,
        );
        debug_assert!(matches!(opts.linear_algebra, LinearAlgebra::Sparse(_)));
        super::progress::line(format_args!("oversample: seed {i}: collecting pool"));
        let pool = collect_factor_base_log_pool(&c, &fb, &opts, f_max).ok_or(
            "factor base has no usable projected columns, or the subgroup exceeds 64 bits",
        )?;
        pools.push(json!({"seed": seed, "seed_index": i, "pool": pool,
            "relations": pool.relations.len()}));
        let mut seed_rows = Vec::new();
        for (order, arm) in rotated(&arm_list, i).iter().enumerate() {
            super::progress::line(format_args!("oversample: seed {i}: arm {}", arm.id));
            seed_rows.push(run_arm(
                &c, &fb, &opts, &pool, arm, seed, i, order, &instance,
            ));
        }
        add_deltas(&mut seed_rows);
        failures += seed_rows.iter().filter(|r| r["verified"] != true).count();
        rows.extend(seed_rows);
    }
    if let Some(path) = &args.rows_out {
        let text: String = rows
            .iter()
            .map(|r| serde_json::to_string(r).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n");
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .and_then(|mut f| {
                use std::io::Write;
                f.write_all(text.as_bytes())?;
                f.write_all(b"\n")?;
                f.sync_all()
            })
            .map_err(|e| format!("could not create {}: {e}", path.display()))?;
    }
    let status = if failures == 0 {
        "complete"
    } else {
        "incomplete"
    };
    Ok(json!({
        "schema_version": 1, "operation": "oversample", "status": status,
        "evidence_scope": "synthetic_known_answer",
        "degree": c.n, "curve_a": c.a, "subgroup_order": c.subgroup_order.to_string(),
        "instance_id": instance,
        "factor_base": factor_base_json(&spec, &fb, rows.first().and_then(|r| r["columns"].as_u64()).unwrap_or(0) as usize),
        "summands": args.summands, "solver": args.solver.name(),
        "sparse_options": sparse,
        "fractions": args.fractions, "aa_control": args.aa_control, "logs_arm": args.logs_arm,
        "arm_order": "rotated left by seed index",
        "failed_arms": failures,
        "pools": pools,
        "rows": rows,
        "elapsed_seconds": begin.elapsed().as_secs_f64(),
        "resources": super::experiment::resources(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiny(fractions: &str, aa: bool) -> OversampleArgs {
        use clap::Parser;
        #[derive(Parser)]
        struct P {
            #[command(flatten)]
            a: OversampleArgs,
        }
        let mut argv = vec![
            "p",
            "--degree",
            "23",
            "--seeds",
            "2",
            "--max-trials",
            "20000",
            "--fractions",
            fractions,
        ];
        if aa {
            argv.push("--aa-control");
        }
        argv.push("--logs-arm");
        P::parse_from(argv).a
    }

    #[test]
    fn arms_reject_duplicates_and_require_zero_for_aa() {
        assert!(arms(&[0.0, 0.0], false, false).is_err());
        assert!(arms(&[0.1], true, false).is_err());
        let a = arms(&[0.0, 0.5], true, false).unwrap();
        assert_eq!(a.len(), 3);
        assert_eq!(arms(&[0.0], false, true).unwrap()[1].id, "logs_default");
        assert_eq!(rotated(&a, 1)[0].id, "f0.5");
    }

    #[test]
    fn oversample_tiny_curve_all_arms_verify_and_costs_are_monotone() {
        let out = run(tiny("0,0.1,0.25,0.5", true), true).expect("oversample runs");
        assert_eq!(out["status"], "complete", "{out:#}");
        let rows = out["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 2 * 6);
        for seed_index in 0..2u64 {
            let mut seed_rows: Vec<&Value> = rows
                .iter()
                .filter(|r| {
                    r["seed_index"] == seed_index
                        && r["role"] != "control_aa"
                        && r["role"] != "reference"
                })
                .collect();
            assert_eq!(seed_rows.len(), 4);
            seed_rows.sort_by(|a, b| {
                a["fraction"]
                    .as_f64()
                    .unwrap()
                    .total_cmp(&b["fraction"].as_f64().unwrap())
            });
            let mut last = (0u64, 0u64, -1.0f64);
            for r in &seed_rows {
                assert_eq!(r["verified"], true, "{r:#}");
                let prefix = r["prefix_relations"].as_u64().unwrap();
                let trials = r["collection"]["trials"].as_u64().unwrap();
                let secs = r["collection"]["seconds_prorated"].as_f64().unwrap();
                assert!(
                    prefix >= last.0 && trials >= last.1 && secs >= last.2,
                    "{r:#}"
                );
                last = (prefix, trials, secs);
            }
            let f0 = seed_rows[0];
            assert_eq!(
                f0["prefix_relations"], f0["full_rank_relations"],
                "f = 0 solves exactly the full-rank prefix"
            );
            assert_eq!(f0["paired_vs_f0"]["collection_trials"], 0.0);
            let aa = rows
                .iter()
                .find(|r| r["seed_index"] == seed_index && r["role"] == "control_aa")
                .unwrap();
            assert_eq!(aa["prefix_relations"], f0["prefix_relations"]);
            assert_eq!(aa["verified"], true);
            let logs = rows
                .iter()
                .find(|r| r["seed_index"] == seed_index && r["role"] == "reference")
                .unwrap();
            assert_eq!(logs["verified"], true, "{logs:#}");
            assert_eq!(
                logs["prefix_relations"],
                logs["logs_first_attempt_relations"]
            );
            assert!(
                logs["prefix_relations"].as_u64() >= f0["prefix_relations"].as_u64(),
                "ic logs never attempts before full rank"
            );
        }
    }
}
