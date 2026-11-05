use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub const ACCOUNT: &str = "104186811093";
pub const REGION: &str = "us-east-2";
pub const CLUSTER: &str = "isolab";
pub const NAMESPACE: &str = "isolab-recovery-system";
pub const BUCKET: &str = "isolab-results-104186811093-us-east-2";
pub const REPOSITORY: &str = "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Build {
    pub image_uri: String,
    pub source_revision: String,
    pub ecbench_sha256: String,
}

/// Explicit bounded memory envelope shared by computation and fresh L2 pods.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Memory {
    #[default]
    #[serde(rename = "4Gi")]
    Gi4,
    #[serde(rename = "8Gi")]
    Gi8,
}
impl Memory {
    fn is_default(&self) -> bool {
        *self == Self::Gi4
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gi4 => "4Gi",
            Self::Gi8 => "8Gi",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub schema: String,
    pub run_name: String,
    pub deadline_unix_seconds: u64,
    pub build: Build,
    pub runtime_image_uri: String,
    /// Original ecbench spec. Absent or null only in command mode; an ecbench
    /// spec is always an object, so legacy request bytes and hashes are unchanged.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub spec: Value,
    pub instance_type: String,
    pub checkpoint_interval_seconds: u64,
    pub max_restores: u32,
    #[serde(default, skip_serializing_if = "Memory::is_default")]
    pub memory: Memory,
    /// Optional opaque launches, independently frozen for computation-L0/fresh-L2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_transport: Option<DiagnosticTransport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_campaign: Option<crate::campaign::Binding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fresh_owner: Option<FreshOwnerAdmission>,
    /// Trusted native command instead of ecbench. Absent for every ecbench request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<CommandMode>,
}

pub const MAX_COMMAND_ARGS: usize = 64;
pub const MAX_COMMAND_ARG_BYTES: usize = 4096;
pub const MAX_COMMAND_BINARIES: usize = 32;
pub const MAX_COMMAND_RESTORES: u32 = 50;
pub const MAX_ECBENCH_RESTORES: u32 = 5;
pub const COMPUTE_DIR: &str = "/work/compute";

fn one() -> u32 {
    1
}
pub const DEFAULT_PVC_GIB: u32 = 8;
pub const MAX_PVC_GIB: u32 = 64;
fn default_pvc_gib() -> u32 {
    DEFAULT_PVC_GIB
}
fn is_default_pvc_gib(n: &u32) -> bool {
    *n == DEFAULT_PVC_GIB
}
fn is_one(n: &u32) -> bool {
    *n == 1
}

/// Command mode: the supervisor execs `argv` (UID 1000, no capabilities,
/// computation-only perf filter) instead of `isolab-criu-progress` + ecbench.
/// `build.ecbench_sha256` binds the SHA-256 of the image file `argv[0]`; each
/// `binaries` entry binds one more absolute image path (for example the native
/// program a launcher script execs). Both are verified before launch and restore.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandMode {
    pub argv: Vec<String>,
    /// Directory under /work/compute uploaded after the command exits.
    pub outputs: String,
    /// RAYON_NUM_THREADS for the payload, 1..4. Explicit 1 canonicalizes to omission.
    #[serde(default = "one", skip_serializing_if = "is_one")]
    pub threads: u32,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub binaries: BTreeMap<String, String>,
    /// Computation PVC size in GiB, 8..64. Explicit 8 canonicalizes to omission.
    #[serde(
        default = "default_pvc_gib",
        skip_serializing_if = "is_default_pvc_gib"
    )]
    pub pvc_gib: u32,
}

fn plain_text(s: &str) -> bool {
    s.len() <= MAX_COMMAND_ARG_BYTES && !s.bytes().any(|c| c == 0 || c == b'\n' || c == b'\r')
}
/// Absolute, normalized (no `.`/`..`/empty components, no trailing slash) path.
fn normalized_absolute(s: &str) -> bool {
    plain_text(s)
        && s.len() > 1
        && s.starts_with('/')
        && !s.ends_with('/')
        && s[1..]
            .split('/')
            .all(|c| !c.is_empty() && c != "." && c != "..")
}
/// Image paths only: never the payload-writable PVC, the request mount or
/// kernel/device/temporary filesystems.
fn image_path(s: &str) -> bool {
    normalized_absolute(s)
        && !["/work", "/request", "/proc", "/sys", "/dev", "/tmp"]
            .iter()
            .any(|root| s == *root || s.starts_with(&format!("{root}/")))
}

