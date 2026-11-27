use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;

const MODULUS: u64 = 2_147_483_647;
const CURVE_ALPHA: u64 = 2_147_483_644;
const CURVE_BETA: u64 = 21;
const MAX_DEPTH: usize = 6;

type Polynomial = Vec<u64>;
type Fields = BTreeMap<String, String>;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct AffinePoint {
    at_infinity: bool,
    x: u64,
    y: u64,
}

const BASE_POINT: AffinePoint = AffinePoint {
    at_infinity: false,
    x: 1_391_443_431,
    y: 177_979_044,
};

impl AffinePoint {
    const IDENTITY: Self = Self {
        at_infinity: true,
        x: 0,
        y: 0,
    };

    fn opposite(self) -> Self {
        if self.at_infinity || self.y == 0 {
            self
        } else {
            Self {
                at_infinity: false,
                x: self.x,
                y: MODULUS - self.y,
            }
        }
    }
}

#[derive(Clone)]
struct RationalLeaf {
    root_position: usize,
    signed_positions: [usize; 2],
}

#[derive(Clone)]
struct RationalBase {
    leaves: Vec<RationalLeaf>,
    signed_points: Vec<AffinePoint>,
}

fn fadd(x: u64, y: u64) -> u64 {
    let z = x + y;
    if z >= MODULUS {
        z - MODULUS
    } else {
        z
    }
}

fn fsub(x: u64, y: u64) -> u64 {
    if x >= y {
        x - y
    } else {
        MODULUS - (y - x)
    }
}

fn fneg(x: u64) -> u64 {
    if x == 0 {
        0
    } else {
        MODULUS - x
    }
}

fn fmul(x: u64, y: u64) -> u64 {
    (x * y) % MODULUS
}

fn fpow(mut x: u64, mut exponent: u64) -> u64 {
    let mut product = 1;
    while exponent > 0 {
        if exponent & 1 == 1 {
            product = fmul(product, x);
        }
        x = fmul(x, x);
        exponent >>= 1;
    }
    product
}

fn finverse(x: u64) -> u64 {
    assert_ne!(x, 0);
    fpow(x, MODULUS - 2)
}

fn square_root(x: u64) -> Option<u64> {
    let candidate = fpow(x, (MODULUS + 1) / 4);
    (fmul(candidate, candidate) == x).then_some(candidate)
}

fn normalized(mut polynomial: Polynomial) -> Polynomial {
    while polynomial.last() == Some(&0) {
        polynomial.pop();
    }
    polynomial
}

fn polynomial_add(left: &[u64], right: &[u64]) -> Polynomial {
    let mut result = vec![0; left.len().max(right.len())];
    for (index, &coefficient) in left.iter().enumerate() {
        result[index] = fadd(result[index], coefficient);
    }
    for (index, &coefficient) in right.iter().enumerate() {
        result[index] = fadd(result[index], coefficient);
    }
    normalized(result)
}

fn polynomial_subtract(left: &[u64], right: &[u64]) -> Polynomial {
    let mut result = vec![0; left.len().max(right.len())];
    for (index, &coefficient) in left.iter().enumerate() {
        result[index] = coefficient;
    }
    for (index, &coefficient) in right.iter().enumerate() {
        result[index] = fsub(result[index], coefficient);
    }
    normalized(result)
}

fn polynomial_multiply(left: &[u64], right: &[u64]) -> Polynomial {
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }
    let mut result = vec![0; left.len() + right.len() - 1];
    for (i, &x) in left.iter().enumerate() {
        for (j, &y) in right.iter().enumerate() {
            result[i + j] = fadd(result[i + j], fmul(x, y));
        }
    }
    normalized(result)
}

fn polynomial_remainder(dividend: &[u64], divisor: &[u64]) -> Polynomial {
    assert!(!divisor.is_empty());
    let mut remainder = normalized(dividend.to_vec());
    let leading_inverse = finverse(*divisor.last().expect("nonempty divisor"));
    while remainder.len() >= divisor.len() {
        let shift = remainder.len() - divisor.len();
        let scale = fmul(
            *remainder.last().expect("nonempty remainder"),
            leading_inverse,
        );
        for (index, &coefficient) in divisor.iter().enumerate() {
            remainder[index + shift] = fsub(remainder[index + shift], fmul(scale, coefficient));
        }
        remainder = normalized(remainder);
    }
    remainder
}

