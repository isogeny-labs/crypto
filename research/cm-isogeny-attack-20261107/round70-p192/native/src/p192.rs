#![allow(dead_code, unused_imports)]
mod big;
use big::{code, load, point, z, Pt, E, K};
use num_bigint::BigUint as Z;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::fs;
include!("poly.rs");
fn mul(e: &E, p: &Pt, n: &Z) -> Pt {
    let mut q = None;
    let mut p = p.clone();
    let mut n = n.clone();
    while !n.is_zero() {
        if (&n & Z::one()) == Z::one() {
            q = e.add(&q, &p)
        }
        p = e.add(&p, &p);
        n >>= 1;
    }
    q
}
fn parsek(v: &Value) -> K {
    (
        v[0].as_str().unwrap().parse().unwrap(),
        v[1].as_str().unwrap().parse().unwrap(),
    )
}
fn parsept(v: &Value) -> Pt {
    if v.is_null() {
        None
    } else {
        Some((parsek(&v[0]), parsek(&v[1])))
    }
}
fn gcd(mut a: Z, mut b: Z) -> Z {
    while !b.is_zero() {
        let r = &a % &b;
        a = b;
        b = r
    }
    a
}
fn powmod(e: &E, a: &[K], n: &Z, h: &[K]) -> Vec<K> {
    let mut r = vec![e.c(1)];
    let mut a = br(e, a.to_vec(), h);
    let mut n = n.clone();
    while !n.is_zero() {
        if (&n & Z::one()) == Z::one() {
            r = br(e, bm(e, &r, &a), h)
        }
        a = br(e, bm(e, &a, &a), h);
        n >>= 1;
    }
    r
}
fn transform(e: &E, f: &[K], a: &[K], b: &[K]) -> Vec<K> {
    let n = f.len() - 1;
    let mut ap = vec![vec![e.c(1)]];
    let mut bp_ = ap.clone();
    for j in 1..=n {
        ap.push(bm(e, &ap[j - 1], a));
        bp_.push(bm(e, &bp_[j - 1], b))
    }
    let mut out = vec![e.c(0)];
    for j in 0..=n {
        out = ba(e, &out, &bm(e, &ap[j], &bp_[n - j]), &f[j])
    }
    out
}
fn sqrt_real(e: &E, v: &K) -> K {
    assert!(v.1.is_zero());
    let exp = (&e.p + Z::one()) >> 2;
    let mut y = (v.0.modpow(&exp, &e.p), Z::zero());
    if e.mulk(&y, &y) != *v {
        y = (Z::zero(), ((&e.p - &v.0) % &e.p).modpow(&exp, &e.p));
    }
    assert_eq!(e.mulk(&y, &y), *v);
    let neg = e.negk(&y);
    if neg < y {
        neg
    } else {
        y
    }
}
fn fval(e: &E, x: &K) -> K {
    e.addk(
        &e.addk(&bp(e, x, 3), &e.mulk(&(e.a.clone(), Z::zero()), x)),
        &(e.b.clone(), Z::zero()),
    )
}
struct Model<'a> {
    e: &'a E,
    a: K,
    b: K,
    q: Vec<K>,
    shift: K,
}
impl<'a> Model<'a> {
    fn new(e: &'a E, q: Vec<K>) -> Self {
        let shift = e.quotient(&q[2], &e.c(3));
        let c = e.mulk(&q[1], &q[3]);
        let d = e.mulk(&q[0], &bp(e, &q[3], 2));
        let a = e.subk(&c, &e.quotient(&bp(e, &q[2], 2), &e.c(3)));
        let b = e.addk(
            &e.subk(&d, &e.quotient(&e.mulk(&q[2], &c), &e.c(3))),
            &e.quotient(&e.mulk(&e.c(2), &bp(e, &q[2], 3)), &e.c(27)),
        );
        assert_ne!(
            e.addk(
                &e.mulk(&e.c(4), &bp(e, &a, 3)),
                &e.mulk(&e.c(27), &bp(e, &b, 2))
            ),
            e.c(0)
        );
        Self { e, a, b, q, shift }
    }
    fn convert(&self, p: Pt) -> Pt {
        p.map(|(x, y)| {
            (
                self.e.addk(&self.e.mulk(&self.q[3], &x), &self.shift),
                self.e.mulk(&self.q[3], &y),
            )
        })
    }
    fn on(&self, p: &Pt) -> bool {
        p.as_ref()
            .map(|(x, y)| {
                self.e.mulk(y, y)
                    == self.e.addk(
                        &self.e.addk(&bp(self.e, x, 3), &self.e.mulk(&self.a, x)),
                        &self.b,
                    )
            })
            .unwrap_or(true)
    }
    fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let e = self.e;
        let ((x, y), (u, v)) = match (p, q) {
            (None, _) => return q.clone(),
            (_, None) => return p.clone(),
            (Some(p), Some(q)) => (p, q),
        };
        if x == u && e.addk(y, v) == e.c(0) {
            return None;
        }
        let slope = if p == q {
            e.quotient(
                &e.addk(&e.mulk(&e.c(3), &bp(e, x, 2)), &self.a),
                &e.mulk(&e.c(2), y),
            )
        } else {
            e.quotient(&e.subk(v, y), &e.subk(u, x))
        };
        let z = e.subk(&e.subk(&bp(e, &slope, 2), x), u);
        Some((z.clone(), e.subk(&e.mulk(&slope, &e.subk(x, &z)), y)))
    }
    fn mul(&self, p: &Pt, n: &Z) -> Pt {
        let mut out = None;
        let mut p = p.clone();
        let mut n = n.clone();
        while !n.is_zero() {
            if (&n & Z::one()) == Z::one() {
                out = self.add(&out, &p)
            }
            p = self.add(&p, &p);
            n >>= 1;
        }
        out
    }
}
fn quotient(e: &E, x: &K, z: &K, l: &K, m: &K, j: usize) -> Pt {
    if x == m {
        return if j == 0 {
            None
        } else {
            Some((e.c(0), z.clone()))
        };
    }
    if x == l {
        return if j == 0 {
            Some((e.c(0), e.negk(z)))
        } else {
            None
        };
    }
    let u = e.quotient(&e.subk(x, l), &e.subk(x, m));
    let v = e.mulk(z, &bp(e, &e.subk(&u, &e.c(1)), 3));
    if j == 0 {
        Some((bp(e, &u, 2), v))
    } else {
        Some((e.inverse(&bp(e, &u, 2)), e.quotient(&v, &bp(e, &u, 3))))
    }
}
fn stream(e: &E, a0: &Pt, b0: &Pt, da: &Pt, db: &Pt) -> Value {
    let mut a = a0.clone();
    let mut b = b0.clone();
    let mut rows = vec![];
    for j in 0..14 {
        let ((x, y), (t, w)) = (a.clone().unwrap(), b.clone().unwrap());
        let s = e.subk(&x, &t);
        let z = e.mulk(&y, &w);
        assert_eq!(
            bp(e, &z, 2),
            e.mulk(&fval(e, &x), &fval(e, &e.subk(&x, &s)))
        );
        rows.push(json!({"j":j+1,"s":[s.0.to_string(),s.1.to_string()],"X":[x.0.to_string(),x.1.to_string()],"Z":[z.0.to_string(),z.1.to_string()]}));
        a = e.add(&a, da);
        b = e.add(&b, db)
    }
    let equal = rows.iter().all(|v| v["s"] == rows[0]["s"]);
    json!({"constant_s":equal,"rows":rows})
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = std::path::Path::new(&args[1]);
    let input = load(root.join("input-p192.json"));
    let inherited = load(root.join("input-blocks-r63.json"));
    let orders = load(root.join("input-orders.json"));
    let e = E {
        p: z(&input["p"]),
        a: z(&input["a"]),
        b: z(&input["b"]),
    };
    let r = z(&input["r"]);
    let g = point(&input["G"]);
    let public = point(&input["Q"]);
    assert!(e.on(&g) && e.on(&public));
    assert!(mul(&e, &g, &r).is_none());
    let f = vec![
        (e.b.clone(), Z::zero()),
        (e.a.clone(), Z::zero()),
        e.c(0),
        e.c(1),
    ];
    let xp = powmod(&e, &[e.c(0), e.c(1)], &e.p, &f);
    let mut aa = f.clone();
    let mut bb = ba(&e, &xp, &[e.c(0), e.c(1)], &e.negk(&e.c(1)));
    while !bz(&e, &bb) {
        let rem = bd(&e, &aa, &bb).1;
        aa = bb;
        bb = rem
    }
    assert_eq!(aa.len(), 1);
    let xp2 = powmod(&e, &xp, &e.p, &f);
    let xp3 = powmod(&e, &xp2, &e.p, &f);
    assert_eq!(xp3, vec![e.c(0), e.c(1)]);
    let source_model = Model::new(&e, f.clone());
    assert_eq!(source_model.add(&g, &g), e.add(&g, &g));
    assert!(source_model.mul(&g, &r).is_none());
    let torus = &e.p * &e.p + &e.p + Z::one();
    let torus_rows: Vec<_> = orders["orders"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            let n = Z::parse_bytes(v["hex"].as_str().unwrap().as_bytes(), 16).unwrap();
            json!({"name":v["name"],"r":n.to_string(),"gcd":gcd(n,torus.clone()).to_string()})
        })
        .collect();
    let mut targets = vec![];
    for row in inherited["rows"].as_array().unwrap() {
        let name = row["target"].as_str().unwrap().to_string();
        if !targets.iter().any(|(n, _): &(String, Pt)| n == &name) {
            targets.push((name, parsept(&row["Q"])))
        }
    }
    let mut rational = vec![];
    let mut generators = vec![
        ("G".to_string(), g.clone()),
        ("minus_G".into(), e.neg(&g)),
        ("2G".into(), e.add(&g, &g)),
        ("3G".into(), mul(&e, &g, &Z::from(3u32))),
    ];
    generators.extend(targets.clone());
    for (name, q) in generators {
        let row = stream(&e, &g, &q, &g, &q);
        if name == "G" || name == "minus_G" {
            assert!(row["constant_s"].as_bool().unwrap())
        } else {
            assert!(!row["constant_s"].as_bool().unwrap())
        }
        rational.push(json!({"target":name,"stream":row}));
    }
    let before = big::counts();
    let pshift = parsept(&inherited["P_shift"]);
    assert!(e.on(&pshift));
    let bsize: Z = inherited["full_r_layout_not_allocated"]["b"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let m: Z = inherited["full_r_layout_not_allocated"]["m"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    let bg = mul(&e, &g, &bsize);
    let a0 = e.add(&pshift, &bg);
    let da = mul(&e, &g, &m);
    let mut marked = vec![];
    for (name, q) in &targets {
        let bq = mul(&e, q, &bsize);
        let db = mul(&e, q, &m);
        for (un, u) in [("O", None), ("G", g.clone())] {
            let b0 = e.add(&e.add(&pshift, &u), &bq);
            let row = stream(&e, &a0, &b0, &da, &db);
            assert!(!row["constant_s"].as_bool().unwrap());
            marked.push(json!({"target":name,"U":un,"stream":row}));
        }
    }
    let marked_counts = big::diff(before, big::counts());
    let disc = e.negk(&e.addk(
        &e.mulk(&e.c(4), &bp(&e, &(e.a.clone(), Z::zero()), 3)),
        &e.mulk(&e.c(27), &bp(&e, &(e.b.clone(), Z::zero()), 2)),
    ));
    let delta = sqrt_real(&e, &disc);
    assert!(delta.1.is_zero());
    let mut specials = vec![];
    let special_start = big::counts();
    for d in [delta.clone(), e.negk(&delta)] {
        let s = e.quotient(&d, &e.mulk(&e.c(3), &(e.a.clone(), Z::zero())));
        let t = e.quotient(
            &e.subk(
                &s,
                &e.quotient(
                    &e.mulk(&e.c(3), &(e.b.clone(), Z::zero())),
                    &(e.a.clone(), Z::zero()),
                ),
            ),
            &e.c(2),
        );
        let bb = e.subk(
            &e.quotient(&(e.a.clone(), Z::zero()), &e.c(3)),
            &e.mulk(&t, &s),
        );
        assert_eq!(e.addk(&bp(&e, &t, 2), &bb), e.negk(&bp(&e, &s, 2)));
        assert_eq!(fval(&e, &t), e.negk(&bp(&e, &s, 3)));
        assert_eq!(fval(&e, &e.subk(&t, &s)), bp(&e, &s, 3));
        let fs = transform(&e, &f, &[e.negk(&s), e.c(1)], &[e.c(1)]);
        let h = bm(&e, &f, &fs);
        assert_eq!(
            transform(&e, &h, &[bb.clone(), t.clone()], &[e.negk(&t), e.c(1)]),
            bs(&e, &h, &e.negk(&bp(&e, &s, 6)))
        );
        let ii = (Z::zero(), Z::one());
        let l = e.addk(&t, &e.mulk(&ii, &s));
        let mm = e.subk(&t, &e.mulk(&ii, &s));
        let even = transform(
            &e,
            &h,
            &[e.negk(&l), mm.clone()],
            &[e.negk(&e.c(1)), e.c(1)],
        );
        assert_eq!(even.len(), 7);
        for j in [1, 3, 5] {
            assert_eq!(even[j], e.c(0))
        }
        let qs = [
            vec![
                even[0].clone(),
                even[2].clone(),
                even[4].clone(),
                even[6].clone(),
            ],
            vec![
                even[6].clone(),
                even[4].clone(),
                even[2].clone(),
                even[0].clone(),
            ],
        ];
        let mut quotients = vec![];
        for (j, q) in qs.iter().enumerate() {
            let model = Model::new(&e, q.clone());
            let inf = model.convert(Some((e.c(1), bp(&e, &e.subk(&mm, &l), 3))));
            assert!(model.on(&inf));
            let mut pts = vec![];
            let mut gg = g.clone();
            for _ in 1..=14 {
                let (x, y) = gg.clone().unwrap();
                let w = sqrt_real(&e, &fval(&e, &e.subk(&x, &s)));
                let zz = e.mulk(&y, &w);
                assert_eq!(bp(&e, &zz, 2), be(&e, &h, &x));
                let p = model.convert(quotient(&e, &x, &zz, &l, &mm, j));
                assert!(model.on(&p));
                let neg = model.convert(quotient(&e, &x, &e.negk(&zz), &l, &mm, j));
                assert_eq!(model.add(&p, &neg), None);
                pts.push(p);
                gg = e.add(&gg, &g)
            }
            let unbased = pts[1] == model.add(&pts[0], &pts[0]);
            let mut bases = vec![];
            for bas in [inf.clone(), e.neg(&inf)] {
                let z1 = model.add(&pts[0], &e.neg(&bas));
                let mut masks = vec![];
                for (idx, p) in pts.iter().enumerate() {
                    let expected = model.mul(&z1, &Z::from(idx + 1));
                    let mut mask = 0;
                    for (sign, pp) in [(1, p.clone()), (2, e.neg(p))] {
                        if model.add(&pp, &e.neg(&bas)) == expected {
                            mask |= sign
                        }
                    }
                    masks.push(mask)
                }
                let mut order_tests = vec![];
                for (name, p) in [
                    ("G", g.clone()),
                    ("2G", e.add(&g, &g)),
                    ("publicQ", public.clone()),
                ] {
                    let (x, y) = p.unwrap();
                    let zz = e.mulk(&y, &sqrt_real(&e, &fval(&e, &e.subk(&x, &s))));
                    let image = model.convert(quotient(&e, &x, &zz, &l, &mm, j));
                    let based = model.add(&image, &e.neg(&bas));
                    assert!(model.on(&based));
                    let rm = model.mul(&based, &r);
                    order_tests.push(json!({"input":name,"image":code(&based),"image_nonzero":based.is_some(),"r_image":code(&rm),"r_annihilates":rm.is_none()}));
                }
                bases.push(json!({"basepoint":code(&bas),"prefix_sign_masks":masks,"scalar_consistent_all14":masks.iter().all(|&v|v!=0),"order_tests":order_tests}));
            }
            // Exceptional finite charts are certified without selecting a general Fp2 square root.
            for (x, present) in [(&l, j == 0), (&mm, j == 1)] {
                if present {
                    let expected = e.mulk(&bp(&e, &q[3], 2), &be(&e, &h, x));
                    let rhs = e.addk(
                        &e.addk(&bp(&e, &model.shift, 3), &e.mulk(&model.a, &model.shift)),
                        &model.b,
                    );
                    assert_eq!(rhs, expected)
                }
            }
            quotients.push(json!({"index":j,"cubic":q.iter().map(|c|vec![c.0.to_string(),c.1.to_string()]).collect::<Vec<_>>(),"short_a":[model.a.0.to_string(),model.a.1.to_string()],"short_b":[model.b.0.to_string(),model.b.1.to_string()],"unbased_doubling":unbased,"points14":pts.iter().map(code).collect::<Vec<_>>(),"bases":bases,"canonical_conjugate_pair_sum_zero":true,"exceptional_finite_chart_identities":2}));
        }
        specials.push(json!({"s":[s.0.to_string(),s.1.to_string()],"quotients":quotients,"mobius_identity":true,"even_sextic_identity":true}));
    }
    let special_counts = big::diff(special_start, big::counts());
    let out = json!({"all_checks_pass":true,"p":e.p.to_string(),"r":r.to_string(),"cubic_irreducible":true,"frobenius_cube":true,"torus_order":torus.to_string(),"torus_order_gcds":torus_rows,"torus_character_matrix":[[0,-1],[1,-1]],"rational_streams":rational,"marked_streams":marked,"marked_generation_and_verification_counts":marked_counts,"special_fibers":specials,"special_construction_and_group_counts":special_counts,"whole_counts":big::counts(),"interpretation":"Branch-lift order/scalar tests qualify these explicit maps only; quotient group cardinalities not computed. Canonical elliptic pullback pushforward zero via conjugate pairs."});
    if let Some(i) = args.iter().position(|a| a == "--check") {
        let prior: Value = serde_json::from_slice(&fs::read(&args[i + 1]).unwrap()).unwrap();
        assert_eq!(out, prior, "changed P192 certificate")
    }
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
