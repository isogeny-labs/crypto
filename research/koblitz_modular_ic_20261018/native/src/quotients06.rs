//! Explicit split quotient equations, polarization and F2 descent.
use crate::{
    round02,
    round04::{read, Curve, Field, Pt},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::Path;
#[path = "geometry14.rs"]
pub(crate) mod geometry14;
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct O {
    a: i64,
    b: i64,
}
impl O {
    fn n(a: i64, b: i64) -> Self {
        Self { a, b }
    }
    fn add(self, v: Self) -> Self {
        Self::n(self.a + v.a, self.b + v.b)
    }
    fn neg(self) -> Self {
        Self::n(-self.a, -self.b)
    }
    fn mul(self, v: Self) -> Self {
        Self::n(
            self.a * v.a - 2 * self.b * v.b,
            self.a * v.b + self.b * v.a - self.b * v.b,
        )
    }
    fn bar(self) -> Self {
        Self::n(self.a - self.b, -self.b)
    }
    fn pow(self, n: u32) -> Self {
        (0..n).fold(Self::n(1, 0), |v, _| v.mul(self))
    }
}
type M = Vec<Vec<O>>;
fn ident(n: usize) -> M {
    (0..n)
        .map(|i| (0..n).map(|j| O::n((i == j) as i64, 0)).collect())
        .collect()
}
fn mm(a: &M, b: &M) -> M {
    a.iter()
        .map(|r| {
            (0..b[0].len())
                .map(|j| {
                    r.iter()
                        .enumerate()
                        .fold(O::default(), |v, (k, x)| v.add(x.mul(b[k][j])))
                })
                .collect()
        })
        .collect()
}
fn ct(a: &M) -> M {
    (0..a[0].len())
        .map(|i| a.iter().map(|r| r[i].bar()).collect())
        .collect()
}
fn det(a: &M) -> O {
    if a.len() == 1 {
        return a[0][0];
    }
    (0..a.len()).fold(O::default(), |v, j| {
        let minor = a[1..]
            .iter()
            .map(|r| {
                r.iter()
                    .enumerate()
                    .filter(|(k, _)| *k != j)
                    .map(|(_, x)| *x)
                    .collect()
            })
            .collect();
        let t = a[0][j].mul(det(&minor));
        v.add(if j % 2 == 0 { t } else { t.neg() })
    })
}
fn rep(x: O) -> Vec<Vec<i64>> {
    vec![vec![x.a, -2 * x.b], vec![x.b, x.a - x.b]]
}
fn imod(a: &[Vec<i64>]) -> Vec<Vec<i64>> {
    a.iter()
        .map(|r| r.iter().map(|v| v.rem_euclid(5)).collect())
        .collect()
}
fn im(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    a.iter()
        .map(|r| {
            (0..b[0].len())
                .map(|j| r.iter().enumerate().map(|(k, x)| x * b[k][j]).sum())
                .collect()
        })
        .collect()
}
fn negp(c: Curve, p: Pt, n: i64) -> Pt {
    let z = c.times(p, n.unsigned_abs());
    if n < 0 {
        c.neg(z)
    } else {
        z
    }
}
fn op(c: Curve, x: O, p: Pt) -> Pt {
    let w = if c.a == 0 {
        c.frob(p, 1)
    } else {
        c.neg(c.frob(p, 1))
    };
    c.add(negp(c, p, x.a), negp(c, w, x.b))
}
fn apply(c: Curve, u: &M, p: &[Pt]) -> Vec<Pt> {
    u.iter()
        .map(|r| {
            r.iter()
                .zip(p)
                .fold(Pt::O, |v, (a, p)| c.add(v, op(c, *a, *p)))
        })
        .collect()
}
fn smallp(v: &Value) -> Pt {
    if v.is_array() {
        Pt::A(v[0].as_u64().unwrap() as u32, v[1].as_u64().unwrap() as u32)
    } else {
        Pt::O
    }
}
fn smat(u: &M) -> Vec<Vec<i64>> {
    (0..2 * u.len())
        .map(|i| {
            (0..2 * u.len())
                .map(|j| rep(u[i / 2][j / 2])[i % 2][j % 2])
                .collect()
        })
        .collect()
}
fn tr(a: &[Vec<i64>]) -> Vec<Vec<i64>> {
    (0..a[0].len())
        .map(|j| a.iter().map(|r| r[j]).collect())
        .collect()
}
pub fn lean(dir: &Path) {
    fn lit(a: &[Vec<i64>]) -> String {
        format!(
            "![{}]",
            a.iter()
                .map(|r| format!(
                    "![{}]",
                    r.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
    let input = read(&dir.join("q4-split-quotient-equations.json"));
    let mut h = ident(5);
    for i in 0..4 {
        for j in 0..4 {
            h[i][j] = O::n(if i == j { 2 } else { 1 }, 0);
        }
    }
    h[4][4] = O::n(5, 0);
    let j: Vec<Vec<i64>> = (0..10)
        .map(|i| {
            (0..10)
                .map(|k| {
                    if i / 2 == k / 2 && i % 2 != k % 2 {
                        if i % 2 == 0 {
                            1
                        } else {
                            -1
                        }
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();
    let mut text=format!("-- Generated from frozen Q4; kernel-checked integral polarization equations.\nimport KoblitzRound05\nopen scoped Matrix\nnamespace KoblitzRound06Matrices\ndef JS : Matrix (Fin 10) (Fin 10) ℤ := {}\n",lit(&im(&j,&smat(&h))));
    for model in input["models"].as_array().unwrap() {
        let k = model["k"].as_u64().unwrap();
        let l: M = serde_json::from_value(model["polarization_L"].clone()).unwrap();
        let q: M = serde_json::from_value(model["quotient_Q"].clone()).unwrap();
        let t: M = serde_json::from_value(model["descent_T"].clone()).unwrap();
        text.push_str(&format!("def J{k} : Matrix (Fin 10) (Fin 10) ℤ := {}\ndef Q{k} : Matrix (Fin 10) (Fin 10) ℤ := {}\ndef T{k} : Matrix (Fin 10) (Fin 10) ℤ := {}\ntheorem polarized_descent_{k} : Q{k}.transpose*J{k}*Q{k}=JS ∧ T{k}.transpose*J{k}*T{k}=J{k} := by decide +kernel\n#print axioms polarized_descent_{k}\n",lit(&im(&j,&smat(&l))),lit(&smat(&q)),lit(&smat(&t))));
    }
    text.push_str("end KoblitzRound06Matrices\n");
    let p = dir.join("KoblitzRound06Matrices.lean");
    assert!(!p.exists());
    std::fs::write(p, text).unwrap();
}
pub fn verify(dir: &Path) {
    use crate::large_field::{neg, plus, times, F, P};
    use num_bigint::BigInt;
    let data = read(&dir.join("q4-split-quotient-equations.json"));
    let forms = read(&dir.join("q3-equivalent-quotients.json"));
    let f = F::new(24, 27);
    let decode = |v: &Value| {
        if v.is_array() {
            P::Affine {
                x: f.e(v[0].as_u64().unwrap()),
                y: f.e(v[1].as_u64().unwrap()),
            }
        } else {
            P::Infinity
        }
    };
    let mut checked = 0;
    for row in data["graphs"].as_array().unwrap() {
        let trace = row["target_trace"].as_i64().unwrap();
        let c = f.curve(f.e(if trace == -1 { 0 } else { 1 }), f.o());
        let u: M = serde_json::from_value(
            forms["forms"]
                .as_array()
                .unwrap()
                .iter()
                .find(|v| v["coefficients"] == row["coefficients"])
                .unwrap()["first"]["U"]
                .clone(),
        )
        .unwrap();
        let k = row["k"].as_u64().unwrap() as u32;
        let eps = row["epsilon"].as_i64().unwrap();
        let apply_o = |x: O, p: &P| {
            let wp = if trace == -1 {
                f.frobp(p, 1)
            } else {
                neg(&f.frobp(p, 1))
            };
            let scalar = |p: &P, n: i64| {
                let z = times(&c, p, n.unsigned_abs());
                if n < 0 {
                    neg(&z)
                } else {
                    z
                }
            };
            plus(&c, &scalar(p, x.a), &scalar(&wp, x.b))
        };
        for entry in row["kernel"].as_array().unwrap() {
            let a: Vec<_> = entry["augmentation_first4"]
                .as_array()
                .unwrap()
                .iter()
                .map(&decode)
                .collect();
            let q = decode(&entry["cofactor"]);
            assert!(a
                .iter()
                .chain([&q])
                .all(|p| c.is_on_curve(p) && times(&c, p, 5) == P::Infinity));
            let aa: Vec<_> = u
                .iter()
                .map(|r| {
                    r.iter()
                        .zip(&a)
                        .fold(P::Infinity, |v, (x, p)| plus(&c, &v, &apply_o(*x, p)))
                })
                .collect();
            let a5 = neg(&aa.iter().fold(P::Infinity, |v, p| plus(&c, &v, p)));
            for p in aa {
                assert_eq!(plus(&c, &p, &neg(&a5)), P::Infinity);
            }
            let eq = if eps == 1 { q.clone() } else { neg(&q) };
            assert_eq!(plus(&c, &f.frobp(&a5, k), &neg(&eq)), P::Infinity);
            checked += 1;
        }
    }
    type BM = Vec<Vec<BigInt>>;
    let big = |a: Vec<Vec<i64>>| {
        a.iter()
            .map(|r| r.iter().map(|v| BigInt::from(*v)).collect())
            .collect::<BM>()
    };
    let bm = |a: &BM, b: &BM| {
        a.iter()
            .map(|r| {
                (0..b[0].len())
                    .map(|j| r.iter().enumerate().map(|(k, x)| x * &b[k][j]).sum())
                    .collect()
            })
            .collect::<BM>()
    };
    let bt = |a: &BM| {
        (0..a[0].len())
            .map(|j| a.iter().map(|r| r[j].clone()).collect())
            .collect::<BM>()
    };
    let j: Vec<Vec<i64>> = (0..10)
        .map(|i| {
            (0..10)
                .map(|k| {
                    if i / 2 == k / 2 && i % 2 != k % 2 {
                        if i % 2 == 0 {
                            1
                        } else {
                            -1
                        }
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();
    let mut hs = ident(5);
    for i in 0..4 {
        for j in 0..4 {
            hs[i][j] = O::n(if i == j { 2 } else { 1 }, 0);
        }
    }
    hs[4][4] = O::n(5, 0);
    let js = big(im(&j, &smat(&hs)));
    for model in data["models"].as_array().unwrap() {
        let l: M = serde_json::from_value(model["polarization_L"].clone()).unwrap();
        let q: M = serde_json::from_value(model["quotient_Q"].clone()).unwrap();
        let t: M = serde_json::from_value(model["descent_T"].clone()).unwrap();
        let jl = big(im(&j, &smat(&l)));
        let q = big(smat(&q));
        let t = big(smat(&t));
        assert_eq!(bm(&bm(&bt(&q), &jl), &q), js);
        assert_eq!(bm(&bm(&bt(&t), &jl), &t), jl);
    }
    round02::save(
        dir,
        "q4b-independent-checks.json",
        &json!({"schema":"koblitz.round06/q4b-v1","library_geometric_kernel_points":checked,"BigInt_polarization_pullbacks":3,"BigInt_descent_pairing_checks":3,"status":"PASS","scope":"separate maintained-library point path and BigInt matrices; geometric effectivity/positivity argument remains written"}),
    );
    println!("Q4b:250 library kernel points and BigInt polarization/descent checks PASS");
}
pub fn run(dir: &Path) {
    let old = read(&dir.join("q3-equivalent-quotients.json"));
    let raw = read(&dir.join("gl2b-integral-polarizations.json"));
    let kernel = read(&dir.join("q2-natural-quotient.json"));
    let gf = Field {
        m: 24,
        modulus: kernel["geometric_kernel_field"]["modulus"]
            .as_u64()
            .unwrap(),
    };
    let mut h = ident(5);
    for i in 0..4 {
        for j in 0..4 {
            h[i][j] = O::n(if i == j { 2 } else { 1 }, 0);
        }
    }
    h[4][4] = O::n(5, 0);
    let mut models = vec![];
    for k in [0, 4, 8] {
        let pi = O::n(0, 1).pow(k);
        let alpha = ((1i64 << k) - 1) / 5;
        let mut l = ident(5);
        for i in 0..4 {
            for j in 0..4 {
                l[i][j] = O::n((i == j) as i64 + alpha, 0);
            }
            l[i][4] = pi.bar();
            l[4][i] = pi;
        }
        l[4][4] = O::n(5, 0);
        let mut q = vec![vec![O::default(); 5]; 5];
        for i in 0..4 {
            for j in 0..4 {
                q[i][j] = h[i][j];
            }
            q[4][i] = pi.neg();
        }
        q[4][4] = O::n(-1, 0);
        let mut t = vec![vec![O::default(); 5]; 5];
        for i in 0..4 {
            t[i][3] = O::n(-1, 0);
            if i > 0 {
                t[i][i - 1] = O::n(1, 0);
            }
        }
        t[4][3] = pi;
        t[4][4] = O::n(1, 0);
        assert_eq!(mm(&mm(&ct(&q), &l), &q), h);
        assert_eq!(mm(&mm(&ct(&t), &l), &t), l);
        assert_eq!((0..5).fold(ident(5), |v, _| mm(&v, &t)), ident(5));
        assert_eq!(det(&l), O::n(1, 0));
        let minors: Vec<_> = (1..=5)
            .map(|n| det(&l[..n].iter().map(|r| r[..n].to_vec()).collect()))
            .collect();
        assert!(minors.iter().all(|v| v.b == 0 && v.a > 0));
        models.push(json!({"k":k,"pi_power":pi,"alpha":alpha,"polarization_L":l,"quotient_Q":q,"descent_T":t,"leading_minors":minors,"determinant":1,"equations":"five projective equations Y_i^2 Z_i+X_i Y_i Z_i=X_i^3+a X_i^2 Z_i+Z_i^3 in (P2)^5 over F32","descent":"T_k after coefficient Frobenius on F32/F2; order5; pi^k is coordinate Frobenius^k on E_a","quotient_map":"Y_i=a_i-a5, Z=pi^k*a5-epsilon*Q; precompose augmentation with retained U"}));
    }
    // Bind the trace+1 companion basis explicitly, rather than changing integer lifts.
    let tplus = vec![vec![3, 3], vec![4, 1]];
    let invt = vec![vec![1, -3], vec![-4, 3]];
    assert_eq!(imod(&im(&tplus, &invt)), vec![vec![1, 0], vec![0, 1]]);
    let mut graphs = vec![];
    for candidate in raw["candidates"].as_array().unwrap() {
        if candidate["determinant"] != 5 {
            continue;
        }
        let form = old["forms"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["H"] == candidate["hermitian_matrix"])
            .unwrap();
        let u: M = serde_json::from_value(form["first"]["U"].clone()).unwrap();
        let ur = smat(&u);
        let kb: Vec<Vec<i64>> =
            serde_json::from_value(candidate["kernel"]["basis_columns"].clone()).unwrap();
        let trace = candidate["target_trace"].as_i64().unwrap();
        let a = if trace == -1 { 0 } else { 1 };
        let curve = Curve { f: gf, a, b: 1 };
        let kp = kernel["geometric_kernels"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["a"] == a)
            .unwrap();
        let p = smallp(&kp["basis"][0]);
        let w = if a == 0 {
            curve.frob(p, 1)
        } else {
            curve.neg(curve.frob(p, 1))
        };
        let basis = [p, w];
        assert_eq!(curve.times(p, 5), Pt::O);
        assert!(!(0..5).any(|i| curve.times(p, i) == w));
        let binding = if a == 0 {
            vec![vec![1, 0], vec![0, 1]]
        } else {
            tplus.clone()
        };
        let pi = if a == 0 {
            rep(O::n(0, 1))
        } else {
            rep(O::n(0, -1))
        };
        let fe = vec![vec![0, -2], vec![1, trace]];
        assert_eq!(imod(&im(&binding, &fe)), imod(&im(&pi, &binding)));
        for e in candidate["elliptic_gluings"].as_array().unwrap() {
            if e["anti_isometries"].as_u64().unwrap() == 0 {
                continue;
            }
            assert_eq!(e["elliptic_trace"], trace);
            let m: Vec<Vec<i64>> =
                serde_json::from_value(e["first_anti_isometry"].clone()).unwrap();
            let g = im(&kb, &m);
            let ug = im(&ur, &g);
            let common = imod(&ug[..2]);
            let effective = if a == 0 {
                common.clone()
            } else {
                imod(&im(&common, &invt))
            };
            let (k, eps) = [0u32, 4, 8]
                .into_iter()
                .flat_map(|k| [1i64, -1].into_iter().map(move |eps| (k, eps)))
                .find(|(k, eps)| {
                    let mut r = rep(O::n(0, 1).pow((24 - k) % 24));
                    for row in &mut r {
                        for v in row {
                            *v *= eps;
                        }
                    }
                    imod(&r) == effective
                })
                .expect("no k0/4/8 graph match");
            let cm = |x: i64, y: i64| {
                curve.add(
                    curve.times(basis[0], x.rem_euclid(5) as u64),
                    curve.times(basis[1], y.rem_euclid(5) as u64),
                )
            };
            let mut points = vec![];
            for x in 0..5 {
                for y in 0..5 {
                    let aold: Vec<_> = (0..4)
                        .map(|i| {
                            cm(
                                g[2 * i][0] * x + g[2 * i][1] * y,
                                g[2 * i + 1][0] * x + g[2 * i + 1][1] * y,
                            )
                        })
                        .collect();
                    let aa = apply(curve, &u, &aold);
                    let a5 = curve.neg(aa.iter().fold(Pt::O, |v, p| curve.add(v, *p)));
                    let q = cm(
                        binding[0][0] * x + binding[0][1] * y,
                        binding[1][0] * x + binding[1][1] * y,
                    );
                    assert!(aa.iter().all(|v| curve.add(*v, curve.neg(a5)) == Pt::O));
                    assert_eq!(
                        curve.add(curve.frob(a5, k), curve.neg(negp(curve, q, eps))),
                        Pt::O
                    );
                    points.push(json!({"coefficients":[x,y],"augmentation_first4":aold.iter().map(|p|p.json()).collect::<Vec<_>>(),"cofactor":q.json()}));
                }
            }
            graphs.push(json!({"coefficients":candidate["coefficients"],"target_trace":trace,"companion_to_CM_basis":binding,"CM_basis":basis.iter().map(|p|p.json()).collect::<Vec<_>>(),"frozen_common_block":common,"effective_block":effective,"k":k,"epsilon":eps,"kernel_points_checked":25,"kernel":points,"scope":"actual realization of the frozen finite graph with explicitly bound companion/CM bases; split equations and complete F2 descent, no expanded F2 projective ideal"}));
        }
    }
    assert_eq!(graphs.len(), 10);
    let f = Field::find(5);
    let mut rational = vec![];
    for a in 0..=1 {
        let c = Curve { f, a, b: 1 };
        let pts = c.points();
        let base: Vec<_> = pts
            .iter()
            .copied()
            .filter(|p| c.frob(*p, 1) == *p)
            .collect();
        let tr0: Vec<_> = pts
            .iter()
            .copied()
            .filter(|p| (0..5).fold(Pt::O, |v, i| c.add(v, c.frob(*p, i))) == Pt::O)
            .collect();
        assert_eq!(tr0.len(), 11);
        for k in [0, 4, 8] {
            let mut entries = vec![];
            for p in &tr0 {
                for q in &base {
                    let aa: Vec<_> = (0..5).map(|i| c.frob(*p, i)).collect();
                    let mut dest: Vec<_> =
                        aa[..4].iter().map(|v| c.add(*v, c.neg(aa[4]))).collect();
                    dest.push(c.add(c.frob(aa[4], k), c.neg(*q)));
                    let fp: Vec<_> = dest.iter().map(|p| c.frob(*p, 1)).collect();
                    let mut desc = vec![];
                    desc.push(c.neg(fp[3]));
                    for i in 1..4 {
                        desc.push(c.add(fp[i - 1], c.neg(fp[3])));
                    }
                    desc.push(c.add(fp[4], c.frob(fp[3], k)));
                    assert_eq!(dest, desc);
                    entries.push((*p, *q, dest));
                }
            }
            let mut checks = 0;
            for (p, q, d) in &entries {
                for (r, s, e) in &entries {
                    let expected = entries
                        .iter()
                        .find(|(x, y, _)| *x == c.add(*p, *r) && *y == c.add(*q, *s))
                        .unwrap();
                    let sum: Vec<_> = d.iter().zip(e).map(|(x, y)| c.add(*x, *y)).collect();
                    assert_eq!(sum, expected.2);
                    checks += 1;
                }
            }
            let mut keys: Vec<_> = entries.iter().map(|(_, _, d)| format!("{d:?}")).collect();
            keys.sort();
            keys.dedup();
            assert_eq!(keys.len(), entries.len());
            rational.push(json!({"a":a,"k":k,"points":entries.len(),"addition_checks":checks,"all_descent_checks":true,"images":entries.iter().map(|(p,q,d)|json!({"trace_zero_point":p.json(),"elliptic_point":q.json(),"image":d.iter().map(|p|p.json()).collect::<Vec<_>>()})).collect::<Vec<_>>()}));
        }
    }
    println!("Q4:10/10 determinant5 graphs realized,250 geometric kernel points; split equations+descent+principal polarizations");
    round02::save(
        dir,
        "q4-split-quotient-equations.json",
        &json!({"schema":"koblitz.round06/q4-v1","split_field":{"degree":5,"modulus":37},"kernel_field":{"degree":24,"modulus":gf.modulus},"models":models,"graphs":graphs,"rational_checks":rational,"determinant19_equations":"OPEN","expanded_F2_affine_or_projective_ideal":false,"scope":"five elliptic equations over F32 and explicit effective F2 descent; every frozen determinant5 graph geometrically bound and checked; not a Jacobian assertion"}),
    );
}
