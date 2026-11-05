//! Replay the supported collector lifecycle as producer consistency checks.
//! Syscall results are observations; replay never establishes hardware truth.
use super::types::*;

fn require(ok: bool, message: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn context(c: &Collection, x: &Context, stage: &str) -> Result<bool, String> {
    let status = if c.owner_pid == 0
        || x.current.pid == 0
        || !c.owner_tid.is_some_and(|v| v > 0)
        || !x.current.tid.is_some_and(|v| v > 0)
    {
        "unavailable"
    } else if c.owner_pid != x.current.pid || c.owner_tid != x.current.tid {
        "mismatch"
    } else {
        "matched"
    };
    require(
        x.stage == stage && x.status == status,
        "invalid checked counter context",
    )?;
    Ok(status == "matched")
}

struct Replay<'a> {
    original: &'a Event,
    expected: Event,
    cursor: usize,
    fd: Option<i32>,
}
impl<'a> Replay<'a> {
    fn new(e: &'a Event, event: &str, group_fd: i32) -> Result<Self, String> {
        let r = &e.request;
        let config = if event == "instructions" { 1u64 } else { 0 };
        let flags = if group_fd == -1 { 97u64 } else { 96 };
        require(
            matches!(r.byte_order.as_str(), "little" | "big"),
            "counter byte order",
        )?;
        let u32bytes = |x: u32| {
            if r.byte_order == "little" {
                x.to_le_bytes()
            } else {
                x.to_be_bytes()
            }
        };
        let u64bytes = |x: u64| {
            if r.byte_order == "little" {
                x.to_le_bytes()
            } else {
                x.to_be_bytes()
            }
        };
        let mut attr = vec![0; 112];
        attr[4..8].copy_from_slice(&u32bytes(112));
        attr[8..16].copy_from_slice(&u64bytes(config));
        attr[32..40].copy_from_slice(&u64bytes(7));
        attr[40..48].copy_from_slice(&u64bytes(flags));
        require(
            r.event == event
                && r.perf_type == 0
                && r.config == config
                && r.attr_size == 112
                && r.read_format == 7
                && r.attr_flags == flags
                && r.attr_bytes_native_endian == attr
                && r.pid == 0
                && r.cpu == -1
                && r.group_fd == group_fd
                && r.syscall_flags == 8,
            "counter request differs from fixed policy",
        )?;
        require(e.lifecycle.len() <= 12, "counter lifecycle exceeds bound")?;
        let mut expected = e.clone();
        expected.kernel_event_id = None;
        expected.identity_error = None;
        expected.status = "not_attempted".into();
        expected.raw = None;
        expected.tuple_assessment = None;
        expected.read_bytes = None;
        expected.cleanup_succeeded = None;
        Ok(Self {
            original: e,
            expected,
            cursor: 0,
            fd: None,
        })
    }
    fn call(&mut self, operation: &str) -> Result<CallResult, String> {
        let call = self
            .original
            .lifecycle
            .get(self.cursor)
            .ok_or("missing counter lifecycle call")?;
        require(call.operation == operation, "counter lifecycle order")?;
        self.cursor += 1;
        if let CallResult::Err(error) = &call.result {
            require(error.message.len() <= 4096, "counter error exceeds bound")?;
        }
        Ok(call.result.clone())
    }
    fn zero_call(&mut self, operation: &str) -> Result<bool, String> {
        match self.call(operation)? {
            CallResult::Ok(0) => Ok(true),
            CallResult::Err(_) => Ok(false),
            _ => Err("counter control success must be zero".into()),
        }
    }
    fn prepare(&mut self, preceding_id: Option<u64>) -> Result<bool, String> {
        match self.call("open")? {
            CallResult::Ok(fd) => {
                self.fd = Some(i32::try_from(fd).map_err(|_| "counter fd overflow")?);
                require(fd >= 0, "negative successful counter fd")?;
            }
            CallResult::Err(error) => {
                self.expected.status = match error.errno {
                    Some(1 | 13) => "permission_denied",
                    Some(19 | 2 | 95) => "unsupported_event",
                    _ => "open_failed",
                }
                .into();
                return Ok(false);
            }
        }
        if self.zero_call("event_id")? {
            let id = self
                .original
                .kernel_event_id
                .ok_or("successful ID call omitted identity")?;
            self.expected.kernel_event_id = Some(id);
            if preceding_id == Some(id) {
                self.expected.identity_error = Some("duplicate kernel event ID in group".into());
                self.expected.status = "identity_failed".into();
                return Ok(false);
            }
            self.expected.status = "prepared".into();
            Ok(true)
        } else {
            self.expected.status = "identity_failed".into();
            Ok(false)
        }
    }
    fn close(&mut self) -> Result<(), String> {
        if self.fd.take().is_some() {
            self.expected.cleanup_succeeded = Some(self.zero_call("close")?);
        }
        Ok(())
    }
    fn read(&mut self) -> Result<(), String> {
        let status = match self.call("read")? {
            CallResult::Err(_) => "read_failed".to_owned(),
            CallResult::Ok(bytes) => {
                let bytes = usize::try_from(bytes).map_err(|_| "negative counter read")?;
                self.expected.read_bytes = Some(bytes);
                if bytes != 32 {
                    "short_read".to_owned()
                } else {
                    let raw = self.original.raw.clone().ok_or("counter tuple absent")?;
                    let a = assess(
                        &raw,
                        self.expected.kernel_event_id.ok_or("counter ID absent")?,
                    );
                    self.expected.raw = Some(raw);
                    self.expected.tuple_assessment = Some(a.clone());
                    a.status
                }
            }
        };
        if self.expected.status != "control_failed" {
            self.expected.status = status;
        }
        Ok(())
    }
    fn finish(self) -> Result<(), String> {
        require(
            self.cursor == self.original.lifecycle.len() && self.expected == *self.original,
            "counter record disagrees with lifecycle replay",
        )
    }
}

