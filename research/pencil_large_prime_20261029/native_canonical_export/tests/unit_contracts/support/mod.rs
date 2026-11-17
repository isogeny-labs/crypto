use super::*;
pub fn provenance(p: &mut V, d: &Path, study: &Path) -> V {
    let keys: Vec<_> = p["extensions"]["analysis_support_sha256"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    let mut sources = serde_json::Map::new();
    let mut receipt_sources = vec![];
    for key in keys {
        let path = d.join("sources").join(&key);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let bytes = if key == "native_price/src/bin/analyze_cohort.rs" {
            fs::read(study.join("fixtures/analyze_cohort.reference.txt")).unwrap()
        } else {
            format!("software fixture source {key}\n").into_bytes()
        };
        fs::write(&path, bytes).unwrap();
        let b = binding(&path);
        p["extensions"]["analysis_support_sha256"][&key] = b["sha256"].clone();
        sources.insert(key.clone(), b.clone());
        receipt_sources.push(json!({"path":key,"sha256":b["sha256"]}));
    }
    let mapping =
        p["extensions"]["analysis_support_sha256"]["native_price/src/analyzer.rs"].clone();
    p["extensions"]["finite_cost_model"]["mapping_sha256"] = mapping.clone();
    p["analysis_plan"]["script"] = sources["native_price/src/bin/analyze_cohort.rs"].clone();
    let mut binaries = serde_json::Map::new();
    let mut receipt_bins = vec![];
    for name in ["analyze_pair", "analyze_cohort"] {
        let path = d.join(format!("{name}.software-binary-fixture"));
        fs::write(&path, b"software fixture; not executed as analyzer").unwrap();
        let mut b = binding(&path);
        b["bytes"] = json!(fs::metadata(&path).unwrap().len());
        binaries.insert(name.to_owned(), b.clone());
        let mut rb = b;
        rb["name"] = json!(name);
        receipt_bins.push(rb);
    }
    let receipt = json!({"schema":"pencil-native-analysis-build/v1","source_revision":"1234567890123456789012345678901234567890","source_files_match_commit":true,"build_exit_code":0,"mapping_sha256":mapping,"sources":receipt_sources,"binaries":receipt_bins,"scope":"software fixtures only; not a real compilation receipt"});
    let rp = d.join("analysis-build-software-receipt.json");
    fs::write(&rp, serde_json::to_vec(&receipt).unwrap()).unwrap();
    p["extensions"]["analysis_build"] = json!({"source_commit":receipt["source_revision"],"receipt_sha256":digest(&rp).unwrap(),"binaries":binaries});
    p["extensions"]["finite_cost_model"]["analyzer_binaries_sha256"] = json!({"analyze_pair":binaries["analyze_pair"]["sha256"],"analyze_cohort":binaries["analyze_cohort"]["sha256"]});
    json!({"sources":sources,"build_receipt":binding(&rp),"binaries":binaries})
}
fn grid(expected: usize, aggregate: bool) -> V {
    let mut out = vec![];
    for square in [0, 1, 2] {
        for inverse in [1, 10, 100, 1000] {
            for logical in [0, 1, 10] {
                let mut c =
                    json!({"square_twice":square,"inversion":inverse,"native_tenths":logical});
                if aggregate {
                    let z = json!({"numerator":"0","denominator":"1"});
                    for(k,v)in json!({"expected_pairs":expected,"admitted_pairs":0,"missing_or_ineligible_pairs":expected,"complete":false,"candidate_sum":null,"reference_sum":null,"algorithm_ratio_of_sums":null,"candidate_sum_including_calibration":null,"reference_sum_including_calibration":null,"ratio_of_sums_including_calibration":null,"qualified_ratio":null,"partial_admitted_sums_diagnostic_only":{"candidate":z,"reference":z,"candidate_including_calibration":z,"reference_including_calibration":z}}).as_object().unwrap(){c[k]=v.clone();}
                } else {
                    for k in [
                        "candidate",
                        "reference",
                        "ratio",
                        "candidate_calibration",
                        "reference_calibration",
                        "candidate_including_calibration",
                        "reference_including_calibration",
                        "ratio_including_calibration",
                    ] {
                        c[k] = V::Null;
                    }
                }
                out.push(c);
            }
        }
    }
    json!(out)
}
pub fn complete_native(v: &mut V, plan: &V, rows: &[V]) {
    v["qualified_ratios"] = V::Null;
    v["aggregate_cells"] = grid(5, true);
    v["streams"] = json!((0..5)
        .map(|i| json!({"stream_index":i,"cells":grid(1,true)}))
        .collect::<Vec<_>>());
    let pairs:Vec<_>=(0..5).map(|i|{let cseq=(i+1)*3;let rseq=cseq+1;json!({"plan":"plan.json","session_id":"software-session","workload_id":"software-W","stream_index":i,"round":i+1,"candidate_seq":cseq,"reference_seq":rseq,"candidate_record_id":rows.iter().find(|r|r["seq"]==json!(cseq)).map(|r|r["record_id"].clone()),"reference_record_id":rows.iter().find(|r|r["seq"]==json!(rseq)).map(|r|r["record_id"].clone()),"coverage_blockers":["software fixture has no independent coverage review"],"cells":grid(1,false)})}).collect();
    v["pair_rows"] = json!(pairs);
    v["expected_workloads"] = json!(1);
    v["expected_measured_pairs"] = json!(5);
    v["expected_warmup_executions"] = json!(3);
    v["observed_warmup_executions"] = json!(rows.iter().filter(|r| r["warmup"] == true).count());
    v["all_expected_records_present"] = json!(rows.len() == 18);
    v["all_target_cold_AA_passed"] = json!(false);
    v["all_target_online_AA_passed"] = json!(false);
    v["target_controls"] = json!([]);
    v["plan_observations"] = json!([]);
    for k in ["integrity", "authenticity", "AA_rule", "resource_policy"] {
        v[k] = json!("software fixture only; no empirical observations or approval");
    }
    let mut outcomes = std::collections::BTreeMap::<String, usize>::new();
    for r in rows {
        *outcomes
            .entry(format!(
                "{}:{}",
                r["arm"].as_str().unwrap(),
                r["outcome"]["status"].as_str().unwrap_or("missing_status")
            ))
            .or_default() += 1;
    }
    v["outcome_counts"] = json!(outcomes);
    v["missing_executions"] = json!(plan["executions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| !rows.iter().any(|r| r["seq"] == e["seq"]))
        .map(|e| json!({"plan":"plan.json","session_id":"software-session","seq":e["seq"]}))
        .collect::<Vec<_>>());
}
