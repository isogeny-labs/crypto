//! Bounded diagnostic recorder for native signed collector events.
//! The byte bound covers serialized records, not producer/callback allocations.
use serde_json::{json, Value};

pub struct Capture {
    pub records: Vec<Value>,
    max_records: usize,
    max_bytes: usize,
    bytes: usize,
    failed: bool,
    complete: bool,
}
impl Capture {
    pub fn new(max_records: usize, max_bytes: usize) -> Result<Self, String> {
        if max_records == 0 || max_bytes == 0 {
            return Err("positive limits required".into());
        }
        Ok(Self {
            records: vec![],
            max_records,
            max_bytes,
            bytes: 0,
            failed: false,
            complete: false,
        })
    }
    pub fn complete(&self) -> bool {
        self.complete && !self.failed
    }
    pub fn push(&mut self, event: Value) -> Result<(), String> {
        if self.failed || self.complete {
            self.failed = true;
            return Err("capture closed".into());
        }
        let kind = event.get("kind").and_then(Value::as_str).ok_or_else(|| {
            self.failed = true;
            "missing event kind".to_string()
        })?;
        if (self.records.is_empty() && kind != "header")
            || (!self.records.is_empty() && kind == "header")
        {
            self.failed = true;
            return Err("header position".into());
        }
        let terminal = kind == "terminal";
        let row = json!({"sequence":self.records.len(),"event":event});
        let size = serde_json::to_vec(&row)
            .map_err(|e| e.to_string())?
            .len()
            .checked_add(1)
            .ok_or("size overflow")?;
        let next = self.bytes.checked_add(size).ok_or("size overflow")?;
        if self.records.len() >= self.max_records || next > self.max_bytes {
            self.failed = true;
            return Err("capture limit".into());
        }
        self.bytes = next;
        self.records.push(row);
        self.complete = terminal;
        Ok(())
    }
}
