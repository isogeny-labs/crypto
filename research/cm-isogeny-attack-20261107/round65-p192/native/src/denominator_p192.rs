#![allow(dead_code, unused_imports)]
#[path = "big.rs"]
mod big;
use big::{Pt, E, K};
use num_bigint::BigUint as Z;
use num_traits::{One, Zero};
use serde_json::{json, Value};
fn pz(v: &Value) -> Z {
    v.as_str()
        .map(|s| s.parse().unwrap())
        .unwrap_or_else(|| big::z(v))
}
fn point(v: &Value) -> Pt {
    Some(((pz(&v[0]), Z::zero()), (pz(&v[1]), Z::zero())))
}
fn field(k: &K) -> Value {
    json!([k.0.to_string(), k.1.to_string()])
}
fn mul(e: &E, mut n: u64, p: &Pt) -> Pt {
    let mut v = None;
    let mut t = p.clone();
    while n > 0 {
        if n & 1 == 1 {
            v = e.add(&v, &t)
        }
        t = e.add(&t, &t);
        n >>= 1
    }
    v
}
fn make(e: &E, t: &Pt) -> (Vec<Pt>, Vec<Pt>) {
    let mut b = vec![None, t.clone()];
    for i in 2..=4 {
        b.push(e.add(&b[i - 1], t))
    }
    let step = e.add(&e.add(&b[4], &b[4]), t);
    let mut g = vec![b[4].clone()];
    for i in 1..3 {
        g.push(e.add(&g[i - 1], &step))
    }
    (b, g)
}
fn hk(e: &E, p: &Pt, t: &Pt) -> (K, K) {
    let (x, y) = p.as_ref().unwrap();
    let (z, w) = t.as_ref().unwrap();
    let den = e.subk(x, z);
    let iz = e.inverse(&e.mulk(&den, &den));
    let a = (e.a.clone(), Z::zero());
    let b = (e.b.clone(), Z::zero());
    let h = e.addk(
        &e.addk(
            &e.addk(
                &e.mulk(z, &e.mulk(x, x)),
                &e.mulk(&e.addk(&a, &e.mulk(z, z)), x),
            ),
            &e.mulk(&a, z),
        ),
        &e.mulk(&e.c(2), &b),
    );
    let k = e.mulk(&e.c(2), &e.mulk(y, w));
    (e.mulk(&h, &iz), e.mulk(&k, &iz))
}
fn upd(e: &E, v: &[K; 2], w: &[K; 2]) -> [K; 2] {
    [
        e.mulk(&v[0], &w[0]),
        e.addk(&e.mulk(&v[1], &w[0]), &e.mulk(&v[0], &w[1])),
    ]
}
fn paired(e: &E, gb: &[Pt], gg: &[Pt], qb: &[Pt], qg: &[Pt], p: &Pt, u: &Pt) -> ([K; 2], u64) {
    let rr = e.add(p, u);
    let mut v = [e.c(1), e.c(0)];
    let mut neg = 0;
    let roots: Vec<_> = gb
        .iter()
        .skip(1)
        .map(|p| p.as_ref().unwrap().0.clone())
        .collect();
    for i in 0..3 {
        let ap = e.add(p, &gg[i]);
        let bp = e.add(&rr, &qg[i]);
        let d = e.subk(&ap.as_ref().unwrap().0, &bp.as_ref().unwrap().0);
        let mut pairs = vec![];
        for j in 1..=4 {
            let (h, k) = hk(e, &ap, &gb[j]);
            let (hh, kk) = hk(e, &bp, &qb[j]);
            let a = e.subk(&h, &hh);
            let b = e.subk(&k, &kk);
            let plus = e.addk(&k, &kk);
            let aa = e.mulk(&a, &a);
            let e0 = e.subk(&aa, &e.mulk(&b, &b));
            assert_ne!(e0, e.subk(&aa, &e.mulk(&plus, &plus)));
            neg += 1;
            pairs.push([e0, e.mulk(&e.c(2), &a)])
        }
        let pairnorm = norm_pairs(e, &roots, &pairs);
        v = upd(e, &v, &upd(e, &[d, e.c(1)], &pairnorm));
    }
    (v, neg)
}
fn reference(e: &E, g: &Pt, q: &Pt, p: &Pt, u: &Pt) -> ([K; 2], usize) {
    let rr = e.add(p, u);
    let mut gt = None;
    let mut qt = None;
    let mut ds = vec![];
    let mut zeros = 0;
    for _ in 0..27 {
        let x = e.add(p, &gt).unwrap().0;
        let y = e.add(&rr, &qt).unwrap().0;
        let d = e.subk(&x, &y);
        if d == e.c(0) {
            zeros += 1
        }
        ds.push(d);
        gt = e.add(&gt, g);
        qt = e.add(&qt, q)
    }
    let mut pre = vec![e.c(1); 28];
    let mut post = pre.clone();
    for i in 0..27 {
        pre[i + 1] = e.mulk(&pre[i], &ds[i])
    }
    for i in (0..27).rev() {
        post[i] = e.mulk(&post[i + 1], &ds[i])
    }
    let b = (0..27).fold(e.c(0), |s, i| e.addk(&s, &e.mulk(&pre[i], &post[i + 1])));
    ([pre[27].clone(), b], zeros)
}
fn sqrt(n: &Z) -> Z {
    let mut lo = Z::zero();
    let mut hi = Z::one() << ((n.bits() + 1) / 2);
    while &lo + Z::one() < hi {
        let mid = (&lo + &hi) >> 1;
        if &mid * &mid <= *n {
            lo = mid
        } else {
            hi = mid
        }
    }
    lo
}
include!("big_norm.rs");

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = std::path::Path::new(&args[1]);
    let input = big::load(root.join("../round52-p192/input.json"));
    let old = big::load(root.join("../round63-p192/P192-blocks.json"));
    let e = E {
        p: pz(&input["p"]),
        a: pz(&input["a"]),
        b: pz(&input["b"]),
    };
    let g = point(&input["G"]);
    let p = old["P_shift"].clone();
    let p = Some(((pz(&p[0][0]), pz(&p[0][1])), (pz(&p[1][0]), pz(&p[1][1]))));
    let (gb, gg) = make(&e, &g);
    let roots: Vec<_> = gb
        .iter()
        .skip(1)
        .map(|p| p.as_ref().unwrap().0.clone())
        .collect();
    let h = bh(&e, &roots);
    let mut rows = vec![];
    for row in old["rows"].as_array().unwrap() {
        let q = point(&row["Q"]);
        let (qb, qg) = make(&e, &q);
        let ys: Vec<_> = qb
            .iter()
            .skip(1)
            .map(|p| p.as_ref().unwrap().0.clone())
            .collect();
        let hq = bh(&e, &ys);
        let beta = bi(&e, &roots, &ys, &h);
        let uv = &row["U"];
        let u = if uv.is_null() {
            None
        } else {
            Some((
                (pz(&uv[0][0]), pz(&uv[0][1])),
                (pz(&uv[1][0]), pz(&uv[1][1])),
            ))
        };
        let rr = e.add(&p, &u);
        for i in 0..3 {
            let a = e.add(&p, &gg[i]).unwrap();
            let b = e.add(&rr, &qg[i]).unwrap();
            let dg = bm(&e, &[e.negk(&a.0), e.c(1)], &[e.negk(&a.0), e.c(1)]);
            let v = ba(&e, &beta, &[e.negk(&b.0)], &e.c(1));
            let dq = bm(&e, &v, &v);
            let d = br(&e, bm(&e, &dg, &dq), &h);
            let d2 = br(&e, bm(&e, &d, &d), &h);
            let st = big::counts();
            let refv = bnorm(&e, &h, &d2, &[e.c(0)]).0;
            let rc = big::diff(st, big::counts());
            let st = big::counts();
            let value = bp(&e, &e.mulk(&be(&e, &h, &a.0), &be(&e, &hq, &b.0)), 4);
            let hc = big::diff(st, big::counts());
            assert_eq!(refv, [value.clone(), e.c(0)]);
            rows.push(json!({"target":row["target"],"j":row["selected_j"],"block":i,"value":field(&value),"horner_counts":hc,"resultant_counts":rc}))
        }
    }
    let out = json!({"all_checks_pass":true,"norms":rows.len(),"rows":rows,"scope":"72 prescribed P192 denominator norms; input preparation explicit; no full response"});
    if let Some(i) = args.iter().position(|s| s == "--check") {
        let v: Value = serde_json::from_slice(&std::fs::read(&args[i + 1]).unwrap()).unwrap();
        assert_eq!(out, v)
    }
    println!("{}", serde_json::to_string_pretty(&out).unwrap())
}
