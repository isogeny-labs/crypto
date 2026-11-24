//! New four-leaf composition of the repository's oriented-pair S3 identity.
//! Uses the unchanged production F4/root visitor, not the RUR backend.
use super::{Base, Discovery, Point, Relation, System};
use crypto_lib::cryptanalysis::f4_fp::{
    solve_visit_traced, F4Options, F4ResourceLimits, Ordering, Poly, VisitEvent,
};
use serde_json::json;
use std::collections::BTreeSet;
use std::time::Instant;

const RESOURCES: F4ResourceLimits = F4ResourceLimits {
    max_pairs: 20_000,
    max_symbolic_monomials: 8_192,
    max_matrix_entries: 2_000_000,
};
const MAX_ASSIGNMENTS: usize = 4_096;

fn univariate(poly: &Poly) -> bool {
    let mut used = BTreeSet::new();
    for (powers, coefficient) in poly {
        if *coefficient != 0 {
            used.extend(
                powers
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| **v != 0)
                    .map(|(i, _)| i),
            );
        }
    }
    used.len() == 1
}

fn nonzero_constant(poly: &Poly) -> bool {
    !poly.is_empty() && poly.iter().all(|(e, _)| e.iter().all(|v| *v == 0))
}

fn replay(base: &Base, target: Point, a: &[u64]) -> Result<Option<Relation>, String> {
    if a.len() < 11 || a.iter().any(|x| *x >= base.curve.p as u64) {
        return Err("invalid oriented assignment".into());
    }
    let p = base.curve.p;
    let v = |i| a[i] as i64;
    let points = [
        Point { x: v(0), y: v(4) },
        Point {
            x: v(1),
            y: (v(4) + v(5) * (v(1) - v(0))).rem_euclid(p),
        },
        Point { x: v(2), y: v(7) },
        Point {
            x: v(3),
            y: (v(7) + v(8) * (v(3) - v(2))).rem_euclid(p),
        },
    ];
    if points.iter().any(|q| q.y * q.y % p != base.curve.rhs(q.x)) {
        return Err("oriented lift is not on the curve".into());
    }
    let left = base.curve.add(Some(points[0]), Some(points[1]));
    let right = base.curve.add(Some(points[2]), Some(points[3]));
    if left.map(|q| q.x) != Some(v(6))
        || right.map(|q| q.x) != Some(v(9))
        || base.curve.add(left, right) != Some(target)
    {
        return Err("oriented assignment failed independent group replay".into());
    }
    let labels = points.map(|point| base.signed.get(&point).copied());
    if labels.iter().any(Option::is_none) {
        return Err("oriented lift is outside the signed base".into());
    }
    let xs: BTreeSet<_> = points.iter().map(|q| q.x).collect();
    if xs.len() != 4 || xs.contains(&target.x) {
        return Ok(None);
    }
    let mut row = labels.map(Option::unwrap);
    row.sort_unstable();
    Ok(Some(row))
}

pub(super) fn discover(base: &Base, target: Point, tower: bool, deadline: Instant) -> Discovery {
    let stopped = |reason: &str| Discovery {
        result: Err(reason.into()),
        planned: 1,
        completed: 0,
        systems: vec![],
    };
    if Instant::now() >= deadline {
        return stopped("oracle budget exhausted");
    }
    if !(3..=5).contains(&base.depth) || base.unsigned.len() > 16 {
        return stopped("oriented development input limit");
    }
    let system = System::oriented(base, target, tower);
    solve(base, target, system, tower, deadline)
}

