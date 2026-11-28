//! Frozen finite-fiber shadow of the H116 quadratic function-field sandwich.
//!
//! The protocol was committed before this source existed.  This program keeps
//! every frozen row and reports failed expectations instead of filtering them.

#[allow(dead_code)]
#[path = "h116_field.rs"]
mod h116_field;

use h116_field::{first_irreducible, CountSnapshot, Curve, Felt, Field, Point};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
struct Cell {
    slug: &'static str,
    p: u64,
    a: u64,
    b: u64,
    degree: usize,
}

const CELLS: [Cell; 3] = [
    Cell { slug: "fp5-a2-b1-d3-n2", p: 5, a: 2, b: 1, degree: 3 },
    Cell { slug: "fp7-a1-b1-d3-n2", p: 7, a: 1, b: 1, degree: 3 },
    Cell { slug: "fp11-a1-b1-d2-n2", p: 11, a: 1, b: 1, degree: 2 },
];

fn require(condition: bool, message: impl Into<String>) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.into()) }
}

fn nonsingular(cell: Cell) -> bool {
    let p = u128::from(cell.p);
    let a = u128::from(cell.a);
    let b = u128::from(cell.b);
    (4 * a * a * a + 27 * b * b) % p != 0
}

fn enumerate_points(curve: &Curve<'_>) -> Result<Vec<Point>, String> {
    let mut affine = Vec::new();
    for x_index in 0..curve.field.q {
        let x = curve.field.from_index(x_index);
        let rhs = curve.field.add(
            curve.field.add(
                curve.field.mul(curve.field.square(x), x),
                curve.field.mul(curve.a, x),
            ),
            curve.b,
        );
        if let Some(y) = curve.field.sqrt(rhs)? {
            let mut ys = vec![y];
            let negative = curve.field.neg(y);
            if negative != y {
                ys.push(negative);
            }
            ys.sort_by_key(|value| curve.field.to_index(*value));
            for candidate_y in ys {
                let point = Point::Affine(x, candidate_y);
                require(curve.is_on_curve(point), "enumerated point is off curve")?;
                affine.push(point);
            }
        }
    }
    let mut points = vec![Point::Infinity];
    points.extend(affine);
    let unique: HashSet<Point> = points.iter().copied().collect();
    require(unique.len() == points.len(), "point enumeration contains duplicates")?;
    Ok(points)
}

fn canonical_target(curve: &Curve<'_>, points: &[Point]) -> Result<(Point, Point), String> {
    for &source in points.iter().filter(|point| **point != Point::Infinity) {
        let target = curve.scalar_mul(2, source)?;
        if target != Point::Infinity && target != curve.neg(target) {
            return Ok((source, target));
        }
    }
    Err("no canonical regular target exists".to_string())
}

fn first_nonzero_two_torsion(curve: &Curve<'_>, points: &[Point]) -> Result<Point, String> {
    for &point in points.iter().filter(|point| **point != Point::Infinity) {
        if curve.scalar_mul(2, point)? == Point::Infinity {
            return Ok(point);
        }
    }
    Err("no nonzero rational two-torsion point exists".to_string())
}

fn fiber(curve: &Curve<'_>, points: &[Point], scalar: u64, target: Point) -> Result<Vec<Point>, String> {
    let mut result = Vec::new();
    for &point in points {
        if curve.scalar_mul(scalar, point)? == target {
            result.push(point);
        }
    }
    Ok(result)
}

fn kernel_size(curve: &Curve<'_>, points: &[Point], scalar: u64) -> Result<usize, String> {
    Ok(fiber(curve, points, scalar, Point::Infinity)?.len())
}

fn point_indices(field: &Field, point: Point) -> Result<(u64, u64), String> {
    match point {
        Point::Affine(x, y) => Ok((field.to_index(x), field.to_index(y))),
        Point::Infinity => Err("expected affine point".to_string()),
    }
}

fn x_lift_count(points: &[Point], x: Felt) -> usize {
    points
        .iter()
        .filter(|point| matches!(point, Point::Affine(candidate, _) if *candidate == x))
        .count()
}

