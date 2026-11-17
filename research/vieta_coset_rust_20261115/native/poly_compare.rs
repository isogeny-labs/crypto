//! Same-input backend comparison; all solving is in the shared Rust library.
use num_bigint::BigUint;
use num_traits::One;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};
use vieta_coset_lab::vieta_coset::*;

const ARMS: [&str; 6] = [
    "schoolbook",
    "karatsuba",
    "newton",
    "newton-aa",
    "lookup",
    "lookup-fast",
];
fn emit(v: Value) {
    let mut out = io::stdout().lock();
    writeln!(out, "{v}").unwrap();
    out.flush().unwrap();
}
fn context<F: PrimeField + Clone>(f: &F, mode: PolynomialBackend, seconds: u64) -> Arithmetic<F> {
    Arithmetic::new(
        f.clone(),
        Limits {
            max_muls: 4_000_000_000,
            timeout: Duration::from_secs(seconds),
            ..Limits::default()
        },
    )
    .unwrap()
    .with_polynomials(mode)
}
fn hash<F: PrimeField>(f: &F, r: &[Relation<F::E>]) -> String {
    let mut h = Sha256::new();
    for (p, q) in r {
        for c in [&p.0, &p.1, &q.0, &q.1] {
            let s = f.hex(c);
            h.update((s.len() as u64).to_le_bytes());
            h.update(s.as_bytes());
        }
    }
    format!("{:x}", h.finalize())
}
#[allow(clippy::too_many_arguments)]
fn run<F: PrimeField + Clone>(
    f: F,
    curve: Curve<F::E>,
    name: &str,
    m: usize,
    rounds: usize,
    warmups: usize,
    seconds: u64,
    timed: bool,
) -> Result<()> {
    let panel = std::env::var("VIETA_PANEL").unwrap_or_else(|_| "poly-development".into());
    let descriptor = context(&f, PolynomialBackend::Schoolbook, seconds);
    let start = Instant::now();
    let coset = descriptor.coset(m, f.element(1))?;
    let descriptor_ns = start.elapsed().as_nanos().to_string();
    let setup = context(&f, PolynomialBackend::Schoolbook, seconds);
    let start = Instant::now();
    let base = setup.factor_base(&curve, &coset)?;
    let set: HashSet<_> = base.iter().cloned().collect();
    let setup_ns = start.elapsed().as_nanos().to_string();
    if base.len() < 2 {
        return Err(Error::Invalid("factor base too small"));
    }
    let base_hash = hash(
        &f,
        &base
            .iter()
            .map(|p| (p.clone(), p.clone()))
            .collect::<Vec<_>>(),
    );
    let fixture_seed = 0x76696574612d7631u64 ^ m as u64;
    let scope = json!({"panel":panel,"curve":name,"p_hex":f.modulus().to_str_radix(16),"a_hex":f.hex(&curve.a),"b_hex":f.hex(&curve.b),"m":m,"c_hex":"1","base_points":base.len(),"base_sha256":base_hash,"timed":timed,"rounds":rounds,"warmups":warmups,"fixture_seed_hex":format!("{fixture_seed:x}"),"seconds_per_stage":seconds,"max_muls":4_000_000_000u64});
    for (kind, work, ns) in [
        ("descriptor", descriptor.work(), descriptor_ns),
        ("setup", setup.work(), setup_ns),
    ] {
        emit(
            json!({"schema":"vieta-coset/v1","kind":kind,"scope":scope,"work":work,"elapsed_ns":if timed{json!(ns)}else{Value::Null}}),
        );
    }
    let mut seed = fixture_seed;
    let mut any_error = false;
    for i in 0..warmups + rounds {
        // A fresh excluded fixture context per input; it never inherits arm time.
        let fixtures = context(&f, PolynomialBackend::Schoolbook, seconds);
        let target = if i % 2 == 0 {
            Some(fixtures.random_point(&curve, &mut seed)?)
        } else {
            fixtures.group_add(
                &curve,
                &Some(base[(i * 17 + 1) % base.len()].clone()),
                &Some(base[(i * 31 + 2) % base.len()].clone()),
            )?
        };
        let population = if i % 2 == 0 {
            "uniform-x-rational"
        } else {
            "conditioned-sum"
        };
        let verifier = context(&f, PolynomialBackend::Schoolbook, seconds);
        let start = Instant::now();
        let expected = verifier.lookup_scalar(&curve, &base, &target)?;
        let verify_ns = start.elapsed().as_nanos().to_string();
        if i % 2 != 0 && expected.is_empty() {
            return Err(Error::Inconsistent("positive fixture missed"));
        }
        let point = target
            .as_ref()
            .map_or(Value::Null, |(x, y)| json!([f.hex(x), f.hex(y)]));
        emit(
            json!({"schema":"vieta-coset/v1","kind":"verification","scope":scope,"target":point,"round":i.saturating_sub(warmups),"warmup":i<warmups,"population":population,"relations":expected.len(),"result_sha256":hash(&f,&expected),"work":verifier.work(),"elapsed_ns":if timed{json!(verify_ns)}else{Value::Null}}),
        );
        let mut order: Vec<_> = (0..ARMS.len()).map(|j| (j + i) % ARMS.len()).collect();
        if i % 2 != 0 {
            order.reverse();
        }
        for (position, arm_index) in order.into_iter().enumerate() {
            let arm = ARMS[arm_index];
            let mode = match arm {
                "karatsuba" => PolynomialBackend::Karatsuba,
                "newton" | "newton-aa" | "lookup-fast" => PolynomialBackend::KaratsubaNewton,
                _ => PolynomialBackend::Schoolbook,
            };
            let a = context(&f, mode, seconds);
            let start = Instant::now();
            let result = if arm.starts_with("lookup") {
                a.lookup_batched(&curve, &base, &set, &target)
            } else {
                a.solve(&curve, &coset, &target, 42 + i as u64)
            };
            let elapsed = start.elapsed().as_nanos().to_string();
            let (status, error, relations, digest) = match result {
                Ok(r) if r == expected => ("ok", Value::Null, json!(r.len()), json!(hash(&f, &r))),
                Ok(r) => {
                    any_error = true;
                    (
                        "mismatch",
                        json!("complete set mismatch"),
                        json!(r.len()),
                        json!(hash(&f, &r)),
                    )
                }
                Err(e) => {
                    any_error = true;
                    ("error", json!(format!("{e:?}")), Value::Null, Value::Null)
                }
            };
            emit(
                json!({"schema":"vieta-coset/v1","kind":"query","scope":scope,"arm":arm,"backend":mode,"position":position,"round":i.saturating_sub(warmups),"warmup":i<warmups,"population":population,"target":point,"status":status,"error":error,"relations":relations,"result_sha256":digest,"work":a.work(),"elapsed_ns":if timed{json!(elapsed)}else{Value::Null}}),
            );
        }
    }
    if any_error {
        Err(Error::Inconsistent("retained failed arm rows"))
    } else {
        Ok(())
    }
}
fn stats(mut x: Vec<f64>) -> Value {
    x.sort_by(f64::total_cmp);
    if x.is_empty() {
        return json!({"n":0});
    }
    let n = x.len();
    let median = if n % 2 == 1 {
        x[n / 2]
    } else {
        (x[n / 2 - 1] + x[n / 2]) / 2.0
    };
    json!({"n":n,"median":median,"min":x[0],"max":x[n-1],"values":x})
}
fn summarize(paths: &[String]) {
    let mut groups: BTreeMap<String, BTreeMap<String, Vec<Value>>> = BTreeMap::new();
    let mut planned_rows = 0;
    for path in paths {
        for line in io::BufReader::new(std::fs::File::open(path).unwrap()).lines() {
            let r: Value = serde_json::from_str(&line.unwrap()).unwrap();
            if r["kind"] != "query" {
                continue;
            }
            planned_rows += 1;
            if r["warmup"] == true {
                continue;
            }
            let s = &r["scope"];
            let key = format!(
                "{}:{}:m{}:{}:{}",
                s["panel"].as_str().unwrap(),
                s["curve"].as_str().unwrap(),
                s["m"],
                r["population"].as_str().unwrap(),
                if r["target"].is_null() {
                    "identity"
                } else {
                    "finite"
                }
            );
            groups
                .entry(key)
                .or_default()
                .entry(r["arm"].as_str().unwrap().into())
                .or_default()
                .push(r);
        }
    }
    let mut output = vec![];
    for (key, arms) in groups {
        let mut cells = BTreeMap::new();
        for (arm, rows) in &arms {
            let ok: Vec<_> = rows.iter().filter(|r| r["status"] == "ok").collect();
            cells.insert(arm.clone(),json!({"attempted":rows.len(),"succeeded":ok.len(),"failed":rows.len()-ok.len(),"muls":stats(ok.iter().map(|r|r["work"]["muls"].as_u64().unwrap() as f64).collect()),"adds":stats(ok.iter().map(|r|r["work"]["adds"].as_u64().unwrap() as f64).collect()),"newton_divisions":stats(ok.iter().map(|r|r["work"]["newton_divisions"].as_u64().unwrap() as f64).collect()),"wall_seconds_diagnostic":stats(ok.iter().filter_map(|r|r["elapsed_ns"].as_str()).map(|x|x.parse::<f64>().unwrap()/1e9).collect())}));
        }
        let mut paired = BTreeMap::new();
        for (left, right) in [
            ("schoolbook", "karatsuba"),
            ("schoolbook", "newton"),
            ("newton", "newton-aa"),
            ("lookup", "lookup-fast"),
            ("newton", "lookup"),
        ] {
            let mut ratios = vec![];
            if let (Some(a), Some(b)) = (arms.get(left), arms.get(right)) {
                for r in a.iter().filter(|r| r["status"] == "ok") {
                    if let Some(q) = b.iter().find(|q| {
                        q["status"] == "ok"
                            && q["round"] == r["round"]
                            && q["target"] == r["target"]
                    }) {
                        ratios.push(
                            r["work"]["muls"].as_u64().unwrap() as f64
                                / q["work"]["muls"].as_u64().unwrap() as f64,
                        );
                    }
                }
            }
            paired.insert(format!("{left}/{right}"), stats(ratios));
        }
        output.push(json!({"group":key,"arms":cells,"paired_multiplication_ratios":paired}));
    }
    emit(
        json!({"schema":"vieta-polynomial-summary/v1","attempted_query_rows_including_warmups":planned_rows,"groups":output,"standing":"producer descriptive analysis [pcc:unverified]; all wall clocks diagnostic below L2"}),
    );
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).is_some_and(|x| x == "summarize") {
        summarize(&args[2..]);
        return;
    }
    if args.len() != 8 || !["check", "bench"].contains(&args[1].as_str()) {
        eprintln!("poly-compare check|bench word62|p224 m rounds warmups seconds v1");
        std::process::exit(2);
    }
    let timed = args[1] == "bench";
    if timed && std::env::var("VIETA_ISOLATED_RUN").as_deref() != Ok("1") {
        eprintln!("isolated launcher required");
        std::process::exit(2);
    }
    // Keep explicit input limits and a versioned fixture policy.
    let parse = |i: usize| args[i].parse::<usize>().expect("integer argument");
    let (m, rounds, warmups, seconds) = (parse(3), parse(4), parse(5), parse(6));
    // Last argument binds the fixed fixture policy and catches accidental CLI drift.
    if args[7] != "v1"
        || !(1..=100).contains(&rounds)
        || warmups > 2
        || !(1..=120).contains(&seconds)
    {
        eprintln!("invalid limits or fixture policy");
        std::process::exit(2);
    }
    let result = match args[2].as_str() {
        "word62" => run(
            WordField(4179340454199820289),
            Curve { a: 2, b: 3 },
            "word62",
            m,
            rounds,
            warmups,
            seconds as u64,
            timed,
        ),
        "p224" => {
            let p = (BigUint::one() << 224usize) - (BigUint::one() << 96usize) + 1u8;
            let b = BigUint::parse_bytes(
                b"b4050a850c04b3abf54132565044b0b7d7bfd8ba270b39432355ffb4",
                16,
            )
            .unwrap();
            run(
                WideField(p.clone()),
                Curve { a: p - 3u8, b },
                "p224",
                m,
                rounds,
                warmups,
                seconds as u64,
                timed,
            )
        }
        _ => Err(Error::Invalid("unknown curve")),
    };
    if let Err(e) = result {
        emit(json!({"kind":"fatal","error":format!("{e:?}")}));
        std::process::exit(1);
    }
}
