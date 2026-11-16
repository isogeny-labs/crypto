//! Independent finite-state check of recorded syscalls, never hardware execution.
use super::schema::*;
use anyhow::{ensure, Context as _, Result};

fn tuple(raw: &Raw, id: u64) -> Assessment {
    let (status, multiplexed, scaled_estimate) = if raw.event_id != id {
        ("identity_mismatch", None, None)
    } else if raw.time_running_ns > raw.time_enabled_ns {
        ("invalid_tuple", None, None)
    } else if raw.time_running_ns == 0 {
        ("not_running", None, None)
    } else if raw.time_running_ns == raw.time_enabled_ns {
        ("observed_unmultiplexed", Some(false), None)
    } else {
        let quotient =
            (raw.value as u128) * (raw.time_enabled_ns as u128) / (raw.time_running_ns as u128);
        match u64::try_from(quotient) {
            Ok(n) => ("observed_multiplexed_estimate", Some(true), Some(n)),
            Err(_) => ("estimate_overflow", Some(true), None),
        }
    };
    Assessment {
        status: status.into(),
        multiplexed,
        scaled_estimate,
    }
}

fn request(r: &Request, event: &str, group: i32) -> Result<()> {
    ensure!(
        matches!(r.byte_order.as_str(), "little" | "big"),
        "phase native byte order"
    );
    let config = u64::from(event == "instructions");
    let flags = if group == -1 { 97u64 } else { 96 };
    let mut expected = [0u8; 112];
    let le = r.byte_order == "little";
    expected[4..8].copy_from_slice(&if le {
        112u32.to_le_bytes()
    } else {
        112u32.to_be_bytes()
    });
    for (offset, n) in [(8, config), (32, 7), (40, flags)] {
        expected[offset..offset + 8].copy_from_slice(&if le {
            n.to_le_bytes()
        } else {
            n.to_be_bytes()
        });
    }
    ensure!(
        r.event == event
            && r.perf_type == 0
            && r.config == config
            && r.attr_size == 112
            && r.read_format == 7
            && r.attr_flags == flags
            && r.attr_bytes_native_endian == expected
            && r.pid == 0
            && r.cpu == -1
            && r.group_fd == group
            && r.syscall_flags == 8,
        "phase native fixed attr/request mismatch"
    );
    Ok(())
}

/// Each cursor consumes a legal trace, keeping only independently derived fields.
struct EventState<'a> {
    record: &'a Event,
    position: usize,
    fd: Option<i32>,
    id: Option<u64>,
    identity_error: Option<&'static str>,
    status: &'static str,
    read_bytes: Option<usize>,
    raw_seen: bool,
    assessment: Option<Assessment>,
    cleanup: Option<bool>,
}
impl<'a> EventState<'a> {
    fn new(record: &'a Event, event: &str, group: i32) -> Result<Self> {
        request(&record.request, event, group)?;
        ensure!(record.lifecycle.len() <= 12, "phase native call limit");
        Ok(Self {
            record,
            position: 0,
            fd: None,
            id: None,
            identity_error: None,
            status: "not_attempted",
            read_bytes: None,
            raw_seen: false,
            assessment: None,
            cleanup: None,
        })
    }
    fn next(&mut self, operation: &str) -> Result<&'a CallResult> {
        let call = self
            .record
            .lifecycle
            .get(self.position)
            .context("phase native missing call")?;
        ensure!(
            call.operation == operation,
            "phase native lifecycle order: expected {operation}"
        );
        if let CallResult::Err(error) = &call.result {
            ensure!(error.message.len() <= 4096, "phase native error bound");
        }
        self.position += 1;
        Ok(&call.result)
    }
    fn control(&mut self, operation: &str) -> Result<bool> {
        match self.next(operation)? {
            CallResult::Ok(0) => Ok(true),
            CallResult::Err(_) => Ok(false),
            _ => anyhow::bail!("phase native nonzero successful control"),
        }
    }
    fn prepare(&mut self, preceding: Option<u64>) -> Result<bool> {
        match self.next("open")? {
            CallResult::Ok(fd) => {
                ensure!(*fd >= 0, "phase native negative descriptor");
                self.fd = Some(i32::try_from(*fd)?);
            }
            CallResult::Err(error) => {
                self.status = match error.errno {
                    Some(1 | 13) => "permission_denied",
                    Some(2 | 19 | 95) => "unsupported_event",
                    _ => "open_failed",
                };
                return Ok(false);
            }
        }
        if !self.control("event_id")? {
            self.status = "identity_failed";
            return Ok(false);
        }
        self.id = Some(
            self.record
                .kernel_event_id
                .context("phase native successful ID missing")?,
        );
        if self.id == preceding {
            self.identity_error = Some("duplicate kernel event ID in group");
            self.status = "identity_failed";
            Ok(false)
        } else {
            self.status = "prepared";
            Ok(true)
        }
    }
    fn readback(&mut self) -> Result<()> {
        let read_status = match self.next("read")? {
            CallResult::Err(_) => "read_failed",
            CallResult::Ok(bytes) => {
                let n = usize::try_from(*bytes).context("phase native negative read length")?;
                self.read_bytes = Some(n);
                if n != 32 {
                    "short_read"
                } else {
                    let raw = self
                        .record
                        .raw
                        .as_ref()
                        .context("phase native tuple absent")?;
                    let a = tuple(
                        raw,
                        self.id.context("phase native tuple lacks prepared ID")?,
                    );
                    let status = match a.status.as_str() {
                        "identity_mismatch" => "identity_mismatch",
                        "invalid_tuple" => "invalid_tuple",
                        "not_running" => "not_running",
                        "observed_unmultiplexed" => "observed_unmultiplexed",
                        "observed_multiplexed_estimate" => "observed_multiplexed_estimate",
                        "estimate_overflow" => "estimate_overflow",
                        _ => unreachable!(),
                    };
                    self.raw_seen = true;
                    self.assessment = Some(a);
                    status
                }
            }
        };
        if self.status != "control_failed" {
            self.status = read_status;
        }
        Ok(())
    }
    fn close(&mut self) -> Result<()> {
        if self.fd.is_some() {
            self.cleanup = Some(self.control("close")?);
        }
        Ok(())
    }
    fn verify(&self) -> Result<()> {
        let e = self.record;
        ensure!(
            self.position == e.lifecycle.len(),
            "phase native extra lifecycle call"
        );
        ensure!(
            e.kernel_event_id == self.id
                && e.identity_error.as_deref() == self.identity_error
                && e.status == self.status
                && e.read_bytes == self.read_bytes
                && e.raw.is_some() == self.raw_seen
                && e.tuple_assessment == self.assessment
                && e.cleanup_succeeded == self.cleanup,
            "phase native derived state disagreement"
        );
        Ok(())
    }
}

