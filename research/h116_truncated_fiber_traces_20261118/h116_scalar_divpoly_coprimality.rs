//! Exact polynomial-gcd producer for H116-SCALAR-DIVPOLY-COPRIMALITY-001.

type Poly = Vec<u64>;

#[derive(Clone, Copy)]
struct Cell {
    slug: &'static str,
    p: u64,
    a: u64,
    b: u64,
    scalar: usize,
    singular_control: bool,
}

const CELLS: [Cell; 13] = [
    Cell { slug: "fp13-a1-b1-n2", p: 13, a: 1, b: 1, scalar: 2, singular_control: false },
    Cell { slug: "fp13-a1-b1-n3", p: 13, a: 1, b: 1, scalar: 3, singular_control: false },
    Cell { slug: "fp13-a1-b1-n5", p: 13, a: 1, b: 1, scalar: 5, singular_control: false },
    Cell { slug: "fp13-a1-b1-n7", p: 13, a: 1, b: 1, scalar: 7, singular_control: false },
    Cell { slug: "fp17-a2-b3-n2", p: 17, a: 2, b: 3, scalar: 2, singular_control: false },
    Cell { slug: "fp17-a2-b3-n3", p: 17, a: 2, b: 3, scalar: 3, singular_control: false },
    Cell { slug: "fp17-a2-b3-n5", p: 17, a: 2, b: 3, scalar: 5, singular_control: false },
    Cell { slug: "fp17-a2-b3-n7", p: 17, a: 2, b: 3, scalar: 7, singular_control: false },
    Cell { slug: "fp19-a1-b4-n2", p: 19, a: 1, b: 4, scalar: 2, singular_control: false },
    Cell { slug: "fp19-a1-b4-n3", p: 19, a: 1, b: 4, scalar: 3, singular_control: false },
    Cell { slug: "fp19-a1-b4-n5", p: 19, a: 1, b: 4, scalar: 5, singular_control: false },
    Cell { slug: "fp19-a1-b4-n7", p: 19, a: 1, b: 4, scalar: 7, singular_control: false },
    Cell { slug: "control-fp13-a0-b0-n2", p: 13, a: 0, b: 0, scalar: 2, singular_control: true },
];

#[derive(Clone, Copy, Default)]
struct Count {
    additions: u64,
    multiplications: u64,
    inversions: u64,
    remainder_steps: u64,
}

fn add(left: u64, right: u64, p: u64, count: &mut Count) -> u64 {
    count.additions += 1;
    ((u128::from(left) + u128::from(right)) % u128::from(p)) as u64
}

fn sub(left: u64, right: u64, p: u64, count: &mut Count) -> u64 {
    count.additions += 1;
    ((u128::from(left) + u128::from(p) - u128::from(right)) % u128::from(p)) as u64
}

fn mul(left: u64, right: u64, p: u64, count: &mut Count) -> u64 {
    count.multiplications += 1;
    ((u128::from(left) * u128::from(right)) % u128::from(p)) as u64
}

fn power(mut base: u64, mut exponent: u64, p: u64, count: &mut Count) -> u64 {
    let mut result = 1_u64;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = mul(result, base, p, count);
        }
        exponent >>= 1;
        if exponent > 0 {
            base = mul(base, base, p, count);
        }
    }
    result
}

fn inverse(value: u64, p: u64, count: &mut Count) -> u64 {
    assert_ne!(value, 0);
    count.inversions += 1;
    power(value, p - 2, p, count)
}

fn trim(mut polynomial: Poly) -> Poly {
    while polynomial.len() > 1 && polynomial.last() == Some(&0) {
        polynomial.pop();
    }
    polynomial
}

fn poly_sub(left: &[u64], right: &[u64], p: u64, count: &mut Count) -> Poly {
    trim((0..left.len().max(right.len())).map(|index| {
        sub(
            left.get(index).copied().unwrap_or(0),
            right.get(index).copied().unwrap_or(0),
            p,
            count,
        )
    }).collect())
}

fn poly_mul(left: &[u64], right: &[u64], p: u64, count: &mut Count) -> Poly {
    if left == [0] || right == [0] {
        return vec![0];
    }
    let mut result = vec![0_u64; left.len() + right.len() - 1];
    for (i, &a) in left.iter().enumerate() {
        for (j, &b) in right.iter().enumerate() {
            let product = mul(a, b, p, count);
            result[i + j] = add(result[i + j], product, p, count);
        }
    }
    trim(result)
}

fn poly_scale(polynomial: &[u64], scalar: u64, p: u64, count: &mut Count) -> Poly {
    trim(polynomial.iter().map(|&coefficient| mul(coefficient, scalar, p, count)).collect())
}

