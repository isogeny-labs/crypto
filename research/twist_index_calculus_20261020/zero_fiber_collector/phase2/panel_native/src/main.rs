//! Fixed workload preparation and reduction of recorded native pipeline runs.
//! No curve solver, timing admission or scientific status transition lives here.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::path::Path;

const PRIMES: [u64; 7] = [37, 73, 97, 157, 229, 241, 337];
const PHI: &str = "phi-l0-active";
const DOUBLE: &str = "double-l0-active";
const BASELINE_CSV: &str = "7d47e6fc77d6dcb631b0bfd0cab0b5e93cd328ad9a9e716e35fdb8d5399f3919";

fn hash(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn file_hash(path: &Path) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut digest = Sha256::new();
    let mut bytes = [0u8; 65536];
    loop {
        let n = file.read(&mut bytes).unwrap();
        if n == 0 {
            break;
        }
        digest.update(&bytes[..n]);
    }
    hex::encode(digest.finalize())
}
fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn maybe_read(path: &Path) -> Option<Value> {
    serde_json::from_slice(&std::fs::read(path).ok()?).ok()
}
fn write_new(path: &Path, value: &Value) {
    assert!(!path.exists());
    std::fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}
fn number(value: &Value) -> u64 {
    value.as_u64().unwrap()
}
fn terminal_ok(path: &Path) -> bool {
    std::fs::read_to_string(path).is_ok_and(|s| s.trim() == "0")
}

