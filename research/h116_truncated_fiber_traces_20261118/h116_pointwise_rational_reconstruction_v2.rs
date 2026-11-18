#[allow(dead_code)]
#[path = "h116_field.rs"]
mod h116_field;

use h116_field::{first_irreducible, Curve, Felt, Field, Point};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::time::Instant;

#[derive(Clone, Copy)]
struct Cell { slug: &'static str, p: u64, a: u64, b: u64, degree: usize }

#[derive(Clone, Copy)]
struct Sample { x: Felt, image_x: Felt }

struct CellResult {
    row: String,
    accepted: bool,
    key: (u64, u64, u64),
    numerator: String,
    denominator: String,
}

const CELLS: [Cell; 8] = [
    Cell { slug: "fp5-a2-b1-d2", p: 5, a: 2, b: 1, degree: 2 },
    Cell { slug: "fp5-a2-b1-d3", p: 5, a: 2, b: 1, degree: 3 },
    Cell { slug: "fp5-a2-b1-d4", p: 5, a: 2, b: 1, degree: 4 },
    Cell { slug: "fp7-a1-b1-d2", p: 7, a: 1, b: 1, degree: 2 },
    Cell { slug: "fp7-a1-b1-d3", p: 7, a: 1, b: 1, degree: 3 },
    Cell { slug: "fp7-a1-b1-d4", p: 7, a: 1, b: 1, degree: 4 },
    Cell { slug: "fp11-a1-b1-d2", p: 11, a: 1, b: 1, degree: 2 },
    Cell { slug: "fp11-a1-b1-d3", p: 11, a: 1, b: 1, degree: 3 },
];

fn require(condition: bool, message: impl Into<String>) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.into()) }
}

fn signed_scalar_mul(curve: &Curve<'_>, scalar: i64, point: Point) -> Result<Point, String> {
    let absolute = curve.scalar_mul(scalar.unsigned_abs(), point)?;
    Ok(if scalar < 0 { curve.neg(absolute) } else { absolute })
}

fn mapped_point(curve: &Curve<'_>, point: Point, trace: i64) -> Result<Point, String> {
    curve.sub(signed_scalar_mul(curve, trace - 1, point)?, curve.frobenius(point))
}

fn point_x(point: Point) -> Option<Felt> {
    match point { Point::Infinity => None, Point::Affine(x, _) => Some(x) }
}

fn samples(curve: &Curve<'_>, trace: i64) -> Result<(Vec<Sample>, Vec<(Felt, u64)>, u64), String> {
    let field = curve.field;
    let mut regular = Vec::new();
    let mut poles = Vec::new();
    let mut pair_failures = 0_u64;
    for index in 0..field.q {
        let x = field.from_index(index);
        let rhs = field.add(
            field.add(field.mul(field.square(x), x), field.mul(curve.a, x)),
            curve.b,
        );
        let Some(y) = field.sqrt(rhs)? else { continue };
        let first = point_x(mapped_point(curve, Point::Affine(x, y), trace)?);
        let multiplicity = if y == field.zero() { 1 } else { 2 };
        if multiplicity == 2 {
            let second = point_x(mapped_point(
                curve,
                Point::Affine(x, field.neg(y)),
                trace,
            )?);
            if first != second { pair_failures += 1; }
        }
        match first {
            Some(image_x) => regular.push(Sample { x, image_x }),
            None => poles.push((x, multiplicity)),
        }
    }
    Ok((regular, poles, pair_failures))
}

fn powers(field: &Field, value: Felt, through: usize) -> Vec<Felt> {
    let mut result = Vec::with_capacity(through + 1);
    result.push(field.one());
    for index in 1..=through {
        result.push(field.mul(result[index - 1], value));
    }
    result
}

