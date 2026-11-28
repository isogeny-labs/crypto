//! Exact algebraic projection controls over M31.
//!
//! These fixed, analytically specified ideals are development controls, not
//! Semaev instances or a timing panel. The sixteen support masks exhaust the
//! subsets of the prescribed four-point section; they do not sample the field.

use crate::f4::{self, Limits, Report, Status, VerificationStatus};
use crate::f4::projection::{
    verify_and_project, ProjectionLimits, ProjectionPhase, ProjectionReport,
    ProjectionStatus,
};
use crate::{Error, Poly, Ring, M31};
use std::cmp::Reverse;
use std::collections::BTreeSet;

const POINTS: [[u64; 2]; 4] = [[0, 0], [0, 1], [1, 0], [1, 1]];
const PRODUCER_LIMITS: Limits = Limits {
    max_pairs: 1_024,
    max_rows: 256,
    max_columns: 256,
    max_nonzeros: 8_192,
    max_basis: 64,
    max_degree: 8,
    max_steps: 128,
    max_reductions: 20_000,
};
const VERIFIER_LIMITS: Limits = Limits {
    max_pairs: 4_096,
    max_rows: 256,
    max_columns: 256,
    max_nonzeros: 8_192,
    max_basis: 128,
    max_degree: 12,
    max_steps: 4_096,
    max_reductions: 100_000,
};

fn coefficient(value: i128) -> u64 {
    value.rem_euclid(i128::from(M31)) as u64
}

fn section(variables: usize, parameters: [usize; 2]) -> [Poly; 2] {
    parameters.map(|index| {
        let x = Poly::variable(variables, index);
        x.mul(&x).unwrap().sub(&x).unwrap()
    })
}

// Multilinear interpolation directly from the four prescribed values of Q.
// This does not call the extractor's indicator or elimination constructors.
fn expected_q(support_mask: u8) -> Poly {
    let values = [0, 1, 2, 3].map(|i| i128::from((support_mask & (1 << i)) == 0));
    Poly::from_terms(2, [
        (vec![0, 0], coefficient(values[0])),
        (vec![1, 0], coefficient(values[2] - values[0])),
        (vec![0, 1], coefficient(values[1] - values[0])),
        (vec![1, 1], coefficient(values[3] - values[2] - values[1] + values[0])),
    ]).unwrap()
}

fn embed(polynomial: &Poly, variables: usize, parameters: [usize; 2]) -> Poly {
    assert_eq!(polynomial.variables, 2);
    Poly::from_terms(variables, polynomial.terms.iter().map(|(powers, &value)| {
        let mut ambient = vec![0; variables];
        ambient[parameters[0]] = powers[0];
        ambient[parameters[1]] = powers[1];
        (ambient, value)
    })).unwrap()
}

fn expected_points(support_mask: u8) -> BTreeSet<[u64; 2]> {
    POINTS.iter().enumerate().filter_map(|(i, &point)| {
        (support_mask & (1 << i) != 0).then_some(point)
    }).collect()
}

// Construct the ambient chart relations directly, rather than taking a
// quotient normal form or asking the producer for its fixed basis.
fn chart_relations(ring: &Ring) -> Vec<Poly> {
    let mut result = Vec::new();
    for block in 0..ring.block_count() {
        let z = Poly::variable(ring.nvars(), 3 * block);
        let w = Poly::variable(ring.nvars(), 3 * block + 1);
        let s = Poly::variable(ring.nvars(), 3 * block + 2);
        result.extend([
            z.mul(&z).unwrap().sub(&z).unwrap(),
            z.mul(&w).unwrap().sub(&w).unwrap(),
            z.mul(&s).unwrap().sub(&s).unwrap(),
            w.mul(&w).unwrap().sub(&s).unwrap(),
        ]);
    }
    result
}

fn ambient_basis(ring: &Ring, report: &Report) -> Vec<Poly> {
    let mut basis = chart_relations(ring);
    basis.extend(report.basis.clone());
    basis
}

fn leading(polynomial: &Poly) -> Option<(Vec<u64>, u64)> {
    polynomial.terms.iter().max_by_key(|(powers, _)| {
        (powers.iter().sum::<u64>(),
         powers.iter().rev().map(|&power| Reverse(power)).collect::<Vec<_>>())
    }).map(|(powers, &value)| (powers.clone(), value))
}