fn dickson_monomials(n: usize) -> Vec<Polynomial> {
    let mut sequence = vec![vec![2], vec![0, 1]];
    for index in 1..n {
        let mut shifted = vec![0];
        shifted.extend_from_slice(&sequence[index]);
        sequence.push(polynomial_subtract(&shifted, &sequence[index - 1]));
    }
    sequence
}

fn monomial_to_dickson(polynomial: &[u64], basis: &[Polynomial], n: usize) -> Vec<u64> {
    let mut remainder = polynomial.to_vec();
    remainder.resize(n, 0);
    let mut coordinates = vec![0; n];
    for degree in (1..n).rev() {
        let coefficient = remainder[degree];
        coordinates[degree] = coefficient;
        for (index, &basis_coefficient) in basis[degree].iter().enumerate() {
            remainder[index] = fsub(remainder[index], fmul(coefficient, basis_coefficient));
        }
    }
    coordinates[0] = fmul(remainder[0], finverse(2));
    coordinates
}

fn independent_product_table(n: usize) -> Vec<Vec<Vec<u64>>> {
    let basis = dickson_monomials(n);
    let modulus = &basis[n];
    (0..n)
        .map(|left| {
            (0..n)
                .map(|right| {
                    let product = polynomial_multiply(&basis[left], &basis[right]);
                    let remainder = polynomial_remainder(&product, modulus);
                    monomial_to_dickson(&remainder, &basis, n)
                })
                .collect()
        })
        .collect()
}

fn expected_fold_product(left: usize, right: usize, n: usize) -> Vec<u64> {
    let mut result = vec![0; n];
    result[left.abs_diff(right)] = fadd(result[left.abs_diff(right)], 1);
    let sum = left + right;
    if sum < n {
        result[sum] = fadd(result[sum], 1);
    } else if sum > n {
        let folded = 2 * n - sum;
        result[folded] = fsub(result[folded], 1);
    }
    result
}

fn independent_channels(coefficients: &[u64]) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    let n = coefficients.len();
    let products = independent_product_table(n);
    let mut exact = vec![0; n * n];
    let mut difference = vec![0; n * n];
    for (basis_index, &coefficient) in coefficients.iter().enumerate() {
        for column in 0..n {
            let difference_row = basis_index.abs_diff(column);
            difference[difference_row * n + column] =
                fadd(difference[difference_row * n + column], coefficient);
            for row in 0..n {
                exact[row * n + column] = fadd(
                    exact[row * n + column],
                    fmul(coefficient, products[basis_index][column][row]),
                );
            }
        }
    }
    let sum_fold = exact
        .iter()
        .zip(&difference)
        .map(|(&total, &difference_part)| fsub(total, difference_part))
        .collect();
    (difference, sum_fold, exact)
}

fn rank(mut matrix: Vec<u64>, rows: usize, columns: usize) -> usize {
    let mut rank = 0;
    for column in 0..columns {
        let Some(pivot) = (rank..rows).find(|&row| matrix[row * columns + column] != 0) else {
            continue;
        };
        if pivot != rank {
            for index in 0..columns {
                matrix.swap(rank * columns + index, pivot * columns + index);
            }
        }
        let inverse = finverse(matrix[rank * columns + column]);
        for index in column..columns {
            matrix[rank * columns + index] = fmul(matrix[rank * columns + index], inverse);
        }
        for row in rank + 1..rows {
            let factor = matrix[row * columns + column];
            if factor == 0 {
                continue;
            }
            for index in column..columns {
                matrix[row * columns + index] = fsub(
                    matrix[row * columns + index],
                    fmul(factor, matrix[rank * columns + index]),
                );
            }
        }
        rank += 1;
        if rank == rows {
            break;
        }
    }
    rank
}

