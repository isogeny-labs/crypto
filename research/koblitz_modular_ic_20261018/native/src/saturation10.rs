//! Saturated projective prototype, with a complete three-polynomial Groebner check.
use crate::round02;
use serde_json::json;
use std::{collections::BTreeSet, path::Path};
type Mon = [u8; 5];
type Poly = BTreeSet<Mon>;
fn poly(ms: &[Mon]) -> Poly {
    ms.iter().cloned().collect()
}
fn toggle(p: &mut Poly, m: Mon) {
    if !p.insert(m) {
        p.remove(&m);
    }
}
fn times(p: &Poly, m: Mon) -> Poly {
    p.iter()
        .map(|a| std::array::from_fn(|i| a[i] + m[i]))
        .collect()
}
fn divides(a: Mon, b: Mon) -> bool {
    (0..5).all(|i| a[i] <= b[i])
}
fn s_poly(p: &Poly, q: &Poly) -> Poly {
    let a = *p.last().unwrap();
    let b = *q.last().unwrap();
    let m: Mon = std::array::from_fn(|i| a[i].max(b[i]));
    let mut z = times(p, std::array::from_fn(|i| m[i] - a[i]));
    for t in times(q, std::array::from_fn(|i| m[i] - b[i])) {
        toggle(&mut z, t)
    }
    z
}
fn remainder(mut p: Poly, gs: &[Poly]) -> Poly {
    let mut r = Poly::new();
    let mut steps = 0;
    while let Some(m) = p.last().cloned() {
        steps += 1;
        assert!(steps < 10000);
        if let Some(g) = gs.iter().find(|g| divides(*g.last().unwrap(), m)) {
            let a = *g.last().unwrap();
            for t in times(g, std::array::from_fn(|i| m[i] - a[i])) {
                toggle(&mut p, t)
            }
        } else {
            p.remove(&m);
            r.insert(m);
        }
    }
    r
}
fn eval(p: &Poly, x: &[u32; 5]) -> u32 {
    p.iter().fold(0, |v, m| {
        v ^ m
            .iter()
            .enumerate()
            .fold(1, |a, (i, n)| a * x[i].pow(*n as u32))
    })
}
pub fn run(dir: &Path) {
    // Lex order x>z>y>u>h. f1=xy-zh,f2=xu-h²,f3=zu-yh.
    let f1 = poly(&[[1, 0, 1, 0, 0], [0, 1, 0, 0, 1]]);
    let f2 = poly(&[[1, 0, 0, 1, 0], [0, 0, 0, 0, 2]]);
    let f3 = poly(&[[0, 1, 0, 1, 0], [0, 0, 1, 0, 1]]);
    let gs = vec![f1, f2, f3];
    let bad = [0, 1, 0, 1, 0];
    assert_eq!(eval(&gs[0], &bad), 0);
    assert_eq!(eval(&gs[1], &bad), 0);
    assert_eq!(eval(&gs[2], &bad), 1);
    let mut sp = vec![];
    for i in 0..3 {
        assert_eq!(gs[i].last().unwrap()[4], 0);
        for j in i + 1..3 {
            let s = s_poly(&gs[i], &gs[j]);
            let r = remainder(s.clone(), &gs);
            assert!(r.is_empty());
            sp.push(json!({"pair":[i,j],"S_polynomial":s,"remainder":r}));
        }
    }
    let mut naive = 0;
    let mut repaired = 0;
    for bits in 1..32 {
        let x = std::array::from_fn(|i| (bits >> i) & 1);
        if eval(&gs[0], &x) == 0 && eval(&gs[1], &x) == 0 {
            naive += 1;
            if eval(&gs[2], &x) == 0 {
                repaired += 1;
            }
        }
    }
    round02::save(
        dir,
        "q8p-projective-saturation-prototype.json",
        &json!({"schema":"koblitz.round10/q8p-v1","variables_lex":["x","z","y","u","h"],"naive_equations":["xy-zh","xu-h²"],"missing_equation":"zu-yh","membership_certificate":"h*(zu-yh)=y*(xu-h²)-u*(xy-zh)","false_projective_F2_point":bad,"Groebner_basis":gs,"S_pairs":sp,"all_initial_monomials_h_free":true,"naive_F2_points":naive,"saturated_F2_points":repaired,"saturation_proof":"Buchberger S pairs reduce to zero;leading monomials omit h;normal forms imply h is regular,so ideal is h-saturated;localization agrees with naive ideal by certificate","full_quotient_projective_saturation":false,"scope":"complete saturation of this two-gate prototype only;naive homogenization of the full quotient is rejected as unverified"}),
    );
    println!("Q8p:naive homogenization has a false boundary point;three-generator saturated prototype passes all S pairs");
}
