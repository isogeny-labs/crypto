use super::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    next_fd: i32,
    faults: Vec<(Operation, i32, i32)>,
    requests: Vec<(i32, EventRequest)>,
    operations: Vec<(Operation, i32)>,
    reads: BTreeMap<i32, Vec<u8>>,
    ids: BTreeMap<i32, u64>,
    unsupported: bool,
    panic_id: bool,
    panic_once: Option<(Operation, i32)>,
    pid: u32,
    tid: Option<u32>,
}

#[derive(Clone)]
struct Mock(Rc<RefCell<State>>);

impl Mock {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(State {
            next_fd: 3,
            pid: 123,
            tid: Some(124),
            ..State::default()
        })))
    }
    fn fail(&self, operation: Operation, fd: i32, errno: i32) {
        self.0.borrow_mut().faults.push((operation, fd, errno));
    }
    fn record(&self, operation: Operation, fd: i32) -> Result<(), CallError> {
        let mut state = self.0.borrow_mut();
        state.operations.push((operation, fd));
        if state.panic_once == Some((operation, fd)) {
            state.panic_once = None;
            drop(state);
            panic!("one-time mock {operation:?} unwind");
        }
        if let Some((_, _, errno)) = state
            .faults
            .iter()
            .find(|(op, n, _)| *op == operation && *n == fd)
        {
            Err(CallError {
                errno: Some(*errno),
                message: format!("mock {operation:?} errno {errno}"),
            })
        } else {
            Ok(())
        }
    }
    fn count(&self, operation: Operation, fd: i32) -> usize {
        self.0
            .borrow()
            .operations
            .iter()
            .filter(|x| **x == (operation, fd))
            .count()
    }
}

impl Calls for Mock {
    fn supported(&self) -> bool {
        !self.0.borrow().unsupported
    }
    fn identity(&self) -> (u32, Option<u32>) {
        let state = self.0.borrow();
        (state.pid, state.tid)
    }
    fn open(&mut self, request: &EventRequest) -> Result<i32, CallError> {
        let fd = self.0.borrow().next_fd;
        self.0.borrow_mut().next_fd += 1;
        self.0.borrow_mut().requests.push((fd, request.clone()));
        self.record(Operation::Open, fd)?;
        Ok(fd)
    }
    fn event_id(&mut self, fd: i32) -> Result<u64, CallError> {
        self.record(Operation::EventId, fd)?;
        assert!(!self.0.borrow().panic_id, "mock ID-query panic");
        Ok(self
            .0
            .borrow()
            .ids
            .get(&fd)
            .copied()
            .unwrap_or(fd as u64 + 100))
    }
    fn control(&mut self, fd: i32, operation: Operation) -> Result<(), CallError> {
        self.record(operation, fd)
    }
    fn read(&mut self, fd: i32) -> Result<Vec<u8>, CallError> {
        self.record(Operation::Read, fd)?;
        Ok(self
            .0
            .borrow()
            .reads
            .get(&fd)
            .cloned()
            .unwrap_or_else(|| bytes(7, 20, 20, fd as u64 + 100)))
    }
    fn close(&mut self, fd: i32) -> Result<(), CallError> {
        self.record(Operation::Close, fd)
    }
}

fn bytes(value: u64, enabled: u64, running: u64, id: u64) -> Vec<u8> {
    [value, enabled, running, id]
        .into_iter()
        .flat_map(u64::to_ne_bytes)
        .collect()
}

fn policy(cycles: CyclesPolicy) -> CounterPolicy {
    CounterPolicy {
        scope: TaskScope::CallingThreadAndNewDescendants,
        cycles,
    }
}

fn run(mock: &Mock, cycles: CyclesPolicy) -> CollectionRecord {
    let mut collector = Collector::open(mock.clone(), policy(cycles));
    collector.start();
    collector.finish()
}

