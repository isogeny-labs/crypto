//! `ic sweep` and `ic sweep-trial`: an Optuna-style sweeper over index-calculus
//! pipeline configurations.
//!
//! ```text
//! ic sweep spaces                          what can be searched, per regime
//! ic sweep run --space koblitz-algebraic --koblitz-a 0 \
//!      --rungs 13,17,19,23 --target-log2r 81 --trials 60 --workers 4 \
//!      --store runs/koblitz-a0.jsonl       search, resumable
//! ic sweep show --store runs/koblitz-a0.jsonl
//! ic sweep-trial --config-json '{…}'       one trial, one scored record
//! ```
//!
//! The pieces, each testable on its own: [`trial`] runs one configuration
//! through `ic bench` and reduces its report to a count-based, §5a-banded
//! score with a failure class; [`space`] says what can vary, conditionally
//! and relative to scale; [`tpe`] proposes the next point; [`objective`]
//! turns rung results into the one scalar; [`store`] keeps the study as an
//! append-only file; [`driver`] runs the loop, with median pruning and one
//! child process per trial.
//!
//! Design and open decisions: `research/ic_sweeper_20261024/README.md`.

pub mod driver;
pub mod objective;
pub mod space;
pub mod store;
pub mod tpe;
pub mod trial;

use clap::{Args, Subcommand};
use serde_json::{json, Value};

use driver::{run_study, ProcessEvaluator, Study};
use space::{space_for, InstanceKind, SPACE_NAMES};
use store::{Header, Store, TrialState, SCHEMA as STORE_SCHEMA};
use tpe::TpeOptions;

#[derive(Args, Clone)]
pub struct SweepArgs {
    #[command(subcommand)]
    pub action: SweepAction,
}

#[derive(Subcommand, Clone)]
pub enum SweepAction {
    /// List the search spaces and their parameters.
    Spaces(SpacesArgs),
    /// Run, or resume, a study.
    Run(RunArgs),
    /// Summarise a study's store.
    Show(ShowArgs),
}

#[derive(Args, Clone)]
pub struct SpacesArgs {
    /// Describe one space in full (parameters, domains, conditions).
    #[arg(long)]
    pub space: Option<String>,
    /// With `--space`: the Koblitz curve (0 or 1); the default is 0.
    #[arg(long, default_value_t = 0)]
    pub koblitz_a: u8,
    /// With `--space`: the prime family (`generic`, `j0`, `j1728`, `d7`, `d8`).
    #[arg(long, default_value = "generic")]
    pub family: String,
}

#[derive(Args, Clone)]
pub struct RunArgs {
    /// One of `ic sweep spaces`.
    #[arg(long)]
    pub space: String,
    /// `koblitz-*` spaces: 0 for `K_0` (the primary target), 1 for `K_1`.
    #[arg(long, default_value_t = 0)]
    pub koblitz_a: u8,
    /// `prime-table`: `generic`, `j0`, `j1728`, `d7`, `d8`.
    #[arg(long, default_value = "generic")]
    pub family: String,
    /// `char2-algebraic`: the largest cofactor a random curve may have.
    #[arg(long, default_value_t = 8)]
    pub max_cofactor: u64,
    /// Comma-separated sizes, ascending: field degrees (binary) or subgroup
    /// bits (prime).  At least four are needed to fit a projection.
    #[arg(long, value_delimiter = ',', required = true)]
    pub rungs: Vec<u32>,
    /// `log2 r` the objective is projected to (81 for the a = 0 n = 83 curve).
    #[arg(long, default_value_t = 81.0)]
    pub target_log2r: f64,
    /// Rungs needed before the objective is a fit rather than the last
    /// rung's measurement.
    #[arg(long, default_value_t = 4)]
    pub min_fit: usize,
    /// Trials in the finished study (a resumed study runs the missing ones).
    #[arg(long, default_value_t = 40)]
    pub trials: usize,
    /// Trials evaluated at once, one child process each.
    #[arg(long, default_value_t = 1)]
    pub workers: usize,
    /// Seed of the sampler and of every planted logarithm.
    #[arg(long, default_value_t = 1)]
    pub seed: u64,
    /// Planted targets per rung.
    #[arg(long, default_value_t = 2)]
    pub repeats: usize,
    /// Counted trials a pipeline may draw before it gives up.
    #[arg(long, default_value_t = 200_000)]
    pub max_trials: u64,
    /// Counted rho runs per rung, for the context column only.
    #[arg(long, default_value_t = 4)]
    pub rho_runs: usize,
    /// Trials chosen from the prior before the sampler models anything.
    #[arg(long, default_value_t = 10)]
    pub startup_trials: usize,
    /// Other trials that must reach a rung before it can prune.
    #[arg(long, default_value_t = 5)]
    pub prune_startup: usize,
    /// Per-trial wall-clock safety net, seconds (0 for none).
    #[arg(long, default_value_t = 900)]
    pub timeout_seconds: u64,
    /// Per-trial peak-memory safety net, MiB (0 for none).
    #[arg(long, default_value_t = 8192)]
    pub max_rss_mb: u64,
    /// The append-only study file; created, or resumed if it exists.
    #[arg(long)]
    pub store: std::path::PathBuf,
}

