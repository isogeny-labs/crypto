#[allow(dead_code, unused_imports)]
#[path = "../big.rs"]
mod big;
#[allow(dead_code)]
#[path = "../fp.rs"]
mod fp;
use num_bigint::BigUint as Z;
use num_traits::{One, Zero};
use serde_json::json;
use std::path::Path;
fn scalar(e: &big::E, n: &Z, p: &big::Pt) -> big::Pt {
    let mut q = None;
    for i in (0..n.bits()).rev() {
        q = e.add(&q, &q);
        if n.bit(i) {
            q = e.add(&q, p)
        }
    }
    q
}
fn affine(p: &big::Pt) -> Vec<String> {
    let (x, y) = p.as_ref().unwrap();
    assert!(x.1.is_zero() && y.1.is_zero());
    vec![x.0.to_string(), y.0.to_string()]
}
fn grid(e: &big::E, p: &big::Pt) -> Vec<big::Pt> {
    let two = e.add(p, p);
    let mut q = p.clone();
    let mut out = vec![];
    for _ in 0..48 {
        assert!(e.on(&q));
        out.push(q.clone());
        q = e.add(&q, &two);
    }
    out
}
fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("usage: degree ROUND57_DIRECTORY");
    let root = Path::new(&arg);
    let input = big::load(root.join("../round52-p192/input.json"));
    let old = big::load(root.join("../round25-p192/orbit-bridge.json"));
    let e = big::E {
        p: big::z(&input["p"]),
        a: big::z(&input["a"]),
        b: big::z(&input["b"]),
    };
    let f = fp::Fp { p: e.p.clone() };
    let g = big::point(&input["G"]);
    let gg = grid(&e, &g);
    let r = big::z(&input["r"]);
    let mu = fp::parse(&old["projected_order5_scalar"]);
    let lam = fp::parse(&old["relation_scalar"]);
    assert_eq!(mu.modpow(&Z::from(5u32), &r), Z::one());
    assert_ne!(mu, Z::one());
    let cases = vec![
        ("G", Some(1), g.clone()),
        ("-G", Some(-1), e.neg(&g)),
        ("2G", Some(2), scalar(&e, &Z::from(2u32), &g)),
        ("3G", Some(3), scalar(&e, &Z::from(3u32), &g)),
        ("public_Q", None, big::point(&input["Q"])),
        ("mu_G", None, scalar(&e, &mu, &g)),
        ("relation_lambda_G", None, scalar(&e, &lam, &g)),
    ];
    let mut targets = vec![];
    for (name, known, q) in cases {
        let qs = grid(&e, &q);
        let xs: Vec<_> = gg
            .iter()
            .map(|p| p.as_ref().unwrap().0 .0.clone())
            .collect();
        let av: Vec<_> = qs
            .iter()
            .map(|p| p.as_ref().unwrap().0 .0.clone())
            .collect();
        let bv: Vec<_> = gg
            .iter()
            .zip(&qs)
            .map(|(u, v)| f.mul(&u.as_ref().unwrap().1 .0, &v.as_ref().unwrap().1 .0))
            .collect();
        let mut systems = vec![];
        for (d, train) in [(8usize, 24usize), (16usize, 40usize)] {
            for (coord, ys) in [&av, &bv].into_iter().enumerate() {
                let matrix = f.matrix(&xs[..train], &ys[..train], d);
                let (rr, pivots, rows, det) = f.rref(matrix);
                let columns = 2 * d + 2;
                let mut row = json!({"degree_cap":d,"training":train,"holdout":8,"coordinate":coord,"rank":pivots.len(),"columns":columns});
                if pivots.len() == columns {
                    row["status"] = json!("full_column_rank_no_nonzero_rational_pair");
                    row["minor_rows"] = json!(&rows[..columns]);
                    row["minor_det"] = json!(det.to_string());
                    if let Some(k) = known {
                        assert!(k == 3 && d == 8);
                    }
                } else {
                    let free: Vec<_> = (0..columns).filter(|j| !pivots.contains(j)).collect();
                    let mut answer = None;
                    for col in free {
                        let mut z = vec![Z::zero(); columns];
                        z[col] = Z::one();
                        for (i, &p) in pivots.iter().enumerate() {
                            z[p] = f.sub(&Z::zero(), &rr[i][col]);
                        }
                        let num = f.trim(z[..d + 1].to_vec());
                        let den = f.trim(z[d + 1..].to_vec());
                        if xs[..train].iter().all(|x| !f.ev(&den, x).is_zero()) {
                            answer = Some((num, den));
                            break;
                        }
                    }
                    let (num, den) = answer.expect(
                        "nonzero denominator candidate required, otherwise report unresolved",
                    );
                    for k in 0..train {
                        assert_eq!(f.ev(&num, &xs[k]), f.mul(&ys[k], &f.ev(&den, &xs[k])))
                    }
                    let holdout = (train..train + 8).all(|k| {
                        !f.ev(&den, &xs[k]).is_zero()
                            && f.ev(&num, &xs[k]) == f.mul(&ys[k], &f.ev(&den, &xs[k]))
                    });
                    row["status"] = json!("rational_pair_found");
                    row["numerator"] = json!(fp::strings(&num));
                    row["denominator"] = json!(fp::strings(&den));
                    row["holdout_pass"] = json!(holdout);
                    if let Some(k) = known {
                        let (nn, dd) = fp::known(&f, &e.a, &e.b, k, coord);
                        assert_eq!(f.pm(&num, &dd), f.pm(&nn, &den));
                        assert!(holdout);
                        row["known_global_map_identity"] = json!(true);
                    }
                }
                systems.push(row);
            }
        }
        targets.push(json!({"name":name,"Q":affine(&q),"Q_samples":qs.iter().map(affine).collect::<Vec<_>>(),"systems":systems}));
    }
    println!("{}",serde_json::to_string(&json!({"all_checks_pass":true,"p":e.p.to_string(),"a":e.a.to_string(),"b":e.b.to_string(),"r":r.to_string(),"G":input["G"],"G_samples":gg.iter().map(affine).collect::<Vec<_>>(),"targets":targets,"scope":"bounded rational interpolation on frozen actual P192 sample indices; unknown targets may have other encodings"})).unwrap());
}
