//! Cross-implementation acceptance and sabotage, never speed evidence.
use certified_relations::Poly;
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
struct Case {
    name: String,
    input: Value,
    wire: Value,
    accept: bool,
}
fn captured(name: String, input: &Path, wire: &Path, accept: bool) -> Result<Case> {
    Ok(Case {
        name,
        input: serde_json::from_slice(&fs::read(input)?)?,
        wire: serde_json::from_slice(&fs::read(wire)?)?,
        accept,
    })
}
fn canonical(w: &Value) -> Result<BTreeSet<Poly>> {
    w["basis"]
        .as_array()
        .ok_or("wire basis")?
        .iter()
        .map(|r| {
            r["polynomial"]
                .as_array()
                .ok_or("row terms")?
                .iter()
                .map(|t| {
                    Ok((
                        serde_json::from_value(t["exponents"].clone())?,
                        t["coefficient"].as_u64().ok_or("coefficient")?,
                    ))
                })
                .collect::<Result<Poly>>()
        })
        .collect()
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    let root_mode = a.len() == 6 && a[5] == "root-verify";
    if a.len() != 5 && !root_mode {
        return Err("usage: rust-checker-qualifier CHECKER REPLAY_CLI SINGULAR NEW_OUTPUT".into());
    }
    let checker = PathBuf::from(&a[1]);
    let cli = PathBuf::from(&a[2]);
    let singular = PathBuf::from(&a[3]);
    let out = Path::new(&a[4]);
    fs::create_dir(out)?;
    let start = Instant::now();
    let mut cases = Vec::new();
    let sparse = Path::new("/workspace/sparse-replay-qualification-207e9a409-03");
    let summary: Value = serde_json::from_slice(&fs::read(sparse.join("summary.json"))?)?;
    for r in summary["records"].as_array().ok_or("sparse records")? {
        let kind = r["kind"].as_str().ok_or("kind")?;
        let fixture = r["fixture"].as_str();
        let (input, wire, accept) = if let Some(fixture) = fixture {
            let d = sparse.join(fixture);
            let sub = d.join(kind);
            (
                if kind.starts_with("fused-") {
                    d.join("input.json")
                } else {
                    sub.join("input.json")
                },
                sub.join("witness.json"),
                true,
            )
        } else {
            let d = sparse.join(kind);
            (d.join("input.json"), d.join("witness.json"), false)
        };
        cases.push(captured(
            format!("sparse-{}-{kind}", fixture.unwrap_or("tamper")),
            &input,
            &wire,
            accept,
        )?);
    }
    let mature = Path::new("/workspace/replay-qualification-ba3b691e4-04");
    let summary: Value = serde_json::from_slice(&fs::read(mature.join("summary.json"))?)?;
    for r in summary["cases"].as_array().ok_or("mature cases")? {
        let name = r["case"].as_str().ok_or("mature case")?;
        let d = mature.join(name);
        cases.push(captured(
            format!("mature-{name}"),
            &d.join("input.json"),
            &d.join("witness.json"),
            r["expected"] == "accept",
        )?);
    }
    let paid = Path::new("/workspace/sparse-paid-results-33e8f2914-01");
    let summary: Value = serde_json::from_slice(&fs::read(paid.join("summary.json"))?)?;
    for c in summary["cases"].as_array().ok_or("paid cases")? {
        let id = c["id"].as_str().ok_or("paid id")?;
        let input = Path::new("/workspace/sparse-paid-inputs-33e8f2914-01")
            .join(id)
            .join("system.json");
        for kind in ["fused-expanded", "fused-compact-ablation", "fused-sparse"] {
            cases.push(captured(
                format!("paid-{id}-{kind}"),
                &input,
                &paid.join(id).join(kind).join("proof/witness.json"),
                true,
            )?);
        }
    }
    let good_input = cases[0].input.clone();
    let good_wire = cases[0].wire.clone();
    for label in [
        "unknown-field",
        "duplicate-poly",
        "duplicate-proof",
        "zero-coefficient",
        "alias-coefficient",
        "overflow-exponent",
        "duplicate-variable",
        "empty-variable",
    ] {
        let mut input = good_input.clone();
        let mut wire = good_wire.clone();
        match label {
            "unknown-field" => wire["unexpected"] = json!(true),
            "duplicate-poly" => {
                let t = wire["basis"][0]["polynomial"][0].clone();
                wire["basis"][0]["polynomial"]
                    .as_array_mut()
                    .unwrap()
                    .push(t);
            }
            "duplicate-proof" => {
                let t = wire["basis"][0]["proof"][0].clone();
                wire["basis"][0]["proof"].as_array_mut().unwrap().push(t);
            }
            "zero-coefficient" => wire["basis"][0]["polynomial"][0]["coefficient"] = json!(0),
            "alias-coefficient" => {
                wire["basis"][0]["polynomial"][0]["coefficient"] = json!(2147483647u64)
            }
            "overflow-exponent" => {
                wire["basis"][0]["polynomial"][0]["exponents"] = json!([usize::MAX, 1, 0])
            }
            "duplicate-variable" => input["variables"][1] = input["variables"][0].clone(),
            "empty-variable" => input["variables"][0] = json!(""),
            _ => unreachable!(),
        }
        cases.push(Case {
            name: format!("extra-{label}"),
            input,
            wire,
            accept: false,
        });
    }
    let mut normalized = good_input;
    normalized["polynomials"]
        .as_array_mut()
        .unwrap()
        .insert(0, json!([]));
    cases.push(Case {
        name: "extra-normalized-zero-generator".into(),
        input: normalized,
        wire: good_wire,
        accept: true,
    });
    let mut records = Vec::new();
    for c in cases {
        let dir = out.join(&c.name);
        fs::create_dir(&dir)?;
        let input = dir.join("input.json");
        let wire = dir.join("witness.json");
        let script = dir.join("check.sing");
        save(&input, &serde_json::to_vec(&c.input)?)?;
        save(&wire, &serde_json::to_vec(&c.wire)?)?;
        let result = (|| -> Result<Value> {
            let mut checker_args = vec![input.display().to_string(), wire.display().to_string()];
            if root_mode {
                checker_args.insert(0, "verify".into());
            }
            let (code, stdout, stderr) = call(&checker, &checker_args, &dir, "rust", start)?;
            let accepted = code == Some(0) && stderr.is_empty();
            let rejected = code == Some(1) && stderr.starts_with("Error:");
            let mut same = false;
            if accepted {
                let answer: Value = serde_json::from_str(&stdout)?;
                let rows: Vec<Vec<(Vec<usize>, u64)>> =
                    serde_json::from_value(answer["basis"].clone())?;
                let basis: BTreeSet<Poly> =
                    rows.into_iter().map(|r| r.into_iter().collect()).collect();
                same = basis == canonical(&c.wire)?;
            }
            let (cas_code, _, cas_stderr) = call(
                &cli,
                &[
                    "script".into(),
                    input.display().to_string(),
                    wire.display().to_string(),
                    script.display().to_string(),
                ],
                &dir,
                "export",
                start,
            )?;
            let cas_ok = if cas_code == Some(0) && cas_stderr.is_empty() {
                let (code, stdout, stderr) = call(
                    &singular,
                    &["-q".into(), script.display().to_string()],
                    &dir,
                    "singular",
                    start,
                )?;
                code == Some(0)
                    && stderr.is_empty()
                    && if c.accept {
                        stdout.trim() == "CERTIFICATE_REPLAY_OK"
                    } else {
                        stdout.trim().starts_with("REPLAY_FAIL:")
                    }
            } else {
                !c.accept && cas_code == Some(1) && cas_stderr.starts_with("Error:")
            };
            Ok(
                json!({"case":c.name,"expected_accept":c.accept,"passed":if c.accept{accepted&&same&&cas_ok}else{rejected&&cas_ok},"rust_exit":code,"rust_rejection":stderr,"canonical_basis_equal":if c.accept{Some(same)}else{None},"fresh_singular_agrees":cas_ok}),
            )
        })();
        records.push(result.unwrap_or_else(|e|json!({"case":c.name,"expected_accept":c.accept,"passed":false,"reason":e.to_string()})));
    }
    let passed = records.iter().all(|r| r["passed"] == true);
    let accepted = records
        .iter()
        .filter(|r| r["expected_accept"] == true && r["passed"] == true)
        .count();
    let rejected = records
        .iter()
        .filter(|r| r["expected_accept"] == false && r["passed"] == true)
        .count();
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(
            &json!({"all_passed":passed,"accepted_cases":accepted,"rejected_cases":rejected,"records":records,"scope":"correctness/cross-implementation only; no speed ratios"}),
        )?,
    )?;
    if !passed {
        return Err("checker corpus qualification failed; all attempts retained".into());
    }
    Ok(())
}
