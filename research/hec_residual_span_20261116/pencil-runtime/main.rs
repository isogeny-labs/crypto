mod core;
#[path = "../production/qualified.rs"]
mod qualified;
use std::{fs, io::Write};
fn main() {
    let reference =
        fs::read_to_string("research/hec_residual_span_20261116/pencil/PREFIX.tsv").unwrap();
    let mut fixtures =
        fs::File::create_new("research/hec_residual_span_20261116/pencil-runtime/FIXTURES.tsv")
            .unwrap();
    writeln!(fixtures, "g\tp\tc\tstatus").unwrap();
    println!("g\tp\tsm\tcase\tanchor\tt\trows\tk\texact_prefix_match");
    for (g, p) in [(2usize, 101u64), (3, 113), (4, 109)] {
        let x = qualified::fixture(g, p, &mut fixtures).unwrap();
        for sm in [1u64, 17] {
            let source = x.c.mul(&x.source, sm);
            for (case, known) in [37u64, 2026, 7001].into_iter().enumerate() {
                let target = x.c.mul(&source, known % x.ell);
                let rep = core::solve(&x.c, &source, &target, x.ell);
                assert_eq!(rep.k, Some(known % x.ell));
                let first = reference
                    .lines()
                    .skip(1)
                    .map(|l| l.split('\t').collect::<Vec<_>>())
                    .find(|v| {
                        v[0] == g.to_string()
                            && v[1] == p.to_string()
                            && v[2] == sm.to_string()
                            && v[3] == case.to_string()
                            && v[8] != "none"
                    })
                    .unwrap();
                assert_eq!(rep.anchors, first[4].parse::<u64>().unwrap());
                assert_eq!(rep.parameter, first[5].parse::<u64>().unwrap());
                assert_eq!(rep.relations, first[6].parse::<usize>().unwrap());
                assert_eq!(rep.k.unwrap(), first[8].parse::<u64>().unwrap());
                assert_eq!(rep.state_len, x.reps.len());
                assert!(rep.max_row_len <= x.reps.len() + 1);
                println!(
                    "{g}\t{p}\t{sm}\t{case}\t{}\t{}\t{}\t{}\ttrue",
                    rep.anchors,
                    rep.parameter,
                    rep.relations,
                    rep.k.unwrap()
                );
            }
        }
    }
}
