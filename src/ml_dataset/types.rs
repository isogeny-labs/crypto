use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A frozen, target-independent factor-base recipe. The legacy absent field
/// remains distinct from an explicit versioned legacy policy.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum FactorBasePolicy {
    // A zero-field struct keeps the same JSON as a unit variant while ensuring
    // serde applies deny_unknown_fields to the recipe payload.
    Legacy {},
    SampledXProjectedControl {
        candidate_limit: u64,
    },
    BinarySubspaceIntersection {
        basis: Vec<u64>,
        candidate_limit: u64,
    },
    BinaryFrobeniusUnionIntersection {
        basis: Vec<u64>,
        candidate_limit: u64,
    },
}

impl FactorBasePolicy {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Legacy {} => "legacy",
            Self::SampledXProjectedControl { .. } => "sampled_x_projected_control",
            Self::BinarySubspaceIntersection { .. } => "binary_subspace_intersection",
            Self::BinaryFrobeniusUnionIntersection { .. } => "binary_frobenius_union_intersection",
        }
    }

    pub fn validate(&self, n: u32) -> Result<(), String> {
        if !(3..=61).contains(&n) {
            return Err("factor-base policy field size must be 3..61".into());
        }
        let (basis, limit) = match self {
            Self::Legacy {} => return Ok(()),
            Self::SampledXProjectedControl { candidate_limit } => (None, *candidate_limit),
            Self::BinarySubspaceIntersection {
                basis,
                candidate_limit,
            }
            | Self::BinaryFrobeniusUnionIntersection {
                basis,
                candidate_limit,
            } => (Some(basis), *candidate_limit),
        };
        if !(1..=65_536).contains(&limit) {
            return Err("factor-base candidate limit must be 1..65536".into());
        }
        if let Some(basis) = basis {
            if basis.is_empty() || basis.len() > 12 || basis.len() > n as usize {
                return Err("binary basis dimension must be 1..min(n,12)".into());
            }
            let mut pivots = [0u64; 61];
            for &value in basis {
                if value == 0 || value >= 1u64 << n {
                    return Err("binary basis coordinate is zero or outside field".into());
                }
                let mut row = value;
                while row != 0 {
                    let i = 63 - row.leading_zeros() as usize;
                    if pivots[i] == 0 {
                        pivots[i] = row;
                        break;
                    }
                    row ^= pivots[i];
                }
                if row == 0 {
                    return Err("binary basis is linearly dependent".into());
                }
            }
        }
        Ok(())
    }
}

