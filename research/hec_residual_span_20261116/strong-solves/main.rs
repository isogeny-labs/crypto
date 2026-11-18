#[path = "../batched/core.rs"]
mod batch;
mod order;
#[path = "../normalized/pencil.rs"]
mod pencil;
#[path = "../strength/qualified.rs"]
mod qualified;
#[path = "../normalized/walk.rs"]
mod walk;
use hec_native_ic::{
    counted,
    jac::{Curve, Div},
    rng::Rng,
};
use std::{fs, io::Write};
type Outcome = (Option<u64>, usize, u64, u64, u64, bool);
fn run(arm: usize, c: &Curve, d: &Div, t: &Div, ell: u64, case: usize) -> Outcome {
    match arm {
        0 => {
            let r = batch::solve(c, d, t, ell);
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
            let r = pencil::solve(c, d, t, ell);
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
                2026111800 + case as u64,
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
fn prime(p: u64) -> bool {
    p >= 2 && (2..).take_while(|d| d * d <= p).all(|d| p % d != 0)
}
fn main() {
    let root = "research/hec_residual_span_20261116/strong-solves";
    let mut receipt = fs::File::create_new(format!("{root}/FIXTURES.tsv")).unwrap();
    writeln!(receipt, "g\tp\tc\tstatus").unwrap();
    let mut outcomes = fs::File::create_new(format!("{root}/OUTCOMES.tsv")).unwrap();
    writeln!(outcomes,"g\tp\tc\tell\tm\torbits\tsm\tcase\tarm\trows\ttrials\tanchor\tt\tnormalized\tk\texpected\tcorrect").unwrap();
    println!("g\tp\tsm\tcase\tarm\trep\tmodulus\tk\tnew\tadd\tsub\tneg\tmul\tpre\tmul_pre\treduce128\tredc\tinv\tinv_steps\tpoly_products\tpoly_divrem\tgroup_add\tgroup_add_place\tgroup_mul");
    let (mut models, mut calls, mut missing, mut incomplete) = (0, 0, 0, 0);
    for (g, p) in [(2usize, 101u64), (2, 251), (2, 601), (3, 113), (3, 421)] {
        assert!(prime(p));
        assert_eq!((p - 1) % (2 * g + 1) as u64, 0);
        counted::disable();
        eprintln!("qualification_start g={g} p={p}");
        let Some(x) = qualified::fixture(g, p, &mut receipt) else {
            missing += 1;
            eprintln!("no_eligible_model g={g} p={p}");
            continue;
        };
        models += 1;
        let exact_order = order::jacobian_order(&x.c);
        assert_eq!(exact_order, x.ell);
        assert!((x.ell as u128) * 4 >= qualified::upper_bound(g, p) as u128);
        eprintln!("whole_jacobian_order g={g} p={p} N={exact_order} prime_order=true");
        eprintln!(
            "qualified g={g} p={p} c={} ell={} m={} orbits={} h={} lambda={}",
            x.constant,
            x.ell,
            x.fb.len(),
            x.reps.len(),
            x.h,
            x.lambda
        );
        let mut rng = Rng::new(20261118 ^ ((g as u64) << 32) ^ p);
        for sm in [1u64, 17] {
            let d = x.c.mul(&x.source, sm);
            if x.c.is_zero(&d) {
                missing += 1;
                eprintln!("zero_public_source g={g} p={p} sm={sm}");
                continue;
            }
            let mut knowns = vec![];
            for case in 0..2 {
                let mut known = rng.below(x.ell - 1) + 1;
                while knowns.contains(&known) {
                    known = rng.below(x.ell - 1) + 1
                }
                knowns.push(known);
                let t = x.c.mul(&d, known);
                eprintln!("public g={g} p={p} sm={sm} case={case} expected={known} source={d:?} target={t:?}");
                let mut all = vec![];
                for arm in 0..4 {
                    let mut old = None;
                    let mut first = None;
                    for rep in 0..2 {
                        counted::reset();
                        let result = run(arm, &x.c, &d, &t, x.ell, case);
                        let counts = counted::snapshot();
                        counted::disable();
                        calls += 1;
                        if let Some(k) = result.0 {
                            assert_eq!(x.c.mul(&d, k), t);
                            assert_eq!(k, known)
                        } else if rep == 0 {
                            incomplete += 1
                        }
                        if let Some(previous) = &old {
                            assert_eq!(previous, &counts);
                            assert_eq!(first, Some(result))
                        } else {
                            first = Some(result)
                        }
                        old = Some(counts.clone());
                        let name = ["batch", "pencil", "folded", "expanded"][arm];
                        if rep == 0 {
                            writeln!(outcomes,"{g}\t{p}\t{}\t{}\t{}\t{}\t{sm}\t{case}\t{name}\t{}\t{}\t{}\t{}\t{}\t{}\t{known}\t{}",x.constant,x.ell,x.fb.len(),x.reps.len(),result.1,result.2,result.3,result.4,result.5,result.0.map(|v|v.to_string()).unwrap_or("none".into()),result.0==Some(known)).unwrap()
                        }
                        for (modulus, v) in counts {
                            assert!(modulus == p || modulus == x.ell);
                            assert_eq!(v[8], 2 * v[7] + v[5] + v[6]);
                            print!(
                                "{g}\t{p}\t{sm}\t{case}\t{name}\t{rep}\t{modulus}\t{}",
                                result.0.map(|v| v.to_string()).unwrap_or("none".into())
                            );
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
                assert!(all.iter().all(|x| x.5 == all[0].5));
            }
        }
    }
    eprintln!("models={models} calls={calls} missing_inputs={missing} incomplete_workload_arms={incomplete}");
    if models != 5 || calls != 160 || missing != 0 || incomplete != 0 {
        std::process::exit(2)
    }
}
