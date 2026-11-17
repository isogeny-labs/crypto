//! Exact full-trace-zero rational-point relation oracle.
//!
//! All S3 intermediate-identity patterns are retained. Membership can be
//! supplied by explicit affine witnesses or by the checked finite domain.
//! A model is still checked in the original group; caps never imply refutation.

use crate::binary_ecc::{
    curve::{point_add, point_neg},
    BinaryPoint, F2mElement, TraceZeroVariety,
};
use crate::cryptanalysis::binary_semaev_chain_sat::{
    finite_domain_clause_count, ChainedS3Encoding,
};
use crate::cryptanalysis::sat::SolveResult;
use num_bigint::BigUint;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceZeroMembershipEncoding {
    /// Add ordinate witnesses, original curve rows and both membership bits.
    ExplicitPoints,
    /// Classical x-only full-membership test with inverse witnesses.
    ExplicitKummerInverse,
    /// Every allowed x is obtained from a validated full-kernel base point.
    /// Its two affine lifts differ by negation, so both lie in the kernel.
    /// Keep the finite x domain and replay actual supplied base lifts.
    CertifiedDomain,
    /// Same admitted finite domain with target/common-bit constant folding.
    CertifiedDomainWithConstants,
    /// m3 only: same complete masks, identity intermediate searched first.
    CertifiedDomainIdentityFirst,
    /// m3 only: finite partial sum constrained to actual x(T-B).
    CertifiedDomainWithTail,
    /// m3 only: preserve the actual (last x, partial-sum x) correlation.
    CertifiedDomainWithTailPairs,
    /// Certified finite domain plus the necessary trace bit on each
    /// finite partial sum. No restriction to the original subspace.
    CertifiedDomainWithPartialTrace,
}

