//! Correctness qualification only. Fixed inputs and full proof checks; no timing output.
use certified_relations::{packed_constructor, replay, System};
use relation_replay_checker::packed;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn save(p: &Path, b: &[u8]) -> Result<()> {
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(p)?
        .write_all(b)?;
    Ok(())
}
fn verify(input: &[u8], proof: &[u8], dag: bool) -> bool {
    if dag {
        packed::verify_dag_json(input, proof).is_ok()
    } else {
        packed::verify_json(input, proof).is_ok()
    }
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage:packed-qualify FIXED_INPUT_ROOT NEW_OUTPUT".into());
    }
    let root = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    fs::create_dir(out)?;
    let mut cases = Vec::new();
    for p in [2147483647u64, 9007199254740881] {
        for n in [3, 4, 6] {
            let raw = fs::read(root.join(format!("p{p}-n{n}-frontier/original.json")))?;
            if raw.len() > 4 * 1024 * 1024 {
                return Err("original cap".into());
            }
            let system: System = serde_json::from_slice(&raw)?;
            if system.prime != p || system.variables.len() != n {
                return Err("fixed input identity".into());
            }
            for dag in [false, true] {
                for (name, kernel) in [
                    ("frontier103", replay::NativeKernels::BothUpdatesFrontier),
                    ("products231", replay::NativeKernels::FrontierPackedProducts),
                    ("batch487", replay::NativeKernels::FrontierPackedBatch),
                ] {
                    let id = format!("p{p}-n{n}-{}-{name}", if dag { "dag" } else { "sparse" });
                    let dir = out.join(&id);
                    fs::create_dir(&dir)?;
                    save(&dir.join("original.json"), &raw)?;
                    let (old, om) = if dag {
                        replay::produce_compact_dag_for_packed(&system, kernel)?
                    } else {
                        replay::produce_native_for_packed(&system, "compact-sparse", kernel)?
                    };
                    save(&dir.join("legacy-proof.json"), &old)?;
                    let (new, nm) = if dag {
                        packed_constructor::produce_dag(&system, kernel)?
                    } else {
                        packed_constructor::produce_sparse(&system, kernel)?
                    };
                    save(&dir.join("packed-proof.json"), &new)?;
                    let checked = if dag {
                        packed::verify_dag_json(&raw, &new)?
                    } else {
                        packed::verify_json(&raw, &new)?
                    };
                    let old_checked = if dag {
                        packed::verify_dag_json(&raw, &old)?
                    } else {
                        packed::verify_json(&raw, &old)?
                    };
                    if new != old
                        || om != nm
                        || serde_json::to_value(&checked.basis)?
                            != serde_json::to_value(&old_checked.basis)?
                        || serde_json::to_value(&checked.stats)?
                            != serde_json::to_value(&old_checked.stats)?
                    {
                        return Err(format!("{id}: legacy equality mismatch").into());
                    }
                    let v: Value = serde_json::from_slice(&new)?;
                    let mut sabotages = Vec::new();
                    let mut bad = v.clone();
                    let c = bad["basis"][0]["polynomial"][0]["coefficient"]
                        .as_u64()
                        .ok_or("coefficient")?;
                    bad["basis"][0]["polynomial"][0]["coefficient"] = json!((c + 1) % p);
                    sabotages.push(("basis-coefficient", bad));
                    let mut bad = v.clone();
                    bad["input_reductions"]
                        .as_array_mut()
                        .ok_or("input reductions")?
                        .pop();
                    sabotages.push(("drop-input-obligation", bad));
                    let mut bad = v.clone();
                    if bad["pair_reductions"]
                        .as_array_mut()
                        .ok_or("pairs")?
                        .pop()
                        .is_none()
                    {
                        return Err("no pair obligation".into());
                    }
                    sabotages.push(("drop-pair-obligation", bad));
                    if dag {
                        let mut bad = v.clone();
                        bad["version"] = json!(1);
                        sabotages.push(("v2-as-v1", bad));
                    }
                    for (name, bad) in &sabotages {
                        let b = serde_json::to_vec(bad)?;
                        save(&dir.join(format!("sabotage-{name}.json")), &b)?;
                        if verify(&raw, &b, dag) {
                            return Err(format!("{id}: sabotage accepted: {name}").into());
                        }
                    }
                    let record = json!({"id":id,"prime":p,"n":n,"seed":7,"graph":"LAST-star","kernel":name,"dag":dag,"input_sha256":sha(&raw),"proof_sha256":sha(&new),"proof_bytes":new.len(),"byte_equal_legacy_proof":true,"metadata_equal":true,"checked_basis":checked.basis,"checked_stats":checked.stats,"metadata":nm,"rejected_mutations":sabotages.iter().map(|(n,_)|*n).collect::<Vec<_>>(),"timing_claim":false,"observed_solving_degree":null});
                    save(
                        &dir.join("outcome.json"),
                        &serde_json::to_vec_pretty(&record)?,
                    )?;
                    cases.push(record);
                }
            }
        }
    }
    let report = json!({"schema":"packed-constructor-correctness/v1","case_count":cases.len(),"cases":cases,"timing_claim":false,"signature":"unsigned producer data"});
    save(
        &out.join("matrix.json"),
        &serde_json::to_vec_pretty(&report)?,
    )?;
    println!(
        "{}",
        json!({"complete_cases":cases.len(),"timing_claim":false})
    );
    Ok(())
}
