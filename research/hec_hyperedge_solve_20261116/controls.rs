#[path = "../hec_anchor_relations_20261114/anchor.rs"]
mod anchor;
#[path = "pipeline_compat.rs"]
mod pipeline;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use hec_native_ic::{
    ic::{self, IcParams},
    instance::pick_instance,
    solve::Solver,
};
fn main() {
    let mut cases = 0;
    let mut verified = 0;
    for (g, p) in [(2, 1009), (3, 503), (4, 127)] {
        let inst = pick_instance(p, g).unwrap();
        for (index, k) in [113, 136].into_iter().enumerate() {
            let target = inst.curve.mul(&inst.d1, k);
            let seed = 20260916 + index as u64;
            let r = anchor::solve_compact(&inst.curve, &inst.d1, &target, inst.ell, seed, true);
            if let Some(answer) = r.k {
                assert_eq!(answer, k);
                assert_eq!(inst.curve.mul(&inst.d1, answer), target);
                verified += 1;
            }
            println!("COMPACT g={g} p={p} target_index={index} k={:?} anchors={} draws={} unique_tuples={} constructed={} admitted={} pure_rows={} descent_steps={} raw_checks={} projected_checks={}",r.k,r.anchors,r.draws,r.unique_tuples,r.unique_tuples*(1<<g),r.admitted,r.pure_rows,r.descent_steps,r.raw_checks,r.pure_checks);
            cases += 1;
            for table in [false, true] {
                for branches in [1, 64] {
                    for solver in [Solver::Fast, Solver::Hybrid] {
                        for reference in [true, false] {
                            if table && !reference {
                                continue;
                            }
                            let params = IcParams {
                                branches,
                                solver,
                                reference_oracle: reference,
                                seed,
                                max_trials: 200000,
                                ..IcParams::default()
                            };
                            let r = if table {
                                pipeline::solve_table(
                                    &inst.curve,
                                    &inst.d1,
                                    &target,
                                    inst.ell,
                                    &params,
                                )
                            } else {
                                ic::solve_dlp(&inst.curve, &inst.d1, &target, inst.ell, &params)
                            };
                            if let Some(answer) = r.k {
                                assert_eq!(answer, k);
                                assert_eq!(inst.curve.mul(&inst.d1, answer), target);
                                verified += 1;
                            }
                            println!("CONTROL g={g} p={p} target_index={index} table={table} branches={branches} solver={solver:?} reference_oracle={reference} k={:?} trials={} relations={} solve_mulmods={}",r.k,r.trials,r.relations,r.solve.mulmods);
                            cases += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases, 78);
    println!(
        "TERMINAL cases={cases} verified={verified} incomplete={} timing_claim=false",
        cases - verified
    );
}
