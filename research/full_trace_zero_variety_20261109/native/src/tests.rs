use full_trace_zero_backend::binary_ecc::{
    curve::{point_add, point_double, point_neg, scalar_mul},
    BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly, TraceZeroAffineMode as Mode,
    TraceZeroCoordinates as Coordinates, TraceZeroError, TraceZeroVariety,
};
use full_trace_zero_backend::cryptanalysis::{
    binary_semaev_chain_sat::{chain_s3_max_variables, ChainedS3Encoding},
    sat::{SolveResult, Solver},
    trace_zero_relations::{
        decompose_full_trace_zero, TraceZeroRelationDisposition as Disposition,
        TraceZeroRelationLimits,
    },
};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

#[test]
fn pinned_telescoping_certificate_replays_and_rejects_a_changed_statement() {
    use full_trace_zero_backend::cryptanalysis::identity_certificate::{
        check_with_extra, IdentityCertificate, Verdict,
    };
    let mut certificate: IdentityCertificate =
        serde_json::from_str(include_str!("../../trace-telescoping.identity.json")).unwrap();
    assert!(matches!(
        check_with_extra(&certificate, Some((0x7265706c6179, 32))).unwrap(),
        Verdict::Accept { .. }
    ));
    certificate.statement.degree_bound = 130;
    assert!(check_with_extra(&certificate, None).is_err());
}

