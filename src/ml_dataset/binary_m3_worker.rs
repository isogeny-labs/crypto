//! Complete ordinary worker orchestration; central replay owns proof admission.
use super::*;
use serde_json::json;

// Normal branch: at most two not-yet-moved buffers + two signed slots +
// four terminal slots + one unretained encoding buffer = nine. Early branches retain at most three owners and
// never enter normal assembly. Capacity ten includes one spare fixed entry.
const _: () = assert!(solvers::WorkerResidualCustody::CAPACITY >= 9);

fn partial(reason: &str) -> solvers::WorkerSolveOutput {
    solvers::WorkerSolveOutput::legacy(types::SolverData {
        outcome: json!({"adapter_id":"binary-m3-native-v1","strategy":"binary-m3-native-v1",
            "status":"partial","reason":reason,"solver_completed":false,
            "scalar_recovered_without_witness":false,"pdp_witness_verified":false,
            "group_domain_complete":false,"exhaustive_no_decomposition":false}),
        artifacts: Vec::new(),
    })
}
// Only reachable after the full terminal work/memory allowance was reserved.
// This uses that allowance once; it does not retry capture or write new roots.
fn partial_admitted(reason: &'static str, budget: &mut Budget) -> solvers::WorkerSolveOutput {
    if budget.stopped.is_none() {
        let _ = budget.observer_failed();
    }
    let ledger = budget.saved_ledger();
    let mut output = partial(reason);
    output.data.outcome["admission_ledger"] = json!(ledger);
    output.data.outcome["first_stop"] = json!(ledger.first_stop);
    output.data.outcome["adapter_reserved_native_work"] = json!(ledger.native_reserved.work);
    output.data.outcome["adapter_reserved_checking_work"] = json!(ledger.checking_reserved_work);
    output.data.outcome["adapter_reserved_bytes"] = json!(ledger.adapter_reserved_bytes);
    output.data.outcome["native_reserved_bytes"] = json!(ledger.native_reserved.bytes);
    output.data.outcome["charged_trace_bytes"] = json!(ledger.charged_trace_bytes);
    output.data.outcome["performed_group_additions"] = json!(ledger.group_additions);
    // Closed fixed keys, one bounded ledger and static reason. No encoded copy.
    assert!(worker_terminal_encoded_size(&output.data.outcome, WORKER_TERMINAL_CAP).is_ok());
    output
}

fn residual(
    custody: &mut solvers::WorkerResidualCustody,
    path: &'static str,
    bytes: Vec<u8>,
    root: Option<types::Artifact>,
) {
    if bytes.is_empty() && root.is_none() {
        return;
    }
    if let Err(_owner) = custody.retain(solvers::WorkerResidualArtifact {
        path,
        bytes,
        untransferred_root: root,
    }) {
        unreachable!("closed ordinary residual inventory exceeds admitted capacity");
    }
}

