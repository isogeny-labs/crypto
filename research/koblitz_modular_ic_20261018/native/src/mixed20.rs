//! Actual Pic2 comparison and complete common kernels for the weighted orbit system.
use super::*;
use serde_json::json;
fn save_lean(dir: &Path, name: &str, text: String) {
    assert!(!dir.join(name).exists());
    std::fs::write(dir.join(name), text).unwrap();
}
pub fn picard(dir: &Path) {
    let data = read(&dir.join("g25-correct-polarization-orbit-sections.json"));
    let prior = read(&dir.join("g15-theta-class-descent.json"));
    let mut bias = [0i64; 4];
    for k in 0..4 {
        for j in 0..5 {
            for (i, z) in bias.iter_mut().enumerate() {
                let a = data["orbits"][k][j][i][0].as_i64().unwrap();
                let b = data["orbits"][k][j][i][1].as_i64().unwrap();
                *z = (*z + data["weights"][k].as_i64().unwrap() * a * b).rem_euclid(2);
            }
        }
    }
    let bits = bias
        .iter()
        .enumerate()
        .fold(0u64, |z, (i, b)| z | ((*b as u64) << i));
    let p = [[1u8, 1, 0, 0], [1, 0, 1, 0], [1, 0, 0, 1], [1, 0, 0, 0]];
    let fixed = (0u8..16)
        .filter(|v| {
            (0..4).all(|i| {
                let y = (0..4).fold(0, |z, j| z ^ (p[i][j] * ((v >> j) & 1)));
                y == ((v >> i) & 1)
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(fixed, vec![0]);
    let records=[0,2].map(|g| {
        let correction=prior["records"][g]["actual_theta_correction_bits"].as_u64().unwrap();
        assert_eq!(bits,correction);
        json!({"graph":g,"orbit_axis_bias":bias,"target_correction_bits":correction,"difference_bits":bits^correction,"canonical_class_matches_under_written_Pic0_bridge":true})
    });
    let lean=String::from("import KoblitzRound19OrbitCover\nimport KoblitzRound15Theta\nnamespace KoblitzRound20Picard\nopen scoped BigOperators\ndef orbitBias (i:Fin 4) : ZMod 2 := ∑ k:Fin 4,∑ j:Fin 5,((KoblitzRound19OrbitCover.weights k)*(KoblitzRound19OrbitCover.rows k j i).1*(KoblitzRound19OrbitCover.rows k j i).2:ℤ)\ntheorem actual_axis_match0 : orbitBias=KoblitzRound15Theta.corr0 := by decide +kernel\ntheorem actual_axis_match2 : orbitBias=KoblitzRound15Theta.corr2 := by decide +kernel\ntheorem no_fixed_Pic2_0 : ∀v:Fin 4→ZMod 2,KoblitzRound15Theta.pic0.mulVec v=v→v=0 := by decide +kernel\ntheorem no_fixed_Pic2_2 : ∀v:Fin 4→ZMod 2,KoblitzRound15Theta.pic2.mulVec v=v→v=0 := by decide +kernel\ntheorem canonical_difference_vanishes (d:Fin 4→ZMod 2) (h:KoblitzRound15Theta.pic0.mulVec d=d) : d=0 := no_fixed_Pic2_0 d h\nend KoblitzRound20Picard\n#print axioms KoblitzRound20Picard.actual_axis_match0\n#print axioms KoblitzRound20Picard.actual_axis_match2\n#print axioms KoblitzRound20Picard.no_fixed_Pic2_0\n#print axioms KoblitzRound20Picard.no_fixed_Pic2_2\n#print axioms KoblitzRound20Picard.canonical_difference_vanishes\n");
    save_lean(dir, "KoblitzRound20Picard.lean", lean);
    crate::round02::save(
        dir,
        "g26-canonical-orbit-Pic2-matching.json",
        &json!({"schema":"koblitz.round20/g26-v1","records":records,"Pic2_Frobenius":p,"all_fixed_vectors":fixed,"scope":"actual axis biases and no fixed geometric Pic0[2] class;written symmetric/F2 line-bundle comparison;no explicit rational trivializing function or full Lean geometry"}),
    );
    println!("G26:graphs0/2 actual orbit axis bias{bits};canonical correction matches;only zero fixed Pic2 vector");
}
fn modpow(mut x: i64, mut n: i64, p: i64) -> i64 {
    let mut y = 1;
    while n > 0 {
        if n & 1 != 0 {
            y = y * x % p;
        }
        x = x * x % p;
        n >>= 1;
    }
    y
}
fn nullspace(mut a: Vec<Vec<i64>>, p: i64, n: usize) -> (usize, Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let mut piv = vec![];
    for j in 0..n {
        let k = piv.len();
        let Some(i) = (k..a.len()).find(|i| a[*i][j] != 0) else {
            continue;
        };
        a.swap(k, i);
        let inv = modpow(a[k][j], p - 2, p);
        for z in &mut a[k] {
            *z = *z * inv % p;
        }
        for i in 0..a.len() {
            if i != k {
                let q = a[i][j];
                for t in 0..a[i].len() {
                    a[i][t] = (a[i][t] - q * a[k][t]).rem_euclid(p);
                }
            }
        }
        piv.push(j);
    }
    let basis = (0..n)
        .filter(|j| !piv.contains(j))
        .map(|j| {
            let mut v = vec![0; n];
            v[j] = 1;
            for (i, k) in piv.iter().enumerate() {
                v[*k] = (-a[i][j]).rem_euclid(p);
            }
            v
        })
        .collect::<Vec<_>>();
    (piv.len(), basis, a)
}
fn pair(c: &crate::binary_ecc::BinaryCurve, base: &[P; 2], v: &[i64]) -> P {
    lf::plus(
        c,
        &lf::times(c, &base[0], v[0] as u64),
        &lf::times(c, &base[1], v[1] as u64),
    )
}
fn encode_point(c: &crate::binary_ecc::BinaryCurve, base: &[P; 2], target: &P, p: i64) -> [i64; 2] {
    let found = (0..p)
        .flat_map(|a| (0..p).map(move |b| [a, b]))
        .filter(|v| pair(c, base, v) == *target)
        .collect::<Vec<_>>();
    assert_eq!(found.len(), 1);
    found[0]
}
fn decode_rows(data: &Value) -> [[[Cm; 4]; 5]; 4] {
    std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            std::array::from_fn(|k| {
                (
                    data["orbits"][i][j][k][0].as_i64().unwrap(),
                    data["orbits"][i][j][k][1].as_i64().unwrap(),
                )
            })
        })
    })
}
fn matrixlit(a: &[Vec<i64>]) -> String {
    format!(
        "![{}]",
        a.iter()
            .map(|r| format!(
                "![{}]",
                r.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}
pub fn kernels(dir: &Path) {
    let data = read(&dir.join("g25-correct-polarization-orbit-sections.json"));
    let old = read(&dir.join("g17-odd-cover-kernel-and-commutators.json"));
    let rows = decode_rows(&data);
    let f = F::new(168, 95);
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound20Picard\nset_option maxHeartbeats 0\nnamespace KoblitzRound20Kernel\nopen scoped BigOperators\ntheorem zero_of_left_inverse {F:Type*} [CommRing F] (L:Matrix (Fin 8) (Fin 40) F) (M:Matrix (Fin 40) (Fin 8) F) (h:L*M=1) (v:Fin 8→F) (hv:M.mulVec v=0) : v=0 := by\n have hz : (L*M).mulVec v=0 := by rw [← Matrix.mulVec_mulVec,hv,Matrix.mulVec_zero]\n rw [h,Matrix.one_mulVec] at hz\n exact hz\n");
    for (sign, record) in old["records"].as_array().unwrap().iter().enumerate() {
        let a = u64::from(record["trace"] == 1);
        let c = f.curve(f.e(a), f.o());
        let mut primary = vec![];
        for t in record["torsion_bases"].as_array().unwrap() {
            let p = t["prime"].as_i64().unwrap();
            let base = [point(&f, &t["P"]), point(&f, &t["Q"])];
            let wc = base.each_ref().map(|v| {
                let q = f.frobp(v, 1);
                encode_point(&c, &base, &if a == 1 { lf::neg(&q) } else { q }, p)
            });
            let mut matrix = vec![];
            for orbit in &rows {
                for row in orbit {
                    for i in 0..2 {
                        matrix.push(
                            (0..8)
                                .map(|j| {
                                    let (aa, b) = row[j / 2];
                                    ((if j % 2 == i { aa } else { 0 }) + b * wc[j % 2][i])
                                        .rem_euclid(p)
                                })
                                .collect::<Vec<_>>(),
                        );
                    }
                }
            }
            let mut columns_checked = 0;
            for j in 0..8 {
                let mut tuple = vec![P::Infinity; 4];
                tuple[j / 2] = base[j % 2].clone();
                for (k, row) in rows.iter().flatten().enumerate() {
                    assert_eq!(
                        act(&f, &c, &tuple, row, a),
                        pair(&c, &base, &[matrix[2 * k][j], matrix[2 * k + 1][j]])
                    );
                    columns_checked += 1;
                }
            }
            let aug = matrix
                .iter()
                .enumerate()
                .map(|(i, r)| {
                    let mut v = r.clone();
                    v.extend((0..40).map(|j| i64::from(i == j)));
                    v
                })
                .collect::<Vec<_>>();
            let (rank, basis, rref) = nullspace(aug, p, 8);
            assert_eq!(rank, 8);
            assert!(basis.is_empty());
            let left = rref[..8]
                .iter()
                .map(|r| r[8..].to_vec())
                .collect::<Vec<_>>();
            for i in 0..8 {
                for j in 0..8 {
                    assert_eq!(
                        (0..40).map(|k| left[i][k] * matrix[k][j]).sum::<i64>() % p,
                        i64::from(i == j)
                    );
                }
            }
            lean.push_str(&format!("def matrix{sign}_{p} : Matrix (Fin 40) (Fin 8) (ZMod {p}) := {}\ndef left{sign}_{p} : Matrix (Fin 8) (Fin 40) (ZMod {p}) := {}\ntheorem actual_left_inverse{sign}_{p} : left{sign}_{p}*matrix{sign}_{p}=1 := by\n ext i j\n change (∑k:Fin 40,left{sign}_{p} i k*matrix{sign}_{p} k j)=(if i=j then 1 else 0)\n fin_cases i <;> simp [left{sign}_{p},Fin.sum_univ_succ] <;> fin_cases j <;> norm_num [matrix{sign}_{p}] <;> decide +kernel\ntheorem actual_kernel_trivial{sign}_{p} (v:Fin 8→ZMod {p}) (h:(matrix{sign}_{p}).mulVec v=0) : v=0 := zero_of_left_inverse (F:=ZMod {p}) left{sign}_{p} matrix{sign}_{p} actual_left_inverse{sign}_{p} v h\n",matrixlit(&matrix),matrixlit(&left)));
            println!("G27 kernel:a{a},p{p},rank{rank},dimension0,order1,{columns_checked} actual column checks;explicit left inverse");
            primary.push(json!({"prime":p,"w_matrix_columns":wc,"matrix":matrix,"left_inverse":left,"rank":rank,"nullspace_basis":basis,"dimension":0,"cardinality":1,"actual_column_map_checks":columns_checked,"complete_native_kernel":true}));
        }
        records.push(json!({"graph":record["graph"],"ordinary_a":a,"primary":primary}));
    }
    lean.push_str(
        "end KoblitzRound20Kernel\n#print axioms KoblitzRound20Kernel.zero_of_left_inverse\n",
    );
    for sign in 0..2 {
        for p in [3, 5, 7] {
            for n in ["actual_left_inverse", "actual_kernel_trivial"] {
                lean.push_str(&format!(
                    "#print axioms KoblitzRound20Kernel.{n}{sign}_{p}\n"
                ));
            }
        }
    }
    save_lean(dir, "KoblitzRound20Kernel.lean", lean);
    crate::round02::save(
        dir,
        "g27-common-orbit-map-kernel.json",
        &json!({"schema":"koblitz.round20/g27-kernel-v2","records":records,"kernel_containment":"all row maps zero implies C(P)=0;B*C=105I implies105P=0","geometric_kernel_trivial_under_existing_odd_etale_bridge":true,"scope":"complete native primary calculations with explicit left inverses and actual point-column checks;Lean proves all six left inverse identities and universal kernel triviality;field/group/scheme bridges remain written"}),
    );
}

pub fn retraction(dir: &Path) {
    use num_bigint::BigInt as Z;
    use num_integer::Integer;
    use num_traits::{ToPrimitive, Zero};
    fn product(a: &[Vec<Z>], b: &[Vec<Z>]) -> Vec<Vec<Z>> {
        (0..a.len())
            .map(|i| {
                (0..b[0].len())
                    .map(|j| (0..b.len()).map(|k| &a[i][k] * &b[k][j]).sum())
                    .collect()
            })
            .collect()
    }
    fn identity(n: usize, m: i64) -> Vec<Vec<Z>> {
        (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| Z::from(if i == j { m } else { 0 }))
                    .collect()
            })
            .collect()
    }
    fn strings(a: &[Vec<Z>]) -> Vec<Vec<String>> {
        a.iter()
            .map(|r| r.iter().map(ToString::to_string).collect())
            .collect()
    }
    let data = read(&dir.join("g25-correct-polarization-orbit-sections.json"));
    let source = read(&dir.join("g16-diagonal-cover-section-bases.json"));
    let rows = decode_rows(&data);
    let mut matrix = vec![];
    for row in rows.iter().flatten() {
        matrix.push(
            row.iter()
                .flat_map(|(a, b)| [Z::from(*a), Z::from(*b)])
                .collect::<Vec<_>>(),
        );
        matrix.push(
            row.iter()
                .flat_map(|(a, b)| [Z::from(-2 * b), Z::from(a - b)])
                .collect::<Vec<_>>(),
        );
    }
    let mut u = vec![vec![Z::zero(); 40]; 8];
    for j in 0..4 {
        for i in 0..4 {
            let c = &source["records"][0]["B_columns"][i][j];
            let d: i64 = if i < 2 { 5 } else { 1 };
            let a: Z = c["a"].as_str().unwrap().parse::<Z>().unwrap() * d;
            let b: Z = c["b"].as_str().unwrap().parse::<Z>().unwrap() * d;
            u[2 * j][10 * i] = a.clone();
            u[2 * j][10 * i + 1] = b.clone();
            u[2 * j + 1][10 * i] = Z::from(-2) * &b;
            u[2 * j + 1][10 * i + 1] = &a - &b;
        }
    }
    assert_eq!(product(&u, &matrix), identity(8, 105));
    let initial = strings(&u);
    let mut m = 105;
    let mut steps = vec![];
    for p in [3i64, 5, 7] {
        let rows = matrix
            .iter()
            .enumerate()
            .map(|(i, r)| {
                let mut v = r
                    .iter()
                    .map(|z| z.mod_floor(&Z::from(p)).to_i64().unwrap())
                    .collect::<Vec<_>>();
                v.extend((0..40).map(|j| i64::from(i == j)));
                v
            })
            .collect::<Vec<_>>();
        let (rank, _, rref) = nullspace(rows, p, 8);
        assert_eq!(rank, 8);
        let t = rref[..8]
            .iter()
            .map(|r| r[8..].iter().map(|n| Z::from(*n)).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let tm = product(&t, &matrix);
        let e = (0..8)
            .map(|i| {
                (0..8)
                    .map(|j| {
                        let z = &tm[i][j] - Z::from(i64::from(i == j));
                        assert!(z.mod_floor(&Z::from(p)).is_zero());
                        z / p
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let eu = product(&e, &u);
        let d = m / p;
        u = (0..8)
            .map(|i| (0..40).map(|j| Z::from(d) * &t[i][j] - &eu[i][j]).collect())
            .collect();
        assert!(u.iter().flatten().all(|z| z.bits() < 160));
        assert_eq!(product(&u, &matrix), identity(8, d));
        steps.push(json!({"prime":p,"prior_scalar":m,"new_scalar":d,"T":strings(&t),"E":strings(&e),"new_left_multiple":strings(&u)}));
        m = d;
    }
    assert_eq!(m, 1);
    let l = (0..4)
        .map(|i| {
            (0..20)
                .map(|j| (u[2 * i][2 * j].clone(), u[2 * i][2 * j + 1].clone()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for i in 0..4 {
        for k in 0..4 {
            let z = (0..20).fold((Z::zero(), Z::zero()), |(x, y), j| {
                let (a, b) = &l[i][j];
                let (c, d) = rows[j / 5][j % 5][k];
                (x + a * c - 2 * b * d, y + a * d + b * c - b * d)
            });
            assert_eq!(z, (Z::from(i64::from(i == k)), Z::zero()));
        }
    }
    let coeff = l
        .iter()
        .map(|r| {
            r.iter()
                .map(|(a, b)| json!([a.to_string(), b.to_string()]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let f = F::new(168, 95);
    let inputs = read(&dir.join("g22-mixed-F2-orbit-sections.json"));
    let mut checks = vec![];
    for sign in 0..2 {
        let a = data["records"][sign]["ordinary_a"].as_u64().unwrap();
        let c = f.curve(f.e(a), f.o());
        for probe in data["records"][sign]["probes"]
            .as_array()
            .unwrap()
            .iter()
            .take(4)
        {
            let id = probe["G22_probe"].as_u64().unwrap() as usize;
            let original = inputs["records"][sign]["tuples"][id]["input"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| point(&f, v))
                .collect::<Vec<_>>();
            let q = probe["points"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|r| r.as_array().unwrap().iter().map(|v| point(&f, v)))
                .collect::<Vec<_>>();
            let recovered = l
                .iter()
                .map(|row| {
                    row.iter().zip(&q).fold(P::Infinity, |z, ((aa, b), p)| {
                        let term = |n: &Z, p: &P| {
                            let t = lf::times(
                                &c,
                                p,
                                n.magnitude()
                                    .to_u64()
                                    .expect("registered scalar-interface control bound"),
                            );
                            if n < &Z::zero() {
                                lf::neg(&t)
                            } else {
                                t
                            }
                        };
                        let w = f.frobp(p, 1);
                        let w = if a == 1 { lf::neg(&w) } else { w };
                        lf::plus(&c, &z, &lf::plus(&c, &term(aa, p), &term(b, &w)))
                    })
                })
                .collect::<Vec<_>>();
            assert_eq!(recovered, original);
            checks.push(json!({"ordinary_a":a,"G22_probe":id,"input":original.iter().map(lf::pj).collect::<Vec<_>>(),"recovered":recovered.iter().map(lf::pj).collect::<Vec<_>>()}));
        }
    }
    let ll = format!(
        "![{}]",
        l.iter()
            .map(|r| format!(
                "![{}]",
                r.iter()
                    .map(|(a, b)| format!("({a},{b})"))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    );
    let lean=format!("import KoblitzRound20Picard\nnamespace KoblitzRound20Retraction\nopen scoped BigOperators\nopen KoblitzRound19OrbitCover\ndef row (i:Fin 20) : Fin 4→CM := rows ⟨i.val/5,by omega⟩ ⟨i.val%5,by omega⟩\ndef recovery : Fin 4→Fin 20→CM := {ll}\ntheorem actual_CM_recovery : ∀i j:Fin 4,(∑k:Fin 20,cmul (recovery i k) (row k j))=if i=j then (1,0) else (0,0) := by decide +kernel\nend KoblitzRound20Retraction\n#print axioms KoblitzRound20Retraction.actual_CM_recovery\n");
    save_lean(dir, "KoblitzRound20Retraction.lean", lean);
    println!("G28:integral CM retraction of20-row orbit map;{} actual tuple round trips;max coefficient bits{}",checks.len(),u.iter().flatten().map(Z::bits).max().unwrap());
    crate::round02::save(
        dir,
        "g28-integral-orbit-retraction.json",
        &json!({"schema":"koblitz.round20/g28-retraction-v1","CM_relation":"w^2+w+2=0","row_lattice_matrix":strings(&matrix),"initial_left_105_multiple":initial,"Bezout_prime_steps":steps,"final_integer_left_inverse":strings(&u),"CM_recovery_coefficients":coeff,"point_round_trips":checks,"full_closed_source_embedding_under_written_morphism_bridge":true,"final_105_19_quotient_embedding_constructed":false,"scope":"explicit integral CM left inverse and actual source point controls;closed immersion via written elliptic/group/properness semantics;not the final quotient embedding"}),
    );
}

#[path = "mixed20_sections.rs"]
pub(crate) mod sections;

#[path = "mixed20_projector.rs"]
pub(crate) mod projector;
