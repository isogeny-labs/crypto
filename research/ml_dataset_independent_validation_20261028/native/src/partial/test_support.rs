//! Finite synthetic wire writer. It does not call any consumer validator.
//! Metadata mutations occur before their enclosing raw hashes are computed.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

pub(super) fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub(super) fn wire(value: &Value) -> Vec<u8> {
    let mut raw = serde_json::to_vec_pretty(value).unwrap();
    raw.push(b'\n');
    raw
}
pub(super) fn saved(name: &str) -> Vec<u8> {
    fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/partial/context")
            .join(name),
    )
    .unwrap()
}
pub(super) fn role(name: &str) -> &'static str {
    match name {
        "run.json" => "restricted-run-binding",
        "plan.json" => "restricted-frozen-plan",
        "protocol.json" => "restricted-frozen-protocol",
        "config.json" => "restricted-provenance",
        "attempt.json" => "attempt-provenance",
        "measurement-policy.json" => "restricted-measurement-policy",
        "host.json" => "host-measurement-context",
        "public.json" => "public-instance",
        "witness.json" => "restricted-known-answer",
        "factor-base.json" => "factor-base",
        "worker-process.json" => "process-provenance",
        n if n.ends_with("-start.json") => "phase-start",
        _ => "hardware-and-resource-telemetry",
    }
}
pub(super) fn descriptor(path: &str, role: &str, encoding: &str, bytes: &[u8]) -> Value {
    json!({"path":path,"role":role,"encoding":encoding,"sha256":hash(bytes),"bytes":bytes.len()})
}
fn object(bytes: &[u8]) -> Value {
    let sha = hash(bytes);
    json!({"key":format!("sha256/{}/{sha}",&sha[..2]),"sha256":sha,"bytes":bytes.len()})
}
fn binding_hash(binding: &Value) -> String {
    // Producer's Binding declaration order, independent of serde_json map order.
    let names = [
        "schema",
        "run_sha256",
        "plan_sha256",
        "protocol_sha256",
        "config_sha256",
        "source_commit",
        "binary_sha256",
        "cell_index",
        "attempt_sha256",
    ];
    let mut out = String::from("{");
    for (i, n) in names.iter().enumerate() {
        if i != 0 {
            out.push(',')
        }
        out.push_str(&format!(
            "{}:{}",
            serde_json::to_string(n).unwrap(),
            binding[n]
        ));
    }
    out.push('}');
    hash(out.as_bytes())
}
fn genesis(binding: &Value) -> Value {
    let digest = binding_hash(binding);
    // Frozen producer genesis is a hash of these literal bytes.
    let mut bytes = b"crypto.ml-dataset.capture-genesis/v1\n".to_vec();
    bytes.extend_from_slice(digest.as_bytes());
    bytes.push(b'\n');
    json!({"byte_end":0,"event_count":0,"last_event_sha256":hash(&bytes),"next_chunk":0,"payload_count":0,"payload_bytes":0})
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Stage {
    Manifest,
    Receipt,
    Catalog,
    Anchor,
}
pub(super) struct Fixture {
    pub binding: Value,
    pub remote: String,
    pub snapshots: Vec<Value>,
    pub blobs: BTreeMap<String, Vec<u8>>,
    cursor: Value,
    stop: Value,
}
pub(super) struct Published {
    pub temp: tempfile::TempDir,
    pub anchor: PathBuf,
    pub mirror: PathBuf,
    pub out: PathBuf,
    pub pin: String,
    pub anchors: Vec<Value>,
    artifact_opens: std::cell::Cell<usize>,
}
pub(super) fn retained_producer() -> Published {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/partial");
    let temp = tempfile::tempdir().unwrap();
    let anchor = temp.path().join("anchor.json");
    fs::copy(base.join("producer-anchor.json"), &anchor).unwrap();
    let mirror = temp.path().join("objects");
    fs::create_dir(&mirror).unwrap();
    fn copy(from: &Path, to: &Path) {
        for e in fs::read_dir(from).unwrap() {
            let e = e.unwrap();
            let target = to.join(e.file_name());
            if e.file_type().unwrap().is_dir() {
                fs::create_dir(&target).unwrap();
                copy(&e.path(), &target);
            } else {
                fs::copy(e.path(), target).unwrap();
            }
        }
    }
    copy(&base.join("producer-mirror"), &mirror);
    let raw = fs::read(&anchor).unwrap();
    let anchors = vec![serde_json::from_slice(&raw).unwrap()];
    Published {
        out: temp.path().join("restored"),
        temp,
        anchor,
        mirror,
        pin: hash(&raw),
        anchors,
        artifact_opens: std::cell::Cell::new(0),
    }
}
impl Published {
    pub fn restore(&self) -> anyhow::Result<Value> {
        use super::source::{AdmittedRef, LocalSource, ObjectReader};
        struct Counted<'a> {
            source: LocalSource,
            count: &'a std::cell::Cell<usize>,
        }
        impl ObjectReader for Counted<'_> {
            fn open(
                &mut self,
                object: &AdmittedRef<'_>,
                path: &str,
                budget: &mut super::budget::Budget,
            ) -> anyhow::Result<Box<dyn std::io::Read>> {
                if path.contains("/artifacts/") {
                    self.count.set(self.count.get() + 1);
                }
                self.source.open(object, path, budget)
            }
        }
        self.artifact_opens.set(0);
        let mut budget = super::budget::Budget::new();
        let (anchor, raw) = super::restore::pinned_anchor(&self.anchor, &self.pin, &mut budget)?;
        let mut source = Counted {
            source: LocalSource::mirror(&self.mirror)?,
            count: &self.artifact_opens,
        };
        let result =
            super::restore::restore(&mut source, anchor, &raw, &self.pin, &self.out, &mut budget)?;
        self.assert_counts(&result, true);
        assert_eq!(
            self.artifact_opens.get() as u64,
            self.expected_counts(true)["artifact_count"]
                .as_u64()
                .unwrap(),
            "positive control: every admitted artifact reaches the counted source exactly once"
        );
        Ok(result)
    }
    pub fn artifact_opens(&self) -> usize {
        self.artifact_opens.get()
    }
    pub fn audit(&self) -> anyhow::Result<Value> {
        let result = super::audit_restored(&self.out, &self.pin)?;
        self.assert_counts(&result, false);
        Ok(result)
    }
    pub fn artifact_path(&self, path: &str) -> PathBuf {
        let a = self.anchors.last().unwrap();
        self.out.join(format!(
            "interrupted-attempts/{}/{}/artifacts/{path}",
            a["run_sha256"].as_str().unwrap(),
            a["attempt_sha256"].as_str().unwrap()
        ))
    }
    pub fn retain(&self, label: &str, expected: &str, outcome: &Value) {
        let Some(root) = std::env::var_os("PARTIAL_FIXTURE_OUTPUT") else {
            return;
        };
        let destination = Path::new(&root).join(label);
        fs::create_dir(&destination).unwrap();
        fn copy(from: &Path, to: &Path, links: &mut BTreeMap<(u64, u64), PathBuf>) {
            use std::os::unix::fs::MetadataExt;
            for entry in fs::read_dir(from).unwrap() {
                let e = entry.unwrap();
                let d = to.join(e.file_name());
                if e.file_type().unwrap().is_dir() {
                    fs::create_dir(&d).unwrap();
                    copy(&e.path(), &d, links)
                } else if e.file_type().unwrap().is_symlink() {
                    std::os::unix::fs::symlink(fs::read_link(e.path()).unwrap(), d).unwrap();
                } else {
                    let metadata = e.metadata().unwrap();
                    let identity = (metadata.dev(), metadata.ino());
                    if let Some(original) = links.get(&identity) {
                        fs::hard_link(original, d).unwrap();
                    } else {
                        fs::copy(e.path(), &d).unwrap();
                        links.insert(identity, d);
                    }
                }
            }
        }
        copy(self.temp.path(), &destination, &mut BTreeMap::new());
        let counts = if outcome.get("schema").and_then(Value::as_str)
            == Some("ml-dataset.independent-partial-audit/v1")
        {
            self.expected_counts(outcome["restored_locally"] == true)
        } else {
            Value::Null
        };
        fs::write(destination.join("expected-and-observed.json"),wire(&json!({"fixture":label,"expected_gate":expected,"anchor_sha256":self.pin,"expected_counts":counts,"observed":outcome,"provenance":"finite synthetic consumer contract fixture; no producer, hardware or remote execution"}))).unwrap();
    }
    fn expected_counts(&self, restored: bool) -> Value {
        let mut metadata = 0u64;
        let mut artifacts = 0u64;
        let mut count = 0u64;
        let mut context = BTreeMap::new();
        let mut phases = 0u64;
        let mut capture = 0u64;
        for anchor in &self.anchors {
            let cat =
                fs::read(self.mirror.join(anchor["catalog"]["key"].as_str().unwrap())).unwrap();
            let receipt =
                fs::read(self.mirror.join(anchor["receipt"]["key"].as_str().unwrap())).unwrap();
            let r: Value = serde_json::from_slice(&receipt).unwrap();
            let manifest =
                fs::read(self.mirror.join(r["manifest"]["key"].as_str().unwrap())).unwrap();
            let m: Value = serde_json::from_slice(&manifest).unwrap();
            metadata += (cat.len() + receipt.len() + manifest.len()) as u64;
            for object in m["objects"].as_array().unwrap() {
                artifacts += object["artifact"]["bytes"].as_u64().unwrap();
                count += 1;
            }
            for a in m["context"].as_array().unwrap() {
                context.insert(
                    a["path"].as_str().unwrap().to_owned(),
                    a["bytes"].as_u64().unwrap(),
                );
            }
            if m["schema"] == "ml-dataset.partial-snapshot/v2" {
                phases += context
                    .iter()
                    .filter(|(p, _)| {
                        p.as_str() == "context/measurement-policy.json"
                            || p.starts_with("context/telemetry/")
                    })
                    .map(|(_, n)| n)
                    .sum::<u64>();
            }
            if let Some(a) = m["delta"]["artifacts"].as_array() {
                capture += a.iter().map(|a| a["bytes"].as_u64().unwrap()).sum::<u64>();
            }
        }
        let authoritative = [
            "run.json",
            "plan.json",
            "protocol.json",
            "config.json",
            "attempt.json",
        ]
        .iter()
        .map(|n| context[&format!("context/{n}")])
        .sum::<u64>();
        let anchor = fs::metadata(&self.anchor).unwrap().len();
        let passes = if restored { 2 } else { 1 };
        json!({"snapshot_count":self.anchors.len(),"artifact_count":count,"artifact_bytes":artifacts,"context_count":context.len(),
            "logical_object_count":passes*(3*self.anchors.len() as u64+count),"logical_object_bytes":passes*(metadata+artifacts),
            "actual_reader_bytes":passes*(anchor+metadata+artifacts+authoritative+phases+capture),
            "written_restore_bytes_before_report":if restored{11+anchor+metadata+artifacts}else{0}})
    }
    pub fn assert_counts(&self, report: &Value, restored: bool) {
        for (key, value) in self.expected_counts(restored).as_object().unwrap() {
            assert_eq!(&report[key], value, "independent expected count {key}");
        }
        assert!(report["modeled_allocation_reserved"].as_u64().unwrap() <= 256 * 1024 * 1024);
        assert!(report["phase_modeled_allocation_used"].as_u64().unwrap() <= 32 * 1024 * 1024);
    }
}
impl Fixture {
    pub fn new(checked: bool) -> Self {
        let run: Value = serde_json::from_slice(&saved("run.json")).unwrap();
        let binding = json!({"schema":"crypto.ml-dataset.attempt-binding/v1","run_sha256":hash(&saved("run.json")),
            "plan_sha256":hash(&saved("plan.json")),"protocol_sha256":hash(&saved("protocol.json")),
            "config_sha256":hash(&saved("config.json")),"source_commit":run["source_commit"],"binary_sha256":run["binary_sha256"],
            "cell_index":0,"attempt_sha256":hash(&saved("attempt.json"))});
        let cursor = genesis(&binding);
        let mut f = Self {
            binding,
            remote: "file:///retained/original/location".into(),
            snapshots: Vec::new(),
            blobs: BTreeMap::new(),
            cursor,
            stop: Value::Null,
        };
        f.push_snapshot(checked);
        for n in [
            "run.json",
            "plan.json",
            "protocol.json",
            "config.json",
            "attempt.json",
        ] {
            f.add_context(0, n, saved(n));
        }
        if checked {
            f.add_context(
                0,
                "measurement-policy.json",
                saved("measurement-policy.json"),
            );
        }
        f
    }
    pub fn push_snapshot(&mut self, checked: bool) -> usize {
        let i = self.snapshots.len();
        self.snapshots.push(json!({"schema":if checked{"ml-dataset.partial-snapshot/v2"}else{"ml-dataset.partial-snapshot/v1"},
            "binding":self.binding,"ordinal":i,"previous":null,"context":[],"delta":null,
            "observation":{"status":"running","observed_unix_seconds":100+i,"exit_code":null,"signal":null,"reason":null},
            "coverage":{"capture_present":false,"committed_events":0,"captured_event_end":0,"capture_stopped":false,"stop":null,
                "artifact_only":true,"in_flight_solver_memory_captured":false,"uncommitted_event_tail_captured":false,
                "unemitted_backend_internals_captured":false,"solver_resume_supported":false},
            "objects":[],"cell_complete":false,"solver_state_restorable":false,"scientific_result_verified":false}));
        if i > 0 {
            self.snapshots[i]["coverage"] = self.snapshots[i - 1]["coverage"].clone();
        }
        i
    }
    pub fn add_context(&mut self, at: usize, name: &str, bytes: Vec<u8>) {
        let path = format!("context/{name}");
        let a = descriptor(&path, role(name), "json", &bytes);
        self.blobs.insert(path, bytes);
        self.snapshots[at]["context"]
            .as_array_mut()
            .unwrap()
            .push(a);
    }
    pub fn add_saved_phase(&mut self, at: usize, name: &str) {
        self.add_context(at, name, saved(name));
    }
    pub fn replace_artifact(&mut self, path: &str, bytes: Vec<u8>) {
        let digest = hash(&bytes);
        let size = bytes.len();
        fn update(v: &mut Value, p: &str, h: &str, n: usize) {
            if v.get("path").and_then(Value::as_str) == Some(p) && v.get("sha256").is_some() {
                v["sha256"] = json!(h);
                v["bytes"] = json!(n);
            }
            match v {
                Value::Object(m) => {
                    for v in m.values_mut() {
                        update(v, p, h, n)
                    }
                }
                Value::Array(a) => {
                    for v in a {
                        update(v, p, h, n)
                    }
                }
                _ => (),
            }
        }
        for s in &mut self.snapshots {
            update(s, path, &digest, size);
        }
        self.blobs.insert(path.into(), bytes);
    }
    /// Update raw context identities after deliberate semantic mutations. This
    /// does not repair canonical commitments inside run/plan/attempt/protocol.
    pub fn refresh_binding_raw(&mut self) {
        for (name, key) in [
            ("run.json", "run_sha256"),
            ("plan.json", "plan_sha256"),
            ("protocol.json", "protocol_sha256"),
            ("config.json", "config_sha256"),
            ("attempt.json", "attempt_sha256"),
        ] {
            self.binding[key] = json!(hash(&self.blobs[&format!("context/{name}")]));
        }
        for s in &mut self.snapshots {
            s["binding"] = self.binding.clone();
            if !s["delta"].is_null() {
                s["delta"]["binding"] = self.binding.clone();
            }
        }
    }
    pub fn capture(
        &mut self,
        at: usize,
        events: Vec<Value>,
        payloads: Vec<(&str, Vec<u8>)>,
        stop: Option<&str>,
    ) {
        let mut artifacts = Vec::new();
        let mut payload_descriptors = Vec::new();
        for (path, bytes) in payloads {
            let a = descriptor(path, "captured-payload", "binary", &bytes);
            self.blobs.insert(path.into(), bytes);
            payload_descriptors.push(a);
        }
        let mut bodies = events;
        for a in &payload_descriptors {
            bodies.push(json!({"kind":"capture_payload_committed","artifact":a}));
        }
        let old = self.cursor.clone();
        let mut raw = Vec::new();
        let mut previous = old["last_event_sha256"].as_str().unwrap().to_owned();
        for (i, event) in bodies.iter().enumerate() {
            let record = json!({"schema":"crypto.ml-dataset.live-event/v2","binding_sha256":binding_hash(&self.binding),
            "sequence":old["event_count"].as_u64().unwrap()+i as u64+1,"previous_event_sha256":previous,"event":event});
            let mut line = serde_json::to_vec(&record).unwrap();
            line.push(b'\n');
            previous = hash(&line);
            raw.extend(line);
        }
        let count = bodies.len() as u64;
        let mut segments = Vec::new();
        if count > 0 {
            let path = format!("events/{:012}.jsonl", old["next_chunk"].as_u64().unwrap());
            let a = descriptor(&path, "committed-solver-event-prefix", "jsonl", &raw);
            segments.push(json!({"artifact":a,"byte_start":old["byte_end"],"byte_end":old["byte_end"].as_u64().unwrap()+raw.len() as u64,
                "first_sequence":old["event_count"].as_u64().unwrap()+1,"last_sequence":old["event_count"].as_u64().unwrap()+count,
                "preceding_event_sha256":old["last_event_sha256"],"last_event_sha256":previous}));
            self.blobs.insert(path, raw.clone());
            artifacts.push(a);
        }
        let payload_bytes: u64 = payload_descriptors
            .iter()
            .map(|a| a["bytes"].as_u64().unwrap())
            .sum();
        self.cursor = json!({"byte_end":old["byte_end"].as_u64().unwrap()+raw.len() as u64,"event_count":old["event_count"].as_u64().unwrap()+count,
            "last_event_sha256":previous,"next_chunk":old["next_chunk"].as_u64().unwrap()+u64::from(count>0),
            "payload_count":old["payload_count"].as_u64().unwrap()+payload_descriptors.len() as u64,
            "payload_bytes":old["payload_bytes"].as_u64().unwrap()+payload_bytes});
        artifacts.extend(payload_descriptors);
        if let Some(stop) = stop {
            self.stop = json!(stop);
        }
        let head = json!({"schema":"crypto.ml-dataset.capture-head/v1","binding_sha256":binding_hash(&self.binding),
            "event_count":self.cursor["event_count"],"byte_end":self.cursor["byte_end"],"last_event_sha256":self.cursor["last_event_sha256"],
            "payload_count":self.cursor["payload_count"],"payload_bytes":self.cursor["payload_bytes"],"stop":self.stop});
        self.snapshots[at]["delta"] = json!({"binding":self.binding,"head":head,"segments":segments,"artifacts":artifacts,"cursor":self.cursor});
        let c = &mut self.snapshots[at]["coverage"];
        c["capture_present"] = json!(true);
        c["committed_events"] = self.cursor["event_count"].clone();
        c["captured_event_end"] = self.cursor["byte_end"].clone();
        c["stop"] = self.stop.clone();
        c["capture_stopped"] = json!(!self.stop.is_null());
    }
    pub fn publish(&self) -> Published {
        self.publish_with(|_, _, _| {})
    }
    pub fn publish_with(&self, mutate: impl FnMut(usize, Stage, &mut Value)) -> Published {
        self.publish_hooks(mutate, |_, _, _| {})
    }
    pub fn publish_raw(&self, mutate: impl FnMut(usize, Stage, &mut Vec<u8>)) -> Published {
        self.publish_hooks(|_, _, _| {}, mutate)
    }
    fn publish_hooks(
        &self,
        mut mutate: impl FnMut(usize, Stage, &mut Value),
        mut raw_mutate: impl FnMut(usize, Stage, &mut Vec<u8>),
    ) -> Published {
        let temp = tempfile::tempdir().unwrap();
        let mirror = temp.path().join("objects");
        fs::create_dir(&mirror).unwrap();
        let put = |r: &Value, raw: &[u8]| {
            let p = mirror.join(r["key"].as_str().unwrap());
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, raw).unwrap();
        };
        let mut previous = Value::Null;
        let mut anchors = Vec::new();
        for (i, template) in self.snapshots.iter().enumerate() {
            let mut m = template.clone();
            m["previous"] = previous.clone();
            let mut members = m["context"].as_array().unwrap().clone();
            if let Some(a) = m["delta"]["artifacts"].as_array() {
                members.extend(a.iter().cloned());
            }
            let mut objects = Vec::new();
            for a in &members {
                let raw = &self.blobs[a["path"].as_str().unwrap()];
                let r = object(raw);
                put(&r, raw);
                objects.push(json!({"artifact":a,"object":r}));
            }
            m["objects"] = json!(objects);
            mutate(i, Stage::Manifest, &mut m);
            let mut bytes = wire(&m);
            raw_mutate(i, Stage::Manifest, &mut bytes);
            let mr = object(&bytes);
            put(&mr, &bytes);
            let binding_sha = binding_hash(&m["binding"]);
            let mut receipt = json!({"schema":"ml-dataset.partial-receipt/v1","remote":self.remote,"binding_sha256":binding_sha,
                "ordinal":i,"previous":previous,"manifest":mr,"artifact_count":members.len(),
                "artifact_bytes":members.iter().map(|a|a["bytes"].as_u64().unwrap()).sum::<u64>(),
                "cell_complete":false,"solver_state_restorable":false,"scientific_result_verified":false});
            mutate(i, Stage::Receipt, &mut receipt);
            let mut bytes = wire(&receipt);
            raw_mutate(i, Stage::Receipt, &mut bytes);
            let rr = object(&bytes);
            put(&rr, &bytes);
            let mut catalog = json!({"schema":"ml-dataset.partial-catalog/v1","remote":self.remote,"binding_sha256":binding_sha,
                "run_sha256":m["binding"]["run_sha256"],"attempt_sha256":m["binding"]["attempt_sha256"],"ordinal":i,"receipt":rr,
                "cell_complete":false,"solver_state_restorable":false,"scientific_result_verified":false});
            mutate(i, Stage::Catalog, &mut catalog);
            let mut bytes = wire(&catalog);
            raw_mutate(i, Stage::Catalog, &mut bytes);
            let mut cr = object(&bytes);
            cr["key"] = json!(format!(
                "ml-dataset/interrupted-attempts/{}/{}/snapshots/{i:020}.json",
                m["binding"]["run_sha256"].as_str().unwrap(),
                m["binding"]["attempt_sha256"].as_str().unwrap()
            ));
            put(&cr, &bytes);
            let mut anchor = json!({"schema":"ml-dataset.partial-anchor/v1","remote":self.remote,"binding_sha256":binding_sha,
                "run_sha256":m["binding"]["run_sha256"],"attempt_sha256":m["binding"]["attempt_sha256"],"ordinal":i,"receipt":rr,"catalog":cr});
            mutate(i, Stage::Anchor, &mut anchor);
            previous = anchor.clone();
            anchors.push(anchor);
        }
        let anchor = temp.path().join("anchor.json");
        let mut raw = wire(&previous);
        raw_mutate(self.snapshots.len() - 1, Stage::Anchor, &mut raw);
        fs::write(&anchor, &raw).unwrap();
        Published {
            out: temp.path().join("restored"),
            temp,
            anchor,
            mirror,
            pin: hash(&raw),
            anchors,
            artifact_opens: std::cell::Cell::new(0),
        }
    }
}
