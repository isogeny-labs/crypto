#[allow(dead_code, unused_imports)]
mod common;
use common::*;
use std::collections::HashMap;
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn neg(f: &fp::Fp, p: &A) -> A {
    (p.0.clone(), f.sub(&Z::zero(), &p.1))
}
fn on(f: &fp::Fp, a: &Z, b: &Z, p: &A) -> bool {
    f.mul(&p.1, &p.1) == curve(f, a, b, &p.0)
}
fn curve(f: &fp::Fp, a: &Z, b: &Z, x: &Z) -> Z {
    f.add(&f.add(&f.mul(&f.mul(x, x), x), &f.mul(a, x)), b)
}
fn affine(f: &fp::Fp, a: &Z, p: Option<A>, q: &A) -> Option<A> {
    let Some(p) = p else { return Some(q.clone()) };
    if p.0 == q.0 && f.add(&p.1, &q.1).is_zero() {
        return None;
    }
    let slope = if p == *q {
        f.mul(
            &f.add(&f.mul(&f.c(3), &f.mul(&p.0, &p.0)), a),
            &f.inv(&f.mul(&f.c(2), &p.1)),
        )
    } else {
        f.mul(&f.sub(&q.1, &p.1), &f.inv(&f.sub(&q.0, &p.0)))
    };
    let x = f.sub(&f.sub(&f.mul(&slope, &slope), &p.0), &q.0);
    let y = f.sub(&f.mul(&slope, &f.sub(&p.0, &x)), &p.1);
    Some((x, y))
}
fn sequence(f: &fp::Fp, a: &Z, q: &A, n: usize, jac: bool) -> Vec<A> {
    if !jac {
        let mut cur = None;
        return (0..n)
            .map(|_| {
                cur = affine(f, a, cur.clone(), q);
                cur.clone().unwrap()
            })
            .collect();
    }
    let mut cur = (Z::zero(), Z::one(), Z::zero());
    let mut js = vec![];
    let mut prefix = vec![Z::one()];
    for _ in 0..n {
        cur = mixed(f, a, &cur, q);
        assert!(!cur.2.is_zero());
        prefix.push(f.mul(prefix.last().unwrap(), &cur.2));
        js.push(cur.clone())
    }
    let mut inverse = f.inv(prefix.last().unwrap());
    let mut out = vec![(Z::zero(), Z::zero()); n];
    for i in (0..n).rev() {
        let iz = f.mul(&inverse, &prefix[i]);
        inverse = f.mul(&inverse, &js[i].2);
        let iz2 = f.mul(&iz, &iz);
        out[i] = (f.mul(&js[i].0, &iz2), f.mul(&js[i].1, &f.mul(&iz2, &iz)))
    }
    out
}
fn carrier(f: &fp::Fp, a: &Z, b: &Z, index: usize, p: &A, q: &A, wrong: bool) -> Z {
    let (x, y) = (&p.0, &q.0);
    let mut z = f.mul(&p.1, &q.1);
    if wrong {
        z = f.sub(&Z::zero(), &z)
    }
    let fx = curve(f, a, b, x);
    match index {
        0 => {
            assert_eq!(x, y);
            f.sub(&z, &fx)
        }
        1 => {
            assert_eq!(x, y);
            f.add(&z, &fx)
        }
        2 => f.add(
            &f.sub(
                &f.mul(&f.c(2), &z),
                &f.mul(&f.add(&f.mul(&f.c(3), &f.mul(x, x)), a), &f.sub(x, y)),
            ),
            &f.mul(&f.c(2), &fx),
        ),
        3 => {
            let xx = f.mul(x, x);
            let n2 = f.add(
                &f.sub(
                    &f.sub(&f.mul(&xx, &xx), &f.mul(&f.c(2), &f.mul(a, &xx))),
                    &f.mul(&f.c(8), &f.mul(b, x)),
                ),
                &f.mul(a, a),
            );
            f.sub(
                &f.mul(
                    &f.mul(&f.sub(y, x), &f.sub(y, x)),
                    &f.add(&n2, &f.mul(&f.c(4), &f.mul(&fx, &f.add(y, x)))),
                ),
                &f.mul(
                    &f.c(4),
                    &f.mul(
                        &fx,
                        &f.add(&f.add(&curve(f, a, b, y), &fx), &f.mul(&f.c(2), &z)),
                    ),
                ),
            )
        }
        _ => panic!("only four symbolic controls"),
    }
}
fn conditions(r: &Z, d: &Z, absd: &Z) -> Value {
    let one = Z::one();
    let rm = r - &one;
    let four = Z::from(4u32);
    let eight = Z::from(8u32);
    let sixteen = Z::from(16u32);
    let dd = d * d;
    let divisor = &four * d;
    let s = (&rm + &divisor - &one) / &divisor;
    let lhs = &s * (&s - &one);
    let rhs = &eight * &dd * &rm;
    assert!(lhs > rhs);
    assert!(&four * &dd < *r);
    assert!(&sixteen * &dd < *absd);
    json!({"d":d.to_string(),"B":(&four*d/Z::from(2u32)).to_string(),"s":s.to_string(),"incidence_lhs":lhs.to_string(),"incidence_rhs":rhs.to_string(),"4d2":(&four*&dd).to_string(),"16d2":(&sixteen*&dd).to_string(),"all_strict_conditions":true})
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = Path::new(
        args.get(1)
            .expect("usage: signed-carrier ROUND59 [--jacobian] [--check FILE] [--wrong-sign]"),
    );
    let jac = args.iter().any(|x| x == "--jacobian");
    let wrong = args.iter().any(|x| x == "--wrong-sign");
    let input = load(root.join("../round52-p192/input.json"));
    let saved = load(root.join("../round57-p192/P192-degrees.json"));
    let labels = load(root.join("../round25-p192/orbit-bridge.json"));
    let f = fp::Fp {
        p: fp::parse(&input["p"]),
    };
    let a = fp::parse(&input["a"]);
    let b = fp::parse(&input["b"]);
    let r = fp::parse(&input["r"]);
    let g = point(&input["G"]);
    let mu = fp::parse(&labels["projected_order5_scalar"]);
    let lambda = fp::parse(&labels["relation_scalar"]);
    assert!(on(&f, &a, &b, &g));
    assert_eq!(mu.modpow(&Z::from(5u32), &r), Z::one());
    assert_ne!(mu, Z::one());
    let gs: Vec<_> = saved["G_samples"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    assert_eq!(gs.len(), 48);
    for (j, p) in gs.iter().enumerate() {
        assert_eq!(*p, mul(&f, &a, &Z::from(2 * j + 1), &g))
    }
    let mut positive = vec![];
    let mut target_rows = vec![];
    let bound = 4096usize;
    let base = sequence(&f, &a, &g, bound, jac);
    let mut table = HashMap::new();
    for (j, p) in base.iter().enumerate() {
        assert!(on(&f, &a, &b, p));
        assert!(table.insert(p.clone(), (j + 1) as i64).is_none());
        assert!(table.insert(neg(&f, p), -((j + 1) as i64)).is_none())
    }
    for (ti, target) in saved["targets"].as_array().unwrap().iter().enumerate() {
        let q = point(&target["Q"]);
        assert!(on(&f, &a, &b, &q));
        match ti {
            0 => assert_eq!(q, g),
            1 => assert_eq!(q, neg(&f, &g)),
            2 | 3 => assert_eq!(q, mul(&f, &a, &Z::from(ti), &g)),
            4 => assert_eq!(q, point(&input["Q"])),
            5 => assert_eq!(q, mul(&f, &a, &mu, &g)),
            6 => assert_eq!(q, mul(&f, &a, &lambda, &g)),
            _ => panic!(),
        }
        if ti < 4 {
            let mut guard_checks = 0;
            let mut wrong_branch = 0;
            for (j, p) in gs.iter().enumerate() {
                let qp = point(&target["Q_samples"][j]);
                assert_eq!(qp, mul(&f, &a, &Z::from(2 * j + 1), &q));
                assert!(carrier(&f, &a, &b, ti, p, &qp, wrong).is_zero());
                assert!(!carrier(&f, &a, &b, ti, p, &qp, true).is_zero());
                if ti >= 2 {
                    let guard = f.sub(&qp.0, &p.0);
                    assert!(!guard.is_zero());
                    assert_eq!(f.mul(&guard, &f.inv(&guard)), Z::one());
                    guard_checks += 1;
                    let false_q = neg(&f, p);
                    assert!(carrier(&f, &a, &b, ti, p, &false_q, false).is_zero());
                    assert_eq!(false_q.0, p.0);
                    wrong_branch += 1
                }
            }
            positive.push(json!({"target":target["name"],"nodes":48,"weighted_degree":if ti<2 {3}else if ti==2 {3}else{6},"relative_sign_rejections":48,"saturation_guard_checks":guard_checks,"extraneous_minus_P_branch_checks":wrong_branch}));
        }
        let qs = sequence(&f, &a, &q, bound, jac);
        let mut hits = vec![];
        let mut compatible = 0;
        for (j, p) in qs.iter().enumerate() {
            assert!(on(&f, &a, &b, p));
            if let Some(&u) = table.get(p) {
                let aa = (j + 1) as i64;
                let usq = u * u;
                if usq % aa == 0 && usq / aa <= bound as i64 {
                    compatible += 1
                }
                hits.push([aa, u])
            }
        }
        if ti >= 4 {
            assert!(hits.is_empty())
        }
        target_rows.push(json!({"name":target["name"],"Q":target["Q"],"matches":hits,"match_count":hits.len(),"necessary_degree_compatible_pairs":compatible,"search_a_max":bound,"search_abs_u_max":bound,"degree_2048_global_carrier_excluded":ti>=4}));
    }
    let t = &f.p + Z::one() - &r;
    let absd = Z::from(4u32) * &f.p - &t * &t;
    let small = conditions(&r, &Z::from(2048u32), &absd);
    let large_d = Z::one() << 44usize;
    let large_b = &large_d * Z::from(2u32);
    let large = conditions(&r, &large_d, &absd);
    let cyclo_bound = Z::from(5u32) * large_b.pow(4);
    assert!(cyclo_bound < r);
    let l = &r - &lambda;
    assert!(l > large_b);
    let unwrap = &large_b * &l + &large_b;
    assert!(unwrap < r);
    let out = json!({"all_checks_pass":true,"p":f.p.to_string(),"r":r.to_string(),"trace":t.to_string(),"abs_D":absd.to_string(),"positive_signed_carriers":positive,"bounded_point_search":{"B":bound,"table_signed_points":table.len(),"point_advances":8*bound,"targets":target_rows},"degree_conditions":{"small":small,"large":large},"labeled_scalar_certificates":{"mu":mu.to_string(),"mu_order":5,"five_B4":cyclo_bound.to_string(),"lambda":lambda.to_string(),"L":l.to_string(),"B_L_plus_B":unwrap.to_string(),"B":large_b.to_string(),"both_no_bounded_relation":true,"degree_2pow44_global_carrier_excluded_for":["mu_G","relation_lambda_G"]},"scope":"unsigned native exact identities/searches; written global carrier theorem required; no arithmetic-circuit or baby-subset bound"});
    if let Some(k) = args.iter().position(|x| x == "--check") {
        assert!(out == load(&args[k + 1]), "certificate mismatch");
        eprintln!("all exact outputs match independent point-generation replay")
    }
    println!("{}", serde_json::to_string(&out).unwrap());
}
