use super::*;
pub(super) fn ladder(root: &Path) -> Result<Value, String> {
    let m = read(&root.join("manifest.json"))?;
    let mut cells = Vec::new();
    for c in a(&m["cells"])? {
        let directory = s(&c["directory"])?;
        let base = root.join(directory);
        let case = read(&base.join("case.json"))?;
        verify_file(&base, "systems.json", s(&c["systems_sha256"])?)?;
        verify_file(&base, "references.json", s(&case["references_sha256"])?)?;
        verify_file(&base, "attempts.json", s(&case["attempt_log_sha256"])?)?;
        let attempts = read(&base.join("attempts.json"))?;
        require(
            a(&attempts)?.len() as u64 == n(&c["attempts"])?,
            "construction attempt count mismatch",
        )?;
        cells.push(json!({"curve_slug":c["curve"],"p":case["curve"]["p"],"a":case["curve"]["a"],"b":case["curve"]["b"],"d":c["d"],"status":c["status"],"construction_attempts":c["attempts"],"forced_interpolation_points":a(&case["forced_interpolation_points"])?.len(),"construction_operations":c["construction_operations"],"systems_count":c["systems_count"],"systems_sha256":c["systems_sha256"],"case_sha256":file_hash(&base.join("case.json"))?,"reference_sha256":case["references_sha256"],"attempt_log_sha256":case["attempt_log_sha256"]}));
    }
    Ok(
        json!({"manifest_sha256":file_hash(&root.join("manifest.json"))?,"requested_cells":m["requested_cells"],"accepted_cells":m["accepted_cells"],"cells":cells}),
    )
}
pub(super) fn report(full: &Value) -> Value {
    let mut cells = BTreeMap::<String, Value>::new();
    for (kind, groups) in [
        ("v1_recorded_status", &full["f4_descriptive"]["groups"]),
        (
            "saved_basis_v2_verified",
            &full["f4_descriptive"]["groups_with_v2_root_set_verification"],
        ),
    ] {
        for g in groups.as_array().into_iter().flatten() {
            let meta = &g["group"];
            let cellkey = key(
                &json!({"p":meta["p"],"a":meta["curve"]["a"],"b":meta["curve"]["b"],"d":meta["design_d"]}),
            );
            let cell=cells.entry(cellkey.clone()).or_insert_with(||json!({"cell":serde_json::from_str::<Value>(&cellkey).unwrap(),"curve_slug":null,"construction":null,"v1_recorded_status":[],"saved_basis_v2_verified":[]}));
            let metrics = json!({"arm":meta["arm"],"control":meta["control_name"],"target_count":g["target_count"],"complete_targets":g["complete_target_count"],"verified_cases":g["verified_cases"],"unknown_or_failed_cases":g["failed_or_censored_cases"],"handled_degree_histogram":g["complete_target_max_handled_degree_histogram"],"pairs_sum_all_branches":g["pairs_reduced_sum_all_cases_and_branches"],"rows_sum_all_branches":g["rows_reduced_sum_all_cases_and_branches"],"peak_columns":g["peak_trace_columns"],"peak_rows":g["peak_trace_rows"],"sat_targets":g["sat_target_count"],"ordered_relations":g["ordered_relations_after_target_branch_union"],"complete_field_cost_per_relation":null});
            cell[kind].as_array_mut().unwrap().push(metrics);
        }
    }
    for c in full["construction"]["cells"]
        .as_array()
        .into_iter()
        .flatten()
    {
        let cellkey = key(&json!({"p":c["p"],"a":c["a"],"b":c["b"],"d":c["d"]}));
        if let Some(cell) = cells.get_mut(&cellkey) {
            cell["curve_slug"] = c["curve_slug"].clone();
            cell["construction"] = json!({"attempts":c["construction_attempts"],"forced_points":c["forced_interpolation_points"],"operations":c["construction_operations"],"status":c["status"]});
        }
    }
    for cell in cells.values_mut() {
        if cell["cell"]["p"] == 17 {
            cell["curve_slug"] = json!("icv1-fp5-tm1-a77cc6af");
        }
    }
    json!({"schema":"secant-descriptive-compact/v1","post_data_descriptive_only":true,"original_v1_records":full["f4_descriptive"]["raw_result_count"],"original_v1_verified":full["f4_descriptive"]["verified_result_count"],"original_v1_failed_or_capped":full["f4_descriptive"]["original_failed_or_capped_records"],"v2_verified_saved_basis_cases":full["f4_descriptive"]["saved_basis_verified_case_count_with_v2"],"v1_caps_resolved_by_separate_v2_proof":full["f4_descriptive"]["original_failed_or_capped_records_proved_by_v2"],"operational_v1_cap_status_changed":false,"cells":cells.into_values().collect::<Vec<_>>(),"complete_field_cost_per_relation":null,"clock_speedup":null})
}
