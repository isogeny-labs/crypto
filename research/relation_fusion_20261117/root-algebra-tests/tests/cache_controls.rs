//! Correctness-only check of actual crypto_lib cache/control entrypoints.
//! Existing fixed F_2^13 descent fixtures from solvers' source regression.
use crypto_lib::cryptanalysis::{
    ic_boundary::koblitz_instance,
    ic_framework::{
        solvers::{F4F2FullWidth, F4F2},
        stages::{BooleanSystem, Params, SolverVerdict, SystemSolver},
    },
    pq_descent_symbolic::descend,
    pq_groebner_f2::{mask_from64, Mask},
};
use rand::{Rng, SeedableRng};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
fn solutions(verdict: SolverVerdict) -> BTreeSet<Mask> {
    match verdict {
        SolverVerdict::Solved(points) => {
            let count = points.len();
            let set: BTreeSet<_> = points.into_iter().collect();
            assert_eq!(set.len(), count, "duplicate model");
            set
        }
        SolverVerdict::Unsatisfiable => BTreeSet::new(),
        SolverVerdict::BudgetExceeded => panic!("fixed correctness fixture censored"),
    }
}
fn ground_truth(system: &BooleanSystem) -> BTreeSet<Mask> {
    assert_eq!(system.n_vars, 12);
    (0..1u64 << system.n_vars)
        .map(mask_from64)
        .filter(|&point| {
            // Independent Boolean evaluation from the original input monomials.
            system.equations.iter().all(|poly| {
                poly.terms
                    .iter()
                    .fold(false, |v, term| v ^ ((point & term.mask) == term.mask))
                    == false
            })
        })
        .collect()
}
#[test]
fn actual_root_plain_skeleton_trace_and_full_width_match_all_models() {
    let inst = koblitz_instance(0, 13).expect("existing source fixture");
    let basis: Vec<_> = (0..6).map(|k| 1u64 << k).collect();
    let output = std::env::var_os("ROOT_CACHE_QUALIFICATION_OUT").map(std::path::PathBuf::from);
    if let Some(path) = &output {
        std::fs::create_dir(path).unwrap();
    }
    let mut rows = Vec::new();
    for mode in ["plain", "skeleton", "trace"] {
        let primary = F4F2::default();
        let control = F4F2FullWidth::default();
        let mut params = Params::default();
        if mode != "plain" {
            params.set(mode, "1");
        }
        // Reuse exactly the existing correctness regression's seed; never select targets.
        let mut rng = rand::rngs::StdRng::seed_from_u64(0x5ce1_17);
        let mut sums: [BTreeMap<String, u64>; 2] = Default::default();
        let mut previous_x = None;
        for k in 0..if mode == "trace" { 17 } else { 16 } {
            // A declared positive coverage control: repeat the final original input.
            // No additional random draw, instance seed or performance observation.
            let x_r = if k == 16 {
                previous_x.unwrap()
            } else {
                rng.gen::<u64>() & ((1u64 << 13) - 1)
            };
            previous_x = Some(x_r);
            let descended = descend(&inst.gf, inst.b, x_r, &basis, 2).unwrap();
            let system = BooleanSystem {
                equations: descended.equations,
                n_vars: descended.n_vars,
            };
            let expected = ground_truth(&system);
            let input: Vec<Vec<_>> = system
                .equations
                .iter()
                .map(|p| p.terms.iter().map(|t| format!("{:x}", t.mask)).collect())
                .collect();
            let arms: [(&str, &dyn SystemSolver); 2] =
                [("primary", &primary), ("full-width", &control)];
            for (i, (arm, solver)) in arms.into_iter().enumerate() {
                let (verdict, cost) = solver.solve(&system, &params, None);
                let raw_verdict = format!("{verdict:?}");
                let got: BTreeSet<Mask> = match &verdict {
                    SolverVerdict::Solved(points) => points.iter().copied().collect(),
                    _ => BTreeSet::new(),
                };
                rows.push(json!({"mode":mode,"case":k,"arm":arm,"x_r":x_r,
                    "calibration_repeat_of":if k == 16 { Some(15) } else { None },
                    "n_vars":system.n_vars,"input_term_masks_hex":input,"raw_verdict":raw_verdict,
                    "ground_truth_models_hex":expected.iter().map(|x|format!("{x:x}")).collect::<Vec<_>>(),
                    "all_models_hex":got.iter().map(|x|format!("{x:x}")).collect::<Vec<_>>(),
                    "all_4096_assignments_checked":true,"diagnostic_degree_reached":cost.degree_reached,
                    "diagnostic_solving_degree":cost.solving_degree,"diagnostic_peak_bytes":cost.peak_bytes,
                    "timed_out":cost.timed_out,"diagnostic_extra":cost.extra}));
                if let Some(path) = &output {
                    let bytes = serde_json::to_vec_pretty(rows.last().unwrap()).unwrap();
                    use std::io::Write;
                    let mut file = std::fs::OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(path.join(format!("{mode}-{k}-{arm}.json")))
                        .unwrap();
                    file.write_all(&bytes).unwrap();
                }
                let got = solutions(verdict);
                assert_eq!(got, expected, "mode={mode} case={k} arm={arm}");
                if i == 1 {
                    assert_eq!(cost.extra["fl_forced_full_width"], 1);
                    assert_eq!(
                        cost.extra["fl_matrices"], 0,
                        "cache control must actually disable FL"
                    );
                }
                for key in [
                    "skeleton_steps_replayed",
                    "trace_records",
                    "trace_replays",
                    "trace_verifications",
                ] {
                    *sums[i].entry(key.into()).or_default() += cost.extra[key];
                }
            }
        }
        for s in sums {
            match mode {
                "skeleton" => assert!(s["skeleton_steps_replayed"] > 0, "real cache must be used"),
                "trace" => {
                    assert!(s["trace_records"] > 0);
                    assert!(s["trace_replays"] > 0);
                    assert!(s["trace_verifications"] > 0);
                }
                "plain" => {
                    assert_eq!(s["skeleton_steps_replayed"], 0);
                    assert_eq!(s["trace_replays"], 0);
                }
                _ => unreachable!(),
            }
        }
    }
    assert_eq!(rows.len(), 98);
    if let Some(path) = &output {
        let report = json!({"schema":"actual-root-f4-cache-control-correctness/v1",
            "fixture_origin":"existing solvers.rs f4_f2_*_matches_plain_on_descent_systems; Koblitz(0,13), original seed0x5ce1_17",
            "calls":98,"original_calls":96,"same_input_trace_controls":2,"rows":rows,"comparative_timing":false,"15x_admitted":false,
            "signed_execution_verdict":false});
        std::fs::write(
            path.join("outcome.json"),
            serde_json::to_vec_pretty(&report).unwrap(),
        )
        .unwrap();
    }
}
