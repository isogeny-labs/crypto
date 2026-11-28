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
        if self.failed || self.terminal { return self.reject(); }
        match self.accept(value) {
            Ok(()) => Ok(()),
            Err(error) => { self.failed=true; self.trace.failure=Some(error.clone()); Err(error) }
        }
    }
    fn reject<T>(&mut self) -> Result<T, Error> {
        self.failed=true; self.trace.failure=Some(Error::State); Err(Error::State)
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
