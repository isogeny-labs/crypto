//! Prospective multi-prime first-omitted-Frobenius boundary producer.
//!
//! The retained full maps come from the separately frozen native Velu oracle.
//! This producer constructs the comparison scalar maps with division
//! polynomials.  The independent verifier uses point actions instead.

#[allow(dead_code)]
#[path = "../../src/hash/sha256.rs"]
mod sha256_impl;

use std::fmt::Write;

type Poly = Vec<u64>;

const ORACLE_BYTES: &[u8] = include_bytes!("FORMAL_SCALAR_OOS_ORACLE.json");
const ORACLE_TEXT: &str = include_str!("FORMAL_SCALAR_OOS_ORACLE.json");
const ORACLE_SHA256: &str = "8fb0dad351eb1b7948e42f553b6e9ff7f33ae91527e4bd09d8566ed767fe4eaf";

#[derive(Clone, Copy)]
struct Cell {
    slug: &'static str,
    p: u64,
    r: u64,
    a: u64,
    b: u64,
    trace: i64,
    boundary: usize,
    predicted_delta: u64,
}

const CELLS: [Cell; 2] = [
    Cell {
        slug: "fp211-r197-d4",
        p: 211,
        r: 197,
        a: 2,
        b: 24,
        trace: 15,
        boundary: 105,
        predicted_delta: 181,
    },
    Cell {
        slug: "fp241-r257-d4",
        p: 241,
        r: 257,
        a: 2,
        b: 4,
        trace: -15,
        boundary: 120,
        predicted_delta: 30,
    },
];

#[derive(Clone, Copy, Default)]
struct Count {
    additions: u64,
    multiplications: u64,
    inversions: u64,
}

fn add(x: u64, y: u64, p: u64, count: &mut Count) -> u64 {
    count.additions += 1;
    ((x as u128 + y as u128) % p as u128) as u64
}

fn sub(x: u64, y: u64, p: u64, count: &mut Count) -> u64 {
    count.additions += 1;
    ((x as u128 + p as u128 - y as u128) % p as u128) as u64
}

fn mul(x: u64, y: u64, p: u64, count: &mut Count) -> u64 {
    count.multiplications += 1;
    ((x as u128 * y as u128) % p as u128) as u64
}

fn pow_mod(mut base: u64, mut exponent: u64, p: u64, count: &mut Count) -> u64 {
    let mut out = 1;
    while exponent != 0 {
        if exponent & 1 == 1 {
            out = mul(out, base, p, count);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = mul(base, base, p, count);
        }
    }
    out
}

fn inv(value: u64, p: u64, count: &mut Count) -> u64 {
    assert_ne!(value, 0);
    count.inversions += 1;
    pow_mod(value, p - 2, p, count)
}

fn trim(mut value: Poly) -> Poly {
    while value.len() > 1 && value.last() == Some(&0) {
        value.pop();
    }
    value
}

fn poly_add(left: &[u64], right: &[u64], p: u64, count: &mut Count) -> Poly {
    trim(
        (0..left.len().max(right.len()))
            .map(|i| {
                add(
                    left.get(i).copied().unwrap_or(0),
                    right.get(i).copied().unwrap_or(0),
                    p,
                    count,
                )
            })
            .collect(),
    )
}

fn poly_sub(left: &[u64], right: &[u64], p: u64, count: &mut Count) -> Poly {
    trim(
        (0..left.len().max(right.len()))
            .map(|i| {
                sub(
                    left.get(i).copied().unwrap_or(0),
                    right.get(i).copied().unwrap_or(0),
                    p,
                    count,
                )
            })
            .collect(),
    )
}

fn poly_mul(left: &[u64], right: &[u64], p: u64, count: &mut Count) -> Poly {
    if left == [0] || right == [0] {
        return vec![0];
    }
    let mut out = vec![0; left.len() + right.len() - 1];
    for (i, &x) in left.iter().enumerate() {
        for (j, &y) in right.iter().enumerate() {
            let product = mul(x, y, p, count);
            out[i + j] = add(out[i + j], product, p, count);
        }
    }
    trim(out)
}

