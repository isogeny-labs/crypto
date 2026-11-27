//! Finite witness and Jacobian audit. This is not a relation extractor.
use super::*;
use cutoff::{images, parameters, root_polynomial, row_reduce, tuples};

fn factor_assignment(base: &Base, xs: [i64; 4]) -> Vec<i64> {
    let mut pools = BTreeMap::<i64, Vec<i64>>::new();
    for x in xs {
        pools
            .entry(dickson(x, base.depth - 3, base.curve.p))
            .or_default()
            .push(x);
    }
    pools
        .values()
        .flat_map(|xs| {
            let mut factor = root_polynomial(xs, base.curve.p);
            factor.pop();
            factor
        })
        .collect()
}

fn rank(mut matrix: Vec<Vec<i64>>, columns: usize, p: i64) -> usize {
    row_reduce(&mut matrix, columns, p).len()
}

pub(super) fn audit(curve: Fixture, depth: usize) -> Result<Value, String> {
    audit_using(curve, depth, false)
}

pub(super) fn audit_liftable(curve: Fixture, depth: usize) -> Result<Value, String> {
    audit_using(curve, depth, true)
}

fn audit_using(curve: Fixture, depth: usize, liftable: bool) -> Result<Value, String> {
    let base = Base::new(curve, depth);
    let charts = cutoff::qualified_catalog(&base)?;
    let fibers = if liftable {
        Some(liftable::fibers(&base)?)
    } else {
        None
    };
    let indexed = charts
        .iter()
        .enumerate()
        .map(|(i, c)| (c.images, i))
        .collect::<BTreeMap<_, _>>();
    if indexed.len() != charts.len() {
        return Err("catalog has duplicate image charts".into());
    }
    let mut counts = vec![0usize; charts.len()];
    let mut checked = vec![0usize; charts.len()];
    let mut old_ranks = vec![[0usize; 4]; charts.len()];
    let mut new_ranks = vec![[0usize; 8]; charts.len()];
    let mut term_ranges = vec![[usize::MAX, 0]; charts.len()];
    let mut degree_max = vec![0u32; charts.len()];
    let (mut identities, mut overlaps) = (0, 0);
    let levels = depth - 3;
    for tuple in tuples(base.unsigned.len()) {
        let xs = tuple.map(|i| base.unsigned[i].x);
        let key = images(&base, xs);
        let index = *indexed.get(&key).ok_or("tuple missing from catalog")?;
        counts[index] += 1;
        let q = root_polynomial(&xs, curve.p);
        let factors = factor_assignment(&base, xs);
        if factors.len() != 4 {
            return Err("factor assignment has the wrong width".into());
        }
        for mask in 0..16 {
            let leaves = std::array::from_fn(|slot| {
                let point = base.unsigned[tuple[slot]];
                if mask & (1 << slot) == 0 {
                    point
                } else {
                    curve.negate(point)
                }
            });
            let target = leaves
                .iter()
                .fold(None, |sum, point| curve.add(sum, Some(*point)));
            let Some(target) = target else {
                identities += 1;
                continue;
            };
            if xs.contains(&target.x) {
                overlaps += 1;
                continue;
            }
            let theta = parameters(curve, target, leaves)?;
            if xs.iter().any(|x| (x + theta[2]).rem_euclid(curve.p) == 0)
                || leaves.iter().any(|point| point.y == 0)
                || system::numeric_quartic(curve, target, theta[0], theta[1], theta[2]) != q
            {
                return Err("invalid RR5 norm or open-chart witness".into());
            }
            let mut assignment = factors.clone();
            assignment.extend(theta);
            let system = match &fibers {
                Some(fibers) => System::fiber_factors(curve, target, key, fibers)?,
                None => System::factorwise(curve, target, levels, key)?,
            };
            if !system.satisfied(&assignment) {
                return Err("factorwise equations reject a valid relation witness".into());
            }
            let jacobian = system.jacobian(&assignment);
            let fiber_rank = rank(
                jacobian[..4].iter().map(|r| r[..4].to_vec()).collect(),
                4,
                curve.p,
            );
            let norm_rank = rank(
                jacobian[4..].iter().map(|r| r[4..].to_vec()).collect(),
                3,
                curve.p,
            );
            let full_rank = rank(jacobian, 7, curve.p);
            if (fiber_rank, norm_rank, full_rank) != (4, 3, 7) {
                return Err(format!(
                    "factorwise differential lost rank: {fiber_rank}/{norm_rank}/{full_rank}"
                ));
            }
            new_ranks[index][full_rank] += 1;
            let (variables, equations, terms, degree) = system.metrics();
            if variables != 7 || equations != 8 || degree > 4 {
                return Err("factorwise constructor exceeded its declared dimensions".into());
            }
            term_ranges[index][0] = term_ranges[index][0].min(terms);
            term_ranges[index][1] = term_ranges[index][1].max(terms);
            degree_max[index] = degree_max[index].max(degree);
            let mut old_assignment = vec![];
            for step in (1..levels).rev() {
                let roots = xs.map(|x| dickson(x, step, curve.p));
                old_assignment.extend(&root_polynomial(&roots, curve.p)[..4]);
            }
            old_assignment.extend(theta);
            let old = System::at_cutoff(curve, target, levels, charts[index].polynomial);
            if !old.satisfied(&old_assignment) {
                return Err("original cutoff rejects the same relation witness".into());
            }
            let old_rank = rank(old.jacobian(&old_assignment), old_assignment.len(), curve.p)
                .checked_sub(4 * levels.saturating_sub(1))
                .ok_or("old graph rank lost")?;
            if old_rank > 3 {
                return Err("invalid old effective rank".into());
            }
            old_ranks[index][old_rank] += 1;
            checked[index] += 1;
        }
    }
    if counts.contains(&0)
        || checked.iter().sum::<usize>() + identities + overlaps
            != 16 * counts.iter().sum::<usize>()
    {
        return Err("factorwise binding accounting mismatch".into());
    }
    let records = charts
        .iter()
        .enumerate()
        .map(|(i, c)| {
            json!({
                "images":c.images,"unsigned_tuples":counts[i],"rr5_bindings_checked":checked[i],
                "old_effective_ranks":old_ranks[i],"factorwise_ranks":new_ranks[i],
                "term_range":term_ranges[i],"maximum_equation_degree":degree_max[i]
            })
        })
        .collect::<Vec<_>>();
    let mut result = json!({
        "type":"factorwise_fiber_audit","format":1,"prime":curve.p,"depth":depth,
        "base_size":base.unsigned.len(),"catalog_charts":charts.len(),"levels":levels,
        "unsigned_tuples":counts.iter().sum::<usize>(),"signed_bindings":16*counts.iter().sum::<usize>(),
        "identity_exclusions":identities,"target_x_exclusions":overlaps,
        "rr5_bindings_checked":checked.iter().sum::<usize>(),
        "factor_variables":4,"rr5_variables":3,"equations":8,
        "required_fiber_rank":4,"required_rr5_rank":3,"required_full_rank":7,
        "charts":records,"solver_calls":0,
        "scope":"finite witness and actual-equation Jacobian check; no relation collection or cost measurement"
    });
    if let Some(fibers) = fibers {
        result["type"] = json!("liftable_fiber_audit");
        result["fibers"] =
            json!(fibers.iter().map(|(b, polynomial)| json!({
            "image":b,"polynomial":polynomial,
            "roots":base.unsigned.iter().filter(|point| dickson(point.x, levels, curve.p) == *b)
                .map(|point| point.x).collect::<Vec<_>>()
        })).collect::<Vec<_>>());
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorwise_rejects_bad_chart_shapes() {
        let curve = FIXTURES[0];
        for (levels, images) in [
            (0, [1, 2, 3, 4]),
            (3, [1, 2, 3, 4]),
            (1, [1, 1, 1, 2]),
            (1, [2, 1, 3, 4]),
            (1, [-1, 1, 2, 3]),
        ] {
            assert!(System::factorwise(curve, curve.q, levels, images).is_err());
        }
    }

    #[test]
    fn repeated_image_factorization_has_full_tangent_rank_and_rejects_mutation() {
        let curve = FIXTURES[0];
        let base = Base::new(curve, 4);
        let tuple = tuples(base.unsigned.len())
            .find(|tuple| {
                let key = images(&base, tuple.map(|i| base.unsigned[i].x));
                key == [7, 7, 82, 82]
            })
            .expect("retained two-image chart exists");
        let xs = tuple.map(|i| base.unsigned[i].x);
        let leaves = tuple.map(|i| base.unsigned[i]);
        let target = leaves
            .iter()
            .fold(None, |sum, point| curve.add(sum, Some(*point)))
            .unwrap();
        assert!(!xs.contains(&target.x));
        let theta = parameters(curve, target, leaves).unwrap();
        let mut binding = factor_assignment(&base, xs);
        binding.extend(theta);
        let system = System::factorwise(curve, target, 1, [7, 7, 82, 82]).unwrap();
        assert!(system.satisfied(&binding));
        assert_eq!(rank(system.jacobian(&binding), 7, curve.p), 7);
        let old = System::at_cutoff(
            curve,
            target,
            1,
            root_polynomial(&[7, 7, 82, 82], curve.p)[..4]
                .try_into()
                .unwrap(),
        );
        assert_eq!(rank(old.jacobian(&theta), 3, curve.p), 2);
        for i in 0..7 {
            let mut altered = binding.clone();
            altered[i] = (altered[i] + 1) % curve.p;
            assert!(
                !system.satisfied(&altered),
                "coordinate {i} mutation admitted"
            );
        }
    }
}
