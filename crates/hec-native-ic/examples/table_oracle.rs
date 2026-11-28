#[path = "../../../research/hec_table_oracle_20261114/pipeline.rs"]
mod pipeline;
#[path = "../src/table_factor.rs"]
mod table_factor;
use hec_native_ic::{
    instance::{factor_base, pick_instance},
    oracle::{split, split_reference, OracleStats, Roots},
    poly::Poly,
    rng::Rng,
};
use std::{hint::black_box, time::Instant};
fn checksum(r: Option<Roots>) -> u64 {
    r.map_or(0, |r| {
        r.as_slice()
            .iter()
            .fold(1, |a, &(x, m)| a.wrapping_add(x.wrapping_mul(m as u64 + 1)))
    })
}
fn main() {
    if std::env::var("HEC_PIPELINE").is_ok() {
        run_pipeline();
        return;
    }
    let repeats: u64 = std::env::var("HEC_REPEATS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    for (g, p) in [(2, 503), (2, 4001), (3, 61), (3, 401), (4, 61), (4, 127)] {
        let i = pick_instance(p, g).unwrap();
        let fb = factor_base(&i.curve);
        let k = i.ell / 3 + 7;
        let target = i.curve.mul(&i.d1, k);
        let factorial = (1..=g).product::<usize>();
        let count = fb.len() * factorial;
        for rep in 0..repeats {
            // Shared fixture: real reduced walk divisors. Its generation is excluded
            // from every oracle arm, so this is explicitly a component experiment.
            let mut rng = Rng::new(20261114 + rep);
            let mut r = i.curve.add(&i.d1, &target);
            let mut polys: Vec<Poly> = Vec::with_capacity(count);
            for _ in 0..count {
                r = match rng.below(4) {
                    0 => i.curve.add(&r, &i.d1),
                    1 => i.curve.add(&r, &target),
                    _ => {
                        let (x, y) = fb[rng.below(fb.len() as u64) as usize];
                        i.curve.add_place(&r, x, y)
                    }
                };
                polys.push(r.u);
            }
            let clock = Instant::now();
            let tables = table_factor::Tables::new(p);
            let setup = clock.elapsed().as_nanos();
            for u in &polys {
                assert_eq!(
                    tables.split(u),
                    split_reference(u, &i.curve.fp, &mut OracleStats::default()),
                    "production walk oracle gate"
                );
            }
            let mut sums = Vec::new();
            let order = if rep % 2 == 0 {
                ["reference", "direct", "table"]
            } else {
                ["table", "direct", "reference"]
            };
            for arm in order {
                let now = Instant::now();
                let mut sum = 0u64;
                for u in &polys {
                    let roots = match arm {
                        "reference" => {
                            split_reference(black_box(u), &i.curve.fp, &mut OracleStats::default())
                        }
                        "direct" => split(black_box(u), &i.curve.fp, &mut OracleStats::default()),
                        _ => tables.split(black_box(u)),
                    };
                    sum = sum.wrapping_add(checksum(black_box(roots)));
                }
                let scan = now.elapsed().as_nanos();
                sums.push(sum);
                println!("JSON {{\"g\":{g},\"p\":{p},\"repeat\":{rep},\"arm\":\"{arm}\",\"n\":{count},\"scan_ns\":{scan},\"setup_ns\":{},\"cold_ns\":{},\"bytes\":{},\"checksum\":{sum},\"verified\":true}}",if arm=="table"{setup}else{0},scan+if arm=="table"{setup}else{0},if arm=="table"{tables.bytes()}else{0});
            }
            assert!(sums.iter().all(|s| s == &sums[0]));
        }
    }
}

fn run_pipeline() {
    use hec_native_ic::{
        ic::{solve_dlp, IcParams},
        solve::Solver,
    };
    let seeds: u64 = std::env::var("HEC_SEEDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    for (g, p) in [
        (2, 101),
        (2, 251),
        (2, 503),
        (2, 1009),
        (2, 2003),
        (2, 4001),
        (3, 61),
        (3, 211),
        (3, 401),
        (4, 31),
        (4, 61),
    ] {
        let i = pick_instance(p, g).unwrap();
        let k = i.ell / 3 + 7;
        let target = i.curve.mul(&i.d1, k);
        for seed in 0..seeds {
            let mut arms = Vec::new();
            for branches in [1, 64] {
                for hybrid in [false, true] {
                    for oracle in ["reference", "direct", "table"] {
                        arms.push((branches, hybrid, oracle));
                    }
                }
            }
            if seed % 2 == 1 {
                arms.reverse();
            }
            for (branches, hybrid, oracle) in arms {
                let params = IcParams {
                    seed: 20260916 + seed,
                    branches,
                    solver: if hybrid { Solver::Hybrid } else { Solver::Fast },
                    reference_oracle: oracle == "reference",
                    ..IcParams::default()
                };
                let r = if oracle == "table" {
                    pipeline::solve_table(&i.curve, &i.d1, &target, i.ell, &params)
                } else {
                    solve_dlp(&i.curve, &i.d1, &target, i.ell, &params)
                };
                assert_eq!(r.k, Some(k), "g={g} p={p} seed={seed}");
                println!("PIPE {{\"g\":{g},\"p\":{p},\"seed\":{seed},\"branches\":{branches},\"hybrid\":{hybrid},\"oracle\":\"{oracle}\",\"trials\":{},\"rows\":{},\"pivot_hash\":\"{:016x}\",\"solve_ns\":{},\"fb_ns\":{},\"oracle_ns\":{},\"total_ns\":{},\"verified\":true}}",r.trials,r.relations,r.solve.pivot_hash,r.solve_ns,r.fb_ns,r.oracle_ns,r.total_ns);
            }
        }
    }
}

#[cfg(test)]
mod pipeline_tests {
    use super::*;
    use hec_native_ic::{
        ic::{solve_dlp, IcParams},
        solve::Solver,
    };
    #[test]
    fn same_catalogue_walk_rank_and_answer() {
        for (g, p) in [
            (2, 101),
            (2, 251),
            (2, 503),
            (2, 1009),
            (2, 2003),
            (2, 4001),
            (3, 61),
            (3, 211),
            (3, 401),
            (4, 31),
            (4, 61),
        ] {
            let i = pick_instance(p, g).unwrap();
            assert_eq!(
                table_factor::Tables::for_degree(p, g).factor_base(&i.curve),
                factor_base(&i.curve)
            );
            let k = i.ell / 3 + 7;
            let target = i.curve.mul(&i.d1, k);
            for seed in 0..5 {
                for branches in [1, 64] {
                    for solver in [Solver::Fast, Solver::Hybrid] {
                        let params = IcParams {
                            seed: 20260916 + seed,
                            branches,
                            solver,
                            ..IcParams::default()
                        };
                        let a = solve_dlp(&i.curve, &i.d1, &target, i.ell, &params);
                        let b = pipeline::solve_table(&i.curve, &i.d1, &target, i.ell, &params);
                        assert_eq!(b.k, Some(k));
                        assert_eq!(
                            (
                                a.k,
                                a.trials,
                                a.relations,
                                a.solve_rounds,
                                a.solve.pivot_hash,
                                a.solve.pivots,
                                a.solve.row_ops,
                                a.solve.mulmods
                            ),
                            (
                                b.k,
                                b.trials,
                                b.relations,
                                b.solve_rounds,
                                b.solve.pivot_hash,
                                b.solve.pivots,
                                b.solve.row_ops,
                                b.solve.mulmods
                            ),
                            "g={g} p={p} seed={seed} b={branches}"
                        );
                    }
                }
            }
        }
    }
}
