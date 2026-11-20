//! Independent replay for H116-QUADRATIC-SANDWICH-FIBERS-001.
//!
//! This file deliberately does not import the producer or `h116_field.rs`.
//! It uses a dynamically sized polynomial-basis field and brute-force point
//! enumeration, then compares every mathematical output column.

use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Elem(Vec<u64>);

#[derive(Clone, Debug)]
struct Field {
    p: u64,
    d: usize,
    q: u64,
    modulus: Vec<u64>,
}

fn power_u64(mut base: u64, mut exponent: usize) -> u64 {
    let mut result = 1_u64;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result.checked_mul(base).expect("field-size overflow");
        }
        exponent >>= 1;
        if exponent > 0 {
            base = base.checked_mul(base).expect("field-size overflow");
        }
    }
    result
}

impl Field {
    fn new(p: u64, d: usize, modulus: Vec<u64>) -> Result<Self, String> {
        if modulus.len() != d + 1 || modulus[d] != 1 {
            return Err("invalid monic modulus".to_string());
        }
        Ok(Self { p, d, q: power_u64(p, d), modulus })
    }

    fn zero(&self) -> Elem {
        Elem(vec![0; self.d])
    }

    fn one(&self) -> Elem {
        self.constant(1)
    }

    fn constant(&self, value: u64) -> Elem {
        let mut coefficients = vec![0; self.d];
        coefficients[0] = value % self.p;
        Elem(coefficients)
    }

    fn decode(&self, mut index: u64) -> Elem {
        assert!(index < self.q);
        let mut coefficients = vec![0; self.d];
        for coefficient in &mut coefficients {
            *coefficient = index % self.p;
            index /= self.p;
        }
        Elem(coefficients)
    }

    fn encode(&self, value: &Elem) -> u64 {
        let mut result = 0_u64;
        let mut place = 1_u64;
        for coefficient in &value.0 {
            result += coefficient * place;
            place *= self.p;
        }
        result
    }

    fn add(&self, left: &Elem, right: &Elem) -> Elem {
        Elem(left.0.iter().zip(&right.0).map(|(a, b)| (a + b) % self.p).collect())
    }

    fn neg(&self, value: &Elem) -> Elem {
        Elem(value.0.iter().map(|a| if *a == 0 { 0 } else { self.p - a }).collect())
    }

    fn sub(&self, left: &Elem, right: &Elem) -> Elem {
        self.add(left, &self.neg(right))
    }

    fn mul(&self, left: &Elem, right: &Elem) -> Elem {
        let mut product = vec![0_u64; 2 * self.d - 1];
        for (i, a) in left.0.iter().enumerate() {
            for (j, b) in right.0.iter().enumerate() {
                product[i + j] = (product[i + j] + a * b) % self.p;
            }
        }
        for degree in (self.d..product.len()).rev() {
            let leading = product[degree];
            for i in 0..self.d {
                let reduction = leading * self.modulus[i] % self.p;
                product[degree - self.d + i] =
                    (product[degree - self.d + i] + self.p - reduction) % self.p;
            }
        }
        product.truncate(self.d);
        Elem(product)
    }

    fn pow(&self, mut base: Elem, mut exponent: u64) -> Elem {
        let mut result = self.one();
        while exponent > 0 {
            if exponent & 1 == 1 {
                result = self.mul(&result, &base);
            }
            exponent >>= 1;
            if exponent > 0 {
                base = self.mul(&base, &base);
            }
        }
        result
    }

    fn inv(&self, value: &Elem) -> Result<Elem, String> {
        if value == &self.zero() {
            Err("division by zero".to_string())
        } else {
            Ok(self.pow(value.clone(), self.q - 2))
        }
    }

    fn div(&self, numerator: &Elem, denominator: &Elem) -> Result<Elem, String> {
        Ok(self.mul(numerator, &self.inv(denominator)?))
    }
}

fn evaluate_base_polynomial(coefficients: &[u64], x: u64, p: u64) -> u64 {
    coefficients.iter().rev().fold(0_u64, |accumulator, coefficient| {
        (accumulator * x + coefficient) % p
    })
}

fn irreducible_degree_at_most_three(coefficients: &[u64], p: u64) -> bool {
    let degree = coefficients.len() - 1;
    match degree {
        1 => true,
        2 | 3 => (0..p).all(|x| evaluate_base_polynomial(coefficients, x, p) != 0),
        _ => false,
    }
}

