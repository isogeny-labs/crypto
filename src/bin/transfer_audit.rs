//! `transfer-audit`: enumerate candidate transfers of one curve's discrete
//! logarithm, certify the subgroup-preservation of each, price it in the
//! phases the transfer discipline requires against the matched generic
//! baseline, and emit a verdict from the closed vocabulary.
//!
//! ```text
//! transfer-audit presets
//! transfer-audit audit  --curve sect113r1 --out DIR
//! transfer-audit suite  --out DIR [--measure-ic]
//! transfer-audit check  --certs DIR
//! transfer-audit sweep  [--k-max 64]
//! transfer-audit report --certs DIR --out DIR
//! ```
//!
//! Protocol: `research/transfer_audit_20261011/PROTOCOL.md`.  Cost rows are
//! formula-level estimates unless a row says `measured`; the tool claims no
//! ECDLP speedup and makes no security statement about a deployed curve.

#[path = "transfer_audit/certificate.rs"]
mod certificate;
#[path = "transfer_audit/controls.rs"]
mod controls;
#[path = "transfer_audit/cost.rs"]
mod cost;
#[path = "transfer_audit/cover.rs"]
mod cover;
#[path = "transfer_audit/ctx.rs"]
mod ctx;
#[path = "transfer_audit/ghs.rs"]
mod ghs;
#[path = "transfer_audit/isogeny.rs"]
mod isogeny;
#[path = "transfer_audit/pairing.rs"]
mod pairing;
#[path = "transfer_audit/report.rs"]
mod report;
#[path = "transfer_audit/spec.rs"]
mod spec;
#[path = "transfer_audit/subgroup.rs"]
mod subgroup;
#[path = "transfer_audit/suite.rs"]
mod suite;
#[path = "transfer_audit/verdict.rs"]
mod verdict;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "transfer-audit",
    about = "Enumerate and certify candidate ECDLP transfers (GHS descent, isogenies, covers, pairings) with complete cost accounting",
    long_about = "A bounded audit tool. For one curve with its generator and prime subgroup it enumerates the candidate transfers of the declared families, executes the maps the repository can execute, certifies subgroup preservation (verified / arithmetic / not_evaluated / refuted), prices each candidate in four phases against the matched Pollard-rho baseline, and assigns a verdict from the closed vocabulary. Cost rows are formula-level estimates unless labelled measured. It recovers no key and makes no deployed-security claim."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List the registered curves and toy specifications the suite audits.
    Presets,
    /// Audit one registered curve (standard name or ICV1 slug).
    Audit {
        #[arg(long)]
        curve: String,
        #[arg(long, default_value = "docs/curves/registry.json")]
        registry: PathBuf,
        #[command(flatten)]
        common: Common,
        /// Directory for the certificates; omit to print the summary only.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Run the frozen protocol set and write certificates, controls and index.
    Suite {
        #[arg(long)]
        out: PathBuf,
        #[arg(long, default_value = "docs/curves/registry.json")]
        registry: PathBuf,
        #[command(flatten)]
        common: Common,
        /// Also run the measured odd-characteristic index-calculus control (F9).
        #[arg(long)]
        measure_ic: bool,
        /// Also run the longer F9 plan, labelled post hoc (not pre-registered).
        #[arg(long)]
        measure_ic_extended: bool,
    },
    /// Regenerate the deterministic part of the suite and compare byte for byte.
    Check {
        #[arg(long)]
        certs: PathBuf,
        #[arg(long, default_value = "docs/curves/registry.json")]
        registry: PathBuf,
        #[command(flatten)]
        common: Common,
    },
    /// Audit every prime-field registry curve with a generator and embedding
    /// degree at most `--k-max` and print the pairing verdicts: the
    /// calibration sweep that pins the `class-weak` set (trait records are
    /// read from the committed `docs/curves/traits.json`).
    Sweep {
        #[arg(long, default_value = "docs/curves/registry.json")]
        registry: PathBuf,
        #[command(flatten)]
        common: Common,
        /// Largest embedding degree swept.
        #[arg(long, default_value_t = 64)]
        k_max: u64,
    },
    /// Build the results table, CSV and SVG figures from the certificates.
    Report {
        #[arg(long)]
        certs: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
}

#[derive(clap::Args, Clone)]
pub struct Common {
    /// Committed factor hints for r - 1 and the discriminant.
    #[arg(long, default_value = "docs/curves/factor-hints.txt")]
    pub hints: PathBuf,
    /// Pollard-rho iterations per composite when factoring.
    #[arg(long, default_value_t = 1 << 18)]
    pub budget: u64,
    /// Seed of every planted scalar and sampled point.
    #[arg(long, default_value_t = 20261011)]
    pub seed: u64,
    /// `docs/curves/jacobian-transfers.json`, for the route cross-check.
    #[arg(long, default_value = "docs/curves/jacobian-transfers.json")]
    pub transfers: PathBuf,
    /// `docs/curves/covers.json`, for the catalog cover rows.
    #[arg(long, default_value = "docs/curves/covers.json")]
    pub covers: PathBuf,
    /// `docs/curves/traits.json`, read only for the control curve whose live
    /// trait computation takes minutes.
    #[arg(long, default_value = "docs/curves/traits.json")]
    pub traits: PathBuf,
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("transfer-audit: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::Presets => {
            for line in suite::preset_lines() {
                println!("{line}");
            }
            Ok(())
        }
        Command::Audit {
            curve,
            registry,
            common,
            out,
        } => suite::audit_one(&curve, &registry, &common, out.as_deref()),
        Command::Suite {
            out,
            registry,
            common,
            measure_ic,
            measure_ic_extended,
        } => suite::run_suite(&out, &registry, &common, measure_ic, measure_ic_extended),
        Command::Check {
            certs,
            registry,
            common,
        } => suite::check_suite(&certs, &registry, &common),
        Command::Sweep {
            registry,
            common,
            k_max,
        } => suite::print_sweep(&registry, &common, k_max),
        Command::Report { certs, out } => report::build(&certs, &out),
    }
}
