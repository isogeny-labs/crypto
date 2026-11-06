use std::{env, fs, process};

use serde_json::{json, Value};

fn run(args: &[String]) -> Result<Value, String> {
    if args.len() != 4 || args[1] != "verify" {
        return Err("usage: fall-degree-replay verify <problem.json> <proof.json>".to_owned());
    }
    let read = |path: &str| -> Result<Value, String> {
        let input =
            fs::read_to_string(path).map_err(|error| format!("cannot read {path}: {error}"))?;
        serde_json::from_str(&input).map_err(|error| format!("invalid JSON in {path}: {error}"))
    };
    fall_degree_replay::verify(&read(&args[2])?, &read(&args[3])?)
        .map_err(|error| error.to_string())
}

fn main() {
    match run(&env::args().collect::<Vec<_>>()) {
        Ok(report) => println!("{}", serde_json::to_string_pretty(&report).unwrap()),
        Err(message) => {
            println!("{}", json!({"valid_refutation": false, "error": message}));
            process::exit(1);
        }
    }
}
