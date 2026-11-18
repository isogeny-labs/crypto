use full_trace_zero_backend::binary_ecc::{
    curve::{point_add, point_neg},
    trace_zero::TraceZeroKummerMode as Mode,
    BinaryPoint, F2mElement, TraceZeroVariety,
};
use full_trace_zero_backend::cryptanalysis::{
    binary_semaev_chain_sat::ChainedS3Encoding,
    sat::{SolveResult, Solver},
};
use num_bigint::BigUint;
use num_traits::One;
use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};
use trace_zero_leads::{fixture, window};

fn actual_x_image(v: &TraceZeroVariety) -> BTreeSet<BigUint> {
    let c = v.curve();
    let mut set = BTreeSet::new();
    for raw in 1..1u32 << c.m {
        let x = F2mElement::from_biguint(&BigUint::from(raw), c.m);
        let inv = x.flt_inverse(&c.irreducible).unwrap();
        let rhs = x.add(&inv.square(&c.irreducible));
        if rhs.trace(&c.irreducible) != 0 {
            continue;
        }
        let y = x.mul(&rhs.half_trace(&c.irreducible).unwrap(), &c.irreducible);
        let p = BinaryPoint::Affine { x, y };
        assert!(c.is_on_curve(&p));
        if v.point_trace(&p).unwrap() == BinaryPoint::Infinity {
            set.insert(BigUint::from(raw));
        }
    }
    set
}

#[test]
fn inverse_membership_matches_actual_group_trace_for_every_small_x() {
    for n in [3, 5, 7, 9] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let expected = actual_x_image(&v);
        assert_eq!(
            BigUint::from(expected.len()),
            (v.rational_point_order() - BigUint::one()) >> 1usize
        );
        for raw in 0..1u32 << n {
            let x = F2mElement::from_biguint(&BigUint::from(raw), n);
            assert_eq!(
                v.kummer_contains_x(&x),
                expected.contains(&BigUint::from(raw)),
                "n={n},x={raw}"
            );
            if expected.contains(&BigUint::from(raw)) {
                let p = v.lift_kummer_x(&x, false).unwrap();
                let q = v.lift_kummer_x(&x, true).unwrap();
                assert_eq!(q, point_neg(&p));
                assert_eq!(v.point_trace(&p).unwrap(), BinaryPoint::Infinity);
                assert_eq!(v.point_trace(&q).unwrap(), BinaryPoint::Infinity);
            }
        }
        println!(
            "KUMMER_IMAGE n={n} distinct_x={} rational_kernel={}",
            expected.len(),
            v.rational_point_order()
        );
    }
}

#[test]
fn both_boolean_models_have_one_inverse_witness_per_actual_x() {
    for n in [3, 5, 7] {
        let v = TraceZeroVariety::new(&fixture(n)).unwrap();
        let expected = actual_x_image(&v);
        for mode in [Mode::Raw, Mode::Compact] {
            let s = v.kummer_system(mode, 2_000_000).unwrap();
            let mut accepted = BTreeSet::new();
            for word in 0..1usize << s.variables {
                let bits: Vec<_> = (0..s.variables).map(|i| word >> i & 1 != 0).collect();
                if s.accepts(&bits) {
                    let x = s.decode_x(&bits).unwrap();
                    assert!(
                        accepted.insert(x.to_biguint()),
                        "inverse witness must be unique"
                    );
                    assert_eq!(s.assignment(&x).unwrap(), bits);
                    assert_eq!(
                        s.lift(&bits, true).unwrap(),
                        point_neg(&s.lift(&bits, false).unwrap())
                    );
                }
            }
            assert_eq!(accepted, expected);
            assert!(s.maximum_degree() <= 2);
        }
    }
}

#[test]
fn rationality_bit_is_necessary_and_torsion_is_not_a_kummer_kernel_point() {
    let c = fixture(3);
    let v = TraceZeroVariety::new(&c).unwrap();
    let x = F2mElement::from_bit_positions(&[1], 3);
    let inverse = x.flt_inverse(&c.irreducible).unwrap();
    assert_eq!(x.trace(&c.irreducible), 0);
    assert_eq!(inverse.trace(&c.irreducible), 1);
    assert!(!v.kummer_contains_x(&x));
    let mut weakened = v.kummer_system(Mode::Raw, 100_000).unwrap();
    let bits: Vec<_> = (0..3)
        .map(|i| x.to_biguint().bit(i))
        .chain((0..3).map(|i| inverse.to_biguint().bit(i)))
        .collect();
    assert!(!weakened.accepts(&bits));
    weakened.equations.remove(4); // remove Tr(inverse)=0 only
    assert!(weakened.accepts(&bits));
    assert!(weakened.decode_x(&bits).is_err());
    assert!(!v.kummer_contains_x(&F2mElement::zero(3)));
    assert!(!v.kummer_contains_x(&F2mElement::one(3)));
    println!("RATIONALITY_COUNTEREXAMPLE n=3 x=z inverse=z^2+1: trace-x and second bit pass, inverse trace rejects");
}

