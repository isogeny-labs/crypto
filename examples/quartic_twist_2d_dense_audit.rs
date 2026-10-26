//! Independent point-keyed signed oracle, group-log and matrix replay of dense IC.
use std::collections::{BTreeSet, HashMap};
use std::{env, fs};

use crypto_lib::cryptanalysis::gaudry_quartic::{Curve4, Pt4, E4};
use crypto_lib::cryptanalysis::residual_walk::inv_mod;
use serde_json::{json, Value};

fn u(v: &Value) -> u64 {
    v.as_u64().unwrap()
}
fn e(v: &Value) -> E4 {
    let a = v.as_array().unwrap();
    E4([u(&a[0]), u(&a[1]), u(&a[2]), u(&a[3])])
}
fn point(v: &Value) -> Pt4 {
    Pt4 {
        x: e(&v[0]),
        y: e(&v[1]),
        inf: false,
    }
}
fn modular(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

// Independent forward elimination with ordered pivots and back-substitution.
fn solve(rows: &[(Vec<u64>, u64)], cols: usize, n: u64) -> (usize, Option<Vec<u64>>) {
    let mut basis: Vec<(usize, Vec<u64>)> = Vec::new();
    for (row, rhs) in rows {
        let mut v = row.clone();
        v.push(*rhs);
        for (pivot, b) in &basis {
            let factor = v[*pivot];
            for j in *pivot..=cols {
                v[j] = (v[j] + n - modular(factor, b[j], n)) % n;
            }
        }
        let Some(pivot) = (0..cols).find(|&j| v[j] != 0) else {
            assert_eq!(v[cols], 0);
            continue;
        };
        let inv = inv_mod(v[pivot], n);
        for item in v.iter_mut().skip(pivot) {
            *item = modular(*item, inv, n);
        }
        basis.push((pivot, v));
        basis.sort_by_key(|entry| entry.0);
    }
    if basis.len() != cols {
        return (basis.len(), None);
    }
    let mut out = vec![0u64; cols];
    for (pivot, row) in basis.iter().rev() {
        let mut value = row[cols];
        for j in (pivot + 1)..cols {
            value = (value + n - modular(row[j], out[j], n)) % n;
        }
        out[*pivot] = value;
    }
    (cols, Some(out))
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(
        args.len(),
        3,
        "usage: quartic_twist_2d_dense_audit DENSE_RAW PRIOR_RAW"
    );
    let rec: Value = serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let prior: Value = serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let (prime, n) = (u(&rec["p"]), u(&rec["n"]));
    assert!([(13, 28_753), (37, 1_873_633)].contains(&(prime, n)));
    assert_eq!(
        (rec["p"].clone(), rec["n"].clone()),
        (prior["p"].clone(), prior["n"].clone())
    );
    assert_eq!(rec["base_coordinates"], prior["base_coordinates"]);
    assert_eq!(rec["generator_x"], prior["generator_x"]);
    assert_eq!(rec["generator_y"], prior["generator_y"]);
    assert_eq!(rec["target_x"], prior["target_x"]);
    assert_eq!(rec["target_y"], prior["target_y"]);
    assert_eq!(rec["query_trace"].as_array().unwrap().len(), 1);
    assert_eq!(rec["query_trace"][0], prior["query_trace"][0]);
    let curve = Curve4::with_params(prime, E4::ZERO, E4([0, 0, 0, u(&rec["b0"])]));
    let base: Vec<_> = rec["base_coordinates"]
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
    assert_eq!(base[0], g);
    assert_eq!(curve.mul(&g, n), Pt4::INF);
    assert_eq!(curve.mul(&g, 271), q);
    let mut logs = HashMap::new();
    let mut acc = Pt4::INF;
    for k in 0..n {
        assert!(logs.insert(acc, k).is_none());
        acc = curve.add(&acc, &g);
    }
    assert_eq!(acc, Pt4::INF);
    let point_logs: Vec<_> = base.iter().map(|pt| *logs.get(pt).unwrap()).collect();
    let t = E4([0, 1, 0, 0]);
    let z = curve.f.pow(&t, (prime - 1) as u128).0[0];
    let h = curve
        .f
        .pow(&t, (prime as u128).pow(4) - 1 - (prime - 1) as u128 / 2);
    let h2 = curve.f.sq(&h);
    let omega = (2..prime)
        .find(|&v| v != 1 && v * v * v % prime == 1)
        .unwrap();
    let by_x: HashMap<_, _> = base.iter().enumerate().map(|(i, pt)| (pt.x, i)).collect();
    let mut col = vec![usize::MAX; base.len()];
    let mut coef = vec![0u64; base.len()];
    let mut reps = Vec::new();
    for (i, pt) in base.iter().enumerate() {
        if col[i] != usize::MAX {
            continue;
        }
        let ci = reps.len();
        reps.push(i);
        let mut seen = BTreeSet::new();
        let mut queue = vec![pt.x];
        seen.insert(pt.x);
        let mut next = 0;
        while next < queue.len() {
            let x = queue[next];
            next += 1;
            for y in [
                curve.f.mul(&h2, &curve.f.frob_p(&x, z)),
                curve.f.scale(&x, omega),
            ] {
                assert!(by_x.contains_key(&y));
                if seen.insert(y) {
                    queue.push(y);
                }
            }
        }
        assert_eq!(seen.len(), 12);
        let inverse = inv_mod(point_logs[i], n);
        for x in seen {
            let j = by_x[&x];
            assert_eq!(col[j], usize::MAX);
            col[j] = ci;
            coef[j] = modular(point_logs[j], inverse, n);
            assert_eq!(curve.mul(&base[i], coef[j]), base[j]);
        }
    }
    let folded = reps.len();
    assert_eq!(folded, u(&rec["folded_columns"]) as usize);
    assert_eq!(reps[0], 0);
    assert_eq!(u(&rec["known_log_column"]), 0);
    assert_eq!(u(&rec["known_log_value"]), 1);
    assert_eq!(u(&rec["anchored_unknowns"]), folded as u64);
    let (a, b) = (u(&rec["query_trace"][0][0]), u(&rec["query_trace"][0][1]));
    let residual = curve.add(&curve.mul(&g, a), &curve.mul(&q, b));
    // Four signs per unordered pair; key by the *whole point*, unlike the producer.
    let mut pair_hash: HashMap<Pt4, Vec<[(usize, i64); 2]>> = HashMap::new();
    let mut pairs = Vec::new();
    for i in 0..base.len() {
        for j in (i + 1)..base.len() {
            for si in [-1, 1] {
                for sj in [-1, 1] {
                    let x = if si == 1 {
                        base[i]
                    } else {
                        curve.neg(&base[i])
                    };
                    let y = if sj == 1 {
                        base[j]
                    } else {
                        curve.neg(&base[j])
                    };
                    let sum = curve.add(&x, &y);
                    let terms = [(i, si), (j, sj)];
                    pair_hash.entry(sum).or_default().push(terms);
                    pairs.push((sum, terms));
                }
            }
        }
    }
    let mut solutions = BTreeSet::new();
    for (sum, left) in &pairs {
        let remaining = curve.sub(&residual, sum);
        if let Some(rights) = pair_hash.get(&remaining) {
            for right in rights {
                if left[0].0 == right[0].0
                    || left[0].0 == right[1].0
                    || left[1].0 == right[0].0
                    || left[1].0 == right[1].0
                {
                    continue;
                }
                let mut terms = left.to_vec();
                terms.extend(*right);
                terms.sort_unstable_by_key(|term| term.0);
                solutions.insert(terms);
            }
        }
    }
    assert_eq!(solutions.len() as u64, u(&rec["query_trace"][0][2]));
    assert_eq!(solutions.len() as u64, u(&rec["decompositions"]));
    let solutions: Vec<_> = solutions.into_iter().collect();
    let mut rows = Vec::new();
    let selected = rec["selected_relations"].as_array().unwrap();
    for item in selected {
        assert_eq!((u(&item["a"]), u(&item["b"])), (a, b));
        assert_eq!(u(&item["query_index"]), 0);
        let index = u(&item["decomposition_index"]) as usize;
        let terms: Vec<(usize, i64)> = item["terms"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| (u(&v[0]) as usize, v[1].as_i64().unwrap()))
            .collect();
        assert_eq!(solutions[index], terms);
        let mut replay = Pt4::INF;
        let mut full_row = vec![0u64; folded + 1];
        for &(i, sign) in &terms {
            replay = if sign == 1 {
                curve.add(&replay, &base[i])
            } else {
                curve.sub(&replay, &base[i])
            };
            let value = if sign == 1 {
                coef[i]
            } else {
                (n - coef[i]) % n
            };
            full_row[col[i]] = (full_row[col[i]] + value) % n;
        }
        assert_eq!(replay, residual);
        full_row[folded] = (n - b) % n;
        assert_eq!(
            full_row,
            item["full_row"]
                .as_array()
                .unwrap()
                .iter()
                .map(u)
                .collect::<Vec<_>>()
        );
        let anchored = full_row[1..].to_vec();
        let rhs = (a + n - full_row[0]) % n;
        assert_eq!(
            anchored,
            item["row"]
                .as_array()
                .unwrap()
                .iter()
                .map(u)
                .collect::<Vec<_>>()
        );
        assert_eq!(rhs, u(&item["rhs"]));
        rows.push((anchored, rhs));
        assert_eq!(solve(&rows, folded, n).0, u(&item["rank"]) as usize);
    }
    assert_eq!(selected.len(), folded);
    let (rank, solution) = solve(&rows, folded, n);
    assert_eq!(rank, folded);
    let solution = solution.unwrap();
    assert_eq!(solution[folded - 1], 271);
    assert_eq!(curve.mul(&g, solution[folded - 1]), q);
    let factor_logs: Vec<_> = std::iter::once(1)
        .chain(solution[..folded - 1].iter().copied())
        .collect();
    assert_eq!(
        factor_logs,
        rec["factor_logs"]
            .as_array()
            .unwrap()
            .iter()
            .map(u)
            .collect::<Vec<_>>()
    );
    for (i, &rep) in reps.iter().enumerate() {
        assert_eq!(point_logs[rep], factor_logs[i]);
    }
    // For one fixed residual, coefficient differences have zero target coordinate
    // and annihilate the nonzero factor-log vector (its first entry is 1): rank <= folded.
    // The anchored rank gives the matching lower bound by projection.
    assert_eq!(u(&rec["unanchored_rank_first_query"]), folded as u64);
    println!(
        "{}",
        json!({"p":prime,"n":n,"point_logs_enumerated":logs.len(),
        "decompositions_replayed":solutions.len(),"selected_rows_replayed":rows.len(),
        "anchored_rank":rank,"unanchored_rank_derived":folded,"recovered":271,"status":"pass"})
    );
}
