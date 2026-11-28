//! Fixed arithmetic and equation-generation contracts for the alternate H2
//! factorization. These tests perform no query search, census or benchmark.
use prime31_aux_matched::bases::{
    independent_add, independent_on_curve, independent_scalar, prime31_fixture, CurveDefinition,
    PointDefinition,
};
use prime31_aux_matched::kernel_fibers::{
    build_fiber, build_fiber_with_factorization, KernelChoice, KernelFactorization, KernelFiber,
    KernelStep, P,
};
use prime31_aux_matched::oracle::{self, OracleLimits, Presentation};
use prime31_aux_matched::GroupOps;
use serde_json::Value;
use std::collections::BTreeMap;

fn sparse() -> KernelFiber {
    build_fiber_with_factorization(
        KernelChoice::AminusH2,
        1,
        KernelFactorization::FullTwoTorsionFirst,
        &mut GroupOps::default(),
    )
    .unwrap()
}
fn nums(value: &Value) -> Vec<u64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap())
        .collect()
}
fn point(x: u64, y: u64) -> PointDefinition {
    PointDefinition {
        x,
        y,
        infinity: false,
    }
}
fn identity() -> PointDefinition {
    PointDefinition {
        x: 0,
        y: 0,
        infinity: true,
    }
}
fn product(a: u64, b: u64) -> u64 {
    a * b % P
}
fn difference(a: u64, b: u64) -> u64 {
    (a + P - b) % P
}
fn power(mut a: u64, mut n: u64) -> u64 {
    let mut out = 1;
    while n != 0 {
        if n & 1 != 0 {
            out = product(out, a);
        }
        a = product(a, a);
        n >>= 1;
    }
    out
}
fn chain(steps: &[KernelStep], mut q: PointDefinition) -> PointDefinition {
    for step in steps {
        assert!(independent_on_curve(&step.domain, q));
        if q.infinity || q.x == step.alpha {
            q = identity();
        } else {
            let inverse = power(difference(q.x, step.alpha), P - 2);
            q = point(
                (q.x + product(step.tau, inverse)) % P,
                product(
                    q.y,
                    difference(1, product(step.tau, product(inverse, inverse))),
                ),
            );
        }
        assert!(independent_on_curve(&step.codomain, q));
    }
    q
}

/// Direct degree-four map and derivative, independent of the two-step formula.
fn direct_delta(q: PointDefinition) -> PointDefinition {
    if q.infinity || q.y == 0 {
        return identity();
    }
    let xx = product(q.x, q.x);
    let plus = (xx + 1) % P;
    let denominator = product(q.x, difference(xx, 1));
    assert_ne!(denominator, 0);
    let inverse = power(denominator, P - 2);
    let xx2_minus_six_xx_plus_one = (difference(product(xx, xx), product(6, xx)) + 1) % P;
    let derivative = product(
        product(plus, xx2_minus_six_xx_plus_one),
        product(inverse, inverse),
    );
    point(
        product(product(plus, plus), inverse),
        product(q.y, derivative),
    )
}

#[test]
fn alternate_h2_has_exactly_the_legacy_kernel_fiber_and_normalized_quotient() {
    let standard = build_fiber(KernelChoice::AminusH2, 1, &mut GroupOps::default()).unwrap();
    let alternate = sparse();
    assert_eq!(standard.factorization, KernelFactorization::Standard);
    assert_eq!(
        alternate.factorization,
        KernelFactorization::FullTwoTorsionFirst
    );
    assert_eq!(alternate.auxiliary_curve, standard.auxiliary_curve);
    assert_eq!(alternate.source_generator, standard.source_generator);
    assert_eq!(alternate.coset_point, standard.coset_point);
    assert_eq!(alternate.kernel_generators, standard.kernel_generators);
    assert_eq!(alternate.kernel_orders, [512, 2]);
    assert_eq!(alternate.kernel_points, standard.kernel_points);
    assert_eq!(alternate.v, standard.v);
    assert_eq!(alternate.endpoint, standard.endpoint);
    assert_eq!(alternate.endpoint, point(1_135_030_993, 876_134_324));
    assert_eq!(
        alternate.steps.last().unwrap().codomain,
        standard.steps.last().unwrap().codomain
    );
    assert_eq!(alternate.certificate, standard.certificate);
    assert_ne!(alternate.steps, standard.steps);
    assert_ne!(alternate.tower, standard.tower);

    // Frozen evidence from the earlier independent std-only verifier is read,
    // never rewritten or regenerated with the new implementation.
    let frozen: Value = serde_json::from_str(include_str!(
        "../../isogeny-choice/fixtures/kernel_correctness_fixtures.json"
    ))
    .unwrap();
    let h2 = frozen["arms"]
        .as_array()
        .unwrap()
        .iter()
        .find(|arm| arm["name"] == "Aminus_H2")
        .unwrap();
    assert_eq!(alternate.v, nums(&h2["cosets"][0]["V"]));
    assert_eq!(
        alternate.certificate.numerator,
        nums(&h2["numerator"]["coefficients_ascending"])
    );
    assert_eq!(
        alternate.certificate.denominator,
        nums(&h2["denominator"]["coefficients_ascending"])
    );

    let legacy_shape = serde_json::to_value(&standard).unwrap();
    assert!(legacy_shape.get("factorization").is_none());
    let legacy_roundtrip: KernelFiber = serde_json::from_value(legacy_shape.clone()).unwrap();
    assert_eq!(
        legacy_roundtrip.factorization,
        KernelFactorization::Standard
    );
    assert_eq!(
        serde_json::to_value(legacy_roundtrip).unwrap(),
        legacy_shape
    );
    let alternate_json = serde_json::to_value(&alternate).unwrap();
    assert_eq!(alternate_json["factorization"], "full_two_torsion_first");
    let alternate_roundtrip: KernelFiber = serde_json::from_value(alternate_json).unwrap();
    assert_eq!(alternate_roundtrip.factorization, alternate.factorization);
    assert_eq!(alternate_roundtrip.steps, alternate.steps);
}

