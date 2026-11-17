//! Paired follow-up instrument. Always verifies answers; prints capped LP arms.
use hec_native_ic::{
    ic::{solve_dlp, IcParams},
    instance::pick_instance,
    solve::Solver,
};
fn main() {
    let seeds: u64 = std::env::var("HEC_SEEDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);
    if std::env::var("HEC_TRACK").as_deref() == Ok("lp") {
        for p in [31, 61, 127] {
            let Some(i) = pick_instance(p, 4) else {
                println!("SKIP g=4 p={p}");
                continue;
            };
            let full = hec_native_ic::instance::factor_base(&i.curve).len();
            for seed in 0..seeds.min(3) {
                for (m, lp) in [(full, 0), (full / 2, 0), (full / 2, 1), (full / 2, 2)] {
                    let r = hec_native_ic::large_prime::run(&i, m, lp, 20260916 + seed, 100000);
                    println!("LP g=4 p={p} m={m} lp={lp} seed={seed} trials={} partials={} pivots={} rows={} identity_checks={} total_ms={:.6} verified={} capped={}",r.trials,r.partials,r.partial_pivots,r.rows,r.projected_checks,r.total_ns as f64/1e6,r.k==Some(i.ell/3+7),r.k.is_none());
                }
            }
        }
        return;
    }
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
        for s in 0..seeds {
            let mut variants = vec![
                ("fast", Solver::Fast, true),
                ("hybrid", Solver::Hybrid, true),
                ("oracle", Solver::Fast, false),
            ];
            if s % 2 == 1 {
                variants.reverse();
            }
            let mut reference = None;
            for (name, solver, reference_oracle) in variants {
                let r = solve_dlp(
                    &i.curve,
                    &i.d1,
                    &target,
                    i.ell,
                    &IcParams {
                        seed: 20260916 + s,
                        solver,
                        reference_oracle,
                        ..IcParams::default()
                    },
                );
                assert_eq!(r.k, Some(k), "g={g} p={p} seed={s} {name}");
                if name != "hybrid" {
                    let exact = (r.trials, r.relations, r.solve.pivot_hash, r.solve.mulmods);
                    if let Some(other) = reference {
                        assert_eq!(exact, other, "oracle changed relation stream");
                    } else {
                        reference = Some(exact);
                    }
                }
                println!("JSON {{\"genus\":{g},\"p\":{p},\"seed\":{s},\"arm\":\"{name}\",\"trials\":{},\"solve_ns\":{},\"oracle_ns\":{},\"total_ns\":{},\"mulmods\":{},\"verified\":true}}",r.trials,r.solve_ns,r.oracle_ns,r.total_ns,r.solve.mulmods);
            }
        }
    }
}
