//! Bounded adapter to the unchanged Isogeny Labs monic-cofactor extractor.
//! No reference pair index or target scalar enters the production call.

use super::{Base, Discovery, Point, Relation, Scratch};
use crypto_lib::cryptanalysis::f4_fp::F4ResourceLimits;
use prime31_algebraic_ic::Curve;
use prime31_four_summand::extractor::{
    extract_cofactor, prepare_cofactor, Limits, PreparedCofactor, ResultRow, Status,
};
use prime31_four_summand::relation_adapter::SignedRelation;
use serde_json::json;
use std::collections::BTreeSet;
use std::path::Path;
use std::time::{Duration, Instant};

const RESOURCES: F4ResourceLimits = F4ResourceLimits {
    max_pairs: 20_000,
    max_symbolic_monomials: 8_192,
    max_matrix_entries: 2_000_000,
};

fn public_input(base: &Base) -> (Curve, Vec<(u64, u64)>) {
    let e = base.curve;
    (
        Curve {
            p: e.p as u64,
            a: e.a as u64,
            b: e.b as u64,
            n: e.n as u64,
            g: (e.g.x as u64, e.g.y as u64),
        },
        base.unsigned
            .iter()
            .map(|point| (point.x as u64, point.y as u64))
            .collect(),
    )
}

fn construction_limits(budget: Duration) -> Limits {
    Limits {
        max_degree: 10,
        seconds: budget.as_secs().max(1),
        max_polynomial_terms: 20_000,
        max_assignments: 4_096,
    }
}

fn render(prepared: &PreparedCofactor) -> String {
    // Native parameters b,c,d mean t0,a1,a0 in the existing RUR extractor.
    let mut names = vec!["t0".to_string(), "a1".into(), "a0".into()];
    names.extend((3..prepared.variables).map(|index| format!("w{}", index - 3)));
    let polynomials: Vec<_> = prepared
        .equations
        .iter()
        .map(|polynomial| {
            if polynomial.is_empty() {
                return "0".to_string();
            }
            polynomial
                .iter()
                .map(|(exponents, coefficient)| {
                    assert_eq!(exponents.len(), names.len());
                    let mut term = coefficient.to_string();
                    for (name, exponent) in names.iter().zip(exponents) {
                        if *exponent != 0 {
                            term.push('*');
                            term.push_str(name);
                            if *exponent != 1 {
                                term.push_str(&format!("^{exponent}"));
                            }
                        }
                    }
                    term
                })
                .collect::<Vec<_>>()
                .join("+")
        })
        .collect();
    format!(
        "{},A\n{}\n{}",
        names.join(","),
        prepared.prime,
        polynomials.join(",\n")
    )
}

