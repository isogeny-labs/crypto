//! Odd reverse seed and antisymmetric divisor.
use crate::{large_field::*, reverse06::*, round02, round04};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use serde_json::json;
use std::path::Path;
pub fn screen(dir: &Path) {
    let s = round04::Field::find(10);
    let small = round04::Field::find(5);
    let ec = round04::Curve {
        f: small,
        a: 0,
        b: 1,
    };
    let p = ec
        .points()
        .into_iter()
        .find(|p| *p != round04::Pt::O && ec.times(*p, 11) == round04::Pt::O)
        .unwrap();
    let embedding = s.embed(small);
    let twist = s.quadratic(1).unwrap();
    let target = match p {
        round04::Pt::A(x, y) => {
            let x = embedding[x as usize];
            round04::Pt::A(x, embedding[y as usize] ^ s.mul(twist, x))
        }
        _ => unreachable!(),
    };
    let dest = round04::Curve { f: s, a: 1, b: 1 };
    assert!(dest.valid(target));
    assert_eq!(dest.frob(target, 1), dest.times(target, 7));
    assert_eq!(dest.frob(target, 5), dest.neg(target));
    let js = crate::modular_geometry::triple(s, target).unwrap();
    assert!(js.iter().all(|j| *j != 0));
    let b = s.inv(js[0]);
    let original = round04::Curve { f: s, a: 0, b };
    let t = 1025 - original.points().len() as i64;
    let mat = vec![
        vec![0u64, (11 - (1u64 << s.m) % 11) % 11],
        vec![1, t.rem_euclid(11) as u64],
    ];
    let mut a = vec![vec![1, 0], vec![0, 1]];
    let mut ord = 0;
    loop {
        let mut b = vec![vec![0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    b[i][j] = (b[i][j] + a[i][k] * mat[k][j]) % 11;
                }
            }
        }
        a = b;
        ord += 1;
        if a == vec![vec![1, 0], vec![0, 1]] {
            break;
        }
        assert!(ord < 133);
    }
    let m = num_integer::lcm(10 * ord, 30) as u32;
    assert!(m <= 1320, "odd full torsion field outside frozen bound");
    assert_eq!(m % (10 * ord as u32), 0);
    println!("R3: underlying trace{t}, Frobenius11 order{ord}, full torsion field degree{m}");
    let f = F::find(m);
    let root = f.root(s.modulus);
    let emb = |x: u32| f.embed_bits(x as u64, &root);
    let c = f.curve(f.z(), emb(b));
    let mut s0 = BigInt::from(2);
    let mut s1 = BigInt::from(t);
    for _ in 2..=m / 10 {
        let z = &s1 * t - &s0 * 1024;
        (s0, s1) = (s1, z);
    }
    let n = ((BigInt::one() << m) + BigInt::one() - s1)
        .to_biguint()
        .unwrap();
    let mut h = n.clone();
    while &h % 11u32 == BigUint::zero() {
        h /= 11u32;
    }
    let mut basis = vec![];
    for x in 1..4096 {
        let Some(p) = f.point(&c, &f.e(x)) else {
            continue;
        };
        assert_eq!(
            crate::binary_ecc::curve::scalar_mul(&c, &p, &n),
            P::Infinity,
            "full group order and embedding must match"
        );
        let mut p = crate::binary_ecc::curve::scalar_mul(&c, &p, &h);
        if p == P::Infinity {
            continue;
        }
        for _ in 0..m {
            if times(&c, &p, 11) == P::Infinity {
                break;
            }
            p = times(&c, &p, 11);
        }
        assert_eq!(times(&c, &p, 11), P::Infinity, "bounded primary extraction");
        if basis.is_empty() {
            basis.push(p);
        } else if !(0..11).any(|i| times(&c, &basis[0], i) == p) {
            basis.push(p);
            break;
        }
    }
    assert_eq!(basis.len(), 2);
    let zeta = (2..4096)
        .map(|i| f.pow(&f.e(i), &(((BigUint::one() << m) - 1u32) / 11u32)))
        .find(|v| *v != f.o())
        .unwrap();
    let val = weil(&f, &c, &basis[0], &basis[1]);
    let power = (1..11).find(|i| f.powu(&zeta, *i) == val).unwrap();
    let inv = (1..11).find(|i| i * power % 11 == 1).unwrap();
    basis[1] = times(&c, &basis[1], inv);
    assert_eq!(weil(&f, &c, &basis[0], &basis[1]), zeta);
    let k1 = kernels7_table(s, b, js[1]);
    let k2 = kernels7_table(s, s.inv(js[1]), js[2]);
    assert_eq!(k1.len(), 1);
    assert_eq!(k2.len(), 1);
    let first: Poly = k1[0].iter().map(|v| emb(*v)).collect();
    let second: Poly = k2[0].iter().map(|v| emb(*v)).collect();
    let first = f.roots(&first);
    let second = f.roots(&second);
    let mut rows = vec![];
    for i in 0..=11 {
        let g = if i == 11 {
            basis[1].clone()
        } else {
            plus(&c, &basis[0], &times(&c, &basis[1], i))
        };
        let comp = if i == 11 {
            basis[0].clone()
        } else {
            basis[1].clone()
        };
        let k: Vec<_> = (1..=5).map(|j| px(&times(&c, &g, j))).collect();
        let b0 = qb(&f, &c.b, &k);
        let transport7 = velu(&f, &k, &first);
        let transport11 = velu(&f, &first, &k);
        let second_new = velu(&f, &transport11, &second);
        let b70 = qb(&f, &b0, &transport7);
        let b490 = qb(&f, &b70, &second_new);
        let source = reconstruct(&f, &[f.inv(&b0), f.inv(&b70), f.inv(&b490)]);
        let dual = image_point(&f, &c, &g, &comp);
        let source_curve = f.curve(c.a.clone(), b0.clone());
        assert_eq!(times(&source_curve, &dual, 11), P::Infinity);
        let dualroots: Vec<_> = (1..=5)
            .map(|j| px(&times(&source_curve, &dual, j)))
            .collect();
        assert_eq!(qb(&f, &b0, &dualroots), c.b);
        let back7 = velu(&f, &dualroots, &transport7);
        assert_eq!(from_roots(&f, &back7), from_roots(&f, &first));
        let dual7 = velu(&f, &transport7, &dualroots);
        let backsecond = velu(&f, &dual7, &second_new);
        assert_eq!(from_roots(&f, &backsecond), from_roots(&f, &second));
        let psi = division(&f, &b0, 11);
        let known = from_roots(&f, &dualroots);
        let (remaining, remainder) = div(&f, &psi, &known);
        assert!(remainder.is_empty());
        let mut xq = vec![f.z(), f.o()];
        for _ in 0..m {
            xq = div(&f, &square(&f, &xq), &psi).1;
        }
        let rational = gcdp(&f, &psi, &add(&xq, &[f.z(), f.o()]));
        assert!([6, 61].contains(&rational.len()));
        if rational.len() == 6 {
            assert_eq!(rational, known);
        }
        rows.push(json!({"neighbor":i,"source_modular_point":pj(&source),"source_a":ej(&c.a),"source_b":ej(&b0),"source_first7_roots":transport7.iter().map(ej).collect::<Vec<_>>(),"source_second7_roots":second_new.iter().map(ej).collect::<Vec<_>>(),"dual11_generator":pj(&dual),"known11_kernel":polyj(&known),"remaining11_polynomial":polyj(&remaining),"rational11_x_degree":rational.len()-1,"missing_complement_extension_degree":if rational.len()==61{1}else{11},"expected_geometric_fiber_size":55}));
        println!(
            "R3 neighbor{i}: reverse49 transport checked, rational11 x degree{}",
            rational.len() - 1
        );
    }
    round02::save(
        dir,
        "o4-seed-screen.json",
        &json!({"schema":"koblitz.round07/o4-screen-v1","target_F1024":target.json(),"target_modular_j_triple_F1024":js,"field":{"degree":m,"low_terms":f.irr.low_terms},"F1024_embedding_root":ej(&root),"underlying_trace_F1024":t,"frobenius_order_mod11":ord,"underlying_order_over_field":n.to_string(),"target_curve_b":ej(&c.b),"basis11":basis.iter().map(pj).collect::<Vec<_>>(),"fixed_pairing":ej(&zeta),"neighbors":rows,"neighbor_count":12,"expected_full_fiber_degree":660,"full_fiber_evaluated":false,"scope":"all12 reverse base neighbors, dual kernels and backward49 transport evaluated; next step explicit degree11 etale complement algebras, no reduced Jacobian class"}),
    );
}

