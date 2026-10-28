//! Independent bounded Boolean-matrix arithmetic replay. No producer dependency.
pub mod schema;

use schema::{Binding, Claims, Event, Limits, Record, StopReason, PROFILE, SCHEMA};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::BufRead;

pub type Result<T> = std::result::Result<T, String>;
const STREAM_CAP: usize = 4 * 1024 * 1024;
const RECORD_CAP: usize = 64 * 1024;
const EVENT_CAP: u64 = 32768;
const WORK_CAP: u64 = 128_000_000;

#[derive(Debug, Serialize)]
pub struct Report {
    pub schema: &'static str,
    pub acknowledged_records: u64,
    pub acknowledged_bytes: usize,
    pub trailing_unacknowledged_bytes: usize,
    pub last_record_sha256: String,
    pub supplied_bindings_matched: bool,
    pub matrix_trace_complete: bool,
    pub independent_rank: Option<usize>,
    pub native_word_xors_checked: Option<u64>,
    pub named_admission_model_checked: Option<u64>,
    pub checker_logical_operations: u64,
    pub stop_reason: Option<StopReason>,
    pub stop_reason_independently_verified: bool,
    pub input_encoding_checked: bool,
    pub producer_execution_attested: bool,
    pub sink_durability_verified: bool,
    pub total_operation_count_verified: bool,
    pub unsat_verified: bool,
    pub groebner_basis_complete: bool,
    pub first_fall_degree: Option<u64>,
    pub last_fall_degree: Option<u64>,
}

pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn require(condition: bool, detail: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(detail.to_owned())
    }
}

