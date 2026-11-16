//! Independent pointwise modular targets and coordinate-algebra traces.
//! No Gram matrix is used by this module.
use crate::{
    large_field::*,
    reverse06::{decode, dp, poly, A, AP, R},
    round02, round04,
};
use serde_json::{json, Value};
use std::path::Path;
pub(crate) fn ca(_r: &R, x: &Poly) -> A {
    A {
        u: trim(x.clone()),
        v: vec![],
    }
}
pub(crate) fn ap(r: &R, x: &Value, y: &Value) -> AP {
    AP::Pt(ca(r, &poly(r.f, x)), ca(r, &poly(r.f, y)))
}
pub(crate) fn ax(p: &AP) -> A {
    if let AP::Pt(x, _) = p {
        x.clone()
    } else {
        panic!("nonzero point required")
    }
}
fn check(r: &R, p: &AP) {
    if let AP::Pt(x, y) = p {
        assert_eq!(
            r.add(&r.sq(y), &r.mul(x, y)),
            r.add(
                &r.add(&r.mul(&r.sq(x), x), &r.mul(&r.c(&r.a), &r.sq(x))),
                &r.c(&r.b)
            )
        );
    }
}
fn av(r: &R, p: &[A], x: &A) -> A {
    p.iter().rev().fold(r.z(), |v, c| r.add(&r.mul(&v, x), c))
}
fn atrim(mut a: Vec<A>) -> Vec<A> {
    while a.last().is_some_and(|x| x.u.is_empty() && x.v.is_empty()) {
        a.pop();
    }
    a
}
fn adiv(r: &R, a: &[A], b: &[A]) -> Vec<A> {
    let mut a = atrim(a.to_vec());
    let inv = r.inv(b.last().unwrap());
    while a.len() >= b.len() {
        let k = a.len() - b.len();
        let c = r.mul(a.last().unwrap(), &inv);
        for (i, v) in b.iter().enumerate() {
            a[k + i] = r.add(&a[k + i], &r.mul(&c, v));
        }
        a = atrim(a);
    }
    a
}
fn agcd(r: &R, mut a: Vec<A>, mut b: Vec<A>) -> Vec<A> {
    while !b.is_empty() {
        let z = adiv(r, &a, &b);
        a = b;
        b = z;
    }
    let v = r.inv(a.last().unwrap());
    a.iter().map(|x| r.mul(x, &v)).collect()
}
fn param(r: &R, a: &A, b: &A) -> A {
    let mut p = vec![r.z(); 9];
    p[0] = r.o();
    p[4] = r.o();
    p[7] = a.clone();
    p[8] = r.o();
    let mut q = vec![r.z(); 9];
    q[0] = r.o();
    q[1] = b.clone();
    q[4] = r.o();
    q[8] = r.o();
    let g = agcd(r, p, q);
    assert_eq!(g.len(), 2, "modular gcd over coordinate field");
    g[0].clone()
}
fn fourth_root_basis(r: &R) -> Vec<A> {
    let d = r.p.len() - 1;
    if d == 1 {
        return vec![r.o()];
    };
    let mut x = ca(r, &vec![r.f.z(), r.f.o()]);
    for _ in 0..r.f.m() * d as u32 - 2 {
        x = r.sq(&x);
    }
    let mut powers = vec![r.o()];
    for _ in 1..d {
        powers.push(r.mul(powers.last().unwrap(), &x));
    }
    powers
}
fn root4(r: &R, a: &A, powers: &[A]) -> A {
    assert!(a.v.is_empty());
    let x = a.u.iter().enumerate().fold(r.z(), |z, (i, c)| {
        r.add(&z, &r.mul(&r.c(&r.f.frob(c, r.f.m() - 2)), &powers[i]))
    });
    assert_eq!(r.sq(&r.sq(&x)), *a);
    x
}
fn reconstruct(r: &R, js: &[A; 3], powers: &[A]) -> AP {
    let t = param(r, &js[0], &js[1]);
    let y = param(r, &js[1], &js[2]);
    let x = root4(r, &r.mul(&t, &y), powers);
    let d = r.add(&r.add(&r.sq(&x), &x), &r.o());
    let yy = r.mul(&r.add(&r.add(&y, &x), &r.o()), &r.inv(&d));
    let p = AP::Pt(x, yy);
    check(r, &p);
    p
}
fn qb_a(r: &R, b: &A, k: &[A]) -> A {
    let t = k.iter().fold(r.z(), |z, x| r.add(&z, x));
    r.add(&r.add(b, &t), &r.sq(&t))
}
fn velu_a(r: &R, k: &[A], xs: &[A]) -> Vec<A> {
    let mut p = vec![r.o()];
    for x in k {
        let mut q = vec![r.z(); p.len() + 1];
        for (i, v) in p.iter().enumerate() {
            q[i] = r.add(&q[i], &r.mul(v, x));
            q[i + 1] = r.add(&q[i + 1], v);
        }
        p = q;
    }
    let derivative: Vec<_> = p
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, c)| if i % 2 == 1 { c.clone() } else { r.z() })
        .collect();
    xs.iter()
        .map(|x| {
            let z = r.mul(&av(r, &derivative, x), &r.inv(&av(r, &p, x)));
            r.add(&r.add(x, &r.mul(x, &z)), &r.mul(&r.sq(x), &r.sq(&z)))
        })
        .collect()
}
pub(crate) fn targets(
    f: &F,
    pmod: &Poly,
    a: &E,
    b: &E,
    k7: &Poly,
    l7: &Poly,
    p: &AP,
    q: &AP,
) -> Vec<AP> {
    let r = R {
        f,
        p: pmod.clone(),
        a: a.clone(),
        b: b.clone(),
        fx: vec![b.clone(), f.z(), a.clone(), f.o()],
    };
    check(&r, p);
    check(&r, q);
    assert_eq!(r.times(p, 11), AP::O);
    assert_eq!(r.times(q, 11), AP::O);
    let dst = R {
        f,
        p: pmod.clone(),
        a: f.o(),
        b: f.o(),
        fx: vec![f.o(), f.z(), f.o(), f.o()],
    };
    let powers = fourth_root_basis(&dst);
    let k: Vec<_> = k7.iter().map(|x| r.c(x)).collect();
    let l: Vec<_> = l7.iter().map(|x| r.c(x)).collect();
    let bb = r.c(b);
    let b7 = qb_a(&r, &bb, &k);
    let mut out = vec![];
    for line in 0..12 {
        let z = if line == 11 {
            q.clone()
        } else {
            r.plus(p, &r.times(q, line))
        };
        let kernel: Vec<_> = (1..=5).map(|i| ax(&r.times(&z, i))).collect();
        let b0 = qb_a(&r, &bb, &kernel);
        let kk = velu_a(&r, &kernel, &k);
        let ell = velu_a(&r, &k, &kernel);
        let b70 = qb_a(&r, &b0, &kk);
        assert_eq!(b70, qb_a(&r, &b7, &ell));
        let ll = velu_a(&r, &ell, &l);
        let b490 = qb_a(&r, &b70, &ll);
        let t = reconstruct(&dst, &[r.inv(&b0), r.inv(&b70), r.inv(&b490)], &powers);
        out.push(t);
    }
    out
}
pub(crate) fn frob(r: &R, p: &AP, n: u32) -> AP {
    match p {
        AP::O => AP::O,
        AP::Pt(x, y) => {
            let (mut x, mut y) = (x.clone(), y.clone());
            for _ in 0..n {
                x = r.sq(&x);
                y = r.sq(&y);
            }
            AP::Pt(x, y)
        }
    }
}
pub(crate) fn constant(f: &F, p: &AP) -> P {
    match p {
        AP::O => P::Infinity,
        AP::Pt(x, y) => {
            assert!(
                x.v.is_empty() && y.v.is_empty() && x.u.len() <= 1 && y.u.len() <= 1,
                "trace did not descend to coefficient field"
            );
            P::Affine {
                x: x.u.first().cloned().unwrap_or_else(|| f.z()),
                y: y.u.first().cloned().unwrap_or_else(|| f.z()),
            }
        }
    }
}
pub(crate) fn ap_json(p: &AP) -> Value {
    match p {
        AP::O => json!({"infinity":true}),
        AP::Pt(x, y) => {
            assert!(x.v.is_empty() && y.v.is_empty());
            json!({"x":polyj(&x.u),"y":polyj(&y.u)})
        }
    }
}
pub(crate) fn trace(f: &F, modulus: &Poly, p: &AP) -> P {
    let r = R {
        f,
        p: modulus.clone(),
        a: f.o(),
        b: f.o(),
        fx: vec![],
    };
    let mut q = p.clone();
    let mut sum = AP::O;
    for _ in 0..modulus.len() - 1 {
        sum = r.plus(&sum, &q);
        q = frob(&r, &q, f.m());
    }
    assert_eq!(q, *p);
    constant(f, &sum)
}
pub(crate) fn line_map(a: u64, b: u64, c: u64, d: u64, line: usize) -> usize {
    let (x, y) = if line == 11 {
        (c, d)
    } else {
        ((a + line as u64 * c) % 11, (b + line as u64 * d) % 11)
    };
    if x == 0 {
        11
    } else {
        (y * (1..11).find(|v| v * x % 11 == 1).unwrap() % 11) as usize
    }
}
pub(crate) fn tuple(ec: &crate::binary_ecc::BinaryCurve, targets: &[P], odd: bool) -> Vec<P> {
    let mut ds = vec![];
    for i in 0..5 {
        let a = 2usize.pow(i) % 11;
        let mut v = plus(
            ec,
            &targets[a],
            &if odd {
                neg(&targets[11 - a])
            } else {
                targets[11 - a].clone()
            },
        );
        if odd && i % 2 == 1 {
            v = neg(&v);
        }
        ds.push(v);
    }
    let total = ds.iter().fold(P::Infinity, |z, p| plus(ec, &z, p));
    ds.iter()
        .map(|p| plus(ec, &times(ec, p, 5), &neg(&total)))
        .collect()
}
pub fn seeds(dir: &Path) {
    let f = F::new(120, 27);
    let pos = round04::read(&dir.join("r3b-complete-split-fiber.json"));
    let pscreen = round04::read(&dir.join("r3-reverse-fiber-screen.json"));
    let ns = round04::read(&dir.join("r3-reverse-fiber-screen-negative.json"));
    let mut positive = vec![];
    let mut negative = vec![];
    for row in pos["neighbors"].as_array().unwrap() {
        let n = row["neighbor"].as_u64().unwrap() as usize;
        let s = &pscreen["neighbors"][n];
        let r = R {
            f: &f,
            p: vec![f.z(), f.o()],
            a: decode(&f, &row["source_a"]),
            b: decode(&f, &row["source_b"]),
            fx: vec![],
        };
        let p = r.cp(&dp(&f, &row["paired_basis"][0]));
        let q = r.cp(&dp(&f, &row["paired_basis"][1]));
        let tt = targets(
            &f,
            &r.p,
            &r.a,
            &r.b,
            &poly(&f, &s["source_first7_roots"]),
            &poly(&f, &s["source_second7_roots"]),
            &p,
            &q,
        );
        let tt: Vec<_> = tt.iter().map(|p| constant(&f, p)).collect();
        for (i, p) in tt.iter().enumerate() {
            assert_eq!(*p, dp(&f, &row["hecke_targets"][i]));
        }
        for v in row["bases"].as_array().unwrap() {
            let c = v["scale"].as_u64().unwrap();
            let t = v["complement"].as_u64().unwrap();
            let ci = (1..11).find(|v| v * c % 11 == 1).unwrap();
            assert_eq!(r.cp(&dp(&f, &v["P"])), r.times(&p, c));
            assert_eq!(
                r.cp(&dp(&f, &v["Q"])),
                r.plus(&r.times(&q, ci), &r.times(&p, t))
            );
            let indices: Vec<_> = (0..12).map(|a| line_map(c, 0, t, ci, a)).collect();
            positive.push(json!({"neighbor":n,"scale":c,"complement":t,"target_indices":indices,"target_traces":indices.iter().map(|i|pj(&tt[*i])).collect::<Vec<_>>()}));
        }
        println!("N4 positive neighbor{n}:12 targets reconstructed independently");
    }
    for row in ns["neighbors"].as_array().unwrap() {
        let n = row["neighbor"].as_u64().unwrap() as usize;
        let data = round04::read(&dir.join(format!("r3b-negative-fiber-neighbor-{n:02}.json")));
        let mut components = vec![];
        for cp in data["paired_components"].as_array().unwrap() {
            let modulus = poly(&f, &cp["Q_x_minpoly"]);
            let r = R {
                f: &f,
                p: modulus.clone(),
                a: decode(&f, &row["source_a"]),
                b: decode(&f, &row["source_b"]),
                fx: vec![],
            };
            let p = r.cp(&dp(&f, &cp["P"]));
            let q = AP::Pt(
                ca(&r, &vec![f.z(), f.o()]),
                ca(&r, &poly(&f, &cp["Q_y_polynomial"])),
            );
            let tt = targets(
                &f,
                &modulus,
                &r.a,
                &r.b,
                &poly(&f, &row["source_first7_roots"]),
                &poly(&f, &row["source_second7_roots"]),
                &p,
                &q,
            );
            let eroot = decode(&f, &ns["F32_embedding_root"]);
            let expected = P::Affine {
                x: f.embed_bits(ns["target_F32"][0].as_u64().unwrap(), &eroot),
                y: f.embed_bits(ns["target_F32"][1].as_u64().unwrap(), &eroot),
            };
            assert_eq!(constant(&f, &tt[0]), expected);
            let traces: Vec<_> = tt.iter().map(|p| trace(&f, &modulus, p)).collect();
            let rec = json!({"seed_component":negative.len(),"neighbor":n,"P_scale":cp["P_scale"],"degree":11,"modulus":polyj(&modulus),"targets":tt.iter().map(ap_json).collect::<Vec<_>>(),"target_traces":traces.iter().map(pj).collect::<Vec<_>>()});
            negative.push(rec.clone());
            components.push(rec);
        }
        round02::save(
            dir,
            &format!("n4-negative-neighbor-{n:02}.json"),
            &json!({"components":components}),
        );
        println!("N4 negative neighbor{n}:60 target algebras and traces checked");
    }
    round02::save(
        dir,
        "n4-independent-seed-targets.json",
        &json!({"schema":"koblitz.round07/n4-seeds-v1","positive":positive,"negative":negative,"scope":"actual Velu transport, commuting squares, modular coordinates and finite-algebra point traces; no Gram matrix"}),
    );
}
pub fn even(dir: &Path) {
    let f = F::new(120, 27);
    let ec = f.curve(f.o(), f.o());
    let cache = round04::read(&dir.join("n4-independent-seed-targets.json"));
    let input = round04::read(&dir.join("r3d-even-reverse-divisor.json"));
    let pos = round04::read(&dir.join("r3b-complete-split-fiber.json"));
    let ns = round04::read(&dir.join("r3-reverse-fiber-screen-negative.json"));
    let mut totals = vec![P::Infinity; 12];
    let mut terms = vec![];
    let mut checked = 0;
    for term in input["terms"].as_array().unwrap() {
        let negative = term["seed"] == "negative";
        let j = term["seed_frobenius_power"].as_u64().unwrap() as u32;
        let a = term["line"].as_u64().unwrap();
        let ai = (1..11).find(|i| i * a % 11 == 1).unwrap();
        let invtwo = (1..11).find(|i| i * 2u64.pow(j) % 11 == 1).unwrap();
        let perm: Vec<_> = (0..12)
            .map(|i| {
                let k = line_map(0, 11 - a, ai, 1, i);
                if k == 11 {
                    11
                } else {
                    (k as u64 * invtwo % 11) as usize
                }
            })
            .collect();
        let mut sum = vec![P::Infinity; 12];
        for cp in term["components"].as_array().unwrap() {
            let ix = cp["seed_component"].as_u64().unwrap() as usize;
            let seed = &cache[if negative { "negative" } else { "positive" }][ix];
            let n = seed["neighbor"].as_u64().unwrap() as usize;
            let r = R {
                f: &f,
                p: poly(&f, &cp["field_modulus_over_F120"]),
                a: decode(&f, &cp["source_a"]),
                b: decode(&f, &cp["source_b"]),
                fx: vec![],
            };
            let (p, q) = if negative {
                let raw =
                    round04::read(&dir.join(format!("r3b-negative-fiber-neighbor-{n:02}.json")));
                let z = &raw["paired_components"][ix % 5];
                (
                    r.cp(&dp(&f, &z["P"])),
                    AP::Pt(
                        ca(&r, &vec![f.z(), f.o()]),
                        ca(&r, &poly(&f, &z["Q_y_polynomial"])),
                    ),
                )
            } else {
                let z = &pos["neighbors"][n]["bases"][ix % 55];
                (r.cp(&dp(&f, &z["P"])), r.cp(&dp(&f, &z["Q"])))
            };
            let pp = frob(&r, &p, j);
            let qq = r.times(&frob(&r, &q, j), invtwo);
            assert_eq!(ap(&r, &cp["P_x"], &cp["P_y"]), r.neg(&r.times(&qq, a)));
            assert_eq!(
                ap(&r, &cp["Q_x"], &cp["Q_y"]),
                r.plus(&r.times(&pp, ai), &qq)
            );
            assert_eq!(
                dp(&f, &cp["source_modular_point"]),
                f.frobp(
                    &dp(
                        &f,
                        &if negative {
                            ns["neighbors"][n]["source_modular_point"].clone()
                        } else {
                            pos["neighbors"][n]["source_modular_point"].clone()
                        }
                    ),
                    j
                )
            );
            for k in 0..12 {
                let p = f.frobp(&dp(&f, &seed["target_traces"][perm[k]]), j);
                sum[k] = plus(&ec, &sum[k], &p);
            }
            checked += 1;
        }
        let coefficient = term["coefficient"].as_i64().unwrap();
        for k in 0..12 {
            totals[k] = plus(
                &ec,
                &totals[k],
                &if coefficient < 0 {
                    neg(&sum[k])
                } else {
                    sum[k].clone()
                },
            );
        }
        terms.push(json!({"line":a,"coefficient":coefficient,"target_permutation":perm,"pointwise_target_norms":sum.iter().map(pj).collect::<Vec<_>>()}));
    }
    let image = tuple(&ec, &totals, false);
    let source: Vec<_> = input["source_A5_point"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| dp(&f, p))
        .collect();
    assert_eq!(image, source);
    assert_eq!(
        image,
        input["forward_image_via_proven_Gram"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| dp(&f, p))
            .collect::<Vec<_>>()
    );
    round02::save(
        dir,
        "n4-even-independent-forward.json",
        &json!({"schema":"koblitz.round07/n4-even-v1","checked_coordinate_entries":checked,"terms":terms,"pointwise_forward12":totals.iter().map(pj).collect::<Vec<_>>(),"image":image.iter().map(pj).collect::<Vec<_>>(),"equals_source":true,"matches_prior_Gram_receipt":true,"Gram_used_to_compute_norms":false,"scope":"complete finite-algebra pointwise norms, exact deck/Frobenius target permutations and parsed-coordinate checks; no Jacobian reduction"}),
    );
    println!(
        "N4:all{checked} reverse entries checked; independent pointwise forward equals source"
    );
}
fn tfrob(f: &F, t: &[P], odd: bool) -> Vec<P> {
    (0..5)
        .map(|i| {
            let p = f.frobp(&t[(i + 4) % 5], 1);
            if odd {
                neg(&p)
            } else {
                p
            }
        })
        .collect()
}
fn closed_image(f: &F, values: &[Vec<P>], members: &[usize], odd: bool) -> Vec<P> {
    let ec = f.curve(f.o(), f.o());
    let mut p: Vec<_> = (0..5)
        .map(|i| {
            members
                .iter()
                .fold(P::Infinity, |z, j| plus(&ec, &z, &values[*j][i]))
        })
        .collect();
    let mut total = vec![P::Infinity; 5];
    for _ in 0..5 {
        for i in 0..5 {
            total[i] = plus(&ec, &total[i], &p[i]);
        }
        p = tfrob(f, &p, odd);
    }
    assert_eq!(tfrob(f, &total, odd), total);
    assert!(total.iter().all(|p| times(&ec, p, 11) == P::Infinity));
    total
}
fn orbits(succ: &[usize]) -> Vec<Vec<usize>> {
    let mut seen = std::collections::BTreeSet::new();
    let mut out = vec![];
    for i in 0..succ.len() {
        if seen.contains(&i) {
            continue;
        }
        let mut j = i;
        let mut v = vec![];
        loop {
            assert!(seen.insert(j));
            v.push(j);
            j = succ[j];
            if j == i {
                break;
            }
        }
        out.push(v);
    }
    out
}
pub fn relations(dir: &Path) {
    use std::collections::BTreeMap;
    let f = F::new(120, 27);
    let ec = f.curve(f.o(), f.o());
    let pos = round04::read(&dir.join("r3b-complete-split-fiber.json"));
    let ns = round04::read(&dir.join("r3-reverse-fiber-screen-negative.json"));
    let cache = round04::read(&dir.join("n4-independent-seed-targets.json"));
    let mut columns = vec![];
    let mut summaries = vec![];
    for negative in [false, true] {
        let name = if negative { "negative" } else { "positive" };
        let seeds = cache[name].as_array().unwrap();
        let mut succ = vec![];
        if !negative {
            let mut pts = vec![];
            let mut keys = BTreeMap::new();
            for row in pos["neighbors"].as_array().unwrap() {
                for b in row["bases"].as_array().unwrap() {
                    let base = dp(&f, &row["source_modular_point"]);
                    let p = dp(&f, &b["P"]);
                    let q = dp(&f, &b["Q"]);
                    keys.insert(crate::reverse06::basiskey(&base, &p, &q), pts.len());
                    pts.push((base, p, q));
                }
            }
            for (base, p, q) in pts {
                succ.push(
                    keys[&crate::reverse06::basiskey(
                        &f.frobp(&base, 5),
                        &f.frobp(&p, 5),
                        &neg(&f.frobp(&q, 5)),
                    )],
                );
            }
        } else {
            let mut cp = vec![];
            for row in ns["neighbors"].as_array().unwrap() {
                let n = row["neighbor"].as_u64().unwrap();
                let data =
                    round04::read(&dir.join(format!("r3b-negative-fiber-neighbor-{n:02}.json")));
                for z in data["paired_components"].as_array().unwrap() {
                    cp.push((
                        dp(&f, &row["source_modular_point"]),
                        dp(&f, &z["P"]),
                        poly(&f, &z["Q_x_minpoly"]),
                        poly(&f, &z["Q_y_polynomial"]),
                    ));
                }
            }
            for (base, p, g, y) in &cp {
                let nb = f.frobp(base, 5);
                let np = f.frobp(p, 5);
                let ng: Poly = g.iter().map(|v| f.frob(v, 5)).collect();
                let ny = add(
                    &y.iter().map(|v| f.frob(v, 5)).collect::<Poly>(),
                    &[f.z(), f.o()],
                );
                let index = cp
                    .iter()
                    .position(|(b, p, g, y)| {
                        *b == nb
                            && *g == ng
                            && ((*p == np && *y == ny)
                                || (*p == neg(&np) && *y == add(&ny, &[f.z(), f.o()])))
                    })
                    .expect("etale component descent");
                succ.push(index);
            }
        }
        let cycles = orbits(&succ);
        let mut allvalues = vec![];
        for odd in [false, true] {
            allvalues.push(
                seeds
                    .iter()
                    .map(|s| {
                        tuple(
                            &ec,
                            &s["target_traces"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|p| dp(&f, p))
                                .collect::<Vec<_>>(),
                            odd,
                        )
                    })
                    .collect::<Vec<_>>(),
            );
        }
        let mut nonzero = [0, 0];
        for cycle in &cycles {
            let degree = 5 * cycle.len() * if negative { 11 } else { 1 };
            let mut images = vec![];
            for sign in 0..2 {
                let t = closed_image(&f, &allvalues[sign], cycle, sign == 1);
                let nz = t.iter().any(|p| *p != P::Infinity);
                if nz {
                    nonzero[sign] += 1;
                }
                images.push(json!({"a":if sign==0{1}else{0},"nonzero":nz,"tuple":t.iter().map(pj).collect::<Vec<_>>()}));
            }
            columns.push(
                json!({"seed":name,"members_over_F32":cycle,"degree":degree,"images":images}),
            );
        }
        summaries.push(json!({"seed":name,"successor":succ,"cycles":cycles.iter().map(|v|v.len()).collect::<Vec<_>>(),"nonzero_columns_by_sign":nonzero}));
    }
    assert_eq!(
        columns
            .iter()
            .map(|c| c["degree"].as_u64().unwrap())
            .sum::<u64>(),
        6600
    );
    let mut sums = vec![vec![P::Infinity; 5]; 2];
    for c in &columns {
        assert!(
            ![6, 30, 210].contains(&c["degree"].as_u64().unwrap()),
            "column identity overlap needs explicit join"
        );
        for s in 0..2 {
            for i in 0..5 {
                sums[s][i] = plus(&ec, &sums[s][i], &dp(&f, &c["images"][s]["tuple"][i]));
            }
        }
    }
    assert!(sums.iter().flatten().all(|p| *p == P::Infinity));
    // All eight cusps of X0(49) are F8-rational and killed by14.
    let sf = round04::Field::find(3);
    let se = round04::Curve { f: sf, a: 1, b: 1 };
    let cusps: Vec<_> = se
        .points()
        .into_iter()
        .filter(|p| crate::modular_geometry::triple(sf, *p).is_none())
        .collect();
    assert_eq!(cusps.len(), 8);
    assert!(cusps.iter().all(|p| se.times(*p, 14) == round04::Pt::O));
    let old = round04::read(&dir.join("m4-direct-modular-relations.json"));
    let n = columns.len();
    let mut matrix: Vec<Vec<num_bigint::BigInt>> = old["principal_relations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let mut v: Vec<_> = r["row_mod11"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| num_bigint::BigInt::from(v.as_u64().unwrap()))
                .collect();
            v.extend(vec![num_bigint::BigInt::from(0); 44 + n]);
            v
        })
        .collect();
    let mut previous = vec![num_bigint::BigInt::from(0); 544];
    previous.extend(vec![num_bigint::BigInt::from(1); 44]);
    previous.extend(vec![num_bigint::BigInt::from(0); n]);
    matrix.push(previous);
    let mut row = vec![num_bigint::BigInt::from(0); 588];
    row.extend(vec![num_bigint::BigInt::from(1); n]);
    matrix.push(row);
    let rank = round02::rank(matrix.clone(), &num_bigint::BigInt::from(11));
    let mut nonzero_ranks = vec![];
    for s in 0..2 {
        let mut selected: Vec<_> = old["closed_points"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_, c)| c["images"][s]["nonzero"] == true)
            .map(|(i, _)| i)
            .collect();
        selected.extend(
            columns
                .iter()
                .enumerate()
                .filter(|(_, c)| c["images"][s]["nonzero"] == true)
                .map(|(i, _)| 588 + i),
        );
        let sub = matrix
            .iter()
            .map(|r| selected.iter().map(|i| r[*i].clone()).collect())
            .collect();
        nonzero_ranks.push(json!({"a":if s==0{1}else{0},"columns":selected.len(),"rank":round02::rank(sub,&num_bigint::BigInt::from(11))}));
    }
    round02::save(
        dir,
        "m8-full-map-principal-row.json",
        &json!({"schema":"koblitz.round07/m8-v1","function":"Norm_F32/F2(x(phi0)+x(P0))","zero_degree":6600,"pole_divisor":"10 phi0^*(O), degree6600; all target cusp images killed by14 and11, hence zero","cusp_points_F8":cusps.iter().map(|p|p.json()).collect::<Vec<_>>(),"descent":summaries,"columns":columns,"total_evaluated_columns":588+n,"evaluated_rank":rank,"nonzero_image_matrices":nonzero_ranks,"principal_sums":sums.iter().map(|t|t.iter().map(pj).collect::<Vec<_>>()).collect::<Vec<_>>(),"scope":"complete ordinary zero fibers and written modular cusp bridge; evaluated closed images; native exact rank; no individual descent"}),
    );
    println!(
        "M8:{} new closed columns; evaluated rank{rank}/{}; nonzero matrices{}",
        n,
        588 + n,
        json!(nonzero_ranks)
    );
}
pub fn verify(dir: &Path) {
    let input = round04::read(&dir.join("m8-full-map-principal-row.json"));
    let seed = round04::read(&dir.join("r3b-complete-split-fiber.json"));
    let f = F::new(120, 27);
    let sf = round04::Field::find(5);
    let root = decode(&f, &seed["F32_embedding_root"]);
    let em: Vec<_> = (0..32).map(|x| f.embed_bits(x, &root)).collect();
    let twist = f.quadratic(&f.o()).unwrap();
    let mut small = vec![vec![]; 2];
    let mut counts = vec![];
    for sign in 0..2 {
        let ec = round04::Curve {
            f: sf,
            a: if sign == 0 { 1 } else { 0 },
            b: 1,
        };
        for col in input["columns"].as_array().unwrap() {
            let p = dp(&f, &col["images"][sign]["tuple"][0]);
            let p = match p {
                P::Infinity => round04::Pt::O,
                P::Affine { x, y } => {
                    let y = if sign == 1 {
                        y.add(&f.mul(&twist, &x))
                    } else {
                        y
                    };
                    round04::Pt::A(
                        em.iter().position(|v| *v == x).unwrap() as u32,
                        em.iter().position(|v| *v == y).unwrap() as u32,
                    )
                }
            };
            assert!(ec.valid(p));
            assert_eq!(ec.times(p, 11), round04::Pt::O);
            assert_eq!(
                p != round04::Pt::O,
                col["images"][sign]["nonzero"].as_bool().unwrap()
            );
            small[sign].push(p);
        }
        assert_eq!(
            small[sign]
                .iter()
                .fold(round04::Pt::O, |a, p| ec.add(a, *p)),
            round04::Pt::O
        );
        counts.push(small[sign].iter().filter(|p| **p != round04::Pt::O).count());
    }
    let ec = round04::Curve { f: sf, a: 1, b: 1 };
    let p = match seed["target_F32"].as_array() {
        Some(a) => round04::Pt::A(a[0].as_u64().unwrap() as u32, a[1].as_u64().unwrap() as u32),
        None => panic!(),
    };
    let round04::Pt::A(x, _) = p else {
        unreachable!()
    };
    let mut polyq = vec![1];
    let mut y = x;
    for _ in 0..5 {
        polyq = round04::mul(sf, &polyq, &[y, 1]);
        y = sf.sq(y);
    }
    assert_eq!(y, x);
    assert!(polyq.iter().all(|c| *c <= 1));
    assert_eq!(polyq.len(), 6);
    let all = ec.points();
    let zeros: Vec<_> = all
        .iter()
        .filter(|p| match p {
            round04::Pt::O => false,
            round04::Pt::A(x, _) => sf.eval(&polyq, *x) == 0,
        })
        .copied()
        .collect();
    assert_eq!(zeros.len(), 10);
    assert!(zeros.iter().all(|p| ec.times(*p, 11) == round04::Pt::O));
    round02::save(
        dir,
        "m8b-independent-F32-check.json",
        &json!({"schema":"koblitz.round07/m8b-v1","quintic_coefficients_F2":polyq,"quintic_zeros_E49_F32":zeros.iter().map(|p|p.json()).collect::<Vec<_>>(),"new_closed_images_in_F32":small.iter().map(|v|v.iter().map(|p|p.json()).collect::<Vec<_>>()).collect::<Vec<_>>(),"nonzero_counts":counts,"principal_sums_checked_with_u32_group_law":true,"scope":"separate small-field arithmetic for all new closed images, row sums and the exact quintic; shares no large-field curve additions"}),
    );
    println!("M8b:quintic{}, all60 closed image coordinates and both principal sums checked via u32 arithmetic",json!(polyq));
}
pub fn relations_next(dir: &Path) {
    use num_bigint::BigInt;
    use std::collections::{BTreeMap, BTreeSet};
    let f = F::new(120, 27);
    let ec = f.curve(f.o(), f.o());
    let pos = round04::read(&dir.join("r3b-complete-split-fiber.json"));
    let prior = round04::read(&dir.join("m8-full-map-principal-row.json"));
    let cache = round04::read(&dir.join("n4-independent-seed-targets.json"));
    let mut points = vec![];
    for r in pos["neighbors"].as_array().unwrap() {
        for b in r["bases"].as_array().unwrap() {
            points.push((
                dp(&f, &r["source_modular_point"]),
                decode(&f, &r["source_a"]),
                decode(&f, &r["source_b"]),
                dp(&f, &b["P"]),
                dp(&f, &b["Q"]),
            ));
        }
    }
    let transformed = |ix: usize, j: u32, deck: bool| {
        let (base, a, b, p, q) = &points[ix];
        let cur = f.curve(f.frob(a, j), f.frob(b, j));
        let pp = f.frobp(p, j);
        let qq = times(
            &cur,
            &f.frobp(q, j),
            (1..11).find(|v| v * 2u64.pow(j) % 11 == 1).unwrap(),
        );
        let (p, q) = if deck {
            let line = 2u64.pow(j) % 11;
            (
                neg(&times(&cur, &qq, line)),
                plus(
                    &cur,
                    &times(&cur, &pp, (1..11).find(|v| v * line % 11 == 1).unwrap()),
                    &qq,
                ),
            )
        } else {
            (pp, qq)
        };
        assert!(cur.is_on_curve(&p) && cur.is_on_curve(&q));
        crate::reverse06::basiskey(&f.frobp(base, j), &p, &q)
    };
    let mut keymap: BTreeMap<String, usize> = BTreeMap::new();
    for (i, c) in prior["columns"].as_array().unwrap().iter().enumerate() {
        if c["seed"] != "positive" {
            continue;
        }
        for ix in c["members_over_F32"].as_array().unwrap() {
            for j in 0..5 {
                let key = transformed(ix.as_u64().unwrap() as usize, j, false);
                assert!(keymap.insert(key, 588 + i).is_none());
            }
        }
    }
    let mut newcols = vec![];
    let mut row: BTreeMap<usize, u64> = BTreeMap::new();
    let mut orbit_records = vec![];
    let mut principal = vec![vec![P::Infinity; 5]; 2];
    let mut overlaps = 0;
    let mut nonconstant_checks = 0;
    for negative in [false, true] {
        let name = if negative { "negative" } else { "positive" };
        let succ: Vec<usize> = serde_json::from_value(
            prior["descent"][if negative { 1 } else { 0 }]["successor"].clone(),
        )
        .unwrap();
        let squared: Vec<_> = succ.iter().map(|i| succ[*i]).collect();
        let cycles = orbits(&squared);
        let seeds = cache[name].as_array().unwrap();
        let mut values = vec![];
        for odd in [false, true] {
            values.push(
                seeds
                    .iter()
                    .map(|seed| {
                        let tt: Vec<_> = (0..12)
                            .map(|i| dp(&f, &seed["target_traces"][line_map(0, 10, 1, 1, i)]))
                            .collect();
                        tuple(&ec, &tt, odd)
                    })
                    .collect::<Vec<_>>(),
            );
        }
        for cycle in cycles {
            let degree = 10 * cycle.len() * if negative { 11 } else { 1 };
            let mut images = vec![];
            for sign in 0..2 {
                let mut z: Vec<_> = (0..5)
                    .map(|i| {
                        cycle
                            .iter()
                            .fold(P::Infinity, |z, j| plus(&ec, &z, &values[sign][*j][i]))
                    })
                    .collect();
                let mut t = vec![P::Infinity; 5];
                for _ in 0..10 {
                    for i in 0..5 {
                        t[i] = plus(&ec, &t[i], &z[i]);
                    }
                    z = tfrob(&f, &z, sign == 1);
                }
                assert_eq!(tfrob(&f, &t, sign == 1), t);
                assert!(t.iter().all(|p| times(&ec, p, 11) == P::Infinity));
                for i in 0..5 {
                    principal[sign][i] = plus(&ec, &principal[sign][i], &t[i]);
                }
                images.push(json!({"a":if sign==0{1}else{0},"nonzero":t.iter().any(|p|*p!=P::Infinity),"tuple":t.iter().map(pj).collect::<Vec<_>>()}));
            }
            let index = if !negative {
                let mut keys = BTreeSet::new();
                for ix in &cycle {
                    for j in 0..10 {
                        assert!(
                            keys.insert(transformed(*ix, j, true)),
                            "unexpected repeated geometric point"
                        );
                    }
                }
                assert_eq!(keys.len(), degree);
                let old: BTreeSet<_> = keys.iter().filter_map(|k| keymap.get(k).copied()).collect();
                if !old.is_empty() {
                    assert_eq!(old.len(), 1);
                    let index = *old.first().unwrap();
                    assert!(keys.iter().all(|k| keymap.get(k) == Some(&index)));
                    let c = if index < 648 {
                        &prior["columns"][index - 588]
                    } else {
                        &newcols[index - 648]
                    };
                    assert_eq!(c["degree"], json!(degree));
                    assert_eq!(c["images"], json!(images));
                    overlaps += 1;
                    index
                } else {
                    let index = 648 + newcols.len();
                    for key in keys {
                        keymap.insert(key, index);
                    }
                    newcols.push(json!({"seed":name,"degree":degree,"members_over_F1024":cycle,"images":images}));
                    index
                }
            } else {
                assert_eq!(degree, 660);
                for ix in &cycle {
                    let seed = &seeds[*ix];
                    let n = seed["neighbor"].as_u64().unwrap();
                    let raw = round04::read(
                        &dir.join(format!("r3b-negative-fiber-neighbor-{n:02}.json")),
                    );
                    let c = &raw["paired_components"][*ix % 5];
                    let r = R {
                        f: &f,
                        p: poly(&f, &c["Q_x_minpoly"]),
                        a: decode(&f, &raw["source_a"]),
                        b: decode(&f, &raw["source_b"]),
                        fx: vec![],
                    };
                    let pp = r.cp(&dp(&f, &c["P"]));
                    let qq = AP::Pt(
                        ca(&r, &vec![f.z(), f.o()]),
                        ca(&r, &poly(&f, &c["Q_y_polynomial"])),
                    );
                    let mix = r.plus(&r.neg(&pp), &r.times(&qq, 2));
                    assert!(ax(&mix).u.len() > 1);
                    nonconstant_checks += 1;
                }
                let index = 648 + newcols.len();
                newcols.push(json!({"seed":name,"degree":degree,"members_over_F1024":cycle,"images":images,"disjointness":"nonconstant P-coordinate and nonconstant -P+2Q exclude old rational-P columns and the half-period identification"}));
                index
            };
            *row.entry(index).or_default() += 1;
            orbit_records.push(json!({"seed":name,"members":cycle,"degree":degree,"column":index}));
        }
    }
    assert!(principal.iter().flatten().all(|p| *p == P::Infinity));
    assert_eq!(
        orbit_records
            .iter()
            .map(|r| r["degree"].as_u64().unwrap())
            .sum::<u64>(),
        13200
    );
    let old = round04::read(&dir.join("m4-direct-modular-relations.json"));
    let n = 648 + newcols.len();
    let mut matrix: Vec<Vec<BigInt>> = old["principal_relations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let mut z: Vec<_> = r["row_mod11"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| BigInt::from(v.as_u64().unwrap()))
                .collect();
            z.resize(n, BigInt::from(0));
            z
        })
        .collect();
    let mut r = vec![BigInt::from(0); n];
    for i in 544..588 {
        r[i] = BigInt::from(1);
    }
    matrix.push(r);
    let mut r = vec![BigInt::from(0); n];
    for i in 588..648 {
        r[i] = BigInt::from(1);
    }
    matrix.push(r);
    let mut r = vec![BigInt::from(0); n];
    for (i, c) in &row {
        r[*i] = BigInt::from(*c);
    }
    matrix.push(r);
    let rank = round02::rank(matrix.clone(), &BigInt::from(11));
    let mut nonzero = vec![];
    for s in 0..2 {
        let mut indices: Vec<_> = old["closed_points"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_, c)| c["images"][s]["nonzero"] == true)
            .map(|(i, _)| i)
            .collect();
        indices.extend(
            prior["columns"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .filter(|(_, c)| c["images"][s]["nonzero"] == true)
                .map(|(i, _)| 588 + i),
        );
        indices.extend(
            newcols
                .iter()
                .enumerate()
                .filter(|(_, c)| c["images"][s]["nonzero"] == true)
                .map(|(i, _)| 648 + i),
        );
        let sub = matrix
            .iter()
            .map(|r| indices.iter().map(|i| r[*i].clone()).collect())
            .collect();
        nonzero.push(json!({"a":if s==0{1}else{0},"columns":indices.len(),"rank":round02::rank(sub,&BigInt::from(11))}));
    }
    round02::save(
        dir,
        "m9-second-full-map-row.json",
        &json!({"schema":"koblitz.round07/m9-v1","function":"Norm_F1024/F2(x(phi1)+x(P0))","zero_degree":13200,"pole_degree":13200,"pole_images":"zero via cusp14-torsion and target11-torsion","new_columns":newcols,"orbit_records":orbit_records,"positive_overlapping_orbits":overlaps,"negative_nonconstant_coordinate_checks":nonconstant_checks,"row":row,"total_columns":n,"evaluated_rank":rank,"nonzero_image_matrices":nonzero,"principal_sums":principal.iter().map(|t|t.iter().map(pj).collect::<Vec<_>>()).collect::<Vec<_>>(),"scope":"complete norm support, pointwise closed images and verified geometric split-column joins; etale disjointness checked by nonconstant coordinates with written Galois bridge"}),
    );
    println!(
        "M9:new{} columns, joined{overlaps}; evaluated rank{rank}/{n}; nonzero matrices{}",
        newcols.len(),
        json!(nonzero)
    );
}
pub fn verify_next(dir: &Path) {
    let r = round04::read(&dir.join("m9-second-full-map-row.json"));
    let m8 = round04::read(&dir.join("m8-full-map-principal-row.json"));
    let base = round04::read(&dir.join("r3b-complete-split-fiber.json"));
    let old = round04::read(&dir.join("m4-direct-modular-relations.json"));
    let f = F::new(120, 27);
    let root = decode(&f, &base["F32_embedding_root"]);
    let em: Vec<_> = (0..32).map(|x| f.embed_bits(x, &root)).collect();
    let twist = f.quadratic(&f.o()).unwrap();
    let sf = round04::Field::find(5);
    let mut checks = vec![];
    for sign in 0..2 {
        let ec = round04::Curve {
            f: sf,
            a: if sign == 0 { 1 } else { 0 },
            b: 1,
        };
        let conv = |v: &Value| match dp(&f, v) {
            P::Infinity => round04::Pt::O,
            P::Affine { x, y } => {
                let y = if sign == 1 {
                    y.add(&f.mul(&twist, &x))
                } else {
                    y
                };
                round04::Pt::A(
                    em.iter().position(|v| *v == x).unwrap() as u32,
                    em.iter().position(|v| *v == y).unwrap() as u32,
                )
            }
        };
        let mut points = vec![];
        for c in r["new_columns"].as_array().unwrap() {
            let p = conv(&c["images"][sign]["tuple"][0]);
            assert!(ec.valid(p));
            assert_eq!(ec.times(p, 11), round04::Pt::O);
            assert_eq!(
                p != round04::Pt::O,
                c["images"][sign]["nonzero"].as_bool().unwrap()
            );
            points.push(p);
        }
        let mut sum = round04::Pt::O;
        for (index, weight) in r["row"].as_object().unwrap() {
            let index = index.parse::<usize>().unwrap();
            assert!(index >= 588);
            let p = if index < 648 {
                conv(&m8["columns"][index - 588]["images"][sign]["tuple"][0])
            } else {
                points[index - 648]
            };
            sum = ec.add(sum, ec.times(p, weight.as_u64().unwrap()));
        }
        assert_eq!(sum, round04::Pt::O);
        checks.push(json!({"a":if sign==0{1}else{0},"new_nonzero":points.iter().filter(|p|**p!=round04::Pt::O).count(),"images":points.iter().map(|p|p.json()).collect::<Vec<_>>()}));
    }
    let n = r["total_columns"].as_u64().unwrap() as usize;
    let mut matrix: Vec<Vec<u64>> = old["principal_relations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let mut v: Vec<_> = r["row_mod11"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap())
                .collect();
            v.resize(n, 0);
            v
        })
        .collect();
    let mut row = vec![0; n];
    row[544..588].fill(1);
    matrix.push(row);
    let mut row = vec![0; n];
    row[588..648].fill(1);
    matrix.push(row);
    let mut row = vec![0; n];
    for (k, v) in r["row"].as_object().unwrap() {
        row[k.parse::<usize>().unwrap()] = v.as_u64().unwrap() % 11;
    }
    matrix.push(row);
    let rank = crate::gluing::rref(matrix.clone(), 11).1.len();
    assert_eq!(rank, r["evaluated_rank"].as_u64().unwrap() as usize);
    let mut ranks = vec![];
    for s in 0..2 {
        let mut selected: Vec<_> = old["closed_points"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_, c)| c["images"][s]["nonzero"] == true)
            .map(|(i, _)| i)
            .collect();
        selected.extend(
            m8["columns"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .filter(|(_, c)| c["images"][s]["nonzero"] == true)
                .map(|(i, _)| 588 + i),
        );
        selected.extend(
            r["new_columns"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .filter(|(_, c)| c["images"][s]["nonzero"] == true)
                .map(|(i, _)| 648 + i),
        );
        let sub = matrix
            .iter()
            .map(|r| selected.iter().map(|i| r[*i]).collect())
            .collect();
        let k = crate::gluing::rref(sub, 11).1.len();
        assert_eq!(
            k,
            r["nonzero_image_matrices"][s]["rank"].as_u64().unwrap() as usize
        );
        ranks.push(k);
    }
    round02::save(
        dir,
        "m9b-independent-small-field-check.json",
        &json!({"schema":"koblitz.round07/m9b-v1","new_F32_image_checks":checks,"u32_rank":rank,"u32_nonzero_ranks":ranks,"scope":"separate u32 finite-field group law, complete principal row sums, and u64 row reduction against BigInt rank records"}),
    );
    println!("M9b:all120 new images and row sums checked via F32; separate modular ranks{rank}, {ranks:?}");
}
pub fn verify_bases(dir: &Path) {
    use std::collections::BTreeSet;
    let f = F::new(120, 27);
    let mut sources = vec![];
    for (name, file) in [
        ("positive", "r3-reverse-fiber-screen.json"),
        ("negative", "r3-reverse-fiber-screen-negative.json"),
    ] {
        let input = round04::read(&dir.join(file));
        let points: Vec<_> = input["neighbors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| dp(&f, &r["source_modular_point"]))
            .collect();
        let mut phases = vec![];
        let mut union = BTreeSet::new();
        for j in 0..5 {
            let set: BTreeSet<_> = points
                .iter()
                .map(|p| pj(&f.frobp(p, j)).to_string())
                .collect();
            assert_eq!(set.len(), 12);
            assert!(set.iter().all(|p| !union.contains(p)));
            union.extend(set.iter().cloned());
            phases.push(set.into_iter().collect::<Vec<_>>());
        }
        assert_eq!(union.len(), 60);
        for p in &points {
            assert_eq!((1..=120).find(|j| f.frobp(p, *j) == *p), Some(60));
        }
        sources.push(json!({"seed":name,"all12_source_points_have_degree":60,"disjoint_five_phase_sets":phases}));
    }
    let input = round04::read(&dir.join("r3-reverse-fiber-screen-negative.json"));
    let mut steps = vec![];
    for row in input["neighbors"].as_array().unwrap() {
        let n = row["neighbor"].as_u64().unwrap();
        let cp = round04::read(&dir.join(format!("r3b-negative-fiber-neighbor-{n:02}.json")));
        for s in cp["paired_components"].as_array().unwrap() {
            let r = R {
                f: &f,
                p: poly(&f, &s["Q_x_minpoly"]),
                a: decode(&f, &row["source_a"]),
                b: decode(&f, &row["source_b"]),
                fx: vec![],
            };
            let p = r.cp(&dp(&f, &s["P"]));
            let q = AP::Pt(
                ca(&r, &vec![f.z(), f.o()]),
                ca(&r, &poly(&f, &s["Q_y_polynomial"])),
            );
            let fq = frob(&r, &q, 120);
            let delta = r.plus(&fq, &r.neg(&q));
            let step = (1..11)
                .find(|i| r.times(&p, *i) == delta)
                .expect("nontrivial even-seed complement Galois translation");
            let mix = r.plus(&r.neg(&p), &r.times(&q, 2));
            assert!(ax(&mix).u.len() > 1);
            steps.push(json!({"neighbor":n,"P_scale":s["P_scale"],"Galois_complement_step":step,"half_period_x_nonconstant":true}));
        }
    }
    assert_eq!(steps.len(), 60);
    round02::save(
        dir,
        "m9c-geometric-base-check.json",
        &json!({"schema":"koblitz.round07/m9c-v1","source_base_checks":sources,"negative_Galois_steps":steps,"scope":"exact source degrees, disjoint Frobenius phase sets, and all60 nontrivial degree11 complement translations; supports closed-degree and no-overlap arguments"}),
    );
    println!("M9c:all source bases degree60, five phases disjoint, all60 complement Galois translations checked");
}