impl CommandMode {
    pub fn validate(&self, argv0_sha256: &str) -> Result<(), String> {
        if self.argv.is_empty() || self.argv.len() > MAX_COMMAND_ARGS {
            return Err("command argv must have 1..64 entries".into());
        }
        if !self.argv.iter().all(|a| plain_text(a)) {
            return Err(
                "command arguments must be at most 4096 bytes without NUL or newline".into(),
            );
        }
        if !image_path(&self.argv[0]) {
            return Err(
                "command argv[0] must be a normalized absolute path inside the image".into(),
            );
        }
        if !hex(argv0_sha256, 64) {
            return Err("command mode binds argv[0] by build.ecbench_sha256".into());
        }
        let relative = self
            .outputs
            .strip_prefix(COMPUTE_DIR)
            .and_then(|r| r.strip_prefix('/'))
            .filter(|_| normalized_absolute(&self.outputs))
            .ok_or("command outputs must be a normalized directory under /work/compute")?;
        let first = relative.split('/').next().unwrap_or("");
        if ["tmp", "run.stdout", "run.stderr"].contains(&first) {
            return Err("command outputs may not be tmp, run.stdout or run.stderr".into());
        }
        if !(1..=4).contains(&self.threads) {
            return Err("command threads must be 1..4".into());
        }
        if !(DEFAULT_PVC_GIB..=MAX_PVC_GIB).contains(&self.pvc_gib) {
            return Err("command pvc_gib must be 8..64".into());
        }
        if self.binaries.len() > MAX_COMMAND_BINARIES
            || !self
                .binaries
                .iter()
                .all(|(path, sha)| image_path(path) && hex(sha, 64) && *path != self.argv[0])
        {
            return Err("command binaries must be at most 32 image paths other than argv[0] with SHA-256 digests".into());
        }
        Ok(())
    }
    /// Path of the outputs directory relative to /work/compute.
    pub fn outputs_relative(&self) -> &str {
        self.outputs
            .strip_prefix("/work/compute/")
            .unwrap_or(&self.outputs)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshOwnerAdmission {
    pub schema: String,
    pub binary_sha256: String,
    /// Immutable process stop, never later than the original request deadline.
    pub stop_utc_seconds: u64,
}

pub const MAX_DIAGNOSTIC_LAUNCH_BYTES: usize = 65_536;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticLaunchBytes {
    pub raw_json: String,
    pub sha256: String,
    /// Exact emitted spec-file bytes, NOT ecbench's identity-view digest.
    pub spec_bytes_sha256: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticTransport {
    pub schema: String,
    pub computation: DiagnosticLaunchBytes,
    pub fresh: DiagnosticLaunchBytes,
}
impl DiagnosticTransport {
    pub fn validate(&self, original: &Value) -> Result<(), String> {
        if self.schema != "isolab.diagnostic-transport/v1" {
            return Err("unsupported diagnostic transport".into());
        }
        for launch in [&self.computation, &self.fresh] {
            if launch.raw_json.is_empty()
                || launch.raw_json.len() > MAX_DIAGNOSTIC_LAUNCH_BYTES
                || !hex(&launch.sha256, 64)
                || digest(launch.raw_json.as_bytes()) != launch.sha256
                || !hex(&launch.spec_bytes_sha256, 64)
            {
                return Err("diagnostic raw launch byte/hash bound invalid".into());
            }
        }
        // Verification against independently supplied frozen values, never mutation
        // of either opaque launch or its internal expected_spec_sha256 binding.
        if !original.is_object() || !original["measurement"].is_object() {
            return Err("diagnostic transport requires a spec measurement object".into());
        }
        let mut computation = original.clone();
        computation["measurement"]["isolation_required"] = serde_json::json!("L0");
        computation["label"] =
            serde_json::json!("CRIU computation only: all timing invalid for benchmark admission");
        let computation = serde_json::to_vec_pretty(&computation).map_err(|e| e.to_string())?;
        let mut fresh = serde_json::to_vec(original).map_err(|e| e.to_string())?;
        fresh.push(b'\n'); // Existing fresh wrapper's printf '%s\n'.
        if digest(&computation) != self.computation.spec_bytes_sha256
            || digest(&fresh) != self.fresh.spec_bytes_sha256
        {
            return Err("separately frozen computation/fresh spec-byte binding mismatch".into());
        }
        Ok(())
    }
}

pub fn hex(s: &str, n: usize) -> bool {
    s.len() == n
        && s.bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

impl Request {
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > 65_536 {
            return Err("request exceeds 64 KiB".into());
        }
        let req: Self = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        req.validate()?;
        Ok(req)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != "isolab.eks.resumable/v1" {
            return Err("unsupported recovery schema".into());
        }
        let name = self.run_name.as_bytes();
        if name.is_empty()
            || name.len() > 32
            || !name[0].is_ascii_lowercase()
            || !name.last().unwrap().is_ascii_alphanumeric()
            || !name
                .iter()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-')
        {
            return Err("run_name must be a DNS label of at most 32 characters".into());
        }
        let prefix = format!("{REPOSITORY}@sha256:");
        if !self
            .runtime_image_uri
            .strip_prefix(&prefix)
            .is_some_and(|s| hex(s, 64))
        {
            return Err("recovery runtime must be digest-pinned in the approved repository".into());
        }
        if !self
            .build
            .image_uri
            .strip_prefix(&prefix)
            .is_some_and(|s| hex(s, 64))
            || !hex(&self.build.source_revision, 40)
            || !hex(&self.build.ecbench_sha256, 64)
        {
            return Err(
                "exact source, binary and approved digest-pinned image are required".into(),
            );
        }
        if self.instance_type != "c8a.2xlarge" {
            return Err("recovery currently qualifies only c8a.2xlarge; ordinary fresh v1 jobs retain broader placement".into());
        }
        if let Some(command) = &self.command {
            return self.validate_command(command);
        }
        if !(30..=900).contains(&self.checkpoint_interval_seconds)
            || !(1..=MAX_ECBENCH_RESTORES).contains(&self.max_restores)
        {
            return Err("checkpoint interval must be 30..900 seconds and max_restores 1..5".into());
        }
        let m = &self.spec["measurement"];
        let arms = self.spec["arms"]
            .as_array()
            .ok_or("spec arms must be an array")?;
        if self.spec["schema"] != "ecbench.spec/v1"
            || m["isolation_required"] != "L2"
            || m["rounds"].as_u64().unwrap_or(0) < 5
            || m["warmup"].as_u64().unwrap_or(0) < 1
            || !arms.iter().any(|a| a["role"] == "reference")
            || !arms.iter().any(|a| {
                a["role"] == "control"
                    && arms.iter().any(|b| {
                        b["role"] != "control"
                            && a["name"] != b["name"]
                            && a["method"] == b["method"]
                    })
            })
        {
            return Err("the original benchmark spec must require L2, warmup, five rounds, reference and A/A control".into());
        }
        if let Some(binding) = &self.worker_campaign {
            let spec_hash = digest(&serde_json::to_vec(&self.spec).map_err(|e| e.to_string())?);
            binding.validate(&spec_hash, self.deadline_unix_seconds)?;
        }
        if let Some(transport) = &self.diagnostic_transport {
            transport.validate(&self.spec)?;
        }
        if let Some(owner) = &self.fresh_owner {
            if owner.schema != "isolab.fresh-owner-admission/v1"
                || !hex(&owner.binary_sha256, 64)
                || self.diagnostic_transport.is_none()
                || owner.stop_utc_seconds == 0
                || owner.stop_utc_seconds > self.deadline_unix_seconds
            {
                return Err("invalid explicit fresh owner admission".into());
            }
        }
        Ok(())
    }

    // Command mode never enters the ecbench spec, campaign, diagnostic or fresh
    // owner paths: each is bound to an ecbench spec and a fresh L2 epoch.
    fn validate_command(&self, command: &CommandMode) -> Result<(), String> {
        if !(30..=900).contains(&self.checkpoint_interval_seconds)
            || !(1..=MAX_COMMAND_RESTORES).contains(&self.max_restores)
        {
            return Err(
                "checkpoint interval must be 30..900 seconds and command max_restores 1..50".into(),
            );
        }
        if !self.spec.is_null() {
            return Err("command mode requires spec to be absent or null".into());
        }
        if self.diagnostic_transport.is_some()
            || self.worker_campaign.is_some()
            || self.fresh_owner.is_some()
        {
            return Err(
                "command mode admits no diagnostic transport, worker campaign or fresh owner"
                    .into(),
            );
        }
        command.validate(&self.build.ecbench_sha256)
    }
    pub fn is_command(&self) -> bool {
        self.command.is_some()
    }
    /// Computation PVC size: always 8 GiB for ecbench; command.pvc_gib otherwise.
    pub fn pvc_gib(&self) -> u32 {
        self.command.as_ref().map_or(DEFAULT_PVC_GIB, |c| c.pvc_gib)
    }

    pub fn hash(&self) -> String {
        digest(&serde_json::to_vec(self).expect("serializable request"))
    }

    pub fn validate_new_deadline(&self, now: u64) -> Result<(), String> {
        // Budget is bound once; reconciliation never resets it after failover.
        let remaining = self
            .deadline_unix_seconds
            .checked_sub(now)
            .ok_or("original deadline expired")?;
        if !(180..=86400).contains(&remaining) {
            return Err(
                "a new run needs 180..86400 seconds before its original UTC deadline".into(),
            );
        }
        Ok(())
    }
}

#[cfg(test)]
pub fn fixture() -> Request {
    Request {
        schema: "isolab.eks.resumable/v1".into(),
        run_name: "test-recovery".into(),
        deadline_unix_seconds: 5000,
        build: Build {
            image_uri: format!("{REPOSITORY}@sha256:{}", "a".repeat(64)),
            source_revision: "b".repeat(40),
            ecbench_sha256: "c".repeat(64),
        },
        runtime_image_uri: format!("{REPOSITORY}@sha256:{}", "d".repeat(64)),
        spec: serde_json::json!({"schema":"ecbench.spec/v1","measurement":{"isolation_required":"L2","rounds":5,"warmup":1},"arms":[{"name":"ref","role":"reference","method":{"id":"rho.negation"}},{"name":"aa","role":"control","method":{"id":"rho.negation"}}]}),
        instance_type: "c8a.2xlarge".into(),
        checkpoint_interval_seconds: 60,
        max_restores: 2,
        memory: Memory::default(),
        diagnostic_transport: None,
        fresh_owner: None,
        worker_campaign: None,
        command: None,
    }
}

#[cfg(test)]
pub fn command_fixture() -> Request {
    let mut request = fixture();
    request.spec = Value::Null;
    request.max_restores = 40;
    request.command = Some(CommandMode {
        argv: vec![
            "/opt/study/launch/job-prime-tower-criu.sh".into(),
            "main".into(),
        ],
        outputs: "/work/compute/out".into(),
        threads: 2,
        binaries: BTreeMap::from([("/opt/study/bin/prime-tower".into(), "f".repeat(64))]),
        pvc_gib: DEFAULT_PVC_GIB,
    });
    request
}

#[cfg(test)]
mod tests {
    use super::*;
    fn diagnostic_fixture() -> Request {
        let mut request = fixture();
        let mut computation = request.spec.clone();
        computation["measurement"]["isolation_required"] = serde_json::json!("L0");
        computation["label"] =
            serde_json::json!("CRIU computation only: all timing invalid for benchmark admission");
        let mut fresh = serde_json::to_vec(&request.spec).unwrap();
        fresh.push(b'\n');
        let raw = "{ \"opaque_launch_fixture\": true }\n".to_owned();
        let launch = DiagnosticLaunchBytes {
            sha256: digest(raw.as_bytes()),
            raw_json: raw,
            spec_bytes_sha256: digest(&fresh),
        };
        request.diagnostic_transport = Some(DiagnosticTransport {
            schema: "isolab.diagnostic-transport/v1".into(),
            computation: DiagnosticLaunchBytes {
                spec_bytes_sha256: digest(&serde_json::to_vec_pretty(&computation).unwrap()),
                ..launch.clone()
            },
            fresh: launch,
        });
        request
    }
    #[test]
    fn diagnostic_transport_preserves_raw_bytes_and_separate_spec_bindings() {
        let request = diagnostic_fixture();
        let bytes = serde_json::to_vec(&request).unwrap();
        let parsed = Request::parse(&bytes).unwrap();
        assert_eq!(parsed.diagnostic_transport, request.diagnostic_transport);
        assert_ne!(request.hash(), fixture().hash());
        let mut wrong = request.clone();
        let t = wrong.diagnostic_transport.as_mut().unwrap();
        t.computation.spec_bytes_sha256 = t.fresh.spec_bytes_sha256.clone();
        assert!(wrong.validate().is_err());
        let mut wrong = request.clone();
        wrong
            .diagnostic_transport
            .as_mut()
            .unwrap()
            .fresh
            .raw_json
            .push(' ');
        assert!(wrong.validate().is_err());
        assert!(request
            .diagnostic_transport
            .as_ref()
            .unwrap()
            .validate(&Value::Null)
            .is_err());
    }
    #[test]
    fn diagnostic_transport_retains_total_request_cap() {
        let mut request = diagnostic_fixture();
        let t = request.diagnostic_transport.as_mut().unwrap();
        for launch in [&mut t.fresh, &mut t.computation] {
            launch.raw_json = " ".repeat(40_000);
            launch.sha256 = digest(launch.raw_json.as_bytes());
        }
        let bytes = serde_json::to_vec(&request).unwrap();
        assert!(bytes.len() > 65_536);
        assert!(Request::parse(&bytes).is_err());
    }
    #[test]
    fn memory_preserves_legacy_request_bytes_and_binds_explicit_eight_gib() {
        // Exact pre-memory serialization layout, independent of Request's fields.
        #[derive(Serialize)]
        struct Legacy<'a> {
            schema: &'a str,
            run_name: &'a str,
            deadline_unix_seconds: u64,
            build: &'a Build,
            runtime_image_uri: &'a str,
            spec: &'a Value,
            instance_type: &'a str,
            checkpoint_interval_seconds: u64,
            max_restores: u32,
        }
        let req = fixture();
        let legacy = serde_json::to_vec(&Legacy {
            schema: &req.schema,
            run_name: &req.run_name,
            deadline_unix_seconds: req.deadline_unix_seconds,
            build: &req.build,
            runtime_image_uri: &req.runtime_image_uri,
            spec: &req.spec,
            instance_type: &req.instance_type,
            checkpoint_interval_seconds: req.checkpoint_interval_seconds,
            max_restores: req.max_restores,
        })
        .unwrap();
        assert_eq!(serde_json::to_vec(&req).unwrap(), legacy);
        assert_eq!(req.hash(), digest(&legacy));
        let old = Request::parse(&legacy).unwrap();
        assert_eq!(old.memory, Memory::Gi4);
        let mut explicit = serde_json::to_value(&old).unwrap();
        explicit["memory"] = Value::String("4Gi".into());
        let four = Request::parse(&serde_json::to_vec(&explicit).unwrap()).unwrap();
        assert_eq!(four.hash(), old.hash());
        explicit["memory"] = Value::String("8Gi".into());
        let eight = Request::parse(&serde_json::to_vec(&explicit).unwrap()).unwrap();
        assert_eq!(eight.memory, Memory::Gi8);
        assert_ne!(eight.hash(), old.hash());
        assert_eq!(serde_json::to_value(&eight).unwrap()["memory"], "8Gi");
        assert_eq!(eight.deadline_unix_seconds, old.deadline_unix_seconds);
        for invalid in [
            serde_json::json!("16Gi"),
            serde_json::json!("8192Mi"),
            serde_json::json!(8),
            Value::Null,
        ] {
            explicit["memory"] = invalid;
            assert!(Request::parse(&serde_json::to_vec(&explicit).unwrap()).is_err());
        }
    }

    #[test]
    fn schema_memory_enum_matches_native_contract_and_is_optional() {
        let schema: Value =
            serde_json::from_str(include_str!("../../request.schema.json")).unwrap();
        assert_eq!(
            schema["properties"]["memory"]["enum"],
            serde_json::json!(["4Gi", "8Gi"])
        );
        assert_eq!(schema["properties"]["memory"]["default"], "4Gi");
        assert!(!schema["required"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("memory")));
        for value in schema["properties"]["memory"]["enum"].as_array().unwrap() {
            let mut req = serde_json::to_value(fixture()).unwrap();
            req["memory"] = value.clone();
            assert!(Request::parse(&serde_json::to_vec(&req).unwrap()).is_ok());
        }
    }

    #[test]
    fn strict_contract_rejects_unknown_fields_and_unpinned_sources() {
        let good = fixture();
        assert!(good.validate().is_ok());
        let mut v = serde_json::to_value(&good).unwrap();
        v["resume"] = Value::Bool(true);
        assert!(Request::parse(&serde_json::to_vec(&v).unwrap()).is_err());
        let mut bad = good.clone();
        bad.build.image_uri = format!("{REPOSITORY}:latest");
        assert!(bad.validate().is_err());
        bad = good.clone();
        bad.spec["measurement"]["isolation_required"] = Value::String("L0".into());
        assert!(bad.validate().is_err());
        bad = good;
        bad.instance_type = "m8a.2xlarge".into();
        assert!(bad.validate().is_err());
    }
    #[test]
    fn request_hash_binds_source_spec_and_original_deadline() {
        let req = fixture();
        let h = req.hash();
        let mut other = req.clone();
        other.deadline_unix_seconds += 1;
        assert_ne!(h, other.hash());
        other = req.clone();
        other.spec["measurement"]["rounds"] = serde_json::json!(6);
        assert_ne!(h, other.hash());
        other = req;
        other.build.source_revision = "d".repeat(40);
        assert_ne!(h, other.hash());
    }
    #[test]
    fn expiry_never_supplies_a_fresh_budget() {
        assert!(fixture().validate_new_deadline(1000).is_ok());
        assert!(fixture().validate_new_deadline(5001).is_err());
        assert!(fixture().validate_new_deadline(4999).is_err());
    }
    #[test]
    fn fresh_owner_requires_diagnostics_and_original_deadline() {
        let mut r = fixture();
        r.fresh_owner = Some(FreshOwnerAdmission {
            schema: "isolab.fresh-owner-admission/v1".into(),
            binary_sha256: "e".repeat(64),
            stop_utc_seconds: 4000,
        });
        assert!(r.validate().is_err());
        let mut r = diagnostic_fixture();
        r.fresh_owner = Some(FreshOwnerAdmission {
            schema: "isolab.fresh-owner-admission/v1".into(),
            binary_sha256: "e".repeat(64),
            stop_utc_seconds: 4000,
        });
        assert!(r.validate().is_ok());
        r.fresh_owner.as_mut().unwrap().stop_utc_seconds = r.deadline_unix_seconds + 1;
        assert!(r.validate().is_err());
    }

    #[test]
    fn recorded_ecbench_request_keeps_its_submitted_hash_and_bytes() {
        // Real periodic-B request and its controller submission receipt.
        let raw = include_bytes!("../../validation/20261022-periodic/attempt-b/request.json");
        let receipt: Value = serde_json::from_slice(include_bytes!(
            "../../validation/20261022-periodic/attempt-b/submission.json"
        ))
        .unwrap();
        let req = Request::parse(raw).unwrap();
        assert!(req.command.is_none());
        assert_eq!(req.hash(), receipt["request_sha256"]);
        // Same content as the recorded bytes, and no command-mode field appears.
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::from_slice::<Value>(raw).unwrap()
        );
        let serialized = serde_json::to_value(&req).unwrap();
        for absent in ["command", "threads"] {
            assert!(serialized.get(absent).is_none());
        }
        // Explicit null command is the same legacy request.
        let mut v: Value = serde_json::from_slice(raw).unwrap();
        v["command"] = Value::Null;
        assert_eq!(
            Request::parse(&serde_json::to_vec(&v).unwrap())
                .unwrap()
                .hash(),
            req.hash()
        );
    }
    #[test]
    fn ecbench_requests_reject_command_only_fields_and_keep_restore_bounds() {
        let mut r = fixture();
        for n in [1, 5] {
            r.max_restores = n;
            assert!(r.validate().is_ok());
        }
        for n in [0, 6, 50] {
            r.max_restores = n;
            assert!(r.validate().is_err());
        }
        let mut v = serde_json::to_value(fixture()).unwrap();
        v["threads"] = serde_json::json!(2);
        assert!(Request::parse(&serde_json::to_vec(&v).unwrap()).is_err());
        let mut v = serde_json::to_value(fixture()).unwrap();
        v.as_object_mut().unwrap().remove("spec");
        assert!(Request::parse(&serde_json::to_vec(&v).unwrap()).is_err());
    }
    #[test]
    fn command_request_accepts_the_documented_shape_and_example() {
        let r = command_fixture();
        r.validate().unwrap();
        let bytes = serde_json::to_vec(&r).unwrap();
        let v: Value = serde_json::from_slice(&bytes).unwrap();
        assert!(v.get("spec").is_none(), "null spec is omitted");
        assert_eq!(v["command"]["threads"], 2);
        let parsed = Request::parse(&bytes).unwrap();
        assert_eq!(parsed, r);
        assert_eq!(parsed.hash(), r.hash());
        // Explicit null spec and absent spec are one identity.
        let mut with_null = v.clone();
        with_null["spec"] = Value::Null;
        assert_eq!(
            Request::parse(&serde_json::to_vec(&with_null).unwrap())
                .unwrap()
                .hash(),
            r.hash()
        );
        // Default threads=1 canonicalizes to omission.
        let mut default = v.clone();
        default["command"]["threads"] = serde_json::json!(1);
        let one = Request::parse(&serde_json::to_vec(&default).unwrap()).unwrap();
        default["command"]
            .as_object_mut()
            .unwrap()
            .remove("threads");
        let omitted = Request::parse(&serde_json::to_vec(&default).unwrap()).unwrap();
        assert_eq!(one.hash(), omitted.hash());
        assert_eq!(omitted.command.as_ref().unwrap().threads, 1);
        assert!(serde_json::to_value(&omitted).unwrap()["command"]
            .get("threads")
            .is_none());
        assert_ne!(one.hash(), r.hash());
        // The checked-in example differs only in placeholder identities.
        let mut example: Value =
            serde_json::from_str(include_str!("../../request.command.example.json")).unwrap();
        example["build"] = serde_json::to_value(&r.build).unwrap();
        example["runtime_image_uri"] = Value::String(r.runtime_image_uri.clone());
        example["command"]["binaries"] =
            serde_json::json!({"/opt/study/bin/prime-tower":"f".repeat(64)});
        let example = Request::parse(&serde_json::to_vec(&example).unwrap()).unwrap();
        assert!(example.is_command());
    }
    #[test]
    fn command_request_rejects_invalid_shapes() {
        type Mutation = Box<dyn Fn(&mut Request)>;
        let bad: Vec<(&str, Mutation)> = vec![
            (
                "empty argv",
                Box::new(|r| r.command.as_mut().unwrap().argv.clear()),
            ),
            (
                "65 args",
                Box::new(|r| r.command.as_mut().unwrap().argv.resize(65, "x".into())),
            ),
            (
                "relative argv0",
                Box::new(|r| r.command.as_mut().unwrap().argv[0] = "bin/run".into()),
            ),
            (
                "dotdot argv0",
                Box::new(|r| r.command.as_mut().unwrap().argv[0] = "/opt/../bin/sh".into()),
            ),
            (
                "trailing slash",
                Box::new(|r| r.command.as_mut().unwrap().argv[0] = "/opt/run/".into()),
            ),
            (
                "pvc argv0",
                Box::new(|r| r.command.as_mut().unwrap().argv[0] = "/work/compute/run".into()),
            ),
            (
                "request argv0",
                Box::new(|r| r.command.as_mut().unwrap().argv[0] = "/request/x".into()),
            ),
            (
                "tmp argv0",
                Box::new(|r| r.command.as_mut().unwrap().argv[0] = "/tmp/x".into()),
            ),
            (
                "NUL arg",
                Box::new(|r| r.command.as_mut().unwrap().argv[1] = "a\0b".into()),
            ),
            (
                "newline arg",
                Box::new(|r| r.command.as_mut().unwrap().argv[1] = "a\nb".into()),
            ),
            (
                "CR arg",
                Box::new(|r| r.command.as_mut().unwrap().argv[1] = "a\rb".into()),
            ),
            (
                "long arg",
                Box::new(|r| r.command.as_mut().unwrap().argv[1] = "a".repeat(4097)),
            ),
            (
                "outputs outside",
                Box::new(|r| r.command.as_mut().unwrap().outputs = "/work/out".into()),
            ),
            (
                "outputs root",
                Box::new(|r| r.command.as_mut().unwrap().outputs = "/work/compute".into()),
            ),
            (
                "outputs prefix",
                Box::new(|r| r.command.as_mut().unwrap().outputs = "/work/computex/out".into()),
            ),
            (
                "outputs dotdot",
                Box::new(|r| {
                    r.command.as_mut().unwrap().outputs = "/work/compute/../control".into()
                }),
            ),
            (
                "outputs tmp",
                Box::new(|r| r.command.as_mut().unwrap().outputs = "/work/compute/tmp/out".into()),
            ),
            (
                "outputs stdout",
                Box::new(|r| {
                    r.command.as_mut().unwrap().outputs = "/work/compute/run.stdout".into()
                }),
            ),
            (
                "outputs slash",
                Box::new(|r| r.command.as_mut().unwrap().outputs = "/work/compute/out/".into()),
            ),
            (
                "threads 0",
                Box::new(|r| r.command.as_mut().unwrap().threads = 0),
            ),
            (
                "threads 5",
                Box::new(|r| r.command.as_mut().unwrap().threads = 5),
            ),
            (
                "binary hash",
                Box::new(|r| {
                    r.command
                        .as_mut()
                        .unwrap()
                        .binaries
                        .insert("/opt/x".into(), "F".repeat(64));
                }),
            ),
            (
                "binary path",
                Box::new(|r| {
                    r.command
                        .as_mut()
                        .unwrap()
                        .binaries
                        .insert("/work/x".into(), "f".repeat(64));
                }),
            ),
            (
                "binary argv0",
                Box::new(|r| {
                    let a = r.command.as_ref().unwrap().argv[0].clone();
                    r.command
                        .as_mut()
                        .unwrap()
                        .binaries
                        .insert(a, "f".repeat(64));
                }),
            ),
            (
                "33 binaries",
                Box::new(|r| {
                    for n in 0..33 {
                        r.command
                            .as_mut()
                            .unwrap()
                            .binaries
                            .insert(format!("/opt/b{n}"), "f".repeat(64));
                    }
                }),
            ),
            ("spec and command", Box::new(|r| r.spec = fixture().spec)),
            (
                "empty spec object",
                Box::new(|r| r.spec = serde_json::json!({})),
            ),
            (
                "diagnostics",
                Box::new(|r| r.diagnostic_transport = diagnostic_fixture().diagnostic_transport),
            ),
            (
                "fresh owner",
                Box::new(|r| {
                    r.fresh_owner = Some(FreshOwnerAdmission {
                        schema: "isolab.fresh-owner-admission/v1".into(),
                        binary_sha256: "e".repeat(64),
                        stop_utc_seconds: 4000,
                    })
                }),
            ),
            (
                "argv0 hash",
                Box::new(|r| r.build.ecbench_sha256 = "x".repeat(64)),
            ),
            (
                "instance",
                Box::new(|r| r.instance_type = "m8a.2xlarge".into()),
            ),
            (
                "unpinned image",
                Box::new(|r| r.build.image_uri = format!("{REPOSITORY}:latest")),
            ),
        ];
        for (name, mutate) in bad {
            let mut r = command_fixture();
            mutate(&mut r);
            assert!(r.validate().is_err(), "{name} was accepted");
        }
        let mut v = serde_json::to_value(command_fixture()).unwrap();
        v["command"]["env"] = serde_json::json!({"A":"B"});
        assert!(Request::parse(&serde_json::to_vec(&v).unwrap()).is_err());
        let mut v = serde_json::to_value(command_fixture()).unwrap();
        v["command"]["argv"] = serde_json::json!(["/opt/run", 7]);
        assert!(Request::parse(&serde_json::to_vec(&v).unwrap()).is_err());
    }
    #[test]
    fn max_restores_bounds_depend_on_mode() {
        let mut r = command_fixture();
        for n in [1, 5, 6, 50] {
            r.max_restores = n;
            assert!(r.validate().is_ok(), "{n}");
        }
        for n in [0, 51] {
            r.max_restores = n;
            assert!(r.validate().is_err(), "{n}");
        }
        let mut e = fixture();
        e.max_restores = 6;
        assert!(e.validate().is_err());
        for interval in [29, 901] {
            let mut r = command_fixture();
            r.checkpoint_interval_seconds = interval;
            assert!(r.validate().is_err());
        }
    }
    #[test]
    fn schema_command_bounds_match_native_contract() {
        let schema: Value =
            serde_json::from_str(include_str!("../../request.schema.json")).unwrap();
        let c = &schema["properties"]["command"];
        assert_eq!(c["properties"]["argv"]["minItems"], 1);
        assert_eq!(c["properties"]["argv"]["maxItems"], MAX_COMMAND_ARGS);
        assert_eq!(c["properties"]["threads"]["minimum"], 1);
        assert_eq!(c["properties"]["threads"]["maximum"], 4);
        assert_eq!(
            c["properties"]["binaries"]["maxProperties"],
            MAX_COMMAND_BINARIES
        );
        assert_eq!(
            schema["properties"]["max_restores"]["maximum"],
            MAX_COMMAND_RESTORES
        );
        assert!(!schema["required"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("spec")));
    }
    #[test]
    fn command_pvc_gib_is_bounded_and_default_keeps_hash() {
        let base = command_fixture();
        let v = serde_json::to_value(&base).unwrap();
        assert!(
            v["command"].get("pvc_gib").is_none(),
            "default 8 is omitted"
        );
        let mut explicit = v.clone();
        explicit["command"]["pvc_gib"] = serde_json::json!(8);
        let eight = Request::parse(&serde_json::to_vec(&explicit).unwrap()).unwrap();
        assert_eq!(eight.hash(), base.hash());
        assert_eq!(eight.pvc_gib(), 8);
        for gib in [9, 32, 64] {
            explicit["command"]["pvc_gib"] = serde_json::json!(gib);
            let r = Request::parse(&serde_json::to_vec(&explicit).unwrap()).unwrap();
            assert_eq!(r.pvc_gib(), gib);
            assert_ne!(r.hash(), base.hash());
            assert_eq!(serde_json::to_value(&r).unwrap()["command"]["pvc_gib"], gib);
        }
        for gib in [
            serde_json::json!(7),
            serde_json::json!(65),
            serde_json::json!(0),
            serde_json::json!("16"),
            Value::Null,
        ] {
            explicit["command"]["pvc_gib"] = gib;
            assert!(Request::parse(&serde_json::to_vec(&explicit).unwrap()).is_err());
        }
        assert_eq!(fixture().pvc_gib(), 8);
        let mut ecbench = serde_json::to_value(fixture()).unwrap();
        ecbench["pvc_gib"] = serde_json::json!(16);
        assert!(Request::parse(&serde_json::to_vec(&ecbench).unwrap()).is_err());
        let schema: Value =
            serde_json::from_str(include_str!("../../request.schema.json")).unwrap();
        let p = &schema["properties"]["command"]["properties"]["pvc_gib"];
        assert_eq!(p["minimum"], DEFAULT_PVC_GIB);
        assert_eq!(p["maximum"], MAX_PVC_GIB);
        assert_eq!(p["default"], DEFAULT_PVC_GIB);
    }
}
