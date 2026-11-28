//! Frozen H116 scalar division-polynomial degree producer.

#[allow(dead_code)]
#[path = "h116_field.rs"]
mod h116_field;

use h116_field::{first_irreducible, Curve, Felt, Field, Point};

type Poly = Vec<u64>;

#[derive(Clone, Copy)]
struct Cell {
    slug: &'static str,
    p: u64,
    a: u64,
    b: u64,
    degree: usize,
    scalar: usize,
}

const CELLS: [Cell; 8] = [
    Cell { slug: "fp5-a2-b1-d2", p: 5, a: 2, b: 1, degree: 2, scalar: 7 },
    Cell { slug: "fp5-a2-b1-d3", p: 5, a: 2, b: 1, degree: 3, scalar: 7 },
    Cell { slug: "fp5-a2-b1-d4", p: 5, a: 2, b: 1, degree: 4, scalar: 7 },
    Cell { slug: "fp7-a1-b1-d2", p: 7, a: 1, b: 1, degree: 2, scalar: 5 },
    Cell { slug: "fp7-a1-b1-d3", p: 7, a: 1, b: 1, degree: 3, scalar: 5 },
    Cell { slug: "fp7-a1-b1-d4", p: 7, a: 1, b: 1, degree: 4, scalar: 5 },
    Cell { slug: "fp11-a1-b1-d2", p: 11, a: 1, b: 1, degree: 2, scalar: 14 },
    Cell { slug: "fp11-a1-b1-d3", p: 11, a: 1, b: 1, degree: 3, scalar: 14 },
];

#[derive(Clone, Copy, Default)]
struct Count {
    add: u64,
    mul: u64,
    inv: u64,
}

fn add(x: u64, y: u64, p: u64, count: &mut Count) -> u64 {
    count.add += 1;
    ((u128::from(x) + u128::from(y)) % u128::from(p)) as u64
}

fn sub(x: u64, y: u64, p: u64, count: &mut Count) -> u64 {
    count.add += 1;
    ((u128::from(x) + u128::from(p) - u128::from(y)) % u128::from(p)) as u64
}

fn mul(x: u64, y: u64, p: u64, count: &mut Count) -> u64 {
    count.mul += 1;
    ((u128::from(x) * u128::from(y)) % u128::from(p)) as u64
}

fn pow_mod(mut value: u64, mut exponent: u64, p: u64, count: &mut Count) -> u64 {
    let mut out = 1;
    while exponent != 0 {
        if exponent & 1 == 1 {
            out = mul(out, value, p, count);
        }
        exponent >>= 1;
        if exponent != 0 {
            value = mul(value, value, p, count);
        }
    }
    out
}

fn inv(value: u64, p: u64, count: &mut Count) -> u64 {
    assert_ne!(value, 0);
    count.inv += 1;
    pow_mod(value, p - 2, p, count)
}

fn trim(mut value: Poly) -> Poly {
    while value.len() > 1 && value.last() == Some(&0) {
        value.pop();
    }
    value
}

