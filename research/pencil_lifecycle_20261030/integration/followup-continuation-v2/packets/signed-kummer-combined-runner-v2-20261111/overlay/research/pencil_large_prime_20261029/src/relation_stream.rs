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
    /// Native callbacks retain their own convention; never pencil Admission rows.
    ObservedHeader { identity: Identity, native: serde_json::Value },
    Observed(serde_json::Value),
    ObservedTerminal(serde_json::Value),
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
        let is_header = matches!(event, Event::Header(_) | Event::ObservedHeader { .. });
        let is_terminal = matches!(event, Event::Terminal(_) | Event::ObservedTerminal(_));
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

/// Diagnostic adapter for existing native callbacks. Reuses Trace limits,
/// sequence, flushing and poisoned-prefix semantics; performs no field work.
/// This validates event coverage, not geometry, scalar correctness or ranks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeProtocol { SignedCollector, KummerLabels }

pub struct NativeEvents<'a, 'w> {
    trace: &'a mut Trace<'w>,
    expected_header: serde_json::Value,
    protocol: NativeProtocol,
    started: bool,
    terminal: bool,
    failed: bool,
    attempts: u64,
    pending: Option<u64>,
    recovered: Option<u64>,
    nodes: u64,
    first: std::collections::BTreeMap<String, u64>,
}
impl<'a, 'w> NativeEvents<'a, 'w> {
    pub fn new(trace: &'a mut Trace<'w>, protocol: NativeProtocol,
        expected_header: serde_json::Value) -> Result<Self, Error> {
        if trace.records_written() != 0 || !expected_header.is_object()
            || expected_header["kind"] != "header" { return Err(Error::State); }
        let schema = match protocol {
            NativeProtocol::SignedCollector => "signed-collector-events/v1",
            NativeProtocol::KummerLabels => "kummer-label-events/v1",
        };
        if expected_header["schema"] != schema { return Err(Error::State); }
        Ok(Self { trace, expected_header, protocol, started:false, terminal:false,
            failed:false, attempts:0, pending:None, recovered:None, nodes:1,
            first:std::collections::BTreeMap::new() })
    }
    pub fn observe(&mut self, value: serde_json::Value) -> Result<(), Error> {
        if let Some(error) = self.trace.failure() { return Err(error.clone()); }
        if self.failed || self.terminal { return self.reject(); }
        match self.accept(value) {
            Ok(()) => Ok(()),
            Err(error) => {
                self.failed=true;
                let first=self.trace.failure.get_or_insert(error).clone();
                Err(first)
            }
        }
    }
    fn reject<T>(&mut self) -> Result<T, Error> {
        self.failed=true;
        let first=self.trace.failure.get_or_insert(Error::State).clone();
        Err(first)
    }
    fn number(value:&serde_json::Value, field:&str) -> Result<u64, Error> {
        value.get(field).and_then(|v|v.as_u64()).ok_or(Error::State)
    }
    fn scalar(value:&serde_json::Value, field:&str) -> Result<Option<u64>, Error> {
        match value.get(field) {
            Some(v) if v.is_null() => Ok(None),
            Some(v) => v.as_u64().map(Some).ok_or(Error::State),
            None => Err(Error::State),
        }
    }
    fn accept(&mut self, value:serde_json::Value) -> Result<(), Error> {
        let kind=value.get("kind").and_then(|x|x.as_str()).ok_or(Error::State)?;
        if !self.started {
            if value != self.expected_header { return Err(Error::State); }
            self.trace.emit(Event::ObservedHeader { identity:self.trace.identity().clone(), native:value })?;
            self.started=true; return Ok(());
        }
        if kind=="header" { return Err(Error::State); }
        if kind=="terminal" {
            if self.pending.is_some() || Self::number(&value,"attempts")? != self.attempts
                || Self::scalar(&value,"recovered")? != self.recovered { return Err(Error::State); }
            self.trace.emit(Event::ObservedTerminal(value))?;
            self.terminal=true; return Ok(());
        }
        match (self.protocol,kind) {
            (NativeProtocol::SignedCollector,"candidate") |
            (NativeProtocol::KummerLabels,"label_candidate") => {
                let attempt=Self::number(&value,"attempt")?;
                if self.pending.is_some() || self.recovered.is_some()
                    || self.attempts.checked_add(1)!=Some(attempt) { return Err(Error::State); }
                self.attempts=attempt; self.pending=Some(attempt);
            }
            (NativeProtocol::SignedCollector,"node") => {
                if self.pending!=Some(Self::number(&value,"attempt")?)
                    || Self::number(&value,"node")? != self.nodes { return Err(Error::State); }
                self.nodes=self.nodes.checked_add(1).ok_or(Error::State)?;
            }
            (NativeProtocol::SignedCollector,"edge") => {
                let attempt=Self::number(&value,"attempt")?;
                if self.pending!=Some(attempt) || Self::number(&value,"node_count")?!=self.nodes {
                    return Err(Error::State);
                }
                let nodes=value["nodes"].as_array().ok_or(Error::State)?;
                let signs=value["signs"].as_array().ok_or(Error::State)?;
                if nodes.len()!=2 || signs.len()!=2 || nodes.iter().any(|n|n.as_u64().is_none_or(|n|n>=self.nodes))
                    || signs.iter().any(|s|!matches!(s.as_i64(),Some(-1)|Some(1))) { return Err(Error::State); }
                if value.get("canonical").is_none() { return Err(Error::State); }
                let key=serde_json::to_string(&value["canonical"]).map_err(|_|Error::State)?;
                if self.first.insert(key,attempt).is_some() { return Err(Error::State); }
                self.recovered=Self::scalar(&value,"recovered")?; self.pending=None;
            }
            (NativeProtocol::SignedCollector,"duplicate") => {
                if self.pending!=Some(Self::number(&value,"attempt")?) { return Err(Error::State); }
                let key=serde_json::to_string(&value["canonical"]).map_err(|_|Error::State)?;
                if self.first.get(&key).copied()!=Some(Self::number(&value,"first_attempt")?) {
                    return Err(Error::State);
                }
                self.pending=None;
            }
            (NativeProtocol::SignedCollector,"identity") => {
                if self.pending!=Some(Self::number(&value,"attempt")?) { return Err(Error::State); }
                self.recovered=Self::scalar(&value,"recovered")?;self.pending=None;
            }
            (NativeProtocol::SignedCollector,"nonsplit") => {
                if self.pending!=Some(Self::number(&value,"attempt")?) { return Err(Error::State); }
                self.pending=None;
            }
            (NativeProtocol::SignedCollector,"choice") => {
                let selected=Self::number(&value,"selected_attempt")?;
                if self.pending.is_some() || self.recovered.is_some() || selected==0 || selected>self.attempts {
                    return Err(Error::State);
                }
            }
            (NativeProtocol::SignedCollector,"restart") => {
                if self.pending.is_some() || self.recovered.is_some()
                    || Self::number(&value,"after_attempt")?!=self.attempts { return Err(Error::State); }
            }
            (NativeProtocol::KummerLabels,"label_result") => {
                if self.pending!=Some(Self::number(&value,"attempt")?) { return Err(Error::State); }
                let result=&value["result"];
                if !matches!(result["status"].as_str(),Some("first"|"redundant"|"invariant_error"|
                    "candidate_verified"|"candidate_rejected"|"noncanonical_label")) { return Err(Error::State); }
                self.recovered=Self::scalar(result,"answer")?;
                self.pending=None;
            }
            _ => return Err(Error::State),
        }
        self.trace.emit(Event::Observed(value))
    }
}

