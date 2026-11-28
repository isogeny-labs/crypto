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
    let unique: HashSet<Point> = points.iter().copied().collect();
    require(unique.len() == points.len(), "point enumeration contains duplicates")?;
    require(points.iter().all(|&point| curve.is_on_curve(point)), "off-curve point")?;
    Ok(points)
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

fn least_period(curve: &Curve<'_>, point: Point, degree: usize) -> Option<usize> {
    (1..=degree).find(|&period| curve.frobenius_power(point, period) == point)
}

fn encode_counts(counts: &[u64]) -> String {
    counts.iter().enumerate().skip(1)
        .map(|(index, count)| format!("{index}:{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn parent_rows(path: &str) -> Result<HashMap<String, (u64, i64, u64)>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().ok_or("empty parent result")?.split('\t').collect();
    let slug_index = header.iter().position(|&name| name == "slug").ok_or("missing slug")?;
    let base_index = header.iter().position(|&name| name == "base_count").ok_or("missing base_count")?;
    let trace_index = header.iter().position(|&name| name == "trace").ok_or("missing trace")?;
    let count_index = header.iter().position(|&name| name == "extension_count")
        .ok_or("missing extension_count")?;
    let mut result = HashMap::new();
    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        let base_count = fields[base_index].parse::<u64>().map_err(|error| error.to_string())?;
        let trace = fields[trace_index].parse::<i64>().map_err(|error| error.to_string())?;
        let extension_count = fields[count_index].parse::<u64>()
            .map_err(|error| error.to_string())?;
        result.insert(fields[slug_index].to_string(), (base_count, trace, extension_count));
    }
    Ok(result)
}

fn run_cell(
    cell: Cell,
    parent_base_count: u64,
    parent_trace: i64,
    parent_count: u64,
) -> Result<(String, bool), String> {
    let started = Instant::now();
    let modulus = first_irreducible(cell.p, cell.degree)?;
    let field = Field::new(cell.p, cell.degree, &modulus)?;
    let curve = Curve { field: &field, a: field.from_base(cell.a), b: field.from_base(cell.b) };
    field.reset_counters();
    let points = enumerate_points(&curve)?;
    let group_order = u64::try_from(points.len()).map_err(|_| "group too large")?;
    require(group_order == parent_count, format!("{}: parent count mismatch", cell.slug))?;

    let mut period_histogram = vec![0_u64; cell.degree + 1];
    let mut characteristic_failures = vec![0_u64; cell.degree + 1];
    let mut wrong_trace_rejections = vec![0_u64; cell.degree + 1];
    let mut wrong_norm_rejections = vec![0_u64; cell.degree + 1];
    let mut period_failures = 0_u64;
    let mut nondividing_periods = 0_u64;

    for &point in &points {
        let Some(period) = least_period(&curve, point, cell.degree) else {
            period_failures += 1;
            continue;
        };
        period_histogram[period] += 1;
        if cell.degree % period != 0 {
            nondividing_periods += 1;
        }
        if characteristic(&curve, point, parent_trace, cell.p)? != Point::Infinity {
            characteristic_failures[period] += 1;
        }
        if characteristic(&curve, point, parent_trace + 1, cell.p)? != Point::Infinity {
            wrong_trace_rejections[period] += 1;
        }
        if characteristic(&curve, point, parent_trace, cell.p + 1)? != Point::Infinity {
            wrong_norm_rejections[period] += 1;
        }
    }

    let mut fixed_counts = vec![0_u64; cell.degree + 1];
    let mut fixed_count_mismatches = 0_u64;
    for exponent in 1..=cell.degree {
        fixed_counts[exponent] = u64::try_from(points.iter().filter(|&&point|
            curve.frobenius_power(point, exponent) == point).count())
            .map_err(|_| "fixed count too large")?;
        let expected: u64 = (1..=cell.degree)
            .filter(|period| exponent % period == 0)
            .map(|period| period_histogram[period])
            .sum();
        if fixed_counts[exponent] != expected {
            fixed_count_mismatches += 1;
        }
    }

    let histogram_total: u64 = period_histogram.iter().sum();
    let proper_period_union: u64 = (1..cell.degree)
        .filter(|period| cell.degree % period == 0)
        .map(|period| period_histogram[period])
        .sum();
    let full_period_count = period_histogram[cell.degree];
    let base_locus_omitted = group_order.saturating_sub(fixed_counts[1]);
    let proper_period_omitted = group_order.saturating_sub(proper_period_union);
    let charpoly_failure_total: u64 = characteristic_failures.iter().sum();
    let wrong_trace_total: u64 = wrong_trace_rejections.iter().sum();
    let wrong_norm_total: u64 = wrong_norm_rejections.iter().sum();
    let snapshot = field.snapshot();

    let accepted = period_failures == 0
        && nondividing_periods == 0
        && histogram_total == group_order
        && fixed_count_mismatches == 0
        && fixed_counts[1] == parent_base_count
        && fixed_counts[cell.degree] == group_order
        && base_locus_omitted > 0
        && proper_period_union < group_order
        && proper_period_omitted == full_period_count
        && full_period_count > 0
        && charpoly_failure_total == 0
        && wrong_trace_total > 0
        && wrong_norm_total > 0;

    let row = format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        cell.slug,
        cell.p,
        cell.degree,
        group_order,
        parent_base_count,
        parent_trace,
        encode_counts(&period_histogram),
        encode_counts(&fixed_counts),
        encode_counts(&characteristic_failures),
        encode_counts(&wrong_trace_rejections),
        encode_counts(&wrong_norm_rejections),
        period_failures,
        nondividing_periods,
        histogram_total,
        fixed_count_mismatches,
        fixed_counts[1],
        fixed_counts[cell.degree],
        proper_period_union,
        full_period_count,
        base_locus_omitted,
        proper_period_omitted,
        charpoly_failure_total,
        wrong_trace_total,
        wrong_norm_total,
        snapshot.frobenius_maps,
        snapshot.point_additions,
        snapshot.scalar_multiplications,
        started.elapsed().as_nanos(),
        if accepted { 1 } else { 0 },
    );
    Ok((row, accepted))
}