// Independent test-only ordinary division: grevlex is encoded as an order
// key, coefficients use signed modular arithmetic, and inverses use Euclid.
// It shares neither the producer's reduction machinery nor the projection
// divider. The finite guards below apply only to these tiny controls.
fn inverse(value: u64) -> u64 {
    let (mut r, mut next_r) = (i128::from(M31), i128::from(value));
    let (mut t, mut next_t) = (0i128, 1i128);
    while next_r != 0 {
        let quotient = r / next_r;
        (r, next_r) = (next_r, r - quotient * next_r);
        (t, next_t) = (next_t, t - quotient * next_t);
    }
    assert_eq!(r, 1);
    coefficient(t)
}

fn ordinary_remainder(mut active: Poly, basis: &[Poly]) -> Poly {
    let degree = active.degree();
    let mut remainder = Poly::zero(active.variables);
    let mut iterations = 0usize;
    while let Some((powers, value)) = leading(&active) {
        iterations += 1;
        assert!(iterations <= 1_000, "analytic control division iteration cap");
        let divisor = basis.iter().find_map(|g| {
            let (lead, scalar) = leading(g)?;
            lead.iter().zip(&powers).all(|(a, b)| a <= b)
                .then_some((g, lead, scalar))
        });
        if let Some((g, lead, scalar)) = divisor {
            let factor = coefficient(i128::from(value) * i128::from(inverse(scalar)));
            let shift: Vec<_> = powers.iter().zip(&lead).map(|(a, b)| a - b).collect();
            for (monomial, &scalar) in &g.terms {
                let shifted: Vec<_> = monomial.iter().zip(&shift).map(|(a, b)| a + b).collect();
                let previous = active.terms.get(&shifted).copied().unwrap_or(0);
                let replacement = coefficient(i128::from(previous)
                    - i128::from(factor) * i128::from(scalar));
                if replacement == 0 {
                    active.terms.remove(&shifted);
                } else {
                    active.terms.insert(shifted, replacement);
                }
            }
        } else {
            active.terms.remove(&powers);
            remainder.terms.insert(powers, value);
        }
        assert!(active.degree() <= degree, "grevlex division increased degree");
        assert!(active.terms.len() <= 1_000 && remainder.terms.len() <= 1_000);
    }
    remainder
}

fn complete_basis(ring: &Ring, inputs: &[Poly]) -> Report {
    let result = f4::compute(ring, inputs, PRODUCER_LIMITS).unwrap();
    assert!(result.is_complete(), "analytic producer did not complete: {:?}", result.status);
    result
}

fn checked_projection(
    ring: &Ring,
    inputs: &[Poly],
    parameters: [usize; 2],
    support_mask: u8,
) -> (Report, ProjectionReport) {
    let basis = complete_basis(ring, inputs);
    let projection = verify_and_project(ring, inputs, &basis, parameters,
        VERIFIER_LIMITS, ProjectionLimits::default()).unwrap();
    assert_eq!(projection.status, ProjectionStatus::Complete);
    assert_eq!(projection.verification.as_ref().unwrap().status, VerificationStatus::Verified);
    let solution = projection.solution().expect("complete projection lacks a solution");
    let expected = expected_points(support_mask);
    assert_eq!(solution.points(), &expected);
    assert_eq!(solution.parameters(), parameters);

    let elimination = solution.elimination_generators();
    assert_eq!(elimination.len(), 3);
    assert_eq!(&elimination[..2], &section(2, [0, 1]));
    assert_eq!(elimination[2], expected_q(support_mask));
    for point in POINTS {
        assert_eq!(elimination[2].evaluate(&point).unwrap(),
            u64::from(!expected.contains(&point)));
        assert_eq!(elimination.iter().all(|p| p.evaluate(&point).unwrap() == 0),
            expected.contains(&point));
    }

    let ambient = ambient_basis(ring, &basis);
    for polynomial in elimination {
        let embedded = embed(polynomial, ring.nvars(), parameters);
        assert!(ordinary_remainder(embedded, &ambient).is_zero(),
            "reported elimination generator is not in the ambient ideal");
    }
    let remainders = solution.indicator_remainders();
    assert_eq!(remainders.len(), 4);
    for (i, point) in POINTS.iter().enumerate() {
        let indicator = embed(&expected_q(15 ^ (1 << i)), ring.nvars(), parameters);
        assert_eq!(remainders[i], ordinary_remainder(indicator, &ambient));
        assert_eq!(!remainders[i].is_zero(), expected.contains(point));
    }
    assert_eq!(projection.metrics.queries_started, 6);
    assert_eq!(projection.metrics.queries_completed, 6);
    assert_eq!(projection.metrics.section_queries_completed, 2);
    assert_eq!(projection.metrics.indicator_queries_completed, 4);
    assert!(projection.metrics.leading_term_iterations >= projection.metrics.polynomial_reductions);
    assert!(projection.metrics.max_degree <= 2);
    (basis, projection)
}

