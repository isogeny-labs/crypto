use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

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
    pub spec: Value,
    pub instance_type: String,
    pub checkpoint_interval_seconds: u64,
    pub max_restores: u32,
    #[serde(default, skip_serializing_if = "Memory::is_default")]
    pub memory: Memory,
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
        if !(30..=900).contains(&self.checkpoint_interval_seconds)
            || !(1..=5).contains(&self.max_restores)
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
        Ok(())
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
