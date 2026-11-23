//! Independent verifier for the prospective H116 multi-prime boundary panel.
//!
//! This verifier does not construct scalar maps with division polynomials.  It
//! checks their action exhaustively on every affine base-field x-coordinate,
//! checks the retained full map against `[t-2]` on base-field points, and
//! recomputes both cleared polynomial identities and all frozen controls.

#[allow(dead_code)]
#[path = "../../src/hash/sha256.rs"]
mod sha256_impl;

use std::fs;

type Poly = Vec<u64>;

const ORACLE_BYTES: &[u8] = include_bytes!("FORMAL_SCALAR_OOS_ORACLE.json");
const ORACLE_TEXT: &str = include_str!("FORMAL_SCALAR_OOS_ORACLE.json");
const ORACLE_SHA256: &str = "8fb0dad351eb1b7948e42f553b6e9ff7f33ae91527e4bd09d8566ed767fe4eaf";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Point {
    Infinity,
    Affine(u64, u64),
}

fn add_mod(x: u64, y: u64, p: u64) -> u64 {
    ((x as u128 + y as u128) % p as u128) as u64
}

fn sub_mod(x: u64, y: u64, p: u64) -> u64 {
    ((x as u128 + p as u128 - y as u128) % p as u128) as u64
}

fn mul_mod(x: u64, y: u64, p: u64) -> u64 {
    ((x as u128 * y as u128) % p as u128) as u64
}

fn pow_mod(mut base: u64, mut exponent: u64, p: u64) -> u64 {
    let mut out = 1;
    while exponent != 0 {
        if exponent & 1 == 1 {
            out = mul_mod(out, base, p);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = mul_mod(base, base, p);
        }
    }
    out
}

fn inverse(value: u64, p: u64) -> u64 {
    assert_ne!(value, 0);
    pow_mod(value, p - 2, p)
}

fn trim(mut value: Poly) -> Poly {
    while value.len() > 1 && value.last() == Some(&0) {
        value.pop();
    }
    value
}

fn poly_add(left: &[u64], right: &[u64], p: u64) -> Poly {
    trim(
        (0..left.len().max(right.len()))
            .map(|i| {
                add_mod(
                    left.get(i).copied().unwrap_or(0),
                    right.get(i).copied().unwrap_or(0),
                    p,
                )
            })
            .collect(),
    )
}

fn poly_sub(left: &[u64], right: &[u64], p: u64) -> Poly {
    trim(
        (0..left.len().max(right.len()))
            .map(|i| {
                sub_mod(
                    left.get(i).copied().unwrap_or(0),
                    right.get(i).copied().unwrap_or(0),
                    p,
                )
            })
            .collect(),
    )
}

fn poly_mul(left: &[u64], right: &[u64], p: u64) -> Poly {
    if left == [0] || right == [0] {
        return vec![0];
    }
    let mut out = vec![0; left.len() + right.len() - 1];
    for (i, &x) in left.iter().enumerate() {
        for (j, &y) in right.iter().enumerate() {
            out[i + j] = add_mod(out[i + j], mul_mod(x, y, p), p);
        }
    }
    trim(out)
}

fn poly_scale(value: &[u64], scalar: u64, p: u64) -> Poly {
    trim(value.iter().map(|&x| mul_mod(x, scalar, p)).collect())
}

fn poly_pow(value: &[u64], mut exponent: usize, p: u64) -> Poly {
    let mut base = value.to_vec();
    let mut out = vec![1];
    while exponent != 0 {
        if exponent & 1 == 1 {
            out = poly_mul(&out, &base, p);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = poly_mul(&base, &base, p);
        }
    }
    out
}

fn poly_derivative(value: &[u64], p: u64) -> Poly {
    if value.len() < 2 {
        return vec![0];
    }
    trim(
        value
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &x)| mul_mod(i as u64 % p, x, p))
            .collect(),
    )
}

fn poly_eval(value: &[u64], x: u64, p: u64) -> u64 {
    value
        .iter()
        .rev()
        .fold(0, |accumulator, &coefficient| {
            add_mod(mul_mod(accumulator, x, p), coefficient, p)
        })
}

