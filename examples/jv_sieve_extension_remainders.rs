//! Public-input extension-remainder component. Scientific measurements use Runpod.
use clap::Parser;
use std::{fs::OpenOptions, path::PathBuf};
#[path = "../research/extension_ic_graph_20261115/extension_remainder_driver.rs"]
mod driver;
#[path = "../research/extension_ic_graph_20261115/extension_remainder.rs"]
mod remainder;
#[derive(Parser)]
struct Args {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    mode: String,
    #[arg(long)]
    out: PathBuf,
    #[arg(long)]
    source_revision: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.source_revision.len() != 40
        || !args.source_revision.bytes().all(|c| c.is_ascii_hexdigit())
    {
        return Err("source SHA required".into());
    }
    let bytes = std::fs::read(&args.input)?;
    if driver::digest(&bytes) != driver::INPUT_SHA {
        return Err("registered public input hash mismatch".into());
    }
    let curve = serde_json::from_slice(&bytes)?;
    let out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args.out)?;
    let overrides: Vec<_> = std::env::vars_os()
        .map(|(k, _)| k)
        .filter(|k| k.to_str().is_some_and(|s| s.starts_with("JV_")))
        .collect();
    for k in overrides {
        std::env::remove_var(k);
    }
    std::env::set_var("JV_ADDS_INC", "1");
    std::env::set_var("JV_ADDS_SHARE", "1");
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build()?;
    let record = pool.install(|| driver::run(curve, &args.mode, args.source_revision))?;
    serde_json::to_writer_pretty(out, &record)?;
    Ok(())
}
