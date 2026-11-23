//! Actual fused dependency graph, all ablations and fresh external proof audits.
use certified_relations::{
    replay::{self, Witness},
    workload, Poly, System,
};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
    time::Instant,
};
#[allow(dead_code)]
#[path = "replay-qualifier.rs"]
mod legacy;
use legacy::{call, save};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn canonical(w: &Witness) -> BTreeSet<Poly> {
    w.basis
        .iter()
        .map(|r| {
            r.polynomial
                .iter()
                .map(|t| (t.exponents.clone(), t.coefficient))
                .collect()
        })
        .collect()
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    let root_only = a.len() == 9 && a[8] == "root-only";
    let root_all = a.len() == 10 && a[8] == "root-all";
    let root = root_only || root_all;
    if a.len() != 8 && !root {
        return Err("usage: fused-rust-qualifier BOTH CONSUMER_ONLY NATIVE_ONLY CLI SINGULAR MSOLVE NEW_OUTPUT".into());
    }
    let out = Path::new(&a[7]);
    fs::create_dir(out)?;
    let start = Instant::now();
    let mut records = Vec::<Value>::new();
    let corpus = Path::new("/workspace/rust-checker-corpus-67330ab4e-03");
    let summary: Value = serde_json::from_slice(&fs::read(corpus.join("summary.json"))?)?;
    for item in summary["records"]
        .as_array()
        .ok_or("corpus records")?
        .iter()
        .filter(|_| !root)
    {
        let name = item["case"].as_str().ok_or("case")?;
        let dir = corpus.join(name);
        let input = fs::read(dir.join("input.json"))?;
        let wire = fs::read(dir.join("witness.json"))?;
        let result = relation_replay_checker::verify_json(&input, &wire);
        let expected = item["expected_accept"] == true;
        let mut same = false;
        if let Ok(checked) = &result {
            let w: Witness = serde_json::from_slice(&wire)?;
            let basis: BTreeSet<Poly> = checked
                .basis
                .iter()
                .map(|r| r.iter().cloned().collect())
                .collect();
            same = basis == canonical(&w);
        }
        records.push(json!({"case":format!("embedded-{name}"),"kind":"embedded-corpus","expected_accept":expected,"passed":if expected{result.is_ok()&&same}else{result.is_err()},"basis_equal":if expected{Some(same)}else{None},"rejection":result.err(),"source":"captured corpus03; actual fused resolved dependency graph"}));
    }
    let mut fixtures = vec![(
        "cycle3".to_string(),
        workload::generate(2147483647, 3, 7)?.system,
    )];
    for hub in [0, 7] {
        let edges: Vec<_> = (0..8).filter(|v| *v != hub).map(|v| (hub, v)).collect();
        fixtures.push((
            format!("star8-{hub}"),
            workload::generate_graph(2147483647, 8, 7, &edges)?.system,
        ));
    }
    for id in ["star-first-n16-s7", "star-last-n16-s7"] {
        let s: System = serde_json::from_slice(&fs::read(
            Path::new("/workspace/sparse-paid-inputs-33e8f2914-01")
                .join(id)
                .join("system.json"),
        )?)?;
        fixtures.push((id.into(), s));
    }
    for (id, system) in fixtures {
        let reference = canonical(&replay::produce(&system, true)?);
        let kinds: &[&str] = if id == "cycle3" && !root {
            &[
                "compact",
                "expanded",
                "compact-sparse",
                "std",
                "slimgb",
                "msolve-lift",
            ]
        } else {
            &["compact", "expanded", "compact-sparse"]
        };
        let variants = if root_all {
            vec![
                ("root-baseline", PathBuf::from(&a[9]), false, false),
                ("root-consumer-only", PathBuf::from(&a[2]), true, false),
                ("root-native-only", PathBuf::from(&a[3]), false, true),
                ("root-both", PathBuf::from(&a[1]), true, true),
            ]
        } else if root_only {
            vec![("root-both", PathBuf::from(&a[1]), true, true)]
        } else {
            vec![
                ("both", PathBuf::from(&a[1]), true, true),
                ("consumer-only", PathBuf::from(&a[2]), true, false),
                ("native-only", PathBuf::from(&a[3]), false, true),
            ]
        };
        for (variant, exe, rust, native) in variants {
            for kind in kinds {
                let name = format!("{id}-{variant}-{kind}");
                let dir = out.join(&name);
                fs::create_dir(&dir)?;
                let input = dir.join("input.json");
                save(&input, &serde_json::to_vec(&system)?)?;
                let proof = dir.join("proof");
                let result = (|| -> Result<Value> {
                    let (code, stdout, stderr) = call(
                        &exe,
                        &[
                            input.display().to_string(),
                            kind.to_string(),
                            a[5].clone(),
                            a[6].clone(),
                            proof.display().to_string(),
                        ],
                        &dir,
                        "fused",
                        start,
                    )?;
                    if code != Some(0) || !stderr.is_empty() {
                        return Err(format!("fused failed/diagnostics: {stderr}").into());
                    }
                    let answer: Value = serde_json::from_str(&stdout)?;
                    let rows: Vec<Vec<(Vec<usize>, u64)>> =
                        serde_json::from_value(answer["basis"].clone())?;
                    let basis: BTreeSet<Poly> =
                        rows.into_iter().map(|r| r.into_iter().collect()).collect();
                    let wire: Witness =
                        serde_json::from_slice(&fs::read(proof.join("witness.json"))?)?;
                    let same = basis == reference && canonical(&wire) == reference;
                    let variant_ok = (!root || answer["root_api"] == rust)
                        && answer["native_product_criterion"] == native
                        && answer["consumer"]["kind"]
                            == if rust {
                                "separate-rust-arithmetic"
                            } else {
                                "singular-identity"
                            };
                    let script = dir.join("audit.sing");
                    let (code, _, stderr) = call(
                        Path::new(&a[4]),
                        &[
                            "script".into(),
                            input.display().to_string(),
                            proof.join("witness.json").display().to_string(),
                            script.display().to_string(),
                        ],
                        &dir,
                        "audit-export",
                        start,
                    )?;
                    if code != Some(0) || !stderr.is_empty() {
                        return Err("audit export failed".into());
                    }
                    let (code, stdout, stderr) = call(
                        Path::new(&a[5]),
                        &["-q".into(), script.display().to_string()],
                        &dir,
                        "audit-check",
                        start,
                    )?;
                    let audit = code == Some(0)
                        && stderr.is_empty()
                        && stdout.trim() == "CERTIFICATE_REPLAY_OK";
                    Ok(
                        json!({"case":name,"kind":"fused","passed":same&&variant_ok&&audit,"complete_basis_equal":same,"variant_correct":variant_ok,"fresh_singular_accepts":audit,"constructor":answer["constructor"],"consumer":answer["consumer"]}),
                    )
                })();
                records.push(result.unwrap_or_else(
                    |e| json!({"case":name,"kind":"fused","passed":false,"reason":e.to_string()}),
                ));
            }
        }
    }
    let passed = records.iter().all(|r| r["passed"] == true);
    let embedded = records
        .iter()
        .filter(|r| r["kind"] == "embedded-corpus")
        .count();
    let fused = records.iter().filter(|r| r["kind"] == "fused").count();
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(
            &json!({"all_passed":passed,"embedded_corpus_cases":embedded,"fused_acceptance_cases":fused,"records":records,"scope":"correctness only; no timing ratios or independent signature"}),
        )?,
    )?;
    if !passed {
        return Err("fused qualification failed; every attempt retained".into());
    }
    Ok(())
}