fn curve_identity(
    numerator: &[u64],
    denominator: &[u64],
    source_a: u64,
    source_b: u64,
    target_a: u64,
    target_b: u64,
    p: u64,
) -> bool {
    let source_curve = vec![source_b, source_a, 0, 1];
    let h = poly_sub(
        &poly_mul(&poly_derivative(numerator, p), denominator, p),
        &poly_mul(numerator, &poly_derivative(denominator, p), p),
        p,
    );
    let lhs = poly_mul(&source_curve, &poly_pow(&h, 2, p), p);
    let d2 = poly_pow(denominator, 2, p);
    let d3 = poly_mul(&d2, denominator, p);
    let target = poly_add(
        &poly_add(
            &poly_pow(numerator, 3, p),
            &poly_scale(&poly_mul(numerator, &d2, p), target_a, p),
            p,
        ),
        &poly_scale(&d3, target_b, p),
        p,
    );
    lhs == poly_mul(denominator, &target, p)
}

fn quotient_prefix(numerator: &[u64], denominator: &[u64], precision: usize, p: u64) -> Vec<u64> {
    let denominator_inverse = inverse(denominator[0], p);
    let mut out = vec![0; precision];
    for degree in 0..precision {
        let mut value = numerator.get(degree).copied().unwrap_or(0);
        for i in 1..=degree.min(denominator.len() - 1) {
            value = sub_mod(value, mul_mod(denominator[i], out[degree - i], p), p);
        }
        out[degree] = mul_mod(value, denominator_inverse, p);
    }
    out
}

fn first_difference(left: &[u64], right: &[u64]) -> i64 {
    left.iter()
        .zip(right)
        .position(|(x, y)| x != y)
        .map_or(-1, |index| index as i64)
}

fn point_neg(point: Point, p: u64) -> Point {
    match point {
        Point::Infinity => Point::Infinity,
        Point::Affine(x, y) => Point::Affine(x, sub_mod(0, y, p)),
    }
}

fn point_add(left: Point, right: Point, a: u64, p: u64) -> Point {
    match (left, right) {
        (Point::Infinity, point) | (point, Point::Infinity) => point,
        (Point::Affine(x1, y1), Point::Affine(x2, y2)) => {
            if x1 == x2 && add_mod(y1, y2, p) == 0 {
                return Point::Infinity;
            }
            let slope = if x1 == x2 {
                let numerator = add_mod(mul_mod(3, mul_mod(x1, x1, p), p), a, p);
                mul_mod(numerator, inverse(mul_mod(2, y1, p), p), p)
            } else {
                mul_mod(sub_mod(y2, y1, p), inverse(sub_mod(x2, x1, p), p), p)
            };
            let x3 = sub_mod(sub_mod(mul_mod(slope, slope, p), x1, p), x2, p);
            let y3 = sub_mod(mul_mod(slope, sub_mod(x1, x3, p), p), y1, p);
            Point::Affine(x3, y3)
        }
    }
}

fn scalar_mul(mut scalar: u64, mut point: Point, a: u64, p: u64) -> Point {
    let mut out = Point::Infinity;
    while scalar != 0 {
        if scalar & 1 == 1 {
            out = point_add(out, point, a, p);
        }
        scalar >>= 1;
        if scalar != 0 {
            point = point_add(point, point, a, p);
        }
    }
    out
}

fn scalar_mul_signed(scalar: i64, point: Point, a: u64, p: u64) -> Point {
    let out = scalar_mul(scalar.unsigned_abs(), point, a, p);
    if scalar < 0 { point_neg(out, p) } else { out }
}

