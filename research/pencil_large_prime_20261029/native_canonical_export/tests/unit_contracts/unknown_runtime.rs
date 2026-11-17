use super::*;
fn fixture() -> V {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/unknown-runtime-gate");
    let mut p = load(&dir.join("protocol.json")).unwrap();
    p["extensions"]["unknown_runtime_characterization_gate"] = json!({
        "schema":"pencil-bounded-unknown-runtime-gate/v1",
        "runtime_estimate_status":"unknown",
        "resource_caps_are_not_runtime_estimates":true,
        "remaining_cohort_blocked_until":["complete_gate","all_15_measured_independent_nonwall_replays","independent_review","separate_prospective_cohort_admission"],
        "resource_authorization":binding(&dir.join("resource_authorization.json")),
        "execution_request":binding(&dir.join("request.json")),
        "operational_review":binding(&dir.join("review.json")),
        "prepared_spec":binding(&dir.join("spec.json"))
    });
    for (key, pointer) in [
        (
            "resource_authorization",
            "/extensions/resource_authorization",
        ),
        (
            "execution_request",
            "/extensions/upper_gate/execution_request",
        ),
        (
            "operational_review",
            "/extensions/continuation_envelope/operational_guard/independent_review",
        ),
    ] {
        *p.pointer_mut(pointer).unwrap() =
            p["extensions"]["unknown_runtime_characterization_gate"][key].clone();
    }
    p["extensions"]["upper_gate"]["spec"] = json!(dir.join("spec.json"));
    p["extensions"]["known_data"]["replacement_context"]["terminal_state"] =
        binding(&dir.join("terminal-state.json"));
    p["extensions"]["upper_gate"]["admission_conditions"] = p["extensions"]
        ["unknown_runtime_characterization_gate"]["remaining_cohort_blocked_until"]
        .clone();
    p["freeze"]["status"] = json!("frozen");
    p["freeze"]["frozen_at"] = json!("2026-10-30T05:30:00Z");
    p["freeze"]["protocol_sha256"] = json!(protocol_hash(&p).unwrap());
    p
}
#[test]
fn explicit_bounded_unknown_gate_accepts_and_default_remains_rejected() {
    let mut p = fixture();
    validate_protocol(&p, None).unwrap();
    p["extensions"]
        .as_object_mut()
        .unwrap()
        .remove("unknown_runtime_characterization_gate");
    p["freeze"]["protocol_sha256"] = json!(protocol_hash(&p).unwrap());
    assert!(validate_protocol(&p, None)
        .unwrap_err()
        .contains("preflight"));
}
#[test]
fn unknown_gate_rejects_caps_time_primary_and_full_cohort() {
    let mutations = [
        (
            "/extensions/continuation_envelope/cumulative_active_worker_seconds",
            json!(0),
        ),
        ("/extensions/continuation_envelope/memory_bytes", V::Null),
        (
            "/extensions/continuation_envelope/coefficients_per_execution",
            V::Null,
        ),
        ("/execution/timeout_seconds", json!(0)),
        ("/metrics/primary/0/kind", json!("timed")),
        (
            "/extensions/analysis_scope/kind",
            json!("full1260-disjoint-p739/v1"),
        ),
        ("/sample_size/instances", json!(1260)),
        ("/extensions/timing_status/export_wall_valid", json!(true)),
        (
            "/extensions/parent_research_scope/later_cohort_execution_authorized",
            json!(true),
        ),
        (
            "/extensions/unknown_runtime_characterization_gate/remaining_cohort_blocked_until",
            json!([]),
        ),
        (
            "/extensions/continuation_envelope/absolute_deadline_utc",
            json!("2026-10-31T11:17:54Z"),
        ),
        (
            "/sample_size/preflight_runtime/estimated_total_seconds",
            json!(10800),
        ),
    ];
    for (pointer, value) in mutations {
        let mut p = fixture();
        *p.pointer_mut(pointer).unwrap() = value;
        assert!(
            protocol_lint::unknown_runtime_gate(&p).is_err(),
            "{pointer}"
        );
    }
}
#[test]
fn unknown_gate_rejects_evidence_digest_and_guard_mismatch() {
    let mut p = fixture();
    p["extensions"]["unknown_runtime_characterization_gate"]["operational_review"]["sha256"] =
        json!("0".repeat(64));
    assert!(protocol_lint::unknown_runtime_gate(&p).is_err());
    let d = tempfile::tempdir().unwrap();
    for field in [
        "operational_review",
        "execution_request",
        "resource_authorization",
    ] {
        let mut p = fixture();
        let mut v =
            load(&bound(&p["extensions"]["unknown_runtime_characterization_gate"][field]).unwrap())
                .unwrap();
        match field {
            "operational_review" => v["guard"]["G"] = json!(1793359074u64),
            "execution_request" => v["spec"]["measurement"]["seed"] = json!(9),
            _ => v["limits"]["coefficients_per_execution"] = json!(1),
        }
        let file = d.path().join(field);
        fs::write(&file, serde_json::to_vec(&v).unwrap()).unwrap();
        p["extensions"]["unknown_runtime_characterization_gate"][field] = binding(&file);
        let (pointer, expected) = match field {
            "operational_review" => (
                "/extensions/continuation_envelope/operational_guard/independent_review",
                "guard inequality",
            ),
            "execution_request" => (
                "/extensions/upper_gate/execution_request",
                "prepared spec mismatch",
            ),
            _ => (
                "/extensions/resource_authorization",
                "authorized cap mismatch",
            ),
        };
        *p.pointer_mut(pointer).unwrap() = binding(&file);
        assert!(
            protocol_lint::unknown_runtime_gate(&p)
                .unwrap_err()
                .contains(expected),
            "{field}"
        );
    }
}
#[test]
fn unknown_gate_rejects_reviewed_contract_mutations() {
    let mutations = [
        (
            "/extensions/resource_authorization/sha256",
            json!("0".repeat(64)),
        ),
        (
            "/extensions/upper_gate/execution_request/sha256",
            json!("0".repeat(64)),
        ),
        (
            "/extensions/continuation_envelope/operational_guard/independent_review/sha256",
            json!("0".repeat(64)),
        ),
        ("/extensions/upper_gate/spec", json!("shadow.json")),
        (
            "/extensions/finite_cost_model/mapping_sha256",
            json!("0".repeat(64)),
        ),
        (
            "/extensions/finite_cost_model/square_per_multiply",
            json!([0, 1, 2]),
        ),
        ("/arms/0/instrument/commit", json!("other")),
        ("/arms/1/instrument/image_digest", json!("other")),
        ("/arms/0/parameters/small_denominator", json!(4)),
        ("/arms/1/parameters/method", json!("other")),
        ("/extensions/replacement_authorization/count", json!(2)),
        (
            "/extensions/replacement_authorization/early_rejection_instrument_integrated",
            json!(true),
        ),
        ("/execution/retries/max_attempts", json!(3)),
        ("/extensions/scientific_admission", json!(true)),
        ("/extensions/launch_authorized", json!(true)),
        (
            "/extensions/parent_research_scope/upper_gate_rows_reused_in_cohort",
            json!(true),
        ),
        ("/extensions/upper_gate/admission_conditions", json!([])),
        ("/decision_rule/statistic", json!("timed winner")),
    ];
    for (pointer, value) in mutations {
        let mut p = fixture();
        *p.pointer_mut(pointer).unwrap() = value;
        assert!(
            protocol_lint::unknown_runtime_gate(&p).is_err(),
            "{pointer}"
        );
    }
}
#[test]
fn unknown_gate_rejects_erased_history_roles_and_promoted_authorization() {
    for (pointer, value) in [
        ("/arms/0/role", json!("reference")),
        ("/arms/2/same_instrument_as", V::Null),
        ("/arms/0/instrument/env/RAYON_NUM_THREADS", json!("2")),
        (
            "/extensions/replacement_authorization/prior_run_id",
            json!("other"),
        ),
        (
            "/extensions/known_data/replacement_context/records_enrolled",
            json!(true),
        ),
        (
            "/extensions/replacement_authorization/prior_measured_rows",
            json!(1),
        ),
    ] {
        let mut p = fixture();
        *p.pointer_mut(pointer).unwrap() = value;
        assert!(
            protocol_lint::unknown_runtime_gate(&p).is_err(),
            "{pointer}"
        );
    }
    let d = tempfile::tempdir().unwrap();
    for key in [
        "scientific_replacement_admission",
        "remaining_cohort_admitted",
        "timing_design_approved",
        "launch_ready",
    ] {
        let mut p = fixture();
        let mut auth = load(&bound(&p["extensions"]["resource_authorization"]).unwrap()).unwrap();
        auth[key] = json!(true);
        let file = d.path().join(key);
        fs::write(&file, serde_json::to_vec(&auth).unwrap()).unwrap();
        p["extensions"]["resource_authorization"] = binding(&file);
        p["extensions"]["unknown_runtime_characterization_gate"]["resource_authorization"] =
            binding(&file);
        assert!(protocol_lint::unknown_runtime_gate(&p)
            .unwrap_err()
            .contains("resource only"));
    }
}