#[test]
fn complete_attributes_ids_and_scope_are_recorded_without_total_claims() {
    let mock = Mock::new();
    let report = run(&mock, CyclesPolicy::Required);
    assert_eq!(report.selected_attempt, Some(0));
    assert_eq!((report.owner_pid, report.owner_tid), (123, Some(124)));
    assert_eq!(
        report.policy.scope,
        TaskScope::CallingThreadAndNewDescendants
    );
    let events = &report.attempts[0].events;
    for (index, event) in events.iter().enumerate() {
        let request = &event.request;
        assert_eq!(
            (request.perf_type, request.attr_size, request.read_format),
            (0, 112, 7)
        );
        assert_eq!(
            (request.pid, request.cpu, request.syscall_flags),
            (0, -1, 8)
        );
        assert_eq!(request.config, if index == 0 { 1 } else { 0 });
        assert_eq!(request.attr_flags, if index == 0 { 99 } else { 98 });
        assert_eq!(request.group_fd, if index == 0 { -1 } else { 3 });
        assert_eq!(request.attr_bytes_native_endian.len(), 112);
        let mut expected = vec![0u8; 112];
        expected[4..8].copy_from_slice(&112u32.to_ne_bytes());
        expected[8..16].copy_from_slice(&request.config.to_ne_bytes());
        expected[32..40].copy_from_slice(&7u64.to_ne_bytes());
        expected[40..48].copy_from_slice(&request.attr_flags.to_ne_bytes());
        assert_eq!(request.attr_bytes_native_endian, expected);
        assert_eq!(event.kernel_event_id, Some(index as u64 + 103));
        assert_eq!(event.status, EventStatus::ObservedUnmultiplexed);
        assert_eq!(event.read_bytes, Some(32));
        assert_eq!(event.cleanup_succeeded, Some(true));
        assert_eq!(event.tuple_assessment.unwrap().scaled_estimate, None);
    }
    let json = serde_json::to_value(report).unwrap();
    for key in [
        "whole_worker_coverage_verified",
        "descendant_lifetimes_verified",
        "complete_execution_verified",
        "hardware_independently_verified",
        "platform_admitted",
        "clock_valid_for_comparison",
    ] {
        assert_eq!(json[key], false, "{key}");
    }
}

#[test]
fn calling_thread_policy_never_silently_enables_inheritance() {
    let mock = Mock::new();
    let collector = Collector::open(
        mock.clone(),
        CounterPolicy {
            scope: TaskScope::CallingThread,
            cycles: CyclesPolicy::Disabled,
        },
    );
    let report = collector.finish();
    assert_eq!(report.attempts[0].events[0].request.attr_flags, 97);
    assert_eq!(report.attempts[0].events[0].status, EventStatus::NotStarted);
    assert_eq!(mock.count(Operation::Read, 3), 0);
    assert_eq!(mock.count(Operation::Close, 3), 1);
}

#[test]
fn unsupported_platform_makes_no_perf_calls() {
    let mock = Mock::new();
    mock.0.borrow_mut().unsupported = true;
    let report = run(&mock, CyclesPolicy::OptionalWithStandaloneFallback);
    assert_eq!(report.attempts.len(), 1);
    assert!(report.attempts[0]
        .events
        .iter()
        .all(|e| e.status == EventStatus::UnsupportedPlatform));
    assert!(mock.0.borrow().operations.is_empty());
}

#[test]
fn native_backend_support_requires_little_endian_linux_64_bit_abi() {
    // Querying support never opens counters. On unsupported targets, exercise
    // the real backend's early exit and require an entirely empty lifecycle.
    let supported = NativeCalls.supported();
    assert_eq!(
        supported,
        cfg!(all(
            target_os = "linux",
            target_pointer_width = "64",
            target_endian = "little",
            any(target_arch = "x86_64", target_arch = "aarch64")
        ))
    );
    if !supported {
        let collector = Collector::open(
            NativeCalls,
            CounterPolicy {
                scope: TaskScope::CallingThread,
                cycles: CyclesPolicy::OptionalWithStandaloneFallback,
            },
        );
        let report = collector.finish();
        assert_eq!(report.attempts.len(), 1);
        assert_eq!(report.selected_attempt, None);
        for event in &report.attempts[0].events {
            assert_eq!(event.status, EventStatus::UnsupportedPlatform);
            assert!(event.lifecycle.is_empty());
            assert_eq!(event.kernel_event_id, None);
            assert_eq!(event.raw, None);
        }
    }
}

