//! A scalar, rotation-compatible refinement of the obstructed diagonal cover.
use super::*;
use crate::large_field as lf;
fn decode(f: &lf::F, v: &Value) -> lf::E {
    lf::E::from_biguint(
        &num_bigint::BigUint::parse_bytes(v.as_str().unwrap().as_bytes(), 16).unwrap(),
        f.m(),
    )
}
fn point(f: &lf::F, v: &Value) -> lf::P {
    lf::P::Affine {
        x: decode(f, &v[0]),
        y: decode(f, &v[1]),
    }
}
fn fromv(v: &Value) -> M {
    v.as_array()
        .unwrap()
        .iter()
        .map(|r| {
            r.as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().parse().unwrap())
                .collect()
        })
        .collect()
}
fn scale(a: &M, k: &Z) -> M {
    a.iter()
        .map(|r| r.iter().map(|x| x * k).collect())
        .collect()
}
fn modulo(a: &M, m: i64) -> M {
    a.iter()
        .map(|r| r.iter().map(|x| Z::from(red(x, m))).collect())
        .collect()
}
fn powmod(a: &M, mut k: u64, m: i64) -> M {
    let mut b = a.clone();
    let mut z = ident(a.len());
    while k > 0 {
        if k % 2 == 1 {
            z = modulo(&mm(&z, &b), m);
        }
        b = modulo(&mm(&b, &b), m);
        k /= 2;
    }
    z
}
fn cm_columns(a: &M) -> Value {
    let columns = (0..4)
        .map(|j| {
            (0..4)
                .map(|i| {
                    let aa = &a[2 * i][2 * j];
                    let b = &a[2 * i + 1][2 * j];
                    assert_eq!(&a[2 * i][2 * j + 1], &(-2 * b));
                    assert_eq!(&a[2 * i + 1][2 * j + 1], &(aa - b));
                    json!({"a":aa.to_string(),"b":b.to_string()})
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    json!(columns)
}
fn cm_action(
    f: &lf::F,
    c: &crate::binary_ecc::BinaryCurve,
    m: &M,
    p: &[lf::P],
    trace: i64,
) -> Vec<lf::P> {
    let times = |p: &lf::P, n: i64| {
        let q = lf::times(c, p, n.unsigned_abs());
        if n < 0 {
            lf::neg(&q)
        } else {
            q
        }
    };
    (0..4)
        .map(|i| {
            (0..4).fold(lf::P::Infinity, |z, j| {
                let a = m[2 * i][2 * j].to_i64().unwrap();
                let b = m[2 * i + 1][2 * j].to_i64().unwrap();
                let mut w = f.frobp(&p[j], 1);
                if trace == 1 {
                    w = lf::neg(&w);
                }
                lf::plus(c, &z, &lf::plus(c, &times(&p[j], a), &times(&w, b)))
            })
        })
        .collect()
}
pub fn run(dir: &Path) {
    let g20 = read(&dir.join("g20-odd-cover-rotation-lift-obstruction.json"));
    let b = fromv(&g20["B"]);
    let ba = fromv(&g20["B_adjugate"]);
    let r = fromv(&g20["R"]);
    let d = det(b.clone());
    let least = ba
        .iter()
        .flatten()
        .map(|a| &d / d.gcd(a))
        .fold(Z::one(), |a, b| a.lcm(&b));
    assert_eq!(least, Z::from(105));
    let c = ba
        .iter()
        .map(|r| {
            r.iter()
                .map(|a| {
                    assert!((a * &least % &d).is_zero());
                    a * &least / &d
                })
                .collect()
        })
        .collect::<M>();
    let scalar = scale(&ident(8), &least);
    assert_eq!(mm(&b, &c), scalar);
    assert_eq!(mm(&c, &b), scalar);
    assert_eq!(mm(&r, &scalar), mm(&scalar, &r));
    let c_cm = cm_columns(&c);
    let _b_cm = cm_columns(&b);
    let degc = det(c.clone());
    let stable_degree = least.pow(8);
    assert_eq!(&d * &degc, stable_degree);
    let section_count = &stable_degree * Z::from(87723);
    let k = read(&dir.join("g17-odd-cover-kernel-and-commutators.json"));
    let f = lf::F::new(168, 95);
    assert!(f.irreducible());
    let mut records = vec![];
    let mut lean=format!("import KoblitzRound18\nnamespace KoblitzRound18Refinement\ndef B : Matrix (Fin 8) (Fin 8) ℤ := {}\ndef C : Matrix (Fin 8) (Fin 8) ℤ := {}\ndef R : Matrix (Fin 8) (Fin 8) ℤ := {}\ntheorem actual_scalar_refinement : B*C=(105:ℤ) • 1 ∧ C*B=(105:ℤ) • 1 := by decide +kernel\ntheorem actual_rotation_compatibility : R^5=1 ∧ R*((105:ℤ) • 1)=((105:ℤ) • 1)*R := by decide +kernel\n",lit(&b,false),lit(&c,false),lit(&r,false));
    for (sign, row) in k["records"].as_array().unwrap().iter().enumerate() {
        let curve = f.curve(f.e(u64::from(row["trace"] == 1)), f.o());
        let mut local = vec![];
        let mut mats = vec![];
        for t in row["torsion_bases"].as_array().unwrap() {
            let p = point(&f, &t["P"]);
            let q = point(&f, &t["Q"]);
            let ell = t["prime"].as_u64().unwrap();
            let mut mat = vec![vec![Z::zero(); 2]; 2];
            for (j, v) in [&p, &q].iter().enumerate() {
                let target = f.frobp(v, 1);
                let mut coordinates = None;
                for a in 0..ell {
                    for b in 0..ell {
                        if lf::plus(&curve, &lf::times(&curve, &p, a), &lf::times(&curve, &q, b))
                            == target
                        {
                            coordinates = Some([a, b]);
                        }
                    }
                }
                let z = coordinates.unwrap();
                mat[0][j] = z[0].into();
                mat[1][j] = z[1].into();
            }
            local.push(json!({"prime":ell,"matrix":matv(&mat)}));
            mats.push((ell, mat));
        }
        let pi = (0..2)
            .map(|i| {
                (0..2)
                    .map(|j| {
                        Z::from(
                            (0..105i64)
                                .find(|a| {
                                    mats.iter().all(|(ell, m)| {
                                        *a % *ell as i64 == red(&m[i][j], *ell as i64)
                                    })
                                })
                                .unwrap(),
                        )
                    })
                    .collect()
            })
            .collect::<M>();
        assert_eq!(powmod(&pi, 168, 105), ident(2));
        for p in [2, 3, 7] {
            assert_ne!(powmod(&pi, 168 / p, 105), ident(2));
        }
        let action = (0..8)
            .map(|i| {
                (0..8)
                    .map(|j| {
                        Z::from(red(
                            &(&r[2 * (i / 2)][2 * (j / 2)] * &pi[i % 2][j % 2]),
                            105,
                        ))
                    })
                    .collect()
            })
            .collect::<M>();
        assert_eq!(powmod(&action, 840, 105), ident(8));
        for p in [2, 3, 5, 7] {
            assert_ne!(powmod(&action, 840 / p, 105), ident(8));
        }
        lean.push_str(&format!("def pi{sign} : Matrix (Fin 2) (Fin 2) (ZMod 105) := {}\ntheorem actual_torsion_period{sign} : pi{sign}^168=1 ∧ pi{sign}^84≠1 ∧ pi{sign}^56≠1 ∧ pi{sign}^24≠1 := by decide +kernel\n",lit(&pi,false)));
        let mut point_controls = vec![];
        for probe in 0..4 {
            let tuple = (0..4)
                .map(|j| {
                    let mut k = 1001 + 101 * probe + 17 * j;
                    loop {
                        if let Some(p) = f.point(&curve, &f.e(k)) {
                            break p;
                        }
                        k += 1;
                        assert!(k < 20000);
                    }
                })
                .collect::<Vec<_>>();
            let bc = cm_action(
                &f,
                &curve,
                &b,
                &cm_action(&f, &curve, &c, &tuple, row["trace"].as_i64().unwrap()),
                row["trace"].as_i64().unwrap(),
            );
            let cb = cm_action(
                &f,
                &curve,
                &c,
                &cm_action(&f, &curve, &b, &tuple, row["trace"].as_i64().unwrap()),
                row["trace"].as_i64().unwrap(),
            );
            let expected = tuple
                .iter()
                .map(|p| lf::times(&curve, p, 105))
                .collect::<Vec<_>>();
            assert_eq!(bc, expected);
            assert_eq!(cb, expected);
            point_controls.push(json!({"input":tuple.iter().map(lf::pj).collect::<Vec<_>>(),"B_C":bc.iter().map(lf::pj).collect::<Vec<_>>(),"C_B":cb.iter().map(lf::pj).collect::<Vec<_>>(),"multiplication105":expected.iter().map(lf::pj).collect::<Vec<_>>()}));
        }
        records.push(json!({"graph":row["graph"],"trace":row["trace"],"actual_prime_Frobenius_matrices":local,"CRT_Frobenius_mod105":matv(&pi),"CRT_Frobenius_order":168,"trace_zero_Frobenius_mod105":matv(&action),"trace_zero_Frobenius_order":840,"proper_prime_divisor_checks":[420,280,168,120],"full_geometric_kernel":"E[105]^4,with semilinear trace-zero R rotation","pointwise_CM_composition_controls":point_controls,"field_identification_in_F2_840_evaluated":false}));
    }
    lean.push_str("end KoblitzRound18Refinement\n#print axioms KoblitzRound18Refinement.actual_scalar_refinement\n#print axioms KoblitzRound18Refinement.actual_rotation_compatibility\n#print axioms KoblitzRound18Refinement.actual_torsion_period0\n#print axioms KoblitzRound18Refinement.actual_torsion_period1\n");
    let path = dir.join("KoblitzRound18Refinement.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    println!("G21:C=105 B^-1 integral;B C=105 I;stable degree{stable_degree},refinement degree{degc};actual trace-zero105-torsion Frobenius order840");
    crate::round02::save(
        dir,
        "g21-rotation-compatible-scalar-refinement.json",
        &json!({"schema":"koblitz.round18/g21-v1","C_integral8":matv(&c),"C_CM_columns":c_cm,"B_C_and_C_B_scalar_checked":true,"least_scalar_for_B_inverse":105,"old_cover_degree":d.to_string(),"refinement_degree":degc.to_string(),"stable_composite_degree":stable_degree.to_string(),"derived_section_count":section_count.to_string(),"stable_cover":"multiplication105 on the actual trace-zero source,commuting with R;the pulled-back polarization is mixed","records":records,"F2_invariant_sections_extracted":false,"scope":"explicit integral refinement and Galois module data under the written split trace-zero bridge;not a descended diagonal basis or243 evaluated target sections"}),
    );
}
