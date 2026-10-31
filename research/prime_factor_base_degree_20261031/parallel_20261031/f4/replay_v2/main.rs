//! Exact offline proof replay of saved F4 bases; no solver or cube search.
#![recursion_limit = "512"]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn read(path: &Path) -> Result<Vec<u8>, String> {
    let len = fs::metadata(path)
        .map_err(|e| format!("{}: {e}", path.display()))?
        .len();
    if len > 64 * 1024 * 1024 {
        return Err("raw record exceeds 64MiB replay limit".into());
    }
    fs::read(path).map_err(|e| e.to_string())
}
fn write_once(path: &Path, value: &Value) -> Result<(), String> {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    f.write_all(&serde_json::to_vec_pretty(value).unwrap())
        .map_err(|e| e.to_string())?;
    f.write_all(b"\n").map_err(|e| e.to_string())
}

#[allow(dead_code)]
mod frozen {
    include!(concat!(env!("OUT_DIR"), "/frozen_v1_helpers.rs"));

    fn two_inclusions(
        input: &[Poly],
        basis: &[Poly],
        reference: &Roots,
        p: u64,
    ) -> Result<Value, String> {
        let checks = check_gb(input, basis, p)?;
        if reference
            .iter()
            .any(|r| input.iter().any(|f| eval(f, r, p) != 0))
        {
            return Err("independent reference excluded by original input".into());
        }
        let rejected: Vec<_> = reference
            .iter()
            .filter(|r| basis.iter().any(|g| eval(g, r, p) != 0))
            .cloned()
            .collect();
        if !rejected.is_empty() {
            return Err(format!(
                "exported basis excludes {} independently valid input roots",
                rejected.len()
            ));
        }
        Ok(
            json!({"checks":checks,"input_in_exported_ideal_by_exact_normal_forms":true,"reference_completeness":"complete signed group-pair enumeration plus independently reconstructed exact model and split-graph/membership equivalence","all_reference_roots_vanish_in_exported_basis":true,"finite_field_root_sets_equal_by_two_inclusions":true,"field_exhaustive_cube_enumeration":false,"probabilistic_branch_pruning":false,"full_ideal_or_multiplicity_equality":"not_claimed"}),
        )
    }
    pub(super) fn replay(
        input_bytes: &[u8],
        solver_bytes: &[u8],
        stderr_bytes: &[u8],
        old_result_bytes: &[u8],
    ) -> Result<Value, String> {
        let input_json: Value = serde_json::from_slice(input_bytes).map_err(|e| e.to_string())?;
        let s: System = serde_json::from_value(input_json.clone()).map_err(|e| e.to_string())?;
        let input = validate(&s)?;
        let mut solver: Value = serde_json::from_slice(solver_bytes).map_err(|e| e.to_string())?;
        if solver["algorithm"] != "stock_msolve_neogb_f4"
            || solver["p"] != s.p
            || solver["variables"] != s.variables.len()
        {
            return Err("saved solver provenance/field mismatch".into());
        }
        let terms: Vec<Vec<Term>> =
            serde_json::from_value(solver["reduced_basis"].clone()).map_err(|e| e.to_string())?;
        let basis = terms
            .iter()
            .map(|t| parse_poly(t, s.variables.len(), s.p))
            .collect::<Result<Vec<_>, _>>()?;
        let old: Value = serde_json::from_slice(old_result_bytes).map_err(|e| e.to_string())?;
        if old["solver_stdout_sha256"] != hash(solver_bytes)
            || old["solver_stderr_sha256"] != hash(stderr_bytes)
            || old["input_json_sha256"] != hash(input_bytes)
        {
            return Err("saved raw hashes differ from v1 result linkage".into());
        }
        let (reference, relations) = references(&s);
        if let Some(expected) = &s.expected_roots {
            if expected.iter().cloned().collect::<Roots>() != reference {
                return Err("producer reference differs from independent group enumeration".into());
            }
        }
        let certificate = two_inclusions(&input, &basis, &reference, s.p)?;
        let old_comparison = if old["roots"].is_array() {
            let roots: Roots =
                serde_json::from_value(old["roots"].clone()).map_err(|e| e.to_string())?;
            if roots != reference {
                return Err("v1 saved root list disagrees with exact replay reference".into());
            }
            "available_and_equal"
        } else if old["status"] == "verified" {
            return Err("v1 verified record omits its extracted root list".into());
        } else {
            "not_available_in_original_failed_record"
        };
        let trace = String::from_utf8_lossy(stderr_bytes);
        let selected = selected_degrees(&trace);
        let max_pair = selected.iter().copied().max();
        let stats = trace_statistics(&trace);
        for (k, v) in stats.as_object().unwrap() {
            solver[k] = v.clone();
        }
        let trace_complete = solver["pairs_reduced"] == 0 || !selected.is_empty();
        let input_degree = input
            .iter()
            .flat_map(|f| f.keys())
            .map(|m| degree(m))
            .max()
            .unwrap_or(0);
        let basis_degree = basis
            .iter()
            .flat_map(|f| f.keys())
            .map(|m| degree(m))
            .max()
            .unwrap_or(0);
        let shapes = trace_matrix_shapes(&trace);
        Ok(
            json!({"v2_status":"verified","source_system_id":s.id,"arm":s.arm,"control_name":input_json.get("control_name"),"d":input_json.get("d"),"p":s.p,"curve":input_json["curve"],"target":s.target,"base_x":s.base_x,"base_columns":s.base_x.len(),"signed_base_points":s.base_points.len(),"fb_id":input_json.get("fb_id"),"signs":s.signs,"verification_method":"exact_two_inclusions_with_complete_group_reference","original_status":old["status"],"original_error":old.get("error"),"original_root_list_comparison":old_comparison,"independent_reference_roots":reference,"reference_roots_count":reference.len(),"ordered_signed_relation_count":relations.len(),"group_replayed_relations":relations,"sat_class":if relations.is_empty(){"UNSAT"}else{"SAT"},"false_positives":0,"false_negatives":0,"proof":certificate,"new_cas_calls":0,"metrics":{"maximum_input_degree":input_degree,"maximum_f4_selected_pair_degree":max_pair,"selected_pair_degree_trace":selected,"selected_degree_trace_complete":trace_complete,"observed_f4_degree":trace_complete.then(||input_degree.max(max_pair.unwrap_or(0))),"maximum_exported_basis_degree":basis_degree,"pairs_reduced":solver["pairs_reduced"],"rows_reduced":solver["rows_reduced"],"zero_reductions":solver["zero_reductions"],"peak_matrix_rows_in_trace":shapes.iter().map(|x|x[0]).max(),"peak_matrix_columns_in_trace":shapes.iter().map(|x|x[1]).max(),"largest_area_matrix_rows":solver["largest_area_matrix_rows"],"largest_area_matrix_columns":solver["largest_area_matrix_columns"],"field_additions":null,"field_multiplications":null,"clock_measurements":"none_added; original_saved_measurements_unchanged"}}),
        )
    }
    #[cfg(test)]
    mod replay_tests {
        use super::*;
        #[test]
        fn exact_inclusions_cover_artificial_roots_without_search() {
            let input = vec![Poly::from([(vec![2], 1), (vec![0], 16)])];
            let reference = Roots::from([vec![1], vec![16]]);
            two_inclusions(&input, &input, &reference, 17).unwrap();
        }
        #[test]
        fn stricter_wrong_basis_is_rejected() {
            let input = vec![Poly::from([(vec![2], 1), (vec![0], 16)])];
            let wrong = vec![Poly::from([(vec![1], 1), (vec![0], 16)])];
            let reference = Roots::from([vec![1], vec![16]]);
            assert!(two_inclusions(&input, &wrong, &reference, 17).is_err());
        }
        #[test]
        fn basis_with_extraneous_root_is_rejected_by_input_remainder() {
            let input = vec![Poly::from([(vec![1], 1), (vec![0], 16)])];
            let wrong = vec![Poly::from([(vec![2], 1), (vec![0], 16)])];
            let reference = Roots::from([vec![1]]);
            assert!(two_inclusions(&input, &wrong, &reference, 17).is_err());
        }
        #[test]
        fn unit_proof_handles_empty_reference() {
            let input = vec![Poly::from([(vec![0], 1)])];
            two_inclusions(&input, &input, &Roots::new(), 17).unwrap();
        }
    }
}

