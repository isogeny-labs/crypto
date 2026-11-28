//! Exact compact copy of the already selected first32 recorded systems.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write};
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 3 {
        return Err("usage: COMPACT ORIGINAL_RAW NEW_FIXTURE NEW_SHA256".into());
    }
    let bytes = fs::read(&a[0])?;
    let parent = hash(&bytes);
    if parent != "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d" {
        return Err("recorded parent".into());
    }
    let w: Value = serde_json::from_slice(&bytes)?;
    let a0 = w["work"]["nagao_attempts"].as_array().ok_or("attempts")?;
    if a0.len() != 3415 {
        return Err("parent cohort".into());
    }
    let mut systems = vec![];
    let mut h = Sha256::new();
    let mut count = 0;
    for (i, t) in a0.iter().take(32).enumerate() {
        if t["residual_index"] != json!(i) {
            return Err("source prefix".into());
        }
        let mut roots: Vec<Vec<u64>> = serde_json::from_value(t["audit"]["lifted_roots"].clone())?;
        roots.sort();
        h.update(serde_json::to_vec(&(i, &roots))?);
        count += roots.len();
        systems.push(json!({"residual_index":i,"audit":{"original":t["audit"]["original"],"lifted_roots":roots,"reduced_sha256":t["audit"]["reduced_sha256"]}}));
    }
    let root_sha = format!("{:x}", h.finalize());
    if count != 34 || root_sha != "7c09ce05178a16e3b27c3c3c07115c6a4c021ef9c7e15c12ddefb1aae5c05e4f"
    {
        return Err("unchanged coordinate subset".into());
    }
    let fixture = json!({"schema":"cover.unit.compact-coordinate-input/v1","parent_input_sha256":parent,"p":w["public"]["p"],"l":w["public"]["l"],
        "seed":389,"systems":systems,"coordinate_sha256":root_sha,"selection":"same first32 systems used by the original profiles"});
    let encoded = serde_json::to_vec_pretty(&fixture)?;
    let mut out = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&a[1])?;
    out.write_all(&encoded)?;
    let mut digest = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&a[2])?;
    writeln!(digest, "{}", hash(&encoded))?;
    println!(
        "Exact compact fixture:32 systems,34 roots,{} bytes",
        encoded.len()
    );
    Ok(())
}
