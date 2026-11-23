#[path = "walk.rs"]
mod core;
#[path = "pencil.rs"]
mod pencil;
#[path = "../production/qualified.rs"]
mod qualified;
use hec_native_ic::counted;
use std::{fs, io::Write};
fn main() {
    let prior =
        std::fs::read_to_string("research/hec_residual_span_20261116/counted/CONTROL.tsv").unwrap();
    let mut phase_out =
        std::fs::File::create_new("research/hec_residual_span_20261116/phases/PHASES.tsv").unwrap();
    writeln!(
        phase_out,
        "g\tp\tsm\tcase\tarm\trep\tphase\tmodulus\tcounts"
    )
    .unwrap();
    let mut receipt =
        fs::File::create_new("research/hec_residual_span_20261116/phases/FIXTURES.tsv").unwrap();
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
                        let phases = counted::phases();
                        let mut sums = counted::Counts::new();
                        for ((phase, modulus), v) in &phases {
                            for j in 0..16 {
                                sums.entry(*modulus).or_insert([0; 16])[j] += v[j]
                            }
                            write!(
                                phase_out,
                                "{g}\t{p}\t{sm}\t{case}\t{}\t{rep}\t{phase}\t{modulus}",
                                ["pencil", "folded", "expanded"][arm]
                            )
                            .unwrap();
                            for n in v {
                                write!(phase_out, "\t{n}").unwrap()
                            }
                            writeln!(phase_out).unwrap()
                        }
                        assert_eq!(sums, counts);

                        assert_eq!(k, Some(known));
                        if let Some(old) = &previous {
                            assert_eq!(old, &counts)
                        }
                        previous = Some(counts.clone());
                        let name = ["pencil", "folded", "expanded"][arm];
                        for (modulus, v) in counts {
                            assert!(modulus == p || modulus == x.ell);
                            let refrow = prior
                                .lines()
                                .skip(1)
                                .map(|l| l.split('\t').collect::<Vec<_>>())
                                .find(|r| {
                                    r[0] == g.to_string()
                                        && r[1] == p.to_string()
                                        && r[2] == sm.to_string()
                                        && r[3] == case.to_string()
                                        && r[4] == name
                                        && r[5] == rep.to_string()
                                        && r[6] == modulus.to_string()
                                })
                                .unwrap();
                            for j in 0..16 {
                                assert_eq!(v[j], refrow[8 + j].parse::<u64>().unwrap())
                            }

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
