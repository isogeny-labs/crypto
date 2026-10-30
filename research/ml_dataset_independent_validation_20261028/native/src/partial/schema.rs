//! Independent partial wire inventory. Closed serde envelopes follow
//! https://serde.rs/container-attrs.html; the caller's bounded duplicate-key
//! admission pass must precede any owned Deserialize operation.
use super::budget::{Budget, ARTIFACT_MAX};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::{self, Write};

pub(crate) use crate::phase::Binding;

pub(crate) const BINDING_SCHEMA: &str = "crypto.ml-dataset.attempt-binding/v1";
pub(crate) const ANCHOR_SCHEMA: &str = "ml-dataset.partial-anchor/v1";
pub(crate) const CATALOG_SCHEMA: &str = "ml-dataset.partial-catalog/v1";
pub(crate) const RECEIPT_SCHEMA: &str = "ml-dataset.partial-receipt/v1";
pub(crate) const MANIFEST_V1: &str = "ml-dataset.partial-snapshot/v1";
pub(crate) const MANIFEST_V2: &str = "ml-dataset.partial-snapshot/v2";
pub(crate) const HEAD_SCHEMA: &str = "crypto.ml-dataset.capture-head/v1";
pub(crate) const EVENT_SCHEMA: &str = "crypto.ml-dataset.live-event/v2";

macro_rules! record {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub(crate) struct $name { $(pub $field: $ty,)* }
    };
}
record!(Artifact {
    path: String,
    role: String,
    encoding: String,
    sha256: String,
    bytes: u64
});
record!(ObjectRef {
    key: String,
    sha256: String,
    bytes: u64
});
record!(Anchor {
    schema: String,
    remote: String,
    binding_sha256: String,
    run_sha256: String,
    attempt_sha256: String,
    ordinal: u64,
    receipt: ObjectRef,
    catalog: ObjectRef,
});
record!(Catalog {
    schema: String,
    remote: String,
    binding_sha256: String,
    run_sha256: String,
    attempt_sha256: String,
    ordinal: u64,
    receipt: ObjectRef,
    cell_complete: bool,
    solver_state_restorable: bool,
    scientific_result_verified: bool,
});
record!(Receipt {
    schema: String, remote: String, binding_sha256: String, ordinal: u64,
    previous: Option<Anchor>, manifest: ObjectRef, artifact_count: u64,
    artifact_bytes: u64, cell_complete: bool, solver_state_restorable: bool,
    scientific_result_verified: bool,
});
record!(SnapshotObject {
    artifact: Artifact,
    object: ObjectRef
});
record!(Manifest {
    schema: String, binding: Binding, ordinal: u64, previous: Option<Anchor>,
    context: Vec<Artifact>, delta: Option<Delta>, observation: Observation,
    coverage: Coverage, objects: Vec<SnapshotObject>, cell_complete: bool,
    solver_state_restorable: bool, scientific_result_verified: bool,
});
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Status {
    Running,
    Exited,
    Interrupted,
}
record!(Observation {
    status: Status, observed_unix_seconds: u64, exit_code: Option<i32>,
    signal: Option<i32>, reason: Option<String>,
});
record!(Coverage {
    capture_present: bool, committed_events: u64, captured_event_end: u64,
    capture_stopped: bool, stop: Option<Stop>, artifact_only: bool,
    in_flight_solver_memory_captured: bool, uncommitted_event_tail_captured: bool,
    unemitted_backend_internals_captured: bool, solver_resume_supported: bool,
});
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Stop {
    EventLimit,
    StreamByteLimit,
    PayloadByteLimit,
    PayloadCountLimit,
    LocalByteLimit,
    ChunkLimit,
    StorageFailure,
}
record!(Delta { binding: Binding, head: Head, segments: Vec<Segment>, artifacts: Vec<Artifact>, cursor: Cursor });
record!(Head {
    schema: String, binding_sha256: String, event_count: u64, byte_end: u64,
    last_event_sha256: String, payload_count: u64, payload_bytes: u64, stop: Option<Stop>,
});
record!(Cursor {
    byte_end: u64,
    event_count: u64,
    last_event_sha256: String,
    next_chunk: u64,
    payload_count: u64,
    payload_bytes: u64,
});
record!(Segment {
    artifact: Artifact,
    byte_start: u64,
    byte_end: u64,
    first_sequence: u64,
    last_sequence: u64,
    preceding_event_sha256: String,
    last_event_sha256: String,
});
record!(EventRecord {
    schema: String,
    binding_sha256: String,
    sequence: u64,
    previous_event_sha256: String,
    event: serde_json::Value,
});

