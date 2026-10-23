//! Actual higher-prime torsion, pairing and graph replay in an independent field implementation.
use super::{
    finite::{Field, Point},
    gluing::anti,
    Source,
};
use crate::{digest, write_json, Result};
use num_bigint::{BigInt, BigUint};
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use std::{collections::BTreeSet, fs, path::Path};
fn mm(a: [u64; 4], b: [u64; 4], l: u64) -> [u64; 4] {
    [
        (a[0] * b[0] + a[1] * b[2]) % l,
        (a[0] * b[1] + a[1] * b[3]) % l,
        (a[2] * b[0] + a[3] * b[2]) % l,
        (a[2] * b[1] + a[3] * b[3]) % l,
    ]
}
fn degree(s: &Source, l: u64) -> usize {
    let p = (&s.p % l).to_u64().unwrap();
    let a = [0, (l - p) % l, 1, s.residue_trace(l)];
    let mut v = [1, 0, 0, 1];
    for k in 1..=512 {
        v = mm(v, a, l);
        if v == [1, 0, 0, 1] {
            return k;
        }
    }
    panic!("Frobenius matrix-order cap");
}
pub fn input(src: &[Source], out: &Path) -> Result<Vec<Value>> {
    let mut rows = vec![];
    for l in [5u64, 7, 11, 13, 17] {
        // The source subgroup and characteristic must be prime to the gluing prime.
        let mut positives = src
            .iter()
            .filter(|s| {
                s.p.to_u64().is_some_and(|p| p < 32768)
                    && s.residue_trace(l) == 0
                    && (&s.p % l) != BigUint::from(0u32)
                    && (&s.n % l) != BigUint::from(0u32)
            })
            .collect::<Vec<_>>();
        positives.sort_by(|a, b| a.name.cmp(&b.name));
        positives.truncate(2);
        if positives.is_empty() {
            rows.push(json!({"experiment":"E20","ell":l,"status":"no-toy-module-positive-in-frozen-prime-panel"}));
        }
        for s in positives {
            let k = degree(s, l);
            let file = format!("{}-l{l}.gp", s.name);
            if k > 12 {
                rows.push(json!({"experiment":"E20","source":s.model(),"ell":l,"degree":k,"status":"unsupported-extension-degree-above-12"}));
                continue;
            }
            let mut script = String::from(include_str!("higher.gp"));
            script.push_str(&format!("\ntwist_higher_construct({},{},{},{},{},{},{},{});\nprint(\"ALL_HIGHER_TORSION_CONSTRUCTION_DONE\");quit;\n",s.p,s.a,s.b,s.n,s.d,l,k,serde_json::to_string(&s.name)?));
            fs::write(out.join(&file), script)?;
            rows.push(json!({"experiment":"E20","source":s.model(),"ell":l,"degree":k,"script":file,"status":"actual-torsion-constructor-input-prepared"}));
        }
    }
    Ok(rows)
}
fn table(k: &Field, p: &Point, q: &Point, l: u64, a: u64) -> Vec<Point> {
    (0..l)
        .flat_map(|i| (0..l).map(move |j| (i, j)))
        .map(|(i, j)| k.pa(&k.scalar(p, i, a), &k.scalar(q, j, a), a))
        .collect()
}
fn coords(table: &[Point], p: &Point, l: u64) -> [u64; 2] {
    let i = table
        .iter()
        .position(|q| q == p)
        .expect("point outside actual torsion basis");
    [i as u64 / l, i as u64 % l]
}
fn extension_order(s: &Source, degree: usize) -> BigUint {
    let p = BigInt::from(s.p.clone());
    let t: BigInt = &p + 1 - BigInt::from(s.n.clone());
    let (mut a, mut b) = (BigInt::from(2), t.clone());
    for _ in 2..=degree {
        let c = &t * &b - &p * &a;
        a = b;
        b = c;
    }
    (p.pow(degree as u32) + BigInt::from(1u32) - b)
        .to_biguint()
        .unwrap()
}
pub fn verify(src: &[Source], stdout: &Path, out: &Path) -> Result<Vec<Value>> {
    let raw = fs::read_to_string(stdout)?;
    let mut rows = vec![];
    for line in raw.lines().filter_map(|s| s.strip_prefix("CERTIFICATE ")) {
        let v: Value = serde_json::from_str(line)?;
        let s = src
            .iter()
            .find(|s| v["source"] == s.name)
            .ok_or("unknown source certificate")?;
        let l = v["ell"].as_u64().unwrap();
        assert!([5, 7, 11, 13, 17].contains(&l));
        let deg = v["degree"].as_u64().unwrap() as usize;
        assert_eq!(deg, degree(s, l));
        let p = s.p.to_u64().unwrap();
        assert_eq!(v["p"], p);
        let modulus: Vec<u64> = serde_json::from_value(v["modulus"].clone())?;
        let k = Field::new(p, modulus);
        assert_eq!(k.degree, deg);
        assert!(k.irreducible(), "CAS field modulus is reducible");
        let parse_point = |v: &Value| -> Point {
            if v.is_null() {
                None
            } else {
                Some((
                    k.parse(v[0].as_str().unwrap()),
                    k.parse(v[1].as_str().unwrap()),
                ))
            }
        };
        let basis = v["basis"].as_array().unwrap();
        assert_eq!(basis.len(), 2);
        let e0 = parse_point(&basis[0]);
        let e1 = parse_point(&basis[1]);
        let a = s.a.to_u64().unwrap();
        let b = s.b.to_u64().unwrap();
        assert!(e0.is_some() && e1.is_some() && k.on(&e0, a, b) && k.on(&e1, a, b));
        assert_eq!(k.scalar(&e0, l, a), None);
        assert_eq!(k.scalar(&e1, l, a), None);
        let ee = table(&k, &e0, &e1, l, a);
        assert_eq!(
            ee.iter().cloned().collect::<BTreeSet<_>>().len() as u64,
            l * l
        );
        assert!(ee.iter().all(|p| k.on(p, a, b)));
        let zeta = k.weil(&e0, &e1, l, a);
        assert_ne!(zeta, k.c(1));
        assert_eq!(k.pow(&zeta, l), k.c(1));
        assert_eq!(
            zeta,
            k.parse(v["pairing"].as_str().unwrap()),
            "independent Weil convention differs from PARI"
        );
        let c0 = coords(&ee, &k.frob(&e0), l);
        let c1 = coords(&ee, &k.frob(&e1), l);
        let frob = [c0[0], c1[0], c0[1], c1[1]];
        assert_eq!((frob[0] + frob[3]) % l, s.residue_trace(l));
        assert_eq!((frob[0] * frob[3] + l * l - frob[1] * frob[2]) % l, p % l);
        let w = k.parse(v["w"].as_str().unwrap());
        let d = s.d.to_u64().unwrap();
        assert_eq!(k.mul(&w, &w), k.c(d));
        assert_eq!(k.pow(&w, p), k.neg(&w));
        let twist = |p: &Point| {
            p.as_ref()
                .map(|(x, y)| (k.mul(&k.c(d), x), k.mul(&k.mul(&k.c(d), &w), y)))
        };
        let da = a * d % p * d % p;
        let db = b * d % p * d % p * d % p;
        let t0 = twist(&e0);
        let t1 = twist(&e1);
        let tt = table(&k, &t0, &t1, l, da);
        assert!(tt.iter().all(|p| k.on(p, da, db)));
        let c0 = coords(&tt, &k.frob(&t0), l);
        let c1 = coords(&tt, &k.frob(&t1), l);
        let tfrob = [c0[0], c1[0], c0[1], c1[1]];
        assert_eq!(tfrob, frob.map(|c| (l - c) % l));
        assert_eq!(k.weil(&t0, &t1, l, da), zeta);
        let matrices = anti(frob, tfrob, l);
        assert!(!matrices.is_empty());
        let mut graphs = vec![];
        for m in &matrices {
            let image = |i: u64, j: u64| {
                tt[(((m[0] * i + m[1] * j) % l) * l + (m[2] * i + m[3] * j) % l) as usize].clone()
            };
            assert_eq!(k.weil(&image(1, 0), &image(0, 1), l, da), k.inv(&zeta));
            let mut points = vec![];
            for i in 0..l {
                for j in 0..l {
                    let q = image(i, j);
                    let fp = k.frob(&ee[(i * l + j) as usize]);
                    let fc = coords(&ee, &fp, l);
                    assert_eq!(k.frob(&q), image(fc[0], fc[1]));
                    points.push(json!({"curve":ee[(i*l+j) as usize],"twist":q}));
                }
            }
            graphs.push(json!({"matrix":m,"points":points,"actual_pairing_direction_and_Galois_stability":true}));
        }
        let expected = extension_order(s, deg);
        assert_eq!(expected.to_string(), v["extension_order"].as_str().unwrap());
        let mut c = expected.clone();
        let mut val = 0;
        while (&c % l) == BigUint::from(0u32) {
            c /= l;
            val += 1;
        }
        assert_eq!(c.to_string(), v["cofactor"].as_str().unwrap());
        assert_eq!(v["ell_valuation"], val);
        let identity = [1, 0, 0, 1];
        assert!(!matrices.contains(&identity));
        assert_ne!(frob, tfrob);
        assert_eq!(
            table(&k, &e0, &e0, l, a)
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
                .len() as u64,
            l
        );
        let row = json!({"experiment":"E20","source":s.model(),"ell":l,"extension_degree":deg,"extension_modulus":k.modulus,"basis":[e0,e1],"twist_basis":[t0,t1],"Weil_pairing":zeta,"actual_Frobenius":frob,"actual_twist_Frobenius":tfrob,"actual_torsion_points":l*l,"anti_isometries":matrices,"graphs":graphs,"all_graph_points_Galois_stable":true,"independent_PARl_pairing_equal":true,"negative_controls":{"dependent_basis_rejected":true,"identity_wrong_pairing_rejected":true,"wrong_twist_Frobenius_sign_rejected":true},"constructor_point_trials":v["point_trials"],"constructor_stdout_sha256":digest(raw.as_bytes()),"extension_order":expected.to_string(),"canonical_extension_slug_registered":false,"quotient_or_maps_constructed":false,"full_E20_complete":false,"performance_claim":false,"status":"actual-higher-torsion-pairing-and-graph-certificate"});
        write_json(out.join(format!("{}-l{l}.json", s.name)), &row)?;
        rows.push(row);
    }
    if rows.is_empty() {
        return Err("no actual torsion certificate in constructor output".into());
    }
    Ok(rows)
}
