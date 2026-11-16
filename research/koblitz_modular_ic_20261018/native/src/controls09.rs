//! Bounded deck-twist controls and restrictions of genuinely coupled functions.
use crate::{
    forward07::{ap, ca, line_map},
    large_field::*,
    reverse06::{dp, poly, AP, R},
    round02, round04,
};
use serde_json::json;
use std::path::Path;
type M = Vec<Vec<i128>>;
fn id(n: usize) -> M {
    (0..n)
        .map(|i| (0..n).map(|j| i128::from(i == j)).collect())
        .collect()
}
fn mm(a: &M, b: &M) -> M {
    let n = a.len();
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    (0..n).fold(0i128, |z, k| {
                        z.checked_add(a[i][k].checked_mul(b[k][j]).unwrap())
                            .unwrap()
                    })
                })
                .collect()
        })
        .collect()
}
fn mp(a: &M, n: usize) -> M {
    (0..n).fold(id(a.len()), |z, _| mm(&z, a))
}
fn tensor(a: &M, b: &M) -> M {
    let m = a.len();
    let n = b.len();
    (0..m * n)
        .map(|i| {
            (0..m * n)
                .map(|j| a[i / n][j / n] * b[i % n][j % n])
                .collect()
        })
        .collect()
}
fn tr(a: &M) -> i128 {
    (0..a.len()).map(|i| a[i][i]).sum()
}
fn characteristic_from_traces(traces: &[i128]) -> Vec<i128> {
    let mut c = vec![1i128];
    for n in 1..=traces.len() {
        let s: i128 = (1..=n).map(|i| c[n - i] * traces[i - 1]).sum();
        assert_eq!(s % n as i128, 0);
        c.push(-s / n as i128);
    }
    c.reverse();
    c
}
fn characteristic(a: &M) -> Vec<i128> {
    characteristic_from_traces(&(1..=a.len()).map(|n| tr(&mp(a, n))).collect::<Vec<_>>())
}
fn poly_mul(a: &[i128], b: &[i128]) -> Vec<i128> {
    let mut z = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            z[i + j] += x * y;
        }
    }
    z
}
pub fn twists(dir: &Path) {
    let s = vec![
        vec![-1, -1, -1, -1],
        vec![1, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 1, 0],
    ];
    let r = vec![
        vec![1, 0, 0, 0],
        vec![-1, -1, -1, -1],
        vec![0, 0, 0, 1],
        vec![0, 0, 1, 0],
    ];
    assert_eq!(mp(&s, 5), id(4));
    assert_eq!(mp(&r, 2), id(4));
    assert_eq!(mm(&mm(&r, &s), &r), mp(&s, 4));
    let g = mp(&s, 2);
    assert_eq!(mm(&mm(&s, &r), &mp(&s, 4)), mm(&g, &r));
    let mut records = vec![];
    let mut simultaneous = 0;
    for t in [-1i128, 1] {
        let e = vec![vec![0, -2], vec![1, t]];
        let desired = characteristic(&tensor(&s, &e));
        let ep = vec![2, -t, 1];
        let mut res = poly_mul(&desired, &ep);
        let mut norm = vec![0; 11];
        norm[0] = 32;
        norm[5] = -11 * t;
        norm[10] = 1;
        assert_eq!(res, norm);
        res.clear();
        for u in 0..5 {
            let k = (2 * u + 1) % 5;
            let f = tensor(&mp(&s, k), &e);
            let cp = characteristic(&f);
            let matches = cp == desired;
            assert_eq!(matches, k != 0);
            for j in 0..5 {
                let w = mm(&mp(&g, j), &r);
                let w8 = tensor(&w, &id(2));
                let stable = mm(&f, &w8) == mm(&w8, &f);
                assert_eq!(stable, k == 0);
                if stable && matches {
                    simultaneous += 1;
                }
                let fixed = if stable {
                    let traces: Vec<_> = (1..=4)
                        .map(|n| {
                            let a = mp(&f, n);
                            let s = tr(&a) + tr(&mm(&w8, &a));
                            assert_eq!(s % 2, 0);
                            s / 2
                        })
                        .collect();
                    let c = characteristic_from_traces(&traces);
                    assert_eq!(c, poly_mul(&ep, &ep));
                    Some(c)
                } else {
                    None
                };
                records.push(json!({"trace":t,"twist_u":u,"reflection_j":j,"Frobenius_rotation":k,"reflection_F2_stable":stable,"known_component_Weil_polynomial":cp,"desired_tracezero_Weil_polynomial":desired,"retains_known_component":matches,"fixed_component_polynomial_if_stable":fixed}));
            }
        }
    }
    assert_eq!(records.len(), 50);
    assert_eq!(simultaneous, 0);
    round02::save(
        dir,
        "l7-diagonal-twist-reflection-controls.json",
        &json!({"schema":"koblitz.round09/l7-v1","augmentation_rotation":s,"reflection":r,"deck_diagonal":g,"records":records,"simultaneous_stable_and_retained":simultaneous,"carrier_genus":2841,"reflection_quotient_genus_upper_bound":1421,"scope":"all25 diagonal-twist/reflection pairs,both signs,on the known tracezero module only;not a full Jacobian-factor exclusion or universal low-genus obstruction"}),
    );
    println!("L7:all50 signed controls;no stable reflection retains the known tracezero component");
}
fn coordinates(p: &AP) -> (crate::reverse06::A, crate::reverse06::A) {
    match p {
        AP::Pt(x, y) => (x.clone(), y.clone()),
        _ => panic!("ordinary support maps must be affine"),
    }
}
pub fn coupled(dir: &Path) {
    let f = F::new(120, 27);
    let cache = round04::read(&dir.join("n4-independent-seed-targets.json"));
    let m8 = round04::read(&dir.join("m8-full-map-principal-row.json"));
    let m11 = round04::read(&dir.join("m11-fixed-infinity-row.json"));
    let mut columns = vec![];
    for (i, c) in m8["columns"].as_array().unwrap().iter().enumerate() {
        columns.push((588 + i, 0, c));
    }
    for (i, c) in m11["columns_from648"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        columns.push((648 + i, c["line"].as_u64().unwrap_or(1), c));
    }
    let mut results:Vec<_>=(0..8).map(|i|json!({"function":(["x0+xinf","x0+xinf+1","y0+yinf","y0+yinf+1","x0*xinf","x0*xinf+1","x0*yinf+xinf*y0","x0*yinf+xinf*y0+1"][i]),"zero_columns":[],"zero_degree":0,"nonzero_image_zero_columns":[0,0]})).collect();
    let mut evaluations = 0;
    for (index, line, col) in &columns {
        let negative = col["seed"] == "negative";
        let name = if negative { "negative" } else { "positive" };
        let members = col
            .get("members_over_F32")
            .unwrap_or(&col["members_over_F1024"])
            .as_array()
            .unwrap();
        let mut first = None;
        for member in members {
            let ix = member.as_u64().unwrap() as usize;
            let seed = &cache[name][ix];
            let modulus = if negative {
                poly(&f, &seed["modulus"])
            } else {
                vec![f.z(), f.o()]
            };
            let r = R {
                f: &f,
                p: modulus,
                a: f.o(),
                b: f.o(),
                fx: vec![],
            };
            let inv = (1..11).find(|i| i * line % 11 == 1).unwrap_or(1);
            let perm = |i| {
                if *line == 0 {
                    i
                } else if *line == 6 {
                    line_map(0, 10, 1, 0, i)
                } else {
                    line_map(0, 11 - line, inv, 1, i)
                }
            };
            let point = |i| {
                if negative {
                    ap(&r, &seed["targets"][i]["x"], &seed["targets"][i]["y"])
                } else {
                    r.cp(&dp(&f, &seed["target_traces"][i]))
                }
            };
            let (x, y) = coordinates(&point(perm(0)));
            let (u, v) = coordinates(&point(perm(11)));
            let fs = [
                r.add(&x, &u),
                r.add(&y, &v),
                r.mul(&x, &u),
                r.add(&r.mul(&x, &v), &r.mul(&u, &y)),
            ];
            let mut zeros = vec![];
            for fun in &fs {
                for one in [false, true] {
                    let value = if one {
                        r.add(fun, &ca(&r, &vec![f.o()]))
                    } else {
                        fun.clone()
                    };
                    zeros.push(value == r.z());
                }
            }
            if let Some(old) = &first {
                assert_eq!(*old, zeros, "coupled zero status varies on closed orbit")
            } else {
                first = Some(zeros)
            }
            evaluations += 8;
        }
        for (i, zero) in first.unwrap().iter().enumerate() {
            if !zero {
                continue;
            }
            results[i]["zero_columns"]
                .as_array_mut()
                .unwrap()
                .push(json!(index));
            let d = results[i]["zero_degree"].as_u64().unwrap() + col["degree"].as_u64().unwrap();
            results[i]["zero_degree"] = json!(d);
            for sign in 0..2 {
                if col["images"][sign]["nonzero"] == true {
                    let n = results[i]["nonzero_image_zero_columns"][sign]
                        .as_u64()
                        .unwrap()
                        + 1;
                    results[i]["nonzero_image_zero_columns"][sign] = json!(n);
                }
            }
        }
    }
    assert_eq!(columns.len(), 720);
    round02::save(
        dir,
        "m12-coupled-function-restrictions.json",
        &json!({"schema":"koblitz.round09/m12-v1","known_closed_columns_tested":columns.len(),"component_function_evaluations":evaluations,"functions":results,"complete_principal_divisors":false,"relation_rank_promoted":false,"scope":"exact coupled-function restrictions on known720 norm columns only;outside zeros,multiplicities and poles remain uncomputed"}),
    );
    println!("M12:8 coupled functions on720 known columns;no rank promotion");
}
