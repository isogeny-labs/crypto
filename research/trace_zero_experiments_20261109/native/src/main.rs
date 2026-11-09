use serde_json::{json, Value};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    time::{Duration, Instant},
};
use trace_zero_experiments::{execute, prepare, summarize, truth, unwire, wire, Case, ARMS};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("prepare") => {
            let path = args.get(1).ok_or("prepare new-workload.json")?;
            let cases = prepare();
            let f = OpenOptions::new().write(true).create_new(true).open(path)?;
            serde_json::to_writer_pretty(
                f,
                &json!({"schema":"trace-zero-panel-inputs/v1","cases":cases,"arms":ARMS,"replicates":2}),
            )?;
            eprintln!(
                "Prepared{} fixed known-answer instances; no candidate solver run",
                cases.len()
            );
        }
        Some("run") => {
            let path = args.get(1).ok_or("run workload.json new-rows.jsonl")?;
            let output = args.get(2).ok_or("missing output")?;
            let input: Value = serde_json::from_slice(&fs::read(path)?)?;
            let cases: Vec<Case> = serde_json::from_value(input["cases"].clone())?;
            let mut f = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(output)?;
            let start = Instant::now();
            let mut count = 0;
            for replicate in 0..2 {
                for (i, case) in cases.iter().enumerate() {
                    for j in 0..4 {
                        if start.elapsed() > Duration::from_secs(600) {
                            return Err("global600-second guard; retain partial rows".into());
                        }
                        let arm = ARMS[(j + i + replicate) % 4];
                        let result = execute(case, arm);
                        let decided = u64::from(result["outcome"] != "Inconclusive");
                        let row = json!({"schema":"trace-zero-panel-row/v1","arm":arm,"instance":case.id,"n":case.n,
                    "dimension":case.dimension,"kind":case.kind,"seed":case.seed,"replicate":replicate,"warmup":false,
                    "base_size":case.base.len(),"status":if result["valid"]==true{"succeeded"}else{"invalid"},
                    "verification":if result["valid"]==true{"passed"}else{"failed"},
                    "decided":decided,"conflicts":result["conflicts"],"peak_variables":result["peak_variables"],
                    "result":result,"wall_time_valid":false});
                        writeln!(f, "{}", serde_json::to_string(&row)?)?;
                        f.flush()?;
                        count += 1;
                        eprintln!(
                            "{count}/{} {} {arm} {}",
                            cases.len() * 8,
                            case.id,
                            row["result"]["outcome"]
                        );
                        if row["result"]["valid"] != true {
                            return Err("invalid result; stop and preserve all rows".into());
                        }
                    }
                }
            }
        }
        Some("preflight") => {
            let input: Value =
                serde_json::from_slice(&fs::read(args.get(1).ok_or("preflight workload.json")?)?)?;
            let cases: Vec<Case> = serde_json::from_value(input["cases"].clone())?;
            let mut rows = Vec::new();
            for n in [7, 9, 83, 131] {
                let mut case = cases
                    .iter()
                    .find(|c| c.n == n && c.dimension == 4)
                    .unwrap()
                    .clone();
                let curve = trace_zero_leads::fixture(n);
                let base: Vec<_> = case.base.iter().map(|p| unwire(p, n)).collect();
                let target = full_trace_zero_backend::binary_ecc::curve::scalar_mul(
                    &curve,
                    &base[0],
                    &num_bigint::BigUint::from(3u32),
                );
                case.id = format!("pilot-n{n}");
                case.target = wire(&target);
                case.kind = "preflight".into();
                case.truth_witness = truth(&curve, &base, &target);
                case.truth = case.truth_witness.is_some();
                for arm in ARMS {
                    let start = Instant::now();
                    let result = execute(&case, arm);
                    rows.push(json!({"n":n,"arm":arm,"instance":case.id,"result":result,"engineering_elapsed_seconds":start.elapsed().as_secs_f64()}));
                }
            }
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &json!({"scope":"engineering runtime preflight; not isolated timing evidence","rows":rows})
                )?
            );
        }
        Some("analyze") => {
            let f = fs::File::open(args.get(1).ok_or("analyze rows.jsonl")?)?;
            let rows: Vec<Value> = BufReader::new(f)
                .lines()
                .map(|s| serde_json::from_str(&s.unwrap()).unwrap())
                .collect();
            println!("{}", serde_json::to_string_pretty(&summarize(&rows))?);
        }
        _ => {
            return Err(
                "usage: prepare FILE | preflight INPUT | run INPUT NEW_ROWS | analyze ROWS".into(),
            )
        }
    }
    Ok(())
}
