//! Explicit, bounded bindings for the optional diagnostic lifecycle.
//!
//! Parsing this document is not scientific execution admission. Source receipts
//! are externally supplied manifest bindings, never runtime source attestation.
//! The historical prefix Context v1 remains a separate, unchanged type.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::canonical::{reject_duplicate_keys, sha256_hex};
use super::diagnostics::Context;
use super::methods::ResolvedMethod;
use super::spec::{Plan, Role, Spec};

pub const MAX_LAUNCH_BYTES: usize = 65_536;
pub const LAUNCH_SCHEMA: &str = "ecbench.diagnostic_launch/v2";
pub const CHILD_SCHEMA: &str = "ecbench.diagnostic_child/v2";
pub const PROFILE_VERSION: &str = "ecbench.all_arm_lifecycle/v1";
const PENCIL: &str = "ic.three_leads_genus2_pencil_symbolic_v2";
const RHO: &str = "rho.three_leads_negation_v1";

/// This is the sole supported source claim: the named receipt externally binds
/// a source manifest. It does not prove which sources produced the executable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceScope {
    ExternalManifestBindingOnly,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceReceipt {
    pub scope: SourceScope,
    pub receipt_sha256: String,
    pub source_manifest_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub version: String,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArmBinding {
    pub name: String,
    pub role: Role,
    /// Registry identifier, not the short ECM1 configuration identifier.
    pub method: String,
    pub method_config_sha256: String,
    pub profile: Profile,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaunchDocument {
    pub schema: String,
    pub expected_spec_sha256: String,
    pub binary_sha256: String,
    pub protocol_sha256: String,
    pub source_receipt: SourceReceipt,
    pub scientific_run_id: String,
    /// Explicit caller-assigned attempt; no inference from process/retry state.
    pub attempt: u64,
    pub profile: Profile,
    /// Exact spec order. Duplicate method IDs are expected for rho and rho_AA.
    pub arms: Vec<ArmBinding>,
}

/// Exact raw UTF-8 is retained so RunOptions serde round trips cannot substitute
/// document fields while keeping an unrelated launch digest. Every validation
/// reparses and checks these bytes. No filesystem path is scientific authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticLaunch {
    raw_json: String,
    raw_sha256: String,
}

fn digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn valid_run_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
}

/// Existing prefix-reader convention: SHA-256 of the serialized JSON Value
/// for the full ResolvedMethod, NOT its internal method_sha256 identity field.
pub fn resolved_method_binding(method: &ResolvedMethod) -> Result<String, String> {
    let value = serde_json::to_value(method).map_err(|e| e.to_string())?;
    let bytes = serde_json::to_vec(&value).map_err(|e| e.to_string())?;
    Ok(sha256_hex(&bytes))
}

