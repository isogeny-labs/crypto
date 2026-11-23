//! Interleaved cold pilot; COMPACT-PROTOCOL.md freezes gates and comparison.
mod anchor;
#[path = "../hec_table_oracle_20261114/pipeline.rs"]
mod pipeline;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use hec_native_ic::{ic::IcParams, instance::pick_instance, solve::Solver};

fn fingerprint(r: &anchor::Report) -> [u128; 14] {
    [
        r.k.unwrap_or(u64::MAX) as u128,
        r.m as u128,
        r.anchors as u128,
        r.draws as u128,
        r.unique_tuples as u128,
        r.splits as u128,
        r.admitted as u128,
        r.duplicate_relations as u128,
        r.pure_rows as u128,
        r.unions as u128,
        r.pins as u128,
        r.solves as u128,
        r.descent_steps as u128,
        r.memory_bytes as u128,
    ]
}
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
            let mut order = if gate {
                vec!["orbit", "compact"]
            } else {
                vec!["orbit", "compact", "table"]
            };
            if !gate {
                order.rotate_left(seed as usize % 3);
                if seed % 2 == 1 {
                    order.reverse();
                }
            }
            let mut previous = None;
            for arm in order {
                if arm == "table" {
                    let r = pipeline::solve_table(&i.curve, &i.d1, &target, i.ell, &params);
                    println!("COMPACT {{\"g\":{g},\"p\":{p},\"seed\":{seed},\"arm\":\"table\",\"gate\":false,\"total_ns\":{},\"branches\":{branches},\"hybrid\":{hybrid},\"correct\":{}}}",r.total_ns,r.k==Some(k));
                    assert_eq!(r.k, Some(k));
                } else {
                    let row_gate = gate && arm == "compact";
                    let r = if arm == "compact" {
                        anchor::solve_compact(
                            &i.curve,
                            &i.d1,
                            &target,
                            i.ell,
                            20260916 + seed,
                            row_gate,
                        )
                    } else {
                        anchor::solve_orbit(&i.curve, &i.d1, &target, i.ell, 20260916 + seed, false)
                    };
                    let fp = fingerprint(&r);
                    if let Some(other) = previous {
                        assert_eq!(fp, other, "stream/forest/solve g={g} p={p} seed={seed}");
                    }
                    previous = Some(fp);
                    println!("COMPACT {{\"g\":{g},\"p\":{p},\"seed\":{seed},\"arm\":\"{arm}\",\"gate\":{row_gate},\"total_ns\":{},\"setup_ns\":{},\"relation_ns\":{},\"forest_ns\":{},\"solve_ns\":{},\"anchors\":{},\"draws\":{},\"tuples\":{},\"admitted\":{},\"pure_rows\":{},\"raw_checks\":{},\"pure_checks\":{},\"reported_bytes\":{},\"correct\":{},\"capped\":{}}}",r.total_ns,r.setup_ns,r.relation_ns,r.forest_ns,r.solve_ns,r.anchors,r.draws,r.unique_tuples,r.admitted,r.pure_rows,r.raw_checks,r.pure_checks,r.memory_bytes,r.k==Some(k),r.k.is_none());
                    assert_eq!(r.k, Some(k), "g={g} p={p} seed={seed}");
                }
            }
        }
    }
}
