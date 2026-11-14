//! `check`: local, after fetch. Compares a pod-side `verify.json` (public data and solver
//! output only) with the sealed `hidden.json`: classifier consistency, positive and negative
//! controls, and the exporter's variety-size gate (msolve vs native ordered count).
use crate::util::*;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::Path;

fn syms_of(signed: &Value) -> Result<Vec<u8>, String> {
    let mut v: Vec<u8> = signed
        .as_array()
        .ok_or("signed_base")?
        .iter()
        .map(|x| {
            let s = x.as_i64().unwrap_or(0);
            2 * (s.unsigned_abs() as u8 - 1) + u8::from(s < 0)
        })
        .collect();
    v.sort_unstable();
    Ok(v)
}
fn flip(s: &[u8]) -> Vec<u8> {
    let mut v: Vec<u8> = s.iter().map(|k| k ^ 1).collect();
    v.sort_unstable();
    v
}

pub fn run(args: &[String]) -> Result<(), String> {
    let hidden = read_json(Path::new(&crate::arg_req(args, "--hidden")?))?;
    let ver_path = crate::arg_req(args, "--verify")?;
    let ver = read_json(Path::new(&ver_path))?;
    if !check_seal(&hidden) || !check_seal(&ver) {
        return Err("seal mismatch".into());
    }
    if hidden["cell_sha256"] != ver["cell_sha256"] {
        return Err("hidden.json and verify.json belong to different cells".into());
    }
    let name = ver["target"].as_str().ok_or("target")?;
    let label = ver["label"].as_str().unwrap_or("");
    let ht = hidden["targets"]
        .as_array()
        .ok_or("targets")?
        .iter()
        .find(|t| t["name"].as_str() == Some(name))
        .ok_or("target absent from hidden.json")?;
    let classifier: BTreeSet<Vec<u8>> = ht["decompositions"]
        .as_array()
        .ok_or("decompositions")?
        .iter()
        .map(|d| syms_of(&d["signed_base"]))
        .collect::<Result<_, _>>()?;
    let ext = &ver["extraction"];
    let found: BTreeSet<Vec<u8>> = ext["decompositions_signed_base"]
        .as_array()
        .ok_or("no decompositions in verify.json")?
        .iter()
        .map(syms_of)
        .collect::<Result<_, _>>()?;
    let missing = classifier.difference(&found).count();
    let extra = found.difference(&classifier).count();
    let planted_ok = ht["planted_symbols"].as_array().map(|_| {
        let ps = syms_of(&ht["planted_symbols"]).unwrap_or_default();
        found.contains(&ps) || found.contains(&flip(&ps))
    });
    let completed = ver["msolve_outcome"] == "zero_dimensional";
    let native_ordered = ext["public_counts_from_targets_json"]["ordered_algebraic_solutions"].as_u64();
    let no_sol = ver["msolve_outcome"] == "no_solution_in_closure";
    let distinct = if no_sol { Some(0) } else { ver["eliminating_degree"].as_u64() };
    let q = if no_sol { Some(0) } else { ver["quotient_dimension_with_multiplicity"].as_u64() };
    let rec = json!({
        "schema": "pkm.m8-panel.check/v1",
        "target": name, "label": label, "verify": ver_path, "verify_sha256": ver["sha256"],
        "hidden_sha256": hidden["sha256"],
        "msolve_outcome": ver["msolve_outcome"],
        "consistent_with_classifier": extra == 0,
        "classifier_decompositions": classifier.len(),
        "solver_decompositions": found.len(),
        "classifier_decompositions_missing_from_solver": missing,
        "solver_decompositions_not_in_classifier": extra,
        "negative_control_ok": if label == "unsat" { Some(found.is_empty()) } else { None },
        "positive_control_planted_recovered": planted_ok,
        "all_real_decompositions_recovered": completed.then_some(missing == 0),
        "variety_gate": {
            "native_ordered_algebraic_solutions": native_ordered,
            "msolve_distinct_solutions": distinct,
            "msolve_quotient_dimension": q,
            "msolve_ge_native": match (distinct, native_ordered) { (Some(d), Some(n)) => Some(d >= n), _ => None },
            "equal": match (distinct, native_ordered) { (Some(d), Some(n)) => Some(d == n), _ => None },
            "note": "no-solution outcome counts as 0 distinct solutions",
        },
    });
    let out = crate::arg_opt(args, "--out").unwrap_or_else(|| ver_path.replace(".json", ".check.json"));
    write_json(Path::new(&out), &seal(rec.clone()))?;
    println!("check {name}: {}", serde_json::to_string(&rec["variety_gate"]).unwrap_or_default());
    Ok(())
}
