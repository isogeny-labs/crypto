use super::schema::Context;
use super::*;

fn request(event: &str, group: i32) -> Request {
    let config = u64::from(event == "instructions");
    let flags = if group < 0 { 97u64 } else { 96 };
    let mut attr = vec![0; 112];
    attr[4..8].copy_from_slice(&112u32.to_le_bytes());
    for (offset, value) in [(8, config), (32, 7), (40, flags)] {
        attr[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    }
    Request {
        event: event.into(),
        perf_type: 0,
        config,
        attr_size: 112,
        read_format: 7,
        attr_flags: flags,
        attr_bytes_native_endian: attr,
        byte_order: "little".into(),
        pid: 0,
        cpu: -1,
        group_fd: group,
        syscall_flags: 8,
    }
}
fn call(operation: &str, n: i64) -> Call {
    Call {
        operation: operation.into(),
        result: CallResult::Ok(n),
    }
}
fn denied(operation: &str) -> Call {
    Call {
        operation: operation.into(),
        result: CallResult::Err(CallError {
            errno: Some(13),
            message: "permission denied".into(),
        }),
    }
}
fn event(event: &str, group: i32, fd: i64, id: u64) -> Event {
    let mut lifecycle = vec![call("open", fd), call("event_id", 0)];
    if group < 0 {
        lifecycle.extend([
            call("reset_group", 0),
            call("enable_group", 0),
            call("disable_group", 0),
        ]);
    }
    lifecycle.extend([call("read", 32), call("close", 0)]);
    Event {
        request: request(event, group),
        kernel_event_id: Some(id),
        identity_error: None,
        lifecycle,
        status: "observed_unmultiplexed".into(),
        raw: Some(Raw {
            value: 0,
            time_enabled_ns: 10,
            time_running_ns: 10,
            event_id: id,
        }),
        tuple_assessment: Some(Assessment {
            status: "observed_unmultiplexed".into(),
            multiplexed: Some(false),
            scaled_estimate: None,
        }),
        read_bytes: Some(32),
        cleanup_succeeded: Some(true),
    }
}
fn collection() -> Collection {
    Collection {
        collector_version: "checked-native-pmu/v1".into(),
        policy: Policy {
            scope: "calling_thread".into(),
            cycles: "optional_with_standalone_fallback".into(),
        },
        owner_pid: 71,
        owner_tid: Some(73),
        context_checks: ["start", "finish"]
            .into_iter()
            .map(|stage| Context {
                stage: stage.into(),
                current: Identity {
                    pid: 71,
                    tid: Some(73),
                },
                status: "matched".into(),
            })
            .collect(),
        os: "linux".into(),
        architecture: "x86_64".into(),
        attempts: vec![Attempt {
            mode: "instructions_and_cycles_group".into(),
            events: vec![
                event("instructions", -1, 9, 0),
                event("cpu_cycles", 9, 10, 1),
            ],
            start_succeeded: true,
        }],
        selected_attempt: Some(0),
        whole_worker_coverage_verified: false,
        descendant_lifetimes_verified: false,
        complete_execution_verified: false,
        hardware_independently_verified: false,
        platform_admitted: false,
        clock_valid_for_comparison: false,
    }
}
fn check(c: &Collection) -> Vec<Observation> {
    native::validate(c, 71).unwrap().1
}

#[test]
fn leader_preparation_failures_keep_an_unattempted_cycles_placeholder() {
    for (errno, status, id_failure) in [
        (13, "permission_denied", false),
        (95, "unsupported_event", false),
        (42, "open_failed", false),
        (13, "identity_failed", true),
    ] {
        let mut c = collection();
        c.selected_attempt = None;
        c.attempts[0].start_succeeded = false;
        let leader = &mut c.attempts[0].events[0];
        leader.lifecycle = if id_failure {
            vec![call("open", 9), denied("event_id"), call("close", 0)]
        } else {
            vec![Call {
                operation: "open".into(),
                result: CallResult::Err(CallError {
                    errno: Some(errno),
                    message: "recorded diagnostic".into(),
                }),
            }]
        };
        leader.status = status.into();
        leader.kernel_event_id = None;
        leader.raw = None;
        leader.tuple_assessment = None;
        leader.read_bytes = None;
        leader.cleanup_succeeded = id_failure.then_some(true);
        let member = &mut c.attempts[0].events[1];
        member.request = request("cpu_cycles", -1);
        member.lifecycle.clear();
        member.status = "not_attempted".into();
        member.kernel_event_id = None;
        member.raw = None;
        member.tuple_assessment = None;
        member.read_bytes = None;
        member.cleanup_succeeded = None;
        assert!(check(&c).is_empty());
        c.attempts[0].events[1].request.group_fd = 9;
        assert!(native::validate(&c, 71).is_err());
    }
}

#[test]
fn optional_identity_failure_and_cleanup_gate_fallback() {
    for duplicate in [false, true] {
        for cleanup in [false, true] {
            let mut c = collection();
            c.selected_attempt = None;
            c.attempts[0].start_succeeded = false;
            let leader = &mut c.attempts[0].events[0];
            leader.lifecycle = vec![call("open", 9), call("event_id", 0), call("close", 0)];
            leader.status = "cancelled_before_start".into();
            leader.raw = None;
            leader.tuple_assessment = None;
            leader.read_bytes = None;
            let member = &mut c.attempts[0].events[1];
            member.lifecycle = vec![
                call("open", 10),
                if duplicate {
                    call("event_id", 0)
                } else {
                    denied("event_id")
                },
                if cleanup {
                    call("close", 0)
                } else {
                    denied("close")
                },
            ];
            member.status = "identity_failed".into();
            member.kernel_event_id = duplicate.then_some(0);
            member.identity_error = duplicate.then(|| "duplicate kernel event ID in group".into());
            member.raw = None;
            member.tuple_assessment = None;
            member.read_bytes = None;
            member.cleanup_succeeded = Some(cleanup);
            if cleanup {
                c.attempts.push(Attempt {
                    mode: "standalone_instructions_fallback".into(),
                    events: vec![event("instructions", -1, 9, 0)],
                    start_succeeded: true,
                });
                c.selected_attempt = Some(1);
            }
            assert_eq!(check(&c).len(), usize::from(cleanup));
            if cleanup {
                c.attempts.push(c.attempts[1].clone());
            } else {
                c.attempts.push(Attempt {
                    mode: "standalone_instructions_fallback".into(),
                    events: vec![event("instructions", -1, 9, 0)],
                    start_succeeded: true,
                });
                c.selected_attempt = Some(1);
            }
            assert!(native::validate(&c, 71).is_err());
        }
    }
}

#[test]
fn architecture_and_observations_remain_event_scoped() {
    let mut c = collection();
    c.architecture = "aarch64".into();
    assert_eq!(check(&c).len(), 2);
    let cycles = &mut c.attempts[0].events[1];
    cycles.lifecycle[2] = denied("read");
    cycles.status = "read_failed".into();
    cycles.read_bytes = None;
    cycles.raw = None;
    cycles.tuple_assessment = None;
    let observations = check(&c);
    assert_eq!(observations[0].status, "scoped_raw");
    assert_eq!(observations[1].status, "unavailable");
}

#[test]
fn strict_wire_rejects_duplicate_escaped_keys_numeric_tokens_and_result_tags() {
    let m = record(Phase::CurveConstruction, true).4.unwrap().0;
    let raw = serde_json::to_string(&m).unwrap();
    for changed in [
        raw.replacen("\"ordinal\":0", "\"ordinal\":0,\"ord\\u0069nal\":0", 1),
        raw.replacen("\"ordinal\":0", "\"ordinal\":0.0", 1),
        raw.replacen("\"owner_pid\":71", "\"owner_pid\":4294967296", 1),
        format!("{raw} null"),
        raw.replacen(
            "\"Ok\":9",
            "\"Ok\":9,\"Err\":{\"errno\":null,\"message\":\"x\"}",
            1,
        ),
        raw.replacen("{\"Ok\":9}", "{}", 1),
    ] {
        assert!(io::typed::<Metrics>(changed.as_bytes(), &mut Budget::new(LIMIT)).is_err());
    }
    for (object, key) in [
        ("", "returned_error"),
        ("", "usage_after"),
        ("", "whole_worker_retired_instructions"),
        ("/collection", "owner_tid"),
        ("/collection", "selected_attempt"),
        ("/collection/context_checks/0/current", "tid"),
        ("/collection/attempts/0/events/0", "kernel_event_id"),
        ("/collection/attempts/0/events/0", "identity_error"),
        ("/collection/attempts/0/events/0", "raw"),
        ("/collection/attempts/0/events/0", "tuple_assessment"),
        ("/collection/attempts/0/events/0", "read_bytes"),
        ("/collection/attempts/0/events/0", "cleanup_succeeded"),
        (
            "/collection/attempts/0/events/0/tuple_assessment",
            "multiplexed",
        ),
        (
            "/collection/attempts/0/events/0/tuple_assessment",
            "scaled_estimate",
        ),
        ("/observations/0", "raw"),
        ("/observations/0", "estimate"),
    ] {
        let mut value = serde_json::to_value(&m).unwrap();
        value
            .pointer_mut(object)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove(key);
        assert!(
            io::typed::<Metrics>(
                &serde_json::to_vec(&value).unwrap(),
                &mut Budget::new(LIMIT)
            )
            .is_err(),
            "missing {object}/{key}"
        );
    }
}
fn binding() -> Binding {
    Binding {
        schema: "crypto.ml-dataset.phase-attempt-binding/v1".into(),
        run_sha256: "a".repeat(64),
        plan_sha256: "b".repeat(64),
        protocol_sha256: "c".repeat(64),
        config_sha256: "d".repeat(64),
        source_commit: "e".repeat(40),
        binary_sha256: "f".repeat(64),
        cell_index: 0,
        attempt_sha256: "0".repeat(64),
    }
}
fn record(p: Phase, end: bool) -> Recorded {
    let start = Start {
        schema: "crypto.ml-dataset.phase-start/v2".into(),
        phase: p,
        ordinal: p.ordinal(),
        binding: binding(),
        policy_sha256: POLICY_HASH.into(),
        recording_pid: 71,
        usage_before: None,
        counter_boundary: "durable_start_before_open".into(),
    };
    let value = serde_json::to_value(&start).unwrap();
    let sha = crate::common::digest(&serde_json::to_vec(&start).unwrap());
    let metrics = if end {
        let c = collection();
        let m = Metrics {
            schema: "crypto.ml-dataset.phase-metrics/v2".into(),
            phase: p,
            ordinal: p.ordinal(),
            binding: binding(),
            policy_sha256: POLICY_HASH.into(),
            start_sha256: sha.clone(),
            returned_ok: true,
            returned_error: None,
            counter_start_succeeded: true,
            observations: check(&c),
            collection: c,
            usage_after: None,
            elapsed_ns_diagnostic: "0".into(),
            training_cost_eligible: false,
            whole_worker_retired_instructions: None,
            whole_worker_coverage_verified: false,
            independently_verified: false,
            platform_admitted: false,
            clock_valid_for_comparison: false,
            interrupted_phase_readback: false,
            consumer_v2_admitted: false,
        };
        let v = serde_json::to_value(&m).unwrap();
        Some((m, v, "1".repeat(64)))
    } else {
        None
    };
    (p, start, value, sha, metrics)
}
fn context(records: Vec<Recorded>) -> Result<Option<CheckedPhaseAudit>> {
    validate_context(binding(), records, vec![], &Budget::new(LIMIT))
        .map(|context| Some(context.sealed()))
}

#[test]
fn zero_kernel_id_and_zero_raw_are_valid_without_hardware_claims() {
    let obs = check(&collection());
    assert_eq!(obs.len(), 2);
    assert_eq!(obs[0].raw, Some(0));
    assert_eq!(obs[0].status, "scoped_raw");
    let a = context(vec![record(Phase::CurveConstruction, true)])
        .unwrap()
        .unwrap();
    for field in [
        "independently_verified",
        "hardware_independently_verified",
        "complete_execution_verified",
        "training_cost_eligible",
        "partial_snapshot_chain_verified",
    ] {
        assert_eq!(a.summary[field], false);
    }
    assert!(a.summary["whole_worker_instruction_count"].is_null());
}
#[test]
fn tuple_status_matrix_has_exact_integer_projection() {
    for (value, enabled, running, id, status, multiplexed, estimate, projected) in [
        (
            7,
            10,
            3,
            0,
            "observed_multiplexed_estimate",
            Some(true),
            Some(23),
            "scoped_estimate",
        ),
        (
            u64::MAX,
            u64::MAX,
            1,
            0,
            "estimate_overflow",
            Some(true),
            None,
            "unavailable",
        ),
        (7, 10, 0, 0, "not_running", None, None, "unavailable"),
        (7, 10, 11, 0, "invalid_tuple", None, None, "unavailable"),
        (7, 10, 10, 9, "identity_mismatch", None, None, "unavailable"),
    ] {
        let mut c = collection();
        let e = &mut c.attempts[0].events[0];
        e.raw = Some(Raw {
            value,
            time_enabled_ns: enabled,
            time_running_ns: running,
            event_id: id,
        });
        e.status = status.into();
        e.tuple_assessment = Some(Assessment {
            status: status.into(),
            multiplexed,
            scaled_estimate: estimate,
        });
        let obs = check(&c);
        assert_eq!(obs[0].status, projected);
        assert_eq!(obs[0].estimate, estimate);
        c.attempts[0].events[0]
            .tuple_assessment
            .as_mut()
            .unwrap()
            .scaled_estimate = Some(99);
        assert!(native::validate(&c, 71).is_err());
    }
}
#[test]
fn failed_controls_and_cleanup_never_project_counts() {
    for operation in ["reset_group", "enable_group", "disable_group", "close"] {
        let mut c = collection();
        let e = &mut c.attempts[0].events[0];
        *e.lifecycle
            .iter_mut()
            .find(|x| x.operation == operation)
            .unwrap() = denied(operation);
        if operation == "close" {
            e.cleanup_succeeded = Some(false);
        } else {
            e.status = "control_failed".into();
            c.attempts[0].events[1].status = "control_failed".into();
            if operation != "disable_group" {
                c.attempts[0].start_succeeded = false;
            }
            if operation == "reset_group" {
                c.attempts[0].events[0]
                    .lifecycle
                    .retain(|x| x.operation != "enable_group");
            }
        }
        let obs = check(&c);
        assert!(obs
            .iter()
            .all(|o| o.status == "unavailable" && o.raw.is_none()));
    }
}
#[test]
fn failed_and_short_reads_are_preserved() {
    for n in [None, Some(0), Some(31), Some(33)] {
        let mut c = collection();
        let e = &mut c.attempts[0].events[0];
        *e.lifecycle
            .iter_mut()
            .find(|x| x.operation == "read")
            .unwrap() = n.map_or_else(|| denied("read"), |n| call("read", n));
        e.status = if n.is_none() {
            "read_failed"
        } else {
            "short_read"
        }
        .into();
        e.raw = None;
        e.tuple_assessment = None;
        e.read_bytes = n.map(|n| n as usize);
        assert_eq!(check(&c)[0].status, "unavailable");
    }
}
#[test]
fn unsupported_platform_is_explicit_and_has_no_projection() {
    let mut c = collection();
    c.os = "other".into();
    c.architecture = "powerpc64".into();
    c.owner_tid = None;
    c.selected_attempt = None;
    c.attempts[0].start_succeeded = false;
    for x in &mut c.context_checks {
        x.current.tid = None;
        x.status = "unavailable".into();
    }
    for e in &mut c.attempts[0].events {
        e.request = request(&e.request.event, -1);
        e.request.byte_order = "big".into();
        e.request.attr_bytes_native_endian.fill(0);
        e.request.attr_bytes_native_endian[4..8].copy_from_slice(&112u32.to_be_bytes());
        for (offset, v) in [(8, e.request.config), (32, 7), (40, 97)] {
            e.request.attr_bytes_native_endian[offset..offset + 8]
                .copy_from_slice(&v.to_be_bytes());
        }
        e.lifecycle.clear();
        e.status = "unsupported_platform".into();
        e.kernel_event_id = None;
        e.raw = None;
        e.tuple_assessment = None;
        e.read_bytes = None;
        e.cleanup_succeeded = None;
    }
    assert!(check(&c).is_empty());
    c.attempts[0].events[0].request.attr_bytes_native_endian[4] = 112;
    assert!(native::validate(&c, 71).is_err());
}
#[test]
fn start_and_finish_context_failures_preserve_lifecycle() {
    for stage in [0, 1] {
        let mut c = collection();
        c.context_checks[stage].current.pid = 99;
        c.context_checks[stage].status = "mismatch".into();
        c.attempts[0].start_succeeded = false;
        for e in &mut c.attempts[0].events {
            e.status = "context_mismatch".into();
            e.raw = None;
            e.tuple_assessment = None;
            e.read_bytes = None;
            e.lifecycle.retain(|x| {
                matches!(x.operation.as_str(), "open" | "event_id" | "close")
                    || (stage == 1
                        && matches!(x.operation.as_str(), "reset_group" | "enable_group"))
            });
        }
        assert!(check(&c).iter().all(|o| o.status == "unavailable"));
    }
}
#[test]
fn standalone_fallback_reuses_closed_fd_and_rejects_missing_retry() {
    let mut c = collection();
    let leader = &mut c.attempts[0].events[0];
    leader
        .lifecycle
        .retain(|x| matches!(x.operation.as_str(), "open" | "event_id" | "close"));
    leader.status = "cancelled_before_start".into();
    leader.raw = None;
    leader.read_bytes = None;
    leader.tuple_assessment = None;
    let member = &mut c.attempts[0].events[1];
    member.lifecycle = vec![denied("open")];
    member.status = "permission_denied".into();
    member.raw = None;
    member.tuple_assessment = None;
    member.read_bytes = None;
    member.cleanup_succeeded = None;
    member.kernel_event_id = None;
    c.attempts[0].start_succeeded = false;
    c.selected_attempt = Some(1);
    c.attempts.push(Attempt {
        mode: "standalone_instructions_fallback".into(),
        events: vec![event("instructions", -1, 9, 0)],
        start_succeeded: true,
    });
    let obs = check(&c);
    assert_eq!(obs.len(), 1);
    assert_eq!(obs[0].attempt_index, 1);
    c.attempts.pop();
    assert!(native::validate(&c, 71).is_err());
}
#[test]
fn strict_native_rejects_reordered_calls_aliases_unknown_fields_and_claims() {
    let c = collection();
    for mutate in [0, 1, 2, 3, 4, 5, 6] {
        let mut v = serde_json::to_value(&c).unwrap();
        match mutate {
            0 => {
                v["attempts"][0]["events"][0]["lifecycle"]
                    .as_array_mut()
                    .unwrap()
                    .swap(2, 3);
            }
            1 => v["attempts"][0]["events"][1]["lifecycle"][0]["result"]["Ok"] = json!(9),
            2 => {
                v["attempts"][0]["events"][0]["request"]["attr_bytes_native_endian"][111] = json!(1)
            }
            3 => v["complete_execution_verified"] = json!(true),
            4 => v["unknown"] = json!(null),
            5 => {
                v.as_object_mut().unwrap().remove("owner_tid");
            }
            _ => v["attempts"][0]["events"][0]["lifecycle"][1]["result"]["Ok"] = json!(1),
        }
        match serde_json::from_value::<Collection>(v) {
            Ok(c) => assert!(native::validate(&c, 71).is_err(), "mutation {mutate}"),
            Err(_) => assert!(matches!(mutate, 4 | 5)),
        }
    }
}
#[test]
fn all_prefixes_start_only_and_operation_errors_have_explicit_states() {
    for completed in 0..=4 {
        let mut records: Vec<_> = Phase::ALL[..completed]
            .iter()
            .map(|p| record(*p, true))
            .collect();
        let a = context(records.clone()).unwrap().unwrap();
        assert_eq!(a.summary["supplied_ends"], completed);
        if completed < 4 {
            records.push(record(Phase::ALL[completed], false));
            let a = context(records).unwrap().unwrap();
            assert_eq!(a.summary["prefix_state"], "interrupted_or_unfinished");
        }
    }
    let mut r = record(Phase::CurveConstruction, true);
    let m = &mut r.4.as_mut().unwrap().0;
    m.returned_ok = false;
    m.returned_error = Some(ErrorSummary {
        message_preview: "é".into(),
        message_bytes: 2,
        message_sha256: crate::common::digest("é".as_bytes()),
        truncated: false,
    });
    assert_eq!(
        context(vec![r.clone()]).unwrap().unwrap().summary["prefix_state"],
        "operation_error"
    );
    assert!(context(vec![r, record(Phase::KnownAnswerReplay, false)]).is_err());
    assert!(context(vec![record(Phase::KnownAnswerReplay, false)]).is_err());
    assert!(context(vec![
        record(Phase::CurveConstruction, false),
        record(Phase::KnownAnswerReplay, true)
    ])
    .is_err());
}
#[test]
fn operation_error_bytes_and_null_fields_are_strict() {
    let error = ErrorSummary {
        message_preview: "é".into(),
        message_bytes: 3,
        message_sha256: "f".repeat(64),
        truncated: true,
    };
    assert_eq!(
        error_summary(&Some(error.clone())).unwrap(),
        "truncated_suffix_unverified"
    );
    let mut bad = error;
    bad.message_bytes = 2;
    assert!(error_summary(&Some(bad)).is_err());
    let r = record(Phase::CurveConstruction, true);
    let mut value = serde_json::to_value(r.4.unwrap().0).unwrap();
    value.as_object_mut().unwrap().remove("returned_error");
    assert!(serde_json::from_value::<Metrics>(value).is_err());
}
#[test]
fn legacy_marker_is_hash_bound_and_preserves_opaque_bytes() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("telemetry")).unwrap();
    let samples: [(&[u8], bool); 8] = [
        (b"not JSON", false),
        (b"{\"schema\":\"crypto.ml-dataset.phase-start/v1\"}", false),
        (
            b"{\"schema\":\"crypto.ml-dataset.phase-start/v2\"}junk",
            true,
        ),
        (
            br#"{"sch\u0065ma":"crypto.ml-dataset.phase-start/v\u0032"}"#,
            true,
        ),
        (
            b"{\"schema\":\"crypto.ml-dataset.phase-start/v2\",\"schema\":\"v1\"}",
            true,
        ),
        (
            b"{\"schema\":\"v1\",\"schema\":\"crypto.ml-dataset.phase-metrics/v2\"}",
            true,
        ),
        (
            b"{\"nested\":{\"schema\":\"crypto.ml-dataset.phase-start/v2\"}}",
            false,
        ),
        (
            b"{\"schema\":42,\"schema\":\"crypto.ml-dataset.phase-start/v2\"}",
            false,
        ),
    ];
    for (raw, expected) in samples {
        std::fs::write(dir.path().join("telemetry/diagnostic.json"), raw).unwrap();
        let a = json!({"path":"telemetry/diagnostic.json","bytes":raw.len(),"sha256":crate::common::digest(raw)});
        assert_eq!(
            io::legacy_has_v2(dir.path(), &a, &mut Budget::new(LIMIT)).unwrap(),
            expected
        );
        let mut tampered = a;
        tampered["sha256"] = json!("0".repeat(64));
        assert!(io::legacy_has_v2(dir.path(), &tampered, &mut Budget::new(LIMIT)).is_err());
    }
    let raw = format!(
        "{{\"diagnostic\":\"{}\",\"schema\":\"crypto.ml-dataset.phase-start/v2\"}}",
        "x".repeat(500000)
    );
    std::fs::write(dir.path().join("telemetry/diagnostic.json"), &raw).unwrap();
    let a = json!({"path":"telemetry/diagnostic.json","bytes":raw.len(),"sha256":crate::common::digest(raw.as_bytes())});
    assert!(io::legacy_has_v2(dir.path(), &a, &mut Budget::new(LIMIT)).unwrap());
}