fn first_irreducible(p: u64, degree: usize) -> Result<Vec<u64>, String> {
    let candidates = power_u64(p, degree);
    for code in 0..candidates {
        let mut remainder = code;
        let mut coefficients = vec![0_u64; degree + 1];
        for (i, coefficient) in coefficients.iter_mut().take(degree).enumerate() {
            let place = power_u64(p, degree - 1 - i);
            *coefficient = remainder / place;
            remainder %= place;
        }
        coefficients[degree] = 1;
        if irreducible_degree_at_most_three(&coefficients, p) {
            return Ok(coefficients);
        }
    }
    Err("no irreducible polynomial found".to_string())
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Point {
    Infinity,
    Affine(Elem, Elem),
}

#[derive(Clone)]
struct Curve {
    field: Field,
    a: Elem,
    b: Elem,
}

impl Curve {
    fn new(field: Field, a: u64, b: u64) -> Self {
        let coefficient_a = field.constant(a);
        let coefficient_b = field.constant(b);
        Self { field, a: coefficient_a, b: coefficient_b }
    }

    fn on_curve(&self, point: &Point) -> bool {
        match point {
            Point::Infinity => true,
            Point::Affine(x, y) => {
                let y2 = self.field.mul(y, y);
                let x2 = self.field.mul(x, x);
                let x3 = self.field.mul(&x2, x);
                let ax = self.field.mul(&self.a, x);
                y2 == self.field.add(&self.field.add(&x3, &ax), &self.b)
            }
        }
    }

    fn neg(&self, point: &Point) -> Point {
        match point {
            Point::Infinity => Point::Infinity,
            Point::Affine(x, y) => Point::Affine(x.clone(), self.field.neg(y)),
        }
    }

    fn add(&self, left: &Point, right: &Point) -> Result<Point, String> {
        match (left, right) {
            (Point::Infinity, point) | (point, Point::Infinity) => Ok(point.clone()),
            (Point::Affine(x1, y1), Point::Affine(x2, y2)) => {
                if x1 == x2 && self.field.add(y1, y2) == self.field.zero() {
                    return Ok(Point::Infinity);
                }
                let slope = if left == right {
                    let three = self.field.constant(3);
                    let two = self.field.constant(2);
                    let numerator = self.field.add(
                        &self.field.mul(&three, &self.field.mul(x1, x1)),
                        &self.a,
                    );
                    self.field.div(&numerator, &self.field.mul(&two, y1))?
                } else {
                    self.field.div(&self.field.sub(y2, y1), &self.field.sub(x2, x1))?
                };
                let x3 = self.field.sub(&self.field.sub(&self.field.mul(&slope, &slope), x1), x2);
                let y3 = self.field.sub(&self.field.mul(&slope, &self.field.sub(x1, &x3)), y1);
                let result = Point::Affine(x3, y3);
                if self.on_curve(&result) { Ok(result) } else { Err("group law left curve".to_string()) }
            }
        }
    }

    fn scalar(&self, mut scalar: u64, point: &Point) -> Result<Point, String> {
        let mut accumulator = Point::Infinity;
        let mut multiple = point.clone();
        while scalar > 0 {
            if scalar & 1 == 1 {
                accumulator = self.add(&accumulator, &multiple)?;
            }
            scalar >>= 1;
            if scalar > 0 {
                multiple = self.add(&multiple, &multiple)?;
            }
        }
        Ok(accumulator)
    }
}

fn enumerate_points(curve: &Curve) -> Vec<Point> {
    let mut points = vec![Point::Infinity];
    for x_index in 0..curve.field.q {
        let x = curve.field.decode(x_index);
        for y_index in 0..curve.field.q {
            let point = Point::Affine(x.clone(), curve.field.decode(y_index));
            if curve.on_curve(&point) {
                points.push(point);
            }
        }
    }
    points
}

fn canonical_target(curve: &Curve, points: &[Point]) -> Result<(Point, Point), String> {
    for source in points.iter().filter(|point| **point != Point::Infinity) {
        let target = curve.scalar(2, source)?;
        if target != Point::Infinity && target != curve.neg(&target) {
            return Ok((source.clone(), target));
        }
    }
    Err("no regular canonical target".to_string())
}

fn fiber(curve: &Curve, points: &[Point], scalar: u64, target: &Point) -> Result<Vec<Point>, String> {
    let mut result = Vec::new();
    for point in points {
        if curve.scalar(scalar, point)? == *target {
            result.push(point.clone());
        }
    }
    Ok(result)
}

fn first_two_torsion(curve: &Curve, points: &[Point]) -> Result<Point, String> {
    for point in points.iter().filter(|point| **point != Point::Infinity) {
        if curve.scalar(2, point)? == Point::Infinity {
            return Ok(point.clone());
        }
    }
    Err("missing two-torsion".to_string())
}

fn coordinates(field: &Field, point: &Point) -> Result<(u64, u64), String> {
    match point {
        Point::Affine(x, y) => Ok((field.encode(x), field.encode(y))),
        Point::Infinity => Err("expected affine point".to_string()),
    }
}

fn x_lifts(points: &[Point], target: &Point) -> Result<usize, String> {
    let target_x = match target {
        Point::Affine(x, _) => x,
        Point::Infinity => return Err("infinite target has no x-coordinate".to_string()),
    };
    Ok(points.iter().filter(|point| matches!(point, Point::Affine(x, _) if x == target_x)).count())
}

fn recompute(p: u64, a: u64, b: u64, degree: usize) -> Result<HashMap<&'static str, String>, String> {
    let modulus = first_irreducible(p, degree)?;
    let field = Field::new(p, degree, modulus.clone())?;
    let curve = Curve::new(field, a, b);
    let points = enumerate_points(&curve);
    let (source, target) = canonical_target(&curve, &points)?;
    let negative_target = curve.neg(&target);
    let plus = fiber(&curve, &points, 2, &target)?;
    let minus = fiber(&curve, &points, 2, &negative_target)?;
    let plus_set: HashSet<Point> = plus.iter().cloned().collect();
    let minus_set: HashSet<Point> = minus.iter().cloned().collect();
    let union: HashSet<Point> = plus_set.union(&minus_set).cloned().collect();
    let mut x_counts = HashMap::<Elem, usize>::new();
    for point in &union {
        if let Point::Affine(x, _) = point {
            *x_counts.entry(x.clone()).or_default() += 1;
        }
    }
    let minimum_x_lifts = x_counts.values().copied().min().unwrap_or(0);
    let maximum_x_lifts = x_counts.values().copied().max().unwrap_or(0);
    let two_kernel = fiber(&curve, &points, 2, &Point::Infinity)?.len();
    let branch = first_two_torsion(&curve, &points)?;
    let branch_plus = fiber(&curve, &points, 2, &branch)?;
    let branch_negative = curve.neg(&branch);
    let branch_minus = fiber(&curve, &points, 2, &branch_negative)?;
    let branch_union: HashSet<Point> = branch_plus.into_iter().chain(branch_minus).collect();
    let p_kernel = fiber(&curve, &points, p, &Point::Infinity)?.len();

    let base_modulus = first_irreducible(p, 1)?;
    let base_curve = Curve::new(Field::new(p, 1, base_modulus)?, a, b);
    let base_points = enumerate_points(&base_curve);
    let (_, base_target) = canonical_target(&base_curve, &base_points)?;
    let base_fiber = fiber(&base_curve, &base_points, 2, &base_target)?.len();

    let target_x_lifts = x_lifts(&points, &target)?;
    let branch_target_x_lifts = x_lifts(&points, &branch)?;
    let primary_pass = two_kernel == 4
        && plus.len() == 4
        && minus.len() == 4
        && plus_set.is_disjoint(&minus_set)
        && union.len() == 8
        && x_counts.len() == 4
        && minimum_x_lifts == 2
        && maximum_x_lifts == 2
        && target_x_lifts == 2;
    let base_control_pass = base_fiber < 4;
    let branch_control_pass = branch_union.len() < 8 && branch_target_x_lifts == 1;
    let characteristic_control_pass = p_kernel < (p * p) as usize;
    let accepted = primary_pass && base_control_pass && branch_control_pass && characteristic_control_pass;
    let (source_x, source_y) = coordinates(&curve.field, &source)?;
    let (target_x, target_y) = coordinates(&curve.field, &target)?;
    let (branch_x, branch_y) = coordinates(&curve.field, &branch)?;
    let (base_target_x, base_target_y) = coordinates(&base_curve.field, &base_target)?;

    let mut values = HashMap::new();
    macro_rules! record {
        ($name:literal, $value:expr) => { values.insert($name, ($value).to_string()); };
    }
    record!("modulus", modulus.iter().map(u64::to_string).collect::<Vec<_>>().join(","));
    record!("field_size", curve.field.q);
    record!("point_count", points.len());
    record!("source_x", source_x); record!("source_y", source_y);
    record!("target_x", target_x); record!("target_y", target_y);
    record!("two_kernel", two_kernel); record!("plus_fiber", plus.len());
    record!("minus_fiber", minus.len()); record!("fibers_disjoint", plus_set.is_disjoint(&minus_set));
    record!("union", union.len()); record!("distinct_x", x_counts.len());
    record!("minimum_x_lifts", minimum_x_lifts); record!("maximum_x_lifts", maximum_x_lifts);
    record!("target_x_lifts", target_x_lifts); record!("balance_left", 8); record!("balance_right", 8);
    record!("base_point_count", base_points.len()); record!("base_target_x", base_target_x);
    record!("base_target_y", base_target_y); record!("base_fiber", base_fiber);
    record!("branch_x", branch_x); record!("branch_y", branch_y);
    record!("branch_union", branch_union.len()); record!("branch_target_x_lifts", branch_target_x_lifts);
    record!("p_kernel", p_kernel); record!("p_squared", p * p);
    record!("primary_pass", primary_pass); record!("base_control_pass", base_control_pass);
    record!("branch_control_pass", branch_control_pass);
    record!("characteristic_control_pass", characteristic_control_pass); record!("accepted", accepted);
    Ok(values)
}

