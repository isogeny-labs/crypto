//! Explicit symmetric orbit sections on an auxiliary mixed bundle of the scalar cover.
use crate::{
    large_field::{self as lf, E, F, P},
    round04::read,
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::path::Path;
fn decode(f: &F, v: &Value) -> E {
    E::from_biguint(
        &BigUint::parse_bytes(v.as_str().unwrap().as_bytes(), 16).unwrap(),
        f.m(),
    )
}
fn point(f: &F, v: &Value) -> P {
    P::Affine {
        x: decode(f, &v[0]),
        y: decode(f, &v[1]),
    }
}
fn rotation(c: &crate::binary_ecc::BinaryCurve, p: &[P]) -> Vec<P> {
    let sum = p.iter().fold(P::Infinity, |z, q| lf::plus(c, &z, q));
    vec![lf::neg(&sum), p[0].clone(), p[1].clone(), p[2].clone()]
}
fn sections(f: &F, c: &crate::binary_ecc::BinaryCurve, p: &[P]) -> Option<(Vec<E>, Vec<P>)> {
    let mut five = p.to_vec();
    five.push(lf::neg(
        &p.iter().fold(P::Infinity, |z, q| lf::plus(c, &z, q)),
    ));
    let mut e = vec![f.o()];
    for q in &five {
        let P::Affine { x, .. } = q else {
            return None;
        };
        e.push(f.z());
        for i in (1..e.len()).rev() {
            e[i] = e[i].add(&f.mul(x, &e[i - 1]));
        }
    }
    Some((e, five))
}
fn values(e: &[E]) -> Vec<String> {
    e.iter().map(|x| x.to_biguint().to_str_radix(16)).collect()
}
pub fn run(dir: &Path) {
    let f = F::new(168, 95);
    assert!(f.irreducible());
    let input = read(&dir.join("g17-odd-cover-kernel-and-commutators.json"));
    let mut records = vec![];
    for row in input["records"].as_array().unwrap() {
        let a = u64::from(row["trace"] == 1);
        let curve = f.curve(f.e(a), f.o());
        let mut torsion = vec![];
        for t in row["torsion_bases"].as_array().unwrap() {
            for key in ["P", "Q"] {
                let t = point(&f, &t[key]);
                assert_eq!(lf::times(&curve, &t, 105), P::Infinity);
                torsion.push(t);
            }
        }
        assert_eq!(torsion.len(), 6);
        let mut tuples = vec![];
        let mut rejected = vec![];
        for probe in 0..512 {
            let mut tuple = vec![];
            for j in 0..4 {
                let mut x = 1001 + 101 * probe + 17 * j;
                loop {
                    if let Some(p) = f.point(&curve, &f.e(x)) {
                        tuple.push(p);
                        break;
                    }
                    x += 1;
                    assert!(x < 100000);
                }
            }
            let q = tuple
                .iter()
                .map(|p| lf::times(&curve, p, 105))
                .collect::<Vec<_>>();
            let Some((es, five)) = sections(&f, &curve, &q) else {
                rejected.push(probe);
                continue;
            };
            let rotated = rotation(&curve, &q);
            let (esr, fiver) = sections(&f, &curve, &rotated).unwrap();
            assert_eq!(es, esr);
            assert_eq!(
                fiver,
                vec![
                    five[4].clone(),
                    five[0].clone(),
                    five[1].clone(),
                    five[2].clone(),
                    five[3].clone()
                ]
            );
            let frob = tuple.iter().map(|p| f.frobp(p, 1)).collect::<Vec<_>>();
            let twisted = rotation(&curve, &frob);
            let tq = twisted
                .iter()
                .map(|p| lf::times(&curve, p, 105))
                .collect::<Vec<_>>();
            let (efs, _) = sections(&f, &curve, &tq).unwrap();
            assert_eq!(efs, es.iter().map(|z| f.sq(z)).collect::<Vec<_>>());
            let mut controls = 0;
            for j in 0..4 {
                for t in &torsion {
                    let p = lf::plus(&curve, &tuple[j], t);
                    let changed = lf::times(&curve, &p, 105);
                    assert_eq!(changed, q[j]);
                    let mut shifted = q.clone();
                    shifted[j] = changed;
                    assert_eq!(sections(&f, &curve, &shifted).unwrap().0, es);
                    controls += 1;
                }
            }
            tuples.push(json!({"probe":probe,"input":tuple.iter().map(lf::pj).collect::<Vec<_>>(),"multiplication105":q.iter().map(lf::pj).collect::<Vec<_>>(),"five_target_points":five.iter().map(lf::pj).collect::<Vec<_>>(),"sections_e0_e5":values(&es),"twisted_Frobenius_sections":values(&efs),"rotation_checked":true,"torsion_translate_checks":controls}));
            if tuples.len() == 16 {
                break;
            }
        }
        assert_eq!(tuples.len(), 16);
        assert!(tuples
            .windows(2)
            .any(|v| v[0]["sections_e0_e5"] != v[1]["sections_e0_e5"]));
        println!("G22:a{a},16 mixed orbit-section tuples,384 actual torsion translates,rotation and twisted Frobenius controls pass");
        records.push(json!({"graph":row["graph"],"ordinary_a":a,"tuples":tuples,"poles_rejected":rejected,"torsion_basis_controls":6,"torsion_translate_controls":384}));
    }
    let dimension = BigUint::from(80u32) * BigUint::from(105u32).pow(8);
    assert_ne!(&dimension % BigUint::from(361u32), BigUint::from(0u32));
    let companion = read(&dir.join("g16-diagonal-cover-section-bases.json"));
    assert_eq!(companion["records"][0]["factor_bases"][4]["degree"], 57);
    let full_dimension = &dimension * BigUint::from(57u32);
    assert_ne!(&full_dimension % BigUint::from(361u32), BigUint::from(0u32));
    crate::round02::save(
        dir,
        "g22-mixed-F2-orbit-sections.json",
        &json!({"schema":"koblitz.round19/g22-v1","field_degree":168,"modulus_low":95,"sections":"coefficients e0..e5 of product_i(1+x(Q_i)t), Q_i=[105]P_i, Q5=-sum Q1..Q4","auxiliary_divisor":"[105]^*(2 sum_i ell_i^*[O])","polarization_matrix":[[4,2,2,2],[2,4,2,2],[2,2,4,2],[2,2,2,4]],"h0_before_scalar_pullback":80,"h0_after_scalar_pullback":dimension.to_string(),"companion_degree":57,"full_product_h0":full_dimension.to_string(),"section_count_constructed":6,"coherent_auxiliary_F2_sections_under_divisor_bridge":true,"records":records,"canonical_3L_bundle_identified":false,"requested_243_sections_extracted":false,"second_19_kernel_descent":"excluded for the full auxiliary product by361 not dividing57*80*105^8;changing a character cannot fix its polarization degree","scope":"actual rational orbit functions and common mixed pole bound on the stable scalar cover;not the requested canonical theta basis or proper quotient embedding"}),
    );
}

type Cm = (i64, i64);
fn ca(x: Cm, y: Cm) -> Cm {
    (x.0 + y.0, x.1 + y.1)
}
fn cn(x: Cm) -> Cm {
    (-x.0, -x.1)
}
fn cb(x: Cm) -> Cm {
    (x.0 - x.1, -x.1)
}
fn cm(x: Cm, y: Cm) -> Cm {
    (x.0 * y.0 - 2 * x.1 * y.1, x.0 * y.1 + x.1 * y.0 - x.1 * y.1)
}
fn rowrot(x: [Cm; 4]) -> [Cm; 4] {
    [
        ca(cn(x[0]), x[1]),
        ca(cn(x[0]), x[2]),
        ca(cn(x[0]), x[3]),
        cn(x[0]),
    ]
}
pub fn polarization(dir: &Path) {
    let input = read(&dir.join("g16-diagonal-cover-section-bases.json"));
    let mut records = vec![];
    for g in 0..2 {
        let h = std::array::from_fn::<_, 4, _>(|i| {
            std::array::from_fn::<_, 4, _>(|j| {
                let z = &input["records"][g]["H"][i][j];
                (z["a"].as_i64().unwrap(), z["b"].as_i64().unwrap())
            })
        });
        let trace = (0..4).map(|i| h[i][i].0).sum::<i64>();
        let mut fits = vec![];
        let mut filtered = 0;
        let mut subbox = 0;
        let mut subfits = 0;
        for code in 0..390625usize {
            let mut n = code;
            let coeff = std::array::from_fn::<_, 8, _>(|_| {
                let z = n % 5;
                n /= 5;
                z as i64 - 2
            });
            let small = coeff.iter().all(|a| a.abs() <= 1);
            if small {
                subbox += 1;
            }
            let row = std::array::from_fn::<_, 4, _>(|i| (coeff[2 * i], coeff[2 * i + 1]));
            let mut orbit = vec![row];
            for k in 1..5 {
                orbit.push(rowrot(orbit[k - 1]));
            }
            assert_eq!(rowrot(orbit[4]), row);
            let tr = orbit
                .iter()
                .map(|r| r.iter().map(|x| cm(cb(*x), *x).0).sum::<i64>())
                .sum::<i64>();
            if tr != trace {
                continue;
            }
            filtered += 1;
            let gram = std::array::from_fn::<_, 4, _>(|i| {
                std::array::from_fn::<_, 4, _>(|j| {
                    orbit.iter().fold((0, 0), |z, r| ca(z, cm(cb(r[i]), r[j])))
                })
            });
            if gram == h {
                if small {
                    subfits += 1;
                }
                fits.push(json!({"row":row,"orbit":orbit,"Gram":gram}));
            }
        }
        assert_eq!(subbox, 6561);
        println!("G23:H{g},390625 rows,6561 in subbox,{filtered} trace matches,{} full orbit-Gram matches",fits.len());
        records.push(json!({"H_graph":g,"H":h,"coefficient_box":[-2,2],"rows_examined":390625,"subbox_rows":subbox,"subbox_matches":subfits,"trace_candidates":filtered,"matches":fits,"scope":"bounded single5-orbit Gram decomposition only;does not test sums of orbits,larger coefficients or fractional CM maps"}));
    }
    crate::round02::save(
        dir,
        "g23-integral-orbit-polarization-search.json",
        &json!({"schema":"koblitz.round19/g23-v1","CM_relation":"w²+w+2=0;bar(w)=-1-w","records":records,"canonical_Pic2_bundle_identified":false,"final_19_kernel_action_computed":false}),
    );
}

pub fn obstruction(dir: &Path) {
    let input = read(&dir.join("g16-diagonal-cover-section-bases.json"));
    let r = vec![
        vec![-1i64, -1, -1, -1],
        vec![1, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 1, 0],
    ];
    let mut hs = vec![];
    let mut products = vec![];
    let lit = |m: &Vec<Vec<i64>>| {
        format!(
            "!![{}]",
            m.iter()
                .map(|r| r
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(","))
                .collect::<Vec<_>>()
                .join(";")
        )
    };
    for g in 0..2 {
        let h = (0..4)
            .map(|i| {
                (0..4)
                    .map(|j| input["records"][g]["H"][i][j]["a"].as_i64().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let hv = h
            .iter()
            .map(|r| r.iter().sum::<i64>().rem_euclid(5))
            .collect::<Vec<_>>();
        assert!(hv.iter().any(|x| *x != 0));
        let three = hv.iter().map(|x| 3 * x % 5).collect::<Vec<_>>();
        assert!(three.iter().any(|x| *x != 0));
        products.push(json!({"H_graph":g,"real_coefficient_matrix":h,"H_times_fixed_vector_mod5":hv,"three_H_times_fixed_vector_mod5":three}));
        hs.push(h);
    }
    let mut lean=format!("import KoblitzRound19\nnamespace KoblitzRound19Polarization\nopen scoped BigOperators\ndef R : Matrix (Fin 4) (Fin 4) (ZMod 5) := {}\ndef v : Fin 4 → ZMod 5 := fun _ => 1\ndef H0 : Matrix (Fin 4) (Fin 4) (ZMod 5) := {}\ndef H1 : Matrix (Fin 4) (Fin 4) (ZMod 5) := {}\ndef transfer (M:Matrix (Fin 4) (Fin 4) (ZMod 5)) := ∑ j:Fin 5,(R^j.val).transpose*M*(R^j.val)\ntheorem actual_cycle : R^5=1 ∧ (∑ j:Fin 5,(R^j.val).transpose)=0 ∧ R.mulVec v=v := by decide +kernel\ntheorem transfer_annihilates (M:Matrix (Fin 4) (Fin 4) (ZMod 5)) : (transfer M).mulVec v=0 := by\n have fixed (k:Nat) : (R^k).mulVec v=v := by\n  induction k with\n  | zero => simp\n  | succ k hk => rw [pow_succ,← Matrix.mulVec_mulVec,actual_cycle.2.2,hk]\n unfold transfer\n rw [Matrix.sum_mulVec]\n simp_rw [← Matrix.mulVec_mulVec,fixed]\n rw [← Matrix.sum_mulVec,actual_cycle.2.1]\n simp\ntheorem sum_transfers (M N:Matrix (Fin 4) (Fin 4) (ZMod 5)) : transfer (M+N)=transfer M+transfer N := by\n simp [transfer,mul_add,add_mul,Finset.sum_add_distrib]\n",lit(&r),lit(&hs[0]),lit(&hs[1]));
    for g in 0..2 {
        lean.push_str(&format!("theorem actual_H{g}_nonzero : H{g}.mulVec v≠0 ∧ ((3:ZMod 5) • H{g}).mulVec v≠0 := by decide +kernel\ntheorem no_transfer_H{g} : ¬ ∃ M:Matrix (Fin 4) (Fin 4) (ZMod 5), transfer M=H{g} ∨ transfer M=(3:ZMod 5) • H{g} := by\n rintro ⟨M,hM⟩\n have hz:=transfer_annihilates M\n rcases hM with hM|hM\n · exact actual_H{g}_nonzero.1 (hM ▸ hz)\n · exact actual_H{g}_nonzero.2 (hM ▸ hz)\n"));
    }
    lean.push_str("end KoblitzRound19Polarization\n");
    for name in [
        "actual_cycle",
        "transfer_annihilates",
        "sum_transfers",
        "actual_H0_nonzero",
        "actual_H1_nonzero",
        "no_transfer_H0",
        "no_transfer_H1",
    ] {
        lean.push_str(&format!(
            "#print axioms KoblitzRound19Polarization.{name}\n"
        ));
    }
    let p = dir.join("KoblitzRound19Polarization.lean");
    assert!(!p.exists());
    std::fs::write(p, lean).unwrap();
    println!(
        "G24:actual H0,H1 and3H0,3H1 are outside the integral5-orbit transfer lattice modulo5"
    );
    crate::round02::save(
        dir,
        "g24-orbit-transfer-mod5-obstruction.json",
        &json!({"schema":"koblitz.round19/g24-v1","R":r,"modulus":5,"fixed_vector":[1,1,1,1],"records":products,"structural_bridge":"real CM coefficient of every integral Hermitian orbit Gram is a matrix transfer;finite sums are another transfer","scope":"all integral orbit-sum Gram decompositions of H or3H excluded in this actual augmentation basis;arbitrary invariant divisors,fractional maps and general sections are not excluded"}),
    );
}

fn act(f: &F, c: &crate::binary_ecc::BinaryCurve, p: &[P], row: &[Cm; 4], a: u64) -> P {
    let times = |p: &P, n: i64| {
        let q = lf::times(c, p, n.unsigned_abs());
        if n < 0 {
            lf::neg(&q)
        } else {
            q
        }
    };
    row.iter().enumerate().fold(P::Infinity, |z, (j, (aa, b))| {
        let mut w = f.frobp(&p[j], 1);
        if a == 1 {
            w = lf::neg(&w);
        }
        lf::plus(c, &z, &lf::plus(c, &times(&p[j], *aa), &times(&w, *b)))
    })
}
fn es(f: &F, xs: &[E]) -> Vec<E> {
    let mut z = vec![f.o()];
    for x in xs {
        z.push(f.z());
        for i in (1..z.len()).rev() {
            z[i] = z[i].add(&f.mul(x, &z[i - 1]));
        }
    }
    z
}
fn cover_sections(
    f: &F,
    c: &crate::binary_ecc::BinaryCurve,
    p: &[P],
    orbits: &[[[Cm; 4]; 5]; 4],
    a: u64,
) -> Option<(Vec<E>, Vec<Vec<P>>)> {
    let mut output = vec![f.o()];
    let mut points = vec![];
    for orbit in orbits {
        let q = orbit.iter().map(|r| act(f, c, p, r, a)).collect::<Vec<_>>();
        let mut x = vec![];
        let mut y = vec![];
        for v in &q {
            let P::Affine { x: xx, y: yy } = v else {
                return None;
            };
            x.push(xx.clone());
            y.push(yy.clone());
        }
        output.extend(es(f, &x).into_iter().skip(1));
        output.extend(es(f, &y).into_iter().skip(1));
        points.push(q);
    }
    assert_eq!(output.len(), 41);
    Some((output, points))
}
pub fn cover(dir: &Path) {
    let refinement = read(&dir.join("g21-rotation-compatible-scalar-refinement.json"));
    let source = read(&dir.join("g16-diagonal-cover-section-bases.json"));
    let h = std::array::from_fn::<_, 4, _>(|i| {
        std::array::from_fn::<_, 4, _>(|j| {
            let c = &source["records"][0]["H"][i][j];
            (c["a"].as_i64().unwrap(), c["b"].as_i64().unwrap())
        })
    });
    let rows = std::array::from_fn::<_, 4, _>(|i| {
        std::array::from_fn::<_, 4, _>(|j| {
            let c = &refinement["C_CM_columns"][j][i];
            let (a, b): (i64, i64) = (
                c["a"].as_str().unwrap().parse().unwrap(),
                c["b"].as_str().unwrap().parse().unwrap(),
            );
            let d = if i < 2 { 5 } else { 1 };
            assert_eq!(a % d, 0);
            assert_eq!(b % d, 0);
            (a / d, b / d)
        })
    });
    let weights = [45i64, 315, 63, 171];
    let orbits = rows.map(|r| {
        let mut out = [r; 5];
        for j in 1..5 {
            out[j] = rowrot(out[j - 1]);
        }
        assert_eq!(rowrot(out[4]), out[0]);
        out
    });
    let gram = std::array::from_fn::<_, 4, _>(|i| {
        std::array::from_fn::<_, 4, _>(|j| {
            (0..4).fold((0, 0), |z, k| {
                ca(
                    z,
                    orbits[k].iter().fold((0, 0), |z, r| {
                        let x = cm(cb(r[i]), r[j]);
                        ca(z, (weights[k] * x.0, weights[k] * x.1))
                    }),
                )
            })
        })
    });
    assert_eq!(gram, h.map(|r| r.map(|x| (33075 * x.0, 33075 * x.1))));
    let f = F::new(168, 95);
    let g22 = read(&dir.join("g22-mixed-F2-orbit-sections.json"));
    let torsion = read(&dir.join("g17-odd-cover-kernel-and-commutators.json"));
    let mut records = vec![];
    for (sign, row) in g22["records"].as_array().unwrap().iter().enumerate() {
        let a = row["ordinary_a"].as_u64().unwrap();
        let curve = f.curve(f.e(a), f.o());
        let mut probes = vec![];
        let mut poles = vec![];
        let mut translates = vec![];
        for (id, t) in row["tuples"].as_array().unwrap().iter().enumerate() {
            let p = t["input"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| point(&f, v))
                .collect::<Vec<_>>();
            let Some((s, q)) = cover_sections(&f, &curve, &p, &orbits, a) else {
                poles.push(id);
                continue;
            };
            let rot = rotation(&curve, &p);
            assert_eq!(cover_sections(&f, &curve, &rot, &orbits, a).unwrap().0, s);
            let fp = rotation(&curve, &p.iter().map(|v| f.frobp(v, 1)).collect::<Vec<_>>());
            let (fs, _) = cover_sections(&f, &curve, &fp, &orbits, a).unwrap();
            assert_eq!(fs, s.iter().map(|x| f.sq(x)).collect::<Vec<_>>());
            if probes.is_empty() {
                for j in 0..4 {
                    for t in torsion["records"][sign]["torsion_bases"]
                        .as_array()
                        .unwrap()
                    {
                        for key in ["P", "Q"] {
                            let mut shifted = p.clone();
                            shifted[j] = lf::plus(&curve, &shifted[j], &point(&f, &t[key]));
                            if let Some((after, _)) =
                                cover_sections(&f, &curve, &shifted, &orbits, a)
                            {
                                let diff = s.iter().zip(&after).position(|(x, y)| x != y);
                                translates.push(json!({"coordinate":j,"prime":t["prime"],"basis":key,"first_different_section":diff,"nonzero_difference":diff.map(|i|s[i].add(&after[i]).to_biguint().to_str_radix(16)),"shifted_input":shifted.iter().map(lf::pj).collect::<Vec<_>>(),"after_sections":values(&after)}));
                            } else {
                                translates.push(json!({"coordinate":j,"prime":t["prime"],"basis":key,"pole":true}));
                            }
                        }
                    }
                }
            }
            probes.push(json!({"G22_probe":id,"points":q.iter().map(|q|q.iter().map(lf::pj).collect::<Vec<_>>()).collect::<Vec<_>>(),"sections":values(&s),"twisted_Frobenius_sections":values(&fs)}));
        }
        assert!(!probes.is_empty());
        let failed = translates
            .iter()
            .filter(|r| !r["first_different_section"].is_null())
            .count();
        println!("G25:a{a},{} valid tuples,41 correct-polarization mixed orbit sections,{} torsion noninvariance witnesses",probes.len(),failed);
        records.push(json!({"ordinary_a":a,"graph":row["graph"],"probes":probes,"pole_rejections":poles,"translation_diagnostics":translates,"ratios_invariant_under_all_tested_scalar_kernel_translates":translates.iter().all(|r|r["pole"]!=true && r["first_different_section"].is_null())}));
    }
    let cmstr = |r: &[Cm; 4]| {
        format!(
            "![{}]",
            r.iter()
                .map(|(a, b)| format!("({a},{b})"))
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    let array = orbits
        .iter()
        .map(|orbit| {
            format!(
                "![{}]",
                orbit.iter().map(cmstr).collect::<Vec<_>>().join(",")
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let hstr = h.iter().map(cmstr).collect::<Vec<_>>().join(",");
    let lean=format!("import KoblitzRound19Polarization\nimport KoblitzRound18Refinement\nnamespace KoblitzRound19OrbitCover\nopen scoped BigOperators\nabbrev CM := ℤ×ℤ\ndef cmul (x y:CM):CM := (x.1*y.1-2*x.2*y.2,x.1*y.2+x.2*y.1-x.2*y.2)\ndef bar (x:CM):CM := (x.1-x.2,-x.2)\ndef scale (n:ℤ) (x:CM):CM := (n*x.1,n*x.2)\ndef rows : Fin 4→Fin 5→Fin 4→CM := ![{array}]\ndef H : Fin 4→Fin 4→CM := ![{hstr}]\ndef weights : Fin 4→ℤ := ![45,315,63,171]\ndef gram (i j:Fin 4):CM := ∑ k:Fin 4,∑ s:Fin 5,scale (weights k) (cmul (bar (rows k s i)) (rows k s j))\ndef rotateRow (r:Fin 4→CM):Fin 4→CM := ![-r 0+r 1,-r 0+r 2,-r 0+r 3,-r 0]\ndef ix (i:Fin 4) (b:Fin 2):Fin 8 := ⟨2*i.val+b.val,by omega⟩\ntheorem actual_weighted_Gram : ∀i j:Fin 4,gram i j=scale 33075 (H i j) := by decide +kernel\ntheorem actual_orbits : ∀k:Fin 4,∀s:Fin 5,rotateRow (rows k s)=rows k (s+1) := by decide +kernel\ntheorem actual_weight_bounds : ∀k:Fin 4,3<weights k := by decide +kernel\ntheorem actual_refinement_binding : ∀i j:Fin 4,scale (if i.val<2 then 5 else 1) (rows i 0 j)=(KoblitzRound18Refinement.C (ix i 0) (ix j 0),KoblitzRound18Refinement.C (ix i 1) (ix j 0)) := by decide +kernel\nend KoblitzRound19OrbitCover\n#print axioms KoblitzRound19OrbitCover.actual_weighted_Gram\n#print axioms KoblitzRound19OrbitCover.actual_orbits\n#print axioms KoblitzRound19OrbitCover.actual_weight_bounds\n#print axioms KoblitzRound19OrbitCover.actual_refinement_binding\n");
    let p = dir.join("KoblitzRound19OrbitCover.lean");
    assert!(!p.exists());
    std::fs::write(p, lean).unwrap();
    crate::round02::save(
        dir,
        "g25-correct-polarization-orbit-sections.json",
        &json!({"schema":"koblitz.round19/g25-v1","graphs":[0,2],"rows":rows,"orbits":orbits,"weights":weights,"actual_Gram":gram,"target_polarization_scalar":33075,"field_degree":168,"modulus_low":95,"forced_fixed_divisor_weights":weights.map(|n|n-3),"target":"3*105^2 H on the stable scalar cover","constructed_sections":41,"ordinary_divisor":"sum_i weight_i sum_j(ell_i R^j)^*[O]","companion_divisor":"57[O]","records":records,"canonical_symmetric_Pic2_class_matched":false,"theta_kernel_actions_constructed":false,"final_243_sections_extracted":false,"scope":"explicit F2-invariant mixed effective divisor with correct pulled-back polarization and41 evaluated sections;canonical bundle class/kernel descent/proper embedding still open;G24 primitive obstruction does not apply to scalar105 pullback"}),
    );
}
