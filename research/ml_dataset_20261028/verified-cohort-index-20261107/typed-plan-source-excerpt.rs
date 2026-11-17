#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatasetPlan {
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "types::present_channels")]
    pub channels: Option<types::ChannelRequests>,
    pub schema: String,
    pub field_sizes: Vec<u32>,
    pub arities: Vec<usize>,
    pub families: Vec<String>,
    pub solvers: Vec<String>,
    pub curve_seeds: u64,
    pub generators: u64,
    pub keys: u64,
    pub seed: u64,
    pub factor_base_size: usize,
    pub degree_bound: u32,
    pub work_limit: u64,
    pub twists: bool,
    pub protocol_sha256: Option<String>,
    pub split_rule: String,
    /// Empty and absent for frozen plan/v1. Explicit nonempty policy axis for plan/v2.
    #[serde(
        default,
        deserialize_with = "nonempty_policy_axis",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub factor_base_policies: Vec<FactorBasePolicy>,
}

fn nonempty_policy_axis<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<FactorBasePolicy>, D::Error> {
    let policies = Vec::<FactorBasePolicy>::deserialize(deserializer)?;
    if policies.is_empty() || policies.len() > 32 {
        return Err(serde::de::Error::custom(
            "a present policy axis must contain 1..32 recipes",
        ));
    }
    Ok(policies)
}

impl DatasetPlan {
    /// Existing run/grant identity: compact serialization of this typed plan
    /// in struct field order, not generic canonical JSON or the saved file bytes.
    fn binding_sha256(&self) -> Result<String, String> {
        Ok(digest(
            &serde_json::to_vec(self).map_err(|e| e.to_string())?,
        ))
    }