#[test]
fn open_failures_preserve_errno_and_never_broaden_scope() {
    for (errno, expected) in [
        (libc::EPERM, EventStatus::PermissionDenied),
        (libc::EACCES, EventStatus::PermissionDenied),
        (libc::ENODEV, EventStatus::UnsupportedEvent),
        (libc::ENOENT, EventStatus::UnsupportedEvent),
        (libc::EOPNOTSUPP, EventStatus::UnsupportedEvent),
        (libc::EINVAL, EventStatus::OpenFailed),
        (libc::EINTR, EventStatus::OpenFailed),
    ] {
        let mock = Mock::new();
        mock.fail(Operation::Open, 3, errno);
        let report = run(&mock, CyclesPolicy::OptionalWithStandaloneFallback);
        let event = &report.attempts[0].events[0];
        assert_eq!(event.status, expected);
        assert_eq!(
            event.lifecycle[0].result.as_ref().unwrap_err().errno,
            Some(errno)
        );
        assert_eq!(report.attempts.len(), 1);
        assert_eq!(report.selected_attempt, None);
        assert_eq!(mock.count(Operation::Close, 3), 0);
        assert_eq!(
            report.attempts[0].events[1].status,
            EventStatus::NotAttempted
        );
    }
}

#[test]
fn id_failure_closes_returned_fd_without_starting() {
    let mock = Mock::new();
    mock.fail(Operation::EventId, 3, libc::EIO);
    let report = run(&mock, CyclesPolicy::Required);
    assert_eq!(
        report.attempts[0].events[0].status,
        EventStatus::IdentityFailed
    );
    assert_eq!(mock.count(Operation::Close, 3), 1);
    assert_eq!(mock.count(Operation::EnableGroup, 3), 0);
}

#[test]
fn duplicate_kernel_ids_are_rejected_and_retained() {
    let mock = Mock::new();
    mock.0.borrow_mut().ids.insert(4, 103);
    let report = run(&mock, CyclesPolicy::Required);
    let events = &report.attempts[0].events;
    assert_eq!(events[1].status, EventStatus::IdentityFailed);
    assert_eq!(events[1].kernel_event_id, Some(103));
    assert!(events[1].identity_error.is_some());
    assert_eq!(events[0].status, EventStatus::CancelledBeforeStart);
    assert_eq!(mock.count(Operation::Close, 3), 1);
    assert_eq!(mock.count(Operation::Close, 4), 1);
}

#[test]
fn optional_cycles_failure_has_separate_standalone_attempt_before_work() {
    for operation in [Operation::Open, Operation::EventId] {
        let mock = Mock::new();
        mock.fail(operation, 4, libc::EIO);
        let report = run(&mock, CyclesPolicy::OptionalWithStandaloneFallback);
        assert_eq!(report.attempts.len(), 2);
        assert_eq!(report.selected_attempt, Some(1));
        assert_eq!(
            report.attempts[0].events[0].status,
            EventStatus::CancelledBeforeStart
        );
        assert_eq!(
            report.attempts[1].mode,
            CollectionMode::StandaloneInstructionsFallback
        );
        assert_eq!(report.attempts[1].events.len(), 1);
        assert_eq!(report.attempts[1].events[0].request.group_fd, -1);
        assert_eq!(
            report.attempts[1].events[0].status,
            EventStatus::ObservedUnmultiplexed
        );
        assert_eq!(mock.count(Operation::EnableGroup, 3), 0);
        assert_eq!(mock.count(Operation::EnableGroup, 5), 1);
        assert_eq!(mock.count(Operation::Close, 3), 1);
        assert_eq!(mock.count(Operation::Close, 5), 1);
    }
}

#[test]
fn required_cycles_failure_does_not_fallback() {
    let mock = Mock::new();
    mock.fail(Operation::Open, 4, libc::ENODEV);
    let report = run(&mock, CyclesPolicy::Required);
    assert_eq!(report.attempts.len(), 1);
    assert_eq!(report.selected_attempt, None);
    assert_eq!(
        report.attempts[0].events[0].status,
        EventStatus::CancelledBeforeStart
    );
}

#[test]
fn failed_preparation_cleanup_prevents_fallback_and_never_retries_close() {
    let mock = Mock::new();
    mock.fail(Operation::Open, 4, libc::EIO);
    mock.fail(Operation::Close, 3, libc::EINTR);
    let report = run(&mock, CyclesPolicy::OptionalWithStandaloneFallback);
    assert_eq!(report.attempts.len(), 1);
    assert_eq!(report.attempts[0].events[0].cleanup_succeeded, Some(false));
    assert_eq!(mock.count(Operation::Close, 3), 1);
}

