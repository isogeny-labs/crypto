//! Target-independent fine-image multiset catalog and finite qualification.
//! This module does not collect relations or call an algebraic solver.
use super::*;

fn root_polynomial(roots: &[i64], p: i64) -> Vec<i64> {
    roots
        .iter()
        .fold(vec![1], |q, x| polynomial::product(&q, &[-x, 1], p))
}

fn images(base: &Base, xs: [i64; 4]) -> [i64; 4] {
    let mut result = xs.map(|x| dickson(x, base.depth - 3, base.curve.p));
    result.sort_unstable();
    result
}

fn pools(base: &Base) -> BTreeMap<i64, usize> {
    let mut result = BTreeMap::new();
    for point in &base.unsigned {
        *result
            .entry(dickson(point.x, base.depth - 3, base.curve.p))
            .or_default() += 1;
    }
    result
}

/// Enumerate multiplicity vectors, never four-element subsets of the base.
fn catalog(p: i64, capacities: &BTreeMap<i64, usize>) -> Vec<Refinement> {
    fn extend(p: i64, pools: &[(i64, usize)], chosen: &mut Vec<i64>, result: &mut Vec<Refinement>) {
        if chosen.len() == 4 {
            let q = root_polynomial(chosen, p);
            result.push(Refinement {
                images: chosen.clone().try_into().unwrap(),
                polynomial: q[..4].try_into().unwrap(),
            });
            return;
        }
        let Some((&(image, capacity), rest)) = pools.split_first() else {
            return;
        };
        let start = chosen.len();
        for count in 0..=capacity.min(4 - start) {
            chosen.resize(start + count, image);
            extend(p, rest, chosen, result);
        }
        chosen.truncate(start);
    }
    let mut result = vec![];
    extend(
        p,
        &capacities.iter().map(|(b, n)| (*b, *n)).collect::<Vec<_>>(),
        &mut vec![],
        &mut result,
    );
    result.sort_by_key(|chart| chart.images);
    result
}

pub(super) fn qualified_catalog(base: &Base) -> Result<Vec<Refinement>, String> {
    if ![(127, 4), (8191, 4), (8191, 5)].contains(&(base.curve.p, base.depth)) {
        return Err("full cutoff is limited to its three qualified fixture/depth inputs".into());
    }
    Ok(catalog(base.curve.p, &pools(base)))
}

fn tuples(n: usize) -> impl Iterator<Item = [usize; 4]> {
    (0..n).flat_map(move |a| {
        (a + 1..n)
            .flat_map(move |b| (b + 1..n).flat_map(move |c| (c + 1..n).map(move |d| [a, b, c, d])))
    })
}

fn catalog_check(base: &Base, charts: &[Refinement]) -> Result<Vec<usize>, String> {
    let mut indexed = BTreeMap::new();
    for (index, chart) in charts.iter().enumerate() {
        let q = root_polynomial(&chart.images, base.curve.p);
        if q[..4] != chart.polynomial
            || !chart.images.windows(2).all(|pair| pair[0] <= pair[1])
            || indexed.insert(chart.images, index).is_some()
        {
            return Err("duplicate or malformed cutoff chart".into());
        }
    }
    let mut counts = vec![0; charts.len()];
    for tuple in tuples(base.unsigned.len()) {
        let key = images(base, tuple.map(|i| base.unsigned[i].x));
        let index = indexed.get(&key).ok_or("missing cutoff chart")?;
        counts[*index] += 1;
    }
    if counts.contains(&0) {
        return Err("cutoff chart has no distinct-abscissa tuple".into());
    }
    Ok(counts)
}

fn row_reduce(matrix: &mut [Vec<i64>], columns: usize, p: i64) -> Vec<usize> {
    let mut pivots = vec![];
    for column in 0..columns {
        let row = pivots.len();
        let Some(pivot) = (row..matrix.len()).find(|&i| matrix[i][column] != 0) else {
            continue;
        };
        matrix.swap(row, pivot);
        let scale = inverse(matrix[row][column], p);
        for value in &mut matrix[row] {
            *value = (*value * scale).rem_euclid(p);
        }
        for i in 0..matrix.len() {
            if i == row {
                continue;
            }
            let multiplier = matrix[i][column];
            for j in 0..matrix[i].len() {
                matrix[i][j] = (matrix[i][j] - multiplier * matrix[row][j]).rem_euclid(p);
            }
        }
        pivots.push(column);
    }
    pivots
}

/// Fit the unique normalized RR5 function from the signed points, without logs.
fn parameters(curve: Fixture, target: Point, leaves: [Point; 4]) -> Result<[i64; 3], String> {
    let mut matrix: Vec<_> = leaves
        .iter()
        .map(|point| {
            let dx = point.x - target.x;
            let dy = point.y + target.y;
            [dx, point.x * dx, dy, -point.x * dy]
                .map(|v| v.rem_euclid(curve.p))
                .to_vec()
        })
        .collect();
    let pivots = row_reduce(&mut matrix, 3, curve.p);
    if pivots != [0, 1, 2] || matrix[3][3] != 0 {
        return Err("RR5 interpolation is inconsistent or nonunique".into());
    }
    Ok([matrix[0][3], matrix[1][3], matrix[2][3]])
}

