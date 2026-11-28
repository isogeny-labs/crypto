//! Native recorded-state anchored-descent component. Measurements use Runpod.
#![allow(clippy::duplicate_mod)]
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/anchored_descent.rs"]
mod anchored;
#[path = "../research/extension_ic_graph_20261115/anchored_input.rs"]
mod anchored_input;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_pipeline_collection_check.rs"]
mod collection_check;
#[path = "../research/extension_ic_graph_20261115/anchored_driver.rs"]
mod component;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder_driver.rs"]
mod driver;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/anchored_nagao.rs"]
mod nagao;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder.rs"]
mod remainder;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::Path,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() == 3 && args[0] == "--prepare" {
        let receipt = anchored_input::prepare(Path::new(&args[1]), Path::new(&args[2]))?;
        println!("{receipt}");
        return Ok(());
    }
    if args.len() != 3 {
        return Err(
            "usage: WORKER --prepare FROZEN41_WORKER NEW_INPUT | WORKER MODE INPUT NEW_OUTPUT"
                .into(),
        );
    }
    let source = std::env::var("IC_SOURCE_REVISION")?;
    let bytes = std::fs::read(&args[1])?;
    let result = component::run(&bytes, &args[0], &source)?;
    let mut out = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args[2])?,
    );
    serde_json::to_writer(&mut out, &result)?;
    out.write_all(b"\n")?;
    out.flush()?;
    println!(
        "{}",
        serde_json::json!({"mode":args[0],"residuals":result["trials"].as_array().map(Vec::len),"rank":result["rank"],"verified":result["verified"]})
    );
    Ok(())
}
