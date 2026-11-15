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

struct TargetCache {
    qb: Vec<Pt>,
    qg: Vec<Pt>,
    beta: Vec<K>,
    gamma: Vec<K>,
    hq: Vec<K>,
}
fn rational_norm(
    e: &E,
    h: &[K],
    c: &TargetCache,
    ap: &Pt,
    bp_: &Pt,
    legacy: bool,
) -> ([K; 2], Value) {
    let st = big::counts();
    let (x, y) = ap.as_ref().unwrap();
    let (z, w) = bp_.as_ref().unwrap();
    let aa = (e.a.clone(), Z::zero());
    let bb = (e.b.clone(), Z::zero());
    let dg = bm(e, &[e.negk(x), e.c(1)], &[e.negk(x), e.c(1)]);
    let v = ba(e, &c.beta, &[e.negk(z)], &e.c(1));
    let dq = bm(e, &v, &v);
    let ng = vec![
        e.addk(&e.mulk(&aa, x), &e.mulk(&e.c(2), &bb)),
        e.addk(&e.mulk(x, x), &aa),
        x.clone(),
    ];
    let nq = ba(
        e,
        &ba(
            e,
            &[e.addk(&e.mulk(&aa, z), &e.mulk(&e.c(2), &bb))],
            &bs(e, &c.beta, &e.addk(&e.mulk(z, z), &aa)),
            &e.c(1),
        ),
        &bs(e, &bm(e, &c.beta, &c.beta), z),
        &e.c(1),
    );
    let l = br(
        e,
        ba(e, &bm(e, &ng, &dq), &bm(e, &nq, &dg), &e.negk(&e.c(1))),
        h,
    );
    let den = br(e, bm(e, &dg, &dq), h);
    let ft = vec![bb.clone(), aa.clone(), e.c(0), e.c(1)];
    let fb = ba(
        e,
        &ba(e, &[bb.clone()], &c.beta, &aa),
        &bm(e, &bm(e, &c.beta, &c.beta), &c.beta),
        &e.c(1),
    );
    let mut v = bm(e, &l, &l);
    v = ba(
        e,
        &v,
        &bm(e, &ft, &bm(e, &dq, &dq)),
        &e.negk(&e.mulk(&e.c(4), &e.mulk(y, y))),
    );
    v = ba(
        e,
        &v,
        &bm(e, &fb, &bm(e, &dg, &dg)),
        &e.negk(&e.mulk(&e.c(4), &e.mulk(w, w))),
    );
    v = ba(
        e,
        &v,
        &bm(e, &c.gamma, &den),
        &e.mulk(&e.c(8), &e.mulk(y, w)),
    );
    let e0 = br(e, v, h);
    let e1 = br(e, bs(e, &bm(e, &l, &den), &e.c(2)), h);
    let (num, branch) = bnorm(e, h, &e0, &e1);
    let numerator = big::diff(st, big::counts());
    let st = big::counts();
    let d = if legacy {
        let d2 = br(e, bm(e, &den, &den), h);
        bnorm(e, h, &d2, &[e.c(0)]).0[0].clone()
    } else {
        bp(e, &e.mulk(&be(e, h, x), &be(e, &c.hq, z)), 4)
    };
    assert_ne!(d, e.c(0));
    let denominator = big::diff(st, big::counts());
    let st = big::counts();
    let out = [e.quotient(&num[0], &d), e.quotient(&num[1], &d)];
    let division = big::diff(st, big::counts());
    (
        out,
        json!({"numerator_formation_and_norm_counts":numerator,"denominator_formation_and_norm_counts":denominator,"division_counts":division,"numerator_branch":branch}),
    )
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = std::path::Path::new(&args[1]);
    let legacy = args.iter().any(|s| s == "--legacy-denominator");
    let input = big::load(root.join("../round52-p192/input.json"));
    let old = big::load(root.join("../round63-p192/P192-blocks.json"));
    let e = E {
        p: pz(&input["p"]),
        a: pz(&input["a"]),
        b: pz(&input["b"]),
    };
    let g = point(&input["G"]);
    let quad = |v: &Value| -> Pt {
        if v.is_null() {
            None
        } else {
            Some(((pz(&v[0][0]), pz(&v[0][1])), (pz(&v[1][0]), pz(&v[1][1]))))
        }
    };
    let p = quad(&old["P_shift"]);
    let st = big::counts();
    let (gb, gg) = make(&e, &g);
    let roots: Vec<_> = gb
        .iter()
        .skip(1)
        .map(|v| v.as_ref().unwrap().0.clone())
        .collect();
    let h = bh(&e, &roots);
    let source_setup = big::diff(st, big::counts());
    let mut caches = std::collections::BTreeMap::<String, TargetCache>::new();
    let mut setups = vec![];
    let mut rows = vec![];
    for row in old["rows"].as_array().unwrap() {
        let key = row["target"].as_str().unwrap().to_string();
        let q = quad(&row["Q"]);
        let u = quad(&row["U"]);
        if !caches.contains_key(&key) {
            let st = big::counts();
            let (qb, qg) = make(&e, &q);
            let point_counts = big::diff(st, big::counts());
            let ys: Vec<_> = qb
                .iter()
                .skip(1)
                .map(|v| v.as_ref().unwrap().0.clone())
                .collect();
            let st = big::counts();
            let beta = bi(&e, &roots, &ys, &h);
            let wz: Vec<_> = gb
                .iter()
                .zip(&qb)
                .skip(1)
                .map(|(a, b)| e.mulk(&a.as_ref().unwrap().1, &b.as_ref().unwrap().1))
                .collect();
            let gamma = bi(&e, &roots, &wz, &h);
            let algebra_counts = big::diff(st, big::counts());
            let st = big::counts();
            let hq = bh(&e, &ys);
            let cache_counts = big::diff(st, big::counts());
            setups.push(json!({"target":key,"point_counts":point_counts,"algebra_counts":algebra_counts,"H_Q_cache_counts":cache_counts,"H_Q_coefficient_slots":hq.len(),"point_slots":qb.len()+qg.len(),"persistent_polynomial_coefficient_slots":beta.len()+gamma.len()+hq.len()}));
            caches.insert(
                key.clone(),
                TargetCache {
                    qb,
                    qg,
                    beta,
                    gamma,
                    hq,
                },
            );
        }
        let c = &caches[&key];
        let st = big::counts();
        let rr = e.add(&p, &u);
        let mut v = [e.c(1), e.c(0)];
        let mut blocks = vec![];
        let mut direct_counts = [0u64; 3];
        for i in 0..3 {
            let ap = e.add(&p, &gg[i]);
            let bpt = e.add(&rr, &c.qg[i]);
            let (norm, receipt) = rational_norm(&e, &h, c, &ap, &bpt, legacy);
            let start = big::counts();
            let mut explicit = [e.c(1), e.c(0)];
            for j in 1..=4 {
                let (ha, ka) = hk(&e, &ap, &gb[j]);
                let (hb, kb) = hk(&e, &bpt, &c.qb[j]);
                let l = e.subk(&ha, &hb);
                let k = e.subk(&ka, &kb);
                let v0 = e.subk(&e.mulk(&l, &l), &e.mulk(&k, &k));
                let wrong = e.addk(&ka, &kb);
                assert_ne!(v0, e.subk(&e.mulk(&l, &l), &e.mulk(&wrong, &wrong)));
                explicit = upd(&e, &explicit, &[v0, e.mulk(&e.c(2), &l)]);
            }
            let dc = big::diff(start, big::counts());
            for j in 0..3 {
                direct_counts[j] += dc[j]
            }
            assert_eq!(norm, explicit);
            let d = e.subk(&ap.as_ref().unwrap().0, &bpt.as_ref().unwrap().0);
            v = upd(&e, &v, &upd(&e, &[d, e.c(1)], &norm));
            blocks.push(receipt);
        }
        let total = big::diff(st, big::counts());
        let selected: [u64; 3] = std::array::from_fn(|i| total[i] - direct_counts[i]);
        let st = big::counts();
        let (w, zeros) = reference(&e, &g, &q, &p, &u);
        let rc = big::diff(st, big::counts());
        assert_eq!(v, w);
        assert_eq!(zeros, 1);
        assert_eq!(
            json!(v.iter().map(field).collect::<Vec<_>>()),
            row["coefficients"]
        );
        rows.push(json!({"target":key,"j":row["selected_j"],"coefficients":v.iter().map(field).collect::<Vec<_>>(),"selected_response_counts":selected,"direct_validation_counts":direct_counts,"full27_reference_counts":rc,"blocks":blocks}));
    }
    let out = json!({"all_checks_pass":true,"mode":if legacy{"legacy_denominator"}else{"cached_horner"},"source_setup_counts":source_setup,"target_setups":setups,"rows":rows,"whole_qualifier_counts":big::counts(),"scope":"24 prescribed P192 27-factor block responses,72 rational baby norms; no complete order-r response"});
    if let Some(i) = args.iter().position(|s| s == "--check") {
        let expected = big::load(&args[i + 1]);
        assert_eq!(out, expected, "changed P192 response record")
    }
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