pub fn algebras(dir: &Path) {
    let input = round04::read(&dir.join("o4-seed-screen.json"));
    let m = input["field"]["degree"].as_u64().unwrap() as u32;
    let terms: Vec<u32> = serde_json::from_value(input["field"]["low_terms"].clone()).unwrap();
    let low = terms.iter().fold(0u64, |v, i| v | (1 << i));
    let f = F::new(m, low);
    let zeta = decode(&f, &input["fixed_pairing"]);
    let mut total = 0;
    let mut split_points = 0;
    let mut etale_components = 0;
    for row in input["neighbors"].as_array().unwrap() {
        let index = row["neighbor"].as_u64().unwrap();
        let name = format!("o4-fiber-neighbor-{index:02}.json");
        let a = decode(&f, &row["source_a"]);
        let b = decode(&f, &row["source_b"]);
        let p = poly(&f, &row["remaining11_polynomial"]);
        let c = f.curve(a.clone(), b.clone());
        let g = dp(&f, &row["dual11_generator"]);
        if row["rational11_x_degree"] == 60 {
            let mut h = BigUint::parse_bytes(
                input["underlying_order_over_field"]
                    .as_str()
                    .unwrap()
                    .as_bytes(),
                10,
            )
            .unwrap();
            while &h % 11u32 == BigUint::zero() {
                h /= 11u32;
            }
            let mut complement = None;
            for x in 1..4096 {
                let Some(p) = f.point(&c, &f.e(x)) else {
                    continue;
                };
                let mut p = crate::binary_ecc::curve::scalar_mul(&c, &p, &h);
                if p == P::Infinity {
                    continue;
                }
                while times(&c, &p, 11) != P::Infinity {
                    p = times(&c, &p, 11);
                }
                if !(0..11).any(|i| times(&c, &g, i) == p) {
                    complement = Some(p);
                    break;
                }
            }
            let q = complement.expect("odd split complement search bound");
            let val = weil(&f, &c, &g, &q);
            let exp = (1..11).find(|i| f.powu(&zeta, *i) == val).unwrap();
            let q = times(&c, &q, (1..11).find(|i| i * exp % 11 == 1).unwrap());
            let mut components = vec![];
            for scale in 1..=5 {
                for j in 0..11 {
                    let pp = times(&c, &g, scale);
                    let qq = plus(
                        &c,
                        &times(&c, &q, (1..11).find(|i| i * scale % 11 == 1).unwrap()),
                        &times(&c, &g, j),
                    );
                    assert_eq!(weil(&f, &c, &pp, &qq), zeta);
                    let P::Affine { x, y } = qq else {
                        unreachable!()
                    };
                    components.push(json!({"P_scale":scale,"complement":j,"P":pj(&pp),"Q_x_minpoly":polyj(&[f.z(),f.o()]),"Q_x_polynomial":polyj(&[x]),"Q_y_polynomial":polyj(&[y]),"degree_over_evaluation_field":1,"Weil_pairing":ej(&zeta)}));
                    total += 1;
                    split_points += 1;
                }
            }
            round02::save(
                dir,
                &name,
                &json!({"schema":"koblitz.round07/o4-split-v1","neighbor":index,"source_modular_point":row["source_modular_point"],"source_a":row["source_a"],"source_b":row["source_b"],"paired_components":components,"geometric_degree":55,"all_Q_have_order11":true}),
            );
            println!("O4 neighbor{index}:55 split paired bases checked");
            continue;
        }
        let r = R {
            f: &f,
            p: p.clone(),
            a: a.clone(),
            b: b.clone(),
            fx: vec![b.clone(), f.z(), a, f.o()],
        };
        let q = AP::Pt(
            A {
                u: vec![f.z(), f.o()],
                v: vec![],
            },
            A {
                u: vec![],
                v: vec![f.o()],
            },
        );
        let gp = r.cp(&g);
        assert_eq!(r.times(&q, 11), AP::O);
        let s = (1..1024)
            .filter_map(|i| f.point(&c, &f.e(i)))
            .find(|p| times(&c, p, 11) != P::Infinity)
            .expect("auxiliary budget");
        println!("R3b neighbor{index}: compute Weil resolvent in degree110 etale algebra");
        let e = r.weil(&gp, &q, &r.cp(&s));
        assert_eq!(r.pow(&e, 11), r.o());
        let sym = r.add(&e, &r.inv(&e));
        assert!(sym.v.is_empty());
        let mut factors = vec![];
        for scale in 1..=5 {
            let inverse = (1..11).find(|i| i * scale % 11 == 1).unwrap();
            let z = f.powu(&zeta, inverse);
            let symz = z.add(&f.inv(&z));
            let factor = gcdp(&f, &p, &add(&sym.u, &[symz]));
            assert_eq!(factor.len(), 12, "paired complement factor degree");
            let yy = div(
                &f,
                &mul(&f, &add(&e.u, &[z.clone()]), &invmod(&f, &e.v, &factor)),
                &factor,
            )
            .1;
            let yy2 = square(&f, &yy);
            let residual = add(&add(&yy2, &mul(&f, &[f.z(), f.o()], &yy)), &r.fx);
            assert!(div(&f, &residual, &factor).1.is_empty());
            let mut x = vec![f.z(), f.o()];
            for _ in 0..m {
                x = div(&f, &square(&f, &x), &factor).1;
            }
            assert_eq!(gcdp(&f, &factor, &add(&x, &[f.z(), f.o()])).len(), 1);
            let mut x = vec![f.z(), f.o()];
            for _ in 0..m * 11 {
                x = div(&f, &square(&f, &x), &factor).1;
            }
            assert_eq!(x, vec![f.z(), f.o()]);
            factors.push(json!({"P_scale":scale,"P":pj(&times(&c,&g,scale)),"Q_x_minpoly":polyj(&factor),"Q_y_polynomial":polyj(&yy),"degree_over_evaluation_field":11,"Weil_pairing":ej(&zeta),"Q_y_orientation_pairing_to_unscaled_G":ej(&z),"irreducibility_checked":true}));
            total += 11;
            etale_components += 1;
        }
        let product = factors
            .iter()
            .map(|v| poly(&f, &v["Q_x_minpoly"]))
            .fold(vec![f.o()], |v, g| mul(&f, &v, &g));
        assert_eq!(product, p);
        round02::save(
            dir,
            &name,
            &json!({"schema":"koblitz.round06/r3b-v1","neighbor":index,"source_modular_point":row["source_modular_point"],"source_a":row["source_a"],"source_b":row["source_b"],"auxiliary":pj(&s),"pairing_polynomial_u":polyj(&e.u),"pairing_polynomial_v":polyj(&e.v),"paired_components":factors,"geometric_degree":55,"all_Q_have_order11":true,"scope":"explicit etale point coordinates, not reduced Jacobian representation"}),
        );
        println!("R3b neighbor{index}: five irreducible degree11 paired components certified");
    }
    assert_eq!(total, 660);
    round02::save(
        dir,
        "o4-complete-seed.json",
        &json!({"schema":"koblitz.round07/o4-seed-v1","target_F1024":input["target_F1024"],"evaluation_field":input["field"],"split_points":split_points,"etale_components":etale_components,"geometric_fiber_degree":total,"map":"phi_0, the P-line quotient on determinant-normalized full11 level","fiber_evaluated_as_etale_algebras":true,"whole_reverse_20_fibers_evaluated":false,"reduced_Jacobian_class_evaluated":false,"scope":"one complete degree660 ordinary pullback fiber in explicit paired torsion algebras; not all reverse divisors or Jacobian reduction"}),
    );
}
// Same exhaustive cubic search as the maintained solver, with a pinned
// multiplication table. Every accepted factor is checked by the old path.
fn kernels7_table(s: round04::Field, b: u32, target: u32) -> Vec<Vec<u32>> {
    use std::sync::OnceLock;
    static TABLE: OnceLock<Vec<u32>> = OnceLock::new();
    assert_eq!(s.m, 10);
    let table = TABLE.get_or_init(|| {
        (0..1024usize * 1024)
            .map(|i| s.mul((i >> 10) as u32, (i & 1023) as u32))
            .collect()
    });
    let Some(t) = s.quadratic(b ^ s.inv(target)) else {
        return vec![];
    };
    let f = F::new(10, s.modulus ^ (1 << 10));
    let d: Vec<u32> = division(&f, &f.e(b as u64), 7)
        .iter()
        .map(|e| e.raw_bits()[0] as u32)
        .collect();
    let mut out = vec![];
    for t in [t, t ^ 1] {
        for c in 0..1024u32 {
            for v in 1..1024u32 {
                let k = vec![v, c, t, 1];
                let mut r = d.clone();
                for i in (3..r.len()).rev() {
                    let a = r[i] as usize;
                    r[i] = 0;
                    for j in 0..3 {
                        r[i - 3 + j] ^= table[(a << 10) | k[j] as usize];
                    }
                }
                if r[..3].iter().any(|x| *x != 0) {
                    continue;
                }
                assert!(round04::div(s, &d, &k).1.is_empty());
                if round04::gcd(s, &k, &round04::derivative(&k)).len() != 1 {
                    continue;
                }
                let x2 = round04::add(
                    &[0, 0, 1],
                    &round04::scale(s, &round04::invmod(s, &[0, 0, 1], &k), b),
                );
                if round04::composemod(s, &k, &x2, &k).is_empty() {
                    out.push(k);
                }
            }
        }
    }
    out
}
fn field(dir: &Path) -> (serde_json::Value, F) {
    let s = round04::read(&dir.join("o4-seed-screen.json"));
    let m = s["field"]["degree"].as_u64().unwrap() as u32;
    let low = s["field"]["low_terms"]
        .as_array()
        .unwrap()
        .iter()
        .fold(0, |v, i| v | (1u64 << i.as_u64().unwrap()));
    (s, F::new(m, low))
}
pub fn target_norms(dir: &Path) {
    use crate::forward07::{ap_json, ca, constant, frob, targets, trace};
    let (input, f) = field(dir);
    let root = decode(&f, &input["F1024_embedding_root"]);
    let target = P::Affine {
        x: f.embed_bits(input["target_F1024"][0].as_u64().unwrap(), &root),
        y: f.embed_bits(input["target_F1024"][1].as_u64().unwrap(), &root),
    };
    let mut records = vec![];
    for row in input["neighbors"].as_array().unwrap() {
        let n = row["neighbor"].as_u64().unwrap();
        let data = round04::read(&dir.join(format!("o4-fiber-neighbor-{n:02}.json")));
        let mut local = vec![];
        for cp in data["paired_components"].as_array().unwrap() {
            let r = R {
                f: &f,
                p: poly(&f, &cp["Q_x_minpoly"]),
                a: decode(&f, &row["source_a"]),
                b: decode(&f, &row["source_b"]),
                fx: vec![],
            };
            let p = r.cp(&dp(&f, &cp["P"]));
            let qx = if cp["Q_x_polynomial"].is_array() {
                poly(&f, &cp["Q_x_polynomial"])
            } else {
                vec![f.z(), f.o()]
            };
            let qy = poly(&f, &cp["Q_y_polynomial"]);
            let q = AP::Pt(ca(&r, &qx), ca(&r, &qy));
            let gal_step = if r.p.len() == 12 {
                let delta = r.plus(&frob(&r, &q, f.m()), &r.neg(&q));
                Some(
                    (1..11)
                        .find(|i| r.times(&p, *i) == delta)
                        .expect("degree11 Galois action must be nontrivial complement translation"),
                )
            } else {
                None
            };
            let tt = targets(
                &f,
                &r.p,
                &r.a,
                &r.b,
                &poly(&f, &row["source_first7_roots"]),
                &poly(&f, &row["source_second7_roots"]),
                &p,
                &q,
            );
            assert_eq!(constant(&f, &tt[0]), target);
            let norms: Vec<_> = tt.iter().map(|p| trace(&f, &r.p, p)).collect();
            let record = json!({"seed_component":records.len(),"neighbor":n,"degree":r.p.len()-1,"modulus":polyj(&r.p),"source_modular_point":row["source_modular_point"],"source_a":row["source_a"],"source_b":row["source_b"],"P":cp["P"],"Q_x":polyj(&qx),"Q_y":polyj(&qy),"Galois_complement_step":gal_step,"targets":tt.iter().map(ap_json).collect::<Vec<_>>(),"target_traces":norms.iter().map(pj).collect::<Vec<_>>()});
            records.push(record.clone());
            local.push(record);
        }
        round02::save(
            dir,
            &format!("o4-target-neighbor-{n:02}.json"),
            &json!({"components":local}),
        );
        println!(
            "O4 norms neighbor{n}:{} paired components evaluated",
            data["paired_components"].as_array().unwrap().len()
        );
    }
    assert_eq!(
        records
            .iter()
            .map(|r| r["degree"].as_u64().unwrap())
            .sum::<u64>(),
        660
    );
    round02::save(
        dir,
        "o4-independent-seed-targets.json",
        &json!({"schema":"koblitz.round07/o4-targets-v1","target_large":pj(&target),"field":input["field"],"components":records,"Gram_used":false,"scope":"all12 modular targets and point traces for every split/etale component, with actual nontrivial Galois complement steps"}),
    );
}
pub fn divisor(dir: &Path) {
    use crate::forward07::{ca, frob, line_map, tuple};
    use std::collections::BTreeSet;
    let (input, f) = field(dir);
    let data = round04::read(&dir.join("o4-independent-seed-targets.json"));
    let seeds = data["components"].as_array().unwrap();
    let ec = f.curve(f.o(), f.o());
    let p0 = dp(&f, &data["target_large"]);
    assert_eq!(f.frobp(&p0, 1), times(&ec, &p0, 7));
    assert_eq!(f.frobp(&p0, 5), neg(&p0));
    let zeta = decode(&f, &input["fixed_pairing"]);
    let mut splitkeys = BTreeSet::new();
    for s in seeds.iter().filter(|s| s["degree"] == 1) {
        let p = dp(&f, &s["P"]);
        let q = P::Affine {
            x: poly(&f, &s["Q_x"])
                .first()
                .cloned()
                .unwrap_or_else(|| f.z()),
            y: poly(&f, &s["Q_y"])
                .first()
                .cloned()
                .unwrap_or_else(|| f.z()),
        };
        splitkeys.insert(basiskey(&dp(&f, &s["source_modular_point"]), &p, &q));
    }
    let mut descent_checks = 0;
    for s in seeds {
        let base = f.frobp(&dp(&f, &s["source_modular_point"]), 10);
        let p = f.frobp(&dp(&f, &s["P"]), 10);
        if s["degree"] == 1 {
            let q = P::Affine {
                x: poly(&f, &s["Q_x"])
                    .first()
                    .cloned()
                    .unwrap_or_else(|| f.z()),
                y: poly(&f, &s["Q_y"])
                    .first()
                    .cloned()
                    .unwrap_or_else(|| f.z()),
            };
            assert!(splitkeys.contains(&basiskey(&base, &p, &f.frobp(&q, 10))));
        } else {
            let g: Poly = poly(&f, &s["modulus"])
                .iter()
                .map(|v| f.frob(v, 10))
                .collect();
            let y: Poly = poly(&f, &s["Q_y"]).iter().map(|v| f.frob(v, 10)).collect();
            let other = seeds
                .iter()
                .find(|t| {
                    t["degree"] == 11
                        && dp(&f, &t["source_modular_point"]) == base
                        && poly(&f, &t["modulus"]) == g
                        && {
                            let q = dp(&f, &t["P"]);
                            q == p || q == neg(&p)
                        }
                })
                .expect("odd etale Frobenius10 component");
            let expected = if dp(&f, &other["P"]) == p {
                y
            } else {
                add(&y, &[f.z(), f.o()])
            };
            assert_eq!(poly(&f, &other["Q_y"]), expected);
        }
        descent_checks += 1;
    }
    let mut terms = vec![];
    let mut degrees = [0usize; 2];
    let mut forward12 = vec![P::Infinity; 12];
    let mut entries = 0;
    for i in 0..5 {
        for line_sign in [1i64, -1] {
            let line = (line_sign * 2i64.pow(i)).rem_euclid(11) as u64;
            let ai = (1..11).find(|v| v * line % 11 == 1).unwrap();
            for positive in [true, false] {
                let j = (i + if positive { 9 } else { 4 }) % 10;
                let coefficient = line_sign * if positive { 1 } else { -1 };
                let invtwo = (1..11).find(|v| v * 2u64.pow(j) % 11 == 1).unwrap();
                let perm: Vec<_> = (0..12)
                    .map(|b| {
                        let k = line_map(0, 11 - line, ai, 1, b);
                        if k == 11 {
                            11
                        } else {
                            (k as u64 * invtwo % 11) as usize
                        }
                    })
                    .collect();
                let mut components = vec![];
                let mut norms = vec![P::Infinity; 12];
                for s in seeds {
                    let a = f.frob(&decode(&f, &s["source_a"]), j);
                    let b = f.frob(&decode(&f, &s["source_b"]), j);
                    let r = R {
                        f: &f,
                        p: poly(&f, &s["modulus"]),
                        a: a.clone(),
                        b: b.clone(),
                        fx: vec![],
                    };
                    let p = r.cp(&dp(&f, &s["P"]));
                    let q = AP::Pt(ca(&r, &poly(&f, &s["Q_x"])), ca(&r, &poly(&f, &s["Q_y"])));
                    let p = frob(&r, &p, j);
                    let q = r.times(&frob(&r, &q, j), invtwo);
                    let pp = r.neg(&r.times(&q, line));
                    let qq = r.plus(&r.times(&p, ai), &q);
                    assert_eq!(r.plus(&pp, &r.times(&qq, line)), p);
                    assert_eq!(r.times(&pp, 11), AP::O);
                    assert_eq!(r.times(&qq, 11), AP::O);
                    if s["seed_component"] == 0 {
                        let c = f.curve(a.clone(), b.clone());
                        let aux = (1..1024)
                            .filter_map(|x| f.point(&c, &f.e(x)))
                            .find(|p| times(&c, p, 11) != P::Infinity)
                            .unwrap();
                        assert_eq!(r.weil(&pp, &qq, &r.cp(&aux)), r.c(&zeta));
                    }
                    let (AP::Pt(px, py), AP::Pt(qx, qy)) = (pp, qq) else {
                        unreachable!()
                    };
                    assert!(
                        px.v.is_empty() && py.v.is_empty() && qx.v.is_empty() && qy.v.is_empty()
                    );
                    for k in 0..12 {
                        norms[k] = plus(
                            &ec,
                            &norms[k],
                            &f.frobp(&dp(&f, &s["target_traces"][perm[k]]), j),
                        );
                    }
                    let degree = s["degree"].as_u64().unwrap() as usize;
                    degrees[if coefficient > 0 { 0 } else { 1 }] += degree;
                    entries += 1;
                    components.push(json!({"seed_component":s["seed_component"],"degree":degree,"modulus":s["modulus"],"source_modular_point":pj(&f.frobp(&dp(&f,&s["source_modular_point"]),j)),"source_a":ej(&a),"source_b":ej(&b),"P_x":polyj(&px.u),"P_y":polyj(&py.u),"Q_x":polyj(&qx.u),"Q_y":polyj(&qy.u)}));
                }
                for k in 0..12 {
                    forward12[k] = plus(
                        &ec,
                        &forward12[k],
                        &if coefficient > 0 {
                            norms[k].clone()
                        } else {
                            neg(&norms[k])
                        },
                    );
                }
                let target = f.frobp(&p0, j);
                let pi = f.frobp(&p0, i);
                assert_eq!(target, times(&ec, &pi, if positive { 8 } else { 3 }));
                terms.push(json!({"i":i,"line":line,"coefficient":coefficient,"seed_frobenius_power":j,"target":pj(&target),"components":components,"independent_target_norms":norms.iter().map(pj).collect::<Vec<_>>()}));
                println!(
                    "O4 divisor term{} line{line}, Frobenius{j}, coefficient{coefficient}",
                    terms.len()
                );
            }
        }
    }
    assert_eq!(degrees, [6600, 6600]);
    let mut successor = vec![];
    for t in &terms {
        let line = 2 * t["line"].as_u64().unwrap() % 11;
        let target = f.frobp(&dp(&f, &t["target"]), 1);
        let index = terms
            .iter()
            .position(|s| {
                s["line"] == line
                    && s["coefficient"] == t["coefficient"]
                    && dp(&f, &s["target"]) == target
            })
            .expect("odd formal divisor Frobenius permutation");
        successor.push(index);
    }
    assert_eq!(successor.iter().copied().collect::<BTreeSet<_>>().len(), 20);
    let source: Vec<_> = (0..5)
        .map(|i| {
            let p = f.frobp(&p0, i);
            if i % 2 == 1 {
                neg(&p)
            } else {
                p
            }
        })
        .collect();
    assert_eq!(
        source.iter().fold(P::Infinity, |z, p| plus(&ec, &z, p)),
        P::Infinity
    );
    let image = tuple(&ec, &forward12, true);
    assert_eq!(image, source, "odd independent forward norm equals source");
    round02::save(
        dir,
        "o4-odd-reverse-divisor.json",
        &json!({"schema":"koblitz.round07/o4-divisor-v1","source_A5_odd_point":source.iter().map(pj).collect::<Vec<_>>(),"target_seed_E49":pj(&p0),"field":input["field"],"target_representative":"[8T]-[-8T], class16T=5T on11-torsion, formally odd","terms":terms,"point_algebra_entries":entries,"positive_degree":6600,"negative_degree":6600,"seed_Frobenius10_checks":descent_checks,"formal_Frobenius_permutation":successor,"F2_divisor_descent_checked":true,"independent_forward12":forward12.iter().map(pj).collect::<Vec<_>>(),"independent_forward_image":image.iter().map(pj).collect::<Vec<_>>(),"equals_source":true,"Gram_used_for_forward":false,"Jacobian_reduction":false,"scope":"all20 odd reverse pullbacks in actual split/etale coordinates, formal F2 descent and independent pointwise forward norms; no reduced genus2841 Jacobian class"}),
    );
    println!(
        "O4:all20 odd reverse fibers, {entries} entries; F2 descent and independent forward PASS"
    );
}
pub fn verify(dir: &Path) {
    use crate::forward07::{ap, frob};
    let (_input, f) = field(dir);
    let record = round04::read(&dir.join("o4-odd-reverse-divisor.json"));
    let cache = round04::read(&dir.join("o4-independent-seed-targets.json"));
    let mut count = 0;
    let mut degree = [0usize; 2];
    for term in record["terms"].as_array().unwrap() {
        let j = term["seed_frobenius_power"].as_u64().unwrap() as u32;
        let line = term["line"].as_u64().unwrap();
        for cp in term["components"].as_array().unwrap() {
            let r = R {
                f: &f,
                p: poly(&f, &cp["modulus"]),
                a: decode(&f, &cp["source_a"]),
                b: decode(&f, &cp["source_b"]),
                fx: vec![],
            };
            let p = ap(&r, &cp["P_x"], &cp["P_y"]);
            let q = ap(&r, &cp["Q_x"], &cp["Q_y"]);
            for z in [&p, &q] {
                let AP::Pt(x, y) = z else { panic!() };
                assert_eq!(
                    r.add(&r.sq(y), &r.mul(x, y)),
                    r.add(
                        &r.add(&r.mul(&r.sq(x), x), &r.mul(&r.c(&r.a), &r.sq(x))),
                        &r.c(&r.b)
                    )
                );
                assert_eq!(r.times(z, 11), AP::O);
            }
            let s = &cache["components"][cp["seed_component"].as_u64().unwrap() as usize];
            let old = r.cp(&dp(&f, &s["P"]));
            assert_eq!(r.plus(&p, &r.times(&q, line)), frob(&r, &old, j));
            degree[if term["coefficient"].as_i64().unwrap() > 0 {
                0
            } else {
                1
            }] += cp["degree"].as_u64().unwrap() as usize;
            count += 1;
        }
    }
    assert_eq!(degree, [6600, 6600]);
    let source: Vec<_> = record["source_A5_odd_point"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| dp(&f, p))
        .collect();
    let image: Vec<_> = record["independent_forward_image"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| dp(&f, p))
        .collect();
    assert_eq!(source, image);
    let sf = round04::Field::find(5);
    let root = f.root(37);
    let em: Vec<_> = (0..32).map(|x| f.embed_bits(x, &root)).collect();
    let tw = f.quadratic(&f.o()).unwrap();
    let P::Affine { x, y } = &source[0] else {
        panic!()
    };
    let yy = y.add(&f.mul(x, &tw));
    let small = round04::Pt::A(
        em.iter().position(|t| t == x).unwrap() as u32,
        em.iter().position(|t| *t == yy).unwrap() as u32,
    );
    let ec = round04::Curve { f: sf, a: 0, b: 1 };
    assert!(ec.valid(small));
    assert_ne!(small, round04::Pt::O);
    assert_eq!(ec.times(small, 11), round04::Pt::O);
    round02::save(
        dir,
        "o4b-odd-coordinate-check.json",
        &json!({"schema":"koblitz.round07/o4b-v1","coordinate_entries":count,"curve_checks":2*count,"torsion_checks":2*count,"line_checks":count,"degrees":degree,"source_E0_F32_point":small.json(),"nonzero_order11_checked_with_u32_arithmetic":true,"scope":"separate parse-and-evaluate checks for every odd reverse component; group/algebra primitives shared, final subgroup check uses small-field arithmetic"}),
    );
    println!("O4b:{count} coordinate entries checked; source nonzero order11 in E0(F32)");
}
