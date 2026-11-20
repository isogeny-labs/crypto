#[allow(dead_code)]
#[path = "h116_field.rs"]
mod h116_field;

use h116_field::{first_irreducible, Curve, Field, Point};
use std::collections::{HashMap, HashSet};
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

fn enumerate_points(curve: &Curve<'_>) -> Result<Vec<Point>, String> {
    let mut points = vec![Point::Infinity];
    for index in 0..curve.field.q {
        let x = curve.field.from_index(index);
        let rhs = curve.field.add(
            curve.field.add(
                curve.field.mul(curve.field.square(x), x),
                curve.field.mul(curve.a, x),
            ),
            curve.b,
        );
        if let Some(y) = curve.field.sqrt(rhs)? {
            points.push(Point::Affine(x, y));
            if y != curve.field.zero() {
                points.push(Point::Affine(x, curve.field.neg(y)));
            }
        }
    }
    points.sort_by_key(|point| point_key(curve.field, *point));
    let unique: HashSet<Point> = points.iter().copied().collect();
    require(unique.len() == points.len(), "point enumeration contains duplicates")?;
    require(points.iter().all(|&point| curve.is_on_curve(point)), "off-curve point")?;
    Ok(points)
}

fn point_key(field: &Field, point: Point) -> (u64, u64, u64) {
    match point {
        Point::Infinity => (0, 0, 0),
        Point::Affine(x, y) => (1, field.to_index(x), field.to_index(y)),
    }
}

fn encode_point(field: &Field, point: Point) -> String {
    match point {
        Point::Infinity => "O".to_string(),
        Point::Affine(x, y) => format!("{}:{}", field.to_index(x), field.to_index(y)),
    }
}

fn prime_divisors(mut value: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut divisor = 2_u64;
    while divisor * divisor <= value {
        if value % divisor == 0 {
            result.push(divisor);
            while value % divisor == 0 {
                value /= divisor;
            }
        }
        divisor += 1;
    }
    if value > 1 {
        result.push(value);
    }
    result
}

fn point_order(curve: &Curve<'_>, point: Point, group_order: u64) -> Result<u64, String> {
    require(curve.scalar_mul(group_order, point)? == Point::Infinity, "group order does not kill point")?;
    let mut order = group_order;
    for prime in prime_divisors(group_order) {
        while order % prime == 0
            && curve.scalar_mul(order / prime, point)? == Point::Infinity
        {
            order /= prime;
        }
    }
    Ok(order)
}

fn repeated_order(curve: &Curve<'_>, point: Point, bound: u64) -> Result<u64, String> {
    let mut sum = Point::Infinity;
    for order in 1..=bound {
        sum = curve.add(sum, point)?;
        if sum == Point::Infinity {
            return Ok(order);
        }
    }
    Err("repeated-addition order exceeded group order".to_string())
}

fn cyclic_points(curve: &Curve<'_>, point: Point, order: u64) -> Result<Vec<Point>, String> {
    let mut result = Vec::with_capacity(usize::try_from(order).map_err(|_| "order too large")?);
    let mut multiple = Point::Infinity;
    for _ in 0..order {
        result.push(multiple);
        multiple = curve.add(multiple, point)?;
    }
    require(multiple == Point::Infinity, "cyclic enumeration did not close")?;
    Ok(result)
}

fn pair_closure(
    curve: &Curve<'_>,
    first: Point,
    first_order: u64,
    second: Point,
    second_order: u64,
) -> Result<HashSet<Point>, String> {
    let left = cyclic_points(curve, first, first_order)?;
    let right = cyclic_points(curve, second, second_order)?;
    let mut result = HashSet::new();
    for &a in &left {
        for &b in &right {
            result.insert(curve.add(a, b)?);
        }
    }
    Ok(result)
}

fn signed_scalar_mul(curve: &Curve<'_>, scalar: i64, point: Point) -> Result<Point, String> {
    let absolute = curve.scalar_mul(scalar.unsigned_abs(), point)?;
    Ok(if scalar < 0 { curve.neg(absolute) } else { absolute })
}

fn characteristic(
    curve: &Curve<'_>,
    point: Point,
    trace: i64,
    norm: u64,
) -> Result<Point, String> {
    let pi = curve.frobenius(point);
    let pi_squared = curve.frobenius(pi);
    curve.add(
        curve.sub(pi_squared, signed_scalar_mul(curve, trace, pi)?)?,
        curve.scalar_mul(norm, point)?,
    )
}

fn missing_inverse_rejects(
    curve: &Curve<'_>,
    point: Point,
    trace: i64,
    p: u64,
    degree: usize,
) -> Result<bool, String> {
    let expected = curve.sub(signed_scalar_mul(curve, trace, point)?, curve.frobenius(point))?;
    let inverse_pi = curve.frobenius_power(point, degree - 1);
    require(curve.scalar_mul(p, inverse_pi)? == expected, "positive V identity failed")?;
    Ok(curve.scalar_mul(p, point)? != expected)
}

fn parent_rows(path: &str) -> Result<HashMap<String, (i64, u64)>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().ok_or("empty parent result")?.split('\t').collect();
    let slug_index = header.iter().position(|&name| name == "slug").ok_or("missing slug column")?;
    let trace_index = header.iter().position(|&name| name == "trace").ok_or("missing trace column")?;
    let count_index = header.iter().position(|&name| name == "extension_count")
        .ok_or("missing extension_count column")?;
    let mut result = HashMap::new();
    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        let trace = fields[trace_index].parse::<i64>().map_err(|error| error.to_string())?;
        let count = fields[count_index].parse::<u64>().map_err(|error| error.to_string())?;
        result.insert(fields[slug_index].to_string(), (trace, count));
    }
    Ok(result)
}

