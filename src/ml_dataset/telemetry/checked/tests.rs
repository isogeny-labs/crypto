use super::*;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT: AtomicU64 = AtomicU64::new(0);

struct Temp(std::path::PathBuf);
impl Temp {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "checked-phase-mock-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        install_policy(&path).unwrap();
        Self(path)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn binding() -> AttemptBinding {
    AttemptBinding {
        schema: "crypto.ml-dataset.attempt-binding/v1".into(),
        run_sha256: "1".repeat(64),
        plan_sha256: "2".repeat(64),
        protocol_sha256: "3".repeat(64),
        config_sha256: "4".repeat(64),
        source_commit: "5".repeat(40),
        binary_sha256: "6".repeat(64),
        cell_index: 0,
        attempt_sha256: "7".repeat(64),
    }
}
fn call(operation: &str, value: i64) -> Call {
    Call {
        operation: operation.into(),
        result: CallResult::Ok(value),
    }
}
fn event(kind: &str, fd: i32, id: u64, value: u64) -> Event {
    let config = u64::from(kind == "instructions");
    let flags = if fd == -1 { 97u64 } else { 96 };
    let mut attr = vec![0; 112];
    attr[4..8].copy_from_slice(&112u32.to_le_bytes());
    attr[8..16].copy_from_slice(&config.to_le_bytes());
    attr[32..40].copy_from_slice(&7u64.to_le_bytes());
    attr[40..48].copy_from_slice(&flags.to_le_bytes());
    let mut lifecycle = vec![
        call("open", if fd == -1 { 10 } else { 11 }),
        call("event_id", 0),
    ];
    if fd == -1 {
        lifecycle.extend([
            call("reset_group", 0),
            call("enable_group", 0),
            call("disable_group", 0),
        ]);
    }
    lifecycle.extend([call("read", 32), call("close", 0)]);
    Event {
        request: Request {
            event: kind.into(),
            perf_type: 0,
            config,
            attr_size: 112,
            read_format: 7,
            attr_flags: flags,
            attr_bytes_native_endian: attr,
            byte_order: "little".into(),
            pid: 0,
            cpu: -1,
            group_fd: fd,
            syscall_flags: 8,
        },
        kernel_event_id: Some(id),
        identity_error: None,
        lifecycle,
        status: "observed_unmultiplexed".into(),
        raw: Some(Raw {
            value,
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
        owner_pid: std::process::id(),
        owner_tid: Some(1),
        context_checks: ["start", "finish"]
            .map(|stage| Context {
                stage: stage.into(),
                current: Identity {
                    pid: std::process::id(),
                    tid: Some(1),
                },
                status: "matched".into(),
            })
            .to_vec(),
        os: "linux".into(),
        architecture: "x86_64".into(),
        attempts: vec![Attempt {
            mode: "instructions_and_cycles_group".into(),
            events: vec![
                event("instructions", -1, 101, 0),
                event("cpu_cycles", 10, 102, 100),
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

/// Software-only producer records for cross-module persistence tests.
pub(crate) fn write_phase(root: &Path, binding: &AttemptBinding, phase: Phase, start_only: bool) {
    let dropped = Rc::new(Cell::new(0));
    let result = measured_with(
        root,
        &mut vec![],
        binding,
        phase,
        || Mock {
            record: collection(),
            dropped,
        },
        || Ok(()),
        |path, role, bytes| {
            if start_only && !path.ends_with("-start.json") {
                return Err("mock interrupted before end persistence".into());
            }
            storage::write_bytes(root, path, role, "json", bytes)
        },
    );
    assert_eq!(result.is_err(), start_only);
}
fn unsupported() -> Collection {
    let mut c = collection();
    c.os = "unsupported_fixture_os".into();
    c.selected_attempt = None;
    c.attempts[0].start_succeeded = false;
    c.attempts[0].events[1] = event("cpu_cycles", -1, 102, 100);
    for e in &mut c.attempts[0].events {
        e.lifecycle.clear();
        e.kernel_event_id = None;
        e.raw = None;
        e.tuple_assessment = None;
        e.read_bytes = None;
        e.cleanup_succeeded = None;
        e.status = "unsupported_platform".into();
    }
    c
}
struct Mock {
    record: Collection,
    dropped: Rc<Cell<u32>>,
}
impl Counter for Mock {
    fn start(&mut self) -> bool {
        self.record.selected_attempt.is_some()
    }
    fn finish(self) -> Result<Collection, String> {
        Ok(self.record.clone())
    }
}
impl Drop for Mock {
    fn drop(&mut self) {
        self.dropped.set(self.dropped.get() + 1);
    }
}

#[test]
fn fixed_policy_and_required_nullable_fields_are_strict() {
    assert_eq!(POLICY_BYTES.len(), 785);
    assert_eq!(digest(POLICY_BYTES), POLICY_HASH);
    let t = Temp::new();
    assert!(policy_present(&t.0).unwrap());
    fs::write(t.0.join(POLICY_PATH), [POLICY_BYTES, b" "].concat()).unwrap();
    assert!(policy_present(&t.0).is_err());
    let mut value = serde_json::to_value(collection()).unwrap();
    value.as_object_mut().unwrap().remove("owner_tid");
    assert!(serde_json::from_value::<Collection>(value).is_err());
    let mut value = serde_json::to_value(collection()).unwrap();
    value["extra"] = true.into();
    assert!(serde_json::from_value::<Collection>(value).is_err());
    assert!(
        serde_json::from_str::<CallResult>(r#"{"Ok":0,"Err":{"errno":null,"message":"x"}}"#)
            .is_err()
    );
}

#[test]
fn scoped_zero_estimate_unavailable_and_overflow_stay_distinct() {
    let mut c = collection();
    let (_, o) = native::validate(&c, std::process::id()).unwrap();
    assert_eq!(o[0].raw, Some(0));
    assert_eq!(o[0].status, "scoped_raw");
    let e = &mut c.attempts[0].events[0];
    e.raw.as_mut().unwrap().time_running_ns = 5;
    e.raw.as_mut().unwrap().value = 7;
    e.status = "observed_multiplexed_estimate".into();
    e.tuple_assessment = Some(Assessment {
        status: e.status.clone(),
        multiplexed: Some(true),
        scaled_estimate: Some(14),
    });
    let (_, o) = native::validate(&c, std::process::id()).unwrap();
    assert_eq!(o[0].estimate, Some(14));
    assert_eq!(o[1].status, "scoped_raw");
    let e = &mut c.attempts[0].events[0];
    e.raw.as_mut().unwrap().value = u64::MAX;
    e.status = "estimate_overflow".into();
    e.tuple_assessment.as_mut().unwrap().status = e.status.clone();
    e.tuple_assessment.as_mut().unwrap().scaled_estimate = None;
    assert_eq!(
        native::validate(&c, std::process::id()).unwrap().1[0].status,
        "unavailable"
    );
    let e = &mut c.attempts[0].events[0];
    e.raw.as_mut().unwrap().time_running_ns = 0;
    e.status = "not_running".into();
    e.tuple_assessment = Some(Assessment {
        status: e.status.clone(),
        multiplexed: None,
        scaled_estimate: None,
    });
    assert_eq!(
        native::validate(&c, std::process::id()).unwrap().1[0].raw,
        None
    );
    assert_eq!(
        native::validate(&unsupported(), std::process::id()).unwrap(),
        (false, vec![])
    );
}

#[test]
fn preparation_identity_failure_and_optional_fallback_are_retained() {
    let mut c = collection();
    let mut fallback = c.attempts[0].clone();
    fallback.mode = "standalone_instructions_fallback".into();
    fallback.events.truncate(1);
    c.attempts[0].start_succeeded = false;
    let leader = &mut c.attempts[0].events[0];
    leader.lifecycle = vec![call("open", 10), call("event_id", 0), call("close", 0)];
    leader.status = "cancelled_before_start".into();
    leader.raw = None;
    leader.tuple_assessment = None;
    leader.read_bytes = None;
    let member = &mut c.attempts[0].events[1];
    member.lifecycle = vec![
        call("open", 11),
        Call {
            operation: "event_id".into(),
            result: CallResult::Err(CallError {
                errno: Some(5),
                message: "mock ID failure".into(),
            }),
        },
        call("close", 0),
    ];
    member.status = "identity_failed".into();
    member.kernel_event_id = None;
    member.raw = None;
    member.tuple_assessment = None;
    member.read_bytes = None;
    c.attempts.push(fallback);
    c.selected_attempt = Some(1);
    assert_eq!(native::validate(&c, std::process::id()).unwrap().1.len(), 1);
    c.attempts[0].events[0].cleanup_succeeded = Some(false);
    c.attempts[0].events[0].lifecycle[2].result = CallResult::Err(CallError {
        errno: Some(5),
        message: "mock close failure".into(),
    });
    assert!(native::validate(&c, std::process::id()).is_err());
}

#[test]
fn lifecycle_and_projected_measurements_reject_tampering() {
    let original = collection();
    for mutate in [
        |c: &mut Collection| c.attempts[0].events[0].raw.as_mut().unwrap().event_id = 999,
        |c: &mut Collection| c.attempts[0].events[0].request.attr_flags = 99,
        |c: &mut Collection| c.attempts[0].events[1].request.group_fd = 55,
        |c: &mut Collection| c.attempts[0].events[0].cleanup_succeeded = Some(false),
        |c: &mut Collection| c.context_checks[1].current.tid = Some(99),
        |c: &mut Collection| {
            c.attempts[0].events[0].lifecycle[3].operation = "repeated_start".into()
        },
        |c: &mut Collection| c.attempts[0].events[0].status = "prepared".into(),
        |c: &mut Collection| {
            c.attempts[0].events[0]
                .raw
                .as_mut()
                .unwrap()
                .time_running_ns = 11
        },
        |c: &mut Collection| c.platform_admitted = true,
    ] {
        let mut c = original.clone();
        mutate(&mut c);
        assert!(native::validate(&c, std::process::id()).is_err());
    }
}

#[test]
fn wrapper_start_failure_op_once_and_original_errors_are_preserved() {
    let t = Temp::new();
    let opened = Cell::new(0);
    let calls = Cell::new(0);
    let dropped = Rc::new(Cell::new(0));
    let open = || {
        opened.set(opened.get() + 1);
        Mock {
            record: unsupported(),
            dropped: dropped.clone(),
        }
    };
    let result = measured_with::<(), _>(
        &t.0,
        &mut vec![],
        &binding(),
        Phase::CurveConstruction,
        open,
        || {
            calls.set(calls.get() + 1);
            Ok(())
        },
        |_, _, _| Err("mock start failure".into()),
    );
    assert!(result.unwrap_err().contains("mock start failure"));
    assert_eq!(opened.get(), 0);
    assert_eq!(calls.get(), 0);
    let result = measured_with::<(), _>(
        &t.0,
        &mut vec![],
        &binding(),
        Phase::CurveConstruction,
        open,
        || {
            calls.set(calls.get() + 1);
            Err("original operation failure".into())
        },
        |path, role, bytes| storage::write_bytes(&t.0, path, role, "json", bytes),
    );
    assert_eq!(result.unwrap_err(), "original operation failure");
    assert_eq!(opened.get(), 1);
    assert_eq!(calls.get(), 1);
    assert_eq!(dropped.get(), 1);
    let files = context_files(&t.0).unwrap();
    assert_eq!(validate_context(&binding(), &files).unwrap().len(), 3);
    let metrics: Metrics =
        serde_json::from_slice(&files[&Phase::CurveConstruction.path(false)]).unwrap();
    assert_eq!(
        metrics.returned_error.unwrap().message_preview,
        "original operation failure"
    );
    assert!(!metrics.counter_start_succeeded);
    assert!(metrics.observations.is_empty());
}

#[test]
fn panic_and_end_write_failure_keep_start_only_and_cleanup() {
    let t = Temp::new();
    let dropped = Rc::new(Cell::new(0));
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = measured_with::<(), _>(
            &t.0,
            &mut vec![],
            &binding(),
            Phase::CurveConstruction,
            || Mock {
                record: collection(),
                dropped: dropped.clone(),
            },
            || panic!("mock operation panic"),
            |path, role, bytes| storage::write_bytes(&t.0, path, role, "json", bytes),
        );
    }));
    assert!(caught.is_err());
    assert_eq!(dropped.get(), 1);
    assert_eq!(
        validate_context(&binding(), &context_files(&t.0).unwrap())
            .unwrap()
            .len(),
        2
    );
    let t = Temp::new();
    let result = measured_with::<(), _>(
        &t.0,
        &mut vec![],
        &binding(),
        Phase::CurveConstruction,
        || Mock {
            record: collection(),
            dropped: dropped.clone(),
        },
        || Err("solver failed".into()),
        |path, role, bytes| {
            if path.ends_with("-start.json") {
                storage::write_bytes(&t.0, path, role, "json", bytes)
            } else {
                Err("disk failed".into())
            }
        },
    );
    let error = result.unwrap_err();
    assert!(error.contains("solver failed") && error.contains("disk failed"));
    assert_eq!(
        validate_context(&binding(), &context_files(&t.0).unwrap())
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn streamed_error_preview_is_utf8_bounded_and_hashes_full_display() {
    let message = format!("{}😀suffix", "a".repeat(4094));
    let summary = summarize(&message).unwrap();
    assert_eq!(summary.message_preview.len(), 4094);
    assert!(summary.truncated);
    assert_eq!(summary.message_bytes, message.len() as u64);
    assert_eq!(summary.message_sha256, digest(message.as_bytes()));
    struct Chunks;
    impl fmt::Display for Chunks {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for _ in 0..10000 {
                f.write_str("abcdefgh")?;
            }
            Ok(())
        }
    }
    let summary = summarize(&Chunks).unwrap();
    assert_eq!(summary.message_preview.len(), 4096);
    assert_eq!(summary.message_bytes, 80000);
    assert_eq!(
        summary.message_sha256,
        digest("abcdefgh".repeat(10000).as_bytes())
    );
}

#[test]
fn phase_dependency_order_and_start_binding_are_enforced() {
    let t = Temp::new();
    let dropped = Rc::new(Cell::new(0));
    measured_with(
        &t.0,
        &mut vec![],
        &binding(),
        Phase::CurveConstruction,
        || Mock {
            record: collection(),
            dropped: dropped.clone(),
        },
        || Ok(()),
        |path, role, bytes| storage::write_bytes(&t.0, path, role, "json", bytes),
    )
    .unwrap();
    let original = context_files(&t.0).unwrap();
    let mut files = original.clone();
    files.remove(&Phase::CurveConstruction.path(true));
    assert!(validate_context(&binding(), &files).is_err());
    let mut files = original.clone();
    let mut m: Metrics =
        serde_json::from_slice(&files[&Phase::CurveConstruction.path(false)]).unwrap();
    m.start_sha256 = "0".repeat(64);
    files.insert(
        Phase::CurveConstruction.path(false),
        encode(&m, METRICS_MAX).unwrap(),
    );
    assert!(validate_context(&binding(), &files).is_err());
    let mut altered = binding();
    altered.cell_index = 1;
    assert!(validate_context(&altered, &original).is_err());
    assert!(measured_with(
        &t.0,
        &mut vec![],
        &binding(),
        Phase::Solver,
        || Mock {
            record: collection(),
            dropped
        },
        || Ok(()),
        |path, role, bytes| storage::write_bytes(&t.0, path, role, "json", bytes)
    )
    .is_err());
}
