//! Native JSON interface for verifier-backed ECDLP index-calculus challenges.

use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use crypto_lib::cryptanalysis::ecdlp_challenges::{
    baseline, known_answer_fixture, verify, ChallengeFixture, ChallengeSubmission,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;

#[derive(Parser)]
#[command(
    name = "ecdlp-challenges",
    version,
    about = "Generate and verify certificates for six ECDLP index-calculus engineering tracks"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Write the deterministic public known-answer fixture.
    Fixture {
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Generate the native reference submission for a fixture.
    Baseline {
        #[arg(long)]
        fixture: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Verify a submission and emit a direction-tagged frontier receipt.
    Verify {
        #[arg(long)]
        fixture: PathBuf,
        #[arg(long)]
        submission: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let file = File::open(path).map_err(|error| format!("open {}: {error}", path.display()))?;
    serde_json::from_reader(BufReader::new(file))
        .map_err(|error| format!("parse {}: {error}", path.display()))
}

fn write_json<T: Serialize>(value: &T, path: Option<&Path>) -> Result<(), String> {
    match path {
        Some(path) => {
            let file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
                .map_err(|error| format!("create {}: {error}", path.display()))?;
            let mut writer = BufWriter::new(file);
            serde_json::to_writer_pretty(&mut writer, value)
                .map_err(|error| format!("serialize {}: {error}", path.display()))?;
            writer
                .write_all(b"\n")
                .map_err(|error| format!("write {}: {error}", path.display()))?;
            writer
                .flush()
                .map_err(|error| format!("flush {}: {error}", path.display()))
        }
        None => {
            let stdout = io::stdout();
            let mut writer = BufWriter::new(stdout.lock());
            serde_json::to_writer_pretty(&mut writer, value)
                .map_err(|error| format!("serialize stdout: {error}"))?;
            writer
                .write_all(b"\n")
                .map_err(|error| format!("write stdout: {error}"))?;
            writer
                .flush()
                .map_err(|error| format!("flush stdout: {error}"))
        }
    }
}

fn execute(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::Fixture { out } => {
            let fixture = known_answer_fixture().map_err(|error| error.to_string())?;
            write_json(&fixture, out.as_deref())
        }
        Command::Baseline { fixture, out } => {
            let fixture: ChallengeFixture = read_json(&fixture)?;
            let submission = baseline(&fixture).map_err(|error| error.to_string())?;
            write_json(&submission, out.as_deref())
        }
        Command::Verify {
            fixture,
            submission,
            out,
        } => {
            let fixture: ChallengeFixture = read_json(&fixture)?;
            let submission: ChallengeSubmission = read_json(&submission)?;
            let receipt = verify(&fixture, &submission).map_err(|error| error.to_string())?;
            write_json(&receipt, out.as_deref())
        }
    }
}

fn main() -> ExitCode {
    match execute(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let record = json!({
                "schema": "crypto.ecdlp-challenge-error/v1",
                "status": "rejected",
                "error": error,
            });
            let _ = serde_json::to_writer(io::stderr().lock(), &record);
            eprintln!();
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use crypto_lib::cryptanalysis::ecdlp_challenges::{baseline, known_answer_fixture, verify};

    #[test]
    fn baseline_has_six_verified_frontier_coordinates() {
        let fixture = known_answer_fixture().unwrap();
        let submission = baseline(&fixture).unwrap();
        let receipt = verify(&fixture, &submission).unwrap();
        assert_eq!(receipt.status, "verified");
        assert_eq!(receipt.tracks.len(), 6);
        assert_eq!(receipt.frontier.len(), 6);
        assert_eq!(
            receipt.tracks["relation_forge"].metrics["independent_rank"],
            5
        );
        assert_eq!(receipt.claim_boundary.performance, "not_measured");
        assert_eq!(receipt.claim_boundary.transfer_advantage, "unknown");
    }

    #[test]
    fn every_track_rejects_a_mutated_witness() {
        let fixture = known_answer_fixture().unwrap();
        let reference = baseline(&fixture).unwrap();

        let mut candidate = reference.clone();
        candidate.relation_forge.rows[0].a += 1;
        assert!(verify(&fixture, &candidate)
            .unwrap_err()
            .0
            .contains("relation_forge"));

        let mut candidate = reference.clone();
        candidate.factor_base_lab.decompositions[0].left = 1;
        assert!(verify(&fixture, &candidate)
            .unwrap_err()
            .0
            .contains("factor_base_lab"));

        let mut candidate = reference.clone();
        candidate.matrix_smash.solution[0] += 1;
        assert!(verify(&fixture, &candidate)
            .unwrap_err()
            .0
            .contains("matrix_smash"));

        let mut candidate = reference.clone();
        let root = candidate.descent_tree.root.clone();
        candidate
            .descent_tree
            .nodes
            .iter_mut()
            .find(|node| node.id == root)
            .unwrap()
            .point = fixture.curve.generator.clone();
        assert!(verify(&fixture, &candidate)
            .unwrap_err()
            .0
            .contains("descent_tree"));

        let mut candidate = reference.clone();
        candidate.transfer_audit.images[0].recovered = fixture.curve.target.clone();
        assert!(verify(&fixture, &candidate)
            .unwrap_err()
            .0
            .contains("transfer_audit"));

        let mut candidate = reference;
        candidate.orbit_relations.folded_rows[0][0] += 1;
        assert!(verify(&fixture, &candidate)
            .unwrap_err()
            .0
            .contains("orbit_relations"));
    }
}
