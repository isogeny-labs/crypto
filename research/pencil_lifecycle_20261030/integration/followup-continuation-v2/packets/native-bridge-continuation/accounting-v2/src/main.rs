use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
fn sha(p: &Path) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn lines(p: &Path) -> Vec<Value> {
    fs::read_to_string(p)
        .unwrap()
        .lines()
        .map(|x| serde_json::from_str(x).unwrap())
        .collect()
}
fn id(x: &Value) -> (u64, u64) {
    let z = x.get("identity").unwrap_or(x);
    (
        z["archive_row"].as_u64().unwrap(),
        z["case_index"].as_u64().unwrap(),
    )
}
fn identities(x: &Value, a: &Value, location: Value, errors: &mut Vec<Value>) {
    for f in [
        "target_index",
        "workload_id",
        "workload_sha256",
        "algorithm_seed",
    ] {
        let typed = if f == "target_index" {
            x[f].as_u64().is_some()
        } else {
            x[f].as_str().is_some_and(|s| !s.is_empty())
        };
        if !typed || x[f] != a[f] {
            errors.push(
                json!({"identity_binding_failed":location,"field":f,"actual":x[f],"archive":a[f]}),
            );
        }
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 3);
    let b = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    let archive = lines(&b.join("archive.jsonl"));
    let hash = sha(&b.join("archive.jsonl"));
    let index = lines(&b.join("input-index.jsonl"));
    let observed = lines(&b.join("run.stdout"));
    let prior: Value =
        serde_json::from_str(&fs::read_to_string(b.join("analysis.json")).unwrap()).unwrap();
    let receipt: Value =
        serde_json::from_str(&fs::read_to_string(b.join("receipt.json")).unwrap()).unwrap();
    let mut errors = vec![];
    let mut provenance_errors = vec![];
    let mp = b.join("pre-execution-manifest.json");
    if mp.exists() {
        let manifest: Value = serde_json::from_slice(&fs::read(&mp).unwrap()).unwrap();
        if receipt["preexecution_manifest_sha256"].as_str() != Some(sha(&mp).as_str()) {
            provenance_errors.push(json!({"receipt_manifest_hash_mismatch":true}));
        }
        for name in ["archive.jsonl", "input-index.jsonl"] {
            let file = b.join(name);
            if manifest["files"][name]["sha256"].as_str() != Some(sha(&file).as_str())
                || manifest["files"][name]["bytes"].as_u64()
                    != Some(fs::metadata(&file).unwrap().len())
            {
                provenance_errors.push(json!({"manifest_input_binding_failed":name}));
            }
        }
    } else {
        provenance_errors.push(json!({"missing_preexecution_manifest":true}));
    }
    for x in &index {
        let key = id(x);
        if let Some(a) = archive.get(key.0 as usize) {
            identities(x, a, json!({"index":key}), &mut provenance_errors);
            let case = a["cases"]
                .as_array()
                .and_then(|cs| cs.iter().find(|c| c["case_index"].as_u64() == Some(key.1)));
            if let Some(c) = case {
                for f in ["a", "b"] {
                    if x[f].as_u64().is_none() || x[f] != c[f] {
                        provenance_errors.push(json!({"index_case_label_mismatch":key,"field":f}));
                    }
                }
            } else {
                provenance_errors.push(json!({"index_case_missing":key}));
            }
        } else {
            provenance_errors.push(json!({"index_archive_row_missing":key}));
        }
    }
    let expected: BTreeSet<_> = index.iter().map(id).collect();
    let mut seen = BTreeSet::new();
    let mut dispositions = vec![];
    let mut setup: BTreeMap<u64, Value> = BTreeMap::new();
    let mut status: BTreeMap<String, u64> = BTreeMap::new();
    let mut adapters: BTreeMap<String, u64> = BTreeMap::new();
    let mut grouped: BTreeMap<u64, Vec<Value>> = BTreeMap::new();
    for x in &observed {
        let key = id(x);
        let ident = x.get("identity").unwrap_or(x);
        let r = x.get("result").unwrap_or(x);
        if !seen.insert(key) {
            errors.push(json!({"duplicate":key}));
        }
        if !expected.contains(&key) {
            errors.push(json!({"unexpected":key}));
        }
        let a = archive.get(key.0 as usize);
        let case = a
            .and_then(|v| v["cases"].as_array())
            .and_then(|cs| cs.iter().find(|c| c["case_index"].as_u64() == Some(key.1)));
        if a.is_none() || case.is_none() {
            errors.push(json!({"unbound_archive_case":key}));
        }
        if let Some(a) = a {
            identities(ident, a, json!({"observed":key}), &mut provenance_errors);
            for f in [
                "target_index",
                "workload_id",
                "workload_sha256",
                "algorithm_seed",
            ] {
                if !ident[f].is_null() && ident[f] != a[f] {
                    errors.push(json!({"identity_mismatch":key,"field":f}));
                }
            }
        }
        let st = r["status"].as_str().unwrap_or("missing").to_string();
        *status.entry(st.clone()).or_default() += 1;
        let ast = r["adapter"]["status"].as_str();
        if let Some(s) = ast {
            *adapters.entry(s.into()).or_default() += 1;
        }
        let meta = json!({"setup_checks_field":x.get("setup_checks_field"),"constructor_field":x.get("constructor_field")});
        if let Some(old) = setup.get(&key.0) {
            if old != &meta {
                errors.push(json!({"inconsistent_setup_copy":key}));
            }
        } else {
            setup.insert(key.0, meta);
        }
        let phases = [
            "label_reference_field",
            "eligibility_field",
            "key_reference_field",
            "terminal_reference_field",
        ]
        .map(|k| (k.to_string(), r.get(k).cloned().unwrap_or(Value::Null)))
        .into_iter()
        .collect::<serde_json::Map<String, Value>>();
        let record = json!({"physical_identity":[hash,key.0,key.1],"target_index":ident["target_index"],"status":st,"adapter_status":ast,"native_evaluated":r.get("native_evaluated"),"post_terminal_diagnostic":r.get("post_terminal_diagnostic"),"reference_and_eligibility_phases":phases,"adapter_field":r["adapter"]["field"],"adapter_scalar_before":r["adapter"]["scalar_before"],"adapter_scalar_after":r["adapter"]["scalar_after"],"graph":r.get("graph"),"missing_fields_are_zero":false});
        grouped.entry(key.0).or_default().push(record.clone());
        dispositions.push(record);
    }
    let missing: Vec<_> = expected.difference(&seen).copied().collect();
    if !missing.is_empty() {
        errors.push(json!({"missing":missing}));
    }
    if expected.len() != index.len() {
        errors.push(json!({"duplicate_expected_index":true}));
    }
    let mut totals: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    let mut physical = vec![];
    for (n, a) in archive.iter().enumerate() {
        let cases = a["cases"].as_array().map(|x| x.len());
        let emitted = grouped.remove(&(n as u64)).unwrap_or_default();
        let included = expected.iter().any(|k| k.0 == n as u64);
        let entry = setup.get(&(n as u64));
        if included && entry.is_none() {
            errors.push(json!({"missing_setup":n}));
        }
        if let Some(entry) = entry {
            for phase in ["setup_checks_field", "constructor_field"] {
                if let Some(metrics) = entry[phase].as_object() {
                    for (k, v) in metrics {
                        if let Some(n) = v.as_u64() {
                            let dst = totals
                                .entry(phase.into())
                                .or_default()
                                .entry(k.clone())
                                .or_default();
                            *dst = dst.checked_add(n).unwrap();
                        } else {
                            errors.push(json!({"invalid_counter":{"row":n,"phase":phase,"metric":k,"value":v}}));
                        }
                    }
                } else {
                    errors.push(json!({"missing_counter_phase":{"row":n,"phase":phase}}));
                }
            }
        }
        physical.push(json!({"physical_identity":[hash,n],"p":a["p"],"target_index":a["target_index"],"workload_id":a["workload_id"],"classification":if included{"selected_retained_partition"}else{"archive_row_not_selected_by_index"},"archived_cases":cases,"observed_dispositions":emitted.len(),"setup_once":entry,"dispositions":emitted}));
    }
    let tv = serde_json::to_value(&totals).unwrap();
    let totals_match = tv == prior["setup_accounting"]["totals_count_each_target_once"];
    if !totals_match {
        errors.push(json!({"saved_totals_mismatch":true}));
    }
    let categories = json!([{"category":"field_setup_and_constructor","status":"recorded_once_per_physical_row"},{"category":"per_case_field_and_ring_counters","status":"retained_with_nulls_and_disposition_boundaries"},{"category":"row_generation_or_search","status":"absent_retained_coordinates_are_inputs"},{"category":"allocation_hashing_comparison_memory","status":"not_fully_measured"},{"category":"primality_constructor_work","status":"not_fully_measured_by_field_counters"},{"category":"sqrt_nonresidue_search","status":"not_full_operation_census"},{"category":"parsing_curve_construction_process_setup","status":"not_complete_cost_boundary"},{"category":"offline_dense_and_rank_diagnostics","status":"reference_analysis_not_candidate_cost; potential counter_overlap_requires_isolation"},{"category":"matched_rho_and_controls","status":"not_in_this_retained_archive_audit"}]);
    let mut sources = vec![];
    for name in [
        "archive.jsonl",
        "input-index.jsonl",
        "run.stdout",
        "run.stderr",
        "receipt.json",
        "analysis.json",
        "run.py",
        "src/pruned.rs",
        "src/duplicate.rs",
        "src/graph.rs",
    ] {
        sources.push(json!({"path":b.join(name),"sha256":sha(&b.join(name))}));
    }
    if mp.exists() {
        sources.push(json!({"path":mp,"sha256":sha(&mp)}));
    }
    let pass = errors.is_empty() && provenance_errors.is_empty();
    let result = json!({"schema":"research.readonly-accounting-reconciliation/v2","method":"native Rust parsing of saved files; no execution of producer or cryptographic replay","scientific_execution":false,"full_cohort_obligation":1260,"complete_cost_ratio":null,"natural_yield":null,"archive_rows":archive.len(),"selected_physical_rows":setup.len(),"expected_dispositions":expected.len(),"observed_dispositions":dispositions.len(),"status_counts":status,"adapter_status_counts":adapters,"historical_native_exit":receipt["exit_code"],"errors":errors,"accounting_pass":errors.is_empty(),"provenance_errors":provenance_errors,"provenance_pass":provenance_errors.is_empty(),"provenance_scope":"receipt-to-manifest-to-two-input hash chain and required target identities; not receipt authenticity, output execution authentication or all manifest file verification","reconciliation_pass":pass,"saved_setup_totals_match":totals_match,"setup_totals_counted_once":totals,"physical_rows":physical,"cost_categories":categories,"sources":sources});
    fs::write(out, serde_json::to_string_pretty(&result).unwrap() + "\n").unwrap();
    println!(
        "{}",
        json!({"reconciliation_pass":pass,"provenance_errors":provenance_errors,"archive_rows":archive.len(),"dispositions":dispositions.len(),"errors":errors,"saved_totals_match":totals_match})
    );
    if !pass {
        std::process::exit(1)
    }
}
