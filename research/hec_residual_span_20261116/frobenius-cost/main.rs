mod character;
#[path = "../cost/orbit.rs"]
mod orbit;
use hec_native_ic::{
    fp::Fp,
    ic::{solve_dlp, IcParams},
    instance::factor_base,
    jac::{Curve, Div},
    poly::Poly,
};
use std::{hint::black_box, time::Instant};
fn run(arm: usize, case: usize, c: &Curve, source: &Div, target: &Div) -> Option<u64> {
    match arm {
        0 | 1 => {
            let fb = factor_base(c);
            let (logs, _, _, _, _) = character::character_logs(c, &fb, source);
            if arm == 1 {
                character::qualify(c, &fb, source, &logs);
            }
            character::recover(c, &fb, source, target, &logs).0
        }
        2 => {
            let fb = factor_base(c);
            let (logs, _, _, _, _) = orbit::orbit_logs(c, &fb, source);
            orbit::recover(c, &fb, source, target, &logs).0
        }
        3 | 4 => {
            solve_dlp(
                c,
                source,
                target,
                8623,
                &IcParams {
                    seed: 2026111600 + case as u64,
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
    let c = Curve::new(
        Fp::new(19),
        Poly::from_coeffs(&[2, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
    );
    assert!(orbit::irreducible(&c.f, &c.fp));
    assert_eq!(hec_native_ic::instance::prime_factors(8623), vec![8623]);
    let fb = factor_base(&c);
    assert_eq!(fb.len(), 9);
    let source = c.mul(&c.place(fb[0].0, fb[0].1), 13);
    assert!(!c.is_zero(&source));
    assert!(c.is_zero(&c.mul(&source, 8623)));
    let known = [37u64, 2026, 7001];
    let targets: Vec<_> = known.iter().map(|&k| c.mul(&source, k)).collect();
    eprintln!("cold interval: per-call catalogue/setup/checks/descent/final source verification and deallocation; common public fixture outside; 5batches*16repeats*3independenttargets*5arms=1200calls");
    println!("batch\tcase\trep\tarm\tns\tk\tcorrect");
    let names = ["character", "qualified", "orbit", "native", "native_aa"];
    let mut failed = 0;
    for batch in 0..5 {
        for case in 0..3 {
            for rep in 0..16 {
                for step in 0..5 {
                    let arm = (batch + rep + step) % 5;
                    let t = Instant::now();
                    let k = black_box(run(
                        arm,
                        case,
                        black_box(&c),
                        black_box(&source),
                        black_box(&targets[case]),
                    ));
                    let ns = t.elapsed().as_nanos();
                    let correct = k == Some(known[case]);
                    if !correct {
                        failed += 1
                    }
                    println!(
                        "{batch}\t{case}\t{rep}\t{}\t{ns}\t{}\t{correct}",
                        names[arm],
                        k.map_or_else(|| "incomplete".into(), |x| x.to_string())
                    );
                }
            }
        }
    }
    if failed != 0 {
        eprintln!("INCOMPLETE_CALLS count={failed}");
        std::process::exit(2)
    }
}
