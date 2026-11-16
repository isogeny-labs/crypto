use serde_json::{json, Value};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    time::{Duration, Instant},
};
use trace_zero_experiments::Case;
use trace_zero_linearized::{execute, prepare, ARMS};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = env::args().skip(1).collect();
    let mode = a.first().ok_or("mode")?;
    if mode == "analyze" {
        let rows: Vec<Value> = BufReader::new(fs::File::open(&a[1])?)
            .lines()
            .map(|l| serde_json::from_str(&l.unwrap()).unwrap())
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&trace_zero_linearized::summarize(&rows))?
        );
        return Ok(());
    }
    let input: Value = serde_json::from_slice(&fs::read(&a[1])?)?;
    let cases: Vec<Case> = serde_json::from_value(input["cases"].clone())?;
    if mode == "prepare" {
        let cases = prepare(cases);
        let f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&a[2])?;
        serde_json::to_writer_pretty(
            f,
            &json!({"schema":"trace-zero-terminal-inputs/v1","cases":cases,"arms":ARMS,"replicates":2}),
        )?;
    } else if mode == "preflight" {
        let mut rows = Vec::new();
        for (n, d) in [
            (5, 0),
            (7, 4),
            (9, 4),
            (83, 5),
            (83, 8),
            (83, 12),
            (131, 5),
            (131, 8),
            (131, 12),
        ] {
            let mut c = cases
                .iter()
                .find(|c| c.n == n && c.dimension == d)
                .unwrap()
                .clone();
            let curve = trace_zero_leads::fixture(n);
            let b: Vec<_> = c
                .base
                .iter()
                .map(|p| trace_zero_experiments::unwire(p, n))
                .collect();
            let t = if n == 5 {
                full_trace_zero_backend::binary_ecc::curve::scalar_mul(&curve, &b[0], &3u32.into())
            } else {
                use num_bigint::RandBigInt;
                use rand::{rngs::StdRng, SeedableRng};
                let mut rng = StdRng::seed_from_u64(202611154000 + 100 * n as u64 + d as u64);
                let g = if curve.generator
                    == full_trace_zero_backend::binary_ecc::BinaryPoint::Infinity
                {
                    &b[0]
                } else {
                    &curve.generator
                };
                full_trace_zero_backend::binary_ecc::curve::scalar_mul(
                    &curve,
                    g,
                    &rng.gen_biguint_below(&curve.order),
                )
            };
            c.target = trace_zero_experiments::wire(&t);
            c.truth_witness = trace_zero_experiments::truth(&curve, &b, &t);
            c.truth = c.truth_witness.is_some();
            for arm in ARMS {
                let start = Instant::now();
                let r = execute(&c, arm, Some(Duration::from_millis(750)));
                rows.push(json!({"n":n,"dimension":d,"arm":arm,"result":r,"engineering_elapsed_seconds":start.elapsed().as_secs_f64()}));
            }
        }
        println!("{}", serde_json::to_string_pretty(&json!({"rows":rows}))?);
    } else if mode == "run" {
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&a[2])?;
        let start = Instant::now();
        let mut count = 0;
        for replicate in 0..2 {
            for (i, c) in cases.iter().enumerate() {
                for j in 0..ARMS.len() {
                    if start.elapsed() >= Duration::from_secs(600) {
                        return Err("global guard; retain prefix".into());
                    }
                    let arm = ARMS[(i + replicate + j) % ARMS.len()];
                    let r = execute(c, arm, Some(Duration::from_millis(750)));
                    let valid = r["valid"] == true;
                    let row = json!({"schema":"trace-zero-linearized-row/v1","panel":"linearized","arm":arm,"instance":c.id,"n":c.n,"dimension":c.dimension,"kind":c.kind,"seed":c.seed,"replicate":replicate,"warmup":false,
        "status":if valid{"succeeded"}else{"failed"},"verification":if !valid{"refuted"}else if r["outcome"]=="Inconclusive"{"no_claim"}else{"verified"},
        "decided":u64::from(r["outcome"]!="Inconclusive"),"conflicts":r["conflicts"],"peak_variables":r["peak_variables"],"linear_columns":r["linear_columns"],"group_ops":r["charged_group_add_double_count"],"result":r,"wall_time_valid":false});
                    writeln!(f, "{}", serde_json::to_string(&row)?)?;
                    f.flush()?;
                    count += 1;
                    eprintln!(
                        "{count}/{} {} {arm} {}",
                        cases.len() * 10,
                        c.id,
                        row["result"]["outcome"]
                    );
                    if !valid {
                        return Err("invalid decision; stop".into());
                    }
                }
            }
        }
    } else {
        return Err(
            "usage: prepare ORIGINAL NEW | preflight INPUT | run INPUT NEW_ROWS | analyze ROWS"
                .into(),
        );
    }
    Ok(())
}
