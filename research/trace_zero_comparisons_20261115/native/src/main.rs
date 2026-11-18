use serde_json::{json, Value};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    time::{Duration, Instant},
};
use trace_zero_comparisons::{arms, execute, summarize};
use trace_zero_experiments::{truth, unwire, wire, Case};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = env::args().skip(1).collect();
    let mode = a.first().ok_or("mode")?;
    if mode == "analyze" {
        let rs: Vec<Value> = BufReader::new(fs::File::open(&a[1])?)
            .lines()
            .map(|l| serde_json::from_str(&l.unwrap()).unwrap())
            .collect();
        println!("{}", serde_json::to_string_pretty(&summarize(&rs))?);
        return Ok(());
    }
    let panel = a.get(1).ok_or("panel")?;
    let names = arms(panel);
    let input: Value = serde_json::from_slice(&fs::read(a.get(2).ok_or("inputs")?)?)?;
    let cases: Vec<Case> = serde_json::from_value(input["cases"].clone())?;
    if mode == "preflight" {
        let mut rows = Vec::new();
        for n in [5, 7, 9, 83, 131] {
            let mut c = cases.iter().find(|c| c.n == n).unwrap().clone();
            let curve = trace_zero_leads::fixture(n);
            let base: Vec<_> = c.base.iter().map(|p| unwire(p, n)).collect();
            let target = full_trace_zero_backend::binary_ecc::curve::scalar_mul(
                &curve,
                &base[0],
                &3u32.into(),
            );
            c.target = wire(&target);
            c.truth_witness = truth(&curve, &base, &target);
            c.truth = c.truth_witness.is_some();
            c.id = format!("pilot-n{n}");
            for arm in names {
                let start = Instant::now();
                let result = execute(&c, panel, arm, Some(Duration::from_millis(750)));
                rows.push(json!({"n":n,"arm":arm,"result":result,"engineering_elapsed_seconds":start.elapsed().as_secs_f64()}));
            }
        }
        println!(
            "{}",
            serde_json::to_string_pretty(
                &json!({"rows":rows,"scope":"Engineering pilot only; n5 all arms must Found/replay, every row valid; no timing-performance claim"})
            )?
        );
    } else if mode == "run" {
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(a.get(3).ok_or("output")?)?;
        let start = Instant::now();
        let mut count = 0;
        for replicate in 0..2 {
            for (i, c) in cases.iter().enumerate() {
                for j in 0..names.len() {
                    if start.elapsed() >= Duration::from_secs(600) {
                        return Err("global guard; retain prefix".into());
                    }
                    let arm = names[(j + i + replicate) % names.len()];
                    let result = execute(c, panel, arm, Some(Duration::from_millis(750)));
                    let valid = result["valid"] == true;
                    let row = json!({"schema":"trace-zero-comparison-row/v1","panel":panel,"arm":arm,"instance":c.id,"n":c.n,"dimension":c.dimension,"kind":c.kind,"seed":c.seed,"replicate":replicate,"warmup":false,
                "status":if valid{"succeeded"}else{"failed"},"verification":if !valid{"refuted"}else if result["outcome"]=="Inconclusive"{"no_claim"}else{"verified"},
                "decided":u64::from(result["outcome"]!="Inconclusive"),"conflicts":result["conflicts"],"peak_variables":result["peak_variables"],"group_ops":result["charged_group_add_double_count"],"result":result,"wall_time_valid":false});
                    writeln!(f, "{}", serde_json::to_string(&row)?)?;
                    f.flush()?;
                    count += 1;
                    eprintln!(
                        "{count}/{} {} {arm} {}",
                        cases.len() * 2 * names.len(),
                        c.id,
                        row["result"]["outcome"]
                    );
                    if !valid {
                        return Err("invalid result; stop".into());
                    }
                }
            }
        }
    } else {
        return Err(
            "usage: preflight PANEL INPUT | run PANEL INPUT NEW_ROWS | analyze ROWS".into(),
        );
    }
    Ok(())
}
