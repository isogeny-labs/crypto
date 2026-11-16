//! Diagnostic resource custody, separate from mathematical costs and old Timing.
//!
//! Offsets are supplied by the runner from ONE session monotonic clock. This
//! module performs no IO, does not attest external receipts, and never infers a
//! missing observation from a timeout. The caller records actual write+sync
//! completion before supplying a publication tail. Journal publication remains
//! pending until an externally observed runner exit binds its exact bytes.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const MAX_PREFIX_BYTES: usize = 64 * 1024;
pub const MAX_JOURNAL_BYTES: usize = 8 * 1024 * 1024;

fn bounded_json<T: serde::de::DeserializeOwned>(bytes: &[u8], cap: usize) -> Result<T, String> {
    if bytes.len() > cap {
        return Err("resource JSON exceeds byte cap".into());
    }
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    super::canonical::reject_duplicate_keys(text)?;
    serde_json::from_str(text).map_err(|e| e.to_string())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    pub session_id: String,
    pub clock_id: String,
    pub runner_id: String,
    pub scientific_run_id: String,
    pub attempt: u64,
    pub seq: u64,
    pub protocol_sha256: String,
    pub source_manifest_sha256: String,
    pub binary_sha256: String,
}
impl Identity {
    fn validate(&self) -> Result<(), String> {
        if [
            &self.session_id,
            &self.clock_id,
            &self.runner_id,
            &self.scientific_run_id,
        ]
        .iter()
        .any(|s| s.is_empty())
        {
            return Err("empty resource identity".into());
        }
        for h in [
            &self.protocol_sha256,
            &self.source_manifest_sha256,
            &self.binary_sha256,
        ] {
            hash_valid(h)?;
        }
        Ok(())
    }
}
fn hash_valid(h: &str) -> Result<(), String> {
    if h.len() != 64
        || !h
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("expected lowercase SHA256".into());
    }
    Ok(())
}
pub fn exact_sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    ChildStarted,
    ChildReaped,
    RecordDurable,
    DiagnosticsDurable,
    SessionDurable,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mark {
    pub stage: Stage,
    pub offset_ns: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bindings {
    pub context_sha256: Option<String>,
    pub record_sha256: Option<String>,
    pub diagnostics_sha256: Option<String>,
    pub session_sha256: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "reason", rename_all = "snake_case")]