#[test]
fn m31_all_sixteen_section_supports_have_exact_elimination_generators() {
    let ring = Ring::new(0, 2).unwrap();
    // This includes full (15), empty (0), all four three-point supports,
    // and the disconnected antidiagonal (6), with no random instances.
    for support_mask in 0..16u8 {
        let mut inputs = section(2, [0, 1]).to_vec();
        inputs.push(expected_q(support_mask));
        checked_projection(&ring, &inputs, [0, 1], support_mask);
    }
}

#[test]
fn m31_nonreduced_auxiliary_fibers_preserve_both_diagonal_points() {
    let ring = Ring::new(0, 3).unwrap();
    let x = Poly::variable(3, 0);
    let t = Poly::variable(3, 1);
    let u = Poly::variable(3, 2);
    let mut inputs = section(3, [1, 2]).to_vec();
    inputs.extend([x.mul(&x).unwrap(), t.mul(&x).unwrap(), u.sub(&t).unwrap()]);
    let (basis, _) = checked_projection(&ring, &inputs, [1, 2], 0b1001);
    let ambient = ambient_basis(&ring, &basis);
    // At t=u=0, x is nonzero nilpotent in the fiber algebra. The extractor
    // must use nonzero idempotents, without assuming that the ideal is radical.
    assert!(!ordinary_remainder(x.clone(), &ambient).is_zero());
    assert!(ordinary_remainder(x.mul(&x).unwrap(), &ambient).is_zero());
}

fn extension_evaluate(polynomial: &Poly, point: &[(u64, u64)]) -> (u64, u64) {
    fn multiply(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
        (coefficient(i128::from(a.0) * i128::from(b.0) - i128::from(a.1) * i128::from(b.1)),
         coefficient(i128::from(a.0) * i128::from(b.1) + i128::from(a.1) * i128::from(b.0)))
    }
    assert_eq!(polynomial.variables, point.len());
    let mut sum = (0u64, 0u64);
    for (powers, &value) in &polynomial.terms {
        let mut term = (value, 0);
        for (&power, &coordinate) in powers.iter().zip(point) {
            assert!(power <= 2, "explicit extension witness is for the input equations");
            for _ in 0..power {
                term = multiply(term, coordinate);
            }
        }
        sum = (coefficient(i128::from(sum.0) + i128::from(term.0)),
               coefficient(i128::from(sum.1) + i128::from(term.1)));
    }
    sum
}

#[test]
fn m31_geometric_extension_only_chart_selects_the_origin() {
    let ring = Ring::new(1, 2).unwrap();
    let [z, w, s, t, u] = [0, 1, 2, 3, 4].map(|i| Poly::variable(5, i));
    let one = Poly::constant(5, 1);
    let mut inputs = section(5, [3, 4]).to_vec();
    inputs.extend([z.sub(&one).unwrap(), s.add(&one).unwrap(),
                   w.mul(&t).unwrap().sub(&u).unwrap()]);
    assert_eq!(M31 % 4, 3); // -1 is nonsquare in the base field.
    // The geometric witness w=i exists in M31[i]/(i^2+1). At t=1,
    // w=u would instead be 0 or 1, contradicting w^2=-1. At t=0, u=0.
    let witness = [(1, 0), (0, 1), (M31 - 1, 0), (0, 0), (0, 0)];
    for equation in chart_relations(&ring).iter().chain(&inputs) {
        assert_eq!(extension_evaluate(equation, &witness), (0, 0));
    }
    let (basis, _) = checked_projection(&ring, &inputs, [3, 4], 0b0001);
    assert!(ordinary_remainder(w.mul(&w).unwrap().add(&one).unwrap(),
        &ambient_basis(&ring, &basis)).is_zero());
}

