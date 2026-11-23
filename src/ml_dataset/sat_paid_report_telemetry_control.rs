//! Actual checked solver-telemetry terminal-write failure after paid SAT report.
use super::*;
use crate::ml_dataset::{self as dataset, capture, storage, telemetry};
#[test]
fn paid_sat_report_and_checkpoint_survive_actual_terminal_telemetry_write_failure() {
    let f = verified_worker_original_tests::Fixture::with_sat_initial_shape(23, false);
    dataset::execution::create(&f.root).unwrap();
    let mut registry = Vec::new();
    let case = telemetry::measured(&f.root, &mut registry, "curve-construction", || {
        dataset::curves::generate(&f.config)
    })
    .unwrap();
    let original = verify_and_store_worker_originals(
        &f.config,
        &case,
        &f.root,
        Some(&f.binding),
        &mut registry,
    )
    .unwrap();
    let context = dataset::execution::admit(&f.root).unwrap().unwrap();
    let (_, caps) = dataset::admit_capture_context(&f.root).unwrap().unwrap();
    registry
        .try_reserve_exact(WORKER_STORED_RECEIPT_MAX + 2)
        .unwrap();
    let mut produced = None;
    let measurement = telemetry::measured(&f.root, &mut registry, "solver", || {
        let output = solve_with_verified_worker_originals(
            &f.config,
            &case,
            &f.root.join("live"),
            f.binding.clone(),
            caps,
            Some(&context),
            &original,
        )?;
        assert_eq!(
            output.data.outcome["initial_checked"], true,
            "actual original-budget positive required before fault"
        );
        assert!(!output.residual.sat_report_unavailable());
        produced = Some(output);
        // Start record and actual native call have completed. Only the durable
        // checked telemetry end record now conflicts with a real directory.
        fs::create_dir(f.root.join("telemetry/solver.json")).unwrap();
        Ok(())
    });
    assert!(
        measurement.is_err(),
        "real terminal telemetry write must fail"
    );
    assert!(f.root.join("telemetry/solver-start.json").is_file());
    let output = produced.expect("owned result survives measured failure");
    let head = fs::read(f.root.join("live/committed.json")).unwrap();
    let expected_head: capture::CommittedHead = serde_json::from_slice(&head).unwrap();
    assert_eq!(
        output.data.outcome["checkpoint_sequence"],
        expected_head.event_count
    );
    assert_eq!(
        output.data.outcome["checkpoint_sha256"],
        expected_head.last_event_sha256
    );
    let mut retained = WorkerResidualCustody::default();
    let result = dataset::finish_retained_worker_output(
        &f.root,
        &mut registry,
        output,
        measurement,
        &mut retained,
    );
    assert!(
        result.is_err(),
        "SAT telemetry failure cannot become completion"
    );
    let owner = retained
        .sat
        .as_ref()
        .expect("actual native/checkpoint custody retained");
    assert!(owner.report_admitted);
    assert_eq!(owner.stop, None);
    assert_eq!(
        owner.stage,
        koblitz_m3::sat_original_owner::worker::Stage::Checkpoint
    );
    assert_eq!(owner.coverage.checked, 1);
    fs::create_dir(f.root.join("result.json")).unwrap();
    assert!(storage::write_json(
        &f.root,
        "result.json",
        "solver-outcome",
        &json!({"status":"error"})
    )
    .is_err());
    assert_eq!(retained.sat.as_ref().unwrap().coverage.checked, 1);
    assert_eq!(head, fs::read(f.root.join("live/committed.json")).unwrap());
}
