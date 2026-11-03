//! Independent point-keyed signed-pair and BSGS audit of the k=1 scale panel.
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
fn mm(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn sub(a: u64, b: u64, n: u64) -> u64 {
    (a + n - b) % n
}
fn signed(v: &Pt4, sign: i8, c: &Curve4) -> Pt4 {
    if sign == 1 {
        *v
    } else {
        c.neg(v)
    }
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
        .map(|x| serde_json::from_value(x.clone()).unwrap())
        .collect()
}

/// Independent subgroup logarithms with a reusable sqrt(n)-point baby table.
fn bsgs_logs(c: &Curve4, g: &Pt4, targets: &[Pt4], n: u64) -> (Vec<u64>, u64) {
    let mut m = 1u64;
    while m * m < n {
        m += 1;
    }
    let mut baby = HashMap::with_capacity(m as usize);
    let mut acc = Pt4::INF;
    for j in 0..m {
        assert!(baby.insert(acc, j).is_none());
        acc = c.add(&acc, g);
    }
    let giant = c.neg(&acc);
    let mut out = Vec::with_capacity(targets.len());
    for target in targets {
        let mut probe = *target;
        let mut found = None;
        for i in 0..=m {
            if let Some(&j) = baby.get(&probe) {
                let log = i * m + j;
                if log < n && c.mul(g, log) == *target {
                    found = Some(log);
                    break;
                }
            }
            probe = c.add(&probe, &giant);
        }
        out.push(found.expect("independent BSGS logarithm"));
    }
    (out, m)
}

/// A separately ordered exact k=1 group-pair oracle: two signed L0 points
/// are hashed by their *group sum*, then one L0 and one L2 are subtracted.
fn pair_table(c: &Curve4, base: &[Pt4], line0_len: usize) -> HashMap<Pt4, Vec<Pair>> {
    let mut hash = HashMap::<Pt4, Vec<Pair>>::new();
    for j in 1..line0_len {
        for i in 0..j {
            for sj in [1, -1] {
                for si in [1, -1] {
                    let sum = c.add(&signed(&base[j], sj, c), &signed(&base[i], si, c));
                    hash.entry(sum).or_default().push([(i, si), (j, sj)]);
                }
            }
        }
    }
    hash
}

fn exact_k1(
    c: &Curve4,
    base: &[Pt4],
    line0_len: usize,
    pairs: &HashMap<Pt4, Vec<Pair>>,
    target: &Pt4,
) -> BTreeSet<Terms> {
    let mut relations = BTreeSet::new();
    for j in line0_len..base.len() {
        for sj in [1, -1] {
            let after_l2 = c.sub(target, &signed(&base[j], sj, c));
            for i in 0..line0_len {
                for si in [1, -1] {
                    let needed = c.sub(&after_l2, &signed(&base[i], si, c));
                    if let Some(first) = pairs.get(&needed) {
                        for pair in first {
                            if pair[0].0 == i || pair[1].0 == i {
                                continue;
                            }
                            let mut terms = [pair[0], pair[1], (i, si), (j, sj)];
                            terms.sort_unstable();
                            relations.insert(terms);
                        }
                    }
                }
            }
        }
    }
    relations
}

fn rank_and_solution(rows: &[Vec<u64>], cols: usize, n: u64) -> (usize, Option<Vec<u64>>) {
    let mut a = rows.to_vec();
    let mut rank = 0;
    let mut pivots = Vec::new();
    for col in 0..cols {
        let Some(pivot) = (rank..a.len()).find(|&i| a[i][col] != 0) else {
            continue;
        };
        a.swap(rank, pivot);
        let inverse = inv_mod(a[rank][col], n);
        for j in col..=cols {
            a[rank][j] = mm(a[rank][j], inverse, n);
        }
        for i in 0..a.len() {
            if i == rank {
                continue;
            }
            let factor = a[i][col];
            for j in col..=cols {
                a[i][j] = sub(a[i][j], mm(factor, a[rank][j], n), n);
            }
        }
        pivots.push(col);
        rank += 1;
        if rank == a.len() {
            break;
        }
    }
    for row in a.iter().skip(rank) {
        assert_eq!(row[cols], 0, "inconsistent audited relation");
    }
    if rank == cols {
        assert_eq!(pivots, (0..cols).collect::<Vec<_>>());
        (rank, Some(a.iter().take(cols).map(|r| r[cols]).collect()))
    } else {
        (rank, None)
    }
}

fn row_from_terms(
    terms: &Terms,
    col: &[usize],
    coef: &[u64],
    anchor_log: u64,
    a: u64,
    b: u64,
    n: u64,
    cols: usize,
) -> Vec<u64> {
    let mut sums = vec![0u64; cols];
    for &(id, sign) in terms {
        sums[col[id]] = (sums[col[id]] + signed_log(coef[id], sign, n)) % n;
    }
    let mut row = sums[1..].to_vec();
    row.push(sub(0, b, n));
    row.push(sub(a, mm(anchor_log, sums[0], n), n));
    row
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(
        args.len(),
        3,
        "usage: quartic_twist_k1_scale_audit RUN_JSON MIXED_NDJSON"
    );
    let rec: Value = serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let source: Vec<Value> = fs::read_to_string(&args[2])
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let p = u(&rec["p"]);
    let b0 = u(&rec["b0"]);
    let seed = u(&rec["seed"]);
    let n = u(&rec["n"]);
    let prior = source
        .iter()
        .find(|v| u(&v["p"]) == p && u(&v["seed"]) == seed && v["line_powers"] == json!([0, 2]))
        .unwrap();
    assert_eq!(rec["base_coordinates"], prior["base_coordinates"]);
    assert_eq!(rec["enabled_patterns"], json!([1]));
    let frozen_trace = prior["query_trace"].as_array().unwrap();
    assert_eq!(u(&rec["recorded_stream_cap"]) as usize, frozen_trace.len());
    let c = Curve4::with_params(p, E4::ZERO, E4([0, 0, 0, b0]));
    let base: Vec<Pt4> = rec["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    assert!(base.iter().all(|pt| c.on_curve(pt)));
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
    assert!(c.on_curve(&g) && c.on_curve(&q) && c.mul(&g, n).inf);
    let mut targets = base.clone();
    targets.push(q);
    let (logs, baby_steps) = bsgs_logs(&c, &g, &targets, n);
    let base_logs = &logs[..base.len()];
    assert_eq!(logs[base.len()], 271);
    let anchor = base_logs[0];
    assert!(anchor == 1 || anchor == n - 1);
    assert_eq!(u(&rec["anchor_log"]), anchor);
    let line0_len = base.iter().take_while(|pt| pt.x.0[2] == 0).count();
    let cols = base.len() / 12;
    assert_eq!(u(&rec["folded_columns"]) as usize, cols);

    let inst = construct_twist24(p, b0, seed).unwrap();
    assert_eq!(inst.core.g, g);
    assert_eq!(inst.core.q, q);
    assert_eq!(inst.lambda_frob, u(&rec["lambda_frob"]));
    assert_eq!(inst.core.lambda, u(&rec["lambda_cubic"]));
    let lf = inst.lambda_frob;
    let lc = inst.core.lambda;
    let by_log: HashMap<u64, usize> = base_logs.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let mut orbit_col = vec![usize::MAX; base.len()];
    let mut orbit_coef = vec![0u64; base.len()];
    let mut reps = Vec::new();
    for (id, &rep_log) in base_logs.iter().enumerate() {
        if orbit_col[id] != usize::MAX {
            continue;
        }
        let ci = reps.len();
        reps.push(id);
        let inv = inv_mod(rep_log, n);
        let mut covered = BTreeSet::new();
        for j in 0..4 {
            for k in 0..3 {
                let wanted = mm(rep_log, mm(pow_mod(lf, j, n), pow_mod(lc, k, n), n), n);
                let idx = by_log
                    .get(&wanted)
                    .or_else(|| by_log.get(&sub(0, wanted, n)))
                    .copied()
                    .expect("scalar orbit left base");
                covered.insert(idx);
                let coef = mm(base_logs[idx], inv, n);
                if orbit_col[idx] == usize::MAX {
                    orbit_col[idx] = ci;
                    orbit_coef[idx] = coef;
                } else {
                    assert_eq!((orbit_col[idx], orbit_coef[idx]), (ci, coef));
                }
            }
        }
        assert_eq!(covered.len(), 12);
    }
    assert_eq!(reps.len(), cols);
    assert_eq!(rec["orbit_column"], json!(orbit_col));
    assert_eq!(rec["orbit_coefficient"], json!(orbit_coef));
    assert_eq!(rec["orbit_representatives"], json!(reps));

    let pairs = pair_table(&c, &base, line0_len);
    let pair_entries = 4 * line0_len * (line0_len - 1) / 2;
    let mut selected = Vec::new();
    let mut rows: Vec<Vec<u64>> = Vec::new();
    let mut expected_total = 0usize;
    let mut polynomial_total = 0usize;
    let mut missing_total = 0usize;
    let mut failed_systems = 0u64;
    let trace = rec["trace"].as_array().unwrap();
    assert!(trace.len() <= frozen_trace.len());
    for (index, query) in trace.iter().enumerate() {
        let (a, b) = (u(&query["a"]), u(&query["b"]));
        assert_eq!(
            (a, b),
            (u(&frozen_trace[index][0]), u(&frozen_trace[index][1]))
        );
        let target = c.add(&c.mul(&g, a), &c.mul(&q, b));
        let expected = exact_k1(&c, &base, line0_len, &pairs, &target);
        let pattern = &query["patterns"][0];
        assert_eq!(u(&pattern["k"]), 1);
        let polynomial = parse_set(&pattern["signed_relations"]);
        assert!(
            polynomial.is_subset(&expected),
            "foreign polynomial relation at q={index}"
        );
        expected_total += expected.len();
        polynomial_total += polynomial.len();
        missing_total += expected.difference(&polynomial).count();
        failed_systems += u(&pattern["system_failures"]);
        for terms in &polynomial {
            let mut sum = Pt4::INF;
            for &(id, sign) in terms {
                sum = c.add(&sum, &signed(&base[id], sign, &c));
            }
            assert_eq!(sum, target);
            if rows.len() == cols {
                continue;
            }
            let row = row_from_terms(terms, &orbit_col, &orbit_coef, anchor, a, b, n, cols);
            let old_rank = rank_and_solution(&rows, cols, n).0;
            let mut candidate = rows.clone();
            candidate.push(row.clone());
            if rank_and_solution(&candidate, cols, n).0 > old_rank {
                rows.push(row.clone());
                selected.push(json!({"query_index":index,"k":1,"a":a,"b":b,
                    "terms":terms,"row":&row[..cols],"rhs":row[cols],"rank":rows.len()}));
            }
        }
        assert_eq!(rows.len() as u64, u(&query["rank_after"]));
    }
    assert_eq!(rec["selected_relations"], json!(selected));
    let (rank, solution) = rank_and_solution(&rows, cols, n);
    assert_eq!(rank as u64, u(&rec["anchored_rank"]));
    assert_eq!(rec["solved_factor_and_target_logs"], json!(solution));
    let recovered = solution.as_ref().map(|v| v[cols - 1]);
    assert_eq!(rec["recovered"], json!(recovered));
    if let Some(logs) = &solution {
        let rep_logs = std::iter::once(anchor)
            .chain(logs[..cols - 1].iter().copied())
            .collect::<Vec<_>>();
        for (id, &actual) in base_logs.iter().enumerate() {
            assert_eq!(mm(rep_logs[orbit_col[id]], orbit_coef[id], n), actual);
        }
        assert_eq!(rec["physical_factor_logs"], json!(base_logs));
        assert_eq!(c.mul(&g, logs[cols - 1]), q);
    }
    let rho = rho_twist24(&inst, seed);
    assert_eq!(json!(rho), rec["rho"]);
    let ic_fp_muls: u64 = [
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
    .map(|k| u(&rec[*k]))
    .sum();
    let ic_group_ops: u64 = [
        "eigen_group_ops",
        "base_group_ops",
        "map_group_ops",
        "stream_group_ops",
        "verify_group_ops",
    ]
    .iter()
    .map(|k| u(&rec[*k]))
    .sum();
    assert_eq!(ic_fp_muls, u(&rec["counted_ic_fp_muls"]));
    assert_eq!(ic_group_ops, u(&rec["counted_ic_group_ops"]));
    assert_eq!(failed_systems, u(&rec["failed_systems"]));
    let complete = missing_total == 0 && failed_systems == 0;
    let success = complete && recovered == Some(271) && rho.correct && rho.recovered == Some(271);
    assert_eq!(rec["status"], if success { "pass" } else { "partial" });
    println!(
        "{}",
        json!({"status":if success {"pass"} else {"partial"},
        "p":p,"b0":b0,"n":n,"seed":seed,"anchor_log":anchor,
        "independent_bsgs_baby_steps":baby_steps,"base_logs_verified":base_logs.len(),
        "pair_entries":pair_entries,"processed_queries":trace.len(),
        "expected_signed_relations":expected_total,"polynomial_signed_relations":polynomial_total,
        "missing_signed_relations":missing_total,"system_failures":failed_systems,
        "anchored_rank":rank,"recovered":recovered,"factor_logs_verified":solution.is_some(),
        "ic_counted_fp_muls":ic_fp_muls,"ic_counted_group_ops":ic_group_ops,
        "rho_counted_fp_muls":rho.fp_muls,"rho_counted_group_ops":rho.group_ops})
    );
}