fn poly_sub(left: &[u64], right: &[u64], p: u64, count: &mut Count) -> Poly {
    trim(
        (0..left.len().max(right.len()))
            .map(|i| sub(
                left.get(i).copied().unwrap_or(0),
                right.get(i).copied().unwrap_or(0),
                p,
                count,
            ))
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

fn poly_shift(value: &[u64]) -> Poly {
    let mut out = vec![0];
    out.extend_from_slice(value);
    trim(out)
}

fn poly_div_exact(numerator: &[u64], denominator: &[u64], p: u64, count: &mut Count) -> Poly {
    let mut remainder = trim(numerator.to_vec());
    let denominator = trim(denominator.to_vec());
    assert_ne!(denominator, vec![0]);
    assert!(remainder.len() >= denominator.len());
    let mut quotient = vec![0; remainder.len() - denominator.len() + 1];
    let lead_inverse = inv(*denominator.last().expect("nonempty polynomial"), p, count);
    while remainder != [0] && remainder.len() >= denominator.len() {
        let shift = remainder.len() - denominator.len();
        let coefficient = mul(
            *remainder.last().expect("nonempty remainder"),
            lead_inverse,
            p,
            count,
        );
        quotient[shift] = coefficient;
        for (i, &term) in denominator.iter().enumerate() {
            let index = shift + i;
            remainder[index] = sub(
                remainder[index],
                mul(coefficient, term, p, count),
                p,
                count,
            );
        }
        remainder = trim(remainder);
    }
    assert_eq!(remainder, vec![0]);
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

fn psi_pow(value: &Psi, exponent: usize, curve: &[u64], p: u64, count: &mut Count) -> Psi {
    let mut base = value.clone();
    let mut power = exponent;
    let mut out = Psi { polynomial: vec![1], has_y: false };
    while power != 0 {
        if power & 1 == 1 {
            out = psi_mul(&out, &base, curve, p, count);
        }
        power >>= 1;
        if power != 0 {
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
        mul(12 % p, b, p, count),
        mul(6 % p, a, p, count),
        0,
        3 % p,
    ];
    let psi4_core = vec![
        sub(sub(0, mul(8 % p, b2, p, count), p, count), a3, p, count),
        sub(0, mul(4, mul(a, b, p, count), p, count), p, count),
        sub(0, mul(5 % p, a2, p, count), p, count),
        mul(20 % p, b, p, count),
        mul(5 % p, a, p, count),
        0,
        1,
    ];
    let mut psi = vec![
        Psi { polynomial: vec![0], has_y: false },
        Psi { polynomial: vec![1], has_y: false },
        Psi { polynomial: vec![2], has_y: true },
        Psi { polynomial: psi3, has_y: false },
        Psi { polynomial: poly_scale(&psi4_core, 4, p, count), has_y: true },
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
    let denominator = psi_mul(&psi[n], &psi[n], &curve, p, count);
    let adjacent = psi_mul(&psi[n - 1], &psi[n + 1], &curve, p, count);
    assert!(!denominator.has_y && !adjacent.has_y);
    (
        poly_sub(&poly_shift(&denominator.polynomial), &adjacent.polynomial, p, count),
        denominator.polynomial,
    )
}

fn eval(field: &Field, polynomial: &[u64], x: Felt) -> Felt {
    polynomial
        .iter()
        .rev()
        .fold(field.zero(), |value, &coefficient| {
            field.add(field.mul(value, x), field.from_base(coefficient))
        })
}

fn run_cell(cell: Cell) -> Result<String, String> {
    let modulus = first_irreducible(cell.p, cell.degree)?;
    let field = Field::new(cell.p, cell.degree, &modulus)?;
    let curve = Curve {
        field: &field,
        a: field.from_base(cell.a),
        b: field.from_base(cell.b),
    };
    let mut construction = Count::default();
    let (numerator, denominator) =
        scalar_x_map(cell.scalar, cell.a, cell.b, cell.p, &mut construction);
    let mut char_count = Count::default();
    let (_, char_denominator) =
        scalar_x_map(cell.p as usize, cell.a, cell.b, cell.p, &mut char_count);

    let numerator_degree = numerator.len() - 1;
    let denominator_degree = denominator.len() - 1;
    let expected_numerator_degree = cell.scalar * cell.scalar;
    let expected_denominator_degree = expected_numerator_degree - 1;
    let expected_denominator_lead =
        (u128::from(cell.scalar as u64) * u128::from(cell.scalar as u64)
            % u128::from(cell.p)) as u64;
    let char_expected_degree = cell.p as usize * cell.p as usize - 1;
    let char_expected_coefficient =
        char_denominator.get(char_expected_degree).copied().unwrap_or(0);

    field.reset_counters();
    let mut affine_points = 0_u64;
    let mut regular_points = 0_u64;
    let mut pole_points = 0_u64;
    let mut map_failures = 0_u64;
    let mut pole_failures = 0_u64;
    let mut cancellations = 0_u64;
    let mut mutation_failures = 0_u64;
    for index in 0..field.q {
        let x = field.from_index(index);
        let rhs = field.add(
            field.add(field.mul(field.square(x), x), field.mul(curve.a, x)),
            curve.b,
        );
        let Some(y) = field.sqrt(rhs)? else { continue };
        let mut ys = vec![y];
        if y != field.zero() {
            ys.push(field.neg(y));
        }
        for ordinate in ys {
            affine_points += 1;
            let point = Point::Affine(x, ordinate);
            let image = curve.scalar_mul(cell.scalar as u64, point)?;
            let numerator_value = eval(&field, &numerator, x);
            let denominator_value = eval(&field, &denominator, x);
            match image {
                Point::Infinity => {
                    pole_points += 1;
                    if denominator_value != field.zero() {
                        pole_failures += 1;
                    }
                    if numerator_value == field.zero() {
                        cancellations += 1;
                    }
                }
                Point::Affine(image_x, _) => {
                    regular_points += 1;
                    if denominator_value == field.zero() {
                        pole_failures += 1;
                        map_failures += 1;
                        mutation_failures += 1;
                        continue;
                    }
                    let predicted = field.div(numerator_value, denominator_value)?;
                    if predicted != image_x {
                        map_failures += 1;
                    }
                    let mutated = field.div(
                        field.add(numerator_value, field.one()),
                        denominator_value,
                    )?;
                    if mutated != image_x {
                        mutation_failures += 1;
                    }
                }
            }
        }
    }
    let operations = field.snapshot();
    let degree_ok = numerator_degree == expected_numerator_degree
        && denominator_degree == expected_denominator_degree;
    let leading_ok = numerator.last() == Some(&1)
        && denominator.last() == Some(&expected_denominator_lead);
    let pointwise_ok = map_failures == 0 && pole_failures == 0 && cancellations == 0;
    let mutation_rejected = mutation_failures > 0;
    let characteristic_control = char_denominator.len() - 1 < char_expected_degree
        && char_expected_coefficient == 0;
    let accepted = degree_ok
        && leading_ok
        && pointwise_ok
        && mutation_rejected
        && characteristic_control;
    Ok(format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        cell.slug,
        cell.p,
        cell.a,
        cell.b,
        cell.degree,
        cell.scalar,
        numerator_degree,
        denominator_degree,
        numerator.last().expect("numerator nonempty"),
        denominator.last().expect("denominator nonempty"),
        char_denominator.len() - 1,
        char_expected_degree,
        char_expected_coefficient,
        affine_points,
        regular_points,
        pole_points,
        map_failures,
        pole_failures,
        cancellations,
        mutation_failures,
        construction.add,
        construction.mul,
        construction.inv,
        char_count.add,
        char_count.mul,
        char_count.inv,
        operations.extension_additions,
        operations.extension_multiplications,
        operations.extension_inversions,
        operations.point_additions,
        operations.scalar_multiplications,
        characteristic_control,
        accepted,
    ))
}

fn self_test() -> Result<(), String> {
    h116_field::selftest()?;
    let mut count = Count::default();
    let (numerator, denominator) = scalar_x_map(3, 2, 1, 5, &mut count);
    if (numerator.len() - 1, denominator.len() - 1) != (9, 8) {
        return Err("degree self-test failed".to_string());
    }
    Ok(())
}

fn main() -> Result<(), String> {
    if std::env::args().any(|argument| argument == "--self-test") {
        self_test()?;
        println!("h116 scalar division-polynomial degree self-test ok");
        return Ok(());
    }
    println!(concat!(
        "slug\tp\ta\tb\textension_degree\tN\tnumerator_degree\tdenominator_degree\t",
        "numerator_lead\tdenominator_lead\tchar_denominator_degree\t",
        "char_expected_degree\tchar_expected_coefficient\taffine_points\t",
        "regular_points\tpole_points\tmap_failures\tpole_failures\tcancellations\t",
        "mutation_failures\tconstruction_additions\tconstruction_multiplications\t",
        "construction_inversions\tchar_control_additions\tchar_control_multiplications\t",
        "char_control_inversions\textension_additions\textension_multiplications\t",
        "extension_inversions\tpoint_additions\tscalar_multiplications\t",
        "characteristic_control\taccepted"
    ));
    for cell in CELLS {
        println!("{}", run_cell(cell)?);
    }
    Ok(())
}
