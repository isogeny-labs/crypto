//! Watch ECC2K-130 relation collection with live progress and a JSONL export.
//!
//! This is a thin, opinionated front end over `ic fixed`: it defaults to the
//! checked-in ECC2K-130 profile, the `collect` stage, live progress on stderr,
//! and `relations.jsonl` inside the campaign directory. The Python engine
//! remains the source of truth; every relation is committed to
//! `campaign.sqlite3` first and then appended to the export.
use super::fixed::{self, FixedArgs};
use clap::Args;
use serde_json::Value;
use std::path::PathBuf;

#[derive(Args, Clone)]
pub struct Ecc2k130Args {
    /// Fixed K_0 parameters; defaults to the checked-in ECC2K-130 profile.
    #[arg(long)]
    pub params: Option<PathBuf>,
    /// Durable campaign directory; rerun with the same path to resume.
    #[arg(long)]
    pub dir: Option<PathBuf>,
    /// Stage; `collect` finds relations without descending to the target.
    #[arg(long, default_value = "collect", value_parser = ["select", "pairs", "collect", "logs", "solve", "all", "status"])]
    pub stage: String,
    /// Exact pair table, bounded SAT, or an opt-in learned policy with pair fallback.
    #[arg(long, default_value = "pairs", value_parser = ["pairs", "sat", "learned"])]
    pub solver: String,
    /// JSON selector trained with indexcalc_selector.py; required for learned.
    #[arg(long)]
    pub selector_model: Option<PathBuf>,
    /// Additional probes per collection stream or target in this invocation.
    #[arg(long, default_value_t = 32)]
    pub attempts: u32,
    /// Maximum new pair candidates built in this invocation. The ECC2K-130
    /// weight-2 base has 3,668 signed points, so 6,728,946 candidates complete
    /// the pair table in one run.
    #[arg(long, default_value_t = 10_000_000)]
    pub pair_budget: u64,
    /// Native SAT solve budget; a separate watchdog bounds each complete SAT query.
    #[arg(long, default_value_t = 0.25)]
    pub query_seconds: f64,
    /// Python interpreter; python-sat and pycryptosat are needed only for SAT.
    #[arg(long, default_value = "python3")]
    pub python: PathBuf,
    /// Suppress live progress lines.
    #[arg(long)]
    pub quiet: bool,
    /// Attempts between progress heartbeats; 0 prints only relations and stage changes.
    #[arg(long, default_value_t = 10)]
    pub progress_every: u32,
    /// JSON Lines relation export; defaults to <dir>/relations.jsonl.
    #[arg(long)]
    pub relations_out: Option<PathBuf>,
    /// Do not write a relation export.
    #[arg(long)]
    pub no_relations_out: bool,
}

fn default_params() -> PathBuf {
    super::install::share_file_or_source("docs/ic/params/ecc2k130-fixed.json")
}

pub fn run(args: Ecc2k130Args) -> Result<Value, String> {
    let params = args.params.unwrap_or_else(default_params);
    let dir = args.dir.unwrap_or_else(|| PathBuf::from("runs/ecc2k130"));
    let relations_out = if args.no_relations_out {
        None
    } else {
        Some(
            args.relations_out
                .unwrap_or_else(|| dir.join("relations.jsonl")),
        )
    };
    fixed::run(FixedArgs {
        params,
        dir,
        stage: args.stage,
        solver: args.solver,
        selector_model: args.selector_model,
        attempts: args.attempts,
        pair_budget: args.pair_budget,
        query_seconds: args.query_seconds,
        python: args.python,
        progress: !args.quiet,
        progress_every: args.progress_every,
        relations_out,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_the_small_profile_export_and_live_progress() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dir = std::env::temp_dir().join(format!("ic-ecc2k130-unit-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let report = run(Ecc2k130Args {
            params: Some(root.join("docs/ic/params/k0n9-fixed.json")),
            dir: Some(dir.clone()),
            stage: "collect".into(),
            solver: "pairs".into(),
            selector_model: None,
            attempts: 64,
            pair_budget: 100_000,
            query_seconds: 2.0,
            python: "python3".into(),
            quiet: false,
            progress_every: 1,
            relations_out: None,
            no_relations_out: false,
        })
        .unwrap();
        assert_eq!(report["status"], "complete");
        let export = dir.join("relations.jsonl");
        assert_eq!(report["relations_out"], export.display().to_string());
        let rows: Vec<Value> = std::fs::read_to_string(&export)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(
            rows.len() as u64,
            report["relations_saved"].as_u64().unwrap()
        );
        assert!(rows
            .iter()
            .all(|row| row["operation"] == "fixed-relation" && row["verified"] == true));
        std::fs::remove_dir_all(dir).unwrap();
    }
}
