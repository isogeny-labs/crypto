//! Exact model metadata only, derived from already verified immutable fixtures.
#![allow(dead_code)]
#[path = "../cold_20261029/arithmetic.rs"]
mod arithmetic;
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
mod toy_field {
    pub use crate::arithmetic::inv;
}
#[path = "../registry.rs"]
mod registry;
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, io::Write};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("models VERIFIED_FIXTURES FRESH_METADATA".into());
    }
    let input: Value = serde_json::from_slice(&fs::read(&a[0])?)?;
    let mut models = BTreeMap::new();
    let mut links = vec![];
    for group in ["development", "heldout", "sizing_only"] {
        for v in input[group].as_array().ok_or("fixture group")? {
            let f: arithmetic::Fixture = serde_json::from_value(v.clone())?;
            let mut slugs = vec![];
            for (role, e, n) in [("curve", f.curve(), f.order), ("twist", f.twist(), f.tn())] {
                let mut m = registry::identity(e.p, e.a, e.b, n as u128, None);
                m["sources"] = json!([
                    "research/twist_routes_1_3_20261021/execution-v15/FIXTURES.json",
                    "research/twist_routes_1_3_20261021/execution-v15/reference-v2/ADMISSION.json"
                ]);
                m["ec1_unresolved"]=json!("Exact prime field/model and independently checked whole order are recorded in the frozen fixture. Source subgroup/generator are explicit; twist discovery generator is method-specific. No cross-repository EC1 comparison or representation match is asserted.");
                let slug = m["slug"].as_str().unwrap().to_owned();
                slugs.push(json!({"role":role,"slug":slug}));
                models.entry(slug).or_insert(m);
            }
            links.push(json!({"fixture_id":f.id,"group":group,"role":f.role,"models":slugs}));
        }
    }
    let mut o = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[1])?;
    writeln!(
        o,
        "{}",
        serde_json::to_string_pretty(
            &json!({"schema":"twist.cold-model-metadata/v1","fixture_sha256":registry::digest(&fs::read(&a[0])?),"models":models.values().collect::<Vec<_>>(),"links":links,"new_arithmetic_outcomes":false})
        )?
    )?;
    Ok(())
}