fn poly_scale(value: &[u64], scalar: u64, p: u64, count: &mut Count) -> Poly {
    trim(value.iter().map(|&x| mul(x, scalar, p, count)).collect())
}

fn poly_shift(value: &[u64], amount: usize) -> Poly {
    let mut out = vec![0; amount];
    out.extend_from_slice(value);
    trim(out)
}

fn poly_pow(value: &[u64], mut exponent: usize, p: u64, count: &mut Count) -> Poly {
    let mut base = value.to_vec();
    let mut out = vec![1];
    while exponent != 0 {
        if exponent & 1 == 1 {
            out = poly_mul(&out, &base, p, count);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = poly_mul(&base, &base, p, count);
        }
    }
    out
}

fn poly_derivative(value: &[u64], p: u64, count: &mut Count) -> Poly {
    if value.len() < 2 {
        return vec![0];
    }
    trim(
        value
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &x)| mul(i as u64 % p, x, p, count))
            .collect(),
    )
}

fn poly_div_exact(
    numerator: &[u64],
    denominator: &[u64],
    p: u64,
    count: &mut Count,
) -> Poly {
    let mut remainder = trim(numerator.to_vec());
    let denominator = trim(denominator.to_vec());
    assert_ne!(denominator, vec![0]);
    assert!(remainder.len() >= denominator.len());
    let mut quotient = vec![0; remainder.len() - denominator.len() + 1];
    let lead_inv = inv(*denominator.last().unwrap(), p, count);
    while remainder != [0] && remainder.len() >= denominator.len() {
        let shift = remainder.len() - denominator.len();
        let coefficient = mul(*remainder.last().unwrap(), lead_inv, p, count);
        quotient[shift] = coefficient;
        for (i, &term) in denominator.iter().enumerate() {
            let index = i + shift;
            remainder[index] = sub(
                remainder[index],
                mul(coefficient, term, p, count),
                p,
                count,
            );
        }
        remainder = trim(remainder);
    }
    assert_eq!(remainder, vec![0], "division-polynomial exact division failed");
    trim(quotient)
}

#[derive(Clone)]
struct Psi {
    polynomial: Poly,
    has_y: bool,
}

fn psi_mul(left: &Psi, right: &Psi, curve: &[u64], p: u64, count: &mut Count) -> Psi {
    let mut polynomial = poly_mul(&left.polynomial, &right.polynomial, p, count);
    let has_y = left.has_y ^ right.has_y;
    if left.has_y && right.has_y {
        polynomial = poly_mul(&polynomial, curve, p, count);
    }
    Psi { polynomial, has_y }
}

fn psi_pow(value: &Psi, mut exponent: usize, curve: &[u64], p: u64, count: &mut Count) -> Psi {
    let mut base = value.clone();
    let mut out = Psi { polynomial: vec![1], has_y: false };
    while exponent != 0 {
        if exponent & 1 == 1 {
            out = psi_mul(&out, &base, curve, p, count);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = psi_mul(&base, &base, curve, p, count);
        }
    }
    out
}

fn psi_sub(left: &Psi, right: &Psi, p: u64, count: &mut Count) -> Psi {
    assert_eq!(left.has_y, right.has_y);
    Psi {
        polynomial: poly_sub(&left.polynomial, &right.polynomial, p, count),
        has_y: left.has_y,
    }
}

fn divide_two_y(value: &Psi, curve: &[u64], p: u64, count: &mut Count) -> Psi {
    let inverse_two = inv(2, p, count);
    if value.has_y {
        Psi {
            polynomial: poly_scale(&value.polynomial, inverse_two, p, count),
            has_y: false,
        }
    } else {
        Psi {
            polynomial: poly_scale(
                &poly_div_exact(&value.polynomial, curve, p, count),
                inverse_two,
                p,
                count,
            ),
            has_y: true,
        }
    }
}

