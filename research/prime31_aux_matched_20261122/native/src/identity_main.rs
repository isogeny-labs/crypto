//! Issue the one fixed sparse-H2 identity certificate using the existing native
//! certificate implementation. This is a correctness artifact, not a benchmark.
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use prime31_aux_matched::{sparse_identity, SOURCE_MANIFEST};
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn write_new<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    serde_json::to_writer_pretty(&mut file, value).map_err(|error| error.to_string())?;
    writeln!(file).map_err(|error| error.to_string())
}
fn execute() -> Result<(), String> {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() != 3 || arguments[1] != "issue" {
        return Err("usage: prime31-sparse-identity issue ABSOLUTE_NEW_OUTPUT_DIRECTORY".into());
    }
    let output = Path::new(&arguments[2]);
    if !output.is_absolute() || output.exists() {
        return Err("identity output must be a new absolute directory".into());
    }
    let executable = std::env::current_exe().map_err(|error| error.to_string())?;
    let binary = fs::read(&executable).map_err(|error| error.to_string())?;
    let mut checked = sparse_identity::issue_checked()?;
    checked.audit["executable_sha256"] = json!(digest(&binary));
    checked.audit["executable_bytes"] = json!(binary.len());
    checked.audit["source_manifest_sha256"] = json!(digest(SOURCE_MANIFEST.as_bytes()));
    checked.audit["native_helper"] = json!("prime31-sparse-identity");
    // Creation and all file writes are non-overwriting. A partial output from
    // a failed write remains visible; never overwrite it to conceal a failure.
    fs::create_dir(output).map_err(|error| error.to_string())?;
    write_new(&output.join("certificate.json"), &checked.certificate)?;
    write_new(&output.join("identity-audit.json"), &checked.audit)?;
    println!(
        "{}",
        json!({"event":"identity_artifact_written","certificate_id":checked.certificate.id,
            "output_directory":output,"signed_verdict":false,"formal_proof":null,
            "benchmark_executed":false})
    );
    Ok(())
}
fn main() {
    if let Err(error) = execute() {
        eprintln!(
            "{}",
            json!({"event":"identity_artifact_error","error":error})
        );
        std::process::exit(1);
    }
}
