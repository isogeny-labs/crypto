//! Correctness-only child-bounded constructor comparison; retain every failure.
use certified_relations::replay;
use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn save(p: &Path, b: &[u8]) -> Result<()> {
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(p)?
        .write_all(b)?;
    Ok(())
}
fn expanded_shadow(original: &[u8], wire: &[u8]) -> Result<Vec<u8>> {
    use std::collections::BTreeMap;
    type Proof = BTreeMap<(usize, Vec<usize>), u64>;
    let system: certified_relations::System = serde_json::from_slice(original)?;
    let n = system.variables.len();
    let p = system.prime;
    let count = system.decode()?.len();
    let mut w: replay::Witness = serde_json::from_slice(wire)?;
    let mut generators: Vec<Proof> = (0..count)
        .map(|i| BTreeMap::from([((i, vec![0; n]), 1)]))
        .collect();
    let expand = |proof: &[replay::CofactorTerm], gens: &[Proof]| -> Result<Proof> {
        let mut out = Proof::new();
        for t in proof {
            for ((i, m), c) in gens.get(t.generator).ok_or("shadow future reference")? {
                let shift = m
                    .iter()
                    .zip(&t.exponents)
                    .map(|(a, b)| a.checked_add(*b).ok_or("shadow exponent overflow"))
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                let key = (*i, shift);
                let product = ((u128::from(*c) * u128::from(t.coefficient)) % u128::from(p)) as u64;
                let sum = (out.get(&key).copied().unwrap_or(0) + product) % p;
                if sum == 0 {
                    out.remove(&key);
                } else {
                    out.insert(key, sum);
                }
                if out.len() > 100000 {
                    return Err("shadow proof cap".into());
                }
            }
        }
        Ok(out)
    };
    for row in &w.definitions {
        generators.push(expand(&row.proof, &generators)?);
    }
    for row in &mut w.basis {
        row.proof = expand(&row.proof, &generators)?
            .into_iter()
            .map(
                |((generator, exponents), coefficient)| replay::CofactorTerm {
                    generator,
                    exponents,
                    coefficient,
                },
            )
            .collect();
    }
    w.version = 1;
    w.definitions.clear();
    Ok(serde_json::to_vec(&w)?)
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage:dag-qualify NEW_OUT WORKER CORPUS66 SINGULAR".into());
    }
    let out = Path::new(&args[1]);
    fs::create_dir(out)?;
    let corpus = Path::new(&args[3]);
    let mut records = vec![];
    for p in [2147483647u64, 9007199254740881] {
        for n in [3, 4, 6] {
            let dir = out.join(format!("p{p}-n{n}"));
            fs::create_dir(&dir)?;
            let source = corpus.join(format!("p{p}-n{n}-frontier"));
            let original = fs::read(source.join("original.json"))?;
            let expected = relation_replay_checker::packed::verify_json(
                &original,
                &fs::read(source.join("witness.json"))?,
            )?;
            let attempt = (|| -> Result<Value> {
                let stdout = fs::File::options()
                    .write(true)
                    .create_new(true)
                    .open(dir.join("call.stdout"))?;
                let stderr = fs::File::options()
                    .write(true)
                    .create_new(true)
                    .open(dir.join("call.stderr"))?;
                let status = Command::new("timeout")
                    .args([
                        "--kill-after=1s",
                        "30s",
                        "prlimit",
                        "--as=2147483648",
                        "--fsize=16777216",
                        "--",
                        &args[2],
                        "cubic-dag",
                        "packed",
                        "frontier",
                    ])
                    .arg(dir.join("proof"))
                    .env("RAYON_NUM_THREADS", "1")
                    .stdin(Stdio::from(fs::File::open(source.join("original.json"))?))
                    .stdout(Stdio::from(stdout))
                    .stderr(Stdio::from(stderr))
                    .status()?;
                save(
                    &dir.join("process.json"),
                    &serde_json::to_vec(&json!({"exit":status.code()}))?,
                )?;
                if !status.success() || !fs::read(dir.join("call.stderr"))?.is_empty() {
                    return Err("constructor failed/refused/deadline".into());
                }
                let v: Value = serde_json::from_slice(&fs::read(dir.join("call.stdout"))?)?;
                let wire = fs::read(dir.join("proof/witness.json"))?;
                let checked = relation_replay_checker::packed::verify_dag_json(&original, &wire)?;
                if relation_replay_checker::packed::verify_json(&original, &wire).is_ok() {
                    return Err("version1 accepted DAGwire".into());
                }
                if checked.basis != expected.basis
                    || v["basis"] != serde_json::to_value(&checked.basis)?
                    || v["consumer_stats"] != serde_json::to_value(&checked.stats)?
                {
                    return Err("DAG/frontier full basis or receipt mismatch".into());
                }
                if p == 2147483647 {
                    let shadow = expanded_shadow(&original, &wire)?;
                    save(&dir.join("expanded-v1-shadow.json"), &shadow)?;
                    let old = relation_replay_checker::verify_json(&original, &shadow)?;
                    if old.basis != checked.basis {
                        return Err("old/packed replay mismatch".into());
                    }
                }
                let system = serde_json::from_slice(&original)?;
                let witness: replay::Witness = serde_json::from_slice(&wire)?;
                let script = dir.join("replay.sing");
                save(
                    &script,
                    replay::singular_script_dag(&system, &witness)?.as_bytes(),
                )?;
                let result = Command::new("timeout")
                    .args([
                        "30s",
                        "prlimit",
                        "--as=2147483648",
                        "--fsize=16777216",
                        "--",
                        &args[4],
                        "-q",
                    ])
                    .arg(&script)
                    .env(
                        "LD_LIBRARY_PATH",
                        "/workspace/msolve-source/src/neogb/.libs:/workspace/native-cas/lib",
                    )
                    .env("SINGULARPATH", "/workspace/native-cas/share/singular/LIB")
                    .output()?;
                save(&dir.join("singular.stdout"), &result.stdout)?;
                save(&dir.join("singular.stderr"), &result.stderr)?;
                if !result.status.success()
                    || !result.stderr.is_empty()
                    || std::str::from_utf8(&result.stdout)?.trim() != "CERTIFICATE_REPLAY_OK"
                {
                    return Err("Singular cubic-proof replay".into());
                }
                Ok(
                    json!({"prime":p,"n":n,"status":"complete","same_complete_frontier_basis":true,"all_obligations_replayed":true,"stats":checked.stats}),
                )
            })();
            let record = match attempt {
                Ok(v) => v,
                Err(e) => json!({"prime":p,"n":n,"status":"failed","reason":e.to_string()}),
            };
            save(
                &dir.join("outcome.json"),
                &serde_json::to_vec_pretty(&record)?,
            )?;
            records.push(record);
        }
    }
    let all = records.iter().all(|v| v["status"] == "complete");
    save(
        &out.join("audit.json"),
        &serde_json::to_vec_pretty(
            &json!({"cases":records,"all_correct":all,"scope":"correctness only; no performance ratios","goal15x":"active"}),
        )?,
    )
}
