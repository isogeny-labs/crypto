mod core;
#[path = "../production/qualified.rs"]
mod qualified;
#[path = "../normalized/pencil.rs"]
mod scalar;
#[path = "../normalized/walk.rs"]
mod walk;
use hec_native_ic::{
    counted,
    fp::Fp,
    jac::{Curve, Div},
};
use std::{fs, io::Write};
type Outcome = (Option<u64>, usize, u64, u64, u64, bool);
fn run(arm: usize, c: &Curve, d: &Div, t: &Div, ell: u64, case: usize) -> Outcome {
    match arm {
        0 => {
            let r = core::solve(c, d, t, ell);
            (
                r.k,
                r.relations,
                0,
                r.anchors,
                r.parameter,
                r.source_normalized,
            )
        }
        1 => {
            let r = scalar::solve(c, d, t, ell);
            (
                r.k,
                r.relations,
                0,
                r.anchors,
                r.parameter,
                r.source_normalized,
            )
        }
        a => {
            let r = walk::solve(
                c,
                d,
                t,
                ell,
                2026111600 + case as u64,
                if a == 2 {
                    walk::Mode::Folded
                } else {
                    walk::Mode::Expanded
                },
            );
            (r.k, r.relations, r.trials, 0, 0, r.source_normalized)
        }
    }
}
fn main() {
    let mut checks = 0;
    for p in [101u64, 113, 109, 7481, 143669, 432343] {
        let f = Fp::new(p);
        for n in 0..=128 {
            let v: Vec<_> = (0..n)
                .map(|i| {
                    let x = ((i as u64 * 37 + 5) % p) as u64;
                    if x == 0 {
                        1
                    } else {
                        x
                    }
                })
                .collect();
            let expected: Vec<_> = v.iter().map(|&x| f.inv(x)).collect();
            counted::reset();
            let out = core::inverse::invert(&v, &f);
            let cnt = counted::snapshot().get(&p).copied().unwrap_or([0; 16]);
            assert_eq!(out, expected);
            assert_eq!(cnt[9], if n == 0 { 0 } else { 1 });
            assert_eq!(cnt[4], if n == 0 { 0 } else { 3 * (n - 1) as u64 });
            checks += 1
        }
    }
    assert_eq!(checks, 774);
    eprintln!("inverse_cases={checks} exact=true cost_formula=true");
    let root = "research/hec_residual_span_20261116/batched";
    let mut receipt = fs::File::create_new(format!("{root}/FIXTURES.tsv")).unwrap();
    writeln!(receipt, "g\tp\tc\tstatus").unwrap();
    let mut outcomes = fs::File::create_new(format!("{root}/OUTCOMES.tsv")).unwrap();
    writeln!(
        outcomes,
        "g\tp\tsm\tcase\tarm\trows\ttrials\tanchor\tt\tsource_normalized\tk"
    )
    .unwrap();
    println!("g\tp\tsm\tcase\tarm\trep\tmodulus\tk\tnew\tadd\tsub\tneg\tmul\tpre\tmul_pre\treduce128\tredc\tinv\tinv_steps\tpoly_products\tpoly_divrem\tgroup_add\tgroup_add_place\tgroup_mul");
    for (g, p) in [(2usize, 101u64), (3, 113), (4, 109)] {
        let x = qualified::fixture(g, p, &mut receipt).unwrap();
        for sm in [1u64, 17] {
            let d = x.c.mul(&x.source, sm);
            for (case, k) in [37u64, 2026, 7001].into_iter().enumerate() {
                let known = k % x.ell;
                let t = x.c.mul(&d, known);
                let mut all = vec![];
                for arm in 0..4 {
                    let mut prior = None;
                    let mut first = None;
                    for rep in 0..2 {
                        counted::reset();
                        let result = run(arm, &x.c, &d, &t, x.ell, case);
                        let counts = counted::snapshot();
                        assert_eq!(result.0, Some(known));
                        assert_eq!(result.5, g == 2);
                        if let Some(previous) = &prior {
                            assert_eq!(previous, &counts);
                            assert_eq!(first, Some(result))
                        } else {
                            first = Some(result)
                        }
                        prior = Some(counts.clone());
                        let name = ["batch", "pencil", "folded", "expanded"][arm];
                        if rep == 0 {
                            writeln!(
                                outcomes,
                                "{g}\t{p}\t{sm}\t{case}\t{name}\t{}\t{}\t{}\t{}\t{}\t{known}",
                                result.1, result.2, result.3, result.4, result.5
                            )
                            .unwrap()
                        }
                        for (modulus, v) in counts {
                            assert!(modulus == p || modulus == x.ell);
                            assert_eq!(v[8], 2 * v[7] + v[5] + v[6]);
                            print!("{g}\t{p}\t{sm}\t{case}\t{name}\t{rep}\t{modulus}\t{known}");
                            for n in v {
                                print!("\t{n}")
                            }
                            println!()
                        }
                    }
                    all.push(first.unwrap())
                }
                assert_eq!(all[0], all[1]);
                assert_eq!(all[2], all[3]);
            }
        }
    }
}
