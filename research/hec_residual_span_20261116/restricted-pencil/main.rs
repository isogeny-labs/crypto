#![allow(dead_code, unused_imports)]
mod binomial;
mod c4;
#[path = "../c4-gaussian/core.rs"]
mod gaussian;
#[path = "../strong-solves/order.rs"]
mod order;
#[path = "../c4-integrated/setup.rs"]
mod setup;
#[path = "../orbit-core/sparse.rs"]
mod sparse;
use hec_native_ic::{
    counted,
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    poly::Poly,
    rng::Rng,
};
use std::{fs, io::Write};
type Out = (Option<u64>, usize, u64, u64, bool);
fn run(c: &Curve, d: &Div, t: &Div, ell: u64, n: u64, gate: bool) -> Out {
    if c.g == 4 {
        let r = c4::solve(c, d, t, ell, n, gate);
        (
            r.k,
            r.relations,
            r.anchors,
            r.parameter,
            r.source_normalized,
        )
    } else {
        let r = binomial::solve(c, d, t, ell, gate);
        (
            r.k,
            r.relations,
            r.anchors,
            r.parameter,
            r.source_normalized,
        )
    }
}
fn main() {
    let root = "research/hec_residual_span_20261116/restricted-pencil";
    let mut gates = fs::File::create_new(format!("{root}/GATES.tsv")).unwrap();
    writeln!(gates, "g\tp\tsm\tcase\trows\tanchor\tt\tk\tnormalized").unwrap();
    println!("g\tp\tsm\tcase\trep\tmodulus\tk\texpected\tcorrect\trows\tanchors\tparameter\tnormalized\tcounts");
    let mut calls = 0;
    let mut prefixes = 0;
    let mut incomplete = 0;
    for (g, p, cc, n, ell) in [
        (2usize, 101u64, 2u64, 7481u64, 7481u64),
        (2, 251, 18, 67931, 67931),
        (2, 601, 7, 411491, 411491),
        (3, 113, 3, 1247429, 1247429),
        (3, 421, 2, 77152909, 77152909),
        (4, 101, 50, 96713378, 48356689),
        (4, 149, 1, 464841698, 232420849),
        (4, 197, 52, 1419828866, 709914433),
    ] {
        counted::disable();
        let fc = if g == 4 {
            vec![0, 2, 0, cc % 4, 0, (cc / 4) % 4, 0, cc / 16, 0, 1]
        } else {
            let mut a = vec![0; 2 * g + 2];
            a[0] = cc;
            a[2 * g + 1] = 1;
            a
        };
        let c = Curve::new(Fp::new(p), Poly::from_coeffs(&fc));
        if g < 4 {
            assert_eq!(order::jacobian_order(&c), ell)
        }
        let fb = factor_base(&c);
        let source = fb
            .iter()
            .map(|&(x, y)| c.mul(&c.place(x, y), n / ell))
            .find(|d| !c.is_zero(d))
            .unwrap();
        assert!(c.is_zero(&c.mul(&source, ell)));
        let mut rng = Rng::new(if g == 4 {
            2026111900 ^ p
        } else {
            2026112200 ^ p ^ ((g as u64) << 32)
        });
        for sm in [1u64, 17] {
            let d = c.mul(&source, sm);
            for case in 0..2 {
                let expected = 1 + rng.below(ell - 1);
                let t = c.mul(&d, expected);
                let gate = run(&c, &d, &t, ell, n, true);
                prefixes += gate.1;
                writeln!(
                    gates,
                    "{g}\t{p}\t{sm}\t{case}\t{}\t{}\t{}\t{}\t{}",
                    gate.1,
                    gate.2,
                    gate.3,
                    gate.0.map_or("none".to_string(), |x| x.to_string()),
                    gate.4
                )
                .unwrap();
                let mut prev = None;
                for rep in 0..2 {
                    counted::reset();
                    let out = run(&c, &d, &t, ell, n, false);
                    let counts = counted::snapshot();
                    counted::disable();
                    assert_eq!(out, gate);
                    let correct = out.0.is_some_and(|x| x == expected && c.mul(&d, x) == t);
                    if out.0.is_some() {
                        assert!(correct)
                    } else {
                        incomplete += 1
                    }
                    let signature = (out.clone(), counts.clone());
                    if let Some(prev) = &prev {
                        assert_eq!(prev, &signature)
                    } else {
                        prev = Some(signature)
                    }
                    for (modulus, values) in counts {
                        let s = values
                            .iter()
                            .map(u64::to_string)
                            .collect::<Vec<_>>()
                            .join(",");
                        println!("{g}\t{p}\t{sm}\t{case}\t{rep}\t{modulus}\t{}\t{expected}\t{correct}\t{}\t{}\t{}\t{}\t{s}",out.0.map_or("none".to_string(),|x|x.to_string()),out.1,out.2,out.3,out.4)
                    }
                    calls += 1;
                }
            }
        }
    }
    assert_eq!(calls, 64);
    eprintln!("calls={calls} incomplete={incomplete} checked_prefixes={prefixes} all_gate_outcomes_and_repeats=true models=8");
}