fn context(c: &Collection, x: &Context, stage: &str) -> Result<bool> {
    let status = if c.owner_pid == 0
        || x.current.pid == 0
        || !c.owner_tid.is_some_and(|n| n > 0)
        || !x.current.tid.is_some_and(|n| n > 0)
    {
        "unavailable"
    } else if c.owner_pid != x.current.pid || c.owner_tid != x.current.tid {
        "mismatch"
    } else {
        "matched"
    };
    ensure!(
        x.stage == stage && x.status == status,
        "phase native context mismatch"
    );
    Ok(status == "matched")
}
fn context_failure(x: &Context) -> &'static str {
    if x.status == "mismatch" {
        "context_mismatch"
    } else {
        "context_unavailable"
    }
}

pub(super) fn validate(c: &Collection, pid: u32) -> Result<(bool, Vec<Observation>)> {
    ensure!(
        c.collector_version == "checked-native-pmu/v1"
            && c.policy.scope == "calling_thread"
            && c.policy.cycles == "optional_with_standalone_fallback"
            && c.owner_pid == pid
            && pid > 0,
        "phase native collector identity/policy"
    );
    ensure!(
        !c.whole_worker_coverage_verified
            && !c.descendant_lifetimes_verified
            && !c.complete_execution_verified
            && !c.hardware_independently_verified
            && !c.platform_admitted
            && !c.clock_valid_for_comparison,
        "phase native prohibited claim"
    );
    ensure!(
        !c.os.is_empty()
            && c.os.len() <= 64
            && !c.architecture.is_empty()
            && c.architecture.len() <= 64,
        "phase native platform identifier"
    );
    ensure!(
        c.context_checks.len() == 2 && (1..=2).contains(&c.attempts.len()),
        "phase native collection cardinality"
    );
    let start_context = context(c, &c.context_checks[0], "start")?;
    let finish_context = context(c, &c.context_checks[1], "finish")?;
    let order = &c.attempts[0]
        .events
        .first()
        .context("phase native empty attempt")?
        .request
        .byte_order;
    ensure!(
        matches!(order.as_str(), "little" | "big")
            && !(c.architecture == "x86_64" && order != "little")
            && c.attempts
                .iter()
                .flat_map(|a| &a.events)
                .all(|e| &e.request.byte_order == order),
        "phase native byte order consistency"
    );
    let supported = c.os == "linux"
        && matches!(c.architecture.as_str(), "x86_64" | "aarch64")
        && order == "little";
    if !supported {
        ensure!(
            c.owner_tid.is_none() && c.context_checks.iter().all(|x| x.current.tid.is_none()),
            "phase native unsupported TID"
        );
    }
    let mut selected = None;
    let mut started = false;
    let mut fallback = false;
    for (ai, a) in c.attempts.iter().enumerate() {
        ensure!(
            (ai == 0 && a.mode == "instructions_and_cycles_group" && a.events.len() == 2)
                || (ai == 1
                    && fallback
                    && a.mode == "standalone_instructions_fallback"
                    && a.events.len() == 1),
            "phase native attempt/fallback shape"
        );
        let mut leader = EventState::new(&a.events[0], "instructions", -1)?;
        let leader_prepared = supported && leader.prepare(None)?;
        let mut member = if a.events.len() == 2 {
            Some(EventState::new(
                &a.events[1],
                "cpu_cycles",
                if leader_prepared {
                    leader.fd.context("phase native leader FD")?
                } else {
                    -1
                },
            )?)
        } else {
            None
        };
        let ready = if leader_prepared {
            if let Some(e) = member.as_mut() {
                let ready = e.prepare(leader.id)?;
                ensure!(
                    e.fd.is_none() || e.fd != leader.fd,
                    "phase native simultaneous descriptor alias"
                );
                ready
            } else {
                true
            }
        } else {
            false
        };
        let set_status =
            |leader: &mut EventState<'_>, member: &mut Option<EventState<'_>>, status| {
                leader.status = status;
                if let Some(e) = member {
                    e.status = status;
                }
            };
        if !supported {
            set_status(&mut leader, &mut member, "unsupported_platform");
        }
        let mut retained_start = false;
        if ready {
            selected = Some(ai);
            ensure!(
                ai + 1 == c.attempts.len(),
                "phase native attempt after preparation"
            );
            if start_context {
                started = leader.control("reset_group")? && leader.control("enable_group")?;
                retained_start = started;
                if !started {
                    set_status(&mut leader, &mut member, "control_failed");
                }
            } else {
                set_status(
                    &mut leader,
                    &mut member,
                    context_failure(&c.context_checks[0]),
                );
            }
            if !finish_context {
                retained_start = false;
                set_status(
                    &mut leader,
                    &mut member,
                    context_failure(&c.context_checks[1]),
                );
            } else if start_context {
                if !leader.control("disable_group")? {
                    set_status(&mut leader, &mut member, "control_failed");
                }
                leader.readback()?;
                if let Some(e) = member.as_mut() {
                    e.readback()?;
                }
            }
        } else {
            if leader.status == "prepared" {
                leader.status = "cancelled_before_start";
            }
            if let Some(e) = member.as_mut() {
                if e.status == "prepared" {
                    e.status = "cancelled_before_start";
                }
            }
        }
        if let Some(e) = member.as_mut() {
            e.close()?;
        }
        leader.close()?;
        ensure!(
            a.start_succeeded == retained_start,
            "phase native recorded start disagreement"
        );
        fallback = ai == 0
            && !ready
            && leader.id.is_some()
            && leader.cleanup != Some(false)
            && member.as_ref().is_some_and(|e| {
                e.cleanup != Some(false)
                    && matches!(
                        e.status,
                        "open_failed"
                            | "unsupported_event"
                            | "permission_denied"
                            | "identity_failed"
                    )
            });
        if ai == 0 {
            ensure!(
                c.attempts.len() == if fallback { 2 } else { 1 },
                "phase native missing or gratuitous fallback"
            );
        }
        leader.verify()?;
        if let Some(e) = member {
            e.verify()?;
        }
    }
    ensure!(
        c.selected_attempt == selected,
        "phase native selection mismatch"
    );
    let mut observations = Vec::with_capacity(2);
    if let Some(ai) = selected {
        let a = &c.attempts[ai];
        let common_clean = started
            && start_context
            && finish_context
            && a.start_succeeded
            && a.events.iter().all(|e| e.cleanup_succeeded == Some(true))
            && a.events[0]
                .lifecycle
                .iter()
                .filter(|x| {
                    matches!(
                        x.operation.as_str(),
                        "reset_group" | "enable_group" | "disable_group"
                    )
                })
                .all(|x| matches!(x.result, CallResult::Ok(0)));
        for (ei, e) in a.events.iter().enumerate() {
            let clean = common_clean
                && e.lifecycle
                    .iter()
                    .all(|x| matches!(x.result, CallResult::Ok(_)));
            let (status, raw, estimate) = if clean && e.status == "observed_unmultiplexed" {
                ("scoped_raw", e.raw.as_ref().map(|r| r.value), None)
            } else if clean && e.status == "observed_multiplexed_estimate" {
                (
                    "scoped_estimate",
                    e.raw.as_ref().map(|r| r.value),
                    e.tuple_assessment.as_ref().and_then(|a| a.scaled_estimate),
                )
            } else {
                ("unavailable", None, None)
            };
            observations.push(Observation {
                event: e.request.event.clone(),
                attempt_index: ai,
                event_index: ei,
                status: status.into(),
                raw,
                estimate,
                lifecycle_clean: clean,
            });
        }
    }
    Ok((started, observations))
}
