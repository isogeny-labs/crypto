#[path = "../production/core.rs"]
mod core;
#[path = "../pencil-runtime/core.rs"]
mod pencil;
#[path = "../production/qualified.rs"]
mod qualified;
use hec_native_ic::counted;
use std::{fs, io::Write};
fn main() {
    let mut receipt =
        fs::File::create_new("research/hec_residual_span_20261116/counted/FIXTURES.tsv").unwrap();
    writeln!(receipt, "g\tp\tc\tstatus").unwrap();
    println!("g\tp\tsm\tcase\tarm\trep\tmodulus\tk\tnew\tadd\tsub\tneg\tmul\tpre\tmul_pre\treduce128\tredc\tinv\tinv_steps\tpoly_products\tpoly_divrem\tgroup_add\tgroup_add_place\tgroup_mul");
    for (g, p) in [(2usize, 101u64), (3, 113), (4, 109)] {
        let x = qualified::fixture(g, p, &mut receipt).unwrap();
        for sm in [1u64, 17] {
            let source = x.c.mul(&x.source, sm);
            for (case, known) in [37u64, 2026, 7001].into_iter().enumerate() {
                let known = known % x.ell;
                let target = x.c.mul(&source, known);
                for arm in 0..3 {
                    let mut previous = None;
                    for rep in 0..2 {
                        counted::reset();
                        let k = match arm {
                            0 => pencil::solve(&x.c, &source, &target, x.ell).k,
                            1 => {
                                core::solve(
                                    &x.c,
                                    &source,
                                    &target,
                                    x.ell,
                                    2026111600 + case as u64,
                                    core::Mode::Folded,
                                )
                                .k
                            }
                            2 => {
                                core::solve(
                                    &x.c,
                                    &source,
                                    &target,
                                    x.ell,
                                    2026111600 + case as u64,
                                    core::Mode::Expanded,
                                )
                                .k
                            }
                            _ => unreachable!(),
                        };
                        let counts = counted::snapshot();
                        assert_eq!(k, Some(known));
                        if let Some(old) = &previous {
                            assert_eq!(old, &counts)
                        }
                        previous = Some(counts.clone());
                        let name = ["pencil", "folded", "expanded"][arm];
                        for (modulus, v) in counts {
                            assert!(modulus == p || modulus == x.ell);
                            assert_eq!(v[8], 2 * v[7] + v[5] + v[6]);
                            print!(
                                "{g}\t{p}\t{sm}\t{case}\t{name}\t{rep}\t{modulus}\t{}",
                                k.unwrap()
                            );
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
