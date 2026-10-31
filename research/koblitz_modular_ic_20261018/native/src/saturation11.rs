//! Exact h-saturation of a coupled multiplication / two-inverse graph.
use crate::round02;
use serde_json::json;
use std::{collections::BTreeSet, path::Path};
// Lex order w>x>z>y>u>v>h, w eliminated.
type M = [u8; 7];
type P = BTreeSet<M>;
fn poly(ms: &[M]) -> P {
    ms.iter().copied().collect()
}
fn xor(a: &mut P, b: P) {
    for m in b {
        if !a.insert(m) {
            a.remove(&m);
        }
    }
}
fn tm(p: &P, m: M) -> P {
    p.iter()
        .map(|a| std::array::from_fn(|i| a[i] + m[i]))
        .collect()
}
fn div(a: M, b: M) -> bool {
    (0..7).all(|i| a[i] <= b[i])
}
fn lm(p: &P, rev: bool) -> Option<M> {
    if !rev {
        return p.last().copied();
    }
    p.iter().copied().max_by(|a, b| {
        let da: u32 = a.iter().map(|x| *x as u32).sum();
        let db: u32 = b.iter().map(|x| *x as u32).sum();
        da.cmp(&db)
            .then_with(|| a.iter().rev().cmp(b.iter().rev()).reverse())
    })
}
fn rem(mut p: P, g: &[P], steps: &mut usize, rev: bool) -> P {
    let mut r = P::new();
    while let Some(m) = lm(&p, rev) {
        *steps += 1;
        assert!(*steps < 1000000);
        if let Some(a) = g.iter().find(|a| div(lm(a, rev).unwrap(), m)) {
            let n = lm(a, rev).unwrap();
            xor(&mut p, tm(a, std::array::from_fn(|i| m[i] - n[i])));
        } else {
            p.remove(&m);
            r.insert(m);
        }
    }
    r
}
fn sp(a: &P, b: &P, rev: bool) -> P {
    let x = lm(a, rev).unwrap();
    let y = lm(b, rev).unwrap();
    let l: M = std::array::from_fn(|i| x[i].max(y[i]));
    let mut p = tm(a, std::array::from_fn(|i| l[i] - x[i]));
    xor(&mut p, tm(b, std::array::from_fn(|i| l[i] - y[i])));
    p
}
fn gb(mut g: Vec<P>, steps: &mut usize, rev: bool) -> Vec<P> {
    let mut pairs = vec![];
    for i in 0..g.len() {
        for j in 0..i {
            pairs.push((j, i));
        }
    }
    while let Some((i, j)) = pairs.pop() {
        let r = rem(sp(&g[i], &g[j], rev), &g, steps, rev);
        if !r.is_empty() {
            assert!(g.len() < 100);
            assert!(r
                .iter()
                .all(|m| m.iter().map(|v| *v as u32).sum::<u32>() <= 24));
            let k = g.len();
            for i in 0..k {
                pairs.push((i, k));
            }
            g.push(r);
        }
    }
    g
}
fn ev(p: &P, b: u32) -> u32 {
    p.iter().fold(0, |r, m| {
        r ^ m
            .iter()
            .enumerate()
            .fold(1, |v, (i, e)| v * ((b >> i) & 1).pow(*e as u32))
    })
}
pub fn run(dir: &Path) {
    let naive = vec![
        poly(&[[0, 1, 0, 1, 0, 0, 0], [0, 0, 1, 0, 0, 0, 1]]),
        poly(&[[0, 1, 0, 0, 1, 0, 0], [0, 0, 0, 0, 0, 0, 2]]),
        poly(&[[0, 0, 0, 1, 0, 1, 0], [0, 0, 0, 0, 0, 0, 2]]),
    ];
    let mut ext = naive.clone();
    ext.push(poly(&[[1, 0, 0, 0, 0, 0, 1], [0; 7]]));
    let mut steps = 0;
    let extended = gb(ext, &mut steps, false);
    let eliminated: Vec<P> = extended
        .into_iter()
        .filter(|p| p.iter().all(|m| m[0] == 0))
        .collect();
    let mut saturated = gb(eliminated, &mut steps, true);
    // Interreduce and remove redundant leading monomials without changing the ideal.
    saturated.sort();
    saturated.dedup();
    let minimal: Vec<_> = saturated
        .iter()
        .enumerate()
        .filter(|(i, p)| {
            !saturated.iter().enumerate().any(|(j, q)| {
                j != *i
                    && div(lm(q, true).unwrap(), lm(p, true).unwrap())
                    && (lm(q, true) != lm(p, true) || j < *i)
            })
        })
        .map(|(_, p)| p.clone())
        .collect();
    saturated = minimal
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let others: Vec<_> = minimal
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, q)| q.clone())
                .collect();
            rem(p.clone(), &others, &mut steps, true)
        })
        .collect();
    let ngb = gb(naive.clone(), &mut steps, false);
    let mut certificates = vec![];
    for p in &saturated {
        assert_eq!(lm(p, true).unwrap()[6], 0, "h-regular initial ideal");
        let k = (0..=24)
            .find(|k| {
                let mut m = [0; 7];
                m[6] = *k;
                rem(tm(p, m), &ngb, &mut steps, false).is_empty()
            })
            .unwrap();
        // All terms have equal toric images x^a*y^b*h^d under z=xy/h,u=h²/x,v=h²/y.
        let images: BTreeSet<_> = p
            .iter()
            .map(|m| {
                (
                    m[1] as i32 + m[2] as i32 - m[4] as i32,
                    m[2] as i32 + m[3] as i32 - m[5] as i32,
                    -(m[2] as i32) + 2 * m[4] as i32 + 2 * m[5] as i32 + m[6] as i32,
                )
            })
            .collect();
        assert_eq!(images.len(), 1);
        assert_eq!(p.len(), 2);
        certificates
            .push(json!({"polynomial":p,"h_power_membership_in_naive":k,"toric_exponent":images}));
    }
    let mut pairs = vec![];
    for i in 0..saturated.len() {
        for j in 0..i {
            let s = sp(&saturated[j], &saturated[i], true);
            let r = rem(s.clone(), &saturated, &mut steps, true);
            assert!(r.is_empty());
            pairs.push(json!({"pair":[j,i],"S":s,"remainder":r}));
        }
    }
    let mut points = vec![];
    let mut falsepoints = vec![];
    for b in 2..128 {
        if b & 1 != 0 {
            continue;
        }
        if naive.iter().all(|p| ev(p, b) == 0) {
            let sat = saturated.iter().all(|p| ev(p, b) == 0);
            let p: Vec<_> = (1..7).map(|i| (b >> i) & 1).collect();
            if sat {
                points.push(p);
            } else {
                falsepoints.push(p);
            }
        }
    }
    let boundary: Vec<_> = points.iter().filter(|p| p[5] == 0).cloned().collect();
    let charts: Vec<_> = (1..7)
        .map(|c| {
            let equations: Vec<P> = saturated
                .iter()
                .map(|p| {
                    let mut out = P::new();
                    for m in p {
                        let mut a = *m;
                        a[c] = 0;
                        xor(&mut out, poly(&[a]));
                    }
                    out
                })
                .collect();
            json!({"coordinate_set_to_one":c,"equations":equations})
        })
        .collect();
    round02::save(
        dir,
        "q9-coupled-projective-saturation.json",
        &json!({"schema":"koblitz.round11/q9-v1","variables":["w","x","z","y","u","v","h"],"elimination_order":"lex w>x>z>y>u>v>h","final_Groebner_order":"graded reverse lex x>z>y>u>v>h","affine_equations":["xy=z","xu=1","yv=1"],"naive_homogeneous":naive,"saturated_Groebner_basis":saturated,"membership_certificates":certificates,"S_pairs":pairs,"reduction_steps":steps,"all_leading_monomials_h_free":true,"F2_projective_points":points,"F2_boundary_points":boundary,"naive_false_boundary_points":falsepoints,"dimension":2,"six_dehomogenized_charts":charts,"all_chart_recovery":"for each coordinate c, dehomogenize every displayed generator at c=1; transition X_j/c -> (X_j/d)/(c/d) on c*d!=0","full_quotient_h_saturation":false,"proper_abelian_quotient_embedding":false,"scope":"complete projective closure and six-chart closed embedding of coupled two-inverse monomial graph;not the full F2^360 quotient circuit"}),
    );
    println!(
        "Q9:{} generators,{} S-pairs,{} boundary F2 points,{} rejected naive points",
        saturated.len(),
        pairs.len(),
        boundary.len(),
        falsepoints.len()
    );
}
