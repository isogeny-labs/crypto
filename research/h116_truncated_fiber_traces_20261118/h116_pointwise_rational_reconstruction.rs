#[allow(dead_code)]
#[path = "h116_field.rs"]
mod h116_field;

use h116_field::{first_irreducible, Curve, Felt, Field, Point};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::time::Instant;

#[derive(Clone, Copy)]
struct Cell {
    slug: &'static str,
    p: u64,
    a: u64,
    b: u64,
    degree: usize,
}

#[derive(Clone, Copy)]
struct RegularSample {
    x: Felt,
    image_x: Felt,
}

#[derive(Clone, Copy)]
struct Pole {
    x: Felt,
    multiplicity: u64,
}

struct CellResult {
    row: String,
    accepted: bool,
    family_key: (u64, u64, u64),
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

fn pointwise_map(curve: &Curve<'_>, point: Point, trace: i64) -> Result<Point, String> {
    curve.sub(signed_scalar_mul(curve, trace - 1, point)?, curve.frobenius(point))
}

fn image_x(point: Point) -> Option<Felt> {
    match point {
        Point::Infinity => None,
        Point::Affine(x, _) => Some(x),
    }
}

fn curve_rhs(field: &Field, curve: &Curve<'_>, x: Felt) -> Felt {
    field.add(
        field.add(field.mul(field.square(x), x), field.mul(curve.a, x)),
        curve.b,
    )
}

fn samples_and_poles(
    curve: &Curve<'_>,
    trace: i64,
) -> Result<(Vec<RegularSample>, Vec<Pole>, u64), String> {
    let field = curve.field;
    let mut regular = Vec::new();
    let mut poles = Vec::new();
    let mut pair_consistency_failures = 0_u64;
    for index in 0..field.q {
        let x = field.from_index(index);
        let Some(y) = field.sqrt(curve_rhs(field, curve, x))? else {
            continue;
        };
        let first = image_x(pointwise_map(curve, Point::Affine(x, y), trace)?);
        let multiplicity = if y == field.zero() { 1 } else { 2 };
        if y != field.zero() {
            let second = image_x(pointwise_map(
                curve,
                Point::Affine(x, field.neg(y)),
                trace,
            )?);
            if first != second {
                pair_consistency_failures += 1;
            }
        }
        match first {
            Some(mapped_x) => regular.push(RegularSample { x, image_x: mapped_x }),
            None => poles.push(Pole { x, multiplicity }),
        }
    }
    Ok((regular, poles, pair_consistency_failures))
}

fn multiply_by_linear(field: &Field, coefficients: &[Felt], root: Felt) -> Vec<Felt> {
    let mut result = vec![field.zero(); coefficients.len() + 1];
    let negative_root = field.neg(root);
    for (index, &coefficient) in coefficients.iter().enumerate() {
        result[index] = field.add(result[index], field.mul(negative_root, coefficient));
        result[index + 1] = field.add(result[index + 1], coefficient);
    }
    result
}

fn pole_denominator(field: &Field, poles: &[Pole]) -> Vec<Felt> {
    let mut result = vec![field.one()];
    for pole in poles {
        for _ in 0..pole.multiplicity {
            result = multiply_by_linear(field, &result, pole.x);
        }
    }
    result
}

fn evaluate(field: &Field, coefficients: &[Felt], value: Felt) -> Felt {
    coefficients.iter().rev().fold(field.zero(), |result, &coefficient| {
        field.add(field.mul(result, value), coefficient)
    })
}

fn add_scaled(field: &Field, destination: &mut [Felt], source: &[Felt], scale: Felt) {
    for (index, &coefficient) in source.iter().enumerate() {
        destination[index] = field.add(destination[index], field.mul(scale, coefficient));
    }
}

fn interpolate_numerator(
    field: &Field,
    samples: &[RegularSample],
    denominator: &[Felt],
    degree: usize,
) -> Result<Vec<Felt>, String> {
    require(samples.len() == degree + 1, "wrong interpolation-window size")?;
    let mut result = vec![field.zero(); degree + 1];
    for (i, sample) in samples.iter().enumerate() {
        let mut basis = vec![field.one()];
        let mut divisor = field.one();
        for (j, other) in samples.iter().enumerate() {
            if i == j {
                continue;
            }
            basis = multiply_by_linear(field, &basis, other.x);
            divisor = field.mul(divisor, field.sub(sample.x, other.x));
        }
        let target = field.mul(sample.image_x, evaluate(field, denominator, sample.x));
        add_scaled(field, &mut result, &basis, field.div(target, divisor)?);
    }
    Ok(result)
}

fn polynomial_degree(field: &Field, coefficients: &[Felt]) -> Option<usize> {
    coefficients.iter().rposition(|&coefficient| coefficient != field.zero())
}

fn encode(field: &Field, coefficients: &[Felt]) -> String {
    coefficients
        .iter()
        .map(|&coefficient| field.to_index(coefficient).to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn all_base(field: &Field, coefficients: &[Felt]) -> bool {
    coefficients.iter().all(|&coefficient| field.is_base(coefficient))
}

fn parent_rows(path: &str) -> Result<HashMap<String, (u64, i64)>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().ok_or("empty parent result")?.split('\t').collect();
    let slug_i = header.iter().position(|&name| name == "slug").ok_or("missing slug")?;
    let base_i = header.iter().position(|&name| name == "base_count").ok_or("missing base_count")?;
    let trace_i = header.iter().position(|&name| name == "trace").ok_or("missing trace")?;
    let mut result = HashMap::new();
    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        result.insert(
            fields[slug_i].to_string(),
            (
                fields[base_i].parse::<u64>().map_err(|error| error.to_string())?,
                fields[trace_i].parse::<i64>().map_err(|error| error.to_string())?,
            ),
        );
    }
    Ok(result)
}

fn capacity_rows(path: &str) -> Result<HashMap<String, u64>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().ok_or("empty capacity result")?.split('\t').collect();
    let slug_i = header.iter().position(|&name| name == "slug").ok_or("missing slug")?;
    let count_i = header
        .iter()
        .position(|&name| name == "distinct_x_count")
        .ok_or("missing distinct_x_count")?;
    let mut result = HashMap::new();
    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        result.insert(
            fields[slug_i].to_string(),
            fields[count_i].parse::<u64>().map_err(|error| error.to_string())?,
        );
    }
    Ok(result)
}