#[test]
fn m31_infinity_chart_keeps_both_values_of_the_free_parameter() {
    let ring = Ring::new(1, 2).unwrap();
    let z = Poly::variable(5, 0);
    let t = Poly::variable(5, 3);
    let mut inputs = section(5, [3, 4]).to_vec();
    inputs.extend([z.clone(), t.sub(&z).unwrap()]);
    for u in [0, 1] {
        let witness = [0, 0, 0, 0, u];
        assert!(chart_relations(&ring).iter().chain(&inputs)
            .all(|p| p.evaluate(&witness).unwrap() == 0));
    }
    checked_projection(&ring, &inputs, [3, 4], 0b0011);
}

#[test]
fn m31_projection_obeys_reordered_nonadjacent_parameter_indices() {
    let ring = Ring::new(0, 3).unwrap();
    let parameters = [2, 0];
    let mut inputs = section(3, parameters).to_vec();
    inputs.push(embed(&expected_q(0b0010), 3, parameters));
    checked_projection(&ring, &inputs, parameters, 0b0010);
}

#[test]
fn m31_absent_section_equations_are_rejected_after_verification() {
    let ring = Ring::new(0, 2).unwrap();
    let [bt, bu] = section(2, [0, 1]);
    for inputs in [vec![], vec![bt], vec![bu]] {
        let basis = complete_basis(&ring, &inputs);
        let projection = verify_and_project(&ring, &inputs, &basis, [0, 1],
            VERIFIER_LIMITS, ProjectionLimits::default()).unwrap();
        assert!(matches!(projection.status, ProjectionStatus::Rejected(_)));
        assert_eq!(projection.verification.as_ref().unwrap().status, VerificationStatus::Verified);
        assert!(projection.solution().is_none());
        assert_eq!(projection.metrics.indicator_queries_completed, 0);
    }
}

#[test]
fn m31_invalid_parameter_indices_and_polynomial_widths_are_errors() {
    let ring = Ring::new(0, 2).unwrap();
    let inputs = section(2, [0, 1]);
    let basis = complete_basis(&ring, &inputs);
    for parameters in [[0, 0], [2, 1], [0, 2], [usize::MAX, 1]] {
        assert!(matches!(verify_and_project(&ring, &inputs, &basis, parameters,
            VERIFIER_LIMITS, ProjectionLimits::default()), Err(Error::Invalid(_))));
    }
    let mut wrong_inputs = inputs.to_vec();
    wrong_inputs.push(Poly::constant(3, 1));
    assert!(matches!(verify_and_project(&ring, &wrong_inputs, &basis, [0, 1],
        VERIFIER_LIMITS, ProjectionLimits::default()), Err(Error::Invalid(_))));
    let mut wrong_basis = basis;
    wrong_basis.basis.push(Poly::constant(3, 1));
    assert!(matches!(verify_and_project(&ring, &inputs, &wrong_basis, [0, 1],
        VERIFIER_LIMITS, ProjectionLimits::default()), Err(Error::Invalid(_))));
}

#[test]
fn m31_forged_unit_basis_cannot_turn_a_full_section_into_empty_output() {
    let ring = Ring::new(0, 2).unwrap();
    let inputs = section(2, [0, 1]);
    let mut forged = complete_basis(&ring, &inputs);
    forged.basis = vec![Poly::constant(2, 1)];
    let projection = verify_and_project(&ring, &inputs, &forged, [0, 1],
        VERIFIER_LIMITS, ProjectionLimits::default()).unwrap();
    assert!(matches!(projection.status, ProjectionStatus::Rejected(_)));
    assert!(matches!(&projection.verification.as_ref().unwrap().status,
        VerificationStatus::Rejected(reason) if reason.contains("not in original ideal")));
    assert!(projection.solution().is_none());
    assert_eq!(projection.metrics.queries_started, 0);
}

#[test]
fn m31_same_ideal_non_groebner_generators_are_rejected() {
    let ring = Ring::new(0, 2).unwrap();
    let mut inputs = section(2, [0, 1]).to_vec();
    let t = Poly::variable(2, 0);
    let u = Poly::variable(2, 1);
    inputs.push(t.mul(&u).unwrap().sub(&Poly::constant(2, 1)).unwrap());
    let mut forged = complete_basis(&ring, &inputs);
    // The original generators generate the right ideal, but their leading
    // monomials t^2,u^2,tu miss the linear consequences t-1 and u-1.
    forged.basis = inputs.clone();
    let projection = verify_and_project(&ring, &inputs, &forged, [0, 1],
        VERIFIER_LIMITS, ProjectionLimits::default()).unwrap();
    assert!(matches!(projection.status, ProjectionStatus::Rejected(_)));
    assert!(matches!(&projection.verification.as_ref().unwrap().status,
        VerificationStatus::Rejected(reason) if reason.contains("S-pair")));
    assert!(projection.solution().is_none());
    assert_eq!(projection.metrics.queries_started, 0);
}

