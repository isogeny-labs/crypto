//! Versioned factor-base policy plans without rewriting historical v1 identity.
//!
//! JSON admission deliberately probes the schema and then parses the original
//! bytes as a strict typed plan. A Value round-trip would discard duplicate keys.
//! No generic Deserialize implementation is provided for PlanDocument.
//! This module alone does not enable v2 worker, CLI, or export admission.

use super::CaseConfig;
use crate::ml_dataset::{digest, DatasetPlan, JSON_INPUT_MAX_BYTES};
use serde::{Deserialize, Serialize, Serializer};
use std::collections::BTreeSet;

pub const PLAN_V1_SCHEMA: &str = "crypto.ml-dataset.plan/v1";
pub const PLAN_V2_SCHEMA: &str = "crypto.ml-dataset.plan/v2";
pub const DEFAULT_MAX_CANDIDATES: u64 = 131_200;
pub const DEFAULT_MAX_POINTS: usize = 4_096;
pub const DEFAULT_MAX_ACTION_STEPS: u64 = 10_000_000;
pub const DEFAULT_MAX_ARTIFACT_BYTES: u64 = 64 * 1024 * 1024;
pub const MAX_CANDIDATES: u64 = 1_000_000;
pub const MAX_POINTS: usize = 4_096;
pub const MAX_ACTION_STEPS: u64 = 10_000_000;
pub const MIN_ARTIFACT_BYTES: u64 = 4_096;
pub const MAX_ARTIFACT_BYTES: u64 = 64 * 1024 * 1024;

/// Concrete construction choices. Signed-union parameters must all be present
/// in JSON; defaults are an explicit constructor choice, never a parsing rule.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum FactorBasePolicy {
    #[serde(rename = "legacy-small-x-projected/v1")]
    LegacySmallXProjectedV1,
    #[serde(rename = "signed-frobenius-union/v1")]
    SignedFrobeniusUnionV1 {
        max_candidates: u64,
        max_points: usize,
        max_action_steps: u64,
        max_artifact_bytes: u64,
    },
}