fn shifted_difference(matrix: &[u64], n: usize, left_shift: bool) -> Vec<u64> {
    let mut result = vec![0; n * n];
    for row in 0..n {
        for column in 0..n {
            let previous_column = if left_shift {
                column.checked_add(1).filter(|&value| value < n)
            } else {
                column.checked_sub(1)
            };
            let shifted = if row == 0 {
                0
            } else if let Some(previous_column) = previous_column {
                matrix[(row - 1) * n + previous_column]
            } else {
                0
            };
            result[row * n + column] = fsub(matrix[row * n + column], shifted);
        }
    }
    result
}

fn backward_fibre(depth: usize) -> Vec<u64> {
    let mut fibre = vec![0];
    for _ in 0..depth {
        let mut expanded = Vec::with_capacity(2 * fibre.len());
        for value in fibre {
            let square = fadd(value, 2);
            let root = square_root(square).expect("registered Dickson fibre splits");
            let low = root.min(MODULUS - root);
            expanded.extend([low, fneg(low)]);
        }
        fibre = expanded;
    }
    fibre
}

fn dickson_value(mut degree: usize, x: u64) -> u64 {
    if degree == 0 {
        return 2;
    }
    if degree == 1 {
        return x;
    }
    let mut previous = 2;
    let mut current = x;
    while degree > 1 {
        let next = fsub(fmul(x, current), previous);
        previous = current;
        current = next;
        degree -= 1;
    }
    current
}

fn s3_coefficients(first: u64, second: u64) -> [u64; 3] {
    let product = fmul(first, second);
    let sum = fadd(first, second);
    let squared_difference = fmul(fsub(first, second), fsub(first, second));
    let linear = fneg(fmul(
        2,
        fadd(fmul(sum, fadd(product, CURVE_ALPHA)), fmul(2, CURVE_BETA)),
    ));
    let constant = fsub(
        fmul(fsub(product, CURVE_ALPHA), fsub(product, CURVE_ALPHA)),
        fmul(fmul(4, CURVE_BETA), sum),
    );
    [constant, linear, squared_difference]
}

fn polynomial_determinant(matrix: &[Vec<Polynomial>]) -> Polynomial {
    fn expand(
        matrix: &[Vec<Polynomial>],
        row: usize,
        used: usize,
        term: Polynomial,
        negative: bool,
        total: &mut Polynomial,
    ) {
        let dimension = matrix.len();
        if row == dimension {
            if negative {
                *total = polynomial_subtract(total, &term);
            } else {
                *total = polynomial_add(total, &term);
            }
            return;
        }
        for column in 0..dimension {
            if used & (1 << column) != 0 {
                continue;
            }
            let later_used = (used >> (column + 1)).count_ones() & 1 == 1;
            let next = polynomial_multiply(&term, &matrix[row][column]);
            expand(
                matrix,
                row + 1,
                used | (1 << column),
                next,
                negative ^ later_used,
                total,
            );
        }
    }

    let mut determinant = Vec::new();
    expand(matrix, 0, 0, vec![1], false, &mut determinant);
    determinant
}

fn generic_s4(first: u64, second: u64, third: u64) -> Polynomial {
    let left = s3_coefficients(first, second);
    let third_z_sum = vec![third, 1];
    let third_z_product = vec![0, third];
    let difference = vec![third, fneg(1)];
    let right_quadratic = polynomial_multiply(&difference, &difference);
    let right_linear = {
        let inside = polynomial_add(
            &polynomial_multiply(
                &third_z_sum,
                &polynomial_add(&third_z_product, &[CURVE_ALPHA]),
            ),
            &[fmul(2, CURVE_BETA)],
        );
        inside
            .into_iter()
            .map(|value| fneg(fmul(2, value)))
            .collect()
    };
    let right_constant = polynomial_subtract(
        &polynomial_multiply(
            &polynomial_subtract(&third_z_product, &[CURVE_ALPHA]),
            &polynomial_subtract(&third_z_product, &[CURVE_ALPHA]),
        ),
        &polynomial_multiply(&[fmul(4, CURVE_BETA)], &third_z_sum),
    );
    let left_polynomials = [vec![left[2]], vec![left[1]], vec![left[0]]];
    let right_polynomials = [right_quadratic, right_linear, right_constant];
    let mut sylvester = vec![vec![Vec::new(); 4]; 4];
    for shift in 0..2 {
        sylvester[shift][shift..shift + 3].clone_from_slice(&left_polynomials);
        sylvester[2 + shift][shift..shift + 3].clone_from_slice(&right_polynomials);
    }
    let result = polynomial_determinant(&sylvester);
    debug_assert!(result.len() <= 5);
    result
}