fn scalar_x_map(n: usize, a: u64, b: u64, p: u64, count: &mut Count) -> (Poly, Poly) {
    assert!(n >= 2);
    let curve = vec![b, a, 0, 1];
    let a2 = mul(a, a, p, count);
    let a3 = mul(a2, a, p, count);
    let b2 = mul(b, b, p, count);
    let psi3 = vec![
        sub(0, a2, p, count),
        mul(12, b, p, count),
        mul(6, a, p, count),
        0,
        3 % p,
    ];
    let psi4_core = vec![
        sub(sub(0, mul(8, b2, p, count), p, count), a3, p, count),
        sub(0, mul(4, mul(a, b, p, count), p, count), p, count),
        sub(0, mul(5, a2, p, count), p, count),
        mul(20, b, p, count),
        mul(5, a, p, count),
        0,
        1,
    ];
    let mut psi = vec![
        Psi { polynomial: vec![0], has_y: false },
        Psi { polynomial: vec![1], has_y: false },
        Psi { polynomial: vec![2], has_y: true },
        Psi { polynomial: psi3, has_y: false },
        Psi {
            polynomial: poly_scale(&psi4_core, 4, p, count),
            has_y: true,
        },
    ];
    for index in 5..=n + 1 {
        let next = if index & 1 == 1 {
            let m = (index - 1) / 2;
            let left = psi_mul(
                &psi[m + 2],
                &psi_pow(&psi[m], 3, &curve, p, count),
                &curve,
                p,
                count,
            );
            let right = psi_mul(
                &psi[m - 1],
                &psi_pow(&psi[m + 1], 3, &curve, p, count),
                &curve,
                p,
                count,
            );
            psi_sub(&left, &right, p, count)
        } else {
            let m = index / 2;
            let left = psi_mul(
                &psi[m + 2],
                &psi_pow(&psi[m - 1], 2, &curve, p, count),
                &curve,
                p,
                count,
            );
            let right = psi_mul(
                &psi[m - 2],
                &psi_pow(&psi[m + 1], 2, &curve, p, count),
                &curve,
                p,
                count,
            );
            let inside = psi_sub(&left, &right, p, count);
            divide_two_y(&psi_mul(&psi[m], &inside, &curve, p, count), &curve, p, count)
        };
        psi.push(next);
    }
    let denominator_psi = psi_mul(&psi[n], &psi[n], &curve, p, count);
    assert!(!denominator_psi.has_y);
    let adjacent = psi_mul(&psi[n - 1], &psi[n + 1], &curve, p, count);
    assert!(!adjacent.has_y);
    let numerator = poly_sub(
        &poly_shift(&denominator_psi.polynomial, 1),
        &adjacent.polynomial,
        p,
        count,
    );
    (numerator, denominator_psi.polynomial)
}

fn curve_identity(
    numerator: &[u64],
    denominator: &[u64],
    source_a: u64,
    source_b: u64,
    target_a: u64,
    target_b: u64,
    p: u64,
    count: &mut Count,
) -> bool {
    let source_curve = vec![source_b, source_a, 0, 1];
    let derivative_n = poly_derivative(numerator, p, count);
    let derivative_d = poly_derivative(denominator, p, count);
    let h = poly_sub(
        &poly_mul(&derivative_n, denominator, p, count),
        &poly_mul(numerator, &derivative_d, p, count),
        p,
        count,
    );
    let lhs = poly_mul(&source_curve, &poly_pow(&h, 2, p, count), p, count);
    let d2 = poly_pow(denominator, 2, p, count);
    let d3 = poly_mul(&d2, denominator, p, count);
    let n3 = poly_pow(numerator, 3, p, count);
    let target = poly_add(
        &poly_add(
            &n3,
            &poly_scale(&poly_mul(numerator, &d2, p, count), target_a, p, count),
            p,
            count,
        ),
        &poly_scale(&d3, target_b, p, count),
        p,
        count,
    );
    lhs == poly_mul(denominator, &target, p, count)
}