#[test]
fn inverse_trace_character_swaps_on_reciprocal_two_e_cosets() {
    for n in [5, 7, 9] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let torsion = BinaryPoint::Affine {
            x: F2mElement::zero(n),
            y: F2mElement::one(n),
        };
        for raw in 1..1u32 << n {
            let x = F2mElement::from_biguint(&BigUint::from(raw), n);
            let inv = x.flt_inverse(&c.irreducible).unwrap();
            if x.trace(&c.irreducible) != 0 || inv.trace(&c.irreducible) != 0 {
                continue;
            }
            assert_ne!(v.kummer_contains_x(&x), v.kummer_contains_x(&inv));
            if v.kummer_contains_x(&x) {
                let p = v.lift_kummer_x(&x, false).unwrap();
                let shifted = point_add(&c, &p, &torsion);
                assert!(matches!(shifted,BinaryPoint::Affine{x:ref z,..} if *z==inv));
                assert!(!v.contains(&shifted));
            }
        }
    }
}

#[test]
fn inverse_sat_encoder_matches_every_small_x_and_cap_is_atomic() {
    let c = fixture(5);
    let v = TraceZeroVariety::new(&c).unwrap();
    let s = v.kummer_system(Mode::Raw, 100_000).unwrap();
    for raw in 0..32u32 {
        let x = F2mElement::from_biguint(&BigUint::from(raw), 5);
        let mut solver = Solver::new(s.variables);
        let wires: Vec<_> = (1..=s.variables).collect();
        s.append_to_solver(&mut solver, &wires, 100_000).unwrap();
        for i in 0..5 {
            solver.add_clause(vec![if raw >> i & 1 != 0 {
                (i + 1) as i32
            } else {
                -((i + 1) as i32)
            }]);
        }
        let outcome = solver.solve();
        assert_eq!(outcome == SolveResult::Sat, v.kummer_contains_x(&x));
        if outcome == SolveResult::Sat {
            assert_eq!(
                s.decode_x(&solver.model()[..s.variables as usize]).unwrap(),
                x
            );
        }
    }
    let mut solver = Solver::new(s.variables);
    let wires: Vec<_> = (1..=s.variables).collect();
    assert!(s
        .append_to_solver(&mut solver, &wires, s.variables)
        .is_err());
    assert_eq!(solver.n_vars(), s.variables);
    assert_eq!(solver.n_clauses(), 0);
    assert!(v.kummer_system(Mode::Compact, 0).is_err());
}

#[test]
fn actual_wide_quadratic_models_replay_public_points_and_torsion_cosets() {
    for n in [83, 131] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let target = if n == 131 {
            full_trace_zero_backend::binary_ecc::BinaryCurve::certicom_challenge("ECC2K-130")
                .unwrap()
                .1
        } else {
            point_add(&c, &c.generator, &c.generator)
        };
        let torsion = BinaryPoint::Affine {
            x: F2mElement::zero(n),
            y: F2mElement::one(n),
        };
        for mode in [Mode::Raw, Mode::Compact] {
            let s = v.kummer_system(mode, 2_000_000).unwrap();
            assert_eq!(s.maximum_degree(), 2);
            assert_eq!(
                s.variables,
                if mode == Mode::Raw { 2 * n } else { 2 * n - 2 }
            );
            assert_eq!(
                s.equations.len(),
                if mode == Mode::Raw { n + 3 } else { n + 1 } as usize
            );
            for p in [&c.generator, &target] {
                let BinaryPoint::Affine { x, .. } = p else {
                    panic!()
                };
                let a = s.assignment(x).unwrap();
                assert!(s.accepts(&a));
                assert_eq!(s.decode_x(&a).unwrap(), *x);
                let p0 = s.lift(&a, false).unwrap();
                let p1 = s.lift(&a, true).unwrap();
                assert!(p0 == *p || p1 == *p);
            }
            let shifted = point_add(&c, &c.generator, &torsion);
            let BinaryPoint::Affine { x, .. } = shifted else {
                panic!()
            };
            assert!(s.assignment(&x).is_err());
            println!("KUMMER_MODEL n={n} mode={mode:?} variables={} equations={} degree={} terms={} and_aux={}",s.variables,s.equations.len(),s.maximum_degree(),s.monomials(),s.auxiliary_variables());
        }
        let restricted = v
            .kummer_system_over_subspace(&window(&c, 16), Mode::Compact, 2_000_000)
            .unwrap();
        println!(
            "KUMMER_SUBSPACE n={n} d=16 variables={} equations={} degree={} terms={} and_aux={}",
            restricted.variables,
            restricted.equations.len(),
            restricted.maximum_degree(),
            restricted.monomials(),
            restricted.auxiliary_variables()
        );
        assert!(restricted.maximum_degree() <= 2);
    }
}

#[test]
fn wide_inverse_membership_s3_control_retains_budget_outcomes() {
    for n in [83, 131] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let BinaryPoint::Affine { x, .. } = &c.generator else {
            panic!()
        };
        let mut e = ChainedS3Encoding::build(3, &c.irreducible, &c.b, Some(x), 1, 500_000).unwrap();
        e.constrain_summands(&[x.to_biguint()], 1_000_000).unwrap();
        e.constrain_kummer_trace_zero_summands(&v, 500_000, 2_000_000)
            .unwrap();
        e.solver.conflict_budget = 2000;
        e.solver.deadline = Some(Instant::now() + Duration::from_secs(2));
        let outcome = e.solver.solve();
        assert_ne!(outcome, SolveResult::Unsat);
        if outcome == SolveResult::Sat {
            assert!(e.verify_model());
        }
        println!("KUMMER_S3_CONTROL n={n} outcome={outcome:?} variables={} clauses={} xors={}; known singleton control, no natural-search claim",e.solver.n_vars(),e.solver.n_clauses(),e.solver.n_xors());
    }
}