fn solve(base: &Base, target: Point, system: System, tower: bool, deadline: Instant) -> Discovery {
    let (variables, polynomials, terms, degree) = system.metrics();
    let mut record = json!({
        "backend":"native_oriented_s3_chain", "membership":if tower {"dickson_tower"} else {"exact_base_dense"},
        "scope":"full_distinct_x_target_disjoint", "variables":variables,
        "equations":polynomials, "terms":terms, "degree":degree,
        "input":system.render(base.curve.p), "max_degree":16,
        "max_assignments":MAX_ASSIGNMENTS, "max_construction_terms":20_000,
        "resource_limits":{"pairs":20_000,"symbolic_monomials":8_192,"matrix_entries":2_000_000},
        "accounting":"local_correctness_only"
    });
    let mut out = Discovery {
        result: Err("oracle budget exhausted".into()),
        planned: 1,
        completed: 0,
        systems: vec![],
    };
    if Instant::now() >= deadline || terms > 20_000 {
        record["status"] = json!("construction_limit");
        if terms > 20_000 {
            out.result = Err("oriented input term cap".into());
        }
        out.systems.push(record);
        return out;
    }
    let equations = system.native_equations();
    let mut options = F4Options::new(Ordering::Grevlex, 16);
    options.deadline = Some(deadline);
    options.resource_limits = Some(RESOURCES);
    let mut assignments = vec![];
    let mut output_limit = false;
    let mut refused = 0;
    let traversal = solve_visit_traced(
        &equations,
        variables,
        base.curve.p as u64,
        &options,
        &vec![None; variables],
        |a| {
            if assignments.len() >= MAX_ASSIGNMENTS {
                output_limit = true;
                return true;
            }
            assignments.push(a.to_vec());
            false
        },
        |event| {
            match event {
                VisitEvent::NativeResult {
                    report,
                    generating_set,
                    ..
                } if report.resource_limit.is_none()
                    && !report.timed_out
                    && !generating_set.iter().any(nonzero_constant)
                    && !generating_set.iter().any(univariate) =>
                {
                    refused += 1;
                    return Err("no algebraic univariate; enumeration refused".into());
                }
                VisitEvent::Branch { source, .. } if source != "univariate_roots" => {
                    refused += 1;
                    return Err(format!("forbidden enumeration branch {source}"));
                }
                _ => {}
            }
            Ok(())
        },
    );
    record["assignments_extracted"] = json!(assignments.len());
    record["forbidden_fallbacks_refused"] = json!(refused);
    record["output_limit"] = json!(output_limit);
    let traversal = match traversal {
        Ok(value) => value,
        Err(error) => {
            out.result = Err(error);
            out.systems.push(record);
            return out;
        }
    };
    let complete = traversal.complete
        && !traversal.timed_out
        && !traversal.trace_stopped
        && traversal.resource_limit.is_none()
        && !output_limit
        && Instant::now() < deadline;
    let reason = traversal
        .resource_limit
        .as_ref()
        .map(ToString::to_string)
        .or_else(|| traversal.trace_error.clone())
        .unwrap_or_else(|| {
            if output_limit {
                "assignment cap".into()
            } else if !complete {
                "incomplete algebraic traversal".into()
            } else {
                "complete".into()
            }
        });
    record["traversal"] = json!(traversal);
    if !complete {
        // Retain diagnostics, but never expose a partial relation set as exact.
        out.result = Err(reason);
    } else {
        let mut rows = BTreeSet::new();
        let mut filtered = 0;
        let mut error = None;
        for a in &assignments {
            if Instant::now() >= deadline {
                error = Some("oracle budget exhausted".into());
                break;
            }
            match replay(base, target, a) {
                Ok(Some(row)) => {
                    rows.insert(row);
                }
                Ok(None) => filtered += 1,
                Err(e) => {
                    error = Some(e);
                    break;
                }
            }
        }
        record["outside_requested_chart"] = json!(filtered);
        record["distinct_rows"] = json!(rows.len());
        out.result = error.map_or(Ok(rows), Err);
        out.completed = usize::from(out.result.is_ok());
    }
    if Instant::now() >= deadline {
        out.result = Err("oracle budget exhausted".into());
        out.completed = 0;
    }
    record["status"] = json!(if out.completed == 1 {
        "complete"
    } else {
        "incomplete"
    });
    out.systems.push(record);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dickson, inverse, FIXTURES};
    use std::time::Duration;

    fn assignment(base: &Base, points: [Point; 4], target: Point, tower: bool) -> Vec<i64> {
        let p = base.curve.p;
        let mut a: Vec<_> = points.iter().map(|q| q.x).collect();
        let mut sums = vec![];
        for [left, right] in [[points[0], points[1]], [points[2], points[3]]] {
            let slope = ((right.y - left.y) * inverse(right.x - left.x, p)).rem_euclid(p);
            let sum = base.curve.add(Some(left), Some(right)).unwrap();
            a.extend([left.y, slope, sum.x]);
            sums.push(sum);
        }
        let slope = if sums[0].x == sums[1].x {
            ((3 * sums[0].x * sums[0].x + base.curve.a) * inverse(2 * sums[0].y, p)).rem_euclid(p)
        } else {
            ((sums[1].y - sums[0].y) * inverse(sums[1].x - sums[0].x, p)).rem_euclid(p)
        };
        assert_eq!(base.curve.add(Some(sums[0]), Some(sums[1])), Some(target));
        a.push(slope);
        if tower {
            for point in points {
                a.extend((1..base.depth).map(|j| dickson(point.x, j, p)));
            }
        }
        a
    }

    #[test]
    fn all_f127_reference_rows_and_orderings_bind_both_memberships() {
        let base = Base::new(FIXTURES[0], 4);
        let target = Point { x: 41, y: 95 };
        let rows = base.reference(&base.pair_index(), target);
        assert_eq!(rows.len(), 13);
        for tower in [false, true] {
            let system = System::oriented(&base, target, tower);
            let mut seen = BTreeSet::new();
            for row in &rows {
                for i in 0..4 {
                    for j in 0..4 {
                        for k in 0..4 {
                            for l in 0..4 {
                                if BTreeSet::from([i, j, k, l]).len() != 4 {
                                    continue;
                                }
                                let points = [i, j, k, l].map(|q| base.orient(row[q]));
                                let a = assignment(&base, points, target, tower);
                                assert!(system.satisfied(&a));
                                let raw: Vec<_> = a.iter().map(|v| *v as u64).collect();
                                assert_eq!(replay(&base, target, &raw).unwrap(), Some(*row));
                                seen.insert(a);
                            }
                        }
                    }
                }
            }
            assert_eq!(seen.len(), 24 * rows.len());
            assert!(seen.len() > base.curve.p as usize);
        }
    }

    #[test]
    fn expired_budget_never_builds_oriented_system() {
        let base = Base::new(FIXTURES[0], 4);
        let result = discover(&base, base.curve.q, false, Instant::now());
        assert!(result.result.is_err());
        assert!(result.systems.is_empty());
    }

    #[test]
    fn planted_fixed_slots_use_actual_native_root_visitor() {
        let base = Base::new(FIXTURES[0], 4);
        let target = Point { x: 20, y: 91 };
        let row = *base
            .reference(&base.pair_index(), target)
            .iter()
            .next()
            .unwrap();
        let mut system = System::oriented(&base, target, false);
        system.fix_leaf_x(row.map(|label| base.orient(label).x));
        let result = solve(
            &base,
            target,
            system,
            false,
            Instant::now() + Duration::from_secs(3),
        );
        let expected = base
            .reference(&base.pair_index(), target)
            .into_iter()
            .filter(|r| {
                r.iter().map(|l| l.unsigned_abs()).collect::<BTreeSet<_>>()
                    == row
                        .iter()
                        .map(|l| l.unsigned_abs())
                        .collect::<BTreeSet<_>>()
            })
            .collect();
        assert_eq!(result.result, Ok(expected), "{:?}", result.systems);
        assert_eq!(result.completed, 1);
    }
}
