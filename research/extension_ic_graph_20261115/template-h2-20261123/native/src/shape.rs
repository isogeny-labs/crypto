//! Read-only structure analysis of already captured polynomial bases; no solve.
use nagao_template::Poly;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    env, fs,
    io::Write,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn order(a: &[u32], b: &[u32]) -> Ordering {
    a.iter()
        .sum::<u32>()
        .cmp(&b.iter().sum::<u32>())
        .then_with(|| b.iter().rev().cmp(a.iter().rev()))
}
fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        return Err("usage: SAVED_BASES NEW_OUTPUT_JSON".into());
    }
    let bytes = fs::read(&args[1])?;
    let digest = format!("{:x}", Sha256::digest(&bytes));
    if digest != "e63a6126b3309bc7e387347f4f8629d78f12897a7c55c928b7152e75c21e76f1" {
        return Err("original hash mismatch".into());
    }
    let mut cases = Vec::new();
    for (index, line) in std::str::from_utf8(&bytes)?.lines().take(2).enumerate() {
        let row: Value = serde_json::from_str(line)?;
        if row["index"] != index || row["variables"] != 6 {
            return Err("original shape identity".into());
        }
        let basis: Vec<Poly> = serde_json::from_value(row["basis"].clone())?;
        let leads = basis
            .iter()
            .map(|f| {
                f.iter()
                    .filter(|(_, c)| *c != 0)
                    .map(|(m, _)| m.clone())
                    .max_by(|a, b| order(a, b))
                    .ok_or("empty basis polynomial")
            })
            .collect::<std::result::Result<BTreeSet<_>, _>>()?;
        let minimal: Vec<_> = leads
            .iter()
            .filter(|m| {
                !leads
                    .iter()
                    .any(|other| other != *m && other.iter().zip(m.iter()).all(|(a, b)| a <= b))
            })
            .cloned()
            .collect();
        let mut histogram = BTreeMap::<u32, usize>::new();
        for m in &minimal {
            *histogram.entry(m.iter().sum()).or_default() += 1;
        }
        cases.push(json!({"case":index,"basis_polynomials":basis.len(),"minimal_leading_monomials":minimal.len(),
            "minimal_leading_degree_histogram":histogram,"max_minimal_leading_degree":histogram.keys().max(),
            "required_minimal_leads":minimal}));
    }
    let result = json!({"schema":"nagao-saved-reference-shape/v1","input_sha256":digest,
        "cases":cases,"scope":"captured-reference structure; no solver, target, random draw or timing"});
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    writeln!(file, "{}", serde_json::to_string_pretty(&result)?)?;
    file.sync_all()?;
    println!("{result}");
    Ok(())
}
