mod arms;
mod curve;
mod exact;
mod field;
mod graph;
mod poly;
mod rng;
#[cfg(test)]
mod tests;

use arms::*;
use std::io::Write;

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: l09 census|cell|exact ...");
        std::process::exit(2);
    }
    let p: u64 = arg(&args, "--p").expect("--p").parse().unwrap();
    let seeds: u64 = arg(&args, "--seeds").expect("--seeds").parse().unwrap();
    let from: u64 = arg(&args, "--seed-from").map(|s| s.parse().unwrap()).unwrap_or(0);
    let out = match args[1].as_str() {
        "census" => census_output(p, from, seeds),
        "cell" | "exact" => {
            let b: usize = arg(&args, "--beta-idx").expect("--beta-idx").parse().unwrap();
            let arms_s = arg(&args, "--arms").expect("--arms");
            let arms: Vec<Arm> = arms_s.split(',').map(Arm::parse).collect();
            cell_output(p, b, from, seeds, &arms, args[1] == "exact")
        }
        _ => panic!("unknown command"),
    };
    std::io::stdout().write_all(out.as_bytes()).unwrap();
}