fn run_cell(cell: Cell, parent_trace: i64, parent_count: u64) -> Result<(String, bool), String> {
    let started = Instant::now();
    let modulus = first_irreducible(cell.p, cell.degree)?;
    let field = Field::new(cell.p, cell.degree, &modulus)?;
    let curve = Curve { field: &field, a: field.from_base(cell.a), b: field.from_base(cell.b) };
    let points = enumerate_points(&curve)?;
    let group_order = u64::try_from(points.len()).map_err(|_| "group too large")?;
    require(group_order == parent_count, format!("{}: parent count mismatch", cell.slug))?;

    let mut orders = Vec::with_capacity(points.len());
    for &point in &points {
        orders.push(point_order(&curve, point, group_order)?);
    }
    let maximum_order = *orders.iter().max().ok_or("empty point list")?;
    let first_index = orders.iter().position(|&order| order == maximum_order)
        .ok_or("maximum-order point absent")?;
    let first = points[first_index];
    require(first != Point::Infinity, "selected identity generator")?;
    let first_closure = cyclic_points(&curve, first, maximum_order)?;
    let (generators, generator_orders, closure_size) = if maximum_order == group_order {
        (vec![first], vec![maximum_order], maximum_order)
    } else {
        let mut selected = None;
        for (index, &second) in points.iter().enumerate().skip(1) {
            let second_order = orders[index];
            let closure = pair_closure(&curve, first, maximum_order, second, second_order)?;
            if closure.len() == points.len() {
                selected = Some((second, second_order, closure.len()));
                break;
            }
        }
        let (second, second_order, size) = selected.ok_or("no canonical second generator")?;
        (vec![first, second], vec![maximum_order, second_order], u64::try_from(size).map_err(|_| "closure too large")?)
    };
    require(generators.len() <= 2, "more than two generators")?;
    for (&generator, &order) in generators.iter().zip(&generator_orders) {
        require(repeated_order(&curve, generator, group_order)? == order, "repeated order mismatch")?;
    }

    let charpoly_failures = generators.iter().filter(|&&point|
        characteristic(&curve, point, parent_trace, cell.p).ok() != Some(Point::Infinity)).count();
    let wrong_trace_rejections = generators.iter().filter(|&&point|
        characteristic(&curve, point, parent_trace + 1, cell.p).ok() != Some(Point::Infinity)).count();
    let wrong_norm_rejections = generators.iter().filter(|&&point|
        characteristic(&curve, point, parent_trace, cell.p + 1).ok() != Some(Point::Infinity)).count();
    let mut missing_inverse_rejections = 0_usize;
    for &point in &generators {
        if missing_inverse_rejects(&curve, point, parent_trace, cell.p, cell.degree)? {
            missing_inverse_rejections += 1;
        }
    }
    let dropped_closure_size = if generators.len() == 1 {
        1_u64
    } else {
        u64::try_from(first_closure.len()).map_err(|_| "dropped closure too large")?
    };
    let second_encoding = generators.get(1).map(|&point| encode_point(&field, point))
        .unwrap_or_else(|| "-".to_string());
    let second_order = generator_orders.get(1).copied().unwrap_or(0);
    let generator_count = generators.len();
    let accepted = closure_size == group_order
        && dropped_closure_size < group_order
        && charpoly_failures == 0
        && wrong_trace_rejections > 0
        && wrong_norm_rejections > 0;
    let row = format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        cell.slug,
        group_order,
        parent_trace,
        generator_count,
        encode_point(&field, generators[0]),
        generator_orders[0],
        second_encoding,
        second_order,
        closure_size,
        dropped_closure_size,
        charpoly_failures,
        wrong_trace_rejections,
        wrong_norm_rejections,
        missing_inverse_rejections,
        group_order,
        generator_count,
        group_order - u64::try_from(generator_count).map_err(|_| "generator count too large")?,
        started.elapsed().as_nanos(),
    );
    Ok((row, accepted))
}

fn run(parent_path: &str, output_path: &str) -> Result<(), String> {
    let parents = parent_rows(parent_path)?;
    let mut output = String::from(
        "slug\tgroup_order\ttrace\tgenerator_count\tgenerator_1\torder_1\tgenerator_2\torder_2\tclosure_size\tdropped_closure_size\tcharpoly_failures\twrong_trace_rejections\twrong_norm_rejections\tmissing_inverse_rejections\texhaustive_checks\tgenerator_checks\tchecks_saved\telapsed_ns\n",
    );
    let mut all_accepted = true;
    for cell in CELLS {
        let &(trace, count) = parents.get(cell.slug).ok_or("missing parent cell")?;
        let (row, accepted) = run_cell(cell, trace, count)?;
        write!(output, "{row}").map_err(|error| error.to_string())?;
        all_accepted &= accepted;
    }
    std::fs::write(output_path, output).map_err(|error| error.to_string())?;
    require(all_accepted, "one or more generator cells failed acceptance")?;
    println!("h116 generator charpoly panel: 8/8 cells accepted");
    Ok(())
}

fn selftest() -> Result<(), String> {
    h116_field::selftest()?;
    require(prime_divisors(60) == vec![2, 3, 5], "factor selftest failed")?;
    println!("h116 generator charpoly selftest ok");
    Ok(())
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let result = match arguments.as_slice() {
        [_, flag] if flag == "--selftest" => selftest(),
        [_, parent, output] => run(parent, output),
        _ => Err("usage: h116_generator_charpoly (--selftest|PARENT.tsv OUTPUT.tsv)".to_string()),
    };
    if let Err(error) = result {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
