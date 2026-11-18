//! Native APS export support. Parsing/recomputation is not execution attestation.
pub mod audit;
pub mod inclusive;
pub mod package;
pub mod value;
pub use value::*;

use serde_json::Value;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const MAX_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const EXPORT_SCHEMA: &str = include_str!("../schemas/experiment-export.v1.json");
pub const PROTOCOL_SCHEMA: &str = include_str!("../schemas/experiment-protocol.v1.json");

pub fn validate_schema(v: &Value, protocol: bool) -> Result<()> {
    let s: Value = serde_json::from_str(if protocol {
        PROTOCOL_SCHEMA
    } else {
        EXPORT_SCHEMA
    })
    .map_err(|e| e.to_string())?;
    let validator = jsonschema::validator_for(&s).map_err(|e| e.to_string())?;
    validator
        .validate(v)
        .map_err(|e| format!("{}: {e}", e.instance_path))
}

/// All evidence must stay inside the explicitly selected root, including symlinks.
pub struct EvidenceRoot(PathBuf);
impl EvidenceRoot {
    pub fn new(p: &Path) -> Result<Self> {
        fs::canonicalize(p).map(Self).map_err(|e| e.to_string())
    }
    pub fn resolve(&self, name: &str) -> Result<PathBuf> {
        let p = Path::new(name);
        require(
            !p.is_absolute() && !p.components().any(|c| matches!(c, Component::ParentDir)),
            "evidence path must be relative without parent components",
        )?;
        let p = fs::canonicalize(self.0.join(p)).map_err(|e| format!("{name}: {e}"))?;
        require(p.starts_with(&self.0), "evidence symlink escapes root")?;
        require(p.is_file(), "evidence is not a regular file")?;
        Ok(p)
    }
    pub fn bytes(&self, name: &str) -> Result<Vec<u8>> {
        use std::io::Read;
        let p = self.resolve(name)?;
        require(
            fs::metadata(&p).map_err(|e| e.to_string())?.len() <= MAX_FILE_BYTES,
            "evidence exceeds byte bound",
        )?;
        let mut bytes = Vec::new();
        fs::File::open(p)
            .map_err(|e| e.to_string())?
            .take(MAX_FILE_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        require(
            bytes.len() as u64 <= MAX_FILE_BYTES,
            "evidence grew beyond byte bound",
        )?;
        Ok(bytes)
    }
    pub fn json(&self, name: &str) -> Result<Value> {
        serde_json::from_slice(&self.bytes(name)?).map_err(|e| format!("{name}: {e}"))
    }
    pub fn pin(&self, name: &str, kind: &str) -> Result<Value> {
        let bytes = self.bytes(name)?;
        Ok(serde_json::json!({"path":name,"sha256":hash(&bytes),"bytes":bytes.len(),"kind":kind}))
    }
}

/// Never overwrite a prior export or receipt.
pub fn write_new(p: &Path, v: &Value) -> Result<()> {
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .map_err(|e| e.to_string())?;
    let mut bytes = serde_json::to_vec_pretty(v).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    f.write_all(&bytes).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests;
