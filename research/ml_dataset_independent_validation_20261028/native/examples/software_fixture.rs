//! Emit fixed test files only; never register these as campaign observations.
#[path = "../tests/support/mod.rs"]
mod support;
use anyhow::{Context, Result};
use ml_dataset_independent_consumer::common;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    anyhow::ensure!(
        args.len() == 2,
        "usage: software_fixture NEW_DIRECTORY S3_REMOTE"
    );
    let root = Path::new(&args[0]);
    support::fixture(root, "2")?;
    let receipt = support::receipt(root, &args[1])?;
    let path = root
        .parent()
        .context("fixture parent")?
        .join("completion.json");
    common::write_new(&path, &serde_json::to_vec_pretty(&receipt)?)?;
    println!(
        "{}",
        serde_json::json!({"software_fixture":true,"completion_sha256":common::hash_file(&path)?,"receipt":path})
    );
    Ok(())
}