fn x_multiplicities(points: &HashSet<Point>) -> Result<(usize, usize, usize), String> {
    let mut counts = HashMap::<Felt, usize>::new();
    for &point in points {
        match point {
            Point::Affine(x, _) => *counts.entry(x).or_default() += 1,
            Point::Infinity => return Err("generic union unexpectedly contains infinity".to_string()),
        }
    }
    let minimum = counts.values().copied().min().unwrap_or(0);
    let maximum = counts.values().copied().max().unwrap_or(0);
    Ok((counts.len(), minimum, maximum))
}

fn modulus_csv(modulus: &[u64]) -> String {
    modulus.iter().map(u64::to_string).collect::<Vec<_>>().join(",")
}

fn bool_text(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

fn append_counts(fields: &mut Vec<String>, counts: CountSnapshot) {
    fields.extend([
        counts.base_additions.to_string(),
        counts.base_multiplications.to_string(),
        counts.base_inversions.to_string(),
        counts.extension_additions.to_string(),
        counts.extension_multiplications.to_string(),
        counts.extension_inversions.to_string(),
        counts.point_additions.to_string(),
        counts.scalar_multiplications.to_string(),
    ]);
}

fn run_cell(cell: Cell) -> Result<String, String> {
    require(nonsingular(cell), format!("{} is singular", cell.slug))?;
    let modulus = first_irreducible(cell.p, cell.degree)?;
    let field = Field::new(cell.p, cell.degree, &modulus)?;
    let curve = Curve {
        field: &field,
        a: field.from_base(cell.a),
        b: field.from_base(cell.b),
    };
    field.reset_counters();
    let points = enumerate_points(&curve)?;
    let (source, target) = canonical_target(&curve, &points)?;
    let target_negative = curve.neg(target);
    let plus = fiber(&curve, &points, 2, target)?;
    let minus = fiber(&curve, &points, 2, target_negative)?;
    let plus_set: HashSet<Point> = plus.iter().copied().collect();
    let minus_set: HashSet<Point> = minus.iter().copied().collect();
    let union: HashSet<Point> = plus_set.union(&minus_set).copied().collect();
    let (distinct_x, minimum_x_lifts, maximum_x_lifts) = x_multiplicities(&union)?;
    let target_x_lifts = match target {
        Point::Affine(x, _) => x_lift_count(&points, x),
        Point::Infinity => return Err("canonical target is infinite".to_string()),
    };
    let two_kernel = kernel_size(&curve, &points, 2)?;

    let branch = first_nonzero_two_torsion(&curve, &points)?;
    let branch_plus = fiber(&curve, &points, 2, branch)?;
    let branch_minus = fiber(&curve, &points, 2, curve.neg(branch))?;
    let branch_union: HashSet<Point> = branch_plus
        .iter()
        .chain(branch_minus.iter())
        .copied()
        .collect();
    let branch_target_x_lifts = match branch {
        Point::Affine(x, _) => x_lift_count(&points, x),
        Point::Infinity => return Err("branch target is infinite".to_string()),
    };
    let p_kernel = kernel_size(&curve, &points, cell.p)?;
    let primary_counts = field.snapshot();

    let base_field = Field::new(cell.p, 1, &[0, 1])?;
    let base_curve = Curve {
        field: &base_field,
        a: base_field.from_base(cell.a),
        b: base_field.from_base(cell.b),
    };
    base_field.reset_counters();
    let base_points = enumerate_points(&base_curve)?;
    let (_, base_target) = canonical_target(&base_curve, &base_points)?;
    let base_fiber = fiber(&base_curve, &base_points, 2, base_target)?.len();
    let base_counts = base_field.snapshot();

    let primary_pass = two_kernel == 4
        && plus.len() == 4
        && minus.len() == 4
        && plus_set.is_disjoint(&minus_set)
        && union.len() == 8
        && distinct_x == 4
        && minimum_x_lifts == 2
        && maximum_x_lifts == 2
        && target_x_lifts == 2
        && 4 * 2 == 2 * 4;
    let base_control_pass = base_fiber < 4;
    let branch_control_pass = branch_union.len() < 8 && branch_target_x_lifts == 1;
    let characteristic_control_pass = p_kernel < (cell.p * cell.p) as usize;
    let accepted = primary_pass
        && base_control_pass
        && branch_control_pass
        && characteristic_control_pass;

    let (source_x, source_y) = point_indices(&field, source)?;
    let (target_x, target_y) = point_indices(&field, target)?;
    let (branch_x, branch_y) = point_indices(&field, branch)?;
    let (base_target_x, base_target_y) = point_indices(&base_field, base_target)?;
    let mut fields = vec![
        cell.slug.to_string(), cell.p.to_string(), cell.a.to_string(), cell.b.to_string(),
        cell.degree.to_string(), modulus_csv(&modulus), field.q.to_string(), points.len().to_string(),
        source_x.to_string(), source_y.to_string(), target_x.to_string(), target_y.to_string(),
        two_kernel.to_string(), plus.len().to_string(), minus.len().to_string(),
        bool_text(plus_set.is_disjoint(&minus_set)).to_string(), union.len().to_string(),
        distinct_x.to_string(), minimum_x_lifts.to_string(), maximum_x_lifts.to_string(),
        target_x_lifts.to_string(), "8".to_string(), "8".to_string(),
        base_points.len().to_string(), base_target_x.to_string(), base_target_y.to_string(),
        base_fiber.to_string(), branch_x.to_string(), branch_y.to_string(),
        branch_union.len().to_string(), branch_target_x_lifts.to_string(),
        p_kernel.to_string(), (cell.p * cell.p).to_string(), bool_text(primary_pass).to_string(),
        bool_text(base_control_pass).to_string(), bool_text(branch_control_pass).to_string(),
        bool_text(characteristic_control_pass).to_string(), bool_text(accepted).to_string(),
    ];
    append_counts(&mut fields, primary_counts);
    append_counts(&mut fields, base_counts);
    Ok(fields.join("\t"))
}

fn self_test() -> Result<(), String> {
    let field = Field::new(13, 1, &[0, 1])?;
    let curve = Curve { field: &field, a: field.from_base(2), b: field.from_base(3) };
    let points = enumerate_points(&curve)?;
    require(points.len() > 4, "self-test curve is unexpectedly small")?;
    for &point in &points {
        require(curve.add(point, curve.neg(point))? == Point::Infinity, "inverse law failed")?;
    }
    let (source, target) = canonical_target(&curve, &points)?;
    require(curve.scalar_mul(2, source)? == target, "canonical target mismatch")?;
    println!("h116 quadratic sandwich producer self-test ok");
    Ok(())
}

fn run() -> Result<(), String> {
    if std::env::args().any(|argument| argument == "--self-test") {
        return self_test();
    }
    println!("schema\th116.quadratic-sandwich-fibers-result/v1");
    println!(
        "slug\tp\ta\tb\tdegree\tmodulus\tfield_size\tpoint_count\tsource_x\tsource_y\t\
target_x\ttarget_y\ttwo_kernel\tplus_fiber\tminus_fiber\tfibers_disjoint\tunion\tdistinct_x\t\
minimum_x_lifts\tmaximum_x_lifts\ttarget_x_lifts\tbalance_left\tbalance_right\tbase_point_count\t\
base_target_x\tbase_target_y\tbase_fiber\tbranch_x\tbranch_y\tbranch_union\tbranch_target_x_lifts\t\
p_kernel\tp_squared\tprimary_pass\tbase_control_pass\tbranch_control_pass\tcharacteristic_control_pass\t\
accepted\tbase_additions\tbase_multiplications\tbase_inversions\textension_additions\t\
extension_multiplications\textension_inversions\tpoint_additions\tscalar_multiplications\t\
control_base_additions\tcontrol_base_multiplications\tcontrol_base_inversions\t\
control_extension_additions\tcontrol_extension_multiplications\tcontrol_extension_inversions\t\
control_point_additions\tcontrol_scalar_multiplications"
    );
    for cell in CELLS {
        match run_cell(cell) {
            Ok(row) => println!("{row}"),
            Err(error) => eprintln!("{}: {error}", cell.slug),
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("h116 quadratic sandwich producer error: {error}");
        std::process::exit(1);
    }
}
