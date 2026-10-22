//! Independent lookup-based replay of the bounded theta-constant search.
use serde_json::Value;
use std::fs;

fn m(x: u64, y: u64, p: u64) -> u64 {
    x * y % p
}
fn s(x: u64, y: u64, p: u64) -> u64 {
    (x + p - y) % p
}
fn pow(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut z = 1;
    while n != 0 {
        if n & 1 != 0 {
            z = m(z, x, p);
        }
        x = m(x, x, p);
        n >>= 1;
    }
    z
}
fn roots(x: u64, lookup: &[u64], p: u64) -> Vec<u64> {
    let r = lookup[x as usize];
    if r == u64::MAX {
        Vec::new()
    } else if r == 0 {
        vec![0]
    } else {
        vec![r, s(0, r, p)]
    }
}
fn nonzero(x: u64) -> bool {
    x != 0
}
fn check(
    p: u64,
    l: u64,
    mu: u64,
    nu: u64,
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    e: u64,
    f: u64,
    alpha: u64,
) -> bool {
    let add = |x: u64, y: u64| (x + y) % p;
    let aa = add(add(a, b), add(c, d));
    let bb = s(add(a, b), add(c, d), p);
    let cc = add(s(a, b, p), s(c, d, p));
    let dd = add(s(a, b, p), s(d, c, p));
    [
        a,
        b,
        c,
        d,
        e,
        f,
        aa,
        bb,
        cc,
        dd,
        s(m(a, d, p), m(b, c, p), p),
        s(m(a, c, p), m(b, d, p), p),
        s(m(a, b, p), m(c, d, p), p),
    ]
    .iter()
    .all(|x| nonzero(*x))
        && m(a, c, p) == m(l, m(b, d, p), p)
        && m(c, e, p) == m(mu, m(d, f, p), p)
        && m(a, e, p) == m(nu, m(b, f, p), p)
        && m(e, s(1, alpha, p), p) == m(f, add(1, alpha), p)
        && m(m(alpha, alpha, p), m(aa, bb, p), p) == m(cc, dd, p)
}
fn main() {
    let source =
        fs::read_to_string("research/four_followups_20261021/results/kummer-model-gate.jsonl")
            .unwrap();
    let rows =
        fs::read_to_string("research/kummer_theta_20261022/results/theta-models.jsonl").unwrap();
    assert_eq!(source.lines().count(), rows.lines().count());
    let mut n = 0;
    let mut grand = 0;
    for (source, row) in source.lines().zip(rows.lines()) {
        let x: Value = serde_json::from_str(source).unwrap();
        let y: Value = serde_json::from_str(row).unwrap();
        let p = x["p"].as_u64().unwrap();
        assert_eq!(p, y["p"].as_u64().unwrap());
        let rr: Vec<u64> = x["finite_rational_weierstrass_roots"]
            .as_array()
            .unwrap()
            .iter()
            .map(|z| z.as_u64().unwrap())
            .collect();
        let mut lookup = vec![u64::MAX; p as usize];
        for i in 0..p {
            lookup[m(i, i, p) as usize] = i;
        }
        let mut pairs = 0;
        let mut squares = 0;
        let mut orders = 0;
        let mut solutions = 0;
        for &r0 in &rr {
            for &r1 in &rr {
                if r0 == r1 {
                    continue;
                }
                pairs += 1;
                let delta = s(r1, r0, p);
                let scale = m(
                    x["quintic_coefficients"][5].as_u64().unwrap(),
                    pow(delta, 5, p),
                    p,
                );
                if roots(scale, &lookup, p).is_empty() {
                    continue;
                }
                squares += 1;
                let rem: Vec<u64> = rr
                    .iter()
                    .copied()
                    .filter(|z| *z != r0 && *z != r1)
                    .map(|z| m(s(z, r0, p), pow(delta, p - 2, p), p))
                    .collect();
                for perm in [
                    [0, 1, 2],
                    [0, 2, 1],
                    [1, 0, 2],
                    [1, 2, 0],
                    [2, 0, 1],
                    [2, 1, 0],
                ] {
                    orders += 1;
                    let (l, mu, nu) = (rem[perm[0]], rem[perm[1]], rem[perm[2]]);
                    let u2 = m(m(l, nu, p), pow(mu, p - 2, p), p);
                    for u in roots(u2, &lookup, p) {
                        if u == 0 {
                            continue;
                        }
                        let v = m(l, pow(u, p - 2, p), p);
                        let t = m(nu, pow(u, p - 2, p), p);
                        if t == 0 || (t + 1) % p == 0 {
                            continue;
                        }
                        let alpha = m(s(t, 1, p), pow((t + 1) % p, p - 2, p), p);
                        let alpha2 = m(alpha, alpha, p);
                        let n = s(
                            m(s(u, 1, p), s(u, 1, p), p),
                            m(alpha2, m((u + 1) % p, (u + 1) % p, p), p),
                            p,
                        );
                        let q = s(
                            m(s(v, 1, p), s(v, 1, p), p),
                            m(alpha2, m((v + 1) % p, (v + 1) % p, p), p),
                            p,
                        );
                        if q == 0 {
                            continue;
                        }
                        for d in roots(m(n, pow(q, p - 2, p), p), &lookup, p) {
                            if d == 0 {
                                continue;
                            }
                            if check(p, l, mu, nu, u, 1, m(v, d, p), d, t, 1, alpha) {
                                solutions += 1;
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(pairs, y["ordered_pairs"].as_u64().unwrap());
        assert_eq!(squares, y["square_pairs"].as_u64().unwrap());
        assert_eq!(orders, y["eligible_orders"].as_u64().unwrap());
        assert_eq!(solutions, y["theta_tuples"].as_u64().unwrap());
        assert_eq!(solutions > 0, y["fp_theta_tuple_found"].as_bool().unwrap());
        if solutions > 0 {
            let z = &y["selected"];
            let k = &z["constants"];
            let v = |name: &str| k[name].as_u64().unwrap();
            assert!(check(
                p,
                v("lambda"),
                v("mu"),
                v("nu"),
                v("a"),
                v("b"),
                v("c"),
                v("d"),
                v("e"),
                v("f"),
                v("alpha")
            ));
            assert_eq!(
                z["lambda_mu_nu"],
                serde_json::json!([v("lambda"), v("mu"), v("nu")])
            );
            assert_eq!(v("A"), (v("a") + v("b") + v("c") + v("d")) % p);
            assert_eq!(v("B"), s((v("a") + v("b")) % p, (v("c") + v("d")) % p, p));
            assert_eq!(v("C"), (s(v("a"), v("b"), p) + s(v("c"), v("d"), p)) % p);
            assert_eq!(v("D"), (s(v("a"), v("b"), p) + s(v("d"), v("c"), p)) % p);
            let r0 = z["r0"].as_u64().unwrap();
            let r1 = z["r1"].as_u64().unwrap();
            assert!(rr.contains(&r0) && rr.contains(&r1) && r0 != r1);
            let delta = s(r1, r0, p);
            assert_eq!(delta, z["delta"].as_u64().unwrap());
            assert_eq!(
                m(
                    z["sqrt_scale"].as_u64().unwrap(),
                    z["sqrt_scale"].as_u64().unwrap(),
                    p
                ),
                m(
                    x["quintic_coefficients"][5].as_u64().unwrap(),
                    pow(delta, 5, p),
                    p
                )
            );
            let lmn = [v("lambda"), v("mu"), v("nu")];
            assert!(lmn[0] != lmn[1] && lmn[0] != lmn[2] && lmn[1] != lmn[2]);
            for z0 in lmn {
                assert!(rr
                    .iter()
                    .copied()
                    .filter(|r| *r != r0 && *r != r1)
                    .any(|r| z0 == m(s(r, r0, p), pow(delta, p - 2, p), p)));
            }
        }
        grand += solutions;
        n += 1;
    }
    assert_eq!(n, 21);
    println!("validated_models={n} validated_theta_tuples={grand} lookup_replay=pass selected_original_equations=pass");
}
