//! Bounded adapter to the unchanged Isogeny Labs monic-cofactor extractor.
//! No reference pair index or target scalar enters the production call.

use super::{Base, Discovery, Point, Relation};
use crypto_lib::cryptanalysis::f4_fp::F4ResourceLimits;
use prime31_algebraic_ic::Curve;
use prime31_four_summand::extractor::{extract_cofactor, Limits, ResultRow, Status};
use prime31_four_summand::relation_adapter::SignedRelation;
use serde_json::json;
use std::collections::BTreeSet;
use std::time::{Duration, Instant};

const RESOURCES: F4ResourceLimits = F4ResourceLimits {
    max_pairs: 20_000,
    max_symbolic_monomials: 8_192,
    max_matrix_entries: 2_000_000,
};

fn convert(base: &Base, target: Point, relation: &SignedRelation) -> Result<Relation, String> {
    if relation.terms.len() != 4 {
        return Err("production row must have four distinct signed leaves".into());
    }
    let mut labels = [0; 4];
    let mut indices = BTreeSet::new();
    let mut sum = None;
    for (label, &(index, sign)) in labels.iter_mut().zip(&relation.terms) {
        if index >= base.unsigned.len() || !matches!(sign, -1 | 1) || !indices.insert(index) {
            return Err("invalid production index, multiplicity or repeated leaf".into());
        }
        *label = (index as i32 + 1) * sign as i32;
        let point = base.orient(*label);
        if point.x == target.x {
            return Err("production row leaves the target-disjoint chart".into());
        }
        sum = base.curve.add(sum, Some(point));
    }
    if sum != Some(target) {
        return Err("production row failed separate group replay".into());
    }
    labels.sort_unstable();
    Ok(labels)
}

fn accepted_rows(
    base: &Base,
    target: Point,
    native: &ResultRow,
) -> Result<BTreeSet<Relation>, String> {
    if !matches!(native.status, Status::Complete | Status::Empty) {
        return Err(format!("production {:?}: {}", native.status, native.reason));
    }
    if native.status == Status::Empty && !native.relations.is_empty() {
        return Err("production Empty status contains rows".into());
    }
    if native.diagnostics.forbidden_domain_evaluations != 0 {
        return Err("production call executed forbidden domain evaluations".into());
    }
    native
        .relations
        .iter()
        .map(|relation| convert(base, target, relation))
        .collect()
}

pub(super) fn discover(
    base: &Base,
    target: Point,
    deadline: Instant,
    budget: Duration,
) -> Discovery {
    let mut outcome = Discovery {
        result: Err("oracle budget exhausted".into()),
        planned: 1,
        completed: 0,
        systems: vec![],
    };
    if Instant::now() >= deadline || budget.is_zero() {
        return outcome;
    }
    let e = base.curve;
    let curve = Curve {
        p: e.p as u64,
        a: e.a as u64,
        b: e.b as u64,
        n: e.n as u64,
        g: (e.g.x as u64, e.g.y as u64),
    };
    let points: Vec<_> = base
        .unsigned
        .iter()
        .map(|point| (point.x as u64, point.y as u64))
        .collect();
    let native = extract_cofactor(
        curve,
        &points,
        Some((target.x as u64, target.y as u64)),
        Limits {
            max_degree: 10,
            // The existing API takes whole seconds and starts its deadline
            // after validation. The enclosing absolute deadline still governs
            // acceptance, including conversion and replay. This is cooperative.
            seconds: budget.as_secs().max(1),
            max_polynomial_terms: 20_000,
            max_assignments: 4_096,
        },
        RESOURCES,
    );
    let rows = accepted_rows(base, target, &native);
    let regular: Option<Vec<_>> = rows.as_ref().ok().map(|rows| {
        rows.iter()
            .filter(|row| base.separated(row))
            .copied()
            .collect()
    });
    outcome.systems.push(json!({
        "backend":"isogeny_labs_extract_cofactor",
        "scope":"full_distinct_x_target_disjoint_rr5",
        "resource_limits":RESOURCES,
        "native":native,
        "matched_regular_rows":regular,
        "accounting":"local_correctness_only",
    }));
    if Instant::now() < deadline {
        outcome.completed = usize::from(rows.is_ok());
        outcome.result = rows;
    }
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FIXTURES;
    use prime31_four_summand::extractor::Diagnostics;

    fn fixture() -> (Base, Point, SignedRelation) {
        let base = Base::new(FIXTURES[0], 4);
        let target = Point { x: 20, y: 91 };
        // The pair index is used only by this test to supply expected rows.
        let rows = base.reference(&base.pair_index(), target);
        let row = *rows.first().expect("existing positive fixture");
        let relation = SignedRelation {
            terms: row
                .iter()
                .map(|label| (label.unsigned_abs() as usize - 1, label.signum() as i64))
                .collect(),
        };
        (base, target, relation)
    }

    #[test]
    fn conversion_replays_signs_and_rejects_invalid_rows() {
        let (base, target, relation) = fixture();
        assert!(convert(&base, target, &relation).is_ok());
        let mut altered = relation.clone();
        altered.terms[0].1 *= -1;
        assert!(convert(&base, target, &altered).is_err());
        altered = relation.clone();
        altered.terms[0].0 = base.unsigned.len();
        assert!(convert(&base, target, &altered).is_err());
        altered = relation.clone();
        altered.terms[0] = altered.terms[1];
        assert!(convert(&base, target, &altered).is_err());
        altered = relation.clone();
        altered.terms[0].1 = 2;
        assert!(convert(&base, target, &altered).is_err());
        altered = relation;
        altered.terms.pop();
        assert!(convert(&base, target, &altered).is_err());
    }

    #[test]
    fn incomplete_or_enumerated_output_never_becomes_exact() {
        let (base, target, relation) = fixture();
        let mut native = ResultRow {
            status: Status::Complete,
            reason: String::new(),
            relations: vec![relation],
            diagnostics: Diagnostics::default(),
            traversal: None,
        };
        assert_eq!(accepted_rows(&base, target, &native).unwrap().len(), 1);
        for status in [
            Status::Inconclusive,
            Status::Deadline,
            Status::ResourceLimit,
            Status::OutputLimit,
            Status::UnsupportedChart,
            Status::Empty,
        ] {
            native.status = status;
            assert!(accepted_rows(&base, target, &native).is_err());
        }
        native.status = Status::Complete;
        native.diagnostics.forbidden_domain_evaluations = 1;
        assert!(accepted_rows(&base, target, &native).is_err());
    }

    #[test]
    fn zero_budget_does_not_call_production() {
        let (base, target, _) = fixture();
        let result = discover(&base, target, Instant::now(), Duration::ZERO);
        assert!(result.result.is_err());
        assert!(result.systems.is_empty());
        assert_eq!(result.completed, 0);
    }
}
