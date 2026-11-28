#[allow(dead_code)]
#[path = "h116_field.rs"]
mod h116_field;

use h116_field::{first_irreducible, trace_group_order, Curve, Field, Point};
use std::collections::HashSet;
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

fn discriminant_nonzero(cell: Cell) -> bool {
    let p = u128::from(cell.p);
    let a = u128::from(cell.a);
    let b = u128::from(cell.b);
    (4 * a * a * a + 27 * b * b) % p != 0
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
            let point = Point::Affine(x, y);
            require(curve.is_on_curve(point), "enumerated point is off curve")?;
            points.push(point);
            if y != curve.field.zero() {
                let negative = Point::Affine(x, curve.field.neg(y));
                require(curve.is_on_curve(negative), "negative point is off curve")?;
                points.push(negative);
            }
        }
    }
    let unique: HashSet<Point> = points.iter().copied().collect();
    require(unique.len() == points.len(), "point enumeration contains duplicates")?;
    Ok(points)
}

fn signed_scalar_mul(curve: &Curve<'_>, scalar: i64, point: Point) -> Result<Point, String> {
    let absolute = curve.scalar_mul(scalar.unsigned_abs(), point)?;
    Ok(if scalar < 0 { curve.neg(absolute) } else { absolute })
}

fn charpoly_value(
    curve: &Curve<'_>,
    point: Point,
    trace: i64,
    norm: u64,
) -> Result<Point, String> {
    let pi = curve.frobenius(point);
    let pi_squared = curve.frobenius(pi);
    let trace_pi = signed_scalar_mul(curve, trace, pi)?;
    let norm_point = curve.scalar_mul(norm, point)?;
    curve.add(curve.sub(pi_squared, trace_pi)?, norm_point)
}

fn pointwise_v(
    curve: &Curve<'_>,
    point: Point,
    norm: u64,
    use_inverse: bool,
    degree: usize,
) -> Result<Point, String> {
    let input = if use_inverse {
        curve.frobenius_power(point, degree - 1)
    } else {
        point
    };
    curve.scalar_mul(norm, input)
}

fn trace_minus_pi(curve: &Curve<'_>, point: Point, trace: i64) -> Result<Point, String> {
    curve.sub(signed_scalar_mul(curve, trace, point)?, curve.frobenius(point))
}

fn base_count_and_trace(cell: Cell) -> Result<(u64, i64), String> {
    let field = Field::new(cell.p, 1, &[0, 1])?;
    let curve = Curve {
        field: &field,
        a: field.from_base(cell.a),
        b: field.from_base(cell.b),
    };
    let count = u64::try_from(enumerate_points(&curve)?.len())
        .map_err(|_| "base point count does not fit u64".to_string())?;
    let trace_i128 = i128::from(cell.p) + 1 - i128::from(count);
    let trace = i64::try_from(trace_i128).map_err(|_| "trace does not fit i64".to_string())?;
    Ok((count, trace))
}