fn solve_window(field: &Field, window: &[Sample], degree: usize) -> Result<(Vec<Felt>, Vec<Felt>, usize), String> {
    let width = 2 * degree;
    require(window.len() == width, "wrong joint interpolation window")?;
    let mut matrix = vec![vec![field.zero(); width + 1]; width];
    for (row, sample) in window.iter().enumerate() {
        let x_powers = powers(field, sample.x, degree);
        matrix[row][..=degree].copy_from_slice(&x_powers[..=degree]);
        for index in 0..degree - 1 {
            matrix[row][degree + 1 + index] = field.neg(field.mul(sample.image_x, x_powers[index]));
        }
        matrix[row][width] = field.mul(sample.image_x, x_powers[degree - 1]);
    }

    let mut pivot_row = 0_usize;
    let mut pivots = Vec::new();
    for column in 0..width {
        let Some(found) = (pivot_row..width).find(|&row| matrix[row][column] != field.zero()) else {
            continue;
        };
        matrix.swap(pivot_row, found);
        let inverse = field.inv(matrix[pivot_row][column])?;
        for entry in &mut matrix[pivot_row][column..=width] {
            *entry = field.mul(*entry, inverse);
        }
        for row in 0..width {
            if row == pivot_row { continue; }
            let factor = matrix[row][column];
            if factor == field.zero() { continue; }
            for index in column..=width {
                matrix[row][index] = field.sub(
                    matrix[row][index],
                    field.mul(factor, matrix[pivot_row][index]),
                );
            }
        }
        pivots.push(column);
        pivot_row += 1;
        if pivot_row == width { break; }
    }
    let rank = pivots.len();
    let mut solution = vec![field.zero(); width];
    for (row, &column) in pivots.iter().enumerate() {
        solution[column] = matrix[row][width];
    }
    let numerator = solution[..=degree].to_vec();
    let mut denominator = solution[degree + 1..].to_vec();
    denominator.push(field.one());
    Ok((numerator, denominator, rank))
}

fn evaluate(field: &Field, polynomial: &[Felt], value: Felt) -> Felt {
    polynomial.iter().rev().fold(field.zero(), |accumulator, &coefficient| {
        field.add(field.mul(accumulator, value), coefficient)
    })
}

fn polynomial_degree(field: &Field, polynomial: &[Felt]) -> Option<usize> {
    polynomial.iter().rposition(|&coefficient| coefficient != field.zero())
}

fn encode(field: &Field, polynomial: &[Felt]) -> String {
    polynomial.iter().map(|&value| field.to_index(value).to_string()).collect::<Vec<_>>().join(",")
}

fn all_base(field: &Field, polynomial: &[Felt]) -> bool {
    polynomial.iter().all(|&value| field.is_base(value))
}

fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 { let remainder = left % right; left = right; right = remainder; }
    left
}

fn scalar_mod(value: i64, modulus: u64) -> u64 {
    let modulus_i = i128::from(modulus);
    u64::try_from((i128::from(value) % modulus_i + modulus_i) % modulus_i).expect("reduced scalar")
}

fn pow_mod(mut base: u64, mut exponent: usize, modulus: u64) -> u64 {
    let mut result = 1 % modulus;
    while exponent != 0 {
        if exponent & 1 == 1 { result = result * base % modulus; }
        base = base * base % modulus;
        exponent >>= 1;
    }
    result
}

fn predicted_visible_kernel(order: u64, trace: i64, degree: usize) -> u64 {
    let eigenvalue = scalar_mod(trace - 1, order);
    gcd(order, (pow_mod(eigenvalue, degree, order) + order - 1) % order)
}

fn table(path: &str, wanted: &[&str]) -> Result<HashMap<String, Vec<String>>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().ok_or("empty table")?.split('\t').collect();
    let slug_index = header.iter().position(|&name| name == "slug").ok_or("missing slug")?;
    let indices = wanted.iter().map(|wanted_name| {
        header.iter().position(|name| name == wanted_name).ok_or_else(|| format!("missing {wanted_name}"))
    }).collect::<Result<Vec<_>, _>>()?;
    let mut result = HashMap::new();
    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        result.insert(fields[slug_index].to_string(), indices.iter().map(|&i| fields[i].to_string()).collect());
    }
    Ok(result)
}

