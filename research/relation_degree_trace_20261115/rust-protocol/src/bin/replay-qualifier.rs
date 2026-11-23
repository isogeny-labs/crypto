//! Bounded correctness/tamper checks; elapsed test clocks are not performance data.
use certified_relations::{
    replay::{CanonicalTerm, CofactorTerm, Member, PairReduction, Witness},
    workload, Poly, System, Term,
};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub(crate) fn save(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(bytes)?;
    Ok(())
}
pub(crate) fn call(
    exe: &Path,
    args: &[String],
    out: &Path,
    label: &str,
    start: Instant,
) -> Result<(Option<i32>, String, String)> {
    let remaining = 600u64.saturating_sub(start.elapsed().as_secs()).min(30);
    if remaining == 0 {
        return Err("qualification global deadline".into());
    }
    let stdout = out.join(format!("{label}.stdout"));
    let stderr = out.join(format!("{label}.stderr"));
    let status = Command::new("timeout")
        .args([
            "--kill-after=1s",
            &format!("{remaining}s"),
            "prlimit",
            "--as=2147483648",
            "--fsize=16777216",
            "--",
            "taskset",
            "-c",
            "0",
        ])
        .arg(exe)
        .args(args)
        .env("RAYON_NUM_THREADS", "1")
        .env("OMP_NUM_THREADS", "1")
        .env("OPENBLAS_NUM_THREADS", "1")
        .env("SINGULARPATH", "/workspace/native-cas/share/singular/LIB")
        .env(
            "LD_LIBRARY_PATH",
            "/workspace/msolve-source/src/neogb/.libs:/workspace/native-cas/lib",
        )
        .stdin(Stdio::null())
        .stdout(
            fs::File::options()
                .write(true)
                .create_new(true)
                .open(&stdout)?,
        )
        .stderr(
            fs::File::options()
                .write(true)
                .create_new(true)
                .open(&stderr)?,
        )
        .status()?;
    save(
        &out.join(format!("{label}.process.json")),
        &serde_json::to_vec_pretty(
            &json!({"program":exe,"args":args,"status":status.to_string(),"exit_code":status.code(),"scope":"correctness only; not performance"}),
        )?,
    )?;
    Ok((
        status.code(),
        fs::read_to_string(stdout)?,
        fs::read_to_string(stderr)?,
    ))
}
fn unit(index: usize) -> Vec<CofactorTerm> {
    vec![CofactorTerm {
        generator: index,
        exponents: vec![0; 3],
        coefficient: 1,
    }]
}
pub(crate) fn non_gb() -> (System, Witness) {
    let p = 2147483647;
    let t = |m: [usize; 3], c: i64| Term {
        exponents: m.to_vec(),
        coefficient: c,
    };
    let system = System {
        prime: p,
        variables: vec!["x".into(), "y".into(), "z".into()],
        polynomials: vec![
            vec![t([2, 0, 0], 1), t([0, 1, 0], p as i64 - 1)],
            vec![t([1, 1, 0], 1), t([0, 0, 0], p as i64 - 1)],
        ],
    };
    let basis = system
        .decode()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, g)| Member {
            polynomial: g
                .iter()
                .map(|(m, c)| CanonicalTerm {
                    exponents: m.clone(),
                    coefficient: *c,
                })
                .collect(),
            proof: unit(i),
        })
        .collect();
    let witness = Witness {
        version: 1,
        definitions: vec![],
        basis,
        input_reductions: vec![unit(0), unit(1)],
        pair_reductions: vec![PairReduction {
            left: 0,
            right: 1,
            product_criterion: false,
            proof: vec![],
        }],
    };
    (system, witness)
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: replay-qualifier REPLAY_CLI SINGULAR MSOLVE NEW_OUTPUT".into());
    }
    let cli = PathBuf::from(&args[1]);
    let singular = PathBuf::from(&args[2]);
    let msolve = PathBuf::from(&args[3]);
    let out = Path::new(&args[4]);
    fs::create_dir(out)?;
    let start = Instant::now();
    let system = workload::generate(2147483647, 3, 7)?.system;
    let input = out.join("system.json");
    save(&input, &serde_json::to_vec_pretty(&system)?)?;
    let mut records = Vec::<Value>::new();
    let mut positive = Vec::new();
    for mode in ["compact", "expanded"] {
        let wire = out.join(format!("{mode}.json"));
        let (status, _, stderr) = call(
            &cli,
            &[
                "produce".into(),
                input.display().to_string(),
                mode.into(),
                wire.display().to_string(),
            ],
            out,
            &format!("produce-{mode}"),
            start,
        )?;
        if status != Some(0) || !stderr.is_empty() {
            return Err(format!("{mode} producer failed").into());
        }
        let witness: Witness = serde_json::from_slice(&fs::read(wire)?)?;
        positive.push((mode.to_string(), system.clone(), witness));
    }
    for method in ["std", "slimgb", "msolve-lift"] {
        let script = out.join(format!("{method}-producer.sing"));
        if method == "msolve-lift" {
            let ms_input = out.join("input.ms");
            let (code, _, stderr) = call(
                &cli,
                &[
                    "program".into(),
                    input.display().to_string(),
                    "msolve".into(),
                    ms_input.display().to_string(),
                ],
                out,
                "msolve-input",
                start,
            )?;
            if code != Some(0) || !stderr.is_empty() {
                return Err("msolve input preparation failed".into());
            }
            let ms_basis = out.join("msolve-basis.txt");
            let (code, _, stderr) = call(
                &msolve,
                &[
                    "-g".into(),
                    "2".into(),
                    "-t".into(),
                    "1".into(),
                    "-f".into(),
                    ms_input.display().to_string(),
                    "-o".into(),
                    ms_basis.display().to_string(),
                ],
                out,
                "msolve",
                start,
            )?;
            if code != Some(0) || !stderr.is_empty() {
                return Err("msolve producer failed".into());
            }
            let (code, _, stderr) = call(
                &cli,
                &[
                    "program-lift".into(),
                    input.display().to_string(),
                    ms_basis.display().to_string(),
                    script.display().to_string(),
                ],
                out,
                "msolve-lift-program",
                start,
            )?;
            if code != Some(0) || !stderr.is_empty() {
                return Err("msolve lifting program failed".into());
            }
        } else {
            let (code, _, stderr) = call(
                &cli,
                &[
                    "program".into(),
                    input.display().to_string(),
                    method.into(),
                    script.display().to_string(),
                ],
                out,
                &format!("{method}-program"),
                start,
            )?;
            if code != Some(0) || !stderr.is_empty() {
                return Err("liftstd program failed".into());
            }
        }
        let label = format!("{method}-producer");
        let (code, _, stderr) = call(
            &singular,
            &["-q".into(), script.display().to_string()],
            out,
            &label,
            start,
        )?;
        if code != Some(0) || !stderr.is_empty() {
            return Err(format!("{method} producer diagnostics").into());
        }
        let wire = out.join(format!("{method}.json"));
        let (code, _, stderr) = call(
            &cli,
            &[
                "import".into(),
                input.display().to_string(),
                out.join(format!("{label}.stdout")).display().to_string(),
                wire.display().to_string(),
            ],
            out,
            &format!("{method}-import"),
            start,
        )?;
        if code != Some(0) || !stderr.is_empty() {
            return Err(format!("{method} import failed").into());
        }
        positive.push((
            method.into(),
            system.clone(),
            serde_json::from_slice(&fs::read(wire)?)?,
        ));
    }
    let canonical = |w: &Witness| -> BTreeSet<Poly> {
        w.basis
            .iter()
            .map(|row| {
                row.polynomial
                    .iter()
                    .map(|t| (t.exponents.clone(), t.coefficient))
                    .collect()
            })
            .collect()
    };
    let reference = canonical(&positive[0].2);
    if positive.iter().any(|(_, _, w)| canonical(w) != reference) {
        return Err("producer bases differ".into());
    }
    let original = positive[0].2.clone();
    let mut cases: Vec<(String, System, Witness, &str)> = positive
        .into_iter()
        .map(|(name, s, w)| (name, s, w, "accept"))
        .collect();
    let mut add = |name: &str, w: Witness, stage: &'static str| {
        cases.push((name.into(), system.clone(), w, stage))
    };
    let mut w = original.clone();
    w.definitions[0].proof[0].coefficient = if w.definitions[0].proof[0].coefficient == 1 {
        2
    } else {
        1
    };
    add("wrong-membership-coefficient", w, "arithmetic");
    let mut w = original.clone();
    w.definitions[0].proof[0].generator = system.polynomials.len();
    add("circular-reference", w, "shape");
    let mut w = original.clone();
    w.input_reductions.pop();
    add("missing-input", w, "shape");
    let mut w = original.clone();
    w.pair_reductions.pop();
    add("missing-pair", w, "shape");
    let mut w = original.clone();
    w.definitions[0].proof[0].exponents.push(0);
    add("malformed-exponent", w, "shape");
    let mut w = original.clone();
    w.definitions[0].proof[0].generator = usize::MAX;
    add("out-of-range-index", w, "shape");
    let mut w = original.clone();
    for t in &mut w.basis[0].polynomial {
        t.coefficient = t.coefficient * 2 % system.prime;
    }
    add("non-monic", w, "shape");
    let mut w = original.clone();
    w.basis.push(w.basis[0].clone());
    add("non-reduced", w, "shape");
    let (bad_system, w) = non_gb();
    cases.push((
        "non-gb-forged-zero-pair".into(),
        bad_system.clone(),
        w.clone(),
        "arithmetic",
    ));
    let mut w = w;
    w.pair_reductions[0].proof = vec![
        CofactorTerm {
            generator: 0,
            exponents: vec![0, 1, 0],
            coefficient: 1,
        },
        CofactorTerm {
            generator: 1,
            exponents: vec![1, 0, 0],
            coefficient: system.prime - 1,
        },
    ];
    cases.push((
        "non-gb-true-nonstandard-pair".into(),
        bad_system,
        w,
        "shape",
    ));
    for (name, s, w, expected) in cases {
        let dir = out.join(&name);
        fs::create_dir(&dir)?;
        let input = dir.join("input.json");
        let wire = dir.join("witness.json");
        let script = dir.join("check.sing");
        save(&input, &serde_json::to_vec_pretty(&s)?)?;
        save(&wire, &serde_json::to_vec_pretty(&w)?)?;
        let (code, _, stderr) = call(
            &cli,
            &[
                "script".into(),
                input.display().to_string(),
                wire.display().to_string(),
                script.display().to_string(),
            ],
            &dir,
            "script",
            start,
        )?;
        let (passed, detail) = if expected == "shape" {
            (
                code == Some(1) && stderr.starts_with("Error:"),
                json!({"consumer_exit":code,"rejection":stderr}),
            )
        } else if code != Some(0) {
            (
                false,
                json!({"unexpected_consumer_exit":code,"stderr":stderr}),
            )
        } else {
            let (code, stdout, stderr) = call(
                &singular,
                &["-q".into(), script.display().to_string()],
                &dir,
                "arithmetic",
                start,
            )?;
            let accepted =
                code == Some(0) && stderr.is_empty() && stdout.trim() == "CERTIFICATE_REPLAY_OK";
            let rejected =
                code == Some(0) && stderr.is_empty() && stdout.trim() == "REPLAY_FAIL:identity";
            (
                if expected == "accept" {
                    accepted
                } else {
                    rejected
                },
                json!({"arithmetic_exit":code,"accepted":accepted,"stdout":stdout,"stderr":stderr}),
            )
        };
        records.push(json!({"case":name,"expected":expected,"passed":passed,"detail":detail}));
    }
    let passed = records.iter().all(|r| r["passed"] == true);
    save(
        &out.join("summary.json"),
        &serde_json::to_vec_pretty(
            &json!({"schema":"replay-qualification/v1","all_passed":passed,"same_constructor_basis":true,"cases":records,"standing":"unsigned correctness/tamper qualification; no runtime claim"}),
        )?,
    )?;
    if !passed {
        return Err("replay qualification failed; retained all outcomes".into());
    }
    Ok(())
}
