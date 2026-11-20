//! Bounded diagnostic capture. No rank calculation or counter reset occurs here.
//! A failed sink leaves an explicitly incomplete prefix; callers must propagate errors.
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    pub source_revision: String,
    pub source_sha256: String,
    pub workload_id: String,
    pub run_id: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Divisor {
    pub degree: usize,
    pub u: [u64; 3],
    pub v: [u64; 2],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Convention {
    PencilAugmentedRhs,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    pub schema: String,
    pub identity: Identity,
    pub convention: Convention,
    pub p: u64,
    pub r: u64,
    pub order: u64,
    pub cofactor: u64,
    pub seed: u64,
    pub source_coefficients: Vec<u64>,
    pub polynomial: Vec<u64>,
    pub generator: Point,
    pub target: Point,
    pub x_to_column: Vec<Option<usize>>,
    pub nuisance_columns: usize,
    pub small_columns: Vec<usize>,
    pub column_map_version: u64,
    pub denominator: usize,
    pub coefficient_cap: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Disposition {
    DegreeSkip,
    Rejected,
    Verified,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Candidate {
    pub walk_index: u64,
    pub lambda: Option<u64>,
    pub a: u64,
    pub b: u64,
    pub divisor: Divisor,
    pub points: Vec<[u64; 2]>,
    pub disposition: Disposition,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Admission {
    Joined,
    Anchored,
    Independent,
    Dependent,
    RejectedThreeLarge,
    StoppedWorkingCap,
    StoppedRetainedCap,
    AlreadyStopped,
}
impl From<crate::collector::Admission> for Admission {
    fn from(a: crate::collector::Admission) -> Self {
        match a {
            crate::collector::Admission::Joined => Self::Joined,
            crate::collector::Admission::Anchored => Self::Anchored,
            crate::collector::Admission::Independent => Self::Independent,
            crate::collector::Admission::Dependent => Self::Dependent,
            crate::collector::Admission::RejectedThreeLarge => Self::RejectedThreeLarge,
            crate::collector::Admission::StoppedWorkingCap => Self::StoppedWorkingCap,
            crate::collector::Admission::StoppedRetainedCap => Self::StoppedRetainedCap,
            crate::collector::Admission::AlreadyStopped => Self::AlreadyStopped,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Relation {
    pub walk_index: u64,
    pub lambda: u64,
    /// Exact original terms, including their order, duplicate columns and zeros.
    pub submitted: Vec<(usize, u64)>,
    pub admission: Admission,
    /// Small-column coordinates. A cap can mutate the graph before this pending row.
    pub pending_constraint: Option<Vec<(usize, u64)>>,
    pub residual_rank: usize,
    pub candidate_scalar: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Terminal {
    pub walks: u64,
    /// Enumerated lambdas, including remaining candidates on an early stop.
    pub candidate_lambdas: usize,
    pub verified_relations: usize,
    pub rejected_candidates: usize,
    pub submitted_rows: u64,
    pub admitted_rows: u64,
    pub recovered: Option<u64>,
    pub source_verified: bool,
    pub failure: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", deny_unknown_fields)]
pub enum Event {
    Header(Header),
    Candidate(Candidate),
    Relation(Relation),
    Terminal(Terminal),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub sequence: u64,
    pub event: Event,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    InvalidIdentity,
    InvalidLimits,
    RecordLimit,
    ByteLimit,
    Output(String),
    State,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

/// Each instance is single-use. Limits include Header and Terminal and JSONL newlines.
/// Output failure may leave a partial final record; `failure()` remains authoritative.
pub struct Trace<'a> {
    output: &'a mut dyn Write,
    identity: Identity,
    max_records: u64,
    max_bytes: usize,
    records: u64,
    bytes: usize,
    started: bool,
    terminal: bool,
    failure: Option<Error>,
}
impl<'a> Trace<'a> {
    pub fn new(
        output: &'a mut dyn Write,
        identity: Identity,
        max_records: u64,
        max_bytes: usize,
    ) -> Result<Self, Error> {
        let hex = |s: &str, n| s.len() == n && s.bytes().all(|b| b.is_ascii_hexdigit());
        if !hex(&identity.source_revision, 40)
            || !hex(&identity.source_sha256, 64)
            || identity.workload_id.is_empty()
            || identity.run_id.is_empty()
        {
            return Err(Error::InvalidIdentity);
        }
        if max_records < 2 || max_bytes == 0 {
            return Err(Error::InvalidLimits);
        }
        Ok(Self {
            output,
            identity,
            max_records,
            max_bytes,
            records: 0,
            bytes: 0,
            started: false,
            terminal: false,
            failure: None,
        })
    }
    pub fn identity(&self) -> &Identity {
        &self.identity
    }
    pub fn failure(&self) -> Option<&Error> {
        self.failure.as_ref()
    }
    pub fn complete(&self) -> bool {
        self.terminal && self.failure.is_none()
    }
    pub fn records_written(&self) -> u64 {
        self.records
    }
    pub fn bytes_written(&self) -> usize {
        self.bytes
    }
    /// Low-level native adapter entry point, also used by the root ecbench crate.
    /// Events are caller assertions; this writer does not authenticate their provenance.
    pub fn emit(&mut self, event: Event) -> Result<(), Error> {
        if let Some(e) = &self.failure {
            return Err(e.clone());
        }
        let result = self.write_event(event);
        if let Err(e) = &result {
            self.failure = Some(e.clone());
        }
        result
    }
    fn write_event(&mut self, event: Event) -> Result<(), Error> {
        let is_header = matches!(event, Event::Header(_));
        let is_terminal = matches!(event, Event::Terminal(_));
        if self.terminal || self.started == is_header {
            return Err(Error::State);
        }
        if self.records >= self.max_records {
            return Err(Error::RecordLimit);
        }
        let remaining = self.max_bytes - self.bytes;
        let mut bounded = Buffer {
            bytes: Vec::new(),
            limit: remaining.saturating_sub(1),
            exceeded: false,
        };
        serde_json::to_writer(
            &mut bounded,
            &Record {
                sequence: self.records,
                event,
            },
        )
        .map_err(|e| {
            if bounded.exceeded {
                Error::ByteLimit
            } else {
                Error::Output(e.to_string())
            }
        })?;
        if remaining == 0 {
            return Err(Error::ByteLimit);
        }
        bounded.bytes.push(b'\n');
        self.output
            .write_all(&bounded.bytes)
            .map_err(|e| Error::Output(e.to_string()))?;
        // Flush every record, including the terminal; a buffered failed terminal is not complete.
        self.output
            .flush()
            .map_err(|e| Error::Output(e.to_string()))?;
        self.bytes += bounded.bytes.len();
        self.records += 1;
        self.started = true;
        self.terminal = is_terminal;
        Ok(())
    }
}
struct Buffer {
    bytes: Vec<u8>,
    limit: usize,
    exceeded: bool,
}
impl Write for Buffer {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        if b.len() > self.limit.saturating_sub(self.bytes.len()) {
            self.exceeded = true;
            return Err(io::Error::other("trace byte limit"));
        }
        self.bytes.extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn identity() -> Identity {
        Identity {
            source_revision: "a".repeat(40),
            source_sha256: "b".repeat(64),
            workload_id: "fixture".into(),
            run_id: "attempt1".into(),
        }
    }
    fn header() -> Event {
        Event::Header(Header {
            schema: "pencil-relation-stream/v1".into(),
            identity: identity(),
            convention: Convention::PencilAugmentedRhs,
            p: 739,
            r: 34147,
            order: 34147,
            cofactor: 1,
            seed: 1,
            source_coefficients: vec![],
            polynomial: vec![],
            generator: Point {
                x: 0,
                y: 1,
                infinity: false,
            },
            target: Point {
                x: 0,
                y: 1,
                infinity: false,
            },
            x_to_column: vec![Some(0)],
            nuisance_columns: 1,
            small_columns: vec![0, 1, 2],
            column_map_version: 0,
            denominator: 2,
            coefficient_cap: 100,
        })
    }
    fn terminal() -> Event {
        Event::Terminal(Terminal {
            walks: 0,
            candidate_lambdas: 0,
            verified_relations: 0,
            rejected_candidates: 0,
            submitted_rows: 0,
            admitted_rows: 0,
            recovered: None,
            source_verified: false,
            failure: Some("walk_cap".into()),
        })
    }
    #[test]
    fn ordered_terminal_is_single_use() {
        let mut bytes = Vec::new();
        {
            let mut t = Trace::new(&mut bytes, identity(), 2, 8192).unwrap();
            t.emit(header()).unwrap();
            t.emit(terminal()).unwrap();
            assert!(t.complete());
            assert_eq!(t.emit(terminal()), Err(Error::State));
            assert!(!t.complete());
        }
        let records: Vec<Record> = bytes
            .split(|&b| b == b'\n')
            .filter(|b| !b.is_empty())
            .map(|b| serde_json::from_slice(b).unwrap())
            .collect();
        assert_eq!(records.len(), 2);
        assert_eq!(records[1].sequence, 1);
    }
    #[test]
    fn byte_and_record_caps_are_explicit() {
        let mut out = Vec::new();
        let mut t = Trace::new(&mut out, identity(), 2, 1).unwrap();
        assert_eq!(t.emit(header()), Err(Error::ByteLimit));
        assert_eq!(t.bytes_written(), 0);
        assert!(!t.complete());
        let mut out = Vec::new();
        let mut t = Trace::new(&mut out, identity(), 2, 8192).unwrap();
        t.emit(header()).unwrap();
        let candidate = Event::Candidate(Candidate {
            walk_index: 0,
            lambda: None,
            a: 0,
            b: 1,
            divisor: Divisor {
                degree: 0,
                u: [1, 0, 0],
                v: [0, 0],
            },
            points: vec![],
            disposition: Disposition::DegreeSkip,
        });
        t.emit(candidate).unwrap();
        assert_eq!(t.emit(terminal()), Err(Error::RecordLimit));
        assert!(!t.complete());
    }
    #[test]
    fn terminal_flush_failure_keeps_capture_incomplete_even_with_full_bytes() {
        struct FailSecondFlush {
            bytes: Vec<u8>,
            flushes: usize,
        }
        impl Write for FailSecondFlush {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                self.bytes.extend_from_slice(bytes);
                Ok(bytes.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                self.flushes += 1;
                if self.flushes == 2 {
                    Err(io::Error::other("terminal flush failure"))
                } else {
                    Ok(())
                }
            }
        }
        let mut sink = FailSecondFlush {
            bytes: vec![],
            flushes: 0,
        };
        {
            let mut t = Trace::new(&mut sink, identity(), 10, 8192).unwrap();
            t.emit(header()).unwrap();
            assert!(matches!(t.emit(terminal()), Err(Error::Output(_))));
            assert!(!t.complete());
            assert_eq!(t.records_written(), 1);
        }
        assert_eq!(sink.bytes.iter().filter(|&&b| b == b'\n').count(), 2);
    }
    struct Broken;
    impl Write for Broken {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("intentional output failure"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[test]
    fn sink_and_state_errors_poison_capture() {
        let mut b = Broken;
        let mut t = Trace::new(&mut b, identity(), 10, 8192).unwrap();
        assert!(matches!(t.emit(header()), Err(Error::Output(_))));
        assert!(!t.complete());
        let mut out = Vec::new();
        let mut t = Trace::new(&mut out, identity(), 10, 8192).unwrap();
        assert_eq!(t.emit(terminal()), Err(Error::State));
        assert_eq!(t.emit(header()), Err(Error::State));
    }
}