fn prepare(args: &[String]) {
    assert_eq!(args.len(), 6);
    let baseline = Path::new(&args[0]);
    let public_root = std::fs::canonicalize(&args[1]).unwrap();
    let zero_root = std::fs::canonicalize(&args[2]).unwrap();
    let nine_root = std::fs::canonicalize(&args[3]).unwrap();
    let double_root = std::fs::canonicalize(&args[4]).unwrap();
    let out = Path::new(&args[5]);
    assert!(!out.exists());
    let selected = read(baseline);
    assert_eq!(
        selected["schema"],
        "twist-zero-pipeline-baseline-selection/v1"
    );
    assert_eq!(selected["source_cold_csv_sha256"], BASELINE_CSV);
    let rows = selected["cases"].as_array().unwrap();
    assert_eq!(rows.len(), 21);
    let mut best = BTreeMap::new();
    for row in rows {
        assert!(best
            .insert((number(&row["p"]), number(&row["seed"])), row)
            .is_none());
    }
    let mut workloads = Vec::new();
    let mut sources = BTreeMap::new();
    sources.insert(
        std::fs::canonicalize(baseline)
            .unwrap()
            .to_string_lossy()
            .into_owned(),
        file_hash(baseline),
    );
    let mut tsv = String::new();
    for p in PRIMES {
        for seed in 0..3 {
            let case = format!("p{p}-seed{seed}");
            let public_path = public_root.join(format!("{case}.json"));
            let public_sha = file_hash(&public_path);
            let public = read(&public_path);
            assert_eq!(public["p"], p);
            assert_eq!(public["seed"], seed);
            let selected = best[&(p, seed)];
            let selected_arm = selected["plain_arm"].as_str().unwrap();
            let mut zero_refs = BTreeMap::new();
            for arm in [PHI, DOUBLE] {
                let folder = zero_root.join(&case).join(arm);
                let path = folder.join("first/record.json");
                let sha = file_hash(&path);
                let record = read(&path);
                let audit_path = folder.join("audit/summary.json");
                let audit = read(&audit_path);
                assert_eq!(record["status"], "complete-polynomial-enumeration");
                assert_eq!(record["p"], p);
                assert_eq!(record["seed"], seed);
                assert_eq!(record["arm"], arm);
                assert_eq!(record["public_input_sha256"], public_sha);
                assert_eq!(audit["status"], "pass");
                assert_eq!(audit["candidate_sha256"], sha);
                sources.insert(path.to_string_lossy().into_owned(), sha.clone());
                sources.insert(
                    audit_path.to_string_lossy().into_owned(),
                    file_hash(&audit_path),
                );
                zero_refs.insert(arm, (path, sha));
            }
            let mut configs = vec![
                ("zero", PHI, "zero-phi"),
                ("zero", DOUBLE, "zero-double"),
                ("plain", PHI, "plain-phi"),
                ("plain", DOUBLE, "plain-double"),
            ];
            if ![PHI, DOUBLE].contains(&selected_arm) {
                configs.push(("plain", selected_arm, "best-parent"));
            }
            for (mode, arm, role) in configs {
                let target_path = if arm == DOUBLE {
                    &double_root
                } else {
                    &nine_root
                }
                .join(&case)
                .join(format!("{arm}.json"));
                let target_sha = file_hash(&target_path);
                let target = read(&target_path);
                assert_eq!(target["p"], p);
                assert_eq!(target["seed"], seed);
                assert_eq!(target["arm"], arm);
                assert_eq!(target["all_target_and_factor_logs_verified"], true);
                assert_eq!(target["censored"], false);
                let trace = target["trace"].as_array().unwrap();
                assert_eq!(trace.len(), public["query_trace"].as_array().unwrap().len());
                for (i, q) in trace.iter().enumerate() {
                    assert_eq!(q["query_index"], i);
                    assert_eq!(q["a"], public["query_trace"][i][0]);
                    assert_eq!(q["b"], public["query_trace"][i][1]);
                }
                let (zero_path, zero_sha) = if mode == "zero" {
                    let (path, sha) = &zero_refs[arm];
                    (path.to_string_lossy().into_owned(), sha.clone())
                } else {
                    (String::from("-"), String::from("-"))
                };
                let id = format!("{case}-{mode}-{arm}");
                let mut roles = vec![role];
                if mode == "plain" && arm == selected_arm && role != "best-parent" {
                    roles.push("best-parent");
                }
                let public_path = public_path.to_string_lossy();
                let target_path = target_path.to_string_lossy();
                for value in [
                    id.as_str(),
                    public_path.as_ref(),
                    zero_path.as_str(),
                    target_path.as_ref(),
                ] {
                    assert!(!value.contains(['\t', '\n', '\r']));
                }
                tsv.push_str(&format!("{id}\t{p}\t{seed}\t{mode}\t{arm}\t{public_path}\t{public_sha}\t{zero_path}\t{zero_sha}\t{target_path}\t{target_sha}\n"));
                workloads.push(json!({"id":id,"p":p,"seed":seed,"mode":mode,"arm":arm,"roles":roles,
                "public_path":public_path,"public_sha256":public_sha,"zero_reference_path":zero_path,
                "zero_reference_sha256":zero_sha,"target_reference_path":target_path,"target_reference_sha256":target_sha,
                "selected_parent_arm":selected_arm,"old_parent_median_cold_ns":selected["old_median_cold_ns"]}));
                sources.insert(target_path.into_owned(), target_sha);
            }
            sources.insert(public_path.to_string_lossy().into_owned(), public_sha);
        }
    }
    assert_eq!(workloads.len(), 85);
    assert_eq!(
        workloads
            .iter()
            .map(|v| v["id"].as_str().unwrap())
            .collect::<BTreeSet<_>>()
            .len(),
        85
    );
    let source_manifest = sources
        .iter()
        .map(|(path, sha)| format!("{sha}  {path}\n"))
        .collect::<String>();
    write_new(
        out,
        &json!({"schema":"twist-zero-pipeline-fixed-workload/v1","baseline_sha256":file_hash(baseline),
        "baseline_source_csv_sha256":BASELINE_CSV,"passes":["first","aa1","aa2"],"expected_configurations":85,
        "expected_candidate_executions":255,"workloads":workloads,"frozen_sources":sources,
        "scope":"All original 21 cases. Four matched configurations each plus the frozen p97/seed1 union-l2 control. Duplicate best-parent roles reuse the identical registered plain execution. Shared host: no timing admission."}),
    );
    let tsv_path = out.with_extension("tsv");
    assert!(!tsv_path.exists());
    std::fs::write(tsv_path, tsv).unwrap();
    let source_path = out.with_extension("sha256");
    assert!(!source_path.exists());
    std::fs::write(source_path, source_manifest).unwrap();
}