fn collect_cases(
    root: &Path,
    out: &mut Vec<PathBuf>,
    panels: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            collect_cases(&path, out, panels)?;
        } else if entry.file_name() == "result.json"
            && path.parent().unwrap().join("input.json").is_file()
            && path.parent().unwrap().join("solver.json").is_file()
        {
            out.push(path.parent().unwrap().to_path_buf());
        } else if entry.file_name() == "summary.json" {
            panels.push(path);
        }
    }
    Ok(())
}
fn memory_cap() -> Result<(), String> {
    let limits = fs::read_to_string("/proc/self/limits").map_err(|e| e.to_string())?;
    let n = limits
        .lines()
        .find(|l| l.starts_with("Max address space"))
        .and_then(|l| l.split_whitespace().nth(3))
        .and_then(|s| s.parse::<u64>().ok());
    if n.is_none_or(|n| n > 2147483648) {
        return Err("run under ulimit -v 2097152".into());
    }
    Ok(())
}
fn run(runroot: &Path, outdir: &Path, seconds: u64) -> Result<(), String> {
    memory_cap()?;
    if seconds == 0 || seconds > 300 {
        return Err("offline analysis cap must be 1..300 seconds".into());
    }
    if outdir.exists() {
        return Err("refuse companion directory overwrite".into());
    }
    let mut cases = Vec::new();
    let mut panels = Vec::new();
    collect_cases(&runroot.join("f4"), &mut cases, &mut panels)?;
    cases.sort();
    panels.sort();
    let mut requested = 0u64;
    let mut attempted = 0u64;
    let mut panel_hashes = Vec::new();
    for path in &panels {
        let data = read(path)?;
        let panel: Value = serde_json::from_slice(&data).map_err(|e| e.to_string())?;
        if panel["schema"] != "secant-f4-panel/v1" {
            continue;
        }
        requested += panel["requested_systems"]
            .as_u64()
            .ok_or("original panel requested count missing")?;
        attempted += panel["attempted_cells"]
            .as_u64()
            .ok_or("original panel attempted count missing")?;
        panel_hashes.push(json!({"path":path.strip_prefix(runroot).map_err(|e|e.to_string())?.to_string_lossy(),"sha256":hash(&data)}));
    }
    if attempted != cases.len() as u64 || requested < attempted {
        return Err("saved case inventory does not match original panel manifest counts".into());
    }
    if cases.len() > 20000 {
        return Err("case count exceeds replay scope".into());
    }
    fs::create_dir_all(outdir).map_err(|e| e.to_string())?;
    let start = Instant::now();
    let mut records = Vec::new();
    let mut failed = 0;
    let mut recovered = 0;
    for case in &cases {
        if start.elapsed() > Duration::from_secs(seconds) {
            break;
        }
        let relative = case.strip_prefix(runroot).map_err(|e| e.to_string())?;
        let destination = outdir.join(relative);
        fs::create_dir_all(&destination).map_err(|e| e.to_string())?;
        let input = read(&case.join("input.json"))?;
        let solver = read(&case.join("solver.json"))?;
        let trace = read(&case.join("solver.stderr"))?;
        let old = read(&case.join("result.json"))?;
        let original: Value = serde_json::from_slice(&old).map_err(|e| e.to_string())?;
        let checked = frozen::replay(&input, &solver, &trace, &old);
        let mut value = match checked {
            Ok(value) => value,
            Err(e) => {
                failed += 1;
                json!({"v2_status":"failed","error":e,"original_status":original["status"],"original_error":original.get("error")})
            }
        };
        if value["v2_status"] == "verified" && original["status"] != "verified" {
            recovered += 1;
        }
        let object = value.as_object_mut().unwrap();
        object.insert("schema".into(), json!("secant-f4-offline-replay/v2"));
        object.insert(
            "original_case_path".into(),
            json!(relative.to_string_lossy()),
        );
        object.insert("raw_hashes".into(),json!({"input_json_sha256":hash(&input),"solver_stdout_sha256":hash(&solver),"solver_stderr_sha256":hash(&trace),"original_result_sha256":hash(&old)}));
        object.insert(
            "frozen_v1_source_sha256".into(),
            json!(env!("FROZEN_V1_SHA256")),
        );
        object.insert(
            "replay_v2_source_sha256".into(),
            json!(hash(include_bytes!("main.rs"))),
        );
        write_once(&destination.join("replay.json"), &value)?;
        records.push(json!({"original_case_path":relative.to_string_lossy(),"companion_path":relative.join("replay.json").to_string_lossy(),"companion_sha256":hash(&read(&destination.join("replay.json"))?),"original_status":value["original_status"],"v2_status":value["v2_status"]}));
    }
    let unattempted = cases.len() - records.len();
    write_once(
        &outdir.join("manifest.json"),
        &json!({"schema":"secant-f4-offline-replay-manifest/v2","original_run_root":runroot.to_string_lossy(),"verification_method":"exact_two_inclusions_with_complete_group_reference","requested_cases":cases.len(),"original_requested_cases":requested,"original_attempted_cases":attempted,"original_unattempted_cases":requested-attempted,"original_panel_manifests":panel_hashes,"replayed_cases":records.len(),"verified_cases":records.len()-failed,"failed_cases":failed,"unattempted_cases":unattempted,"v1_failed_or_capped_cases_verified_by_v2":recovered,"new_cas_calls":0,"coordinate_cube_search":false,"source_module_reuse":"frozen v1 algebra/model/reference helpers, hash checked at build; no v1 source/raw record edited","records":records}),
    )?;
    if failed != 0 || unattempted != 0 {
        return Err("offline replay incomplete or failed; companion records retained".into());
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 || args[1] != "run" {
        return Err("usage: secant-f4-offline-replay-v2 run ORIGINAL_RUN_ROOT FRESH_COMPANION_ROOT MAX_SECONDS".into());
    }
    run(Path::new(&args[2]), Path::new(&args[3]), args[4].parse()?)?;
    Ok(())
}