fn fixture(n: u32) -> BinaryCurve {
    if n == 131 {
        return BinaryCurve::certicom_challenge("ECC2K-130").unwrap().0;
    }
    let terms = match n {
        3 => vec![0, 1],
        5 => vec![0, 2],
        7 => vec![0, 1],
        9 => vec![0, 4],
        83 => vec![0, 1, 2, 45],
        _ => panic!("fixture"),
    };
    let (mut a, mut b) = (BigInt::from(2), BigInt::from(-1));
    for _ in 1..n {
        let c = -&b - (&a << 1);
        a = b;
        b = c;
    }
    let order = (((BigInt::one() << n) + BigInt::one() - b)
        .to_biguint()
        .unwrap())
        >> 2;
    let generator = if n == 83 {
        BinaryPoint::Affine {
            x: F2mElement::from_hex("477f77103dfad59850800", n),
            y: F2mElement::from_hex("2fa5e737d542c4e4fd5c3", n),
        }
    } else {
        BinaryPoint::Infinity
    };
    BinaryCurve {
        m: n,
        irreducible: IrreduciblePoly {
            degree: n,
            low_terms: terms,
        },
        a: F2mElement::zero(n),
        b: F2mElement::one(n),
        generator,
        order,
        cofactor: BigUint::from(4u32),
    }
}
fn all_points(c: &BinaryCurve) -> Vec<BinaryPoint> {
    let mut out = vec![BinaryPoint::Infinity];
    for x in 0..1u32 << c.m {
        for y in 0..1u32 << c.m {
            let p = BinaryPoint::Affine {
                x: F2mElement::from_biguint(&BigUint::from(x), c.m),
                y: F2mElement::from_biguint(&BigUint::from(y), c.m),
            };
            if c.is_on_curve(&p) {
                out.push(p)
            }
        }
    }
    out
}
#[test]
fn factor_base_intersection_preserves_points_and_eliminates_only_the_trace_bit() {
    let c = fixture(5);
    let v = TraceZeroVariety::new(&c).unwrap();
    let basis: Vec<_> = (0..3)
        .map(|i| F2mElement::from_bit_positions(&[i], 5))
        .collect();
    let parent: Vec<_> = all_points(&c)
        .into_iter()
        .filter(|p| matches!(p, BinaryPoint::Affine{x,..} if x.to_biguint() < BigUint::from(8u32)))
        .collect();
    let base = v.intersect_factor_base(&parent, &basis).unwrap();
    assert_eq!(
        base.points,
        parent
            .iter()
            .filter(|p| v.contains(p))
            .cloned()
            .collect::<Vec<_>>()
    );
    assert_eq!(base.subspace_basis.len(), 2);
    assert!(base
        .subspace_basis
        .iter()
        .all(|x| x.trace(&c.irreducible) == 0));
    assert!(!base.points.is_empty());
    let target = point_add(&c, &base.points[0], &base.points[0]);
    let result = decompose_full_trace_zero(
        &v,
        &base.points,
        &target,
        2,
        TraceZeroRelationLimits::default(),
    );
    assert_eq!(result.disposition, Disposition::Found);
    assert!(v
        .intersect_factor_base(&parent, &[basis[0].clone(), basis[0].clone()])
        .is_err());
    assert!(v.intersect_factor_base(&parent, &[]).is_err());
    assert!(v
        .intersect_factor_base(&[BinaryPoint::Infinity], &basis)
        .is_err());
    // Full-kernel cardinality is independent of a caller's selected odd subgroup.
    let mut proper = c.clone();
    proper.order = BigUint::one();
    proper.cofactor = &c.order * &c.cofactor;
    assert_eq!(
        TraceZeroVariety::new(&proper)
            .unwrap()
            .rational_point_order(),
        &c.order
    );
}
fn bits(value: usize, n: u32) -> Vec<bool> {
    (0..n).map(|j| (value >> j) & 1 != 0).collect()
}
fn raw_bits(p: &BinaryPoint, n: u32) -> Vec<bool> {
    let BinaryPoint::Affine { x, y } = p else {
        panic!("affine")
    };
    let xb = x.to_biguint();
    let yb = y.to_biguint();
    (0..n)
        .map(|i| xb.bit(i as u64))
        .chain((0..n).map(|i| yb.bit(i as u64)))
        .collect()
}
#[test]
fn full_membership_projection_and_coordinate_codec_match_point_trace_exhaustively() {
    for n in [3, 5, 7, 9] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let points = all_points(&c);
        let mut accepted = 0usize;
        for p in &points {
            let truth = v.point_trace(p).unwrap() == BinaryPoint::Infinity;
            assert_eq!(v.contains(p), truth);
            assert_eq!(
                v.contains(p),
                scalar_mul(&c, p, &c.order) == BinaryPoint::Infinity
            );
            let projected = v.project(p).unwrap();
            assert!(v.contains(&projected));
            assert_eq!(v.project(&projected).unwrap(), projected);
            assert!(v.contains(&v.cofactor_image(p).unwrap()));
            if truth {
                accepted += 1;
                assert_eq!(v.from_coordinates(&v.coordinates(p).unwrap()).unwrap(), *p);
                assert_eq!(projected, *p);
            } else {
                assert!(matches!(
                    v.coordinates(p),
                    Err(TraceZeroError::NotTraceZero)
                ));
            }
        }
        assert_eq!(BigUint::from(accepted), c.order);
        println!(
            "FULL_KERNEL n={n} points={} kernel={accepted} dimension={}",
            points.len(),
            v.dimension()
        );
    }
}
#[test]
fn all_three_boolean_models_are_exact_on_all_assignments() {
    for n in [3, 5] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        for mode in [Mode::Raw, Mode::Compact, Mode::Factored] {
            let model = v.affine_system(mode, 2_000_000).unwrap();
            let mut accepted = Vec::new();
            for a in 0..1usize << model.variables {
                let assignment = bits(a, model.variables);
                if model.accepts(&assignment) {
                    let p = model.decode(&assignment).unwrap();
                    assert!(v.contains(&p));
                    assert_eq!(model.assignment(&p).unwrap(), assignment);
                    accepted.push(p);
                }
            }
            let expected: Vec<_> = all_points(&c)
                .into_iter()
                .filter(|p| *p != BinaryPoint::Infinity && v.contains(p))
                .collect();
            assert_eq!(accepted.len(), expected.len());
            for p in expected {
                let witness = model.assignment(&p).unwrap();
                assert!(model.accepts(&witness));
                assert!(accepted.contains(&p));
            }
            assert!(model.maximum_degree() <= if mode == Mode::Compact { 3 } else { 2 });
            println!("BOOLEAN n={n} mode={mode:?} variables={} equations={} terms={} degree={} affine_points={}",
                model.variables,model.equations.len(),model.monomials(),model.maximum_degree(),accepted.len());
        }
    }
}
#[test]
fn actual_degree_83_and_131_equations_replay_public_points_and_torsion_cosets() {
    for n in [83, 131] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let target = if n == 131 {
            BinaryCurve::certicom_challenge("ECC2K-130").unwrap().1
        } else {
            point_double(&c, &c.generator)
        };
        let torsion = BinaryPoint::Affine {
            x: F2mElement::zero(n),
            y: F2mElement::one(n),
        };
        let order4 = BinaryPoint::Affine {
            x: F2mElement::one(n),
            y: F2mElement::zero(n),
        };
        assert_eq!(v.point_trace(&c.generator).unwrap(), BinaryPoint::Infinity);
        assert_eq!(v.point_trace(&target).unwrap(), BinaryPoint::Infinity);
        assert!(!v.contains(&torsion));
        assert!(!v.contains(&order4));
        for mode in [Mode::Raw, Mode::Compact, Mode::Factored] {
            let model = v.affine_system(mode, 2_000_000).unwrap();
            assert_eq!(
                model.variables,
                match mode {
                    Mode::Raw => 2 * n,
                    Mode::Compact => 2 * n - 2,
                    Mode::Factored => 2 * n - 1,
                }
            );
            assert_eq!(
                model.equations.len(),
                match mode {
                    Mode::Raw => n + 2,
                    Mode::Compact => n,
                    Mode::Factored => n + 1,
                } as usize
            );
            for p in [&c.generator, &target] {
                let a = model.assignment(p).unwrap();
                assert!(model.accepts(&a));
                assert_eq!(model.decode(&a).unwrap(), *p);
            }
            if mode == Mode::Raw {
                assert!(!model.accepts(&raw_bits(&torsion, n)));
                assert!(!model.accepts(&raw_bits(&order4, n)));
                assert!(!model.accepts(&raw_bits(&point_add(&c, &c.generator, &torsion), n)));
            }
            println!("FULL_MODEL n={n} mode={mode:?} variables={} equations={} terms={} degree={} and_aux={}",
                model.variables,model.equations.len(),model.monomials(),model.maximum_degree(),model.auxiliary_variables());
        }
        assert_eq!(
            v.from_coordinates(&v.coordinates(&target).unwrap())
                .unwrap(),
            target
        );
        let shifted = point_add(&c, &c.generator, &torsion);
        assert_eq!(v.project(&shifted).unwrap(), c.generator);
    }
}
#[test]
fn malformed_inputs_and_caps_do_not_silently_admit_another_model() {
    let mut c = fixture(5);
    c.a = F2mElement::one(5);
    assert!(matches!(
        TraceZeroVariety::new(&c),
        Err(TraceZeroError::UnsupportedCurve)
    ));
    c = fixture(5);
    c.irreducible.low_terms = vec![0, 1, 2, 3, 4]; // x5+x4+x3+x2+x+1 has x+1.
    assert!(matches!(
        TraceZeroVariety::new(&c),
        Err(TraceZeroError::InvalidField)
    ));
    c = fixture(5);
    c.order += 1u32;
    assert!(matches!(
        TraceZeroVariety::new(&c),
        Err(TraceZeroError::UnsupportedCurve)
    ));
    let c = fixture(5);
    let v = TraceZeroVariety::new(&c).unwrap();
    assert!(matches!(
        v.affine_system(Mode::Raw, 0),
        Err(TraceZeroError::TermCap)
    ));
    assert!(matches!(
        v.from_coordinates(&Coordinates::Affine {
            x_free: BigUint::one() << 4,
            y_free: BigUint::zero()
        }),
        Err(TraceZeroError::InvalidCoordinates)
    ));
    let model = v.affine_system(Mode::Raw, 2_000_000).unwrap();
    let mut solver = Solver::new(model.variables);
    let mapping: Vec<_> = (1..=model.variables).collect();
    let before = (solver.n_vars(), solver.n_clauses(), solver.n_xors());
    let cap = model.variables + model.auxiliary_variables() as u32 - 1;
    assert!(matches!(
        model.append_to_solver(&mut solver, &mapping, cap),
        Err(TraceZeroError::VariableCap { .. })
    ));
    assert_eq!(
        before,
        (solver.n_vars(), solver.n_clauses(), solver.n_xors())
    );
}
#[test]
fn full_trace_zero_sat_rejects_the_two_torsion_point_at_degree_131() {
    let c = fixture(131);
    let v = TraceZeroVariety::new(&c).unwrap();
    let mut chain = ChainedS3Encoding::build(2, &c.irreducible, &c.b, None, 0, 200_000).unwrap();
    chain
        .constrain_summands(&[BigUint::zero()], 1_000_000)
        .unwrap();
    chain
        .constrain_full_trace_zero_summands(&v, 200_000, 2_000_000)
        .unwrap();
    assert_eq!(chain.solver.solve(), SolveResult::Unsat);
}
#[test]
fn wide_native_chain_pinned_points_have_full_membership_and_group_replay() {
    for n in [83, 131] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        assert!(chain_s3_max_variables(n, 3).is_some());
        let BinaryPoint::Affine { x, .. } = &c.generator else {
            panic!("generator")
        };
        // Identity intermediate: P + (-P) + P = P. No affine-only shortcut.
        let mut enc =
            ChainedS3Encoding::build(3, &c.irreducible, &c.b, Some(x), 1, 300_000).unwrap();
        enc.constrain_summands(&[x.to_biguint()], 1_000_000)
            .unwrap();
        enc.constrain_full_trace_zero_summands(&v, 300_000, 2_000_000)
            .unwrap();
        enc.pin_full_trace_zero_summands(&[
            c.generator.clone(),
            point_neg(&c.generator),
            c.generator.clone(),
        ])
        .unwrap();
        enc.solver.conflict_budget = 10_000;
        enc.solver.deadline = Some(std::time::Instant::now() + std::time::Duration::from_secs(10));
        assert_eq!(enc.solver.solve(), SolveResult::Sat);
        assert!(enc.verify_model());
        for p in enc.decode_full_trace_zero_summands().unwrap() {
            assert!(v.contains(&p));
            assert_eq!(v.point_trace(&p).unwrap(), BinaryPoint::Infinity);
        }
        println!(
            "FULL_CHAIN n={n} m=3 pattern=identity variables={} clauses={} xors={}",
            enc.solver.n_vars(),
            enc.solver.n_clauses(),
            enc.solver.n_xors()
        );
    }
}

