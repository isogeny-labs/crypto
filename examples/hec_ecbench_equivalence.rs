//! Compare an ecbench session of the `hec.*` methods with the standalone
//! hyperelliptic study it ports, run for run, in `bitop/v1`.
//!
//! Each record's arm carries its study seed (`fixed_seed`); its `work`
//! total (repriced from its counts) must equal the study's
//! `bitops_per_run` entry for that seed on the same `p` and genus.
//!
//! ```sh
//! cargo run --release --example hec_ecbench_equivalence -- \
//!     research/hec_bitop_20261020/raw/g2.jsonl 41 \
//!     research/ic_acct3_hec_ecbench_20261021/sessions/g2-p41
//! ```
//!
//! Prints a Markdown table and a JSON summary line; exits 1 on any
//! mismatch.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{json, Value};

use crypto_lib::cryptanalysis::ecbench::runner::read_records;
use crypto_lib::cryptanalysis::ecbench::work::price_record;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        eprintln!("usage: hec_ecbench_equivalence <study.jsonl> <p> <session dir>");
        std::process::exit(2);
    }
    let p: u64 = args[1].parse().expect("p");
    let row: Value = std::fs::read_to_string(&args[0])
        .expect("study rows")
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).expect("json"))
        .find(|v| v["p"].as_u64() == Some(p))
        .expect("a row for p");
    // (method, seed) -> the study's per-run total.
    let mut study: BTreeMap<(String, u64), u128> = BTreeMap::new();
    for (arm, method) in [("ic", "hec.ic_fb_walk"), ("rho", "hec.rho_full_width")] {
        let seeds = row[arm]["seeds"].as_array().expect("seeds");
        let per = row[arm]["bitops_per_run"].as_array().expect("per run");
        for (s, b) in seeds.iter().zip(per) {
            study.insert(
                (method.to_string(), s.as_u64().expect("seed")),
                b.as_str().expect("decimal").parse().expect("u128"),
            );
        }
    }
    let records = read_records(Path::new(&args[2])).expect("session records");
    println!("| arm | method | seed | status | ecbench bitop/v1 | study bitop/v1 | equal |");
    println!("|---|---|---:|---|---:|---:|---|");
    let (mut equal, mut differ, mut missing) = (0usize, 0usize, 0usize);
    let mut sums: BTreeMap<String, (u128, u128, usize)> = BTreeMap::new();
    // (method, phase) -> gates summed over runs.
    let mut phase_sums: BTreeMap<(String, String), u128> = BTreeMap::new();
    for r in &records {
        for ph in price_record(r).expect("priced").phases {
            *phase_sums
                .entry((r.method.id.clone(), ph.name))
                .or_default() += ph.bitops.parse::<u128>().expect("u128");
        }
        let seed: u64 = r.method.params["fixed_seed"].parse().expect("pinned seed");
        let ours: u128 = price_record(r)
            .expect("priced")
            .bitops
            .parse()
            .expect("u128");
        let theirs = study.get(&(r.method.id.clone(), seed)).copied();
        let same = theirs == Some(ours) && r.outcome.status == "verified";
        match theirs {
            None => missing += 1,
            Some(_) if same => equal += 1,
            Some(_) => differ += 1,
        }
        let e = sums.entry(r.method.id.clone()).or_default();
        e.0 += ours;
        e.1 += theirs.unwrap_or(0);
        e.2 += 1;
        println!(
            "| {} | {} | {} | {} | {} | {} | {} |",
            r.arm,
            r.method.id,
            seed,
            r.outcome.status,
            ours,
            theirs.map_or("-".into(), |v| v.to_string()),
            if same { "yes" } else { "NO" }
        );
    }
    let means: BTreeMap<String, Value> = sums
        .iter()
        .map(|(m, (a, b, n))| {
            (
                m.clone(),
                json!({"runs": n, "ecbench_sum": a.to_string(), "study_sum": b.to_string(),
                       "ecbench_mean": *a as f64 / *n as f64, "study_mean": *b as f64 / *n as f64}),
            )
        })
        .collect();
    println!();
    println!("| method | phase | ecbench Σ bitop/v1 | study Σ bitop/v1 | equal |");
    println!("|---|---|---:|---:|---|");
    let mut phases_equal = true;
    for (arm, method) in [("ic", "hec.ic_fb_walk"), ("rho", "hec.rho_full_width")] {
        let study_phases = row[arm]["phases_summed_over_runs"]
            .as_object()
            .expect("phases");
        for (name, v) in study_phases {
            if name == "total" {
                continue;
            }
            let theirs: u128 = v["bitops"]
                .as_str()
                .expect("decimal")
                .parse()
                .expect("u128");
            let ours = phase_sums
                .get(&(method.to_string(), name.clone()))
                .copied()
                .unwrap_or(0);
            phases_equal &= ours == theirs;
            println!(
                "| {method} | {name} | {ours} | {theirs} | {} |",
                if ours == theirs { "yes" } else { "NO" }
            );
        }
    }
    let summary = json!({
        "phases_equal": phases_equal,
        "schema": "hec_ecbench_equivalence/v1",
        "p": p,
        "records": records.len(),
        "equal": equal,
        "differ": differ,
        "missing_from_study": missing,
        "by_method": means,
        "equivalent": differ == 0 && missing == 0 && equal == study.len() && phases_equal,
    });
    println!();
    println!("{}", serde_json::to_string(&summary).expect("json"));
    if summary["equivalent"] != true {
        std::process::exit(1);
    }
}
