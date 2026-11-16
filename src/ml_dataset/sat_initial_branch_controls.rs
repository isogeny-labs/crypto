//! Source-only genuine retained-branch controls; register under solvers cfg(test).
use super::*;
use crate::ml_dataset::{self as dataset, capture, storage, telemetry};
fn policy() -> SatReplayPolicy {
    SatReplayPolicy {
        work: 1_000_000_000_000,
        bytes: 8_000_000_000,
        deadline: Instant::now() + Duration::from_secs(120),
        stream_bytes: 16 * 1024 * 1024,
        payload_bytes: 16 * 1024 * 1024,
    }
}
fn contexts(root: &Path, artifacts: &mut Vec<dataset::types::Artifact>) {
    for (path, role) in dataset::CAPTURE_CONTEXT
        .into_iter()
        .chain([(dataset::execution::PATH, dataset::execution::ROLE)])
    {
        let (sha256, bytes) = crate::store::hash_file(&root.join(path)).unwrap();
        artifacts.push(dataset::types::Artifact {
            path: path.into(),
            role: role.into(),
            encoding: "json".into(),
            sha256,
            bytes,
        });
    }
}
#[test]
fn actual_initial_without_final_checkpoint_is_incomplete_and_rejects_success_claims() {
    let f = verified_worker_original_tests::Fixture::with_sat_initial_shape(23, false);
    dataset::execution::create(&f.root).unwrap();
    let mut artifacts = Vec::new();
    let case = telemetry::measured(&f.root, &mut artifacts, "curve-construction", || {
        dataset::curves::generate(&f.config)
    })
    .unwrap();
    let token = verify_and_store_worker_originals(
        &f.config,
        &case,
        &f.root,
        Some(&f.binding),
        &mut artifacts,
    )
    .unwrap();
    let context = dataset::execution::admit(&f.root).unwrap().unwrap();
    let (_, caps) = dataset::admit_capture_context(&f.root).unwrap().unwrap();
    let output = solve_with_verified_worker_originals(
        &f.config,
        &case,
        &f.root.join("live"),
        f.binding.clone(),
        caps,
        Some(&context),
        &token,
    )
    .unwrap();
    let (data, stored, residual) = output.into_parts();
    assert!(stored.is_empty());
    assert!(residual.sat.is_some());
    assert_eq!(
        data.outcome["initial_checked"], true,
        "required actual prefix positive"
    );
    artifacts.push(
        storage::write_json(&f.root, "result.json", "solver-outcome", &data.outcome).unwrap(),
    );
    contexts(&f.root, &mut artifacts);
    let events = fs::read(f.root.join("live/events.jsonl")).unwrap();
    let mut lines: Vec<&[u8]> = events.split_inclusive(|b| *b == b'\n').collect();
    let last = lines.pop().expect("actual producer checkpoint");
    let last_value: Value = serde_json::from_slice(last).unwrap();
    assert_eq!(
        last_value["event"]["payload"]["schema"],
        "ml-dataset.sat-owner-ledger/v1"
    );
    let previous = lines.last().unwrap();
    let previous_value: Value = serde_json::from_slice(previous).unwrap();
    let prefix_len = events.len() - last.len();
    let mut head: capture::CommittedHead =
        serde_json::from_slice(&fs::read(f.root.join("live/committed.json")).unwrap()).unwrap();
    assert_eq!(head.event_count, last_value["sequence"].as_u64().unwrap());
    head.event_count = previous_value["sequence"].as_u64().unwrap();
    head.byte_end = prefix_len as u64;
    head.last_event_sha256 = dataset::digest(previous);
    // Only the acknowledged final ledger is removed. Actual map/native marker
    // bytes and chain up to that marker remain untouched and authenticated.
    fs::write(f.root.join("live/events.jsonl"), &events[..prefix_len]).unwrap();
    fs::write(
        f.root.join("live/committed.json"),
        serde_json::to_vec(&head).unwrap(),
    )
    .unwrap();
    let partial = json!({"status":"partial","solver_completed":false,"initial_checked":false});
    let incomplete = verify_sat_initial_cell(&f.root, &f.config, &partial, &artifacts, policy());
    assert_eq!(incomplete.verdict(), SatReplayVerdict::Incomplete);
    assert!(matches!(
        &incomplete.owner,
        SatReplayOwner::Outcome(
            koblitz_m3::sat_original_owner::central::Outcome::IncompleteMissingProducerLedger(_)
        )
    ));
    let mut forged = partial.clone();
    forged["scalar_recovered_without_witness"] = json!(true);
    let rejected = verify_sat_initial_cell(&f.root, &f.config, &forged, &artifacts, policy());
    assert_eq!(rejected.verdict(), SatReplayVerdict::Rejected);
    assert!(matches!(
        &rejected.owner,
        SatReplayOwner::Outcome(
            koblitz_m3::sat_original_owner::central::Outcome::IncompleteMissingProducerLedger(_)
        )
    ));
}
#[test]
fn authenticated_unsupported_policy_keeps_unknown_owner_and_rejects_overclaim() {
    let f = verified_worker_original_tests::Fixture::with_policy_profile(
        23,
        false,
        4,
        "sat",
        16384,
        true,
        false,
        Some(dataset::types::FactorBasePolicy::SampledXProjectedControl {
            candidate_limit: 64,
        }),
    );
    // Freeze the original checked policy/capture context before measured setup.
    telemetry::checked::install_policy(&f.root).unwrap();
    let limits = capture::capture_limits(128).unwrap();
    storage::write_json(
        &f.root,
        "capture-binding.json",
        "partial-capture-binding",
        &f.binding,
    )
    .unwrap();
    storage::write_json(
        &f.root,
        "capture-limits.json",
        "partial-capture-limits",
        &limits,
    )
    .unwrap();
    dataset::execution::create(&f.root).unwrap();
    let mut artifacts = Vec::new();
    let case = telemetry::measured(&f.root, &mut artifacts, "curve-construction", || {
        dataset::curves::generate(&f.config)
    })
    .unwrap();
    let _original = verify_and_store_worker_originals(
        &f.config,
        &case,
        &f.root,
        Some(&f.binding),
        &mut artifacts,
    )
    .unwrap();
    contexts(&f.root, &mut artifacts);
    let partial = json!({"status":"partial","solver_completed":false});
    let unknown = verify_sat_initial_cell(&f.root, &f.config, &partial, &artifacts, policy());
    assert_eq!(unknown.verdict(), SatReplayVerdict::Unknown);
    assert!(matches!(
        &unknown.owner,
        SatReplayOwner::Outcome(
            koblitz_m3::sat_original_owner::central::Outcome::UnknownOriginalMath(_)
        )
    ));
    let mut forged = partial;
    forged["ecdlp_pipeline_completed"] = json!(true);
    let rejected = verify_sat_initial_cell(&f.root, &f.config, &forged, &artifacts, policy());
    assert_eq!(rejected.verdict(), SatReplayVerdict::Rejected);
    assert!(matches!(
        &rejected.owner,
        SatReplayOwner::Outcome(
            koblitz_m3::sat_original_owner::central::Outcome::UnknownOriginalMath(_)
        )
    ));
}
