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

fn v_minus_one(curve: &Curve<'_>, point: Point, trace: i64) -> Result<Point, String> {
    curve.sub(signed_scalar_mul(curve, trace - 1, point)?, curve.frobenius(point))
}

fn pi_minus_one(curve: &Curve<'_>, point: Point) -> Result<Point, String> {
    curve.sub(curve.frobenius(point), point)
}

fn point_x(point: Point) -> Option<Felt> {
    match point { Point::Infinity => None, Point::Affine(x, _) => Some(x) }
}

fn samples(curve: &Curve<'_>) -> Result<(Vec<Sample>, Vec<(Felt, u64)>, u64), String> {
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
        let first = point_x(pi_minus_one(curve, Point::Affine(x, y))?);
        let multiplicity = if y == field.zero() { 1 } else { 2 };
        if multiplicity == 2 {
            let second = point_x(pi_minus_one(curve, Point::Affine(x, field.neg(y)))?);
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
    for index in 1..=through { result.push(field.mul(result[index - 1], value)); }
    result
}

fn solve_window(field: &Field, window: &[Sample], degree: usize) -> Result<(Vec<Felt>, Vec<Felt>, usize), String> {
    let width = 2 * degree;
    require(window.len() == width, "wrong interpolation window")?;
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
        let Some(found) = (pivot_row..width).find(|&row| matrix[row][column] != field.zero()) else { continue };
        matrix.swap(pivot_row, found);
        let inverse = field.inv(matrix[pivot_row][column])?;
        for entry in &mut matrix[pivot_row][column..=width] { *entry = field.mul(*entry, inverse); }
        for row in 0..width {
            if row == pivot_row { continue; }
            let factor = matrix[row][column];
            if factor == field.zero() { continue; }
            for index in column..=width {
                matrix[row][index] = field.sub(matrix[row][index], field.mul(factor, matrix[pivot_row][index]));
            }
        }
        pivots.push(column);
        pivot_row += 1;
        if pivot_row == width { break; }
    }
    let mut solution = vec![field.zero(); width];
    for (row, &column) in pivots.iter().enumerate() { solution[column] = matrix[row][width]; }
    let numerator = solution[..=degree].to_vec();
    let mut denominator = solution[degree + 1..].to_vec();
    denominator.push(field.one());
    Ok((numerator, denominator, pivots.len()))
}

fn evaluate(field: &Field, polynomial: &[Felt], value: Felt) -> Felt {
    polynomial.iter().rev().fold(field.zero(), |accumulator, &coefficient| {
        field.add(field.mul(accumulator, value), coefficient)
    })
}

fn divide_by_x_minus_root(field: &Field, polynomial: &[Felt], root: Felt) -> Result<Vec<Felt>, String> {
    require(polynomial.len() >= 2, "cannot divide constant")?;
    let degree = polynomial.len() - 1;
    let mut quotient = vec![field.zero(); degree];
    quotient[degree - 1] = polynomial[degree];
    for index in (1..degree).rev() {
        quotient[index - 1] = field.add(polynomial[index], field.mul(root, quotient[index]));
    }
    require(field.add(polynomial[0], field.mul(root, quotient[0])) == field.zero(), "nonzero remainder")?;
    Ok(quotient)
}

fn root_multiplicity(field: &Field, polynomial: &[Felt], root: Felt) -> Result<usize, String> {
    let mut quotient = polynomial.to_vec();
    let mut multiplicity = 0_usize;
    while quotient.len() >= 2 && evaluate(field, &quotient, root) == field.zero() {
        quotient = divide_by_x_minus_root(field, &quotient, root)?;
        multiplicity += 1;
    }
    Ok(multiplicity)
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

fn table(path: &str) -> Result<HashMap<String, HashMap<String, String>>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().ok_or("empty table")?.split('\t').collect();
    let slug_index = header.iter().position(|&name| name == "slug").ok_or("missing slug")?;
    let mut result = HashMap::new();
    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        require(fields.len() == header.len(), "malformed table row")?;
        result.insert(
            fields[slug_index].to_string(),
            header.iter().zip(fields).map(|(&name, value)| (name.to_string(), value.to_string())).collect(),
        );
    }
    Ok(result)
}