fn run_cell(cell: Cell) -> Result<(String, bool), String> {
    let started = Instant::now();
    require(discriminant_nonzero(cell), format!("{}: singular curve", cell.slug))?;
    let (base_count, trace) = base_count_and_trace(cell)?;
    let modulus = first_irreducible(cell.p, cell.degree)?;
    let field = Field::new(cell.p, cell.degree, &modulus)?;
    let curve = Curve {
        field: &field,
        a: field.from_base(cell.a),
        b: field.from_base(cell.b),
    };
    field.reset_counters();
    let points = enumerate_points(&curve)?;
    let extension_count = u64::try_from(points.len())
        .map_err(|_| "extension point count does not fit u64".to_string())?;
    let predicted_count = trace_group_order(cell.p, trace, cell.degree)?;

    let mut pi_order_failures = 0_u64;
    let mut charpoly_failures = 0_u64;
    let mut v_failures = 0_u64;
    let mut wrong_trace_rejections = 0_u64;
    let mut wrong_norm_rejections = 0_u64;
    let mut missing_inverse_rejections = 0_u64;

    for &point in &points {
        if curve.frobenius_power(point, cell.degree) != point {
            pi_order_failures += 1;
        }
        if charpoly_value(&curve, point, trace, cell.p)? != Point::Infinity {
            charpoly_failures += 1;
        }
        let expected_v = trace_minus_pi(&curve, point, trace)?;
        if pointwise_v(&curve, point, cell.p, true, cell.degree)? != expected_v {
            v_failures += 1;
        }
        if charpoly_value(&curve, point, trace + 1, cell.p)? != Point::Infinity {
            wrong_trace_rejections += 1;
        }
        if pointwise_v(&curve, point, cell.p + 1, true, cell.degree)? != expected_v {
            wrong_norm_rejections += 1;
        }
        if pointwise_v(&curve, point, cell.p, false, cell.degree)? != expected_v {
            missing_inverse_rejections += 1;
        }
    }

    let snapshot = field.snapshot();
    let elapsed_ns = started.elapsed().as_nanos();
    let modulus_text = modulus.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
    let positive_pass = extension_count == predicted_count
        && pi_order_failures == 0
        && charpoly_failures == 0
        && v_failures == 0;
    let controls_pass = wrong_trace_rejections > 0
        && wrong_norm_rejections > 0
        && missing_inverse_rejections > 0;
    let accepted = positive_pass && controls_pass;
    let row = format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        cell.slug,
        cell.p,
        cell.a,
        cell.b,
        cell.degree,
        modulus_text,
        field.q,
        base_count,
        trace,
        extension_count,
        predicted_count,
        points.len(),
        pi_order_failures,
        charpoly_failures,
        v_failures,
        wrong_trace_rejections,
        wrong_norm_rejections,
        missing_inverse_rejections,
        snapshot.base_additions,
        snapshot.base_multiplications,
        snapshot.base_inversions,
        snapshot.extension_additions,
        snapshot.extension_multiplications,
        snapshot.extension_inversions,
        snapshot.frobenius_maps,
        snapshot.point_additions,
        snapshot.scalar_multiplications,
        elapsed_ns,
    );
    Ok((row, accepted))
}

fn run(output_path: &str) -> Result<(), String> {
    let mut output = String::from(
        "slug\tp\ta\tb\tdegree\tmodulus\tfield_size\tbase_count\ttrace\textension_count\tpredicted_count\tpoints_checked\tpi_order_failures\tcharpoly_failures\tv_failures\twrong_trace_rejections\twrong_norm_rejections\tmissing_inverse_rejections\tbase_additions\tbase_multiplications\tbase_inversions\textension_additions\textension_multiplications\textension_inversions\tfrobenius_maps\tpoint_additions\tscalar_multiplications\telapsed_ns\n",
    );
    let mut all_accepted = true;
    for cell in CELLS {
        let (row, accepted) = run_cell(cell)?;
        write!(output, "{row}").map_err(|error| error.to_string())?;
        all_accepted &= accepted;
    }
    std::fs::write(output_path, output).map_err(|error| error.to_string())?;
    require(all_accepted, "one or more frozen cells failed acceptance")?;
    println!("h116 pointwise charpoly panel: 8/8 cells accepted");
    Ok(())
}

fn selftest() -> Result<(), String> {
    h116_field::selftest()?;
    let field = Field::new(5, 1, &[0, 1])?;
    let curve = Curve { field: &field, a: field.from_base(2), b: field.from_base(1) };
    let points = enumerate_points(&curve)?;
    require(points.len() > 1, "selftest curve unexpectedly trivial")?;
    for &point in &points {
        require(curve.scalar_mul(0, point)? == Point::Infinity, "zero scalar failed")?;
        require(curve.add(point, curve.neg(point))? == Point::Infinity, "negation failed")?;
    }
    println!("h116 pointwise charpoly selftest ok");
    Ok(())
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let result = match arguments.as_slice() {
        [_, flag] if flag == "--selftest" => selftest(),
        [_, output_path] => run(output_path),
        _ => Err("usage: h116_pointwise_charpoly (--selftest|OUTPUT.tsv)".to_string()),
    };
    if let Err(error) = result {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
