#[allow(dead_code)]
#[path = "../fp.rs"]
mod fp;
use num_bigint::BigUint as Z;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{fs, path::Path};
type A = (Z, Z);
type J = (Z, Z, Z);
fn double(f: &fp::Fp, a: &Z, p: &J) -> J {
    if p.2.is_zero() || p.1.is_zero() {
        return (Z::zero(), Z::one(), Z::zero());
    }
    let aa = f.mul(&p.0, &p.0);
    let b = f.mul(&p.1, &p.1);
    let c = f.mul(&b, &b);
    let mut d = f.mul(&f.add(&p.0, &b), &f.add(&p.0, &b));
    d = f.mul(&f.c(2), &f.sub(&f.sub(&d, &aa), &c));
    let zz = f.mul(&p.2, &p.2);
    let e = f.add(&f.mul(&f.c(3), &aa), &f.mul(a, &f.mul(&zz, &zz)));
    let ff = f.mul(&e, &e);
    let x = f.sub(&ff, &f.mul(&f.c(2), &d));
    let y = f.sub(&f.mul(&e, &f.sub(&d, &x)), &f.mul(&f.c(8), &c));
    let z = f.mul(&f.c(2), &f.mul(&p.1, &p.2));
    (x, y, z)
}
fn mixed(f: &fp::Fp, a: &Z, p: &J, q: &A) -> J {
    if p.2.is_zero() {
        return (q.0.clone(), q.1.clone(), Z::one());
    }
    let zz = f.mul(&p.2, &p.2);
    let u = f.mul(&q.0, &zz);
    let s = f.mul(&q.1, &f.mul(&p.2, &zz));
    let h = f.sub(&u, &p.0);
    let rr = f.mul(&f.c(2), &f.sub(&s, &p.1));
    if h.is_zero() {
        if rr.is_zero() {
            return double(f, a, p);
        }
        return (Z::zero(), Z::one(), Z::zero());
    }
    let hh = f.mul(&h, &h);
    let i = f.mul(&f.c(4), &hh);
    let j = f.mul(&h, &i);
    let v = f.mul(&p.0, &i);
    let x = f.sub(&f.sub(&f.mul(&rr, &rr), &j), &f.mul(&f.c(2), &v));
    let y = f.sub(
        &f.mul(&rr, &f.sub(&v, &x)),
        &f.mul(&f.c(2), &f.mul(&p.1, &j)),
    );
    let z = f.sub(&f.sub(&f.mul(&f.add(&p.2, &h), &f.add(&p.2, &h)), &zz), &hh);
    (x, y, z)
}
fn mul(f: &fp::Fp, a: &Z, n: &Z, q: &A) -> A {
    let mut p = (Z::zero(), Z::one(), Z::zero());
    for k in (0..n.bits()).rev() {
        p = double(f, a, &p);
        if n.bit(k) {
            p = mixed(f, a, &p, q)
        }
    }
    assert!(!p.2.is_zero());
    let iz = f.inv(&p.2);
    let xx = f.mul(&p.0, &f.mul(&iz, &iz));
    let yy = f.mul(&p.1, &f.mul(&f.mul(&iz, &iz), &iz));
    (xx, yy)
}
fn point(v: &Value) -> A {
    (fp::parse(&v[0]), fp::parse(&v[1]))
}
fn matmul(f: &fp::Fp, a: &[Vec<Z>], b: &[Vec<Z>]) -> Vec<Vec<Z>> {
    let n = a.len();
    let mut z = vec![vec![Z::zero(); n]; n];
    for i in 0..n {
        for k in 0..n {
            if a[i][k].is_zero() {
                continue;
            }
            for j in 0..n {
                z[i][j] = f.add(&z[i][j], &f.mul(&a[i][k], &b[k][j]));
            }
        }
    }
    z
}
// Independent of row elimination: determinant from matrix powers and Newton identities.
fn determinant(f: &fp::Fp, a: &[Vec<Z>]) -> Z {
    let n = a.len();
    assert!(Z::from(n) < f.p);
    let mut power = vec![vec![Z::zero(); n]; n];
    for (i, row) in power.iter_mut().enumerate() {
        row[i] = Z::one()
    }
    let mut trace = vec![Z::zero(); n + 1];
    let mut c = vec![Z::one()];
    for k in 1..=n {
        power = matmul(f, &power, a);
        trace[k] = (0..n).fold(Z::zero(), |z, i| f.add(&z, &power[i][i]));
        let s = (1..=k).fold(Z::zero(), |z, i| f.add(&z, &f.mul(&c[k - i], &trace[i])));
        c.push(f.mul(&f.sub(&Z::zero(), &s), &f.inv(&Z::from(k))));
    }
    if n % 2 == 0 {
        c[n].clone()
    } else {
        f.sub(&Z::zero(), &c[n])
    }
}
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_str(&fs::read_to_string(p).unwrap()).unwrap()
}
fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("usage: verify_degree ROUND57_DIRECTORY");
    let root = Path::new(&arg);
    let c = load(root.join("P192-degrees.json"));
    let input = load(root.join("../round52-p192/input.json"));
    let old = load(root.join("../round25-p192/orbit-bridge.json"));
    let f = fp::Fp {
        p: fp::parse(&input["p"]),
    };
    let a = fp::parse(&input["a"]);
    let b = fp::parse(&input["b"]);
    let g = point(&input["G"]);
    assert_eq!(c["G"], input["G"]);
    let gs: Vec<_> = c["G_samples"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    assert_eq!(gs.len(), 48);
    for (i, p) in gs.iter().enumerate() {
        assert_eq!(*p, mul(&f, &a, &Z::from(2 * i + 1), &g));
    }
    let (mut coordinates, mut fits, mut minors) = (48, 0, 0);
    let mut summary = vec![];
    for t in c["targets"].as_array().unwrap() {
        let q = point(&t["Q"]);
        let name = t["name"].as_str().unwrap();
        let known = match name {
            "G" => Some(1),
            "-G" => Some(-1),
            "2G" => Some(2),
            "3G" => Some(3),
            _ => None,
        };
        let expected = match name {
            "public_Q" => point(&input["Q"]),
            "mu_G" => mul(&f, &a, &fp::parse(&old["projected_order5_scalar"]), &g),
            "relation_lambda_G" => mul(&f, &a, &fp::parse(&old["relation_scalar"]), &g),
            "-G" => (g.0.clone(), f.sub(&Z::zero(), &g.1)),
            _ => mul(&f, &a, &Z::from(known.unwrap() as u64), &g),
        };
        assert_eq!(q, expected);
        let qs: Vec<_> = t["Q_samples"]
            .as_array()
            .unwrap()
            .iter()
            .map(point)
            .collect();
        assert_eq!(qs.len(), 48);
        for (i, p) in qs.iter().enumerate() {
            assert_eq!(*p, mul(&f, &a, &Z::from(2 * i + 1), &q));
            assert_eq!(
                f.mul(&p.1, &p.1),
                f.add(
                    &f.add(&f.mul(&f.mul(&p.0, &p.0), &p.0), &f.mul(&a, &p.0)),
                    &b
                )
            );
        }
        coordinates += qs.len();
        let xs: Vec<_> = gs.iter().map(|p| p.0.clone()).collect();
        for row in t["systems"].as_array().unwrap() {
            let d = row["degree_cap"].as_u64().unwrap() as usize;
            let train = row["training"].as_u64().unwrap() as usize;
            let coord = row["coordinate"].as_u64().unwrap() as usize;
            let ys: Vec<_> = qs
                .iter()
                .zip(&gs)
                .map(|(q, g)| {
                    if coord == 0 {
                        q.0.clone()
                    } else {
                        f.mul(&q.1, &g.1)
                    }
                })
                .collect();
            let matrix = f.matrix(&xs[..train], &ys[..train], d);
            if row["status"] == "full_column_rank_no_nonzero_rational_pair" {
                let rows: Vec<_> = row["minor_rows"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_u64().unwrap() as usize)
                    .collect();
                assert_eq!(rows.len(), 2 * d + 2);
                assert_eq!(
                    rows.iter().collect::<std::collections::HashSet<_>>().len(),
                    rows.len()
                );
                let minor: Vec<_> = rows.iter().map(|&i| matrix[i].clone()).collect();
                let det = determinant(&f, &minor);
                assert!(!det.is_zero());
                assert_eq!(det, fp::parse(&row["minor_det"]));
                minors += 1;
            } else {
                assert_eq!(row["status"], "rational_pair_found");
                let num = fp::values(&row["numerator"]);
                let den = fp::values(&row["denominator"]);
                assert!(num.len() <= d + 1 && den.len() <= d + 1);
                for k in 0..train + 8 {
                    assert!(!f.ev(&den, &xs[k]).is_zero());
                    assert_eq!(f.ev(&num, &xs[k]), f.mul(&ys[k], &f.ev(&den, &xs[k])));
                }
                if let Some(k) = known {
                    let (nn, dd) = fp::known(&f, &a, &b, k, coord);
                    assert_eq!(f.pm(&num, &dd), f.pm(&nn, &den));
                }
                fits += 1;
            }
        }
        summary.push(json!({"target":name,"systems":t["systems"].as_array().unwrap().iter().map(|v|json!({"cap":v["degree_cap"],"coordinate":v["coordinate"],"status":v["status"]})).collect::<Vec<_>>()}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"all_checks_pass":true,"Jacobian_point_checks":coordinates,"positive_rational_identities":fits,"nonzero_minor_certificates":minors,"determinant_algorithm":"matrix powers and Newton identities; no row elimination","summary":summary,"standing":"unsigned producer evidence"})).unwrap());
}
