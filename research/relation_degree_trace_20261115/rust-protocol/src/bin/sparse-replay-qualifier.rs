//! Correctness only. Run under timeout180s; all child receipts are retained.
use certified_relations::{
    replay::{self, CofactorTerm, Witness},
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
fn check(
    cli: &Path,
    singular: &Path,
    dir: &Path,
    s: &System,
    w: &Witness,
    expected: &str,
    start: Instant,
) -> Result<Value> {
    fs::create_dir(dir)?;
    let input = dir.join("input.json");
    let wire = dir.join("witness.json");
    let script = dir.join("check.sing");
    save(&input, &serde_json::to_vec(s)?)?;
    save(&wire, &serde_json::to_vec(w)?)?;
    let (code, _, stderr) = call(
        cli,
        &[
            "script".into(),
            input.display().to_string(),
            wire.display().to_string(),
            script.display().to_string(),
        ],
        dir,
        "export",
        start,
    )?;
    if code != Some(0) {
        let passed = expected != "accept"
            && expected != "arithmetic"
            && code == Some(1)
            && stderr.starts_with("Error:");
        return Ok(json!({"expected":expected,"passed":passed,"stage":"shape","stderr":stderr}));
    }
    if !stderr.is_empty() {
        return Err("successful export has diagnostics".into());
    }
    let (code, stdout, stderr) = call(
        singular,
        &["-q".into(), script.display().to_string()],
        dir,
        "check",
        start,
    )?;
    let accepted = code == Some(0) && stderr.is_empty() && stdout.trim() == "CERTIFICATE_REPLAY_OK";
    let rejected =
        code == Some(0) && stderr.is_empty() && stdout.trim().starts_with("REPLAY_FAIL:");
    Ok(
        json!({"expected":expected,"passed":if expected=="accept"{accepted}else if expected=="shape"{false}else{rejected},"stage":"arithmetic","stdout":stdout,"stderr":stderr,"exit_code":code}),
    )
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 6 {
        return Err("usage: sparse-replay-qualifier CLI FUSED SINGULAR MSOLVE NEW_OUTPUT".into());
    }
    let cli = PathBuf::from(&args[1]);
    let fused = PathBuf::from(&args[2]);
    let singular = PathBuf::from(&args[3]);
    let out = Path::new(&args[5]);
    fs::create_dir(out)?;
    let start = Instant::now();
    let mut records = Vec::<Value>::new();
    let mut fixtures = vec![(
        "cycle3".to_string(),
        workload::generate(2147483647, 3, 7)?.system,
    )];
    for hub in [0, 7] {
        let edges: Vec<_> = (0..8).filter(|v| *v != hub).map(|v| (hub, v)).collect();
        fixtures.push((
            format!("star8-hub{hub}"),
            workload::generate_graph(2147483647, 8, 7, &edges)?.system,
        ));
    }
    for (name, s) in &fixtures {
        let dir = out.join(name);
        fs::create_dir(&dir)?;
        let input = dir.join("input.json");
        save(&input, &serde_json::to_vec(s)?)?;
        let reference = replay::produce(s, true)?;
        for kind in ["compact-product", "expanded-product", "compact-sparse"] {
            let wire = dir.join(format!("{kind}.json"));
            let (code, _, stderr) = call(
                &cli,
                &[
                    "produce".into(),
                    input.display().to_string(),
                    kind.into(),
                    wire.display().to_string(),
                ],
                &dir,
                &format!("produce-{kind}"),
                start,
            )?;
            if code != Some(0) || !stderr.is_empty() {
                return Err(format!("producer failed {name}/{kind}").into());
            }
            let w: Witness = serde_json::from_slice(&fs::read(wire)?)?;
            let mut result = check(&cli, &singular, &dir.join(kind), s, &w, "accept", start)?;
            result["same_basis"] = json!(canonical(&w) == canonical(&reference));
            result["passed"] = json!(result["passed"] == true && result["same_basis"] == true);
            result["fixture"] = json!(name);
            result["kind"] = json!(kind);
            records.push(result);
        }
        let kinds: &[&str] = if name == "cycle3" {
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
        for kind in kinds {
            let dest = dir.join(format!("fused-{kind}"));
            let (code, stdout, stderr) = call(
                &fused,
                &[
                    input.display().to_string(),
                    kind.to_string(),
                    args[3].clone(),
                    args[4].clone(),
                    dest.display().to_string(),
                ],
                &dir,
                &format!("fused-{kind}"),
                start,
            )?;
            let mut result = json!({"fixture":name,"kind":format!("fused-{kind}"),"passed":false,"exit_code":code,"stderr":stderr});
            if code == Some(0) && stderr.is_empty() {
                let answer: Value = serde_json::from_str(&stdout)?;
                let w: Witness = serde_json::from_slice(&fs::read(dest.join("witness.json"))?)?;
                let rows: Vec<Vec<(Vec<usize>, u64)>> =
                    serde_json::from_value(answer["basis"].clone())?;
                let declared: BTreeSet<Poly> = rows
                    .into_iter()
                    .map(|row| row.into_iter().collect())
                    .collect();
                result["passed"] = json!(
                    canonical(&w) == canonical(&reference) && declared == canonical(&reference)
                );
                result["answer"] = answer;
            }
            records.push(result);
        }
    }
    let s = fixtures[0].1.clone();
    let original = replay::produce_product(&s, true)?;
    let mut cases = Vec::<(String, System, Witness, &str)>::new();
    let mut add = |name: &str, w: Witness, expected: &'static str| {
        cases.push((name.into(), s.clone(), w, expected))
    };
    let mut w = original.clone();
    w.pair_reductions.pop();
    add("missing-pair", w, "shape");
    let mut w = original.clone();
    w.pair_reductions[0].right = 0;
    add("duplicate-pair", w, "shape");
    let mut w = original.clone();
    w.input_reductions.pop();
    add("missing-input", w, "shape");
    let mut w = original.clone();
    w.definitions[0].proof[0].generator = s.polynomials.len();
    add("future-definition", w, "shape");
    let mut w = original.clone();
    w.definitions[0].proof[0].coefficient = if w.definitions[0].proof[0].coefficient == 1 {
        2
    } else {
        1
    };
    add("altered-identity", w, "arithmetic");
    let mut w = original.clone();
    w.pair_reductions
        .iter_mut()
        .find(|r| r.product_criterion)
        .unwrap()
        .proof
        .push(CofactorTerm {
            generator: 0,
            exponents: vec![0; 3],
            coefficient: 1,
        });
    add("product-with-proof", w, "shape");
    let mut w = original.clone();
    w.basis[0].polynomial[0].exponents = vec![0, 0, 3];
    add("altered-leading-monomial", w, "reject");
    let (bad, mut w) = legacy::non_gb();
    w.pair_reductions[0].product_criterion = true;
    cases.push(("noncoprime-product".into(), bad.clone(), w, "shape"));
    let (_, mut w) = legacy::non_gb();
    w.pair_reductions[0].proof = vec![
        CofactorTerm {
            generator: 0,
            exponents: vec![0, 1, 0],
            coefficient: 1,
        },
        CofactorTerm {
            generator: 1,
            exponents: vec![1, 0, 0],
            coefficient: s.prime - 1,
        },
    ];
    cases.push(("trivial-nonstandard".into(), bad.clone(), w, "shape"));
    let (_, w) = legacy::non_gb();
    cases.push(("forged-zero-pair".into(), bad, w, "arithmetic"));
    for (name, s, w, expected) in cases {
        let mut r = check(&cli, &singular, &out.join(&name), &s, &w, expected, start)?;
        r["kind"] = json!(name);
        records.push(r);
    }
    let passed = records.iter().all(|r| r["passed"] == true);
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(
            &json!({"passed":passed,"records":records,"scope":"correctness only; no performance ratios"}),
        )?,
    )?;
    if !passed {
        return Err("sparse qualification failed; retained every attempted case".into());
    }
    Ok(())
}