#[test]
fn every_control_failure_invalidates_all_group_intervals_but_retains_reads() {
    for operation in [
        Operation::ResetGroup,
        Operation::EnableGroup,
        Operation::DisableGroup,
    ] {
        let mock = Mock::new();
        mock.fail(operation, 3, libc::EIO);
        let report = run(&mock, CyclesPolicy::Required);
        for event in &report.attempts[0].events {
            assert_eq!(event.status, EventStatus::ControlFailed);
            assert!(event.raw.is_some());
            assert_eq!(event.cleanup_succeeded, Some(true));
        }
        assert_eq!(mock.count(operation, 3), 1);
        if operation == Operation::ResetGroup {
            assert_eq!(mock.count(Operation::EnableGroup, 3), 0);
        }
    }
}

#[test]
fn repeated_start_invalidates_without_resetting_the_interval() {
    let mock = Mock::new();
    let mut collector = Collector::open(mock.clone(), policy(CyclesPolicy::Disabled));
    assert!(collector.start());
    assert!(!collector.start());
    let report = collector.finish();
    assert_eq!(
        report.attempts[0].events[0].status,
        EventStatus::ControlFailed
    );
    assert!(!report.attempts[0].start_succeeded);
    assert_eq!(mock.count(Operation::ResetGroup, 3), 1);
    assert_eq!(mock.count(Operation::EnableGroup, 3), 1);
}

#[test]
fn read_error_and_eintr_are_not_retried_or_promoted() {
    for errno in [libc::EIO, libc::EINTR] {
        let mock = Mock::new();
        mock.fail(Operation::Read, 3, errno);
        let report = run(&mock, CyclesPolicy::Disabled);
        let event = &report.attempts[0].events[0];
        assert_eq!(event.status, EventStatus::ReadFailed);
        assert_eq!(event.raw, None);
        assert_eq!(event.read_bytes, None);
        assert_eq!(mock.count(Operation::Read, 3), 1);
        assert_eq!(mock.count(Operation::Close, 3), 1);
    }
}

#[test]
fn missing_short_and_oversized_tuples_are_not_completed_from_other_events() {
    for length in [0, 8, 16, 24, 31, 33, 40] {
        let mock = Mock::new();
        mock.0.borrow_mut().reads.insert(3, vec![0; length]);
        let report = run(&mock, CyclesPolicy::Required);
        let event = &report.attempts[0].events[0];
        assert_eq!(event.status, EventStatus::ShortRead);
        assert_eq!(event.read_bytes, Some(length));
        assert_eq!(event.raw, None);
        assert_eq!(
            report.attempts[0].events[1].status,
            EventStatus::ObservedUnmultiplexed
        );
    }
}

#[test]
fn tuple_id_time_zero_and_overflow_assessment_preserves_raw_evidence() {
    for (value, enabled, running, id, expected, estimate) in [
        (7, 20, 20, 999, EventStatus::IdentityMismatch, None),
        (7, 19, 20, 103, EventStatus::InvalidTuple, None),
        (0, 0, 0, 103, EventStatus::NotRunning, None),
        (7, 20, 0, 103, EventStatus::NotRunning, None),
        (0, 20, 20, 103, EventStatus::ObservedUnmultiplexed, None),
        (
            7,
            20,
            10,
            103,
            EventStatus::ObservedMultiplexedEstimate,
            Some(14),
        ),
        (
            7,
            20,
            13,
            103,
            EventStatus::ObservedMultiplexedEstimate,
            Some(10),
        ),
        (
            u64::MAX,
            u64::MAX,
            1,
            103,
            EventStatus::EstimateOverflow,
            None,
        ),
    ] {
        let mock = Mock::new();
        mock.0
            .borrow_mut()
            .reads
            .insert(3, bytes(value, enabled, running, id));
        let report = run(&mock, CyclesPolicy::Disabled);
        let event = &report.attempts[0].events[0];
        assert_eq!(event.status, expected);
        assert_eq!(event.tuple_assessment.unwrap().scaled_estimate, estimate);
        assert_eq!(
            event.raw.unwrap(),
            RawTuple {
                value,
                time_enabled_ns: enabled,
                time_running_ns: running,
                event_id: id
            }
        );
    }
}

