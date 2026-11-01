use super::*;
pub(super) struct Index {
    pub records: BTreeMap<String, Value>,
    pub summary: Value,
}
pub(super) fn load(root: &Path) -> Result<Index, String> {
    let path = root.join("manifest.json");
    let m = read(&path)?;
    require(
        m["schema"] == "secant-f4-offline-replay-manifest/v2",
        "wrong v2 replay manifest",
    )?;
    require(m["new_cas_calls"] == 0, "v2 manifest reports new CAS calls")?;
    let mut records = BTreeMap::new();
    let mut counts = BTreeMap::<String, u64>::new();
    for entry in a(&m["records"])? {
        let rel = s(&entry["companion_path"])?;
        verify_file(root, rel, s(&entry["companion_sha256"])?)?;
        let r = read(&root.join(rel))?;
        require(
            r["schema"] == "secant-f4-offline-replay/v2"
                && r["original_status"] == entry["original_status"]
                && r["v2_status"] == entry["v2_status"],
            "v2 companion linkage mismatch",
        )?;
        let original = s(&r["raw_hashes"]["original_result_sha256"])?.to_owned();
        require(
            records.insert(original, r.clone()).is_none(),
            "duplicate v2 original-result hash",
        )?;
        *counts.entry(s(&r["v2_status"])?.to_owned()).or_default() += 1;
    }
    let summary = json!({"manifest_sha256":file_hash(&path)?,"requested_cases":m["requested_cases"],"replayed_cases":m["replayed_cases"],"verified_cases":m["verified_cases"],"failed_cases":m["failed_cases"],"unattempted_cases":m["unattempted_cases"],"v1_failed_or_capped_cases_verified_by_v2":m["v1_failed_or_capped_cases_verified_by_v2"],"independently_counted_companion_statuses":counts,"companion_file_hashes_verified":records.len(),"new_cas_calls":0,"coordinate_cube_search":m["coordinate_cube_search"],"verification_basis":"separate_saved_basis_exact_root_set_proof;original_v1_caps_remain_caps"});
    Ok(Index { records, summary })
}
pub(super) fn effective(raw: &Value, c: &Value, input: &Value) -> Result<Value, String> {
    require(
        c["v2_status"] == "verified" && c["new_cas_calls"] == 0,
        "v2 companion is not verified without new CAS",
    )?;
    require(
        c["original_status"] == raw["status"]
            && c["raw_hashes"]["input_json_sha256"] == raw["input_json_sha256"]
            && c["raw_hashes"]["solver_stdout_sha256"] == raw["solver_stdout_sha256"]
            && c["raw_hashes"]["solver_stderr_sha256"] == raw["solver_stderr_sha256"],
        "v2/raw data hashes or original status differ",
    )?;
    for flag in [
        "input_in_exported_ideal_by_exact_normal_forms",
        "all_reference_roots_vanish_in_exported_basis",
        "finite_field_root_sets_equal_by_two_inclusions",
    ] {
        require(c["proof"][flag] == true, "v2 proof obligation absent")?;
    }
    require(
        c["proof"]["checks"]["input_generator_remainders_zero"] == true
            && c["proof"]["checks"]["buchberger_criterion"] == true
            && c["false_positives"] == 0
            && c["false_negatives"] == 0,
        "v2 proof check failed",
    )?;
    let p = n(&raw["field_p"])?;
    let ca = n(&raw["curve"]["a"])?;
    let cb = n(&raw["curve"]["b"])?;
    let target = point(&raw["target"])?;
    let base: BTreeSet<_> = a(&input["base_points"])?
        .iter()
        .map(point)
        .collect::<Result<_, _>>()?;
    for pair in a(&c["group_replayed_relations"])? {
        let pair = a(pair)?;
        require(pair.len() == 2, "v2 pair shape")?;
        let u = point(&pair[0])?;
        let v = point(&pair[1])?;
        require(
            base.contains(&u)
                && base.contains(&v)
                && on_curve(u, p, ca, cb)
                && on_curve(v, p, ca, cb)
                && add_points(u, v, p, ca) == Some(target),
            "v2 companion relation independent replay failed",
        )?;
    }
    let mut out = raw.clone();
    out["status"] = json!("verified");
    out["false_positives"] = json!(0);
    out["false_negatives"] = json!(0);
    out["every_reported_relation_group_replayed"] = json!(true);
    out["gb_checks"] = c["proof"]["checks"].clone();
    out["group_replayed_relations"] = c["group_replayed_relations"].clone();
    for field in [
        "maximum_input_degree",
        "maximum_f4_selected_pair_degree",
        "observed_f4_degree",
        "maximum_exported_basis_degree",
        "peak_matrix_rows_in_trace",
        "peak_matrix_columns_in_trace",
    ] {
        if !raw[field].is_null() {
            require(
                raw[field] == c["metrics"][field],
                "v2 changes recorded v1 metric",
            )?;
        }
        out[field] = c["metrics"][field].clone();
    }
    if !out["solver"].is_object() {
        out["solver"] = json!({});
    }
    for field in ["pairs_reduced", "rows_reduced", "zero_reductions"] {
        if !raw["solver"][field].is_null() {
            require(
                raw["solver"][field] == c["metrics"][field],
                "v2 changes recorded v1 counter",
            )?;
        }
        out["solver"][field] = c["metrics"][field].clone();
    }
    out["solver"]["field_additions"] = Value::Null;
    out["solver"]["field_multiplications"] = Value::Null;
    let selected = a(&c["metrics"]["selected_pair_degree_trace"])?;
    let max = selected
        .iter()
        .map(n)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .max()
        .unwrap_or(0);
    if c["metrics"]["selected_degree_trace_complete"] == true {
        require(
            out["observed_f4_degree"] == json!(max.max(n(&out["maximum_input_degree"])?)),
            "v2 handled-degree definition mismatch",
        )?;
    }
    Ok(out)
}