fn poly_shift(polynomial: &[u64]) -> Poly {
    let mut result = vec![0];
    result.extend_from_slice(polynomial);
    trim(result)
}

fn poly_remainder(numerator: &[u64], denominator: &[u64], p: u64, count: &mut Count) -> Poly {
    let mut remainder = trim(numerator.to_vec());
    let denominator = trim(denominator.to_vec());
    assert_ne!(denominator, vec![0]);
    let lead_inverse = inverse(*denominator.last().expect("denominator nonempty"), p, count);
    while remainder != [0] && remainder.len() >= denominator.len() {
        count.remainder_steps += 1;
        let shift = remainder.len() - denominator.len();
        let coefficient = mul(
            *remainder.last().expect("remainder nonempty"),
            lead_inverse,
            p,
            count,
        );
        for (index, &term) in denominator.iter().enumerate() {
            let destination = shift + index;
            remainder[destination] = sub(
                remainder[destination],
                mul(coefficient, term, p, count),
                p,
                count,
            );
        }
        remainder = trim(remainder);
    }
    remainder
}

fn poly_div_exact(numerator: &[u64], denominator: &[u64], p: u64, count: &mut Count) -> Poly {
    let mut remainder = trim(numerator.to_vec());
    let denominator = trim(denominator.to_vec());
    assert_ne!(denominator, vec![0]);
    assert!(remainder.len() >= denominator.len());
    let mut quotient = vec![0; remainder.len() - denominator.len() + 1];
    let lead_inverse = inverse(*denominator.last().expect("denominator nonempty"), p, count);
    while remainder != [0] && remainder.len() >= denominator.len() {
        let shift = remainder.len() - denominator.len();
        let coefficient = mul(
            *remainder.last().expect("remainder nonempty"),
            lead_inverse,
            p,
            count,
        );
        quotient[shift] = coefficient;
        for (index, &term) in denominator.iter().enumerate() {
            let destination = shift + index;
            remainder[destination] = sub(
                remainder[destination],
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

fn monic_gcd(left: &[u64], right: &[u64], p: u64, count: &mut Count) -> Poly {
    let mut a = trim(left.to_vec());
    let mut b = trim(right.to_vec());
    while b != [0] {
        let remainder = poly_remainder(&a, &b, p, count);
        a = b;
        b = remainder;
    }
    if a == [0] {
        return a;
    }
    let scale = inverse(*a.last().expect("gcd nonempty"), p, count);
    poly_scale(&a, scale, p, count)
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
    let mut remaining = exponent;
    let mut result = Psi { polynomial: vec![1], has_y: false };
    while remaining > 0 {
        if remaining & 1 == 1 {
            result = psi_mul(&result, &base, curve, p, count);
        }
        remaining >>= 1;
        if remaining > 0 {
            base = psi_mul(&base, &base, curve, p, count);
        }
    }
    result
}

fn psi_sub(left: &Psi, right: &Psi, p: u64, count: &mut Count) -> Psi {
    assert_eq!(left.has_y, right.has_y);
    Psi { polynomial: poly_sub(&left.polynomial, &right.polynomial, p, count), has_y: left.has_y }
}

fn divide_two_y(value: &Psi, curve: &[u64], p: u64, count: &mut Count) -> Psi {
    let half = inverse(2, p, count);
    if value.has_y {
        Psi { polynomial: poly_scale(&value.polynomial, half, p, count), has_y: false }
    } else {
        let quotient = poly_div_exact(&value.polynomial, curve, p, count);
        Psi { polynomial: poly_scale(&quotient, half, p, count), has_y: true }
    }
}

fn scalar_x_map(n: usize, a: u64, b: u64, p: u64, count: &mut Count) -> (Poly, Poly) {
    assert!(n >= 2);
    let curve = vec![b, a, 0, 1];
    let a2 = mul(a, a, p, count);
    let a3 = mul(a2, a, p, count);
    let b2 = mul(b, b, p, count);
    let psi3 = vec![
        sub(0, a2, p, count), mul(12 % p, b, p, count),
        mul(6 % p, a, p, count), 0, 3 % p,
    ];
    let psi4_core = vec![
        sub(sub(0, mul(8 % p, b2, p, count), p, count), a3, p, count),
        sub(0, mul(4, mul(a, b, p, count), p, count), p, count),
        sub(0, mul(5 % p, a2, p, count), p, count),
        mul(20 % p, b, p, count), mul(5 % p, a, p, count), 0, 1,
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
            let left = psi_mul(&psi[m + 2], &psi_pow(&psi[m], 3, &curve, p, count), &curve, p, count);
            let right = psi_mul(&psi[m - 1], &psi_pow(&psi[m + 1], 3, &curve, p, count), &curve, p, count);
            psi_sub(&left, &right, p, count)
        } else {
            let m = index / 2;
            let left = psi_mul(&psi[m + 2], &psi_pow(&psi[m - 1], 2, &curve, p, count), &curve, p, count);
            let right = psi_mul(&psi[m - 2], &psi_pow(&psi[m + 1], 2, &curve, p, count), &curve, p, count);
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

fn csv(polynomial: &[u64]) -> String {
    polynomial.iter().map(u64::to_string).collect::<Vec<_>>().join(",")
}

fn discriminant(cell: Cell, count: &mut Count) -> u64 {
    let a2 = mul(cell.a, cell.a, cell.p, count);
    let a3 = mul(a2, cell.a, cell.p, count);
    let b2 = mul(cell.b, cell.b, cell.p, count);
    add(mul(4, a3, cell.p, count), mul(27 % cell.p, b2, cell.p, count), cell.p, count)
}

fn run_cell(cell: Cell) -> String {
    let mut count = Count::default();
    let curve_discriminant = discriminant(cell, &mut count);
    let (numerator, denominator) = scalar_x_map(cell.scalar, cell.a, cell.b, cell.p, &mut count);
    let gcd = monic_gcd(&numerator, &denominator, cell.p, &mut count);
    let shared = vec![1, 1];
    let mutated_numerator = poly_mul(&numerator, &shared, cell.p, &mut count);
    let mutated_denominator = poly_mul(&denominator, &shared, cell.p, &mut count);
    let mutation_gcd = monic_gcd(&mutated_numerator, &mutated_denominator, cell.p, &mut count);
    let nominal_numerator_degree = cell.scalar * cell.scalar;
    let nominal_denominator_degree = nominal_numerator_degree - 1;
    let degrees_pass = numerator.len() - 1 == nominal_numerator_degree
        && denominator.len() - 1 == nominal_denominator_degree;
    let primary_pass = !cell.singular_control && curve_discriminant != 0 && degrees_pass && gcd == [1];
    let mutation_pass = !cell.singular_control && mutation_gcd == shared;
    let singular_control_pass = cell.singular_control && curve_discriminant == 0 && gcd.len() > 1;
    let accepted = if cell.singular_control {
        singular_control_pass
    } else {
        primary_pass && mutation_pass
    };
    [
        cell.slug.to_string(),
        if cell.singular_control { "singular" } else { "primary" }.to_string(),
        cell.p.to_string(), cell.a.to_string(), cell.b.to_string(), cell.scalar.to_string(),
        curve_discriminant.to_string(), (numerator.len() - 1).to_string(),
        (denominator.len() - 1).to_string(), numerator.last().unwrap().to_string(),
        denominator.last().unwrap().to_string(), (gcd.len() - 1).to_string(), csv(&gcd),
        (mutation_gcd.len() - 1).to_string(), csv(&mutation_gcd),
        degrees_pass.to_string(), primary_pass.to_string(), mutation_pass.to_string(),
        singular_control_pass.to_string(), accepted.to_string(), count.additions.to_string(),
        count.multiplications.to_string(), count.inversions.to_string(), count.remainder_steps.to_string(),
    ].join("\t")
}

fn self_test() {
    let mut count = Count::default();
    let common = vec![3, 1];
    let left = poly_mul(&common, &[2, 0, 1], 23, &mut count);
    let right = poly_mul(&common, &[5, 1], 23, &mut count);
    assert_eq!(monic_gcd(&left, &right, 23, &mut count), common);
    let (numerator, denominator) = scalar_x_map(4, 1, 2, 23, &mut count);
    assert_eq!((numerator.len() - 1, denominator.len() - 1), (16, 15));
    println!("h116 scalar divpoly coprimality producer self-test ok");
}

fn main() {
    if std::env::args().any(|argument| argument == "--self-test") {
        self_test();
        return;
    }
    println!("schema\th116.scalar-divpoly-coprimality-result/v1");
    println!("slug\tkind\tp\ta\tb\tscalar\tdiscriminant\tnumerator_degree\tdenominator_degree\tnumerator_lead\tdenominator_lead\tgcd_degree\tgcd_coefficients\tmutation_gcd_degree\tmutation_gcd_coefficients\tdegrees_pass\tprimary_pass\tmutation_pass\tsingular_control_pass\taccepted\tadditions\tmultiplications\tinversions\tremainder_steps");
    for cell in CELLS {
        println!("{}", run_cell(cell));
    }
}
