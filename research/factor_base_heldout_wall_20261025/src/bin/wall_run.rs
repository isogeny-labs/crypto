//! Frozen serial schedule for the N12 paired wall experiment.
use serde_json::{json, Value};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn unix_ns() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
}
fn io_some_avg10() -> Option<f64> {
    let pressure = fs::read_to_string("/proc/pressure/io").ok()?;
    pressure.lines().find(|line| line.starts_with("some "))?
        .split_whitespace().find_map(|token| token.strip_prefix("avg10="))?
        .parse().ok()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "usage: wall-run HELDOUT-WALL-BINARY OUTDIR");
    let binary = &args[1];
    let outdir = Path::new(&args[2]);
    fs::create_dir_all(outdir.join("raw")).unwrap();
    let mut receipt = OpenOptions::new().create_new(true).write(true)
        .open(outdir.join("processes.jsonl")).expect("fresh output directory required");
    let schedule = [
        ("aa1", 0, "rho"), ("aa2", 0, "rho"),
        ("ab1a", 1, "candidate"), ("ab1b", 1, "rho"),
        ("ab2a", 2, "rho"), ("ab2b", 2, "candidate"),
        ("ab3a", 3, "candidate"), ("ab3b", 3, "rho"),
        ("ab4a", 4, "rho"), ("ab4b", 4, "candidate"),
        ("ab5a", 5, "candidate"), ("ab5b", 5, "rho"),
    ];
    let mut any_failure = false;
    for (slot, round, arm) in schedule {
        let output_name = format!("{slot}-{arm}.jsonl");
        let stdout = File::create(outdir.join("raw").join(&output_name)).unwrap();
        let stderr = File::create(outdir.join("raw").join(format!("{slot}-{arm}.stderr"))).unwrap();
        let io_before = io_some_avg10();
        if !io_before.is_some_and(|x| x <= 5.0) {
            let record = json!({"schema":"factor-base-heldout-wall-process/v1", "slot":slot,
                "round":round,"arm":arm,"start_unix_ns":null,"wall_ns":null,
                "exit_code":null,"rows":0,"verified":0,"parse_errors":0,"passed":false,
                "io_some_avg10_before":io_before,"io_some_avg10_after":null,
                "raw_file":format!("raw/{output_name}"),"error":"io_pressure_preflight"});
            writeln!(receipt, "{record}").unwrap();
            receipt.flush().unwrap();
            any_failure = true;
            break;
        }
        let start_unix_ns = unix_ns();
        let start = Instant::now();
        let status = Command::new(binary).arg(arm)
            .env("RAYON_NUM_THREADS", "1")
            .stdout(Stdio::from(stdout)).stderr(Stdio::from(stderr))
            .status();
        let wall_ns = start.elapsed().as_nanos();
        let io_after = io_some_avg10();
        let mut rows = 0;
        let mut verified = 0;
        let mut parse_errors = 0;
        for line in BufReader::new(File::open(outdir.join("raw").join(&output_name)).unwrap()).lines() {
            match line.ok().and_then(|s| serde_json::from_str::<Value>(&s).ok()) {
                Some(row) => { rows += 1; if row["verified"] == true { verified += 1; } },
                None => parse_errors += 1,
            }
        }
        let exit_code = status.as_ref().ok().and_then(|s| s.code());
        let passed = exit_code == Some(0) && rows == 420 && verified == 420 && parse_errors == 0
            && io_after.is_some_and(|x| x <= 5.0);
        any_failure |= !passed;
        let record = json!({"schema":"factor-base-heldout-wall-process/v1", "slot":slot,
            "round":round,"arm":arm,"start_unix_ns":start_unix_ns.to_string(),
            "wall_ns":wall_ns.to_string(),"exit_code":exit_code,"rows":rows,
            "verified":verified,"parse_errors":parse_errors,"passed":passed,
            "io_some_avg10_before":io_before,"io_some_avg10_after":io_after,
            "raw_file":format!("raw/{output_name}")});
        writeln!(receipt, "{record}").unwrap();
        receipt.flush().unwrap();
        if !passed { break; }
    }
    if any_failure { std::process::exit(1); }
}