pub(crate) fn validate_path(path: &str) -> Result<()> {
    ensure!(
        !path.is_empty()
            && path.len() <= 1024
            && !path.bytes().any(|b| b.is_ascii_control() || b == b'\\')
            && path
                .split('/')
                .all(|p| !p.is_empty() && p != "." && p != ".."),
        "partial invalid relative path"
    );
    Ok(())
}

pub(crate) fn validate_artifact(a: &Artifact) -> Result<()> {
    validate_path(&a.path)?;
    ensure!(
        !a.role.trim().is_empty()
            && a.role.len() <= 256
            && !a.encoding.trim().is_empty()
            && a.encoding.len() <= 64
            && crate::common::valid_hex(&a.sha256, 64)
            && a.bytes <= ARTIFACT_MAX,
        "partial invalid artifact descriptor"
    );
    Ok(())
}

pub(crate) fn validate_binding(b: &Binding) -> Result<()> {
    ensure!(b.schema == BINDING_SCHEMA, "partial binding schema");
    for hash in [
        &b.run_sha256,
        &b.plan_sha256,
        &b.protocol_sha256,
        &b.config_sha256,
        &b.binary_sha256,
        &b.attempt_sha256,
    ] {
        ensure!(
            crate::common::valid_hex(hash, 64),
            "partial binding digest grammar"
        );
    }
    ensure!(
        crate::common::valid_hex(&b.source_commit, 40),
        "partial source commit grammar"
    );
    Ok(())
}

pub(crate) fn validate_ref(r: &ObjectRef, cap: u64, cas: bool) -> Result<()> {
    validate_path(&r.key)?;
    ensure!(
        crate::common::valid_hex(&r.sha256, 64) && r.bytes <= cap,
        "partial object reference digest/size"
    );
    if cas {
        // Compare borrowed components; do not allocate a formatted key.
        let mut parts = r.key.split('/');
        ensure!(
            parts.next() == Some("sha256")
                && parts.next() == Some(&r.sha256[..2])
                && parts.next() == Some(r.sha256.as_str())
                && parts.next().is_none(),
            "partial noncanonical CAS key"
        );
    }
    Ok(())
}

pub(crate) fn validate_remote(remote: &str) -> Result<()> {
    // A file URI is only a lineage label; the local mirror is chosen separately.
    // Producer RemoteSpec::parse requires an absolute remainder. Never follow it.
    if let Some(path) = remote.strip_prefix("file://") {
        ensure!(
            path.starts_with('/') && !path.bytes().any(|b| b.is_ascii_control()),
            "partial file remote must be absolute"
        );
        return Ok(());
    }
    let suffix = remote
        .strip_prefix("s3://setec-astronomy-artifacts")
        .ok_or_else(|| anyhow::anyhow!("partial canonical S3 bucket required"))?;
    ensure!(
        suffix.is_empty() || suffix.starts_with('/'),
        "partial canonical S3 bucket required"
    );
    ensure!(
        !suffix.contains(['?', '#', '%', '\\']) && !suffix.chars().any(char::is_control),
        "partial noncanonical S3 remote"
    );
    if !suffix.is_empty() {
        let prefix = &suffix[1..];
        // Remote is an exact lineage identity, not a location to normalize.
        // Empty terminal components reject both bucket/ and prefix/ aliases.
        ensure!(
            !prefix.is_empty()
                && prefix
                    .split('/')
                    .all(|p| !p.is_empty() && p != "." && p != ".."),
            "partial noncanonical S3 prefix"
        );
    }
    Ok(())
}

pub(crate) fn validate_observation(o: &Observation) -> Result<()> {
    ensure!(
        o.reason.as_ref().is_none_or(|r| r.len() <= 4096)
            && o.signal.is_none_or(|s| (1..=127).contains(&s))
            && (o.status != Status::Running || (o.exit_code.is_none() && o.signal.is_none())),
        "partial invalid observation"
    );
    Ok(())
}

