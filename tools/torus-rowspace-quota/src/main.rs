//! Metadata record counts only; no field arithmetic or rank/solver calls.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{env, fs, path::Path};
type Result<T> = std::result::Result<T, String>;
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn sources() -> Value {
    json!({"Cargo.toml":sha(include_bytes!("../Cargo.toml")),"Cargo.lock":sha(include_bytes!("../Cargo.lock")),"src/main.rs":sha(include_bytes!("main.rs"))})
}
fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().ok_or("array missing/invalid".into())
}
fn add(a: u64, b: usize) -> Result<u64> {
    a.checked_add(b as u64).ok_or("count overflow".into())
}
fn count(v: &Value) -> Result<Value> {
    if v["schema"] != "torus.rowspace-audit/v1" {
        return Err("audit schema".into());
    }
    let mut total = 0u64;
    let mut phases = serde_json::Map::new();
    for name in ["projection", "baseline", "combined"] {
        let mut terms = 0u64;
        let mut weights = 0u64;
        for step in array(&v[name]["steps"])? {
            terms = add(terms, array(&step["row"])?.len())?;
            weights = add(weights, array(&step["weights"])?.len())?;
        }
        let phase = terms.checked_add(weights).ok_or("count overflow")?;
        total = total.checked_add(phase).ok_or("count overflow")?;
        phases.insert(
            name.into(),
            json!({"terms":terms,"origin_records":weights,"entries":phase}),
        );
    }
    let mut images = 0u64;
    for image in array(&v["images"])? {
        images = add(images, array(image)?.len())?;
    }
    total = total.checked_add(images).ok_or("count overflow")?;
    Ok(
        json!({"rank_phases":phases,"image_terms":images,"whole_Audit_represented_entries":total,"under_1million_entries":total<=1_000_000}),
    )
}
fn run() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.get(1).is_some_and(|s| s == "--identity") {
        println!("{}", sources());
        return Ok(());
    }
    if args.len() != 3 {
        return Err("expected INPUT_DIR NEW_OUTPUT_DIR".into());
    }
    let input = Path::new(&args[1]);
    let output = Path::new(&args[2]);
    fs::create_dir(output).map_err(|e| e.to_string())?;
    let ids = [
        "constant-conflict",
        "existing-input",
        "new-quadratic",
        "original-input204",
        "original-raw12",
        "private-auxiliary",
        "zero-image",
    ];
    if fs::read_dir(input).map_err(|e| e.to_string())?.count() != ids.len() {
        return Err("exact seven input files required".into());
    }
    let mut results = vec![];
    for id in ids {
        let bytes = fs::read(input.join(format!("{id}.json"))).map_err(|e| e.to_string())?;
        if bytes.len() > 32 << 20 {
            return Err("metadata input byte cap".into());
        }
        let parsed: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        results.push(json!({"id":id,"input_bytes":bytes.len(),"input_sha256":sha(&bytes),"counts":count(&parsed)?}));
    }
    let record = json!({"schema":"rowspace.quota-reader/v1","compiled_sources":sources(),"metadata_cases":results,"old_rank_or_solver_calls":0,"standing":"producer unsigned/unverified"});
    fs::write(
        output.join("counts.json"),
        serde_json::to_vec_pretty(&record).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_and_refuse() {
        let v = json!({"schema":"torus.rowspace-audit/v1","projection":{"steps":[{"row":[1,2],"weights":[1]}]},"baseline":{"steps":[{"row":[],"weights":[1]}]},"combined":{"steps":[]},"images":[[1]]});
        assert_eq!(count(&v).unwrap()["whole_Audit_represented_entries"], 5);
        let mut bad = v.clone();
        bad["projection"]["steps"][0]["weights"] = Value::Null;
        assert!(count(&bad).is_err());
        let mut bad = v;
        bad["schema"] = json!("wrong");
        assert!(count(&bad).is_err());
    }
}
