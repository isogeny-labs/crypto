//! Standalone pilot harness, built with the same release library as the crate.
mod anchor;
#[path = "../hec_table_oracle_20261114/pipeline.rs"]
mod pipeline;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use hec_native_ic::{ic::IcParams, instance::pick_instance, solve::Solver};

fn main() {
    let gate = std::env::var_os("HEC_GATE").is_some();
    for (g, p, branches, hybrid) in [
        (2, 101, 1, false),
        (2, 251, 1, false),
        (2, 503, 1, false),
        (2, 1009, 1, false),
        (2, 2003, 1, false),
        (2, 4001, 1, false),
        (3, 61, 1, true),
        (3, 211, 1, false),
        (3, 401, 1, true),
        (4, 31, 64, false),
        (4, 61, 1, false),
    ] {
        let i = pick_instance(p, g).unwrap();
        let k = i.ell / 3 + 7;
        let target = i.curve.mul(&i.d1, k);
        for seed in 0..5 {
            let params = IcParams {
                branches,
                solver: if hybrid { Solver::Hybrid } else { Solver::Fast },
                seed: 20260916 + seed,
                ..IcParams::default()
            };
            let order = if gate {
                vec!["orbit"]
            } else if seed % 2 == 0 {
                vec!["orbit", "table"]
            } else {
                vec!["table", "orbit"]
            };
            for arm in order {
                if arm == "orbit" {
                    let r =
                        anchor::solve_orbit(&i.curve, &i.d1, &target, i.ell, 20260916 + seed, gate);
                    println!("ORBIT {{\"g\":{g},\"p\":{p},\"seed\":{seed},\"arm\":\"orbit\",\"gate\":{gate},\"total_ns\":{},\"setup_ns\":{},\"relation_ns\":{},\"forest_ns\":{},\"solve_ns\":{},\"anchors\":{},\"draws\":{},\"tuples\":{},\"admitted\":{},\"pure_rows\":{},\"raw_checks\":{},\"pure_checks\":{},\"reported_bytes\":{},\"correct\":{},\"capped\":{}}}",r.total_ns,r.setup_ns,r.relation_ns,r.forest_ns,r.solve_ns,r.anchors,r.draws,r.unique_tuples,r.admitted,r.pure_rows,r.raw_checks,r.pure_checks,r.memory_bytes,r.k==Some(k),r.k.is_none());
                    assert_eq!(r.k, Some(k), "orbit g={g} p={p} seed={seed}");
                } else {
                    let r = pipeline::solve_table(&i.curve, &i.d1, &target, i.ell, &params);
                    println!("ORBIT {{\"g\":{g},\"p\":{p},\"seed\":{seed},\"arm\":\"table\",\"gate\":false,\"total_ns\":{},\"branches\":{branches},\"hybrid\":{hybrid},\"correct\":{}}}",r.total_ns,r.k==Some(k));
                    assert_eq!(r.k, Some(k));
                }
            }
        }
    }
}
