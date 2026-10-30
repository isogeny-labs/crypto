//! Checked counting-only PMU lifecycle. This API does not admit a platform or
//! establish coverage of a whole worker, all descendants, or a complete solve.
//! No dataset schema or legacy `HwCounters` behavior is changed by this module.
use serde::Serialize;
use std::marker::PhantomData;
use std::rc::Rc;

const ATTR_SIZE: usize = 112;
const READ_BYTES: usize = 32;
const READ_FORMAT: u64 = 1 | 2 | 4; // time enabled, time running, event ID
const FD_CLOEXEC: u64 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskScope {
    CallingThread,
    CallingThreadAndNewDescendants,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CyclesPolicy {
    Disabled,
    Required,
    OptionalWithStandaloneFallback,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct CounterPolicy {
    pub scope: TaskScope,
    pub cycles: CyclesPolicy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Instructions,
    CpuCycles,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionMode {
    InstructionsOnly,
    InstructionsAndCyclesGroup,
    StandaloneInstructionsFallback,
}

/// All bytes supplied to perf_event_open, including zero/reserved fields.
/// `pid=0,cpu=-1` targets the calling thread, not every thread in the process.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EventRequest {
    pub event: EventKind,
    pub perf_type: u32,
    pub config: u64,
    pub attr_size: u32,
    pub read_format: u64,
    pub attr_flags: u64,
    pub attr_bytes_native_endian: Vec<u8>,
    pub byte_order: &'static str,
    pub pid: i32,
    pub cpu: i32,
    pub group_fd: i32,
    pub syscall_flags: u64,
}

impl EventRequest {
    fn new(event: EventKind, scope: TaskScope, group_fd: i32) -> Self {
        let config = match event {
            EventKind::Instructions => 1,
            EventKind::CpuCycles => 0,
        };
        let mut flags = (1 << 5) | (1 << 6); // exclude kernel and hypervisor
        if group_fd < 0 {
            flags |= 1; // disabled leader; members follow leader
        }
        if scope == TaskScope::CallingThreadAndNewDescendants {
            flags |= 1 << 1;
        }
        let mut bytes = vec![0; ATTR_SIZE];
        bytes[0..4].copy_from_slice(&0u32.to_ne_bytes());
        bytes[4..8].copy_from_slice(&(ATTR_SIZE as u32).to_ne_bytes());
        bytes[8..16].copy_from_slice(&u64::to_ne_bytes(config));
        bytes[32..40].copy_from_slice(&READ_FORMAT.to_ne_bytes());
        bytes[40..48].copy_from_slice(&u64::to_ne_bytes(flags));
        Self {
            event,
            perf_type: 0,
            config,
            attr_size: ATTR_SIZE as u32,
            read_format: READ_FORMAT,
            attr_flags: flags,
            attr_bytes_native_endian: bytes,
            byte_order: if cfg!(target_endian = "little") {
                "little"
            } else {
                "big"
            },
            pid: 0,
            cpu: -1,
            group_fd,
            syscall_flags: FD_CLOEXEC,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CallError {
    pub errno: Option<i32>,
    pub message: String,
}

impl CallError {
    #[cfg(all(
        target_os = "linux",
        target_pointer_width = "64",
        target_endian = "little",
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    fn last() -> Self {
        let error = std::io::Error::last_os_error();
        Self {
            errno: error.raw_os_error(),
            message: error.to_string(),
        }
    }

    #[cfg(not(all(
        target_os = "linux",
        target_pointer_width = "64",
        target_endian = "little",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )))]
    fn unsupported() -> Self {
        Self {
            errno: None,
            message: "native perf ABI supported only on little-endian 64-bit Linux x86_64/aarch64"
                .into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Open,
    EventId,
    ResetGroup,
    EnableGroup,
    DisableGroup,
    Read,
    Close,
    RepeatedStart,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LifecycleCall {
    pub operation: Operation,
    pub result: Result<i64, CallError>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventStatus {
    NotAttempted,
    Prepared,
    UnsupportedPlatform,
    UnsupportedEvent,
    PermissionDenied,
    OpenFailed,
    IdentityFailed,
    CancelledBeforeStart,
    NotStarted,
    ContextMismatch,
    ContextUnavailable,
    ControlFailed,
    ReadFailed,
    ShortRead,
    IdentityMismatch,
    InvalidTuple,
    NotRunning,
    ObservedUnmultiplexed,
    ObservedMultiplexedEstimate,
    EstimateOverflow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct RawTuple {
    pub value: u64,
    pub time_enabled_ns: u64,
    pub time_running_ns: u64,
    pub event_id: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct TupleAssessment {
    pub status: EventStatus,
    pub multiplexed: Option<bool>,
    pub scaled_estimate: Option<u64>,
}

fn assess(raw: RawTuple, expected_id: u64) -> TupleAssessment {
    let mut result = TupleAssessment {
        status: EventStatus::InvalidTuple,
        multiplexed: None,
        scaled_estimate: None,
    };
    if raw.event_id != expected_id {
        result.status = EventStatus::IdentityMismatch;
    } else if raw.time_running_ns > raw.time_enabled_ns {
        result.status = EventStatus::InvalidTuple;
    } else if raw.time_running_ns == 0 {
        result.status = EventStatus::NotRunning;
    } else if raw.time_running_ns == raw.time_enabled_ns {
        result.status = EventStatus::ObservedUnmultiplexed;
        result.multiplexed = Some(false);
    } else {
        result.multiplexed = Some(true);
        let scaled = u128::from(raw.value) * u128::from(raw.time_enabled_ns)
            / u128::from(raw.time_running_ns);
        match u64::try_from(scaled) {
            Ok(value) => {
                result.status = EventStatus::ObservedMultiplexedEstimate;
                result.scaled_estimate = Some(value);
            }
            Err(_) => result.status = EventStatus::EstimateOverflow,
        }
    }
    result
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EventRecord {
    pub request: EventRequest,
    pub kernel_event_id: Option<u64>,
    pub identity_error: Option<&'static str>,
    pub lifecycle: Vec<LifecycleCall>,
    pub status: EventStatus,
    pub raw: Option<RawTuple>,
    pub tuple_assessment: Option<TupleAssessment>,
    pub read_bytes: Option<usize>,
    pub cleanup_succeeded: Option<bool>,
}

impl EventRecord {
    fn new(request: EventRequest) -> Self {
        Self {
            request,
            kernel_event_id: None,
            identity_error: None,
            lifecycle: Vec::new(),
            status: EventStatus::NotAttempted,
            raw: None,
            tuple_assessment: None,
            read_bytes: None,
            cleanup_succeeded: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AttemptRecord {
    pub mode: CollectionMode,
    pub events: Vec<EventRecord>,
    pub start_succeeded: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct TaskIdentity {
    pub pid: u32,
    pub tid: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextStatus {
    Matched,
    Mismatch,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextStage {
    Start,
    Finish,
    DropCleanup,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct ContextCheck {
    pub stage: ContextStage,
    pub current: TaskIdentity,
    pub status: ContextStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CollectionRecord {
    pub collector_version: &'static str,
    pub policy: CounterPolicy,
    pub owner_pid: u32,
    pub owner_tid: Option<u32>,
    pub context_checks: Vec<ContextCheck>,
    pub os: &'static str,
    pub architecture: &'static str,
    pub attempts: Vec<AttemptRecord>,
    pub selected_attempt: Option<usize>,
    pub whole_worker_coverage_verified: bool,
    pub descendant_lifetimes_verified: bool,
    pub complete_execution_verified: bool,
    pub hardware_independently_verified: bool,
    pub platform_admitted: bool,
    pub clock_valid_for_comparison: bool,
}

// This private backend's identity/close operations must not unwind. The native
// implementation only calls the OS and returns errors. Close is never retried,
// so a hypothetical backend panic before actually closing cannot be repaired
// safely by reusing that FD. Cleanup guarantees cover caller unwind and a
// one-time control/read unwind, not panic during cleanup itself or SIGKILL.
trait Calls {
    fn supported(&self) -> bool;
    fn identity(&self) -> (u32, Option<u32>);
    fn open(&mut self, request: &EventRequest) -> Result<i32, CallError>;
    fn event_id(&mut self, fd: i32) -> Result<u64, CallError>;
    fn control(&mut self, fd: i32, operation: Operation) -> Result<(), CallError>;
    fn read(&mut self, fd: i32) -> Result<Vec<u8>, CallError>;
    fn close(&mut self, fd: i32) -> Result<(), CallError>;
}

struct NativeCalls;

// These little-endian architectures use the asm-generic ioctl encoding and
// 64-bit pointer size for PERF_EVENT_IOC_ID. The perf_event_attr flags are a C
// bitfield, so their low-bit masks cannot be encoded with to_ne_bytes on a
// big-endian target. Other ABIs fail before any syscall instead of guessing.
#[cfg(all(
    target_os = "linux",
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
impl Calls for NativeCalls {
    fn supported(&self) -> bool {
        true
    }
    fn identity(&self) -> (u32, Option<u32>) {
        // SAFETY: gettid takes no pointer arguments.
        (
            std::process::id(),
            u32::try_from(unsafe { libc::syscall(libc::SYS_gettid) }).ok(),
        )
    }
    fn open(&mut self, request: &EventRequest) -> Result<i32, CallError> {
        // SAFETY: owned attr bytes have the declared 112-byte length; this
        // private backend receives requests built by EventRequest::new only.
        let fd = unsafe {
            libc::syscall(
                libc::SYS_perf_event_open,
                request.attr_bytes_native_endian.as_ptr(),
                request.pid,
                request.cpu,
                request.group_fd,
                request.syscall_flags as libc::c_ulong,
            )
        };
        if fd < 0 {
            Err(CallError::last())
        } else {
            Ok(fd as i32)
        }
    }
    fn event_id(&mut self, fd: i32) -> Result<u64, CallError> {
        let mut id = 0u64;
        // SAFETY: _IOR('$',7,__u64*) on the explicitly supported 64-bit ABIs;
        // kernel writes to a valid u64, and fd belongs to this collector.
        let result = unsafe { libc::ioctl(fd, 0x8008_2407u32 as _, &mut id) };
        if result < 0 {
            Err(CallError::last())
        } else {
            Ok(id)
        }
    }
    fn control(&mut self, fd: i32, operation: Operation) -> Result<(), CallError> {
        let request: libc::c_ulong = match operation {
            Operation::ResetGroup => 0x2403,
            Operation::EnableGroup => 0x2400,
            Operation::DisableGroup => 0x2401,
            _ => unreachable!("private backend accepts control operations only"),
        };
        // SAFETY: valid owned leader fd; PERF_IOC_FLAG_GROUP=1 is scalar.
        let result = unsafe { libc::ioctl(fd, request as _, 1 as libc::c_ulong) };
        if result < 0 {
            Err(CallError::last())
        } else {
            Ok(())
        }
    }
    fn read(&mut self, fd: i32) -> Result<Vec<u8>, CallError> {
        let mut bytes = vec![0; READ_BYTES];
        // SAFETY: writable READ_BYTES buffer and an owned event fd. Interrupted
        // reads remain explicit failures; no implicit retry changes the record.
        let n = unsafe { libc::read(fd, bytes.as_mut_ptr().cast(), bytes.len()) };
        if n < 0 {
            Err(CallError::last())
        } else {
            bytes.truncate(n as usize);
            Ok(bytes)
        }
    }
    fn close(&mut self, fd: i32) -> Result<(), CallError> {
        // SAFETY: descriptor is removed from collector ownership before close.
        // Never retry close, including EINTR: that could close a reused fd.
        if unsafe { libc::close(fd) } < 0 {
            Err(CallError::last())
        } else {
            Ok(())
        }
    }
}

#[cfg(not(all(
    target_os = "linux",
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64")
)))]
impl Calls for NativeCalls {
    fn supported(&self) -> bool {
        false
    }
    fn identity(&self) -> (u32, Option<u32>) {
        (std::process::id(), None)
    }
    fn open(&mut self, _: &EventRequest) -> Result<i32, CallError> {
        Err(CallError::unsupported())
    }
    fn event_id(&mut self, _: i32) -> Result<u64, CallError> {
        Err(CallError::unsupported())
    }
    fn control(&mut self, _: i32, _: Operation) -> Result<(), CallError> {
        Err(CallError::unsupported())
    }
    fn read(&mut self, _: i32) -> Result<Vec<u8>, CallError> {
        Err(CallError::unsupported())
    }
    fn close(&mut self, _: i32) -> Result<(), CallError> {
        Err(CallError::unsupported())
    }
}

/// Thread-bound native counting session. `start()` must succeed to establish an
/// interval. `finish()` returns diagnostics even when preparation/control fails.
/// PID/TID are checked again at start/finish, including after fork. Context
/// mismatch closes this process's descriptors without controlling the owner's
/// event. Drop cleans up descriptors, but cannot persist a measurement receipt.
pub struct CheckedCounters {
    inner: Collector<NativeCalls>,
}

impl CheckedCounters {
    pub fn open(policy: CounterPolicy) -> Self {
        Self {
            inner: Collector::open(NativeCalls, policy),
        }
    }
    pub fn start(&mut self) -> bool {
        self.inner.start()
    }
    pub fn finish(self) -> CollectionRecord {
        self.inner.finish()
    }
}

struct Collector<C: Calls> {
    calls: C,
    record: CollectionRecord,
    fds: Vec<Option<i32>>,
    start_called: bool,
    context_failed: bool,
    disable_attempted: bool,
    finalized: bool,
    _thread_bound: PhantomData<Rc<()>>,
}

impl<C: Calls> Collector<C> {
    fn open(calls: C, policy: CounterPolicy) -> Self {
        let (owner_pid, owner_tid) = calls.identity();
        let mut this = Self {
            calls,
            record: CollectionRecord {
                collector_version: "checked-native-pmu/v1",
                policy,
                owner_pid,
                owner_tid,
                context_checks: Vec::new(),
                os: std::env::consts::OS,
                architecture: std::env::consts::ARCH,
                attempts: Vec::new(),
                selected_attempt: None,
                whole_worker_coverage_verified: false,
                descendant_lifetimes_verified: false,
                complete_execution_verified: false,
                hardware_independently_verified: false,
                platform_admitted: false,
                clock_valid_for_comparison: false,
            },
            fds: Vec::new(),
            start_called: false,
            context_failed: false,
            disable_attempted: false,
            finalized: false,
            _thread_bound: PhantomData,
        };
        let mode = if policy.cycles == CyclesPolicy::Disabled {
            CollectionMode::InstructionsOnly
        } else {
            CollectionMode::InstructionsAndCyclesGroup
        };
        let prepared = this.prepare(mode);
        if !prepared && policy.cycles == CyclesPolicy::OptionalWithStandaloneFallback {
            let events = &this.record.attempts[0].events;
            // Only loss of the optional member after the leader opened and was
            // identified permits a new, separately recorded standalone attempt.
            if events.len() == 2
                && events[0].kernel_event_id.is_some()
                && events
                    .iter()
                    .all(|event| event.cleanup_succeeded != Some(false))
                && matches!(
                    events[1].status,
                    EventStatus::OpenFailed
                        | EventStatus::UnsupportedEvent
                        | EventStatus::PermissionDenied
                        | EventStatus::IdentityFailed
                )
            {
                this.prepare(CollectionMode::StandaloneInstructionsFallback);
            }
        }
        this
    }

    fn prepare(&mut self, mode: CollectionMode) -> bool {
        let mut events = vec![EventRecord::new(EventRequest::new(
            EventKind::Instructions,
            self.record.policy.scope,
            -1,
        ))];
        if mode == CollectionMode::InstructionsAndCyclesGroup {
            events.push(EventRecord::new(EventRequest::new(
                EventKind::CpuCycles,
                self.record.policy.scope,
                -1,
            )));
        }
        let attempt = self.record.attempts.len();
        self.record.attempts.push(AttemptRecord {
            mode,
            events,
            start_succeeded: false,
        });
        self.fds = vec![None; self.record.attempts[attempt].events.len()];
        if !self.calls.supported() {
            for event in &mut self.record.attempts[attempt].events {
                event.status = EventStatus::UnsupportedPlatform;
            }
            return false;
        }
        for index in 0..self.fds.len() {
            if index > 0 {
                self.record.attempts[attempt].events[index].request = EventRequest::new(
                    EventKind::CpuCycles,
                    self.record.policy.scope,
                    self.fds[0].expect("prepared leader"),
                );
            }
            let result = self
                .calls
                .open(&self.record.attempts[attempt].events[index].request);
            self.log(
                attempt,
                index,
                Operation::Open,
                result.clone().map(i64::from),
            );
            let fd = match result {
                Ok(fd) => {
                    self.fds[index] = Some(fd);
                    fd
                }
                Err(error) => {
                    self.record.attempts[attempt].events[index].status = match error.errno {
                        Some(libc::EPERM | libc::EACCES) => EventStatus::PermissionDenied,
                        Some(libc::ENODEV | libc::ENOENT | libc::EOPNOTSUPP) => {
                            EventStatus::UnsupportedEvent
                        }
                        _ => EventStatus::OpenFailed,
                    };
                    self.cancel_prepared(attempt);
                    return false;
                }
            };
            let identity = self.calls.event_id(fd);
            // Lifecycle numeric result is only a success marker: a u64 event
            // ID need not fit i64 and is retained losslessly in its own field.
            self.log(
                attempt,
                index,
                Operation::EventId,
                identity.clone().map(|_| 0),
            );
            match identity {
                Ok(id) => {
                    let duplicate = self.record.attempts[attempt].events[..index]
                        .iter()
                        .any(|event| event.kernel_event_id == Some(id));
                    let event = &mut self.record.attempts[attempt].events[index];
                    event.kernel_event_id = Some(id);
                    if duplicate {
                        event.identity_error = Some("duplicate kernel event ID in group");
                        event.status = EventStatus::IdentityFailed;
                        self.cancel_prepared(attempt);
                        return false;
                    }
                    event.status = EventStatus::Prepared;
                }
                Err(_) => {
                    self.record.attempts[attempt].events[index].status =
                        EventStatus::IdentityFailed;
                    self.cancel_prepared(attempt);
                    return false;
                }
            }
        }
        self.record.selected_attempt = Some(attempt);
        true
    }

    fn log(
        &mut self,
        attempt: usize,
        index: usize,
        operation: Operation,
        result: Result<i64, CallError>,
    ) {
        self.record.attempts[attempt].events[index]
            .lifecycle
            .push(LifecycleCall { operation, result });
    }

    fn close_all(&mut self, attempt: usize) {
        // Close members before the leader. Each fd is taken before calling
        // close so every exit path, including an error, releases ownership once.
        for index in (0..self.fds.len()).rev() {
            if let Some(fd) = self.fds[index].take() {
                let result = self.calls.close(fd);
                self.record.attempts[attempt].events[index].cleanup_succeeded =
                    Some(result.is_ok());
                self.log(attempt, index, Operation::Close, result.map(|_| 0));
            }
        }
    }

    fn cancel_prepared(&mut self, attempt: usize) {
        for event in &mut self.record.attempts[attempt].events {
            if event.status == EventStatus::Prepared {
                event.status = EventStatus::CancelledBeforeStart;
            }
        }
        self.close_all(attempt);
    }

    fn control(&mut self, attempt: usize, operation: Operation) -> bool {
        let Some(fd) = self.fds.first().copied().flatten() else {
            return false;
        };
        let result = self.calls.control(fd, operation);
        let success = result.is_ok();
        self.log(attempt, 0, operation, result.map(|_| 0));
        if !success {
            for event in &mut self.record.attempts[attempt].events {
                if !matches!(
                    event.status,
                    EventStatus::ContextMismatch | EventStatus::ContextUnavailable
                ) {
                    event.status = EventStatus::ControlFailed;
                }
            }
        }
        success
    }

    fn check_context(&mut self, stage: ContextStage) -> bool {
        let (pid, tid) = self.calls.identity();
        let status = if self.record.owner_pid == 0
            || pid == 0
            || !self.record.owner_tid.is_some_and(|tid| tid > 0)
            || !tid.is_some_and(|tid| tid > 0)
        {
            ContextStatus::Unavailable
        } else if pid != self.record.owner_pid || tid != self.record.owner_tid {
            ContextStatus::Mismatch
        } else {
            ContextStatus::Matched
        };
        self.record.context_checks.push(ContextCheck {
            stage,
            current: TaskIdentity { pid, tid },
            status,
        });
        if status != ContextStatus::Matched {
            self.context_failed = true;
            if let Some(attempt) = self.record.selected_attempt {
                for event in &mut self.record.attempts[attempt].events {
                    event.status = if status == ContextStatus::Mismatch {
                        EventStatus::ContextMismatch
                    } else {
                        EventStatus::ContextUnavailable
                    };
                }
                self.record.attempts[attempt].start_succeeded = false;
            }
        }
        status == ContextStatus::Matched
    }

    fn start(&mut self) -> bool {
        if !self.check_context(ContextStage::Start) || self.context_failed {
            return false;
        }
        let Some(attempt) = self.record.selected_attempt else {
            return false;
        };
        if self.start_called {
            self.log(
                attempt,
                0,
                Operation::RepeatedStart,
                Err(CallError {
                    errno: None,
                    message: "start called more than once; interval invalidated".into(),
                }),
            );
            for event in &mut self.record.attempts[attempt].events {
                event.status = EventStatus::ControlFailed;
            }
            self.record.attempts[attempt].start_succeeded = false;
            return false;
        }
        self.start_called = true;
        let success = self.control(attempt, Operation::ResetGroup)
            && self.control(attempt, Operation::EnableGroup);
        self.record.attempts[attempt].start_succeeded = success;
        success
    }

    fn finalize(&mut self, read: bool) {
        if self.finalized {
            return;
        }
        let same_owner = self.check_context(if read {
            ContextStage::Finish
        } else {
            ContextStage::DropCleanup
        });
        if !self.fds.iter().any(Option::is_some) {
            self.finalized = true;
            return;
        }
        // During preparation an ID-query panic may precede selection. Every fd
        // already returned by open still belongs to the latest attempt.
        let Some(attempt) = self.record.attempts.len().checked_sub(1) else {
            return;
        };
        // After fork an inherited descriptor can refer to the original owner's
        // event. Close this process's reference, but never disable/reset/read it.
        if !same_owner {
            self.close_all(attempt);
            self.finalized = true;
            return;
        }
        if self.start_called && !self.disable_attempted {
            self.disable_attempted = true;
            self.control(attempt, Operation::DisableGroup);
        }
        if self.context_failed {
            self.close_all(attempt);
            self.finalized = true;
            return;
        }
        for index in 0..self.fds.len() {
            if !self.start_called {
                self.record.attempts[attempt].events[index].status = EventStatus::NotStarted;
                continue;
            }
            if !read {
                continue;
            }
            let Some(fd) = self.fds[index] else {
                continue;
            };
            let response = self.calls.read(fd);
            self.log(
                attempt,
                index,
                Operation::Read,
                response
                    .as_ref()
                    .map(|bytes| bytes.len() as i64)
                    .map_err(Clone::clone),
            );
            let event = &mut self.record.attempts[attempt].events[index];
            let read_status = match response {
                Err(_) => EventStatus::ReadFailed,
                Ok(bytes) => {
                    event.read_bytes = Some(bytes.len());
                    if bytes.len() != READ_BYTES {
                        EventStatus::ShortRead
                    } else {
                        let number = |offset| {
                            u64::from_ne_bytes(
                                bytes[offset..offset + 8]
                                    .try_into()
                                    .expect("fixed read size"),
                            )
                        };
                        let raw = RawTuple {
                            value: number(0),
                            time_enabled_ns: number(8),
                            time_running_ns: number(16),
                            event_id: number(24),
                        };
                        let assessment = assess(
                            raw,
                            event.kernel_event_id.expect("identified prepared event"),
                        );
                        event.raw = Some(raw);
                        event.tuple_assessment = Some(assessment);
                        assessment.status
                    }
                }
            };
            if event.status != EventStatus::ControlFailed {
                event.status = read_status;
            }
        }
        self.close_all(attempt);
        // Set only after cleanup. A read/control unwind leaves still-owned FDs
        // visible to Drop, without retrying a previously attempted disable.
        self.finalized = true;
    }

    fn finish(mut self) -> CollectionRecord {
        self.finalize(true);
        self.record.clone()
    }
}

impl<C: Calls> Drop for Collector<C> {
    fn drop(&mut self) {
        self.finalize(false);
    }
}

#[cfg(test)]
mod tests;