impl DiagnosticLaunch {
    /// Reads at most MAX_LAUNCH_BYTES+1 bytes; rejects oversize before parsing.
    pub fn load(path: &Path) -> Result<Self, String> {
        let mut bytes = Vec::new();
        File::open(path)
            .map_err(|e| format!("diagnostic launch open: {e}"))?
            .take((MAX_LAUNCH_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|e| format!("diagnostic launch read: {e}"))?;
        Self::from_bytes(&bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > MAX_LAUNCH_BYTES {
            return Err("diagnostic launch exceeds byte bound".into());
        }
        let value = Self {
            raw_json: std::str::from_utf8(bytes)
                .map_err(|_| "diagnostic launch is not UTF-8")?
                .into(),
            raw_sha256: sha256_hex(bytes),
        };
        value.document()?;
        Ok(value)
    }

    pub fn raw_sha256(&self) -> &str {
        &self.raw_sha256
    }
    pub fn raw_json(&self) -> &str {
        &self.raw_json
    }

    /// Revalidates on each use, including values deserialized as RunOptions.
    pub fn document(&self) -> Result<LaunchDocument, String> {
        if self.raw_json.len() > MAX_LAUNCH_BYTES
            || self.raw_sha256 != sha256_hex(self.raw_json.as_bytes())
        {
            return Err("diagnostic launch raw bytes/hash mismatch or oversized".into());
        }
        reject_duplicate_keys(&self.raw_json)?;
        let doc: LaunchDocument = serde_json::from_str(&self.raw_json)
            .map_err(|e| format!("diagnostic launch schema: {e}"))?;
        doc.validate()?;
        Ok(doc)
    }

    /// Pure pre-plan gate: does not construct workloads or invoke algorithms.
    /// spec_sha256 must be ecbench's identity-view digest, not raw spec bytes.
    pub fn validate_spec(
        &self,
        spec: &Spec,
        spec_sha256: &str,
        binary_sha256: Option<&str>,
    ) -> Result<(), String> {
        let doc = self.document()?;
        if doc.expected_spec_sha256 != spec_sha256
            || binary_sha256 != Some(doc.binary_sha256.as_str())
        {
            return Err(
                "diagnostic launch spec/binary binding mismatch or missing binary hash".into(),
            );
        }
        if spec.arms.len() != doc.arms.len()
            || spec
                .arms
                .iter()
                .zip(&doc.arms)
                .any(|(a, b)| a.name != b.name || a.role != b.role || a.method.id != b.method)
        {
            return Err("diagnostic launch differs from exact ordered spec arms".into());
        }
        Ok(())
    }

    /// Configuration check after ordinary plan expansion. No algorithm executes.
    pub fn validate_plan(&self, plan: &Plan) -> Result<(), String> {
        let doc = self.document()?;
        self.validate_spec(&plan.spec, &plan.spec_sha256, Some(&doc.binary_sha256))?;
        if plan.arms.len() != doc.arms.len()
            || plan.arms.iter().zip(&doc.arms).any(|(a, b)| {
                a.name != b.name
                    || a.role != b.role
                    || a.method.id != b.method
                    || resolved_method_binding(&a.method).as_deref()
                        != Ok(b.method_config_sha256.as_str())
            })
        {
            return Err("diagnostic launch resolved arm/configuration mismatch".into());
        }
        Ok(())
    }
}

impl LaunchDocument {
    fn validate(&self) -> Result<(), String> {
        if self.schema != LAUNCH_SCHEMA
            || !valid_run_id(&self.scientific_run_id)
            || self.profile.version != PROFILE_VERSION
        {
            return Err("diagnostic launch schema/run/profile unsupported".into());
        }
        for hash in [
            &self.expected_spec_sha256,
            &self.binary_sha256,
            &self.protocol_sha256,
            &self.source_receipt.receipt_sha256,
            &self.source_receipt.source_manifest_sha256,
            &self.profile.sha256,
        ] {
            if !digest(hash) {
                return Err("diagnostic launch requires lowercase SHA-256 digests".into());
            }
        }
        let expected = [
            ("pencil", Role::Candidate, PENCIL),
            ("rho", Role::Reference, RHO),
            ("rho_AA", Role::Control, RHO),
        ];
        if self.arms.len() != expected.len() {
            return Err("diagnostic launch requires all three frozen arms".into());
        }
        // Exact set, but ordering is the spec's ordering, checked separately.
        for (name, role, method) in expected {
            let found: Vec<_> = self.arms.iter().filter(|a| a.name == name).collect();
            if found.len() != 1 {
                return Err("diagnostic launch missing/duplicate frozen arm".into());
            }
            let a = found[0];
            if a.role != role
                || a.method != method
                || a.profile != self.profile
                || !digest(&a.method_config_sha256)
            {
                return Err(
                    "diagnostic launch arm role/method/profile/configuration mismatch".into(),
                );
            }
        }
        Ok(())
    }
}

/// An additional child field, never a reinterpretation of Context v1.
/// Parent and child must validate against the independently supplied launch.
/// This binds metadata only; it does not activate a producer or admit a trial.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChildEnvelope {
    pub schema: String,
    pub launch_sha256: String,
    pub scientific_run_id: String,
    pub profile: Profile,
    pub source_receipt: SourceReceipt,
    pub arm: ArmBinding,
    pub prefix: Context,
}

impl ChildEnvelope {
    pub fn validate(&self, launch: &DiagnosticLaunch) -> Result<(), String> {
        let doc = launch.document()?;
        self.prefix.validate()?;
        let identity = &self.prefix.identity;
        if self.schema != CHILD_SCHEMA
            || self.launch_sha256 != launch.raw_sha256()
            || self.scientific_run_id != doc.scientific_run_id
            || self.profile != doc.profile
            || self.source_receipt != doc.source_receipt
            || !doc.arms.contains(&self.arm)
            || identity.attempt != doc.attempt
            || identity.binary_sha256 != doc.binary_sha256
            || identity.protocol_sha256 != doc.protocol_sha256
            || identity.source_sha256 != doc.source_receipt.source_manifest_sha256
            || identity.method_config_sha256 != self.arm.method_config_sha256
        {
            return Err("diagnostic child envelope binding mismatch".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod diagnostic_launch_tests {
    use super::super::diagnostics::Identity;
    use super::*;
    use serde_json::{json, Value};

    fn fixture() -> Value {
        let profile = json!({"version": PROFILE_VERSION, "sha256": "a".repeat(64)});
        let arms: Vec<_> = [
            ("pencil", "candidate", PENCIL),
            ("rho", "reference", RHO),
            ("rho_AA", "control", RHO),
        ]
        .iter()
        .map(|(name, role, method)| {
            json!({
                "name": name, "role": role, "method": method,
                "method_config_sha256": "b".repeat(64), "profile": profile
            })
        })
        .collect();
        json!({"schema": LAUNCH_SCHEMA, "expected_spec_sha256": "c".repeat(64),
            "binary_sha256": "d".repeat(64), "protocol_sha256": "e".repeat(64),
            "source_receipt": {"scope": "external_manifest_binding_only",
                "receipt_sha256": "f".repeat(64), "source_manifest_sha256": "1".repeat(64)},
            "scientific_run_id": "synthetic-fixture", "attempt": 2,
            "profile": profile, "arms": arms})
    }
    fn parse(value: &Value) -> Result<DiagnosticLaunch, String> {
        DiagnosticLaunch::from_bytes(&serde_json::to_vec(value).unwrap())
    }
    fn spec(doc: &LaunchDocument) -> Spec {
        // Deserializing an empty workload description performs no generation.
        serde_json::from_value(json!({"schema": "ecbench.spec/v1", "label": "fixture",
            "workloads": {"curves": [], "targets_per_curve": 0, "target_seed": 0},
            "arms": doc.arms.iter().map(|a| json!({"name":a.name,"role":a.role,
                "method":{"id":a.method}})).collect::<Vec<_>>(),
            "measurement":{"rounds":0,"seed":0}}))
        .unwrap()
    }
    fn envelope(launch: &DiagnosticLaunch) -> ChildEnvelope {
        let d = launch.document().unwrap();
        ChildEnvelope {
            schema: CHILD_SCHEMA.into(),
            launch_sha256: launch.raw_sha256().into(),
            scientific_run_id: d.scientific_run_id,
            profile: d.profile,
            source_receipt: d.source_receipt.clone(),
            arm: d.arms[0].clone(),
            prefix: Context {
                schema: "ecbench.diagnostic_context/v1".into(),
                relative_leaf: "execution_0".into(),
                identity: Identity {
                    session: "fixture-session".into(),
                    execution: 0,
                    workload_sha256: "2".repeat(64),
                    binary_sha256: d.binary_sha256,
                    protocol_sha256: d.protocol_sha256,
                    seed: 0,
                    attempt: d.attempt,
                    source_sha256: d.source_receipt.source_manifest_sha256,
                    method_config_sha256: d.arms[0].method_config_sha256.clone(),
                },
            },
        }
    }

    #[test]
    fn launch_roundtrip_preserves_raw_custody() {
        let l = parse(&fixture()).unwrap();
        let again: DiagnosticLaunch =
            serde_json::from_slice(&serde_json::to_vec(&l).unwrap()).unwrap();
        assert_eq!(again, l);
        assert!(again.document().is_ok());
        let mut forged = serde_json::to_value(&l).unwrap();
        forged["raw_json"] = json!(l.raw_json().replace("synthetic-fixture", "other-fixture"));
        let forged: DiagnosticLaunch = serde_json::from_value(forged).unwrap();
        assert!(forged.document().is_err());
    }
    #[test]
    fn launch_duplicate_unknown_and_oversize_rejected() {
        let bytes = serde_json::to_string(&fixture()).unwrap();
        let duplicate = bytes.replacen('{', "{\"schema\":\"duplicate\",", 1);
        assert!(DiagnosticLaunch::from_bytes(duplicate.as_bytes()).is_err());
        let mut v = fixture();
        v["unrecognized"] = json!(true);
        assert!(parse(&v).is_err());
        let mut v = fixture();
        v["source_receipt"]["attested"] = json!(true);
        assert!(parse(&v).is_err());
        assert!(DiagnosticLaunch::from_bytes(&vec![b' '; MAX_LAUNCH_BYTES + 1]).is_err());
        assert!(DiagnosticLaunch::from_bytes(&[0xff]).is_err());
    }
    #[test]
    fn launch_all_three_arms_roles_methods_and_profiles_required() {
        for field in ["name", "role", "method", "method_config_sha256"] {
            let mut v = fixture();
            v["arms"][0][field] = json!("wrong");
            assert!(parse(&v).is_err());
        }
        let mut v = fixture();
        v["arms"].as_array_mut().unwrap().pop();
        assert!(parse(&v).is_err());
        let mut v = fixture();
        v["arms"][1] = v["arms"][0].clone();
        assert!(parse(&v).is_err());
        let mut v = fixture();
        v["arms"][1]["profile"]["sha256"] = json!("9".repeat(64));
        assert!(parse(&v).is_err());
    }
    #[test]
    fn launch_source_attestation_and_bad_digests_refused() {
        let mut v = fixture();
        v["source_receipt"]["scope"] = json!("runtime_source_attestation");
        assert!(parse(&v).is_err());
        for field in ["binary_sha256", "protocol_sha256", "expected_spec_sha256"] {
            let mut v = fixture();
            v[field] = json!("A".repeat(64));
            assert!(parse(&v).is_err());
        }
        let mut v = fixture();
        v["scientific_run_id"] = json!("../escape");
        assert!(parse(&v).is_err());
    }
    #[test]
    fn launch_preplan_binding_requires_binary_and_exact_spec_order() {
        let l = parse(&fixture()).unwrap();
        let d = l.document().unwrap();
        let mut s = spec(&d);
        assert!(l
            .validate_spec(&s, &d.expected_spec_sha256, Some(&d.binary_sha256))
            .is_ok());
        assert!(l.validate_spec(&s, &d.expected_spec_sha256, None).is_err());
        assert!(l
            .validate_spec(&s, &"0".repeat(64), Some(&d.binary_sha256))
            .is_err());
        assert!(l
            .validate_spec(&s, &d.expected_spec_sha256, Some(&"0".repeat(64)))
            .is_err());
        s.arms.swap(0, 1);
        assert!(l
            .validate_spec(&s, &d.expected_spec_sha256, Some(&d.binary_sha256))
            .is_err());
    }
    #[test]
    fn launch_resolved_configuration_bound_without_workloads() {
        use super::super::methods::ResolvedMethod;
        use super::super::spec::Arm;
        let l = parse(&fixture()).unwrap();
        let d = l.document().unwrap();
        let mut p = Plan {
            spec: spec(&d),
            spec_id: "fixture".into(),
            spec_sha256: d.expected_spec_sha256.clone(),
            arms: d
                .arms
                .iter()
                .map(|a| Arm {
                    name: a.name.clone(),
                    role: a.role,
                    method: ResolvedMethod {
                        id: a.method.clone(),
                        family: "fixture".into(),
                        params: Default::default(),
                        method_id: "fixture".into(),
                        method_sha256: a.method_config_sha256.clone(),
                        entry: "unused".into(),
                    },
                })
                .collect(),
            workloads: vec![],
            warnings: vec![],
            instances: vec![],
            curve_of: vec![],
            executions: vec![],
        };
        let mut bound = fixture();
        for (index, arm) in p.arms.iter().enumerate() {
            bound["arms"][index]["method_config_sha256"] =
                json!(resolved_method_binding(&arm.method).unwrap());
        }
        let l = parse(&bound).unwrap();
        assert!(l.validate_plan(&p).is_ok());
        assert_ne!(
            resolved_method_binding(&p.arms[0].method).unwrap(),
            p.arms[0].method.method_sha256
        );
        p.arms[0].method.method_sha256 = "0".repeat(64);
        assert!(l.validate_plan(&p).is_err());
    }
    #[test]
    fn child_v2_preserves_v1_and_binds_launch_metadata() {
        let l = parse(&fixture()).unwrap();
        let e = envelope(&l);
        assert!(e.validate(&l).is_ok());
        let v1 = serde_json::to_value(&e.prefix).unwrap();
        let roundtrip: Context = serde_json::from_value(v1).unwrap();
        assert_eq!(roundtrip, e.prefix);
        let mut bad = e.clone();
        bad.prefix.identity.attempt += 1;
        assert!(bad.validate(&l).is_err());
        let mut bad = e.clone();
        bad.prefix.identity.source_sha256 = "0".repeat(64);
        assert!(bad.validate(&l).is_err());
        let mut bad = e.clone();
        bad.source_receipt.receipt_sha256 = "0".repeat(64);
        assert!(bad.validate(&l).is_err());
        let mut bad = e.clone();
        bad.profile.sha256 = "0".repeat(64);
        assert!(bad.validate(&l).is_err());
        let mut bad = e.clone();
        bad.arm.role = Role::Reference;
        assert!(bad.validate(&l).is_err());
        let mut bad = e;
        bad.launch_sha256 = "0".repeat(64);
        assert!(bad.validate(&l).is_err());
    }
}
