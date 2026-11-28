//! Fresh-process timing of the unchanged shared counted Rust implementation.
use num_bigint::BigUint;
use num_traits::One;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};
use vieta_coset_lab::vieta_coset::*;

const ARMS: [&str; 6] = [
    "school",
    "newton",
    "half",
    "half-aa",
    "lookup",
    "half-double",
];
#[derive(Clone, Serialize, Deserialize)]
struct Config {
    panel: String,
    field: String,
    representation: String,
    m: usize,
    rounds: usize,
    warmups: usize,
    seconds: u64,
    seed: u64,
}
trait Decode: PrimeField {
    fn decode(&self, x: &str) -> Self::E;
}
impl Decode for WordField {
    fn decode(&self, x: &str) -> u64 {
        u64::from_str_radix(x, 16).unwrap()
    }
}
impl Decode for WideField {
    fn decode(&self, x: &str) -> BigUint {
        BigUint::parse_bytes(x.as_bytes(), 16).unwrap()
    }
}
fn ctx<F: PrimeField + Clone>(f: &F, backend: PolynomialBackend, seconds: u64) -> Arithmetic<F> {
    Arithmetic::new(
        f.clone(),
        Limits {
            max_muls: 4_000_000_000,
            max_split_attempts: 65536,
            timeout: Duration::from_secs(seconds),
        },
    )
    .unwrap()
    .with_polynomials(backend)
}
enum Member<E> {
    Coset(Coset<E>),
    Tower(SplitTower<E>),
}
fn member<F: PrimeField>(a: &Arithmetic<F>, c: &Config) -> Result<Member<F::E>> {
    match c.representation.as_str() {
        "coset" => Ok(Member::Coset(a.coset(c.m, a.field.element(1))?)),
        "rational" => Ok(Member::Tower(a.conjugate_square_tower(
            c.m.trailing_zeros(),
            a.field.element(3),
            42,
        )?)),
        _ => Err(Error::Invalid("representation")),
    }
}
fn base<F: PrimeField>(
    a: &Arithmetic<F>,
    e: &Curve<F::E>,
    m: &Member<F::E>,
) -> Result<Vec<(F::E, F::E)>> {
    match m {
        Member::Coset(m) => a.factor_base(e, m),
        Member::Tower(m) => a.tower_factor_base(e, m),
    }
}
fn solve<F: PrimeField>(
    a: &Arithmetic<F>,
    e: &Curve<F::E>,
    m: &Member<F::E>,
    r: &Point<F::E>,
    seed: u64,
) -> Result<Vec<Relation<F::E>>> {
    match m {
        Member::Coset(m) => a.solve(e, m, r, seed),
        Member::Tower(m) => a.solve_tower(e, m, r, seed),
    }
}
fn hex_relations<F: PrimeField>(f: &F, r: &[Relation<F::E>]) -> Value {
    json!(r
        .iter()
        .map(|(p, q)| vec![f.hex(&p.0), f.hex(&p.1), f.hex(&q.0), f.hex(&q.1)])
        .collect::<Vec<_>>())
}
fn digest(v: &Value) -> String {
    format!("{:x}", Sha256::digest(serde_json::to_vec(v).unwrap()))
}
fn plus(a: Work, b: Work) -> Value {
    let mut x = serde_json::to_value(a).unwrap();
    let y = serde_json::to_value(b).unwrap();
    for (k, v) in y.as_object().unwrap() {
        x[k] = json!(x[k].as_u64().unwrap() + v.as_u64().unwrap());
    }
    x
}
fn point<F: Decode>(f: &F, v: &Value) -> Point<F::E> {
    if v.is_null() {
        None
    } else {
        Some((
            f.decode(v[0].as_str().unwrap()),
            f.decode(v[1].as_str().unwrap()),
        ))
    }
}
fn worker<F: Decode + Clone>(
    f: F,
    e: Curve<F::E>,
    c: &Config,
    target: &Value,
    arm: &str,
    seed: u64,
) -> Value {
    let backend = match arm {
        "school" => PolynomialBackend::Schoolbook,
        "newton" | "lookup" => PolynomialBackend::KaratsubaNewton,
        "half" | "half-aa" | "half-double" => PolynomialBackend::HalfGcd,
        _ => panic!("unknown arm"),
    };
    let r = point(&f, target);
    let cold = Instant::now();
    let construction = ctx(&f, PolynomialBackend::KaratsubaNewton, c.seconds);
    let built = member(&construction, c).and_then(|m| {
        let b = base(&construction, &e, &m)?;
        let set: HashSet<_> = if arm == "lookup" {
            b.iter().cloned().collect()
        } else {
            HashSet::new()
        };
        Ok((m, b, set))
    });
    let construction_ns = cold.elapsed().as_nanos().to_string();
    let construction_ok = built.is_ok();
    let construction_error = built.as_ref().err().map(|e| format!("{e:?}"));
    let query = ctx(&f, backend, c.seconds);
    let begin = Instant::now();
    // Keep setup alive through both clocks; its teardown is excluded explicitly.
    let result = built
        .as_ref()
        .map_err(|_| Error::Invalid("construction failed"))
        .and_then(|(m, b, set)| {
            if arm == "lookup" {
                query.lookup_batched(&e, b, set, &r)
            } else {
                let answer = solve(&query, &e, m, &r, seed)?;
                if arm == "half-double" {
                    let again = solve(&query, &e, m, &r, seed)?;
                    if answer != again {
                        return Err(Error::Inconsistent("positive repeat differs"));
                    }
                    std::hint::black_box(again);
                }
                Ok(answer)
            }
        });
    let query_ns = begin.elapsed().as_nanos().to_string();
    let cold_ns = cold.elapsed().as_nanos().to_string();
    let (status, error, relations) = match result {
        Ok(r) => ("ok", Value::Null, hex_relations(&f, &r)),
        Err(e) => ("failed", json!(format!("{e:?}")), Value::Null),
    };
    json!({"schema":"vieta-speed/v1","kind":"query","arm":arm,"status":status,"error":error,
        "construction_ok":construction_ok,"construction_error":construction_error,"construction_ns":construction_ns,"query_ns":query_ns,
        "cold_ns":cold_ns,"construction_work":construction.work(),"query_work":query.work(),
        "total_work":plus(construction.work(),query.work()),"relations":relations,
        "result_sha256":if status=="ok"{json!(digest(&relations))}else{Value::Null}})
}
fn p224() -> (WideField, Curve<BigUint>) {
    let p = (BigUint::one() << 224usize) - (BigUint::one() << 96usize) + 1u8;
    let b = BigUint::parse_bytes(
        b"b4050a850c04b3abf54132565044b0b7d7bfd8ba270b39432355ffb4",
        16,
    )
    .unwrap();
    (WideField(p.clone()), Curve { a: &p - 3u8, b })
}
fn write_new(path: &Path, v: &Value) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    writeln!(file, "{v}").unwrap();
    file.sync_all().unwrap();
}
fn arm_order(i: usize, warmups: usize) -> Vec<usize> {
    let n = i.saturating_sub(warmups);
    let mut order: Vec<_> = [0, 1, 5, 2, 4, 3].iter().map(|j| (j + n) % 6).collect();
    if (n / 6) % 2 == 1 {
        order.reverse();
    }
    if i == 0 {
        order.retain(|j| *j != 5);
        order.insert(0, 5);
    }
    order
}
fn panel<F: Decode + Clone>(f: F, e: Curve<F::E>, c: &Config, output: &Path) {
    fs::create_dir(output).unwrap();
    let fixture_begin = Instant::now();
    let a = ctx(&f, PolynomialBackend::KaratsubaNewton, c.seconds);
    let membership = member(&a, c).unwrap();
    let b = base(&a, &e, &membership).unwrap();
    assert!(b.len() > 1);
    let scope = json!({"panel":c.panel,"field":c.field,"curve":c.field,"representation":c.representation,"m":c.m,"p_hex":f.modulus().to_str_radix(16),
        "a_hex":f.hex(&e.a),"b_hex":f.hex(&e.b),"base_points":b.len(),"rounds":c.rounds,"warmups":c.warmups,
        "seconds_per_stage":c.seconds,"max_muls_per_stage":4_000_000_000u64,"seed":c.seed,
        "construction":"full descriptor/split certification and full point base per arm; lookup additionally indexes"});
    write_new(
        &output.join("fixture.json"),
        &json!({"scope":scope,"excluded_work":a.work(),"fixture_ns":fixture_begin.elapsed().as_nanos().to_string()}),
    );
    let mut seed = c.seed;
    for i in 0..c.warmups + c.rounds {
        let preparation_begin = Instant::now();
        let generator = ctx(&f, PolynomialBackend::KaratsubaNewton, c.seconds);
        let r = if i % 2 == 0 {
            Some(generator.random_point(&e, &mut seed).unwrap())
        } else {
            generator
                .group_add(
                    &e,
                    &Some(b[(17 * i + 1) % b.len()].clone()),
                    &Some(b[(31 * i + 2) % b.len()].clone()),
                )
                .unwrap()
        };
        let verifier = ctx(&f, PolynomialBackend::Schoolbook, c.seconds);
        let expected = hex_relations(&f, &verifier.lookup_scalar(&e, &b, &r).unwrap());
        if i % 2 == 1 {
            assert!(
                !expected.as_array().unwrap().is_empty(),
                "positive target missed"
            );
        }
        let target = r
            .as_ref()
            .map_or(Value::Null, |(x, y)| json!([f.hex(x), f.hex(y)]));
        let population = if i % 2 == 0 {
            "seeded-random-abscissa"
        } else {
            "conditioned-sum"
        };
        write_new(
            &output.join(format!("input-{i:03}.json")),
            &json!({"scope":scope,"target":target,"expected":expected,
            "result_sha256":digest(&expected),"input_index":i,"warmup":i<c.warmups,"population":population,"excluded_verifier_work":verifier.work(),"preparation_ns":preparation_begin.elapsed().as_nanos().to_string()}),
        );
        let order = arm_order(i, c.warmups);
        for arm in order.into_iter().map(|j| ARMS[j]) {
            let process_begin = Instant::now();
            let result = Command::new("timeout")
                .args([
                    "--signal=TERM",
                    "--kill-after=5s",
                    &(2 * c.seconds + 15).to_string(),
                ])
                .arg(std::env::current_exe().unwrap())
                .arg("worker")
                .arg(serde_json::to_string(c).unwrap())
                .arg(target.to_string())
                .arg(arm)
                .arg((0x53500000u64 + i as u64).to_string())
                .env("RAYON_NUM_THREADS", "1")
                .output()
                .unwrap();
            let process_ns = process_begin.elapsed().as_nanos().to_string();
            let prefix = format!("cell-{i:03}-{arm}");
            fs::write(output.join(format!("{prefix}.stderr")), &result.stderr).unwrap();
            let mut row:Value=serde_json::from_slice(&result.stdout).unwrap_or_else(|_|json!({"kind":"query","status":"worker-no-row","stdout":String::from_utf8_lossy(&result.stdout)}));
            row["scope"] = scope.clone();
            row["target"] = target.clone();
            row["arm"] = json!(arm);
            row["input_index"] = json!(i);
            row["warmup"] = json!(i < c.warmups);
            row["population"] = json!(population);
            row["process_exit"] = json!(result.status.code());
            row["process_ns"] = json!(process_ns);
            row["round"] = json!(i.saturating_sub(c.warmups));
            if !result.status.success() && row["status"] == "ok" {
                row["status"] = json!("worker-exit-failed");
            }
            row["complete_set_matches"] =
                json!(row["relations"] == expected && row["result_sha256"] == digest(&expected));
            if row["status"] == "ok" && row["complete_set_matches"] != true {
                row["status"] = json!("mismatch");
            }
            write_new(&output.join(format!("{prefix}.json")), &row);
            println!(
                "{{\"input\":{i},\"arm\":\"{arm}\",\"status\":{}}}",
                row["status"]
            );
            std::io::stdout().flush().unwrap();
            if row["status"] == "mismatch"
                || (i == 0 && arm == "half-double" && row["status"] != "ok")
            {
                panic!("invalid instrument/control; retained failed row");
            }
        }
    }
}
fn stats(mut xs: Vec<f64>) -> Value {
    if xs.is_empty() {
        return json!({"n":0});
    }
    xs.sort_by(f64::total_cmp);
    let n = xs.len();
    let med = if n.is_multiple_of(2) {
        (xs[n / 2 - 1] + xs[n / 2]) / 2.0
    } else {
        xs[n / 2]
    };
    let mut state = 0x424f4f5453545241u64 ^ n as u64;
    let mut boot = Vec::with_capacity(4096);
    for _ in 0..4096 {
        let mut sample = Vec::with_capacity(n);
        for _ in 0..n {
            state = state.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            sample.push(xs[((z ^ (z >> 31)) % n as u64) as usize]);
        }
        sample.sort_by(f64::total_cmp);
        boot.push(if n.is_multiple_of(2) {
            (sample[n / 2 - 1] + sample[n / 2]) / 2.0
        } else {
            sample[n / 2]
        });
    }
    boot.sort_by(f64::total_cmp);
    json!({"n":n,"min":xs[0],"median":med,"max":xs[n-1],"values":xs,
        "bootstrap_median95":[boot[102],boot[3993]],"bootstrap_replicates":4096,
        "uncertainty_scope":"descriptive percentile resampling conditional on these sampled targets; small-N, no p-value or population/exponent inference"})
}
fn summarize(paths: &[String]) {
    let mut groups: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let (mut total, mut ok, mut warm) = (0, 0, 0);
    for path in paths {
        for file in fs::read_dir(path).unwrap() {
            let path = file.unwrap().path();
            if !path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("cell-")
                || path.extension().is_none_or(|e| e != "json")
            {
                continue;
            }
            let r: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
            total += 1;
            if r["status"] == "ok" {
                ok += 1;
            }
            if r["warmup"] == true {
                warm += 1;
                continue;
            }
            let s = &r["scope"];
            let key = format!(
                "{}:{}:m{}:{}:{}",
                s["representation"].as_str().unwrap(),
                s["field"].as_str().unwrap(),
                s["m"],
                r["population"].as_str().unwrap(),
                if r["target"].is_null() {
                    "identity"
                } else {
                    "finite"
                }
            );
            groups.entry(key).or_default().push(r);
        }
    }
    let mut output = Vec::new();
    for (key, rows) in groups {
        let mut metrics = BTreeMap::new();
        for arm in ARMS {
            let ar: Vec<_> = rows.iter().filter(|r| r["arm"] == arm).collect();
            let good: Vec<_> = ar.iter().filter(|r| r["status"] == "ok").collect();
            let mut v =
                json!({"attempted":ar.len(),"successful":good.len(),"failed":ar.len()-good.len()});
            for phase in ["construction_ns", "query_ns", "cold_ns"] {
                v[phase] = stats(
                    good.iter()
                        .map(|r| r[phase].as_str().unwrap().parse::<f64>().unwrap())
                        .collect(),
                );
            }
            for phase in ["construction_work", "query_work", "total_work"] {
                v[format!("{phase}_muls")] = stats(
                    good.iter()
                        .map(|r| r[phase]["muls"].as_u64().unwrap() as f64)
                        .collect(),
                );
            }
            metrics.insert(arm.to_string(), v);
        }
        let mut ratios = BTreeMap::new();
        for (num, den) in [
            ("school", "half"),
            ("newton", "half"),
            ("lookup", "half"),
            ("half", "half-aa"),
            ("half-double", "half"),
        ] {
            for phase in ["query_ns", "cold_ns", "query_muls", "cold_muls"] {
                let mut values = Vec::new();
                for r in rows
                    .iter()
                    .filter(|r| r["arm"] == num && r["status"] == "ok")
                {
                    if let Some(s) = rows.iter().find(|s| {
                        s["arm"] == den
                            && s["input_index"] == r["input_index"]
                            && s["status"] == "ok"
                    }) {
                        let value = |r: &Value| match phase {
                            "query_muls" => r["query_work"]["muls"].as_u64().unwrap() as f64,
                            "cold_muls" => r["total_work"]["muls"].as_u64().unwrap() as f64,
                            _ => r[phase].as_str().unwrap().parse::<f64>().unwrap(),
                        };
                        let a = value(r);
                        let b = value(s);
                        if b > 0.0 {
                            values.push(a / b);
                        }
                    }
                }
                ratios.insert(format!("{num}/{den}:{phase}"), stats(values));
            }
        }
        output.push(json!({"group":key,"arms":metrics,"paired_ratios":ratios}));
    }
    println!(
        "{}",
        json!({"schema":"vieta-speed-summary/v1","attempted":total,"successful":ok,"failed":total-ok,"warmup_calls":warm,"groups":output,"timing_standing":"local diagnostic; subject to isolation/A-A audit, no L2 admission or independent signed standing"})
    );
}
fn forecast(paths: &[String], rounds: usize, warmups: usize) {
    let mut classes = Vec::new();
    let mut total = 0.0;
    for path in paths {
        let fixture: Value =
            serde_json::from_slice(&fs::read(Path::new(path).join("fixture.json")).unwrap())
                .unwrap();
        let fixed = fixture["fixture_ns"]
            .as_str()
            .unwrap()
            .parse::<f64>()
            .unwrap()
            / 1e9;
        let mut inputs: BTreeMap<u64, f64> = BTreeMap::new();
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap().path();
            let name = entry.file_name().unwrap().to_string_lossy();
            if entry.extension().is_none_or(|x| x != "json") {
                continue;
            }
            if name.starts_with("input-") || name.starts_with("cell-") {
                let r: Value = serde_json::from_slice(&fs::read(&entry).unwrap()).unwrap();
                let value = if name.starts_with("input-") {
                    &r["preparation_ns"]
                } else {
                    &r["process_ns"]
                };
                *inputs
                    .entry(r["input_index"].as_u64().unwrap())
                    .or_default() += value.as_str().unwrap().parse::<f64>().unwrap() / 1e9;
            }
        }
        let max = inputs.values().copied().fold(0.0, f64::max);
        let estimate = fixed + (rounds + warmups) as f64 * max;
        total += estimate;
        classes.push(json!({"scope":fixture["scope"],"fixed_seconds":fixed,"max_observed_per_input_seconds":max,"main_calls_per_arm":rounds+warmups,"estimated_seconds":estimate}));
    }
    println!(
        "{}",
        json!({"schema":"vieta-speed-forecast/v1","classes":classes,"estimated_total_seconds":total,"method":"fixed fixture plus (retained+warmup inputs)*maximum calibration whole-input process/preparation time; planning point estimate, not a guarantee or admitted timing"})
    );
}
fn replay(paths: &[String]) {
    let (mut checked, mut distinct, mut failed) = (0, 0, 0);
    for path in paths {
        let fixture: Value =
            serde_json::from_slice(&fs::read(Path::new(path).join("fixture.json")).unwrap())
                .unwrap();
        let s = &fixture["scope"];
        let p = BigUint::parse_bytes(s["p_hex"].as_str().unwrap().as_bytes(), 16).unwrap();
        let f = WideField(p);
        let e = Curve {
            a: f.decode(s["a_hex"].as_str().unwrap()),
            b: f.decode(s["b_hex"].as_str().unwrap()),
        };
        let c = Config {
            panel: "replay".into(),
            field: s["field"].as_str().unwrap().into(),
            representation: s["representation"].as_str().unwrap().into(),
            m: s["m"].as_u64().unwrap() as usize,
            rounds: 0,
            warmups: 0,
            seconds: 120,
            seed: 0,
        };
        let build = ctx(&f, PolynomialBackend::KaratsubaNewton, 120);
        let m = member(&build, &c).unwrap();
        let b = base(&build, &e, &m).unwrap();
        let mut expected: BTreeMap<u64, Value> = BTreeMap::new();
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap().path();
            if !entry
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("input-")
                || entry.extension().is_none_or(|e| e != "json")
            {
                continue;
            }
            let r: Value = serde_json::from_slice(&fs::read(entry).unwrap()).unwrap();
            let a = ctx(&f, PolynomialBackend::Schoolbook, 120);
            let result = hex_relations(
                &f,
                &a.lookup_scalar(&e, &b, &point(&f, &r["target"])).unwrap(),
            );
            assert_eq!(result, r["expected"]);
            assert!(expected
                .insert(r["input_index"].as_u64().unwrap(), result)
                .is_none());
            distinct += 1;
        }
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap().path();
            if !entry
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("cell-")
                || entry.extension().is_none_or(|e| e != "json")
            {
                continue;
            }
            let r: Value = serde_json::from_slice(&fs::read(entry).unwrap()).unwrap();
            if r["status"] != "ok" {
                failed += 1;
                continue;
            }
            let e = &expected[&r["input_index"].as_u64().unwrap()];
            assert_eq!(&r["relations"], e);
            assert_eq!(r["result_sha256"], digest(e));
            assert_eq!(r["process_exit"], 0);
            checked += 1;
        }
    }
    println!(
        "{}",
        json!({"schema":"vieta-speed-replay/v1","checked":checked,"distinct_inputs":distinct,"preserved_failed_rows":failed,"method":"separate-process BigUint scalar complete-set replay; producer-owned, no independent signed standing"})
    );
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("worker") => {let c:Config=serde_json::from_str(&args[2]).unwrap();let r:Value=serde_json::from_str(&args[3]).unwrap();let seed=args[5].parse().unwrap();
            let row=if c.field=="word62" {worker(WordField(4179340454199820289),Curve{a:2,b:3},&c,&r,&args[4],seed)}else{assert_eq!(c.field,"p224");let(f,e)=p224();worker(f,e,&c,&r,&args[4],seed)};println!("{row}");}
        Some("panel") => {let c:Config=serde_json::from_slice(&fs::read(&args[2]).unwrap()).unwrap();assert!(c.m.is_power_of_two()&&c.m<=16384&&c.warmups<=2&&c.rounds<=24&&c.seconds<=120);
            if c.field=="word62"{panel(WordField(4179340454199820289),Curve{a:2,b:3},&c,Path::new(&args[3]));}else{assert_eq!(c.field,"p224");let(f,e)=p224();panel(f,e,&c,Path::new(&args[3]));}}
        Some("summarize") => summarize(&args[2..]),
        Some("forecast") => forecast(&args[4..],args[2].parse().unwrap(),args[3].parse().unwrap()),
        Some("replay") => replay(&args[2..]),
        Some("transport") => println!("{{\"schema\":\"vieta-speed-transport/v1\",\"owns_raw\":false}}"),
        _ => panic!("worker CONFIG TARGET ARM SEED | panel CONFIG_FILE NEW_OUTPUT_DIR | summarize OUTPUT_DIR... | forecast ROUNDS WARMUPS OUTPUT_DIR... | replay OUTPUT_DIR... | transport"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retained_williams_orders_balance_each_population() {
        for population in 0..2 {
            let mut counts = [[0; 6]; 6];
            for n in (population..12).step_by(2) {
                for (position, arm) in arm_order(n + 2, 2).into_iter().enumerate() {
                    counts[arm][position] += 1;
                }
            }
            assert_eq!(counts, [[1; 6]; 6]);
        }
    }
    #[test]
    fn all_backends_full_cost_and_positive_control() {
        let c = Config {
            panel: "correctness-only".into(),
            field: "toy".into(),
            representation: "coset".into(),
            m: 4,
            rounds: 1,
            warmups: 0,
            seconds: 5,
            seed: 1,
        };
        let f = WordField(17);
        let e = Curve { a: 2, b: 2 };
        let ref_arith = ctx(&f, PolynomialBackend::Schoolbook, 5);
        let m = member(&ref_arith, &c).unwrap();
        let b = base(&ref_arith, &e, &m).unwrap();
        let target = ref_arith
            .group_add(&e, &Some(b[0]), &Some(b[2]))
            .unwrap()
            .unwrap();
        let t = json!([f.hex(&target.0), f.hex(&target.1)]);
        let expected = hex_relations(&f, &ref_arith.lookup_scalar(&e, &b, &Some(target)).unwrap());
        assert!(expected.as_array().unwrap().len() >= 2);
        let rows: Vec<_> = ARMS
            .iter()
            .map(|a| worker(f.clone(), e.clone(), &c, &t, a, 42))
            .collect();
        assert!(rows.iter().all(|r| r["status"] == "ok"));
        assert!(rows.iter().all(|r| r["relations"] == expected));
        let h = rows[2]["query_work"]["muls"].as_u64().unwrap();
        assert_eq!(rows[5]["query_work"]["muls"].as_u64().unwrap(), 2 * h);
        assert!(rows
            .iter()
            .all(|r| r["construction_work"]["muls"].as_u64().unwrap() > 0));
    }
    #[test]
    fn rational_assumptions_reject_repeated_partial_and_non_coprime_fibers() {
        let a = ctx(&WordField(17), PolynomialBackend::HalfGcd, 5);
        let square = RationalMap {
            numerator: vec![0, 0, 1],
            denominator: vec![1],
        };
        assert!(a
            .certify_tower(std::slice::from_ref(&square), 0, 42)
            .is_err());
        assert!(a
            .certify_tower(std::slice::from_ref(&square), 3, 42)
            .is_err());
        assert!(a
            .certify_tower(
                &[RationalMap {
                    numerator: vec![0, 0, 1],
                    denominator: vec![0, 1]
                }],
                4,
                42
            )
            .is_err());
        assert!(a
            .certify_tower(
                &[RationalMap {
                    numerator: vec![1, 0, 1],
                    denominator: vec![1, 0, 1]
                }],
                4,
                42
            )
            .is_err());
        let auxiliary = Curve { a: 1, b: 0 };
        let map = a.velu_two_map(&auxiliary, 0).unwrap();
        let tower = a.certify_tower(&[map], 12, 42).unwrap();
        let result = a
            .solve_tower(&Curve { a: 2, b: 2 }, &tower, &Some((7, 11)), 42)
            .unwrap();
        assert_eq!(result, vec![((5, 1), (7, 6)), ((7, 6), (5, 1))]);
    }
}
