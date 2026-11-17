//! Native harness. `check` does correctness only and emits no timings.
//! `bench` is for the frozen, isolated remote protocol, never a substitute
//! for admission by the lab. Each row survives errors and is flushed.
use num_bigint::BigUint;
use num_traits::One;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{self, Write};
use std::time::{Duration, Instant};
use vieta_coset_lab::vieta_coset::*;

fn emit(v: Value) {
    let mut o = io::stdout().lock();
    writeln!(o, "{v}").unwrap();
    o.flush().unwrap();
}
fn context<F: PrimeField + Clone>(f: &F, seconds: u64) -> Arithmetic<F> {
    Arithmetic::new(
        f.clone(),
        Limits {
            timeout: Duration::from_secs(seconds),
            ..Limits::default()
        },
    )
    .unwrap()
}
fn point<F: PrimeField>(f: &F, p: &Point<F::E>) -> Value {
    p.as_ref()
        .map_or(Value::Null, |(x, y)| json!([f.hex(x), f.hex(y)]))
}
fn digest<F: PrimeField>(f: &F, r: &[Relation<F::E>]) -> String {
    let mut h = Sha256::new();
    for (p, q) in r {
        for x in [&p.0, &p.1, &q.0, &q.1] {
            let s = f.hex(x);
            h.update((s.len() as u64).to_le_bytes());
            h.update(s.as_bytes());
        }
    }
    format!("{:x}", h.finalize())
}
fn pair_table<F: PrimeField>(
    a: &Arithmetic<F>,
    e: &Curve<F::E>,
    base: &[(F::E, F::E)],
) -> Result<HashMap<Point<F::E>, Vec<Relation<F::E>>>> {
    if base.len() > 512 {
        return Err(Error::Invalid(
            "pair-table control caps the base at 512 points",
        ));
    }
    let mut out: HashMap<_, Vec<_>> = HashMap::new();
    for (i, p) in base.iter().enumerate() {
        for q in &base[i..] {
            a.check()?;
            let t = a.group_add(e, &Some(p.clone()), &Some(q.clone()))?;
            let bucket = out.entry(t).or_default();
            bucket.push((p.clone(), q.clone()));
            if p != q {
                bucket.push((q.clone(), p.clone()));
            }
        }
    }
    for v in out.values_mut() {
        v.sort();
    }
    Ok(out)
}

