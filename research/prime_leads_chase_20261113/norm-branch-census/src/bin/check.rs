use norm_branch_census::{analyze_decision, check_experiment, ExperimentResult};
use std::{fs::OpenOptions, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: check INPUT_JSON RECEIPT_JSON".into());
    }
    let input = std::fs::read(&args[1])?;
    let result: ExperimentResult = serde_json::from_slice(&input)?;
    let receipt = check_experiment(&result);
    let bytes = serde_json::to_vec_pretty(&receipt)?;
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[2])?;
    file.write_all(&bytes)?;
    file.write_all(b"\n")?;
    if !receipt.pass {
        return Err(format!(
            "independent checker failed with {} errors",
            receipt.errors.len()
        )
        .into());
    }
    let (verdict, reason) = analyze_decision(&result);
    println!(
        "checks={} verdict={} reason={}",
        receipt.split_ranks_checked, verdict, reason
    );
    Ok(())
}
