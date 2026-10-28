use serde::{Deserialize, Serialize};

pub const SCHEMA: &str = "crypto.boolean-f4-suffix-trace/v1";
pub const PROFILE: &str = "boolean-macaulay-f4-suffix/v1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub source_commit: String,
    pub producer_sha256: String,
    pub configuration_sha256: String,
    pub context_sha256: String,
    pub call_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub max_candidates: u64,
    pub max_events: u64,
    pub max_stream_bytes: u64,
    pub max_event_bytes: u64,
    pub max_workspace_bytes: u64,
    pub max_memory_bytes: u64,
    pub max_admission_units: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Claims {
    pub input_encoding_checked: bool,
    pub matrix_rank_independently_verified: bool,
    pub execution_stream_complete: bool,
    pub total_operation_count_verified: bool,
    pub unsat_verified: bool,
    pub groebner_basis_complete: bool,
    pub first_fall_degree: Option<u64>,
    pub last_fall_degree: Option<u64>,
}

impl Claims {
    pub(crate) fn unclaimed() -> Self {
        Self {
            input_encoding_checked: false,
            matrix_rank_independently_verified: false,
            execution_stream_complete: false,
            total_operation_count_verified: false,
            unsat_verified: false,
            groebner_basis_complete: false,
            first_fall_degree: None,
            last_fall_degree: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum StopReason {
    InvalidInput { detail: String },
    UnsupportedProfile { detail: String },
    CandidateLimit,
    EventLimit,
    StreamByteLimit,
    EventByteLimit,
    WorkspaceLimit,
    MemoryLimit,
    AdmissionLimit,
    NativeSizeLimit,
    Deadline,
    Cancelled,
    SinkFailure { detail: String },
    Integrity { detail: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Event {
    Begin {
        profile: String,
        binding: Binding,
        n_vars: u8,
        degree: u8,
        input_count: u64,
        input_sha256: String,
        limits: Limits,
        native_row_cap: u64,
        native_column_cap: u64,
        kernel: String,
        ring: String,
        order: String,
    },
    Input {
        index: u64,
        terms: Vec<u8>,
    },
    Candidate {
        index: u64,
        generator: u64,
        multiplier: u8,
        terms: Vec<u8>,
        matrix_row: Option<u64>,
    },
    Columns {
        monomials: Vec<u8>,
    },
    MatrixRow {
        row: u64,
        version: u64,
        words: Vec<String>,
    },
    MatrixReady {
        rows: u64,
        columns: u64,
        candidates: u64,
    },
    Pivot {
        column: u64,
        pivot_row: u64,
        selected_row: Option<u64>,
    },
    Swap {
        left: u64,
        right: u64,
        left_version: u64,
        right_version: u64,
        left_after_version: u64,
        right_after_version: u64,
    },
    RowXor {
        target: u64,
        pivot: u64,
        target_version: u64,
        pivot_version: u64,
        after_version: u64,
        start_word: u64,
        words_after: Vec<String>,
    },
    OutputRow {
        row: u64,
        version: u64,
        words: Vec<String>,
    },
    End {
        rank_native: u64,
        word_xors_native: u64,
        rows: u64,
        columns: u64,
        candidates: u64,
        admission_units: u64,
        claims: Claims,
    },
    Stopped {
        reason: StopReason,
        acknowledged_before_stop: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub schema: String,
    pub sequence: u64,
    pub previous_sha256: String,
    pub event: Event,
}
