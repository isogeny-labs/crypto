use hec_native_ic::{
    counted,
    instance::pick_instance,
    jac::{Curve, Div},
    rng::Rng,
};
use std::{hint::black_box, time::Instant};
const L: u64 = 239753;
#[path = "../hec_carried_pencil_20261118/buckets.rs"]
mod buckets;
#[path = "../hec_carried_pencil_20261118/carried.rs"]
mod carried;
#[path = "../hec_lazy_affine_dag_20261118/dag.rs"]
mod dag;
#[path = "solver.rs"]
mod delayed;
mod jet;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/finish.rs"]
mod finish;
#[path = "../hec_residual_span_20261116/native-pencil-images/images.rs"]
mod images;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/linear.rs"]
mod linear;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/pencil.rs"]
mod pencil;
#[path = "../hec_place_pencil_20261118/place.rs"]
mod place;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/prepared.rs"]
mod prepared;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/public.rs"]
mod public;
#[path = "../hec_slope_recurrence_20261118/slopes.rs"]
mod slopes;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/small.rs"]
mod small;
#[path = "../hec_carried_pencil_20261118/solver.rs"]
mod solver;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/table_oracle.rs"]
mod table_oracle;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/tail.rs"]
mod tail;
#[path = "../hec_residual_span_20261116/native-deferred-membership/validate.rs"]
mod validate;
#[path = "../hec_wide_sieve_20261118/wide.rs"]
mod wide;
fn main() {
    let fixtures = include_str!("../hec_residual_span_20261116/native-pencil-3lp/INSTANCES.tsv");
    for line in fixtures.lines().skip(1) {
        let cols: Vec<_> = line.split('\t').collect();
        let g = cols[0].parse::<usize>().unwrap();
        let p = cols[1].parse::<u64>().unwrap();
        let ell = cols[3].parse().unwrap();
        counted::disable();
        let inst = pick_instance(p, g).unwrap();
        assert_eq!((inst.c, inst.ell), (cols[2].parse().unwrap(), ell));
        let c = &inst.curve;
        let mut rng = Rng::new(2026112907 ^ p ^ ((g as u64) << 32));
        for case in 0..4 {
            let d = c.mul(&inst.d1, 1 + rng.below(ell - 1));
            let truth = 1 + rng.below(ell - 1);
            let target = c.mul(&d, truth);
            let seed = 2026112200 ^ p ^ ((g as u64) << 32) ^ case as u64;
            let run = |arm: usize, audit: bool| {
                assert!(validate::eligible(c, &d, &target, ell));
                let prep = prepared::prepare(c, audit);
                if arm == 0 {
                    let r = pencil::solve(
                        c,
                        &d,
                        &target,
                        ell,
                        512,
                        audit,
                        &prep,
                        pencil::Options {
                            normalize: true,
                            finish: false,
                            seed,
                        },
                    );
                    (r.k, r.anchors, 0, r.accepted)
                } else {
                    let r = delayed::solve(
                        c,
                        &d,
                        &target,
                        ell,
                        512,
                        audit,
                        &prep,
                        delayed::Options {
                            normalize: true,
                            finish: false,
                            seed,
                            certificate: false,
                            rolling: true,
                            carry: true,
                            indexed: true,
                            mode: if g == 3 { 1 } else { 3 },
                            eager: false,
                            orders: if arm >= 2 {2} else {0},
                            jet: arm == 3,
                        },
                    );
                    (r.k, r.anchors, r.startup, r.accepted)
                }
            };
            let audited: Vec<_> = (0..4).map(|arm| run(arm, true)).collect();


            assert_eq!(audited[2], audited[3]);
            for arm in 0..4 {
                counted::reset();
                let result = run(arm, false);
                let counters = counted::snapshot();
                counted::disable();
                assert_eq!(result, audited[arm]);
                if let Some(k) = result.0 {
                    assert_eq!(c.mul(&d, k), target);
                    assert_eq!(k, truth);
                }
                let mut red = [0; 2];
                for (m, v) in counters {
                    let i = if m == p {
                        0
                    } else {
                        assert_eq!(m, ell);
                        1
                    };
                    red[i] = v[8];
                    println!("COUNT g={g} p={p} case={case} arm={arm} modulus={m} vector={v:?}");
                }
                println!("CELL g={g} p={p} case={case} arm={arm} solved={} anchors={} startup={} accepted={} base={} subgroup={}",result.0.is_some(),result.1,result.2,result.3,red[0],red[1]);
            }
            for rep in 0..12 {
                for h in 0..4 {
                    let arm = (h + rep + case) % 4;
                    let start = Instant::now();
                    let r = std::hint::black_box(run(arm, false));
                    let ns = start.elapsed().as_nanos();
                    assert_eq!(r, audited[arm]);
                    println!("WALL g={g} p={p} case={case} rep={rep} arm={arm} ns={ns}");
                }
            }
        }
    }
}
