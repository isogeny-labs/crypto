use std::fmt::Write;

const P: u64 = 31;
const A: u64 = 1;
const B: u64 = 19;
const SCALE: u64 = 25;
const PRECISION: usize = 18;

const PHI_NUMERATOR_REVERSED: [u64; 38] = [
    1, 7, 11, 7, 15, 30, 20, 14, 25, 11, 2, 16, 10, 4, 19, 14, 0, 26, 17,
    6, 0, 12, 12, 28, 12, 5, 8, 14, 26, 17, 15, 7, 28, 1, 11, 20, 21, 3,
];
const PHI_DENOMINATOR_REVERSED: [u64; 37] = [
    1, 7, 22, 22, 27, 19, 11, 13, 26, 12, 25, 18, 22, 6, 6, 7, 18, 18, 6,
    29, 29, 9, 3, 17, 28, 17, 6, 15, 24, 29, 27, 4, 9, 18, 28, 11, 19,
];

type Poly = Vec<u64>;

#[derive(Default)]
struct Counter {
    additions: u64,
    multiplications: u64,
    inversions: u64,
}

fn add(x: u64, y: u64, c: &mut Counter) -> u64 {
    c.additions += 1;
    (x + y) % P
}

fn sub(x: u64, y: u64, c: &mut Counter) -> u64 {
    c.additions += 1;
    (x + P - y) % P
}

fn mul(x: u64, y: u64, c: &mut Counter) -> u64 {
    c.multiplications += 1;
    x * y % P
}

fn pow_mod(mut x: u64, mut exponent: u64) -> u64 {
    let mut out = 1;
    while exponent > 0 {
        if exponent & 1 == 1 {
            out = out * x % P;
        }
        x = x * x % P;
        exponent >>= 1;
    }
    out
}

fn inv(x: u64, c: &mut Counter) -> u64 {
    assert_ne!(x, 0);
    c.inversions += 1;
    pow_mod(x, P - 2)
}

fn trim(mut value: Poly) -> Poly {
    while value.len() > 1 && value.last() == Some(&0) {
        value.pop();
    }
    value
}

fn poly_add(left: &[u64], right: &[u64], c: &mut Counter) -> Poly {
    let mut out = vec![0; left.len().max(right.len())];
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = add(
            left.get(i).copied().unwrap_or(0),
            right.get(i).copied().unwrap_or(0),
            c,
        );
    }
    trim(out)
}

fn poly_sub(left: &[u64], right: &[u64], c: &mut Counter) -> Poly {
    let mut out = vec![0; left.len().max(right.len())];
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = sub(
            left.get(i).copied().unwrap_or(0),
            right.get(i).copied().unwrap_or(0),
            c,
        );
    }
    trim(out)
}

fn poly_mul(left: &[u64], right: &[u64], c: &mut Counter) -> Poly {
    let mut out = vec![0; left.len() + right.len() - 1];
    for (i, &x) in left.iter().enumerate() {
        for (j, &y) in right.iter().enumerate() {
            let term = mul(x, y, c);
            out[i + j] = add(out[i + j], term, c);
        }
    }
    trim(out)
}

fn poly_scale(value: &[u64], scalar: u64, c: &mut Counter) -> Poly {
    trim(value.iter().map(|&x| mul(x, scalar, c)).collect())
}

fn poly_pow(value: &[u64], exponent: usize, c: &mut Counter) -> Poly {
    let mut out = vec![1];
    for _ in 0..exponent {
        out = poly_mul(&out, value, c);
    }
    out
}

fn poly_derivative(value: &[u64], c: &mut Counter) -> Poly {
    if value.len() < 2 {
        return vec![0];
    }
    trim(
        value
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &x)| mul(i as u64 % P, x, c))
            .collect(),
    )
}

fn shift_x(value: &[u64]) -> Poly {
    let mut out = Vec::with_capacity(value.len() + 1);
    out.push(0);
    out.extend_from_slice(value);
    trim(out)
}