fn quotient_prefix(
    numerator_reversed: &[u64],
    denominator_reversed: &[u64],
    precision: usize,
    p: u64,
    count: &mut Count,
) -> Vec<u64> {
    let denominator_inverse = inv(denominator_reversed[0], p, count);
    let mut out = vec![0; precision];
    for degree in 0..precision {
        let mut value = numerator_reversed.get(degree).copied().unwrap_or(0);
        for i in 1..=degree.min(denominator_reversed.len() - 1) {
            value = sub(
                value,
                mul(denominator_reversed[i], out[degree - i], p, count),
                p,
                count,
            );
        }
        out[degree] = mul(value, denominator_inverse, p, count);
    }
    out
}

fn first_difference(left: &[u64], right: &[u64]) -> i64 {
    left.iter()
        .zip(right)
        .position(|(x, y)| x != y)
        .map_or(-1, |index| index as i64)
}

fn csv(values: &[u64]) -> String {
    let mut out = String::new();
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        write!(&mut out, "{value}").unwrap();
    }
    out
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
    if start == end {
        return Vec::new();
    }
    cell[start..end]
        .split(',')
        .map(|value| value.parse().expect("bad array integer"))
        .collect()
}

fn run_cell(cell: Cell) -> String {
    let p = cell.p;
    let mut count = Count::default();
    let oracle = cell_slice(cell.slug);
    let full_n_reversed = parse_array(oracle, "numerator");
    let full_d_reversed = parse_array(oracle, "denominator");
    assert_eq!(full_n_reversed.len() - 1, cell.r as usize);
    assert_eq!(full_d_reversed.len() - 1, cell.r as usize - 1);

    let scale = (cell.trace - 1).rem_euclid(p as i64) as u64;
    let scale_abs = (cell.trace - 1).unsigned_abs() as usize;
    let scale2 = mul(scale, scale, p, &mut count);
    let scale4 = mul(scale2, scale2, p, &mut count);
    let scale6 = mul(scale4, scale2, p, &mut count);
    let target_a = mul(scale4, cell.a, p, &mut count);
    let target_b = mul(scale6, cell.b, p, &mut count);
    let wrong_target_a = mul(scale2, cell.a, p, &mut count);
    let wrong_target_b = mul(scale4, cell.b, p, &mut count);

    let (scalar_n, scalar_d) = scalar_x_map(scale_abs, cell.a, cell.b, p, &mut count);
    assert_eq!(scalar_n.len() - 1, scale_abs * scale_abs);
    assert_eq!(scalar_d.len() - 1, scale_abs * scale_abs - 1);
    let scalar_n_normalized = poly_scale(&scalar_n, scale2, p, &mut count);
    let construction_count = count;

    let mut scalar_n_reversed = scalar_n_normalized.clone();
    scalar_n_reversed.reverse();
    let mut scalar_d_reversed = scalar_d.clone();
    scalar_d_reversed.reverse();
    let precision = cell.boundary + 3;
    let full_prefix = quotient_prefix(
        &full_n_reversed,
        &full_d_reversed,
        precision,
        p,
        &mut count,
    );
    let scalar_prefix = quotient_prefix(
        &scalar_n_reversed,
        &scalar_d_reversed,
        precision,
        p,
        &mut count,
    );
    let first = first_difference(&full_prefix, &scalar_prefix);
    assert!(first >= 0);
    let delta = sub(
        full_prefix[first as usize],
        scalar_prefix[first as usize],
        p,
        &mut count,
    );

    let mut full_n = full_n_reversed.clone();
    full_n.reverse();
    let mut full_d = full_d_reversed.clone();
    full_d.reverse();
    let full_identity = curve_identity(
        &full_n,
        &full_d,
        cell.a,
        cell.b,
        target_a,
        target_b,
        p,
        &mut count,
    );
    let scalar_identity = curve_identity(
        &scalar_n_normalized,
        &scalar_d,
        cell.a,
        cell.b,
        target_a,
        target_b,
        p,
        &mut count,
    );
    let mut scalar_unscaled_reversed = scalar_n.clone();
    scalar_unscaled_reversed.reverse();
    let omitted_scale_first = first_difference(
        &full_prefix,
        &quotient_prefix(
            &scalar_unscaled_reversed,
            &scalar_d_reversed,
            precision,
            p,
            &mut count,
        ),
    );
    let wrong_target_rejected = !curve_identity(
        &scalar_n_normalized,
        &scalar_d,
        cell.a,
        cell.b,
        wrong_target_a,
        wrong_target_b,
        p,
        &mut count,
    );
    let mut mutated_reversed = scalar_n_reversed.clone();
    mutated_reversed[cell.boundary] = add(
        mutated_reversed[cell.boundary],
        1,
        p,
        &mut count,
    );
    let mutation_first = first_difference(
        &quotient_prefix(
            &mutated_reversed,
            &scalar_d_reversed,
            precision,
            p,
            &mut count,
        ),
        &scalar_prefix,
    );
    mutated_reversed.reverse();
    let mutation_identity_rejected = !curve_identity(
        &mutated_reversed,
        &scalar_d,
        cell.a,
        cell.b,
        target_a,
        target_b,
        p,
        &mut count,
    );

    format!(
        concat!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}"
        ),
        cell.slug,
        cell.p,
        cell.r,
        cell.a,
        cell.b,
        cell.trace,
        scale,
        precision,
        cell.boundary,
        cell.predicted_delta,
        csv(&full_prefix),
        csv(&scalar_prefix),
        first,
        delta,
        full_identity,
        scalar_identity,
        omitted_scale_first,
        wrong_target_rejected,
        mutation_first,
        mutation_identity_rejected,
        csv(&scalar_n_reversed),
        csv(&scalar_d_reversed),
        construction_count.additions,
        construction_count.multiplications,
        construction_count.inversions,
        count.additions,
        count.multiplications,
    )
}

