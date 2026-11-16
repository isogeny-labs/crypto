#![allow(dead_code, unused_imports)]
#[path = "../big.rs"]
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
    for i in 0..3 {
        let ap = e.add(p, &gg[i]);
        let bp = e.add(&rr, &qg[i]);
        let d = e.subk(&ap.as_ref().unwrap().0, &bp.as_ref().unwrap().0);
        v = upd(e, &v, &[d, e.c(1)]);
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
            v = upd(e, &v, &[e0, e.mulk(&e.c(2), &a)])
        }
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
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = std::path::Path::new(&args[1]);
    let input = big::load(root.join("../round52-p192/input.json"));
    let sh = big::load(root.join("../round54-p192/P192-shift.json"));
    let targets = big::load(root.join("../round57-p192/P192-degrees.json"));
    let e = E {
        p: pz(&input["p"]),
        a: pz(&input["a"]),
        b: pz(&input["b"]),
    };
    let r = pz(&input["r"]);
    let g = point(&input["G"]);
    let p = Some((
        (pz(&sh["R_6S"][0][0]), pz(&sh["R_6S"][0][1])),
        (pz(&sh["R_6S"][1][0]), pz(&sh["R_6S"][1][1])),
    ));
    assert!(e.on(&p));
    assert_ne!(e.frob(&e.add(&p, &p)), e.add(&p, &p));
    let st = big::counts();
    let (gb, gg) = make(&e, &g);
    let gc = big::diff(st, big::counts());
    assert_eq!(gc[2], 7);
    let mut rows = vec![];
    let mut setups = vec![];
    for target in targets["targets"].as_array().unwrap().iter().skip(4) {
        let q = point(&target["Q"]);
        assert!(e.on(&q));
        let delta = e.add(&g, &e.neg(&q));
        let st = big::counts();
        let (qb, qg) = make(&e, &q);
        let qc = big::diff(st, big::counts());
        assert_eq!(qc[2], 7);
        setups.push(json!({"target":target["name"],"counts":qc}));
        for j in [0, 1, 4, 8, 9, 13, 17, 26] {
            let u = mul(&e, j, &delta);
            let st = big::counts();
            let (v, neg) = paired(&e, &gb, &gg, &qb, &qg, &p, &u);
            let pc = big::diff(st, big::counts());
            assert_eq!(pc[2], 7);
            let st = big::counts();
            let (w, zeros) = reference(&e, &g, &q, &p, &u);
            let rc = big::diff(st, big::counts());
            assert_eq!(v, w);
            assert_eq!(zeros, 1);
            assert_eq!(v[0], e.c(0));
            assert_ne!(v[1], e.c(0));
            rows.push(json!({"target":target["name"],"Q":big::code(&q),"selected_j":j,"U":big::code(&u),"coefficients":v.iter().map(field).collect::<Vec<_>>(),"candidate_counts":pc,"reference_counts":rc,"zero_factors":zeros,"wrong_relative_sign_pairs_rejected":neg}))
        }
    }
    assert_eq!(rows.len(), 24);
    let b = sqrt(&r) / Z::from(2u32);
    let m = &b * Z::from(2u32) + Z::one();
    let n = (&r + &m - Z::one()) / &m;
    let out = json!({"all_checks_pass":true,"p":input["p"],"r":input["r"],"P_shift":big::code(&p),"selected_layout":{"b":4,"m":9,"n":3,"indices":"0..26"},"source_setup_counts":gc,"target_setups":setups,"rows":rows,"full_r_layout_not_allocated":{"b":b.to_string(),"m":m.to_string(),"n":n.to_string(),"constructor_add_calls_per_generator":(&b+&n).to_string(),"point_slots_per_generator":(&b+&n+Z::one()).to_string()},"count_order":["quadratic_mulk_calls_excluding_inverse_internals","quadratic_inverse_calls","point_add_calls"],"scope":"24 planted full-width27-factor blocks; subset containment explicit, no full P192 B response or arbitrary-input decoder"});
    if let Some(i) = args.iter().position(|s| s == "--check") {
        let v: Value = serde_json::from_slice(&std::fs::read(&args[i + 1]).unwrap()).unwrap();
        assert_eq!(out, v, "changed P192 block")
    }
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