fn curve_polynomial() -> Poly {
    vec![B, A, 0, 1]
}

fn scalar_six_map(c: &mut Counter) -> (Poly, Poly) {
    let f = curve_polynomial();
    let a2 = mul(A, A, c);
    let b2 = mul(B, B, c);
    let psi3 = vec![sub(0, a2, c), mul(12, B, c), mul(6, A, c), 0, 3];
    let g = vec![
        sub(sub(0, mul(8, b2, c), c), mul(a2, A, c), c),
        sub(0, mul(4, mul(A, B, c), c), c),
        sub(0, mul(5, a2, c), c),
        mul(20, B, c),
        mul(5, A, c),
        0,
        1,
    ];
    let denominator_three = poly_pow(&psi3, 2, c);
    let numerator_three = poly_sub(
        &shift_x(&denominator_three),
        &poly_scale(&poly_mul(&f, &g, c), 8, c),
        c,
    );

    let n2 = poly_pow(&numerator_three, 2, c);
    let n3 = poly_mul(&n2, &numerator_three, c);
    let n4 = poly_pow(&n2, 2, c);
    let d2 = poly_pow(&denominator_three, 2, c);
    let d3 = poly_mul(&d2, &denominator_three, c);
    let d4 = poly_pow(&d2, 2, c);
    let numerator_six = poly_sub(
        &poly_sub(
            &poly_sub(
                &n4,
                &poly_scale(&poly_mul(&n2, &d2, c), mul(2, A, c), c),
                c,
            ),
            &poly_scale(
                &poly_mul(&numerator_three, &d3, c),
                mul(8, B, c),
                c,
            ),
            c,
        ),
        &poly_scale(&d4, a2, c),
        c,
    );
    let inside = poly_add(
        &poly_add(&n3, &poly_scale(&poly_mul(&numerator_three, &d2, c), A, c), c),
        &poly_scale(&d3, B, c),
        c,
    );
    let denominator_six = poly_scale(&poly_mul(&denominator_three, &inside, c), 4, c);
    (numerator_six, denominator_six)
}

fn normalized_curve_identity(
    numerator: &[u64],
    denominator: &[u64],
    target_a: u64,
    target_b: u64,
    c: &mut Counter,
) -> bool {
    let np = poly_derivative(numerator, c);
    let dp = poly_derivative(denominator, c);
    let h = poly_sub(&poly_mul(&np, denominator, c), &poly_mul(numerator, &dp, c), c);
    let lhs = poly_mul(&curve_polynomial(), &poly_pow(&h, 2, c), c);
    let d2 = poly_pow(denominator, 2, c);
    let d3 = poly_mul(&d2, denominator, c);
    let rhs_inner = poly_add(
        &poly_add(
            &poly_pow(numerator, 3, c),
            &poly_scale(&poly_mul(numerator, &d2, c), target_a, c),
            c,
        ),
        &poly_scale(&d3, target_b, c),
        c,
    );
    lhs == poly_mul(denominator, &rhs_inner, c)
}

fn quotient_prefix(numerator_reversed: &[u64], denominator_reversed: &[u64], c: &mut Counter) -> Vec<u64> {
    let d0_inv = inv(denominator_reversed[0], c);
    let mut out = vec![0; PRECISION];
    for n in 0..PRECISION {
        let mut value = numerator_reversed.get(n).copied().unwrap_or(0);
        for i in 1..=n.min(denominator_reversed.len() - 1) {
            value = sub(value, mul(denominator_reversed[i], out[n - i], c), c);
        }
        out[n] = mul(value, d0_inv, c);
    }
    out
}

fn first_difference(left: &[u64], right: &[u64]) -> i64 {
    left.iter().zip(right).position(|(x, y)| x != y).map_or(-1, |i| i as i64)
}

fn csv(values: &[u64]) -> String {
    let mut out = String::new();
    for (i, value) in values.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        write!(&mut out, "{value}").unwrap();
    }
    out
}