#[test]
fn first_two_steps_are_normalized_doubling_with_the_correct_residual_kernel() {
    let fiber = sparse();
    let prefix = &fiber.steps[..2];
    assert_eq!((prefix[0].alpha, prefix[0].tau), (0, P - 1));
    assert_eq!((prefix[1].alpha, prefix[1].tau), (0, 4));
    assert_eq!(prefix[0].codomain, CurveDefinition { p: P, a: 4, b: 0 });
    assert_eq!(
        prefix[1].codomain,
        CurveDefinition {
            p: P,
            a: P - 16,
            b: 0
        }
    );
    let e = &fiber.auxiliary_curve;
    let mut probes = vec![identity(), point(0, 0), point(1, 0), point(P - 1, 0)];
    for scalar in [1, 2, 3, 17] {
        probes.push(independent_scalar(e, fiber.source_generator, scalar));
    }
    probes.push(fiber.kernel_generators[0]);
    for q in probes {
        let twice = independent_add(e, q, q);
        let normalized = if twice.infinity {
            identity()
        } else {
            point(product(4, twice.x), product(8, twice.y))
        };
        assert_eq!(chain(prefix, q), normalized);
        assert_eq!(direct_delta(q), normalized);
    }

    let residual = chain(prefix, fiber.kernel_generators[0]);
    let residual_curve = &prefix[1].codomain;
    assert!(!residual.infinity);
    assert!(independent_scalar(residual_curve, residual, 256).infinity);
    assert!(!independent_scalar(residual_curve, residual, 128).infinity);
    assert_eq!(fiber.steps.len() - prefix.len(), 8);
    assert!(chain(&fiber.steps[2..], residual).infinity);
    // The sparse route has a different order-512 prefix, so dropping its route
    // label must not pass an auditor's standard-prefix check.
    assert!(!chain(&fiber.steps[..9], fiber.kernel_generators[0]).infinity);
}

#[test]
fn existing_oracle_emits_the_sparse_five_term_block_and_unchanged_layouts() {
    let fiber = sparse();
    let (target, _, g) = prime31_fixture();
    let limits = OracleLimits {
        max_degree: 1024,
        max_variables: 64,
        ..Default::default()
    };
    for (presentation, expected_n) in [
        (Presentation::Tower, 31),
        (Presentation::Tower4, 16),
        (Presentation::Dense, 4),
    ] {
        let (equations, n) = oracle::equations_for_chart(
            &target,
            &fiber.v,
            Some(&fiber.tower),
            g,
            3,
            presentation,
            0,
            &limits,
        )
        .unwrap()
        .unwrap();
        assert_eq!(n, expected_n);
        let (_, identity_n) = oracle::equations_for_chart(
            &target,
            &fiber.v,
            Some(&fiber.tower),
            g,
            3,
            presentation,
            1,
            &limits,
        )
        .unwrap()
        .unwrap();
        assert_eq!(identity_n, expected_n - 1);
        if presentation == Presentation::Tower4 {
            let mut expected = BTreeMap::new();
            for (x_power, z_power, coefficient) in
                [(4, 0, 1), (3, 1, P - 1), (2, 0, 2), (1, 1, 1), (0, 0, 1)]
            {
                let mut exponents = vec![0; n];
                exponents[0] = x_power;
                exponents[3] = z_power;
                expected.insert(exponents, coefficient);
            }
            assert_eq!(equations[0].len(), 5);
            assert_eq!(
                equations[0].iter().cloned().collect::<BTreeMap<_, _>>(),
                expected
            );
            // At each original pole, the coefficient of every positive power
            // of z is zero and the constant part is nonzero. Thus no finite z
            // can turn a denominator pole into a spurious membership root.
            for (x, expected_constant) in [(0, 1), (1, 4), (P - 1, 4)] {
                let mut by_z = BTreeMap::<u32, u64>::new();
                for (exponents, coefficient) in &equations[0] {
                    assert!(exponents
                        .iter()
                        .enumerate()
                        .all(|(i, &e)| i == 0 || i == 3 || e == 0));
                    let value = product(*coefficient, power(x, u64::from(exponents[0])));
                    let entry = by_z.entry(exponents[3]).or_default();
                    *entry = (*entry + value) % P;
                }
                assert_eq!(by_z.remove(&0), Some(expected_constant));
                assert!(by_z.values().all(|&coefficient| coefficient == 0));
            }
        }
    }
}

#[test]
fn unsupported_factorization_is_rejected_before_any_group_work() {
    for choice in [
        KernelChoice::AplusCyclic,
        KernelChoice::AminusH0,
        KernelChoice::AminusH1,
    ] {
        let mut ops = GroupOps {
            adds: 7,
            ..Default::default()
        };
        let before = ops;
        let error = build_fiber_with_factorization(
            choice,
            1,
            KernelFactorization::FullTwoTorsionFirst,
            &mut ops,
        )
        .unwrap_err();
        assert!(error.contains("requires the AminusH2 kernel"));
        assert_eq!(ops, before);
    }
}
