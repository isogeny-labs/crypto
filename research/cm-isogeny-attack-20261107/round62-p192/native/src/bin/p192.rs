#![allow(dead_code, unused_imports)]
include!("../common.rs");
fn compact(f: &fp::Fp, e: &Value, p: &A, a: &Z, b: &Z) -> A {
    let h: Vec<_> = e["kernel_polynomial"]
        .as_array()
        .unwrap()
        .iter()
        .map(fp::parse)
        .collect();
    let ell = fp::parse(&e["degree"]);
    let x = &p.0;
    let y = &p.1;
    let dh = f.der(&h);
    let d2 = f.der(&dh);
    let d3 = f.der(&d2);
    let hi = f.inv(&f.ev(&h, x));
    let l = f.mul(&f.ev(&dh, x), &hi);
    let q = f.mul(&f.ev(&d2, x), &hi);
    let t = f.mul(&f.ev(&d3, x), &hi);
    let l2 = f.mul(&l, &l);
    let u = f.sub(&q, &l2);
    let v = f.add(
        &f.sub(&t, &f.mul(&f.c(3), &f.mul(&l, &q))),
        &f.mul(&f.c(2), &f.mul(&l, &l2)),
    );
    let x2 = f.mul(x, x);
    let ff = f.add(&f.add(&f.mul(&x2, x), &f.mul(a, x)), b);
    let ff1 = f.add(&f.mul(&f.c(3), &x2), a);
    let ff2 = f.mul(&f.c(6), x);
    let xx = f.sub(
        &f.sub(
            &f.add(&f.mul(&ell, x), &f.mul(&f.c(2), &h[h.len() - 2])),
            &f.mul(&f.c(2), &f.mul(&ff1, &l)),
        ),
        &f.mul(&f.c(4), &f.mul(&ff, &u)),
    );
    let dy = f.sub(
        &f.sub(
            &f.sub(&ell, &f.mul(&f.c(2), &f.mul(&ff2, &l))),
            &f.mul(&f.c(6), &f.mul(&ff1, &u)),
        ),
        &f.mul(&f.c(4), &f.mul(&ff, &v)),
    );
    (xx, f.mul(y, &dy))
}
fn dense(f: &fp::Fp, e: &Value, p: &A) -> A {
    let v = |n: &str| {
        e[n].as_array()
            .unwrap()
            .iter()
            .map(fp::parse)
            .collect::<Vec<_>>()
    };
    let nn = f.ev(&v("x_numerator"), &p.0);
    let dd = f.ev(&v("x_denominator"), &p.0);
    let dy = f.ev(&v("derivative_numerator"), &p.0);
    let di = f.inv(&dd);
    (f.mul(&nn, &di), f.mul(&p.1, &f.mul(&dy, &f.mul(&di, &di))))
}
fn on(f: &fp::Fp, p: &A, a: &Z, b: &Z) -> bool {
    f.mul(&p.1, &p.1) == f.add(&f.add(&f.mul(&f.mul(&p.0, &p.0), &p.0), &f.mul(a, &p.0)), b)
}
fn neg(f: &fp::Fp, p: &A) -> A {
    (p.0.clone(), f.sub(&Z::zero(), &p.1))
}
fn sum(f: &fp::Fp, a: &Z, p: &A, q: &A) -> A {
    let j = mixed(f, a, &(p.0.clone(), p.1.clone(), Z::one()), q);
    assert!(!j.2.is_zero());
    let iz = f.inv(&j.2);
    (
        f.mul(&j.0, &f.mul(&iz, &iz)),
        f.mul(&j.1, &f.mul(&f.mul(&iz, &iz), &iz)),
    )
}
fn cp(p: &A) -> Value {
    json!([p.0.to_string(), p.1.to_string()])
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let root = Path::new(&args[1]);
    let load = |s: &str| serde_json::from_slice::<Value>(&fs::read(root.join(s)).unwrap()).unwrap();
    let input = load("../round52-p192/input.json");
    let pr = load("../round27-p192/prefix.json");
    let dual = load("../round28-p192/duals.json");
    let rel = load("../round25-p192/search.json");
    let f = fp::Fp {
        p: fp::parse(&input["p"]),
    };
    let r = fp::parse(&input["r"]);
    let a = fp::parse(&input["a"]);
    let b = fp::parse(&input["b"]);
    let g = point(&input["G"]);
    let q = point(&input["Q"]);
    let delta = sum(&f, &a, &g, &neg(&f, &q));
    let two = mul(&f, &a, &Z::from(2u32), &g);
    let edges = pr["edges"].as_array().unwrap();
    let ds = dual["duals"].as_array().unwrap();
    assert_eq!(edges.len(), 47);
    assert_eq!(ds.len(), 47);
    let degree = edges
        .iter()
        .fold(Z::one(), |d, e| d * fp::parse(&e["degree"]));
    assert_eq!(degree, fp::parse(&pr["degree"]));
    let dinv = (&degree % &r).modpow(&(&r - Z::from(2u32)), &r);
    assert_eq!((&degree * &dinv) % &r, Z::one());
    let mut rows = vec![];
    let mut forward = vec![];
    let mut checked = 0;
    for (label, start) in [
        ("G", g.clone()),
        ("public_Q", q.clone()),
        ("G_minus_Q", delta.clone()),
        ("2G", two.clone()),
    ] {
        let mut state = start.clone();
        let mut trace = vec![];
        let mut reads = 0;
        for (i, e) in edges.iter().enumerate() {
            let aa = fp::parse(&pr["vertices"][i]["a"]);
            let bb = fp::parse(&pr["vertices"][i]["b"]);
            assert!(on(&f, &state, &aa, &bb));
            let v = dense(&f, e, &state);
            assert_eq!(v, compact(&f, e, &state, &aa, &bb));
            let ta = fp::parse(&pr["vertices"][i + 1]["a"]);
            let tb = fp::parse(&pr["vertices"][i + 1]["b"]);
            assert!(on(&f, &v, &ta, &tb));
            for key in ["x_numerator", "x_denominator", "derivative_numerator"] {
                reads += e[key].as_array().unwrap().len()
            }
            trace.push(cp(&v));
            state = v;
            checked += 1
        }
        let image = state.clone();
        forward.push(image.clone());
        for d in ds.iter().rev() {
            let e = &d["map"];
            let aa = fp::parse(&d["source"]["a"]);
            let bb = fp::parse(&d["source"]["b"]);
            assert!(on(&f, &state, &aa, &bb));
            let v = dense(&f, e, &state);
            assert_eq!(v, compact(&f, e, &state, &aa, &bb));
            state = (
                f.mul(&v.0, &fp::parse(&d["closing_x_scale"])),
                f.mul(&v.1, &fp::parse(&d["closing_y_scale"])),
            );
            assert!(on(
                &f,
                &state,
                &fp::parse(&d["closed_target"]["a"]),
                &fp::parse(&d["closed_target"]["b"])
            ));
            for key in ["x_numerator", "x_denominator", "derivative_numerator"] {
                reads += e[key].as_array().unwrap().len()
            }
            checked += 1
        }
        let expect = mul(&f, &a, &(&degree % &r), &start);
        assert_eq!(state, expect);
        let out = mul(&f, &a, &dinv, &state);
        assert_eq!(out, start);
        assert!(on(&f, &out, &a, &b));
        rows.push(json!({"input_label":label,"input":cp(&start),"forward_trace":trace,"prefix_image":cp(&image),"dual_output":cp(&state),"retracted":cp(&out),"degree_inverse_mod_r":dinv.to_string(),"map_evaluations_each_representation":94,"dense_coefficients_read":reads,"scalar_retraction_bits":dinv.bits(),"norm_calls":0}));
    }
    let va = fp::parse(&pr["vertices"][47]["a"]);
    assert_eq!(sum(&f, &va, &forward[0], &neg(&f, &forward[1])), forward[2]);
    assert_eq!(mul(&f, &va, &Z::from(2u32), &forward[0]), forward[3]);
    let factors = rel["hits"][0]["factorization"].as_array().unwrap();
    let mut nd = Z::one();
    let mut ec = 0;
    let mut sumd = 0;
    for z in factors {
        let p = fp::parse(&z["prime"]);
        let n = z["exponent"].as_u64().unwrap();
        nd *= p.pow(n as u32);
        ec += n;
        sumd += z["prime"].as_u64().unwrap() * n
    }
    assert_eq!(nd, fp::parse(&rel["hits"][0]["norm"]));
    assert_eq!(ec, 53);
    assert_eq!(&nd % &degree, Z::zero());
    let out = json!({"all_checks_pass":true,"p":input["p"],"r":input["r"],"rows":rows,"independent_representation_map_pairs":checked,"prefix_degree":degree.to_string(),"prefix_degree_bits":degree.bits(),"whole_relation_degree":nd.to_string(),"whole_relation_degree_bits":nd.bits(),"whole_relation_edges":ec,"whole_relation_sum_prime_degrees":sumd,"residual_degree":(&nd/&degree).to_string(),"residual_edges":6,"prefix_branch_count_if_fully_expanded":degree.to_string(),"full_branch_count_if_fully_expanded":nd.to_string(),"norm_calls":0,"unknown_scalar_used":false,"scope":"actual 47-edge P192 point transport and dual retraction, dense vs compact maps; no full graph norm or missing six maps constructed"});
    if let Some(i) = args.iter().position(|s| s == "--check") {
        let v: Value = serde_json::from_slice(&fs::read(&args[i + 1]).unwrap()).unwrap();
        assert_eq!(out, v, "changed P192 transport certificate")
    }
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
