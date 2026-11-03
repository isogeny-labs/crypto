//! Independent scalar-keyed audit of complete point-keyed k=1 occupancy.
use std::collections::{BTreeSet, HashMap};
use std::{env, fs};

use crypto_lib::cryptanalysis::gaudry_quartic::{Curve4, Pt4, E4};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

type Pair = [(usize, i8); 2];
type Terms = [(usize, i8); 4];

fn u(v: &Value) -> u64 {
    v.as_u64().expect("u64")
}
fn e(v: &Value) -> E4 {
    let a = v.as_array().expect("Fp4 element");
    E4([u(&a[0]), u(&a[1]), u(&a[2]), u(&a[3])])
}
fn point(v: &Value) -> Pt4 {
    Pt4 {
        x: e(&v[0]),
        y: e(&v[1]),
        inf: false,
    }
}
fn mul(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn sub(a: u64, b: u64, n: u64) -> u64 {
    (a + n - b) % n
}
fn signed_log(log: u64, sign: i8, n: u64) -> u64 {
    if sign == 1 {
        log
    } else {
        sub(0, log, n)
    }
}
fn choose3(x: usize) -> u64 {
    (x * (x - 1) * (x - 2) / 6) as u64
}

fn bsgs_logs(c: &Curve4, g: &Pt4, targets: &[Pt4], n: u64) -> (Vec<u64>, u64) {
    let mut m = 1u64;
    while m * m < n {
        m += 1;
    }
    let mut baby = HashMap::with_capacity(m as usize);
    let mut acc = Pt4::INF;
    for j in 0..m {
        assert!(baby.insert(acc, j).is_none(), "baby table collision");
        acc = c.add(&acc, g);
    }
    let backward = c.neg(&acc);
    let mut logs = Vec::with_capacity(targets.len());
    for target in targets {
        let mut candidate = *target;
        let mut found = None;
        for i in 0..=m {
            if let Some(&j) = baby.get(&candidate) {
                let log = i * m + j;
                if log < n && c.mul(g, log) == *target {
                    found = Some(log);
                    break;
                }
            }
            candidate = c.add(&candidate, &backward);
        }
        logs.push(found.expect("independent subgroup logarithm"));
    }
    (logs, m)
}

fn scalar_pairs(logs: &[u64], line0: usize, n: u64) -> HashMap<u64, Vec<Pair>> {
    let mut table = HashMap::<u64, Vec<Pair>>::new();
    for i in 0..line0 {
        for j in (i + 1)..line0 {
            for si in [-1, 1] {
                for sj in [-1, 1] {
                    let key = (signed_log(logs[i], si, n) + signed_log(logs[j], sj, n)) % n;
                    table.entry(key).or_default().push([(i, si), (j, sj)]);
                }
            }
        }
    }
    table
}

fn exact_k1(
    logs: &[u64],
    line0: usize,
    n: u64,
    pairs: &HashMap<u64, Vec<Pair>>,
    target_log: u64,
) -> BTreeSet<Terms> {
    let mut set = BTreeSet::new();
    for j in line0..logs.len() {
        for sj in [-1, 1] {
            let after_l2 = sub(target_log, signed_log(logs[j], sj, n), n);
            for i in 0..line0 {
                for si in [-1, 1] {
                    let needed = sub(after_l2, signed_log(logs[i], si, n), n);
                    if let Some(matches) = pairs.get(&needed) {
                        for pair in matches {
                            if pair[0].0 == i || pair[1].0 == i {
                                continue;
                            }
                            let mut terms = [pair[0], pair[1], (i, si), (j, sj)];
                            terms.sort_unstable();
                            set.insert(terms);
                        }
                    }
                }
            }
        }
    }
    set
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(
        args.len(),
        3,
        "usage: quartic_twist_k1_census_audit CENSUS_JSON MIXED_NDJSON"
    );
    let rec: Value = serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let source: Vec<Value> = fs::read_to_string(&args[2])
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let p = u(&rec["p"]);
    let seed = u(&rec["seed"]);
    assert!(matches!(p, 73 | 97) && seed <= 2);
    let prior = source
        .iter()
        .find(|v| u(&v["p"]) == p && u(&v["seed"]) == seed && v["line_powers"] == json!([0, 2]))
        .expect("registered mixed-line trace");
    let n = u(&prior["n"]);
    let b0 = u(&prior["b0"]);
    assert_eq!(u(&rec["n"]), n);
    assert_eq!(u(&rec["b0"]), b0);
    assert_eq!(rec["status"], "pass");
    let c = Curve4::with_params(p, E4::ZERO, E4([0, 0, 0, b0]));
    let base: Vec<Pt4> = prior["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    let line0 = base
        .iter()
        .take_while(|pt| pt.x.0[1..] == [0, 0, 0])
        .count();
    assert!(line0 >= 3 && line0 < base.len());
    assert!(base[..line0].iter().all(|pt| pt.x.0[1..] == [0, 0, 0]));
    assert!(base[line0..]
        .iter()
        .all(|pt| pt.x.0[0] == 0 && pt.x.0[1] == 0 && pt.x.0[3] == 0));
    assert_eq!(u(&rec["physical_base"]) as usize, base.len());
    assert_eq!(u(&rec["line0"]) as usize, line0);
    assert_eq!(u(&rec["line2"]) as usize, base.len() - line0);
    assert!(base.iter().all(|pt| c.on_curve(pt)));
    let g = Pt4 {
        x: e(&prior["generator_x"]),
        y: e(&prior["generator_y"]),
        inf: false,
    };
    let q = Pt4 {
        x: e(&prior["target_x"]),
        y: e(&prior["target_y"]),
        inf: false,
    };
    assert!(c.on_curve(&g) && c.on_curve(&q) && c.mul(&g, n).inf);
    let mut targets = base.clone();
    targets.push(q);
    let (all_logs, baby_steps) = bsgs_logs(&c, &g, &targets, n);
    let target_log = *all_logs.last().unwrap();
    assert_eq!(target_log, 271, "public target postcheck");
    let logs = &all_logs[..base.len()];
    for (&log, pt) in logs.iter().zip(&base) {
        assert_eq!(c.mul(&g, log), *pt);
    }
    let table = scalar_pairs(logs, line0, n);
    let pair_entries: usize = table.values().map(Vec::len).sum();
    assert_eq!(pair_entries, 2 * line0 * (line0 - 1));
    assert_eq!(u(&rec["pair_entries"]) as usize, pair_entries);
    let stream = prior["query_trace"].as_array().unwrap();
    let trace = rec["trace"].as_array().unwrap();
    assert_eq!(stream.len(), trace.len());
    assert_eq!(u(&rec["queries"]) as usize, trace.len());
    let mut total = 0;
    let mut zeros = 0;
    let mut first_hit = None;
    for (index, (query, row)) in stream.iter().zip(trace).enumerate() {
        let a = u(&query[0]);
        let b = u(&query[1]);
        assert_eq!(u(&row["query_index"]) as usize, index);
        assert_eq!(u(&row["a"]), a);
        assert_eq!(u(&row["b"]), b);
        assert_eq!(u(&row["recorded_all_pattern_hits"]), u(&query[2]));
        let residual_log = (a + mul(b, target_log, n)) % n;
        let signed_set = exact_k1(logs, line0, n, &table, residual_log);
        let digest = format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&signed_set).unwrap())
        );
        assert_eq!(
            u(&row["k1_signed_relations"]) as usize,
            signed_set.len(),
            "count at query {index}"
        );
        assert_eq!(
            row["signed_set_sha256"], digest,
            "signed set at query {index}"
        );
        if signed_set.is_empty() {
            zeros += 1;
        } else if first_hit.is_none() {
            first_hit = Some(index);
        }
        total += signed_set.len();
        assert_eq!(u(&row["cumulative_k1_signed_relations"]) as usize, total);
    }
    assert_eq!(u(&rec["total_k1_signed_relations"]) as usize, total);
    assert_eq!(u(&rec["zero_k1_queries"]) as usize, zeros);
    if let Some(index) = first_hit {
        assert_eq!(u(&rec["first_k1_hit"]) as usize, index);
    } else {
        assert!(rec["first_k1_hit"].is_null());
    }
    let numerator = 16 * choose3(line0) * (base.len() - line0) as u64;
    assert_eq!(u(&rec["uniform_expected_per_query_numerator"]), numerator);
    assert_eq!(u(&rec["uniform_expected_per_query_denominator"]), n);
    let expected = numerator as f64 * trace.len() as f64 / n as f64;
    assert!((rec["uniform_expected_total"].as_f64().unwrap() - expected).abs() < 1e-9);
    assert!(
        (rec["observed_over_uniform_expected"].as_f64().unwrap() - total as f64 / expected).abs()
            < 1e-9
    );
    println!(
        "{}",
        json!({"status":"pass","p":p,"seed":seed,"queries":trace.len(),
        "independent_bsgs_baby_steps":baby_steps,"factor_logs_verified":logs.len(),
        "independent_target_log":target_log,"scalar_pair_entries":pair_entries,
        "total_k1_signed_relations":total,"zero_k1_queries":zeros,"first_k1_hit":first_hit,
        "per_query_signed_set_digests_verified":trace.len()})
    );
}
