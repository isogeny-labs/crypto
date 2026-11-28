//! Checked geometric projection onto the prescribed four-point M31 section.
//!
//! If I contains t²-t and u²-u, R/I is the product of its four parameter
//! fiber algebras. The four indicator polynomials are their identities. A
//! fiber has a geometric point exactly when its identity has nonzero normal
//! form modulo a genuine ambient Gröbner basis of I. This remains valid for
//! nonreduced or positive-dimensional auxiliary fibers; it does not require
//! the auxiliary coordinates to be rational over M31.
//!
//! The basis is checked against the actual original inputs in this call.
//! Both section memberships and all four indicator divisions must finish
//! before any complete point set is exposed. No partial point set, caller-
//! asserted verification flag, lex conversion, or new F4 solve is used.
//! This is exact projection on this section, not general root extraction or
//! signed elliptic-curve relation recovery.

use super::{independent_chart_generators, verify, Limits, OrdinaryOracle, Report,
            Status, Verification, VerificationStatus};
use crate::quotient::{Error, Poly, Ring};
use std::collections::BTreeSet;

/// Fixed iteration order of the four indicator remainders.
pub const SECTION_POINTS: [[u64; 2]; 4] = [[0, 0], [0, 1], [1, 0], [1, 1]];

#[derive(Clone, Copy, Debug)]
pub struct ProjectionLimits {
    /// Total leading-term iterations across the six ordinary divisions.
    pub max_iterations: usize,
    /// Total polynomial subtraction operations started across those divisions.
    pub max_reductions: usize,
    /// Terms in each active division polynomial, remainder or output
    /// polynomial, separately. Already checked basis storage is controlled
    /// by verifier_limits; this cap is not a process-RSS bound.
    pub max_terms: usize,
    /// Maximum degree encountered in the query divisions or output
    /// polynomials, not a bound on the checked basis's generator degrees.
    pub max_degree: u64,
}