fn value<'a>(row: &'a HashMap<String, String>, name: &str) -> Result<&'a str, String> {
    row.get(name).map(String::as_str).ok_or_else(|| format!("missing {name}"))
}

fn composition_census(curve: &Curve<'_>, trace: i64, order: u64) -> Result<(u64, u64, u64, u64), String> {
    let field = curve.field;
    let mut points = vec![Point::Infinity];
    for index in 0..field.q {
        let x = field.from_index(index);
        let rhs = field.add(field.add(field.mul(field.square(x), x), field.mul(curve.a, x)), curve.b);
        let Some(y) = field.sqrt(rhs)? else { continue };
        points.push(Point::Affine(x, y));
        if y != field.zero() { points.push(Point::Affine(x, field.neg(y))); }
    }
    let mut correct_failures = 0_u64;
    let mut omitted_minus_one_failures = 0_u64;
    let mut wrong_scalar_failures = 0_u64;
    for &point in &points {
        let right = pi_minus_one(curve, point)?;
        let composed = v_minus_one(curve, right, trace)?;
        let expected = curve.scalar_mul(order, point)?;
        correct_failures += u64::from(composed != expected);
        let omitted = v_minus_one(curve, curve.frobenius(point), trace)?;
        omitted_minus_one_failures += u64::from(omitted != expected);
        let wrong_scalar = curve.scalar_mul(order + 1, point)?;
        wrong_scalar_failures += u64::from(composed != wrong_scalar);
    }
    Ok((u64::try_from(points.len()).map_err(|_| "point count too large")?, correct_failures,
        omitted_minus_one_failures, wrong_scalar_failures))
}

