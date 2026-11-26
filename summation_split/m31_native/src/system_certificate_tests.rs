//! Witness checks for the generated polynomial systems. These tests evaluate
//! every equation on actual signed elliptic-curve sums, including geometric
//! quadratic-extension coordinates. They are not Gröbner extraction tests.

use crate::multiplicity::{self, AffinePoint, Fp2, Point};
use crate::quotient::Polynomial;
use crate::systems::{self, Arm, Plane, System, WitnessMode};
use crate::M31;

fn fp2_power(mut value: Fp2, mut exponent: u64) -> Fp2 {
    let mut result = Fp2::ONE;
    while exponent != 0 {
        if exponent & 1 != 0 { result = result.times(value); }
        exponent >>= 1;
        if exponent != 0 { value = value.square(); }
    }
    result
}

// Evaluation uses the independent oracle's field operations, not the
// candidate Polynomial::evaluate or its coefficient arithmetic.
fn evaluate_extension(polynomial: &Polynomial, values: &[Fp2]) -> Fp2 {
    assert_eq!(polynomial.variables, values.len());
    polynomial.terms.iter().fold(Fp2::ZERO, |sum, (powers, &coefficient)| {
        let term = powers.iter().zip(values).fold(Fp2::real(coefficient), |v, (&e, &x)| {
            v.times(fp2_power(x, e))
        });
        sum.plus(term)
    })
}

fn all_zero(system: &System, assignment: &[Fp2]) -> bool {
    system.equations.iter().all(|f| evaluate_extension(f, assignment) == Fp2::ZERO)
}

/// Handwritten point-addition trees, avoiding reuse of the producer's
/// recursive tree builder or tensor construction.
fn intermediate_points(points: &[Point], arm: Arm) -> (Vec<Point>, Point) {
    let add = multiplicity::add_points;
    match (points.len(), arm == Arm::PreclosedSequential) {
        (3, true) => {
            let first = add(points[0], points[1]);
            (vec![first], add(first, points[2]))
        }
        (3, false) => {
            let first = add(points[1], points[2]);
            (vec![first], add(points[0], first))
        }
        (4, true) => {
            let first = add(points[0], points[1]);
            let second = add(first, points[2]);
            (vec![first, second], add(second, points[3]))
        }
        (4, false) => {
            let first = add(points[0], points[1]);
            let second = add(points[2], points[3]);
            (vec![first, second], add(first, second))
        }
        _ => panic!("certificate tests support three or four summands"),
    }
}

fn assignment(
    system: &System,
    parameter: [u64; 2],
    signed_points: &[Point],
) -> (Vec<Fp2>, usize, usize) {
    let m = signed_points.len();
    let mut out = vec![Fp2::ZERO; system.variables.len()];
    for (i, point) in signed_points.iter().enumerate() {
        out[i] = point.expect("original inputs are finite").x.square();
    }
    let (intermediates, target) = intermediate_points(signed_points, system.arm);
    assert_eq!(target.unwrap().x, Fp2::real(system.plane.target_x));
    let mut infinity = 0;
    let mut extension = 0;
    for (i, point) in intermediates.into_iter().enumerate() {
        assert!(multiplicity::on_curve(point));
        let stride = if system.witness_mode == WitnessMode::Geometric { 3 } else { 5 };
        let offset = m + stride * i;
        match point {
            None => {
                // The normalized infinity chart is z=w=s=0, whose Veronese
                // image is (A,B,C)=(1,0,0).
                infinity += 1;
            }
            Some(point) => {
                let w = point.x;
                let s = w.square();
                out[offset] = Fp2::ONE;
                if w.b != 0 { extension += 1; }
                if system.witness_mode == WitnessMode::Geometric {
                    out[offset + 1] = w;
                    out[offset + 2] = s;
                } else {
                    out[offset + 1] = Fp2::real(w.a);
                    out[offset + 2] = Fp2::real(w.b);
                    out[offset + 3] = Fp2::real(s.a);
                    out[offset + 4] = Fp2::real(s.b);
                }
            }
        }
    }
    out[system.parameters[0]] = Fp2::real(parameter[0]);
    out[system.parameters[1]] = Fp2::real(parameter[1]);
    if system.witness_mode == WitnessMode::ExplicitFp2 {
        assert!(out.iter().all(|v| v.b == 0));
    }
    (out, infinity, extension)
}

fn plane(m: usize, case: usize) -> Plane {
    systems::fixture_plane(m, case).unwrap()
}

fn accepted_witnesses(plane: &Plane) -> Vec<([u64; 2], Vec<Point>)> {
    let m = plane.forms.len();
    let mut witnesses = Vec::new();
    for t in 0..=1 { for u in 0..=1 {
        let xs = systems::original_coordinates(plane, t, u);
        let points: Vec<AffinePoint> = xs.into_iter().map(|x| multiplicity::lift_x(x).unwrap()).collect();
        for signs in 0..1usize << (m - 1) {
            // Fix the first sign: global negation preserves all abscissae.
            let signed: Vec<Point> = points.iter().enumerate().map(|(i, &p)| {
                if i != 0 && signs & (1 << (i - 1)) != 0 {
                    multiplicity::negate(Some(p))
                } else { Some(p) }
            }).collect();
            let sum = signed.iter().copied().fold(None, multiplicity::add_points);
            if sum.is_some_and(|q| q.x == Fp2::real(plane.target_x)) {
                witnesses.push(([t, u], signed));
            }
        }
    } }
    witnesses
}

