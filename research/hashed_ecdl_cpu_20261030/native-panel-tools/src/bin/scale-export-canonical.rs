//! Metadata-only successor to the preserved raw-faithful export.
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
    export["export_id"] = json!("HASHED-ECDL-CPU-SCALE-LOCAL-20261123-CANONICAL-V2");
    export["created"] = json!(created);
    export["notes"].as_array_mut().ok_or("notes")?.push(json!({"kind":"observation","by":"native metadata adapter","text":"Preserved v1 export used the native CSV replicate0 for both seeds, causing12 duplicate canonical execution-cell keys. This metadata-only successor maps declared seed101 to ordinal0 and202 to ordinal1, retaining native_replicate0. The two seeds are distinct public targets per curve; this creates no new observation or same-target repetition. Original rows, measurements, statuses, seeds, artifacts and protocol are unchanged."}));
    Ok(export)
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
    let export = canonicalize(serde_json::from_slice(&bytes)?, &args[4])?;
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    out.write_all(&serde_json::to_vec_pretty(&export)?)?;
    out.write_all(b"\n")?;
    println!(
        "PASS metadata-only canonicalization:24unique execution cells; original export unchanged"
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
        json!({"schema":"experiment.export/v1","flow":{"planned":24,"completed":24},"runs":rows,"notes":[]})
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
}
