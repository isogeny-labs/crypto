// Focused public integration controls. The full legacy lib-test build is
// separately retained; this target does not compile its cfg(test) modules.
use crypto_lib::cryptanalysis::ecbench::prime_mont4::{bsgs, PrimeMont4};
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
pub use crypto_lib::ecc;
use num_bigint::BigUint;

mod prime_mont4 {
    use super::*;
    use crate::ecc::{field::FieldElement, point::Point as ReferencePoint};

    #[test]
    fn workloads_dispatch_and_exact_replays() {
        use crypto_lib::cryptanalysis::ecbench::{
            methods,
            workload::{CurveSpec, TargetKind, Workload},
        };
        use std::collections::BTreeMap;
        let spec = CurveSpec::PrimeMont4Explicit {
            p: "223".into(),
            a: "74".into(),
            b: "122".into(),
            group_order: "205".into(),
            r: 41,
            gx: "119".into(),
            gy: "198".into(),
        };
        let inst = spec.build().unwrap();
        assert_eq!(CurveSpec::explicit(&inst), Some(spec.clone()));
        let facts = inst.facts(&spec);
        for id in ["rho.negation", "bsgs.prime_mont4_v1"] {
            let method = methods::resolve(&methods::MethodSpec {
                id: id.into(),
                params: BTreeMap::new(),
            })
            .unwrap();
            methods::applicable(&method, &inst, &facts).unwrap();
            for index in 0..8 {
                let workload =
                    Workload::on(&spec, &inst, 20261118, index, TargetKind::PlantedShared).unwrap();
                let a = methods::solve(&method, &inst, &facts, &workload.target, 17).unwrap();
                let b = methods::solve(&method, &inst, &facts, &workload.target, 17).unwrap();
                assert_eq!(a.recovered, workload.planted);
                assert_eq!(a.recovered, b.recovered);
                assert_eq!(a.counters, b.counters);
                assert_eq!(a.total_gae, b.total_gae);
                assert!(a.field_ops.is_some());
            }
        }
        assert!(Workload::on(&spec, &inst, 20261118, 0, TargetKind::Public).is_err());
        let word_bsgs = methods::resolve(&methods::MethodSpec {
            id: "bsgs.textbook".into(),
            params: BTreeMap::new(),
        })
        .unwrap();
        assert!(methods::applicable(&word_bsgs, &inst, &facts).is_err());
        assert!(methods::dump_factor_base(&spec, "prime-abscissa:size=8").is_err());
        let vectors: serde_json::Value = serde_json::from_str(include_str!(
            "../research/prime_conductor_gap80_20261031/wide-harness-20261118/source-vectors.json"
        ))
        .unwrap();
        for case in vectors.as_array().unwrap() {
            let text = |name: &str| case[name].as_str().unwrap().to_string();
            let spec = CurveSpec::PrimeMont4Explicit {
                p: text("p"),
                a: text("a"),
                b: text("b"),
                group_order: text("group_order"),
                r: text("r").parse().unwrap(),
                gx: case["generator"][0].as_str().unwrap().into(),
                gy: case["generator"][1].as_str().unwrap().into(),
            };
            let inst = spec.build().unwrap();
            assert_eq!(CurveSpec::explicit(&inst), Some(spec.clone()));
            assert_eq!(
                inst.facts(&spec).group_order.to_biguint().to_string(),
                text("group_order")
            );
            for (index, row) in case["targets"].as_array().unwrap().iter().enumerate() {
                let w = Workload::on(
                    &spec,
                    &inst,
                    20261118,
                    index as u64,
                    TargetKind::PlantedShared,
                )
                .unwrap();
                assert_eq!(
                    w.planted.unwrap().to_biguint().to_string(),
                    row["scalar"].as_str().unwrap()
                );
                for (actual, expected) in w.target.iter().zip(row["point"].as_array().unwrap()) {
                    assert_eq!(
                        BigUint::parse_bytes(&actual.as_bytes()[2..], 16)
                            .unwrap()
                            .to_string(),
                        expected.as_str().unwrap()
                    );
                }
            }
        }
    }

    fn small() -> PrimeMont4 {
        PrimeMont4::build(["223", "74", "122", "205", "119", "198"], 41).unwrap()
    }

