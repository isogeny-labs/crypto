use norm_branch_census::{analyze_decision, run_experiment};
use std::{fs::OpenOptions, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: produce OUTPUT_JSON SOURCE_REVISION PROTOCOL_SHA256".into());
    }
    let result = run_experiment(args[2].clone(), args[3].clone());
    let bytes = serde_json::to_vec_pretty(&result)?;
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[1])?;
    file.write_all(&bytes)?;
    file.write_all(b"\n")?;
    let (verdict, reason) = analyze_decision(&result);
    println!(
        "cells={} verdict={} reason={}",
        result.cells.len(),
        verdict,
        reason
    );
    Ok(())
}
