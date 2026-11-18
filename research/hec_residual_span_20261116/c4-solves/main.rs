mod pencil;
mod setup;
mod walk;
use hec_native_ic::{counted, fp::Fp, jac::Curve, poly::Poly, rng::Rng};
fn main() {
    println!("p\tmodel\tsm\tcase\trep\tarm\tmodulus\tk\texpected\tcorrect\trows\ttrials\tanchors\tparameter\tcounts");
    let mut calls = 0;
    let mut incomplete = 0;
    for (p, code, n, ell) in [
        (101u64, 50u64, 96713378u64, 48356689u64),
        (149, 1, 464841698, 232420849),
        (197, 52, 1419828866, 709914433),
    ] {
        counted::disable();
        let c = Curve::new(
            Fp::new(p),
            Poly::from_coeffs(&[0, 2, 0, code % 4, 0, (code / 4) % 4, 0, code / 16, 0, 1]),
        );
        let fb = hec_native_ic::instance::factor_base(&c);
        let source = fb
            .iter()
            .map(|&(x, y)| c.mul(&c.place(x, y), 2))
            .find(|d| !c.is_zero(d))
            .unwrap();
        let mut rng = Rng::new(2026111900 ^ p);
        for sm in [1, 17] {
            let d = c.mul(&source, sm);
            assert!(!c.is_zero(&d));
            for case in 0..2 {
                let expected = 1 + rng.below(ell - 1);
                let target = c.mul(&d, expected);
                let mut prior = std::collections::BTreeMap::new();
                for rep in 0..2 {
                    for a in 0..3 {
                        let arm = if rep % 2 == 0 { a } else { 2 - a };
                        counted::reset();
                        let (k, rows, trials, anchors, parameter) = if arm == 0 {
                            let r = pencil::solve(&c, &d, &target, ell, n);
                            (r.k, r.relations, 0, r.anchors, r.parameter)
                        } else {
                            let r = walk::solve(
                                &c,
                                &d,
                                &target,
                                ell,
                                n,
                                2026111900 + case,
                                if arm == 1 {
                                    walk::Mode::Folded
                                } else {
                                    walk::Mode::Expanded
                                },
                            );
                            (r.k, r.relations, r.trials, 0, 0)
                        };
                        // Candidate's own final scalar check is inside its count; independent replay below is outside.
                        let counts = counted::snapshot();
                        counted::disable();
                        let correct = k.is_some_and(|x| x == expected && c.mul(&d, x) == target);
                        if k.is_some() {
                            assert!(correct)
                        } else {
                            incomplete += 1
                        }
                        let signature = (k, rows, trials, anchors, parameter, counts.clone());
                        if rep == 0 && arm == 2 {
                            let other: &(
                                Option<u64>,
                                usize,
                                u64,
                                u64,
                                u64,
                                hec_native_ic::counted::Counts,
                            ) = &prior[&1];
                            assert_eq!((other.0, other.1, other.2), (k, rows, trials));
                        }

                        if rep == 0 {
                            prior.insert(arm, signature);
                        } else {
                            assert_eq!(prior[&arm], signature)
                        }
                        for (modulus, v) in &counts {
                            let s = v.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
                            println!("{p}\t{code}\t{sm}\t{case}\t{rep}\t{arm}\t{modulus}\t{}\t{expected}\t{correct}\t{rows}\t{trials}\t{anchors}\t{parameter}\t{s}",k.map_or("none".to_string(),|x|x.to_string()));
                        }
                        calls += 1;
                    }
                }
            }
        }
    }
    assert_eq!(calls, 72);
    eprintln!("calls={calls} incomplete={incomplete} repeated_vectors_exact=true scalar_checks=true cold_setup_charged=true units_vector_only=true");
}
