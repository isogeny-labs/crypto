//! Separately executing verifier: recompute complete relations by scalar
//! group subtraction over BigUint, not the Vieta elimination algorithm.
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader};
use vieta_coset_lab::vieta_coset::*;

fn integer(s: &Value) -> BigUint {
    BigUint::parse_bytes(s.as_str().expect("hex string").as_bytes(), 16).expect("valid hex")
}
fn hash(r: &[Relation<BigUint>]) -> String {
    let mut h = Sha256::new();
    for (p, q) in r {
        for x in [&p.0, &p.1, &q.0, &q.1] {
            let s = x.to_str_radix(16);
            h.update((s.len() as u64).to_le_bytes());
            h.update(s.as_bytes());
        }
    }
    format!("{:x}", h.finalize())
}
fn main() {
    let files: Vec<_> = std::env::args().skip(1).collect();
    assert!(
        !files.is_empty(),
        "usage: vieta-coset-replay records.jsonl [...]"
    );
    let mut checked = 0;
    let mut failed_rows = 0;
    for path in files {
        for line in BufReader::new(File::open(&path).expect("open record")).lines() {
            let row: Value = serde_json::from_str(&line.expect("read record")).expect("JSON row");
            if row["kind"] != "query" {
                continue;
            }
            if row["status"] != "ok" {
                failed_rows += 1;
                continue;
            }
            let scope = &row["scope"];
            let p = integer(&scope["p_hex"]);
            let a = Arithmetic::new(WideField(p), Limits::default()).unwrap();
            let e = Curve {
                a: integer(&scope["a_hex"]),
                b: integer(&scope["b_hex"]),
            };
            let coset = a
                .coset(
                    scope["m"].as_u64().unwrap() as usize,
                    integer(&scope["c_hex"]),
                )
                .unwrap();
            let base = a.factor_base(&e, &coset).unwrap();
            assert_eq!(
                base.len() as u64,
                scope["base_points"].as_u64().unwrap(),
                "base size mismatch"
            );
            assert_eq!(
                hash(
                    &base
                        .iter()
                        .map(|p| (p.clone(), p.clone()))
                        .collect::<Vec<_>>()
                ),
                scope["base_sha256"].as_str().unwrap(),
                "base hash mismatch"
            );
            let t = &row["target"];
            let target = if t.is_null() {
                None
            } else {
                Some((integer(&t[0]), integer(&t[1])))
            };
            assert!(a.on_curve(&e, &target), "invalid target");
            let result = a.lookup_scalar(&e, &base, &target).unwrap();
            assert_eq!(
                result.len() as u64,
                row["relations"].as_u64().unwrap(),
                "relation count mismatch"
            );
            assert_eq!(
                hash(&result),
                row["result_sha256"].as_str().unwrap(),
                "relation hash mismatch"
            );
            checked += 1;
        }
    }
    assert!(checked > 0, "no successful two-term query rows to replay");
    println!(
        "{}",
        json!({"schema":"vieta-coset-replay/v1","checked":checked,"preserved_failed_rows":failed_rows,"algorithm":"wide-scalar-subtraction","standing":"producer check; independent signed PCC verdict required"})
    );
    if failed_rows > 0 {
        std::process::exit(1);
    }
}