pub(super) fn audit(curve: Fixture, depth: usize) -> Result<Value, String> {
    if ![(127, 4), (8191, 4), (8191, 5)].contains(&(curve.p, depth)) {
        return Err("partition audit is limited to its three frozen public fixtures".into());
    }
    let base = Base::new(curve, depth);
    let capacities = pools(&base);
    let charts = catalog(curve.p, &capacities);
    let counts = catalog_check(&base, &charts)?;
    let indexed: BTreeMap<_, _> = charts
        .iter()
        .enumerate()
        .map(|(i, chart)| (chart.images, i))
        .collect();
    let mut ranks = vec![[0usize; 4]; charts.len()];
    let (mut identity, mut overlap, mut checked) = (0, 0, 0);
    let levels = depth - 3;
    let graph_variables = 4 * levels.saturating_sub(1);
    for tuple in tuples(base.unsigned.len()) {
        let xs = tuple.map(|i| base.unsigned[i].x);
        let key = images(&base, xs);
        let index = indexed[&key];
        let chart = &charts[index];
        let q = root_polynomial(&xs, curve.p);
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
                identity += 1;
                continue;
            };
            if xs.contains(&target.x) {
                overlap += 1;
                continue;
            }
            let [alpha, beta, t] = parameters(curve, target, leaves)?;
            if xs.iter().any(|x| (x + t).rem_euclid(curve.p) == 0)
                || system::numeric_quartic(curve, target, alpha, beta, t) != q
            {
                return Err("RR5 denominator or norm-root mismatch".into());
            }
            let mut assignment = vec![];
            for step in (1..levels).rev() {
                let roots = xs.map(|x| dickson(x, step, curve.p));
                assignment.extend(&root_polynomial(&roots, curve.p)[..4]);
            }
            assignment.extend([alpha, beta, t]);
            let system = System::at_cutoff(curve, target, levels, chart.polynomial);
            if !system.satisfied(&assignment) {
                return Err("a relation witness fails its cutoff system".into());
            }
            let mut jacobian = system.jacobian(&assignment);
            let rank = row_reduce(&mut jacobian, assignment.len(), curve.p)
                .len()
                .checked_sub(graph_variables)
                .ok_or("graph equations lost rank")?;
            let distinct = key.into_iter().collect::<BTreeSet<_>>().len();
            if rank > 3.min(distinct) || (distinct == 4 && rank != 3) {
                return Err("cutoff differential rank violates its bound".into());
            }
            ranks[index][rank] += 1;
            checked += 1;
        }
    }
    let tuples: usize = counts.iter().sum();
    if checked + identity + overlap != 16 * tuples {
        return Err("signed binding accounting mismatch".into());
    }
    let regular = base.refinements();
    if regular.iter().any(|chart| {
        let mut key = chart.images;
        key.sort_unstable();
        !indexed.contains_key(&key)
    }) {
        return Err("existing regular chart missing from full catalog".into());
    }
    let records: Vec<_> = charts
        .iter()
        .enumerate()
        .map(|(i, chart)| {
            json!({"images":chart.images,"coefficients":chart.polynomial,
                "unsigned_tuples":counts[i],"effective_jacobian_ranks":ranks[i]})
        })
        .collect();
    Ok(json!({
        "type":"cutoff_partition_audit","format":1,"prime":curve.p,"depth":depth,
        "curve":{"a":curve.a,"b":curve.b,"order":curve.n},
        "selector_size":1usize << depth,"terminal":base.terminal,
        "base_size":base.unsigned.len(),"levels":levels,"variables_per_chart":graph_variables+3,
        "capacities":capacities.into_iter().map(|(b,n)| json!([b,n])).collect::<Vec<_>>(),
        "regular_charts":regular.len(),"full_charts":charts.len(),"unsigned_tuples":tuples,
        "signed_bindings":16*tuples,"identity_exclusions":identity,"target_x_exclusions":overlap,
        "rr5_bindings_checked":checked,"charts":records,
        "solver_calls":0,"scope":"finite catalog and witness qualification; no cost measurement"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacities_keep_repeated_images_and_forbid_overuse() {
        let charts = catalog(127, &BTreeMap::from([(2, 4), (3, 2)]));
        let keys: Vec<_> = charts.iter().map(|c| c.images).collect();
        assert_eq!(keys, [[2, 2, 2, 2], [2, 2, 2, 3], [2, 2, 3, 3]]);
        assert!(catalog(127, &BTreeMap::from([(2, 1), (3, 2)])).is_empty());
        let repeated = &charts[0];
        assert_eq!(
            root_polynomial(&[2, 2, 2, 2], 127)[..4],
            repeated.polynomial
        );
    }

    #[test]
    fn full_catalog_partitions_every_unsigned_tuple_and_rejects_mutations() {
        for (curve, depth) in [(FIXTURES[0], 4), (FIXTURES[1], 4), (FIXTURES[1], 5)] {
            let base = Base::new(curve, depth);
            let charts = catalog(curve.p, &pools(&base));
            let counts = catalog_check(&base, &charts).unwrap();
            assert_eq!(
                counts.iter().sum::<usize>(),
                tuples(base.unsigned.len()).count()
            );
            let mut missing = charts.clone();
            missing.pop();
            assert_eq!(
                catalog_check(&base, &missing).unwrap_err(),
                "missing cutoff chart"
            );
            let mut duplicate = charts.clone();
            duplicate.push(charts[0].clone());
            assert!(catalog_check(&base, &duplicate).is_err());
            let mut changed = charts;
            changed[0].polynomial[0] = (changed[0].polynomial[0] + 1) % curve.p;
            assert!(catalog_check(&base, &changed).is_err());
        }
    }

    #[test]
    fn audit_refuses_unregistered_fixture_depth() {
        assert!(audit(FIXTURES[0], 3).is_err());
        assert!(audit(FIXTURES[0], 5).is_err());
    }
}
