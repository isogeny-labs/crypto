//! Bounded native filter transitions and independent source-combination replay.
//!
//! Cavallar, CWI MAS-R0012 (2000), sections 2.1–2.2 describes singleton peeling
//! and merging; the arithmetic here is the repository's odd-modulus ECDLP
//! filter. This is neither an execution-complete trace nor a total-cost proof.
use super::{
    CompleteRelation, LargePrimeFilterError, LargePrimeFilterOptions, LargePrimeFilterReport,
    PartialRelation, WorkRow,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

mod check;
pub use check::verify_filter_trace;
#[cfg(test)]
mod tests;

pub const SCHEMA: &str = "large-prime-filter-trace/v1";
pub const WORK_SCOPE: &str = "normalization input terms and RHS, peel degree updates, static core degree terms, pivot selection terms and sparse AXPY operand terms plus RHS; excludes arithmetic internals, validation/replay, hashing, map comparisons, snapshots, serialization, initial incidence construction, queue scanning, outer control and group/rank work";
pub const BYTE_SCOPE: &str = "compact JSON FilterEvent bytes; excludes newline, live-sink wrapper, artifact duplication and run envelope";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilterTraceLimits {
    pub input_rows: usize,
    pub input_terms: usize,
    pub row_terms: usize,
    pub events: u64,
    pub bytes: u64,
    pub work: u64,
}
impl Default for FilterTraceLimits {
    fn default() -> Self {
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
impl FilterTraceLimits {
    fn valid(self) -> bool {
        let h = Self::default();
        self.input_rows <= h.input_rows
            && self.input_terms <= h.input_terms
            && self.row_terms <= h.row_terms
            && self.events <= h.events
            && self.bytes <= h.bytes
            && self.work <= h.work
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FilterTraceError {
    Native(LargePrimeFilterError),
    InvalidLimits,
}
impl From<LargePrimeFilterError> for FilterTraceError {
    fn from(value: LargePrimeFilterError) -> Self {
        Self::Native(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterStop {
    Completed,
    InputLimit,
    StateLimit,
    WorkLimit,
    EventLimit,
    ByteLimit,
    SinkFailure,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RowRef {
    pub input_id: u64,
    pub version: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RowSnapshot {
    pub small: Vec<(u32, u64)>,
    pub large: Vec<(u64, u64)>,
    pub rhs: u64,
    pub sources: Vec<(u64, u64)>,
}
impl RowSnapshot {
    fn terms(&self) -> usize {
        self.small.len() + self.large.len() + self.sources.len() + 1
    }
    fn complete(&self) -> CompleteRelation {
        CompleteRelation {
            small: self.small.clone(),
            rhs: self.rhs,
            sources: self.sources.clone(),
        }
    }
}
impl From<&WorkRow> for RowSnapshot {
    fn from(row: &WorkRow) -> Self {
        Self {
            small: row.small.clone(),
            large: row.large.clone(),
            rhs: row.rhs,
            sources: row.sources.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
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

/// A conservative pre-computation reservation. State size is an operand bound,
/// not a claim about the final nonzero candidate size. Choice binds a peel column.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reservation {
    pub phase: Phase,
    pub choice: Option<u64>,
    pub work: u64,
    pub state_terms: usize,
}
impl Reservation {
    fn new(phase: Phase, work: usize, state_terms: usize) -> Self {
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
        options: LargePrimeFilterOptions,
        limits: FilterTraceLimits,
    },
    Normalized {
        position: usize,
        row: PartialRelation,
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
        row: RowSnapshot,
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
        candidate: RowSnapshot,
        accepted: bool,
    },
    PivotStored {
        reference: RowRef,
        column: u64,
        row: RowSnapshot,
    },
    CompleteRow {
        reference: RowRef,
        row: RowSnapshot,
    },
    Finish {
        rows: Vec<CompleteRelation>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilterEvent {
    pub sequence: u64,
    pub reservation: Reservation,
    pub transition: Transition,
    pub report_after: LargePrimeFilterReport,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilterRun {
    pub schema: String,
    /// SHA256 of compact serde JSON of the ordered typed input before
    /// coefficient normalization; not the original file's whitespace/bytes.
    /// Absent only when raw input admission fails before hashing.
    pub input_sha256: Option<String>,
    pub input_rows: usize,
    pub input_terms: usize,
    pub options: LargePrimeFilterOptions,
    pub limits: FilterTraceLimits,
    pub stop: FilterStop,
    pub events: Vec<FilterEvent>,
    pub unacknowledged: Option<FilterEvent>,
    pub next_reservation: Option<Reservation>,
    pub sink_error: Option<String>,
    pub computed_work: u64,
    pub acknowledged_work: u64,
    pub inverse_calls: u64,
    pub acknowledged_inverse_calls: u64,
    pub acknowledged_bytes: u64,
    pub unacknowledged_bytes: u64,
    pub rows: Vec<CompleteRelation>,
    pub report: LargePrimeFilterReport,
    pub report_final: bool,
    pub execution_trace_complete_verified: bool,
    pub total_operation_count_verified: bool,
    pub dependency_basis_complete_verified: bool,
    pub work_scope: String,
    pub byte_scope: String,
}

fn raw_terms(inputs: &[PartialRelation]) -> Option<usize> {
    inputs.iter().try_fold(0usize, |n, r| {
        n.checked_add(r.small.len())?.checked_add(r.large.len())
    })
}
fn input_hash(inputs: &[PartialRelation]) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(inputs).expect("integer relation JSON"))
    )
}
fn validate_modulus(modulus: u64) -> Result<(), FilterTraceError> {
    if !(3..1u64 << 63).contains(&modulus) || modulus.is_multiple_of(2) {
        return Err(LargePrimeFilterError::InvalidModulus.into());
    }
    Ok(())
}
fn row_terms(row: &WorkRow) -> usize {
    row.small.len() + row.large.len() + row.sources.len() + 1
}

struct Capture<'a> {
    run: FilterRun,
    sink: &'a mut dyn FnMut(&FilterEvent) -> Result<(), String>,
}
impl Capture<'_> {
    fn admitted(&mut self, plan: Reservation) -> bool {
        let stop = if plan.state_terms > self.run.limits.row_terms {
            Some(FilterStop::StateLimit)
        } else if self.run.events.len() as u64 >= self.run.limits.events {
            Some(FilterStop::EventLimit)
        } else if plan.work > self.run.limits.work - self.run.computed_work {
            Some(FilterStop::WorkLimit)
        } else {
            None
        };
        if let Some(stop) = stop {
            self.run.stop = stop;
            self.run.next_reservation = Some(plan);
            false
        } else {
            true
        }
    }
    // Call only after admission and computation; acknowledgment precedes all
    // committed algorithm/report changes. The sink must durably save before Ok.
    fn emit(
        &mut self,
        plan: Reservation,
        transition: Transition,
        report: LargePrimeFilterReport,
    ) -> bool {
        let event = FilterEvent {
            sequence: self.run.events.len() as u64,
            reservation: plan,
            transition,
            report_after: report,
        };
        self.run.computed_work += plan.work;
        let inversions = u64::from(matches!(event.transition, Transition::Reduction { .. }));
        self.run.inverse_calls += inversions;
        let bytes = serde_json::to_vec(&event)
            .expect("integer trace JSON")
            .len() as u64;
        let error = if bytes > self.run.limits.bytes - self.run.acknowledged_bytes {
            self.run.stop = FilterStop::ByteLimit;
            None
        } else if let Err(error) = (self.sink)(&event) {
            self.run.stop = FilterStop::SinkFailure;
            Some(error.chars().take(1024).collect())
        } else {
            self.run.acknowledged_work += plan.work;
            self.run.acknowledged_inverse_calls += inversions;
            self.run.acknowledged_bytes += bytes;
            self.run.report = event.report_after.clone();
            if let Transition::CompleteRow { row, .. } = &event.transition {
                self.run.rows.push(row.complete());
            }
            self.run.events.push(event);
            return true;
        };
        self.run.sink_error = error;
        self.run.unacknowledged_bytes = bytes;
        self.run.unacknowledged = Some(event);
        false
    }
}

/// Capture a bounded batch. Caps are censored prefixes, never UNSAT/recovery.
/// Mathematical errors return Err; already acknowledged bytes remain in the sink.
pub fn filter_large_primes_with_trace(
    inputs: Vec<PartialRelation>,
    options: &LargePrimeFilterOptions,
    limits: FilterTraceLimits,
    sink: &mut dyn FnMut(&FilterEvent) -> Result<(), String>,
) -> Result<FilterRun, FilterTraceError> {
    validate_modulus(options.modulus)?;
    if !limits.valid() {
        return Err(FilterTraceError::InvalidLimits);
    }
    let total_terms = raw_terms(&inputs).unwrap_or(usize::MAX);
    let mut capture = Capture {
        run: FilterRun {
            schema: SCHEMA.into(),
            input_sha256: None,
            input_rows: inputs.len(),
            input_terms: total_terms,
            options: *options,
            limits,
            stop: FilterStop::InputLimit,
            events: vec![],
            unacknowledged: None,
            next_reservation: None,
            sink_error: None,
            computed_work: 0,
            acknowledged_work: 0,
            inverse_calls: 0,
            acknowledged_inverse_calls: 0,
            acknowledged_bytes: 0,
            unacknowledged_bytes: 0,
            rows: vec![],
            report: LargePrimeFilterReport {
                input_relations: inputs.len(),
                ..Default::default()
            },
            report_final: false,
            execution_trace_complete_verified: false,
            total_operation_count_verified: false,
            dependency_basis_complete_verified: false,
            work_scope: WORK_SCOPE.into(),
            byte_scope: BYTE_SCOPE.into(),
        },
        sink,
    };
    if inputs.len() > limits.input_rows || total_terms > limits.input_terms {
        return Ok(capture.run);
    }
    let mut ids = BTreeSet::new();
    for row in &inputs {
        if !ids.insert(row.id) {
            return Err(LargePrimeFilterError::DuplicateId(row.id).into());
        }
    }
    capture.run.input_sha256 = Some(input_hash(&inputs));
    execute(inputs, &mut capture)?;
    Ok(capture.run)
}

fn execute(inputs: Vec<PartialRelation>, c: &mut Capture<'_>) -> Result<(), FilterTraceError> {
    let modulus = c.run.options.modulus;
    let options = c.run.options;
    let mut report = c.run.report.clone();
    macro_rules! admit {
        ($p:expr) => {
            if !c.admitted($p) {
                return Ok(());
            }
        };
    }
    macro_rules! emit {
        ($p:expr, $t:expr) => {
            if !c.emit($p, $t, report.clone()) {
                return Ok(());
            }
        };
    }
    let p = Reservation::new(Phase::Begin, 0, 0);
    admit!(p);
    emit!(
        p,
        Transition::Begin {
            input_sha256: c.run.input_sha256.clone().unwrap(),
            input_rows: c.run.input_rows,
            input_terms: c.run.input_terms,
            options,
            limits: c.run.limits
        }
    );
    let mut rows = Vec::new();
    let mut seen = BTreeMap::new();
    for (position, mut row) in inputs.into_iter().enumerate() {
        let p = Reservation::new(
            Phase::Normalize,
            row.small.len() + row.large.len() + 1,
            row.small.len() + row.large.len() + 2,
        );
        admit!(p);
        row.normalise(modulus);
        emit!(
            p,
            Transition::Normalized {
                position,
                row: row.clone()
            }
        );
        let key = super::duplicate_key(&row);
        if let Some(&representative) = seen.get(&key) {
            let p = Reservation::new(Phase::Duplicate, 0, 0);
            admit!(p);
            report.duplicate_relations += 1;
            emit!(
                p,
                Transition::DuplicateDropped {
                    input_id: row.id,
                    representative
                }
            );
        } else {
            seen.insert(key, row.id);
            rows.push(row);
        }
    }
    let mut degree = BTreeMap::<u64, usize>::new();
    for row in &rows {
        for &(col, _) in &row.large {
            *degree.entry(col).or_default() += 1;
        }
    }
    report.input_large_vertices = degree.len();
    let mut active = vec![true; rows.len()];
    // Deterministic legal singleton order is permitted: the legacy HashMap
    // queue order changes only the peel transcript, never the surviving core.
    while let Some((&column, _)) = degree.iter().find(|(_, d)| **d == 1) {
        let index = rows
            .iter()
            .enumerate()
            .position(|(i, row)| {
                active[i] && row.large.binary_search_by_key(&column, |t| t.0).is_ok()
            })
            .expect("singleton incident row");
        let row = &rows[index];
        let mut p = Reservation::new(
            Phase::Peel,
            row.large.len(),
            row.small.len() + row.large.len() + 2,
        );
        p.choice = Some(column);
        admit!(p);
        let before: Vec<_> = row.large.iter().map(|t| (t.0, degree[&t.0])).collect();
        let after: Vec<_> = before.iter().map(|&(col, d)| (col, d - 1)).collect();
        report.peeled_relations += 1;
        emit!(
            p,
            Transition::Peel {
                column,
                input_id: row.id,
                degrees_before: before,
                degrees_after: after.clone()
            }
        );
        active[index] = false;
        for (col, d) in after {
            degree.insert(col, d);
        }
    }
    let smooth: Vec<_> = rows
        .iter()
        .filter(|r| r.large.is_empty())
        .cloned()
        .collect();
    let mut core: Vec<_> = rows
        .into_iter()
        .zip(active)
        .filter_map(|(r, a)| (a && !r.large.is_empty()).then_some(r))
        .collect();
    core.sort_by_key(|r| (r.large.len(), r.small.len(), r.id));
    let mut core_degree = BTreeMap::<u64, usize>::new();
    let work = core.iter().map(|r| r.large.len()).sum();
    let p = Reservation::new(Phase::Core, work, 0);
    admit!(p);
    for row in &core {
        for &(col, _) in &row.large {
            *core_degree.entry(col).or_default() += 1;
        }
    }
    report.core_relations = core.len();
    report.core_large_vertices = core_degree.len();
    emit!(
        p,
        Transition::CoreReady {
            row_ids: core.iter().map(|r| r.id).collect(),
            smooth_ids: smooth.iter().map(|r| r.id).collect(),
            degrees: core_degree.iter().map(|(&k, &v)| (k, v)).collect()
        }
    );
    for row in smooth {
        let reference = RowRef {
            input_id: row.id,
            version: 0,
        };
        let p = Reservation::new(Phase::Complete, 0, row.small.len() + 2);
        admit!(p);
        let row = WorkRow::from_partial(row, modulus);
        report.complete_relations += 1;
        emit!(
            p,
            Transition::CompleteRow {
                reference,
                row: (&row).into()
            }
        );
    }
    let mut pivots = BTreeMap::<u64, (RowRef, WorkRow)>::new();
    for partial in core {
        let mut reference = RowRef {
            input_id: partial.id,
            version: 0,
        };
        let p = Reservation::new(Phase::Row, 0, partial.small.len() + partial.large.len() + 2);
        admit!(p);
        let mut row = WorkRow::from_partial(partial, modulus);
        emit!(
            p,
            Transition::RowBegin {
                reference,
                row: (&row).into()
            }
        );
        loop {
            if row.large.is_empty() {
                let p = Reservation::new(Phase::Complete, 0, row_terms(&row));
                admit!(p);
                report.complete_relations += 1;
                report.cycles_closed += usize::from(row.sources.len() > 1);
                emit!(
                    p,
                    Transition::CompleteRow {
                        reference,
                        row: (&row).into()
                    }
                );
                break;
            }
            let p = Reservation::new(Phase::Select, row.large.len(), row_terms(&row));
            admit!(p);
            let score = |col: u64| {
                core_degree
                    .get(&col)
                    .copied()
                    .unwrap_or(1)
                    .saturating_sub(1)
                    * row.large.len().saturating_sub(1)
            };
            let mut existing = None;
            let mut any = None;
            for &(col, coeff) in &row.large {
                let candidate = (col, coeff, score(col));
                if any.is_none_or(|(_, _, s)| candidate.2 < s) {
                    any = Some(candidate);
                }
                if pivots.contains_key(&col) && existing.is_none_or(|(_, _, s)| candidate.2 < s) {
                    existing = Some(candidate);
                }
            }
            let (column, coefficient, _) = existing.or(any).unwrap();
            let pivot_ref = pivots.get(&column).map(|p| p.0);
            emit!(
                p,
                Transition::PivotSelected {
                    reference,
                    column,
                    pivot: pivot_ref,
                    score: score(column)
                }
            );
            if let Some((pivot_reference, pivot)) = pivots.get(&column) {
                let p = Reservation::new(
                    Phase::Reduce,
                    row_terms(&row) + row_terms(pivot) - 1,
                    row_terms(&row) + row_terms(pivot) - 1,
                );
                admit!(p);
                let pc = pivot.coefficient(column);
                let inverse = super::inverse_mod(pc, modulus).ok_or(
                    LargePrimeFilterError::NonInvertiblePivot {
                        large_prime: column,
                        coefficient: pc,
                    },
                )?;
                let scale = super::neg_mod(super::mul_mod(coefficient, inverse, modulus), modulus);
                let candidate = row.axpy(scale, pivot, modulus);
                let accepted = candidate.small.len() <= options.max_small_weight
                    && candidate.sources.len() <= options.max_sources;
                if accepted {
                    report.merges += 1;
                } else {
                    report.rejected_fill += 1;
                }
                emit!(
                    p,
                    Transition::Reduction {
                        reference,
                        column,
                        pivot: *pivot_reference,
                        inverse,
                        scale,
                        candidate: (&candidate).into(),
                        accepted
                    }
                );
                if !accepted {
                    break;
                }
                row = candidate;
                reference.version += 1;
            } else {
                let p = Reservation::new(Phase::Store, 0, row_terms(&row));
                admit!(p);
                report.pivots += 1;
                emit!(
                    p,
                    Transition::PivotStored {
                        reference,
                        column,
                        row: (&row).into()
                    }
                );
                pivots.insert(column, (reference, row));
                break;
            }
        }
    }
    let p = Reservation::new(Phase::Finish, 0, 0);
    admit!(p);
    emit!(
        p,
        Transition::Finish {
            rows: c.run.rows.clone()
        }
    );
    c.run.stop = FilterStop::Completed;
    c.run.report_final = true;
    Ok(())
}