fn run_cell(
    cell: Cell,
    base_count: u64,
    trace: i64,
    expected_distinct_x: u64,
) -> Result<CellResult, String> {
    let started = Instant::now();
    let field = Field::new(cell.p, cell.degree, &first_irreducible(cell.p, cell.degree)?)?;
    let curve = Curve {
        field: &field,
        a: field.from_base(cell.a),
        b: field.from_base(cell.b),
    };
    field.reset_counters();
    let (regular, poles, pair_failures) = samples_and_poles(&curve, trace)?;
    let regular_count = u64::try_from(regular.len()).map_err(|_| "too many regular samples")?;
    let pole_count = u64::try_from(poles.len()).map_err(|_| "too many poles")?;
    let distinct_x_count = regular_count + pole_count;
    let pole_multiplicity: u64 = poles.iter().map(|pole| pole.multiplicity).sum();
    let denominator = pole_denominator(&field, &poles);
    let expected_degree = usize::try_from(base_count).map_err(|_| "degree too large")?;
    require(regular.len() >= expected_degree + 1, "insufficient interpolation samples")?;
    let first = interpolate_numerator(
        &field,
        &regular[..=expected_degree],
        &denominator,
        expected_degree,
    )?;
    let last_start = regular.len() - expected_degree - 1;
    let second = interpolate_numerator(
        &field,
        &regular[last_start..],
        &denominator,
        expected_degree,
    )?;

    let numerator_degree = polynomial_degree(&field, &first).unwrap_or(0);
    let denominator_degree = polynomial_degree(&field, &denominator).unwrap_or(0);
    let cross_degree = numerator_degree + denominator_degree;
    let separation_margin = i64::try_from(regular_count).map_err(|_| "count too large")?
        - i64::try_from(cross_degree).map_err(|_| "degree too large")?;
    let full_sample_mismatches: u64 = regular
        .iter()
        .filter(|sample| {
            evaluate(&field, &first, sample.x)
                != field.mul(sample.image_x, evaluate(&field, &denominator, sample.x))
        })
        .count()
        .try_into()
        .map_err(|_| "mismatch count too large")?;
    let pole_cancellations: u64 = poles
        .iter()
        .filter(|pole| evaluate(&field, &first, pole.x) == field.zero())
        .count()
        .try_into()
        .map_err(|_| "pole count too large")?;

    let mut numerator_mutation = first.clone();
    numerator_mutation[0] = field.add(numerator_mutation[0], field.one());
    let numerator_mutation_rejections: u64 = regular
        .iter()
        .filter(|sample| {
            evaluate(&field, &numerator_mutation, sample.x)
                != field.mul(sample.image_x, evaluate(&field, &denominator, sample.x))
        })
        .count()
        .try_into()
        .map_err(|_| "mutation count too large")?;
    let mut denominator_mutation = denominator.clone();
    denominator_mutation[0] = field.add(denominator_mutation[0], field.one());
    let denominator_mutation_destroyed_poles: u64 = poles
        .iter()
        .filter(|pole| evaluate(&field, &denominator_mutation, pole.x) != field.zero())
        .count()
        .try_into()
        .map_err(|_| "mutation pole count too large")?;

    let coefficients_base = all_base(&field, &first)
        && all_base(&field, &second)
        && all_base(&field, &denominator);
    let windows_equal = first == second;
    let regular_strict_pass = regular_count > u64::try_from(cross_degree).unwrap_or(u64::MAX);
    let boundary_sample_count = u64::try_from(cross_degree).map_err(|_| "degree too large")?;
    let boundary_strict_pass = boundary_sample_count > u64::try_from(cross_degree).unwrap_or(0);
    let denominator_zero_regular = regular
        .iter()
        .any(|sample| evaluate(&field, &denominator, sample.x) == field.zero());
    let pole_x_all_base = poles.iter().all(|pole| field.is_base(pole.x));
    let snapshot = field.snapshot();
    let numerator_encoding = encode(&field, &first);
    let second_encoding = encode(&field, &second);
    let denominator_encoding = encode(&field, &denominator);
    let accepted = distinct_x_count == expected_distinct_x
        && pair_failures == 0
        && pole_count == base_count / 2
        && pole_multiplicity == base_count - 1
        && pole_x_all_base
        && numerator_degree == expected_degree
        && denominator_degree + 1 == expected_degree
        && windows_equal
        && coefficients_base
        && !denominator_zero_regular
        && full_sample_mismatches == 0
        && pole_cancellations == 0
        && regular_strict_pass
        && !boundary_strict_pass
        && numerator_mutation_rejections == regular_count
        && denominator_mutation_destroyed_poles == pole_count;

    let fields = vec![
        cell.slug.to_string(), cell.p.to_string(), cell.degree.to_string(),
        base_count.to_string(), trace.to_string(), distinct_x_count.to_string(),
        regular_count.to_string(), pole_count.to_string(), pole_multiplicity.to_string(),
        numerator_degree.to_string(), denominator_degree.to_string(), cross_degree.to_string(),
        separation_margin.to_string(), u8::from(regular_strict_pass).to_string(),
        boundary_sample_count.to_string(), u8::from(boundary_strict_pass).to_string(),
        pair_failures.to_string(), (expected_degree + 1).to_string(),
        u8::from(windows_equal).to_string(), u8::from(coefficients_base).to_string(),
        numerator_encoding.clone(), second_encoding, denominator_encoding.clone(),
        full_sample_mismatches.to_string(), pole_cancellations.to_string(),
        numerator_mutation_rejections.to_string(),
        denominator_mutation_destroyed_poles.to_string(),
        snapshot.extension_additions.to_string(), snapshot.extension_multiplications.to_string(),
        snapshot.extension_inversions.to_string(), snapshot.point_additions.to_string(),
        snapshot.scalar_multiplications.to_string(), started.elapsed().as_nanos().to_string(),
        u8::from(accepted).to_string(),
    ];
    Ok(CellResult {
        row: format!("{}\n", fields.join("\t")),
        accepted,
        family_key: (cell.p, cell.a, cell.b),
        numerator: numerator_encoding,
        denominator: denominator_encoding,
    })
}