struct HashWriter(Sha256);
impl Write for HashWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.update(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Writes hexadecimal into an already reserved 64-byte String without growth.
pub(super) fn replace_hex(out: &mut String, digest: &[u8]) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    debug_assert_eq!(digest.len(), 32);
    debug_assert!(out.capacity() >= 64);
    out.clear();
    for &b in digest {
        out.push(char::from(HEX[usize::from(b >> 4)]));
        out.push(char::from(HEX[usize::from(b & 15)]));
    }
}

pub(crate) fn binding_digest(binding: &Binding, budget: &mut Budget) -> Result<String> {
    validate_binding(binding)?;
    budget.charge(128, "partial binding digest output and serializer envelope")?;
    let mut result = String::new();
    result.try_reserve_exact(64)?;
    // Consumer Binding's frozen declaration order is the wire hash order.
    // Streaming serialization avoids an uncharged canonical JSON copy.
    let mut writer = HashWriter(Sha256::new());
    serde_json::to_writer(&mut writer, binding)?;
    replace_hex(&mut result, &writer.0.finalize());
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    const BINDING: &[u8] = include_bytes!(
        "../../../../ml_dataset_consumer_partial_20261029/design/vectors/binding-canonical.bin"
    );

    #[test]
    fn frozen_binding_order_and_early_refusal() {
        let b: Binding = serde_json::from_slice(BINDING).unwrap();
        assert_eq!(serde_json::to_vec(&b).unwrap(), BINDING);
        assert_eq!(
            binding_digest(&b, &mut Budget::new()).unwrap(),
            "cf6eb83dd75443e2077e3fac761650963f380209f683b70e565b9e6fb76f8b67"
        );
        assert!(binding_digest(&b, &mut Budget::with_limit(0)).is_err());
    }

    #[test]
    fn path_and_reference_grammar_has_no_normalization() {
        for path in ["", "/a", "a//b", "a/../b", "./a", "a\\b", "a\n"] {
            assert!(validate_path(path).is_err(), "{path:?}");
        }
        assert!(validate_path("events/000000000000.jsonl").is_ok());
        let digest = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let mut r = ObjectRef {
            key: format!("sha256/e3/{digest}"),
            sha256: digest.into(),
            bytes: 0,
        };
        assert!(validate_ref(&r, 0, true).is_ok());
        r.key = format!("sha256/00/{digest}");
        assert!(validate_ref(&r, 0, true).is_err());
    }

    #[test]
    fn remote_identity_is_not_a_fallback_location() {
        for r in [
            "file:///absolute/mirror",
            "s3://setec-astronomy-artifacts",
            "s3://setec-astronomy-artifacts/prefix",
            "s3://setec-astronomy-artifacts/prefix/nested",
        ] {
            assert!(validate_remote(r).is_ok(), "{r}");
        }
        for r in [
            "file://relative",
            "file://",
            "/absolute",
            "s3://setec-astronomy-artifacts/",
            "s3://setec-astronomy-artifacts/prefix/",
            "s3://setec-astronomy-artifacts.evil/x",
            "s3://setec-astronomy-artifacts/a//b",
            "s3://setec-astronomy-artifacts/%2e",
            "s3://setec-astronomy-artifacts/a?x",
        ] {
            assert!(validate_remote(r).is_err(), "{r}");
        }
    }

    #[test]
    fn closed_fields_and_optional_observation_compatibility() {
        let omitted: Observation =
            serde_json::from_str(r#"{"status":"running","observed_unix_seconds":7}"#).unwrap();
        let explicit: Observation = serde_json::from_str(r#"{"status":"running","observed_unix_seconds":7,"exit_code":null,"signal":null,"reason":null}"#).unwrap();
        assert_eq!(omitted, explicit);
        assert!(validate_observation(&omitted).is_ok());
        assert!(serde_json::from_str::<Observation>(
            r#"{"status":"running","observed_unix_seconds":7,"extra":false}"#
        )
        .is_err());
        assert!(serde_json::from_str::<Observation>(
            r#"{"status":"running","observed_unix_seconds":-1}"#
        )
        .is_err());
        let mut terminal = omitted;
        terminal.status = Status::Interrupted;
        assert!(validate_observation(&terminal).is_ok());
        terminal.signal = Some(128);
        assert!(validate_observation(&terminal).is_err());
    }
}
