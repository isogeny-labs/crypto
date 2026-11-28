mod core;
mod qualified;
use hec_native_ic::ic::{solve_dlp, IcParams};
use std::{fs, io::Write};
fn main() {
    let mut fixtures =
        fs::File::create_new("research/hec_residual_span_20261116/production/FIXTURES.tsv")
            .unwrap();
    writeln!(fixtures, "g\tp\tc\tstatus").unwrap();
    println!("g\tp\tc\tm\torbits\tell\tsource_multiplier\tcase\tqualified_trials\tfolded_k\tfolded_trials\tfolded_relations\tfolded_state\tmax_folded_row\texpanded_k\texpanded_trials\tplain_k\tplain_trials\tnative_k\tnative_trials\tcorrect");
    let mut failures = 0;
    for (g, p) in [(2usize, 101u64), (3, 113), (4, 109)] {
        let Some(base) = qualified::fixture(g, p, &mut fixtures) else {
            failures += 1;
            continue;
        };
        for sm in [1u64, 17] {
            let mut x = base.clone();
            x.source = x.c.mul(&base.source, sm);
            assert!(!x.c.is_zero(&x.source));
            assert!(x.c.is_zero(&x.c.mul(&x.source, x.ell)));
            for (case, k0) in [37u64, 2026, 7001].into_iter().enumerate() {
                let known = k0 % x.ell;
                let target = x.c.mul(&x.source, known);
                let (qk, qt, qr, _, _) = qualified::gate(&x, case, &target);
                let seed = 2026111600 + case as u64;
                let folded = core::solve(&x.c, &x.source, &target, x.ell, seed, core::Mode::Folded);
                let expanded =
                    core::solve(&x.c, &x.source, &target, x.ell, seed, core::Mode::Expanded);
                let plain = core::solve(&x.c, &x.source, &target, x.ell, seed, core::Mode::Plain);
                let native = solve_dlp(
                    &x.c,
                    &x.source,
                    &target,
                    x.ell,
                    &IcParams {
                        seed,
                        max_trials: 200000,
                        ..IcParams::default()
                    },
                );
                let correct = [qk, folded.k, expanded.k, plain.k, native.k]
                    .iter()
                    .all(|&k| k == Some(known));
                if !correct {
                    failures += 1
                }
                assert_eq!(folded.k, qk);
                assert_eq!(expanded.k, qk);
                assert_eq!(folded.trials, qt);
                assert_eq!(expanded.trials, qt);
                assert_eq!(folded.relations, qr);
                assert_eq!(expanded.relations, qr);
                assert_eq!(folded.state_len, x.reps.len());
                assert!(folded.max_row_len <= x.reps.len() + 1);
                assert_eq!(folded.m, x.fb.len());
                assert_eq!(folded.unknowns, x.reps.len());
                assert_eq!(expanded.unknowns, x.fb.len());
                println!("{g}\t{p}\t{}\t{}\t{}\t{}\t{sm}\t{case}\t{qt}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{correct}",x.constant,x.fb.len(),x.reps.len(),x.ell,folded.k.unwrap_or(0),folded.trials,folded.relations,folded.state_len,folded.max_row_len,expanded.k.unwrap_or(0),expanded.trials,plain.k.unwrap_or(0),plain.trials,native.k.unwrap_or(0),native.trials);
                eprintln!("case g={g} p={p} source_multiplier={sm} target_case={case} source={:?} target={target:?} folded={folded:?} expanded={expanded:?} plain={plain:?}",x.source);
            }
        }
    }
    if failures != 0 {
        eprintln!("INCOMPLETE_STANDARD_INPUT_CONTROLS count={failures}");
        std::process::exit(2)
    }
}
