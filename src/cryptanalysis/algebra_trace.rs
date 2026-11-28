//! Bounded observations of the existing Boolean Macaulay/suffix implementation.
//!
//! This is a native matrix trace, not an independently checked proof or a
//! dataset adapter. See the frozen contract in `research/ml_dataset_f4_trace_20261028`.

use std::io::{self, Write};
use std::time::Instant;

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{F2BoolMono, F2BoolPoly, Mask};

pub const SCHEMA: &str = "crypto.boolean-f4-suffix-trace/v1";
pub const PROFILE: &str = "boolean-macaulay-f4-suffix/v1";
const TERMINAL_BYTES: u64 = 1024;

/// Caller-declared metadata, not attestation of the executing source/binary.
#[derive(Clone, Debug, Serialize)]
pub struct TraceBinding {
    pub source_commit: String,
    pub producer_sha256: String,
    pub configuration_sha256: String,
    pub context_sha256: String,
    pub call_id: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct TraceLimits {
    pub max_candidates: u64,
    pub max_events: u64,
    pub max_stream_bytes: u64,
    pub max_event_bytes: u64,
    pub max_workspace_bytes: u64,
    pub max_memory_bytes: u64,
    pub max_admission_units: u64,
}

impl Default for TraceLimits {
    fn default() -> Self {
        Self {
            max_candidates: 4096,
            max_events: 32768,
            max_stream_bytes: 4 * 1024 * 1024,
            max_event_bytes: 64 * 1024,
            max_workspace_bytes: 2 * 1024 * 1024,
            max_memory_bytes: 32 * 1024 * 1024,
            max_admission_units: 16 * 1024 * 1024,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", content = "detail", rename_all = "snake_case")]
pub enum TraceStop {
    InvalidInput(String),
    UnsupportedProfile(String),
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
    SinkFailure(String),
    Integrity(String),
}

/// Producer observations never certify their own arithmetic or completeness.
#[derive(Clone, Debug, Default, Serialize)]
pub struct TraceClaims {
    pub input_encoding_checked: bool,
    pub matrix_rank_independently_verified: bool,
    pub execution_stream_complete: bool,
    pub total_operation_count_verified: bool,
    pub unsat_verified: bool,
    pub groebner_basis_complete: bool,
    pub first_fall_degree: Option<u32>,
    pub last_fall_degree: Option<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TracePrefix {
    pub acknowledged_events: u64,
    pub acknowledged_bytes: u64,
    pub last_sha256: String,
}

impl Default for TracePrefix {
    fn default() -> Self {
        Self {
            acknowledged_events: 0,
            acknowledged_bytes: 0,
            last_sha256: "0".repeat(64),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TraceFailure {
    pub reason: TraceStop,
    pub prefix: TracePrefix,
    pub claims: TraceClaims,
}

#[derive(Debug)]
pub struct TracedMatrixOutcome {
    pub reduced_rows: Vec<F2BoolPoly>,
    pub rank_native: usize,
    pub word_xors_native: u64,
    pub prefix: TracePrefix,
    pub claims: TraceClaims,
}

/// A successful call acknowledges exactly one complete JSONL record. A caller
/// persists the prefix and owns its storage budget; the producer keeps no log.
pub trait TraceSink {
    fn append(&mut self, record: &[u8]) -> Result<(), String>;
}

impl<F: FnMut(&[u8]) -> Result<(), String>> TraceSink for F {
    fn append(&mut self, record: &[u8]) -> Result<(), String> {
        self(record)
    }
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Event<'a> {
    Begin {
        profile: &'static str,
        binding: &'a TraceBinding,
        n_vars: usize,
        degree: u32,
        input_count: usize,
        input_sha256: String,
        limits: &'a TraceLimits,
        native_row_cap: usize,
        native_column_cap: usize,
        kernel: &'static str,
        ring: &'static str,
        order: &'static str,
    },
    Input {
        index: usize,
        terms: Vec<u8>,
    },
    Candidate {
        index: u64,
        generator: usize,
        multiplier: u8,
        terms: Vec<u8>,
        matrix_row: Option<usize>,
    },
    Columns {
        monomials: Vec<u8>,
    },
    MatrixRow {
        row: usize,
        version: u32,
        words: Vec<String>,
    },
    MatrixReady {
        rows: usize,
        columns: usize,
        candidates: u64,
    },
    Pivot {
        column: usize,
        pivot_row: usize,
        selected_row: Option<usize>,
    },
    Swap {
        left: usize,
        right: usize,
        left_version: u32,
        right_version: u32,
        left_after_version: u32,
        right_after_version: u32,
    },
    RowXor {
        target: usize,
        pivot: usize,
        target_version: u32,
        pivot_version: u32,
        after_version: u32,
        start_word: usize,
        words_after: Vec<String>,
    },
    OutputRow {
        row: usize,
        version: u32,
        words: Vec<String>,
    },
    End {
        rank_native: usize,
        word_xors_native: u64,
        rows: usize,
        columns: usize,
        candidates: u64,
        admission_units: u64,
        claims: TraceClaims,
    },
    Stopped {
        reason: &'a TraceStop,
        acknowledged_before_stop: u64,
    },
}

#[derive(Serialize)]
struct Record<'a> {
    schema: &'static str,
    sequence: u64,
    previous_sha256: &'a str,
    event: &'a Event<'a>,
}

struct BoundedBytes {
    bytes: Vec<u8>,
    limit: usize,
}

impl Write for BoundedBytes {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.limit.saturating_sub(self.bytes.len()) {
            return Err(io::Error::other("algebra trace event byte cap"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(super) struct Session<'a> {
    sink: &'a mut dyn TraceSink,
    cancel: &'a mut dyn FnMut() -> bool,
    deadline: Instant,
    limits: TraceLimits,
    prefix: TracePrefix,
    failed: Option<TraceStop>,
    candidates: u64,
    selected: usize,
    workspace: u64,
    admission_units: u64,
    versions: Vec<u32>,
}

impl Session<'_> {
    pub(super) fn poll(&mut self, units: u64) -> Result<(), TraceStop> {
        if let Some(reason) = &self.failed {
            return Err(reason.clone());
        }
        if Instant::now() >= self.deadline {
            return self.stop(TraceStop::Deadline);
        }
        if (self.cancel)() {
            return self.stop(TraceStop::Cancelled);
        }
        let Some(total) = self.admission_units.checked_add(units) else {
            return self.stop(TraceStop::AdmissionLimit);
        };
        if total > self.limits.max_admission_units {
            return self.stop(TraceStop::AdmissionLimit);
        }
        self.admission_units = total;
        Ok(())
    }

    fn stop<T>(&mut self, reason: TraceStop) -> Result<T, TraceStop> {
        let first = self.failed.get_or_insert(reason);
        Err(first.clone())
    }

    fn reserve_workspace(&mut self, bytes: u64) -> Result<(), TraceStop> {
        let Some(total) = self.workspace.checked_add(bytes) else {
            return self.stop(TraceStop::WorkspaceLimit);
        };
        if total > self.limits.max_workspace_bytes {
            return self.stop(TraceStop::WorkspaceLimit);
        }
        self.workspace = total;
        Ok(())
    }

    fn emit(&mut self, event: &Event<'_>, terminal: bool) -> Result<(), TraceStop> {
        if !terminal {
            self.poll(0)?;
        }
        let reserved_events = u64::from(!terminal);
        if self.prefix.acknowledged_events >= self.limits.max_events - reserved_events {
            return self.stop(TraceStop::EventLimit);
        }
        let mut encoded = BoundedBytes {
            bytes: Vec::with_capacity(self.limits.max_event_bytes as usize),
            limit: self.limits.max_event_bytes as usize - 1,
        };
        let record = Record {
            schema: SCHEMA,
            sequence: self.prefix.acknowledged_events + 1,
            previous_sha256: &self.prefix.last_sha256,
            event,
        };
        if serde_json::to_writer(&mut encoded, &record).is_err() {
            return self.stop(TraceStop::EventByteLimit);
        }
        let digest = format!("{:x}", Sha256::digest(&encoded.bytes));
        encoded.bytes.push(b'\n');
        let bytes = encoded.bytes.len() as u64;
        let reserve = if terminal { 0 } else { TERMINAL_BYTES };
        if bytes
            > self
                .limits
                .max_stream_bytes
                .saturating_sub(self.prefix.acknowledged_bytes)
                .saturating_sub(reserve)
        {
            return self.stop(TraceStop::StreamByteLimit);
        }
        if let Err(reason) = self.sink.append(&encoded.bytes) {
            // Never retry a failed sink or advance over an ambiguous write.
            let failure = TraceStop::SinkFailure(reason.chars().take(64).collect());
            self.failed = Some(failure.clone());
            return Err(failure);
        }
        self.prefix.acknowledged_events += 1;
        self.prefix.acknowledged_bytes += bytes;
        self.prefix.last_sha256 = digest;
        Ok(())
    }

    pub(super) fn before_candidate(&mut self, terms: usize) -> Result<(), TraceStop> {
        if self.candidates >= self.limits.max_candidates {
            return self.stop(TraceStop::CandidateLimit);
        }
        // A conservative, named admission model, not a machine-operation tally.
        self.poll(1 + 2 * terms as u64 + (terms as u64).pow(2))
    }

    pub(super) fn candidate(
        &mut self,
        generator: usize,
        multiplier: Mask,
        terms: &[Mask],
    ) -> Result<(), TraceStop> {
        let matrix_row = if terms.is_empty() {
            None
        } else {
            Some(self.selected)
        };
        if matrix_row.is_some() {
            // Two capacities' worth covers growth of the outer native vectors;
            // reserve each source row, packed row and version before the
            // visitor copies it. Output density is admitted separately.
            let bytes = terms.len() as u64 * std::mem::size_of::<Mask>() as u64
                + 4 * std::mem::size_of::<Vec<u64>>() as u64
                + 64;
            self.reserve_workspace(bytes)?;
        }
        self.emit(
            &Event::Candidate {
                index: self.candidates,
                generator,
                multiplier: multiplier as u8,
                terms: terms.iter().map(|&m| m as u8).collect(),
                matrix_row,
            },
            false,
        )?;
        self.candidates += 1;
        if matrix_row.is_some() {
            self.selected += 1;
        }
        Ok(())
    }

    pub(super) fn matrix_row(&mut self, row: usize, words: &[u64]) -> Result<(), TraceStop> {
        self.emit(
            &Event::MatrixRow {
                row,
                version: 0,
                words: hex_words(words),
            },
            false,
        )
    }

    pub(super) fn pivot(
        &mut self,
        column: usize,
        pivot_row: usize,
        selected_row: Option<usize>,
    ) -> Result<(), TraceStop> {
        self.emit(
            &Event::Pivot {
                column,
                pivot_row,
                selected_row,
            },
            false,
        )
    }

    pub(super) fn swapped(&mut self, left: usize, right: usize) -> Result<(), TraceStop> {
        let lv = self.versions[left];
        let rv = self.versions[right];
        let la = lv + 1;
        let ra = if left == right { la } else { rv + 1 };
        self.emit(
            &Event::Swap {
                left,
                right,
                left_version: lv,
                right_version: rv,
                left_after_version: la,
                right_after_version: ra,
            },
            false,
        )?;
        self.versions[left] = la;
        self.versions[right] = ra;
        Ok(())
    }

    pub(super) fn xored(
        &mut self,
        target: usize,
        pivot: usize,
        start_word: usize,
        words: &[u64],
    ) -> Result<(), TraceStop> {
        let before = self.versions[target];
        self.emit(
            &Event::RowXor {
                target,
                pivot,
                target_version: before,
                pivot_version: self.versions[pivot],
                after_version: before + 1,
                start_word,
                words_after: hex_words(words),
            },
            false,
        )?;
        self.versions[target] = before + 1;
        Ok(())
    }

    fn failure(mut self, reason: TraceStop) -> TraceFailure {
        let mut reason = self.failed.clone().unwrap_or(reason);
        if self.prefix.acknowledged_events > 0 && !matches!(reason, TraceStop::SinkFailure(_)) {
            let before = self.prefix.acknowledged_events;
            if let Err(TraceStop::SinkFailure(detail)) = self.emit(
                &Event::Stopped {
                    reason: &reason,
                    acknowledged_before_stop: before,
                },
                true,
            ) {
                reason = TraceStop::SinkFailure(detail);
            }
        }
        TraceFailure {
            reason,
            prefix: self.prefix,
            claims: TraceClaims::default(),
        }
    }
}

fn hex_words(words: &[u64]) -> Vec<String> {
    words.iter().map(|w| format!("{w:016x}")).collect()
}

fn valid_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn validate(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    binding: &TraceBinding,
    limits: &TraceLimits,
) -> Result<(), TraceStop> {
    if n_vars > 8 || degree > 4 || polys.len() > 64 {
        return Err(TraceStop::UnsupportedProfile(
            "requires q<=8, D<=4 and <=64 inputs".into(),
        ));
    }
    let hard = TraceLimits::default();
    if limits.max_candidates > hard.max_candidates
        || !(2..=hard.max_events).contains(&limits.max_events)
        || !(2048..=hard.max_stream_bytes).contains(&limits.max_stream_bytes)
        || !(1024..=hard.max_event_bytes).contains(&limits.max_event_bytes)
        || limits.max_workspace_bytes > hard.max_workspace_bytes
        || limits.max_memory_bytes > hard.max_memory_bytes
        || limits.max_admission_units > hard.max_admission_units
    {
        return Err(TraceStop::InvalidInput(
            "trace limits exceed frozen profile".into(),
        ));
    }
    let memory = 2 * limits.max_stream_bytes
        + limits.max_workspace_bytes
        + 2 * limits.max_event_bytes
        + 1024 * 1024;
    if memory > limits.max_memory_bytes {
        return Err(TraceStop::MemoryLimit);
    }
    if !valid_hex(&binding.source_commit, 40)
        || [
            &binding.producer_sha256,
            &binding.configuration_sha256,
            &binding.context_sha256,
            &binding.call_id,
        ]
        .iter()
        .any(|s| !valid_hex(s, 64))
    {
        return Err(TraceStop::InvalidInput(
            "noncanonical source/call binding".into(),
        ));
    }
    let mask = super::all_variable_mask(n_vars);
    for p in polys {
        if p.n_vars != n_vars
            || p.terms.len() > 256
            || p.terms
                .iter()
                .any(|m| m.mask & !mask != 0 || m.mask.count_ones() > degree)
            || p.terms
                .windows(2)
                .any(|w| super::cmp_mono(w[0], w[1]) != std::cmp::Ordering::Greater)
        {
            return Err(TraceStop::InvalidInput(
                "noncanonical polynomial or input degree exceeds D".into(),
            ));
        }
    }
    // Direct full-readback calls do not select the solver's flat/inherited
    // paths. Reject explicit requests for these unrelated capture profiles;
    // never silently replace an environment-selected algorithm.
    for key in [
        "KIC_F4_ACTIVE_MULTIPLIERS",
        "KIC_F4_SOLVER_LINEAR_TAIL",
        "KIC_F4_FLAT_MATRIX",
        "KIC_F4_STD_COLUMN_HASH",
        "KIC_F4_HASH_COLUMN_INDEX",
    ] {
        if std::env::var(key).as_deref() == Ok("1") {
            return Err(TraceStop::UnsupportedProfile(format!(
                "unsupported selected option {key}"
            )));
        }
    }
    if std::env::var("KIC_F4_CRITERION").as_deref() == Ok("f5") {
        return Err(TraceStop::UnsupportedProfile(
            "F5 criterion requires a separate witness profile".into(),
        ));
    }
    Ok(())
}

/// Observe one existing Boolean Macaulay/suffix call. All incomplete outcomes
/// return the last acknowledged prefix; no independent-rank claim is produced.
#[allow(clippy::too_many_arguments)]
pub fn matrix_f4_f2_traced(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    binding: &TraceBinding,
    limits: TraceLimits,
    deadline: Instant,
    cancel: &mut dyn FnMut() -> bool,
    sink: &mut dyn TraceSink,
) -> Result<TracedMatrixOutcome, TraceFailure> {
    if let Err(reason) = validate(polys, n_vars, degree, binding, &limits) {
        return Err(TraceFailure {
            reason,
            prefix: TracePrefix::default(),
            claims: TraceClaims::default(),
        });
    }
    let mut session = Session {
        sink,
        cancel,
        deadline,
        limits: limits.clone(),
        prefix: TracePrefix::default(),
        failed: None,
        candidates: 0,
        selected: 0,
        workspace: 0,
        admission_units: 0,
        versions: Vec::new(),
    };
    let mut word_ops = 0;
    let result = (|| {
        session.poll(0)?;
        let native_row_cap = super::max_f4_rows();
        let native_column_cap = super::max_f4_cols();
        // Fixed reserve covers original input encoding, native multiplier
        // schedules, <=256-entry column indexes, sort scratch and metadata.
        session.reserve_workspace(128 * 1024)?;
        let inputs: Vec<Vec<u8>> = polys
            .iter()
            .map(|p| p.terms.iter().map(|m| m.mask as u8).collect())
            .collect();
        let input_bytes = serde_json::to_vec(&(n_vars, degree, &inputs))
            .map_err(|_| TraceStop::Integrity("input serialization failed".into()))?;
        session.emit(
            &Event::Begin {
                profile: PROFILE,
                binding,
                n_vars,
                degree,
                input_count: polys.len(),
                input_sha256: format!("{:x}", Sha256::digest(&input_bytes)),
                limits: &limits,
                native_row_cap,
                native_column_cap,
                kernel: "native-suffix-rref",
                ring: "F2[v]/(vi^2+vi)",
                order: "degrevlex:v0>v1>...",
            },
            false,
        )?;
        for (index, terms) in inputs.into_iter().enumerate() {
            session.emit(&Event::Input { index, terms }, false)?;
        }
        let mut rows = Vec::new();
        let count = super::visit_macaulay_rows_observed::<false>(
            polys,
            n_vars,
            degree,
            super::all_variable_mask(n_vars),
            None,
            native_row_cap,
            Some(&mut session),
            |row| rows.push(row.to_vec()),
        )?
        .ok_or(TraceStop::NativeSizeLimit)?;
        let columns =
            super::macaulay_columns_observed(&rows, native_column_cap, Some(&mut session))?
                .ok_or(TraceStop::NativeSizeLimit)?;
        if columns.len() > 163 || count.0 > 4096 {
            return Err(TraceStop::Integrity(
                "admitted matrix exceeded dimension bound".into(),
            ));
        }
        session.emit(
            &Event::Columns {
                monomials: columns.iter().map(|&m| m as u8).collect(),
            },
            false,
        )?;
        let layout = super::F4ColumnLayout::new(columns);
        let mut matrix =
            super::pack_rows_with_layout_observed(&rows, &layout, false, Some(&mut session))?
                .ok_or(TraceStop::Integrity(
                    "native column layout rejected its own support".into(),
                ))?;
        session.versions = vec![0; matrix.len()];
        session.emit(
            &Event::MatrixReady {
                rows: matrix.len(),
                columns: layout.columns.len(),
                candidates: session.candidates,
            },
            false,
        )?;
        // The unchanged dispatcher chooses suffix automatically at <256 cols.
        if super::rref_uses_four_russians(matrix.len(), layout.columns.len()) {
            return Err(TraceStop::UnsupportedProfile(
                "native dispatcher selected another kernel".into(),
            ));
        }
        let rank = super::echelon_f2_suffix_observed(
            &mut matrix,
            layout.columns.len(),
            &mut word_ops,
            true,
            Some(&mut session),
        )?;
        session.reserve_workspace((rank * std::mem::size_of::<F2BoolPoly>()) as u64)?;
        let mut reduced = Vec::with_capacity(rank);
        for (row, words) in matrix.iter().enumerate() {
            session.poll(words.len() as u64)?;
            session.emit(
                &Event::OutputRow {
                    row,
                    version: session.versions[row],
                    words: hex_words(words),
                },
                false,
            )?;
            if row < rank {
                let terms: u64 = words.iter().map(|word| u64::from(word.count_ones())).sum();
                session.reserve_workspace(terms * std::mem::size_of::<F2BoolMono>() as u64)?;
                reduced.push(super::poly_from_column_order(
                    super::row_monos(words, &layout.columns),
                    n_vars,
                ));
            }
        }
        session.poll(0)?;
        session.emit(
            &Event::End {
                rank_native: rank,
                word_xors_native: word_ops,
                rows: matrix.len(),
                columns: layout.columns.len(),
                candidates: session.candidates,
                admission_units: session.admission_units,
                claims: TraceClaims::default(),
            },
            true,
        )?;
        Ok((rank, reduced))
    })();
    super::charge_word_ops(word_ops);
    match result {
        Ok((rank_native, reduced_rows)) => Ok(TracedMatrixOutcome {
            reduced_rows,
            rank_native,
            word_xors_native: word_ops,
            prefix: session.prefix,
            claims: TraceClaims::default(),
        }),
        Err(reason) => Err(session.failure(reason)),
    }
}

#[cfg(test)]
#[path = "algebra_trace/tests.rs"]
mod tests;
