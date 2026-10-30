//! Independent wire definitions. Field order is part of native byte commitments.
use serde::{Deserialize, Serialize};
use serde_json::Value;
macro_rules! record {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $name { $(pub $field: $ty),* }
    };
}
record!(Partial { id:u64, small:Vec<(u32,u64)>, large:Vec<(u64,u64)>, rhs:u64 });
record!(Complete { small:Vec<(u32,u64)>, rhs:u64, sources:Vec<(u64,u64)> });
record!(Options {
    modulus: u64,
    max_small_weight: usize,
    max_sources: usize
});
record!(Limits {
    input_rows: usize,
    input_terms: usize,
    row_terms: usize,
    events: u64,
    bytes: u64,
    work: u64
});
impl Limits {
    pub fn policy() -> Self {
        Self {
            input_rows: 4096,
            input_terms: 1_000_000,
            row_terms: 65_536,
            events: 100_000,
            bytes: 16 * 1024 * 1024,
            work: 1_000_000,
        }
    }
}
record!(Report {
    input_relations: usize,
    input_large_vertices: usize,
    duplicate_relations: usize,
    peeled_relations: usize,
    core_relations: usize,
    core_large_vertices: usize,
    pivots: usize,
    merges: usize,
    rejected_fill: usize,
    complete_relations: usize,
    cycles_closed: usize
});
record!(RowRef {
    input_id: u64,
    version: u64
});
record!(Row {small:Vec<(u32,u64)>,large:Vec<(u64,u64)>,rhs:u64,sources:Vec<(u64,u64)>});
impl Row {
    pub fn terms(&self) -> usize {
        self.small.len() + self.large.len() + self.sources.len() + 1
    }
    pub fn complete(&self) -> Complete {
        Complete {
            small: self.small.clone(),
            rhs: self.rhs,
            sources: self.sources.clone(),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stop {
    Completed,
    InputLimit,
    StateLimit,
    WorkLimit,
    EventLimit,
    ByteLimit,
    SinkFailure,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    #[default]
    Begin,
    Normalize,
    Duplicate,
    Peel,
    Core,
    Row,
    Select,
    Reduce,
    Store,
    Complete,
    Finish,
}
record!(Reservation {phase:Phase,choice:Option<u64>,work:u64,state_terms:usize});
impl Reservation {
    pub fn new(phase: Phase, work: usize, state_terms: usize) -> Self {
        Self {
            phase,
            choice: None,
            work: work as u64,
            state_terms,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Transition {
    Begin {
        input_sha256: String,
        input_rows: usize,
        input_terms: usize,
        options: Options,
        limits: Limits,
    },
    Normalized {
        position: usize,
        row: Partial,
    },
    DuplicateDropped {
        input_id: u64,
        representative: u64,
    },
    Peel {
        column: u64,
        input_id: u64,
        degrees_before: Vec<(u64, usize)>,
        degrees_after: Vec<(u64, usize)>,
    },
    CoreReady {
        row_ids: Vec<u64>,
        smooth_ids: Vec<u64>,
        degrees: Vec<(u64, usize)>,
    },
    RowBegin {
        reference: RowRef,
        row: Row,
    },
    PivotSelected {
        reference: RowRef,
        column: u64,
        pivot: Option<RowRef>,
        score: usize,
    },
    Reduction {
        reference: RowRef,
        column: u64,
        pivot: RowRef,
        inverse: u64,
        scale: u64,
        candidate: Row,
        accepted: bool,
    },
    PivotStored {
        reference: RowRef,
        column: u64,
        row: Row,
    },
    CompleteRow {
        reference: RowRef,
        row: Row,
    },
    Finish {
        rows: Vec<Complete>,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub sequence: u64,
    pub reservation: Reservation,
    pub transition: Transition,
    pub report_after: Report,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Run {
    pub schema: String,
    pub input_sha256: Option<String>,
    pub input_rows: usize,
    pub input_terms: usize,
    pub options: Options,
    pub limits: Limits,
    pub stop: Stop,
    pub events: Vec<Event>,
    pub unacknowledged: Option<Event>,
    pub next_reservation: Option<Reservation>,
    pub sink_error: Option<String>,
    pub computed_work: u64,
    pub acknowledged_work: u64,
    pub inverse_calls: u64,
    pub acknowledged_inverse_calls: u64,
    pub acknowledged_bytes: u64,
    pub unacknowledged_bytes: u64,
    pub rows: Vec<Complete>,
    pub report: Report,
    pub report_final: bool,
    pub execution_trace_complete_verified: bool,
    pub total_operation_count_verified: bool,
    pub dependency_basis_complete_verified: bool,
    pub work_scope: String,
    pub byte_scope: String,
}
record!(CollectionLimits {
    queries: usize,
    pairs: usize,
    residual_points: usize,
    residual_columns: usize,
    x_candidates: u64,
    filter_every: usize,
    max_small_weight: usize,
    max_sources: usize
});
impl CollectionLimits {
    pub fn new(work: u64) -> Self {
        let w = work.min(100_000);
        let pairs = (w / 4).min(8192) as usize;
        let mut n = 0;
        while (n + 1) * (n + 2) / 2 <= pairs {
            n += 1;
        }
        Self {
            queries: w.min(4096) as usize,
            pairs,
            residual_points: n,
            residual_columns: 32,
            x_candidates: 2048,
            filter_every: 32,
            max_small_weight: 4096,
            max_sources: 4096,
        }
    }
}
record!(Signed {
    column: usize,
    coefficient: u64
});
record!(Domain {points:Vec<Value>,columns:Vec<Value>,signed:Vec<Signed>});
record!(Pool {domain:Domain,construction:Vec<Value>,projection_metrics:Value});
record!(DomainsPayload {
    orbit_action: String,
    small: Domain,
    residual: Pool
});
record!(PairsPayload {pairs:Vec<Pair>,identity_sums_excluded:bool,duplicate_sums_keep_first:bool});
record!(Pair {
    left: usize,
    right: usize,
    sum: Value
});
record!(Attempt {sequence:u64,coef_a:u64,coef_b:u64,target:Value,small_indices:Vec<usize>,remainder:Value,residual_pair:Option<(usize,usize)>,row:Option<Partial>});
record!(Rank {columns:usize,rank:usize,determined:Vec<bool>,values:Vec<u64>,rref:Vec<Vec<String>>,rhs:Vec<String>});
record!(Batch {through_attempts:usize,source_count:usize,report:Report,completed:Vec<Complete>,matrix:Rank});
record!(Bindings {
    public_sha256: String,
    factor_base_sha256: String,
    config_sha256: String,
    domains_and_pairs_sha256: String
});
record!(Totals {
    acknowledged_events: u64,
    attempted_events: u64,
    acknowledged_bytes: u64,
    refused_event_bytes: u64,
    acknowledged_work: u64,
    computed_work: u64,
    inverse_calls: u64,
    acknowledged_inverse_calls: u64
});
record!(BatchBinding {
    case: Bindings,
    batch_index: usize,
    through_attempts: usize,
    source_count: usize,
    ordered_sources_sha256: String
});
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum Terminal {
    Completed { legacy_batch: Box<Batch> },
    Censored { stop: Stop },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TracedBatch {
    pub schema: String,
    pub binding: BatchBinding,
    pub native: Run,
    pub terminal: Terminal,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transcript {
    pub schema: String,
    pub public_instance: Value,
    pub config: Value,
    pub binding: Bindings,
    pub orbit_action: String,
    pub m: usize,
    pub requested_work: u64,
    pub limits: CollectionLimits,
    pub small: Domain,
    pub residual: Pool,
    pub pairs: Vec<Pair>,
    pub attempts: Vec<Attempt>,
    pub batches: Vec<TracedBatch>,
    pub trace_policy: String,
    pub trace_limits: Limits,
    pub trace_totals: Totals,
    pub trace_work_scope: String,
    pub trace_byte_scope: String,
    pub terminal: String,
    pub recovered_scalar: Option<u64>,
    pub recovered_column_logs: Option<Vec<u64>>,
    pub recorded_row_transitions_verified: bool,
    pub execution_trace_complete_verified: bool,
    pub total_operation_count_verified: bool,
    pub dependency_basis_complete_verified: bool,
}
pub const WORK_SCOPE:&str="normalization input terms and RHS, peel degree updates, static core degree terms, pivot selection terms and sparse AXPY operand terms plus RHS; excludes arithmetic internals, validation/replay, hashing, map comparisons, snapshots, serialization, initial incidence construction, queue scanning, outer control and group/rank work";
pub const BYTE_SCOPE:&str="compact JSON FilterEvent bytes; excludes newline, live-sink wrapper, artifact duplication and run envelope";
