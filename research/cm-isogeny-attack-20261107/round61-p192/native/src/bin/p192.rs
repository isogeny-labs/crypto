#[allow(dead_code, unused_imports)]
#[path = "../common.rs"]
mod base;
use base::*;
type Mat = [Z; 4];
type Poly = [Z; 3];
fn id() -> Mat {
    [Z::one(), Z::zero(), Z::zero(), Z::one()]
}
fn mn(f: &fp::Fp, mut m: Mat) -> Mat {
    let inv = f.inv(m.iter().find(|x| !x.is_zero()).unwrap());
    for x in &mut m {
        *x = f.mul(x, &inv)
    }
    assert!(!f.sub(&f.mul(&m[0], &m[3]), &f.mul(&m[1], &m[2])).is_zero());
    m
}
fn mm(f: &fp::Fp, a: &Mat, b: &Mat) -> Mat {
    mn(
        f,
        [
            f.add(&f.mul(&a[0], &b[0]), &f.mul(&a[1], &b[2])),
            f.add(&f.mul(&a[0], &b[1]), &f.mul(&a[1], &b[3])),
            f.add(&f.mul(&a[2], &b[0]), &f.mul(&a[3], &b[2])),
            f.add(&f.mul(&a[2], &b[1]), &f.mul(&a[3], &b[3])),
        ],
    )
}
fn scale(f: &fp::Fp, a: &Mat, z: &Z) -> Mat {
    mn(f, std::array::from_fn(|i| f.mul(&a[i], z)))
}
fn eval(f: &fp::Fp, m: &Mat, z: &Z) -> Option<Z> {
    let n = f.add(&f.mul(&m[0], z), &m[1]);
    let d = f.add(&f.mul(&m[2], z), &m[3]);
    if d.is_zero() {
        assert!(!n.is_zero());
        None
    } else {
        Some(f.mul(&n, &f.inv(&d)))
    }
}
fn collision(f: &fp::Fp, a: &Mat, b: &Mat) -> Poly {
    [
        f.sub(&f.mul(&a[0], &b[2]), &f.mul(&b[0], &a[2])),
        f.sub(
            &f.add(&f.mul(&a[0], &b[3]), &f.mul(&a[1], &b[2])),
            &f.add(&f.mul(&b[0], &a[3]), &f.mul(&b[1], &a[2])),
        ),
        f.sub(&f.mul(&a[1], &b[3]), &f.mul(&b[1], &a[3])),
    ]
}
fn sqrt_mod(f: &fp::Fp, n: &Z) -> Option<Z> {
    if n.is_zero() {
        return Some(Z::zero());
    }
    let one = Z::one();
    let rm = &f.p - &one;
    if n.modpow(&(&rm >> 1usize), &f.p) != one {
        return None;
    }
    let mut q = rm.clone();
    let mut s = 0usize;
    while !q.bit(0) {
        q >>= 1usize;
        s += 1
    }
    let z = (2u32..128)
        .map(Z::from)
        .find(|z| z.modpow(&(&rm >> 1usize), &f.p) == rm)
        .expect("frozen nonresidue-search cap");
    let mut c = z.modpow(&q, &f.p);
    let mut x = n.modpow(&((&q + &one) >> 1usize), &f.p);
    let mut t = n.modpow(&q, &f.p);
    let mut m = s;
    while t != one {
        let mut u = t.clone();
        let mut i = 0;
        while u != one && i < m {
            u = f.mul(&u, &u);
            i += 1
        }
        assert!(i < m);
        let b = c.modpow(&(Z::one() << (m - i - 1)), &f.p);
        x = f.mul(&x, &b);
        let bb = f.mul(&b, &b);
        t = f.mul(&t, &bb);
        c = bb;
        m = i
    }
    assert_eq!(f.mul(&x, &x), *n);
    Some(x)
}
fn roots(f: &fp::Fp, p: &Poly) -> Vec<Z> {
    let [a, b, c] = p;
    if a.is_zero() {
        return if b.is_zero() {
            vec![]
        } else {
            vec![f.mul(&f.sub(&Z::zero(), c), &f.inv(b))]
        };
    }
    let disc = f.sub(&f.mul(b, b), &f.mul(&f.c(4), &f.mul(a, c)));
    let Some(s) = sqrt_mod(f, &disc) else {
        return vec![];
    };
    let inv = f.inv(&f.mul(&f.c(2), a));
    let nb = f.sub(&Z::zero(), b);
    let mut v = vec![f.mul(&f.add(&nb, &s), &inv), f.mul(&f.sub(&nb, &s), &inv)];
    v.sort();
    v.dedup();
    for z in &v {
        assert!(f.add(&f.mul(&f.add(&f.mul(a, z), b), z), c).is_zero())
    }
    v
}
fn recover(f: &fp::Fp, fr: &fp::Fp, a: &Z, g: &A, q: &A, p: &Poly) -> Option<Z> {
    let v: Vec<_> = roots(fr, p)
        .into_iter()
        .filter(|k| !k.is_zero() && mul(f, a, k, g) == *q)
        .collect();
    assert!(v.len() <= 1);
    v.first().cloned()
}
fn code_m(m: &Mat) -> Value {
    json!(m.iter().map(ToString::to_string).collect::<Vec<_>>())
}
fn code_p(p: &Option<A>) -> Value {
    p.as_ref()
        .map(|(x, y)| json!([x.to_string(), y.to_string()]))
        .unwrap_or(Value::Null)
}
fn smul(f: &fp::Fp, a: &Z, n: &Z, p: &Option<A>) -> Option<A> {
    if n.is_zero() {
        None
    } else {
        p.as_ref().map(|p| mul(f, a, n, p))
    }
}
fn add(f: &fp::Fp, a: &Z, p: &Option<A>, q: &Option<A>) -> Option<A> {
    let (p, q) = match (p, q) {
        (None, _) => return q.clone(),
        (_, None) => return p.clone(),
        (Some(p), Some(q)) => (p, q),
    };
    if p.0 == q.0 && f.add(&p.1, &q.1).is_zero() {
        return None;
    }
    let s = if p == q {
        f.mul(
            &f.add(&f.mul(&f.c(3), &f.mul(&p.0, &p.0)), a),
            &f.inv(&f.mul(&f.c(2), &p.1)),
        )
    } else {
        f.mul(&f.sub(&q.1, &p.1), &f.inv(&f.sub(&q.0, &p.0)))
    };
    let x = f.sub(&f.sub(&f.mul(&s, &s), &p.0), &q.0);
    let y = f.sub(&f.mul(&s, &f.sub(&p.0, &x)), &p.1);
    Some((x, y))
}
fn mixed_sum(f: &fp::Fp, a: &Z, p: &Option<A>, q: &Option<A>) -> Option<A> {
    let (p, q) = match (p, q) {
        (None, _) => return q.clone(),
        (_, None) => return p.clone(),
        (Some(p), Some(q)) => (p, q),
    };
    let z = mixed(f, a, &(p.0.clone(), p.1.clone(), Z::one()), q);
    if z.2.is_zero() {
        None
    } else {
        let iz = f.inv(&z.2);
        let iz2 = f.mul(&iz, &iz);
        Some((f.mul(&z.0, &iz2), f.mul(&z.1, &f.mul(&iz2, &iz))))
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = Path::new(args.get(1).expect("usage: p192 ROUND61"));
    let input: Value =
        serde_json::from_slice(&fs::read(root.join("../round52-p192/input.json")).unwrap())
            .unwrap();
    let labels: Value =
        serde_json::from_slice(&fs::read(root.join("../round25-p192/orbit-bridge.json")).unwrap())
            .unwrap();
    let fixture: Value =
        serde_json::from_slice(&fs::read(root.join("../round57-p192/P192-degrees.json")).unwrap())
            .unwrap();
    let f = fp::Fp {
        p: fp::parse(&input["p"]),
    };
    let fr = fp::Fp {
        p: fp::parse(&input["r"]),
    };
    let a = fp::parse(&input["a"]);
    let g = point(&input["G"]);
    let public = point(&input["Q"]);
    let mu = fp::parse(&labels["projected_order5_scalar"]);
    let lambda = fp::parse(&labels["relation_scalar"]);
    let mut rows = vec![];
    for (name, k) in [
        ("17G", Z::from(17u32)),
        ("31G", Z::from(31u32)),
        ("mu_G", mu),
        ("relation_lambda_G", lambda),
    ] {
        let q = mul(&f, &a, &k, &g);
        if name == "mu_G" || name == "relation_lambda_G" {
            let saved = fixture["targets"]
                .as_array()
                .unwrap()
                .iter()
                .find(|v| v["name"] == name)
                .unwrap();
            assert_eq!(q, point(&saved["Q"]))
        }
        let m0 = mn(&fr, [fr.c(2), fr.c(3), fr.c(5), fr.c(7)]);
        for kind in ["linear", "quadratic", "pole"] {
            let (m1, m2) = if kind == "pole" {
                (
                    mn(&fr, [fr.c(1), fr.c(0), fr.c(1), fr.sub(&Z::zero(), &k)]),
                    mn(&fr, [fr.c(1), fr.c(1), fr.c(1), fr.sub(&Z::zero(), &k)]),
                )
            } else {
                let fix = if kind == "linear" {
                    [fr.c(2), fr.sub(&Z::zero(), &k), fr.c(0), fr.c(1)]
                } else {
                    [fr.c(0), fr.mul(&k, &k), fr.c(1), fr.c(0)]
                };
                (m0.clone(), mm(&fr, &m0, &mn(&fr, fix)))
            };
            assert_ne!(m1, m2);
            assert_eq!(eval(&fr, &m1, &k), eval(&fr, &m2, &k));
            let p = collision(&fr, &m1, &m2);
            assert!(p.iter().any(|x| !x.is_zero()));
            let out = recover(&f, &fr, &a, &g, &q, &p).unwrap();
            assert_eq!(out, k);
            let mut bad = p.clone();
            bad[2] = fr.add(&bad[2], &Z::one());
            assert!(recover(&f, &fr, &a, &g, &q, &bad).is_none());
            let same = scale(&fr, &m1, &fr.c(17));
            assert_eq!(same, m1);
            let zero = collision(&fr, &m1, &same);
            assert!(zero.iter().all(Zero::is_zero));
            assert!(recover(&f, &fr, &a, &g, &q, &zero).is_none());
            rows.push(json!({"fixture":name,"kind":kind,"manufactured_with_known_label":true,"Q":[q.0.to_string(),q.1.to_string()],"M1":code_m(&m1),"M2":code_m(&m2),"polynomial":p.iter().map(ToString::to_string).collect::<Vec<_>>(),"roots":roots(&fr,&p).iter().map(ToString::to_string).collect::<Vec<_>>(),"verified_scalar":out.to_string(),"changed_polynomial_rejected":true,"tautology_rejected":true}));
        }
    }
    let mut public_rows = vec![];
    let mut state = Some(public.clone());
    let mut m = id();
    let mut aa = Z::one();
    let mut bb = Z::zero();
    for (i, (c, d)) in [
        (1, 0),
        (1, 1),
        (2, 3),
        (3, 5),
        (5, 7),
        (7, 11),
        (11, 13),
        (13, 17),
    ]
    .into_iter()
    .enumerate()
    {
        let cc = fr.c(c);
        let dd = fr.c(d);
        state = add(
            &f,
            &a,
            &smul(&f, &a, &cc, &state),
            &smul(&f, &a, &dd, &Some(g.clone())),
        );
        m = mm(
            &fr,
            &mn(&fr, [cc.clone(), dd.clone(), Z::zero(), Z::one()]),
            &m,
        );
        aa = fr.mul(&cc, &aa);
        bb = fr.add(&fr.mul(&cc, &bb), &dd);
        let expected = mixed_sum(
            &f,
            &a,
            &smul(&f, &a, &aa, &Some(public.clone())),
            &smul(&f, &a, &bb, &Some(g.clone())),
        );
        assert_eq!(state, expected);
        let raw = mn(&fr, [aa.clone(), bb.clone(), Z::zero(), Z::one()]);
        assert_eq!(m, raw);
        let same = scale(&fr, &m, &fr.c(i as i64 + 2));
        assert_eq!(collision(&fr, &m, &same), [Z::zero(), Z::zero(), Z::zero()]);
        public_rows.push(json!({"step":i,"affine_a":aa.to_string(),"affine_b":bb.to_string(),"M":code_m(&m),"point":code_p(&state),"tautology_rejected":true}));
    }
    let out = json!({"all_checks_pass":true,"p":input["p"],"r":input["r"],"labeled_collision_controls":rows,"labeled_recoveries":12,"public_Q":input["Q"],"public_affine_checks":public_rows,"public_unknown_scalar_recovered":false,"norm_evaluations":0,"scope":"full-width matrix/root/point certificate checks; labeled manufactured collisions; no P192 Mobius norm or unknown-target recovery"});
    if let Some(i) = args.iter().position(|s| s == "--check") {
        let v: Value = serde_json::from_slice(&fs::read(&args[i + 1]).unwrap()).unwrap();
        assert!(out == v, "certificate mismatch");
    }
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