fn sum(mut values: impl Iterator<Item = u64>) -> u64 {
    values.try_fold(0u64, |a, b| a.checked_add(b)).unwrap()
}
fn count_array(value: &Value) -> u64 {
    value.as_array().unwrap().len() as u64
}
fn ratio(a: &Value, b: &Value) -> Value {
    let (a, b) = (number(a), number(b));
    if b == 0 {
        Value::Null
    } else {
        json!(a as f64 / b as f64)
    }
}
fn rss(path: &Path) -> Option<u64> {
    std::fs::read_to_string(path)
        .ok()?
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("Maximum resident set size (kbytes):")?
                .trim()
                .parse()
                .ok()
        })
}

fn summarize(run: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let workload = read(&run.join("metadata/workload.json"));
    assert_eq!(workload["schema"], "twist-zero-pipeline-fixed-workload/v1");
    let tasks = workload["workloads"].as_array().unwrap();
    assert_eq!(tasks.len(), 85);
    let mut rows = Vec::new();
    let mut unverified = Vec::new();
    let mut executed = 0usize;
    let mut csv=String::from("p,seed,mode,arm,status,zero_f4_systems,target_f4_systems,total_f4_systems,zero_factor_rank,target_queries,rank,columns,zero_unique_signed,target_unique_signed,unique_projective_rows,zero_f4_row_fp_muls,target_f4_row_fp_muls,native_fp_muls,subgroup_muls,group_add_calls,allocation_calls,allocated_bytes,peak_live_rust_bytes,peak_process_rss_kib\n");
    for task in tasks {
        let id = task["id"].as_str().unwrap();
        let root = run.join("cases").join(id);
        let first_path = root.join("first/record.json");
        let first = maybe_read(&first_path);
        let audit = maybe_read(&root.join("audit/summary.json"));
        let mut ok = true;
        for pass in ["first", "aa1", "aa2"] {
            let tag = format!("{id}-{pass}-collect");
            let exit_path = run.join(format!("metadata/{tag}.exit"));
            if exit_path.exists() {
                executed += 1;
            }
            ok &= terminal_ok(&exit_path);
            if pass != "first" {
                let path = root.join(pass).join("record.json");
                let parity = maybe_read(&root.join(format!("{pass}-parity.json")));
                ok &= parity.as_ref().is_some_and(|v| {
                    v["status"] == "pass"
                        && first_path.is_file()
                        && path.is_file()
                        && v["first_sha256"] == file_hash(&first_path)
                        && v["second_sha256"] == file_hash(&path)
                });
                ok &= terminal_ok(&run.join(format!("metadata/{id}-{pass}-parity.exit")));
            }
        }
        ok &= terminal_ok(&run.join(format!("metadata/{id}-audit.exit")));
        ok &= first.as_ref().is_some_and(|v| {
            v["schema"] == "twist-zero-one-target-pipeline/v1"
                && v["deterministic"]["status"] == "pass"
                && v["deterministic"]["public_input_sha256"] == task["public_sha256"]
                && v["deterministic"]["arm"] == task["arm"]
                && v["deterministic"]["mode"] == task["mode"]
                && v["deterministic"]["p"] == task["p"]
                && v["deterministic"]["seed"] == task["seed"]
                && v["deterministic"]["all_usable_factor_logs_and_target_verified"] == true
        });
        ok &= audit.as_ref().is_some_and(|v| {
            v["status"] == "pass"
                && first_path.is_file()
                && v["candidate_sha256"] == file_hash(&first_path)
                && v["public_input_sha256"] == task["public_sha256"]
                && v["target_reference_sha256"] == task["target_reference_sha256"]
                && (task["mode"] != "zero"
                    || v["zero_reference_sha256"] == task["zero_reference_sha256"])
                && first.as_ref().is_some_and(|first| {
                    let d = &first["deterministic"];
                    let checked = &v["independent_checks"];
                    checked["zero_systems_checked"] == count_array(&d["zero_systems"])
                        && checked["target_queries_checked"] == count_array(&d["target_queries"])
                        && checked["target_finite_systems_checked"]
                            == sum(d["target_queries"].as_array().unwrap().iter().map(|q| {
                                q["systems"]
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .filter(|s| s["f4_attempted"] == true)
                                    .count() as u64
                            }))
                        && checked["zero_factor_rank"] == d["zero_factor_log_rank"]
                        && checked["rank"] == d["rank"]
                        && checked["columns"] == d["columns"]
                        && checked["first_full_factor_system"] == d["first_full_factor_system"]
                        && checked["first_full_rank_query"] == d["first_full_rank_query"]
                        && checked["target_and_factor_points_verified"] == true
                })
        });
        if !ok {
            unverified.push(id.to_owned());
        }
        let mut row = json!({"id":id,"p":task["p"],"seed":task["seed"],"mode":task["mode"],"arm":task["arm"],
            "roles":task["roles"],"status":if ok{"verified-first-aa"}else{"unverified"}});
        if let Some(first) = &first {
            let d = &first["deterministic"];
            let zero_systems = d["zero_systems"].as_array().unwrap();
            let queries = d["target_queries"].as_array().unwrap();
            let z = zero_systems
                .iter()
                .filter(|s| s["f4_attempted"] == true)
                .count() as u64;
            let t = sum(queries.iter().map(|q| {
                q["systems"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|s| s["f4_attempted"] == true)
                    .count() as u64
            }));
            let phases = d["work_by_phase"].as_object().unwrap();
            let mut projective: BTreeSet<Vec<u64>> =
                serde_json::from_value(d["zero_projective_rows"].clone()).unwrap();
            projective.extend(
                serde_json::from_value::<BTreeSet<Vec<u64>>>(d["target_projective_rows"].clone())
                    .unwrap(),
            );
            let peak_rss = rss(&run.join(format!("metadata/{id}-first-collect.time.txt")));
            let metrics = json!({"zero_f4_systems":z,"target_f4_systems":t,"total_f4_systems":z+t,
                "zero_factor_rank":d["zero_factor_log_rank"],"target_queries":queries.len(),
                "rank":d["rank"],"columns":d["columns"],"zero_unique_signed":count_array(&d["zero_signed_physical_set"]),
                "target_unique_signed":count_array(&d["target_signed_physical_set"]),"unique_projective_rows":projective.len(),
                "zero_f4_row_fp_muls":d["zero_f4_row_reduction_fp_muls"],"target_f4_row_fp_muls":d["target_f4_row_reduction_fp_muls"],
                "native_fp_muls":sum(phases.values().map(|v|number(&v["fp_muls"]))),
                "subgroup_muls":sum(phases.values().map(|v|number(&v["subgroup_muls"]))),
                "group_add_calls":sum(phases.values().map(|v|number(&v["group_add_calls"]))),
                "allocation_calls":first["memory"]["allocation_calls"],"allocated_bytes":first["memory"]["allocated_bytes"],
                "peak_live_rust_bytes":first["memory"]["peak_live_rust_bytes"],"peak_process_rss_kib":peak_rss});
            for (key, value) in metrics.as_object().unwrap() {
                row[key] = value.clone();
            }
            row["first_diagnostic_wall_ns"] = first["diagnostic_wall_ns"].clone();
        }
        let columns = [
            "p",
            "seed",
            "mode",
            "arm",
            "status",
            "zero_f4_systems",
            "target_f4_systems",
            "total_f4_systems",
            "zero_factor_rank",
            "target_queries",
            "rank",
            "columns",
            "zero_unique_signed",
            "target_unique_signed",
            "unique_projective_rows",
            "zero_f4_row_fp_muls",
            "target_f4_row_fp_muls",
            "native_fp_muls",
            "subgroup_muls",
            "group_add_calls",
            "allocation_calls",
            "allocated_bytes",
            "peak_live_rust_bytes",
            "peak_process_rss_kib",
        ];
        csv.push_str(
            &columns
                .iter()
                .map(|key| match &row[*key] {
                    Value::String(s) => s.clone(),
                    Value::Null => String::new(),
                    v => v.to_string(),
                })
                .collect::<Vec<_>>()
                .join(","),
        );
        csv.push('\n');
        rows.push(row);
    }
    let mut comparisons = Vec::new();
    for p in PRIMES {
        for seed in 0..3 {
            let case: Vec<_> = rows
                .iter()
                .filter(|r| r["p"] == p && r["seed"] == seed)
                .collect();
            let get = |role: &str| {
                *case
                    .iter()
                    .find(|r| r["roles"].as_array().unwrap().iter().any(|v| v == role))
                    .unwrap()
            };
            for (reference, candidate) in [
                ("plain-phi", "zero-phi"),
                ("plain-double", "zero-double"),
                ("zero-double", "zero-phi"),
                ("best-parent", "zero-phi"),
                ("best-parent", "zero-double"),
            ] {
                let a = get(reference);
                let b = get(candidate);
                let ok = a["status"] == "verified-first-aa" && b["status"] == "verified-first-aa";
                comparisons.push(json!({"p":p,"seed":seed,"reference_role":reference,"candidate_role":candidate,
                "reference_id":a["id"],"candidate_id":b["id"],"verified":ok,
                "reference_over_candidate_f4_systems":if ok{ratio(&a["total_f4_systems"],&b["total_f4_systems"])}else{Value::Null},
                "scope":"Finite F4 system counts only; no complete cost, calibrated GAE or admitted timing ratio."}));
            }
        }
    }
    let pass = unverified.is_empty() && executed == 255;
    write_new(
        &out.join("summary.json"),
        &json!({"schema":"twist-zero-pipeline-panel-summary/v1",
        "status":if pass{"complete-verified-first-aa"}else{"partial"},"expected_configurations":85,
        "expected_candidate_executions":255,"recorded_candidate_exits":executed,
        "unverified_configurations":unverified,"rows":rows,"comparisons":comparisons,
        "scope":"First/A/A on all 85 configurations; independent first-run physical/system/RREF replay and sealed deterministic AA parity. Counts and allocation/RSS diagnostics only. Shared-host elapsed values are not admitted timing; complete IC/rho/GAE price remains open."}),
    );
    std::fs::write(out.join("cases.csv"), csv).unwrap();
    pass
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("prepare")=>prepare(&args[2..]),
        Some("inspect-record")=>{
            assert_eq!(args.len(),3);
            let value=read(Path::new(&args[2]));
            assert_eq!(value["schema"],"twist-zero-one-target-pipeline/v1");
            let d=&value["deterministic"];
            let z=d["zero_systems"].as_array().unwrap().iter().filter(|s|s["f4_attempted"]==true).count();
            let t=sum(d["target_queries"].as_array().unwrap().iter().map(|q|
                q["systems"].as_array().unwrap().iter().filter(|s|s["f4_attempted"]==true).count() as u64));
            println!("{}",json!({"p":d["p"],"seed":d["seed"],"arm":d["arm"],"mode":d["mode"],
                "status":d["status"],"zero_systems":z,"target_f4_systems":t,"total_f4_systems":z as u64+t,
                "zero_factor_rank":d["zero_factor_log_rank"],"first_full_factor_system":d["first_full_factor_system"],
                "target_queries":count_array(&d["target_queries"]),"first_full_rank_query":d["first_full_rank_query"],
                "rank":d["rank"],"columns":d["columns"],"recovered":d["recovered"],
                "all_usable_factor_logs_and_target_verified":d["all_usable_factor_logs_and_target_verified"],
                "zero_f4_row_fp_muls":d["zero_f4_row_reduction_fp_muls"],"target_f4_row_fp_muls":d["target_f4_row_reduction_fp_muls"],
                "scope":"Recorded native counts and verified-answer flags only; no timing admission or complete GAE price."}));
        },
        Some("summarize")=>{assert_eq!(args.len(),4);std::process::exit(if summarize(Path::new(&args[2]),Path::new(&args[3])){0}else{2});},
        _=>panic!("usage: prepare BASELINE PUBLIC ZERO_CASES NINE_REFS DOUBLE_REFS OUT | summarize RUN OUT"),
    }
}
