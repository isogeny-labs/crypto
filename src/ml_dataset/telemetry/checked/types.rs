//! Strict wire types for producer consistency checks; not a hardware verifier.
use crate::ml_dataset::capture::AttemptBinding;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FixedPolicy {
    pub schema: String,
    pub policy_id: String,
    pub collector_version: String,
    pub task_scope: String,
    pub cycles: String,
    pub counter_interval: String,
    pub usage_scope: String,
    pub estimate_rule: String,
    pub max_policy_bytes: u64,
    pub max_start_bytes: u64,
    pub max_metrics_bytes: u64,
    pub max_error_preview_bytes: u64,
    pub max_attempts: u64,
    pub max_events_per_attempt: u64,
    pub max_calls_per_event: u64,
    pub max_context_checks: u64,
    pub training_cost_eligible: bool,
    pub whole_worker_coverage_verified: bool,
    pub hardware_independently_verified: bool,
    pub platform_admitted: bool,
    pub clock_valid_for_comparison: bool,
}

pub fn required_option<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    Option::deserialize(d)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Phase {
    CurveConstruction,
    KnownAnswerReplay,
    FactorBaseReplay,
    Solver,
}
impl Phase {
    pub const ALL: [Self; 4] = [
        Self::CurveConstruction,
        Self::KnownAnswerReplay,
        Self::FactorBaseReplay,
        Self::Solver,
    ];
    pub fn ordinal(self) -> u8 {
        match self {
            Self::CurveConstruction => 0,
            Self::KnownAnswerReplay => 1,
            Self::FactorBaseReplay => 2,
            Self::Solver => 3,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::CurveConstruction => "curve-construction",
            Self::KnownAnswerReplay => "known-answer-replay",
            Self::FactorBaseReplay => "factor-base-replay",
            Self::Solver => "solver",
        }
    }
    pub fn path(self, start: bool) -> String {
        format!(
            "telemetry/{}{}.json",
            self.name(),
            if start { "-start" } else { "" }
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Usage {
    pub user_cpu_us: u64,
    pub system_cpu_us: u64,
    pub peak_rss_native: u64,
    pub peak_rss_unit: String,
    pub minor_faults: u64,
    pub major_faults: u64,
    pub voluntary_context_switches: u64,
    pub involuntary_context_switches: u64,
    pub scope: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Start {
    pub schema: String,
    pub phase: Phase,
    pub ordinal: u8,
    pub binding: AttemptBinding,
    pub policy_sha256: String,
    pub recording_pid: u32,
    #[serde(deserialize_with = "required_option")]
    pub usage_before: Option<Usage>,
    pub counter_boundary: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorSummary {
    pub message_preview: String,
    pub message_bytes: u64,
    pub message_sha256: String,
    pub truncated: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub event: String,
    pub attempt_index: usize,
    pub event_index: usize,
    pub status: String,
    #[serde(deserialize_with = "required_option")]
    pub raw: Option<u64>,
    #[serde(deserialize_with = "required_option")]
    pub estimate: Option<u64>,
    pub lifecycle_clean: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metrics {
    pub schema: String,
    pub phase: Phase,
    pub ordinal: u8,
    pub binding: AttemptBinding,
    pub policy_sha256: String,
    pub start_sha256: String,
    pub returned_ok: bool,
    #[serde(deserialize_with = "required_option")]
    pub returned_error: Option<ErrorSummary>,
    pub counter_start_succeeded: bool,
    pub collection: Collection,
    pub observations: Vec<Observation>,
    #[serde(deserialize_with = "required_option")]
    pub usage_after: Option<Usage>,
    pub elapsed_ns_diagnostic: String,
    pub training_cost_eligible: bool,
    #[serde(deserialize_with = "required_option")]
    pub whole_worker_retired_instructions: Option<u64>,
    pub whole_worker_coverage_verified: bool,
    pub independently_verified: bool,
    pub platform_admitted: bool,
    pub clock_valid_for_comparison: bool,
    pub interrupted_phase_readback: bool,
    pub consumer_v2_admitted: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub scope: String,
    pub cycles: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    pub pid: u32,
    #[serde(deserialize_with = "required_option")]
    pub tid: Option<u32>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    pub stage: String,
    pub current: Identity,
    pub status: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Collection {
    pub collector_version: String,
    pub policy: Policy,
    pub owner_pid: u32,
    #[serde(deserialize_with = "required_option")]
    pub owner_tid: Option<u32>,
    pub context_checks: Vec<Context>,
    pub os: String,
    pub architecture: String,
    pub attempts: Vec<Attempt>,
    #[serde(deserialize_with = "required_option")]
    pub selected_attempt: Option<usize>,
    pub whole_worker_coverage_verified: bool,
    pub descendant_lifetimes_verified: bool,
    pub complete_execution_verified: bool,
    pub hardware_independently_verified: bool,
    pub platform_admitted: bool,
    pub clock_valid_for_comparison: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Attempt {
    pub mode: String,
    pub events: Vec<Event>,
    pub start_succeeded: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub event: String,
    pub perf_type: u32,
    pub config: u64,
    pub attr_size: u32,
    pub read_format: u64,
    pub attr_flags: u64,
    pub attr_bytes_native_endian: Vec<u8>,
    pub byte_order: String,
    pub pid: i32,
    pub cpu: i32,
    pub group_fd: i32,
    pub syscall_flags: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CallError {
    #[serde(deserialize_with = "required_option")]
    pub errno: Option<i32>,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallResult {
    Ok(i64),
    Err(CallError),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Call {
    pub operation: String,
    pub result: CallResult,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Raw {
    pub value: u64,
    pub time_enabled_ns: u64,
    pub time_running_ns: u64,
    pub event_id: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assessment {
    pub status: String,
    #[serde(deserialize_with = "required_option")]
    pub multiplexed: Option<bool>,
    #[serde(deserialize_with = "required_option")]
    pub scaled_estimate: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub request: Request,
    #[serde(deserialize_with = "required_option")]
    pub kernel_event_id: Option<u64>,
    #[serde(deserialize_with = "required_option")]
    pub identity_error: Option<String>,
    pub lifecycle: Vec<Call>,
    pub status: String,
    #[serde(deserialize_with = "required_option")]
    pub raw: Option<Raw>,
    #[serde(deserialize_with = "required_option")]
    pub tuple_assessment: Option<Assessment>,
    #[serde(deserialize_with = "required_option")]
    pub read_bytes: Option<usize>,
    #[serde(deserialize_with = "required_option")]
    pub cleanup_succeeded: Option<bool>,
}
