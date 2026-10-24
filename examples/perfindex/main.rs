//! `perfindex`: the performance index of `docs/perf/PERFORMANCE_INDEX.md`,
//! natively.  Compares two `perfbench` binaries over the registry's kernels,
//! paired and interleaved, refuses a comparison whose outputs differ, and
//! leaves a record anyone can replay.
//!
//! ```bash
//! cargo run --release --example perfindex -- build --reference REV --out DIR
//! cargo run --release --example perfindex -- compare --base A --cand B --out DIR
//! cargo run --release --example perfindex -- null --bin A --out DIR
//! cargo run --release --example perfindex -- replay DIR
//! ```
//!
//! This replaces `scripts/perf/perfindex.py` (AGENTS.md: no Python for
//! benchmark drivers, replay tools or correctness tests).  The Python tool
//! stays, untouched, as the legacy driver `scripts/perf/perfhistory.py`
//! imports; new work goes here.

mod buildcmd;
mod compare;
mod evidence;
mod fingerprints;
mod host;
mod instr;
mod stats;
mod summarize;
mod util;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "perfindex",
    about = "Performance index: baseline vs candidate over the perfbench kernels"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Build perfbench for a revision (or the working tree) with this tree's harness.
    Build(buildcmd::BuildArgs),
    /// Wall-time comparison of two binaries: paired, interleaved, recorded row by row.
    Compare {
        #[arg(long)]
        base: PathBuf,
        #[arg(long)]
        cand: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[command(flatten)]
        args: compare::CompareArgs,
    },
    /// Callgrind instruction counts of both binaries.
    Instr {
        #[arg(long)]
        base: PathBuf,
        #[arg(long)]
        cand: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[command(flatten)]
        args: instr::InstrArgs,
    },
    /// A binary against a copy of itself: every faster/slower verdict is false.
    Null {
        #[arg(long)]
        bin: PathBuf,
        #[arg(long)]
        out: PathBuf,
        #[command(flatten)]
        args: compare::CompareArgs,
    },
    /// Print compare.json / instr.json as a table.
    Report { json: PathBuf },
    /// Recompute compare.json from plan.json and raw.jsonl.
    Replay { dir: PathBuf },
    /// Copy a replayed run into a directory fit to commit.
    Bundle {
        dir: PathBuf,
        #[arg(long)]
        dest: PathBuf,
        #[arg(long, default_value_t = 25.0)]
        max_mb: f64,
    },
    /// Check kernels' fingerprints across thread counts and against the recorded ones.
    Fingerprints(fingerprints::FingerprintArgs),
}

fn run(cli: Cli) -> Result<i32, String> {
    match cli.cmd {
        Cmd::Build(a) => buildcmd::run_build(&a),
        Cmd::Compare {
            base,
            cand,
            out,
            args,
        } => compare::run_compare(&base, &cand, &out, &args),
        Cmd::Instr {
            base,
            cand,
            out,
            args,
        } => instr::run_instr(&base, &cand, &out, &args),
        Cmd::Null { bin, out, args } => compare::run_null(&bin, &out, &args),
        Cmd::Report { json } => {
            let s: summarize::Summary =
                serde_json::from_value(util::read_json(&json)?).map_err(|e| e.to_string())?;
            print!("{}", summarize::markdown(&s, "perfbench comparison"));
            Ok(0)
        }
        Cmd::Replay { dir } => evidence::run_replay(&dir),
        Cmd::Bundle { dir, dest, max_mb } => evidence::run_bundle(&dir, &dest, max_mb),
        Cmd::Fingerprints(a) => fingerprints::run_fingerprints(&a),
    }
}

fn main() {
    match run(Cli::parse()) {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(2);
        }
    }
}