fn run_cell(
    cell: Cell,
    pointwise: &HashMap<String, String>,
    parent: &HashMap<String, String>,
) -> Result<CellResult, String> {
    let started = Instant::now();
    let order = value(pointwise, "base_count")?.parse::<u64>().map_err(|e| e.to_string())?;
    let trace = value(pointwise, "trace")?.parse::<i64>().map_err(|e| e.to_string())?;
    let expected_points = value(pointwise, "extension_count")?.parse::<u64>().map_err(|e| e.to_string())?;
    let distinct_x = value(parent, "distinct_x_count")?.parse::<u64>().map_err(|e| e.to_string())?;
    require(value(parent, "accepted")? == "1", "unaccepted V-1 parent")?;
    let parent_num_degree = value(parent, "numerator_degree")?.parse::<usize>().map_err(|e| e.to_string())?;
    let parent_den_degree = value(parent, "denominator_degree")?.parse::<usize>().map_err(|e| e.to_string())?;
    let field = Field::new(cell.p, cell.degree, &first_irreducible(cell.p, cell.degree)?)?;
    let curve = Curve { field: &field, a: field.from_base(cell.a), b: field.from_base(cell.b) };
    field.reset_counters();
    let (regular, poles, pair_failures) = samples(&curve)?;
    let map_degree = usize::try_from(order).map_err(|_| "order too large")?;
    let window_size = 2 * map_degree;
    require(regular.len() >= window_size, "insufficient regular samples")?;
    let (first_n, first_d, first_rank) = solve_window(&field, &regular[..window_size], map_degree)?;
    let last_start = regular.len() - window_size;
    let (second_n, second_d, second_rank) = solve_window(&field, &regular[last_start..], map_degree)?;
    let numerator_degree = polynomial_degree(&field, &first_n).unwrap_or(0);
    let denominator_degree = polynomial_degree(&field, &first_d).unwrap_or(0);
    let regular_count = u64::try_from(regular.len()).map_err(|_| "count too large")?;
    let pole_count = u64::try_from(poles.len()).map_err(|_| "count too large")?;
    let pole_multiplicity: u64 = poles.iter().map(|(_, multiplicity)| multiplicity).sum();
    let predicted_poles = order / 2;
    let predicted_simple = u64::from(order % 2 == 0);
    let predicted_double = predicted_poles - predicted_simple;
    let predicted_regular = distinct_x - predicted_poles;

    let mut simple = 0_u64;
    let mut double = 0_u64;
    let mut higher = 0_u64;
    for (x, _) in &poles {
        match root_multiplicity(&field, &first_d, *x)? {
            1 => simple += 1,
            2 => double += 1,
            _ => higher += 1,
        }
    }
    let denominator_roots_all = (0..field.q).filter(|&index| {
        evaluate(&field, &first_d, field.from_index(index)) == field.zero()
    }).count() as u64;
    let regular_mismatches = regular.iter().filter(|sample| {
        evaluate(&field, &first_n, sample.x) != field.mul(sample.image_x, evaluate(&field, &first_d, sample.x))
    }).count() as u64;
    let missed_poles = poles.iter().filter(|(x, _)| evaluate(&field, &first_d, *x) != field.zero()).count() as u64;
    let pole_cancellations = poles.iter().filter(|(x, _)| evaluate(&field, &first_n, *x) == field.zero()).count() as u64;
    let mut numerator_mutation = first_n.clone();
    numerator_mutation[0] = field.add(numerator_mutation[0], field.one());
    let numerator_mutation_rejections = regular.iter().filter(|sample| {
        evaluate(&field, &numerator_mutation, sample.x)
            != field.mul(sample.image_x, evaluate(&field, &first_d, sample.x))
    }).count() as u64;
    let mut denominator_mutation = first_d.clone();
    denominator_mutation[0] = field.add(denominator_mutation[0], field.one());
    let denominator_mutation_destroyed_poles = poles.iter().filter(|(x, _)| {
        evaluate(&field, &denominator_mutation, *x) != field.zero()
    }).count() as u64;
    let (point_count, composition_failures, omitted_failures, wrong_scalar_failures) =
        composition_census(&curve, trace, order)?;
    let windows_equal = first_n == second_n && first_d == second_d;
    let coefficients_base = all_base(&field, &first_n) && all_base(&field, &first_d)
        && all_base(&field, &second_n) && all_base(&field, &second_d);
    let cross_degree = numerator_degree + denominator_degree;
    let separation_margin = i64::try_from(regular_count).map_err(|_| "count too large")?
        - i64::try_from(cross_degree).map_err(|_| "degree too large")?;
    let strict_pass = regular_count > u64::try_from(cross_degree).map_err(|_| "degree too large")?;
    let degree_pair_equal = numerator_degree == parent_num_degree && denominator_degree == parent_den_degree;
    let accepted = regular_count == predicted_regular && pole_count == predicted_poles
        && pole_multiplicity + 1 == order && simple == predicted_simple && double == predicted_double
        && higher == 0 && denominator_roots_all == pole_count && pair_failures == 0
        && first_rank == window_size && second_rank == window_size
        && numerator_degree == map_degree && denominator_degree + 1 == map_degree
        && windows_equal && coefficients_base && degree_pair_equal && strict_pass
        && regular_mismatches == 0 && missed_poles == 0 && pole_cancellations == 0
        && numerator_mutation_rejections == regular_count
        && denominator_mutation_destroyed_poles == pole_count
        && point_count == expected_points && composition_failures == 0
        && omitted_failures > 0 && wrong_scalar_failures > 0;
    let numerator_encoding = encode(&field, &first_n);
    let denominator_encoding = encode(&field, &first_d);
    let snapshot = field.snapshot();
    let fields = vec![
        cell.slug.to_string(), cell.p.to_string(), cell.degree.to_string(), order.to_string(), trace.to_string(),
        distinct_x.to_string(), predicted_regular.to_string(), regular_count.to_string(), predicted_poles.to_string(),
        pole_count.to_string(), pole_multiplicity.to_string(), predicted_simple.to_string(), simple.to_string(),
        predicted_double.to_string(), double.to_string(), higher.to_string(), denominator_roots_all.to_string(),
        window_size.to_string(), first_rank.to_string(), second_rank.to_string(), numerator_degree.to_string(),
        denominator_degree.to_string(), parent_num_degree.to_string(), parent_den_degree.to_string(),
        u8::from(degree_pair_equal).to_string(), cross_degree.to_string(), separation_margin.to_string(),
        u8::from(strict_pass).to_string(), cross_degree.to_string(), "0".to_string(), pair_failures.to_string(),
        u8::from(windows_equal).to_string(), u8::from(coefficients_base).to_string(), numerator_encoding.clone(),
        encode(&field, &second_n), denominator_encoding.clone(), encode(&field, &second_d),
        regular_mismatches.to_string(), missed_poles.to_string(), pole_cancellations.to_string(),
        numerator_mutation_rejections.to_string(), denominator_mutation_destroyed_poles.to_string(),
        point_count.to_string(), composition_failures.to_string(), omitted_failures.to_string(),
        wrong_scalar_failures.to_string(), snapshot.extension_additions.to_string(),
        snapshot.extension_multiplications.to_string(), snapshot.extension_inversions.to_string(),
        snapshot.point_additions.to_string(), snapshot.scalar_multiplications.to_string(),
        started.elapsed().as_nanos().to_string(), u8::from(accepted).to_string(),
    ];
    Ok(CellResult { row: format!("{}\n", fields.join("\t")), accepted,
        key: (cell.p, cell.a, cell.b), numerator: numerator_encoding, denominator: denominator_encoding })
}

