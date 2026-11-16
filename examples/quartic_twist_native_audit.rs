//! Native independent row/rank/scalar replay for the quartic twist panels.
//! Usage: cargo run --release --example quartic_twist_native_audit -- FILE
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;

use crypto_lib::cryptanalysis::gaudry_quartic::{Curve4, Pt4, E4};
use crypto_lib::cryptanalysis::residual_walk::{inv_mod, is_prime_u64, pow_mod};
use rayon::prelude::*;
use serde_json::{json, Value};

fn u(v: &Value) -> u64 {
    v.as_u64().expect("unsigned integer")
}
fn a4(v: &Value) -> E4 {
    let a = v.as_array().expect("field element");
    E4([u(&a[0]), u(&a[1]), u(&a[2]), u(&a[3])])
}
fn pt(x: &Value, y: &Value) -> Pt4 {
    Pt4 {
        x: a4(x),
        y: a4(y),
        inf: false,
    }
}
fn mm(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn neg(a: u64, n: u64) -> u64 {
    if a == 0 {
        0
    } else {
        n - a
    }
}
fn psi(c: &Curve4, p: Pt4, h2: E4, h3: E4, omega: u64, frob: bool, cubic: u64) -> Pt4 {
    if p.inf {
        return p;
    }
    let (mut x, mut y) = (p.x, p.y);
    if frob {
        x = c.f.mul(&h2, &c.f.pow(&x, c.f.p as u128));
        y = c.f.mul(&h3, &c.f.pow(&y, c.f.p as u128));
    }
    x = c.f.scale(&x, pow_mod(omega, cubic, c.f.p));
    Pt4 { x, y, inf: false }
}

fn columns(
    c: &Curve4,
    pts: &[Pt4],
    n: u64,
    omega: u64,
    lf: u64,
    lc: u64,
    h2: E4,
    h3: E4,
    combined: bool,
) -> (Vec<usize>, Vec<u64>, usize) {
    let idx: HashMap<E4, usize> = pts.iter().enumerate().map(|(i, p)| (p.x, i)).collect();
    assert_eq!(idx.len(), pts.len());
    let mut col = vec![usize::MAX; pts.len()];
    let mut coef = vec![0; pts.len()];
    let mut cols = 0;
    for (i, &rep) in pts.iter().enumerate() {
        if col[i] != usize::MAX {
            continue;
        }
        let mut frob_image = rep;
        for j in 0..if combined { 4 } else { 1 } {
            if j > 0 {
                frob_image = psi(c, frob_image, h2, h3, omega, true, 0);
            }
            for k in 0..3 {
                let image = psi(c, frob_image, h2, h3, omega, false, k);
                let m = mm(pow_mod(lf, j as u64, n), pow_mod(lc, k, n), n);
                assert_eq!(c.mul(&rep, m), image, "coefficient replay");
                let at = *idx.get(&image.x).expect("orbit closure");
                let signed = if pts[at].y == image.y {
                    m
                } else {
                    assert_eq!(pts[at].y, c.f.neg(&image.y));
                    neg(m, n)
                };
                if col[at] == usize::MAX {
                    col[at] = cols;
                    coef[at] = signed;
                } else {
                    assert_eq!((col[at], coef[at]), (cols, signed));
                }
            }
        }
        cols += 1;
    }
    assert!(col.iter().all(|&x| x != usize::MAX));
    (col, coef, cols)
}

fn row(terms: &Value, col: &[usize], coef: &[u64], nc: usize, a: u64, b: u64, n: u64) -> Value {
    let mut map = BTreeMap::<usize, u64>::new();
    for term in terms.as_array().unwrap() {
        let i = u(&term[0]) as usize;
        let sign = term[1].as_i64().unwrap();
        let x = if sign == 1 { coef[i] } else { neg(coef[i], n) };
        *map.entry(col[i]).or_default() = (*map.get(&col[i]).unwrap_or(&0) + x) % n;
    }
    map.retain(|_, v| *v != 0);
    map.insert(nc, neg(b, n));
    json!({"cols":map.into_iter().collect::<Vec<_>>(),"rhs":a})
}

fn target_from_rows(rows: &[Value], nc: usize, n: u64) -> (usize, Option<u64>) {
    let mut pivots: BTreeMap<usize, (BTreeMap<usize, u64>, u64)> = BTreeMap::new();
    for item in rows {
        let mut v: BTreeMap<usize, u64> = item["cols"]
            .as_array()
            .unwrap()
            .iter()
            .map(|z| (u(&z[0]) as usize, u(&z[1]) % n))
            .filter(|&(_, x)| x != 0)
            .collect();
        let mut rhs = u(&item["rhs"]) % n;
        while let Some((&pivot, &value)) = v.iter().next() {
            if let Some((basis, brhs)) = pivots.get(&pivot) {
                for (&key, &x) in basis {
                    let z = (u128::from(*v.get(&key).unwrap_or(&0)) + n as u128
                        - mm(value, x, n) as u128)
                        % n as u128;
                    if z == 0 {
                        v.remove(&key);
                    } else {
                        v.insert(key, z as u64);
                    }
                }
                rhs = (rhs + n - mm(value, *brhs, n)) % n;
            } else {
                let inv = inv_mod(value, n);
                for x in v.values_mut() {
                    *x = mm(*x, inv, n);
                }
                rhs = mm(rhs, inv, n);
                pivots.insert(pivot, (v.clone(), rhs));
                break;
            }
        }
        if v.is_empty() {
            assert_eq!(rhs, 0, "inconsistent relation matrix");
        }
    }
    let rank = pivots.len();
    let mut v = BTreeMap::from([(nc, 1u64)]);
    let mut answer = 0u64;
    while let Some((&pivot, &value)) = v.iter().next() {
        let Some((basis, brhs)) = pivots.get(&pivot) else {
            return (rank, None);
        };
        for (&key, &x) in basis {
            let z = (u128::from(*v.get(&key).unwrap_or(&0)) + n as u128 - mm(value, x, n) as u128)
                % n as u128;
            if z == 0 {
                v.remove(&key);
            } else {
                v.insert(key, z as u64);
            }
        }
        answer = (answer + mm(value, *brhs, n)) % n;
    }
    (rank, Some(answer))
}

/// A second four-sum oracle: hash every fully signed two-point sum, then
/// complement another signed pair. This differs from the producer's
/// abscissa/y-sign table and independently checks every recorded hit count.
fn replay_query_trace(
    p: u64,
    b0: u64,
    g: Pt4,
    target: Pt4,
    pts: &[Pt4],
    trace: &[(u64, u64, u16)],
) {
    type Pair = (Pt4, usize, usize, i8, i8);
    let c = Curve4::with_params(p, E4::ZERO, E4([0, 0, 0, b0]));
    let mut pairs = Vec::<Pair>::new();
    let mut index = HashMap::<Pt4, Vec<usize>>::new();
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            for si in [1i8, -1] {
                for sj in [1i8, -1] {
                    let a = if si == 1 { pts[i] } else { c.neg(&pts[i]) };
                    let b = if sj == 1 { pts[j] } else { c.neg(&pts[j]) };
                    let sum = c.add(&a, &b);
                    let at = pairs.len();
                    pairs.push((sum, i, j, si, sj));
                    index.entry(sum).or_default().push(at);
                }
            }
        }
    }
    trace.par_iter().for_each(|&(a, b, expected)| {
        let c = Curve4::with_params(p, E4::ZERO, E4([0, 0, 0, b0]));
        let r = c.add(&c.mul(&g, a), &c.mul(&target, b));
        let mut unique = HashSet::<[(usize, i8); 4]>::new();
        for &(sum, i, j, si, sj) in &pairs {
            let needed = c.sub(&r, &sum);
            if let Some(matches) = index.get(&needed) {
                for &k in matches {
                    let (_, u, v, su, sv) = pairs[k];
                    if i == u || i == v || j == u || j == v {
                        continue;
                    }
                    let mut terms = [(i, si), (j, sj), (u, su), (v, sv)];
                    terms.sort_unstable();
                    unique.insert(terms);
                }
            }
        }
        assert_eq!(
            unique.len(),
            expected as usize,
            "independent query yield p={p} a={a} b={b}"
        );
    });
}

