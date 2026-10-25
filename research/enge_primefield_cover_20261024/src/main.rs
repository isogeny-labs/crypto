//! Exhaustive rational linear-section sieve on a public F_101 cover fixture.
//! This is a bounded relation-source experiment, not Enge's Jacobian algorithm.

use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

const P: i64 = 101;
const ORDER: i64 = 109;
const G: Point = Point { x: 2, y: 37 };
const Q: Point = Point { x: 64, y: 19 };

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}
type Row = BTreeMap<Point, i64>;

fn m(x: i64, modulus: i64) -> i64 {
    x.rem_euclid(modulus)
}
fn pow(mut x: i64, mut n: usize, modulus: i64) -> i64 {
    let mut result = 1;
    while n > 0 {
        if n & 1 == 1 {
            result = m(result * x, modulus);
        }
        x = m(x * x, modulus);
        n >>= 1;
    }
    result
}
fn inv(x: i64, modulus: i64) -> i64 {
    assert_ne!(m(x, modulus), 0);
    pow(m(x, modulus), (modulus - 2) as usize, modulus)
}
fn f(x: i64) -> i64 {
    m(x * x * x + x + 46, P)
}
fn add(left: Option<Point>, right: Option<Point>) -> Option<Point> {
    let (Some(a), Some(b)) = (left, right) else {
        return left.or(right);
    };
    if a.x == b.x && m(a.y + b.y, P) == 0 {
        return None;
    }
    let slope = if a == b {
        m((3 * a.x * a.x + 1) * inv(2 * a.y, P), P)
    } else {
        m((b.y - a.y) * inv(b.x - a.x, P), P)
    };
    let x = m(slope * slope - a.x - b.x, P);
    Some(Point {
        x,
        y: m(slope * (a.x - x) - a.y, P),
    })
}
fn mul(mut point: Option<Point>, mut n: i64) -> Option<Point> {
    let mut result = None;
    while n > 0 {
        if n & 1 == 1 {
            result = add(result, point);
        }
        point = add(point, point);
        n >>= 1;
    }
    result
}
fn row(points: &[Point]) -> Row {
    let mut result = Row::new();
    for &p in points {
        *result.entry(p).or_default() += 1;
    }
    result
}
fn signed(input: &Row) -> Row {
    let mut result = Row::new();
    for (&point, &count) in input {
        let inverse = Point {
            x: point.x,
            y: m(-point.y, P),
        };
        let canonical = point.min(inverse);
        *result.entry(canonical).or_default() += if point == canonical { count } else { -count };
    }
    result.retain(|_, count| m(*count, ORDER) != 0);
    result
}
fn columns(rows: &[Row]) -> Vec<Point> {
    let mut points: BTreeSet<Point> = rows.iter().flat_map(|r| r.keys().copied()).collect();
    points.insert(G);
    points.insert(Q);
    points.into_iter().collect()
}
fn rank(rows: &[Row], cols: &[Point]) -> usize {
    let index: BTreeMap<Point, usize> = cols.iter().enumerate().map(|(i, &p)| (p, i)).collect();
    let mut pivots: BTreeMap<usize, Vec<i64>> = BTreeMap::new();
    for row in rows {
        let mut vector = vec![0; cols.len()];
        for (&point, &count) in row {
            vector[index[&point]] = m(count, ORDER);
        }
        while let Some(pivot) = vector.iter().position(|&v| v != 0) {
            if let Some(known) = pivots.get(&pivot) {
                let factor = vector[pivot];
                for i in pivot..vector.len() {
                    vector[i] = m(vector[i] - factor * known[i], ORDER);
                }
            } else {
                let factor = inv(vector[pivot], ORDER);
                for value in &mut vector {
                    *value = m(*value * factor, ORDER);
                }
                pivots.insert(pivot, vector);
                break;
            }
        }
    }
    pivots.len()
}
fn status(rows: &[Row]) -> Value {
    let mut signed_rows: Vec<Row> = rows.iter().map(signed).collect();
    let anchor = signed(&row(&[G]));
    let target = signed(&row(&[Q]));
    let cols = columns(&signed_rows);
    let base_rank = rank(&signed_rows, &cols);
    signed_rows.push(anchor);
    let anchored = rank(&signed_rows, &cols);
    signed_rows.push(target);
    json!({
        "signed_columns_including_generator_target": cols.len(),
        "signed_rank": base_rank,
        "target_log_determined": rank(&signed_rows, &cols) == anchored
    })
}
fn recover(rows: &[Row]) -> Option<i64> {
    let mut signed_rows: Vec<Row> = rows.iter().map(signed).collect();
    signed_rows.push(signed(&row(&[G])));
    let cols = columns(&signed_rows);
    let index: BTreeMap<Point, usize> = cols.iter().enumerate().map(|(i, &p)| (p, i)).collect();
    let mut matrix = vec![vec![0; cols.len() + 1]; signed_rows.len()];
    for (ri, relation) in signed_rows.iter().enumerate() {
        for (&point, &count) in relation {
            matrix[ri][index[&point]] = m(count, ORDER);
        }
    }
    *matrix.last_mut().unwrap().last_mut().unwrap() = 1;
    let mut pivot_row = vec![None; cols.len()];
    let mut lead = 0;
    for col in 0..cols.len() {
        let Some(pivot) = (lead..matrix.len()).find(|&r| matrix[r][col] != 0) else {
            continue;
        };
        matrix.swap(lead, pivot);
        let scale = inv(matrix[lead][col], ORDER);
        for v in &mut matrix[lead] {
            *v = m(*v * scale, ORDER);
        }
        let known = matrix[lead].clone();
        for (r, candidate) in matrix.iter_mut().enumerate() {
            if r == lead || candidate[col] == 0 {
                continue;
            }
            let factor = candidate[col];
            for (j, value) in candidate.iter_mut().enumerate().skip(col) {
                *value = m(*value - factor * known[j], ORDER);
            }
        }
        pivot_row[col] = Some(lead);
        lead += 1;
    }
    if pivot_row.iter().any(Option::is_none) {
        return None;
    }
    let target = signed(&row(&[Q]));
    let scalar = m(
        target
            .iter()
            .map(|(point, &count)| count * matrix[pivot_row[index[point]].unwrap()][cols.len()])
            .sum(),
        ORDER,
    );
    assert_eq!(mul(Some(G), scalar), Some(Q));
    Some(scalar)
}
fn rho() -> Value {
    fn step(
        state: (Option<Point>, i64, i64),
        salt: i64,
        ops: &mut usize,
    ) -> (Option<Point>, i64, i64) {
        let (point, a, b) = state;
        let bucket = m(point.map_or(0, |p| p.x) + salt, 3);
        *ops += 1;
        match bucket {
            0 => (add(point, Some(G)), m(a + 1, ORDER), b),
            1 => (add(point, point), m(2 * a, ORDER), m(2 * b, ORDER)),
            _ => (add(point, Some(Q)), a, m(b + 1, ORDER)),
        }
    }
    let mut ops = 0;
    for salt in 0..20 {
        let mut tortoise = (Some(G), 1, 0);
        let mut hare = tortoise;
        for _ in 0..500 {
            tortoise = step(tortoise, salt, &mut ops);
            hare = step(step(hare, salt, &mut ops), salt, &mut ops);
            if tortoise.0 != hare.0 {
                continue;
            }
            let denominator = m(hare.2 - tortoise.2, ORDER);
            if denominator != 0 {
                let scalar = m((tortoise.1 - hare.1) * inv(denominator, ORDER), ORDER);
                assert_eq!(mul(Some(G), scalar), Some(Q));
                return json!({"group_operations_through_collision":ops,"recovered_scalar_mod_109":scalar,"verified_by_ec_multiplication":true});
            }
            break;
        }
    }
    panic!("rho did not find a nondegenerate collision");
}
fn target_probe(d: usize, xs: &[i64], values: &[i64], elliptic: &[Point], public: &[Row]) -> Value {
    let fiber = xs.iter().filter(|&&x| x == Q.x).count();
    if fiber != d {
        return json!({"rational_target_fiber_size":fiber,"status":"full_rational_split_impossible_for_this_target"});
    }
    let mut rows = Vec::new();
    let mut native_rows = Vec::new();
    let mut by_type: BTreeMap<&str, usize> = BTreeMap::new();
    let mut attempted = 0;
    let mut direct_matches = 0;
    let mut direct_full = 0;
    let mut first_with_public: Option<Value> = None;
    let mut first_native: Option<Value> = None;
    for c in 0..P {
        if m(c * c, P) == 1 {
            continue;
        }
        for b in 0..P {
            attempted += 1;
            let points: Vec<Point> = (0..P as usize)
                .filter_map(|t| {
                    let x = xs[t];
                    let y = m(Q.y + (x - Q.x) * (c * t as i64 + b), P);
                    (m(y * y, P) == values[t]).then_some(Point { x, y })
                })
                .collect();
            if c != 0 && d == 2 {
                let zeros: BTreeSet<Point> = elliptic
                    .iter()
                    .copied()
                    .filter(|point| {
                        let xgap = point.x - Q.x;
                        let a = point.y - Q.y - b * xgap;
                        m(a * a - c * c * xgap * xgap * (point.x - xs[0]), P) == 0
                    })
                    .collect();
                let full = zeros.len() == 3 * d - d + 1;
                assert_eq!(full, points.len() == 3 * d);
                if full {
                    assert_eq!(zeros, points.iter().copied().collect());
                    direct_full += 1;
                }
            }
            if points.len() != 3 * d {
                continue;
            }
            assert_eq!(points.iter().filter(|&&p| p == Q).count(), d);
            assert_eq!(
                points
                    .iter()
                    .copied()
                    .fold(None, |sum, p| add(sum, Some(p))),
                None
            );
            let relation = row(&points);
            rows.push(relation.clone());
            if c != 0 {
                if d == 2 {
                    direct_matches += 1;
                }
                native_rows.push(relation);
                if first_with_public.is_none() {
                    let combined: Vec<Row> = public.iter().chain(&native_rows).cloned().collect();
                    if let Some(scalar) = recover(&combined) {
                        first_with_public = Some(
                            json!({"target_candidate_attempt":attempted,"native_target_rows_collected":native_rows.len(),"recovered_scalar_mod_109":scalar,"verified_by_ec_multiplication":true}),
                        );
                    }
                }
                if first_native.is_none() {
                    if let Some(scalar) = recover(&native_rows) {
                        first_native = Some(
                            json!({"target_candidate_attempt":attempted,"candidate_point_evaluations":attempted * P as usize,"native_target_rows_collected":native_rows.len(),"linear_system":status(&native_rows),"recovered_scalar_mod_109":scalar,"verified_by_ec_multiplication":true}),
                        );
                    }
                }
            }
            *by_type
                .entry(if c == 0 {
                    "invariant_c_zero"
                } else {
                    "native_c_nonzero"
                })
                .or_default() += 1;
        }
    }
    let public_all: Vec<Row> = public.iter().chain(&rows).cloned().collect();
    let public_native: Vec<Row> = public.iter().chain(&native_rows).cloned().collect();
    json!({
        "rational_target_fiber_size":fiber,"status":"scanned","candidate_sections":attempted,
        "full_split_target_rows":rows.len(),"full_split_by_type":by_type,
        "native_full_split_rows_matching_direct_norm_zero_sets":direct_matches,
        "direct_elliptic_norm_full_split_native_rows":direct_full,
        "first_blind_target_recovery":first_with_public,
        "first_blind_target_recovery_without_public_rows":first_native,
        "public_plus_target":status(&public_all),
        "public_plus_native_target":status(&public_native)
    })
}
fn degree(d: usize, shift: i64, elliptic: &[Point]) -> Value {
    assert_ne!(f(shift), 0);
    let xs: Vec<i64> = (0..P).map(|t| m(pow(t, d, P) + shift, P)).collect();
    let values: Vec<i64> = xs.iter().map(|&x| f(x)).collect();
    let mut projected = Row::new();
    for (&x, &value) in xs.iter().zip(&values) {
        for y in 0..P {
            if m(y * y, P) == value {
                *projected.entry(Point { x, y }).or_default() += 1;
            }
        }
    }
    let mut hist = BTreeMap::<usize, usize>::new();
    let mut rows = Vec::new();
    let mut by_type = BTreeMap::<&str, usize>::new();
    let mut norm_matches = 0;
    for a in 0..P {
        for b in 0..P {
            let points: Vec<Point> = (0..P as usize)
                .filter_map(|t| {
                    let y = m(a * t as i64 + b, P);
                    (m(y * y, P) == values[t]).then_some(Point { x: xs[t], y })
                })
                .collect();
            if a != 0 {
                let zeros: BTreeSet<Point> = elliptic
                    .iter()
                    .copied()
                    .filter(|point| {
                        m(
                            pow(m(point.y - b, P), d, P) - pow(a, d, P) * (point.x - shift),
                            P,
                        ) == 0
                    })
                    .collect();
                assert_eq!(zeros, points.iter().copied().collect());
                norm_matches += 1;
            }
            *hist.entry(points.len()).or_default() += 1;
            if points.len() == 3 * d {
                assert_eq!(
                    points
                        .iter()
                        .copied()
                        .fold(None, |sum, p| add(sum, Some(p))),
                    None
                );
                *by_type
                    .entry(if a == 0 { "a_zero" } else { "a_nonzero" })
                    .or_default() += 1;
                rows.push(row(&points));
            }
        }
    }
    let cols = columns(&rows);
    let signed_rows: Vec<Row> = rows.iter().map(signed).collect();
    let signed_status = status(&rows);
    let distinct_rows: BTreeSet<Vec<(Point, i64)>> = rows
        .iter()
        .map(|r| r.iter().map(|(&p, &c)| (p, c)).collect())
        .collect();
    let probe = target_probe(d, &xs, &values, elliptic, &rows);
    let gcd = (1..=d)
        .rev()
        .find(|k| d.is_multiple_of(*k) && 100usize.is_multiple_of(*k))
        .unwrap();
    json!({
        "degree":d,"shift":shift,"genus":if d%2==1 {(3*d-1)/2} else {(3*d-2)/2},
        "gcd_d_p_minus_1":gcd,"cover_affine_points":projected.values().sum::<i64>(),
        "distinct_projected_affine_points":projected.len(),
        "max_projection_multiplicity":projected.values().copied().max().unwrap_or(0),
        "linear_sections_examined":(P*P),"nondegenerate_sections_matching_direct_elliptic_norm_zeros":norm_matches,
        "full_rational_split_sections":rows.len(),"full_split_sections_by_linear_coefficient":by_type,
        "projected_relation_columns_including_generator_target":cols.len(),
        "projected_relation_rank_mod_109":rank(&rows,&cols),"distinct_projected_rows":distinct_rows.len(),
        "nonzero_rows_after_negation":signed_rows.iter().filter(|r|!r.is_empty()).count(),
        "signed_relation_columns_including_generator_target":signed_status["signed_columns_including_generator_target"],
        "signed_relation_rank_mod_109":signed_status["signed_rank"],
        "target_log_determined_after_generator_anchor":signed_status["target_log_determined"],
        "target_coupled_probe":probe,
        "generator_in_any_relation":rows.iter().any(|r|r.contains_key(&G)),
        "target_in_any_relation":rows.iter().any(|r|r.contains_key(&Q)),
        "root_count_histogram":hist
    })
}
fn main() {
    let elliptic: Vec<Point> = (0..P)
        .flat_map(|x| (0..P).filter_map(move |y| (m(y * y, P) == f(x)).then_some(Point { x, y })))
        .collect();
    assert_eq!(elliptic.len() + 1, ORDER as usize);
    assert_eq!(mul(Some(G), ORDER), None);
    assert!(elliptic.contains(&Q));
    let result = json!({
        "fixture":{"field":"F_101","elliptic_curve":"y^2=x^3+x+46","group_order":ORDER,"generator":[G.x,G.y],"target":[Q.x,Q.y]},
        "method":"Exhaust every h=y-(a*t+b), a,b in F_101; count only sections whose full zero divisor splits into distinct F_101-rational points.",
        "rho_control":rho(),
        "results":[degree(2,0,&elliptic),degree(3,0,&elliptic),degree(5,0,&elliptic),degree(7,0,&elliptic),degree(2,6,&elliptic)]
    });
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn public_fixture_group_is_correct() {
        assert_eq!(mul(Some(G), ORDER), None);
        assert_eq!(mul(Some(G), 37), Some(Q));
    }
    #[test]
    fn rational_norm_rows_recover_the_public_target() {
        let elliptic: Vec<Point> = (0..P)
            .flat_map(|x| {
                (0..P).filter_map(move |y| (m(y * y, P) == f(x)).then_some(Point { x, y }))
            })
            .collect();
        let result = degree(2, 6, &elliptic);
        assert_eq!(
            result["target_coupled_probe"]["first_blind_target_recovery_without_public_rows"]
                ["recovered_scalar_mod_109"],
            37
        );
        assert_eq!(
            result["target_coupled_probe"]["native_full_split_rows_matching_direct_norm_zero_sets"],
            202
        );
    }
}
