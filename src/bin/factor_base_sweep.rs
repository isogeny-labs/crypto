//! Unified native structured-factor-base search with durable line telemetry.
#[path = "factor_base_sweep/audit.rs"]
mod audit;
#[path = "factor_base_sweep/engine.rs"]
mod engine;
#[path = "factor_base_sweep/leads.rs"]
mod leads;
#[path = "factor_base_sweep/leads_algebra.rs"]
mod leads_algebra;
#[path = "factor_base_sweep/model.rs"]
mod model;
#[path = "factor_base_sweep/polynomials.rs"]
mod polynomials;
#[path = "factor_base_sweep/run.rs"]
mod run;
#[path = "factor_base_sweep/shapes.rs"]
mod shapes;
use clap::{Args, Parser, Subcommand};
use model::{Plan, Result};
use std::{path::PathBuf, time::Duration};

#[derive(Parser)]
#[command(
    name = "factor-base-sweep",
    about = "Search structured factor bases, verify decompositions and stream CADO-style stage telemetry"
)]
struct Cli {
    #[command(subcommand)]
    command: Action,
}
#[derive(Args, Default)]
struct Tuning {
    #[arg(long)]
    train: Option<u32>,
    #[arg(long)]
    holdout: Option<u32>,
    #[arg(long)]
    max_degree: Option<u32>,
    #[arg(long)]
    candidate_seconds: Option<u64>,
    #[arg(long)]
    max_abscissae: Option<usize>,
    #[arg(long)]
    max_matrix_cells: Option<u64>,
    #[arg(long)]
    memory_mib: Option<u64>,
    #[arg(long)]
    threads: Option<usize>,
}
impl Tuning {
    fn apply(self, p: &mut Plan, resume: bool) -> Result<()> {
        macro_rules! set {($($field:ident),*)=>{$(if let Some(value)=self.$field {if resume{return Err("resume uses frozen budgets; omit tuning flags".into());}p.$field=value;})*};}
        set!(
            train,
            holdout,
            max_degree,
            candidate_seconds,
            max_abscissae,
            max_matrix_cells,
            memory_mib,
            threads
        );
        Ok(())
    }
}
#[derive(Subcommand)]
enum Action {
    /// Compare asymmetric, selector and torsion-quotient decomposition leads.
    CompareLeads {
        #[arg(long)]
        plan: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    #[command(hide = true)]
    LeadsWorker {
        #[arg(long)]
        job: PathBuf,
    },
    /// Launch or resume a bounded sweep. Human progress is streamed to stderr.
    Run {
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        plan: Option<PathBuf>,
        #[arg(long,value_parser=["both","binary","prime"])]
        preset: Option<String>,
        #[arg(long)]
        seed: Option<u64>,
        /// Expand constructor seeds without changing matched probe targets.
        #[arg(long)]
        seed_count: Option<u32>,
        /// Binary dimensions to sweep (comma separated); default uses preset dimensions.
        #[arg(long, value_delimiter = ',')]
        dimensions: Vec<u32>,
        #[arg(long)]
        resume: bool,
        #[arg(long)]
        json_progress: bool,
        #[arg(long, default_value_t = 1.0)]
        heartbeat_seconds: f64,
        #[command(flatten)]
        tuning: Tuning,
    },
    /// Write an editable versioned plan; no search runs.
    Plan {
        #[arg(long)]
        out: PathBuf,
        #[arg(long,default_value="both",value_parser=["both","binary","prime"])]
        preset: String,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        #[arg(long, default_value_t = 1)]
        seed_count: u32,
        #[arg(long, value_delimiter = ',')]
        dimensions: Vec<u32>,
    },
    /// Verify immutable attempt hashes and exact plan binding.
    Verify {
        #[arg(long)]
        run: PathBuf,
    },
    #[command(hide = true)]
    Worker {
        #[arg(long)]
        attempt: PathBuf,
    },
}
fn dispatch() -> Result<()> {
    match Cli::parse().command {
        Action::Run {
            out,
            plan,
            preset,
            seed,
            seed_count,
            dimensions,
            resume,
            json_progress,
            heartbeat_seconds,
            tuning,
        } => {
            if !heartbeat_seconds.is_finite() || !(0.05..=60.0).contains(&heartbeat_seconds) {
                return Err("heartbeat-seconds must be finite and in0.05..60".into());
            }
            if plan.is_some()
                && (preset.is_some()
                    || seed.is_some()
                    || seed_count.is_some()
                    || !dimensions.is_empty())
            {
                return Err(
                    "plan declares its own preset/candidates and seeds; omit preset/seed flags"
                        .into(),
                );
            }
            let mut plan = if let Some(path) = plan {
                model::read(&path)?
            } else if resume {
                if preset.is_some()
                    || seed.is_some()
                    || seed_count.is_some()
                    || !dimensions.is_empty()
                {
                    return Err(
                        "resume loads the frozen manifest; do not change preset/seed".into(),
                    );
                }
                let old: serde_json::Value = model::read(&out.join("manifest.json"))?;
                serde_json::from_value(old["plan"].clone()).map_err(|e| e.to_string())?
            } else {
                Plan::grid(
                    preset.as_deref().unwrap_or("both"),
                    seed.unwrap_or(1),
                    seed_count.unwrap_or(1),
                    &dimensions,
                )?
            };
            tuning.apply(&mut plan, resume)?;
            let summary = run::run(
                plan,
                &out,
                resume,
                json_progress,
                Duration::from_secs_f64(heartbeat_seconds),
            )?;
            let cases = summary["candidates"]
                .as_array()
                .ok_or("missing candidate summary")?;
            println!(
                "{}",
                serde_json::json!({"run":out,"candidates":cases.len(),"completed":cases.iter().filter(|v|v["outcome"]=="completed").count(),"summary":summary["summary_file"],"full_pipeline_speedup":null,"certified_degree_of_regularity":null})
            );
        }
        Action::Plan {
            out,
            preset,
            seed,
            seed_count,
            dimensions,
        } => {
            let p = Plan::grid(&preset, seed, seed_count, &dimensions)?;
            p.validate()?;
            model::write_new(&out, &p)?;
            println!("{}", out.display());
        }
        Action::CompareLeads { plan, out } => println!("{}", leads::run(&plan, &out)?),
        Action::LeadsWorker { job } => leads::worker(&job)?,
        Action::Verify { run } => println!(
            "{}",
            if run.join("leads-manifest.json").exists() {
                leads::verify(&run)?
            } else {
                run::verify(&run)?
            }
        ),
        Action::Worker { attempt } => {
            let request = model::read(&attempt.join("request.json"))?;
            engine::run(&request, &attempt)?;
        }
    }
    Ok(())
}
fn main() -> std::process::ExitCode {
    match dispatch() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            let _ = model::emit("fatal", serde_json::json!({"message":e}));
            eprintln!("Error:factor-base-sweep:{e}");
            std::process::ExitCode::FAILURE
        }
    }
}