impl Default for ProjectionLimits {
    fn default() -> Self {
        Self { max_iterations: 1_000, max_reductions: 1_000,
               max_terms: 1_000, max_degree: 2 }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionPhase { Producer, Verification, Projection }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectionStatus {
    Complete,
    Rejected(String),
    Incomplete { phase: ProjectionPhase, reason: &'static str },
}

#[derive(Clone, Debug, Default)]
pub struct ProjectionMetrics {
    /// A started query may be interrupted by its degree/term/work cap.
    pub queries_started: usize,
    pub queries_completed: usize,
    pub section_queries_completed: usize,
    pub indicator_queries_completed: usize,
    /// Counts leading terms handled, including terms moved to the remainder.
    pub leading_term_iterations: usize,
    /// OrdinaryOracle's subtraction-operation counter. An operation that
    /// reaches a term limit partway through is counted as started.
    pub polynomial_reductions: usize,
    pub max_degree: u64,
}

/// Complete solution data can only be constructed inside this module.
#[derive(Clone, Debug)]
pub struct ProjectedSolution {
    parameters: [usize; 2],
    points: BTreeSet<[u64; 2]>,
    indicator_remainders: Vec<Poly>,
    elimination_generators: Vec<Poly>,
}

impl ProjectedSolution {
    pub fn parameters(&self) -> [usize; 2] { self.parameters }
    pub fn points(&self) -> &BTreeSet<[u64; 2]> { &self.points }
    /// Ambient-ring normal forms in SECTION_POINTS order.
    pub fn indicator_remainders(&self) -> &[Poly] { &self.indicator_remainders }
    /// Polynomials in TWO variables, positions [t,u]: t²-t, u²-u, Q.
    /// Q is the sum of the excluded indicators. These generate the exact
    /// elimination ideal; they are not promised to be a reduced basis.
    pub fn elimination_generators(&self) -> &[Poly] { &self.elimination_generators }
}

#[derive(Clone, Debug)]
pub struct ProjectionReport {
    pub status: ProjectionStatus,
    /// The actual verification performed by this call; absent only when the
    /// producer was incomplete. Its counts are separate from projection work.
    pub verification: Option<Verification>,
    pub metrics: ProjectionMetrics,
    solution: Option<ProjectedSolution>,
}

impl ProjectionReport {
    pub fn solution(&self) -> Option<&ProjectedSolution> {
        if self.status == ProjectionStatus::Complete { self.solution.as_ref() }
        else { None }
    }
}

// The ordinary division in the parent module uses this optional meter only
// for projection. The producer and ordinary verifier counters are unchanged.
pub(super) struct DivisionMeter {
    limits: ProjectionLimits,
    metrics: ProjectionMetrics,
}

impl DivisionMeter {
    pub(super) fn observe_degree(&mut self, degree: u64) -> Result<(), Error> {
        self.metrics.max_degree = self.metrics.max_degree.max(degree);
        if degree > self.limits.max_degree { Err(Error::Limit("projection degree")) }
        else { Ok(()) }
    }

    pub(super) fn start_iteration(&mut self, degree: u64) -> Result<(), Error> {
        self.observe_degree(degree)?;
        let next = self.metrics.leading_term_iterations.checked_add(1)
            .ok_or(Error::Limit("projection leading-term iterations"))?;
        if next > self.limits.max_iterations {
            return Err(Error::Limit("projection leading-term iterations"));
        }
        self.metrics.leading_term_iterations = next;
        Ok(())
    }

    fn check_output(&mut self, p: &Poly) -> Result<(), Error> {
        self.observe_degree(p.degree())?;
        if p.terms.len() > self.limits.max_terms {
            Err(Error::Limit("projection output terms"))
        } else { Ok(()) }
    }
}

fn section_polynomials(n: usize, parameters: [usize; 2]) -> Result<[Poly; 2], Error> {
    let t = Poly::variable(n, parameters[0]);
    let u = Poly::variable(n, parameters[1]);
    Ok([t.pow(2)?.sub(&t)?, u.pow(2)?.sub(&u)?])
}

fn indicator(n: usize, parameters: [usize; 2], point: [u64; 2]) -> Result<Poly, Error> {
    let one = Poly::constant(n, 1);
    let t = Poly::variable(n, parameters[0]);
    let u = Poly::variable(n, parameters[1]);
    let left = if point[0] == 0 { one.sub(&t)? } else { t };
    let right = if point[1] == 0 { one.sub(&u)? } else { u };
    left.mul(&right)
}

fn projection_limit_reason(reason: &'static str) -> &'static str {
    match reason {
        "ordinary verification reductions" => "projection polynomial reductions",
        "ordinary verification terms" => "projection division terms",
        "ordinary verification degree" => "projection degree",
        other => other,
    }
}

/// Verify the actual candidate/input ideal and recover its exact geometric
/// projection on the asserted section. Invalid indices/polynomial data are
/// errors; a valid ideal without the section or an invalid candidate basis
/// is rejected. Limits at every stage remain explicitly incomplete.
pub fn verify_and_project(
    ring: &Ring,
    inputs: &[Poly],
    basis: &Report,
    parameters: [usize; 2],
    verifier_limits: Limits,
    projection_limits: ProjectionLimits,
) -> Result<ProjectionReport, Error> {
    let n = ring.nvars();
    if parameters[0] == parameters[1] || parameters.iter().any(|&i| i >= n) {
        return Err(Error::Invalid("two distinct in-range projection parameters required".into()));
    }
    for p in inputs.iter().chain(&basis.basis) {
        p.validate()?;
        if p.variables != n { return Err(Error::Invalid("projection polynomial width".into())); }
    }
    let mut result = ProjectionReport { status: ProjectionStatus::Complete,
        verification: None, metrics: ProjectionMetrics::default(), solution: None };
    if let Status::Incomplete { reason } = &basis.status {
        result.status = ProjectionStatus::Incomplete {
            phase: ProjectionPhase::Producer, reason: *reason,
        };
        return Ok(result);
    }

    let verification = verify(ring, inputs, basis, verifier_limits)?;
    let verification_status = verification.status.clone();
    result.verification = Some(verification);
    match verification_status {
        VerificationStatus::Verified => {},
        VerificationStatus::Rejected(reason) => {
            result.status = ProjectionStatus::Rejected(reason);
            return Ok(result);
        },
        VerificationStatus::Incomplete { reason } => {
            result.status = ProjectionStatus::Incomplete {
                phase: ProjectionPhase::Verification, reason,
            };
            return Ok(result);
        },
    }

    let mut ambient = independent_chart_generators(ring)?;
    ambient.extend(basis.basis.iter().filter(|p| !p.is_zero()).cloned());
    let mut oracle = OrdinaryOracle {
        limits: Limits {
            max_reductions: projection_limits.max_reductions,
            max_nonzeros: projection_limits.max_terms,
            max_degree: projection_limits.max_degree,
            ..verifier_limits
        },
        reductions: 0, pairs: 0,
    };
    let mut meter = DivisionMeter { limits: projection_limits, metrics: ProjectionMetrics::default() };
    let extraction = (|| -> Result<Option<ProjectedSolution>, Error> {
        for (i, p) in section_polynomials(n, parameters)?.into_iter().enumerate() {
            meter.metrics.queries_started += 1;
            let remainder = oracle.remainder_measured(p, &ambient, Some(&mut meter))?;
            meter.metrics.queries_completed += 1;
            meter.metrics.section_queries_completed += 1;
            if !remainder.is_zero() {
                result.status = ProjectionStatus::Rejected(format!(
                    "original ideal does not imply section polynomial for parameter {}", parameters[i]));
                return Ok(None);
            }
        }

        let mut points = BTreeSet::new();
        let mut remainders = Vec::with_capacity(4);
        let mut excluded = Poly::zero(2);
        for point in SECTION_POINTS {
            meter.metrics.queries_started += 1;
            let p = indicator(n, parameters, point)?;
            let remainder = oracle.remainder_measured(p, &ambient, Some(&mut meter))?;
            meter.metrics.queries_completed += 1;
            meter.metrics.indicator_queries_completed += 1;
            if remainder.is_zero() {
                excluded = excluded.add(&indicator(2, [0, 1], point)?)?;
                meter.check_output(&excluded)?;
            } else { points.insert(point); }
            remainders.push(remainder);
        }

        let mut generators = section_polynomials(2, [0, 1])?.to_vec();
        generators.push(excluded);
        for p in &generators { meter.check_output(p)?; }
        Ok(Some(ProjectedSolution { parameters, points,
            indicator_remainders: remainders, elimination_generators: generators }))
    })();
    meter.metrics.polynomial_reductions = oracle.reductions;
    result.metrics = meter.metrics;
    match extraction {
        Ok(solution) => result.solution = solution,
        Err(Error::Limit(reason)) => result.status = ProjectionStatus::Incomplete {
            phase: ProjectionPhase::Projection, reason: projection_limit_reason(reason),
        },
        Err(error) => return Err(error),
    }
    Ok(result)
}
