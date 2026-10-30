//! Proposed policy occurrence admission. Pure byte/value helpers, no filesystem
//! mutation or implicit CLI admission. Register as types::case_binding.
use super::plan_v2::{CaseOrigin, CaseSpec, FactorBasePolicy, PlanDocument};
use super::{Artifact, CaseConfig};
use crate::ml_dataset::{curves, digest, JSON_INPUT_MAX_BYTES};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const POLICY_PATH: &str = "case-policy.json";
pub const POLICY_ROLE: &str = "restricted-case-policy-binding";
pub const POLICY_MAX_BYTES: usize = 16 * 1024;
pub const HISTORY_KEY: &str = "factor_base_policy_construction";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CasePolicyV2 {
    schema: String,
    origin: String,
    plan_sha256: String,
    plan_file_sha256: String,
    config_sha256: String,
    cell_index: u64,
    axes_cell_index: u64,
    policy_index: u64,
    policy: FactorBasePolicy,
}

/// Only construct through original-byte plan admission. A caller cannot replace
/// the normalized config/policy or stored digest after admission.
#[derive(Clone, Debug)]
pub struct BoundCase {
    spec: CaseSpec,
    plan_sha256: String,
    plan_file_sha256: String,
}
impl BoundCase {
    pub fn from_plan_bytes(plan_bytes: &[u8], index: u64) -> Result<Self, String> {
        let plan = PlanDocument::parse_json(plan_bytes)?;
        Ok(Self {
            spec: plan.cell(index)?,
            plan_sha256: plan.binding_sha256()?,
            plan_file_sha256: digest(plan_bytes),
        })
    }
    pub fn spec(&self) -> &CaseSpec {
        &self.spec
    }
    pub fn config(&self) -> &CaseConfig {
        &self.spec.config
    }
    pub fn plan_sha256(&self) -> &str {
        &self.plan_sha256
    }
    /// Removed only by the coordinated capture/startup/budget/export migration.
    /// No environment variable or optional custody file can enable this path.
    pub fn require_worker_activation(&self) -> Result<(), String> {
        if self.is_v2() {
            return Err(
                "V2 worker disabled pending complete custody and reader integration".into(),
            );
        }
        Ok(())
    }
    /// Call only after full expected-policy replay. Empty bounded constructor
    /// outcomes are retained; they are never solver proofs about a domain.
    pub fn constructor_stop(&self, base: &Value, private: &Value) -> Option<Value> {
        if matches!(
            &self.spec.policy,
            FactorBasePolicy::SignedFrobeniusUnionV1 { .. }
        ) && base["actual_size"].as_u64() == Some(0)
        {
            return Some(serde_json::json!({"status":"censored",
                "reason":"factor_base_domain_not_admitted",
                "factor_base_construction_status":private[HISTORY_KEY]["terminal_status"],
                "solver_completed":false,"scalar_recovered_without_witness":false,
                "exhaustive_no_decomposition":false,"domain_digest":null}));
        }
        None
    }
    pub fn is_v2(&self) -> bool {
        self.spec.origin == CaseOrigin::V2
    }
    pub fn cell_schema(&self) -> &'static str {
        if self.is_v2() {
            "crypto.ml-dataset.cell/v2"
        } else {
            "crypto.ml-dataset.cell/v1"
        }
    }
    pub fn run_schema(&self) -> &'static str {
        if self.is_v2() {
            "crypto.ml-dataset.run/v2"
        } else {
            "crypto.ml-dataset.run/v1"
        }
    }
    pub fn attempt_schema(&self) -> &'static str {
        if self.is_v2() {
            "crypto.ml-dataset.attempt/v2"
        } else {
            "crypto.ml-dataset.attempt/v1"
        }
    }
    /// No sidecar at all for historical V1. Explicit legacy-only V2 has one.
    pub fn occurrence(&self) -> Result<Option<CasePolicyV2>, String> {
        if !self.is_v2() {
            return Ok(None);
        }
        Ok(Some(CasePolicyV2 {
            schema: "crypto.ml-dataset.case-policy/v1".into(),
            origin: "v2".into(),
            plan_sha256: self.plan_sha256.clone(),
            plan_file_sha256: self.plan_file_sha256.clone(),
            config_sha256: digest(&serde_json::to_vec(self.config()).map_err(|e| e.to_string())?),
            cell_index: self.spec.cell_index,
            axes_cell_index: self.spec.axes_cell_index,
            policy_index: self.spec.policy_index,
            policy: self.spec.policy.clone(),
        }))
    }
    pub fn verify_config_bytes(&self, bytes: &[u8]) -> Result<(), String> {
        if bytes.len() as u64 > JSON_INPUT_MAX_BYTES {
            return Err("oversized config".into());
        }
        let config: CaseConfig = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        if config != *self.config() {
            return Err("config differs from exact plan occurrence".into());
        }
        Ok(())
    }
    pub fn verify_occurrence_bytes(&self, bytes: Option<&[u8]>) -> Result<(), String> {
        match (self.occurrence()?, bytes) {
            (None, None) => Ok(()),
            (Some(expected), Some(bytes)) => {
                if bytes.len() > POLICY_MAX_BYTES {
                    return Err("oversized case policy binding".into());
                }
                let saved: CasePolicyV2 =
                    serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
                if saved != expected {
                    return Err("case policy differs from exact frozen occurrence".into());
                }
                Ok(())
            }
            (None, Some(_)) => Err("V1 occurrence cannot carry a V2 policy binding".into()),
            (Some(_), None) => Err("V2 occurrence lacks case policy binding".into()),
        }
    }
    /// Existing run/protocol/source checks remain mandatory. This checks only
    /// the versioned plan/attempt identity, without weakening those checks.
    pub fn verify_parent_binding(&self, run: &Value, attempt: &Value) -> Result<(), String> {
        if run["schema"] != self.run_schema()
            || attempt["schema"] != self.attempt_schema()
            || run["plan_sha256"] != self.plan_sha256
            || attempt["plan_sha256"] != self.plan_sha256
            || attempt["cell_index"].as_u64() != Some(self.spec.cell_index)
        {
            return Err("run/attempt differs from versioned plan occurrence".into());
        }
        Ok(())
    }
    /// Call after storage::verify_manifest. Also checks reserved-path aliasing:
    /// changing both path and role cannot remove a mandatory V2 binding.
    pub fn verify_manifest_binding(
        &self,
        schema: &str,
        index: u64,
        plan_hash: &str,
        config: &CaseConfig,
        artifacts: &[Artifact],
        occurrence_bytes: Option<&[u8]>,
    ) -> Result<(), String> {
        if schema != self.cell_schema()
            || index != self.spec.cell_index
            || plan_hash != self.plan_sha256
            || config != self.config()
        {
            return Err("manifest differs from versioned plan occurrence".into());
        }
        self.verify_occurrence_bytes(occurrence_bytes)?;
        let rows: Vec<_> = artifacts
            .iter()
            .filter(|a| a.path == POLICY_PATH || a.role == POLICY_ROLE)
            .collect();
        if let Some(bytes) = occurrence_bytes {
            if rows.len() != 1
                || rows[0].path != POLICY_PATH
                || rows[0].role != POLICY_ROLE
                || rows[0].encoding != "json"
                || rows[0].bytes != bytes.len() as u64
                || rows[0].sha256 != digest(bytes)
            {
                return Err("case policy artifact path/role/bytes/hash mismatch".into());
            }
        } else if !rows.is_empty() {
            return Err("policy artifact has no admitted occurrence".into());
        }
        Ok(())
    }
    /// Known-answer replay stays a separate phase. The signed constructor's
    /// restricted transcript is already retained in witness.json; do not clone
    /// it into a second large file or accept mathematical-only replay instead.
    pub fn verify_factor_base(
        &self,
        public: &Value,
        base: &Value,
        private: Option<&Value>,
    ) -> Result<(), String> {
        let c = self.config();
        if public["family"].as_str() != Some(c.family.as_str())
            || public["n"].as_u64() != Some(u64::from(c.n))
            || public["m"].as_u64() != Some(c.m as u64)
            || public["twist"].as_bool() != Some(c.twist)
        {
            return Err("factor-base public context differs from plan occurrence".into());
        }
        if base["requested_size"].as_u64() != Some(c.factor_base_size as u64) {
            return Err("factor-base requested size differs from plan occurrence".into());
        }
        match &self.spec.policy {
            FactorBasePolicy::LegacySmallXProjectedV1 => {
                if base["schema"] != "ml-ecdlp-factor-base/v1" {
                    return Err("legacy occurrence requires legacy factor base".into());
                }
                curves::verify_factor_base(public, base)
            }
            policy @ FactorBasePolicy::SignedFrobeniusUnionV1 { .. } => {
                let private =
                    private.ok_or("signed occurrence lacks restricted witness artifact")?;
                if private["schema"] != "ml-ecdlp-private-witness/v1" {
                    return Err("signed occurrence has wrong witness schema".into());
                }
                let history = private
                    .get(HISTORY_KEY)
                    .ok_or("signed occurrence lacks restricted construction transcript")?;
                curves::factor_base_policy::verify(
                    public,
                    policy,
                    c.factor_base_size,
                    base,
                    history,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    const V1: &str = r#"{"schema":"crypto.ml-dataset.plan/v1","field_sizes":[7],"arities":[2],"families":["koblitz"],"solvers":["sat"],"curve_seeds":1,"generators":1,"keys":1,"seed":20261030,"factor_base_size":8,"degree_bound":3,"work_limit":10000,"twists":true,"protocol_sha256":null,"split_rule":"curve-twist-component-sha256-v1"}"#;
    fn v2() -> Vec<u8> {
        format!(r#"{{"schema":"crypto.ml-dataset.plan/v2","axes":{},"factor_base_policies":[{{"kind":"legacy-small-x-projected/v1"}}]}}"#, V1).into_bytes()
    }
    #[test]
    fn legacy_origin_rejects_signed_schema_before_mathematical_dispatch() {
        let bound = BoundCase::from_plan_bytes(V1.as_bytes(), 0).unwrap();
        let c = bound.config();
        let public = json!({"family":c.family,"n":c.n,"m":c.m,"twist":c.twist});
        let base = json!({"schema":"ml-ecdlp-factor-base/v2","requested_size":c.factor_base_size});
        assert_eq!(
            bound.verify_factor_base(&public, &base, None).unwrap_err(),
            "legacy occurrence requires legacy factor base"
        );
    }
    #[test]
    fn explicit_legacy_v2_cannot_collapse_into_v1_occurrence() {
        let old = BoundCase::from_plan_bytes(V1.as_bytes(), 0).unwrap();
        let new = BoundCase::from_plan_bytes(&v2(), 0).unwrap();
        assert_eq!(old.config(), new.config());
        assert!(old.occurrence().unwrap().is_none());
        let bytes = serde_json::to_vec(&new.occurrence().unwrap().unwrap()).unwrap();
        assert!(old.verify_occurrence_bytes(Some(&bytes)).is_err());
        assert!(new.verify_occurrence_bytes(None).is_err());
        new.verify_occurrence_bytes(Some(&bytes)).unwrap();
    }
    #[test]
    fn exact_plan_bytes_and_index_are_authenticated_separately() {
        let raw = v2();
        let a = BoundCase::from_plan_bytes(&raw, 0).unwrap();
        let mut pretty = raw.clone();
        pretty.push(b'\n');
        let b = BoundCase::from_plan_bytes(&pretty, 0).unwrap();
        assert_eq!(a.plan_sha256(), b.plan_sha256());
        let bytes = serde_json::to_vec(&a.occurrence().unwrap().unwrap()).unwrap();
        assert!(b.verify_occurrence_bytes(Some(&bytes)).is_err());
        assert!(BoundCase::from_plan_bytes(&raw, 1)
            .unwrap()
            .verify_occurrence_bytes(Some(&bytes))
            .is_err());
        let mut duplicate = String::from_utf8(bytes).unwrap();
        duplicate = duplicate.replacen(
            "\"policy_index\":0",
            "\"policy_index\":0,\"policy_index\":0",
            1,
        );
        assert!(a
            .verify_occurrence_bytes(Some(duplicate.as_bytes()))
            .is_err());
    }
    #[test]
    fn rehashed_manifest_role_or_policy_mutation_cannot_change_expected_domain() {
        let b = BoundCase::from_plan_bytes(&v2(), 0).unwrap();
        let bytes = serde_json::to_vec(&b.occurrence().unwrap().unwrap()).unwrap();
        let mut a = Artifact {
            path: POLICY_PATH.into(),
            role: POLICY_ROLE.into(),
            encoding: "json".into(),
            sha256: digest(&bytes),
            bytes: bytes.len() as u64,
        };
        b.verify_manifest_binding(
            b.cell_schema(),
            0,
            b.plan_sha256(),
            b.config(),
            &[a.clone()],
            Some(&bytes),
        )
        .unwrap();
        a.role = "unregistered-partial-artifact".into();
        assert!(b
            .verify_manifest_binding(
                b.cell_schema(),
                0,
                b.plan_sha256(),
                b.config(),
                &[a],
                Some(&bytes)
            )
            .is_err());
        let mut changed = serde_json::to_value(b.occurrence().unwrap().unwrap()).unwrap();
        changed["policy"] = json!({"kind":"signed-frobenius-union/v1","max_candidates":1,"max_points":8,"max_action_steps":10,"max_artifact_bytes":4096});
        let forged = serde_json::to_vec(&changed).unwrap();
        assert!(b.verify_occurrence_bytes(Some(&forged)).is_err());
    }
}
