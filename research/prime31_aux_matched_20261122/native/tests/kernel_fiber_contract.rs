//! Focused fixed-coset arithmetic contracts; no census or performance run.
use prime31_aux_matched::bases::GroupWork;
use prime31_aux_matched::kernel_fibers::{build_fiber, KernelChoice, KernelFiber, DEGREE, P};
use prime31_aux_matched::{GroupOps, PrimeCurve};
use serde_json::Value;
use std::collections::BTreeSet;

fn nums(value: &Value) -> Vec<u64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_u64().unwrap())
        .collect()
}

#[test]
fn all_four_fixed_kernel_fibers_match_the_independent_exact_fixture() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../isogeny-choice/fixtures/kernel_correctness_fixtures.json"
    ))
    .unwrap();
    let targets = [
        PrimeCurve {
            p: P,
            a: 847_514_846,
            b: 2_087_444_724,
        },
        PrimeCurve {
            p: P,
            a: 323_891_735,
            b: 1_985_248_359,
        },
    ];
    let mut minus = Vec::new();
    for (choice, name, expected) in [
        (KernelChoice::AplusCyclic, "Aplus_cyclic", [518, 506]),
        (KernelChoice::AminusH0, "Aminus_H0", [508, 508]),
        (KernelChoice::AminusH1, "Aminus_H1", [512, 519]),
        (KernelChoice::AminusH2, "Aminus_H2", [515, 533]),
    ] {
        let mut ops = GroupOps::default();
        let fiber = build_fiber(choice, 1, &mut ops).unwrap();
        let reference = fixture["arms"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["name"] == name)
            .unwrap();
        let coset = &reference["cosets"][0];
        assert_eq!(fiber.v.len(), DEGREE);
        assert_eq!(fiber.v, nums(&coset["V"]));
        assert_eq!(fiber.endpoint.x, coset["T"][0].as_u64().unwrap());
        assert_eq!(fiber.endpoint.y, coset["T"][1].as_u64().unwrap());
        assert_eq!(
            fiber.certificate.numerator,
            nums(&reference["numerator"]["coefficients_ascending"])
        );
        assert_eq!(
            fiber.certificate.denominator,
            nums(&reference["denominator"]["coefficients_ascending"])
        );
        assert_eq!(fiber.tower.depth(), 10);
        assert_eq!(fiber.steps.len(), 10);
        assert_eq!(fiber.kernel_points.len(), DEGREE);
        for (step, expected_step) in fiber
            .steps
            .iter()
            .zip(reference["maps"].as_array().unwrap())
        {
            assert_eq!(step.alpha, expected_step["alpha"].as_u64().unwrap());
            assert_eq!(step.tau, expected_step["tau"].as_u64().unwrap());
            assert_eq!(
                step.codomain.a,
                expected_step["codomain"][0].as_u64().unwrap()
            );
            assert_eq!(
                step.codomain.b,
                expected_step["codomain"][1].as_u64().unwrap()
            );
        }
        for (e, count) in targets.iter().zip(expected) {
            let accepted = fiber
                .v
                .iter()
                .filter(|&&x| e.legendre(e.rhs(x)) == 1)
                .count();
            assert_eq!(accepted, count);
            assert!(fiber.v.iter().all(|&x| e.rhs(x) != 0));
        }
        assert_eq!(fiber.group_work, GroupWork::from_ops(&ops));
        assert!(fiber.group_work.adds >= 2048);
        assert!(fiber.group_work.scalar_mults > 0);
        assert!(fiber.group_work.field_muls > 0);
        assert!(fiber.map_work.muls > 0);
        assert!(fiber.map_work.sqrs > 0);
        assert!(fiber.map_work.inversion_calls > 0);
        assert!(fiber.map_work.point_map_calls >= 20_480);
        assert!(fiber.certificate.native_operation_counts_partial);
        assert!(fiber.certificate.denominator_nonzero_on_v);
        assert!(fiber.certificate.fiber_polynomial_equals_root_product);
        assert!(fiber.certificate.full_kernel_velu_codomain_matches);
        let two_torsion = fiber
            .kernel_points
            .iter()
            .filter(|q| q.infinity || q.y == 0)
            .count();
        if choice == KernelChoice::AminusH2 {
            assert_eq!(fiber.kernel_orders, [512, 2]);
            assert_eq!(fiber.kernel_generators.len(), 2);
            assert_eq!(two_torsion, 4, "H2 contains full rational two-torsion");
            assert_eq!(fiber.certificate.kernel_x_poles_checked, 513);
        } else {
            assert_eq!(fiber.kernel_orders, [1024]);
            assert_eq!(fiber.kernel_generators.len(), 1);
            assert_eq!(
                two_torsion, 2,
                "cyclic kernels have one nonzero order-two point"
            );
            assert_eq!(fiber.certificate.kernel_x_poles_checked, 512);
        }
        let encoded = serde_json::to_vec(&fiber).unwrap();
        let decoded: KernelFiber = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded.v, fiber.v);
        assert_eq!(decoded.tower, fiber.tower);
        assert_eq!(decoded.map_work, fiber.map_work);
        assert_eq!(decoded.group_work, fiber.group_work);
        if choice != KernelChoice::AplusCyclic {
            minus.push(fiber);
        }
    }
    assert_eq!(minus[0].steps[..9], minus[1].steps[..9]);
    assert_eq!(minus[0].steps[..9], minus[2].steps[..9]);
    let final_kernels = minus
        .iter()
        .map(|f| f.steps[9].alpha)
        .collect::<BTreeSet<_>>();
    assert_eq!(final_kernels.len(), 3);
    let kernels = minus
        .iter()
        .map(|f| {
            f.kernel_points
                .iter()
                .map(|q| (q.infinity, q.x, q.y))
                .collect::<BTreeSet<_>>()
        })
        .collect::<Vec<_>>();
    for i in 0..3 {
        for j in i + 1..3 {
            assert_eq!(kernels[i].intersection(&kernels[j]).count(), 512);
            let vi = minus[i].v.iter().copied().collect::<BTreeSet<_>>();
            let vj = minus[j].v.iter().copied().collect::<BTreeSet<_>>();
            assert_eq!(vi.intersection(&vj).count(), 512);
        }
    }
}

#[test]
fn invalid_cosets_are_rejected_before_construction_and_no_work_is_hidden() {
    for choice in [
        KernelChoice::AplusCyclic,
        KernelChoice::AminusH0,
        KernelChoice::AminusH1,
        KernelChoice::AminusH2,
    ] {
        for scalar in [0, 2, 1024, 1 << 20, (1 << 20) + 1, u64::MAX] {
            let mut ops = GroupOps {
                adds: 7,
                ..Default::default()
            };
            let before = ops;
            let error = build_fiber(choice, scalar, &mut ops).unwrap_err();
            assert!(error.contains("nonzero, odd and less than 2^20"));
            assert_eq!(ops, before);
        }
    }
}
