//! Direct closed traces of one actual split scalar fiber, with fixed old pairing.
use super::*;
use crate::binary_ecc::BinaryCurve;
use crate::large_field as lf;
type G = lf::P;
type E = lf::E;
fn eb(v: &B, n: u32) -> E {
    E::from_biguint(&BigUint::parse_bytes(hex(v).as_bytes(), 16).unwrap(), n)
}
fn be(v: &E) -> B {
    parse(&v.to_biguint().to_str_radix(16))
}
fn gx(v: &G) -> E {
    let G::Affine { x, .. } = v else {
        panic!("finite point")
    };
    x.clone()
}
fn line(f: &lf::F, c: &BinaryCurve, p: &G, q: &G, r: &G) -> Option<E> {
    let (G::Affine { x, y }, G::Affine { x: u, y: v }, G::Affine { x: rx, y: ry }) = (p, q, r)
    else {
        return None;
    };
    if (x == u && y != v) || (x.is_zero() && p == q) {
        let a = rx.add(x);
        return (!a.is_zero()).then_some(a);
    }
    let slope = if p == q {
        x.add(&f.mul(y, &f.inv(x)))
    } else {
        f.mul(&y.add(v), &f.inv(&x.add(u)))
    };
    let value = ry.add(&f.mul(&slope, &rx.add(x))).add(y);
    if value.is_zero() {
        return None;
    }
    match lf::plus(c, p, q) {
        G::Infinity => Some(value),
        G::Affine { x: s, .. } => {
            let d = rx.add(&s);
            if d.is_zero() {
                None
            } else {
                Some(f.mul(&value, &f.inv(&d)))
            }
        }
    }
}
fn miller(f: &lf::F, c: &BinaryCurve, p: &G, r: &G) -> Option<E> {
    let mut v = p.clone();
    let mut z = f.o();
    for bit in [0, 1, 1] {
        z = f.mul(&f.sq(&z), &line(f, c, &v, &v, r)?);
        v = lf::plus(c, &v, &v);
        if bit == 1 {
            z = f.mul(&z, &line(f, c, &v, p, r)?);
            v = lf::plus(c, &v, p);
        }
    }
    assert_eq!(v, G::Infinity);
    Some(z)
}
fn weil(f: &lf::F, c: &BinaryCurve, p: &G, q: &G) -> E {
    for x in 1..1024 {
        let Some(s) = f.point(c, &f.e(x)) else {
            continue;
        };
        let ns = lf::neg(&s);
        let a = lf::plus(c, q, &s);
        let b = lf::plus(c, p, &ns);
        if let (Some(a), Some(b), Some(d), Some(e)) = (
            miller(f, c, p, &a),
            miller(f, c, p, &s),
            miller(f, c, q, &ns),
            miller(f, c, q, &b),
        ) {
            let v = f.mul(&f.mul(&a, &d), &f.inv(&f.mul(&b, &e)));
            assert_eq!(f.powu(&v, 11), f.o());
            return v;
        }
    }
    panic!("frozen Weil auxiliary budget")
}
fn express(basis: &[u32], value: u32) -> u32 {
    let mut rows: BTreeMap<u32, (u32, u32)> = BTreeMap::new();
    for (i, v) in basis.iter().enumerate() {
        let mut v = *v;
        let mut mask = 1u32 << i;
        while v != 0 {
            let k = 31 - v.leading_zeros();
            if let Some((r, m)) = rows.get(&k) {
                v ^= r;
                mask ^= m;
            } else {
                rows.insert(k, (v, mask));
                break;
            }
        }
        assert_ne!(v, 0, "independent cyclotomic basis");
    }
    let mut v = value;
    let mut mask = 0;
    while v != 0 {
        let k = 31 - v.leading_zeros();
        let (r, m) = rows.get(&k).expect("value in common subfield");
        v ^= r;
        mask ^= m;
    }
    assert_eq!(
        basis
            .iter()
            .enumerate()
            .filter(|(i, _)| (mask >> i) & 1 != 0)
            .fold(0, |a, (_, v)| a ^ v),
        value
    );
    mask
}
fn tuple(f: &lf::F, ec: &BinaryCurve, t: &[G], sign: i64) -> Vec<G> {
    let ds = (0..5)
        .map(|i| {
            let a = 2usize.pow(i) % 11;
            let v = lf::plus(
                ec,
                &t[a],
                &if sign == 1 {
                    t[11 - a].clone()
                } else {
                    lf::neg(&t[11 - a])
                },
            );
            if sign == -1 && i % 2 == 1 {
                lf::neg(&v)
            } else {
                v
            }
        })
        .collect::<Vec<_>>();
    let sum = ds.iter().fold(G::Infinity, |v, p| lf::plus(ec, &v, p));
    let _ = f;
    ds.iter()
        .map(|p| lf::plus(ec, &lf::times(ec, p, 5), &lf::neg(&sum)))
        .collect()
}
fn frob(f: &lf::F, ds: &[G], sign: i64) -> Vec<G> {
    (0..5)
        .map(|i| {
            let p = f.frobp(&ds[(i + 4) % 5], 1);
            if sign == 1 {
                p
            } else {
                lf::neg(&p)
            }
        })
        .collect()
}
pub fn run(dir: &Path) {
    run_inner(dir, 15, "o25-degree7-individual-images.json");
}
pub fn generic(dir: &Path, index: usize) {
    assert!([0, 1, 2, 5, 12, 17, 20].contains(&index));
    run_inner(dir, index, &format!("o26-point-{index}-images.json"));
}
fn embed(f: &lf::F, bits: &BigUint, root: &E) -> E {
    let mut v = f.z();
    for i in (0..bits.bits()).rev() {
        v = f.mul(&v, root);
        if bits.bit(i) {
            v = v.add(&f.o());
        }
    }
    v
}
fn run_inner(dir: &Path, index: usize, output: &str) {
    REDUCTION_LIMIT.store(1_000_000_000, Ordering::Relaxed);
    let r = crate::round04::read(&dir.join(format!("o22-point-{index}.json")));
    let d = r["field_degree"].as_u64().unwrap() as u32;
    let lift = &r["verified_lifts"][0];
    let pts = lift["source_closed_points"].as_array().unwrap();
    assert_eq!(pts.len(), 5);
    let m = 2 * d * if d % 5 == 0 { 1 } else { 5 };
    assert!(m <= 560);
    let f = lf::F::find(m);
    assert!(f.irreducible());
    let k = K::new(parse(&f.modulus().to_str_radix(16)));
    let base_poly = BigUint::parse_bytes(
        pts[0]["base_field_modulus_bits"]
            .as_str()
            .unwrap()
            .as_bytes(),
        16,
    )
    .unwrap();
    let poly = (0..base_poly.bits())
        .map(|i| f.e(u64::from(base_poly.bit(i))))
        .collect::<Vec<_>>();
    let root = f
        .roots(&poly)
        .into_iter()
        .min_by_key(|v| v.to_biguint())
        .unwrap();
    assert_eq!(f.frob(&root, d), root);
    let embedding = |s: &str| embed(&f, &BigUint::parse_bytes(s.as_bytes(), 16).unwrap(), &root);
    let dec = |v: &serde_json::Value| {
        v.as_array()
            .unwrap()
            .iter()
            .map(|s| embedding(s.as_str().unwrap()))
            .collect::<Vec<_>>()
    };
    let jm = dec(&lift["middle_j"]);
    let b = f.inv(&jm[0]);
    let b7 = f.inv(&jm[1]);
    let ec = f.curve(f.o(), f.o());
    let universal = f.curve(f.z(), b.clone());
    let kp = dec(&lift["first11_kernel"]);
    let kq = dec(&lift["second11_kernel"]);
    let k1 = dec(&lift["cyclic49_kernels"][0])
        .iter()
        .map(be)
        .collect::<P>();
    let k2 = dec(&lift["cyclic49_kernels"][1])
        .iter()
        .map(be)
        .collect::<P>();
    let px = f.roots(&kp);
    let qx = f.roots(&kq);
    let p = f.point(&universal, &px[0]).unwrap();
    let mut q = f.point(&universal, &qx[0]).unwrap();
    assert_eq!(lf::times(&universal, &p, 11), G::Infinity);
    assert_eq!(lf::times(&universal, &q, 11), G::Infinity);
    let old = crate::round04::read(&dir.join("i1-degree210-images.json"));
    let s = crate::round04::Field {
        m: 30,
        modulus: old["F30_modulus"].as_u64().unwrap(),
    };
    let z = old["fixed_weil_pairing_F30"].as_u64().unwrap() as u32;
    let basis = (0..10).map(|i| s.pow(z, i)).collect::<Vec<_>>();
    let zp = (1u32 << 10) ^ express(&basis, s.pow(z, 10));
    let zeta = f.root(zp as u64);
    assert_ne!(zeta, f.o());
    assert_eq!(f.powu(&zeta, 11), f.o());
    let small = crate::round04::Field {
        m: 5,
        modulus: old["F32_modulus"].as_u64().unwrap(),
    };
    let map = s.embed(small);
    let r32 = f.embed_bits(express(&basis, map[2]) as u64, &zeta);
    let ef32 = (0..32).map(|i| f.embed_bits(i, &r32)).collect::<Vec<_>>();
    assert_eq!(f.frob(&r32, 5), r32);
    let pairing = weil(&f, &universal, &p, &q);
    assert_ne!(pairing, f.o());
    assert_eq!(weil(&f, &universal, &q, &p), f.inv(&pairing));
    assert_eq!(weil(&f, &universal, &p, &p), f.o());
    assert_eq!(
        weil(
            &f,
            &universal,
            &lf::times(&universal, &p, 2),
            &lf::times(&universal, &q, 3)
        ),
        f.powu(&pairing, 6)
    );
    let scale = (1..11).find(|i| f.powu(&pairing, *i) == zeta).unwrap();
    q = lf::times(&universal, &q, scale);
    assert_eq!(weil(&f, &universal, &p, &q), zeta);
    let old_s4 = s.quadratic(1).unwrap();
    let s4 = f.embed_bits(express(&basis, old_s4) as u64, &zeta);
    assert_eq!(f.sq(&s4).add(&s4), f.o());
    let mut columns = vec![];
    let mut sums = [G::Infinity, G::Infinity];
    let mut nonzero = [0usize; 2];
    for i in 0..5 {
        let a = 2u64.pow(i);
        let pp = lf::times(&universal, &p, a);
        let inverse = (1..11).find(|v| v * a % 11 == 1).unwrap();
        let qq = lf::times(&universal, &q, inverse);
        assert_eq!(weil(&f, &universal, &pp, &qq), zeta);
        let factor = pts
            .iter()
            .position(|p| {
                let poly = dec(&p["extension_polynomial_coefficients"]);
                f.eval(&poly, &gx(&pp)).is_zero()
            })
            .expect("actual scalar factor");
        let mut targets = vec![];
        let mut lines = vec![];
        for lambda in 0..12 {
            let v = if lambda == 11 {
                qq.clone()
            } else {
                lf::plus(&universal, &pp, &lf::times(&universal, &qq, lambda as u64))
            };
            assert_ne!(v, G::Infinity);
            let xs = (1..=5)
                .map(|j| gx(&lf::times(&universal, &v, j)))
                .collect::<Vec<_>>();
            let kernel = lf::from_roots(&f, &xs).iter().map(be).collect::<P>();
            assert_eq!(kernel.len(), 6);
            assert!(pd(&k, &division(&k, &be(&b), 11), &kernel).1.is_empty());
            assert!(pevalmod(&k, &kernel, &doubling(&k, &be(&b), &kernel), &kernel).is_empty());
            let js = transported(&k, &be(&b), &be(&b7), &k1, &k2, &kernel);
            let js = js.map(|x| eb(&x, m));
            let target = lf::reconstruct(&f, &js);
            targets.push(target);
            lines.push(kernel.iter().map(|v| hex(v)).collect::<Vec<_>>());
        }
        assert_eq!(
            lf::polyj(&lf::from_roots(
                &f,
                &(1..=5)
                    .map(|j| gx(&lf::times(&universal, &pp, j)))
                    .collect::<Vec<_>>()
            )),
            lf::polyj(&kp)
        );
        let expected_source = dec(&crate::round04::read(
            &dir.join(format!("o21-point-{index}.json")),
        )["source_target"]);
        let expected_target = dec(&lift["target"]);
        assert_eq!(
            targets[0],
            G::Affine {
                x: expected_source[0].clone(),
                y: expected_source[1].clone()
            }
        );
        assert_eq!(
            targets[11],
            G::Affine {
                x: expected_target[0].clone(),
                y: expected_target[1].clone()
            }
        );
        let mut images = vec![];
        for (si, sign) in [1, -1].into_iter().enumerate() {
            let ds = tuple(&f, &ec, &targets, sign);
            let mut term = ds.clone();
            let mut image = vec![G::Infinity; 5];
            for _ in 0..d {
                for j in 0..5 {
                    image[j] = lf::plus(&ec, &image[j], &term[j]);
                }
                term = frob(&f, &term, sign);
            }
            assert_eq!(term, ds, "normalized Frobenius over base field");
            assert_eq!(frob(&f, &image, sign), image);
            assert_eq!(
                image.iter().fold(G::Infinity, |v, p| lf::plus(&ec, &v, p)),
                G::Infinity
            );
            assert_eq!(lf::times(&ec, &image[0], 11), G::Infinity);
            let descended = match &image[0] {
                G::Infinity => G::Infinity,
                G::Affine { x, y } => G::Affine {
                    x: x.clone(),
                    y: if sign == 1 {
                        y.clone()
                    } else {
                        y.add(&f.mul(x, &s4))
                    },
                },
            };
            let target = f.curve(f.e(if sign == 1 { 1 } else { 0 }), f.o());
            assert!(target.is_on_curve(&descended));
            assert_eq!(f.frobp(&descended, 5), descended);
            if descended != G::Infinity {
                nonzero[si] += 1;
            }
            sums[si] = lf::plus(&target, &sums[si], &descended);
            let compressed = match &descended {
                G::Infinity => json!({"infinity":true}),
                G::Affine { x, y } => json!([
                    ef32.iter().position(|v| v == x).expect("F32x"),
                    ef32.iter().position(|v| v == y).expect("F32y")
                ]),
            };
            images.push(json!({"a":if sign==1{1}else{0},"image_F32":compressed,"image_F70":lf::pj(&descended),"trace_tuple":image.iter().map(lf::pj).collect::<Vec<_>>() }));
        }
        columns.push(json!({"source_scalar_factor_index":factor,"P11":lf::pj(&pp),"Q11":lf::pj(&qq),"line_kernel_polynomials":lines,"all12_targets":targets.iter().map(lf::pj).collect::<Vec<_>>(),"images":images}));
        println!("O25/O26:point{index},degree{d} scalar factor{factor},both images evaluated");
    }
    assert_eq!(
        columns
            .iter()
            .map(|p| p["source_scalar_factor_index"].as_u64().unwrap())
            .collect::<BTreeSet<_>>()
            .len(),
        5
    );
    assert_eq!(sums, [G::Infinity, G::Infinity]);
    let mut record = json!({"schema":"koblitz.round14/o25-v1","source_point_index":15,"base_field_modulus_bits":base_poly.to_str_radix(16),"F70_modulus_bits":f.modulus().to_str_radix(16),"F7_embedding_root":lf::ej(&root),"cyclotomic_minimal_polynomial_bits":format!("{zp:x}"),"fixed_pairing_F70":lf::ej(&zeta),"old_pairing_F30":z,"F32_embedding":ef32.iter().map(lf::ej).collect::<Vec<_>>(),"columns":columns,"nonzero_by_sign":nonzero,"sum_by_sign":sums.iter().map(lf::pj).collect::<Vec<_>>(),"commuting_squares":5*12*2,"individual_closed_points_evaluated":5,"normalized_Frobenius7_checks":10,"reduction_steps":REDUCTIONS.load(Ordering::Relaxed),"historical_rank_promoted":false});
    record["old_twist_root_F30"] = json!(old_s4);
    record["twist_root_F70"] = lf::ej(&s4);
    record["odd_twist_alignment"] =
        json!("old F30 root transported through the fixed cyclotomic embedding");
    if index != 15 {
        let obj = record.as_object_mut().unwrap();
        for (old, new) in [
            ("F70_modulus_bits", "work_field_modulus_bits"),
            ("F7_embedding_root", "base_embedding_root"),
            ("fixed_pairing_F70", "fixed_pairing"),
            ("twist_root_F70", "twist_root_work_field"),
            (
                "normalized_Frobenius7_checks",
                "normalized_Frobenius_checks",
            ),
        ] {
            let v = obj.remove(old).unwrap();
            obj.insert(new.into(), v);
        }
        obj.insert("schema".into(), json!("koblitz.round14/o26-v1"));
        obj.insert("source_point_index".into(), json!(index));
        obj.insert("base_degree".into(), json!(d));
        obj.insert("work_field_degree".into(), json!(m));
        for col in obj.get_mut("columns").unwrap().as_array_mut().unwrap() {
            for im in col["images"].as_array_mut().unwrap() {
                let ob = im.as_object_mut().unwrap();
                let v = ob.remove("image_F70").unwrap();
                ob.insert("image_work_field".into(), v);
            }
        }
    }
    round02::save(dir, output, &record);
}
