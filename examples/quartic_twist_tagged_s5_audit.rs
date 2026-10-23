//! Independent scalar-keyed replay of the registered mixed-line Semaev pilot.
use std::collections::{BTreeSet, HashMap};
use std::{env, fs};

use crypto_lib::cryptanalysis::gaudry_quartic::{Curve4, Pt4, E4};
use serde_json::{json, Value};

type Terms = [(usize, i8); 4];
type Pair = [(usize, i8); 2];

fn u(v: &Value) -> u64 {
    v.as_u64().expect("u64")
}
fn e(v: &Value) -> E4 {
    let a = v.as_array().expect("F4 element");
    E4([u(&a[0]), u(&a[1]), u(&a[2]), u(&a[3])])
}
fn point(v: &Value) -> Pt4 {
    Pt4 {
        x: e(&v[0]),
        y: e(&v[1]),
        inf: false,
    }
}
fn parse_set(v: &Value) -> BTreeSet<Terms> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|item| serde_json::from_value(item.clone()).unwrap())
        .collect()
}
fn signed_scalar(log: u64, sign: i8, n: u64) -> u64 {
    if sign == 1 {
        log
    } else {
        (n - log) % n
    }
}
fn replay_add(c: &Curve4, base: &[Pt4], target: &Pt4, terms: &Terms) {
    assert!(terms.windows(2).all(|w| w[0].0 < w[1].0));
    let mut sum = Pt4::INF;
    for &(id, sign) in terms {
        assert!(id < base.len() && (sign == 1 || sign == -1));
        let pt = if sign == 1 {
            base[id]
        } else {
            c.neg(&base[id])
        };
        sum = c.add(&sum, &pt);
    }
    assert_eq!(&sum, target);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(),
        7,
        "usage: quartic_twist_tagged_s5_audit MIXED_NDJSON K0 K1 K2 K3 K4"
    );
    let source: Vec<Value> = fs::read_to_string(&args[1])
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let prior = source
        .iter()
        .find(|v| u(&v["p"]) == 37 && u(&v["seed"]) == 0 && v["line_powers"] == json!([0, 2]))
        .expect("frozen mixed-line record");
    let n = u(&prior["n"]);
    assert_eq!(n, 1_873_633);
    let c = Curve4::with_params(37, E4::ZERO, E4([0, 0, 0, u(&prior["b0"])]));
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
    let base: Vec<Pt4> = prior["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    assert_eq!(base.len(), 48);
    assert_eq!(c.mul(&g, 271), q);
    let mut logs = HashMap::with_capacity(n as usize);
    let mut acc = Pt4::INF;
    for scalar in 0..n {
        assert!(logs.insert(acc, scalar).is_none());
        acc = c.add(&acc, &g);
    }
    assert_eq!(acc, Pt4::INF);
    assert_eq!(logs.len() as u64, n);
    assert_eq!(logs[&q], 271);
    let base_logs: Vec<u64> = base.iter().map(|pt| logs[pt]).collect();

    // This auditor keys the pair table by independently enumerated scalar
    // logs; the producer's reference keys it by whole curve points.
    let mut pair_hash: HashMap<u64, Vec<Pair>> = HashMap::new();
    let mut pairs = Vec::new();
    for i in 0..base.len() {
        for j in i + 1..base.len() {
            for si in [-1, 1] {
                for sj in [-1, 1] {
                    let key = (signed_scalar(base_logs[i], si, n)
                        + signed_scalar(base_logs[j], sj, n))
                        % n;
                    let terms = [(i, si), (j, sj)];
                    pair_hash.entry(key).or_default().push(terms);
                    pairs.push((key, terms));
                }
            }
        }
    }
    let frozen_trace = prior["query_trace"].as_array().unwrap();
    let mut reports = Vec::new();
    for k in 0..5 {
        let rec: Value = serde_json::from_str(&fs::read_to_string(&args[k + 2]).unwrap()).unwrap();
        assert_eq!(u(&rec["k"]) as usize, k);
        assert_eq!(u(&rec["n"]), n);
        let rows = rec["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 4);
        let mut total_relations = 0;
        let mut polynomial_relations = 0;
        let mut failed_systems = 0;
        let mut missing_relations = 0;
        for (index, row) in rows.iter().enumerate() {
            let (a, b) = (u(&frozen_trace[index][0]), u(&frozen_trace[index][1]));
            assert_eq!((u(&row["a"]), u(&row["b"])), (a, b));
            let scalar = (a + 271 * b) % n;
            let target = c.mul(&g, scalar);
            let mut expected = BTreeSet::new();
            for (left_sum, left) in &pairs {
                let complement = (scalar + n - left_sum) % n;
                if let Some(rights) = pair_hash.get(&complement) {
                    for right in rights {
                        let mut terms = [left[0], left[1], right[0], right[1]];
                        terms.sort_unstable();
                        if terms.windows(2).any(|w| w[0].0 == w[1].0) {
                            continue;
                        }
                        if terms.iter().filter(|&&(id, _)| id >= 36).count() == k {
                            expected.insert(terms);
                        }
                    }
                }
            }
            let raw_reference = parse_set(&row["reference_signed_relations"]);
            let polynomial = parse_set(&row["polynomial_signed_relations"]);
            assert_eq!(
                expected, raw_reference,
                "producer oracle mismatch, k={k}, q={index}"
            );
            assert_eq!(row["agree"], expected == polynomial);
            assert!(
                polynomial.is_subset(&expected),
                "foreign polynomial relation, k={k}, q={index}"
            );
            for relation in &polynomial {
                replay_add(&c, &base, &target, relation);
            }
            total_relations += expected.len();
            polynomial_relations += polynomial.len();
            failed_systems += u(&row["system_failures"]);
            missing_relations += expected.difference(&polynomial).count();
        }
        let complete = failed_systems == 0 && missing_relations == 0;
        assert_eq!(
            rec["status"],
            if failed_systems > 0 {
                "partial"
            } else if missing_relations > 0 {
                "mismatch"
            } else {
                "pass"
            }
        );
        reports.push(
            json!({"k":k,"queries":4,"oracle_signed_relations":total_relations,
            "polynomial_signed_relations":polynomial_relations,"failed_systems":failed_systems,
            "missing_signed_relations":missing_relations,"complete":complete}),
        );
    }
    let total: usize = reports
        .iter()
        .map(|v| u(&v["oracle_signed_relations"]) as usize)
        .sum();
    let recorded_total: usize = frozen_trace.iter().take(4).map(|z| u(&z[2]) as usize).sum();
    assert_eq!(total, recorded_total);
    let all_complete = reports.iter().all(|r| r["complete"] == true);
    println!(
        "{}",
        json!({"status":if all_complete {"pass"} else {"partial"},
        "p":37,"n":n,"enumerated_subgroup_logs":n,
        "base_size":48,"pair_postings":pairs.len(),"first_four_recorded_hits":recorded_total,
        "first_four_replayed_hits":total,"arms":reports})
    );
}
