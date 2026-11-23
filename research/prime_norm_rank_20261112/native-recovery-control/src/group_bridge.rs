//! One fixed known-answer bridge control, never a target logarithm search.
use super::public_jets::{interpolate, kernel, norm};
use super::signed_hankel::{hankel, weights};
use super::*;

type Point = Option<(u64, u64)>;

fn on_curve(point: Point) -> bool {
    point.is_none_or(|(x, y)| x < P && y < P && mul(y, y) == add(add(power(x, 3), mul(2, x)), 2))
}

fn checked_point(x: u64, y: u64) -> Result<Point, &'static str> {
    let point = Some((x, y));
    if on_curve(point) {
        Ok(point)
    } else {
        Err("point is outside the stated curve or canonical field range")
    }
}

fn negate(point: Point) -> Point {
    point.map(|(x, y)| (x, sub(0, y)))
}

fn point_add(left: Point, right: Point) -> Point {
    assert!(on_curve(left) && on_curve(right));
    let result = match (left, right) {
        (None, point) | (point, None) => point,
        (Some((x1, y1)), Some((x2, y2))) => {
            if x1 == x2 && add(y1, y2) == 0 {
                None
            } else {
                let slope = if x1 == x2 {
                    assert_eq!(y1, y2);
                    mul(add(mul(3, mul(x1, x1)), 2), inv(mul(2, y1)))
                } else {
                    mul(sub(y2, y1), inv(sub(x2, x1)))
                };
                let x3 = sub(sub(mul(slope, slope), x1), x2);
                let y3 = sub(mul(slope, sub(x1, x3)), y1);
                Some((x3, y3))
            }
        }
    };
    assert!(on_curve(result));
    result
}

fn scalar_mul(mut point: Point, scalar: i64) -> Point {
    assert!(on_curve(point));
    if scalar < 0 {
        point = negate(point);
    }
    let mut magnitude = scalar.unsigned_abs();
    let mut result = None;
    while magnitude != 0 {
        if magnitude & 1 != 0 {
            result = point_add(result, point);
        }
        point = point_add(point, point);
        magnitude >>= 1;
    }
    result
}

pub(super) fn control() -> Value {
    let f = vec![2, 2, 0, 1];
    let branches = vec![(0, 6), (3, 1), (5, 1), (6, 3), (10, 11), (13, 7)];
    let points = branches
        .iter()
        .map(|(x, y)| checked_point(*x, *y).unwrap())
        .collect::<Vec<_>>();
    let positive_sum = points.iter().copied().fold(None, point_add);
    assert_eq!(positive_sum, None);

    let g = roots(&branches.iter().map(|(x, _)| *x).collect::<Vec<_>>());
    let m = hankel(&branches, &weights(&branches));
    assert_eq!(det(&m), 0);
    let basis = kernel(m);
    assert_eq!(basis.len(), 1);
    let b = trim(basis[0].clone());
    let a = remainder(&pm(&interpolate(&branches), &b), &g);
    let n = norm(&f, &a, &b);
    let c = coeff(&n, 6);
    assert_ne!(c, 0);
    assert_eq!(n, scale(&g, c));

    let mut flipped = branches.clone();
    flipped[5].1 = sub(0, flipped[5].1);
    let flipped_sum = flipped
        .iter()
        .map(|(x, y)| checked_point(*x, *y).unwrap())
        .fold(None, point_add);
    assert_ne!(flipped_sum, None);
    assert_eq!(flipped_sum, scalar_mul(points[5], -2));
    let flipped_matrix = hankel(&flipped, &weights(&flipped));
    assert_eq!(det(&flipped_matrix), 1);
    assert!(kernel(flipped_matrix).is_empty());

    let base = checked_point(5, 1).unwrap();
    let labels: [i64; 6] = [7, 4, 1, 2, 16, 8];
    for (point, label) in points.iter().zip(labels) {
        assert_eq!(scalar_mul(base, label), *point);
    }
    let recovered_integer = -labels[1..].iter().sum::<i64>();
    assert_eq!(recovered_integer, -31);
    assert_eq!(scalar_mul(base, recovered_integer), points[0]);
    assert_eq!(scalar_mul(base, labels[0]), points[0]);

    let off_curve = checked_point(0, 13);
    assert!(off_curve.is_err());
    assert!(checked_point(P, 6).is_err());
    json!({"schema":"norm.actual-elliptic-group-bridge/v1","p":P,
        "curve_coefficients":f,"branches":branches,"all_inputs_on_curve":true,
        "positive_sum":positive_sum,"positive_hankel_determinant":0,
        "A":a,"B":b,"norm":n,"c":c,"full_norm_pass":true,
        "flipped_branches":flipped,"flipped_sum":flipped_sum,
        "flipped_hankel_determinant":1,"flipped_kernel_dimension":0,
        "difference_equals_minus_twice_original_point_at13":true,
        "base":base,"predetermined_labels":labels,"target_index":0,
        "recovered_integer":recovered_integer,"scalar_multiplication_replay_pass":true,
        "off_curve_control":[0,13],"off_curve_control_rejected_before_addition":true,
        "out_of_range_control_rejected":true,"infinity_encoding":"null",
        "physical_artifact_invocations":1,"scalar_search_executed":false,
        "timing_measured":false,"standing":"unsigned producer evidence",
        "scope":"one frozen supplied-branch known-answer relation; no order, yield, speed or finder bound"})
}

pub(super) fn write(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = serde_json::to_vec_pretty(&control())?;
    let mut out = OpenOptions::new().create_new(true).write(true).open(path)?;
    out.write_all(&bytes)?;
    println!("actual group bridge: positive sum infinity; flipped sum nonzero; supplied-label target replay passes");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_norm_and_actual_group_control() {
        control();
    }
    #[test]
    fn group_identity_inverse_and_doubling_known_answer() {
        let base = checked_point(5, 1).unwrap();
        assert_eq!(point_add(base, None), base);
        assert_eq!(point_add(None, base), base);
        assert_eq!(point_add(base, negate(base)), None);
        assert_eq!(point_add(base, base), checked_point(6, 3).unwrap());
        assert_eq!(scalar_mul(base, 0), None);
        assert_eq!(scalar_mul(base, -1), negate(base));
    }
    #[test]
    fn invalid_curve_inputs_rejected_before_group_use() {
        assert!(checked_point(0, 13).is_err());
        assert!(checked_point(P, 6).is_err());
        assert!(checked_point(0, P + 6).is_err());
    }
}
