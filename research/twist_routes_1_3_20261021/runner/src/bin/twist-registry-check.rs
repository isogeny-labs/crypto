//! Validate every added model with the repository's canonical Rust constructor.
#![allow(dead_code)]
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{collections::BTreeMap, error::Error, fs, path::Path};
#[path = "../../../../../src/hash/sha256.rs"]
pub mod source_sha256;
mod hash {
    pub use crate::source_sha256 as sha256;
}
// curve_id only reads these two representation fields; no arithmetic backend
// is substituted. Prime/extension identity construction is the actual source.
mod binary_ecc {
    pub struct IrreduciblePoly {
        pub degree: u32,
        pub low_terms: Vec<u32>,
    }
}
#[path = "../../../../../src/cryptanalysis/curve_id.rs"]
mod curve_id;
fn big(v: &Value) -> BigUint {
    v.as_str().unwrap().parse().unwrap()
}
fn word(v: &Value) -> u64 {
    v.as_str().unwrap().parse().unwrap()
}
fn vector(v: &Value) -> Vec<u64> {
    v.as_array().unwrap().iter().map(word).collect()
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    let root = Path::new(args.get(1).ok_or("repository root")?);
    let registry: Value =
        serde_json::from_slice(&fs::read(root.join("docs/curves/registry.json"))?)?;
    let registered: BTreeMap<_, _> = registry["curves"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| (v["slug"].as_str().unwrap(), v))
        .collect();
    let mut models = BTreeMap::new();
    for path in ["research/twist_routes_1_3_20261021/execution-v1/registry-additions.json","research/twist_routes_1_3_20261021/execution-v1/runs/isogeny-20261021T2320Z/E14-registry-additions.json","research/twist_routes_1_3_20261021/execution-v1/pairing-fixture/registry-additions.json"] {
        let v:Value=serde_json::from_slice(&fs::read(root.join(path))?)?;
        for row in v.as_array().unwrap(){models.insert(row["slug"].as_str().unwrap().to_string(),row.clone());}
    }
    let mut checked = vec![];
    let mut extensions = 0;
    for (slug, row) in &models {
        let p = &row["params"];
        let order = big(&row["order"]);
        let id = if row["family"] == "prime" {
            curve_id::prime(&big(&p["p"]), &big(&p["a"]), &big(&p["b"]), &order)
        } else {
            extensions += 1;
            curve_id::extension(
                word(&p["p"]),
                &vector(&p["modulus"]),
                &vector(&p["a"]),
                &vector(&p["b"]),
                &order,
            )
        }
        .ok_or("invalid canonical model")?;
        assert_eq!(&id.slug, slug);
        assert_eq!(id.icv1, row["icv1"].as_str().unwrap());
        assert_eq!(id.model_json, row["model_json"].as_str().unwrap());
        let record = registered
            .get(slug.as_str())
            .ok_or("model absent from registry")?;
        assert_eq!(record["icv1"], row["icv1"]);
        assert_eq!(record["model_json"], row["model_json"]);
        assert_eq!(curve_id::resolve(slug), Some(slug.as_str()));
        assert_eq!(curve_id::resolve(&id.icv1), Some(slug.as_str()));
        checked.push(json!({"slug":slug,"canonical_constructor_matches":true,"registry_matches":true,"alias_and_ICV1_resolve":true}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"schema":"twist.registry-check/v1","added_models":models.len(),"extension_models_added":extensions,"all_checked":true,"checks":checked})
        )?
    );
    Ok(())
}
