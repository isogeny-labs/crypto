//! Lossless metadata successor to preserved v1; rejected v2 stays unchanged.
//! The inherited CSV uses replicate0 for both declared public-target seeds.
//! Canonical execution-cell keys omit seed, so represent the protocol's two
//! seed workloads as ordinal slots0/1. No solver or cryptographic replay runs.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn check(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn canonicalize(mut export: Value, created: &str) -> Result<Value> {
    check(export["schema"] == "experiment.export/v1", "export schema")?;
    check(
        export["flow"]["planned"] == 24 && export["flow"]["completed"] == 24,
        "complete24-call flow required",
    )?;
    let runs = export["runs"].as_array_mut().ok_or("missing runs")?;
    check(runs.len() == 24, "24 rows required")?;
    let mut keys = BTreeSet::new();
    for run in runs {
        let arm = run["arm"].as_str().ok_or("arm")?.to_owned();
        let case = run["instance_id"]
            .as_str()
            .ok_or("case")?
            .parse::<usize>()?;
        let seed = run["seed"].as_u64().ok_or("seed")?;
        check(
            case < 6 && ["u", "x"].contains(&arm.as_str()),
            "cell identity",
        )?;
        let ordinal = match seed {
            101 => 0u64,
            202 => 1u64,
            _ => return Err("unexpected seed".into()),
        };
        check(
            run["replicate"] == 0 && run["warmup"] == false,
            "expected raw native replicate0",
        )?;
        check(
            run["status"] == "succeeded"
                && run["outcome_class"] == "completed"
                && run["verification"]["status"] == "verified",
            "verified complete row required",
        )?;
        check(
            keys.insert((arm, case, ordinal)),
            "duplicate canonical cell",
        )?;
        let counters = run["measurements"]["counters"]
            .as_object_mut()
            .ok_or("native counters")?;
        check(
            !counters.contains_key("native_replicate"),
            "already canonicalized",
        )?;
        counters.insert("native_replicate".into(), json!(0));
        run["replicate"] = json!(ordinal);
    }
    check(keys.len() == 24, "canonical completeness")?;
    export["export_id"] = json!("HASHED-ECDL-CPU-SCALE-LOCAL-20261123-CANONICAL-V3-LOSSLESS");
    export["created"] = json!(created);
    export["notes"].as_array_mut().ok_or("notes")?.push(json!({"kind":"observation","by":"native lossless metadata adapter","text":"Preserved v1 uses native CSV replicate0 for both seeds, causing12 duplicate canonical keys. Preserved v2 fixes keys but rounds five timing values; it fails exact preservation. This v3 uses arbitrary-precision JSON numbers and changes only export_id,created,one appended note,seed-ordinal replicate and added native_replicate0. Seed101 maps to slot0 and202 to1; these are distinct public targets, not added observations or same-target repeats. All other JSON fields and number lexemes equal original v1."}));
    Ok(export)
}
fn exact_delta(original: &Value, output: &Value) -> Result<()> {
    let mut restored = output.clone();
    for key in ["export_id", "created"] {
        restored[key] = original[key].clone();
    }
    let before_notes = original["notes"].as_array().ok_or("original notes")?;
    let after_notes = output["notes"].as_array().ok_or("output notes")?;
    check(after_notes.len() == before_notes.len() + 1, "one note only")?;
    check(
        &after_notes[..before_notes.len()] == before_notes,
        "old notes changed",
    )?;
    restored["notes"] = original["notes"].clone();
    let before = original["runs"].as_array().ok_or("original runs")?;
    let after = restored["runs"].as_array_mut().ok_or("output runs")?;
    check(before.len() == after.len(), "row count changed")?;
    for (a, b) in before.iter().zip(after) {
        let expected = match a["seed"].as_u64() {
            Some(101) => 0,
            Some(202) => 1,
            _ => return Err("seed".into()),
        };
        check(b["replicate"] == expected, "wrong seed ordinal")?;
        b["replicate"] = a["replicate"].clone();
        let removed = b["measurements"]["counters"]
            .as_object_mut()
            .ok_or("counters")?
            .remove("native_replicate");
        check(removed == Some(json!(0)), "raw replicate missing/changed")?;
    }
    check(
        restored == *original,
        "unauthorized JSON field/number change",
    )
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    check(args.len() == 5, "INPUT OUTPUT EXPECTED_INPUT_SHA256 UTC")?;
    check(
        args[1] != args[2] && !Path::new(&args[2]).exists(),
        "output must be new",
    )?;
    let bytes = fs::read(&args[1])?;
    check(
        format!("{:x}", Sha256::digest(&bytes)) == args[3],
        "input digest mismatch",
    )?;
    let original: Value = serde_json::from_slice(&bytes)?;
    let export = canonicalize(original.clone(), &args[4])?;
    // Check both in-memory delta and serialized readback before creating output.
    exact_delta(&original, &export)?;
    let serialized = serde_json::to_vec_pretty(&export)?;
    exact_delta(&original, &serde_json::from_slice(&serialized)?)?;
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    out.write_all(&serialized)?;
    out.write_all(b"\n")?;
    println!(
        "PASS lossless canonicalization:24unique cells; exact recursive delta including number lexemes; no solver/EC calls"
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Value {
        let mut rows = Vec::new();
        for c in 0..6 {
            for seed in [101, 202] {
                for arm in ["u", "x"] {
                    rows.push(json!({"arm":arm,"instance_id":c.to_string(),"seed":seed,"replicate":0,"warmup":false,"status":"succeeded","outcome_class":"completed","verification":{"status":"verified"},"measurements":{"counters":{"relations":123}},"untouched":"preserve"}));
                }
            }
        }
        json!({"schema":"experiment.export/v1","export_id":"raw","created":"before","flow":{"planned":24,"completed":24},"runs":rows,"notes":[]})
    }
    #[test]
    fn declared_seed_slots_are_unique_and_preserve_data() {
        let original = fixture();
        let out = canonicalize(original.clone(), "2026-11-23T00:00:00Z").unwrap();
        for (a, b) in original["runs"]
            .as_array()
            .unwrap()
            .iter()
            .zip(out["runs"].as_array().unwrap())
        {
            assert_eq!(a["seed"], b["seed"]);
            assert_eq!(a["untouched"], b["untouched"]);
            assert_eq!(
                a["measurements"]["counters"]["relations"],
                b["measurements"]["counters"]["relations"]
            );
            assert_eq!(b["measurements"]["counters"]["native_replicate"], 0);
            assert_eq!(b["replicate"], if b["seed"] == 101 { 0 } else { 1 });
        }
    }
    #[test]
    fn duplicate_declared_seed_is_rejected() {
        let mut x = fixture();
        x["runs"][2]["seed"] = json!(101);
        assert!(canonicalize(x, "now").is_err());
    }
    #[test]
    fn failed_row_cannot_be_promoted() {
        let mut x = fixture();
        x["runs"][0]["status"] = json!("failed");
        assert!(canonicalize(x, "now").is_err());
    }
    #[test]
    fn already_normalized_view_is_rejected() {
        let x = canonicalize(fixture(), "now").unwrap();
        assert!(canonicalize(x, "later").is_err());
    }
    #[test]
    fn five_reported_fractional_values_survive_serialization_exactly() {
        let mut a = fixture();
        let values: Value = serde_json::from_str("[1.9122045619999999,1.9178180999999999,1.9178180999999999,3.6425995650000003,3.6425995650000003]").unwrap();
        a["runs"][0]["measurements"]["regression_values"] = values.clone();
        let b = canonicalize(a.clone(), "now").unwrap();
        let encoded = serde_json::to_vec_pretty(&b).unwrap();
        let readback: Value = serde_json::from_slice(&encoded).unwrap();
        exact_delta(&a, &readback).unwrap();
        assert_eq!(
            readback["runs"][0]["measurements"]["regression_values"],
            values
        );
        assert!(String::from_utf8(encoded)
            .unwrap()
            .contains("3.6425995650000003"));
    }
    #[test]
    fn unauthorized_measurement_rounding_is_rejected() {
        let mut a = fixture();
        a["runs"][0]["measurements"]["wall_seconds"] =
            serde_json::from_str("1.9178180999999999").unwrap();
        let mut b = canonicalize(a.clone(), "now").unwrap();
        b["runs"][0]["measurements"]["wall_seconds"] = serde_json::from_str("1.9178181").unwrap();
        assert!(exact_delta(&a, &b).is_err());
    }
    #[test]
    fn any_unlisted_field_change_is_rejected() {
        let a = fixture();
        let mut b = canonicalize(a.clone(), "now").unwrap();
        b["runs"][0]["untouched"] = json!("changed");
        assert!(exact_delta(&a, &b).is_err());
    }
}