#[test]
fn m31_50_fixtures_per_arity_arm_mode_check_all_generator_witnesses() {
    let arms = [Arm::PreclosedSequential, Arm::PreclosedBalanced, Arm::MultiplicityBalanced];
    let modes = [WitnessMode::Geometric, WitnessMode::ExplicitFp2];
    for m in [3, 4] {
        let mut cells = [[(0usize, 0usize, 0usize, 0usize, 0usize); 2]; 3];
        for case in 0..50 {
            let plane = plane(m, case);
            let witnesses = accepted_witnesses(&plane);
            assert!(witnesses.iter().any(|(point, _)| *point == [0, 0]));
            let projected: std::collections::BTreeSet<_> = witnesses.iter().map(|(p, _)| *p).collect();
            assert_eq!(projected, systems::expected_projection(&plane).unwrap());
            for (ai, &arm) in arms.iter().enumerate() {
                for (mi, &mode) in modes.iter().enumerate() {
                    let system = systems::build(&plane, arm, mode).unwrap();
                    cells[ai][mi].0 += 1;
                    for (parameter, points) in &witnesses {
                        let (values, infinity, extension) = assignment(&system, *parameter, points);
                        for (index, f) in system.equations.iter().enumerate() {
                            assert_eq!(evaluate_extension(f, &values), Fp2::ZERO,
                                "m={m}, case={case}, arm={arm:?}, mode={mode:?}, equation={index}, parameter={parameter:?}");
                        }
                        cells[ai][mi].1 += 1;
                        cells[ai][mi].2 += infinity;
                        cells[ai][mi].3 += extension;
                        // The leaf square lift is genuinely checked.
                        let mut corrupted = values.clone();
                        corrupted[0] = corrupted[0].plus(Fp2::ONE);
                        assert!(!all_zero(&system, &corrupted));
                        cells[ai][mi].4 += 1;
                    }
                }
            }
        }
        for (ai, &arm) in arms.iter().enumerate() { for (mi, &mode) in modes.iter().enumerate() {
            let (cases, certificates, infinity, extension, mutants) = cells[ai][mi];
            assert_eq!(cases, 50);
            assert!(certificates >= 50);
            assert!(infinity > 0, "no infinity certificate in m={m}/{arm:?}/{mode:?}");
            assert!(extension > 0, "no nonrational intermediate in m={m}/{arm:?}/{mode:?}");
            assert_eq!(mutants, certificates);
            eprintln!("projective_witness_cell m={m} arm={} mode={mode:?} fixtures={cases} certificates={certificates} infinity_nodes={infinity} extension_nodes={extension} corrupted_leaf_rejections={mutants}",arm.name());
        } }
    }
}

#[test]
fn m31_50_monolithic_planes_per_arity_match_numeric_resultant_values() {
    for m in [3, 4] { for case in 0..50 {
        let plane = plane(m, case);
        let system = systems::build(&plane, Arm::Monolithic, WitnessMode::Geometric).unwrap();
        for t in 0..=1 { for u in 0..=1 {
            let xs = systems::original_coordinates(&plane, t, u);
            let fiber = multiplicity::semaev_fiber(&xs).unwrap();
            let expected = multiplicity::evaluate(&fiber, plane.target_x);
            let actual = evaluate_extension(&system.equations[0], &[Fp2::real(t), Fp2::real(u)]);
            assert_eq!(actual, Fp2::real(expected), "m={m}, case={case}, t={t}, u={u}");
        } }
    } }
}

#[test]
fn m31_missing_extension_and_infinity_witnesses_are_detected() {
    let mut nonrational_mutants = 0;
    let mut infinity_mutants = 0;
    for m in [3, 4] { for case in [2, 5] {
        let plane = plane(m, case);
        for arm in [Arm::PreclosedSequential, Arm::PreclosedBalanced] {
            let system = systems::build(&plane, arm, WitnessMode::ExplicitFp2).unwrap();
            for (parameter, points) in accepted_witnesses(&plane) {
                let (values, _, _) = assignment(&system, parameter, &points);
                assert!(all_zero(&system, &values));
                for i in 0..m - 2 {
                    let start = m + 5 * i;
                    if values[start + 2].a != 0 {
                        let mut lost_extension = values.clone();
                        lost_extension[start + 2] = Fp2::ZERO;
                        assert!(!all_zero(&system, &lost_extension));
                        nonrational_mutants += 1;
                    }
                    if values[start] == Fp2::ZERO {
                        let mut invalid_infinity = values.clone();
                        invalid_infinity[start + 1] = Fp2::ONE;
                        assert!(!all_zero(&system, &invalid_infinity));
                        infinity_mutants += 1;
                    }
                }
            }
        }
    } }
    assert!(nonrational_mutants > 0);
    assert!(infinity_mutants > 0);
    assert_eq!(M31, 2_147_483_647);
}