fn scalar_determinant(matrix: &[Vec<u64>]) -> u64 {
    let dimension = matrix.len();
    let mut dynamic = vec![0; 1usize << dimension];
    dynamic[0] = 1;
    for (row, matrix_row) in matrix.iter().enumerate() {
        let mut next = vec![0; 1usize << dimension];
        for (mask, &partial) in dynamic.iter().enumerate() {
            if mask.count_ones() as usize != row || partial == 0 {
                continue;
            }
            for (column, &entry) in matrix_row.iter().enumerate() {
                if mask & (1 << column) != 0 {
                    continue;
                }
                let term = fmul(partial, entry);
                let odd = (mask >> (column + 1)).count_ones() & 1 == 1;
                let destination = mask | (1 << column);
                next[destination] = if odd {
                    fsub(next[destination], term)
                } else {
                    fadd(next[destination], term)
                };
            }
        }
        dynamic = next;
    }
    dynamic[(1 << dimension) - 1]
}

fn independent_s5(values: [u64; 5]) -> u64 {
    let s4 = generic_s4(values[0], values[1], values[2]);
    let s3 = s3_coefficients(values[3], values[4]);
    let mut left_descending = [0; 5];
    for (degree, &coefficient) in s4.iter().take(5).enumerate() {
        left_descending[4 - degree] = coefficient;
    }
    let right_descending = [s3[2], s3[1], s3[0]];
    let mut sylvester = vec![vec![0; 6]; 6];
    for shift in 0..2 {
        sylvester[shift][shift..shift + 5].copy_from_slice(&left_descending);
    }
    for shift in 0..4 {
        sylvester[2 + shift][shift..shift + 3].copy_from_slice(&right_descending);
    }
    scalar_determinant(&sylvester)
}

fn curve_right_side(x: u64) -> u64 {
    fadd(fadd(fmul(fmul(x, x), x), fmul(CURVE_ALPHA, x)), CURVE_BETA)
}

fn add_points(left: AffinePoint, right: AffinePoint) -> AffinePoint {
    if left.at_infinity {
        return right;
    }
    if right.at_infinity {
        return left;
    }
    let slope = if left.x == right.x {
        if fadd(left.y, right.y) == 0 {
            return AffinePoint::IDENTITY;
        }
        fmul(
            fadd(fmul(3, fmul(left.x, left.x)), CURVE_ALPHA),
            finverse(fmul(2, left.y)),
        )
    } else {
        fmul(fsub(right.y, left.y), finverse(fsub(right.x, left.x)))
    };
    let x = fsub(fsub(fmul(slope, slope), left.x), right.x);
    let y = fsub(fmul(slope, fsub(left.x, x)), left.y);
    AffinePoint {
        at_infinity: false,
        x,
        y,
    }
}

fn multiply_point(mut point: AffinePoint, mut scalar: u64) -> AffinePoint {
    let mut result = AffinePoint::IDENTITY;
    while scalar > 0 {
        if scalar & 1 == 1 {
            result = add_points(result, point);
        }
        point = add_points(point, point);
        scalar >>= 1;
    }
    result
}

fn rational_base(roots: &[u64]) -> RationalBase {
    let mut leaves = Vec::new();
    let mut signed_points = Vec::new();
    for (root_position, &x) in roots.iter().enumerate() {
        let right_side = curve_right_side(x);
        if right_side == 0 {
            let point = AffinePoint {
                at_infinity: false,
                x,
                y: 0,
            };
            let position = signed_points.len();
            signed_points.push(point);
            leaves.push(RationalLeaf {
                root_position,
                signed_positions: [position, position],
            });
        } else if let Some(root) = square_root(right_side) {
            let y = root.min(MODULUS - root);
            let point = AffinePoint {
                at_infinity: false,
                x,
                y,
            };
            let position = signed_points.len();
            signed_points.extend([point, point.opposite()]);
            leaves.push(RationalLeaf {
                root_position,
                signed_positions: [position, position + 1],
            });
        }
    }
    RationalBase {
        leaves,
        signed_points,
    }
}