pub enum Status {
    Returned,
    Failed(String),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcePrefix {
    pub schema: String,
    pub identity: Identity,
    /// Relative to the session clock, not the child process clock.
    pub execution_start_ns: u64,
    /// Relative to execution_start_ns, observed after summary preparation.
    pub covered_through_offset_ns: u64,
    pub marks: Vec<Mark>,
    pub bindings: Bindings,
    pub status: Status,
    pub final_publication_tail_pending: bool,
    pub total_active_worker_ns: Option<u64>,
}
pub struct ExecutionEnvelope {
    identity: Identity,
    start: u64,
    marks: Vec<Mark>,
}
impl ExecutionEnvelope {
    /// Call before preparing context, directories or ChildInput.
    pub fn begin(identity: Identity, session_offset_ns: u64) -> Result<Self, String> {
        identity.validate()?;
        Ok(Self {
            identity,
            start: session_offset_ns,
            marks: Vec::new(),
        })
    }
    pub fn mark(&mut self, stage: Stage, offset_ns: u64) -> Result<(), String> {
        let order = [
            Stage::ChildStarted,
            Stage::ChildReaped,
            Stage::RecordDurable,
            Stage::DiagnosticsDurable,
            Stage::SessionDurable,
        ];
        if order.get(self.marks.len()) != Some(&stage) {
            return Err("resource stage repeated, skipped or out of order".into());
        }
        if self.marks.last().is_some_and(|m| offset_ns < m.offset_ns) {
            return Err("resource clock moved backwards".into());
        }
        self.start
            .checked_add(offset_ns)
            .ok_or("resource offset overflow")?;
        self.marks.push(Mark { stage, offset_ns });
        Ok(())
    }
    pub fn seal_prefix(
        self,
        bindings: Bindings,
        status: Status,
        offset_ns: u64,
    ) -> Result<ResourcePrefix, String> {
        if self.marks.last().is_some_and(|m| offset_ns < m.offset_ns) {
            return Err("prefix precedes last resource mark".into());
        }
        self.start
            .checked_add(offset_ns)
            .ok_or("resource offset overflow")?;
        for h in [
            &bindings.context_sha256,
            &bindings.record_sha256,
            &bindings.diagnostics_sha256,
            &bindings.session_sha256,
        ]
        .into_iter()
        .flatten()
        {
            hash_valid(h)?;
        }
        // Do not accept a durable-object hash without its observation, or vice versa.
        for (stage, binding) in [
            (Stage::RecordDurable, &bindings.record_sha256),
            (Stage::DiagnosticsDurable, &bindings.diagnostics_sha256),
            (Stage::SessionDurable, &bindings.session_sha256),
        ] {
            if self.marks.iter().any(|m| m.stage == stage) != binding.is_some() {
                return Err("resource durable mark/binding mismatch".into());
            }
        }
        if !self.marks.is_empty() && bindings.context_sha256.is_none() {
            return Err("started execution lacks context binding".into());
        }
        match &status {
            Status::Returned if self.marks.len() != 5 => {
                return Err("returned resource prefix incomplete".into())
            }
            Status::Failed(reason) if reason.is_empty() => {
                return Err("empty failure reason".into())
            }
            _ => {}
        }
        Ok(ResourcePrefix {
            schema: "ecbench.execution_resources/v1".into(),
            identity: self.identity,
            execution_start_ns: self.start,
            covered_through_offset_ns: offset_ns,
            marks: self.marks,
            bindings,
            status,
            final_publication_tail_pending: true,
            total_active_worker_ns: None,
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationTail {
    pub prefix_sha256: String,
    pub identity: Identity,
    pub execution_start_ns: u64,
    pub prefix_covered_through_ns: u64,
    pub receipt_write_started_ns: u64,
    pub receipt_write_durable_ns: u64,
}
impl PublicationTail {
    /// Exact bytes are the bytes actually published, not reserialized guesses.
    pub fn observed(
        prefix: &ResourcePrefix,
        exact_bytes: &[u8],
        start: u64,
        durable: u64,
    ) -> Result<Self, String> {
        let decoded: ResourcePrefix = bounded_json(exact_bytes, MAX_PREFIX_BYTES)?;
        if &decoded != prefix {
            return Err("published prefix bytes mismatch".into());
        }
        // Revalidate externally constructed/deserialized prefixes as well.
        let mut envelope =
            ExecutionEnvelope::begin(prefix.identity.clone(), prefix.execution_start_ns)?;
        for m in &prefix.marks {
            envelope.mark(m.stage, m.offset_ns)?;
        }
        let expected = envelope.seal_prefix(
            prefix.bindings.clone(),
            prefix.status.clone(),
            prefix.covered_through_offset_ns,
        )?;
        if expected != *prefix {
            return Err("invalid resource prefix claims".into());
        }
        let covered = prefix
            .execution_start_ns
            .checked_add(prefix.covered_through_offset_ns)
            .ok_or("resource offset overflow")?;
        if start < covered || durable < start {
            return Err("publication interval is backwards".into());
        }
        Ok(Self {
            prefix_sha256: exact_sha256(exact_bytes),
            identity: prefix.identity.clone(),
            execution_start_ns: prefix.execution_start_ns,
            prefix_covered_through_ns: covered,
            receipt_write_started_ns: start,
            receipt_write_durable_ns: durable,
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalExit {
    pub session_id: String,
    pub runner_id: String,
    pub clock_id: String,
    pub journal_sha256: String,
    pub journal_durable_ns: u64,
    pub runner_exit_ns: u64,
    /// Observer's payload wall interval, in the same clock; not active worker time.
    pub payload_start_ns: u64,
    pub observation_receipt_sha256: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClosureStatus {
    pub closed_payload_wall_ns: Option<u64>,
    pub final_publication_tail_pending: bool,
    pub total_active_worker_ns: Option<u64>,
    pub external_observation_sha256: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionResourceJournal {
    pub schema: String,
    pub session_id: String,
    pub runner_id: String,
    pub clock_id: String,
    pub payload_start_ns: u64,
    tails: Vec<PublicationTail>,
}
impl SessionResourceJournal {
    pub fn begin(
        session_id: String,
        runner_id: String,
        clock_id: String,
        payload_start_ns: u64,
    ) -> Result<Self, String> {
        if [&session_id, &runner_id, &clock_id]
            .iter()
            .any(|v| v.is_empty())
        {
            return Err("empty journal identity".into());
        }
        Ok(Self {
            schema: "ecbench.resource_publication_journal/v1".into(),
            session_id,
            runner_id,
            clock_id,
            payload_start_ns,
            tails: Vec::new(),
        })
    }
    /// Caller publishes returned journal bytes; this call measures no IO.
    /// Journal writing is covered by the next enclosing interval or external
    /// closure, never claimed complete by the journal's own final write.
    pub fn append_tail(&mut self, tail: PublicationTail) -> Result<(), String> {
        tail.identity.validate()?;
        hash_valid(&tail.prefix_sha256)?;
        if tail.identity.session_id != self.session_id
            || tail.identity.runner_id != self.runner_id
            || tail.identity.clock_id != self.clock_id
        {
            return Err("journal identity mismatch".into());
        }
        let lower = self
            .tails
            .last()
            .map_or(self.payload_start_ns, |t| t.receipt_write_durable_ns);
        if tail.execution_start_ns < lower
            || tail.prefix_covered_through_ns < tail.execution_start_ns
            || tail.receipt_write_started_ns < tail.prefix_covered_through_ns
            || tail.receipt_write_durable_ns < tail.receipt_write_started_ns
        {
            return Err("overlapping or backwards publication interval".into());
        }
        if self
            .tails
            .last()
            .is_some_and(|t| tail.identity.seq <= t.identity.seq)
        {
            return Err("execution seq must increase; new attempts need separate journals".into());
        }
        if self.tails.last().is_some_and(|t| {
            let a = &t.identity;
            let b = &tail.identity;
            a.scientific_run_id != b.scientific_run_id
                || a.attempt != b.attempt
                || a.protocol_sha256 != b.protocol_sha256
                || a.source_manifest_sha256 != b.source_manifest_sha256
                || a.binary_sha256 != b.binary_sha256
        }) {
            return Err("journal scientific identity changed".into());
        }
        self.tails.push(tail);
        Ok(())
    }
    pub fn close_on_external_exit(
        &self,
        exact_journal_bytes: &[u8],
        exit: Option<&ExternalExit>,
    ) -> Result<ClosureStatus, String> {
        let decoded: Self = bounded_json(exact_journal_bytes, MAX_JOURNAL_BYTES)?;
        if decoded != *self {
            return Err("journal bytes mismatch".into());
        }
        let mut validated = Self::begin(
            self.session_id.clone(),
            self.runner_id.clone(),
            self.clock_id.clone(),
            self.payload_start_ns,
        )?;
        for tail in &self.tails {
            validated.append_tail(tail.clone())?;
        }
        if validated != *self {
            return Err("invalid journal schema or intervals".into());
        }
        let Some(e) = exit else {
            return Ok(ClosureStatus {
                closed_payload_wall_ns: None,
                final_publication_tail_pending: true,
                total_active_worker_ns: None,
                external_observation_sha256: None,
            });
        };
        hash_valid(&e.observation_receipt_sha256)?;
        if e.session_id != self.session_id
            || e.runner_id != self.runner_id
            || e.clock_id != self.clock_id
            || e.payload_start_ns != self.payload_start_ns
            || e.journal_sha256 != exact_sha256(exact_journal_bytes)
        {
            return Err("external closure binding mismatch".into());
        }
        let last = self
            .tails
            .last()
            .map_or(self.payload_start_ns, |t| t.receipt_write_durable_ns);
        if e.journal_durable_ns < last || e.runner_exit_ns < e.journal_durable_ns {
            return Err("external closure precedes journal durability".into());
        }
        Ok(ClosureStatus {
            closed_payload_wall_ns: Some(e.runner_exit_ns - self.payload_start_ns),
            final_publication_tail_pending: false,
            total_active_worker_ns: None,
            external_observation_sha256: Some(e.observation_receipt_sha256.clone()),
        })
    }
}

/// A different estimand from ExternalExit: actual run_session origin through
/// trusted owner's post-reap evidence sync. Never exact runner latency.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnclosingEvidenceClosure {
    pub schema: String,
    pub enclosing_session_evidence_wall_ns: u64,
    pub reap_observed_offset_ns: u64,
    pub owner_evidence_sync_offset_ns: u64,
    pub preparation_sha256: String,
    pub journal_sha256: String,
    pub owner_receipt_sha256: String,
    pub exact_runner_exit_ns: Option<u64>,
    pub total_active_worker_ns: Option<u64>,
    pub owner_receipt_publication_tail_pending: bool,
}
/// Independent caller pins for a fresh run. Their external authenticity is a caller obligation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshOwnerPins {
    pub config_sha256: String,
    pub request_sha256: String,
    pub binary_sha256: String,
    pub image_sha256: String,
    pub pod_uid: String,
    pub job_uid: String,
}
enum OwnerEvidenceKind<'a> {
    Recovery,
    Fresh {
        config_bytes: &'a [u8],
        pins: &'a FreshOwnerPins,
    },
}
impl SessionResourceJournal {
    /// Fresh run intake reuses the complete journal, prefix, clock and interval checks.
    /// Replay is an observation-only phase and cannot close a runner session here.
    pub fn close_fresh_enclosing_evidence(
        &self,
        journal_bytes: &[u8],
        preparation_bytes: &[u8],
        owner_bytes: &[u8],
        config_bytes: &[u8],
        pins: &FreshOwnerPins,
    ) -> Result<EnclosingEvidenceClosure, String> {
        self.close_enclosing_owner(
            journal_bytes,
            preparation_bytes,
            owner_bytes,
            &pins.request_sha256,
            &pins.binary_sha256,
            OwnerEvidenceKind::Fresh { config_bytes, pins },
        )
    }
    /// Caller must authenticate owner receipt custody against the expected
    /// request/runtime/Pod independently. This function validates bytes/semantics,
    /// not a signature, deployment or permission to run.
    pub fn close_enclosing_evidence(
        &self,
        journal_bytes: &[u8],
        preparation_bytes: &[u8],
        owner_bytes: &[u8],
        expected_request_sha256: &str,
        expected_binary_sha256: &str,
    ) -> Result<EnclosingEvidenceClosure, String> {
        self.close_enclosing_owner(
            journal_bytes,
            preparation_bytes,
            owner_bytes,
            expected_request_sha256,
            expected_binary_sha256,
            OwnerEvidenceKind::Recovery,
        )
    }
    fn close_enclosing_owner(
        &self,
        journal_bytes: &[u8],
        preparation_bytes: &[u8],
        owner_bytes: &[u8],
        expected_request_sha256: &str,
        expected_binary_sha256: &str,
        kind: OwnerEvidenceKind<'_>,
    ) -> Result<EnclosingEvidenceClosure, String> {
        use serde_json::Value;
        self.close_on_external_exit(journal_bytes, None)?;
        hash_valid(expected_request_sha256)?;
        hash_valid(expected_binary_sha256)?;
        let p: Value = bounded_json(preparation_bytes, 256 * 1024)?;
        let o: Value = bounded_json(owner_bytes, 256 * 1024)?;
        let b = &o["session_binding"];
        let text = |v: &Value, k: &str| {
            v.get(k)
                .and_then(Value::as_str)
                .map(str::to_string)
                .ok_or_else(|| format!("missing {k}"))
        };
        let number = |v: &Value, k: &str| {
            v.get(k)
                .and_then(Value::as_u64)
                .ok_or_else(|| format!("missing integer {k}"))
        };
        let expected_starttime = match kind {
            OwnerEvidenceKind::Recovery => {
                if o["schema"] != "isolab.recovery.owner-exit/v1"
                    || text(&o, "request_sha256")? != expected_request_sha256
                    || number(&o, "attempt")? != 0
                {
                    return Err("recovery owner admission mismatch".into());
                }
                &o["payload_starttime"]
            }
            OwnerEvidenceKind::Fresh { config_bytes, pins } => {
                hash_valid(&pins.config_sha256)?;
                hash_valid(&pins.image_sha256)?;
                if pins.pod_uid.is_empty()
                    || pins.job_uid.is_empty()
                    || pins.pod_uid.len() > 128
                    || pins.job_uid.len() > 128
                {
                    return Err("fresh independent Pod/Job pins missing".into());
                }
                if exact_sha256(config_bytes) != pins.config_sha256 {
                    return Err("fresh config external digest mismatch".into());
                }
                let config: Value = bounded_json(config_bytes, 65536)?;
                if config["schema"] != "isolab.fresh-owner-config/v1"
                    || config["phase"] != "run"
                    || config["request_sha256"] != pins.request_sha256
                    || config["binary_sha256"] != pins.binary_sha256
                    || config["image_digest"] != pins.image_sha256
                    || config["pod_uid"] != pins.pod_uid
                    || config["job_uid"] != pins.job_uid
                    || o["schema"] != "isolab.fresh-owner-observation/v1"
                    || o["binding"] != config
                    || o["config_sha256"] != pins.config_sha256
                    || o["exit_code"] != 0
                    || o.get("exact_kernel_exit_ns") != Some(&Value::Null)
                {
                    return Err("fresh run identity/config admission mismatch; replay has no session closure".into());
                }
                // Fresh producer captures starttime once in its post-exec observation;
                // it has no separate top-level starttime assertion to manufacture.
                &o["postexec_runner_observation"]["starttime"]
            }
        };
        if o["reap_observed"] != true
            || !o["exit_code"].is_i64()
            || o["findings"].as_array().is_none_or(|v| !v.is_empty())
            || o.get("accepted_external_closure") != Some(&Value::Null)
            || o.get("total_active_worker_ns") != Some(&Value::Null)
            || p["schema"] != "ecbench.session_preparation/v1"
            || p["clock_type"] != "linux_CLOCK_MONOTONIC"
            || p.get("clock_id").is_some()
        {
            return Err("owner/preparation admission mismatch".into());
        }
        let ph = exact_sha256(preparation_bytes);
        let jh = exact_sha256(journal_bytes);
        let cid = format!("{}-preparation-sha256-{ph}", self.runner_id);
        for (key, expected) in [
            ("session_id", self.session_id.as_str()),
            ("runner_id", self.runner_id.as_str()),
        ] {
            if text(&p, key)? != expected || text(b, key)? != expected {
                return Err("owner session identity mismatch".into());
            }
        }
        if self.clock_id != cid
            || text(b, "clock_id")? != cid
            || text(b, "preparation_sha256")? != ph
            || text(b, "journal_sha256")? != jh
            || b["estimand"] != "run_session_origin_to_owner_post_reap_evidence_sync"
            || self.payload_start_ns != 0
        {
            return Err("owner exact evidence binding mismatch".into());
        }
        let exec = &o["postexec_runner_observation"];
        if text(exec, "binary_sha256")? != expected_binary_sha256
            || exec["pid"] != o["payload_pid"]
            || &exec["starttime"] != expected_starttime
            || !exec["pid"].is_i64()
            || text(exec, "starttime")?.is_empty()
        {
            return Err("post-exec identity mismatch".into());
        }
        for tail in &self.tails {
            if tail.identity.binary_sha256 != expected_binary_sha256 || tail.identity.attempt != 0 {
                return Err("journal binary/attempt mismatch".into());
            }
        }
        let clocks = [
            &o["owner_begin_clock"],
            &exec["clock"],
            &o["reap_observed_clock"],
            &b["journal_sync_clock"],
        ];
        let mut previous = 0;
        for clock in clocks {
            for field in ["boot_id", "time_namespace", "time_namespace_offsets"] {
                if text(clock, field)? != text(&p, field)? {
                    return Err("owner clock namespace mismatch".into());
                }
            }
            let raw = number(clock, "raw_ns")?;
            if raw < previous {
                return Err("owner clocks backwards".into());
            }
            previous = raw;
        }
        let origin = number(&p, "origin_ns")?;
        if origin < number(&o["owner_begin_clock"], "raw_ns")? {
            return Err("preparation origin precedes owner epoch".into());
        }
        let reaped = number(&o["reap_observed_clock"], "raw_ns")?
            .checked_sub(origin)
            .ok_or("origin after reap")?;
        let synced = number(&b["journal_sync_clock"], "raw_ns")?
            .checked_sub(origin)
            .ok_or("origin after sync")?;
        if number(b, "reap_observed_offset_ns")? != reaped
            || number(b, "enclosing_evidence_wall_ns")? != synced
            || self
                .tails
                .last()
                .is_some_and(|tail| tail.receipt_write_durable_ns > reaped)
        {
            return Err("owner/journal interval mismatch".into());
        }
        Ok(EnclosingEvidenceClosure {
            schema: "ecbench.enclosing_session_evidence_closure/v1".into(),
            enclosing_session_evidence_wall_ns: synced,
            reap_observed_offset_ns: reaped,
            owner_evidence_sync_offset_ns: synced,
            preparation_sha256: ph,
            journal_sha256: jh,
            owner_receipt_sha256: exact_sha256(owner_bytes),
            exact_runner_exit_ns: None,
            total_active_worker_ns: None,
            owner_receipt_publication_tail_pending: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resource_json_rejects_duplicate_keys_and_oversize_before_decoding() {
        let journal =
            SessionResourceJournal::begin("s".into(), "r".into(), "fake-monotonic".into(), 0)
                .unwrap();
        let bytes = serde_json::to_string(&journal).unwrap();
        let duplicate = bytes.replacen('{', "{\"session_id\":\"s\",", 1);
        assert!(journal
            .close_on_external_exit(duplicate.as_bytes(), None)
            .is_err());
        let oversized = vec![b' '; MAX_JOURNAL_BYTES + 1];
        assert_eq!(
            journal
                .close_on_external_exit(&oversized, None)
                .unwrap_err(),
            "resource JSON exceeds byte cap"
        );
        let p = prefix(0, 0, 5);
        let bytes = serde_json::to_string(&p).unwrap();
        let duplicate = bytes.replacen('{', "{\"schema\":\"ecbench.execution_resources/v1\",", 1);
        assert!(PublicationTail::observed(&p, duplicate.as_bytes(), 61, 70).is_err());
        assert!(PublicationTail::observed(&p, &vec![b' '; MAX_PREFIX_BYTES + 1], 61, 70).is_err());
    }
    fn hash() -> String {
        "a".repeat(64)
    }
    fn identity(seq: u64) -> Identity {
        Identity {
            session_id: "s".into(),
            runner_id: "r".into(),
            clock_id: "fake-monotonic".into(),
            scientific_run_id: "fixture".into(),
            attempt: 0,
            seq,
            protocol_sha256: hash(),
            source_manifest_sha256: hash(),
            binary_sha256: hash(),
        }
    }
    fn bindings(n: usize) -> Bindings {
        Bindings {
            context_sha256: (n > 0).then(hash),
            record_sha256: (n >= 3).then(hash),
            diagnostics_sha256: (n >= 4).then(hash),
            session_sha256: (n >= 5).then(hash),
        }
    }
    fn prefix(seq: u64, start: u64, n: usize) -> ResourcePrefix {
        let mut e = ExecutionEnvelope::begin(identity(seq), start).unwrap();
        for (i, stage) in [
            Stage::ChildStarted,
            Stage::ChildReaped,
            Stage::RecordDurable,
            Stage::DiagnosticsDurable,
            Stage::SessionDurable,
        ]
        .into_iter()
        .take(n)
        .enumerate()
        {
            e.mark(stage, (i as u64 + 1) * 10).unwrap();
        }
        e.seal_prefix(
            bindings(n),
            if n == 5 {
                Status::Returned
            } else {
                Status::Failed("injected".into())
            },
            60,
        )
        .unwrap()
    }
    fn tail(p: &ResourcePrefix) -> PublicationTail {
        PublicationTail::observed(
            p,
            &serde_json::to_vec(p).unwrap(),
            p.execution_start_ns + 61,
            p.execution_start_ns + 70,
        )
        .unwrap()
    }
    #[test]
    fn fake_clock_failure_prefixes_never_fill_missing_marks_with_zero() {
        // Setup, spawn, record, ingest and session publication failure prefixes.
        for reached in 0..=5 {
            let p = prefix(0, 100, reached);
            assert_eq!(p.marks.len(), reached);
            assert_eq!(p.total_active_worker_ns, None);
            assert!(p.final_publication_tail_pending);
            assert_eq!(p.bindings.record_sha256.is_some(), reached >= 3);
            assert_eq!(tail(&p).receipt_write_durable_ns, 170);
        }
    }
    #[test]
    fn stage_and_binding_errors_preserve_envelope() {
        let mut e = ExecutionEnvelope::begin(identity(0), 100).unwrap();
        assert!(e.mark(Stage::ChildReaped, 10).is_err());
        e.mark(Stage::ChildStarted, 10).unwrap();
        assert!(e.mark(Stage::ChildReaped, 9).is_err());
        assert!(e.mark(Stage::ChildStarted, 11).is_err());
        assert_eq!(e.marks.len(), 1);
        assert!(e
            .seal_prefix(bindings(0), Status::Failed("spawn".into()), 11)
            .is_err());
        assert!(ExecutionEnvelope::begin(identity(0), u64::MAX)
            .unwrap()
            .seal_prefix(bindings(0), Status::Failed("setup".into()), 1)
            .is_err());
    }
    #[test]
    fn publication_failure_leaves_total_unknown_and_exact_bytes_are_bound() {
        let p = prefix(0, 0, 5);
        let bytes = serde_json::to_vec(&p).unwrap();
        assert!(PublicationTail::observed(&p, &bytes, 61, 60).is_err());
        let mut changed = p.clone();
        changed.identity.seq = 4;
        assert!(PublicationTail::observed(&changed, &bytes, 61, 70).is_err());
        let mut journal =
            SessionResourceJournal::begin("s".into(), "r".into(), "fake-monotonic".into(), 0)
                .unwrap();
        journal.append_tail(tail(&p)).unwrap();
        let pending = journal
            .close_on_external_exit(&serde_json::to_vec(&journal).unwrap(), None)
            .unwrap();
        assert!(pending.final_publication_tail_pending);
        assert_eq!(pending.total_active_worker_ns, None);
    }
    #[test]
    fn external_closure_covers_tail_once_without_summing_nested_intervals() {
        let mut journal =
            SessionResourceJournal::begin("s".into(), "r".into(), "fake-monotonic".into(), 0)
                .unwrap();
        journal.append_tail(tail(&prefix(0, 10, 5))).unwrap();
        assert!(journal.append_tail(tail(&prefix(1, 79, 5))).is_err());
        journal.append_tail(tail(&prefix(1, 100, 2))).unwrap();
        let bytes = serde_json::to_vec(&journal).unwrap();
        let mut exit = ExternalExit {
            session_id: "s".into(),
            runner_id: "r".into(),
            clock_id: "fake-monotonic".into(),
            journal_sha256: exact_sha256(&bytes),
            journal_durable_ns: 180,
            runner_exit_ns: 200,
            payload_start_ns: 0,
            observation_receipt_sha256: hash(),
        };
        let result = journal.close_on_external_exit(&bytes, Some(&exit)).unwrap();
        assert_eq!(result.closed_payload_wall_ns, Some(200));
        assert_eq!(result.total_active_worker_ns, None);
        exit.journal_durable_ns = 169;
        assert!(journal.close_on_external_exit(&bytes, Some(&exit)).is_err());
        exit.journal_durable_ns = 180;
        exit.clock_id = "other-clock".into();
        assert!(journal.close_on_external_exit(&bytes, Some(&exit)).is_err());
    }
    #[test]
    fn enclosing_wall_is_distinct_and_stale_origin_is_rejected_after_rebinding() {
        fn fixture(origin: u64) -> (SessionResourceJournal, Vec<u8>, Vec<u8>, Vec<u8>) {
            use serde_json::json;
            let p = json!({"schema":"ecbench.session_preparation/v1","session_id":"s","runner_id":"r","clock_type":"linux_CLOCK_MONOTONIC","origin_ns":origin,"boot_id":"b","time_namespace":"n","time_namespace_offsets":"o"});
            let pb = serde_json::to_vec(&p).unwrap();
            let ph = exact_sha256(&pb);
            let cid = format!("r-preparation-sha256-{ph}");
            let journal =
                SessionResourceJournal::begin("s".into(), "r".into(), cid.clone(), 0).unwrap();
            let jb = serde_json::to_vec(&journal).unwrap();
            let clock = |raw| json!({"raw_ns":raw,"boot_id":"b","time_namespace":"n","time_namespace_offsets":"o"});
            let o = json!({"schema":"isolab.recovery.owner-exit/v1","request_sha256":"a".repeat(64),"attempt":0,"reap_observed":true,"exit_code":0,"findings":[],"accepted_external_closure":null,"total_active_worker_ns":null,"payload_pid":12,"payload_starttime":"17",
                "owner_begin_clock":clock(10),"postexec_runner_observation":{"pid":12,"starttime":"17","binary_sha256":"b".repeat(64),"clock":clock(25)},"reap_observed_clock":clock(30),
                "session_binding":{"session_id":"s","runner_id":"r","clock_id":cid,"preparation_sha256":ph,"journal_sha256":exact_sha256(&jb),"estimand":"run_session_origin_to_owner_post_reap_evidence_sync","reap_observed_offset_ns":30-origin,"enclosing_evidence_wall_ns":40-origin,"journal_sync_clock":clock(40)}});
            (journal, jb, pb, serde_json::to_vec(&o).unwrap())
        }
        let (j, jb, pb, ob) = fixture(20);
        let result = j
            .close_enclosing_evidence(&jb, &pb, &ob, &"a".repeat(64), &"b".repeat(64))
            .unwrap();
        assert_eq!(result.enclosing_session_evidence_wall_ns, 20);
        assert_eq!(result.reap_observed_offset_ns, 10);
        assert!(result.exact_runner_exit_ns.is_none());
        assert!(result.total_active_worker_ns.is_none());
        assert!(result.owner_receipt_publication_tail_pending);
        let (j, jb, pb, ob) = fixture(5);
        assert!(j
            .close_enclosing_evidence(&jb, &pb, &ob, &"a".repeat(64), &"b".repeat(64))
            .unwrap_err()
            .contains("origin precedes"));
    }
    #[test]
    fn fresh_closure_reuses_full_journal_semantics_and_external_pins() {
        use serde_json::{json, Value};
        let prep = json!({"schema":"ecbench.session_preparation/v1","session_id":"s","runner_id":"r","clock_type":"linux_CLOCK_MONOTONIC","origin_ns":20,"boot_id":"b","time_namespace":"n","time_namespace_offsets":"o"});
        let pb = serde_json::to_vec(&prep).unwrap();
        let ph = exact_sha256(&pb);
        let cid = format!("r-preparation-sha256-{ph}");
        let mut journal =
            SessionResourceJournal::begin("s".into(), "r".into(), cid.clone(), 0).unwrap();
        let mut prefix = prefix(0, 0, 5);
        prefix.identity.clock_id = cid.clone();
        prefix.identity.binary_sha256 = "b".repeat(64);
        journal.append_tail(tail(&prefix)).unwrap();
        let jb = serde_json::to_vec(&journal).unwrap();
        let config = json!({"schema":"isolab.fresh-owner-config/v1","phase":"run","request_sha256":"a".repeat(64),"binary_sha256":"b".repeat(64),"image_digest":"c".repeat(64),"pod_uid":"pod","job_uid":"job"});
        let cb = serde_json::to_vec(&config).unwrap();
        let pins = FreshOwnerPins {
            config_sha256: exact_sha256(&cb),
            request_sha256: "a".repeat(64),
            binary_sha256: "b".repeat(64),
            image_sha256: "c".repeat(64),
            pod_uid: "pod".into(),
            job_uid: "job".into(),
        };
        let clock = |raw| json!({"raw_ns":raw,"boot_id":"b","time_namespace":"n","time_namespace_offsets":"o"});
        let owner = json!({"schema":"isolab.fresh-owner-observation/v1","config_sha256":pins.config_sha256,"binding":config,"reap_observed":true,"exit_code":0,"findings":[],"accepted_external_closure":null,"total_active_worker_ns":null,"exact_kernel_exit_ns":null,"payload_pid":12,
            "owner_begin_clock":clock(10),"postexec_runner_observation":{"pid":12,"starttime":"17","binary_sha256":"b".repeat(64),"clock":clock(25)},"reap_observed_clock":clock(200),
            "session_binding":{"session_id":"s","runner_id":"r","clock_id":cid,"preparation_sha256":ph,"journal_sha256":exact_sha256(&jb),"estimand":"run_session_origin_to_owner_post_reap_evidence_sync","reap_observed_offset_ns":180,"enclosing_evidence_wall_ns":190,"journal_sync_clock":clock(210)}});
        let ob = serde_json::to_vec(&owner).unwrap();
        let close =
            |j: &SessionResourceJournal, jb: &[u8], o: &[u8], c: &[u8], pins: &FreshOwnerPins| {
                j.close_fresh_enclosing_evidence(jb, &pb, o, c, pins)
            };
        let closed = close(&journal, &jb, &ob, &cb, &pins).unwrap();
        assert_eq!(closed.enclosing_session_evidence_wall_ns, 190);
        assert_eq!(closed.owner_receipt_sha256, exact_sha256(&ob));
        assert!(closed.total_active_worker_ns.is_none());
        assert!(closed.exact_runner_exit_ns.is_none());
        for field in 0..6 {
            let mut wrong = pins.clone();
            match field {
                0 => wrong.config_sha256 = "d".repeat(64),
                1 => wrong.request_sha256 = "d".repeat(64),
                2 => wrong.binary_sha256 = "d".repeat(64),
                3 => wrong.image_sha256 = "d".repeat(64),
                4 => wrong.pod_uid = "other".into(),
                _ => wrong.job_uid = "other".into(),
            };
            assert!(close(&journal, &jb, &ob, &cb, &wrong).is_err());
        }
        // Rehash a semantically invalid journal consistently: hash checks alone must not pass.
        let mut invalid = journal.clone();
        invalid.tails[0].receipt_write_durable_ns = invalid.tails[0].receipt_write_started_ns - 1;
        let bad_jb = serde_json::to_vec(&invalid).unwrap();
        let mut bad_owner = owner.clone();
        bad_owner["session_binding"]["journal_sha256"] = json!(exact_sha256(&bad_jb));
        assert!(close(
            &invalid,
            &bad_jb,
            &serde_json::to_vec(&bad_owner).unwrap(),
            &cb,
            &pins
        )
        .is_err());
        let mut replay_config = config.clone();
        replay_config["phase"] = json!("replay");
        let replay_cb = serde_json::to_vec(&replay_config).unwrap();
        let mut replay_pins = pins.clone();
        replay_pins.config_sha256 = exact_sha256(&replay_cb);
        let mut replay_owner = owner.clone();
        replay_owner["binding"] = replay_config;
        replay_owner["config_sha256"] = json!(replay_pins.config_sha256);
        replay_owner["session_binding"] = Value::Null;
        assert!(close(
            &journal,
            &jb,
            &serde_json::to_vec(&replay_owner).unwrap(),
            &replay_cb,
            &replay_pins
        )
        .unwrap_err()
        .contains("replay"));
        for mutation in ["clock", "pid", "starttime", "missing"] {
            let mut changed = owner.clone();
            match mutation {
                "clock" => changed["reap_observed_clock"]["boot_id"] = json!("different"),
                "pid" => changed["payload_pid"] = json!(13),
                "starttime" => changed["postexec_runner_observation"]["starttime"] = json!(""),
                _ => changed["session_binding"] = Value::Null,
            };
            assert!(close(
                &journal,
                &jb,
                &serde_json::to_vec(&changed).unwrap(),
                &cb,
                &pins
            )
            .is_err());
        }
    }
}
