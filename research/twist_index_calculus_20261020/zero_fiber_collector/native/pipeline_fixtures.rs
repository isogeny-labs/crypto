//! Compact test-reference projection from immutable completed runs. No solves.
use super::*;

pub(super) fn prepare(zero_cases: &Path, nine: &Path, double: &Path, out: &Path) {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let public: Value =
        serde_json::from_str(include_str!("../fixtures/p37-seed0-public.json")).unwrap();
    let public_sha = hex::encode(hash::sha256::sha256(include_bytes!(
        "../fixtures/p37-seed0-public.json"
    )));
    assert_eq!(
        public_sha,
        "5a55715c3d8f532634722ff6f58562e8f7e9f7d339f529cda724677da0f9b393"
    );
    let mut sources = Vec::new();
    for arm in [7, 9] {
        let name = ARMS[arm];
        let root = zero_cases.join("p37-seed0").join(name);
        let zero_bytes = std::fs::read(root.join("first/record.json")).unwrap();
        let zero_sha = hex::encode(hash::sha256::sha256(&zero_bytes));
        let zero: Value = serde_json::from_slice(&zero_bytes).unwrap();
        let audit_bytes = std::fs::read(root.join("audit/summary.json")).unwrap();
        let audit: Value = serde_json::from_slice(&audit_bytes).unwrap();
        assert_eq!(audit["status"], "pass");
        assert_eq!(audit["candidate_sha256"], zero_sha);
        assert_eq!(zero["public_input_sha256"], public_sha);
        assert_eq!(zero["status"], "complete-polynomial-enumeration");
        assert_eq!(zero["p"], 37);
        assert_eq!(zero["seed"], 0);
        assert_eq!(zero["arm"], name);
        let systems: Vec<_> = zero["systems"].as_array().unwrap().iter().map(|r| {
            json!({"pair":r["pair"],"accepted_representative_relations":r["accepted_representative_relations"],
                "field_roots":r["field_roots"],"f4_row_reduction_fp_muls":r["f4_row_reduction_fp_muls"],
                "max_rows":r["max_rows"],"max_cols":r["max_cols"]})
        }).collect();
        assert_eq!(
            systems.len() as u64,
            zero["systems_attempted"].as_u64().unwrap()
        );
        let fixture = json!({"schema":"twist-zero-pipeline-zero-test-fixture/v1",
            "source_complete_record_sha256":zero_sha,"status":zero["status"],"p":37,"seed":0,
            "arm":name,"public_input_sha256":public_sha,"systems":systems});
        let target_path = if arm == 9 { double } else { nine }
            .join("p37-seed0")
            .join(format!("{name}.json"));
        let target_bytes = std::fs::read(target_path).unwrap();
        let target: Value = serde_json::from_slice(&target_bytes).unwrap();
        assert_eq!(target["p"], 37);
        assert_eq!(target["seed"], 0);
        assert_eq!(target["arm"], name);
        assert_eq!(target["censored"], false);
        assert_eq!(target["all_target_and_factor_logs_verified"], true);
        let trace = target["trace"].as_array().unwrap();
        assert_eq!(trace.len(), public["query_trace"].as_array().unwrap().len());
        for (i, q) in trace.iter().enumerate() {
            assert_eq!(q["query_index"], i);
            assert_eq!(q["a"], public["query_trace"][i][0]);
            assert_eq!(q["b"], public["query_trace"][i][1]);
        }
        let folder = out.join(name);
        std::fs::create_dir(&folder).unwrap();
        write(&folder.join("zero-reference.json"), &fixture);
        std::fs::write(folder.join("target-reference.json"), &target_bytes).unwrap();
        sources.push(json!({"arm":name,"zero_source_record_sha256":zero_sha,
            "zero_source_audit_sha256":hex::encode(hash::sha256::sha256(&audit_bytes)),
            "zero_fixture_sha256":hex::encode(hash::sha256::sha256(&std::fs::read(folder.join("zero-reference.json")).unwrap())),
            "target_reference_sha256":hex::encode(hash::sha256::sha256(&target_bytes))}));
    }
    write(
        &out.join("provenance.json"),
        &json!({"schema":"twist-zero-pipeline-test-fixture-provenance/v1",
        "public_input_sha256":public_sha,"case":"p37-seed0","sources":sources,
        "zero_source_store":"store:evidence/twist-index-calculus/20261024-zero-fiber-runs.tar.xz",
        "target_source_store":"store:evidence/twist-index-calculus/20261023-image-base-runs.tar.xz",
        "scope":"Native projection of already independently audited complete zero records and byte-identical copies of the complete target references. No F4, group census, factor-log recovery or BSGS is run."}),
    );
}
