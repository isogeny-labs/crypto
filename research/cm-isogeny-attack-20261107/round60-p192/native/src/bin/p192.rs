#[allow(dead_code, unused_imports)]
#[path = "../big.rs"]
mod big;
use big::{Pt, E, K};
use num_bigint::BigUint as Z;
use num_traits::{One, Zero};
use serde_json::{json, Value};
fn parse(v: &Value) -> Z {
    v.as_str()
        .map(|s| s.parse().unwrap())
        .unwrap_or_else(|| big::z(v))
}
fn point(v: &Value) -> Pt {
    Some(((parse(&v[0]), Z::zero()), (parse(&v[1]), Z::zero())))
}
fn mul(e: &E, mut n: u64, p: &Pt) -> Pt {
    let mut z = None;
    let mut cur = p.clone();
    while n > 0 {
        if n & 1 == 1 {
            z = e.add(&z, &cur)
        }
        cur = e.add(&cur, &cur);
        n >>= 1
    }
    z
}
fn field(v: &K) -> Value {
    json!([v.0.to_string(), v.1.to_string()])
}
fn coeff(e: &E, ds: &[K], xs: &[K], reference: bool) -> Vec<K> {
    if !reference {
        let mut v = vec![e.c(1), e.c(0), e.c(0)];
        for (d, x) in ds.iter().zip(xs) {
            v = vec![
                e.mulk(&v[0], d),
                e.addk(&e.mulk(&v[1], d), &v[0]),
                e.addk(&e.mulk(&v[2], d), &e.mulk(&v[0], x)),
            ]
        }
        return v;
    }
    let n = ds.len();
    let mut pre = vec![e.c(1); n + 1];
    let mut post = pre.clone();
    for i in 0..n {
        pre[i + 1] = e.mulk(&pre[i], &ds[i])
    }
    for i in (0..n).rev() {
        post[i] = e.mulk(&post[i + 1], &ds[i])
    }
    let mut v = vec![pre[n].clone(), e.c(0), e.c(0)];
    for i in 0..n {
        let c = e.mulk(&pre[i], &post[i + 1]);
        v[1] = e.addk(&v[1], &c);
        v[2] = e.addk(&v[2], &e.mulk(&c, &xs[i]))
    }
    v
}
fn square_root(e: &E, x: &K) -> K {
    let exp = (&e.p + Z::one()) / Z::from(4u32);
    if x.1.is_zero() {
        let a = x.0.modpow(&exp, &e.p);
        if &a * &a % &e.p == x.0 {
            return (a, Z::zero());
        }
        let b = ((&e.p - &x.0) % &e.p).modpow(&exp, &e.p);
        assert_eq!(&b * &b % &e.p, (&e.p - &x.0) % &e.p);
        return (Z::zero(), b);
    }
    let norm = (&x.0 * &x.0 + &x.1 * &x.1) % &e.p;
    let sn = norm.modpow(&exp, &e.p);
    assert_eq!(&sn * &sn % &e.p, norm);
    let half = (&e.p + Z::one()) / Z::from(2u32);
    for n in [sn.clone(), (&e.p - &sn) % &e.p] {
        let v = (&x.0 + n) * &half % &e.p;
        let a = v.modpow(&exp, &e.p);
        if a.is_zero() || &a * &a % &e.p != v {
            continue;
        }
        let b = &x.1 * (Z::from(2u32) * &a).modpow(&(&e.p - Z::from(2u32)), &e.p) % &e.p;
        let out = (a, b);
        assert_eq!(e.mulk(&out, &out), *x);
        return out;
    }
    panic!("non-square extension element")
}
fn decode(e: &E, x: &K, r0: &Pt) -> Pt {
    let rhs = e.addk(
        &e.addk(
            &e.mulk(&e.mulk(x, x), x),
            &e.mulk(&(e.a.clone(), Z::zero()), x),
        ),
        &(e.b.clone(), Z::zero()),
    );
    let y = square_root(e, &rhs);
    let mut out = vec![];
    for y in [y.clone(), e.negk(&y)] {
        let p = e.add(&Some((x.clone(), y)), &e.neg(r0));
        if p.is_none() {
            out.push(p.clone());
            continue;
        }
        if let Some((xx, yy)) = &p {
            if xx.1.is_zero() && yy.1.is_zero() {
                out.push(p)
            }
        }
    }
    assert_eq!(out.len(), 1);
    out.remove(0)
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = std::path::Path::new(
        args.get(1)
            .expect("usage: p192 ROUND60 [--cofactor-reference]"),
    );
    let reference = args.iter().any(|s| s == "--cofactor-reference");
    let input = big::load(root.join("../round52-p192/input.json"));
    let sh = big::load(root.join("../round54-p192/P192-shift.json"));
    let fixtures = big::load(root.join("../round57-p192/P192-degrees.json"));
    let e = E {
        p: parse(&input["p"]),
        a: parse(&input["a"]),
        b: parse(&input["b"]),
    };
    let g = point(&input["G"]);
    let shift = Some((
        (parse(&sh["R_6S"][0][0]), parse(&sh["R_6S"][0][1])),
        (parse(&sh["R_6S"][1][0]), parse(&sh["R_6S"][1][1])),
    ));
    assert!(e.on(&shift));
    assert_ne!(e.frob(&e.add(&shift, &shift)), e.add(&shift, &shift));
    let mut rows = vec![];
    for target in fixtures["targets"].as_array().unwrap().iter().skip(4) {
        let q = point(&target["Q"]);
        assert_ne!(q, g);
        assert_ne!(q, e.neg(&g));
        let delta = e.add(&g, &e.neg(&q));
        for j in [0u64, 1, 2, 3, 5, 7, 11, 15] {
            let u = mul(&e, j, &delta);
            let rr = e.add(&shift, &u);
            let phip = e.add(&shift, &e.neg(&e.frob(&shift)));
            let phir = e.add(&rr, &e.neg(&e.frob(&rr)));
            assert_eq!(phip, phir);
            let mut ds = vec![];
            let mut xs = vec![];
            for k in 0..16 {
                let pg = e.add(&shift, &mul(&e, k, &g));
                let rq = e.add(&rr, &mul(&e, k, &q));
                let x = pg.unwrap().0;
                let y = rq.unwrap().0;
                ds.push(e.subk(&x, &y));
                xs.push(x)
            }
            let v = coeff(&e, &ds, &xs, reference);
            assert_eq!(v[0], e.c(0));
            assert_ne!(v[1], e.c(0));
            let x = e.quotient(&v[2], &v[1]);
            let out = decode(&e, &x, &shift);
            assert_eq!(out, mul(&e, j, &g));
            assert!(e.on(&out));
            rows.push(json!({"target":target["name"],"Q":big::code(&q),"j":j,"U":big::code(&u),"coefficients":v.iter().map(field).collect::<Vec<_>>(),"decoded":big::code(&out),"normalizer_difference_zero":true}));
        }
    }
    assert_eq!(rows.len(), 24);
    println!("{}",serde_json::to_string_pretty(&json!({"all_checks_pass":true,"p":input["p"],"r":input["r"],"G":input["G"],"P_shift":big::code(&shift),"block_indices":"0..15","selected_blocks":24,"zero_input_blocks":3,"rows":rows,"scope":"selected P192 blocks with public U=j(G-Q); no complete P192 norm or arbitrary-input action"})).unwrap());
}