fn self_test() {
    let mut count = Count::default();
    let (n, d) = scalar_x_map(6, 1, 19, 31, &mut count);
    let scale = 25;
    let scale2 = mul(scale, scale, 31, &mut count);
    let normalized = poly_scale(&n, scale2, 31, &mut count);
    assert_eq!((normalized.len() - 1, d.len() - 1), (36, 35));
    let mut nr = normalized.clone();
    nr.reverse();
    let mut dr = d.clone();
    dr.reverse();
    let prefix = quotient_prefix(&nr, &dr, 18, 31, &mut count);
    assert_eq!(
        prefix,
        vec![1, 0, 20, 0, 13, 7, 2, 4, 8, 14, 5, 25, 12, 2, 6, 8, 7, 20]
    );
    let scale4 = mul(scale2, scale2, 31, &mut count);
    let scale6 = mul(scale4, scale2, 31, &mut count);
    assert!(curve_identity(
        &normalized,
        &d,
        1,
        19,
        mul(scale4, 1, 31, &mut count),
        mul(scale6, 19, 31, &mut count),
        31,
        &mut count,
    ));
    println!("h116 Frobenius boundary panel self-test ok");
}

fn main() {
    assert_eq!(sha256_hex(ORACLE_BYTES), ORACLE_SHA256);
    if std::env::args().any(|argument| argument == "--self-test") {
        self_test();
        return;
    }
    println!("schema\th116.frobenius-boundary-panel-result/v1");
    println!(concat!(
        "slug\tp\tr\ta\tb\ttrace\tscale\tprecision\tpredicted_first_difference\tpredicted_delta\t",
        "full_prefix\tscalar_prefix\tfirst_difference\tdelta\tfull_identity\tscalar_identity\t",
        "omitted_scale_first_difference\twrong_target_rejected\tmutation_first_difference\t",
        "mutation_identity_rejected\tscalar_n_reversed\tscalar_d_reversed\t",
        "construction_additions\tconstruction_multiplications\tconstruction_inversions\t",
        "total_additions\ttotal_multiplications"
    ));
    for cell in CELLS {
        println!("{}", run_cell(cell));
    }
}
