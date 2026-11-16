//! Syntax-only admission of a manifest-bound supervisor interruption envelope.
//! The caller retains the original Value, authenticates its raw manifest/context,
//! and supplies the disposition derived from the admitted original configuration.
//! Success is neither proof of interruption cause nor authority to export rows.
use super::wire;
use serde_json::Value;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    Timeout,
    TraceBudgetExhausted,
    WorkerFailed,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    Shape,
    Disposition,
    Contradiction,
    Cap,
    Depth,
    Overflow,
    Owner,
    Deadline,
}
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub work: u64,
    pub depth: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Envelope {
    pub reason: Reason,
    pub exit_code: Option<i32>,
}
#[derive(Debug)]
pub struct Audit {
    pub result: Result<Envelope, Failure>,
    pub charged_work: u64,
}
struct Meter<'a, O> {
    owner: &'a mut O,
    deadline: Instant,
    limits: Limits,
    work: u64,
}
impl<O: wire::Admission> Meter<'_, O> {
    fn pay(&mut self, work: u64) -> Result<(), Failure> {
        if Instant::now() >= self.deadline {
            return Err(Failure::Deadline);
        }
        self.owner.check().map_err(|_| Failure::Owner)?;
        if Instant::now() >= self.deadline {
            return Err(Failure::Deadline);
        }
        let next = self.work.checked_add(work).ok_or(Failure::Overflow)?;
        if next > self.limits.work {
            return Err(Failure::Cap);
        }
        self.owner.reserve(work, 0).map_err(|_| Failure::Owner)?;
        self.work = next; // A successful reservation is retained even on postcheck failure.
        self.owner.check().map_err(|_| Failure::Owner)?;
        if Instant::now() >= self.deadline {
            return Err(Failure::Deadline);
        }
        Ok(())
    }
    // Prepay every byte comparison; do not use unmetered Value equality on an
    // arbitrarily large caller-owned disposition. No heap allocations or clones.
    fn equal(&mut self, a: &Value, b: &Value, depth: usize) -> Result<bool, Failure> {
        self.pay(1)?;
        if depth > self.limits.depth.min(64) {
            return Err(Failure::Depth);
        }
        Ok(match (a, b) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => {
                self.pay(a.len().checked_add(b.len()).ok_or(Failure::Overflow)? as u64)?;
                a == b
            }
            (Value::Array(a), Value::Array(b)) => {
                if a.len() != b.len() {
                    false
                } else {
                    for (a, b) in a.iter().zip(b) {
                        if !self.equal(a, b, depth + 1)? {
                            return Ok(false);
                        }
                    }
                    true
                }
            }
            (Value::Object(a), Value::Object(b)) => {
                if a.len() != b.len() {
                    false
                } else {
                    // Iteration order may differ under preserve_order. Bounded
                    // pairwise search preserves serde_json object equality.
                    for (key, value) in a {
                        let mut found = false;
                        for (other, expected) in b {
                            self.pay(
                                key.len()
                                    .checked_add(other.len())
                                    .and_then(|n| n.checked_add(1))
                                    .ok_or(Failure::Overflow)?
                                    as u64,
                            )?;
                            if key == other {
                                if !self.equal(value, expected, depth + 1)? {
                                    return Ok(false);
                                }
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            return Ok(false);
                        }
                    }
                    true
                }
            }
            _ => false,
        })
    }
}
fn exit(v: &Value) -> Result<Option<i32>, Failure> {
    if v.is_null() {
        return Ok(None);
    }
    v.as_i64()
        .and_then(|n| i32::try_from(n).ok())
        .map(Some)
        .ok_or(Failure::Shape)
}
fn run<O: wire::Admission>(
    outcome: &Value,
    expected: Option<&Value>,
    m: &mut Meter<'_, O>,
) -> Result<Envelope, Failure> {
    m.pay(1)?;
    let obj = outcome.as_object().ok_or(Failure::Shape)?;
    // Reject reserved report markers, unsupported labels and unknown extensions.
    // No generic error/budget_exhausted envelope acquires this narrow scope.
    for (key, value) in obj {
        m.pay(
            (key.len() as u64)
                .checked_add(256)
                .ok_or(Failure::Overflow)?,
        )?;
        match key.as_str() {
            "status" | "solver_completed" | "exit_code" | "requested_channels" | "supervisor" => {}
            "scalar_recovered"
            | "scalar_recovered_without_witness"
            | "ecdlp_pipeline_completed"
            | "all_patterns_covered"
            | "all_patterns_negative"
            | "full_span_search"
            | "training_export_admitted" => {
                if value != &Value::Bool(false) {
                    return Err(Failure::Contradiction);
                }
            }
            "first_fall_degree" | "last_fall_degree" | "degree_of_regularity" => {
                if !value.is_null() {
                    return Err(Failure::Contradiction);
                }
            }
            _ => return Err(Failure::Shape),
        }
    }
    let reason = match obj.get("status").and_then(Value::as_str) {
        Some("timeout") => Reason::Timeout,
        Some("trace_budget_exhausted") => Reason::TraceBudgetExhausted,
        Some("worker_failed") => Reason::WorkerFailed,
        _ => return Err(Failure::Shape),
    };
    if obj.get("solver_completed") != Some(&Value::Bool(false)) {
        return Err(Failure::Contradiction);
    }
    match (obj.get("requested_channels"), expected) {
        (None, None) => {}
        (Some(a), Some(b)) => {
            if !m.equal(a, b, 0)? {
                return Err(Failure::Disposition);
            }
        }
        _ => return Err(Failure::Disposition),
    }
    let supervisor = obj
        .get("supervisor")
        .and_then(Value::as_object)
        .ok_or(Failure::Shape)?;
    if supervisor.len() != 11 {
        return Err(Failure::Shape);
    }
    for (key, value) in supervisor {
        m.pay(
            (key.len() as u64)
                .checked_add(256)
                .ok_or(Failure::Overflow)?,
        )?;
        let valid = match key.as_str() {
            "exit_code" => exit(value).is_ok(),
            "elapsed_ms_diagnostic" => value.as_f64().is_some_and(|n| n.is_finite() && n >= 0.0),
            "clock_valid_for_comparison" | "checkpoint_mid_call" => value == &Value::Bool(false),
            "retired_instructions" => value.is_null(),
            "instruction_counter_status" => value.as_str()
                == Some(
                    "whole_worker_not_counted; see phase telemetry for available hardware counts",
                ),
            "memory_limit_bytes" | "worker_output_bytes_before_seal" | "output_budget_bytes" => {
                value.as_u64().is_some()
            }
            "output_budget_scope" => value.as_str() == Some("all_cell_files_before_manifest_seal"),
            "interrupted_trace_loss_possible" => value == &Value::Bool(true),
            _ => false,
        };
        if !valid {
            return Err(Failure::Shape);
        }
    }
    let exit_code = exit(supervisor.get("exit_code").ok_or(Failure::Shape)?)?;
    if let Some(v) = obj.get("exit_code") {
        // terminal_outcome emits this outer field only for a failed exit.
        if reason != Reason::WorkerFailed || exit(v)? != exit_code || exit_code == Some(0) {
            return Err(Failure::Contradiction);
        }
    }
    m.pay(1)?;
    Ok(Envelope { reason, exit_code })
}
/// Validate only the observed outer form. Inputs remain borrowed and must remain
/// in the caller's retained custody on failure. The caller must prepay JSON parse
/// and storage; this function charges inspection through its existing owner.
/// `expected` is the original config's requested_channel_disposition, not input
/// supplied by the producer. No budget renewal or provenance token is created.
pub fn verify(
    outcome: &Value,
    expected: Option<&Value>,
    limits: Limits,
    deadline: Instant,
    owner: &mut impl wire::Admission,
) -> Audit {
    let mut meter = Meter {
        owner,
        deadline,
        limits,
        work: 0,
    };
    let result = run(outcome, expected, &mut meter);
    Audit {
        result,
        charged_work: meter.work,
    }
}