fn multisets(count: usize, size: usize) -> Vec<Vec<usize>> {
    fn recurse(
        lower_bound: usize,
        count: usize,
        size: usize,
        selected: &mut Vec<usize>,
        output: &mut Vec<Vec<usize>>,
    ) {
        if selected.len() == size {
            output.push(selected.clone());
            return;
        }
        for value in lower_bound..count {
            selected.push(value);
            recurse(value, count, size, selected, output);
            selected.pop();
        }
    }
    let mut output = Vec::new();
    recurse(0, count, size, &mut Vec::new(), &mut output);
    output
}

fn planted_target(base: &RationalBase) -> AffinePoint {
    for multiset in multisets(base.signed_points.len(), 4) {
        let sum = multiset.iter().fold(AffinePoint::IDENTITY, |acc, &index| {
            add_points(acc, base.signed_points[index])
        });
        if !sum.at_infinity {
            return sum;
        }
    }
    panic!("no finite planted target")
}

fn direct_witnesses(base: &RationalBase, target: AffinePoint) -> BTreeSet<[usize; 4]> {
    let mut output = BTreeSet::new();
    for multiset in multisets(base.signed_points.len(), 4) {
        let sum = multiset.iter().fold(AffinePoint::IDENTITY, |acc, &index| {
            add_points(acc, base.signed_points[index])
        });
        if sum == target {
            output.insert([multiset[0], multiset[1], multiset[2], multiset[3]]);
        }
    }
    output
}

fn recovered_witnesses(
    roots: &[u64],
    base: &RationalBase,
    target: AffinePoint,
) -> (BTreeSet<[usize; 4]>, usize) {
    let mut output = BTreeSet::new();
    let mut zero_multisets = 0;
    for multiset in multisets(base.leaves.len(), 4) {
        let abscissae = [
            roots[base.leaves[multiset[0]].root_position],
            roots[base.leaves[multiset[1]].root_position],
            roots[base.leaves[multiset[2]].root_position],
            roots[base.leaves[multiset[3]].root_position],
            target.x,
        ];
        if independent_s5(abscissae) != 0 {
            continue;
        }
        zero_multisets += 1;
        for sign_mask in 0..16 {
            let mut witness = [0; 4];
            let mut sum = AffinePoint::IDENTITY;
            for position in 0..4 {
                let leaf = &base.leaves[multiset[position]];
                let signed_position = leaf.signed_positions[(sign_mask >> position) & 1];
                witness[position] = signed_position;
                sum = add_points(sum, base.signed_points[signed_position]);
            }
            witness.sort_unstable();
            if sum == target {
                output.insert(witness);
            }
        }
    }
    (output, zero_multisets)
}

fn encoded_witnesses(witnesses: &BTreeSet<[usize; 4]>) -> String {
    if witnesses.is_empty() {
        return "-".to_owned();
    }
    witnesses
        .iter()
        .map(|witness| {
            format!(
                "{},{},{},{}",
                witness[0], witness[1], witness[2], witness[3]
            )
        })
        .collect::<Vec<_>>()
        .join(";")
}

