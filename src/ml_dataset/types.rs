use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CaseConfig {
    pub family: String,
    pub n: u32,
    pub m: usize,
    pub twist: bool,
    pub curve_seed: u64,
    pub generator_index: u64,
    pub key_index: u64,
    pub seed: u64,
    pub factor_base_size: usize,
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
