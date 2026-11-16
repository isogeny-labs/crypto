//! Post-data descriptive verification/aggregation. No new experiment or solver.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
mod compact;
mod replay;

fn bytes(path: &Path) -> Result<Vec<u8>, String> {
    let b = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if b.len() > 128 * 1024 * 1024 {
        return Err("report input exceeds128MiB".into());
    }
    Ok(b)
}
fn read(path: &Path) -> Result<Value, String> {
    serde_json::from_slice(&bytes(path)?).map_err(|e| format!("{}: {e}", path.display()))
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn file_hash(p: &Path) -> Result<String, String> {
    Ok(hash(&bytes(p)?))
}
fn reporter_source_hash() -> String {
    hash(
        &[
            include_bytes!("main.rs").as_slice(),
            include_bytes!("compact.rs").as_slice(),
            include_bytes!("replay.rs").as_slice(),
            include_bytes!("../Cargo.toml").as_slice(),
            include_bytes!("../Cargo.lock").as_slice(),
        ]
        .concat(),
    )
}
fn require(yes: bool, message: &str) -> Result<(), String> {
    if yes {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn a(v: &Value) -> Result<&Vec<Value>, String> {
    v.as_array().ok_or_else(|| "array absent".into())
}
fn n(v: &Value) -> Result<u64, String> {
    v.as_u64().ok_or_else(|| "unsigned integer absent".into())
}
fn s(v: &Value) -> Result<&str, String> {
    v.as_str().ok_or_else(|| "string absent".into())
}
fn key(v: &Value) -> String {
    serde_json::to_string(v).expect("value")
}
fn point(v: &Value) -> Result<[u64; 2], String> {
    let v = a(v)?;
    require(v.len() == 2, "point dimension")?;
    Ok([n(&v[0])?, n(&v[1])?])
}
fn mod_pow(mut x: u64, mut e: u64, p: u64) -> u64 {
    let mut y = 1;
    while e > 0 {
        if e & 1 != 0 {
            y = y * x % p;
        }
        x = x * x % p;
        e >>= 1;
    }
    y
}
fn on_curve(q: [u64; 2], p: u64, ca: u64, cb: u64) -> bool {
    q[0] < p && q[1] < p && q[1] * q[1] % p == (q[0] * q[0] % p * q[0] + ca * q[0] + cb) % p
}
fn add_points(u: [u64; 2], v: [u64; 2], p: u64, ca: u64) -> Option<[u64; 2]> {
    if u[0] == v[0] && (u[1] + v[1]) % p == 0 {
        return None;
    }
    let (num, den) = if u[0] == v[0] {
        ((3 * u[0] * u[0] + ca) % p, 2 * u[1] % p)
    } else {
        ((v[1] + p - u[1]) % p, (v[0] + p - u[0]) % p)
    };
    if den == 0 {
        return None;
    }
    let m = num * mod_pow(den, p - 2, p) % p;
    let x = (m * m + 2 * p - u[0] - v[0]) % p;
    Some([x, (m * ((u[0] + p - x) % p) + p - u[1]) % p])
}
fn verify_file(root: &Path, relative: &str, expected: &str) -> Result<(), String> {
    let p = Path::new(relative);
    require(
        !p.is_absolute()
            && !p
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir)),
        "unsafe manifest path",
    )?;
    require(
        file_hash(&root.join(p))? == expected,
        &format!("hash mismatch: {relative}"),
    )
}
fn verify_source_lock(repo: &Path) -> Result<Value, String> {
    let p =
        repo.join("research/prime_factor_base_degree_20261031/parallel_20261031/source-lock.json");
    let lock = read(&p)?;
    let mut count = 0;
    for f in a(&lock["source_files"])? {
        verify_file(repo, s(&f["path"])?, s(&f["sha256"])?)?;
        count += 1;
    }
    Ok(
        json!({"source_lock_sha256":file_hash(&p)?,"instrument_source_files_verified":count,"code_source_revision":lock["code_source_revision"],"instrument_source_unchanged":true}),
    )
}
fn verify_audit(root: &Path) -> Result<Value, String> {
    let manifest = read(&root.join("manifest.json"))?;
    require(
        manifest["complete"] == true
            && manifest["passes"] == true
            && manifest["mismatch_count"] == 0,
        "audit did not pass complete scope",
    )?;
    let cells = a(&manifest["cells"])?;
    require(cells.len() == 36, "audit cell count")?;
    let mut structured = 0;
    let mut dense = 0;
    let mut replayed = 0usize;
    let mut hashes = 0usize;
    let mut targets = BTreeMap::<String, BTreeSet<String>>::new();
    let mut arm_counts = BTreeMap::<String, (u64, u64, u64, BTreeSet<String>)>::new();
    for record in cells {
        let rel = s(&record["path"])?;
        verify_file(root, rel, s(&record["sha256"])?)?;
        hashes += 1;
        let b = bytes(&root.join(rel))?;
        require(
            b.len() as u64 == n(&record["bytes"])?,
            "audit cell size mismatch",
        )?;
        let c: Value = serde_json::from_slice(&b).map_err(|e| e.to_string())?;
        require(
            c["complete"] == true
                && c["mismatch_count"] == 0
                && c["model_dense_x_projection_equal"] == true,
            "audit cell failed",
        )?;
        for field in [
            "false_positive_roots",
            "false_negative_roots",
            "dense_false_positive_x_pairs",
            "dense_false_negative_x_pairs",
            "group_replay_failures",
        ] {
            require(
                a(&c[field])?.is_empty(),
                "audit discrepancy vector nonempty",
            )?;
        }
        require(
            c["dense_group_replay_failures"] == 0,
            "dense witness replay failures",
        )?;
        let branches = a(&c["branch_records"])?;
        require(branches.len() == 4, "audit missing signs")?;
        let signs: BTreeSet<_> = branches.iter().map(|b| key(&b["signs"])).collect();
        let all: BTreeSet<_> = [[1, 1], [1, -1], [-1, 1], [-1, -1]]
            .into_iter()
            .map(|x| key(&json!(x)))
            .collect();
        require(signs == all, "audit duplicate sign branches")?;
        require(
            branches.iter().all(|b| b["assignments"] == 4913),
            "audit branch assignment count",
        )?;
        structured += n(&c["structured_assignments_checked"])?;
        dense += n(&c["dense_assignments_checked"])?;
        let actual: BTreeSet<_> = a(&c["reported_roots"])?.iter().map(key).collect();
        let expected: BTreeSet<_> = a(&c["reference_roots"])?.iter().map(key).collect();
        require(actual == expected, "reported/reference root data differ")?;
        let r = point(&c["target"])?;
        let arm = s(&c["arm"])?.to_owned();
        targets
            .entry(arm.clone())
            .or_default()
            .insert(key(&c["target"]));
        let count = arm_counts.entry(arm).or_default();
        count.0 += n(&c["distinct_ordered_point_pairs"])?;
        count.1 += n(&c["doubles"])?;
        count.2 += n(&c["unordered_pairs"])?;
        for root in a(&c["reported_roots"])? {
            let u = point(&root["p"])?;
            let v = point(&root["q"])?;
            require(
                on_curve(u, 17, 2, 2)
                    && on_curve(v, 17, 2, 2)
                    && add_points(u, v, 17, 2) == Some(r),
                "independent audit relation replay failed",
            )?;
            count.3.insert(key(&json!([r, u, v])));
            replayed += 1;
        }
        for witness in a(&c["dense_witnesses"])? {
            require(witness["group_replay"] == true, "dense replay flag false")?;
            let root = &witness["group_witness"];
            require(
                add_points(point(&root["p"])?, point(&root["q"])?, 17, 2) == Some(r),
                "dense independent replay failed",
            )?;
        }
    }
    let mut references = Vec::new();
    for record in a(&manifest["reference_files"])? {
        let rel = s(&record["path"])?;
        verify_file(root, rel, s(&record["sha256"])?)?;
        hashes += 1;
        let r = read(&root.join(rel))?;
        let arm = s(&r["arm"])?;
        let observed = arm_counts.get(arm).ok_or("reference arm missing")?;
        require(
            observed.0 == 56 && observed.1 == 8 && observed.2 == 32,
            "independent aggregate invariant failed",
        )?;
        require(
            targets.get(arm).is_some_and(|x| x.len() == 18),
            "target count missing",
        )?;
        let inv = a(&r["infinity_target"]["relations"])?;
        require(inv.len() == 8, "inverse reference count")?;
        for pair in inv {
            let pair = a(pair)?;
            require(
                pair.len() == 2 && add_points(point(&pair[0])?, point(&pair[1])?, 17, 2).is_none(),
                "inverse pair failed replay",
            )?;
        }
        for v in r["independent_aggregation_invariants"]
            .as_object()
            .ok_or("aggregate invariants absent")?
            .values()
        {
            require(*v == true, "producer aggregate invariant false")?;
        }
        references.push(json!({"arm":arm,"finite_ordered_relations_recomputed":observed.0,"tangent_relations_recomputed":observed.1,"unordered_pairs_recomputed":observed.2,"distinct_target_point_pairs":observed.3.len(),"targets":18,"inverse_pairs_replayed":8,"natural_target_success_ratio":r["natural_target_success_ratio"]}));
    }
    require(
        structured == 707472 && dense == 10404,
        "assignment aggregate mismatch",
    )?;
    require(
        structured == n(&manifest["structured_assignments_checked"])?
            && dense == n(&manifest["dense_assignments_checked"])?,
        "manifest assignment totals mismatch",
    )?;
    Ok(
        json!({"manifest_sha256":file_hash(&root.join("manifest.json"))?,"cell_and_reference_hashes_verified":hashes,"cells":36,"structured_assignments_recorded_and_summed":structured,"dense_assignments_recorded_and_summed":dense,"reported_model_relations_independently_replayed":replayed,"roots_reenumerated":false,"false_positives":0,"false_negatives":0,"all_four_sign_branches_verified":true,"references":references,"passes":true}),
    )
}
fn walk(root: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut paths = fs::read_dir(root)
        .map_err(|e| e.to_string())?
        .map(|e| e.map(|v| v.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    for p in paths {
        require(!p.is_symlink(), "symlink in raw record tree")?;
        if p.is_dir() {
            walk(&p, out)?;
        } else if p
            .file_name()
            .is_some_and(|v| v == "summary.json" || v == "result.json")
        {
            out.push(p);
        }
    }
    Ok(())
}
fn verify_result_files(dir: &Path, r: &Value) -> Result<usize, String> {
    let fields = [
        ("input.json", "input_json_sha256"),
        ("input.tokens", "input_tokens_sha256"),
        ("input.ms", "input_msolve_sha256"),
        ("solver.json", "solver_stdout_sha256"),
        ("solver.stderr", "solver_stderr_sha256"),
    ];
    let mut count = 0;
    for (file, field) in fields {
        if let Some(h) = r[field].as_str() {
            require(
                file_hash(&dir.join(file))? == h,
                &format!("F4 raw file hash mismatch: {}", dir.join(file).display()),
            )?;
            count += 1;
        }
    }
    Ok(count)
}
fn verified(r: &Value) -> bool {
    r["status"] == "verified"
        && r["false_positives"] == 0
        && r["false_negatives"] == 0
        && r["every_reported_relation_group_replayed"] == true
        && r["gb_checks"]["buchberger_criterion"] == true
        && r["gb_checks"]["input_generator_remainders_zero"] == true
}
fn design_d(r: &Value) -> Value {
    if !r["d"].is_null() {
        r["d"].clone()
    } else {
        r["base_columns"]
            .as_u64()
            .filter(|b| b.is_multiple_of(2))
            .map(|b| json!(b / 2))
            .unwrap_or(Value::Null)
    }
}
fn group_key(r: &Value) -> String {
    key(
        &json!({"p":r["field_p"],"curve":r["curve"],"design_d":design_d(r),"arm":r["arm"],"control_name":r["control_name"],"base_x":r["base_x"]}),
    )
}
fn sum_known(rows: &[Value], field: &str) -> Value {
    let values = rows
        .iter()
        .map(|r| r["solver"][field].as_u64())
        .collect::<Option<Vec<_>>>();
    values
        .map(|v| json!(v.iter().sum::<u64>()))
        .unwrap_or(Value::Null)
}
fn max_known(rows: &[Value], field: &str) -> Value {
    let values = rows
        .iter()
        .map(|r| r[field].as_u64())
        .collect::<Option<Vec<_>>>();
    values
        .and_then(|v| v.into_iter().max())
        .map(|v| json!(v))
        .unwrap_or(Value::Null)
}
fn histogram(values: impl IntoIterator<Item = Option<u64>>) -> Value {
    let mut counts = BTreeMap::<String, u64>::new();
    for v in values {
        *counts
            .entry(v.map_or_else(|| "unknown".into(), |n| n.to_string()))
            .or_default() += 1;
    }
    json!(counts)
}
fn summarize_group(rows: &[Value]) -> Result<Value, String> {
    let first = &rows[0];
    let arm = s(&first["arm"])?;
    let secant = arm.ends_with("-secant");
    let mut targets = BTreeMap::<String, Vec<Value>>::new();
    for r in rows {
        targets
            .entry(key(&r["target"]))
            .or_default()
            .push(r.clone());
    }
    let mut target_records = Vec::new();
    let all_signs: BTreeSet<_> = [[1, 1], [1, -1], [-1, 1], [-1, -1]]
        .into_iter()
        .map(|x| key(&json!(x)))
        .collect();
    for group in targets.values() {
        let signs: BTreeSet<_> = group.iter().map(|r| key(&r["signs"])).collect();
        let complete = group.iter().all(verified)
            && if secant {
                group.len() == 4 && signs == all_signs
            } else {
                group.len() == 1
            };
        let relations: BTreeSet<_> = group
            .iter()
            .flat_map(|r| {
                r["group_replayed_relations"]
                    .as_array()
                    .into_iter()
                    .flatten()
            })
            .map(key)
            .collect();
        target_records.push(json!({"target":group[0]["target"],"complete_branches":complete,"observed_cases":group.len(),"maximum_handled_degree":if complete{max_known(group,"observed_f4_degree")}else{Value::Null},"pairs_reduced_sum_all_branches":sum_known(group,"pairs_reduced"),"rows_reduced_sum_all_branches":sum_known(group,"rows_reduced"),"zero_reductions_sum_all_branches":sum_known(group,"zero_reductions"),"peak_trace_columns":max_known(group,"peak_matrix_columns_in_trace"),"peak_trace_rows":max_known(group,"peak_matrix_rows_in_trace"),"ordered_relations_after_union":if complete{Some(relations.len())}else{None},"sat":if complete{Some(!relations.is_empty())}else{None},"complete_field_operation_cost_per_relation":null}));
    }
    let complete_targets = target_records
        .iter()
        .filter(|r| r["complete_branches"] == true)
        .count();
    let sat = target_records.iter().filter(|r| r["sat"] == true).count();
    let relation_sum: usize = target_records
        .iter()
        .filter_map(|r| r["ordered_relations_after_union"].as_u64())
        .map(|n| n as usize)
        .sum();
    Ok(
        json!({"group":serde_json::from_str::<Value>(&group_key(first)).unwrap(),"design_d_metadata":if first["d"].is_null(){"derived_from_half_matched_base_cardinality"}else{"recorded"},"observed_case_count":rows.len(),"verified_cases":rows.iter().filter(|r|verified(r)).count(),"failed_or_censored_cases":rows.iter().filter(|r|!verified(r)).count(),"target_count":targets.len(),"complete_target_count":complete_targets,"sat_target_count":sat,"natural_yield":{"numerator":sat,"denominator_completed_targets":complete_targets,"planned_target_population_known_only_from_panel":true},"ordered_relations_after_target_branch_union":relation_sum,"case_handled_degree_histogram":histogram(rows.iter().map(|r|r["observed_f4_degree"].as_u64())),"complete_target_max_handled_degree_histogram":histogram(target_records.iter().map(|r|r["maximum_handled_degree"].as_u64())),"case_input_degree_histogram":histogram(rows.iter().map(|r|r["maximum_input_degree"].as_u64())),"case_selected_pair_degree_histogram":histogram(rows.iter().map(|r|r["maximum_f4_selected_pair_degree"].as_u64())),"pairs_reduced_sum_all_cases_and_branches":sum_known(rows,"pairs_reduced"),"rows_reduced_sum_all_cases_and_branches":sum_known(rows,"rows_reduced"),"zero_reductions_sum_all_cases_and_branches":sum_known(rows,"zero_reductions"),"peak_trace_columns":max_known(rows,"peak_matrix_columns_in_trace"),"peak_trace_rows":max_known(rows,"peak_matrix_rows_in_trace"),"complete_field_operation_cost_per_relation":null,"clock_speedup":null,"targets":target_records}),
    )
}

fn compact_groups(mut groups: Vec<Value>) -> Vec<Value> {
    for g in &mut groups {
        let records = g["targets"].as_array().cloned().unwrap_or_default();
        g["target_metric_records_sha256"] = json!(hash(&serde_json::to_vec(&records).unwrap()));
        g["per_target_all_branch_pairs_histogram"] = histogram(
            records
                .iter()
                .map(|r| r["pairs_reduced_sum_all_branches"].as_u64()),
        );
        g["per_target_all_branch_rows_histogram"] = histogram(
            records
                .iter()
                .map(|r| r["rows_reduced_sum_all_branches"].as_u64()),
        );
        g.as_object_mut().unwrap().remove("targets");
    }
    groups
}
fn f4_report(root: &Path, replay: Option<&replay::Index>) -> Result<Value, String> {
    let mut files = Vec::new();
    walk(root, &mut files)?;
    let mut rows = Vec::new();
    let mut panels = Vec::new();
    let mut result_paths = BTreeMap::<PathBuf, String>::new();
    let mut source_files = Vec::new();
    let mut hashes = 0;
    let mut relation_replays = 0usize;
    let mut ids = BTreeSet::new();
    let mut effective_rows = Vec::new();
    let mut recovered = 0usize;
    let mut v2_linked = 0usize;
    for path in &files {
        if path.file_name().is_some_and(|n| n == "summary.json") {
            let panel = read(path)?;
            require(
                panel["schema"] == "secant-f4-panel/v1",
                "unknown panel summary",
            )?;
            for result in a(&panel["results"])? {
                let rel = s(&result["result_path"])?;
                verify_file(path.parent().unwrap(), rel, s(&result["result_sha256"])?)?;
                hashes += 1;
                result_paths.insert(
                    path.parent().unwrap().join(rel),
                    s(&result["result_sha256"])?.to_owned(),
                );
            }
            panels.push(json!({"path":path.strip_prefix(root).unwrap().display().to_string(),"sha256":file_hash(path)?,"requested_systems":panel["requested_systems"],"attempted_cells":panel["attempted_cells"],"completed_cells":panel["completed_cells"],"failed_or_unfinished":panel["failed_or_unfinished"],"result_records":a(&panel["results"])?.len()}));
        }
    }
    for path in files
        .iter()
        .filter(|p| p.file_name().is_some_and(|n| n == "result.json"))
    {
        let r = read(path)?;
        hashes += verify_result_files(path.parent().unwrap(), &r)?;
        require(
            ids.insert(s(&r["id"])?.to_owned()),
            "duplicate F4 result id",
        )?;
        if verified(&r) {
            let p = n(&r["field_p"])?;
            let ca = n(&r["curve"]["a"])?;
            let cb = n(&r["curve"]["b"])?;
            let target = point(&r["target"])?;
            require(
                (5..=65521).contains(&p) && ca < p && cb < p && on_curve(target, p, ca, cb),
                "F4 target/curve metadata invalid",
            )?;
            let actual: BTreeSet<_> = a(&r["roots"])?.iter().map(key).collect();
            let expected: BTreeSet<_> = a(&r["expected_roots"])?.iter().map(key).collect();
            require(actual == expected, "F4 root/reference data disagree")?;
            let input = read(&path.parent().unwrap().join("input.json"))?;
            require(
                hash(&serde_json::to_vec(&input).unwrap()) == s(&r["input_sha256"])?,
                "F4 semantic input hash mismatch",
            )?;
            let base: BTreeSet<_> = a(&input["base_points"])?
                .iter()
                .map(point)
                .collect::<Result<_, _>>()?;
            for pair in a(&r["group_replayed_relations"])? {
                let pair = a(pair)?;
                require(pair.len() == 2, "relation pair shape")?;
                let u = point(&pair[0])?;
                let v = point(&pair[1])?;
                require(
                    base.contains(&u)
                        && base.contains(&v)
                        && on_curve(u, p, ca, cb)
                        && on_curve(v, p, ca, cb)
                        && add_points(u, v, p, ca) == Some(target),
                    "F4 relation independent replay failed",
                )?;
                relation_replays += 1;
            }
        }
        let result_sha = file_hash(path)?;
        let mut effective = r.clone();
        if let Some(companion) = replay.and_then(|v| v.records.get(&result_sha)) {
            v2_linked += 1;
            if companion["v2_status"] == "verified" {
                let input = read(&path.parent().unwrap().join("input.json"))?;
                effective = replay::effective(&r, companion, &input)?;
                if !verified(&r) {
                    recovered += 1;
                }
            }
        }
        effective_rows.push(effective);
        source_files.push(json!({"path":path.strip_prefix(root).unwrap().display().to_string(),"sha256":result_sha,"listed_in_finished_panel_manifest":result_paths.contains_key(path)}));
        rows.push(r);
    }
    let mut groups = BTreeMap::<String, Vec<Value>>::new();
    for r in &rows {
        groups.entry(group_key(r)).or_default().push(r.clone());
    }
    let groups = groups
        .values()
        .map(|g| summarize_group(g))
        .collect::<Result<Vec<_>, _>>()?;
    let groups = compact_groups(groups);
    let mut v2groups = BTreeMap::<String, Vec<Value>>::new();
    for r in &effective_rows {
        v2groups.entry(group_key(r)).or_default().push(r.clone());
    }
    let v2groups = if replay.is_some() {
        json!(compact_groups(
            v2groups
                .values()
                .map(|g| summarize_group(g))
                .collect::<Result<Vec<_>, _>>()?
        ))
    } else {
        Value::Null
    };
    let requested = panels
        .iter()
        .map(|p| p["requested_systems"].as_u64())
        .collect::<Option<Vec<_>>>()
        .map(|v| v.iter().sum::<u64>());
    let whole = (!panels.is_empty())
        && requested == Some(rows.len() as u64)
        && rows.iter().all(verified)
        && panels.iter().all(|p| p["failed_or_unfinished"] == 0)
        && source_files
            .iter()
            .all(|f| f["listed_in_finished_panel_manifest"] == true);
    let unlisted: Vec<_> = source_files
        .into_iter()
        .filter(|f| f["listed_in_finished_panel_manifest"] != true)
        .collect();
    Ok(
        json!({"panels":panels,"whole_observed_panel_complete":whole,"raw_result_count":rows.len(),"verified_result_count":rows.iter().filter(|r|verified(r)).count(),"original_failed_or_capped_records":rows.iter().filter(|r|!verified(r)).count(),"recorded_raw_hashes_verified":hashes,"reported_relations_independently_replayed":relation_replays,"groups":groups,"offline_replay_v2":replay.map(|r|&r.summary),"v2_linked_original_records":v2_linked,"original_failed_or_capped_records_proved_by_v2":recovered,"saved_basis_verified_case_count_with_v2":effective_rows.iter().filter(|r|verified(r)).count(),"groups_with_v2_root_set_verification":v2groups,"original_extraction_cap_status_changed":false,"source_hash_binding":"finished_panel_summary_hashes_recursively_bind_all_result_hashes;each_result_raw_file_hash_was_rechecked","unlisted_result_records":unlisted,"complete_field_operation_cost_per_relation":null,"wall_time_speedup":null}),
    )
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if !(5..=7).contains(&args.len()) {
        return Err(
            "usage: secant-descriptive-report REPO_ROOT AUDIT_DIR|- F4_ROOT FRESH_OUTPUT.json [REPLAY_V2_ROOT|-] [LADDER_ROOT|-]"
                .into(),
        );
    }
    let source = verify_source_lock(Path::new(&args[1]))?;
    let audit = if args[2] == "-" {
        Value::Null
    } else {
        verify_audit(Path::new(&args[2]))?
    };
    let replays = match args.get(5).filter(|v| *v != "-") {
        Some(path) => Some(replay::load(Path::new(path))?),
        None => None,
    };
    let f4 = f4_report(Path::new(&args[3]), replays.as_ref())?;
    let construction = match args.get(6).filter(|v| *v != "-") {
        Some(path) => compact::ladder(Path::new(path))?,
        None => Value::Null,
    };
    let report = json!({"schema":"secant-post-data-descriptive-report/v1","reporter_source_sha256":reporter_source_hash(),"post_data_analysis":true,"preregistered_decisive_analysis":false,"instrument_or_protocol_changed":false,"instrument_source_audit":source,"toy_audit":audit,"f4_descriptive":f4,"construction":construction,"interpretation_limits":["Descriptive aggregation from sealed observed records; no new experiments or retries.","Four sign branches are summed for pair/row counters and maximized for handled degree.","Unknown/missing cases retain null degree or cost; no discarded branch is credited.","Actual reported handled degree remains distinct from homogeneous regularity and sugar degree.","Stock solver field-operation totals remain null; pair/row counts alone do not establish cost per relation.","L0 shared-host clocks are not used to infer speedup.","Independent replay here rechecks recorded relations and hashes; it does not re-enumerate the root search or certify full ideal/multiplicity equality.","Separate v2 saved-basis root-set proof does not relabel capped v1 extraction as completed."]});
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[4])?;
    file.write_all(&serde_json::to_vec_pretty(&report)?)?;
    file.write_all(b"\n")?;
    let short = compact::report(&report);
    let short_bytes = serde_json::to_vec(&short)?;
    require(short_bytes.len() <= 65536, "compact summary exceeds64KiB")?;
    let compact_path = format!("{}.compact.json", &args[4]);
    let mut cf = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&compact_path)?;
    cf.write_all(&short_bytes)?;
    cf.write_all(b"\n")?;
    println!(
        "{}",
        json!({"report_sha256":file_hash(Path::new(&args[4]))?,"f4_results":report["f4_descriptive"]["raw_result_count"],"groups":report["f4_descriptive"]["groups"].as_array().map(Vec::len),"source_lock_verified":true})
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn row(sign: [i8; 2], pairs: u64) -> Value {
        json!({"arm":"candidate-secant","field_p":17,"curve":{"p":17,"a":2,"b":2},"base_columns":4,"base_x":[5,6,10,16],"target":[10,6],"signs":sign,"status":"verified","false_positives":0,"false_negatives":0,"every_reported_relation_group_replayed":true,"gb_checks":{"buchberger_criterion":true,"input_generator_remainders_zero":true},"observed_f4_degree":3,"maximum_input_degree":2,"maximum_f4_selected_pair_degree":3,"peak_matrix_columns_in_trace":15,"peak_matrix_rows_in_trace":17,"solver":{"pairs_reduced":pairs,"rows_reduced":pairs*2,"zero_reductions":0},"group_replayed_relations":[]} )
    }
    #[test]
    fn incomplete_branch_has_no_target_degree_credit() {
        let r = summarize_group(&[row([1, 1], 7)]).unwrap();
        assert_eq!(r["complete_target_count"], 0);
        assert_eq!(r["targets"][0]["maximum_handled_degree"], Value::Null);
    }
    #[test]
    fn all_branch_counters_are_summed_not_maximized() {
        let rows: Vec<_> = [[1, 1], [1, -1], [-1, 1], [-1, -1]]
            .into_iter()
            .enumerate()
            .map(|(i, s)| row(s, i as u64 + 1))
            .collect();
        let r = summarize_group(&rows).unwrap();
        assert_eq!(r["complete_target_count"], 1);
        assert_eq!(r["targets"][0]["pairs_reduced_sum_all_branches"], 10);
        assert_eq!(r["rows_reduced_sum_all_cases_and_branches"], 20);
        assert_eq!(r["complete_field_operation_cost_per_relation"], Value::Null);
    }
    #[test]
    fn supplied_group_replays_match_known_values() {
        assert_eq!(add_points([5, 1], [6, 3], 17, 2), Some([10, 6]));
        assert_eq!(add_points([5, 1], [5, 16], 17, 2), None);
    }
    #[test]
    fn manifest_path_escape_is_rejected() {
        assert!(verify_file(Path::new("/tmp"), "../outside", "00").is_err());
    }
}