fn run_cell(cell: Cell, order: u64, trace: i64, expected_x_count: u64) -> Result<CellResult, String> {
    let started = Instant::now();
    let field = Field::new(cell.p, cell.degree, &first_irreducible(cell.p, cell.degree)?)?;
    let curve = Curve { field: &field, a: field.from_base(cell.a), b: field.from_base(cell.b) };
    field.reset_counters();
    let (regular, poles, pair_failures) = samples(&curve, trace)?;
    let degree = usize::try_from(order).map_err(|_| "order too large")?;
    let window_size = 2 * degree;
    require(regular.len() >= window_size, "insufficient regular samples")?;
    let (first_n, first_d, first_rank) = solve_window(&field, &regular[..window_size], degree)?;
    let last_start = regular.len() - window_size;
    let (second_n, second_d, second_rank) = solve_window(&field, &regular[last_start..], degree)?;

    let regular_count = u64::try_from(regular.len()).map_err(|_| "count too large")?;
    let pole_count = u64::try_from(poles.len()).map_err(|_| "count too large")?;
    let pole_multiplicity: u64 = poles.iter().map(|(_, multiplicity)| multiplicity).sum();
    let visible_kernel = pole_multiplicity + 1;
    let predicted_kernel = predicted_visible_kernel(order, trace, cell.degree);
    let expected_pole_count = predicted_kernel / 2;
    let numerator_degree = polynomial_degree(&field, &first_n).unwrap_or(0);
    let denominator_degree = polynomial_degree(&field, &first_d).unwrap_or(0);
    let cross_degree = numerator_degree + denominator_degree;
    let margin = i64::try_from(regular_count).map_err(|_| "count too large")?
        - i64::try_from(cross_degree).map_err(|_| "degree too large")?;
    let windows_equal = first_n == second_n && first_d == second_d;
    let coefficients_base = all_base(&field, &first_n) && all_base(&field, &first_d)
        && all_base(&field, &second_n) && all_base(&field, &second_d);
    let regular_mismatches: u64 = regular.iter().filter(|sample| {
        evaluate(&field, &first_n, sample.x)
            != field.mul(sample.image_x, evaluate(&field, &first_d, sample.x))
    }).count().try_into().map_err(|_| "count too large")?;
    let denominator_zero_regular: u64 = regular.iter().filter(|sample| {
        evaluate(&field, &first_d, sample.x) == field.zero()
    }).count().try_into().map_err(|_| "count too large")?;
    let missed_observed_poles: u64 = poles.iter().filter(|(x, _)| {
        evaluate(&field, &first_d, *x) != field.zero()
    }).count().try_into().map_err(|_| "count too large")?;
    let pole_cancellations: u64 = poles.iter().filter(|(x, _)| {
        evaluate(&field, &first_n, *x) == field.zero()
    }).count().try_into().map_err(|_| "count too large")?;

    let mut numerator_mutation = first_n.clone();
    numerator_mutation[0] = field.add(numerator_mutation[0], field.one());
    let numerator_mutation_rejections: u64 = regular.iter().filter(|sample| {
        evaluate(&field, &numerator_mutation, sample.x)
            != field.mul(sample.image_x, evaluate(&field, &first_d, sample.x))
    }).count().try_into().map_err(|_| "count too large")?;
    let mut denominator_mutation = first_d.clone();
    denominator_mutation[0] = field.add(denominator_mutation[0], field.one());
    let denominator_mutation_destroyed_poles: u64 = poles.iter().filter(|(x, _)| {
        evaluate(&field, &denominator_mutation, *x) != field.zero()
    }).count().try_into().map_err(|_| "count too large")?;

    let full_rank = 2 * degree;
    let strict_pass = regular_count > u64::try_from(cross_degree).unwrap_or(u64::MAX);
    let boundary_count = u64::try_from(cross_degree).map_err(|_| "degree too large")?;
    let boundary_pass = boundary_count > u64::try_from(cross_degree).unwrap_or(0);
    let numerator_encoding = encode(&field, &first_n);
    let denominator_encoding = encode(&field, &first_d);
    let snapshot = field.snapshot();
    let accepted = regular_count + pole_count == expected_x_count
        && pair_failures == 0
        && visible_kernel == predicted_kernel
        && pole_count == expected_pole_count
        && first_rank == full_rank && second_rank == full_rank
        && numerator_degree == degree && denominator_degree + 1 == degree
        && windows_equal && coefficients_base
        && regular_mismatches == 0 && denominator_zero_regular == 0
        && missed_observed_poles == 0 && pole_cancellations == 0
        && strict_pass && !boundary_pass
        && numerator_mutation_rejections == regular_count
        && denominator_mutation_destroyed_poles == pole_count;
    let fields = vec![
        cell.slug.to_string(), cell.p.to_string(), cell.degree.to_string(), order.to_string(), trace.to_string(),
        expected_x_count.to_string(), regular_count.to_string(), pole_count.to_string(), pole_multiplicity.to_string(),
        predicted_kernel.to_string(), visible_kernel.to_string(), expected_pole_count.to_string(),
        window_size.to_string(), first_rank.to_string(), second_rank.to_string(),
        numerator_degree.to_string(), denominator_degree.to_string(), cross_degree.to_string(), margin.to_string(),
        u8::from(strict_pass).to_string(), boundary_count.to_string(), u8::from(boundary_pass).to_string(),
        pair_failures.to_string(), u8::from(windows_equal).to_string(), u8::from(coefficients_base).to_string(),
        numerator_encoding.clone(), encode(&field, &second_n), denominator_encoding.clone(), encode(&field, &second_d),
        regular_mismatches.to_string(), denominator_zero_regular.to_string(), missed_observed_poles.to_string(),
        pole_cancellations.to_string(), numerator_mutation_rejections.to_string(),
        denominator_mutation_destroyed_poles.to_string(), snapshot.extension_additions.to_string(),
        snapshot.extension_multiplications.to_string(), snapshot.extension_inversions.to_string(),
        snapshot.point_additions.to_string(), snapshot.scalar_multiplications.to_string(),
        started.elapsed().as_nanos().to_string(), u8::from(accepted).to_string(),
    ];
    Ok(CellResult {
        row: format!("{}\n", fields.join("\t")), accepted, key: (cell.p, cell.a, cell.b),
        numerator: numerator_encoding, denominator: denominator_encoding,
    })
}

