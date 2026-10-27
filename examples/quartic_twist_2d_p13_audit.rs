//! Independent signed-pair, orbit/log, row, and rank replay of the p=13 pilot.
use std::collections::{BTreeSet, HashMap};
use std::env;
use std::fs;

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
fn p(v: &Value) -> Pt4 {
    Pt4 {
        x: e(&v[0]),
        y: e(&v[1]),
        inf: false,
    }
}
fn mul(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn rank(rows: &[(Vec<u64>, u64)], cols: usize, n: u64) -> (usize, Option<Vec<u64>>) {
    let mut basis: Vec<(usize, Vec<u64>)> = Vec::new();
    for (row, rhs) in rows {
        let mut v = row.clone();
        v.push(*rhs);
        for (pivot, b) in &basis {
            let factor = v[*pivot];
            for j in *pivot..=cols {
                v[j] = (v[j] + n - mul(factor, b[j], n)) % n;
            }
        }
        let Some(pivot) = (0..cols).find(|&j| v[j] != 0) else {
            assert_eq!(v[cols], 0);
            continue;
        };
        let inv = inv_mod(v[pivot], n);
        for item in v.iter_mut().skip(pivot) {
            *item = mul(*item, inv, n);
        }
        basis.push((pivot, v));
        basis.sort_by_key(|item| item.0);
    }
    if basis.len() != cols {
        return (basis.len(), None);
    }
    let mut solution = vec![0u64; cols];
    for (pivot, row) in basis.iter().rev() {
        let mut value = row[cols];
        for j in (pivot + 1)..cols {
            value = (value + n - mul(row[j], solution[j], n)) % n;
        }
        solution[*pivot] = value;
    }
    (cols, Some(solution))
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(args.len(), 2, "usage: quartic_twist_2d_p13_audit RECORD");
    let rec: Value = serde_json::from_str(&fs::read_to_string(&args[1]).unwrap()).unwrap();
    let (prime, n) = (u(&rec["p"]), u(&rec["n"]));
    assert_eq!((prime, n), (13, 28_753));
    let c = Curve4::with_params(prime, E4::ZERO, E4([0, 0, 0, u(&rec["b0"])]));
    let base: Vec<Pt4> = rec["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(p)
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
    assert_eq!(c.mul(&g, n), Pt4::INF);
    assert_eq!(c.mul(&g, 271), q);
    let mut logs = HashMap::new();
    let mut acc = Pt4::INF;
    for k in 0..n {
        assert!(logs.insert(acc, k).is_none());
        acc = c.add(&acc, &g);
    }
    assert_eq!(acc, Pt4::INF);
    let point_logs: Vec<_> = base.iter().map(|pt| *logs.get(pt).unwrap()).collect();
    let t = E4([0, 1, 0, 0]);
    let z = c.f.pow(&t, (prime - 1) as u128).0[0];
    let h =
        c.f.pow(&t, (prime as u128).pow(4) - 1 - (prime - 1) as u128 / 2);
    let h2 = c.f.sq(&h);
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
            for y in [c.f.mul(&h2, &c.f.frob_p(&x, z)), c.f.scale(&x, omega)] {
                assert!(by_x.contains_key(&y));
                if seen.insert(y) {
                    queue.push(y);
                }
            }
        }
        assert_eq!(seen.len(), 12);
        let inv_rep = inv_mod(point_logs[i], n);
        for x in seen {
            let j = by_x[&x];
            assert_eq!(col[j], usize::MAX);
            col[j] = ci;
            coef[j] = mul(point_logs[j], inv_rep, n);
            assert_eq!(c.mul(&base[i], coef[j]), base[j]);
        }
    }
    assert_eq!(reps.len(), 6);
    assert_eq!(u(&rec["folded_columns"]), 6);
    // Hash all four signed choices for each unordered physical pair.
    let mut pair_hash: HashMap<Pt4, Vec<[(usize, i64); 2]>> = HashMap::new();
    let mut pairs = Vec::new();
    for i in 0..base.len() {
        for j in (i + 1)..base.len() {
            for si in [-1, 1] {
                for sj in [-1, 1] {
                    let a = if si == 1 { base[i] } else { c.neg(&base[i]) };
                    let b = if sj == 1 { base[j] } else { c.neg(&base[j]) };
                    let sum = c.add(&a, &b);
                    let terms = [(i, si), (j, sj)];
                    pair_hash.entry(sum).or_default().push(terms);
                    pairs.push((sum, terms));
                }
            }
        }
    }
    assert_eq!(pairs.len(), base.len() * (base.len() - 1) * 2);
    let mut rows = Vec::new();
    let mut total_decompositions = 0u64;
    let selected = rec["selected_relations"].as_array().unwrap();
    for (index, item) in rec["query_trace"].as_array().unwrap().iter().enumerate() {
        let (a, b) = (u(&item[0]), u(&item[1]));
        let residual = c.add(&c.mul(&g, a), &c.mul(&q, b));
        let mut solutions = BTreeSet::new();
        for (sum, left) in &pairs {
            let remaining = c.sub(&residual, sum);
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
        assert_eq!(solutions.len() as u64, u(&item[2]));
        total_decompositions += solutions.len() as u64;
        if index >= selected.len() {
            continue;
        }
        let sel = &selected[index];
        assert_eq!((u(&sel["a"]), u(&sel["b"])), (a, b));
        let terms: Vec<(usize, i64)> = sel["terms"]
            .as_array()
            .unwrap()
            .iter()
            .map(|z| (u(&z[0]) as usize, z[1].as_i64().unwrap()))
            .collect();
        assert!(solutions.contains(&terms));
        let mut row = vec![0u64; 7];
        let mut replay = Pt4::INF;
        for &(i, sign) in &terms {
            let point = if sign == 1 { base[i] } else { c.neg(&base[i]) };
            replay = c.add(&replay, &point);
            let value = if sign == 1 {
                coef[i]
            } else {
                (n - coef[i]) % n
            };
            row[col[i]] = (row[col[i]] + value) % n;
        }
        assert_eq!(replay, residual);
        row[6] = (n - b) % n;
        assert_eq!(
            row,
            sel["row"]
                .as_array()
                .unwrap()
                .iter()
                .map(u)
                .collect::<Vec<_>>()
        );
        rows.push((row, a));
        assert_eq!(rank(&rows, 7, n).0, u(&sel["rank"]) as usize);
    }
    assert_eq!(total_decompositions, u(&rec["decompositions"]));
    let (final_rank, solution) = rank(&rows, 7, n);
    assert_eq!(final_rank, 7);
    let answer = solution.unwrap();
    assert_eq!(answer[6], 271);
    assert_eq!(c.mul(&g, answer[6]), q);
    assert_eq!(
        answer,
        rec["factor_logs"]
            .as_array()
            .unwrap()
            .iter()
            .map(u)
            .chain([271])
            .collect::<Vec<_>>()
    );
    for (i, &rep) in reps.iter().enumerate() {
        assert_eq!(point_logs[rep], answer[i]);
    }
    println!(
        "{}",
        json!({"p":prime,"n":n,"queries_replayed":rec["query_trace"].as_array().unwrap().len(),
        "decompositions_replayed":total_decompositions,"selected_rows_replayed":rows.len(),
        "point_logs_enumerated":logs.len(),"rank":final_rank,"recovered":answer[6],"status":"pass"})
    );
}