impl FactorBasePolicy {
    pub fn signed_frobenius_union_v1_defaults() -> Self {
        Self::SignedFrobeniusUnionV1 {
            max_candidates: DEFAULT_MAX_CANDIDATES,
            max_points: DEFAULT_MAX_POINTS,
            max_action_steps: DEFAULT_MAX_ACTION_STEPS,
            max_artifact_bytes: DEFAULT_MAX_ARTIFACT_BYTES,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if let Self::SignedFrobeniusUnionV1 {
            max_candidates,
            max_points,
            max_action_steps,
            max_artifact_bytes,
        } = self
        {
            if !(1..=MAX_CANDIDATES).contains(max_candidates) {
                return Err("factor-base max_candidates must be 1..1000000".into());
            }
            if !(1..=MAX_POINTS).contains(max_points) {
                return Err("factor-base max_points must be 1..4096".into());
            }
            if !(1..=MAX_ACTION_STEPS).contains(max_action_steps) {
                return Err("factor-base max_action_steps must be 1..10000000".into());
            }
            if !(MIN_ARTIFACT_BYTES..=MAX_ARTIFACT_BYTES).contains(max_artifact_bytes) {
                return Err("factor-base max_artifact_bytes must be 4096..67108864".into());
            }
        }
        Ok(())
    }

    pub fn validate_requested_points(&self, requested: usize) -> Result<(), String> {
        self.validate()?;
        let max_points = match self {
            Self::LegacySmallXProjectedV1 => MAX_POINTS,
            Self::SignedFrobeniusUnionV1 { max_points, .. } => *max_points,
        };
        if requested == 0 || requested > max_points {
            return Err(
                "requested factor-base population must be nonzero and <= max_points".into(),
            );
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatasetPlanV2 {
    pub schema: String,
    pub axes: DatasetPlan,
    pub factor_base_policies: Vec<FactorBasePolicy>,
}

impl DatasetPlanV2 {
    pub fn new(
        axes: DatasetPlan,
        factor_base_policies: Vec<FactorBasePolicy>,
    ) -> Result<Self, String> {
        let plan = Self {
            schema: PLAN_V2_SCHEMA.into(),
            axes,
            factor_base_policies,
        };
        plan.validate()?;
        Ok(plan)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != PLAN_V2_SCHEMA {
            return Err("expected crypto.ml-dataset.plan/v2".into());
        }
        self.axes.validate()?;
        if self.factor_base_policies.is_empty() {
            return Err("factor_base_policies must be nonempty".into());
        }
        // Typed equality is canonical: object field order in incoming JSON does
        // not create another policy or another independent axis value.
        let mut seen = BTreeSet::new();
        for policy in &self.factor_base_policies {
            policy.validate_requested_points(self.axes.factor_base_size)?;
            if !seen.insert(policy) {
                return Err("duplicate canonical factor-base policy".into());
            }
        }
        self.checked_cell_count()?;
        Ok(())
    }

    fn checked_cell_count(&self) -> Result<u64, String> {
        let policies = u64::try_from(self.factor_base_policies.len())
            .map_err(|_| "factor-base policy count does not fit u64")?;
        self.axes
            .cell_count()?
            .checked_mul(policies)
            .ok_or_else(|| "v2 plan cell count overflow".into())
    }

    pub fn cell_count(&self) -> Result<u64, String> {
        self.validate()?;
        self.checked_cell_count()
    }

    pub fn cell(&self, index: u64) -> Result<CaseSpec, String> {
        // Legacy cell() assumes nonzero radices: validate before any division.
        self.validate()?;
        if index >= self.checked_cell_count()? {
            return Err("cell index outside plan".into());
        }
        let block = self.axes.cell_count()?;
        let policy_index = index / block;
        let axes_cell_index = index % block;
        let policy_offset = usize::try_from(policy_index)
            .map_err(|_| "factor-base policy index does not fit usize")?;
        let policy = self
            .factor_base_policies
            .get(policy_offset)
            .ok_or("factor-base policy index outside plan")?
            .clone();
        Ok(CaseSpec {
            origin: CaseOrigin::V2,
            config: self.axes.cell(axes_cell_index)?,
            policy,
            policy_index,
            cell_index: index,
            axes_cell_index,
        })
    }

    pub fn binding_sha256(&self) -> Result<String, String> {
        self.validate()?;
        Ok(digest(
            &serde_json::to_vec(self).map_err(|e| e.to_string())?,
        ))
    }
}

/// Occurrence origin, independent of equality of the normalized policy/domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaseOrigin {
    V1,
    V2,
}

/// Internal normalization, deliberately not Serialize: legacy case identity is
/// still the unchanged serialized CaseConfig. Future v2 occurrence envelopes
/// must explicitly bind this origin and policy/index provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaseSpec {
    pub origin: CaseOrigin,
    pub config: CaseConfig,
    pub policy: FactorBasePolicy,
    pub policy_index: u64,
    pub cell_index: u64,
    pub axes_cell_index: u64,
}

#[derive(Clone, Debug)]
pub enum PlanDocument {
    V1(DatasetPlan),
    V2(DatasetPlanV2),
}

impl PlanDocument {
    pub fn parse_json(bytes: &[u8]) -> Result<Self, String> {
        if u64::try_from(bytes.len()).map_err(|_| "JSON input length does not fit u64")?
            > JSON_INPUT_MAX_BYTES
        {
            return Err("plan JSON input exceeds32MiB limit".into());
        }
        // Only this probe ignores unrelated fields. The selected second parse
        // uses the original bytes, including every duplicate nested field.
        #[derive(Deserialize)]
        struct SchemaProbe {
            schema: String,
        }
        let probe: SchemaProbe = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        let plan = match probe.schema.as_str() {
            PLAN_V1_SCHEMA => Self::V1(serde_json::from_slice(bytes).map_err(|e| e.to_string())?),
            PLAN_V2_SCHEMA => Self::V2(serde_json::from_slice(bytes).map_err(|e| e.to_string())?),
            _ => return Err(format!("unsupported dataset plan schema: {}", probe.schema)),
        };
        plan.validate()?;
        Ok(plan)
    }

    pub fn axes(&self) -> &DatasetPlan {
        match self {
            Self::V1(plan) => plan,
            Self::V2(plan) => &plan.axes,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::V1(plan) => plan.validate(),
            Self::V2(plan) => plan.validate(),
        }
    }

    pub fn cell_count(&self) -> Result<u64, String> {
        self.validate()?;
        match self {
            Self::V1(plan) => plan.cell_count(),
            Self::V2(plan) => plan.checked_cell_count(),
        }
    }

    pub fn cell(&self, index: u64) -> Result<CaseSpec, String> {
        match self {
            Self::V1(plan) => {
                plan.validate()?;
                Ok(CaseSpec {
                    origin: CaseOrigin::V1,
                    config: plan.cell(index)?,
                    policy: FactorBasePolicy::LegacySmallXProjectedV1,
                    policy_index: 0,
                    cell_index: index,
                    axes_cell_index: index,
                })
            }
            Self::V2(plan) => plan.cell(index),
        }
    }

    pub fn binding_sha256(&self) -> Result<String, String> {
        self.validate()?;
        match self {
            // Keep the actual historic implementation as the v1 authority.
            Self::V1(plan) => plan.binding_sha256(),
            Self::V2(plan) => plan.binding_sha256(),
        }
    }
}

impl Serialize for PlanDocument {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::V1(plan) => plan.serialize(serializer),
            Self::V2(plan) => plan.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const LEGACY_PLAN: &str = r#"{"schema":"crypto.ml-dataset.plan/v1","field_sizes":[7],"arities":[2],"families":["koblitz"],"solvers":["sat"],"curve_seeds":1,"generators":1,"keys":1,"seed":20261030,"factor_base_size":8,"degree_bound":3,"work_limit":10000,"twists":true,"protocol_sha256":null,"split_rule":"curve-twist-component-sha256-v1"}"#;
    const LEGACY_CASE: &str = r#"{"family":"koblitz","n":7,"m":2,"twist":false,"curve_seed":0,"generator_index":0,"key_index":0,"seed":20261030,"factor_base_size":8,"degree_bound":3,"work_limit":10000,"solver":"sat"}"#;

    fn axes() -> DatasetPlan {
        serde_json::from_str(LEGACY_PLAN).unwrap()
    }

    fn signed() -> FactorBasePolicy {
        FactorBasePolicy::signed_frobenius_union_v1_defaults()
    }

    fn wrapper() -> DatasetPlanV2 {
        DatasetPlanV2::new(
            axes(),
            vec![FactorBasePolicy::LegacySmallXProjectedV1, signed()],
        )
        .unwrap()
    }

    #[test]
    fn legacy_serialization_hash_and_case_golden_remain_exact() {
        let document = PlanDocument::parse_json(LEGACY_PLAN.as_bytes()).unwrap();
        assert_eq!(
            serde_json::to_vec(&document).unwrap(),
            LEGACY_PLAN.as_bytes()
        );
        assert_eq!(
            document.binding_sha256().unwrap(),
            "e516fbc7a4799b2df4910af35fe898e6f037a38e1172462bebd43c451dfeae83"
        );
        assert_eq!(
            document.binding_sha256().unwrap(),
            axes().binding_sha256().unwrap()
        );
        let first = document.cell(0).unwrap();
        assert_eq!(first.origin, CaseOrigin::V1);
        let bytes = serde_json::to_vec(&first.config).unwrap();
        assert_eq!(bytes, LEGACY_CASE.as_bytes());
        assert_eq!(
            digest(&bytes),
            "27d0cb239c6b08ce29f90dde9b49a4de12272269c3338d52079042398c290f8c"
        );
        assert!(!first.config.twist);
        assert!(document.cell(1).unwrap().config.twist);
        assert!(document.cell(2).is_err());
    }

    #[test]
    fn reordered_pretty_v1_input_has_identical_typed_identity() {
        // Generic JSON is used only to manufacture a differently ordered test
        // input, never in production dispatch.
        let mut value = serde_json::to_value(axes()).unwrap();
        value["protocol_sha256"] = json!(null);
        let bytes = serde_json::to_vec_pretty(&value).unwrap();
        assert_ne!(bytes, LEGACY_PLAN.as_bytes());
        let document = PlanDocument::parse_json(&bytes).unwrap();
        assert_eq!(
            serde_json::to_vec(&document).unwrap(),
            LEGACY_PLAN.as_bytes()
        );
        assert_eq!(
            document.binding_sha256().unwrap(),
            axes().binding_sha256().unwrap()
        );
    }

    #[test]
    fn v2_policy_is_most_significant_axis_with_legacy_mixed_radix_unchanged() {
        let mut axes = axes();
        axes.field_sizes = vec![7, 11];
        axes.arities = vec![2, 3];
        axes.families = vec!["koblitz".into(), "binary".into(), "prime".into()];
        axes.solvers = vec!["enumerate".into(), "sat".into()];
        axes.curve_seeds = 2;
        axes.generators = 2;
        axes.keys = 2;
        let block = axes.cell_count().unwrap();
        assert_eq!(block, 384);
        let plan = DatasetPlanV2::new(
            axes.clone(),
            vec![FactorBasePolicy::LegacySmallXProjectedV1, signed()],
        )
        .unwrap();
        assert_eq!(plan.cell_count().unwrap(), 768);
        for policy_index in 0..2 {
            for within in 0..block {
                let index = policy_index * block + within;
                let spec = plan.cell(index).unwrap();
                assert_eq!(spec.config, axes.cell(within).unwrap());
                assert_eq!(spec.origin, CaseOrigin::V2);
                assert_eq!(spec.policy_index, policy_index);
                assert_eq!(spec.cell_index, index);
                assert_eq!(spec.axes_cell_index, within);
                assert_eq!(
                    spec.policy,
                    plan.factor_base_policies[policy_index as usize]
                );
            }
        }
        // These controls pin the actual historic inner solver/twist order.
        assert_eq!(plan.cell(0).unwrap().config.solver, "enumerate");
        assert!(!plan.cell(1).unwrap().config.twist);
        assert_eq!(plan.cell(1).unwrap().config.solver, "sat");
        assert!(plan.cell(2).unwrap().config.twist);
        assert_eq!(plan.cell(4).unwrap().config.m, 3);
        assert!(plan.cell(768).is_err());
    }

    #[test]
    fn explicit_legacy_only_v2_preserves_origin_and_distinct_plan_identity() {
        let old = PlanDocument::V1(axes());
        let new = PlanDocument::V2(
            DatasetPlanV2::new(axes(), vec![FactorBasePolicy::LegacySmallXProjectedV1]).unwrap(),
        );
        let a = old.cell(0).unwrap();
        let b = new.cell(0).unwrap();
        assert_eq!(a.config, b.config);
        assert_eq!(a.policy, b.policy);
        assert_eq!(a.origin, CaseOrigin::V1);
        assert_eq!(b.origin, CaseOrigin::V2);
        assert_ne!(a, b);
        assert_ne!(old.binding_sha256().unwrap(), new.binding_sha256().unwrap());
        let bytes = serde_json::to_vec(&new).unwrap();
        let parsed = PlanDocument::parse_json(&bytes).unwrap();
        assert_eq!(serde_json::to_vec(&parsed).unwrap(), bytes);
        assert_eq!(parsed.cell(0).unwrap().origin, CaseOrigin::V2);
    }

    #[test]
    fn exact_large_integer_axes_and_indices_survive_dispatch() {
        let mut plan = axes();
        plan.keys = 9_007_199_254_740_993;
        plan.seed = u64::MAX;
        let base = plan.cell_count().unwrap();
        assert_eq!(base, 18_014_398_509_481_986);
        let v2 = DatasetPlanV2::new(
            plan,
            vec![FactorBasePolicy::LegacySmallXProjectedV1, signed()],
        )
        .unwrap();
        let bytes = serde_json::to_vec(&v2).unwrap();
        let parsed = PlanDocument::parse_json(&bytes).unwrap();
        assert_eq!(parsed.axes().keys, 9_007_199_254_740_993);
        assert_eq!(parsed.axes().seed, u64::MAX);
        assert_eq!(parsed.cell_count().unwrap(), 36_028_797_018_963_972);
        let last = parsed.cell(base * 2 - 1).unwrap();
        assert_eq!(last.policy_index, 1);
        assert_eq!(last.axes_cell_index, base - 1);
        assert_eq!(last.config.key_index, 9_007_199_254_740_992);
        assert!(last.config.twist);
    }

    #[test]
    fn overflow_is_rejected_at_both_existing_and_policy_axes() {
        let mut plan = axes();
        plan.keys = u64::MAX / 2;
        assert_eq!(plan.cell_count().unwrap(), u64::MAX - 1);
        assert!(DatasetPlanV2::new(
            plan.clone(),
            vec![FactorBasePolicy::LegacySmallXProjectedV1, signed()]
        )
        .unwrap_err()
        .contains("overflow"));
        plan.keys = u64::MAX;
        assert!(PlanDocument::V1(plan.clone()).cell_count().is_err());
        assert!(DatasetPlanV2::new(plan, vec![signed()]).is_err());
    }

    #[test]
    fn canonical_policy_duplicates_are_rejected_regardless_of_json_key_order() {
        let signed_reordered = r#"{"max_points":4096,"max_artifact_bytes":67108864,"kind":"signed-frobenius-union/v1","max_action_steps":10000000,"max_candidates":131200}"#;
        let source = format!(
            r#"{{"schema":"crypto.ml-dataset.plan/v2","axes":{},"factor_base_policies":[{},{}]}}"#,
            LEGACY_PLAN,
            serde_json::to_string(&signed()).unwrap(),
            signed_reordered
        );
        assert!(PlanDocument::parse_json(source.as_bytes())
            .unwrap_err()
            .contains("duplicate canonical"));
        assert!(DatasetPlanV2::new(
            axes(),
            vec![
                FactorBasePolicy::LegacySmallXProjectedV1,
                FactorBasePolicy::LegacySmallXProjectedV1
            ]
        )
        .is_err());
        assert!(DatasetPlanV2::new(axes(), vec![]).is_err());
    }

    #[test]
    fn unknown_schema_and_fields_are_rejected_at_every_typed_level() {
        let source = serde_json::to_value(wrapper()).unwrap();
        let mut cases = Vec::new();
        let mut changed = source.clone();
        changed["extra"] = json!(1);
        cases.push(changed);
        let mut changed = source.clone();
        changed["axes"]["extra"] = json!(1);
        cases.push(changed);
        let mut changed = source.clone();
        changed["factor_base_policies"][0]["extra"] = json!(1);
        cases.push(changed);
        let mut changed = source.clone();
        changed["factor_base_policies"][1]["extra"] = json!(1);
        cases.push(changed);
        let mut changed = source.clone();
        changed["schema"] = json!("crypto.ml-dataset.plan/v3");
        cases.push(changed);
        let mut changed = source.clone();
        changed["axes"]["schema"] = json!(PLAN_V2_SCHEMA);
        cases.push(changed);
        let mut changed = source.clone();
        changed["factor_base_policies"][1]["kind"] = json!("unknown/v1");
        cases.push(changed);
        for value in cases {
            assert!(
                PlanDocument::parse_json(&serde_json::to_vec(&value).unwrap()).is_err(),
                "accepted {value}"
            );
        }
        let legacy_extra = LEGACY_PLAN.replacen('{', "{\"extra\":1,", 1);
        assert!(PlanDocument::parse_json(legacy_extra.as_bytes()).is_err());
    }

    #[test]
    fn duplicate_json_keys_are_not_lost_by_schema_dispatch() {
        let source = serde_json::to_string(&wrapper()).unwrap();
        for (needle, replacement) in [
            (
                r#""schema":"crypto.ml-dataset.plan/v2""#,
                r#""schema":"crypto.ml-dataset.plan/v2","schema":"crypto.ml-dataset.plan/v2""#,
            ),
            (r#""seed":20261030"#, r#""seed":20261030,"seed":20261030"#),
            (
                r#""kind":"legacy-small-x-projected/v1""#,
                r#""kind":"legacy-small-x-projected/v1","kind":"legacy-small-x-projected/v1""#,
            ),
            (
                r#""max_candidates":131200"#,
                r#""max_candidates":131200,"max_candidates":131200"#,
            ),
        ] {
            assert!(source.contains(needle));
            let duplicate = source.replacen(needle, replacement, 1);
            assert!(
                PlanDocument::parse_json(duplicate.as_bytes()).is_err(),
                "accepted duplicate: {duplicate}"
            );
        }
        let legacy_duplicate =
            LEGACY_PLAN.replace(r#""seed":20261030"#, r#""seed":20261030,"seed":20261030"#);
        assert!(PlanDocument::parse_json(legacy_duplicate.as_bytes()).is_err());
    }

    #[test]
    fn signed_parameters_are_explicit_integral_and_bounded() {
        signed().validate().unwrap();
        let source = serde_json::to_value(signed()).unwrap();
        for field in [
            "max_candidates",
            "max_points",
            "max_action_steps",
            "max_artifact_bytes",
        ] {
            let mut missing = source.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(serde_json::from_value::<FactorBasePolicy>(missing).is_err());
            for value in [json!(-1), json!(1.5), json!("2"), json!(null), json!(true)] {
                let mut changed = source.clone();
                changed[field] = value;
                assert!(serde_json::from_value::<FactorBasePolicy>(changed).is_err());
            }
        }
        for (field, invalid) in [
            ("max_candidates", 0),
            ("max_candidates", 1_000_001),
            ("max_points", 0),
            ("max_points", 4097),
            ("max_action_steps", 0),
            ("max_action_steps", 10_000_001),
            ("max_artifact_bytes", 4095),
            ("max_artifact_bytes", 67_108_865),
        ] {
            let mut changed = source.clone();
            changed[field] = json!(invalid);
            let policy: FactorBasePolicy = serde_json::from_value(changed).unwrap();
            assert!(policy.validate().is_err());
        }
        FactorBasePolicy::SignedFrobeniusUnionV1 {
            max_candidates: MAX_CANDIDATES,
            max_points: MAX_POINTS,
            max_action_steps: MAX_ACTION_STEPS,
            max_artifact_bytes: MAX_ARTIFACT_BYTES,
        }
        .validate_requested_points(MAX_POINTS)
        .unwrap();
        let smallest = FactorBasePolicy::SignedFrobeniusUnionV1 {
            max_candidates: 1,
            max_points: 1,
            max_action_steps: 1,
            max_artifact_bytes: 4096,
        };
        smallest.validate_requested_points(1).unwrap();
        assert!(smallest.validate_requested_points(0).is_err());
        assert!(smallest.validate_requested_points(2).is_err());
        assert!(DatasetPlanV2::new(axes(), vec![smallest]).is_err());
    }

    #[test]
    fn malformed_axes_fail_normalization_without_panics() {
        let mut cases = Vec::new();
        let mut plan = axes();
        plan.field_sizes.clear();
        cases.push(plan);
        let mut plan = axes();
        plan.arities.clear();
        cases.push(plan);
        let mut plan = axes();
        plan.families.clear();
        cases.push(plan);
        let mut plan = axes();
        plan.solvers.clear();
        cases.push(plan);
        let mut plan = axes();
        plan.keys = 0;
        cases.push(plan);
        let mut plan = axes();
        plan.curve_seeds = 0;
        cases.push(plan);
        let mut plan = axes();
        plan.generators = 0;
        cases.push(plan);
        for axes in cases {
            assert!(PlanDocument::V1(axes.clone()).cell(0).is_err());
            let v2 = DatasetPlanV2 {
                schema: PLAN_V2_SCHEMA.into(),
                axes,
                factor_base_policies: vec![signed()],
            };
            assert!(v2.cell(0).is_err());
            assert!(PlanDocument::V2(v2).cell_count().is_err());
        }
    }

    #[test]
    fn parser_rejects_malformed_or_trailing_json_and_oversized_input() {
        for invalid in ["{}", "[]", "null", "{", "{\"schema\":null}"] {
            assert!(PlanDocument::parse_json(invalid.as_bytes()).is_err());
        }
        let trailing = format!("{LEGACY_PLAN} {LEGACY_PLAN}");
        assert!(PlanDocument::parse_json(trailing.as_bytes()).is_err());
        let oversized = vec![b' '; JSON_INPUT_MAX_BYTES as usize + 1];
        assert!(PlanDocument::parse_json(&oversized)
            .unwrap_err()
            .contains("exceeds32MiB"));
    }
}