fn kernel_statistics(
    roots: &[u64],
    target_x: u64,
) -> (usize, usize, usize, usize, String, Vec<u64>) {
    let n = roots.len();
    let pair_count = n * n;
    let mut sums = vec![0; pair_count];
    let mut products = vec![0; pair_count];
    for left in 0..n {
        for right in 0..n {
            let position = left * n + right;
            sums[position] = fadd(roots[left], roots[right]);
            products[position] = fmul(roots[left], roots[right]);
        }
    }
    let mut kernel = vec![0; pair_count * pair_count];
    let mut sum_displacement = vec![0; pair_count * pair_count];
    let mut product_displacement = vec![0; pair_count * pair_count];
    let mut zero_count = 0;
    let mut first_zero = None;
    for row in 0..pair_count {
        for column in 0..pair_count {
            let value = independent_s5([
                roots[row / n],
                roots[row % n],
                roots[column / n],
                roots[column % n],
                target_x,
            ]);
            let position = row * pair_count + column;
            kernel[position] = value;
            sum_displacement[position] = fmul(fsub(sums[row], sums[column]), value);
            product_displacement[position] = fmul(fsub(products[row], products[column]), value);
            if value == 0 {
                zero_count += 1;
                first_zero.get_or_insert(position);
            }
        }
    }
    let kernel_rank = rank(kernel.clone(), pair_count, pair_count);
    (
        kernel_rank,
        rank(sum_displacement, pair_count, pair_count),
        rank(product_displacement, pair_count, pair_count),
        zero_count,
        first_zero.map_or_else(|| "none".to_owned(), |value| value.to_string()),
        kernel,
    )
}

fn encoded_field_values(values: &[u64]) -> String {
    values
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn choose(n: usize, k: usize) -> usize {
    let k = k.min(n - k);
    (0..k).fold(1, |accumulator, index| {
        accumulator * (n - index) / (index + 1)
    })
}

fn parse_records(text: &str) -> Vec<(String, Fields)> {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut words = line.split_whitespace();
            let tag = words.next().unwrap_or_default().to_owned();
            let fields = words
                .filter_map(|word| word.split_once('='))
                .map(|(key, value)| (key.to_owned(), value.to_owned()))
                .collect();
            (tag, fields)
        })
        .collect()
}

fn find_record<'a>(
    records: &'a [(String, Fields)],
    tag: &str,
    selectors: &[(&str, String)],
) -> Option<&'a Fields> {
    records.iter().find_map(|(record_tag, fields)| {
        (record_tag == tag
            && selectors
                .iter()
                .all(|(key, value)| fields.get(*key) == Some(value)))
        .then_some(fields)
    })
}

fn check_field(
    errors: &mut Vec<String>,
    fields: &Fields,
    context: &str,
    key: &str,
    expected: impl ToString,
) {
    let expected = expected.to_string();
    if fields.get(key) != Some(&expected) {
        errors.push(format!(
            "{context} key={key} expected={expected} observed={}",
            fields.get(key).map_or("<missing>", String::as_str)
        ));
    }
}

fn require_record<'a>(
    errors: &mut Vec<String>,
    records: &'a [(String, Fields)],
    tag: &str,
    selectors: &[(&str, String)],
) -> Option<&'a Fields> {
    let record = find_record(records, tag, selectors);
    if record.is_none() {
        errors.push(format!("missing tag={tag} selectors={selectors:?}"));
    }
    record
}