#[test]
fn m31_incomplete_producer_report_never_exposes_a_solution() {
    let ring = Ring::new(0, 2).unwrap();
    let inputs = section(2, [0, 1]);
    let mut incomplete = complete_basis(&ring, &inputs);
    incomplete.status = Status::Incomplete { reason: "synthetic producer cap" };
    let projection = verify_and_project(&ring, &inputs, &incomplete, [0, 1],
        VERIFIER_LIMITS, ProjectionLimits::default()).unwrap();
    assert!(matches!(projection.status, ProjectionStatus::Incomplete {
        phase: ProjectionPhase::Producer, ..
    }));
    assert!(projection.verification.is_none());
    assert!(projection.solution().is_none());
    assert_eq!(projection.metrics.queries_started, 0);
}

#[test]
fn m31_verifier_limits_remain_distinct_from_empty_projection() {
    let ring = Ring::new(0, 2).unwrap();
    let inputs = section(2, [0, 1]);
    let basis = complete_basis(&ring, &inputs);
    for limits in [Limits { max_pairs: 0, ..VERIFIER_LIMITS },
                   Limits { max_reductions: 0, ..VERIFIER_LIMITS }] {
        let projection = verify_and_project(&ring, &inputs, &basis, [0, 1],
            limits, ProjectionLimits::default()).unwrap();
        assert!(matches!(projection.status, ProjectionStatus::Incomplete {
            phase: ProjectionPhase::Verification, ..
        }));
        assert!(matches!(projection.verification.as_ref().unwrap().status,
            VerificationStatus::Incomplete { .. }));
        assert!(projection.solution().is_none());
        assert_eq!(projection.metrics.queries_started, 0);
    }
}

#[test]
fn m31_each_projection_limit_preserves_verified_but_incomplete_status() {
    let ring = Ring::new(0, 2).unwrap();
    let inputs = section(2, [0, 1]);
    let basis = complete_basis(&ring, &inputs);
    for limits in [
        ProjectionLimits { max_iterations: 0, ..ProjectionLimits::default() },
        ProjectionLimits { max_reductions: 0, ..ProjectionLimits::default() },
        ProjectionLimits { max_terms: 1, ..ProjectionLimits::default() },
        ProjectionLimits { max_degree: 1, ..ProjectionLimits::default() },
    ] {
        let projection = verify_and_project(&ring, &inputs, &basis, [0, 1],
            VERIFIER_LIMITS, limits).unwrap();
        assert!(matches!(projection.status, ProjectionStatus::Incomplete {
            phase: ProjectionPhase::Projection, ..
        }));
        assert_eq!(projection.verification.as_ref().unwrap().status, VerificationStatus::Verified);
        assert!(projection.solution().is_none());
        assert!(projection.metrics.queries_completed < 6);
    }
}

#[test]
fn m31_partial_indicator_progress_does_not_escape_as_complete_points() {
    let ring = Ring::new(0, 2).unwrap();
    let inputs = section(2, [0, 1]);
    let basis = complete_basis(&ring, &inputs);
    // Two section subtractions plus four irreducible terms of the first
    // indicator consume six iterations. The next indicator must hit the cap.
    let limits = ProjectionLimits { max_iterations: 6, ..ProjectionLimits::default() };
    let projection = verify_and_project(&ring, &inputs, &basis, [0, 1],
        VERIFIER_LIMITS, limits).unwrap();
    assert!(matches!(projection.status, ProjectionStatus::Incomplete {
        phase: ProjectionPhase::Projection, ..
    }));
    assert_eq!(projection.verification.as_ref().unwrap().status, VerificationStatus::Verified);
    assert_eq!(projection.metrics.section_queries_completed, 2);
    assert_eq!(projection.metrics.indicator_queries_completed, 1);
    assert_eq!(projection.metrics.queries_completed, 3);
    assert!(projection.solution().is_none());
}
