#[path = "../production/core.rs"]
mod core;
#[path = "../production/qualified.rs"]
mod qualified;
use hec_native_ic::{
    ic::{solve_dlp, IcParams},
    jac::{Curve, Div},
};
use std::{fs, hint::black_box, io::Write, time::Instant};
struct Workload {
    fixture: usize,
    source_mul: u64,
    case: usize,
    source: Div,
    target: Div,
    known: u64,
}
fn run(arm: usize, c: &Curve, source: &Div, target: &Div, ell: u64, case: usize) -> Option<u64> {
    let seed = 2026111600 + case as u64;
    match arm {
        0 => core::solve(c, source, target, ell, seed, core::Mode::Folded).k,
        1 => core::solve(c, source, target, ell, seed, core::Mode::Expanded).k,
        2 => core::solve(c, source, target, ell, seed, core::Mode::Plain).k,
        3 | 4 => {
            solve_dlp(
                c,
                source,
                target,
                ell,
                &IcParams {
                    seed,
                    max_trials: 200000,
                    ..IcParams::default()
                },
            )
            .k
        }
        _ => unreachable!(),
    }
}
fn main() {
    let root = "research/hec_residual_span_20261116/production-cost";
    let mut receipt = fs::File::create_new(format!("{root}/FIXTURES.tsv")).unwrap();
    writeln!(receipt, "g\tp\tc\tstatus").unwrap();
    let mut inputs = fs::File::create_new(format!("{root}/WORKLOADS.tsv")).unwrap();
    writeln!(
        inputs,
        "id\tg\tp\tc\tell\tsource_multiplier\tcase\texpected"
    )
    .unwrap();
    let mut fixtures = vec![];
    let mut workloads = vec![];
    for (g, p) in [(2usize, 101u64), (3, 113), (4, 109)] {
        let x = qualified::fixture(g, p, &mut receipt).expect("qualified public instance");
        let index = fixtures.len();
        for sm in [1u64, 17] {
            let source = x.c.mul(&x.source, sm);
            assert!(!x.c.is_zero(&source));
            assert!(x.c.is_zero(&x.c.mul(&source, x.ell)));
            for (case, k0) in [37u64, 2026, 7001].into_iter().enumerate() {
                let known = k0 % x.ell;
                let target = x.c.mul(&source, known);
                let id = workloads.len();
                writeln!(
                    inputs,
                    "{id}\t{g}\t{p}\t{}\t{}\t{sm}\t{case}\t{known}",
                    x.constant, x.ell
                )
                .unwrap();
                eprintln!("public workload={id} source={source:?} target={target:?}");
                workloads.push(Workload {
                    fixture: index,
                    source_mul: sm,
                    case,
                    source,
                    target,
                    known,
                });
            }
        }
        fixtures.push(x)
    }
    assert_eq!(workloads.len(), 18);
    println!("batch\tworkload\trep\tarm\tns\tk\tcorrect");
    let names = ["folded", "expanded", "plain_prefix", "native", "native_aa"];
    let mut failures = 0;
    for batch in 0..5 {
        for (id, w) in workloads.iter().enumerate() {
            let x = &fixtures[w.fixture];
            for rep in 0..16 {
                for step in 0..5 {
                    let arm = (batch + rep + step) % 5;
                    let t = Instant::now();
                    let k = black_box(run(
                        arm,
                        black_box(&x.c),
                        black_box(&w.source),
                        black_box(&w.target),
                        x.ell,
                        w.case,
                    ));
                    let ns = t.elapsed().as_nanos();
                    let correct = k == Some(w.known);
                    if !correct {
                        failures += 1
                    }
                    println!(
                        "{batch}\t{id}\t{rep}\t{}\t{ns}\t{}\t{correct}",
                        names[arm],
                        k.map_or_else(|| "incomplete".into(), |k| k.to_string())
                    );
                }
            }
        }
    }
    eprintln!("calls=7200 failures={failures} targets=18 source_variants=2 common_qualification_outside=true source_multiplier_fields={}",workloads.iter().map(|w|w.source_mul as usize).sum::<usize>());
    if failures != 0 {
        std::process::exit(2)
    }
}
