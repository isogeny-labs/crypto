//! Algebraic trace-zero PDP: no finite factor-base point table is constructed.
use crate::binary_ecc::{
    curve::{point_add, point_neg},
    BinaryPoint, F2mElement, TraceZeroVariety,
};
use crate::cryptanalysis::{
    binary_semaev_chain_sat::ChainedS3Encoding,
    sat::SolveResult,
    trace_zero_relations::{TraceZeroRelationDisposition as Disposition, TraceZeroRelationLimits},
};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct TraceZeroSubspaceRelationReport {
    pub disposition: Disposition,
    pub points: Option<Vec<BinaryPoint>>,
    pub patterns_attempted: usize,
    pub patterns_refuted: usize,
    pub models: usize,
    pub conflicts: u64,
    pub peak_variables: u32,
    pub reason: Option<&'static str>,
}
impl Default for TraceZeroSubspaceRelationReport {
    fn default() -> Self {
        Self {
            disposition: Disposition::Inconclusive,
            points: None,
            patterns_attempted: 0,
            patterns_refuted: 0,
            models: 0,
            conflicts: 0,
            peak_variables: 0,
            reason: None,
        }
    }
}
fn lift(v: &TraceZeroVariety, xs: &[F2mElement], target: &BinaryPoint) -> Option<Vec<BinaryPoint>> {
    let choices: Vec<_> = xs
        .iter()
        .map(|x| {
            v.lift_kummer_x(x, false)
                .ok()
                .map(|p| [p.clone(), point_neg(&p)])
        })
        .collect::<Option<_>>()?;
    fn visit(
        v: &TraceZeroVariety,
        choices: &[[BinaryPoint; 2]],
        target: &BinaryPoint,
        i: usize,
        sum: &BinaryPoint,
        path: &mut Vec<BinaryPoint>,
    ) -> bool {
        if i == choices.len() {
            return sum == target;
        }
        for p in &choices[i] {
            path.push(p.clone());
            if visit(
                v,
                choices,
                target,
                i + 1,
                &point_add(v.curve(), sum, p),
                path,
            ) {
                return true;
            }
            path.pop();
        }
        false
    }
    let mut path = Vec::new();
    visit(v, &choices, target, 0, &BinaryPoint::Infinity, &mut path).then_some(path)
}

/// B={P in T_n affine: x(P) in span(basis)} is specified algebraically.
/// The report returns actual points rather than indices into a finite table.
pub fn decompose_trace_zero_subspace(
    variety: &TraceZeroVariety,
    basis: &[F2mElement],
    target: &BinaryPoint,
    m: usize,
    limits: TraceZeroRelationLimits,
) -> TraceZeroSubspaceRelationReport {
    let mut report = TraceZeroSubspaceRelationReport::default();
    if !(2..=9).contains(&m)
        || limits.conflict_budget == 0
        || limits.max_models == 0
        || limits.max_patterns == 0
    {
        report.reason = Some("arity or zero budget");
        return report;
    }
    if !variety.contains(target) {
        report.reason = Some("invalid trace-zero target");
        return report;
    }
    let deadline = limits.wall_budget.map(|d| Instant::now() + d);
    for mask in 0..1u32 << (m - 2) {
        if mask as usize >= limits.max_patterns
            || deadline.is_some_and(|d| Instant::now() >= d)
            || report.conflicts >= limits.conflict_budget
        {
            report.reason = Some("pattern, deadline or conflict cap");
            return report;
        }
        let mut enc = match ChainedS3Encoding::build_for_trace_zero_subspace(
            m,
            variety,
            basis,
            target,
            mask,
            limits.max_variables,
            limits.max_polynomial_terms,
        ) {
            Ok(e) => e,
            Err(_) => {
                report.reason = Some("subspace inverse encoding admission or cap");
                return report;
            }
        };
        report.patterns_attempted += 1;
        report.peak_variables = report.peak_variables.max(enc.solver.n_vars());
        let prior = report.conflicts;
        enc.solver.conflict_budget = limits.conflict_budget - prior;
        enc.solver.deadline = deadline;
        loop {
            let outcome = enc.solver.solve();
            report.conflicts = prior.saturating_add(enc.solver.conflicts());
            match outcome {
                SolveResult::Unknown => {
                    report.reason = Some("subspace SAT budget");
                    return report;
                }
                SolveResult::Unsat => {
                    report.patterns_refuted += 1;
                    break;
                }
                SolveResult::Sat => {
                    report.models += 1;
                    if !enc.verify_model() {
                        report.reason = Some("subspace model failed original field/inverse replay");
                        return report;
                    }
                    let xs = enc.decode_summands();
                    if let Some(points) = lift(variety, &xs, target) {
                        if points.iter().fold(BinaryPoint::Infinity, |s, p| {
                            point_add(variety.curve(), &s, p)
                        }) != *target
                        {
                            report.reason = Some("subspace original sum replay failed");
                            return report;
                        }
                        report.points = Some(points);
                        report.disposition = Disposition::Found;
                        return report;
                    }
                    if report.models >= limits.max_models {
                        report.reason = Some("subspace model cap");
                        return report;
                    }
                    // Block only after exhausting every actual +/- lift.
                    let bits = enc.solver.model();
                    enc.solver.reset_search();
                    enc.solver.add_clause(
                        (0..m * variety.curve().m as usize)
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
    report.disposition = Disposition::Refuted;
    report
}