fn verify(candidate_text: &str) -> Vec<String> {
    let records = parse_records(candidate_text);
    let mut errors = Vec::new();
    if let Some(header) = require_record(&mut errors, &records, "HEADER", &[]) {
        check_field(
            &mut errors,
            header,
            "HEADER",
            "schema",
            "pplus-dickson-s5-structure-v1",
        );
        check_field(&mut errors, header, "HEADER", "prime", MODULUS);
        check_field(&mut errors, header, "HEADER", "seed", "none");
    }

    for depth in 1..=MAX_DEPTH {
        let n = 1usize << depth;
        let products = independent_product_table(n);
        let product_ok = (0..n).all(|left| {
            (0..n).all(|right| products[left][right] == expected_fold_product(left, right, n))
        });
        let coefficients: Vec<_> = (0..n)
            .map(|index| (((index + 1) as u64).pow(3) + 17) % MODULUS)
            .collect();
        let (difference, sum_fold, _exact) = independent_channels(&coefficients);
        let diff_dr = rank(shifted_difference(&difference, n, false), n, n);
        let diff_dl = rank(shifted_difference(&difference, n, true), n, n);
        let sum_dr = rank(shifted_difference(&sum_fold, n, false), n, n);
        let sum_dl = rank(shifted_difference(&sum_fold, n, true), n, n);
        let context = format!("QUOTIENT depth={depth}");
        if let Some(fields) = require_record(
            &mut errors,
            &records,
            "QUOTIENT",
            &[("depth", depth.to_string())],
        ) {
            check_field(&mut errors, fields, &context, "n", n);
            check_field(
                &mut errors,
                fields,
                &context,
                "product_ok",
                product_ok as usize,
            );
            check_field(&mut errors, fields, &context, "channel_ok", 1);
            check_field(&mut errors, fields, &context, "diff_dr_rank", diff_dr);
            check_field(&mut errors, fields, &context, "diff_dl_rank", diff_dl);
            check_field(&mut errors, fields, &context, "sum_dr_rank", sum_dr);
            check_field(&mut errors, fields, &context, "sum_dl_rank", sum_dl);
        }
    }

    for depth in 2..=4 {
        let n = 1usize << depth;
        let roots = backward_fibre(depth);
        let root_set: BTreeSet<_> = roots.iter().copied().collect();
        if root_set.len() != n || roots.iter().any(|&root| dickson_value(n, root) != 0) {
            errors.push(format!("root reconstruction failed depth={depth}"));
        }
        let base = rational_base(&roots);
        for degree in 1..=4 {
            let expected = choose(n, degree);
            let context = format!("DIVISOR depth={depth} degree={degree}");
            if let Some(fields) = require_record(
                &mut errors,
                &records,
                "DIVISOR",
                &[("depth", depth.to_string()), ("degree", degree.to_string())],
            ) {
                check_field(&mut errors, fields, &context, "n", n);
                check_field(&mut errors, fields, &context, "count", expected);
                check_field(&mut errors, fields, &context, "expected", expected);
                check_field(&mut errors, fields, &context, "recurrence_ok", 1);
            }
        }

        let infinity = direct_witnesses(&base, AffinePoint::IDENTITY);
        let context = format!("INFINITY depth={depth}");
        if let Some(fields) = require_record(
            &mut errors,
            &records,
            "INFINITY",
            &[("depth", depth.to_string())],
        ) {
            check_field(&mut errors, fields, &context, "n", n);
            check_field(&mut errors, fields, &context, "lift_x", base.leaves.len());
            check_field(
                &mut errors,
                fields,
                &context,
                "signed_leaves",
                base.signed_points.len(),
            );
            check_field(&mut errors, fields, &context, "witnesses", infinity.len());
            check_field(
                &mut errors,
                fields,
                &context,
                "witness_set",
                encoded_witnesses(&infinity),
            );
        }

        let targets = [
            ("planted", planted_target(&base)),
            ("g1", BASE_POINT),
            ("g17", multiply_point(BASE_POINT, 17)),
        ];
        for (label, target) in targets {
            let (kernel_rank, delta_s_rank, delta_p_rank, zero_count, first_zero, kernel) =
                kernel_statistics(&roots, target.x);
            let direct = direct_witnesses(&base, target);
            let (recovered, zero_multisets) = recovered_witnesses(&roots, &base, target);
            if direct != recovered {
                errors.push(format!(
                    "independent recovery mismatch depth={depth} label={label}"
                ));
            }
            let context = format!("CELL depth={depth} label={label}");
            if let Some(fields) = require_record(
                &mut errors,
                &records,
                "CELL",
                &[("depth", depth.to_string()), ("label", label.to_owned())],
            ) {
                check_field(&mut errors, fields, &context, "n", n);
                check_field(&mut errors, fields, &context, "target_x", target.x);
                check_field(&mut errors, fields, &context, "target_y", target.y);
                check_field(&mut errors, fields, &context, "pair_dimension", n * n);
                check_field(
                    &mut errors,
                    fields,
                    &context,
                    "zero_operator_dimension",
                    n * n * n * n,
                );
                check_field(&mut errors, fields, &context, "kernel_rank", kernel_rank);
                check_field(&mut errors, fields, &context, "delta_s_rank", delta_s_rank);
                check_field(&mut errors, fields, &context, "delta_p_rank", delta_p_rank);
                check_field(&mut errors, fields, &context, "zero_count", zero_count);
                check_field(&mut errors, fields, &context, "first_zero", first_zero);
                check_field(&mut errors, fields, &context, "kernel_agree", 1);
                check_field(&mut errors, fields, &context, "lift_x", base.leaves.len());
                check_field(
                    &mut errors,
                    fields,
                    &context,
                    "signed_leaves",
                    base.signed_points.len(),
                );
                check_field(
                    &mut errors,
                    fields,
                    &context,
                    "zero_multisets",
                    zero_multisets,
                );
                check_field(
                    &mut errors,
                    fields,
                    &context,
                    "signed_witnesses",
                    direct.len(),
                );
                check_field(&mut errors, fields, &context, "signed_ok", 1);
                check_field(
                    &mut errors,
                    fields,
                    &context,
                    "witness_set",
                    encoded_witnesses(&direct),
                );
            }
            let kernel_context = format!("KERNEL_VALUES depth={depth} label={label}");
            if let Some(fields) = require_record(
                &mut errors,
                &records,
                "KERNEL_VALUES",
                &[("depth", depth.to_string()), ("label", label.to_owned())],
            ) {
                check_field(&mut errors, fields, &kernel_context, "n", n);
                check_field(&mut errors, fields, &kernel_context, "count", kernel.len());
                check_field(
                    &mut errors,
                    fields,
                    &kernel_context,
                    "values",
                    encoded_field_values(&kernel),
                );
            }
        }
        let context = format!("STATE depth={depth}");
        if let Some(fields) = require_record(
            &mut errors,
            &records,
            "STATE",
            &[("depth", depth.to_string())],
        ) {
            check_field(&mut errors, fields, &context, "n", n);
            check_field(
                &mut errors,
                fields,
                &context,
                "four_multisets",
                choose(n + 3, 4),
            );
            check_field(&mut errors, fields, &context, "pair_square", n * n * n * n);
            check_field(&mut errors, fields, &context, "oracle", "none");
        }
    }
    if let Some(final_record) = require_record(&mut errors, &records, "FINAL", &[]) {
        check_field(&mut errors, final_record, "FINAL", "algebra_ok", 1);
        check_field(&mut errors, final_record, "FINAL", "exact_zero_oracle", 0);
        check_field(
            &mut errors,
            final_record,
            "FINAL",
            "production_authorized",
            0,
        );
    }
    errors
}

