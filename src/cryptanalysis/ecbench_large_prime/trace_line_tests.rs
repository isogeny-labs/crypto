use crate::cryptanalysis::ecbench_large_prime::{
    independent_verify, solve_with_options,
    trace_line::{construct, expand, validate_basis, verify_relations},
    LargePrimeGraph, SolveConfig, SolveOptions,
};
use crate::cryptanalysis::ic_boundary::random_binary_instance;

#[test]
fn trace_lines_lift_and_standard_is_the_actual_subfield() {
    for n in [8, 12, 16] {
        let i = random_binary_instance(n, 20261027, 16).unwrap();
        let d = n / 4;
        let standard = construct(&i, d, "standard", (1 << n) - 1).unwrap();
        let selected = construct(&i, d, "trace", (1 << n) - 1).unwrap();
        assert_eq!(selected.trace_a, 0);
        for v in 1..1 << d {
            let x = expand(&standard.basis, v);
            assert_eq!(i.gf.sqr_k(x, d), x);
            let x = expand(&selected.basis, v);
            let points = i.points_with_x(x);
            assert_eq!(points.len(), 2);
            // General arithmetic independently checks both lifts.
            let general = crate::binary_ecc::BinaryCurve {
                m: n,
                irreducible: i.irreducible.clone(),
                a: i.gf.to_element(i.a),
                b: i.gf.to_element(i.b),
                generator: i.fast.lower(i.generator),
                order: i.r.into(),
                cofactor: i.cofactor.into(),
            };
            for p in points {
                assert!(general.is_on_curve(&i.fast.lower(p)));
            }
        }
        assert_eq!(
            serde_json::to_vec(&construct(&i, d, "trace", (1 << n) - 1).unwrap()).unwrap(),
            serde_json::to_vec(&selected).unwrap()
        );
    }
}

#[test]
fn malformed_bases_twists_and_exhaustion_fail_closed() {
    let mut i = random_binary_instance(12, 20261027, 16).unwrap();
    assert!(validate_basis(&i, &[1, 1], 2).is_err());
    assert!(validate_basis(&i, &[0], 1).is_err());
    assert!(validate_basis(&i, &[1 << 12], 1).is_err());
    assert!(construct(&i, u32::MAX, "trace", 10).is_err());
    assert!(construct(&i, 0, "trace", 10).is_err());
    assert!(construct(&i, 3, "invalid", 10).is_err());
    assert!(construct(&i, 3, "trace", 0).is_err());
    // n=12, d=3: 1 is in the trace kernel, but c may not be.
    let found = construct(&i, 3, "trace", 4095).unwrap();
    if found.multiplier > 1 {
        assert!(construct(&i, 3, "trace", found.multiplier - 1).is_err());
    }
    i.a = (1..1 << 12)
        .find(|&x| (0..12).fold(0, |t, k| t ^ i.gf.sqr_k(x, k)) == 1)
        .unwrap();
    assert!(construct(&i, 3, "trace", 4095)
        .unwrap_err()
        .contains("twist"));
}

#[test]
fn explicit_bases_recover_and_every_relation_replays_independently() {
    let i = random_binary_instance(12, 20261027, 16).unwrap();
    let target = i.fast.mul_u64(i.generator, 7);
    for policy in ["standard", "trace"] {
        let line = construct(&i, 3, policy, 4095).unwrap();
        let report = solve_with_options(
            &i,
            target,
            SolveConfig {
                small_dimension: 2,
                envelope_dimension: 3,
                summands: 4,
                max_large_primes: 2,
                max_trials: 5000,
                max_states: 100000,
                seed: 23,
                large_prime_graph: LargePrimeGraph::Online,
            },
            SolveOptions {
                abscissa_basis: Some(line.basis),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(report.recovered, Some(7));
        independent_verify(&i, target, report.recovered.unwrap()).unwrap();
        let evidence = report.trace_evidence.as_ref().unwrap();
        verify_relations(
            &i,
            target,
            2,
            evidence,
            &report.relation_samples,
            report.counters.accepted_relations,
        )
        .unwrap();
        assert!(verify_relations(
            &i,
            target,
            2,
            evidence,
            &report.relation_samples,
            report.counters.accepted_relations + 1
        )
        .is_err());
        let mut damaged = report.relation_samples.clone();
        damaged[0].a = (damaged[0].a + 1) % i.r;
        assert!(verify_relations(
            &i,
            target,
            2,
            evidence,
            &damaged,
            report.counters.accepted_relations
        )
        .is_err());
        let mut damaged = report.relation_samples.clone();
        damaged[0].columns[0].1 = (damaged[0].columns[0].1 + 1) % i.r;
        assert!(verify_relations(
            &i,
            target,
            2,
            evidence,
            &damaged,
            report.counters.accepted_relations
        )
        .is_err());
    }
}

#[test]
fn ecbench_trace_line_adapter_retains_transcripts_and_unpriced_work() {
    use crate::cryptanalysis::ecbench::{
        methods::{resolve, solve, MethodSpec},
        workload::{CurveSpec, TargetKind, Workload},
    };
    let spec = CurveSpec::BinaryRandom {
        n: 12,
        seed: 20261027,
        max_cofactor: 16,
    };
    let (workload, instance) = Workload::build(&spec, 20261027, 0, TargetKind::Planted).unwrap();
    for line in ["standard", "trace"] {
        let method = resolve(&MethodSpec {
            id: "ic.trace_line_2lp".into(),
            params: std::collections::BTreeMap::from([
                ("line".into(), line.into()),
                ("d".into(), "3".into()),
                ("small_dimension".into(), "2".into()),
                ("max_u".into(), "4095".into()),
                ("max_trials".into(), "5000".into()),
                ("max_states".into(), "100000".into()),
            ]),
        })
        .unwrap();
        let out = solve(&method, &instance, &workload.curve, &workload.target, 23).unwrap();
        assert_eq!(out.recovered, workload.planted);
        assert!(out
            .unpriced
            .iter()
            .any(|n| n == "line_field_multiplications_uncharged"));
        assert_eq!(
            out.detail["relation_samples"].as_array().unwrap().len() as u64,
            out.counters["accepted_relations"]
        );
        assert!(out.detail["trace_evidence"].is_object());
    }
}