fn hex(s: &str, width: usize) -> bool {
    s.len() == width
        && s.bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

pub fn validate_binding(b: &Binding) -> Result<()> {
    require(
        hex(&b.source_commit, 40),
        "source_commit must be 40 lowercase hex digits",
    )?;
    for s in [
        &b.producer_sha256,
        &b.configuration_sha256,
        &b.context_sha256,
        &b.call_id,
    ] {
        require(hex(s, 64), "binding hash must be 64 lowercase hex digits")?;
    }
    Ok(())
}

fn validate_limits(l: &Limits) -> Result<()> {
    require(
        l.max_candidates <= 4096
            && l.max_events <= EVENT_CAP
            && l.max_stream_bytes <= STREAM_CAP as u64
            && l.max_event_bytes <= RECORD_CAP as u64
            && l.max_workspace_bytes <= 2 * 1024 * 1024
            && l.max_memory_bytes <= 32 * 1024 * 1024
            && l.max_admission_units <= 16 * 1024 * 1024,
        "producer limits exceed frozen profile",
    )?;
    require(
        l.max_events >= 2 && l.max_stream_bytes >= 1024 && l.max_event_bytes > 0,
        "producer limits cannot hold reserved terminal",
    )?;
    // Named component admission only, not a proof of actual allocation or RSS.
    let components =
        2 * l.max_stream_bytes + l.max_workspace_bytes + 2 * l.max_event_bytes + 1024 * 1024;
    require(
        components <= l.max_memory_bytes,
        "producer component admission exceeds memory limit",
    )
}

fn order(terms: &mut [u8]) {
    terms.sort_unstable_by_key(|m| (std::cmp::Reverse(m.count_ones()), *m));
}

fn multipliers(q: u8, degree: u8) -> Vec<u8> {
    fn choose(q: u8, left: u8, start: u8, mask: u8, out: &mut Vec<u8>) {
        if left == 0 {
            out.push(mask);
            return;
        }
        for v in start..q {
            if q - v >= left {
                choose(q, left - 1, v + 1, mask | (1 << v), out);
            }
        }
    }
    let mut out = Vec::new();
    for d in 0..=degree.min(q) {
        choose(q, d, 0, 0, &mut out);
    }
    out
}

fn words(row: &[u64]) -> Vec<String> {
    row.iter().map(|w| format!("{w:016x}")).collect()
}

#[derive(Clone, Copy, Debug)]
enum Stage {
    Candidates,
    Matrix(usize),
    Ready,
    Pivot,
    Swap(usize),
    Clear(usize),
    Output(usize),
    End,
    Done,
}

struct Replay {
    q: u8,
    degree: u8,
    input_count: usize,
    input_digest: String,
    limits: Limits,
    row_cap: u64,
    column_cap: u64,
    inputs: Vec<Vec<u8>>,
    gen: usize,
    schedule: Vec<u8>,
    schedule_index: usize,
    candidates: u64,
    products: Vec<Vec<u8>>,
    columns: Vec<u8>,
    rows: Vec<Vec<u64>>,
    original: Vec<Vec<u64>>,
    versions: Vec<u64>,
    stage: Stage,
    rank: usize,
    column: usize,
    xors: u64,
    admission: u64,
    work: u64,
    oracle_rank: Option<usize>,
}

impl Replay {
    fn begin(e: Event, expected: &Binding) -> Result<Self> {
        let Event::Begin {
            profile,
            binding,
            n_vars,
            degree,
            input_count,
            input_sha256,
            limits,
            native_row_cap,
            native_column_cap,
            kernel,
            ring,
            order,
        } = e
        else {
            return Err("first event must be begin".into());
        };
        require(
            profile == PROFILE
                && kernel == "native-suffix-rref"
                && ring == "F2[v]/(vi^2+vi)"
                && order == "degrevlex:v0>v1>...",
            "wrong profile/ring/order/kernel",
        )?;
        validate_binding(&binding)?;
        require(&binding == expected, "trusted caller binding mismatch")?;
        require(
            n_vars <= 8 && degree <= 4 && input_count <= 64,
            "unsupported input dimensions",
        )?;
        require(hex(&input_sha256, 64), "noncanonical input hash")?;
        validate_limits(&limits)?;
        Ok(Self {
            q: n_vars,
            degree,
            input_count: input_count as usize,
            input_digest: input_sha256,
            limits,
            row_cap: native_row_cap,
            column_cap: native_column_cap,
            inputs: Vec::new(),
            gen: 0,
            schedule: Vec::new(),
            schedule_index: 0,
            candidates: 0,
            products: Vec::new(),
            columns: Vec::new(),
            rows: Vec::new(),
            original: Vec::new(),
            versions: Vec::new(),
            stage: Stage::Candidates,
            rank: 0,
            column: 0,
            xors: 0,
            admission: 0,
            work: 0,
            oracle_rank: None,
        })
    }

    fn work(&mut self, n: usize) -> Result<()> {
        self.work = self
            .work
            .checked_add(n as u64)
            .ok_or("checker work overflow")?;
        require(self.work <= WORK_CAP, "checker logical-work limit exceeded")
    }

    fn charge(&mut self, n: u64) -> Result<()> {
        self.admission = self.admission.checked_add(n).ok_or("admission overflow")?;
        require(
            self.admission <= self.limits.max_admission_units,
            "trace exceeds declared admission limit",
        )
    }

    fn input(&mut self, e: &Event) -> Result<()> {
        let Event::Input { index, terms } = e else {
            return Err("missing input event".into());
        };
        require(
            *index == self.inputs.len() as u64 && terms.len() <= 256,
            "input index/size mismatch",
        )?;
        let allowed = (1u16 << self.q) - 1;
        require(
            terms
                .iter()
                .all(|m| u16::from(*m) <= allowed && m.count_ones() <= u32::from(self.degree)),
            "input term outside variables or degree",
        )?;
        let mut sorted = terms.clone();
        order(&mut sorted);
        require(
            sorted == *terms && terms.windows(2).all(|w| w[0] != w[1]),
            "noncanonical input terms",
        )?;
        self.work(terms.len())?;
        self.inputs.push(terms.clone());
        Ok(())
    }

    fn verify_input_digest(&mut self) -> Result<()> {
        let bytes =
            serde_json::to_vec(&(self.q, self.degree, &self.inputs)).map_err(|e| e.to_string())?;
        require(digest(&bytes) == self.input_digest, "input digest mismatch")?;
        self.work(bytes.len())
    }

    fn make_matrix(&mut self) -> Result<()> {
        let mut support = [false; 256];
        let mut terms_total = 0;
        for p in &self.products {
            for m in p {
                support[*m as usize] = true;
                terms_total += 1;
            }
        }
        self.work(terms_total)?;
        self.columns = (0..=255).filter(|m| support[*m]).map(|m| m as u8).collect();
        order(&mut self.columns);
        require(
            self.columns.len() <= 163 && self.columns.len() as u64 <= self.column_cap,
            "matrix exceeds native column cap",
        )?;
        let width = self.columns.len().div_ceil(64);
        let mut indices = [0usize; 256];
        for (i, m) in self.columns.iter().enumerate() {
            indices[*m as usize] = i;
        }
        for p in &self.products {
            let mut row = vec![0; width];
            for m in p {
                let c = indices[*m as usize];
                row[c / 64] |= 1u64 << (c % 64);
            }
            self.rows.push(row);
        }
        self.work(terms_total + self.products.len() * width)?;
        if self.products.len() > 1 {
            self.charge(terms_total as u64)?;
        }
        self.original = self.rows.clone();
        self.versions = vec![0; self.rows.len()];
        Ok(())
    }

    fn bit(&self, row: usize, col: usize) -> bool {
        self.rows[row][col / 64] & (1u64 << (col % 64)) != 0
    }

    fn next(&mut self) -> Result<Event> {
        loop {
            match self.stage {
                Stage::Candidates => {
                    if self.gen == self.inputs.len() {
                        self.make_matrix()?;
                        self.stage = Stage::Matrix(0);
                        return Ok(Event::Columns {
                            monomials: self.columns.clone(),
                        });
                    }
                    if self.schedule.is_empty() {
                        let terms = &self.inputs[self.gen];
                        let d = terms
                            .iter()
                            .map(|m| m.count_ones() as u8)
                            .max()
                            .unwrap_or(0);
                        self.schedule = multipliers(self.q, self.degree - d);
                        self.charge(terms.len() as u64)?;
                    }
                    if self.schedule_index == self.schedule.len() {
                        self.gen += 1;
                        self.schedule_index = 0;
                        self.schedule.clear();
                        continue;
                    }
                    require(
                        self.candidates < self.limits.max_candidates,
                        "candidate count exceeds declared limit",
                    )?;
                    let multiplier = self.schedule[self.schedule_index];
                    let n = self.inputs[self.gen].len();
                    self.charge(1 + 2 * n as u64 + (n * n) as u64)?;
                    let mut parity = [false; 256];
                    for t in &self.inputs[self.gen] {
                        parity[(t | multiplier) as usize] ^= true;
                    }
                    self.work(n + 256)?;
                    let terms: Vec<u8> =
                        (0..=255).filter(|m| parity[*m]).map(|m| m as u8).collect();
                    let matrix_row = if terms.is_empty() {
                        None
                    } else {
                        require(
                            self.products.len() < 4096
                                && (self.products.len() as u64) < self.row_cap,
                            "matrix exceeds native row cap",
                        )?;
                        self.products.push(terms.clone());
                        Some((self.products.len() - 1) as u64)
                    };
                    let event = Event::Candidate {
                        index: self.candidates,
                        generator: self.gen as u64,
                        multiplier,
                        terms,
                        matrix_row,
                    };
                    self.candidates += 1;
                    self.schedule_index += 1;
                    return Ok(event);
                }
                Stage::Matrix(i) => {
                    if i == self.rows.len() {
                        self.stage = Stage::Ready;
                        continue;
                    }
                    self.charge((self.products[i].len() + self.rows[i].len()) as u64)?;
                    self.stage = Stage::Matrix(i + 1);
                    return Ok(Event::MatrixRow {
                        row: i as u64,
                        version: 0,
                        words: words(&self.rows[i]),
                    });
                }
                Stage::Ready => {
                    self.stage = Stage::Pivot;
                    return Ok(Event::MatrixReady {
                        rows: self.rows.len() as u64,
                        columns: self.columns.len() as u64,
                        candidates: self.candidates,
                    });
                }
                Stage::Pivot => {
                    if self.rank == self.rows.len() || self.column == self.columns.len() {
                        self.stage = Stage::Output(0);
                        continue;
                    }
                    let mut selected = None;
                    for r in self.rank..self.rows.len() {
                        self.work(1)?;
                        self.charge(1)?;
                        if self.bit(r, self.column) {
                            selected = Some(r);
                            break;
                        }
                    }
                    let event = Event::Pivot {
                        column: self.column as u64,
                        pivot_row: self.rank as u64,
                        selected_row: selected.map(|r| r as u64),
                    };
                    if let Some(r) = selected {
                        self.stage = Stage::Swap(r);
                    } else {
                        self.column += 1;
                    }
                    return Ok(event);
                }
                Stage::Swap(right) => {
                    self.work(1)?;
                    self.charge(1)?;
                    let left = self.rank;
                    let lv = self.versions[left];
                    let rv = self.versions[right];
                    self.rows.swap(left, right);
                    self.versions[left] += 1;
                    if left != right {
                        self.versions[right] += 1;
                    }
                    self.stage = Stage::Clear(0);
                    return Ok(Event::Swap {
                        left: left as u64,
                        right: right as u64,
                        left_version: lv,
                        right_version: rv,
                        left_after_version: self.versions[left],
                        right_after_version: self.versions[right],
                    });
                }
                Stage::Clear(target) => {
                    if target == self.rows.len() {
                        self.rank += 1;
                        self.column += 1;
                        self.stage = Stage::Pivot;
                        continue;
                    }
                    self.stage = Stage::Clear(target + 1);
                    if target == self.rank {
                        continue;
                    }
                    self.work(1)?;
                    self.charge(1)?;
                    if !self.bit(target, self.column) {
                        continue;
                    }
                    let start = self.column / 64;
                    require(
                        self.rows[self.rank][..start].iter().all(|w| *w == 0),
                        "nonzero omitted pivot word",
                    )?;
                    let width = self.rows[target].len();
                    self.work(width)?;
                    self.charge((width - start) as u64)?;
                    let tv = self.versions[target];
                    let pv = self.versions[self.rank];
                    // Full-row identity is checked independently of the suffix count.
                    for w in 0..width {
                        self.rows[target][w] ^= self.rows[self.rank][w];
                    }
                    self.versions[target] += 1;
                    self.xors += (width - start) as u64;
                    return Ok(Event::RowXor {
                        target: target as u64,
                        pivot: self.rank as u64,
                        target_version: tv,
                        pivot_version: pv,
                        after_version: self.versions[target],
                        start_word: start as u64,
                        words_after: words(&self.rows[target]),
                    });
                }
                Stage::Output(i) => {
                    if i == self.rows.len() {
                        self.stage = Stage::End;
                        continue;
                    }
                    self.work(self.rows[i].len())?;
                    self.charge(self.rows[i].len() as u64)?;
                    self.stage = Stage::Output(i + 1);
                    return Ok(Event::OutputRow {
                        row: i as u64,
                        version: self.versions[i],
                        words: words(&self.rows[i]),
                    });
                }
                Stage::End => {
                    let original = self.original.clone();
                    let oracle = scalar_rank(&original, self.columns.len(), &mut self.work)?;
                    require(oracle == self.rank, "independent scalar rank disagrees")?;
                    self.oracle_rank = Some(oracle);
                    self.stage = Stage::Done;
                    return Ok(Event::End {
                        rank_native: self.rank as u64,
                        word_xors_native: self.xors,
                        rows: self.rows.len() as u64,
                        columns: self.columns.len() as u64,
                        candidates: self.candidates,
                        admission_units: self.admission,
                        claims: Claims::unclaimed(),
                    });
                }
                Stage::Done => return Err("data after mathematical end".into()),
            }
        }
    }
}

// Independent scalar basis insertion, not the recorded RREF or packed XOR path.
fn scalar_rank(rows: &[Vec<u64>], columns: usize, work: &mut u64) -> Result<usize> {
    let mut basis: Vec<Option<Vec<bool>>> = vec![None; columns];
    let mut rank = 0;
    for packed in rows {
        let mut row: Vec<bool> = (0..columns)
            .map(|c| packed[c / 64] & (1 << (c % 64)) != 0)
            .collect();
        *work += columns as u64;
        for c in 0..columns {
            *work += 1;
            require(*work <= WORK_CAP, "checker rank work limit exceeded")?;
            if !row[c] {
                continue;
            }
            if let Some(b) = &basis[c] {
                for j in 0..columns {
                    row[j] ^= b[j];
                }
                *work += columns as u64;
            } else {
                basis[c] = Some(row);
                rank += 1;
                break;
            }
        }
    }
    require(*work <= WORK_CAP, "checker rank work limit exceeded")?;
    Ok(rank)
}

// fill_buf bounds the pending allocation before copying; no unbounded read_line.
fn line<R: BufRead>(reader: &mut R, total: &mut usize) -> Result<Option<(Vec<u8>, bool)>> {
    let mut out = Vec::new();
    loop {
        let buf = reader.fill_buf().map_err(|e| e.to_string())?;
        if buf.is_empty() {
            return Ok((!out.is_empty()).then_some((out, false)));
        }
        let newline = buf.iter().position(|c| *c == b'\n');
        let n = newline.map_or(buf.len(), |i| i + 1);
        require(out.len() + n <= RECORD_CAP, "record byte limit exceeded")?;
        require(*total + n <= STREAM_CAP, "stream byte limit exceeded")?;
        out.extend_from_slice(&buf[..n]);
        reader.consume(n);
        *total += n;
        if newline.is_some() {
            out.pop();
            return Ok(Some((out, true)));
        }
    }
}

pub fn verify<R: BufRead>(mut reader: R, expected: &Binding) -> Result<Report> {
    validate_binding(expected)?;
    let mut report = Report {
        schema: "crypto.boolean-matrix-independent-review/v1",
        acknowledged_records: 0,
        acknowledged_bytes: 0,
        trailing_unacknowledged_bytes: 0,
        last_record_sha256: "0".repeat(64),
        supplied_bindings_matched: false,
        matrix_trace_complete: false,
        independent_rank: None,
        native_word_xors_checked: None,
        named_admission_model_checked: None,
        checker_logical_operations: 0,
        stop_reason: None,
        stop_reason_independently_verified: false,
        input_encoding_checked: false,
        producer_execution_attested: false,
        sink_durability_verified: false,
        total_operation_count_verified: false,
        unsat_verified: false,
        groebner_basis_complete: false,
        first_fall_degree: None,
        last_fall_degree: None,
    };
    let mut replay: Option<Replay> = None;
    let mut terminal = false;
    let mut total = 0;
    let mut input_hash_checked = false;
    while let Some((bytes, complete)) = line(&mut reader, &mut total)? {
        require(!terminal, "data after terminal event")?;
        if !complete {
            report.trailing_unacknowledged_bytes = bytes.len();
            break;
        }
        require(
            report.acknowledged_records < EVENT_CAP,
            "event count limit exceeded",
        )?;
        let record: Record =
            serde_json::from_slice(&bytes).map_err(|e| format!("record schema: {e}"))?;
        let canonical = serde_json::to_vec(&record).map_err(|e| e.to_string())?;
        require(canonical == bytes, "record is not canonical JSON")?;
        require(
            record.schema == SCHEMA
                && record.sequence == report.acknowledged_records + 1
                && record.previous_sha256 == report.last_record_sha256,
            "schema/sequence/hash-chain mismatch",
        )?;
        if let Some(state) = &mut replay {
            if let Event::Stopped {
                reason,
                acknowledged_before_stop,
            } = record.event
            {
                require(
                    acknowledged_before_stop == report.acknowledged_records,
                    "stop prefix count mismatch",
                )?;
                match &reason {
                    StopReason::InvalidInput { detail }
                    | StopReason::UnsupportedProfile { detail }
                    | StopReason::SinkFailure { detail }
                    | StopReason::Integrity { detail } => {
                        require(detail.len() <= 256, "stop detail exceeds cap")?
                    }
                    _ => (),
                }
                report.stop_reason = Some(reason);
                terminal = true;
            } else if state.inputs.len() < state.input_count {
                state.input(&record.event)?;
            } else {
                if !input_hash_checked {
                    state.verify_input_digest()?;
                    input_hash_checked = true;
                }
                let wanted = state.next()?;
                require(
                    record.event == wanted,
                    "arithmetic/schedule/claims mismatch",
                )?;
                if matches!(record.event, Event::End { .. }) {
                    report.matrix_trace_complete = true;
                    report.independent_rank = state.oracle_rank;
                    report.native_word_xors_checked = Some(state.xors);
                    report.named_admission_model_checked = Some(state.admission);
                    terminal = true;
                }
            }
        } else {
            replay = Some(Replay::begin(record.event, expected)?);
            report.supplied_bindings_matched = true;
        }
        let state = replay.as_ref().ok_or("missing replay state")?;
        require(
            record.sequence <= state.limits.max_events
                && total as u64 <= state.limits.max_stream_bytes
                && (bytes.len() + 1) as u64 <= state.limits.max_event_bytes,
            "record exceeds declared producer limits",
        )?;
        if !terminal {
            require(
                record.sequence < state.limits.max_events
                    && total as u64 + 1024 <= state.limits.max_stream_bytes,
                "record consumes reserved terminal budget",
            )?;
        }
        report.acknowledged_records += 1;
        report.acknowledged_bytes = total;
        report.last_record_sha256 = digest(&bytes);
        report.checker_logical_operations = state.work;
    }
    Ok(report)
}

#[cfg(test)]
mod tests;
