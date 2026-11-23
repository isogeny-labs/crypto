#[allow(dead_code, unused_imports)]
#[path = "../big.rs"]
mod big;
use serde_json::json;
use std::path::Path;
fn mul(e: &big::E, n: u64, p: &big::Pt) -> big::Pt {
    let mut z = None;
    for b in (0..64).rev() {
        z = e.add(&z, &z);
        if (n >> b) & 1 == 1 {
            z = e.add(&z, p)
        }
    }
    z
}
fn field(_e: &big::E, x: &big::K) -> serde_json::Value {
    json!([x.0.to_string(), x.1.to_string()])
}
fn side(e: &big::E, s: &big::Pt, t: &big::Pt) -> (big::K, big::K) {
    let (x, y) = s.as_ref().unwrap();
    let (u, v) = t.as_ref().unwrap();
    let d = e.subk(x, u);
    let d = e.mulk(&d, &d);
    let h = e.subk(
        &e.subk(&e.quotient(&e.addk(&e.mulk(y, y), &e.mulk(v, v)), &d), x),
        u,
    );
    let k = e.quotient(&e.mulk(&e.c(2), &e.mulk(y, v)), &d);
    (h, k)
}
fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("usage: p192_pairs ROUND57_DIRECTORY");
    let root = Path::new(&arg);
    let input = big::load(root.join("../round52-p192/input.json"));
    let sh = big::load(root.join("../round54-p192/P192-shift.json"));
    let e = big::E {
        p: big::z(&input["p"]),
        a: big::z(&input["a"]),
        b: big::z(&input["b"]),
    };
    let g = big::point(&input["G"]);
    let q = big::point(&input["Q"]);
    let s = Some((
        (big::z(&sh["S"][0][0]), big::z(&sh["S"][0][1])),
        (big::z(&sh["S"][1][0]), big::z(&sh["S"][1][1])),
    ));
    let pp = e.add(&g, &s);
    let rr = mul(&e, 6, &s);
    let mut rows = vec![];
    for i in [1, 2, 4, 8] {
        for j in [1, 3, 5, 7] {
            let u = e.add(&pp, &mul(&e, i, &g));
            let v = e.add(&rr, &mul(&e, i, &q));
            let tg = mul(&e, j, &g);
            let tq = mul(&e, j, &q);
            let (h1, k1) = side(&e, &u, &tg);
            let (h2, k2) = side(&e, &v, &tq);
            let trace = e.mulk(&e.c(2), &e.addk(&e.mulk(&h1, &h2), &e.mulk(&k1, &k2)));
            let norm = e.mulk(
                &e.subk(&e.mulk(&h1, &h1), &e.mulk(&k1, &k1)),
                &e.subk(&e.mulk(&h2, &h2), &e.mulk(&k2, &k2)),
            );
            let plus = e.mulk(&e.add(&u, &tg).unwrap().0, &e.add(&v, &tq).unwrap().0);
            let minus = e.mulk(
                &e.add(&u, &e.neg(&tg)).unwrap().0,
                &e.add(&v, &e.neg(&tq)).unwrap().0,
            );
            assert_eq!(trace, e.addk(&plus, &minus));
            assert_eq!(norm, e.mulk(&plus, &minus));
            let wrong = e.mulk(&e.c(2), &e.subk(&e.mulk(&h1, &h2), &e.mulk(&k1, &k2)));
            assert_ne!(wrong, trace);
            let cross = e.addk(
                &e.mulk(
                    &e.add(&u, &tg).unwrap().0,
                    &e.add(&v, &e.neg(&tq)).unwrap().0,
                ),
                &e.mulk(
                    &e.add(&u, &e.neg(&tg)).unwrap().0,
                    &e.add(&v, &tq).unwrap().0,
                ),
            );
            assert_eq!(cross, wrong);
            rows.push(json!({"giant":i,"baby":j,"trace":field(&e,&trace),"norm":field(&e,&norm),"reversed_sign_trace":field(&e,&wrong)}));
        }
    }
    println!("{}",serde_json::to_string_pretty(&json!({"all_checks_pass":true,"p":e.p.to_string(),"G":input["G"],"Q":input["Q"],"P":big::code(&pp),"R":big::code(&rr),"pairs":rows,"scope":"sixteen full-width paired quadratic identities only; no complete P192 trace"})).unwrap());
}