fn run(parent_path: &str, output_path: &str) -> Result<(), String> {
    let parents = parent_rows(parent_path)?;
    let mut output = String::from(
        "slug\tp\tdegree\tgroup_order\tbase_count\ttrace\tperiod_histogram\tfixed_counts\tcharpoly_failure_histogram\twrong_trace_rejection_histogram\twrong_norm_rejection_histogram\tperiod_failures\tnondividing_periods\thistogram_total\tfixed_count_mismatches\tbase_fixed_count\tdegree_fixed_count\tproper_period_union\tfull_period_count\tbase_locus_omitted\tproper_period_omitted\tcharpoly_failures\twrong_trace_rejections\twrong_norm_rejections\tfrobenius_maps\tpoint_additions\tscalar_multiplications\telapsed_ns\taccepted\n",
    );
    let mut all_accepted = true;
    for cell in CELLS {
        let &(base_count, trace, count) = parents.get(cell.slug).ok_or("missing parent cell")?;
        let (row, accepted) = run_cell(cell, base_count, trace, count)?;
        write!(output, "{row}").map_err(|error| error.to_string())?;
        all_accepted &= accepted;
    }
    std::fs::write(output_path, output).map_err(|error| error.to_string())?;
    require(all_accepted, "one or more fixed-loci cells failed acceptance")?;
    println!("h116 Frobenius fixed-loci panel: 8/8 cells accepted");
    Ok(())
}

fn selftest() -> Result<(), String> {
    h116_field::selftest()?;
    let field = Field::new(3, 2, &first_irreducible(3, 2)?)?;
    let curve = Curve { field: &field, a: field.from_base(1), b: field.from_base(1) };
    require(least_period(&curve, Point::Infinity, 2) == Some(1), "infinity period failed")?;
    require(encode_counts(&[0, 3, 6]) == "1:3,2:6", "count encoding failed")?;
    println!("h116 Frobenius fixed-loci selftest ok");
    Ok(())
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let result = match arguments.as_slice() {
        [_, flag] if flag == "--selftest" => selftest(),
        [_, parent, output] => run(parent, output),
        _ => Err("usage: h116_frobenius_fixed_loci (--selftest|PARENT.tsv OUTPUT.tsv)".to_string()),
    };
    if let Err(error) = result {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