fn run(pointwise_path: &str, parent_path: &str, output_path: &str) -> Result<(), String> {
    let pointwise = table(pointwise_path)?;
    let parents = table(parent_path)?;
    let mut results = Vec::new();
    for cell in CELLS {
        results.push(run_cell(cell, pointwise.get(cell.slug).ok_or("missing pointwise row")?,
            parents.get(cell.slug).ok_or("missing parent row")?)?);
    }
    let mut families: HashMap<(u64, u64, u64), (String, String)> = HashMap::new();
    let mut stable = true;
    for result in &results {
        let map = (result.numerator.clone(), result.denominator.clone());
        if let Some(previous) = families.insert(result.key, map.clone()) { stable &= previous == map; }
    }
    let mut output = String::from(
        "slug\tp\tdegree\tbase_count\ttrace\tdistinct_x_count\tpredicted_regular_x\tregular_x_count\tpredicted_pole_roots\tpole_x_count\tpole_multiplicity\tpredicted_simple\tobserved_simple\tpredicted_double\tobserved_double\tobserved_higher\tdenominator_roots_all\twindow_size\tfirst_rank\tsecond_rank\tnumerator_degree\tdenominator_degree\tparent_numerator_degree\tparent_denominator_degree\tdegree_pair_equal\tcross_degree\tseparation_margin\tstrict_pass\tboundary_sample_count\tboundary_strict_pass\tpair_consistency_failures\twindows_equal\tcoefficients_base_field\tnumerator_first\tnumerator_second\tdenominator_first\tdenominator_second\tregular_mismatches\tmissed_poles\tpole_cancellations\tnumerator_mutation_rejections\tdenominator_mutation_destroyed_poles\tpoint_count\tcomposition_failures\tomitted_minus_one_failures\twrong_scalar_failures\textension_additions\textension_multiplications\textension_inversions\tpoint_additions\tscalar_multiplications\telapsed_ns\taccepted\n",
    );
    for result in &results { write!(output, "{}", result.row).map_err(|error| error.to_string())?; }
    std::fs::write(output_path, output).map_err(|error| error.to_string())?;
    require(results.iter().all(|result| result.accepted), "one or more cells failed")?;
    require(stable && families.len() == 3, "dual map families unstable")?;
    println!("h116 dual pair composition: 8/8 cells accepted; 3/3 families stable");
    Ok(())
}

fn selftest() -> Result<(), String> {
    h116_field::selftest()?;
    let field = Field::new(5, 2, &first_irreducible(5, 2)?)?;
    let root = field.from_index(7);
    let polynomial = vec![field.square(root), field.neg(field.add(root, root)), field.one()];
    require(root_multiplicity(&field, &polynomial, root)? == 2, "multiplicity selftest failed")?;
    println!("h116 dual pair composition selftest ok");
    Ok(())
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let result = match arguments.as_slice() {
        [_, flag] if flag == "--selftest" => selftest(),
        [_, pointwise, parent, output] => run(pointwise, parent, output),
        _ => Err("usage: dual_pair [--selftest | POINTWISE.tsv V_MINUS_ONE.tsv OUTPUT.tsv]".to_string()),
    };
    if let Err(error) = result { eprintln!("error: {error}"); std::process::exit(1); }
}
