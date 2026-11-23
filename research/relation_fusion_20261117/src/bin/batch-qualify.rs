//! Correctness-only corpus. No benchmark ratios or seed selection.
use certified_relations::{replay, workload};
use relation_replay_checker::{packed, verify_json};
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path, process::Command};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn save(p: &Path, bytes: &[u8]) -> Result<()> {
    use std::io::Write;
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(p)?
        .write_all(bytes)?;
    Ok(())
}
fn check(input: &[u8], wire: &Value, expected: bool) -> Result<()> {
    if packed::verify_json(input, &serde_json::to_vec(wire)?).is_ok() != expected {
        return Err("packed tamper outcome".into());
    }
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage:batch-qualify NEW_OUT SINGULAR".into());
    }
    let out = Path::new(&args[1]);
    fs::create_dir(out)?;
    let mut records = Vec::new();
    for p in [2147483647, 9007199254740881] {
        for n in [3, 4, 6] {
            let fixture = workload::generate_graph(
                p,
                n,
                7,
                &(0..n - 1).map(|i| (i, n - 1)).collect::<Vec<_>>(),
            )?;
            let input = serde_json::to_vec(&fixture.system)?;
            let mut expected = None;
            for (name, kernel) in [
                (
                    "packed-control",
                    replay::NativeKernels::FrontierPackedProducts,
                ),
                ("packed-batch", replay::NativeKernels::FrontierPackedBatch),
            ] {
                let dir = out.join(format!("p{p}-n{n}-{name}"));
                fs::create_dir(&dir)?;
                let (wire, constructor) =
                    replay::produce_native_for_packed(&fixture.system, "compact-sparse", kernel)?;
                save(&dir.join("original.json"), &input)?;
                save(&dir.join("witness.json"), &wire)?;
                let result = packed::verify_json(&input, &wire)?;
                let mut basis = result.basis.clone();
                basis.sort();
                if expected.as_ref().is_some_and(|e| e != &basis) {
                    return Err("kernel complete basis differs".into());
                }
                expected = Some(basis);
                if p == 2147483647 {
                    let old = verify_json(&input, &wire)?;
                    if old.basis != result.basis
                        || serde_json::to_value(old.stats)? != serde_json::to_value(&result.stats)?
                    {
                        return Err("old/packed basis or obligation stats differ".into());
                    }
                } else if verify_json(&input, &wire).is_ok() {
                    return Err("legacy field scope unexpectedly expanded".into());
                }
                let w: Value = serde_json::from_slice(&wire)?;
                for kind in [
                    "basis-coefficient",
                    "basis-reference",
                    "definition-reference",
                    "input-coverage",
                    "input-coefficient",
                    "pair-coverage",
                    "pair-index",
                    "nonstandard",
                    "field",
                    "dimension",
                    "duplicate",
                    "degree",
                ] {
                    let mut v = w.clone();
                    let mut original: Value = serde_json::from_slice(&input)?;
                    match kind {
                        "basis-coefficient" => {
                            let c = v["basis"][0]["polynomial"][0]["coefficient"]
                                .as_u64()
                                .unwrap();
                            v["basis"][0]["polynomial"][0]["coefficient"] = json!((c + 1) % p);
                        }
                        "basis-reference" => v["basis"][0]["proof"][0]["generator"] = json!(100000),
                        "definition-reference" => {
                            if v["definitions"].as_array().unwrap().is_empty() {
                                continue;
                            }
                            v["definitions"][0]["proof"][0]["generator"] =
                                json!(fixture.system.polynomials.len());
                        }
                        "input-coverage" => {
                            v["input_reductions"].as_array_mut().unwrap().pop();
                        }
                        "input-coefficient" => {
                            let c = v["input_reductions"][0][0]["coefficient"].as_u64().unwrap();
                            v["input_reductions"][0][0]["coefficient"] = json!((c + 1) % p);
                        }
                        "pair-coverage" => {
                            v["pair_reductions"].as_array_mut().unwrap().pop();
                        }
                        "pair-index" => v["pair_reductions"][0]["right"] = json!(100000),
                        "nonstandard" => {
                            v["pair_reductions"][0]["product_criterion"] = json!(false);
                            v["pair_reductions"][0]["proof"] =
                                json!([{"generator":0,"exponents":vec![64;n],"coefficient":1}]);
                        }
                        "field" => original["prime"] = json!(53),
                        "dimension" => v["basis"][0]["polynomial"][0]["exponents"] = json!([1]),
                        "duplicate" => {
                            let t = v["basis"][0]["polynomial"][0].clone();
                            v["basis"][0]["polynomial"].as_array_mut().unwrap().push(t);
                        }
                        "degree" => {
                            v["basis"][0]["proof"][0]["exponents"] = json!(vec![usize::MAX; n])
                        }
                        _ => unreachable!(),
                    }
                    check(&serde_json::to_vec(&original)?, &v, false)?;
                    save(
                        &dir.join(format!("bad-{kind}.json")),
                        &serde_json::to_vec(&v)?,
                    )?;
                }
                let witness: replay::Witness = serde_json::from_slice(&wire)?;
                let script = dir.join("replay.sing");
                save(
                    &script,
                    replay::singular_script_residue(&fixture.system, &witness)?.as_bytes(),
                )?;
                let process = Command::new("timeout")
                    .args([
                        "30s",
                        "prlimit",
                        "--as=2147483648",
                        "--fsize=16777216",
                        "--",
                        &args[2],
                        "-q",
                    ])
                    .arg(&script)
                    .env(
                        "LD_LIBRARY_PATH",
                        "/workspace/msolve-source/src/neogb/.libs:/workspace/native-cas/lib",
                    )
                    .env("SINGULARPATH", "/workspace/native-cas/share/singular/LIB")
                    .output()?;
                save(&dir.join("singular.stdout"), &process.stdout)?;
                save(&dir.join("singular.stderr"), &process.stderr)?;
                if !process.status.success()
                    || !process.stderr.is_empty()
                    || std::str::from_utf8(&process.stdout)?.trim() != "CERTIFICATE_REPLAY_OK"
                {
                    return Err("fresh Singular complete proof check".into());
                }
                let record = json!({"prime":p,"n":n,"kernel":name,"whole_certificate_Singular_agreement":true,"old_p31_stats_and_basis_equal":p==2147483647,"stats":result.stats,"constructor":constructor});
                save(
                    &dir.join("outcome.json"),
                    &serde_json::to_vec_pretty(&record)?,
                )?;
                records.push(record);
            }
        }
    }
    save(
        &out.join("audit.json"),
        &serde_json::to_vec_pretty(
            &json!({"scope":"correctness only; no speed ratios","cases":records,"tamper_kinds":12,"whole_basis_checks_retained":true}),
        )?,
    )
}
