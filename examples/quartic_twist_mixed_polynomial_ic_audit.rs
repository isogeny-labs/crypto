//! Scalar-log and fresh-RREF audit of the mixed-pattern polynomial IC run.
use std::collections::{BTreeSet, HashMap};
use std::{env, fs};

use crypto_lib::cryptanalysis::gaudry_quartic::{Curve4, Pt4, E4};
use crypto_lib::cryptanalysis::quartic_folds::{construct_twist24, rho_twist24};
use crypto_lib::cryptanalysis::residual_walk::{inv_mod, pow_mod};
use serde_json::{json, Value};

type Pair = [(usize, i8); 2];
type Terms = [(usize, i8); 4];

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
fn mm(a: u64, b: u64, n: u64) -> u64 {
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
fn parse_set(v: &Value) -> BTreeSet<Terms> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|item| serde_json::from_value(item.clone()).unwrap())
        .collect()
}
fn rank_and_solution(rows: &[[u64; 5]], n: u64) -> (usize, Option<[u64; 4]>) {
    let mut a = rows.to_vec();
    let mut pivots = Vec::new();
    let mut r = 0;
    for col in 0..4 {
        let Some(pivot) = (r..a.len()).find(|&i| a[i][col] != 0) else {
            continue;
        };
        a.swap(r, pivot);
        let inverse = inv_mod(a[r][col], n);
        for j in col..5 {
            a[r][j] = mm(a[r][j], inverse, n);
        }
        for i in 0..a.len() {
            if i == r {
                continue;
            }
            let factor = a[i][col];
            for j in col..5 {
                a[i][j] = sub(a[i][j], mm(factor, a[r][j], n), n);
            }
        }
        pivots.push(col);
        r += 1;
        if r == a.len() {
            break;
        }
    }
    for row in a.iter().skip(r) {
        assert_eq!(row[4], 0, "inconsistent row");
    }
    if r == 4 {
        assert_eq!(pivots, [0, 1, 2, 3]);
        (r, Some([a[0][4], a[1][4], a[2][4], a[3][4]]))
    } else {
        (r, None)
    }
}
fn row_from_terms(terms: &Terms, col: &[usize], coef: &[u64], a: u64, b: u64, n: u64) -> [u64; 5] {
    let mut sums = [0u64; 4];
    for &(id, sign) in terms {
        let value = signed_log(coef[id], sign, n);
        sums[col[id]] = (sums[col[id]] + value) % n;
    }
    [sums[1], sums[2], sums[3], sub(0, b, n), sub(a, sums[0], n)]
}
fn add_relation(c: &Curve4, base: &[Pt4], target: &Pt4, terms: &Terms) {
    let mut sum = Pt4::INF;
    for &(id, sign) in terms {
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
    let args: Vec<_> = env::args().collect();
    assert_eq!(
        args.len(),
        3,
        "usage: quartic_twist_mixed_polynomial_ic_audit RUN_JSON MIXED_NDJSON"
    );
    let rec: Value = serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let source: Vec<Value> = fs::read_to_string(&args[2])
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let prior = source
        .iter()
        .find(|v| u(&v["p"]) == 37 && u(&v["seed"]) == 0 && v["line_powers"] == json!([0, 2]))
        .unwrap();
    let n = 1_873_633;
    assert_eq!((u(&rec["p"]), u(&rec["n"])), (37, n));
    assert_eq!(rec["status"], "pass");
    assert_eq!(rec["base_coordinates"], prior["base_coordinates"]);
    let c = Curve4::with_params(37, E4::ZERO, E4([0, 0, 0, 2]));
    let base: Vec<Pt4> = rec["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    let g = Pt4 {
        x: e(&rec["generator_x"]),
        y: e(&rec["generator_y"]),
        inf: false,
    };
    let q = Pt4 {
        x: e(&rec["target_x"]),
        y: e(&rec["target_y"]),
        inf: false,
    };
    assert_eq!(g, base[0]);
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
    let by_log: HashMap<u64, usize> = base_logs.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let lf = u(&rec["lambda_frob"]);
    let lc = u(&rec["lambda_cubic"]);
    let mut orbit_col = vec![usize::MAX; base.len()];
    let mut orbit_coef = vec![0u64; base.len()];
    let mut reps = Vec::new();
    for (id, &rep_log) in base_logs.iter().enumerate() {
        if orbit_col[id] != usize::MAX {
            continue;
        }
        let ci = reps.len();
        reps.push(id);
        let inverse = inv_mod(rep_log, n);
        let mut covered = BTreeSet::new();
        for j in 0..4 {
            for k in 0..3 {
                let wanted = mm(rep_log, mm(pow_mod(lf, j, n), pow_mod(lc, k, n), n), n);
                let idx = by_log
                    .get(&wanted)
                    .or_else(|| by_log.get(&sub(0, wanted, n)))
                    .copied()
                    .expect("orbit point missing by scalar log");
                covered.insert(idx);
                let coefficient = mm(base_logs[idx], inverse, n);
                if orbit_col[idx] == usize::MAX {
                    orbit_col[idx] = ci;
                    orbit_coef[idx] = coefficient;
                } else {
                    assert_eq!((orbit_col[idx], orbit_coef[idx]), (ci, coefficient));
                }
            }
        }
        assert_eq!(covered.len(), 12);
    }
    assert_eq!(reps.len(), 4);
    assert_eq!(rec["orbit_column"], json!(orbit_col));
    assert_eq!(rec["orbit_coefficient"], json!(orbit_coef));
    assert_eq!(rec["orbit_representatives"], json!(reps));

    // Pair sums are keyed by independently enumerated subgroup scalars.
    let mut pair_hash: HashMap<u64, Vec<Pair>> = HashMap::new();
    let mut pairs = Vec::new();
    for i in 0..base.len() {
        for j in i + 1..base.len() {
            for si in [-1, 1] {
                for sj in [-1, 1] {
                    let key =
                        (signed_log(base_logs[i], si, n) + signed_log(base_logs[j], sj, n)) % n;
                    let terms = [(i, si), (j, sj)];
                    pairs.push((key, terms));
                    pair_hash.entry(key).or_default().push(terms);
                }
            }
        }
    }
    let mut selected_from_audit = Vec::new();
    let mut rows = Vec::new();
    let trace = rec["trace"].as_array().unwrap();
    let enabled_patterns: Vec<usize> = rec["enabled_patterns"]
        .as_array()
        .map(|v| v.iter().map(|x| u(x) as usize).collect())
        .unwrap_or_else(|| vec![1, 2, 3]);
    assert!(matches!(
        enabled_patterns.as_slice(),
        [1] | [1, 2] | [1, 2, 3]
    ));
    assert!(trace.len() <= 61);
    let frozen_trace = prior["query_trace"].as_array().unwrap();
    let mut processed_relations = 0usize;
    for (index, query) in trace.iter().enumerate() {
        let (a, b) = (u(&query["a"]), u(&query["b"]));
        assert_eq!(
            (a, b),
            (u(&frozen_trace[index][0]), u(&frozen_trace[index][1]))
        );
        let scalar = (a + mm(271, b, n)) % n;
        let target = c.mul(&g, scalar);
        let mut expected: [BTreeSet<Terms>; 5] = std::array::from_fn(|_| BTreeSet::new());
        for (left_sum, left) in &pairs {
            let complement = sub(scalar, *left_sum, n);
            if let Some(rights) = pair_hash.get(&complement) {
                for right in rights {
                    let mut terms = [left[0], left[1], right[0], right[1]];
                    terms.sort_unstable();
                    if terms.windows(2).any(|w| w[0].0 == w[1].0) {
                        continue;
                    }
                    let k = terms.iter().filter(|&&(id, _)| id >= 36).count();
                    expected[k].insert(terms);
                }
            }
        }
        let all_hits: usize = expected.iter().map(BTreeSet::len).sum();
        assert_eq!(all_hits as u64, u(&query["recorded_all_pattern_hits"]));
        let patterns = query["patterns"].as_array().unwrap();
        assert_eq!(patterns.len(), enabled_patterns.len());
        for (position, &k) in enabled_patterns.iter().enumerate() {
            let pattern = &patterns[position];
            assert_eq!(u(&pattern["k"]), k as u64);
            assert_eq!(u(&pattern["system_failures"]), 0);
            let polynomial = parse_set(&pattern["signed_relations"]);
            assert_eq!(
                polynomial, expected[k],
                "polynomial signed set at q={index}, k={k}"
            );
            for terms in &polynomial {
                add_relation(&c, &base, &target, terms);
                processed_relations += 1;
                if selected_from_audit.len() == 4 {
                    continue;
                }
                let row = row_from_terms(terms, &orbit_col, &orbit_coef, a, b, n);
                let old_rank = rank_and_solution(&rows, n).0;
                let mut candidate = rows.clone();
                candidate.push(row);
                if rank_and_solution(&candidate, n).0 > old_rank {
                    rows.push(row);
                    selected_from_audit.push(json!({"query_index":index,"k":k,"a":a,"b":b,
                        "terms":terms,"row":[row[0],row[1],row[2],row[3]],
                        "rhs":row[4],"rank":rows.len()}));
                }
            }
        }
        assert_eq!(rows.len() as u64, u(&query["rank_after"]));
    }
    assert_eq!(rec["selected_relations"], json!(selected_from_audit));
    let (rank, solution) = rank_and_solution(&rows, n);
    assert_eq!(rank, 4);
    let solved = solution.unwrap();
    assert_eq!(solved[3], 271);
    assert_eq!(u(&rec["recovered"]), solved[3]);
    assert_eq!(rec["solved_factor_and_target_logs"], json!(solved));
    let logs_by_rep = [1, solved[0], solved[1], solved[2]];
    for (id, &actual) in base_logs.iter().enumerate() {
        assert_eq!(mm(logs_by_rep[orbit_col[id]], orbit_coef[id], n), actual);
    }
    assert_eq!(rec["physical_factor_logs"], json!(base_logs));
    assert_eq!(c.mul(&g, solved[3]), q);
    let inst = construct_twist24(37, 2, 0).unwrap();
    let rho = rho_twist24(&inst, 0);
    assert!(rho.correct && rho.recovered == Some(271));
    assert_eq!(json!(rho), rec["rho"]);
    let ic_muls: u64 = [
        "model_fp_muls",
        "twist_geometry_fp_muls",
        "eigen_fp_muls",
        "base_fp_muls",
        "map_fp_muls",
        "s4_precompute_fp_muls",
        "stream_external_fp_muls",
        "stream_internal_fp_muls",
        "verify_fp_muls",
    ]
    .iter()
    .map(|key| u(&rec[*key]))
    .sum();
    let ic_groups: u64 = [
        "eigen_group_ops",
        "base_group_ops",
        "map_group_ops",
        "stream_group_ops",
        "verify_group_ops",
    ]
    .iter()
    .map(|key| u(&rec[*key]))
    .sum();
    assert_eq!(u(&rec["counted_ic_fp_muls"]), ic_muls);
    assert_eq!(u(&rec["counted_ic_group_ops"]), ic_groups);
    println!(
        "{}",
        json!({"status":"pass","p":37,"n":n,"enumerated_subgroup_logs":n,
        "pair_postings":pairs.len(),"queries":trace.len(),"audited_signed_relations":processed_relations,
        "selected_rows":rows.len(),"anchored_rank":rank,"factor_logs":solved,
        "enabled_patterns":enabled_patterns,
        "recovered":solved[3],"ic_counted_fp_muls":ic_muls,"ic_counted_group_ops":ic_groups,
        "rho_counted_fp_muls":rho.fp_muls,"rho_counted_group_ops":rho.group_ops})
    );
}
