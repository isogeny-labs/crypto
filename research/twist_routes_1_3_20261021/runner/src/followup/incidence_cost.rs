//! Cold anchored two-point decomposition; all construction is charged.
use super::block::{add, constant, group_mitm, membership, mul, pair_ys, variable};
use crate::{f4_fp as f4, toy_field::*, twist_ic_algebra as alg, write_json, Fixture, Result};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    time::{Duration, Instant},
};

fn line(p: u64, values: &[u64]) -> Vec<u64> {
    let mut differences = values.to_vec();
    let mut basis = vec![1];
    let mut factorial = 1;
    let mut result = vec![0; values.len()];
    for j in 0..values.len() {
        if j > 0 {
            factorial = factorial * j as u64 % p;
        }
        let c = differences[0] * inv(factorial, p) % p;
        for (i, b) in basis.iter().enumerate() {
            result[i] = (result[i] + c * b) % p;
        }
        differences = differences
            .windows(2)
            .map(|w| (w[1] + p - w[0]) % p)
            .collect();
        let mut b = vec![0; basis.len() + 1];
        for (i, v) in basis.iter().enumerate() {
            b[i] = (b[i] + v * (p - j as u64) % p) % p;
            b[i + 1] = (b[i + 1] + v) % p;
        }
        basis = b;
    }
    result
}
fn tensor(p: u64, grid: &[Vec<u64>]) -> Vec<Vec<u64>> {
    let h = grid.len();
    let k = grid[0].len();
    let horizontal = grid.iter().map(|r| line(p, r)).collect::<Vec<_>>();
    let mut result = vec![vec![0; k]; h];
    for j in 0..k {
        let column = line(p, &horizontal.iter().map(|r| r[j]).collect::<Vec<_>>());
        for i in 0..h {
            result[i][j] = column[i];
        }
    }
    result
}
fn substitute(q: &f4::Poly, replacements: &[f4::Poly], p: u64) -> f4::Poly {
    let mut result = vec![];
    for (exponents, c) in q {
        let mut term = constant(4, *c);
        for (i, power) in exponents.iter().enumerate() {
            for _ in 0..*power {
                term = mul(&term, &replacements[i], p);
            }
        }
        result = add(&result, &term, p);
    }
    result
}
pub fn worker(input: &Path, out: &Path, budget_ms: u64) -> Result<()> {
    let task: Value = serde_json::from_slice(&fs::read(input)?)?;
    let f: Fixture = serde_json::from_value(task["fixture"].clone())?;
    let e = f.curve();
    let p = f.p;
    let target: P = serde_json::from_value(task["target"].clone())?;
    assert!(e.on(target));
    let tx = target.ok_or("nonidentity target")?.0 .0;
    assert_ne!(target.unwrap().1 .0, 0);
    let h = task["h"].as_u64().ok_or("h")?;
    let k = task["k"].as_u64().ok_or("k")?;
    let lambda = task["lambda"].as_u64().ok_or("lambda")?;
    let arm = task["arm"].as_str().ok_or("arm")?;
    let started = Instant::now();
    let twist = Quad {
        p,
        d: e.d,
        a: e.a * e.d % p * e.d % p,
        b: e.b * e.d % p * e.d % p * e.d % p,
    };
    let square = |v: u64| v == 0 || pow(v, ((p - 1) / 2) as u128, p) == 1;
    let us = (0..k)
        .filter(|u| square(twist.rhs(Q(e.d * u % p, 0)).0))
        .collect::<Vec<_>>();
    let image = |r: u64, u: u64| (r + p - lambda * u % p) % p;
    let base = if arm == "reduced-u" {
        vec![]
    } else {
        (0..h)
            .flat_map(|r| us.iter().map(move |u| image(r, *u)))
            .filter(|x| square(e.rhs(Q(*x, 0)).0))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    };
    let (equations, names) = if arm == "group-mitm" {
        (vec![], vec![])
    } else {
        // The dummy membership polynomials are discarded; only the common F3 is used.
        let template = alg::build(p, e.a, e.b, tx, &[0, 1], 2, "product");
        let f3 = template.equations.last().unwrap();
        if arm == "direct-product" {
            (
                vec![
                    membership(2, 0, &base, p),
                    membership(2, 1, &base, p),
                    f3.clone(),
                ],
                vec!["x1", "x2"],
            )
        } else if arm == "full-mask" || arm == "reduced-u" {
            let mut equations = vec![];
            let mask = if arm == "full-mask" {
                Some(tensor(
                    p,
                    &(0..h)
                        .map(|r| {
                            (0..k)
                                .map(|u| (us.contains(&u) && base.contains(&image(r, u))) as u64)
                                .collect()
                        })
                        .collect::<Vec<_>>(),
                ))
            } else {
                None
            };
            for pair in 0..2 {
                equations.push(membership(4, pair * 2, &(0..h).collect::<Vec<_>>(), p));
                let bounds = if arm == "full-mask" {
                    (0..k).collect::<Vec<_>>()
                } else {
                    us.clone()
                };
                equations.push(membership(4, pair * 2 + 1, &bounds, p));
                if let Some(mask) = &mask {
                    let mut q = vec![];
                    for (i, row) in mask.iter().enumerate() {
                        for (j, c) in row.iter().enumerate() {
                            if *c != 0 {
                                let mut powers = vec![0; 4];
                                powers[pair * 2] = i as u32;
                                powers[pair * 2 + 1] = j as u32;
                                q.push((powers, *c));
                            }
                        }
                    }
                    equations.push(add(&q, &constant(4, p - 1), p));
                }
            }
            let xs = (0..2)
                .map(|i| {
                    add(
                        &variable(4, 2 * i),
                        &mul(
                            &constant(4, (p - lambda % p) % p),
                            &variable(4, 2 * i + 1),
                            p,
                        ),
                        p,
                    )
                })
                .collect::<Vec<_>>();
            equations.push(substitute(f3, &xs, p));
            (equations, vec!["r1", "u1", "r2", "u2"])
        } else {
            return Err("unknown incidence arm".into());
        }
    };
    let mut tuples = BTreeMap::<Vec<u64>, Vec<u64>>::new();
    let mut solutions = None;
    let mut timed_out = false;
    if arm == "group-mitm" {
        let points = base
            .iter()
            .map(|x| {
                (
                    *x,
                    Some((Q(*x, 0), Q(sqrt(e.rhs(Q(*x, 0)).0, p).unwrap(), 0))),
                )
            })
            .collect::<BTreeMap<_, _>>();
        for (t, mask) in group_mitm(e, &base, &points, target, 2) {
            let ys = t
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let y = points[x].unwrap().1 .0;
                    if mask >> i & 1 == 0 {
                        y
                    } else {
                        (p - y) % p
                    }
                })
                .collect::<Vec<_>>();
            tuples.insert(t, ys);
        }
    } else {
        let options =
            f4::F4Options::new(f4::Ordering::Grevlex, 128).with_budget(Duration::from_millis(
                budget_ms
                    .saturating_sub(started.elapsed().as_millis() as u64)
                    .max(1),
            ));
        let r = f4::solve(&equations, names.len(), p, &options);
        timed_out = r.timed_out;
        solutions = match r.verdict {
            f4::Verdict::Solutions(s) => Some(s),
            f4::Verdict::Inconsistent => Some(vec![]),
            f4::Verdict::Undetermined => None,
        };
        if let Some(ss) = &solutions {
            for s in ss {
                assert!(equations.iter().all(|q| f4::eval(q, s, p) == 0));
                let mut t = if arm == "direct-product" {
                    s.clone()
                } else {
                    vec![image(s[0], s[1]), image(s[2], s[3])]
                };
                t.sort();
                let ys = pair_ys(e, t[0], t[1], target).ok_or("invalid F3 lift")?;
                tuples.insert(t, ys.to_vec());
            }
        }
    }
    let complete = arm == "group-mitm" || (solutions.is_some() && !timed_out);
    write_json(
        out,
        &json!({"schema":"twist.incidence-cost-worker/v1","case":task["case"],"arm":arm,"target":target,"U":us,"names":names,"equations":equations,"solutions":solutions,"timed_out":timed_out,"complete_projection":complete,"canonical_pairs":tuples.keys().collect::<Vec<_>>(),"explicit_y_witnesses":tuples.iter().map(|(x,y)|json!({"x":x,"y":y})).collect::<Vec<_>>(),"all_actual_construction_and_roots_and_lifting_included":true,"complete_IC_cost":false}),
    )
}