#[cfg(test)]
mod native_adapter_controls {
 use super::*;
 use serde_json::json;
 fn identity()->Identity { Identity { source_revision:"a".repeat(40),source_sha256:"b".repeat(64),workload_id:"target".into(),run_id:"attempt".into() } }
 fn header()->serde_json::Value { json!({"kind":"header","schema":"signed-collector-events/v1","r":7,"target":"bound-target"}) }
 fn candidate(a:u64)->serde_json::Value { json!({"kind":"candidate","attempt":a}) }
 #[test]
 fn identity_before_terminal_is_retained() {
  let mut out=Vec::new();
  { let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
    { let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
      a.observe(header()).unwrap();a.observe(candidate(1)).unwrap();
      a.observe(json!({"kind":"identity","attempt":1,"recovered":3})).unwrap();
      a.observe(json!({"kind":"terminal","attempts":1,"recovered":3})).unwrap(); }
    assert!(t.complete());assert_eq!(t.records_written(),4); }
  let rows:Vec<Record>=out.split(|b|*b==b'\n').filter(|b|!b.is_empty()).map(|b|serde_json::from_slice(b).unwrap()).collect();
  assert!(matches!(&rows[2].event,Event::Observed(v) if v["kind"]=="identity"));
 }
 #[test]
 fn duplicate_disposition_and_first_reference_are_retained() {
  let mut out=Vec::new();let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
  { let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
    a.observe(header()).unwrap();a.observe(candidate(1)).unwrap();
    a.observe(json!({"kind":"edge","attempt":1,"node_count":1,"nodes":[0,0],"signs":[1,-1],"canonical":[1,2],"recovered":null})).unwrap();
    a.observe(candidate(2)).unwrap();
    a.observe(json!({"kind":"duplicate","attempt":2,"canonical":[1,2],"first_attempt":1})).unwrap();
    a.observe(json!({"kind":"terminal","attempts":2,"recovered":null})).unwrap(); }
  assert!(t.complete());assert_eq!(t.records_written(),6);
 }
 #[test]
 fn missing_disposition_poisoned() {
  let mut out=Vec::new();let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
  { let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
    a.observe(header()).unwrap();a.observe(candidate(1)).unwrap();
    assert_eq!(a.observe(json!({"kind":"terminal","attempts":1,"recovered":null})),Err(Error::State)); }
  assert!(!t.complete());assert_eq!(t.failure(),Some(&Error::State));
 }
 #[test]
 fn post_identity_candidate_rejected() {
  let mut out=Vec::new();let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
  let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
  a.observe(header()).unwrap();a.observe(candidate(1)).unwrap();
  a.observe(json!({"kind":"identity","attempt":1,"recovered":3})).unwrap();
  assert_eq!(a.observe(candidate(2)),Err(Error::State));
 }
 #[test]
 fn changed_target_header_rejected() {
  let mut out=Vec::new();let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
  let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
  let mut wrong=header();wrong["target"]=json!("other");assert_eq!(a.observe(wrong),Err(Error::State));
 }
 #[test]
 fn early_kummer_rejection_has_result_and_terminal() {
  let h=json!({"kind":"header","schema":"kummer-label-events/v1","provenance":"fixed_control","r":7});
  let mut out=Vec::new();let mut t=Trace::new(&mut out,identity(),20,10000).unwrap();
  { let mut a=NativeEvents::new(&mut t,NativeProtocol::KummerLabels,h.clone()).unwrap();
    a.observe(h).unwrap();a.observe(json!({"kind":"label_candidate","attempt":1,"a":7,"b":0})).unwrap();
    a.observe(json!({"kind":"label_result","attempt":1,"result":{"status":"noncanonical_label","emit_graph_edge":false,"answer":null}})).unwrap();
    a.observe(json!({"kind":"terminal","attempts":1,"recovered":null})).unwrap(); }
  assert!(t.complete());assert_eq!(t.records_written(),4);
 }
 struct FailsSecondWrite { calls:usize }
 impl std::io::Write for FailsSecondWrite {
  fn write(&mut self, bytes:&[u8])->std::io::Result<usize> {
   self.calls+=1;
   if self.calls==2 { Err(std::io::Error::other("injected native callback output failure")) }
   else { Ok(bytes.len()) }
  }
  fn flush(&mut self)->std::io::Result<()> { Ok(()) }
 }
 #[test]
 fn callback_preserves_first_output_error_after_later_event() {
  let mut sink=FailsSecondWrite {calls:0};
  let mut t=Trace::new(&mut sink,identity(),20,10000).unwrap();
  let first;
  { let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
    // Exercise the same fallible closure shape passed to the native producer.
    a.observe(header()).unwrap();
    let mut callback=|event| a.observe(event);
    first=callback(candidate(1)).unwrap_err();
    assert!(matches!(&first,Error::Output(message) if message.contains("injected native callback")));
    assert_eq!(callback(json!({"kind":"identity","attempt":1,"recovered":3})),Err(first.clone()));
  }
  assert_eq!(t.failure(),Some(&first));assert!(!t.complete());assert_eq!(t.records_written(),1);
 }
 #[test]
 fn callback_preserves_first_record_limit_after_later_event() {
  let mut sink=Vec::new();let mut t=Trace::new(&mut sink,identity(),2,10000).unwrap();
  { let mut a=NativeEvents::new(&mut t,NativeProtocol::SignedCollector,header()).unwrap();
    a.observe(header()).unwrap();a.observe(candidate(1)).unwrap();
    let error=a.observe(json!({"kind":"identity","attempt":1,"recovered":3})).unwrap_err();
    assert_eq!(error,Error::RecordLimit);
    assert_eq!(a.observe(json!({"kind":"terminal","attempts":1,"recovered":3})),Err(error));
  }
  assert_eq!(t.failure(),Some(&Error::RecordLimit));assert!(!t.complete());
 }

}