fn main() {
    let path = env::args()
        .nth(1)
        .expect("usage: dickson-s5-structure-verify CANDIDATE.txt");
    let candidate_text = fs::read_to_string(path).expect("read candidate evidence");
    let errors = verify(&candidate_text);
    for error in &errors {
        println!("MISMATCH {error}");
    }
    println!(
        "VERIFY schema=pplus-dickson-s5-verify-v1 candidate_match={} exactness_ok={} errors={}",
        usize::from(errors.is_empty()),
        usize::from(errors.is_empty()),
        errors.len()
    );
    assert!(
        errors.is_empty(),
        "candidate evidence failed independent replay"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_resultant_detects_signed_point_relations() {
        let roots = backward_fibre(2);
        let base = rational_base(&roots);
        let target = planted_target(&base);
        let direct = direct_witnesses(&base, target);
        let (recovered, _) = recovered_witnesses(&roots, &base, target);
        assert!(!direct.is_empty());
        assert_eq!(recovered, direct);
    }

    #[test]
    fn monomial_quotient_has_the_frozen_fold() {
        for n in [2, 4, 8, 16] {
            let products = independent_product_table(n);
            for (left, row) in products.iter().enumerate() {
                for (right, product) in row.iter().enumerate() {
                    assert_eq!(*product, expected_fold_product(left, right, n));
                }
            }
        }
    }

    #[test]
    fn reconstructed_fibres_are_complete() {
        for depth in 1..=4 {
            let roots = backward_fibre(depth);
            let n = 1 << depth;
            assert_eq!(roots.iter().copied().collect::<BTreeSet<_>>().len(), n);
            assert!(roots.iter().all(|&root| dickson_value(n, root) == 0));
        }
    }
}