    fn reference_mul(curve: &PrimeMont4, mut scalar: u64) -> Option<[String; 2]> {
        let coordinates = curve.encode(curve.generator).unwrap();
        let coordinate = |s: &str| {
            FieldElement::new(
                BigUint::parse_bytes(&s.as_bytes()[2..], 16).unwrap(),
                curve.p.clone(),
            )
        };
        let mut point = ReferencePoint::Affine {
            x: coordinate(&coordinates[0]),
            y: coordinate(&coordinates[1]),
        };
        let a = FieldElement::new(curve.a.clone(), curve.p.clone());
        let mut result = ReferencePoint::Infinity;
        // Explicit BigUint affine formulas, avoiding the reference's optional
        // Montgomery scalar-multiplication fast path.
        while scalar != 0 {
            if scalar & 1 != 0 {
                result = result.add(&point, &a);
            }
            scalar >>= 1;
            if scalar != 0 {
                point = point.double(&a);
            }
        }
        match result {
            ReferencePoint::Infinity => None,
            ReferencePoint::Affine { x, y } => {
                Some([format!("0x{:x}", x.value), format!("0x{:x}", y.value)])
            }
        }
    }

    #[test]
    fn small_group_law_and_exact_bsgs() {
        let curve = small();
        let mut ops = GroupOps::default();
        for scalar in 0..=curve.r {
            let point = curve.mul(&mut ops, curve.generator, scalar);
            assert_eq!(curve.encode(point), reference_mul(&curve, scalar));
            assert_eq!(
                curve.add(&mut ops, point, curve.neg(point)),
                curve.identity()
            );
            assert_eq!(curve.add(&mut ops, point, curve.identity()), point);
            if scalar > 0 && scalar < curve.r {
                let outcome = bsgs(&curve, point);
                assert_eq!(outcome.recovered, Some(scalar));
                assert!(outcome.setup.field_ops.invs > 0);
            }
        }
        assert!(ops.adds > 0 && ops.doubles > 0 && ops.scalar_mults > 0);
    }

    #[test]
    fn frozen_202_bit_source_scalars_match_biguint_affine() {
        let curve = PrimeMont4::build(
            [
                "4794168233664648458988367110514316682522236917173341621535751",
                "4794168233664648458988367110514316682522236917173341621535611",
                "784",
                "4794168233664648458988367110509937564696162391583477974939208",
                "603209182794240622510892080705207481693427956682909708617837",
                "1682738350748939452891832813322046124324576766734042502997852",
            ],
            1125023250121,
        )
        .unwrap();
        assert_eq!(curve.p.bits(), 202);
        let mut ops = GroupOps::default();
        for scalar in [0, 1, 2, 3, 17, 31, 65537, curve.r - 1, curve.r, curve.r + 1] {
            assert_eq!(
                curve.encode(curve.mul(&mut ops, curve.generator, scalar)),
                reference_mul(&curve, scalar)
            );
        }
    }

    #[test]
    fn canonical_and_order_guards() {
        let valid = ["223", "74", "122", "205", "119", "198"];
        for (slot, bad) in [
            (0, "0223"),
            (0, "224"),
            (0, "221"),
            (1, "223"),
            (1, "0"),
            (2, "0"),
            (3, "206"),
            (4, "223"),
            (4, "1"),
            (5, "1"),
        ] {
            let mut input = valid;
            input[slot] = bad;
            assert!(PrimeMont4::build(input, 41).is_err(), "slot {slot}: {bad}");
        }
        assert!(PrimeMont4::build(valid, 40).is_err());
        let curve = small();
        for value in [
            ["0x077", "0xc6"],
            ["0x77", "0xC6"],
            ["119", "198"],
            ["0xdf", "0xc6"],
        ] {
            assert!(curve.decode(&value.map(String::from)).is_err());
        }
        let encoded = curve.encode(curve.generator).unwrap();
        assert_eq!(curve.decode(&encoded).unwrap(), curve.generator);
    }

    #[test]
    fn frozen_source_vectors_match_pari() {
        let cases: serde_json::Value = serde_json::from_str(include_str!(
            "../research/prime_conductor_gap80_20261031/wide-harness-20261118/source-vectors.json"
        ))
        .unwrap();
        for case in cases.as_array().unwrap() {
            let text = |name: &str| case[name].as_str().unwrap();
            let generator = case["generator"].as_array().unwrap();
            let curve = PrimeMont4::build(
                [
                    text("p"),
                    text("a"),
                    text("b"),
                    text("group_order"),
                    generator[0].as_str().unwrap(),
                    generator[1].as_str().unwrap(),
                ],
                text("r").parse().unwrap(),
            )
            .unwrap();
            let mut ops = GroupOps::default();
            for row in case["targets"].as_array().unwrap() {
                let scalar = row["scalar"].as_str().unwrap().parse().unwrap();
                let actual = curve
                    .encode(curve.mul(&mut ops, curve.generator, scalar))
                    .unwrap();
                for (actual, expected) in actual.iter().zip(row["point"].as_array().unwrap()) {
                    let value = BigUint::parse_bytes(&actual.as_bytes()[2..], 16).unwrap();
                    assert_eq!(value.to_string(), expected.as_str().unwrap());
                }
            }
        }
    }
}