fn assert_row_scaled(before: &Value, after: &Value, multiplier: u64, n: u64) {
    assert_eq!(u(&after["rhs"]), mm(u(&before["rhs"]), multiplier, n));
    let old = before["cols"].as_array().unwrap();
    let new = after["cols"].as_array().unwrap();
    assert_eq!(old.len(), new.len());
    for (x, y) in old.iter().zip(new) {
        assert_eq!(u(&x[0]), u(&y[0]));
        assert_eq!(mm(u(&x[1]), multiplier, n), u(&y[1]));
    }
}

fn audit(rec: &Value) -> Value {
    let (p, b0, n, seed) = (u(&rec["p"]), u(&rec["b0"]), u(&rec["n"]), u(&rec["seed"]));
    assert!([
        (37, 2),
        (73, 5),
        (97, 5),
        (157, 25),
        (229, 6),
        (241, 49),
        (337, 100)
    ]
    .contains(&(p, b0)));
    assert!(is_prime_u64(p) && is_prime_u64(n) && n > 4 * p * p);
    let c = Curve4::with_params(p, E4::ZERO, E4([0, 0, 0, b0]));
    let qfield = (p as u128).pow(4);
    assert!(n as u128 >= qfield + 1 - 2 * (p as u128).pow(2));
    assert!(n as u128 <= qfield + 1 + 2 * (p as u128).pow(2));
    let g = pt(&rec["generator_x"], &rec["generator_y"]);
    let target = pt(&rec["target_x"], &rec["target_y"]);
    assert!(c.on_curve(&g) && c.on_curve(&target));
    assert!(c.mul(&g, n).inf);
    assert_eq!(u(&rec["known_scalar"]), 271);
    assert_eq!(c.mul(&g, 271), target);
    let t = E4([0, 1, 0, 0]);
    let h = c.f.pow(&t, qfield - 1 - (p - 1) as u128 / 2);
    let h2 = c.f.sq(&h);
    let h3 = c.f.mul(&h2, &h);
    let (omega, lf, lc) = (
        u(&rec["omega"]),
        u(&rec["lambda_frob"]),
        u(&rec["lambda_cubic"]),
    );
    assert_eq!(pow_mod(lf, 4, n), n - 1);
    assert_eq!(pow_mod(lc, 3, n), 1);
    assert_eq!(c.mul(&g, lf), psi(&c, g, h2, h3, omega, true, 0));
    assert_eq!(c.mul(&g, lc), psi(&c, g, h2, h3, omega, false, 1));
    assert_ne!(c.mul(&g, (lf + 1) % n), psi(&c, g, h2, h3, omega, true, 0));
    let lines: Vec<u64> = rec["line_powers"]
        .as_array()
        .map(|a| a.iter().map(u).collect())
        .unwrap_or(vec![0]);
    assert!(lines == vec![0] || lines == vec![0, 2]);
    let pts: Vec<Pt4> = rec["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| pt(&v[0], &v[1]))
        .collect();
    assert_eq!(pts.len(), u(&rec["base_points"]) as usize);
    let mut expected = Vec::new();
    for &line in &lines {
        for s in 1..p {
            let x = if line == 0 {
                E4([s, 0, 0, 0])
            } else {
                E4([0, 0, s, 0])
            };
            let rhs = c.rhs(&x);
            if !rhs.is_zero() && c.f.pow(&rhs, (qfield - 1) / 2) == E4::ONE {
                expected.push(x);
            }
        }
    }
    assert_eq!(pts.iter().map(|z| z.x).collect::<Vec<_>>(), expected);
    for &point in &pts {
        assert!(c.on_curve(&point));
        assert_eq!(c.mul(&point, lf), psi(&c, point, h2, h3, omega, true, 0));
        assert_eq!(c.mul(&point, lc), psi(&c, point, h2, h3, omega, false, 1));
    }
    let flat = (
        (0..pts.len()).collect::<Vec<_>>(),
        vec![1u64; pts.len()],
        pts.len(),
    );
    let cubic = columns(&c, &pts, n, omega, lf, lc, h2, h3, false);
    let full = columns(&c, &pts, n, omega, lf, lc, h2, h3, true);
    assert_eq!(cubic.2 * 3, pts.len());
    assert_eq!(full.2 * 12, pts.len());
    let maps = [("flat", flat), ("cubic", cubic), ("twist24", full)];
    let mut selected_index = 0;
    let mut hit_count = 0;
    if let Some(trace) = rec["query_trace"].as_array() {
        assert_eq!(trace.len(), u(&rec["residuals"]) as usize);
        for z in trace {
            let (a, b, hits) = (u(&z[0]), u(&z[1]), u(&z[2]));
            assert!(a < n && b > 0 && b < n);
            if hits > 0 {
                let item = &rec["selected_relations"][selected_index];
                assert_eq!((u(&item["a"]), u(&item["b"])), (a, b));
                selected_index += 1;
                hit_count += 1;
            }
        }
        assert_eq!(
            selected_index,
            rec["selected_relations"].as_array().unwrap().len()
        );
        assert_eq!(hit_count, u(&rec["targets_decomposed"]));
        assert_eq!(
            trace.iter().map(|z| u(&z[2])).sum::<u64>(),
            u(&rec["decompositions"])
        );
        let triples: Vec<(u64, u64, u16)> = trace
            .iter()
            .map(|z| (u(&z[0]), u(&z[1]), u(&z[2]) as u16))
            .collect();
        replay_query_trace(p, b0, g, target, &pts, &triples);
    }
    let mut computed = HashMap::<&str, Vec<Value>>::new();
    let mut seen = HashMap::<&str, HashSet<String>>::new();
    let base_index: HashMap<E4, usize> = pts.iter().enumerate().map(|(i, p)| (p.x, i)).collect();
    for (name, _) in &maps {
        computed.insert(name, Vec::new());
        seen.insert(name, HashSet::new());
    }
    for item in rec["selected_relations"].as_array().unwrap() {
        let (a, b) = (u(&item["a"]), u(&item["b"]));
        let terms = item["terms"].as_array().unwrap();
        assert_eq!(terms.len(), 4);
        let mut distinct = HashSet::new();
        let mut sum = Pt4::INF;
        for z in terms {
            let i = u(&z[0]) as usize;
            let s = z[1].as_i64().unwrap();
            assert!(i < pts.len() && (s == 1 || s == -1) && distinct.insert(i));
            sum = c.add(&sum, &if s == 1 { pts[i] } else { c.neg(&pts[i]) });
        }
        let residual = c.add(&c.mul(&g, a), &c.mul(&target, b));
        assert_eq!(sum, residual, "relation group replay");
        let (tcol, tcoef, tn) = &maps[2].1;
        let original = row(&item["terms"], tcol, tcoef, *tn, a, b, n);
        for (is_frob, multiplier) in [(true, lf), (false, lc)] {
            let transformed: Vec<(usize, i64)> = terms
                .iter()
                .map(|z| {
                    let i = u(&z[0]) as usize;
                    let sign = z[1].as_i64().unwrap();
                    let image = psi(&c, pts[i], h2, h3, omega, is_frob, u64::from(!is_frob));
                    let j = *base_index.get(&image.x).expect("relation orbit closure");
                    let orientation = if pts[j].y == image.y {
                        1
                    } else {
                        assert_eq!(pts[j].y, c.f.neg(&image.y));
                        -1
                    };
                    (j, sign * orientation)
                })
                .collect();
            let after = row(
                &json!(transformed),
                tcol,
                tcoef,
                *tn,
                mm(a, multiplier, n),
                mm(b, multiplier, n),
                n,
            );
            assert_row_scaled(&original, &after, multiplier, n);
        }
        for (name, (cols, coef, nc)) in &maps {
            let rows = computed.get_mut(name).unwrap();
            if rows.len() >= u(&rec[*name]["relations"]) as usize {
                continue;
            }
            let r = row(&item["terms"], cols, coef, *nc, a, b, n);
            let key = r.to_string();
            if seen.get_mut(name).unwrap().insert(key) {
                rows.push(r);
            }
        }
    }
    let mut ranks = BTreeMap::new();
    for (name, (_, _, nc)) in &maps {
        let rows = &computed[name];
        assert_eq!(
            rows,
            &rec[format!("{name}_rows")].as_array().unwrap().to_vec()
        );
        assert_eq!(rows.len(), u(&rec[*name]["relations"]) as usize);
        assert_eq!(*nc, u(&rec[*name]["columns"]) as usize);
        let (rank, answer) = target_from_rows(rows, *nc, n);
        assert_eq!(answer, Some(271));
        assert!(
            rec[*name]["solved"].as_bool().unwrap() && rec[*name]["correct"].as_bool().unwrap()
        );
        assert_eq!(u(&rec[*name]["recovered"]), 271);
        ranks.insert(*name, rank);
    }
    for rr in rec["rho"].as_array().unwrap() {
        assert!(rr["correct"].as_bool().unwrap());
        assert_eq!(u(&rr["recovered"]), 271);
    }
    json!({"p":p,"seed":seed,"lines":lines,"base_points":pts.len(),
        "selected_relations":rec["selected_relations"].as_array().unwrap().len(),
        "ranks":ranks,"rho_solves":rec["rho"].as_array().unwrap().len()})
}

fn main() {
    let path = std::env::args().nth(1).expect("raw NDJSON path");
    let raw = fs::read_to_string(path).expect("raw file");
    let rows: Vec<Value> = raw
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let audited: Vec<Value> = rows.iter().map(audit).collect();
    if rows.iter().all(|v| v.get("line_powers").is_some()) {
        let mut paired = BTreeMap::<(u64, u64), Vec<&Value>>::new();
        for rec in &rows {
            paired
                .entry((u(&rec["p"]), u(&rec["seed"])))
                .or_default()
                .push(rec);
        }
        for ((p, seed), pair) in paired {
            assert_eq!(pair.len(), 2, "paired panel p={p} seed={seed}");
            let (single, mixed) = if pair[0]["line_powers"] == json!([0]) {
                (pair[0], pair[1])
            } else {
                (pair[1], pair[0])
            };
            assert_eq!(single["line_powers"], json!([0]));
            assert_eq!(mixed["line_powers"], json!([0, 2]));
            let n = single["base_coordinates"].as_array().unwrap().len();
            assert_eq!(
                single["base_coordinates"].as_array().unwrap(),
                &mixed["base_coordinates"].as_array().unwrap()[..n]
            );
            let a = single["query_trace"].as_array().unwrap();
            let b = mixed["query_trace"].as_array().unwrap();
            for i in 0..a.len().min(b.len()) {
                assert_eq!(
                    (&a[i][0], &a[i][1]),
                    (&b[i][0], &b[i][1]),
                    "target stream p={p} seed={seed} i={i}"
                );
                if u(&a[i][2]) > 0 {
                    assert!(u(&b[i][2]) > 0, "subset yield p={p} seed={seed} i={i}");
                }
            }
        }
    }
    println!(
        "{}",
        json!({"status":"pass","runs":audited.len(),"rows":audited})
    );
}
