//! Native, bounded Curve Smash six-class research worker.
use std::fs;

use crypto_lib::cryptanalysis::six_class_rho::{ClassDp, SixClassJob};
use crypto_lib::hash::sha256;

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let first = args
        .next()
        .ok_or("usage: six_class_rho [audit] JOB.json MAX_WALKERS|DP.json")?;
    let audit = first == "audit";
    let job_path = if audit {
        args.next().ok_or("missing JOB.json")?
    } else {
        first
    };
    let operand = args.next().ok_or("missing MAX_WALKERS or DP.json")?;
    if args.next().is_some() {
        return Err("too many arguments".into());
    }
    let input = fs::read(job_path).map_err(|e| e.to_string())?;
    let job: SixClassJob = serde_json::from_slice(&input).map_err(|e| e.to_string())?;
    let context = job.build()?;
    if audit {
        let record_bytes = fs::read(operand).map_err(|e| e.to_string())?;
        let dp: ClassDp = serde_json::from_slice(&record_bytes).map_err(|e| e.to_string())?;
        context.verify_trail(&dp)?;
        println!(
            "{}",
            serde_json::json!({
                "schema": "crypto.sixclass.trail-audit.v1",
                "job_sha256": hex::encode(sha256(&input)),
                "record_sha256": hex::encode(sha256(&record_bytes)),
                "verified": true,
                "walker": dp.walker,
                "steps": dp.steps
            })
        );
        return Ok(());
    }
    let count: u64 = operand.parse().map_err(|_| "MAX_WALKERS must be a u64")?;
    let result = context.campaign(count);
    println!(
        "{}",
        serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("six_class_rho: {error}");
        std::process::exit(2);
    }
}
