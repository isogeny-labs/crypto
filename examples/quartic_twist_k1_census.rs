//! Full frozen-stream exact k=1 signed-relation occupancy, independent of S4.
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
fn signed(c: &Curve4, pt: &Pt4, sign: i8) -> Pt4 {
    if sign == 1 {
        *pt
    } else {
        c.neg(pt)
    }
}
fn choose3(x: usize) -> u128 {
    (x as u128) * ((x - 1) as u128) * ((x - 2) as u128) / 6
}

fn run(raw: &str, p: u64, seed: u64) -> Value {
    let source: Vec<Value> = fs::read_to_string(raw)
        .expect("frozen stream")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let prior = source
        .iter()
        .find(|v| u(&v["p"]) == p && u(&v["seed"]) == seed && v["line_powers"] == json!([0, 2]))
        .expect("registered mixed-line trace");
    let b0 = u(&prior["b0"]);
    let n = u(&prior["n"]);
    let curve = Curve4::with_params(p, E4::ZERO, E4([0, 0, 0, b0]));
    let base: Vec<Pt4> = prior["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    assert!(base.iter().all(|pt| curve.on_curve(pt)));
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
    assert!(curve.on_curve(&g) && curve.on_curve(&q));
    assert!(curve.mul(&g, n).inf);
    assert_eq!(curve.mul(&g, 271), q);
    let line0 = base.iter().take_while(|pt| pt.x.0[2] == 0).count();
    let line2 = base.len() - line0;
    assert!(line0 >= 3 && line2 >= 1);
    assert!(base[..line0].iter().all(|pt| pt.x.0[1..] == [0, 0, 0]));
    assert!(base[line0..]
        .iter()
        .all(|pt| pt.x.0[0] == 0 && pt.x.0[1] == 0 && pt.x.0[3] == 0));

    // Reverse ordering from the producer's peel-first polynomial systems:
    // signed L0 pairs are precomputed by group sum, and signed L2/L0
    // complements are queried against those sums.
    curve.f.reset_muls();
    curve.reset_ops();
    let mut pairs = HashMap::<Pt4, Vec<Pair>>::new();
    for j in 1..line0 {
        for i in 0..j {
            for sj in [-1, 1] {
                for si in [-1, 1] {
                    let sum =
                        curve.add(&signed(&curve, &base[j], sj), &signed(&curve, &base[i], si));
                    pairs.entry(sum).or_default().push([(i, si), (j, sj)]);
                }
            }
        }
    }
    let pair_entries = 4 * line0 * (line0 - 1) / 2;
    let setup_fp_muls = curve.f.muls();
    let setup_group_ops = curve.ops();
    curve.f.reset_muls();
    curve.reset_ops();
    let mut rows = Vec::new();
    let mut total = 0usize;
    let mut first_hit = None;
    let mut zero_queries = 0usize;
    for (index, item) in prior["query_trace"].as_array().unwrap().iter().enumerate() {
        let (a, b) = (u(&item[0]), u(&item[1]));
        let target = curve.add(&curve.mul(&g, a), &curve.mul(&q, b));
        let mut signed_set = BTreeSet::<Terms>::new();
        for j in line0..base.len() {
            for sj in [-1, 1] {
                let left = curve.sub(&target, &signed(&curve, &base[j], sj));
                for i in 0..line0 {
                    for si in [-1, 1] {
                        let needed = curve.sub(&left, &signed(&curve, &base[i], si));
                        if let Some(matches) = pairs.get(&needed) {
                            for pair in matches {
                                if pair[0].0 == i || pair[1].0 == i {
                                    continue;
                                }
                                let mut terms = [pair[0], pair[1], (i, si), (j, sj)];
                                terms.sort_unstable();
                                signed_set.insert(terms);
                            }
                        }
                    }
                }
            }
        }
        for terms in &signed_set {
            let mut sum = Pt4::INF;
            for &(id, sign) in terms {
                sum = curve.add(&sum, &signed(&curve, &base[id], sign));
            }
            assert_eq!(sum, target);
        }
        assert!(signed_set.len() as u64 <= u(&item[2]));
        if signed_set.is_empty() {
            zero_queries += 1;
        }
        if !signed_set.is_empty() && first_hit.is_none() {
            first_hit = Some(index);
        }
        total += signed_set.len();
        let canonical = serde_json::to_vec(&signed_set).unwrap();
        let digest = format!("{:x}", Sha256::digest(canonical));
        rows.push(json!({"query_index":index,"a":a,"b":b,
            "k1_signed_relations":signed_set.len(),"cumulative_k1_signed_relations":total,
            "signed_set_sha256":digest,"recorded_all_pattern_hits":u(&item[2])}));
    }
    let expected_numerator = 16 * choose3(line0) * line2 as u128;
    let expected_denominator = n as u128;
    let expected_total = (expected_numerator as f64) * (rows.len() as f64) / (n as f64);
    let observed_over_expected = total as f64 / expected_total;
    json!({"status":"pass","p":p,"b0":b0,"n":n,"seed":seed,
        "physical_base":base.len(),"line0":line0,"line2":line2,
        "queries":rows.len(),"zero_k1_queries":zero_queries,"first_k1_hit":first_hit,
        "total_k1_signed_relations":total,"pair_entries":pair_entries,
        "uniform_expected_per_query_numerator":expected_numerator,
        "uniform_expected_per_query_denominator":expected_denominator,
        "uniform_expected_total":expected_total,
        "observed_over_uniform_expected":observed_over_expected,
        "setup_fp_muls":setup_fp_muls,"setup_group_ops":setup_group_ops,
        "query_fp_muls":curve.f.muls(),"query_group_ops":curve.ops(),
        "trace":rows})
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(),
        4,
        "usage: quartic_twist_k1_census MIXED_NDJSON P SEED"
    );
    let p = args[2].parse::<u64>().unwrap();
    let seed = args[3].parse::<u64>().unwrap();
    assert!(matches!(p, 73 | 97) && seed <= 2);
    println!("{}", run(&args[1], p, seed));
}