#[derive(Clone, Copy, Debug)]
pub struct TraceZeroRelationLimits {
    pub max_variables: u32,
    pub max_domain_clauses: usize,
    pub max_polynomial_terms: usize,
    pub max_models: usize,
    pub max_patterns: usize,
    pub conflict_budget: u64,
    pub wall_budget: Option<Duration>,
}
impl Default for TraceZeroRelationLimits {
    fn default() -> Self {
        Self {
            max_variables: 500_000,
            max_domain_clauses: 1_000_000,
            max_polynomial_terms: 2_000_000,
            max_models: 1024,
            max_patterns: 128,
            conflict_budget: 100_000,
            wall_budget: None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceZeroRelationDisposition {
    Found,
    Refuted,
    Inconclusive,
}
#[derive(Clone, Debug)]
pub struct TraceZeroRelationReport {
    pub disposition: TraceZeroRelationDisposition,
    pub indices: Option<Vec<usize>>,
    pub patterns_attempted: usize,
    pub patterns_refuted: usize,
    pub models: usize,
    pub conflicts: u64,
    pub peak_variables: u32,
    pub full_membership_equations: usize,
    pub partial_trace_equations: usize,
    pub reason: Option<&'static str>,
}
impl Default for TraceZeroRelationReport {
    fn default() -> Self {
        Self {
            disposition: TraceZeroRelationDisposition::Inconclusive,
            indices: None,
            patterns_attempted: 0,
            patterns_refuted: 0,
            models: 0,
            conflicts: 0,
            peak_variables: 0,
            full_membership_equations: 0,
            partial_trace_equations: 0,
            reason: None,
        }
    }
}
fn lift(
    variety: &TraceZeroVariety,
    base: &[BinaryPoint],
    xs: &[F2mElement],
    target: &BinaryPoint,
) -> Option<Vec<usize>> {
    fn visit(
        v: &TraceZeroVariety,
        b: &[BinaryPoint],
        xs: &[F2mElement],
        t: &BinaryPoint,
        i: usize,
        sum: &BinaryPoint,
        path: &mut Vec<usize>,
    ) -> bool {
        if i == xs.len() {
            return sum == t;
        }
        for (j, p) in b.iter().enumerate() {
            if !matches!(p,BinaryPoint::Affine{x,..} if *x==xs[i]) {
                continue;
            }
            path.push(j);
            if visit(v, b, xs, t, i + 1, &point_add(v.curve(), sum, p), path) {
                return true;
            }
            path.pop();
        }
        false
    }
    let mut path = Vec::new();
    visit(
        variety,
        base,
        xs,
        target,
        0,
        &BinaryPoint::Infinity,
        &mut path,
    )
    .then_some(path)
}
pub fn decompose_full_trace_zero(
    variety: &TraceZeroVariety,
    base: &[BinaryPoint],
    target: &BinaryPoint,
    m: usize,
    limits: TraceZeroRelationLimits,
) -> TraceZeroRelationReport {
    decompose_full_trace_zero_with_encoding(
        variety,
        base,
        target,
        m,
        limits,
        TraceZeroMembershipEncoding::ExplicitPoints,
    )
}

/// Same allowed base, target, identity patterns and replay as the explicit
/// encoding. CertifiedDomain existentially eliminates redundant summand
/// membership witnesses; it does not skip admission of any supplied point.
pub fn decompose_full_trace_zero_with_encoding(
    variety: &TraceZeroVariety,
    base: &[BinaryPoint],
    target: &BinaryPoint,
    m: usize,
    limits: TraceZeroRelationLimits,
    encoding: TraceZeroMembershipEncoding,
) -> TraceZeroRelationReport {
    decompose_with_admission(variety, base, target, m, limits, encoding, false)
}

// Only the prepared residual path calls this with admitted=true, after
// validating immutable original inputs and computing residuals in T_n.
fn decompose_with_admission(
    variety: &TraceZeroVariety,
    base: &[BinaryPoint],
    target: &BinaryPoint,
    m: usize,
    limits: TraceZeroRelationLimits,
    encoding: TraceZeroMembershipEncoding,
    admitted: bool,
) -> TraceZeroRelationReport {
    let mut report = TraceZeroRelationReport::default();
    if !(2..=9).contains(&m) {
        report.reason = Some("unsupported arity");
        return report;
    }
    let terminal = matches!(
        encoding,
        TraceZeroMembershipEncoding::CertifiedDomainIdentityFirst
            | TraceZeroMembershipEncoding::CertifiedDomainWithTail
            | TraceZeroMembershipEncoding::CertifiedDomainWithTailPairs
    );
    if terminal && m != 3 {
        report.reason = Some("terminal specialization requires m3");
        return report;
    }
    if !admitted && !variety.contains(target) {
        report.reason = Some("target is not a trace-zero rational point");
        return report;
    }
    if !admitted
        && base
            .iter()
            .any(|p| matches!(p, BinaryPoint::Infinity) || !variety.contains(p))
    {
        report.reason = Some("invalid or non-trace-zero factor-base point");
        return report;
    }
    if limits.max_models == 0 || limits.max_patterns == 0 || limits.conflict_budget == 0 {
        report.reason = Some("zero search budget");
        return report;
    }
    if base.is_empty() {
        report.disposition = TraceZeroRelationDisposition::Refuted;
        return report;
    }
    let mut codes: Vec<BigUint> = base
        .iter()
        .filter_map(|p| match p {
            BinaryPoint::Affine { x, .. } => Some(x.to_biguint()),
            _ => None,
        })
        .collect();
    codes.sort_unstable();
    codes.dedup();
    let curve = variety.curve();
    let target_x = match target {
        BinaryPoint::Affine { x, .. } => Some(x),
        _ => None,
    };
    let all_patterns = 1usize << (m - 2);
    let deadline = limits.wall_budget.map(|d| Instant::now() + d);
    // Computed from the actual supplied signs; no pair-sum truth table.
    let tail: Vec<_> = if matches!(
        encoding,
        TraceZeroMembershipEncoding::CertifiedDomainWithTail
            | TraceZeroMembershipEncoding::CertifiedDomainWithTailPairs
    ) {
        base.iter()
            .map(|p| point_add(curve, target, &point_neg(p)))
            .collect()
    } else {
        Vec::new()
    };
    let constants =
        encoding == TraceZeroMembershipEncoding::CertifiedDomainWithConstants || terminal;
    for ordinal in 0..all_patterns {
        if ordinal >= limits.max_patterns {
            report.reason = Some("identity-pattern budget");
            return report;
        }
        let mask = if encoding == TraceZeroMembershipEncoding::CertifiedDomainIdentityFirst {
            all_patterns - 1 - ordinal
        } else {
            ordinal
        };
        if deadline.is_some_and(|d| Instant::now() >= d) {
            report.reason = Some("deadline");
            return report;
        }
        let built = if constants {
            ChainedS3Encoding::build_for_domain(
                m,
                &curve.irreducible,
                &curve.b,
                target_x,
                mask as u32,
                limits.max_variables,
                &codes,
                limits.max_domain_clauses,
            )
        } else {
            ChainedS3Encoding::build(
                m,
                &curve.irreducible,
                &curve.b,
                target_x,
                mask as u32,
                limits.max_variables,
            )
        };
        let mut enc = match built {
            Ok(e) => e,
            Err(_) => {
                report.reason = Some("S3 encoding admission or variable cap");
                return report;
            }
        };
        if !constants
            && enc
                .constrain_summands(&codes, limits.max_domain_clauses)
                .is_err()
        {
            report.reason = Some("finite factor-base domain cap");
            return report;
        }
        if mask == 0 && !tail.is_empty() {
            let used = finite_domain_clause_count(&codes, curve.m, m).expect("admitted domain");
            let Some(remaining_clauses) = limits.max_domain_clauses.checked_sub(used) else {
                report.reason = Some("terminal total domain clause cap");
                return report;
            };
            let installed = if encoding == TraceZeroMembershipEncoding::CertifiedDomainWithTailPairs
            {
                let pairs: Vec<_> = base
                    .iter()
                    .zip(&tail)
                    .filter_map(|(p, s)| match (p, s) {
                        (BinaryPoint::Affine { x: r, .. }, BinaryPoint::Affine { x: s, .. }) => {
                            Some(r.to_biguint() | (s.to_biguint() << curve.m as usize))
                        }
                        _ => None,
                    })
                    .collect();
                enc.constrain_terminal_pair_domain(&pairs, remaining_clauses)
            } else {
                let xs: Vec<_> = tail
                    .iter()
                    .filter_map(|s| match s {
                        BinaryPoint::Affine { x, .. } => Some(x.to_biguint()),
                        _ => None,
                    })
                    .collect();
                enc.constrain_intermediate(1, &xs, remaining_clauses)
            };
            if installed.is_err() {
                report.reason = Some("terminal domain admission or clause cap");
                return report;
            }
        }
        if encoding == TraceZeroMembershipEncoding::ExplicitPoints {
            match enc.constrain_full_trace_zero_summands(
                variety,
                limits.max_variables,
                limits.max_polynomial_terms,
            ) {
                Ok(n) => report.full_membership_equations = n,
                Err(_) => {
                    report.reason = Some("full trace-zero model or variable cap");
                    return report;
                }
            }
        }
        if encoding == TraceZeroMembershipEncoding::ExplicitKummerInverse {
            match enc.constrain_kummer_trace_zero_summands(
                variety,
                limits.max_variables,
                limits.max_polynomial_terms,
            ) {
                Ok(n) => report.full_membership_equations = n,
                Err(_) => {
                    report.reason = Some("Kummer inverse model or variable cap");
                    return report;
                }
            }
        }
        if encoding == TraceZeroMembershipEncoding::CertifiedDomainWithPartialTrace {
            match enc.constrain_trace_zero_intermediates(variety) {
                Ok(n) => report.partial_trace_equations = n,
                Err(_) => {
                    report.reason = Some("partial trace admission");
                    return report;
                }
            }
        }
        report.peak_variables = report.peak_variables.max(enc.solver.n_vars());
        report.patterns_attempted += 1;
        let prior = report.conflicts;
        if prior >= limits.conflict_budget {
            report.reason = Some("conflict budget");
            return report;
        }
        enc.solver.conflict_budget = limits.conflict_budget - prior;
        enc.solver.deadline = deadline;
        loop {
            let result = enc.solver.solve();
            report.conflicts = prior.saturating_add(enc.solver.conflicts());
            match result {
                SolveResult::Unknown => {
                    report.reason = Some("SAT budget or solver failure");
                    return report;
                }
                SolveResult::Unsat => {
                    report.patterns_refuted += 1;
                    break;
                }
                SolveResult::Sat => {
                    report.models += 1;
                    let xs = enc.decode_summands();
                    if !enc.verify_model()
                        || xs
                            .iter()
                            .any(|x| codes.binary_search(&x.to_biguint()).is_err())
                    {
                        report.reason = Some("model failed original field or trace-zero replay");
                        return report;
                    }
                    if let Some(path) = lift(variety, base, &xs, target) {
                        let sum = path.iter().fold(BinaryPoint::Infinity, |s, &i| {
                            point_add(curve, &s, &base[i])
                        });
                        if sum != *target {
                            report.reason = Some("group replay failure");
                            return report;
                        }
                        report.disposition = TraceZeroRelationDisposition::Found;
                        report.indices = Some(path);
                        return report;
                    }
                    if report.models >= limits.max_models {
                        report.reason = Some("model budget");
                        return report;
                    }
                    let bits = enc.solver.model();
                    enc.solver.reset_search();
                    enc.solver.add_clause(
                        (0..m * curve.m as usize)
                            .map(|i| {
                                if bits[i] {
                                    -((i + 1) as i32)
                                } else {
                                    (i + 1) as i32
                                }
                            })
                            .collect(),
                    );
                }
            }
        }
    }
    report.disposition = TraceZeroRelationDisposition::Refuted;
    report
}

/// Enumerate only the final actual base point and solve P1+P2=T-P3 by
/// constant-target S3. Every residual shares the total conflict/model/deadline
/// budget; a capped residual prevents Refuted unless a later slice finds a sum.
pub fn decompose_trace_zero_terminal_slices(
    variety: &TraceZeroVariety,
    base: &[BinaryPoint],
    target: &BinaryPoint,
    limits: TraceZeroRelationLimits,
) -> TraceZeroRelationReport {
    terminal_slices(variety, base, target, limits, false)
}

/// Same complete residual search, with base/target admission charged once
/// per outer query. Residuals are computed from admitted T_n points.
pub fn decompose_trace_zero_terminal_slices_prepared(
    variety: &TraceZeroVariety,
    base: &[BinaryPoint],
    target: &BinaryPoint,
    limits: TraceZeroRelationLimits,
) -> TraceZeroRelationReport {
    terminal_slices(variety, base, target, limits, true)
}
fn terminal_slices(
    variety: &TraceZeroVariety,
    base: &[BinaryPoint],
    target: &BinaryPoint,
    limits: TraceZeroRelationLimits,
    prepared: bool,
) -> TraceZeroRelationReport {
    let mut out = TraceZeroRelationReport::default();
    if !variety.contains(target)
        || base
            .iter()
            .any(|p| matches!(p, BinaryPoint::Infinity) || !variety.contains(p))
    {
        out.reason = Some("invalid terminal-slice target/base");
        return out;
    }
    if limits.conflict_budget == 0 || limits.max_models == 0 || limits.max_patterns == 0 {
        out.reason = Some("zero terminal-slice budget");
        return out;
    }
    if base.is_empty() {
        out.disposition = TraceZeroRelationDisposition::Refuted;
        return out;
    }
    let deadline = limits.wall_budget.map(|d| Instant::now() + d);
    let residuals: Vec<_> = base
        .iter()
        .map(|p| point_add(variety.curve(), target, &point_neg(p)))
        .collect();
    let mut capped = false;
    for (k, residual) in residuals.iter().enumerate() {
        let remaining = limits.conflict_budget.saturating_sub(out.conflicts);
        let models = limits.max_models.saturating_sub(out.models);
        if remaining == 0 || models == 0 || deadline.is_some_and(|d| Instant::now() >= d) {
            out.reason = Some("terminal-slice total budget");
            return out;
        }
        let count = (base.len() - k) as u64;
        let wall = deadline.map(|d| {
            d.saturating_duration_since(Instant::now()) / (count.min(u32::MAX as u64) as u32)
        });
        let r = decompose_with_admission(
            variety,
            base,
            residual,
            2,
            TraceZeroRelationLimits {
                conflict_budget: (remaining / count).max(1),
                max_models: models,
                wall_budget: wall,
                ..limits
            },
            TraceZeroMembershipEncoding::CertifiedDomainWithConstants,
            prepared,
        );
        out.conflicts = out.conflicts.saturating_add(r.conflicts);
        out.models = out.models.saturating_add(r.models);
        out.patterns_attempted += r.patterns_attempted;
        out.patterns_refuted += r.patterns_refuted;
        out.peak_variables = out.peak_variables.max(r.peak_variables);
        match r.disposition {
            TraceZeroRelationDisposition::Found => {
                let mut indices = r.indices.expect("Found inner witness");
                indices.push(k);
                if indices.len() != 3
                    || indices.iter().fold(BinaryPoint::Infinity, |s, &i| {
                        point_add(variety.curve(), &s, &base[i])
                    }) != *target
                {
                    out.reason = Some("terminal-slice original replay failed");
                    return out;
                }
                out.indices = Some(indices);
                out.disposition = TraceZeroRelationDisposition::Found;
                return out;
            }
            TraceZeroRelationDisposition::Refuted => {}
            TraceZeroRelationDisposition::Inconclusive => capped = true,
        }
    }
    if capped {
        out.reason = Some("one or more terminal slices capped");
    } else {
        out.disposition = TraceZeroRelationDisposition::Refuted;
    }
    out
}
