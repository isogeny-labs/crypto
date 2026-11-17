//! Bounded CLI for finite-primary-module common-kernel certificates.

use std::{fs, path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand};
use crypto_lib::torsionguard::{analyze, parse_specification, synthesize_repair, validation_json};
use serde_json::Value;

const MAX_INPUT_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Parser)]
#[command(
    name = "torsionguard",
    version,
    about = "Certify common kernels on declared finite primary modules",
    long_about = "TorsionGuard checks whether declared linear relations have a nontrivial common kernel on a supplied finite primary module. It emits rank data and replayable witnesses while keeping curve, point, scalar, coefficient, and map facts as explicit evidence obligations."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Analyze the common kernel and emit a certificate.
    Analyze {
        specification: PathBuf,
        /// Emit one-line JSON instead of pretty-printed JSON.
        #[arg(long)]
        compact: bool,
    },
    /// Find a minimum declared-cost subset of candidate repair relations.
    Synthesize {
        specification: PathBuf,
        /// Emit one-line JSON instead of pretty-printed JSON.
        #[arg(long)]
        compact: bool,
    },
    /// Validate and normalize the input contract without analyzing it.
    Validate {
        specification: PathBuf,
        /// Emit one-line JSON instead of pretty-printed JSON.
        #[arg(long)]
        compact: bool,
    },
}

fn read_specification(path: &PathBuf) -> Result<Value, String> {
    let metadata = fs::metadata(path)
        .map_err(|error| format!("cannot inspect '{}': {error}", path.display()))?;
    if metadata.len() > MAX_INPUT_BYTES {
        return Err(format!(
            "input '{}' exceeds the {}-byte limit",
            path.display(),
            MAX_INPUT_BYTES
        ));
    }
    let bytes =
        fs::read(path).map_err(|error| format!("cannot read '{}': {error}", path.display()))?;
    serde_json::from_slice(&bytes)
        .map_err(|error| format!("cannot parse '{}': {error}", path.display()))
}

fn write_json(value: &Value, compact: bool) -> Result<(), String> {
    let output = if compact {
        serde_json::to_string(value)
    } else {
        serde_json::to_string_pretty(value)
    }
    .map_err(|error| format!("cannot serialize certificate: {error}"))?;
    println!("{output}");
    Ok(())
}

fn run(cli: Cli) -> Result<(), String> {
    let (path, compact) = match &cli.command {
        Command::Analyze {
            specification,
            compact,
        }
        | Command::Synthesize {
            specification,
            compact,
        }
        | Command::Validate {
            specification,
            compact,
        } => (specification, *compact),
    };
    let input = read_specification(path)?;
    let specification = parse_specification(&input)?;
    let output = match cli.command {
        Command::Analyze { .. } => analyze(&specification)?.to_json(),
        Command::Synthesize { .. } => synthesize_repair(&specification)?.to_json(),
        Command::Validate { .. } => validation_json(&specification),
    };
    write_json(&output, compact)
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("torsionguard: {error}");
            ExitCode::FAILURE
        }
    }
}
