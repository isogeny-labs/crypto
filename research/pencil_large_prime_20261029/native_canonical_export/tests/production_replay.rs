//! Full denominator analysis-only fixture: no solver, timing or empirical run.
use pencil_native_canonical_export::*;
use serde_json::{json, Value as V};
use std::{fs, path::Path};
fn write(path: &Path, v: &V) {
    fs::write(path, serde_json::to_vec(v).unwrap()).unwrap()
}
fn binding(p: &Path) -> V {
    json!({"path":p,"sha256":digest(p).unwrap()})
}
#[test]
fn production_full_denominator_replay_and_mutation_rejection() {
    let d = tempfile::tempdir().unwrap();
    let dir = d.path();
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let study = here.parent().unwrap();
    let mut p = load(&here.join("fixtures/protocol-draft.json")).unwrap();
    p["protocol_id"] = json!("software-full-denominator-fixture");
    p["sample_size"]["preflight_runtime"]["estimated_total_seconds"] = json!(1);
    p["analysis_plan"]["frozen_before_data"] = json!(true);
    let mut sources = serde_json::Map::new();
    let mut pins = serde_json::Map::new();
    let mut build_sources = vec![];
    for name in [
        "native_price/Cargo.toml",
        "native_price/Cargo.lock",
        "native_price/src/lib.rs",
        "native_price/src/analyzer.rs",
        "native_price/src/aggregate.rs",
        "native_price/src/importer.rs",
        "native_price/src/bin/analyze_pair.rs",
        "native_price/src/bin/analyze_cohort.rs",
        "paired_export/Cargo.toml",
        "paired_export/Cargo.lock",
        "paired_export/src/lib.rs",
        "paired_export/src/plan.rs",
        "paired_export/src/main.rs",
    ] {
        let b = binding(&study.join(name));
        pins.insert(name.to_owned(), b["sha256"].clone());
        sources.insert(name.to_owned(), b.clone());
        build_sources.push(json!({"path":name,"sha256":b["sha256"]}));
    }
    p["extensions"]["analysis_support_sha256"] = json!(pins);
    let mapping = pins["native_price/src/analyzer.rs"].clone();
    p["extensions"]["finite_cost_model"]["mapping_sha256"] = mapping.clone();
    p["analysis_plan"]["script"] = sources["native_price/src/bin/analyze_cohort.rs"].clone();
    // These bytes bind a software receipt fixture, not execution of an analyzer binary.
    let bf = dir.join("software-binary-identity-fixture");
    fs::write(&bf, "software fixture identity only").unwrap();
    let mut b = binding(&bf);
    b["bytes"] = json!(fs::metadata(&bf).unwrap().len());
    let bins = json!({"analyze_pair":b,"analyze_cohort":b});
    let br = json!({"schema":"pencil-native-analysis-build/v1","source_revision":"1234567890123456789012345678901234567890","source_files_match_commit":true,"build_exit_code":0,"mapping_sha256":mapping,"sources":build_sources,"binaries":[{"name":"analyze_pair","path":bf,"sha256":b["sha256"],"bytes":b["bytes"]},{"name":"analyze_cohort","path":bf,"sha256":b["sha256"],"bytes":b["bytes"]}],"scope":"synthetic build receipt; no analyzer binary execution"});
    let brp = dir.join("software-build-receipt.json");
    write(&brp, &br);
    p["extensions"]["analysis_build"] = json!({"source_commit":br["source_revision"],"receipt_sha256":digest(&brp).unwrap(),"binaries":bins});
    p["extensions"]["finite_cost_model"]["analyzer_binaries_sha256"] =
        json!({"analyze_pair":b["sha256"],"analyze_cohort":b["sha256"]});
    let helper = here.join("tools/experiment_protocol.reference.txt");
    p["extensions"]["canonical_packaging"] =
        json!({"adapter_sha256":adapter_hash(),"canonical_tool_sha256":digest(&helper).unwrap()});
    p["freeze"]["status"] = json!("frozen");
    p["freeze"]["frozen_at"] = json!("2026-10-29T00:00:00Z");
    p["freeze"]["protocol_sha256"] = json!(protocol_hash(&p).unwrap());
    let pp = dir.join("protocol.json");
    write(&pp, &p);
    let arms = json!([{"name":"pencil","role":"candidate","method":{"id":"ic.three_leads_genus2_pencil_symbolic_v2","params":{}}},{"name":"rho","role":"reference","method":{"id":"rho.three_leads_negation_v1","params":{}}},{"name":"rho_AA","role":"control","method":{"id":"rho.three_leads_negation_v1","params":{}}}]);
    let workloads: Vec<_> = (0..1260)
        .map(|i| json!({"workload_id":format!("software-W-{i}"),"curve_spec":{"p":739,"r":34147}}))
        .collect();
    let executions:Vec<_>=(0..1260).flat_map(|wi|(0..6).flat_map(move|round|(0..3).map(move|arm|json!({"seq":wi*18+round*3+arm,"round":round,"warmup":round==0,"arm":arm,"workload":wi,"algorithm_seed":round+10})))).collect();
    let plan = json!({"schema":"ecbench.plan/v1","spec_id":"software-plan-not-executed","arms":arms,"workloads":workloads,"executions":executions});
    let plp = dir.join("plan.json");
    write(&plp, &plan);
    let receipt = json!({"schema":"pencil-prepared-plan-validation/v1","passed":true,"targets":1260,"planned_executions":22680,"measured_executions":18900,"warmup_executions":3780,"solver_executions_performed":0,"binary_sha256":p["arms"][0]["instrument"]["binary_sha256"],"files":{"plan.json":digest(&plp).unwrap()}});
    let rp = dir.join("receipt.json");
    write(&rp, &receipt);
    let manifest = json!({"schema":"pencil-price-cohort-input/v1","prepared_plan_validation":binding(&rp),"binary_sha256":p["arms"][0]["instrument"]["binary_sha256"],"protocol_sha256":p["freeze"]["protocol_sha256"],"coverage_review":null,"plans":[{"name":"plan.json","path":plp,"sha256":digest(&plp).unwrap(),"session_id":"software-session-not-executed","records":null}]});
    let native = pencil_accounting_price_draft::aggregate::aggregate(&manifest).unwrap();
    let np = dir.join("native.json");
    write(&np, &native);
    let hp = dir.join("history.json");
    write(
        &hp,
        &json!({"schema":"pencil-execution-history/v1","protocol_sha256":manifest["protocol_sha256"],"binary_sha256":manifest["binary_sha256"],"records":[]}),
    );
    let core = json!({"schema":"pencil-canonical-package-input/v1","canonical_tool":binding(&helper),"protocol":binding(&pp),"native_analysis":binding(&np),"analysis_script":p["analysis_plan"]["script"],"execution_history":binding(&hp),"export_id":"software-full-denominator-export","analysis_provenance":{"sources":sources,"binaries":bins,"build_receipt":binding(&brp)}});
    let request_path = dir.join("request.json");
    write(&request_path, &core);
    let mut request = core.clone();
    request["_request_binding"] = binding(&request_path);
    let (export, findings) = package(&request).unwrap();
    assert!(!findings.is_empty());
    assert_eq!(export["runs"], json!([]));
    assert_eq!(
        export["backend_record"]["missing_slots"]
            .as_array()
            .unwrap()
            .len(),
        22680
    );
    assert!(check_export(&export, &p).is_ok());
    let mut changed = native.clone();
    changed["all_target_cold_AA_passed"] = json!(true);
    changed["target_controls"] = json!([]);
    write(&np, &changed);
    let mut changed_core = core.clone();
    changed_core["native_analysis"] = binding(&np);
    write(&request_path, &changed_core);
    changed_core["_request_binding"] = binding(&request_path);
    let err = package(&changed_core).unwrap_err();
    assert!(err.contains("deterministic source-linked replay"), "{err}");
    // Structurally valid partial aggregate tampering must fail deterministic replay.
    let mut changed = native;
    changed["aggregate_cells"][0]["partial_admitted_sums_diagnostic_only"]["candidate"] =
        json!({"numerator":"7","denominator":"1"});
    write(&np, &changed);
    let mut changed_core = core;
    changed_core["native_analysis"] = binding(&np);
    write(&request_path, &changed_core);
    changed_core["_request_binding"] = binding(&request_path);
    let err = package(&changed_core).unwrap_err();
    assert!(err.contains("deterministic source-linked replay"), "{err}");
}
