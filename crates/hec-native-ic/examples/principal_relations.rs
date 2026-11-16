#[path = "../src/interpolation.rs"]
mod interpolation;
use hec_native_ic::{
    ic::{solve_dlp, IcParams},
    instance::pick_instance,
    solve::Solver,
};
fn main() {
    let gate = std::env::var("HEC_GATE").is_ok();
    let seeds: u64 = std::env::var("HEC_SEEDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    let rows = if std::env::var("HEC_EXTEND").is_ok() {
        vec![(2, 8009), (2, 16001), (3, 1009)]
    } else {
        vec![
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
        ]
    };
    for (g, p) in rows {
        let Some(i) = pick_instance(p, g) else {
            println!("SKIP g={g} p={p}");
            continue;
        };
        let k = i.ell / 3 + 7;
        let target = i.curve.mul(&i.d1, k);
        for seed in 0..seeds {
            let arms = if seed % 2 == 0 {
                vec![
                    "fast",
                    "hybrid",
                    "fast-direct",
                    "hybrid-direct",
                    "one-step",
                    "principal",
                ]
            } else {
                vec![
                    "principal",
                    "one-step",
                    "hybrid-direct",
                    "fast-direct",
                    "hybrid",
                    "fast",
                ]
            };
            for arm in arms {
                if gate && arm != "principal" {
                    continue;
                }
                if arm == "principal" {
                    let r = interpolation::solve(
                        &i.curve,
                        &i.d1,
                        &target,
                        i.ell,
                        20260916 + seed,
                        gate,
                    );
                    println!("JSON {{\"g\":{g},\"p\":{p},\"c\":{},\"ell\":{},\"seed\":{seed},\"arm\":\"{arm}\",\"m\":{},\"trials\":{},\"accepted\":{},\"solve_rounds\":{},\"max_weight\":{},\"descent_steps\":{},\"descent_ns\":{},\"relation_ns\":{},\"solve_ns\":{},\"total_ns\":{},\"verified\":{},\"capped\":{}}}",i.c,i.ell,r.m,r.trials,r.accepted,r.solve_rounds,r.max_weight,r.descent_steps,r.descent_ns,r.relation_ns,r.solve_ns,r.total_ns,r.k==Some(k),r.k.is_none());
                } else {
                    let hybrid = arm.starts_with("hybrid");
                    let direct = arm.ends_with("direct");
                    let branches = if arm == "one-step" { 1 } else { 64 };
                    let r = solve_dlp(
                        &i.curve,
                        &i.d1,
                        &target,
                        i.ell,
                        &IcParams {
                            seed: 20260916 + seed,
                            solver: if hybrid { Solver::Hybrid } else { Solver::Fast },
                            reference_oracle: !direct,
                            branches,
                            ..IcParams::default()
                        },
                    );
                    println!("JSON {{\"g\":{g},\"p\":{p},\"c\":{},\"ell\":{},\"seed\":{seed},\"arm\":\"{arm}\",\"m\":{},\"trials\":{},\"accepted\":{},\"solve_rounds\":{},\"solve_ns\":{},\"total_ns\":{},\"verified\":{}}}",i.c,i.ell,r.m,r.trials,r.relations,r.solve_rounds,r.solve_ns,r.total_ns,r.k==Some(k));
                }
            }
        }
    }
}