fn run(parent_path: &str, capacity_path: &str, output_path: &str) -> Result<(), String> {
    let parents = parent_rows(parent_path)?;
    let capacities = capacity_rows(capacity_path)?;
    let mut results = Vec::new();
    for cell in CELLS {
        let &(base_count, trace) = parents.get(cell.slug).ok_or("missing parent row")?;
        let &distinct_x = capacities.get(cell.slug).ok_or("missing capacity row")?;
        results.push(run_cell(cell, base_count, trace, distinct_x)?);
    }
    let mut families: HashMap<(u64, u64, u64), (String, String)> = HashMap::new();
    let mut stable = true;
    for result in &results {
        let value = (result.numerator.clone(), result.denominator.clone());
        if let Some(previous) = families.insert(result.family_key, value.clone()) {
            stable &= previous == value;
        }
    }
    let mut output = String::from(
        "slug\tp\tdegree\tbase_count\ttrace\tdistinct_x_count\tregular_x_count\tpole_x_count\tpole_multiplicity\tnumerator_degree\tdenominator_degree\tcross_degree\tseparation_margin\tregular_strict_pass\tboundary_sample_count\tboundary_strict_pass\tpair_consistency_failures\tinterpolation_window\tnumerator_windows_equal\tcoefficients_base_field\tnumerator_first\tnumerator_second\tdenominator\tfull_sample_mismatches\tpole_cancellations\tnumerator_mutation_rejections\tdenominator_mutation_destroyed_poles\textension_additions\textension_multiplications\textension_inversions\tpoint_additions\tscalar_multiplications\telapsed_ns\taccepted\n",
    );
    for result in &results {
        write!(output, "{}", result.row).map_err(|error| error.to_string())?;
    }
    std::fs::write(output_path, output).map_err(|error| error.to_string())?;
    require(results.iter().all(|result| result.accepted), "one or more cells failed")?;
    require(stable, "reconstructed coefficients are not extension-stable")?;
    println!("h116 pointwise rational reconstruction: 8/8 cells accepted; 3/3 families stable");
    Ok(())
}

fn selftest() -> Result<(), String> {
    h116_field::selftest()?;
    let field = Field::new(5, 1, &[0, 1])?;
    let samples = [
        RegularSample { x: field.from_base(0), image_x: field.from_base(1) },
        RegularSample { x: field.from_base(1), image_x: field.from_base(3) },
    ];
    let denominator = vec![field.one()];
    let polynomial = interpolate_numerator(&field, &samples, &denominator, 1)?;
    require(encode(&field, &polynomial) == "1,2", "linear interpolation failed")?;
    println!("h116 pointwise rational reconstruction selftest ok");
    Ok(())
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let result = match arguments.as_slice() {
        [_, flag] if flag == "--selftest" => selftest(),
        [_, parent, capacity, output] => run(parent, capacity, output),
        _ => Err("usage: reconstruction [--selftest | PARENT.tsv CAPACITY.tsv OUTPUT.tsv]".to_string()),
    };
    if let Err(error) = result {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