#[derive(Args, Clone)]
pub struct ShowArgs {
    #[arg(long)]
    pub store: std::path::PathBuf,
    /// Trials to list, best first.
    #[arg(long, default_value_t = 10)]
    pub top: usize,
}

pub fn run_sweep(args: SweepArgs, json_only: bool) -> Result<Value, String> {
    match args.action {
        SweepAction::Spaces(a) => spaces(a),
        SweepAction::Run(a) => run(a, json_only),
        SweepAction::Show(a) => show(a),
    }
}

fn instance_for(
    space: &str,
    koblitz_a: u8,
    family: &str,
    max_cofactor: u64,
) -> Result<InstanceKind, String> {
    let regime = match space {
        s if s.starts_with("koblitz") => "koblitz",
        "char2-algebraic" => "char2",
        "prime-table" => "prime",
        other => {
            return Err(format!(
                "unknown space `{other}`; the spaces are {}",
                SPACE_NAMES.join(", ")
            ))
        }
    };
    Ok(InstanceKind {
        regime: regime.into(),
        koblitz_a: (regime == "koblitz").then_some(koblitz_a),
        family: (regime == "prime").then(|| family.to_string()),
        max_cofactor: (regime == "char2").then_some(max_cofactor),
    })
}

fn spaces(a: SpacesArgs) -> Result<Value, String> {
    let listing: Vec<Value> = match &a.space {
        Some(name) => {
            let inst = instance_for(name, a.koblitz_a, &a.family, 8)?;
            vec![space_for(name, &inst)?.params_json()]
        }
        None => SPACE_NAMES
            .iter()
            .map(|n| {
                let inst = instance_for(n, 0, "generic", 8)?;
                let s = space_for(n, &inst)?;
                Ok(json!({
                    "space": s.name,
                    "track": s.track.as_str(),
                    "counted": s.track.counted(),
                    "regime": inst.regime,
                    "doc": s.doc,
                    "parameters": s.params.iter().map(|p| p.name.clone()).collect::<Vec<_>>(),
                }))
            })
            .collect::<Result<_, String>>()?,
    };
    Ok(json!({
        "schema_version": 1,
        "operation": "sweep-spaces",
        "status": "complete",
        "tracks": {
            "algebraic": "counted: relations come from solving summation-polynomial systems (owner decision 2026-10-24)",
            "table": "reference only: never ranked against the algebraic track as an improvement",
        },
        "spaces": listing,
    }))
}

