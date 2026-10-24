//! Native, bounded Curve Smash six-class research worker.
use std::fs;

use crypto_lib::cryptanalysis::six_class_rho::SixClassJob;

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let job_path = args
        .next()
        .ok_or("usage: six_class_rho JOB.json MAX_WALKERS")?;
    let count: u64 = args
        .next()
        .ok_or("missing MAX_WALKERS")?
        .parse()
        .map_err(|_| "MAX_WALKERS must be a u64")?;
    if args.next().is_some() {
        return Err("too many arguments".into());
    }
    let input = fs::read_to_string(job_path).map_err(|e| e.to_string())?;
    let job: SixClassJob = serde_json::from_str(&input).map_err(|e| e.to_string())?;
    let context = job.build()?;
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
