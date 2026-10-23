//! Exact integral Frobenius intertwiners; target lattices are explicit controls, not an actual modular Jacobian.
use crate::{exact16::det, round04::read};
use num_bigint::BigInt as Z;
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::{json, Value};
use std::path::Path;
type M = Vec<Vec<Z>>;
fn ident(n: usize) -> M {
    (0..n)
        .map(|i| (0..n).map(|j| Z::from(i == j)).collect())
        .collect()
}
fn mm(a: &M, b: &M) -> M {
    (0..a.len())
        .map(|i| {
            (0..b[0].len())
                .map(|j| (0..b.len()).map(|k| &a[i][k] * &b[k][j]).sum())
                .collect()
        })
        .collect()
}
fn mv(a: &M, v: &[Z]) -> Vec<Z> {
    a.iter()
        .map(|r| r.iter().zip(v).map(|(x, y)| x * y).sum())
        .collect()
}
fn companion(p: &[Z]) -> M {
    let n = p.len() - 1;
    assert_eq!(p[n], Z::one());
    let mut m = vec![vec![Z::zero(); n]; n];
    for i in 1..n {
        m[i][i - 1] = Z::one();
    }
    for i in 0..n {
        m[i][n - 1] = -&p[i];
    }
    m
}
fn product(a: &[Z], b: &[Z]) -> Vec<Z> {
    let mut p = vec![Z::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            p[i + j] += x * y;
        }
    }
    p
}
fn frob(t: i64) -> M {
    let mut r = vec![vec![0i64; 4]; 4];
    for i in 0..4 {
        r[0][i] = -1;
    }
    for i in 0..3 {
        r[i + 1][i] = 1;
    }
    let a = [[0, -2], [1, t]];
    (0..8)
        .map(|i| {
            (0..8)
                .map(|j| Z::from(r[i / 2][j / 2] * a[i % 2][j % 2]))
                .collect()
        })
        .collect()
}
fn adj(a: &M) -> M {
    let n = a.len();
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    let minor = (0..n)
                        .filter(|r| *r != j)
                        .map(|r| {
                            (0..n)
                                .filter(|c| *c != i)
                                .map(|c| a[r][c].clone())
                                .collect()
                        })
                        .collect();
                    let d = det(minor);
                    if (i + j) % 2 == 0 {
                        d
                    } else {
                        -d
                    }
                })
                .collect()
        })
        .collect()
}
fn polmat(p: &[Z], a: &M) -> M {
    let n = a.len();
    let mut q = vec![vec![Z::zero(); n]; n];
    for c in p.iter().rev() {
        q = mm(&q, a);
        for i in 0..n {
            q[i][i] += c;
        }
    }
    q
}
fn red(x: &Z, m: i64) -> i64 {
    x.mod_floor(&Z::from(m)).to_i64().unwrap()
}
fn kernel(a: &M, p: i64) -> Vec<Vec<Z>> {
    let n = a[0].len();
    let mut a = a
        .iter()
        .map(|r| r.iter().map(|x| red(x, p)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut piv = vec![];
    let mut k = 0;
    for j in 0..n {
        if let Some(i) = (k..a.len()).find(|i| a[*i][j] != 0) {
            a.swap(i, k);
            let d = (1..p).find(|d| (a[k][j] * d) % p == 1).unwrap();
            for x in &mut a[k] {
                *x = (*x * d) % p;
            }
            for i in 0..a.len() {
                if i != k {
                    let d = a[i][j];
                    for l in 0..n {
                        a[i][l] = (a[i][l] - d * a[k][l]).rem_euclid(p);
                    }
                }
            }
            piv.push(j);
            k += 1;
        }
    }
    (0..n)
        .filter(|j| !piv.contains(j))
        .map(|j| {
            let mut v = vec![Z::zero(); n];
            v[j] = Z::one();
            for (i, c) in piv.iter().enumerate() {
                v[*c] = Z::from((-a[i][j]).rem_euclid(p));
            }
            v
        })
        .collect()
}
fn minus_one(a: &M) -> M {
    let mut a = a.clone();
    for i in 0..a.len() {
        a[i][i] -= 1;
    }
    a
}
fn pval(p: &[Z], x: i64, modulus: i64) -> i64 {
    p.iter()
        .rev()
        .fold(0, |v, c| (v * x + red(c, modulus)).rem_euclid(modulus))
}
fn polyv(p: &[Z]) -> Value {
    json!(p.iter().map(ToString::to_string).collect::<Vec<_>>())
}
fn matv(m: &M) -> Value {
    json!(m
        .iter()
        .map(|r| r.iter().map(ToString::to_string).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn lit(m: &M, modular: bool) -> String {
    format!(
        "![{}]",
        m.iter()
            .map(|r| format!(
                "![{}]",
                r.iter()
                    .map(|x| if modular {
                        red(x, 11).to_string()
                    } else {
                        x.to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}
pub fn run(dir: &Path) {
    let cm = read(&dir.join("cm16-integral-sign-separation.json"));
    let p = |key: &str| {
        cm[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().parse::<Z>().unwrap())
            .collect::<Vec<_>>()
    };
    let pm = p("minus");
    let pp = p("plus");
    let pq = product(&pm, &pp);
    let n = companion(&pq);
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound16CM\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace KoblitzRound17Integral\n");
    let mut roots_all = vec![];
    for (sign, poly, other, t) in [(0usize, &pm, &pp, -1), (1, &pp, &pm, 1)] {
        let a = frob(t);
        assert!(polmat(poly, &a).iter().flatten().all(Zero::is_zero));
        let roots = (0..11)
            .filter(|x| pval(poly, *x, 11) == 0)
            .collect::<Vec<_>>();
        let deriv = poly
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| c * i)
            .collect::<Vec<_>>();
        assert_eq!(roots.len(), 8);
        assert!(roots.iter().all(|x| pval(&deriv, *x, 11) != 0));
        roots_all.push(roots.clone());
        let mut chosen = None;
        for j in 0..8 {
            let mut v = vec![Z::zero(); 8];
            v[j] = Z::one();
            let mut cols = vec![];
            for _ in 0..8 {
                cols.push(v.clone());
                v = mv(&a, &v);
            }
            let c = (0..8)
                .map(|i| cols.iter().map(|v| v[i].clone()).collect())
                .collect::<M>();
            let d = det(c.clone());
            if !d.mod_floor(&Z::from(11)).is_zero() {
                chosen = Some((j, c, d));
                break;
            }
        }
        let (cyclic_node, c, d) = chosen.expect("cyclic basis cap");
        let ac = adj(&c);
        assert_eq!(
            mm(&c, &ac),
            ident(8)
                .iter()
                .map(|r| r.iter().map(|x| x * &d).collect())
                .collect::<M>()
        );
        assert_eq!(mm(&a, &c), mm(&c, &companion(poly)));
        let i = (0..16)
            .map(|r| {
                (0..8)
                    .map(|j| {
                        if r >= j && r - j < other.len() {
                            other[r - j].clone()
                        } else {
                            Z::zero()
                        }
                    })
                    .collect()
            })
            .collect::<M>();
        let j = mm(&i, &ac);
        assert_eq!(mm(&n, &j), mm(&j, &a));
        let mut columns = vec![];
        let mut v = vec![Z::zero(); 8];
        v[0] = Z::one();
        for _ in 0..16 {
            columns.push(v.clone());
            v = mv(&companion(poly), &v);
        }
        let reduce = (0..8)
            .map(|i| columns.iter().map(|v| v[i].clone()).collect())
            .collect::<M>();
        let reverse = mm(&c, &reduce);
        assert_eq!(
            mm(&reverse, &j),
            polmat(other, &a)
                .iter()
                .map(|r| r.iter().map(|x| x * &d).collect())
                .collect::<M>()
        );
        let fixed = kernel(&minus_one(&a), 11);
        assert_eq!(fixed.len(), 1);
        let image = mv(&j, &fixed[0])
            .iter()
            .map(|z| Z::from(red(z, 11)))
            .collect::<Vec<_>>();
        assert!(image.iter().any(|x| !x.is_zero()));
        assert!(mv(&minus_one(&n), &image).iter().all(|z| red(z, 11) == 0));
        assert!(mv(&reverse, &image).iter().all(|z| red(z, 11) == 0));
        let vec_lit = |v: &[Z]| {
            format!(
                "![{}]",
                v.iter()
                    .map(|x| red(x, 11).to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        };
        lean.push_str(&format!("def F{sign} : Matrix (Fin 8) (Fin 8) ℤ := {}\ndef J{sign} : Matrix (Fin 16) (Fin 8) ℤ := {}\ndef N : Matrix (Fin 16) (Fin 16) ℤ := {}\n",lit(&a,false),lit(&j,false),lit(&n,false)).replace("def N :",&format!("def N{sign} :")));
        lean.push_str(&format!("theorem actual_integral_intertwiner{sign} : N{sign}*J{sign}=J{sign}*F{sign} := by decide +kernel\ndef Fmod{sign} : Matrix (Fin 8) (Fin 8) (ZMod 11) := {}\ndef Jmod{sign} : Matrix (Fin 16) (Fin 8) (ZMod 11) := {}\ndef Rmod{sign} : Matrix (Fin 8) (Fin 16) (ZMod 11) := {}\ndef v{sign} : Fin 8 → ZMod 11 := {}\ndef image{sign} : Fin 16 → ZMod 11 := {}\ntheorem actual_fixed_source{sign} : Matrix.mulVec Fmod{sign} v{sign}=v{sign} := by decide +kernel\ntheorem actual_forward_nonzero{sign} : Matrix.mulVec Jmod{sign} v{sign}=image{sign} ∧ image{sign}≠0 := by decide +kernel\ntheorem actual_reverse_kills_image{sign} : Matrix.mulVec Rmod{sign} image{sign}=0 := by decide +kernel\n",lit(&a,true),lit(&j,true),lit(&reverse,true),vec_lit(&fixed[0]),vec_lit(&image)));
        let eval = poly
            .iter()
            .enumerate()
            .map(|(i, c)| format!("({c}:ZMod 11)*x^{i}"))
            .collect::<Vec<_>>()
            .join(" + ");
        let evald = deriv
            .iter()
            .enumerate()
            .map(|(i, c)| format!("({c}:ZMod 11)*x^{i}"))
            .collect::<Vec<_>>()
            .join(" + ");
        lean.push_str(&format!("def p{sign} (x:ZMod 11) : ZMod 11 := {eval}\ndef dp{sign} (x:ZMod 11) : ZMod 11 := {evald}\ntheorem actual_squarefree_mod11_{sign} : ∀ x:ZMod 11,p{sign} x=0 → dp{sign} x≠0 := by decide +kernel\n"));
        records.push(json!({"sign":sign,"trace":t,"polynomial":polyv(poly),"roots_mod11":roots,"derivatives_at_roots":roots.iter().map(|x|pval(&deriv,*x,11)).collect::<Vec<_>>(),"trace_zero_Frobenius":matv(&a),"cyclic_basis_seed_coordinate":cyclic_node,"cyclic_basis":matv(&c),"cyclic_basis_determinant":d.to_string(),"integral_forward":matv(&j),"integral_reverse":matv(&reverse),"fixed_source_mod11":polyv(&fixed[0]),"nonzero_fixed_target_image_mod11":polyv(&image),"forward_intertwiner_checked":true,"reverse_composition":"det(C)*P_other(F_source)","reverse_on_fixed11_is_zero":true,"actual_modular_Jacobian_map_evaluated":false}));
        println!("CM18:sign{sign},8 distinct roots mod11;integral cyclic-lattice forward preserves fixed11;reverse kills image");
    }
    let common = roots_all[0]
        .iter()
        .filter(|x| roots_all[1].contains(x))
        .copied()
        .collect::<Vec<_>>();
    assert_eq!(common.len(), 6);
    let mut split = vec![vec![Z::zero(); 16]; 16];
    for (k, t) in [-1, 1].iter().enumerate() {
        let a = frob(*t);
        for i in 0..8 {
            for j in 0..8 {
                split[8 * k + i][8 * k + j] = a[i][j].clone();
            }
        }
    }
    assert_eq!(kernel(&minus_one(&split), 11).len(), 2);
    assert_eq!(kernel(&minus_one(&n), 11).len(), 1);
    assert_eq!(det(minus_one(&split)), Z::from(121));
    assert_eq!(det(minus_one(&n)), Z::from(121));
    for sign in 0..2 {
        lean.push_str(&format!("theorem actual_eight_roots_mod11_{sign} : (Finset.univ.filter fun x:ZMod 11 => p{sign} x=0).card=8 := by decide +kernel\n"));
    }
    lean.push_str("end KoblitzRound17Integral\n");
    for s in 0..2 {
        for name in [
            "actual_integral_intertwiner",
            "actual_fixed_source",
            "actual_forward_nonzero",
            "actual_reverse_kills_image",
        ] {
            lean.push_str(&format!("#print axioms KoblitzRound17Integral.{name}{s}\n"));
        }
        lean.push_str(&format!(
            "#print axioms KoblitzRound17Integral.actual_squarefree_mod11_{s}\n"
        ));
    }
    for sign in 0..2 {
        lean.push_str(&format!(
            "#print axioms KoblitzRound17Integral.actual_eight_roots_mod11_{sign}\n"
        ));
    }
    let out = dir.join("KoblitzRound17Integral.lean");
    assert!(!out.exists());
    std::fs::write(out, lean).unwrap();
    crate::round02::save(
        dir,
        "cm18-integral-Frobenius-lattice-controls.json",
        &json!({"schema":"koblitz.round17/cm18-v1","records":records,"common_roots_mod11":common,"cyclic_glued_Frobenius":matv(&n),"split_fixed11_dimension":2,"cyclic_fixed11_dimension":1,"same_Frobenius_polynomial":polyv(&pq),"same_determinant_F_minus_one":121,"split_cokernel11_primary":[11,11],"cyclic_cokernel11_primary":[121],"cokernel_bridge":"for a companion polynomial P,coker(F-1)=Z/(P(1));split controls are direct sums","actual_level77_integral_lattice_known":false,"existence_bridge":"squarefree mod11 source Frobenius gives unique local lattice;Tate integral Hom approximation supplies prime-to11 isogeny to its isotypic subvariety,under written global hypotheses","actual_factor_correspondence_constructed":false}),
    );
}