#[test]
fn late_cycles_read_failure_keeps_instruction_observation_without_rerunning() {
    let mock = Mock::new();
    mock.fail(Operation::Read, 4, libc::EIO);
    let report = run(&mock, CyclesPolicy::OptionalWithStandaloneFallback);
    assert_eq!(report.attempts.len(), 1);
    assert_eq!(
        report.attempts[0].events[0].status,
        EventStatus::ObservedUnmultiplexed
    );
    assert_eq!(report.attempts[0].events[1].status, EventStatus::ReadFailed);
    assert_eq!(mock.count(Operation::EnableGroup, 3), 1);
}

#[test]
fn cleanup_errors_are_distinct_from_successful_read_and_close_is_not_retried() {
    let mock = Mock::new();
    mock.fail(Operation::Close, 3, libc::EINTR);
    let report = run(&mock, CyclesPolicy::Disabled);
    let event = &report.attempts[0].events[0];
    assert_eq!(event.status, EventStatus::ObservedUnmultiplexed);
    assert_eq!(event.cleanup_succeeded, Some(false));
    assert_eq!(mock.count(Operation::Close, 3), 1);
}

#[test]
fn prepared_and_active_drop_close_every_owned_fd_once() {
    for started in [false, true] {
        let mock = Mock::new();
        let mut collector = Collector::open(mock.clone(), policy(CyclesPolicy::Required));
        if started {
            assert!(collector.start());
        }
        drop(collector);
        assert_eq!(mock.count(Operation::DisableGroup, 3), usize::from(started));
        assert_eq!(mock.count(Operation::Read, 3), 0);
        assert_eq!(mock.count(Operation::Close, 3), 1);
        assert_eq!(mock.count(Operation::Close, 4), 1);
    }
}

#[test]
fn unwind_after_start_releases_group_without_claiming_a_finished_read() {
    let mock = Mock::new();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut collector = Collector::open(mock.clone(), policy(CyclesPolicy::Required));
        assert!(collector.start());
        panic!("mock workload failure");
    }));
    assert!(result.is_err());
    assert_eq!(mock.count(Operation::DisableGroup, 3), 1);
    assert_eq!(mock.count(Operation::Read, 3), 0);
    assert_eq!(mock.count(Operation::Close, 3), 1);
    assert_eq!(mock.count(Operation::Close, 4), 1);
}

#[test]
fn preparation_unwind_still_closes_the_unselected_owned_fd() {
    let mock = Mock::new();
    mock.0.borrow_mut().panic_id = true;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Collector::open(mock.clone(), policy(CyclesPolicy::Required))
    }));
    assert!(result.is_err());
    assert_eq!(mock.count(Operation::Close, 3), 1);
    assert_eq!(mock.count(Operation::EnableGroup, 3), 0);
}

#[test]
fn changed_or_missing_identity_before_start_prevents_any_controls_or_reads() {
    for (pid, tid, expected) in [
        (999, Some(124), EventStatus::ContextMismatch),
        (123, Some(999), EventStatus::ContextMismatch),
        (0, Some(124), EventStatus::ContextUnavailable),
        (123, None, EventStatus::ContextUnavailable),
        (123, Some(0), EventStatus::ContextUnavailable),
    ] {
        let mock = Mock::new();
        let mut collector = Collector::open(mock.clone(), policy(CyclesPolicy::Required));
        mock.0.borrow_mut().pid = pid;
        mock.0.borrow_mut().tid = tid;
        assert!(!collector.start());
        let report = collector.finish();
        assert_eq!((report.owner_pid, report.owner_tid), (123, Some(124)));
        assert_eq!(report.context_checks[0].stage, ContextStage::Start);
        assert_eq!(report.context_checks[0].current, TaskIdentity { pid, tid });
        assert_eq!(report.context_checks[1].stage, ContextStage::Finish);
        for event in &report.attempts[0].events {
            assert_eq!(event.status, expected);
        }
        for operation in [
            Operation::ResetGroup,
            Operation::EnableGroup,
            Operation::DisableGroup,
            Operation::Read,
        ] {
            assert_eq!(mock.count(operation, 3), 0);
        }
        assert_eq!(mock.count(Operation::Close, 3), 1);
        assert_eq!(mock.count(Operation::Close, 4), 1);
    }
}