fn run(parent_path: &str, capacity_path: &str, output_path: &str) -> Result<(), String> {
    let parents = table(parent_path, &["base_count", "trace"])?;
    let capacities = table(capacity_path, &["distinct_x_count"])?;
    let mut results = Vec::new();
    for cell in CELLS {
        let parent = parents.get(cell.slug).ok_or("missing parent")?;
        let order = parent[0].parse::<u64>().map_err(|error| error.to_string())?;
        let trace = parent[1].parse::<i64>().map_err(|error| error.to_string())?;
        let x_count = capacities.get(cell.slug).ok_or("missing capacity")?[0]
            .parse::<u64>().map_err(|error| error.to_string())?;
        results.push(run_cell(cell, order, trace, x_count)?);
    }
    let mut families: HashMap<(u64, u64, u64), (String, String)> = HashMap::new();
    let mut stable = true;
    for result in &results {
        let value = (result.numerator.clone(), result.denominator.clone());
        if let Some(previous) = families.insert(result.key, value.clone()) { stable &= previous == value; }
    }
    let mut output = String::from(
        "slug\tp\tdegree\tbase_count\ttrace\tdistinct_x_count\tregular_x_count\tpole_x_count\tpole_multiplicity\tpredicted_visible_kernel\tobserved_visible_kernel\tpredicted_pole_x_count\twindow_size\tfirst_rank\tsecond_rank\tnumerator_degree\tdenominator_degree\tcross_degree\tseparation_margin\tregular_strict_pass\tboundary_sample_count\tboundary_strict_pass\tpair_consistency_failures\twindows_equal\tcoefficients_base_field\tnumerator_first\tnumerator_second\tdenominator_first\tdenominator_second\tregular_mismatches\tdenominator_zero_regular\tmissed_observed_poles\tpole_cancellations\tnumerator_mutation_rejections\tdenominator_mutation_destroyed_poles\textension_additions\textension_multiplications\textension_inversions\tpoint_additions\tscalar_multiplications\telapsed_ns\taccepted\n",
    );
    for result in &results { write!(output, "{}", result.row).map_err(|error| error.to_string())?; }
    std::fs::write(output_path, output).map_err(|error| error.to_string())?;
    require(results.iter().all(|result| result.accepted), "one or more v2 cells failed")?;
    require(stable && families.len() == 3, "one or more coefficient families are unstable")?;
    println!("h116 pointwise rational reconstruction v2: 8/8 cells accepted; 3/3 families stable");
    Ok(())
}

fn selftest() -> Result<(), String> {
    h116_field::selftest()?;
    require(predicted_visible_kernel(7, -1, 2) == 1, "p5 d2 eigenspace failed")?;
    require(predicted_visible_kernel(5, 3, 4) == 5, "p7 d4 eigenspace failed")?;
    require(predicted_visible_kernel(14, -2, 2) == 2, "p11 d2 eigenspace failed")?;
    require(predicted_visible_kernel(14, -2, 3) == 14, "p11 d3 eigenspace failed")?;
    println!("h116 pointwise rational reconstruction v2 selftest ok");
    Ok(())
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let result = match arguments.as_slice() {
        [_, flag] if flag == "--selftest" => selftest(),
        [_, parent, capacity, output] => run(parent, capacity, output),
        _ => Err("usage: reconstruction_v2 [--selftest | PARENT.tsv CAPACITY.tsv OUTPUT.tsv]".to_string()),
    };
    if let Err(error) = result { eprintln!("error: {error}"); std::process::exit(1); }
}

