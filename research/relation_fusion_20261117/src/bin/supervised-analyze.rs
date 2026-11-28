//! Preparatory analysis for the matched native/wrapped launcher comparison.
//! No native-launcher performance experiment has been collected.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{error::Error, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
include!("../supervised_roster.rs");
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
fn strongest_control(mut get: impl FnMut(&SupervisedArm) -> Result<u64>) -> Result<(String, u64)> {
    let mut best = None;
    for arm in supervised_controls() {
        let elapsed = get(&arm)?;
        require(elapsed > 0, "positive control clock")?;
        if best.as_ref().is_none_or(|(_, old)| elapsed < *old) {
            best = Some((arm.name, elapsed));
        }
    }
    best.ok_or_else(|| "empty control roster".into())
}
fn clears_thresholds(bounds: &Value, aa_spread: f64, uncontended: bool) -> Result<bool> {
    let lower = bounds["six_case_simultaneous_lower"]
        .as_f64()
        .ok_or("lower bound")?;
    let median = bounds["median"].as_f64().ok_or("median ratio")?;
    require(
        lower.is_finite() && median.is_finite() && aa_spread.is_finite() && aa_spread >= 1.0,
        "finite bounds/noise envelope",
    )?;
    Ok(uncontended && lower > 15.0 && median > aa_spread)
}
fn clocks_bound(row: &Value, process: &Value, construct: u64, replay: u64, whole: u64) -> bool {
    let Some(parent) = row["parent_ns"].as_u64() else {
        return false;
    };
    let Some(child) = row["child_process_ns"].as_u64() else {
        return false;
    };
    parent > 0
        && child > 0
        && parent >= child
        && whole <= child
        && process["parent_ns"].as_u64() == Some(child)
        && construct.checked_add(replay).is_some_and(|v| v <= whole)
        && row["construct_ns"].as_u64() == Some(construct)
        && row["replay_ns"].as_u64() == Some(replay)
        && row["worker_ns"].as_u64() == Some(whole)
}
fn all_six_cases_pass(cases: &[Value], complete: bool, uncontended: bool) -> bool {
    complete && uncontended && cases.len() == 6 && cases.iter().all(|c| c["passes_15x"] == true)
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage:supervised-analyze RAW MONITOR PROSPECTIVE_PINS NEW_OUT".into());
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
    require(
        pins["schema"] == "supervised-match-pins/v1",
        "prospective pin schema",
    )?;
    let saved_pins = read(&raw.join("prospective-pins.json"))?;
    require(
        saved_pins == read(Path::new(&args[3]))?
            && summary["prospective_pins_sha256"] == sha(&saved_pins)
            && summary["schema"] == "fusion-supervised-match/v1"
            && summary["primary"] == PRIMARY,
        "prospective declaration/raw binding",
    )?;
    let expected = &pins["worker_sha256"];
    let synthetic_fixture =
        summary["synthetic_fixture"] == true || pins["synthetic_fixture"] == true;
    require(
        value(&raw.join("workers-before.json"))? == *expected
            && value(&raw.join("workers-after.json"))? == *expected,
        "worker pins",
    )?;
    let complete = rows.len() == PLANNED_CALLS
        && summary["planned"] == PLANNED_CALLS
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
    let mut checked_bases = std::collections::BTreeMap::new();
    if complete {
        // Audit the complete immutable roster, raw outputs, original pins and complete proofs.
        for stage in ["AA", "AB"] {
            for round in 0..ROUNDS {
                for p in [2147483647u64, 9007199254740881] {
                    for n in [3u64, 4, 6] {
                        for arm in supervised_roster(stage)? {
                            let row = get(stage, round, p, n, &arm.name)?;
                            require(
                                row["native"] == arm.native
                                    && row["seed"] == 7
                                    && row["graph"] == "LAST-star"
                                    && row.get("observed_solving_degree") == Some(&Value::Null)
                                    && row["kind"] == arm.kind
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
                                process["native"] == arm.native
                                    && process["timed_out"] == false
                                    && process["resource_envelope"]
                                        == json!({"cpu":0,"threads":1,"address_space_bytes":2147483648u64,"file_bytes":16777216u64,"deadline_seconds":30})
                                    && process["exit"] == 0
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
                            let basis = serde_json::to_value(&checked.basis)?;
                            if let Some(old) = checked_bases.insert((p, n), basis.clone()) {
                                require(old == basis, "all-stage checked basis identity")?;
                            }
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
                                clocks_bound(row, &process, construct, replay, whole),
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
                for arm in supervised_controls() {
                    let mut pairs = Vec::new();
                    for r in 0..ROUNDS {
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
                for arm in supervised_roster("AB")? {
                    let mut parents = Vec::new();
                    let mut constructs = Vec::new();
                    let mut replays_ns = Vec::new();
                    let mut ratios = Vec::new();
                    for r in 0..ROUNDS {
                        let row = get("AB", r, p, n, &arm.name)?;
                        let fingerprint = row["basis_sha256"].as_str().ok_or("basis hash")?;
                        require(basis.is_none_or(|b| b == fingerprint), "cross-arm basis")?;
                        basis = Some(fingerprint);
                        let parent = row["parent_ns"].as_u64().ok_or("parent")? as f64;
                        parents.push(parent);
                        constructs.push(row["construct_ns"].as_u64().ok_or("construct")?);
                        replays_ns.push(row["replay_ns"].as_u64().ok_or("replay")?);
                        let (strongest_name, strongest_ns) = strongest_control(|c| {
                            Ok(get("AB", r, p, n, &c.name)?["parent_ns"]
                                .as_u64()
                                .ok_or("control")?)
                        })?;
                        ratios.push(strongest_ns as f64 / parent);
                        if arm.name == PRIMARY {
                            strongest_ids.push(strongest_name);
                            primary.push(strongest_ns as f64 / parent);
                        }
                    }
                    observations.push(json!({"arm":arm.name,"parent_ns":parents,"median_parent_ns":median(&parents),"minimum_parent_ns":parents.iter().copied().min_by(f64::total_cmp),"construct_ns":constructs,"replay_ns":replays_ns,"vs_strongest":bootstrap(&ratios)}));
                }
                let bounds = bootstrap(&primary);
                let pass = clears_thresholds(&bounds, aa_spread, uncontended)?;
                cases.push(json!({"prime":p,"n":n,"arms":observations,"aa_by_control":aa_by_arm,"aa_symmetric_max_ratio":aa_spread,"primary_vs_strongest":bounds,"strongest_arm_each_round":strongest_ids,"passes_15x":pass}));
            }
        }
    }
    let admitted = all_six_cases_pass(&cases, complete, uncontended);
    // A draft cannot promote threshold arithmetic into an admission. Reconcile
    // this source with the canonical protocol/export checker before freezing
    // a separate executable for the actual prospective comparison.
    let report = json!({"schema":"supervised-match-analysis-draft/v1","input_declares_synthetic_fixture":synthetic_fixture,"planned":PLANNED_CALLS,"primary":PRIMARY,"observed":rows.len(),"all_complete":complete,"uncontended":uncontended,"independent_whole_proof_replays":replays,"cases":cases,"threshold_arithmetic_passes":admitted,"primary_15x_admitted":false,"admission_blockers":["Preparatory source; canonical protocol/export validation is pending"],"actual_root_timing_claim":false,"standing":"pcc:unverified"});
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(out.join("analysis.json"))?
        .write_all(&serde_json::to_vec_pretty(&report)?)?;
    println!(
        "{}",
        json!({"all_complete":complete,"uncontended":uncontended,"whole_proof_replays":replays,"threshold_arithmetic_passes":admitted,"primary_15x_admitted":false})
    );
    Ok(())
}

#[cfg(test)]
mod statistical_controls {
    use super::*;

    #[test]
    fn fastest_native_control_prevents_a_wrapper_only_false_gain() {
        // Synthetic nanoseconds, not measured timings. The wrapped baseline
        // would suggest 20x over candidate=100; an eligible native control=80
        // instead makes the strongest-control ratio 0.8.
        let mut visited = Vec::new();
        let (name, time) = strongest_control(|arm| {
            visited.push(arm.name.clone());
            Ok(if arm.name == "current-dag-487-native" {
                80
            } else {
                2000
            })
        })
        .unwrap();
        assert_eq!(visited.len(), 24);
        assert_eq!(name, "current-dag-487-native");
        let bounds = bootstrap(&[time as f64 / 100.0; 5]);
        assert!(!clears_thresholds(&bounds, 1.0, true).unwrap());
    }

    #[test]
    fn fixed_bootstrap_and_noise_controls_reject_false_successes() {
        let unchanged = bootstrap(&[1.0; 5]);
        assert_eq!(unchanged["six_case_simultaneous_lower"], 1.0);
        assert!(!clears_thresholds(&unchanged, 1.0, true).unwrap());
        let fifteen = bootstrap(&[15.0; 5]);
        assert!(!clears_thresholds(&fifteen, 1.0, true).unwrap());
        let twenty = bootstrap(&[20.0; 5]);
        assert_eq!(twenty["six_case_simultaneous_lower"], 20.0);
        assert!(clears_thresholds(&twenty, 1.0, true).unwrap());
        assert!(!clears_thresholds(&twenty, 20.0, true).unwrap());
        assert!(!clears_thresholds(&twenty, 1.0, false).unwrap());
        let unstable = bootstrap(&[1.0, 1.0, 20.0, 20.0, 20.0]);
        assert!(!clears_thresholds(&unstable, 1.0, true).unwrap());
    }

    #[test]
    fn a_missing_or_zero_control_cannot_be_ignored() {
        assert!(strongest_control(|arm| {
            if arm.name == "frozen85-sparse-103-native" {
                Err("missing".into())
            } else {
                Ok(2000)
            }
        })
        .is_err());
        assert!(strongest_control(|_| Ok(0)).is_err());
        assert!(clears_thresholds(&bootstrap(&[20.0; 5]), f64::NAN, true).is_err());
    }

    #[test]
    fn clocks_must_match_the_raw_receipt_and_every_worker_phase() {
        let row = json!({"parent_ns":1000,"child_process_ns":800,
                         "construct_ns":200,"replay_ns":100,"worker_ns":400});
        let process = json!({"parent_ns":800});
        assert!(clocks_bound(&row, &process, 200, 100, 400));
        assert!(!clocks_bound(
            &row,
            &json!({"parent_ns":801}),
            200,
            100,
            400
        ));
        assert!(!clocks_bound(
            &row,
            &json!({"parent_ns":null}),
            200,
            100,
            400
        ));
        for (key, bad) in [
            ("parent_ns", 700),
            ("child_process_ns", 801),
            ("construct_ns", 201),
            ("replay_ns", 101),
            ("worker_ns", 401),
        ] {
            let mut changed = row.clone();
            changed[key] = json!(bad);
            assert!(!clocks_bound(&changed, &process, 200, 100, 400), "{key}");
        }
        assert!(!clocks_bound(&row, &process, u64::MAX, 100, 400));
    }

    #[test]
    fn all_six_cases_and_global_gates_are_required() {
        let mut cases = vec![json!({"passes_15x":true}); 6];
        assert!(all_six_cases_pass(&cases, true, true));
        assert!(!all_six_cases_pass(&cases, false, true));
        assert!(!all_six_cases_pass(&cases, true, false));
        cases[5]["passes_15x"] = json!(false);
        assert!(!all_six_cases_pass(&cases, true, true));
        cases.pop();
        assert!(!all_six_cases_pass(&cases, true, true));
    }
}
