//! Replay exact numeric-JSON certificates at arbitrary-precision moduli.

use serde_json::{json, Value};
use std::{env, fs::File, io::Read, process};

fn run(args: &[String]) -> Result<Value, String> {
    if args.len() != 4 || args[1] != "verify" {
        return Err("usage: big_prime_replay verify <problem.json> <proof.json>".to_owned());
    }
    let read = |path: &str| -> Result<String, String> {
        let file = File::open(path).map_err(|error| format!("cannot read {path}: {error}"))?;
        let limit = fall_degree_replay::bigprime::MAX_DOCUMENT_BYTES;
        let mut contents = String::new();
        file.take((limit + 1) as u64)
            .read_to_string(&mut contents)
            .map_err(|error| format!("cannot read {path}: {error}"))?;
        if contents.len() > limit {
            return Err(format!("{path} exceeds the document byte limit"));
        }
        Ok(contents)
    };
    fall_degree_replay::bigprime::verify_json(&read(&args[2])?, &read(&args[3])?)
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