fn parse_u64(row: &HashMap<&str, &str>, name: &str) -> Result<u64, String> {
    row.get(name).ok_or_else(|| format!("missing {name}"))?.parse().map_err(|_| format!("invalid {name}"))
}

fn verify_file(path: &str) -> Result<(), String> {
    let contents = fs::read_to_string(path).map_err(|error| format!("read {path}: {error}"))?;
    let mut lines = contents.lines();
    if lines.next() != Some("schema\th116.quadratic-sandwich-fibers-result/v1") {
        return Err("unexpected schema".to_string());
    }
    let header: Vec<&str> = lines.next().ok_or_else(|| "missing header".to_string())?.split('\t').collect();
    let expected_slugs = ["fp5-a2-b1-d3-n2", "fp7-a1-b1-d3-n2", "fp11-a1-b1-d2-n2"];
    let compared = [
        "modulus", "field_size", "point_count", "source_x", "source_y", "target_x", "target_y",
        "two_kernel", "plus_fiber", "minus_fiber", "fibers_disjoint", "union", "distinct_x",
        "minimum_x_lifts", "maximum_x_lifts", "target_x_lifts", "balance_left", "balance_right",
        "base_point_count", "base_target_x", "base_target_y", "base_fiber", "branch_x", "branch_y",
        "branch_union", "branch_target_x_lifts", "p_kernel", "p_squared", "primary_pass",
        "base_control_pass", "branch_control_pass", "characteristic_control_pass", "accepted",
    ];
    let operation_columns = &header[38..];
    let rows: Vec<&str> = lines.filter(|line| !line.is_empty()).collect();
    if rows.len() != expected_slugs.len() {
        return Err(format!("expected {} rows, got {}", expected_slugs.len(), rows.len()));
    }
    let mut accepted_rows = 0_usize;
    for (row_index, line) in rows.iter().enumerate() {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != header.len() {
            return Err(format!("row {} has {} fields, expected {}", row_index + 1, fields.len(), header.len()));
        }
        let row: HashMap<&str, &str> = header.iter().copied().zip(fields.iter().copied()).collect();
        if row["slug"] != expected_slugs[row_index] {
            return Err(format!("row {} slug mismatch", row_index + 1));
        }
        let p = parse_u64(&row, "p")?;
        let a = parse_u64(&row, "a")?;
        let b = parse_u64(&row, "b")?;
        let degree = usize::try_from(parse_u64(&row, "degree")?).map_err(|_| "degree overflow".to_string())?;
        let replay = recompute(p, a, b, degree)?;
        for name in compared {
            if row[name] != replay[name] {
                return Err(format!("{}: {name} recorded {}, replay {}", row["slug"], row[name], replay[name]));
            }
        }
        for name in operation_columns {
            let _: u64 = row[name].parse().map_err(|_| format!("{}: invalid operation count {name}", row["slug"]))?;
        }
        accepted_rows += usize::from(row["accepted"] == "true");
    }
    println!("verified_rows\t{}", rows.len());
    println!("accepted_rows\t{accepted_rows}");
    println!("independent_arithmetic\ttrue");
    Ok(())
}

fn self_test() -> Result<(), String> {
    let modulus = first_irreducible(17, 2)?;
    let curve = Curve::new(Field::new(17, 2, modulus)?, 3, 5);
    let points = enumerate_points(&curve);
    if points.len() < 10 {
        return Err("self-test curve too small".to_string());
    }
    for point in points.iter().take(20) {
        if curve.add(point, &curve.neg(point))? != Point::Infinity {
            return Err("self-test inverse law failed".to_string());
        }
    }
    println!("h116 quadratic sandwich verifier self-test ok");
    Ok(())
}

fn run() -> Result<(), String> {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.get(1).is_some_and(|argument| argument == "--self-test") {
        self_test()
    } else if let Some(path) = arguments.get(1) {
        verify_file(path)
    } else {
        Err("usage: verifier RESULT.tsv | --self-test".to_string())
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("h116 quadratic sandwich verifier error: {error}");
        std::process::exit(1);
    }
}
