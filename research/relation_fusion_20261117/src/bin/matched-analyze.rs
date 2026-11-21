//! Analyze every preregistered observation; never discard or replace a call.
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn median(v: &[f64]) -> f64 {
    let mut sorted = v.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted[sorted.len() / 2]
}
fn quantile(sorted: &[f64], q: f64) -> f64 {
    sorted[((sorted.len() - 1) as f64 * q).floor() as usize]
}
fn bootstrap(values: &[f64]) -> Value {
    let mut state = 0xc159_ea72_b084_d36fu64;
    let mut samples = Vec::with_capacity(20000);
    for _ in 0..20000 {
        let mut sample = Vec::with_capacity(values.len());
        for _ in values {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            sample.push(values[(state % values.len() as u64) as usize]);
        }
        samples.push(median(&sample));
    }
    samples.sort_by(f64::total_cmp);
    json!({"median":median(values),"paired_ratios":values,
        "pointwise95_low":quantile(&samples,0.025),"pointwise95_high":quantile(&samples,0.975),
        "six_case_simultaneous_lower":quantile(&samples,0.05/6.0),"resamples":20000})
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage:matched-analyze RAW_DIR ISOLATION_JSONL NEW_OUT".into());
    }
    let raw = Path::new(&args[1]);
    let out = Path::new(&args[3]);
    fs::create_dir(out)?;
    let summary: Value = serde_json::from_slice(&fs::read(raw.join("summary.json"))?)?;
    let rows = summary["outcomes"].as_array().ok_or("outcomes")?;
    let monitors: Vec<Value> = fs::read_to_string(&args[2])?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    if monitors.len() != 1 {
        return Err("exactly one isolation run required".into());
    }
    let uncontended = monitors[0]["exit_status"] == 0 && monitors[0]["contended_samples"] == 0;
    let complete = rows.len() == 240
        && summary["all_correct"] == true
        && rows.iter().all(|r| r["status"] == "complete");
    let get = |stage: &str, round: usize, p: u64, n: u64, arm: &str| -> Result<&Value> {
        let matched: Vec<_> = rows
            .iter()
            .filter(|r| {
                r["stage"] == stage
                    && r["round"] == round
                    && r["prime"] == p
                    && r["n"] == n
                    && r["arm"] == arm
            })
            .collect();
        if matched.len() != 1 || matched[0]["status"] != "complete" {
            return Err("missing/duplicate/failed cell".into());
        }
        Ok(matched[0])
    };
    let mut cells = vec![];
    let mut md = String::from("# Repeated complete-certificate comparison85\n\nAll times below are parent milliseconds including controller replay. Five observations per AB arm and five A/A pairs per case; no trimming. [pcc:unverified]\n\n|Field|Variables|Arm|Parent median ms|Parent minimum ms|Construction median ms|Replay median ms|\n|---|---:|---|---:|---:|---:|---:|\n");
    if complete {
        for p in [2147483647u64, 9007199254740881] {
            for n in [3u64, 4, 6] {
                let arms = ["frontier", "products", "batch", "flat", "dag", "pruned"];
                let mut aa = vec![];
                for round in 0..5 {
                    let a = get("AA", round, p, n, "frontier-A")?["parent_ns"]
                        .as_u64()
                        .ok_or("AA clock")?;
                    let b = get("AA", round, p, n, "frontier-A-copy")?["parent_ns"]
                        .as_u64()
                        .ok_or("AA clock")?;
                    aa.push(a as f64 / b as f64);
                }
                let mut observations = vec![];
                let mut batch_strongest = vec![];
                let mut strongest_ids = vec![];
                let mut fingerprint = None;
                for arm in arms {
                    let mut parents = vec![];
                    let mut constructs = vec![];
                    let mut replays = vec![];
                    let mut paired = vec![];
                    for round in 0..5 {
                        let r = get("AB", round, p, n, arm)?;
                        let basis = r["basis_sha256"].as_str().ok_or("basis fingerprint")?;
                        if fingerprint.is_some_and(|v| v != basis) {
                            return Err("cross-arm basis mismatch".into());
                        }
                        fingerprint = Some(basis);
                        let parent = r["parent_ns"].as_u64().ok_or("parent")? as f64;
                        parents.push(parent);
                        constructs.push(r["construct_ns"].as_u64().ok_or("construct")? as f64);
                        replays.push(r["replay_ns"].as_u64().ok_or("replay")? as f64);
                        paired.push(
                            get("AB", round, p, n, "frontier")?["parent_ns"]
                                .as_u64()
                                .ok_or("control")? as f64
                                / parent,
                        );
                        if arm == "batch" {
                            let mut competitors = vec![];
                            for other in arms.iter().filter(|s| **s != "batch") {
                                competitors.push((
                                    *other,
                                    get("AB", round, p, n, other)?["parent_ns"]
                                        .as_u64()
                                        .ok_or("competitor")?,
                                ));
                            }
                            competitors.sort_by_key(|v| v.1);
                            strongest_ids.push(competitors[0].0);
                            batch_strongest.push(competitors[0].1 as f64 / parent);
                        }
                    }
                    let minimum = parents
                        .iter()
                        .copied()
                        .min_by(f64::total_cmp)
                        .ok_or("minimum")?;
                    md += &format!(
                        "|{p}|{n}|{arm}|{:.6}|{:.6}|{:.6}|{:.6}|\n",
                        median(&parents) / 1e6,
                        minimum / 1e6,
                        median(&constructs) / 1e6,
                        median(&replays) / 1e6
                    );
                    observations.push(json!({"arm":arm,"parent_ns":parents,"median_parent_ns":median(&parents),"minimum_parent_ns":minimum,"construct_ns":constructs,"replay_ns":replays,"vs_frontier":bootstrap(&paired)}));
                }
                let bounds = bootstrap(&batch_strongest);
                let aa_spread = aa
                    .iter()
                    .copied()
                    .map(|v| v.max(1.0 / v))
                    .fold(1.0, f64::max);
                let passed = uncontended
                    && bounds["six_case_simultaneous_lower"]
                        .as_f64()
                        .ok_or("lower")?
                        > 15.0
                    && bounds["median"].as_f64().ok_or("median")? > aa_spread;
                cells.push(json!({"prime":p,"n":n,"arms":observations,"aa_ratios":aa,"aa_symmetric_max_ratio":aa_spread,"batch_vs_strongest":bounds,"strongest_arm_each_round":strongest_ids,"passes_15x":passed}));
            }
        }
    }
    let admitted = complete
        && uncontended
        && cells.len() == 6
        && cells.iter().all(|v| v["passes_15x"] == true);
    let result = json!({"schema":"fusion-matched-analysis/v1","planned":240,"observed":rows.len(),"all_complete":complete,"uncontended":uncontended,"cases":cells,"batch_15x_admitted":admitted,"goal":"active unless separately accepted","evidence_status":"pcc:unverified"});
    md+=&format!("\nAll planned calls complete: {complete}. Uncontended: {uncontended}. Batch15x admitted across all six declared cases: {admitted}. See analysis.json for every raw paired ratio, bootstrap interval, simultaneous lower bound and A/A range.\n");
    fs::write(
        out.join("analysis.json"),
        serde_json::to_vec_pretty(&result)?,
    )?;
    fs::write(out.join("report.md"), md)?;
    Ok(())
}