#[test]
fn wide_x_only_control_preserves_an_unknown_budget_outcome() {
    let c = fixture(83);
    let v = TraceZeroVariety::new(&c).unwrap();
    let BinaryPoint::Affine { x, .. } = &c.generator else {
        panic!("generator")
    };
    let mut enc = ChainedS3Encoding::build(3, &c.irreducible, &c.b, Some(x), 1, 300_000).unwrap();
    enc.constrain_summands(&[x.to_biguint()], 1_000_000)
        .unwrap();
    enc.constrain_full_trace_zero_summands(&v, 300_000, 2_000_000)
        .unwrap();
    enc.solver.conflict_budget = 2_000;
    enc.solver.deadline = Some(std::time::Instant::now() + std::time::Duration::from_secs(1));
    let outcome = enc.solver.solve();
    assert_ne!(
        outcome,
        SolveResult::Unsat,
        "known group witness must not be refuted"
    );
    if outcome == SolveResult::Sat {
        assert!(enc.verify_model());
    }
    println!("BOUNDED_X_ONLY n=83 outcome={outcome:?}; Unknown is not a completed solve");
}
#[test]
fn complete_relation_oracle_matches_group_enumeration_and_keeps_caps_inconclusive() {
    let c = fixture(5);
    let v = TraceZeroVariety::new(&c).unwrap();
    let g = all_points(&c)
        .into_iter()
        .find(|p| *p != BinaryPoint::Infinity && v.contains(p))
        .unwrap();
    let base = vec![g.clone(), point_neg(&g)];
    let subgroup: Vec<_> = all_points(&c)
        .into_iter()
        .filter(|p| v.contains(p))
        .collect();
    for m in [2, 3] {
        for target in &subgroup {
            let mut exists = false;
            for i in 0..2 {
                for j in 0..2 {
                    let sum = point_add(&c, &base[i], &base[j]);
                    if m == 2 {
                        exists |= sum == *target;
                    } else {
                        for k in 0..2 {
                            exists |= point_add(&c, &sum, &base[k]) == *target;
                        }
                    }
                }
            }
            let out =
                decompose_full_trace_zero(&v, &base, target, m, TraceZeroRelationLimits::default());
            assert_ne!(out.disposition, Disposition::Inconclusive, "{out:?}");
            assert_eq!(out.disposition == Disposition::Found, exists);
            if let Some(path) = out.indices {
                let sum = path
                    .iter()
                    .fold(BinaryPoint::Infinity, |s, &i| point_add(&c, &s, &base[i]));
                assert_eq!(sum, *target);
            }
        }
    }
    let mut limits = TraceZeroRelationLimits::default();
    limits.max_variables = 1;
    assert_eq!(
        decompose_full_trace_zero(&v, &base, &g, 2, limits).disposition,
        Disposition::Inconclusive
    );
    limits = TraceZeroRelationLimits::default();
    limits.max_patterns = 0;
    assert_eq!(
        decompose_full_trace_zero(&v, &base, &g, 3, limits).disposition,
        Disposition::Inconclusive
    );
    let bad = BinaryPoint::Affine {
        x: F2mElement::zero(5),
        y: F2mElement::one(5),
    };
    assert_eq!(
        decompose_full_trace_zero(&v, &[bad], &g, 2, TraceZeroRelationLimits::default())
            .disposition,
        Disposition::Inconclusive
    );
}
