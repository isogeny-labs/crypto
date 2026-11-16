//! Replay certificate for the linear two-summand algebraic IC ladder
//! (`research/ic_m2_linear_ladder_20261024/PROTOCOL.md`).
//!
//! Usage: `ic_m2_ladder_replay <binary> <out.json> <rung.json>...`
//!
//! For each committed rung file, re-executes the frozen producer in a fresh
//! process on the same arguments (all targets on rungs below n = 41; target
//! 0 only at n = 41, a sampled replay), requires every integer field of the
//! replayed rows to equal the recorded ones, and checks every recorded
//! scalar of both arms with the library's own curve arithmetic
//! (`KoblitzCurve::mul`, BigUint, independent of the producer's field code):
//! `[d]G = [secret]G` for the IC and the rho answer.

use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::fs;
use std::process::Command;

const INT_FIELDS: [&str; 9] = [
    "secret",
    "ic_recovered",
    "ic_trials",
    "ic_relations",
    "ic_consistent",
    "ic_split_attempts",
    "ic_columns",
    "rho_recovered",
    "rho_walk_steps",
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let binary = &args[1];
    let mut runs = Vec::new();
    let mut all_ok = true;
    let mut scalar_checks = 0usize;
    for path in &args[3..] {
        let rec: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        let (a, n, d) = (
            rec["a"].as_u64().unwrap(),
            rec["n"].as_u64().unwrap(),
            rec["d"].as_u64().unwrap(),
        );
        let rows = rec["rows"].as_array().unwrap();
        let replay_targets = if n >= 41 { 1 } else { rows.len() };
        let tmp = std::env::temp_dir().join(format!("ic_m2_replay_a{a}_n{n}.json"));
        let status = Command::new(binary)
            .args([
                a.to_string(),
                n.to_string(),
                d.to_string(),
                replay_targets.to_string(),
                rec["seed"].to_string(),
                tmp.to_string_lossy().to_string(),
            ])
            .stderr(std::process::Stdio::null())
            .status()
            .expect("run producer");
        let rep: Value = serde_json::from_str(&fs::read_to_string(&tmp).unwrap()).unwrap();
        let mut mismatches = Vec::new();
        for (i, row) in rep["rows"].as_array().unwrap().iter().enumerate() {
            for k in INT_FIELDS {
                if row[k] != rows[i][k] {
                    mismatches.push(format!(
                        "target {i} {k}: recorded {} replayed {}",
                        rows[i][k], row[k]
                    ));
                }
            }
        }
        // Independent scalar check on every recorded row.
        let kc = KoblitzCurve::new(a as u8, n as u32).expect("curve");
        let g = kc.generator().clone();
        let mut bad = Vec::new();
        for (i, row) in rows.iter().enumerate() {
            let secret: BigUint = row["secret"].as_str().unwrap().parse().unwrap();
            let target = kc.mul(&g, &secret);
            for key in ["ic_recovered", "rho_recovered"] {
                let dval: BigUint = row[key].as_str().unwrap().parse().unwrap();
                scalar_checks += 1;
                if kc.mul(&g, &dval) != target {
                    bad.push(format!("target {i} {key}"));
                }
            }
        }
        let ok = status.success() && mismatches.is_empty() && bad.is_empty();
        all_ok &= ok;
        runs.push(json!({"rung": path, "a": a, "n": n, "d": d, "replayed_targets": replay_targets,
                         "recorded_targets": rows.len(), "producer_exit": status.code(),
                         "integer_fields_identical": mismatches.is_empty(), "mismatches": mismatches,
                         "scalar_failures": bad, "pass": ok}));
    }
    let out = json!({
        "schema": "ic-m2-linear-ladder-replay/v1",
        "status": if all_ok { "verified" } else { "failed" },
        "independent": true,
        "independence": "fresh-process re-execution of the frozen producer, and scalar checks by KoblitzCurve::mul (BigUint), a code path separate from the producer's Gf2 point arithmetic",
        "scalars_verified": all_ok,
        "scalar_checks": scalar_checks,
        "producer_binary": binary,
        "runs": runs,
    });
    fs::write(&args[2], serde_json::to_string_pretty(&out).unwrap() + "\n").unwrap();
    println!("replay {} ({} scalar checks)", out["status"], scalar_checks);
    assert!(all_ok);
}
