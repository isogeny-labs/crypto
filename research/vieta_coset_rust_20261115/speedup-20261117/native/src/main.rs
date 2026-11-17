//! Fresh-process field-backend comparison. Existing field/evaluator code only.
mod analysis;
use num_bigint::BigUint;
use num_traits::One;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    fs,
    io::Write,
    path::Path,
    process::Command,
    time::{Duration, Instant},
};
use vieta_coset_lab::vieta_coset::*;
use vieta_shared_fp::fp_backend::{Fp224, Fp64, PrimeField as SharedBackend};
use vieta_speedup_lab::SharedField;
trait Decode: PrimeField {
    fn decode(&self, s: &str) -> Self::E;
}
impl Decode for WordField {
    fn decode(&self, s: &str) -> u64 {
        u64::from_str_radix(s, 16).unwrap()
    }
}
impl Decode for WideField {
    fn decode(&self, s: &str) -> BigUint {
        BigUint::parse_bytes(s.as_bytes(), 16).unwrap()
    }
}
impl<F: SharedBackend> Decode for SharedField<F>
where
    F::Elem: Ord,
{
    fn decode(&self, s: &str) -> Self::E {
        self.from_biguint(&BigUint::parse_bytes(s.as_bytes(), 16).unwrap())
    }
}
fn ctx<F: PrimeField + Clone>(f: &F, b: PolynomialBackend, seconds: u64) -> Arithmetic<F> {
    Arithmetic::new(
        f.clone(),
        Limits {
            max_muls: 4_000_000_000,
            max_split_attempts: 65536,
            timeout: Duration::from_secs(seconds),
        },
    )
    .unwrap()
    .with_polynomials(b)
}
enum Member<E> {
    Coset(Coset<E>),
    Tower(SplitTower<E>),
}
fn member<F: PrimeField>(a: &Arithmetic<F>, c: &Value) -> Result<Member<F::E>> {
    let m = c["m"].as_u64().unwrap() as usize;
    match c["representation"].as_str().unwrap() {
        "coset" => Ok(Member::Coset(a.coset(m, a.field.element(1))?)),
        "rational" => Ok(Member::Tower(a.conjugate_square_tower(
            m.trailing_zeros(),
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
fn curve<F: Decode>(f: &F, field: &str) -> Curve<F::E> {
    if field == "word62" {
        Curve {
            a: f.element(2),
            b: f.element(3),
        }
    } else {
        let p = (BigUint::one() << 224usize) - (BigUint::one() << 96usize) + 1u8;
        Curve {
            a: f.decode(&(&p - 3u8).to_str_radix(16)),
            b: f.decode("b4050a850c04b3abf54132565044b0b7d7bfd8ba270b39432355ffb4"),
        }
    }
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
fn relations<F: PrimeField>(f: &F, rs: Vec<Relation<F::E>>) -> Value {
    // Representation ordering is not canonical-integer ordering. Normalize after clocks.
    let mut v: Vec<[String; 4]> = rs
        .into_iter()
        .map(|(a, b)| [a.0, a.1, b.0, b.1].map(|x| f.hex(&x)))
        .collect();
    v.sort_by_cached_key(|r| {
        r.clone()
            .map(|x| BigUint::parse_bytes(x.as_bytes(), 16).unwrap())
    });
    json!(v)
}
fn digest(v: &Value) -> String {
    format!("{:x}", Sha256::digest(serde_json::to_vec(v).unwrap()))
}
fn plus(a: Work, b: Work) -> Value {
    let mut x = serde_json::to_value(a).unwrap();
    for (k, v) in serde_json::to_value(b).unwrap().as_object().unwrap() {
        x[k] = json!(x[k].as_u64().unwrap() + v.as_u64().unwrap());
    }
    x
}
fn worker<F: Decode + Clone>(
    f: F,
    c: &Value,
    target: &Value,
    arm: &str,
    seed: u64,
    cold: Instant,
    field_setup_ns: String,
) -> Value {
    let fast = arm.starts_with("fast") || arm == "lookup-fast";
    let backend = if arm == "school" {
        PolynomialBackend::Schoolbook
    } else if arm.ends_with("half") {
        PolynomialBackend::HalfGcd
    } else {
        PolynomialBackend::KaratsubaNewton
    };
    let field = c["field"].as_str().unwrap();
    let seconds = c["seconds"].as_u64().unwrap();
    let e = curve(&f, field);
    let r = point(&f, target);
    let construction = ctx(&f, PolynomialBackend::KaratsubaNewton, seconds);
    let built = member(&construction, c).and_then(|m| {
        let b = base(&construction, &e, &m)?;
        let set: HashSet<_> = if arm.starts_with("lookup") {
            b.iter().cloned().collect()
        } else {
            HashSet::new()
        };
        Ok((m, b, set))
    });
    let construction_ns = cold.elapsed().as_nanos().to_string();
    let construction_error = built.as_ref().err().map(|e| format!("{e:?}"));
    let query = ctx(&f, backend, seconds);
    let start = Instant::now();
    let answer = built
        .as_ref()
        .map_err(|_| Error::Invalid("construction failed"))
        .and_then(|(m, b, set)| {
            if arm.starts_with("lookup") {
                query.lookup_batched(&e, b, set, &r)
            } else {
                let a = solve(&query, &e, m, &r, seed)?;
                if arm == "fast-double" {
                    let b = solve(&query, &e, m, &r, seed)?;
                    if a != b {
                        return Err(Error::Inconsistent("repeat differs"));
                    }
                    std::hint::black_box(b);
                }
                Ok(a)
            }
        });
    let query_ns = start.elapsed().as_nanos().to_string();
    let cold_ns = cold.elapsed().as_nanos().to_string();
    let (status, error, rs) = match answer {
        Ok(r) => ("ok", Value::Null, relations(&f, r)),
        Err(e) => ("failed", json!(format!("{e:?}")), Value::Null),
    };
    json!({"schema":"vieta-speedup/v1","arm":arm,"status":status,"error":error,"construction_error":construction_error,"field_setup_ns":field_setup_ns,"field_setup_inside_cold":true,"field_setup_primitive_ops_unpriced":fast,"core_work_is_full_primitive_cost":!fast,"construction_ns":construction_ns,"query_ns":query_ns,"cold_ns":cold_ns,"construction_work":construction.work(),"query_work":query.work(),"total_work":plus(construction.work(),query.work()),"relations":rs,"result_sha256":if status=="ok"{json!(digest(&rs))}else{Value::Null},"counts_scope":"Existing evaluator primitive counters; shared backend constructor/precomputation is inside cold clocks but not intercepted by core counters. No fully priced cold-count claim for fast arms."})
}
fn run_worker(c: &Value, target: &Value, arm: &str, seed: u64) -> Value {
    assert!([
        "school",
        "newton",
        "half",
        "fast",
        "fast-half",
        "fast-aa",
        "fast-double",
        "lookup",
        "lookup-fast"
    ]
    .contains(&arm));
    let cold = Instant::now();
    let fast = arm.starts_with("fast") || arm == "lookup-fast";
    if c["field"] == "word62" {
        if fast {
            let f = SharedField::new(Fp64::new(4179340454199820289));
            let setup = cold.elapsed().as_nanos().to_string();
            worker(f, c, target, arm, seed, cold, setup)
        } else {
            let f = WordField(4179340454199820289);
            let setup = cold.elapsed().as_nanos().to_string();
            worker(f, c, target, arm, seed, cold, setup)
        }
    } else {
        assert_eq!(c["field"], "p224");
        let p = (BigUint::one() << 224usize) - (BigUint::one() << 96usize) + 1u8;
        if fast {
            let f = SharedField::new(Fp224::new(&p));
            let setup = cold.elapsed().as_nanos().to_string();
            worker(f, c, target, arm, seed, cold, setup)
        } else {
            let f = WideField(p);
            let setup = cold.elapsed().as_nanos().to_string();
            worker(f, c, target, arm, seed, cold, setup)
        }
    }
}
fn write_new(p: &Path, v: &Value) {
    let mut f = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(p)
        .unwrap();
    writeln!(f, "{v}").unwrap();
    f.sync_all().unwrap();
}
fn panel<F: Decode + Clone>(f: F, c: &Value, out: &Path) {
    fs::create_dir(out).unwrap();
    let seconds = c["seconds"].as_u64().unwrap();
    let e = curve(&f, c["field"].as_str().unwrap());
    let fixture_start = Instant::now();
    let setup = ctx(&f, PolynomialBackend::KaratsubaNewton, seconds);
    let m = member(&setup, c).unwrap();
    let b = base(&setup, &e, &m).unwrap();
    assert!(b.len() > 1);
    write_new(
        &out.join("fixture.json"),
        &json!({"config":c,"p_hex":f.modulus().to_str_radix(16),"base_points":b.len(),"excluded_fixture_work":setup.work(),"fixture_ns":fixture_start.elapsed().as_nanos().to_string()}),
    );
    let mut seed = c["seed"].as_u64().unwrap();
    let warm = c["warmups"].as_u64().unwrap() as usize;
    let rounds = c["rounds"].as_u64().unwrap() as usize;
    let arms = c["arms"].as_array().unwrap();
    let mut failed = false;
    for i in 0..warm + rounds {
        let preparation_start = Instant::now();
        let generator = ctx(&f, PolynomialBackend::Schoolbook, seconds);
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
        let expected = relations(&f, generator.lookup_scalar(&e, &b, &r).unwrap());
        if i % 2 == 1 {
            assert!(!expected.as_array().unwrap().is_empty());
        }
        let target = r.map_or(Value::Null, |(x, y)| json!([f.hex(&x), f.hex(&y)]));
        let population = if i % 2 == 0 {
            "seeded-random-abscissa"
        } else {
            "conditioned-sum"
        };
        write_new(
            &out.join(format!("input-{i:03}.json")),
            &json!({"config":c,"target":target,"expected":expected,"input_index":i,"warmup":i<warm,"population":population,"excluded_target_and_verifier_work":generator.work(),"preparation_ns":preparation_start.elapsed().as_nanos().to_string()}),
        );
        let n = i.saturating_sub(warm);
        let mut order: Vec<usize> = if arms.len() == 6 {
            let mut v: Vec<_> = [0, 1, 5, 2, 4, 3].iter().map(|j| (j + n) % 6).collect();
            if (n / 6) % 2 == 1 {
                v.reverse();
            }
            v
        } else {
            (0..arms.len()).map(|j| (j + n) % arms.len()).collect()
        };
        if i == 0 {
            if let Some(positive) = arms.iter().position(|x| x == "fast-double") {
                order.retain(|x| *x != positive);
                order.insert(0, positive);
            }
        }
        for ai in order {
            let arm = arms[ai].as_str().unwrap();
            let start = Instant::now();
            let result = Command::new("timeout")
                .args(["--signal=TERM", "--kill-after=3s", "135"])
                .arg(std::env::current_exe().unwrap())
                .arg("worker")
                .arg(c.to_string())
                .arg(target.to_string())
                .arg(arm)
                .arg((0x53555000u64 + i as u64).to_string())
                .env("RAYON_NUM_THREADS", "1")
                .output()
                .unwrap();
            let mut row:Value=serde_json::from_slice(&result.stdout).unwrap_or_else(|_|json!({"status":"worker-no-row","stdout":String::from_utf8_lossy(&result.stdout)}));
            row["config"] = c.clone();
            row["arm"] = json!(arm);
            row["target"] = target.clone();
            row["input_index"] = json!(i);
            row["round"] = json!(i.saturating_sub(warm));
            row["warmup"] = json!(i < warm);
            row["population"] = json!(population);
            row["process_exit"] = json!(result.status.code());
            row["process_ns"] = json!(start.elapsed().as_nanos().to_string());
            row["complete_set_matches"] = json!(row["relations"] == expected);
            if row["status"] == "ok" && row["complete_set_matches"] != true {
                row["status"] = json!("mismatch");
            }
            let prefix = out.join(format!("cell-{i:03}-{arm}"));
            fs::write(prefix.with_extension("stderr"), &result.stderr).unwrap();
            write_new(&prefix.with_extension("json"), &row);
            println!("{}", json!({"input":i,"arm":arm,"status":row["status"]}));
            std::io::stdout().flush().unwrap();
            if row["status"] != "ok" {
                failed = true;
            }
            assert!(row["status"] != "mismatch");
            assert!(
                !(i == 0 && arm == "fast-double" && row["status"] != "ok"),
                "first positive failed"
            );
        }
    }
    if failed {
        std::process::exit(4);
    }
}
fn replay(paths: &[String]) {
    let (mut checked, mut inputs, mut failed) = (0, 0, 0);
    for path in paths {
        let fixture: Value =
            serde_json::from_slice(&fs::read(Path::new(path).join("fixture.json")).unwrap())
                .unwrap();
        let c = &fixture["config"];
        let p = BigUint::parse_bytes(fixture["p_hex"].as_str().unwrap().as_bytes(), 16).unwrap();
        let f = WideField(p);
        let e = curve(&f, c["field"].as_str().unwrap());
        let setup = ctx(&f, PolynomialBackend::Schoolbook, 240);
        let m = member(&setup, c).unwrap();
        let b = base(&setup, &e, &m).unwrap();
        let mut expected = std::collections::BTreeMap::new();
        for entry in fs::read_dir(path).unwrap() {
            let p = entry.unwrap().path();
            if p.extension().is_none_or(|x| x != "json")
                || !p
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("input-")
            {
                continue;
            }
            let r: Value = serde_json::from_slice(&fs::read(p).unwrap()).unwrap();
            let verifier = ctx(&f, PolynomialBackend::Schoolbook, 240);
            let answer = relations(
                &f,
                verifier
                    .lookup_scalar(&e, &b, &point(&f, &r["target"]))
                    .unwrap(),
            );
            assert_eq!(answer, r["expected"]);
            assert!(expected
                .insert(r["input_index"].as_u64().unwrap(), answer)
                .is_none());
            inputs += 1;
        }
        for entry in fs::read_dir(path).unwrap() {
            let p = entry.unwrap().path();
            if p.extension().is_none_or(|x| x != "json")
                || !p
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("cell-")
            {
                continue;
            }
            let r: Value = serde_json::from_slice(&fs::read(p).unwrap()).unwrap();
            if r["status"] != "ok" {
                failed += 1;
                continue;
            }
            let answer = &expected[&r["input_index"].as_u64().unwrap()];
            assert_eq!(&r["relations"], answer);
            assert_eq!(r["result_sha256"], digest(answer));
            assert_eq!(r["process_exit"], 0);
            checked += 1;
        }
    }
    println!(
        "{}",
        json!({"schema":"vieta-speedup-replay/v1","checked":checked,"distinct_inputs":inputs,"preserved_failed_rows":failed,"method":"BigUint scalar complete-set replay, including word-field cases; producer-owned, no independent signature"})
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("replay") => replay(&a[2..]),
        Some("summarize") => analysis::summarize(&a[2..]),
        Some("forecast") => {
            analysis::forecast(&a[4..], a[2].parse().unwrap(), a[3].parse().unwrap())
        }
        Some("worker") => {
            let c: Value = serde_json::from_str(&a[2]).unwrap();
            let target: Value = serde_json::from_str(&a[3]).unwrap();
            println!("{}", run_worker(&c, &target, &a[4], a[5].parse().unwrap()));
        }
        Some("panel") => {
            let c: Value = serde_json::from_slice(&fs::read(&a[2]).unwrap()).unwrap();
            assert!(c["m"].as_u64().unwrap().is_power_of_two());
            if c["field"] == "word62" {
                panel(WordField(4179340454199820289), &c, Path::new(&a[3]));
            } else {
                let p = (BigUint::one() << 224usize) - (BigUint::one() << 96usize) + 1u8;
                panel(WideField(p), &c, Path::new(&a[3]));
            }
        }
        _ => panic!("worker CONFIG_JSON TARGET_JSON ARM ROOT_SEED | panel CONFIG_FILE NEW_OUTPUT"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn worker_sets_and_double_counter_control_are_exact() {
        let c = json!({"field":"word62","representation":"coset","m":32,"seconds":3});
        let f = WordField(4179340454199820289);
        let a = ctx(&f, PolynomialBackend::Schoolbook, 3);
        let e = curve(&f, "word62");
        let m = member(&a, &c).unwrap();
        let b = base(&a, &e, &m).unwrap();
        let r = a.group_add(&e, &Some(b[0]), &Some(b[2])).unwrap();
        let target = r.map_or(Value::Null, |(x, y)| json!([f.hex(&x), f.hex(&y)]));
        let expected = relations(&f, a.lookup_scalar(&e, &b, &point(&f, &target)).unwrap());
        assert!(!expected.as_array().unwrap().is_empty());
        let mut single = 0;
        for arm in [
            "school",
            "newton",
            "half",
            "fast",
            "fast-half",
            "fast-aa",
            "fast-double",
            "lookup",
            "lookup-fast",
        ] {
            let r = run_worker(&c, &target, arm, 17);
            assert_eq!(r["status"], "ok");
            assert_eq!(r["relations"], expected);
            if arm == "fast" {
                single = r["query_work"]["muls"].as_u64().unwrap();
            }
            if arm == "fast-aa" {
                assert_eq!(r["query_work"]["muls"], single);
            }
            if arm == "fast-double" {
                assert_eq!(r["query_work"]["muls"], 2 * single);
            }
            assert!(
                r["field_setup_ns"]
                    .as_str()
                    .unwrap()
                    .parse::<u128>()
                    .unwrap()
                    <= r["construction_ns"]
                        .as_str()
                        .unwrap()
                        .parse::<u128>()
                        .unwrap()
            );
        }
    }
}
