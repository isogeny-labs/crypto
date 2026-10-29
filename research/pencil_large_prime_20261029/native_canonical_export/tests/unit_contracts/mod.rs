//! Public software fixtures only; no experimental cohort or approval.
use crate::*;
use serde_json::{json, Value as V};
use std::{
    fs,
    path::{Path, PathBuf},
};
mod support;
struct Fixture {
    d: tempfile::TempDir,
    p: V,
    rows: Vec<V>,
    req: V,
    provenance: V,
}
fn binding(p: &Path) -> V {
    json!({"path":p,"sha256":digest(p).unwrap()})
}
impl Fixture {
    fn path(&self, name: &str) -> PathBuf {
        self.d.path().join(name)
    }
    fn write(&self, name: &str, v: &V) {
        fs::write(self.path(name), serde_json::to_vec(v).unwrap()).unwrap()
    }
    fn new() -> Self {
        let d = tempfile::tempdir().unwrap();
        let study = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut p = load(&study.join("fixtures/protocol-draft.json")).unwrap();
        let helper = &study.join("tools/experiment_protocol.reference.txt");
        p["analysis_plan"]["script"] =
            binding(&study.join("fixtures/analyze_cohort.reference.txt"));
        p["protocol_id"] = json!("software-fixture-only");
        p["extensions"]["canonical_packaging"] = json!({"adapter_sha256":adapter_hash(),"canonical_tool_sha256":digest(helper).unwrap()});
        p["sample_size"]["instances"] = json!(1);
        p["sample_size"]["preflight_runtime"]["estimated_total_seconds"] = json!(1);
        p["analysis_plan"]["frozen_before_data"] = json!(true);
        let provenance = support::provenance(&mut p, d.path(), study);
        p["freeze"]["status"] = json!("frozen");
        p["freeze"]["frozen_at"] = json!("2026-10-29T00:00:00Z");
        p["freeze"]["protocol_sha256"] = json!(protocol_hash(&p).unwrap());
        let arms: Vec<_> = p["arms"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| json!({"name":a["id"],"method":{"id":a["parameters"]["method"]}}))
            .collect();
        let ex:Vec<_>=(0..18).map(|i|json!({"seq":i,"arm":i%3,"workload":0,"round":i/3,"warmup":i<3,"algorithm_seed":100+i/3})).collect();
        let w = json!({"workload_id":"software-W","curve_spec":{"p":739}});
        let plan = json!({"schema":"ecbench.plan/v1","arms":arms,"workloads":[w],"executions":ex});
        let rows=ex.iter().map(|x|{let arm=&arms[x["arm"].as_u64().unwrap()as usize];json!({"schema":"ecbench.record/v1","record_id":format!("software-{}",x["seq"]),"session_id":"software-session","binary_sha256":p["arms"][0]["instrument"]["binary_sha256"],"seq":x["seq"],"round":x["round"],"warmup":x["warmup"],"arm":arm["name"],"method":arm["method"],"algorithm_seed":x["algorithm_seed"].to_string(),"workload":w,"outcome":{"status":"verified","matches_target":true,"matches_planted":true,"error":null},"isolation":{"level":"L2"},"time":{"method_call_wall_ns":120,"solve_wall_ns":2,"max_rss_kib":null,"parent_audit_verification_ns":3,"online_clock_diagnostics":{"error":null}},"online":{"wall_ns":5,"phases_ns":{"target_query":1,"target_PDP":1,"target_relation_check":1,"target_descent":1,"target_recovery_check":1}},"counters":{"actual":4}})}).collect();
        let mut f = Self {
            d,
            p,
            rows,
            req: V::Null,
            provenance,
        };
        f.write("plan.json", &plan);
        f.write("receipt.json",&json!({"schema":"pencil-prepared-plan-validation/v1","passed":true,"targets":1,"planned_executions":18,"measured_executions":15,"warmup_executions":3,"files":{"plan.json":digest(&f.path("plan.json")).unwrap()}}));
        f.write("resources.json",&json!({"schema":"software-resource-fixture/v1","attempts":[{"status":"interrupted","restored":true}],"not_cohort":true}));
        f.refresh();
        f
    }
    fn refresh(&mut self) {
        self.write("protocol.json", &self.p);
        fs::write(
            self.path("records.jsonl"),
            self.rows
                .iter()
                .map(|r| format!("{r}\n"))
                .collect::<String>(),
        )
        .unwrap();
        let m = json!({"schema":"pencil-price-cohort-input/v1","protocol_sha256":self.p["freeze"]["protocol_sha256"],"binary_sha256":self.p["arms"][0]["instrument"]["binary_sha256"],"prepared_plan_validation":binding(&self.path("receipt.json")),"plans":[{"path":self.path("plan.json"),"sha256":digest(&self.path("plan.json")).unwrap(),"name":"plan.json","session_id":"software-session","records":binding(&self.path("records.jsonl"))}],"separate_resources":binding(&self.path("resources.json"))});
        let pins = &self.p["extensions"]["analysis_support_sha256"];
        self.write("native.json",&json!({"schema":"pencil-price-cohort-output/v1","input_manifest":m,"input_manifest_integrity_sha256":hash(&canonical(&m).unwrap()),"expected_executions":18,"observed_executions":self.rows.len(),"missing_executions":vec![json!({});18-self.rows.len()],"separate_resource_observations":load(&self.path("resources.json")).unwrap(),"aggregate_source_sha256":pins["native_price/src/aggregate.rs"],"paired_export_source_sha256":pins["paired_export/src/lib.rs"],"paired_export_plan_source_sha256":pins["paired_export/src/plan.rs"],"mapping":{"mapping_sha256":self.p["extensions"]["finite_cost_model"]["mapping_sha256"]}}));
        let mut native = load(&self.path("native.json")).unwrap();
        support::complete_native(
            &mut native,
            &load(&self.path("plan.json")).unwrap(),
            &self.rows,
        );
        self.write("native.json", &native);
        self.write("history.json",&json!({"schema":"pencil-execution-history/v1","protocol_sha256":self.p["freeze"]["protocol_sha256"],"binary_sha256":m["binary_sha256"],"records":self.rows.iter().map(|r|json!({"record_id":r["record_id"],"session_id":r["session_id"],"seq":r["seq"],"attempts":1,"resumed":0,"checkpoint":null,"evidence":[binding(&self.path("resources.json"))]})).collect::<Vec<_>>()}));
        let study = Path::new(env!("CARGO_MANIFEST_DIR"));
        self.req = json!({"schema":"pencil-canonical-package-input/v1","canonical_tool":binding(&study.join("tools/experiment_protocol.reference.txt")),"protocol":binding(&self.path("protocol.json")),"native_analysis":binding(&self.path("native.json")),"analysis_script":self.p["analysis_plan"]["script"],"analysis_provenance":self.provenance,"execution_history":binding(&self.path("history.json")),"export_id":"software-fixture-export","created":"2026-10-29T00:00:00Z"});
    }
    fn run(&self) -> Result<(V, Vec<String>)> {
        self.write("packaging-request.json", &self.req);
        let mut req = self.req.clone();
        req["_request_binding"] = binding(&self.path("packaging-request.json"));
        package(&req)
    }
    fn edit(&mut self, name: &str, field: &str, change: impl FnOnce(&mut V)) {
        let mut v = load(&self.path(name)).unwrap();
        change(&mut v);
        self.write(name, &v);
        self.req[field] = binding(&self.path(name));
    }
}
#[test]
fn complete_schema() {
    let f = Fixture::new();
    let (e, findings) = f.run().unwrap();
    assert!(findings.is_empty(), "{findings:?}");
    validate_schema(&e, false).unwrap();
    assert_eq!(e["runs"].as_array().unwrap().len(), 18);
    assert_eq!(
        e["runs"][0]["measurements"]["wall_seconds"].as_f64(),
        Some(120e-9)
    );
    assert!(e["runs"][0]["measurements"]["max_rss_bytes"].is_null());
    assert!(e["runs"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["measurements"]["wall_valid"] == false));
}
#[test]
fn missing_no_fake_attempt() {
    let mut f = Fixture::new();
    f.rows.pop();
    f.refresh();
    let (e, x) = f.run().unwrap();
    assert_eq!(e["runs"].as_array().unwrap().len(), 17);
    assert_eq!(
        e["backend_record"]["missing_slots"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(!x.is_empty());
}
#[test]
fn wrong_phases() {
    let mut f = Fixture::new();
    f.rows[4]["online"]["phases_ns"] = json!({"arbitrary":5});
    f.refresh();
    let (e, _) = f.run().unwrap();
    assert!(
        e["backend_record"]["diagnostics"][4]["online_structure_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("phase_names_not_exact_five"))
    );
}
#[test]
fn phase_sum_diagnostics() {
    let mut f = Fixture::new();
    f.rows[4]["online"]["wall_ns"] = json!(6);
    f.rows[4]["time"]
        .as_object_mut()
        .unwrap()
        .remove("online_clock_diagnostics");
    f.refresh();
    let (e, _) = f.run().unwrap();
    let a = e["backend_record"]["diagnostics"][4]["online_structure_reasons"]
        .as_array()
        .unwrap();
    assert!(
        a.contains(&json!("invalid_phase_sum"))
            && a.contains(&json!("online_diagnostics_absent_or_error"))
    );
}
#[test]
fn failed_resources() {
    let mut f = Fixture::new();
    f.rows[4]["outcome"] = json!({"status":"timeout"});
    f.refresh();
    let (e, _) = f.run().unwrap();
    assert_eq!(e["runs"][4]["status"], "timeout");
    assert_eq!(e["backend_record"]["resource_observations_present"], true);
}
#[test]
fn seed_identity() {
    let mut f = Fixture::new();
    f.rows[4]["algorithm_seed"] = json!("999");
    f.refresh();
    assert!(f.run().unwrap_err().contains("exact plan identity"));
}
#[test]
fn binary_identity() {
    let mut f = Fixture::new();
    f.rows[4]["binary_sha256"] = json!("0".repeat(64));
    f.refresh();
    assert!(f.run().unwrap_err().contains("record identity"));
}
#[test]
fn raw_hash() {
    let f = Fixture::new();
    fs::write(f.path("records.jsonl"), "changed").unwrap();
    assert!(f.run().unwrap_err().contains("hash mismatch"));
}
#[test]
fn changed_attachment() {
    let f = Fixture::new();
    let (e, _) = f.run().unwrap();
    fs::write(f.path("resources.json"), "{}").unwrap();
    assert!(check_files(&e).unwrap_err().contains("file changed"));
}
#[test]
fn duplicate_record() {
    let mut f = Fixture::new();
    f.rows[17] = f.rows[0].clone();
    f.refresh();
    assert!(f.run().unwrap_err().contains("duplicate"));
}
#[test]
fn draft_rejected() {
    let mut f = Fixture::new();
    f.p["freeze"]["status"] = json!("draft");
    f.refresh();
    assert!(f.run().unwrap_err().contains("frozen protocol"));
}
#[test]
fn manifest_integrity() {
    let mut f = Fixture::new();
    f.edit("native.json", "native_analysis", |v| {
        v["input_manifest_integrity_sha256"] = json!("0".repeat(64))
    });
    assert!(f.run().unwrap_err().contains("manifest integrity"));
}
#[test]
fn wholly_missing() {
    let mut f = Fixture::new();
    f.rows.clear();
    f.refresh();
    let (e, x) = f.run().unwrap();
    assert_eq!(e["runs"], json!([]));
    assert_eq!(
        e["backend_record"]["missing_slots"]
            .as_array()
            .unwrap()
            .len(),
        18
    );
    assert!(!x.is_empty());
}
#[test]
fn unknown_history() {
    let mut f = Fixture::new();
    f.edit("history.json", "execution_history", |h| {
        h["records"][0]["resumed"] = V::Null
    });
    assert!(f.run().unwrap_err().contains("explicit known"));
}
#[test]
fn missing_history() {
    let mut f = Fixture::new();
    f.edit("history.json", "execution_history", |h| {
        h["records"].as_array_mut().unwrap().pop();
    });
    assert!(f.run().unwrap_err().contains("missing or mismatched"));
}
#[test]
fn restore_history() {
    let mut f = Fixture::new();
    permit_computation_restore(&mut f);
    f.edit("history.json", "execution_history", |h| {
        h["records"][4]["attempts"] = json!(2);
        h["records"][4]["resumed"] = json!(1);
        h["records"][4]["checkpoint"] =
            json!({"policy":"criu","generations_written":2,"restored_from_generation":1});
    });
    let (e, x) = f.run().unwrap();
    assert!(x.is_empty());
    assert_eq!(e["runs"][4]["attempts"], 2);
    assert_eq!(e["runs"][4]["resumed"], 1);
}
#[test]
fn mapping_mismatch() {
    let mut f = Fixture::new();
    f.edit("native.json", "native_analysis", |v| {
        v["mapping"]["mapping_sha256"] = json!("0".repeat(64))
    });
    assert!(f.run().unwrap_err().contains("mapping identity"));
}
#[test]
fn actual_schema_negative() {
    let f = Fixture::new();
    let (mut e, _) = f.run().unwrap();
    e["runs"][0]["attempts"] = json!(0);
    assert!(validate_schema(&e, false).is_err());
    e["runs"][0]["attempts"] = json!(1);
    e["unknown"] = json!(1);
    assert!(validate_schema(&e, false).is_err());
}
#[test]
fn exclusive_output() {
    let f = Fixture::new();
    let (e, _) = f.run().unwrap();
    write_exclusive(&f.path("export.json"), &e).unwrap();
    assert!(write_exclusive(&f.path("export.json"), &e).is_err());
}
#[test]
fn exact_large_integer() {
    let mut f = Fixture::new();
    f.rows[0]["counters"]["large"] = json!(u64::MAX);
    f.refresh();
    let (e, _) = f.run().unwrap();
    assert_eq!(
        e["runs"][0]["measurements"]["counters"]["native.large"],
        u64::MAX.to_string()
    );
}
#[test]
fn semantic_check_rejects_changed_flow() {
    let f = Fixture::new();
    let (mut e, _) = f.run().unwrap();
    e["flow"]["attempted"] = json!(0);
    assert!(check_export(&e, &f.p).unwrap_err().contains("flow"));
}
#[test]
fn semantic_check_rejects_wall_qualification() {
    let f = Fixture::new();
    let (mut e, _) = f.run().unwrap();
    e["runs"][4]["measurements"]["wall_valid"] = json!(true);
    assert!(check_export(&e, &f.p)
        .unwrap_err()
        .contains("never qualifies"));
}
#[test]
fn exact_u128_counter() {
    let mut f = Fixture::new();
    f.rows[0]["counters"]["wide"] =
        serde_json::from_str("340282366920938463463374607431768211455").unwrap();
    f.refresh();
    let (e, _) = f.run().unwrap();
    assert_eq!(
        e["runs"][0]["measurements"]["counters"]["native.wide"],
        "340282366920938463463374607431768211455"
    );
}
#[test]
fn protocol_semantic_unknown_comparison() {
    let mut f = Fixture::new();
    f.p["claims"][1]["decided_by"] = json!(["absent"]);
    f.p["freeze"]["protocol_sha256"] = json!(protocol_hash(&f.p).unwrap());
    f.refresh();
    assert!(f.run().unwrap_err().contains("unknown comparison"));
}
#[test]
fn no_history_evidence() {
    let mut f = Fixture::new();
    f.edit("history.json", "execution_history", |h| {
        h["records"][0]["evidence"] = json!([])
    });
    assert!(f.run().unwrap_err().contains("bound evidence"));
}
#[test]
fn restore_generation_required() {
    let mut f = Fixture::new();
    f.edit("history.json", "execution_history", |h| {
        h["records"][0]["resumed"] = json!(1)
    });
    assert!(f.run().unwrap_err().contains("checkpoint policy"));
}
// Private unit scope only: output replay is separately exercised through the
// unconditional public production API on the complete 1,260-workload plan.
fn package(req: &V) -> Result<(V, Vec<String>)> {
    crate::package_with(req, |_, _| Ok(()))
}
fn check_export(e: &V, p: &V) -> Result<Vec<String>> {
    crate::check_export_with(e, p, |_, _| Ok(()))
}
fn permit_computation_restore(f: &mut Fixture) {
    f.p["execution"]["measurement_class"] = json!("search");
    f.p["extensions"]["active_execution_epoch"] = json!("computation");
    f.p["execution"]["checkpoint"] = json!({"policy":"criu","interval_seconds":300,"grace_seconds":120,"restore_validation":"criu_selfcheck","after_restore":{"wall_time":"invalid","counted_units":"valid"}});
    f.p["extensions"]["computation_epoch"]["max_restores"] = json!(1);
    f.p["freeze"]["protocol_sha256"] = json!(protocol_hash(&f.p).unwrap());
    f.refresh();
}
#[test]
fn missing_planted_stays_failed() {
    let mut f = Fixture::new();
    f.rows[0]["outcome"]
        .as_object_mut()
        .unwrap()
        .remove("matches_planted");
    f.refresh();
    let (e, _) = f.run().unwrap();
    assert_eq!(e["runs"][0]["status"], "failed");
    assert_eq!(e["runs"][0]["verification"]["status"], "no_claim");
}
#[test]
fn malformed_counters_rejected() {
    for v in [
        json!("not-an-integer"),
        json!("-1"),
        json!(""),
        json!(1.5),
        json!(true),
    ] {
        let mut f = Fixture::new();
        f.rows[0]["counters"]["actual"] = v;
        f.refresh();
        assert!(f.run().unwrap_err().contains("nonnegative integer"));
    }
    let mut f = Fixture::new();
    f.rows[0]["counters"] = V::Null;
    f.refresh();
    assert!(f.run().unwrap_err().contains("counters object"));
}
#[test]
fn omitted_planned_warmups_rejected() {
    let mut f = Fixture::new();
    let mut plan = load(&f.path("plan.json")).unwrap();
    plan["executions"]
        .as_array_mut()
        .unwrap()
        .retain(|e| e["warmup"] == false);
    f.write("plan.json", &plan);
    let mut receipt = load(&f.path("receipt.json")).unwrap();
    receipt["files"]["plan.json"] = json!(digest(&f.path("plan.json")).unwrap());
    f.write("receipt.json", &receipt);
    f.rows.retain(|r| r["warmup"] == false);
    f.refresh();
    assert!(f.run().unwrap_err().contains("prescribed measured/warmup"));
}
#[test]
fn missing_warmup_records_preserved_as_shortfall() {
    let mut f = Fixture::new();
    f.rows.retain(|r| r["warmup"] == false);
    f.refresh();
    let (e, findings) = f.run().unwrap();
    assert_eq!(
        e["backend_record"]["missing_slots"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert!(findings.iter().any(|s| s.contains("warmup")));
}
#[test]
fn retry_and_generation_bounds() {
    let mut f = Fixture::new();
    f.edit("history.json", "execution_history", |h| {
        h["records"][0]["attempts"] = json!(3)
    });
    assert!(f.run().unwrap_err().contains("retry admission"));
    let mut f = Fixture::new();
    permit_computation_restore(&mut f);
    f.edit("history.json", "execution_history", |h| {
        h["records"][0]["resumed"] = json!(1);
        h["records"][0]["checkpoint"] =
            json!({"policy":"criu","generations_written":0,"restored_from_generation":99});
    });
    assert!(f.run().unwrap_err().contains("committed generations"));
}
#[test]
fn none_epoch_cannot_use_computation_restore() {
    let mut f = Fixture::new();
    f.edit("history.json", "execution_history", |h| {
        h["records"][0]["resumed"] = json!(1);
        h["records"][0]["checkpoint"] =
            json!({"policy":"criu","generations_written":1,"restored_from_generation":1});
    });
    assert!(f
        .run()
        .unwrap_err()
        .contains("measurement checkpoint policy"));
}
#[test]
fn backend_mutations_individually_rejected() {
    let f = Fixture::new();
    let (e, _) = f.run().unwrap();
    for (k, v) in [
        ("qualified", json!(true)),
        ("expected_execution_slots", json!(0)),
        ("missing_slots", json!([{"fabricated":true}])),
        ("diagnostics", json!([])),
        ("native_analysis_sha256", json!("0".repeat(64))),
    ] {
        let mut changed = e.clone();
        changed["backend_record"][k] = v;
        assert!(
            check_export(&changed, &f.p)
                .unwrap_err()
                .contains("reconstruction"),
            "{k}"
        );
    }
}
#[test]
fn native_required_inventory_cannot_be_omitted() {
    for field in [
        "aggregate_cells",
        "streams",
        "pair_rows",
        "outcome_counts",
        "qualified_ratios",
    ] {
        let mut f = Fixture::new();
        f.edit("native.json", "native_analysis", |v| {
            v.as_object_mut().unwrap().remove(field);
        });
        assert!(f.run().is_err(), "{field}");
    }
}
#[test]
fn source_provenance_bytes_required() {
    let f = Fixture::new();
    let path = f.provenance["sources"]["native_price/src/importer.rs"]["path"]
        .as_str()
        .unwrap();
    fs::write(path, "mutated").unwrap();
    assert!(f.run().unwrap_err().contains("hash mismatch"));
}
#[test]
fn altered_analyzer_binary_rejected() {
    let f = Fixture::new();
    fs::write(
        f.provenance["binaries"]["analyze_cohort"]["path"]
            .as_str()
            .unwrap(),
        "mutated",
    )
    .unwrap();
    assert!(f.run().unwrap_err().contains("hash mismatch"));
}
#[test]
fn no_restore_with_generation_rejected() {
    let mut f = Fixture::new();
    f.edit("history.json", "execution_history", |h| {
        h["records"][0]["checkpoint"] =
            json!({"policy":"none","generations_written":1,"restored_from_generation":1})
    });
    assert!(f.run().unwrap_err().contains("zero restores"));
}