fn assess(raw: &Raw, id: u64) -> Assessment {
    let mut a = Assessment {
        status: "invalid_tuple".into(),
        multiplexed: None,
        scaled_estimate: None,
    };
    if raw.event_id != id {
        a.status = "identity_mismatch".into();
    } else if raw.time_running_ns > raw.time_enabled_ns {
    } else if raw.time_running_ns == 0 {
        a.status = "not_running".into();
    } else if raw.time_running_ns == raw.time_enabled_ns {
        a.status = "observed_unmultiplexed".into();
        a.multiplexed = Some(false);
    } else {
        a.multiplexed = Some(true);
        match u64::try_from(
            u128::from(raw.value) * u128::from(raw.time_enabled_ns)
                / u128::from(raw.time_running_ns),
        ) {
            Ok(v) => {
                a.status = "observed_multiplexed_estimate".into();
                a.scaled_estimate = Some(v);
            }
            Err(_) => a.status = "estimate_overflow".into(),
        }
    }
    a
}

/// Returns the start-call return value and a recomputed diagnostic projection.
pub(super) fn validate(c: &Collection, pid: u32) -> Result<(bool, Vec<Observation>), String> {
    require(
        c.collector_version == "checked-native-pmu/v1"
            && c.policy.scope == "calling_thread"
            && c.policy.cycles == "optional_with_standalone_fallback"
            && c.owner_pid == pid
            && pid > 0
            && !c.whole_worker_coverage_verified
            && !c.descendant_lifetimes_verified
            && !c.complete_execution_verified
            && !c.hardware_independently_verified
            && !c.platform_admitted
            && !c.clock_valid_for_comparison,
        "checked collector identity, policy or claims",
    )?;
    require(
        !c.os.is_empty()
            && c.os.len() <= 64
            && !c.architecture.is_empty()
            && c.architecture.len() <= 64,
        "checked platform identity",
    )?;
    require(
        c.context_checks.len() == 2 && (1..=2).contains(&c.attempts.len()),
        "checked collector cardinality",
    )?;
    let start_matched = context(c, &c.context_checks[0], "start")?;
    let finish_matched = context(c, &c.context_checks[1], "finish")?;
    let byte_order = c.attempts[0]
        .events
        .first()
        .ok_or("empty checked attempt")?
        .request
        .byte_order
        .as_str();
    require(
        matches!(byte_order, "little" | "big")
            && !(c.architecture == "x86_64" && byte_order != "little")
            && c.attempts
                .iter()
                .flat_map(|a| &a.events)
                .all(|e| e.request.byte_order == byte_order),
        "inconsistent or impossible collector byte order",
    )?;
    // Native perf bitfield layout is admitted only on the retained LE ABI.
    let supported = c.os == "linux"
        && matches!(c.architecture.as_str(), "x86_64" | "aarch64")
        && c.attempts[0]
            .events
            .first()
            .is_some_and(|e| e.request.byte_order == "little");
    if !supported {
        require(
            c.owner_tid.is_none() && c.context_checks.iter().all(|x| x.current.tid.is_none()),
            "unsupported native backend cannot report a thread identity",
        )?;
    }
    let mut selected = None;
    let mut start_returned = false;
    let mut fallback_required = false;
    for (ai, a) in c.attempts.iter().enumerate() {
        require(
            (ai == 0 && a.mode == "instructions_and_cycles_group" && a.events.len() == 2)
                || (ai == 1
                    && fallback_required
                    && a.mode == "standalone_instructions_fallback"
                    && a.events.len() == 1),
            "checked attempt or fallback order",
        )?;
        let mut events = vec![Replay::new(&a.events[0], "instructions", -1)?];
        let mut prepared = supported && events[0].prepare(None)?;
        if a.events.len() == 2 {
            let group_fd = if prepared {
                events[0].fd.ok_or("prepared leader missing")?
            } else {
                -1
            };
            let mut member = Replay::new(&a.events[1], "cpu_cycles", group_fd)?;
            if prepared {
                prepared = member.prepare(events[0].expected.kernel_event_id)?;
                require(
                    member.fd.is_none() || member.fd != events[0].fd,
                    "live counter descriptors alias",
                )?;
            }
            events.push(member);
        }
        if !supported {
            for e in &mut events {
                e.expected.status = "unsupported_platform".into();
            }
        }
        let mut succeeded_after_finish = false;
        if prepared {
            selected = Some(ai);
            require(
                ai + 1 == c.attempts.len(),
                "attempt after successful preparation",
            )?;
            let start_called = start_matched;
            if !start_matched {
                for e in &mut events {
                    e.expected.status = format!("context_{}", c.context_checks[0].status);
                }
            } else {
                let reset = events[0].zero_call("reset_group")?;
                let enable = reset && events[0].zero_call("enable_group")?;
                start_returned = reset && enable;
                succeeded_after_finish = start_returned;
                if !start_returned {
                    for e in &mut events {
                        e.expected.status = "control_failed".into();
                    }
                }
            }
            if !finish_matched {
                succeeded_after_finish = false;
                for e in &mut events {
                    e.expected.status = format!("context_{}", c.context_checks[1].status);
                }
            } else {
                if start_called && !events[0].zero_call("disable_group")? {
                    for e in &mut events {
                        e.expected.status = "control_failed".into();
                    }
                }
                if start_matched {
                    for e in &mut events {
                        e.read()?;
                    }
                }
            }
        } else {
            for e in &mut events {
                if e.expected.status == "prepared" {
                    e.expected.status = "cancelled_before_start".into();
                }
            }
        }
        for e in events.iter_mut().rev() {
            e.close()?;
        }
        require(
            a.start_succeeded == succeeded_after_finish,
            "checked start success mismatch",
        )?;
        fallback_required = ai == 0
            && !prepared
            && events.len() == 2
            && events[0].expected.kernel_event_id.is_some()
            && events
                .iter()
                .all(|e| e.expected.cleanup_succeeded != Some(false))
            && matches!(
                events[1].expected.status.as_str(),
                "open_failed" | "unsupported_event" | "permission_denied" | "identity_failed"
            );
        if ai == 0 {
            require(
                c.attempts.len() == if fallback_required { 2 } else { 1 },
                "omitted or gratuitous fallback",
            )?;
        }
        for e in events {
            e.finish()?;
        }
    }
    require(
        c.selected_attempt == selected,
        "checked selected attempt mismatch",
    )?;
    let mut observations = Vec::new();
    if let Some(ai) = selected {
        let a = &c.attempts[ai];
        for (ei, e) in a.events.iter().enumerate() {
            let clean = start_returned
                && start_matched
                && finish_matched
                && a.start_succeeded
                && a.events
                    .iter()
                    .all(|event| event.cleanup_succeeded == Some(true))
                && a.events[0]
                    .lifecycle
                    .iter()
                    .filter(|x| {
                        matches!(
                            x.operation.as_str(),
                            "reset_group" | "enable_group" | "disable_group"
                        )
                    })
                    .all(|x| matches!(x.result, CallResult::Ok(0)))
                && e.lifecycle
                    .iter()
                    .all(|x| matches!(x.result, CallResult::Ok(_)));
            let mut o = Observation {
                event: e.request.event.clone(),
                attempt_index: ai,
                event_index: ei,
                status: "unavailable".into(),
                raw: None,
                estimate: None,
                lifecycle_clean: clean,
            };
            if clean && e.status == "observed_unmultiplexed" {
                o.status = "scoped_raw".into();
                o.raw = e.raw.as_ref().map(|r| r.value);
            } else if clean && e.status == "observed_multiplexed_estimate" {
                o.status = "scoped_estimate".into();
                o.raw = e.raw.as_ref().map(|r| r.value);
                o.estimate = e.tuple_assessment.as_ref().and_then(|a| a.scaled_estimate);
            }
            observations.push(o);
        }
    }
    Ok((start_returned, observations))
}
