#[path = "../pencil-runtime/core.rs"]
mod old_pencil;
#[path = "../production/core.rs"]
mod old_walk;
mod pencil;
#[path = "../production/qualified.rs"]
mod qualified;
mod walk;
use hec_native_ic::{
    counted,
    jac::{Curve, Div},
};
use std::{fs, io::Write};
fn run(
    fast: bool,
    arm: usize,
    c: &Curve,
    d: &Div,
    t: &Div,
    ell: u64,
    case: usize,
) -> (Option<u64>, usize, u64, u64, u64) {
    let seed = 2026111600 + case as u64;
    match (fast, arm) {
        (true, 0) => {
            let r = pencil::solve(c, d, t, ell);
            (r.k, r.relations, 0, r.anchors, r.parameter)
        }
        (false, 0) => {
            let r = old_pencil::solve(c, d, t, ell);
            (r.k, r.relations, 0, r.anchors, r.parameter)
        }
        (true, a) => {
            let r = walk::solve(
                c,
                d,
                t,
                ell,
                seed,
                if a == 1 {
                    walk::Mode::Folded
                } else {
                    walk::Mode::Expanded
                },
            );
            (r.k, r.relations, r.trials, 0, 0)
        }
        (false, a) => {
            let r = old_walk::solve(
                c,
                d,
                t,
                ell,
                seed,
                if a == 1 {
                    old_walk::Mode::Folded
                } else {
                    old_walk::Mode::Expanded
                },
            );
            (r.k, r.relations, r.trials, 0, 0)
        }
    }
}
fn main() {
    let mut receipt =
        fs::File::create_new("research/hec_residual_span_20261116/shared-setup/FIXTURES.tsv")
            .unwrap();
    writeln!(receipt, "g\tp\tc\tstatus").unwrap();
    println!("g\tp\tsm\tcase\tarm\trep\tmodulus\tk\tnew\tadd\tsub\tneg\tmul\tpre\tmul_pre\treduce128\tredc\tinv\tinv_steps\tpoly_products\tpoly_divrem\tgroup_add\tgroup_add_place\tgroup_mul");
    for (g, p) in [(2usize, 101u64), (3, 113), (4, 109)] {
        let x = qualified::fixture(g, p, &mut receipt).unwrap();
        for sm in [1u64, 17] {
            let d = x.c.mul(&x.source, sm);
            for (case, k) in [37u64, 2026, 7001].into_iter().enumerate() {
                let known = k % x.ell;
                let t = x.c.mul(&d, known);
                for arm in 0..3 {
                    let old = run(false, arm, &x.c, &d, &t, x.ell, case);
                    let mut prior = None;
                    for rep in 0..2 {
                        counted::reset();
                        let result = run(true, arm, &x.c, &d, &t, x.ell, case);
                        let counts = counted::snapshot();
                        assert_eq!(result, old);
                        assert_eq!(result.0, Some(known));
                        if let Some(previous) = &prior {
                            assert_eq!(previous, &counts)
                        }
                        prior = Some(counts.clone());
                        let name = ["pencil", "folded", "expanded"][arm];
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
                }
            }
        }
    }
}
