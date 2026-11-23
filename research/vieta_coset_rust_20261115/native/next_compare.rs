//! New cold arithmetic comparison: descriptor/certification + necessary base
//! lifting + query. Fixtures and a complete scalar verifier are excluded and
//! recorded. All source algorithms are in the shared Rust library.
use num_bigint::BigUint;
use num_traits::One;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};
use vieta_coset_lab::vieta_coset::*;
const ARMS: [&str; 5] = ["newton", "half", "half-aa", "lookup", "lookup-fast"];
fn emit(v: Value) {
    if let Ok(directory) = std::env::var("VIETA_OUTPUT_DIR") {
        let name = v["input_index"]
            .as_u64()
            .map_or_else(|| "fixture.jsonl".into(), |i| format!("input-{i:04}.jsonl"));
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(std::path::Path::new(&directory).join(name))
            .unwrap();
        writeln!(file, "{v}").unwrap();
        file.flush().unwrap();
    }
    let mut out = io::stdout().lock();
    writeln!(out, "{v}").unwrap();
    out.flush().unwrap();
}
fn context<F: PrimeField + Clone>(f: &F, mode: PolynomialBackend, seconds: u64) -> Arithmetic<F> {
    Arithmetic::new(
        f.clone(),
        Limits {
            max_muls: 4_000_000_000,
            max_split_attempts: 65536,
            timeout: Duration::from_secs(seconds),
        },
    )
    .unwrap()
    .with_polynomials(mode)
}
enum Member<E> {
    Coset(Coset<E>),
    Tower(SplitTower<E>),
}
fn member<F: PrimeField>(a: &Arithmetic<F>, kind: &str, m: usize) -> Result<Member<F::E>> {
    if kind == "coset" {
        Ok(Member::Coset(a.coset(m, a.field.element(1))?))
    } else if kind == "rational" {
        Ok(Member::Tower(a.conjugate_square_tower(
            m.trailing_zeros(),
            a.field.element(3),
            42,
        )?))
    } else {
        Err(Error::Invalid("unknown membership representation"))
    }
}
fn base<F: PrimeField>(
    a: &Arithmetic<F>,
    curve: &Curve<F::E>,
    member: &Member<F::E>,
) -> Result<Vec<(F::E, F::E)>> {
    match member {
        Member::Coset(c) => a.factor_base(curve, c),
        Member::Tower(t) => a.tower_factor_base(curve, t),
    }
}
fn solve<F: PrimeField>(
    a: &Arithmetic<F>,
    curve: &Curve<F::E>,
    member: &Member<F::E>,
    target: &Point<F::E>,
    seed: u64,
) -> Result<Vec<Relation<F::E>>> {
    match member {
        Member::Coset(c) => a.solve(curve, c, target, seed),
        Member::Tower(t) => a.solve_tower(curve, t, target, seed),
    }
}
fn hash<F: PrimeField>(f: &F, relations: &[Relation<F::E>]) -> String {
    let mut h = Sha256::new();
    for (p, q) in relations {
        for c in [&p.0, &p.1, &q.0, &q.1] {
            let s = f.hex(c);
            h.update((s.len() as u64).to_le_bytes());
            h.update(s.as_bytes());
        }
    }
    format!("{:x}", h.finalize())
}
fn total(a: Work, b: Work) -> Value {
    let mut a = serde_json::to_value(a).unwrap();
    let b = serde_json::to_value(b).unwrap();
    for (k, v) in b.as_object().unwrap() {
        a[k] = json!(a[k].as_u64().unwrap() + v.as_u64().unwrap());
    }
    a
}
fn run<F: PrimeField + Clone>(
    f: F,
    curve: Curve<F::E>,
    name: &str,
    kind: &str,
    m: usize,
    rounds: usize,
    warmups: usize,
    seconds: u64,
    timed: bool,
) -> Result<()> {
    let fixture_start = Instant::now();
    let fixtures = context(&f, PolynomialBackend::KaratsubaNewton, seconds);
    let membership = member(&fixtures, kind, m)?;
    let fixture_base = base(&fixtures, &curve, &membership)?;
    if fixture_base.len() < 2 {
        return Err(Error::Invalid("fixture base too small"));
    }
    let panel = std::env::var("VIETA_PANEL").unwrap_or_else(|_| "next-development".into());
    let scope = json!({"panel":panel,"curve":name,"representation":kind,"m":m,"p_hex":f.modulus().to_str_radix(16),"a_hex":f.hex(&curve.a),"b_hex":f.hex(&curve.b),"base_points":fixture_base.len(),"fixture_seed_hex":format!("{:x}",0x686763642d7631u64^m as u64),"rounds":rounds,"warmups":warmups,"timed":timed,"seconds_per_stage":seconds,"max_muls_per_stage":4_000_000_000u64,"max_split_attempts":65536});
    emit(
        json!({"schema":"vieta-next/v1","kind":"fixture-construction-excluded","scope":scope,"work":fixtures.work(),"elapsed_ns":if timed{json!(fixture_start.elapsed().as_nanos().to_string())}else{Value::Null}}),
    );
    let mut seed = 0x686763642d7631u64 ^ m as u64;
    let mut failed = false;
    for i in 0..warmups + rounds {
        let draw = context(&f, PolynomialBackend::KaratsubaNewton, seconds);
        let draw_start = Instant::now();
        let target = if i % 2 == 0 {
            Some(draw.random_point(&curve, &mut seed)?)
        } else {
            draw.group_add(
                &curve,
                &Some(fixture_base[(i * 17 + 1) % fixture_base.len()].clone()),
                &Some(fixture_base[(i * 31 + 2) % fixture_base.len()].clone()),
            )?
        };
        let draw_ns = draw_start.elapsed().as_nanos().to_string();
        let verifier = context(&f, PolynomialBackend::Schoolbook, seconds);
        let verifier_start = Instant::now();
        let expected = verifier.lookup_scalar(&curve, &fixture_base, &target)?;
        let verify_ns = verifier_start.elapsed().as_nanos().to_string();
        let population = if i % 2 == 0 {
            "uniform-x-rational"
        } else {
            "conditioned-sum"
        };
        if i % 2 == 1 && expected.is_empty() {
            return Err(Error::Inconsistent("positive target missed"));
        }
        let point = target
            .as_ref()
            .map_or(Value::Null, |(x, y)| json!([f.hex(x), f.hex(y)]));
        emit(
            json!({"schema":"vieta-next/v1","kind":"verification-excluded","scope":scope,"input_index":i,"round":i.saturating_sub(warmups),"warmup":i<warmups,"population":population,"target":point,"relations":expected.len(),"result_sha256":hash(&f,&expected),"work":verifier.work(),"draw_work":draw.work(),"draw_ns":if timed{json!(draw_ns)}else{Value::Null},"elapsed_ns":if timed{json!(verify_ns)}else{Value::Null}}),
        );
        let mut order: Vec<_> = (0..ARMS.len()).map(|j| (j + i) % ARMS.len()).collect();
        if i % 2 == 1 {
            order.reverse();
        }
        for arm_index in order {
            let arm = ARMS[arm_index];
            let mode = if ["half", "half-aa", "lookup-fast"].contains(&arm) {
                PolynomialBackend::HalfGcd
            } else {
                PolynomialBackend::KaratsubaNewton
            };
            let construction = context(&f, PolynomialBackend::KaratsubaNewton, seconds);
            let start = Instant::now();
            let built = member(&construction, kind, m).and_then(|m| {
                let b = if arm.starts_with("lookup") {
                    base(&construction, &curve, &m)?
                } else {
                    vec![]
                };
                let set: HashSet<_> = b.iter().cloned().collect();
                Ok((m, b, set))
            });
            let construction_ns = start.elapsed().as_nanos().to_string();
            let construction_ok = built.is_ok();
            // The query's deadline starts after construction, not before it.
            let query = context(&f, mode, seconds);
            let begin = Instant::now();
            let result = match built {
                Ok((m, b, set)) => {
                    if arm.starts_with("lookup") {
                        query.lookup_batched(&curve, &b, &set, &target)
                    } else {
                        solve(&query, &curve, &m, &target, 0x48470000 + i as u64)
                    }
                }
                Err(e) => Err(e),
            };
            let query_ns = begin.elapsed().as_nanos().to_string();
            let elapsed_ns = start.elapsed().as_nanos().to_string();
            let (status, error, relations, digest) = match result {
                Ok(r) if r == expected => ("ok", Value::Null, json!(r.len()), json!(hash(&f, &r))),
                Ok(_) => {
                    failed = true;
                    (
                        "mismatch",
                        json!("complete relation set differs"),
                        Value::Null,
                        Value::Null,
                    )
                }
                Err(e) => {
                    failed = true;
                    ("failed", json!(format!("{e:?}")), Value::Null, Value::Null)
                }
            };
            emit(
                json!({"schema":"vieta-next/v1","kind":"query","scope":scope,"input_index":i,"arm":arm,"round":i.saturating_sub(warmups),"warmup":i<warmups,"population":population,"target":point,"status":status,"error":error,"error_stage":if status=="ok"{Value::Null}else{json!(if construction_ok{"query"}else{"construction"})},"relations":relations,"result_sha256":digest,"construction_work":construction.work(),"query_work":query.work(),"work":total(construction.work(),query.work()),"elapsed_ns":if timed{json!(elapsed_ns)}else{Value::Null},"construction_ns":if timed{json!(construction_ns)}else{Value::Null},"query_ns":if timed{json!(query_ns)}else{Value::Null},"wall_valid":false}),
            );
        }
    }
    if failed {
        Err(Error::Budget("one or more retained arm failures"))
    } else {
        Ok(())
    }
}
fn stats(mut x: Vec<f64>) -> Value {
    if x.is_empty() {
        return json!({"n":0});
    }
    x.sort_by(f64::total_cmp);
    let n = x.len();
    let median = if n.is_multiple_of(2) {
        (x[n / 2 - 1] + x[n / 2]) / 2.0
    } else {
        x[n / 2]
    };
    json!({"n":n,"min":x[0],"max":x[n-1],"median":median,"values":x})
}
fn summarize(paths: &[String]) {
    let mut groups: BTreeMap<String, BTreeMap<String, Vec<Value>>> = BTreeMap::new();
    let (mut attempted, mut succeeded) = (0, 0);
    for path in paths {
        for line in io::BufReader::new(std::fs::File::open(path).unwrap()).lines() {
            let r: Value = serde_json::from_str(&line.unwrap()).unwrap();
            if r["kind"] != "query" {
                continue;
            }
            attempted += 1;
            if r["status"] == "ok" {
                succeeded += 1;
            }
            if r["warmup"] == true {
                continue;
            }
            let s = &r["scope"];
            let key = format!(
                "{}:{}:m{}:{}:{}",
                s["representation"].as_str().unwrap(),
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
    let mut out = vec![];
    for (group, arms) in groups {
        let mut cells = BTreeMap::new();
        for (arm, rows) in &arms {
            let ok: Vec<_> = rows.iter().filter(|r| r["status"] == "ok").collect();
            let mut metrics = BTreeMap::new();
            for phase in ["work", "construction_work", "query_work"] {
                metrics.insert(phase,json!({"muls":stats(ok.iter().map(|r|r[phase]["muls"].as_u64().unwrap()as f64).collect()),"adds":stats(ok.iter().map(|r|r[phase]["adds"].as_u64().unwrap()as f64).collect()),"half_gcd_batches":stats(ok.iter().map(|r|r[phase]["half_gcd_batches"].as_u64().unwrap()as f64).collect())}));
            }
            cells.insert(arm,json!({"attempted":rows.len(),"succeeded":ok.len(),"failed":rows.len()-ok.len(),"metrics":metrics}));
        }
        let mut paired = BTreeMap::new();
        for (left, right) in [
            ("newton", "half"),
            ("half", "half-aa"),
            ("lookup", "lookup-fast"),
            ("half", "lookup"),
        ] {
            for phase in ["work", "query_work"] {
                let mut ratios = vec![];
                if let (Some(a), Some(b)) = (arms.get(left), arms.get(right)) {
                    for r in a.iter().filter(|r| r["status"] == "ok") {
                        if let Some(q) = b.iter().find(|q| {
                            q["status"] == "ok"
                                && q["round"] == r["round"]
                                && q["target"] == r["target"]
                        }) {
                            let den = q[phase]["muls"].as_u64().unwrap();
                            if den != 0 {
                                ratios.push(r[phase]["muls"].as_u64().unwrap() as f64 / den as f64);
                            }
                        }
                    }
                }
                paired.insert(format!("{phase}:{left}/{right}"), stats(ratios));
            }
        }
        out.push(json!({"group":group,"arms":cells,"paired_ratios":paired}));
    }
    emit(
        json!({"schema":"vieta-next-summary/v1","attempted":attempted,"succeeded":succeeded,"failed":attempted-succeeded,"groups":out,"standing":"producer descriptive counts [pcc:unverified]; clocks diagnostic, memory/hash/sort not field-priced"}),
    );
}
fn replay(paths: &[String]) {
    let mut cache: BTreeMap<String, (String, usize)> = BTreeMap::new();
    let (mut checked, mut failed) = (0, 0);
    for path in paths {
        for line in io::BufReader::new(std::fs::File::open(path).unwrap()).lines() {
            let row: Value = serde_json::from_str(&line.unwrap()).unwrap();
            if row["kind"] != "query" {
                continue;
            }
            if row["status"] != "ok" {
                failed += 1;
                continue;
            }
            let scope = &row["scope"];
            let key = format!(
                "{}:{}:{}:{}",
                scope["representation"], scope["curve"], scope["m"], row["target"]
            );
            if !cache.contains_key(&key) {
                let parse =
                    |v: &Value| BigUint::parse_bytes(v.as_str().unwrap().as_bytes(), 16).unwrap();
                let field = WideField(parse(&scope["p_hex"]));
                let a = context(&field, PolynomialBackend::Schoolbook, 120);
                let curve = Curve {
                    a: parse(&scope["a_hex"]),
                    b: parse(&scope["b_hex"]),
                };
                let membership = member(
                    &a,
                    scope["representation"].as_str().unwrap(),
                    scope["m"].as_u64().unwrap() as usize,
                )
                .unwrap();
                let b = base(&a, &curve, &membership).unwrap();
                let target = if row["target"].is_null() {
                    None
                } else {
                    Some((parse(&row["target"][0]), parse(&row["target"][1])))
                };
                let r = a.lookup_scalar(&curve, &b, &target).unwrap();
                cache.insert(key.clone(), (hash(&field, &r), r.len()));
            }
            let (digest, n) = &cache[&key];
            assert_eq!(row["result_sha256"], *digest, "replayed digest mismatch");
            assert_eq!(row["relations"], *n);
            checked += 1;
        }
    }
    emit(
        json!({"schema":"vieta-next-replay/v1","checked":checked,"preserved_failed_rows":failed,"distinct_inputs":cache.len(),"standing":"producer wide scalar replay; independent signed standing pending [pcc:unverified]"}),
    );
    if failed > 0 {
        std::process::exit(1);
    }
}
fn forecast(paths: &[String]) {
    let mut groups = BTreeMap::<String, (f64, BTreeMap<u64, f64>, usize)>::new();
    for path in paths {
        for line in io::BufReader::new(std::fs::File::open(path).unwrap()).lines() {
            let r: Value = serde_json::from_str(&line.unwrap()).unwrap();
            let s = &r["scope"];
            let group = format!(
                "{}:{}:m{}",
                s["representation"].as_str().unwrap(),
                s["curve"].as_str().unwrap(),
                s["m"]
            );
            let (fixed, input, errors) = groups.entry(group).or_default();
            let seconds = |key: &str| {
                r[key]
                    .as_str()
                    .map(|s| s.parse::<f64>().unwrap() / 1e9)
                    .unwrap_or(0.0)
            };
            if r["kind"] == "fixture-construction-excluded" {
                *fixed += seconds("elapsed_ns");
            }
            if r["kind"] == "verification-excluded" {
                *input.entry(r["round"].as_u64().unwrap()).or_default() +=
                    seconds("elapsed_ns") + seconds("draw_ns");
            }
            if r["kind"] == "query" {
                *input.entry(r["round"].as_u64().unwrap()).or_default() += seconds("elapsed_ns");
                if r["status"] != "ok" {
                    *errors += 1;
                }
            }
        }
    }
    let mut output = vec![];
    let mut grand = 0.0;
    for (group, (fixed, input, errors)) in groups {
        let worst = input.values().copied().fold(0.0, f64::max);
        let estimate = fixed + 8.0 * worst;
        if errors == 0 {
            grand += estimate;
        }
        output.push(json!({"group":group,"fixed_seconds":fixed,"observed_input_seconds":input,"max_input_seconds":worst,"planned_primary_inputs":8,"forecast_seconds":if errors==0{Some(estimate)}else{None},"failed_calibration_calls":errors}));
    }
    emit(
        json!({"schema":"vieta-next-preflight-forecast/v1","configs":output,"sum_of_complete_class_point_forecasts_seconds":grand,"method":"fixed+8*max(observed two input costs), includes five arms/draw/verifier; diagnostic clocks, not an interval or guarantee","hard_primary_envelope_seconds":3600}),
    );
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).is_some_and(|s| s == "summarize") {
        summarize(&args[2..]);
        return;
    }
    if args.get(1).is_some_and(|s| s == "replay") {
        replay(&args[2..]);
        return;
    }
    if args.get(1).is_some_and(|s| s == "forecast") {
        forecast(&args[2..]);
        return;
    }
    if args.len() != 9 || !["check", "bench"].contains(&args[1].as_str()) {
        eprintln!(
            "next-compare check|bench coset|rational word62|p224 m rounds warmups seconds v1"
        );
        std::process::exit(2);
    }
    let timed = args[1] == "bench";
    if timed && std::env::var("VIETA_ISOLATED_RUN").as_deref() != Ok("1") {
        eprintln!("isolated launcher required");
        std::process::exit(2);
    }
    let parse = |i: usize| args[i].parse::<usize>().expect("integer argument");
    let (m, rounds, warmups, seconds) = (parse(4), parse(5), parse(6), parse(7));
    if args[8] != "v1"
        || !m.is_power_of_two()
        || m > 16384
        || !(1..=100).contains(&rounds)
        || warmups > 2
        || !(1..=120).contains(&seconds)
    {
        eprintln!("invalid limits");
        std::process::exit(2);
    }
    let r = match args[3].as_str() {
        "word62" => run(
            WordField(4179340454199820289),
            Curve { a: 2, b: 3 },
            "word62",
            &args[2],
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
                Curve { a: &p - 3u8, b },
                "p224",
                &args[2],
                m,
                rounds,
                warmups,
                seconds as u64,
                timed,
            )
        }
        _ => Err(Error::Invalid("unknown field")),
    };
    if let Err(e) = r {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}