/// Frozen opt-in request; absence preserves legacy bytes and identities.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ChannelRequests {
    pub schema: String,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present_boolean")]
    pub boolean_consequence: Option<BooleanRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present_sat")]
    pub sat_initial: Option<SatRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present_original_row")]
    pub affine_boolean_row: Option<AffineBooleanRowRequest>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum BooleanRequest { Produce { row_index: u32 } }
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum SatRequest { CaptureAndCheckInitial {} }
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag="mode", rename_all="snake_case", deny_unknown_fields)]
pub enum AffineBooleanRowRequest { AffineUnitRow { pattern:u32, row_index:u32 } }
fn present_original_row<'de,D:serde::Deserializer<'de>>(d:D)->Result<Option<AffineBooleanRowRequest>,D::Error>{
    AffineBooleanRowRequest::deserialize(d).map(Some)
}
fn present_boolean<'de,D:serde::Deserializer<'de>>(d:D)->Result<Option<BooleanRequest>,D::Error>{
    BooleanRequest::deserialize(d).map(Some)
}
fn present_sat<'de,D:serde::Deserializer<'de>>(d:D)->Result<Option<SatRequest>,D::Error>{
    SatRequest::deserialize(d).map(Some)
}
pub(super) fn present_channels<'de,D:serde::Deserializer<'de>>(d:D)->Result<Option<ChannelRequests>,D::Error>{
    let c=ChannelRequests::deserialize(d)?;
    c.validate().map_err(serde::de::Error::custom)?;
    Ok(Some(c))
}
impl ChannelRequests {
    pub fn validate(&self)->Result<(),String>{
        if self.schema == "crypto.ml-dataset.channel-requests/v3" {
            if self.affine_boolean_row.is_none() || self.boolean_consequence.is_some() || self.sat_initial.is_some() {
                return Err("v3 requires only affine_boolean_row".into());
            }
            return Ok(());
        }
        if self.affine_boolean_row.is_some() {return Err("original row requires explicit v3".into())}
        if !matches!(self.schema.as_str(), "crypto.ml-dataset.channel-requests/v1" | "crypto.ml-dataset.channel-requests/v2") || (self.boolean_consequence.is_none() && self.sat_initial.is_none())
            || (self.schema == "crypto.ml-dataset.channel-requests/v2" && self.sat_initial.is_none()) {
            return Err("unknown or empty channel request".into());
        }
        if matches!(self.boolean_consequence,Some(BooleanRequest::Produce{row_index}) if row_index>7) {
            return Err("ordinary N3 retained row index must be 0..7".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CaseConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "present_channels")]
    pub channels: Option<ChannelRequests>,
    pub family: String,
    pub n: u32,
    pub m: usize,
    pub twist: bool,
    pub curve_seed: u64,
    pub generator_index: u64,
    pub key_index: u64,
    pub seed: u64,
    pub factor_base_size: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor_base_policy: Option<FactorBasePolicy>,
    pub degree_bound: u32,
    pub work_limit: u64,
    pub solver: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CaseData {
    pub public_instance: Value,
    pub private_witness: Value,
    pub factor_base: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub role: String,
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
    pub encoding: String,
}

#[derive(Clone, Debug)]
pub struct ArtifactPayload {
    pub path: String,
    pub role: String,
    pub encoding: String,
    pub bytes: Vec<u8>,
}

impl ArtifactPayload {
    pub fn json<T: Serialize>(path: &str, role: &str, value: &T) -> Result<Self, String> {
        let mut bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
        bytes.push(b'\n');
        Ok(Self {
            path: path.into(),
            role: role.into(),
            encoding: "json".into(),
            bytes,
        })
    }
}

#[derive(Clone, Debug)]
pub struct SolverData {
    pub outcome: Value,
    pub artifacts: Vec<ArtifactPayload>,
}

#[cfg(test)]
mod policy_tests {
    use super::*;
    #[test]
    fn policy_bounds_and_independence_are_checked() {
        for basis in [vec![], vec![0], vec![1, 1], vec![1, 2, 3], vec![32]] {
            assert!(FactorBasePolicy::BinarySubspaceIntersection {
                basis,
                candidate_limit: 8
            }
            .validate(5)
            .is_err());
        }
        assert!(FactorBasePolicy::BinarySubspaceIntersection {
            basis: vec![1, 2, 4, 8, 16],
            candidate_limit: 32
        }
        .validate(5)
        .is_ok());
        for candidate_limit in [0, 65_537, u64::MAX] {
            assert!(
                FactorBasePolicy::SampledXProjectedControl { candidate_limit }
                    .validate(61)
                    .is_err()
            );
        }
        assert!(FactorBasePolicy::Legacy {}.validate(2).is_err());
        assert!(FactorBasePolicy::Legacy {}.validate(62).is_err());
        assert!(FactorBasePolicy::BinarySubspaceIntersection {
            basis: vec![1 << 60],
            candidate_limit: 1
        }
        .validate(61)
        .is_ok());
    }
    #[test]
    fn policy_serde_is_tagged_and_strict() {
        for (good, name) in [
            (r#"{"kind":"legacy"}"#, "legacy"),
            (
                r#"{"kind":"sampled_x_projected_control","candidate_limit":32}"#,
                "sampled_x_projected_control",
            ),
            (
                r#"{"kind":"binary_subspace_intersection","basis":[1,2],"candidate_limit":32}"#,
                "binary_subspace_intersection",
            ),
            (
                r#"{"kind":"binary_frobenius_union_intersection","basis":[1,2],"candidate_limit":32}"#,
                "binary_frobenius_union_intersection",
            ),
        ] {
            let p: FactorBasePolicy = serde_json::from_str(good).unwrap();
            assert_eq!(p.name(), name);
            assert_eq!(serde_json::to_string(&p).unwrap(), good);
            let mut unknown: Value = serde_json::from_str(good).unwrap();
            unknown["extra"] = Value::from(1);
            assert!(serde_json::from_value::<FactorBasePolicy>(unknown).is_err());
        }
        for bad in [
            r#"{"kind":"legacy","extra":1}"#,
            r#"{"kind":"unknown"}"#,
            r#"{"kind":"sampled_x_projected_control","candidate_limit":true}"#,
        ] {
            assert!(serde_json::from_str::<FactorBasePolicy>(bad).is_err());
        }
    }
    #[test]
    fn absent_policy_preserves_legacy_config_bytes() {
        let legacy = r#"{"family":"prime","n":23,"m":2,"twist":false,"curve_seed":7,"generator_index":1,"key_index":2,"seed":42,"factor_base_size":8,"degree_bound":3,"work_limit":1000,"solver":"enumerate"}"#;
        let parsed: CaseConfig = serde_json::from_str(legacy).unwrap();
        assert!(parsed.factor_base_policy.is_none());
        assert_eq!(serde_json::to_string(&parsed).unwrap(), legacy);
        let mut explicit = parsed;
        explicit.factor_base_policy = Some(FactorBasePolicy::Legacy {});
        assert!(serde_json::to_string(&explicit)
            .unwrap()
            .contains("factor_base_policy"));
    }
}

/// Request disposition only; execution and arithmetic admission are separate receipts.
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum ChannelDispositionStatus { Requested, Unsupported }
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum ChannelDispositionReason {
    RequiresActualOriginalShapeAndRetainedRow, OutsideOrdinaryN3Profile,
    SatCentralWorkerReplayNotIntegrated,
    RequiresActualSatInitialReplay, OutsideSatInitialProfile,
    RequiresActualAffineUnitRow, OutsideAffineUnitRowProfile,
}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct ChannelDisposition<T> {
    pub request:T, pub status:ChannelDispositionStatus, pub reason:ChannelDispositionReason,
    pub execution_verified:bool, pub training_export_admitted:bool,
}
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct RequestedChannelDispositions {
    pub schema:String,
    pub boolean_consequence:Option<ChannelDisposition<BooleanRequest>>,
    pub sat_initial:Option<ChannelDisposition<SatRequest>>,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub affine_boolean_row:Option<ChannelDisposition<AffineBooleanRowRequest>>,
}
#[cfg(test)]
mod channel_tests {
    use super::*;
    #[derive(Deserialize,Serialize)]
    #[serde(deny_unknown_fields)]
    struct Envelope {
        #[serde(default,skip_serializing_if="Option::is_none",deserialize_with="present_channels")]
        channels:Option<ChannelRequests>,
    }
    #[test]
    fn strict_absence_null_duplicates_and_versions() {
        let absent:Envelope=serde_json::from_str("{}").unwrap();
        assert_eq!(serde_json::to_string(&absent).unwrap(),"{}");
        for bad in [
            r#"{"channels":null}"#,
            r#"{"channels":{"schema":"crypto.ml-dataset.channel-requests/v1"}}"#,
            r#"{"channels":{"schema":"crypto.ml-dataset.channel-requests/v1","sat_initial":null}}"#,
            r#"{"channels":{"schema":"crypto.ml-dataset.channel-requests/v3","sat_initial":{"mode":"capture_and_check_initial"}}}"#,
            r#"{"channels":{"schema":"crypto.ml-dataset.channel-requests/v1","boolean_consequence":{"mode":"produce","row_index":8}}}"#,
            r#"{"channels":{"schema":"crypto.ml-dataset.channel-requests/v1","boolean_consequence":{"mode":"produce","row_index":0,"row_index":1}}}"#,
            r#"{"channels":{"schema":"crypto.ml-dataset.channel-requests/v1","sat_initial":{"mode":"capture_and_check_initial","extra":true}}}"#,
        ] { assert!(serde_json::from_str::<Envelope>(bad).is_err(),"{bad}"); }
        let c:Envelope=serde_json::from_str(r#"{"channels":{"schema":"crypto.ml-dataset.channel-requests/v1","boolean_consequence":{"mode":"produce","row_index":0},"sat_initial":{"mode":"capture_and_check_initial"}}}"#).unwrap();
        c.channels.unwrap().validate().unwrap();
        let activated:Envelope=serde_json::from_str(r#"{"channels":{"schema":"crypto.ml-dataset.channel-requests/v2","sat_initial":{"mode":"capture_and_check_initial"}}}"#).unwrap();
        assert_eq!(activated.channels.unwrap().schema,"crypto.ml-dataset.channel-requests/v2");
        assert!(serde_json::from_str::<Envelope>(r#"{"channels":{"schema":"crypto.ml-dataset.channel-requests/v2","boolean_consequence":{"mode":"produce","row_index":0}}}"#).is_err());
    }
}