pub(in crate::ml_dataset::solvers) fn run(
    original: &VerifiedWorkerOriginal<'_>,
    case: &types::CaseData,
    context: &execution::SolverExecutionContext,
    binding: &capture::AttemptBinding,
    limits: &capture::CaptureLimits,
) -> Result<solvers::WorkerSolveOutput, String> {
    if original.binding != Some(binding) || context.launch.binding != *binding {
        return Err("ordinary original token differs from execution/capture attempt".into());
    }
    let mut budget = match solvers::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
        Some(solvers::EventSink::Bound(sink)) => {
            admit_original(original.root, original.config, context, sink)
        }
        _ => Err("ordinary worker lacks original bound sink".into()),
    }) {
        Ok(b) => b,
        Err(_) => return Ok(partial("original resource admission refused")),
    };
    // Output/final capture tariffs are admitted before all producer side effects.
    if budget
        .reserve_checking(
            32 * WORKER_TERMINAL_CAP as u64 + 65536 + WORKER_TERMINAL_GUARD_WORK,
            2 * 1024 * 1024 + 128 * 1024,
            Instant::now(),
        )
        .is_err()
    {
        // No terminal allowance was admitted: retain the existing minimal
        // refusal rather than spend a failed allowance on ledger encoding.
        return Ok(partial("ordinary terminal output admission refused"));
    }
    if budget
        .reserve_trace(4 * WORKER_TERMINAL_CAP as u64 + 4096, Instant::now())
        .is_err()
    {
        return Ok(partial_admitted(
            "ordinary terminal capture admission refused",
            &mut budget,
        ));
    }
    // Re-admission may only shorten the original absolute deadline. Pass this
    // exact admitted deadline to construction/persistence, never renew it.
    let context = execution::SolverExecutionContext {
        launch: context.launch.clone(),
        deadline: budget.deadline,
    };
    let mut custody = match solvers::WorkerResidualCustody::admit(|work, bytes| {
        budget
            .reserve_checking(work, bytes, Instant::now())
            .map_err(|s| format!("{s:?}"))
    }) {
        Ok(c) => c,
        Err(_) => {
            return Ok(partial_admitted(
                "ordinary residual transport admission refused",
                &mut budget,
            ))
        }
    };
    let mut registry = match OrdinaryRootRegistry::admit(original, &mut budget, limits) {
        Ok(r) => r,
        Err(_) => {
            return Ok(partial_admitted(
                "ordinary root registry admission refused",
                &mut budget,
            ))
        }
    };
    let mut domain = false;
    let mut witness = false;
    let mut witness_root = false;
    let mut matrix_complete = false;
    let mut retention_complete = false;
    let mut reason = None;
    // No Err returns after this point: actual acknowledged roots must move out
    // even when subsequent construction, capture, matrix or delivery refuses.
    match signed_stage::OrdinarySignedOwned::admit(binding, limits, &mut budget) {
        Err(_) => reason = Some("ordinary signed retention admission refused"),
        Ok(signed) => match construct(original, &context, &mut budget) {
            Err(_) => reason = Some("ordinary construction admission refused"),
            Ok(retained) => {
                match persist(original, case, &context, limits, retained, &mut budget) {
                    Err(mut failed) => {
                        if failed.root_artifact.is_some() {
                            let _ = registry.adopt_persistence_failure(&mut failed);
                        }
                        residual(
                            &mut custody,
                            super::super::PATH,
                            failed.retained.bytes,
                            None,
                        );
                        residual(
                            &mut custody,
                            INPUT_PATH,
                            failed.input_bytes.into_bytes(),
                            failed.root_artifact,
                        );
                        reason = Some("ordinary input persistence incomplete");
                    }
                    Ok(mut input) => match OrdinaryEvents::admit(original, limits, &mut input) {
                        Err(_) => {
                            let stop = input.ledger.stopped;
                            let step = OrdinaryMatrixStep {
                                input,
                                report: None,
                                adapter_stop: stop,
                            };
                            match registry.adopt_input(step) {
                                Ok(input) => residual(
                                    &mut custody,
                                    super::super::PATH,
                                    input.retained.bytes,
                                    None,
                                ),
                                Err((_, step)) => {
                                    residual(
                                        &mut custody,
                                        super::super::PATH,
                                        step.input.retained.bytes,
                                        None,
                                    );
                                    residual(
                                        &mut custody,
                                        INPUT_PATH,
                                        step.input.bytes.into_bytes(),
                                        Some(step.input.root_artifact),
                                    );
                                }
                            }
                            reason = Some("ordinary event admission refused");
                        }
                        Ok(mut events) => {
                            let engine = match original.config.solver.as_str() {
                                "f4" => native::Engine::F4Macaulay,
                                "f5" => native::Engine::F5KoszulFrobenius,
                                _ => unreachable!("closed dispatch"),
                            };
                            let step = matrix_step(
                                input,
                                engine,
                                original.config.degree_bound,
                                &mut |e, b| events.forward(e, b),
                            );
                            match registry.adopt_input(step) {
                                Err((_, step)) => {
                                    residual(
                                        &mut custody,
                                        super::super::PATH,
                                        step.input.retained.bytes,
                                        None,
                                    );
                                    residual(
                                        &mut custody,
                                        INPUT_PATH,
                                        step.input.bytes.into_bytes(),
                                        Some(step.input.root_artifact),
                                    );
                                    residual(
                                        &mut custody,
                                        Slot::Events.identity().0,
                                        std::mem::take(&mut events.retained_wire),
                                        None,
                                    );
                                    residual(
                                        &mut custody,
                                        "partial-binary-m3-unretained-event.jsonl",
                                        events.take_unretained_encoding(),
                                        None,
                                    );
                                    reason = Some("ordinary original owner transfer refused");
                                }
                                Ok(mut input) => {
                                    matrix_complete = input
                                        .report
                                        .as_ref()
                                        .is_some_and(|r| r.native_step_complete);
                                    let mut signed = input.run_signed(&mut events, signed);
                                    let group_stored =
                                        signed.store(&mut input, &mut registry).is_ok();
                                    (domain, witness, witness_root) = signed.facts();
                                    retention_complete = registry
                                        .retain_authoritative_slots(&mut input, &mut events)
                                        .is_ok()
                                        && group_stored;
                                    residual(
                                        &mut custody,
                                        super::super::PATH,
                                        input.retained.bytes,
                                        None,
                                    );
                                    residual(
                                        &mut custody,
                                        Slot::Events.identity().0,
                                        std::mem::take(&mut events.retained_wire),
                                        None,
                                    );
                                    residual(
                                        &mut custody,
                                        "partial-binary-m3-unretained-event.jsonl",
                                        events.take_unretained_encoding(),
                                        None,
                                    );
                                    signed.move_residuals(&mut custody);
                                }
                            }
                        }
                    },
                }
            }
        },
    }
    if let Some(slots) = registry.slots.take() {
        slots.move_residuals(&mut custody);
    }
    let ledger = budget.saved_ledger();
    let completed = domain
        && matrix_complete
        && retention_complete
        && custody.items.is_empty()
        && ledger.phase == Phase::Complete
        && ledger.first_stop.is_none()
        && (!witness || witness_root);
    let retained_bytes = custody
        .items
        .iter()
        .map(|i| i.bytes.len() as u64)
        .sum::<u64>();
    let mut outcome = json!({
        "adapter_id":"binary-m3-native-v1","strategy":"binary-m3-native-v1",
        "status":if completed{"verified"}else{"partial"},"reason":reason,
        "solver_completed":completed,"pdp_witness_verified":witness&&witness_root,
        "retained_pdp_witness_verified":witness,"certificate_durably_persisted":witness&&witness_root,
        "group_domain_complete":completed&&domain,"exhaustive_no_decomposition":completed&&!witness,
        "native_backend_invoked":completed&&matrix_complete,"algebra_completed":completed&&matrix_complete,
        "native_statistics_complete":completed&&matrix_complete,"scalar_recovered_without_witness":false,
        "first_fall_degree":null,"last_fall_degree":null,"fall_degree_verified":false,
        "exact_instruction_count_verified":false,"operation_trajectory_verified":false,
        "full_execution_trace_verified":false,"independent_derivation_verified":false,
        "first_stop":ledger.first_stop,"parent_delivery_complete":retention_complete,
        "adapter_reserved_native_work":ledger.native_reserved.work,
        "adapter_reserved_checking_work":ledger.checking_reserved_work,
        "adapter_reserved_bytes":ledger.adapter_reserved_bytes,"native_reserved_bytes":ledger.native_reserved.bytes,
        "charged_trace_bytes":ledger.charged_trace_bytes,"performed_group_additions":ledger.group_additions,
        "unpublished_retained_bytes":retained_bytes,"unpublished_owner_count":custody.items.len(),
        "untransferred_root_present":custody.has_untransferred_roots(),
        "residual_custody_scope":"owned through parent result and registry finalization; not durable publication"
    });
    finalize_worker_capture_profile(
        Profile::OrdinaryBinary,
        &mut outcome,
        &mut budget,
        Instant::now,
        solvers::live_event,
        || {
            solvers::LIVE_SINK.with(|slot| match slot.borrow_mut().as_mut() {
                Some(solvers::EventSink::Bound(sink)) => sink.finish().map_err(|e| e.to_string()),
                _ => Err("ordinary capture disappeared during finalization".into()),
            })
        },
    );
    let mut output = registry.receipts.finish(types::SolverData {
        outcome,
        artifacts: Vec::new(),
    });
    output.residual = custody;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml_dataset::{self as dataset, telemetry};
    // Reuse a real deterministic capture, then restore its exact bytes. No
    // producer replay, fresh random input, or invented acknowledged event.
    fn check_pending_index_without_result(
        dir: &std::path::Path,
        config: &types::CaseConfig,
        original_outcome: &serde_json::Value,
        artifacts: &[types::Artifact],
    ) {
        struct Restore(Vec<(std::path::PathBuf, Vec<u8>)>);
        impl Drop for Restore {
            fn drop(&mut self) {
                for (path, bytes) in &self.0 {
                    std::fs::write(path, bytes).unwrap();
                }
            }
        }
        let paths = [
            "live/events.jsonl",
            "live/committed.json",
            Slot::Events.identity().0,
        ];
        let restore = Restore(
            paths
                .iter()
                .map(|p| {
                    let path = dir.join(p);
                    let bytes = std::fs::read(&path).unwrap();
                    (path, bytes)
                })
                .collect(),
        );
        let lines: Vec<&[u8]> = restore.0[0].1.split_inclusive(|b| *b == b'\n').collect();
        let records: Vec<serde_json::Value> = lines
            .iter()
            .map(|line| serde_json::from_slice(line).unwrap())
            .collect();
        let original_index: serde_json::Value = serde_json::from_slice(&restore.0[2].1).unwrap();
        let partial = json!({"adapter_id":"binary-m3-native-v1", "status":"partial",
            "solver_completed":false});
        for phase in ["matrix", "signed"] {
            let cut = records
                .iter()
                .position(|r| {
                    r["event"]["kind"] == "binary_m3_native"
                        && r["event"]["payload"]["event"]["phase"] == phase
                })
                .unwrap();
            assert!(cut > 0);
            let prefix = lines[..cut].concat();
            let mut head: serde_json::Value = serde_json::from_slice(&restore.0[1].1).unwrap();
            head["event_count"] = records[cut - 1]["sequence"].clone();
            head["byte_end"] = json!(prefix.len());
            head["last_event_sha256"] = json!(dataset::digest(lines[cut - 1]));
            head["payload_count"] = json!(records[..cut]
                .iter()
                .filter(|r| r["event"]["kind"] == "capture_payload_committed")
                .count());
            head["payload_bytes"] = json!(records[..cut]
                .iter()
                .filter(|r| r["event"]["kind"] == "capture_payload_committed")
                .map(|r| r["event"]["artifact"]["bytes"].as_u64().unwrap())
                .sum::<u64>());
            head["stop"] = serde_json::Value::Null;
            let mut index = original_index.clone();
            index["end"] = index["start"].clone();
            let (mut offset, mut count, mut wire_bytes) = (0usize, 0u64, 0usize);
            for (line, record) in lines[..cut].iter().zip(&records[..cut]) {
                offset += line.len();
                if record["event"]["kind"] == "binary_m3_native" {
                    count += 1;
                    wire_bytes += line.len();
                    index["end"] = json!({"binding_sha256":head["binding_sha256"],
                        "event_count":record["sequence"],"byte_end":offset,
                        "last_event_sha256":dataset::digest(line)});
                }
            }
            index["acknowledged_events"] = json!(count);
            index["acknowledged_wire_bytes"] = json!(wire_bytes);
            std::fs::write(&restore.0[0].0, &prefix).unwrap();
            std::fs::write(&restore.0[1].0, serde_json::to_vec(&head).unwrap()).unwrap();
            for mutation in 0..5 {
                let mut pending = records[cut].clone();
                match mutation {
                    0 => {} // Authenticated original pending event must be accepted.
                    1 => {
                        pending["event"]["payload"]["input_sha256"][0] = json!(
                            pending["event"]["payload"]["input_sha256"][0]
                                .as_u64()
                                .unwrap()
                                ^ 1
                        )
                    }
                    2 => pending["event"]["kind"] = json!("foreign_native"),
                    3 => pending["event"]["payload"]["input_capture_sequence"] = json!(0),
                    4 if phase == "matrix" => {
                        // Authentic signed arithmetic, rebound to the authentic
                        // pre-matrix boundary: phase order must still reject it.
                        let signed = records
                            .iter()
                            .find(|r| {
                                r["event"]["kind"] == "binary_m3_native"
                                    && r["event"]["payload"]["event"]["phase"] == "signed"
                            })
                            .unwrap();
                        pending["event"]["payload"]["event"] =
                            signed["event"]["payload"]["event"].clone();
                    }
                    4 => pending["event"]["payload"]["event"]["prefix"]["summand"] = json!(99),
                    _ => unreachable!(),
                }
                let mut tail = serde_json::to_vec(&pending).unwrap();
                tail.push(b'\n');
                index["pending_tail"] = json!(tail);
                std::fs::write(&restore.0[2].0, serde_json::to_vec(&index).unwrap()).unwrap();
                let mut manifest = artifacts.to_vec();
                manifest.retain(|a| {
                    ![
                        Slot::Result,
                        Slot::GroupReplay,
                        Slot::ParentDelivery,
                        Slot::Witness,
                    ]
                    .iter()
                    .any(|slot| slot.matches(a))
                });
                assert!(!manifest.iter().any(|a| Slot::Result.matches(a)));
                for path in paths {
                    let member = manifest.iter_mut().find(|a| a.path == path).unwrap();
                    let bytes = std::fs::read(dir.join(path)).unwrap();
                    member.bytes = bytes.len() as u64;
                    member.sha256 = dataset::digest(&bytes);
                }
                let result = replay::verify(dir, config, &partial, &manifest);
                if mutation == 0 {
                    let assessment = result.unwrap().unwrap();
                    assert_eq!(assessment["solver_completed"], false);
                    assert_eq!(assessment["group_domain_complete"], false);
                } else {
                    let error = result.unwrap_err();
                    if mutation <= 3 {
                        assert!(
                            error.contains("ordinary pending event identity"),
                            "{phase}: {error}"
                        );
                    } else if phase == "matrix" {
                        assert!(
                            error.contains("ordinary pending signed before matrix completion"),
                            "{error}"
                        );
                    } else {
                        assert!(error.contains("signed"), "{error}");
                    }
                }
            }
        }
        drop(restore);
        assert_eq!(
            replay::verify(dir, config, original_outcome, artifacts)
                .unwrap()
                .unwrap()["solver_completed"],
            true
        );
    }

    #[test]
    fn early_ordinary_refusals_retain_paid_terminal_ledger_without_capture_retry() {
        for (work, reserved) in [(390, 397312), (391, 397312), (392, 401408)] {
            let f = solvers::verified_worker_original_tests::Fixture::with_ordinary_limits_shape(
                23, false, "f4", false, 3, work,
            );
            execution::create(&f.root).unwrap();
            let context = execution::admit(&f.root).unwrap().unwrap();
            let (_, limits) = dataset::admit_capture_context(&f.root).unwrap().unwrap();
            let mut registry = Vec::new();
            // The checked original verifier requires the real construction
            // measurement predecessor, just like the production worker.
            let case = telemetry::measured(&f.root, &mut registry, "curve-construction", || {
                solvers::curves::generate(&f.config)
            })
            .unwrap();
            let original = solvers::verify_and_store_worker_originals(
                &f.config,
                &case,
                &f.root,
                Some(&f.binding),
                &mut registry,
            )
            .unwrap();
            let output = telemetry::measured(&f.root, &mut registry, "solver", || {
                solvers::solve_with_verified_worker_originals(
                    &f.config,
                    &case,
                    &f.root.join("live"),
                    f.binding.clone(),
                    limits,
                    Some(&context),
                    &original,
                )
            })
            .unwrap();
            let (data, stored, residual) = output.into_parts();
            assert!(stored.is_empty() && residual.items.is_empty() && data.artifacts.is_empty());
            assert_eq!(data.outcome["status"], "partial");
            assert_eq!(data.outcome["strategy"], "binary-m3-native-v1");
            assert_eq!(data.outcome["first_stop"], json!(Stop::Work));
            assert_eq!(
                data.outcome["adapter_reserved_checking_work"],
                json!(reserved)
            );
            assert_eq!(data.outcome["charged_trace_bytes"], json!(36864));
            assert_eq!(data.outcome["performed_group_additions"], json!(0));
            assert!(worker_terminal_encoded_size(&data.outcome, WORKER_TERMINAL_CAP).is_ok());
            assert!(!f.root.join(INPUT_PATH).exists());
            let head: capture::CommittedHead =
                dataset::read_json(&f.root.join("live/committed.json")).unwrap();
            assert_eq!(head.event_count, 0);
            assert_eq!(head.byte_end, 0);
            assert_eq!(head.payload_count, 0);
            storage::write_json(
                &f.root,
                "worker-artifacts.json",
                "artifact-registry",
                &registry,
            )
            .unwrap();
            let artifacts = dataset::collect_cell_artifacts(&f.root).unwrap();
            assert!(
                replay::verify(&f.root, &f.config, &data.outcome, &artifacts)
                    .unwrap()
                    .is_some()
            );
            for mutation in 0..3 {
                let mut bad = data.outcome.clone();
                match mutation {
                    0 => bad["adapter_reserved_checking_work"] = json!(0),
                    1 => bad["admission_ledger"]["original_limits"]["native_work"] = json!(1),
                    _ => bad["first_stop"] = serde_json::Value::Null,
                }
                assert!(replay::verify(&f.root, &f.config, &bad, &artifacts).is_err());
            }
        }
    }

    #[test]
    fn genuine_unsupported_ordinary_profile_exports_only_restricted_summary() {
        // Freeze genuine unsupported degrees before original generation. The
        // n19 control remains; all controls use unchanged 16384 work/capture caps.
        for (n, degree) in [(19, 3), (23, 7), (23, 32)] {
            for engine in ["f4", "f5"] {
                for twist in [false, true] {
                    let f = solvers::verified_worker_original_tests::Fixture::with_ordinary_degree_shape(
                n, twist, engine, false, degree,
            );
                    execution::create(&f.root).unwrap();
                    let context = execution::admit(&f.root).unwrap().unwrap();
                    let (_, limits) = dataset::admit_capture_context(&f.root).unwrap().unwrap();
                    let mut registry = Vec::new();
                    let case =
                        telemetry::measured(&f.root, &mut registry, "curve-construction", || {
                            solvers::curves::generate(&f.config)
                        })
                        .unwrap();
                    let original = solvers::verify_and_store_worker_originals(
                        &f.config,
                        &case,
                        &f.root,
                        Some(&f.binding),
                        &mut registry,
                    )
                    .unwrap();
                    let output = telemetry::measured(&f.root, &mut registry, "solver", || {
                        solvers::solve_with_verified_worker_originals(
                            &f.config,
                            &case,
                            &f.root.join("live"),
                            f.binding.clone(),
                            limits.clone(),
                            Some(&context),
                            &original,
                        )
                    })
                    .unwrap();
                    let (data, stored, residual) = output.into_parts();
                    assert_eq!(data.outcome["status"], "unsupported");
                    assert_eq!(
                        data.outcome["strategy"],
                        "ordinary-binary-profile-unsupported"
                    );
                    assert!(data.artifacts.is_empty());
                    assert!(stored.is_empty());
                    assert!(residual.items.is_empty());
                    storage::write_json(
                        &f.root,
                        "worker-artifacts.json",
                        "artifact-registry",
                        &registry,
                    )
                    .unwrap();
                    let artifacts = dataset::collect_cell_artifacts(&f.root).unwrap();
                    assert!(
                        replay::verify(&f.root, &f.config, &data.outcome, &artifacts)
                            .unwrap()
                            .is_none()
                    );
                    // First accept the actual worker outcome; then reject claims that
                    // relabel unsupported originals as a native source or success.
                    for mutation in 0..4 {
                        let mut bad = data.outcome.clone();
                        match mutation {
                            0 => bad["adapter_id"] = json!("binary-m3-native-v1"),
                            1 => bad["solver_completed"] = json!(true),
                            2 => bad["pdp_witness_verified"] = json!(true),
                            _ => bad["status"] = json!("verified"),
                        }
                        assert!(replay::verify(&f.root, &f.config, &bad, &artifacts).is_err());
                    }
                    let mut callbacks = 0;
                    let summary =
                crate::ml_dataset::solvers::koblitz_m3::ordinary_causal_projection::project(
                    &f.root,
                    &f.config,
                    &data.outcome,
                    &artifacts,
                    &mut |_, _| {
                        callbacks += 1;
                        Ok(())
                    },
                )
                .unwrap();
                    assert_eq!(callbacks, 0);
                    assert_eq!(summary["admission_status"], "unsupported_original_profile");
                    assert_eq!(summary["group_examples"], 0);
                    assert_eq!(summary["unknown_group_suffix"], true);
                    assert_eq!(summary["source_completion_claimed"], false);
                    crate::ml_dataset::export::check_ordinary_export_rows(
                        &f.root,
                        &f.config,
                        &data.outcome,
                        &artifacts,
                        0,
                    );
                }
            }
        }
    }
    #[test]
    fn full_requested_ordinary_profile_dispatch_replays_actual_originals_and_preserves_refusals() {
        let mut visited = 0;
        let mut input_cells = 0;
        let mut native_cells = 0;
        let mut signed_cells = 0;
        let mut combined_mutations = 0;
        for n in (23..=31).chain(47..=61) {
            for twist in [false, true] {
                for engine in ["f4", "f5"] {
                    let f=solvers::verified_worker_original_tests::Fixture::with_ordinary_runtime_shape(n,twist,engine);
                    execution::create(&f.root).unwrap();
                    let context = execution::admit(&f.root).unwrap().unwrap();
                    let (_, limits) = crate::ml_dataset::admit_capture_context(&f.root)
                        .unwrap()
                        .unwrap();
                    let mut registry = Vec::new();
                    let case =
                        telemetry::measured(&f.root, &mut registry, "curve-construction", || {
                            solvers::curves::generate(&f.config)
                        })
                        .unwrap();
                    let original = solvers::verify_and_store_worker_originals(
                        &f.config,
                        &case,
                        &f.root,
                        Some(&f.binding),
                        &mut registry,
                    )
                    .unwrap();
                    let before: [Vec<u8>; 3] = ["public.json", "factor-base.json", "config.json"]
                        .map(|p| std::fs::read(f.root.join(p)).unwrap());
                    let output = telemetry::measured(&f.root, &mut registry, "solver", || {
                        solvers::solve_with_verified_worker_originals(
                            &f.config,
                            &case,
                            &f.root.join("live"),
                            f.binding.clone(),
                            limits.clone(),
                            Some(&context),
                            &original,
                        )
                    })
                    .unwrap();
                    let (data, stored, residual) = output.into_parts();
                    assert!(data.artifacts.is_empty());
                    assert!(residual.items.len() <= solvers::WorkerResidualCustody::CAPACITY);
                    input_cells += usize::from(stored.iter().any(|a| a.path == INPUT_PATH));
                    native_cells += usize::from(
                        stored
                            .iter()
                            .any(|a| a.path == Slot::Events.identity().0 && a.bytes > 0),
                    );
                    signed_cells += usize::from(
                        stored
                            .iter()
                            .any(|a| a.path == Slot::GroupReplay.identity().0),
                    );
                    for root in stored {
                        assert!(!registry.iter().any(|a| a.path == root.path));
                        registry.push(root);
                    }
                    storage::write_json(
                        &f.root,
                        "worker-artifacts.json",
                        "artifact-registry",
                        &registry,
                    )
                    .unwrap();
                    let artifacts = dataset::collect_cell_artifacts(&f.root).unwrap();
                    let verified =
                        replay::verify(&f.root, &f.config, &data.outcome, &artifacts).unwrap();
                    assert!(verified.is_some(), "n={n} twist={twist} engine={engine}");
                    assert_eq!(
                        verified.unwrap()["solver_completed"],
                        data.outcome["solver_completed"]
                    );
                    for (name, bytes) in ["public.json", "factor-base.json", "config.json"]
                        .into_iter()
                        .zip(before)
                    {
                        assert_eq!(std::fs::read(f.root.join(name)).unwrap(), bytes);
                    }
                    if data.outcome["solver_completed"] != true {
                        assert!(matches!(
                            data.outcome["status"].as_str(),
                            Some("partial" | "unsupported")
                        ));
                        let mut forged = data.outcome.clone();
                        forged["solver_completed"] = true.into();
                        assert!(replay::verify(&f.root, &f.config, &forged, &artifacts).is_err());
                    }
                    let mut hidden = data.outcome.clone();
                    hidden["adapter_id"] = serde_json::Value::Null;
                    hidden["strategy"] = serde_json::Value::Null;
                    if artifacts.iter().any(|a| a.path.starts_with("binary-m3-")) {
                        assert!(replay::verify(&f.root, &f.config, &hidden, &artifacts)
                            .unwrap()
                            .is_some());
                    }
                    if combined_mutations == 0
                        && data.outcome["performed_group_additions"]
                            .as_u64()
                            .unwrap_or(0)
                            > 0
                        && artifacts.iter().any(|a| Slot::Result.matches(a))
                        && artifacts.iter().any(|a| Slot::GroupReplay.matches(a))
                    {
                        let mut forged_artifacts = artifacts.clone();
                        let result_index = forged_artifacts
                            .iter()
                            .position(|a| Slot::Result.matches(a))
                            .unwrap();
                        let result_path = f.root.join(&forged_artifacts[result_index].path);
                        let original_result = std::fs::read(&result_path).unwrap();
                        let mut result: serde_json::Value =
                            serde_json::from_slice(&original_result).unwrap();
                        result["ledger"]["group_additions"] = 0.into();
                        let changed = serde_json::to_vec(&result).unwrap();
                        std::fs::write(&result_path, &changed).unwrap();
                        forged_artifacts[result_index].sha256 = dataset::digest(&changed);
                        forged_artifacts[result_index].bytes = changed.len() as u64;
                        let mut restore_delivery = None;
                        if let Some(i) = forged_artifacts
                            .iter()
                            .position(|a| Slot::ParentDelivery.matches(a))
                        {
                            let path = f.root.join(&forged_artifacts[i].path);
                            let original = std::fs::read(&path).unwrap();
                            let mut delivery: serde_json::Value =
                                serde_json::from_slice(&original).unwrap();
                            delivery["ledger"]["group_additions"] = 0.into();
                            for root in delivery["accepted_roots_before_delivery"]
                                .as_array_mut()
                                .unwrap()
                            {
                                if root["path"] == forged_artifacts[result_index].path {
                                    *root = serde_json::to_value(&forged_artifacts[result_index])
                                        .unwrap();
                                }
                            }
                            delivery["prior_slots"][2]["retained_bytes"] = changed.len().into();
                            let changed_delivery = serde_json::to_vec(&delivery).unwrap();
                            std::fs::write(&path, &changed_delivery).unwrap();
                            forged_artifacts[i].sha256 = dataset::digest(&changed_delivery);
                            forged_artifacts[i].bytes = changed_delivery.len() as u64;
                            restore_delivery = Some((path, original));
                        }
                        let mut forged = data.outcome.clone();
                        forged["performed_group_additions"] = 0.into();
                        let error = replay::verify(&f.root, &f.config, &forged, &forged_artifacts)
                            .unwrap_err();
                        assert!(
                            error.contains("ledger undercounts"),
                            "unexpected rejection: {error}"
                        );
                        std::fs::write(&result_path, original_result).unwrap();
                        if let Some((path, bytes)) = restore_delivery {
                            std::fs::write(path, bytes).unwrap();
                        }
                        combined_mutations += 1;
                    }
                    if n == 23 && !twist && engine == "f4" {
                        let index_i = artifacts
                            .iter()
                            .position(|a| Slot::Events.matches(a))
                            .unwrap();
                        let index_path = f.root.join(&artifacts[index_i].path);
                        let original_index = std::fs::read(&index_path).unwrap();
                        for mutation in 0..5 {
                            let mut value: serde_json::Value =
                                serde_json::from_slice(&original_index).unwrap();
                            match mutation {
                                0 => {
                                    value["end"]["byte_end"] =
                                        (value["end"]["byte_end"].as_u64().unwrap() + 1).into()
                                }
                                1 => value["end"]["last_event_sha256"] = "0".repeat(64).into(),
                                2 => {
                                    value["start"]["event_count"] =
                                        (value["start"]["event_count"].as_u64().unwrap() + 1).into()
                                }
                                3 => {
                                    value["acknowledged_events"] =
                                        (value["acknowledged_events"].as_u64().unwrap() + 1).into()
                                }
                                _ => value["pending_tail"] = serde_json::json!([1]),
                            }
                            let changed = serde_json::to_vec(&value).unwrap();
                            std::fs::write(&index_path, &changed).unwrap();
                            let mut forged = artifacts.clone();
                            forged[index_i].bytes = changed.len() as u64;
                            forged[index_i].sha256 = dataset::digest(&changed);
                            let error = replay::verify(&f.root, &f.config, &data.outcome, &forged)
                                .unwrap_err();
                            assert!(error.contains("index"), "wrong rejection: {error}");
                            std::fs::write(&index_path, &original_index).unwrap();
                        }
                        // Canonical role/path ownership cannot be duplicated or
                        // relabeled into another proof namespace.
                        let input = artifacts.iter().find(|a| a.path == INPUT_PATH).unwrap();
                        let mut duplicate = artifacts.clone();
                        duplicate.push(input.clone());
                        assert!(
                            replay::verify(&f.root, &f.config, &data.outcome, &duplicate).is_err()
                        );
                        let mut relabeled = artifacts.clone();
                        relabeled
                            .iter_mut()
                            .find(|a| a.path == INPUT_PATH)
                            .unwrap()
                            .role = "restricted-koblitz-m3-input".into();
                        assert!(
                            replay::verify(&f.root, &f.config, &data.outcome, &relabeled).is_err()
                        );
                        let mut unsupported = data.outcome.clone();
                        unsupported["first_fall_degree"] = 4.into();
                        assert!(
                            replay::verify(&f.root, &f.config, &unsupported, &artifacts).is_err()
                        );
                        if !data.outcome["performed_group_additions"].is_null() {
                            let mut counts = data.outcome.clone();
                            counts["performed_group_additions"] =
                                (counts["performed_group_additions"].as_u64().unwrap() + 1).into();
                            assert!(
                                replay::verify(&f.root, &f.config, &counts, &artifacts).is_err()
                            );
                        }
                    }
                    drop(residual);
                    visited += 1;
                }
            }
        }
        assert_eq!(visited, 96);
        assert_eq!(combined_mutations, 1);
        // These reject vacuous shape-only or universally pre-admission-refused
        // implementations; censored individual cells retain their original caps.
        assert!(input_cells > 0 && native_cells > 0 && signed_cells > 0);
    }

    fn fixed_negative_original(config: &types::CaseConfig) -> types::CaseData {
        let mut case = solvers::curves::generate(config).unwrap();
        let arithmetic = solvers::curves::arithmetic(&case.public_instance).unwrap();
        let generator = solvers::curves::point_words(&case.public_instance["generator"]).unwrap();
        let points: Vec<_> = case.factor_base["points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| solvers::curves::point_words(p).unwrap())
            .collect();
        assert_eq!(points.len(), 4);
        let mut domain = Vec::new();
        for a in &points {
            for b in &points {
                for c in &points {
                    domain.push(arithmetic.reference_add(arithmetic.reference_add(*a, *b), *c));
                }
            }
        }
        let (scalar, target) = (1..=65u64)
            .map(|k| (k, arithmetic.reference_mul(generator, k)))
            .find(|(_, p)| p.is_some() && !domain.contains(p))
            .expect("fixed negative setup bound");
        case.private_witness["scalar"] = scalar.to_string().into();
        case.private_witness["software_control"] = json!({"rule":"first scalar1..65 outside fixed triple domain", "feature_access":"forbidden"});
        case.public_instance["target"] = solvers::curves::point_json(target);
        case.public_instance["population"] = "conditioned-negative-pdp-software-control".into();
        solvers::curves::verify(&case.public_instance, &case.private_witness).unwrap();
        solvers::curves::verify_factor_base_config(
            config,
            &case.public_instance,
            &case.factor_base,
        )
        .unwrap();
        case
    }

    #[test]
    fn deterministic_ordinary_positive_negative_and_infinity_controls_reach_central_replay() {
        for twist in [false, true] {
            for engine in ["f4", "f5"] {
                for positive in [false, true] {
                    let f = solvers::verified_worker_original_tests::Fixture::with_ordinary_control_shape(23, twist, engine, true);
                    execution::create(&f.root).unwrap();
                    let context = execution::admit(&f.root).unwrap().unwrap();
                    let (_, limits) = dataset::admit_capture_context(&f.root).unwrap().unwrap();
                    let mut registry = Vec::new();
                    let case =
                        telemetry::measured(&f.root, &mut registry, "curve-construction", || {
                            if positive {
                                positive_original_bound_worker_tests::positive_original(&f.config)
                            } else {
                                Ok(fixed_negative_original(&f.config))
                            }
                        })
                        .unwrap();
                    let original = solvers::verify_and_store_worker_originals(
                        &f.config,
                        &case,
                        &f.root,
                        Some(&f.binding),
                        &mut registry,
                    )
                    .unwrap();
                    let output = telemetry::measured(&f.root, &mut registry, "solver", || {
                        solvers::solve_with_verified_worker_originals(
                            &f.config,
                            &case,
                            &f.root.join("live"),
                            f.binding.clone(),
                            limits.clone(),
                            Some(&context),
                            &original,
                        )
                    })
                    .unwrap();
                    let (data, roots, custody) = output.into_parts();
                    registry.extend(roots);
                    storage::write_json(
                        &f.root,
                        "worker-artifacts.json",
                        "artifact-registry",
                        &registry,
                    )
                    .unwrap();
                    storage::write_json(
                        &f.root,
                        "worker-process.json",
                        "process-provenance",
                        &json!({"software_control":true}),
                    )
                    .unwrap();
                    std::fs::write(f.root.join("stdout.log"), b"").unwrap();
                    std::fs::write(f.root.join("stderr.log"), b"").unwrap();
                    let artifacts = dataset::collect_cell_artifacts(&f.root).unwrap();
                    let assessment = replay::verify(&f.root, &f.config, &data.outcome, &artifacts)
                        .unwrap()
                        .unwrap();
                    assert_eq!(
                        assessment["solver_completed"], true,
                        "twist={twist} engine={engine} positive={positive}: {}",
                        data.outcome
                    );
                    assert_eq!(data.outcome["pdp_witness_verified"], positive);
                    assert_eq!(data.outcome["exhaustive_no_decomposition"], !positive);
                    assert!(custody.items.is_empty());
                    crate::ml_dataset::export::check_ordinary_export_rows(
                        &f.root,
                        &f.config,
                        &data.outcome,
                        &artifacts,
                        192,
                    );
                    crate::ml_dataset::solvers::koblitz_m3::ordinary_causal_projection::check_admission_mutations(
                        &f.root, &f.config, &data.outcome, &artifacts,
                    );
                    if !twist && engine == "f4" && !positive {
                        check_pending_index_without_result(
                            &f.root,
                            &f.config,
                            &data.outcome,
                            &artifacts,
                        );
                    }
                    let group: SavedSignedReplay = serde_json::from_slice(
                        &std::fs::read(f.root.join(Slot::GroupReplay.identity().0)).unwrap(),
                    )
                    .unwrap();
                    assert_eq!(group.performed_additions, 192);
                    assert!(
                        group
                            .acknowledged
                            .iter()
                            .any(|e| e.summand == 1 && e.prefix.is_none()),
                        "actual inverse-pair infinity prefix missing"
                    );
                    // Independently reseal only delivery, leaving the already-checked
                    // result ledger and all actual arithmetic unchanged.
                    let mut changed_artifacts = artifacts.clone();
                    let i = changed_artifacts
                        .iter()
                        .position(|a| Slot::ParentDelivery.matches(a))
                        .unwrap();
                    let path = f.root.join(&changed_artifacts[i].path);
                    let original_delivery = std::fs::read(&path).unwrap();
                    let mut delivery: serde_json::Value =
                        serde_json::from_slice(&original_delivery).unwrap();
                    delivery["ledger"]["group_additions"] = 0.into();
                    let changed = serde_json::to_vec(&delivery).unwrap();
                    std::fs::write(&path, &changed).unwrap();
                    changed_artifacts[i].bytes = changed.len() as u64;
                    changed_artifacts[i].sha256 = dataset::digest(&changed);
                    let error =
                        replay::verify(&f.root, &f.config, &data.outcome, &changed_artifacts)
                            .unwrap_err();
                    assert!(
                        error.contains("delivery addition ledger differs"),
                        "{error}"
                    );
                    // Exercise the actual outer verifier with optional replay
                    // disabled and labels stripped; reserved paths still force
                    // the ordinary semantic join before accepting the manifest.
                    let run: serde_json::Value =
                        dataset::read_json(&f.root.join("run.json")).unwrap();
                    let mut manifest = dataset::CellManifest {
                        schema: "crypto.ml-dataset.cell/v1".into(),
                        cell_index: u64::from(twist),
                        plan_sha256: run["plan_sha256"].as_str().unwrap().into(),
                        config: f.config.clone(),
                        outcome: data.outcome.clone(),
                        artifacts: changed_artifacts,
                    };
                    manifest.outcome["adapter_id"] = serde_json::Value::Null;
                    manifest.outcome["strategy"] = serde_json::Value::Null;
                    storage::write_json(&f.root, "manifest.json", "manifest", &manifest).unwrap();
                    let outer_error = dataset::verify_cell(&f.root, false)
                        .err()
                        .expect("corrupt delivery accepted without optional replay");
                    assert!(
                        outer_error.contains("delivery addition ledger differs"),
                        "{outer_error}"
                    );
                    std::fs::write(&path, original_delivery).unwrap();
                    manifest.artifacts = artifacts;
                    std::fs::write(
                        f.root.join("manifest.json"),
                        serde_json::to_vec(&manifest).unwrap(),
                    )
                    .unwrap();
                    assert!(dataset::verify_cell(&f.root, false).is_ok());
                }
            }
        }
    }

    #[test]
    fn parent_finalization_collisions_preserve_actual_acknowledged_roots_without_retry() {
        for blocked in ["result.json", "worker-artifacts.json"] {
            let f = solvers::verified_worker_original_tests::Fixture::with_ordinary_runtime_shape(
                23, false, "f4",
            );
            execution::create(&f.root).unwrap();
            // Existing conflicting bytes must not be replaced by an uncharged
            // fallback. The producer and its acknowledged writes still run.
            let collision = b"retained conflicting parent bytes";
            std::fs::write(f.root.join(blocked), collision).unwrap();
            assert!(dataset::worker(&f.config, &f.root).is_err());
            assert_eq!(std::fs::read(f.root.join(blocked)).unwrap(), collision);
            let input = std::fs::read(f.root.join(INPUT_PATH)).unwrap();
            assert!(
                !input.is_empty(),
                "control must reach actual input publication"
            );
            let (_, limits) = dataset::admit_capture_context(&f.root).unwrap().unwrap();
            let live = capture::verify_live_committed_prefix(&f.root, &f.binding, &limits)
                .unwrap()
                .unwrap();
            assert!(live.payloads.iter().any(|(_, artifact)| artifact.path
                == "payloads/binary-m3-input.json"
                && artifact.sha256 == dataset::digest(&input)));
            // No parent registry claims are synthesized on either error path.
            if blocked == "result.json" {
                assert!(!f.root.join("worker-artifacts.json").exists());
                let recovered = dataset::collect_cell_artifacts(&f.root).unwrap();
                let root = recovered.iter().find(|a| a.path == INPUT_PATH).unwrap();
                assert_eq!(root.role, "unregistered-partial-artifact");
                assert_eq!(root.sha256, dataset::digest(&input));
            } else {
                assert!(dataset::collect_cell_artifacts(&f.root).is_err());
            }
            // Failure cannot certify completion, even when producer output was
            // fully computed: parent delivery is an independent requirement.
            let forged = json!({"adapter_id":"binary-m3-native-v1", "strategy":"binary-m3-native-v1",
                "status":"verified", "solver_completed":true});
            assert!(replay::verify(&f.root, &f.config, &forged, &[]).is_err());
        }
    }

    #[test]
    fn residual_transport_admission_and_capacity_keep_rejected_owner() {
        let mut charged = None;
        let mut custody = solvers::WorkerResidualCustody::admit(|w, b| {
            charged = Some((w, b));
            Ok(())
        })
        .unwrap();
        assert!(charged.is_some_and(|(w, b)| w > 0 && b > 0));
        for _ in 0..solvers::WorkerResidualCustody::CAPACITY {
            assert!(custody
                .retain(solvers::WorkerResidualArtifact {
                    path: "partial-binary-m3-test",
                    bytes: vec![1, 2, 3],
                    untransferred_root: None
                })
                .is_ok());
        }
        let item = solvers::WorkerResidualArtifact {
            path: "partial-binary-m3-overflow",
            bytes: vec![4, 5, 6],
            untransferred_root: None,
        };
        let rejected = match custody.retain(item) {
            Err(item) => item,
            Ok(()) => panic!("capacity accepted"),
        };
        assert_eq!(rejected.bytes, [4, 5, 6]);
        assert_eq!(
            custody.items.len(),
            solvers::WorkerResidualCustody::CAPACITY
        );
        let (_, roots, residual) = solvers::WorkerSolveOutput::legacy(types::SolverData {
            outcome: json!({}),
            artifacts: Vec::new(),
        })
        .into_parts();
        assert!(roots.is_empty() && residual.items.is_empty());
    }
}
