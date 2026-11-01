//! Native constructive cap-five DAG emission for the declared guarded model.

#[path = "../constructive.rs"]
mod constructive;

use std::{env, error::Error, fs, process};

fn run() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = env::args().collect();
    if !(3..=4).contains(&arguments.len()) {
        return Err("usage: construct_big_prime PROBLEM.json PROOF.json [REPORT.json]".into());
    }
    let problem = fs::read_to_string(&arguments[1])?;
    let (certificate, report) = constructive::construct_json(&problem)?;
    fs::write(&arguments[2], format!("{certificate}\n"))?;
    let report = format!("{}\n", serde_json::to_string_pretty(&report)?);
    if let Some(path) = arguments.get(3) {
        fs::write(path, &report)?;
    }
    print!("{report}");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("construct_big_prime: {error}");
        process::exit(1);
    }
}