pub(super) fn discover_msolve(
    base: &Base,
    target: Point,
    solver: &Path,
    scratch: &Scratch,
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
    let (curve, points) = public_input(base);
    let prepared = match prepare_cofactor(
        curve,
        &points,
        Some((target.x as u64, target.y as u64)),
        construction_limits(budget),
    ) {
        Ok(prepared) => prepared,
        Err(native) => {
            if Instant::now() < deadline {
                outcome.result = accepted_rows(base, target, &native);
                outcome.completed = usize::from(outcome.result.is_ok());
            }
            outcome.systems.push(
                json!({"backend":"production_equations_msolve", "preparation_result":native}),
            );
            return outcome;
        }
    };
    let input_prefix = render(&prepared);
    let mut record = json!({
        "backend":"production_equations_msolve",
        "format_version":2,
        "scope":"full_distinct_x_target_disjoint_rr5",
        "prepared":prepared,
        "input_prefix":input_prefix,
        "solver_variables":prepared.variables+1,
        "solver_equations":prepared.equations.len()+1,
        "solver_args":["-t","1","-c","0"],
        "form_limit":16,
        "rur_capture_limit_bytes":262_144,
        "attempts":[],
        "accounting":"local_correctness_only",
    });
    let mut rows = Err("separating-form allowance exhausted".to_string());
    let mut captured_bytes = 0;
    // The fixed target-independent family is A=t0+j*a1+j^2*a0.
    // Preparation and every attempted form share the caller's one deadline.
    for j in 0..16i64 {
        if Instant::now() >= deadline {
            break;
        }
        let p = base.curve.p;
        let linear = format!(
            "1*A+{}*t0+{}*a1+{}*a0",
            p - 1,
            (-j).rem_euclid(p),
            (-j * j).rem_euclid(p)
        );
        let input = format!("{input_prefix},\n{linear}\n");
        let mut attempt = json!({"form":j,"linear_equation":linear});
        let candidate = match super::run_solver(solver, &input, deadline, scratch, &["-c", "0"]) {
            Ok(raw) => {
                attempt["rur_bytes"] = json!(raw.len());
                if captured_bytes + raw.len() <= 262_144 {
                    captured_bytes += raw.len();
                    attempt["rur"] = json!(raw);
                } else {
                    attempt["rur_capture"] = json!("omitted_query_capture_cap");
                }
                super::extract(&raw, base, target, super::Mode::Quadratic)
            }
            Err(error) => Err(error),
        };
        let another_form = candidate.as_ref().err().is_some_and(|error| {
            error.starts_with("nonseparating RUR:")
                || error == "nonseparating staircase (msolve -c 0)"
        });
        attempt["status"] = json!(if candidate.is_ok() {
            "complete"
        } else {
            "incomplete"
        });
        attempt["reason"] = json!(candidate.as_ref().err());
        record["attempts"].as_array_mut().unwrap().push(attempt);
        rows = candidate;
        if !another_form {
            break;
        }
    }
    record["matched_regular_rows"] = json!(rows.as_ref().ok().map(|rows| rows
        .iter()
        .filter(|row| base.separated(row))
        .copied()
        .collect::<Vec<_>>()));
    outcome.systems.push(record);
    if Instant::now() < deadline {
        outcome.completed = usize::from(rows.is_ok());
        outcome.result = rows;
    }
    outcome
}

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
    let (curve, points) = public_input(base);
    let native = extract_cofactor(
        curve,
        &points,
        Some((target.x as u64, target.y as u64)),
        // The original API uses whole seconds after validation. The enclosing
        // absolute deadline still governs acceptance, including signed replay.
        construction_limits(budget),
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
    fn only_known_separation_failures_advance_the_form() {
        use std::os::unix::fs::PermissionsExt;
        let (base, target, _) = fixture();
        let scratch = Scratch::new().unwrap();
        let solver = scratch.0.join("fake-msolve");
        std::fs::write(&solver, r#"#!/bin/sh
if [ ! -f "$0.called" ]; then
  : > "$0.called"
  : > "$4"
  printf '%s\n' '=====> Computation failed <=====' 'Try to add a random linear form with a new variable' >&2
  exit 0
fi
exit 9
"#).unwrap();
        std::fs::set_permissions(&solver, std::fs::Permissions::from_mode(0o700)).unwrap();
        let budget = Duration::from_secs(3);
        let result = discover_msolve(
            &base,
            target,
            &solver,
            &scratch,
            Instant::now() + budget,
            budget,
        );
        assert!(result.result.unwrap_err().starts_with("solver exit"));
        assert_eq!(result.completed, 0);
        let attempts = result.systems[0]["attempts"].as_array().unwrap();
        assert_eq!(attempts.len(), 2);
        assert_eq!(
            attempts[0]["reason"],
            "nonseparating staircase (msolve -c 0)"
        );
    }

    #[test]
    fn retained_partial_rur_is_rejected_before_signed_extraction() {
        let record: serde_json::Value = serde_json::from_str(
            include_str!("../../evidence/cofactor_diagnostic_v1/f127_d4.jsonl")
                .lines()
                .nth(1)
                .unwrap(),
        )
        .unwrap();
        let base = Base::new(FIXTURES[0], 4);
        let error = crate::extract(
            record["systems"][0]["rur"].as_str().unwrap(),
            &base,
            Point { x: 116, y: 69 },
            crate::Mode::Quadratic,
        )
        .unwrap_err();
        assert_eq!(
            error,
            "nonseparating RUR: eliminant degree 9 differs from quotient dimension 10"
        );
    }

    #[test]
    fn exported_equations_bind_to_independent_numeric_quartic() {
        for (curve, depth, target) in [
            (FIXTURES[0], 4, Point { x: 20, y: 91 }),
            (FIXTURES[1], 5, Point { x: 792, y: 883 }),
        ] {
            let base = Base::new(curve, depth);
            let (native_curve, points) = public_input(&base);
            let prepared = prepare_cofactor(
                native_curve,
                &points,
                Some((target.x as u64, target.y as u64)),
                construction_limits(Duration::from_secs(3)),
            )
            .unwrap();
            assert_eq!(prepared.variables, base.unsigned.len() - 1);
            assert_eq!(prepared.equations.len(), base.unsigned.len());
            let input = render(&prepared);
            assert!(input.starts_with("t0,a1,a0,w0,"));
            let evaluate = |poly: &crypto_lib::cryptanalysis::f4_fp::Poly, assignment: &[i64]| {
                poly.iter().fold(0i64, |sum, (powers, coefficient)| {
                    let term = powers.iter().zip(assignment).fold(
                        *coefficient as i64,
                        |value, (exponent, coordinate)| {
                            value * crate::polynomial::pow(*coordinate, *exponent as i64, curve.p)
                                % curve.p
                        },
                    );
                    (sum + term).rem_euclid(curve.p)
                })
            };
            let h = base.unsigned.iter().fold(vec![1], |h, point| {
                crate::polynomial::product(&h, &[-point.x, 1], curve.p)
            });
            for alpha in [0, 1, curve.p - 1] {
                for beta in [0, 1, curve.p - 1] {
                    for t in [0, 1, curve.p - 1] {
                        let mut assignment = vec![t, beta, alpha];
                        assignment
                            .extend((3..prepared.variables).map(|i| (3 * i as i64 + 2) % curve.p));
                        let numeric = crate::system::numeric_quartic(curve, target, alpha, beta, t);
                        assert_eq!(
                            prepared
                                .quartic
                                .iter()
                                .map(|poly| evaluate(poly, &assignment))
                                .collect::<Vec<_>>(),
                            numeric
                        );
                        let mut w = assignment[3..].to_vec();
                        w.push(1);
                        let product = crate::polynomial::product(&numeric, &w, curve.p);
                        let expected: Vec<_> = product
                            .iter()
                            .zip(&h)
                            .take(h.len() - 1)
                            .map(|(a, b)| (a - b).rem_euclid(curve.p))
                            .zip(&prepared.equation_multipliers)
                            .map(|(value, scale)| value * *scale as i64 % curve.p)
                            .collect();
                        let actual: Vec<_> = prepared
                            .equations
                            .iter()
                            .map(|poly| evaluate(poly, &assignment))
                            .collect();
                        assert_eq!(actual, expected);
                    }
                }
            }
        }
    }

    #[test]
    fn preparation_preserves_invalid_and_unsupported_boundaries() {
        let (base, target, _) = fixture();
        let (curve, mut points) = public_input(&base);
        let limits = construction_limits(Duration::from_secs(3));
        assert_eq!(
            prepare_cofactor(curve, &points, None, limits)
                .unwrap_err()
                .status,
            Status::UnsupportedChart
        );
        assert_eq!(
            prepare_cofactor(curve, &points, Some(points[0]), limits)
                .unwrap_err()
                .status,
            Status::UnsupportedChart
        );
        points[1] = points[0];
        assert_eq!(
            prepare_cofactor(
                curve,
                &points,
                Some((target.x as u64, target.y as u64)),
                limits
            )
            .unwrap_err()
            .status,
            Status::InvalidInput
        );
    }

    #[test]
    fn expired_external_query_never_starts_solver() {
        let (base, target, _) = fixture();
        let scratch = Scratch::new().unwrap();
        let got = discover_msolve(
            &base,
            target,
            Path::new("/must-not-be-run"),
            &scratch,
            Instant::now(),
            Duration::ZERO,
        );
        assert!(got.result.is_err());
        assert!(got.systems.is_empty());
        assert_eq!(got.completed, 0);
        assert!(scratch.0.read_dir().unwrap().next().is_none());
    }

    #[test]
    fn real_production_output_matches_planted_four_leaf_reference() {
        let (base, target, relation) = fixture();
        let mut points: Vec<_> = relation
            .terms
            .iter()
            .map(|&(index, _)| base.unsigned[index])
            .collect();
        points.sort_unstable();
        let signed = points
            .iter()
            .enumerate()
            .flat_map(|(index, &point)| {
                let label = index as i32 + 1;
                [(point, label), (base.curve.negate(point), -label)]
            })
            .collect();
        // This deliberately planted sub-base is a positive integration test,
        // not the complete Dickson base or a substitute for its failed panel.
        let reduced = Base {
            unsigned: points,
            signed,
            ..base
        };
        let expected = reduced.reference(&reduced.pair_index(), target);
        assert!(!expected.is_empty());
        let budget = Duration::from_secs(3);
        let actual = discover(&reduced, target, Instant::now() + budget, budget);
        assert_eq!(actual.completed, 1);
        assert_eq!(actual.result.unwrap(), expected);
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