#[test]
fn changed_or_missing_identity_before_finish_never_controls_original_owner_event() {
    for (pid, tid, expected) in [
        (999, Some(124), EventStatus::ContextMismatch),
        (123, Some(999), EventStatus::ContextMismatch),
        (0, Some(124), EventStatus::ContextUnavailable),
        (123, None, EventStatus::ContextUnavailable),
    ] {
        let mock = Mock::new();
        let mut collector = Collector::open(mock.clone(), policy(CyclesPolicy::Required));
        assert!(collector.start());
        mock.0.borrow_mut().pid = pid;
        mock.0.borrow_mut().tid = tid;
        let report = collector.finish();
        assert_eq!(report.context_checks[0].status, ContextStatus::Matched);
        assert_eq!(report.context_checks[1].current, TaskIdentity { pid, tid });
        assert!(!report.attempts[0].start_succeeded);
        for event in &report.attempts[0].events {
            assert_eq!(event.status, expected);
            assert_eq!(event.raw, None);
            assert_eq!(event.cleanup_succeeded, Some(true));
        }
        assert_eq!(mock.count(Operation::DisableGroup, 3), 0);
        assert_eq!(mock.count(Operation::Read, 3), 0);
        assert_eq!(mock.count(Operation::Close, 3), 1);
        assert_eq!(mock.count(Operation::Close, 4), 1);
    }
}

#[test]
fn unavailable_original_identity_cannot_be_repaired_by_a_later_identity() {
    let mock = Mock::new();
    mock.0.borrow_mut().tid = None;
    let mut collector = Collector::open(mock.clone(), policy(CyclesPolicy::Disabled));
    mock.0.borrow_mut().tid = Some(124);
    assert!(!collector.start());
    let report = collector.finish();
    assert_eq!(report.owner_tid, None);
    assert!(report
        .context_checks
        .iter()
        .all(|check| check.status == ContextStatus::Unavailable));
    assert_eq!(
        report.attempts[0].events[0].status,
        EventStatus::ContextUnavailable
    );
    assert_eq!(mock.count(Operation::EnableGroup, 3), 0);
}

#[test]
fn a_context_failure_stays_invalid_after_returning_to_the_original_context() {
    let mock = Mock::new();
    let mut collector = Collector::open(mock.clone(), policy(CyclesPolicy::Disabled));
    assert!(collector.start());
    mock.0.borrow_mut().pid = 999;
    assert!(!collector.start());
    mock.0.borrow_mut().pid = 123;
    let report = collector.finish();
    assert_eq!(report.context_checks.len(), 3);
    assert_eq!(report.context_checks[1].status, ContextStatus::Mismatch);
    assert_eq!(report.context_checks[2].status, ContextStatus::Matched);
    assert_eq!(
        report.attempts[0].events[0].status,
        EventStatus::ContextMismatch
    );
    assert_eq!(mock.count(Operation::DisableGroup, 3), 1);
    assert_eq!(mock.count(Operation::Read, 3), 0);
}

#[test]
fn changed_context_drop_only_closes_local_references() {
    let mock = Mock::new();
    let mut collector = Collector::open(mock.clone(), policy(CyclesPolicy::Required));
    assert!(collector.start());
    mock.0.borrow_mut().pid = 999;
    drop(collector);
    assert_eq!(mock.count(Operation::DisableGroup, 3), 0);
    assert_eq!(mock.count(Operation::Close, 3), 1);
    assert_eq!(mock.count(Operation::Close, 4), 1);
}

#[test]
fn finalization_control_or_read_unwind_keeps_owned_fds_visible_to_cleanup() {
    for operation in [Operation::DisableGroup, Operation::Read] {
        let mock = Mock::new();
        let mut collector = Collector::open(mock.clone(), policy(CyclesPolicy::Required));
        assert!(collector.start());
        mock.0.borrow_mut().panic_once = Some((operation, 3));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| collector.finish()));
        assert!(result.is_err());
        assert_eq!(mock.count(Operation::DisableGroup, 3), 1);
        assert_eq!(mock.count(Operation::Close, 3), 1);
        assert_eq!(mock.count(Operation::Close, 4), 1);
        assert_eq!(mock.count(Operation::Read, 4), 0);
    }
}