fn run(a: RunArgs, json_only: bool) -> Result<Value, String> {
    let inst = instance_for(&a.space, a.koblitz_a, &a.family, a.max_cofactor)?;
    let space = space_for(&a.space, &inst)?;
    if a.rungs.len() < 2 {
        return Err("--rungs needs at least two sizes".into());
    }
    if a.rungs.len() < a.min_fit {
        eprintln!(
            "ic sweep: {} rungs is below --min-fit {}, so the objective will be the last rung's \
             measured ratio, not a projection to log2 r = {}",
            a.rungs.len(),
            a.min_fit,
            a.target_log2r
        );
    }
    let study = Study {
        rungs: a.rungs.clone(),
        target_log2_r: a.target_log2r,
        min_fit: a.min_fit,
        trials: a.trials,
        workers: a.workers,
        seed: a.seed,
        repeats: a.repeats,
        max_trials: a.max_trials,
        rho_runs: a.rho_runs,
        prune_startup: a.prune_startup,
        tpe: TpeOptions {
            startup: a.startup_trials,
            ..TpeOptions::default()
        },
    };
    let header = Header {
        schema: STORE_SCHEMA.into(),
        space: space.name.into(),
        track: space.track.as_str().into(),
        counted: space.track.counted(),
        instance: inst.describe(),
        rungs: a.rungs.clone(),
        target_log2_r: a.target_log2r,
        min_fit: a.min_fit,
        repeats: a.repeats,
        seed: a.seed,
        max_trials: a.max_trials,
        rho_runs: a.rho_runs,
        git_commit: crate::git_commit(),
        binary_blake3: crate::binary_hash(),
    };
    let mut store = Store::open(&a.store, &header)?;
    let exe = std::env::current_exe().map_err(|e| format!("cannot find this executable: {e}"))?;
    let eval = ProcessEvaluator {
        exe,
        timeout_seconds: a.timeout_seconds,
        max_rss_mb: a.max_rss_mb,
    };
    let say = |line: &str| {
        if !json_only {
            eprintln!("  {line}");
        }
    };
    say(&format!(
        "{} ({} track, {}) on {}: rungs {:?}, {} trials, {} worker(s), store {}",
        space.name,
        space.track.as_str(),
        if space.track.counted() {
            "counted"
        } else {
            "reference only"
        },
        inst.describe(),
        a.rungs,
        a.trials,
        a.workers,
        a.store.display()
    ));
    let summary = run_study(&space, &study, &mut store, &eval, &say)?;
    Ok(json!({
        "schema_version": 1,
        "operation": "sweep-run",
        "status": "complete",
        "space": space.name,
        "track": space.track.as_str(),
        "counted": space.track.counted(),
        "store": a.store,
        "usable_rungs": summary.usable_rungs,
        "complete": summary.complete,
        "pruned": summary.pruned,
        "failed": summary.failed,
        "ran_now": summary.ran_now,
        "best": summary.best.as_ref().map(|t| json!({
            "id": t.id,
            "assignment": t.assignment,
            "objective_log2_ratio": t.objective,
            "projected_to_target": t.projected,
            "slope_per_bit": t.slope,
        })),
        "reading": "objective is log2 of the worst-case (over the §5a band) S ratio to the frozen baseline; below 0 beats the baseline. A reference-track study is never an improvement claim.",
    }))
}

fn show(a: ShowArgs) -> Result<Value, String> {
    let (header, baselines, trials) = Store::read(&a.store)?;
    let mut done: Vec<&store::TrialLine> = trials
        .iter()
        .filter(|t| t.state == TrialState::Complete)
        .collect();
    done.sort_by(|x, y| {
        x.objective
            .unwrap_or(f64::INFINITY)
            .total_cmp(&y.objective.unwrap_or(f64::INFINITY))
    });
    let count = |s: TrialState| trials.iter().filter(|t| t.state == s).count();
    Ok(json!({
        "schema_version": 1,
        "operation": "sweep-show",
        "status": "complete",
        "header": header,
        "baselines": baselines.iter().map(|b| json!({
            "scale": b.scale, "class": b.record.class.as_str(), "detail": b.record.detail,
            "band_s": b.record.band.as_ref().map(|x| x.s.clone()),
        })).collect::<Vec<_>>(),
        "complete": count(TrialState::Complete),
        "pruned": count(TrialState::Pruned),
        "failed": count(TrialState::Failed),
        "top": done.iter().take(a.top).map(|t| json!({
            "id": t.id, "objective_log2_ratio": t.objective, "projected": t.projected,
            "slope_per_bit": t.slope, "assignment": t.assignment,
            "rung_log2_ratio": t.rungs.iter().map(|r| r.log2_ratio).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    }))
}