fn run<F: PrimeField + Clone>(
    field: F,
    e: Curve<F::E>,
    name: &str,
    m: usize,
    rounds: usize,
    seconds: u64,
    timed: bool,
    three: bool,
) -> Result<()> {
    let mut first_error = None;
    let descriptor = context(&field, seconds);
    let descriptor_start = Instant::now();
    let coset = descriptor.coset(m, field.element(1))?;
    let descriptor_ns = descriptor_start.elapsed().as_nanos().to_string();
    let setup = context(&field, seconds);
    setup.validate_curve(&e)?;
    let s = Instant::now();
    let base = setup.factor_base(&e, &coset)?;
    let set: HashSet<_> = base.iter().cloned().collect();
    let setup_ns = s.elapsed().as_nanos().to_string();
    let base_id = digest(
        &field,
        &base
            .iter()
            .map(|p| (p.clone(), p.clone()))
            .collect::<Vec<_>>(),
    );
    let panel = std::env::var("VIETA_PANEL").unwrap_or_else(|_| "development".into());
    let scope = json!({"panel":panel,"curve":name,"p_hex":field.modulus().to_str_radix(16),"a_hex":field.hex(&e.a),"b_hex":field.hex(&e.b),"m":m,"c_hex":"1","base_points":base.len(),"base_sha256":base_id,"timed":timed,"rounds":rounds,"fixture_seed_hex":format!("{:x}",0x76696574612d7631u64 ^ m as u64),"seconds_per_stage":seconds});
    emit(
        json!({"schema":"vieta-coset/v1","kind":"descriptor","scope":scope,"work":descriptor.work(),"elapsed_ns":if timed{json!(descriptor_ns)}else{Value::Null}}),
    );
    emit(
        json!({"schema":"vieta-coset/v1","kind":"setup","scope":scope,"work":setup.work(),"elapsed_ns":if timed{json!(setup_ns)}else{Value::Null}}),
    );
    if base.len() < 2 {
        return Err(Error::Invalid(
            "factor base too small for known-answer fixtures",
        ));
    }
    let mut seed = 0x76696574612d7631u64 ^ m as u64;
    let fixtures = context(&field, seconds);
    let mut targets = vec![];
    // Separate populations: random rational targets and conditioned known
    // decompositions. This prevents sparse large-field misses from masquerading
    // as a successful-relation workload. Two independent warmup targets.
    for i in 0..rounds + 2 {
        let target = if i % 2 == 0 {
            Some(fixtures.random_point(&e, &mut seed)?)
        } else {
            let p = &base[(i * 17 + 1) % base.len()];
            let q = &base[(i * 31 + 2) % base.len()];
            fixtures.group_add(&e, &Some(p.clone()), &Some(q.clone()))?
        };
        targets.push((
            target,
            if i % 2 == 0 {
                "uniform-x-rational"
            } else {
                "conditioned-two-sum"
            },
        ));
    }
    for (i, (target, population)) in targets.iter().enumerate() {
        let mut expected: Option<Vec<Relation<F::E>>> = None;
        // Alternating execution order, and identical A/A control. All arms
        // return complete ordered sets. Cold adds full setup to lookup;
        // candidate needs only the coset descriptor, never base enumeration.
        let arms = if i % 2 == 0 {
            vec!["vieta", "batched", "batched-aa", "scalar"]
        } else {
            vec!["scalar", "batched-aa", "batched", "vieta"]
        };
        for arm in arms {
            let a = context(&field, seconds);
            let start = Instant::now();
            let output = match arm {
                "vieta" => a.solve(&e, &coset, target, seed ^ i as u64),
                "scalar" => a.lookup_scalar(&e, &base, target),
                _ => a.lookup_batched(&e, &base, &set, target),
            };
            let elapsed = start.elapsed().as_nanos().to_string();
            match output {
                Ok(result) => {
                    let matches = expected.as_ref().is_none_or(|x| *x == result)
                        && (*population != "conditioned-two-sum" || !result.is_empty());
                    emit(
                        json!({"schema":"vieta-coset/v1","kind":"query","scope":scope,"round":i.saturating_sub(2),"warmup":i<2,"population":population,"target":point(&field,target),"arm":arm,"status":if matches{"ok"}else{"mismatch"},"relations":result.len(),"result_sha256":digest(&field,&result),"work":a.work(),"elapsed_ns":if timed{json!(elapsed)}else{Value::Null}}),
                    );
                    if !matches {
                        return Err(Error::Inconsistent("relation-set mismatch"));
                    }
                    expected = Some(result);
                }
                Err(err) => {
                    first_error.get_or_insert(err.clone());
                    emit(
                        json!({"schema":"vieta-coset/v1","kind":"query","scope":scope,"round":i.saturating_sub(2),"warmup":i<2,"population":population,"target":point(&field,target),"arm":arm,"status":"error","error":format!("{err:?}"),"work":a.work(),"elapsed_ns":if timed{json!(elapsed)}else{Value::Null}}),
                    );
                    // Preserve all control arms after a bounded candidate failure.
                }
            }
        }
    }
    if three {
        // Pair-table construction is bounded and charged, even on failure.
        // Only the separate three-term panel creates the O(M^2) table.
        let a = context(&field, seconds);
        let start = Instant::now();
        let table = pair_table(&a, &e, &base);
        emit(
            json!({"schema":"vieta-coset/v1","kind":"pair-setup","scope":scope,"status":if table.is_ok(){"ok"}else{"error"},"error":table.as_ref().err().map(|e|format!("{e:?}")),"work":a.work(),"elapsed_ns":if timed{json!(start.elapsed().as_nanos().to_string())}else{Value::Null}}),
        );
        let table = table?;
        for i in 0..rounds + 2 {
            // Fixture construction is outside the arm intervals. A fresh
            // budget must not inherit elapsed time from previous benchmarks.
            let fixtures = context(&field, seconds);
            let t = fixtures.group_add(
                &e,
                &Some(base[i % base.len()].clone()),
                &Some(base[(i + 1) % base.len()].clone()),
            )?;
            let target = if i % 2 == 0 {
                Some(fixtures.random_point(&e, &mut seed)?)
            } else {
                fixtures.group_add(&e, &t, &Some(base[(i + 2) % base.len()].clone()))?
            };
            let mut expected = None;
            let arms = if i % 2 == 0 {
                ["mitm", "stream-vieta", "nested-batched", "mitm-aa"]
            } else {
                ["mitm-aa", "nested-batched", "stream-vieta", "mitm"]
            };
            for arm in arms {
                let a = context(&field, seconds);
                let start = Instant::now();
                let result = (|| -> Result<Vec<_>> {
                    let mut out = BTreeSet::new();
                    for p in &base {
                        a.check()?;
                        let residual = a.group_add(&e, &target, &a.negate(&Some(p.clone())))?;
                        let pairs = match arm {
                            "mitm" | "mitm-aa" => table.get(&residual).cloned().unwrap_or_default(),
                            "stream-vieta" => a.solve(&e, &coset, &residual, seed ^ i as u64)?,
                            _ => a.lookup_batched(&e, &base, &set, &residual)?,
                        };
                        for (q, r) in pairs {
                            out.insert((p.clone(), q, r));
                        }
                    }
                    Ok(out.into_iter().collect())
                })();
                let elapsed = start.elapsed().as_nanos().to_string();
                match result {
                    Ok(out) => {
                        let matches = expected.as_ref().is_none_or(|x| *x == out)
                            && (i % 2 == 0 || !out.is_empty());
                        let text: Vec<_> = out
                            .iter()
                            .map(|(p, q, r)| {
                                json!([
                                    point(&field, &Some(p.clone())),
                                    point(&field, &Some(q.clone())),
                                    point(&field, &Some(r.clone()))
                                ])
                            })
                            .collect();
                        let hash =
                            format!("{:x}", Sha256::digest(serde_json::to_vec(&text).unwrap()));
                        emit(
                            json!({"schema":"vieta-coset/v1","kind":"three-query","scope":scope,"arm":arm,"round":i.saturating_sub(2),"warmup":i<2,"target":point(&field,&target),"population":if i%2==0{"uniform-x-rational"}else{"conditioned-three-sum"},"status":if matches{"ok"}else{"mismatch"},"relations":out.len(),"result_sha256":hash,"work":a.work(),"elapsed_ns":if timed{json!(elapsed)}else{Value::Null}}),
                        );
                        if !matches {
                            return Err(Error::Inconsistent("three-sum mismatch"));
                        }
                        expected = Some(out);
                    }
                    Err(e) => {
                        first_error.get_or_insert(e.clone());
                        emit(
                            json!({"schema":"vieta-coset/v1","kind":"three-query","scope":scope,"arm":arm,"round":i.saturating_sub(2),"warmup":i<2,"target":point(&field,&target),"population":if i%2==0{"uniform-x-rational"}else{"conditioned-three-sum"},"status":"error","error":format!("{e:?}"),"work":a.work(),"elapsed_ns":if timed{json!(elapsed)}else{Value::Null}}),
                        );
                    }
                }
            }
        }
    }
    first_error.map_or(Ok(()), Err)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 5 || !["check", "bench"].contains(&args[1].as_str()) {
        eprintln!("usage: vieta-coset-lab check|bench word17|word31|word62|p224 m rounds [seconds=120] [three]");
        std::process::exit(2);
    }
    let timed = args[1] == "bench";
    if timed && std::env::var("VIETA_ISOLATED_RUN").as_deref() != Ok("1") {
        eprintln!("bench requires VIETA_ISOLATED_RUN=1 set by the admitted remote launcher; see PROTOCOL.md");
        std::process::exit(2);
    }
    let m: usize = args[3].parse().expect("integer m");
    let rounds: usize = args[4].parse().expect("integer rounds");
    if !(1..=100).contains(&rounds) {
        eprintln!("rounds must be 1..100");
        std::process::exit(2);
    }
    let seconds = args
        .get(5)
        .map_or(120, |v| v.parse().expect("integer seconds"));
    if !(1..=120).contains(&seconds) {
        eprintln!("seconds must be 1..120");
        std::process::exit(2);
    }
    let three = args.get(6).is_some_and(|s| s == "three");
    let name = &args[2];
    let result = match name.as_str() {
        "word17" => run(
            WordField(65537),
            Curve { a: 2, b: 3 },
            name,
            m,
            rounds,
            seconds,
            timed,
            three,
        ),
        "word31" => run(
            WordField(2013265921),
            Curve { a: 2, b: 3 },
            name,
            m,
            rounds,
            seconds,
            timed,
            three,
        ),
        "word62" => run(
            WordField(4179340454199820289),
            Curve { a: 2, b: 3 },
            name,
            m,
            rounds,
            seconds,
            timed,
            three,
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
                name,
                m,
                rounds,
                seconds,
                timed,
                three,
            )
        }
        _ => Err(Error::Invalid("unknown curve")),
    };
    if let Err(e) = result {
        emit(
            json!({"schema":"vieta-coset/v1","kind":"failure","status":"error","argv":args,"error":format!("{e:?}")}),
        );
        std::process::exit(1);
    }
}
