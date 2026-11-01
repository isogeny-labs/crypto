//! Read-only accounting of an existing audited panel. No collector or solver.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, String>;
const PHASES: [&str; 8] = [
    "setup",
    "zero_precomputation",
    "target_point_queries",
    "target_decomposition",
    "target_matrix",
    "target_recovery",
    "target_verification",
    "journal",
];
const ONLINE: [&str; 5] = [
    "target_point_queries",
    "target_decomposition",
    "target_matrix",
    "target_recovery",
    "target_verification",
];
const COUNTERS: [&str; 3] = ["fp_muls", "subgroup_muls", "group_add_calls"];

fn require(test: bool, reason: &str) -> Result<()> {
    if test {
        Ok(())
    } else {
        Err(reason.into())
    }
}
fn number(v: &Value) -> Result<u64> {
    v.as_u64()
        .ok_or_else(|| format!("missing or non-u64 count: {v}"))
}
fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().ok_or_else(|| "missing array".into())
}
fn read(path: &Path) -> Result<Value> {
    serde_json::from_slice(&std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?)
        .map_err(|e| format!("{}: {e}", path.display()))
}
fn hash(path: &Path) -> Result<String> {
    Ok(hex::encode(Sha256::digest(
        std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?,
    )))
}
fn exit_ok(path: &Path) -> Result<()> {
    require(
        std::fs::read_to_string(path)
            .map_err(|e| format!("{}: {e}", path.display()))?
            .trim()
            == "0",
        "nonzero phase exit",
    )
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn ratio(n: u64, d: u64) -> Value {
    if d == 0 {
        return Value::Null;
    }
    let g = gcd(n, d);
    json!({"numerator":n,"denominator":d,"reduced_numerator":n/g,
        "reduced_denominator":d/g,"approximate":n as f64/d as f64})
}
fn set(v: &Value) -> Result<BTreeSet<Vec<i64>>> {
    let rows: Vec<Vec<i64>> = serde_json::from_value(v.clone()).map_err(|e| e.to_string())?;
    let s: BTreeSet<_> = rows.iter().cloned().collect();
    require(s.len() == rows.len(), "duplicate projective row")?;
    Ok(s)
}
fn keys(v: &Value) -> Result<BTreeSet<&str>> {
    Ok(v.as_object()
        .ok_or("missing object")?
        .keys()
        .map(String::as_str)
        .collect())
}

// Infer the two journal portions from the independently recorded cut, never by
// assigning the entire journal phase to one side of the reusable/online cut.
fn clocks(c: &Value) -> Result<Value> {
    require(
        keys(&c["phases"])? == PHASES.into_iter().collect(),
        "unexpected clock phase set",
    )?;
    let mut phase_total = 0u64;
    for name in PHASES {
        phase_total = phase_total
            .checked_add(number(&c["phases"][name])?)
            .ok_or("clock overflow")?;
    }
    let cold = number(&c["cold_including_journals"])?;
    let reusable = number(&c["reusable_with_zero_journals"])?;
    let online = number(&c["online_with_target_journals"])?;
    require(
        phase_total == cold && reusable.checked_add(online) == Some(cold),
        "non-telescoping clock",
    )?;
    let setup = number(&c["phases"]["setup"])?;
    let zero = number(&c["phases"]["zero_precomputation"])?;
    let mut target = 0u64;
    for name in ONLINE {
        target = target
            .checked_add(number(&c["phases"][name])?)
            .ok_or("clock overflow")?;
    }
    let rjournal = reusable
        .checked_sub(setup.checked_add(zero).ok_or("clock overflow")?)
        .ok_or("negative reusable journal")?;
    let tjournal = online
        .checked_sub(target)
        .ok_or("negative target journal")?;
    require(
        rjournal.checked_add(tjournal) == Some(number(&c["journal"])?),
        "journal cut mismatch",
    )?;
    require(
        c["journal"] == c["phases"]["journal"],
        "journal phase mismatch",
    )?;
    Ok(
        json!({"raw":c,"reusable_journal_ns":rjournal,"target_journal_ns":tjournal,
        "scope":"Raw shared-host observations only; no admitted wall-time ratio."}),
    )
}

fn counter_totals(d: &Value) -> Result<Value> {
    require(
        keys(&d["work_by_phase"])? == PHASES.into_iter().collect(),
        "unexpected work phase set",
    )?;
    let mut totals = json!({});
    for counter in COUNTERS {
        let mut cold = 0u64;
        let mut online = 0u64;
        for name in PHASES {
            require(
                keys(&d["work_by_phase"][name])? == COUNTERS.into_iter().collect(),
                "unexpected counter set",
            )?;
            let n = number(&d["work_by_phase"][name][counter])?;
            cold = cold.checked_add(n).ok_or("counter overflow")?;
            if ONLINE.contains(&name) {
                online = online.checked_add(n).ok_or("counter overflow")?;
            }
        }
        // Journal code is not field or group arithmetic; do not invent a split
        // if a future producer changes this contract.
        require(
            number(&d["work_by_phase"]["journal"][counter])? == 0,
            "nonzero journal arithmetic requires scoped split",
        )?;
        totals[counter] = json!({"reusable":cold-online,"online":online,"cold":cold});
    }
    Ok(totals)
}

fn analyze_case(run: &Path, task: &Value, prior: &Value) -> Result<Value> {
    let id = task["id"].as_str().ok_or("missing case id")?;
    require(!id.contains('/') && !id.contains(".."), "invalid case id")?;
    let root = run.join("cases").join(id);
    let first_path = root.join("first/record.json");
    let first = read(&first_path)?;
    require(
        first["schema"] == "twist-zero-one-target-pipeline/v1",
        "record schema",
    )?;
    let d = &first["deterministic"];
    require(
        d["status"] == "pass" && d["all_usable_factor_logs_and_target_verified"] == true,
        "unverified target",
    )?;
    for key in ["p", "seed", "arm", "mode"] {
        require(
            d[key] == task[key] && prior[key] == task[key],
            "workload identity mismatch",
        )?;
    }
    require(
        d["public_input_sha256"] == task["public_sha256"],
        "public input binding",
    )?;
    require(
        number(&d["rank"])? == number(&d["columns"])?,
        "incomplete rank",
    )?;
    require(
        d["cooperative_deadline_hit"] == false && number(&d["undetermined_systems"])? == 0,
        "incomplete native solve",
    )?;
    let audit_path = root.join("audit/summary.json");
    let audit = read(&audit_path)?;
    require(
        audit["status"] == "pass" && audit["candidate_sha256"] == hash(&first_path)?,
        "independent audit binding",
    )?;
    require(
        audit["public_input_sha256"] == task["public_sha256"]
            && audit["target_reference_sha256"] == task["target_reference_sha256"],
        "audit reference binding",
    )?;
    if task["mode"] == "zero" {
        require(
            audit["zero_reference_sha256"] == task["zero_reference_sha256"],
            "zero reference binding",
        )?;
    }
    exit_ok(&run.join(format!("metadata/{id}-audit.exit")))?;
    let mut passes = Vec::new();
    for pass in ["first", "aa1", "aa2"] {
        exit_ok(&run.join(format!("metadata/{id}-{pass}-collect.exit")))?;
        let path = root.join(pass).join("record.json");
        let record = read(&path)?;
        require(
            record["schema"] == first["schema"] && record["deterministic"] == *d,
            "AA deterministic mismatch",
        )?;
        if pass != "first" {
            let parity = read(&root.join(format!("{pass}-parity.json")))?;
            require(
                parity["status"] == "pass"
                    && parity["first_sha256"] == hash(&first_path)?
                    && parity["second_sha256"] == hash(&path)?,
                "AA hash binding",
            )?;
            exit_ok(&run.join(format!("metadata/{id}-{pass}-parity.exit")))?;
        }
        passes.push(json!({"pass":pass,"record_sha256":hash(&path)?,
            "diagnostic_clocks":clocks(&record["diagnostic_wall_ns"])?,"memory":record["memory"]}));
    }
    let zero = array(&d["zero_systems"])?;
    let queries = array(&d["target_queries"])?;
    let finite =
        |systems: &[Value]| systems.iter().filter(|s| s["f4_attempted"] == true).count() as u64;
    let z = finite(zero);
    let mut t = 0u64;
    for q in queries {
        t = t
            .checked_add(finite(array(&q["systems"])?))
            .ok_or("system overflow")?;
    }
    let zp = set(&d["zero_projective_rows"])?;
    let tp = set(&d["target_projective_rows"])?;
    let projective = zp.union(&tp).count() as u64;
    let total = z.checked_add(t).ok_or("system overflow")?;
    let counters = counter_totals(d)?;
    let zm = number(&d["zero_f4_row_reduction_fp_muls"])?;
    let tm = number(&d["target_f4_row_reduction_fp_muls"])?;
    let metrics = json!({"zero_f4_systems":z,"target_f4_systems":t,"total_f4_systems":total,
        "zero_factor_rank":d["zero_factor_log_rank"],"target_queries":queries.len(),"rank":d["rank"],"columns":d["columns"],
        "zero_unique_signed":array(&d["zero_signed_physical_set"])?.len(),
        "target_unique_signed":array(&d["target_signed_physical_set"])?.len(),
        "unique_projective_rows":projective,"zero_unique_projective_rows":zp.len(),
        "target_unique_projective_rows":tp.len(),"target_rows_novel_after_zero":tp.difference(&zp).count(),
        "zero_f4_row_fp_muls":zm,"target_f4_row_fp_muls":tm,
        "cold_f4_row_fp_muls":zm.checked_add(tm).ok_or("F4 counter overflow")?,
        "native_fp_muls":counters["fp_muls"]["cold"],
        "subgroup_muls":counters["subgroup_muls"]["cold"],
        "group_add_calls":counters["group_add_calls"]["cold"],
        "online_native_fp_muls":counters["fp_muls"]["online"],
        "online_subgroup_muls":counters["subgroup_muls"]["online"],
        "online_group_add_calls":counters["group_add_calls"]["online"],
        "allocation_calls":first["memory"]["allocation_calls"],"allocated_bytes":first["memory"]["allocated_bytes"],
        "peak_live_rust_bytes":first["memory"]["peak_live_rust_bytes"],"peak_process_rss_kib":prior["peak_process_rss_kib"]});
    // These were reduced by the separately frozen metadata binary. Require
    // equality rather than trusting the report's precomputed headline values.
    for key in [
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
    ] {
        require(
            metrics[key] == prior[key],
            &format!("prior summary mismatch: {id} {key}"),
        )?;
    }
    let checked = &audit["independent_checks"];
    require(
        number(&checked["zero_systems_checked"])? == zero.len() as u64
            && number(&checked["target_queries_checked"])? == queries.len() as u64
            && number(&checked["target_finite_systems_checked"])? == t
            && checked["zero_factor_rank"] == d["zero_factor_log_rank"]
            && checked["rank"] == d["rank"]
            && checked["columns"] == d["columns"]
            && checked["first_full_factor_system"] == d["first_full_factor_system"]
            && checked["first_full_rank_query"] == d["first_full_rank_query"]
            && checked["target_and_factor_points_verified"] == true,
        "independent accounting checks",
    )?;
    Ok(
        json!({"id":id,"p":task["p"],"seed":task["seed"],"arm":task["arm"],"mode":task["mode"],"roles":task["roles"],
        "status":"verified-first-aa","audit_sha256":hash(&audit_path)?,"public_input_sha256":task["public_sha256"],
        "metrics":metrics,"work_by_phase":d["work_by_phase"],"counter_totals":counters,
        "projective_rows_per_finite_f4_system":ratio(projective,total),
        "first_full_factor_system":d["first_full_factor_system"],"first_full_rank_query":d["first_full_rank_query"],
        "passes":passes,"complete_end_to_end_cost":Value::Null,"admitted_wall_time_speedup":Value::Null,
        "missing_price":"F4 non-row work, additions/inversions and unmetered primitives, memory/hashing/journals, scalar/group calibration; counters overlap at different abstraction levels. Raw clocks have no isolation admission."}),
    )
}

fn analyze(
    run: &Path,
    summary_path: &Path,
    expected_sha: &str,
    mode: &str,
    out: &Path,
) -> Result<()> {
    require(!out.exists(), "output already exists")?;
    require(hash(summary_path)? == expected_sha, "summary hash mismatch")?;
    let summary = read(summary_path)?;
    require(
        summary["schema"] == "twist-zero-pipeline-panel-summary/v1",
        "summary schema",
    )?;
    match mode {
        "complete" => {
            require(
                summary["status"] == "complete-verified-first-aa"
                    && number(&summary["recorded_candidate_exits"])? == 255,
                "complete mode requires completed panel",
            )?;
            exit_ok(&run.join("metadata/terminal-exit"))?;
        }
        "snapshot" => require(
            summary["status"] == "partial",
            "snapshot mode requires partial summary",
        )?,
        _ => return Err("mode must be complete or snapshot".into()),
    }
    let workload_path = run.join("metadata/workload.json");
    let workload = read(&workload_path)?;
    require(
        workload["schema"] == "twist-zero-pipeline-fixed-workload/v1",
        "workload schema",
    )?;
    let tasks = array(&workload["workloads"])?;
    let prior_rows = array(&summary["rows"])?;
    require(
        tasks.len() == 85 && prior_rows.len() == 85,
        "expected all 85 workload rows",
    )?;
    let ids = |rows: &[Value]| -> Result<BTreeSet<String>> {
        let s: BTreeSet<_> = rows
            .iter()
            .map(|r| {
                r["id"]
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| "missing id".into())
            })
            .collect::<Result<_>>()?;
        require(s.len() == rows.len(), "duplicate workload id")?;
        Ok(s)
    };
    require(ids(tasks)? == ids(prior_rows)?, "workload id mismatch")?;
    let prior_map: BTreeMap<_, _> = prior_rows
        .iter()
        .map(|r| (r["id"].as_str().unwrap(), r))
        .collect();
    let mut rows = Vec::new();
    for task in tasks {
        let id = task["id"].as_str().unwrap();
        let prior = prior_map[id];
        if prior["status"] != "verified-first-aa" {
            require(mode == "snapshot", "unverified complete row")?;
            rows.push(json!({"id":id,"p":task["p"],"seed":task["seed"],"arm":task["arm"],"mode":task["mode"],
                "roles":task["roles"],"status":"not-verified-in-source-snapshot","metrics":Value::Null}));
        } else {
            rows.push(analyze_case(run, task, prior).map_err(|e| format!("{id}: {e}"))?);
        }
    }
    let mut comparisons = Vec::new();
    let metric_names = [
        "target_queries",
        "zero_f4_systems",
        "target_f4_systems",
        "total_f4_systems",
        "zero_unique_signed",
        "target_unique_signed",
        "unique_projective_rows",
        "zero_factor_rank",
        "zero_f4_row_fp_muls",
        "target_f4_row_fp_muls",
        "cold_f4_row_fp_muls",
        "online_native_fp_muls",
        "native_fp_muls",
        "online_subgroup_muls",
        "subgroup_muls",
        "online_group_add_calls",
        "group_add_calls",
        "allocation_calls",
        "allocated_bytes",
        "peak_live_rust_bytes",
    ];
    for p in [37, 73, 97, 157, 229, 241, 337] {
        for seed in 0..3 {
            for (reference, candidate) in [
                ("plain-phi", "zero-phi"),
                ("plain-double", "zero-double"),
                ("zero-double", "zero-phi"),
                ("plain-double", "plain-phi"),
                ("best-parent", "zero-phi"),
                ("best-parent", "zero-double"),
            ] {
                let get = |role: &str| -> Result<&Value> {
                    let matched: Vec<_> = rows
                        .iter()
                        .filter(|r| {
                            r["p"] == p
                                && r["seed"] == seed
                                && r["roles"]
                                    .as_array()
                                    .is_some_and(|roles| roles.iter().any(|x| x == role))
                        })
                        .collect();
                    require(matched.len() == 1, "control role not unique")?;
                    Ok(matched[0])
                };
                let a = get(reference)?;
                let b = get(candidate)?;
                let verified =
                    a["status"] == "verified-first-aa" && b["status"] == "verified-first-aa";
                let mut ratios = Value::Null;
                if verified {
                    require(
                        a["public_input_sha256"] == b["public_input_sha256"],
                        "comparison target mismatch",
                    )?;
                    ratios = json!({});
                    for name in metric_names {
                        ratios[name] =
                            ratio(number(&a["metrics"][name])?, number(&b["metrics"][name])?);
                    }
                }
                comparisons.push(json!({"p":p,"seed":seed,"reference_role":reference,"candidate_role":candidate,
                    "reference_id":a["id"],"candidate_id":b["id"],"verified":verified,
                    "reference_over_candidate":ratios,"complete_end_to_end_cost_ratio":Value::Null,
                    "scope":"Each ratio concerns only its named count; counts are not interchangeable or additive costs. Zero denominators stay null."}));
            }
        }
    }
    let result = json!({"schema":"twist-zero-pipeline-accounting/v1","mode":mode,
        "status":if mode=="complete"{"complete-accounting-diagnostics"}else{"historical-partial-accounting-diagnostics"},
        "source_summary_sha256":expected_sha,"workload_sha256":hash(&workload_path)?,
        "source_summary_recorded_exits":summary["recorded_candidate_exits"],
        "verified_configurations":rows.iter().filter(|r|r["status"]=="verified-first-aa").count(),
        "rows":rows,"comparisons":comparisons,"complete_end_to_end_cost":Value::Null,
        "scope":"Re-analysis only. No new solve, target, timing admission or pipeline change. Individual synthetic known-answer targets; presentations do not constitute independent secrets."});
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    std::fs::write(
        out.join("accounting.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    println!(
        "{}",
        json!({"status":result["status"],"verified_configurations":result["verified_configurations"],"comparisons":126})
    );
    Ok(())
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 6 {
        eprintln!(
            "usage: accounting RUN SOURCE_SUMMARY SUMMARY_SHA256 complete|snapshot FRESH_OUT"
        );
        std::process::exit(64);
    }
    if let Err(reason) = analyze(
        &PathBuf::from(&args[1]),
        &PathBuf::from(&args[2]),
        &args[3],
        &args[4],
        &PathBuf::from(&args[5]),
    ) {
        eprintln!("accounting rejected: {reason}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn example_clock() -> Value {
        json!({"cold_including_journals":100,"reusable_with_zero_journals":40,"online_with_target_journals":60,"journal":60,
            "phases":{"setup":10,"zero_precomputation":10,"target_point_queries":4,"target_decomposition":4,
            "target_matrix":4,"target_recovery":4,"target_verification":4,"journal":60}})
    }
    #[test]
    fn journal_is_split_at_target_boundary() {
        let c = clocks(&example_clock()).unwrap();
        assert_eq!(c["reusable_journal_ns"], 20);
        assert_eq!(c["target_journal_ns"], 40);
    }
    #[test]
    fn missing_phase_and_false_total_are_rejected() {
        let mut c = example_clock();
        c["phases"]
            .as_object_mut()
            .unwrap()
            .remove("target_recovery");
        assert!(clocks(&c).is_err());
        let mut c = example_clock();
        c["cold_including_journals"] = json!(99);
        assert!(clocks(&c).is_err());
        let mut c = example_clock();
        c["reusable_with_zero_journals"] = json!(10);
        c["online_with_target_journals"] = json!(90);
        assert!(clocks(&c).is_err());
    }
    #[test]
    fn duplicate_rows_and_undefined_ratios_are_explicit() {
        assert!(set(&json!([[0, 1], [0, 1]])).is_err());
        assert_eq!(ratio(0, 0), Value::Null);
        assert_eq!(ratio(154, 144)["reduced_numerator"], 77);
        assert_eq!(ratio(154, 144)["reduced_denominator"], 72);
        assert!(number(&Value::Null).is_err());
    }
}
