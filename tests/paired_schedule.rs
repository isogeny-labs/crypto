// Public integration controls link the actual library without legacy cfg(test).
pub use crypto_lib::ecc;
#[path = "prime_mont4.rs"]
#[allow(unused_imports)] // Its crate-root reexport is unused only in this nested wrapper.
mod legacy_wide_tests;

mod paired_schedule {
    use crypto_lib::cryptanalysis::ecbench::{methods, spec, workload};
    use serde_json::{json, Value};
    use std::collections::BTreeMap;

    fn tiny() -> Value {
        json!({
            "schema":"ecbench.spec/v1", "label":"fixed paired scheduling controls",
            "workloads":{
                "curves":[
                    {"kind":"prime_mont4_explicit","p":"223","a":"74","b":"122","group_order":"205","r":41,"gx":"119","gy":"198"},
                    {"kind":"prime_mont4_explicit","p":"223","a":"69","b":"3","group_order":"205","r":41,"gx":"30","gy":"23"}
                ],"targets_per_curve":4,"target_seed":20261118,"target_kind":"planted_shared"
            },
            "arms":[
                {"name":"rho","role":"reference","method":{"id":"rho.negation","params":{}}},
                {"name":"bsgs","role":"candidate","method":{"id":"bsgs.prime_mont4_v1","params":{}}},
                {"name":"rho-aa","role":"control","method":{"id":"rho.negation","params":{}}}
            ],
            "measurement":{"rounds":6,"warmup":1,"order":"paired_curves_v1","seed":20261118,"algorithm_seed_round":"measured_index"}
        })
    }
    fn plan(v: &Value) -> spec::Plan {
        spec::plan(spec::Spec::from_json(&v.to_string()).unwrap()).unwrap()
    }
    fn oracle() -> BTreeMap<(u64, u64, bool, u32), u64> {
        let data: Value = serde_json::from_str(include_str!(
            "../research/prime_conductor_gap80_20261031/paired-schedule-20261118/seed-vectors.json"
        ))
        .unwrap();
        data["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                (
                    (
                        r[0].as_u64().unwrap(),
                        r[1].as_u64().unwrap(),
                        r[2].as_bool().unwrap(),
                        r[3].as_u64().unwrap() as u32,
                    ),
                    r[4].as_u64().unwrap(),
                )
            })
            .collect()
    }
    fn check(p: &spec::Plan) {
        let vectors = oracle();
        let mut groups = BTreeMap::<(u32, u64, u64), Vec<&spec::Execution>>::new();
        for (index, e) in p.executions.iter().enumerate() {
            assert_eq!(e.seq, index as u64);
            let w = &p.workloads[e.workload];
            let r = u64::try_from(w.curve.r.checked_u128().unwrap()).unwrap();
            let seed_round = if !e.warmup
                && p.spec.measurement.algorithm_seed_round
                    == spec::AlgorithmSeedRound::MeasuredIndex
            {
                e.round - p.spec.measurement.warmup
            } else {
                e.round
            };
            assert_eq!(
                e.algorithm_seed,
                vectors[&(r, w.target_index, e.warmup, seed_round)]
            );
            groups
                .entry((e.round, w.target_index, r))
                .or_default()
                .push(e);
        }
        for ((round, _, _), events) in groups {
            let phase = if round < p.spec.measurement.warmup {
                round
            } else {
                round - p.spec.measurement.warmup
            };
            let seed = events[0].algorithm_seed;
            let planted = p.workloads[events[0].workload].planted;
            let mut actual_curves = Vec::new();
            let mut actual_arms = Vec::new();
            for e in events {
                assert_eq!(e.algorithm_seed, seed);
                assert_eq!(p.workloads[e.workload].planted, planted);
                actual_curves.push(p.curve_of[e.workload]);
                actual_arms.push(e.arm);
            }
            let mut unique = actual_curves.clone();
            unique.sort_unstable();
            unique.dedup();
            if phase % 2 == 1 {
                unique.reverse();
            }
            let mut arms: Vec<_> = (0..p.arms.len()).collect();
            if phase % 2 == 1 {
                arms.reverse();
            }
            let expected_curves: Vec<_> = unique
                .iter()
                .flat_map(|c| std::iter::repeat_n(*c, arms.len()))
                .collect();
            let expected_arms: Vec<_> = unique.iter().flat_map(|_| arms.iter().copied()).collect();
            assert_eq!(actual_curves, expected_curves);
            assert_eq!(actual_arms, expected_arms);
        }
    }
    #[test]
    fn exact_seed_vectors_and_balanced_endpoint_arm_order() {
        let p = plan(&tiny());
        check(&p);
        assert!(p.warnings.is_empty());
        assert_eq!(p.workloads.len(), 8);
        assert_eq!(p.executions.len(), 168);
        let lead_curves: Vec<_> = p
            .executions
            .iter()
            .filter(|e| !e.warmup && p.workloads[e.workload].target_index == 0 && e.seq % 6 == 0)
            .map(|e| p.curve_of[e.workload])
            .collect();
        assert_eq!(lead_curves.iter().filter(|&&c| c == 0).count(), 3);
        assert_eq!(lead_curves.iter().filter(|&&c| c == 1).count(), 3);
    }
    #[test]
    fn full_two_field_transfers_keep_workloads_and_pair_all_events() {
        let mut v:Value=serde_json::from_str(include_str!("../research/prime_conductor_gap80_20261031/wide-harness-20261118/inputs/wide-paired-plan.json")).unwrap();
        let old = plan(&v);
        v["measurement"]["order"] = json!("paired_curves_v1");
        v["measurement"]["algorithm_seed_round"] = json!("measured_index");
        let new = plan(&v);
        check(&new);
        assert_eq!(new.workloads, old.workloads);
        assert_eq!(new.curve_of, old.curve_of);
        assert_ne!(new.spec_id, old.spec_id);
        assert_eq!(new.workloads.len(), 192);
        assert_eq!(new.executions.len(), old.executions.len());
    }
    #[test]
    fn measured_seeds_ignore_warmup_count_when_explicitly_selected() {
        let a = plan(&tiny());
        let mut v = tiny();
        v["measurement"]["warmup"] = json!(3);
        let b = plan(&v);
        check(&b);
        let measured = |p: &spec::Plan| {
            p.executions
                .iter()
                .filter(|e| !e.warmup)
                .map(|e| (e.workload, e.arm, e.algorithm_seed))
                .collect::<Vec<_>>()
        };
        assert_eq!(measured(&a), measured(&b));
        v["measurement"]["warmup"] = json!(1);
        v["measurement"]["algorithm_seed_round"] = json!("absolute");
        let absolute = plan(&v);
        check(&absolute);
        assert_ne!(measured(&a), measured(&absolute));
    }
    #[test]
    fn invalid_pairing_requests_reject_before_execution() {
        for kind in ["planted", "public"] {
            let mut v = tiny();
            v["workloads"]["target_kind"] = json!(kind);
            let err = spec::plan(spec::Spec::from_json(&v.to_string()).unwrap())
                .err()
                .unwrap();
            assert!(err.contains("planted_shared"));
        }
        let mut odd = tiny();
        odd["measurement"]["rounds"] = json!(5);
        assert!(spec::plan(spec::Spec::from_json(&odd.to_string()).unwrap())
            .err()
            .unwrap()
            .contains("even"));
        let mut overflow = tiny();
        overflow["measurement"]["warmup"] = json!(u32::MAX);
        assert!(
            spec::plan(spec::Spec::from_json(&overflow.to_string()).unwrap())
                .err()
                .unwrap()
                .contains("exceeds u32")
        );
        let mut single = tiny();
        single["workloads"]["curves"].as_array_mut().unwrap().pop();
        assert!(
            spec::plan(spec::Spec::from_json(&single.to_string()).unwrap())
                .err()
                .unwrap()
                .contains("at least two")
        );
        let mut unmatched = tiny();
        unmatched["arms"].as_array_mut().unwrap().truncate(1);
        unmatched["workloads"]["curves"].as_array_mut().unwrap().push(json!({"kind":"prime_explicit","p":43,"a":0,"b":7,"group_order":31,"r":31,"gx":2,"gy":12}));
        assert!(
            spec::plan(spec::Spec::from_json(&unmatched.to_string()).unwrap())
                .err()
                .unwrap()
                .contains("at least two")
        );
    }
    #[test]
    fn legacy_orders_retain_their_workload_seed_law() {
        use crypto_lib::cryptanalysis::ecbench::canonical::derive_u64;
        for order in ["alternate", "alternate_measured", "shuffle"] {
            let mut v = tiny();
            v["measurement"]["order"] = json!(order);
            v["measurement"]
                .as_object_mut()
                .unwrap()
                .remove("algorithm_seed_round");
            let p = plan(&v);
            for e in &p.executions {
                let id =
                    u64::from_str_radix(&p.workloads[e.workload].workload_id[1..], 16).unwrap();
                assert_eq!(
                    e.algorithm_seed,
                    derive_u64(
                        if e.warmup {
                            "ecbench.warmup_seed"
                        } else {
                            "ecbench.algorithm_seed"
                        },
                        &[20261118, id, u64::from(e.round)]
                    )
                );
            }
        }
    }
    #[test]
    fn paired_tiny_solver_recovery_and_deterministic_replay() {
        let mut v = tiny();
        v["measurement"]["rounds"] = json!(2);
        v["workloads"]["targets_per_curve"] = json!(2);
        let p = plan(&v);
        check(&p);
        for e in &p.executions {
            let w = &p.workloads[e.workload];
            let instance = &p.instances[p.curve_of[e.workload]];
            let solve = || {
                methods::solve(
                    &p.arms[e.arm].method,
                    instance,
                    &w.curve,
                    &w.target,
                    e.algorithm_seed,
                )
                .unwrap()
            };
            let a = solve();
            let b = solve();
            assert_eq!(a.recovered, w.planted);
            assert_eq!(a.recovered, b.recovered);
            assert_eq!(a.counters, b.counters);
            assert_eq!(a.total_gae, b.total_gae);
            assert_eq!(
                instance.mul_generator_exact(a.recovered.unwrap()),
                Some(w.target.clone())
            );
        }
        assert_eq!(
            p.spec.workloads.target_kind,
            workload::TargetKind::PlantedShared
        );
    }
}