fn parse_csv(value: &str) -> Vec<u64> {
    value.split(',').map(|term| term.parse().unwrap()).collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    sha256_impl::sha256(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn cell_slice(slug: &str) -> &'static str {
    let marker = format!("\"slug\":\"{slug}\"");
    let start = ORACLE_TEXT.find(&marker).expect("oracle cell missing");
    let tail = &ORACLE_TEXT[start..];
    let end = tail.find("},{\"slug\"").unwrap_or(tail.len());
    &tail[..end]
}

fn parse_array(cell: &str, key: &str) -> Vec<u64> {
    let marker = format!("\"{key}\":[");
    let start = cell.find(&marker).expect("array key missing") + marker.len();
    let end = cell[start..].find(']').expect("array end missing") + start;
    cell[start..end]
        .split(',')
        .map(|value| value.parse().expect("bad array integer"))
        .collect()
}

fn verify_point_actions(
    p: u64,
    r: u64,
    source_a: u64,
    source_b: u64,
    differential: i64,
    scalar_n: &[u64],
    scalar_d: &[u64],
    full_n: &[u64],
    full_d: &[u64],
) -> usize {
    let scale = differential.rem_euclid(p as i64) as u64;
    let scale2 = mul_mod(scale, scale, p);
    let mut affine_count = 0_u64;
    let mut checked_x = 0;
    let mut wrong_action_rejected = false;
    for x in 0..p {
        let rhs = add_mod(
            add_mod(mul_mod(mul_mod(x, x, p), x, p), mul_mod(source_a, x, p), p),
            source_b,
            p,
        );
        let mut roots = Vec::new();
        for y in 0..p {
            if mul_mod(y, y, p) == rhs {
                roots.push(y);
            }
        }
        affine_count += roots.len() as u64;
        let Some(&y) = roots.first() else { continue };
        let point = Point::Affine(x, y);

        let scalar_denominator = poly_eval(scalar_d, x, p);
        let full_denominator = poly_eval(full_d, x, p);
        assert_ne!(scalar_denominator, 0);
        assert_ne!(full_denominator, 0);
        let scalar_image_x = mul_mod(
            poly_eval(scalar_n, x, p),
            inverse(scalar_denominator, p),
            p,
        );
        let full_image_x = mul_mod(
            poly_eval(full_n, x, p),
            inverse(full_denominator, p),
            p,
        );

        let Point::Affine(expected_scalar_x, _) = scalar_mul_signed(differential, point, source_a, p) else {
            panic!("scalar image unexpectedly infinite");
        };
        let Point::Affine(expected_full_x, _) = scalar_mul_signed(differential - 1, point, source_a, p) else {
            panic!("full-map image unexpectedly infinite");
        };
        assert_eq!(scalar_image_x, mul_mod(scale2, expected_scalar_x, p));
        assert_eq!(full_image_x, mul_mod(scale2, expected_full_x, p));
        if let Point::Affine(wrong_x, _) = scalar_mul_signed(differential + 1, point, source_a, p) {
            wrong_action_rejected |= scalar_image_x != mul_mod(scale2, wrong_x, p);
        }
        checked_x += 1;
    }
    assert_eq!(affine_count + 1, r);
    assert!(wrong_action_rejected);
    checked_x
}

fn main() {
    assert_eq!(sha256_hex(ORACLE_BYTES), ORACLE_SHA256);
    let result_path = std::env::args().nth(1).expect("usage: verifier RESULT.tsv");
    let result = fs::read_to_string(result_path).expect("read result");
    let mut lines = result.lines();
    assert_eq!(lines.next(), Some("schema\th116.frobenius-boundary-panel-result/v1"));
    let header = lines.next().expect("missing header");
    assert!(header.starts_with("slug\tp\tr\ta\tb\ttrace\tscale\tprecision"));
    let mut verified_cells = 0;
    let mut equal_coefficients = 0;
    let mut point_x_checks = 0;
    let mut map_identities = 0;
    let mut negative_controls = 0;
    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        assert_eq!(fields.len(), 27);
        let slug = fields[0];
        let p: u64 = fields[1].parse().unwrap();
        let r: u64 = fields[2].parse().unwrap();
        let source_a: u64 = fields[3].parse().unwrap();
        let source_b: u64 = fields[4].parse().unwrap();
        let trace: i64 = fields[5].parse().unwrap();
        let differential = trace - 1;
        let scale: u64 = fields[6].parse().unwrap();
        assert_eq!(scale, differential.rem_euclid(p as i64) as u64);
        let precision: usize = fields[7].parse().unwrap();
        let predicted_first: usize = fields[8].parse().unwrap();
        let predicted_delta: u64 = fields[9].parse().unwrap();
        assert_eq!(predicted_first, (p as usize - 1) / 2);
        assert_eq!(predicted_delta, mul_mod(2, inverse(scale, p), p));
        assert_eq!(precision, predicted_first + 3);

        let oracle = cell_slice(slug);
        let full_n_reversed = parse_array(oracle, "numerator");
        let full_d_reversed = parse_array(oracle, "denominator");
        let scalar_n_reversed = parse_csv(fields[20]);
        let scalar_d_reversed = parse_csv(fields[21]);
        let full_prefix = quotient_prefix(&full_n_reversed, &full_d_reversed, precision, p);
        let scalar_prefix = quotient_prefix(&scalar_n_reversed, &scalar_d_reversed, precision, p);
        assert_eq!(parse_csv(fields[10]), full_prefix);
        assert_eq!(parse_csv(fields[11]), scalar_prefix);
        let first = first_difference(&full_prefix, &scalar_prefix);
        assert_eq!(first, predicted_first as i64);
        assert_eq!(fields[12].parse::<i64>().unwrap(), first);
        let delta = sub_mod(full_prefix[predicted_first], scalar_prefix[predicted_first], p);
        assert_eq!(delta, predicted_delta);
        assert_eq!(fields[13].parse::<u64>().unwrap(), delta);
        equal_coefficients += predicted_first;

        let scale2 = mul_mod(scale, scale, p);
        let scale4 = mul_mod(scale2, scale2, p);
        let scale6 = mul_mod(scale4, scale2, p);
        let target_a = mul_mod(scale4, source_a, p);
        let target_b = mul_mod(scale6, source_b, p);
        let mut full_n = full_n_reversed.clone();
        full_n.reverse();
        let mut full_d = full_d_reversed.clone();
        full_d.reverse();
        let mut scalar_n = scalar_n_reversed.clone();
        scalar_n.reverse();
        let mut scalar_d = scalar_d_reversed.clone();
        scalar_d.reverse();
        assert!(curve_identity(&full_n, &full_d, source_a, source_b, target_a, target_b, p));
        assert!(curve_identity(&scalar_n, &scalar_d, source_a, source_b, target_a, target_b, p));
        assert_eq!(fields[14], "true");
        assert_eq!(fields[15], "true");
        map_identities += 2;

        point_x_checks += verify_point_actions(
            p,
            r,
            source_a,
            source_b,
            differential,
            &scalar_n,
            &scalar_d,
            &full_n,
            &full_d,
        );

        let inverse_scale2 = inverse(scale2, p);
        let scalar_unscaled_reversed = poly_scale(&scalar_n_reversed, inverse_scale2, p);
        assert_eq!(
            first_difference(
                &full_prefix,
                &quotient_prefix(&scalar_unscaled_reversed, &scalar_d_reversed, precision, p),
            ),
            0,
        );
        assert_eq!(fields[16], "0");
        assert!(!curve_identity(
            &scalar_n,
            &scalar_d,
            source_a,
            source_b,
            mul_mod(scale2, source_a, p),
            mul_mod(scale4, source_b, p),
            p,
        ));
        assert_eq!(fields[17], "true");
        let mut mutated_reversed = scalar_n_reversed.clone();
        mutated_reversed[predicted_first] = add_mod(mutated_reversed[predicted_first], 1, p);
        assert_eq!(
            first_difference(
                &quotient_prefix(&mutated_reversed, &scalar_d_reversed, precision, p),
                &scalar_prefix,
            ),
            predicted_first as i64,
        );
        assert_eq!(fields[18].parse::<usize>().unwrap(), predicted_first);
        mutated_reversed.reverse();
        assert!(!curve_identity(
            &mutated_reversed,
            &scalar_d,
            source_a,
            source_b,
            target_a,
            target_b,
            p,
        ));
        assert_eq!(fields[19], "true");
        negative_controls += 4;
        verified_cells += 1;
    }
    assert_eq!(verified_cells, 2);
    println!(
        concat!(
            "{{\"schema\":\"h116.frobenius-boundary-panel-independent-verification/v1\",",
            "\"verified_cells\":{},\"equal_coefficients\":{},\"point_x_checks\":{},",
            "\"map_identities\":{},\"negative_controls_rejected\":{}}}"
        ),
        verified_cells, equal_coefficients, point_x_checks, map_identities, negative_controls,
    );
}
