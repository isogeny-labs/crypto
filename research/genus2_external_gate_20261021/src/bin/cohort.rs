use divisor_arithmetic::field::PrimeField;
use divisor_arithmetic::g2::ramified::not_char2::{add, double, CurveConstants, DivisorCoords};
use five_leads_study::three_leads::{Divisor, Field, Jacobian};
use serde_json::{json, Value};
use std::fs;

fn ext<const P: u64>(d: Divisor) -> DivisorCoords<PrimeField<P>> {
    match d.degree {
        0 => DivisorCoords::identity(),
        1 => DivisorCoords::deg1(PrimeField::new(d.u[0]), PrimeField::new(d.v[0])),
        2 => DivisorCoords::deg2(
            PrimeField::new(d.u[1]),
            PrimeField::new(d.u[0]),
            PrimeField::new(d.v[1]),
            PrimeField::new(d.v[0]),
        ),
        _ => unreachable!(),
    }
}
fn native<const P: u64>(d: DivisorCoords<PrimeField<P>>) -> Divisor {
    match d.degree() {
        0 => Divisor::O,
        1 => Divisor {
            degree: 1,
            u: [d.u0.value(), 1, 0],
            v: [d.v0.value(), 0],
        },
        2 => Divisor {
            degree: 2,
            u: [d.u0.value(), d.u1.value(), 1],
            v: [d.v0.value(), d.v1.value()],
        },
        _ => unreachable!(),
    }
}
fn value(f: &Field, polynomial: &[u64], x: u64) -> u64 {
    polynomial
        .iter()
        .rev()
        .fold(0, |z, &a| f.add(f.mul(z, x), a))
}
fn run<const P: u64>(polynomial: Vec<u64>, bits: u64) -> Value {
    assert_eq!(polynomial.len(), 6);
    assert_eq!(polynomial[5], 1);
    assert_eq!(polynomial[4], 0);
    let j = Jacobian {
        f: Field::new(P, 1, 0),
        polynomial: polynomial.clone(),
    };
    let cc = CurveConstants {
        f3: PrimeField::new(polynomial[3]),
        f2: PrimeField::new(polynomial[2]),
        f1: PrimeField::new(polynomial[1]),
        f0: PrimeField::new(polynomial[0]),
    };
    let mut points = Vec::new();
    for x in 0..P {
        if let Some(y) = j.f.sqrt(value(&j.f, &polynomial, x)) {
            points.push(j.point(x, y));
            if points.len() == 9 {
                break;
            }
        }
    }
    assert_eq!(points.len(), 9);
    let mut pool = vec![Divisor::O];
    for &d in &points {
        pool.push(d);
        pool.push(j.neg(d));
    }
    for i in 0..9 {
        for k in i..9 {
            pool.push(j.add(points[i], points[k]));
        }
    }
    pool.sort();
    pool.dedup();
    assert!(pool.iter().all(|&d| j.valid(d)));
    let mut calls = 0u64;
    let mut direct_equal_input_mismatches = 0u64;
    let mut direct_equal_input_panics = 0u64;
    let mut adapted_mismatches = 0u64;
    let mut adapted_panics = 0u64;
    let mut first_adapted_mismatch = None;
    let mut first_adapted_panic = None;
    for &a in &pool {
        for &b in &pool {
            let expected = j.add(a, b);
            let ea = ext::<P>(a);
            let eb = ext::<P>(b);
            let direct = std::panic::catch_unwind(|| add(&ea, &eb, &cc));
            if a == b {
                match &direct {
                    Ok(result) if native::<P>(*result) != expected => {
                        direct_equal_input_mismatches += 1
                    }
                    Err(_) => direct_equal_input_panics += 1,
                    _ => {}
                }
            }
            let adapted = if a == b {
                std::panic::catch_unwind(|| double(&ea, &cc))
            } else {
                direct
            };
            match adapted {
                Ok(result) => {
                    let actual = native::<P>(result);
                    if actual != expected {
                        adapted_mismatches += 1;
                        if first_adapted_mismatch.is_none() {
                            first_adapted_mismatch = Some(
                                json!({"a":a,"b":b,"expected":expected,"actual":actual,"actual_valid":j.valid(actual)}),
                            );
                        }
                    }
                }
                Err(_) => {
                    adapted_panics += 1;
                    if first_adapted_panic.is_none() {
                        first_adapted_panic = Some(json!({"a":a,"b":b,"expected":expected}));
                    }
                }
            }
            calls += 1;
        }
    }
    json!({"p":P,"bits":bits,"pool":pool.len(),"calls":calls,"direct_equal_input_mismatches":direct_equal_input_mismatches,"direct_equal_input_panics":direct_equal_input_panics,"adapted_mismatches":adapted_mismatches,"adapted_panics":adapted_panics,"first_adapted_mismatch":first_adapted_mismatch,"first_adapted_panic":first_adapted_panic})
}

fn dispatch(p: u64, polynomial: Vec<u64>, bits: u64) -> Value {
    match p {
        739 => run::<739>(polynomial, bits),
        1151 => run::<1151>(polynomial, bits),
        1187 => run::<1187>(polynomial, bits),
        3371 => run::<3371>(polynomial, bits),
        3527 => run::<3527>(polynomial, bits),
        3931 => run::<3931>(polynomial, bits),
        11731 => run::<11731>(polynomial, bits),
        11887 => run::<11887>(polynomial, bits),
        12143 => run::<12143>(polynomial, bits),
        48383 => run::<48383>(polynomial, bits),
        48823 => run::<48823>(polynomial, bits),
        49451 => run::<49451>(polynomial, bits),
        186379 => run::<186379>(polynomial, bits),
        186451 => run::<186451>(polynomial, bits),
        186583 => run::<186583>(polynomial, bits),
        741967 => run::<741967>(polynomial, bits),
        748807 => run::<748807>(polynomial, bits),
        749299 => run::<749299>(polynomial, bits),
        2967343 => run::<2967343>(polynomial, bits),
        2968367 => run::<2968367>(polynomial, bits),
        2972219 => run::<2972219>(polynomial, bits),
        _ => panic!("unexpected p={p}"),
    }
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let path = "research/five_leads_e2e_20261019/correctness/genus2-model-gate-v1.jsonl";
    let rows: Vec<Value> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .filter(|row: &Value| row["cohort"] == "genus-order-fixtures-v1")
        .collect();
    assert_eq!(rows.len(), 21);
    let mut results = Vec::new();
    for row in rows {
        let p = row["p"].as_u64().unwrap();
        let bits = row["bits"].as_u64().unwrap();
        let polynomial = row["normalized_polynomial"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect();
        results.push(dispatch(p, polynomial, bits));
    }
    println!(
        "{}",
        json!({"schema":"genus2-external-21-curve-add-gate/v1","upstream_commit":"87e2ebc1b59e5313b67bf81d11e42ec094251518","results":results})
    );
}