fn main() {
    let mut counter = Counter::default();
    let scale2 = mul(SCALE, SCALE, &mut counter);
    let scale4 = mul(scale2, scale2, &mut counter);
    let scale6 = mul(scale4, scale2, &mut counter);
    let target_a = mul(scale4, A, &mut counter);
    let target_b = mul(scale6, B, &mut counter);

    let (scalar_n, scalar_d) = scalar_six_map(&mut counter);
    assert_eq!((scalar_n.len() - 1, scalar_d.len() - 1), (36, 35));
    let scalar_normalized_n = poly_scale(&scalar_n, scale2, &mut counter);
    let mut scalar_n_reversed = scalar_normalized_n.clone();
    scalar_n_reversed.reverse();
    let mut scalar_d_reversed = scalar_d.clone();
    scalar_d_reversed.reverse();
    let scalar_prefix = quotient_prefix(&scalar_n_reversed, &scalar_d_reversed, &mut counter);
    let phi_prefix = quotient_prefix(
        &PHI_NUMERATOR_REVERSED,
        &PHI_DENOMINATOR_REVERSED,
        &mut counter,
    );
    let first = first_difference(&phi_prefix, &scalar_prefix);
    let delta = if first >= 0 {
        sub(phi_prefix[first as usize], scalar_prefix[first as usize], &mut counter)
    } else {
        0
    };

    let mut phi_n = PHI_NUMERATOR_REVERSED.to_vec();
    phi_n.reverse();
    let mut phi_d = PHI_DENOMINATOR_REVERSED.to_vec();
    phi_d.reverse();
    let phi_identity = normalized_curve_identity(&phi_n, &phi_d, target_a, target_b, &mut counter);
    let scalar_identity = normalized_curve_identity(
        &scalar_normalized_n,
        &scalar_d,
        target_a,
        target_b,
        &mut counter,
    );

    let mut scalar_unscaled_reversed = scalar_n.clone();
    scalar_unscaled_reversed.reverse();
    let omitted_prefix = quotient_prefix(&scalar_unscaled_reversed, &scalar_d_reversed, &mut counter);
    let omitted_first = first_difference(&phi_prefix, &omitted_prefix);
    let wrong_target_rejected = !normalized_curve_identity(
        &scalar_normalized_n,
        &scalar_d,
        mul(scale2, A, &mut counter),
        mul(scale4, B, &mut counter),
        &mut counter,
    );
    let mut mutated_n_reversed = scalar_n_reversed.clone();
    mutated_n_reversed[15] = add(mutated_n_reversed[15], 1, &mut counter);
    let mutated_prefix = quotient_prefix(&mutated_n_reversed, &scalar_d_reversed, &mut counter);
    let mutation_first = first_difference(&mutated_prefix, &scalar_prefix);
    let mut mutated_n = mutated_n_reversed;
    mutated_n.reverse();
    let mutation_identity_rejected = !normalized_curve_identity(
        &mutated_n,
        &scalar_d,
        target_a,
        target_b,
        &mut counter,
    );

    println!("schema\th116.frobenius-boundary-result/v1");
    println!("p\tscale\tprecision\tphi_prefix\tscalar_prefix\tfirst_difference\tdelta\tpredicted_first_difference\tpredicted_delta\tphi_identity\tscalar_identity\tomitted_scale_first_difference\twrong_target_rejected\tmutation_first_difference\tmutation_identity_rejected\tadditions\tmultiplications\tinversions");
    println!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t15\t10\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        P,
        SCALE,
        PRECISION,
        csv(&phi_prefix),
        csv(&scalar_prefix),
        first,
        delta,
        phi_identity,
        scalar_identity,
        omitted_first,
        wrong_target_rejected,
        mutation_first,
        mutation_identity_rejected,
        counter.additions,
        counter.multiplications,
        counter.inversions,
    );
}

