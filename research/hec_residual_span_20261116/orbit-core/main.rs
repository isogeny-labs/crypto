#[path = "../batched/core.rs"]
mod batch;
mod core;
#[path = "../normalized/walk.rs"]
mod oldwalk;
#[path = "../strong-solves/order.rs"]
mod order;
use hec_native_ic::{counted, fp::Fp, instance::factor_base, jac::Curve, poly::Poly, rng::Rng};
use std::{fs, io::Write};
fn main() {
    let root = "research/hec_residual_span_20261116/orbit-core";
    let mut prefix = fs::File::create_new(format!("{root}/PREFIX.tsv")).unwrap();
    writeln!(prefix, "g\tp\ttrial\tprefix\trank\tk").unwrap();
    let mut cases = fs::File::create_new(format!("{root}/CASES.tsv")).unwrap();
    writeln!(cases, "g\tp\tsm\tcase\texpected\tsource\ttarget").unwrap();
    println!("g\tp\tsm\tcase\trep\tarm\tmodulus\tk\texpected\tcorrect\ttrials\trows\trejected\tcore\tcounts");
    let mut calls = 0;
    let mut incomplete = 0;
    for (g, p, cc, ell) in [
        (2usize, 101u64, 2u64, 7481u64),
        (2, 251, 18, 67931),
        (2, 601, 7, 411491),
        (3, 113, 3, 1247429),
        (3, 421, 2, 77152909),
    ] {
        counted::disable();
        let mut fc = vec![0; 2 * g + 2];
        fc[0] = cc;
        fc[2 * g + 1] = 1;
        let c = Curve::new(Fp::new(p), Poly::from_coeffs(&fc));
        assert_eq!(order::jacobian_order(&c), ell);
        let fb = factor_base(&c);
        let source = c.place(fb[0].0, fb[0].1);
        assert!(c.is_zero(&c.mul(&source, ell)));
        let mut rng = Rng::new(2026112200 ^ p ^ ((g as u64) << 32));
        for sm in [1u64, 17] {
            let d = c.mul(&source, sm);
            for case in 0..2 {
                let expected = 1 + rng.below(ell - 1);
                let t = c.mul(&d, expected);
                writeln!(cases, "{g}\t{p}\t{sm}\t{case}\t{expected}\t{d:?}\t{t:?}").unwrap();
                writeln!(prefix, "# workload {g} {p} {sm} {case}").unwrap();
                let seed = 2026112200 + case;
                let gate = core::run(&c, &d, &t, ell, seed, core::Engine::Gate, Some(&mut prefix));
                let mut previous = std::collections::BTreeMap::new();
                for rep in 0..2 {
                    for idx in 0..4 {
                        let arm = if rep == 0 { idx } else { 3 - idx };
                        counted::reset();
                        let (k, trials, rows, rejected, small) = match arm {
                            0 | 1 => {
                                let r = core::run(
                                    &c,
                                    &d,
                                    &t,
                                    ell,
                                    seed,
                                    if arm == 0 {
                                        core::Engine::Online
                                    } else {
                                        core::Engine::Sparse
                                    },
                                    None,
                                );
                                assert_eq!(r, gate);
                                (r.k, r.trials, r.accepted, r.rejected, r.core)
                            }
                            2 => {
                                let r =
                                    oldwalk::solve(&c, &d, &t, ell, seed, oldwalk::Mode::Folded);
                                (r.k, r.trials, r.relations as u64, 0, 0)
                            }
                            _ => {
                                let r = batch::solve(&c, &d, &t, ell);
                                (r.k, 0, r.relations as u64, 0, 0)
                            }
                        };
                        let counts = counted::snapshot();
                        counted::disable();
                        let correct = k.is_some_and(|k| k == expected && c.mul(&d, k) == t);
                        if k.is_some() {
                            assert!(correct)
                        } else {
                            incomplete += 1
                        }
                        let signature = (k, trials, rows, rejected, small, counts.clone());
                        if rep == 0 {
                            previous.insert(arm, signature);
                        } else {
                            assert_eq!(previous[&arm], signature)
                        }
                        for (modulus, v) in counts {
                            let s = v.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
                            println!("{g}\t{p}\t{sm}\t{case}\t{rep}\t{arm}\t{modulus}\t{}\t{expected}\t{correct}\t{trials}\t{rows}\t{rejected}\t{small}\t{s}",k.map_or("none".to_string(),|x|x.to_string()));
                        }
                        calls += 1;
                    }
                }
            }
        }
    }
    assert_eq!(calls, 160);
    eprintln!("calls={calls} incomplete={incomplete} every_prefix_sparse_online_equal=true repeat_vectors=true prime_order_models=5");
}
