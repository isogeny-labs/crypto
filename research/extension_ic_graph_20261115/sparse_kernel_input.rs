//! Strict matrix-only boundary: source answers and rank hints never cross it.
use crate::sparse_kernel::Matrix;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Corpus {
    pub name: String,
    pub origin_sha256: String,
    pub matrix: Matrix,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub schema: String,
    pub origin_revision: String,
    pub corpora: Vec<Corpus>,
}
pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn parse(bytes: &[u8]) -> Result<Input, String> {
    let input: Input = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    if input.schema != "cover.sparse-kernel.matrices/v1"
        || input.origin_revision != "fcda1e22cb0c5424ca3244f783e69be8b99792f4"
        || input.corpora.len() != 2
    {
        return Err("input registration".into());
    }
    for (i, c) in input.corpora.iter().enumerate() {
        if c.name != ["full-smooth", "quadratic-remainders"][i]
            || c.origin_sha256.len() != 64
            || !c.origin_sha256.bytes().all(|b| b.is_ascii_hexdigit())
            || c.matrix.modulus != 263807419614139553
            || c.matrix.columns != 509
            || c.matrix.rows.len() != [509, 510][i]
        {
            return Err("corpus registration".into());
        }
        c.matrix.validate()?;
    }
    Ok(input)
}
pub fn create_json(path: &Path, value: &impl Serialize) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    serde_json::to_writer(&mut f, value)?;
    f.write_all(b"\n")?;
    Ok(())
}
pub fn prepare(paths: &[String], output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if paths.len() != 2 {
        return Err("two original inputs required".into());
    }
    let mut corpora = Vec::new();
    for (i, path) in paths.iter().enumerate() {
        let bytes = fs::read(path)?;
        let original: serde_json::Value = serde_json::from_slice(&bytes)?;
        if original["source_revision"] != "fcda1e22cb0c5424ca3244f783e69be8b99792f4"
            || original["method"] != ["extension-full-smooth", "extension-remainders"][i]
        {
            return Err("original input identity".into());
        }
        let c = &original["work"]["collection"];
        // Only these three mathematical fields are copied. No former h,
        // basis, affine rows, rank, products, target, or scalar is retained.
        let matrix = Matrix {
            modulus: c["l"].as_u64().ok_or("modulus")?,
            columns: c["base"].as_array().ok_or("base")?.len(),
            rows: c["projections"]
                .as_array()
                .ok_or("projections")?
                .iter()
                .map(|p| serde_json::from_value(p["terms"].clone()))
                .collect::<Result<_, _>>()?,
        };
        corpora.push(Corpus {
            name: ["full-smooth", "quadratic-remainders"][i].into(),
            origin_sha256: digest(&bytes),
            matrix,
        });
    }
    let input = Input {
        schema: "cover.sparse-kernel.matrices/v1".into(),
        origin_revision: "fcda1e22cb0c5424ca3244f783e69be8b99792f4".into(),
        corpora,
    };
    let bytes = serde_json::to_vec(&input)?;
    parse(&bytes)?;
    create_json(output, &input)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn answer_and_nested_rank_keys_are_rejected() {
        let base = serde_json::json!({"schema":"cover.sparse-kernel.matrices/v1","origin_revision":"fcda1e22cb0c5424ca3244f783e69be8b99792f4","corpora":[],"kernel":[1]});
        assert!(super::parse(&serde_json::to_vec(&base).unwrap()).is_err());
        assert!(serde_json::from_value::<super::Corpus>(serde_json::json!({"name":"a","origin_sha256":"b","matrix":{"modulus":101,"columns":2,"rows":[[[0,1],[1,1]]],"rank":1}})).is_err());
    }
}
