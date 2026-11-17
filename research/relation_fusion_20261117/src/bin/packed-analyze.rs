//! Recompute the declared full-boundary statistics from every raw call, with full replay.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{error::Error, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
include!("../packed_roster.rs");
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> Result<Vec<u8>> {
    let m = fs::symlink_metadata(p)?;
    if !m.is_file() || m.file_type().is_symlink() || m.len() > 16 * 1024 * 1024 {
        return Err("file type/size".into());
    }
    let b = fs::read(p)?;
    if b.len() > 16 * 1024 * 1024 {
        return Err("file growth".into());
    }
    Ok(b)
}
fn value(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&read(p)?)?)
}
fn require(b: bool, s: &str) -> Result<()> {
    if b {
        Ok(())
    } else {
        Err(s.into())
    }
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn quantile(v: &[f64], q: f64) -> f64 {
    v[((v.len() - 1) as f64 * q).floor() as usize]
}
fn bootstrap(v: &[f64]) -> Value {
    let mut state = 0xc159_ea72_b084_d36fu64;
    let mut samples = Vec::with_capacity(20000);
    for _ in 0..20000 {
        let mut sample = Vec::with_capacity(v.len());
        for _ in v {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            sample.push(v[(state % v.len() as u64) as usize]);
        }
        samples.push(median(&sample));
    }
    samples.sort_by(f64::total_cmp);
    json!({"median":median(v),"paired_ratios":v,"pointwise95_low":quantile(&samples,0.025),"pointwise95_high":quantile(&samples,0.975),"six_case_simultaneous_lower":quantile(&samples,0.05/6.0),"resamples":20000})
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage:packed-analyze RAW MONITOR PROSPECTIVE_PINS NEW_OUT".into());
    }
    let raw = Path::new(&args[1]);
    let out = Path::new(&args[4]);
    fs::create_dir(out)?;
    let summary = value(&raw.join("summary.json"))?;
    let rows = summary["outcomes"].as_array().ok_or("outcomes")?;
    let monitors: Vec<Value> = fs::read_to_string(&args[2])?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    require(monitors.len() == 1, "monitor roster")?;
    let uncontended = monitors[0]["exit_status"] == 0 && monitors[0]["contended_samples"] == 0;
    let pins = value(Path::new(&args[3]))?;
    let expected = &pins["worker_sha256"];
    require(
        value(&raw.join("workers-before.json"))? == *expected
            && value(&raw.join("workers-after.json"))? == *expected,
        "worker pins",
    )?;
    let complete = rows.len() == 1260
        && summary["planned"] == 1260
        && summary["all_correct"] == true
        && rows.iter().all(|r| r["status"] == "complete");
    let get = |stage: &str, round: usize, p: u64, n: u64, arm: &str| -> Result<&Value> {
        let found: Vec<_> = rows
            .iter()
            .filter(|r| {
                r["stage"] == stage
                    && r["round"] == round
                    && r["prime"] == p
                    && r["n"] == n
                    && r["arm"] == arm
            })
            .collect();
        require(found.len() == 1, "missing/duplicate cell")?;
        Ok(found[0])
    };
    let mut cases = Vec::new();
    let mut replays = 0usize;
    if complete {
        // Audit the complete immutable roster, raw outputs, original pins and complete proofs.
        for stage in ["AA", "AB"] {
            for round in 0..5 {
                for p in [2147483647u64, 9007199254740881] {
                    for n in [3u64, 4, 6] {
                        for arm in roster(stage) {
                            let row = get(stage, round, p, n, &arm.name)?;
                            require(
                                row["kind"] == arm.kind
                                    && row["kernel"] == arm.kernel
                                    && row["checker"] == "packed",
                                "arm identity",
                            )?;
                            let dir = raw.join(format!("{stage}-r{round}-p{p}-n{n}-{}", arm.name));
                            require(
                                value(&dir.join("outcome.json"))? == *row,
                                "summary/raw binding",
                            )?;
                            let process = value(&dir.join("process.json"))?;
                            require(
                                process["exit"] == 0
                                    && process["driver_sha256"] == expected[arm.worker],
                                "child receipt",
                            )?;
                            let input = read(&dir.join("proof/original.json"))?;
                            let wire = read(&dir.join("proof/witness.json"))?;
                            let input_pin = pins["input_sha256"][format!("p{p}-n{n}")]
                                .as_str()
                                .ok_or("input pin")?;
                            require(
                                sha(&input) == input_pin
                                    && process["input_sha256"] == input_pin
                                    && row["proof_sha256"] == sha(&wire),
                                "original/proof binding",
                            )?;
                            let checked = if arm.kind.ends_with("-dag") {
                                relation_replay_checker::packed::verify_dag_json(&input, &wire)?
                            } else {
                                relation_replay_checker::packed::verify_json(&input, &wire)?
                            };
                            replays += 1;
                            let worker = value(&dir.join("call.stdout"))?;
                            require(
                                worker["basis"] == serde_json::to_value(&checked.basis)?
                                    && worker["consumer_stats"]
                                        == serde_json::to_value(&checked.stats)?
                                    && row["basis_sha256"]
                                        == sha(&serde_json::to_vec(&checked.basis)?),
                                "checked basis/stats",
                            )?;
                            require(
                                read(&dir.join("call.stderr"))?.is_empty(),
                                "child diagnostics",
                            )?;
                            let parent = row["parent_ns"].as_u64().ok_or("parent clock")?;
                            let child = row["child_process_ns"].as_u64().ok_or("child clock")?;
                            let phase = |name: &str| -> Result<u64> {
                                Ok(worker["phases_diagnostic_ns"][name]
                                    .as_str()
                                    .ok_or("worker phase")?
                                    .parse()?)
                            };
                            let construct = phase("construct_and_wire")?;
                            let replay = phase("independent_replay")?;
                            let whole = phase("full_worker_including_proof_write")?;
                            require(
                                worker["status"] == "complete"
                                    && worker["prime"] == p
                                    && worker["variables"] == n
                                    && worker["checker"] == "packed"
                                    && worker["kernel"] == arm.kernel
                                    && worker["S_pair_and_input_containment_checks"] == true,
                                "worker identity/status",
                            )?;
                            require(
                                parent > 0
                                    && child > 0
                                    && parent >= child
                                    && whole <= child
                                    && construct.checked_add(replay).is_some_and(|v| v <= whole)
                                    && row["construct_ns"] == construct
                                    && row["replay_ns"] == replay
                                    && row["worker_ns"] == whole,
                                "complete phase/boundary binding",
                            )?;
                        }
                    }
                }
            }
        }
        for p in [2147483647u64, 9007199254740881] {
            for n in [3u64, 4, 6] {
                let mut aa = Vec::new();
                let mut aa_by_arm = Vec::new();
                for arm in controls() {
                    let mut pairs = Vec::new();
                    for r in 0..5 {
                        pairs.push(
                            get("AA", r, p, n, &arm.name)?["parent_ns"]
                                .as_u64()
                                .ok_or("AA")? as f64
                                / get("AA", r, p, n, &format!("{}-copy", arm.name))?["parent_ns"]
                                    .as_u64()
                                    .ok_or("AA copy")? as f64,
                        );
                    }
                    aa.extend_from_slice(&pairs);
                    aa_by_arm.push(json!({"arm":arm.name,"ratios":pairs}));
                }
                let aa_spread = aa
                    .iter()
                    .copied()
                    .map(|v| v.max(1.0 / v))
                    .fold(1.0, f64::max);
                let mut observations = Vec::new();
                let mut primary = Vec::new();
                let mut strongest_ids = Vec::new();
                let mut basis = None;
                for arm in roster("AB") {
                    let mut parents = Vec::new();
                    let mut constructs = Vec::new();
                    let mut replays_ns = Vec::new();
                    let mut ratios = Vec::new();
                    for r in 0..5 {
                        let row = get("AB", r, p, n, &arm.name)?;
                        let fingerprint = row["basis_sha256"].as_str().ok_or("basis hash")?;
                        require(basis.is_none_or(|b| b == fingerprint), "cross-arm basis")?;
                        basis = Some(fingerprint);
                        let parent = row["parent_ns"].as_u64().ok_or("parent")? as f64;
                        parents.push(parent);
                        constructs.push(row["construct_ns"].as_u64().ok_or("construct")?);
                        replays_ns.push(row["replay_ns"].as_u64().ok_or("replay")?);
                        let mut competitors = Vec::new();
                        for c in controls() {
                            competitors.push((
                                c.name.clone(),
                                get("AB", r, p, n, &c.name)?["parent_ns"]
                                    .as_u64()
                                    .ok_or("control")?,
                            ));
                        }
                        competitors.sort_by_key(|v| v.1);
                        ratios.push(competitors[0].1 as f64 / parent);
                        if arm.name == "packed-dag-487" {
                            strongest_ids.push(competitors[0].0.clone());
                            primary.push(competitors[0].1 as f64 / parent);
                        }
                    }
                    observations.push(json!({"arm":arm.name,"parent_ns":parents,"median_parent_ns":median(&parents),"minimum_parent_ns":parents.iter().copied().min_by(f64::total_cmp),"construct_ns":constructs,"replay_ns":replays_ns,"vs_strongest":bootstrap(&ratios)}));
                }
                let bounds = bootstrap(&primary);
                let pass = uncontended
                    && bounds["six_case_simultaneous_lower"]
                        .as_f64()
                        .ok_or("lower")?
                        > 15.0
                    && bounds["median"].as_f64().ok_or("median")? > aa_spread;
                cases.push(json!({"prime":p,"n":n,"arms":observations,"aa_by_control":aa_by_arm,"aa_symmetric_max_ratio":aa_spread,"primary_packed_dag_487_vs_strongest":bounds,"strongest_arm_each_round":strongest_ids,"passes_15x":pass}));
            }
        }
    }
    let admitted = complete
        && uncontended
        && cases.len() == 6
        && cases.iter().all(|c| c["passes_15x"] == true);
    let report = json!({"schema":"packed-constructor-analysis/v1","planned":1260,"observed":rows.len(),"all_complete":complete,"uncontended":uncontended,"independent_whole_proof_replays":replays,"cases":cases,"primary_15x_admitted":admitted,"actual_root_timing_claim":false,"standing":"pcc:unverified"});
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(out.join("analysis.json"))?
        .write_all(&serde_json::to_vec_pretty(&report)?)?;
    println!(
        "{}",
        json!({"all_complete":complete,"uncontended":uncontended,"whole_proof_replays":replays,"primary_15x_admitted":admitted})
    );
    Ok(())
}
