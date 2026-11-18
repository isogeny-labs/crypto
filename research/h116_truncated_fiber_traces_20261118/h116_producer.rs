mod h116_field;
#[allow(dead_code)]
#[path = "../../src/hash/sha256.rs"]
mod sha256;

use h116_field::{
    first_irreducible, mod_inverse, torsion_table, trace_group_order, CountSnapshot, Curve, Felt,
    Field, Point,
};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;

const J: usize = 8;
const PROTOCOL_SHA256: &str = "42b510bd0f67d23ad49e5827d947510de5e1c68bf82d29c78fa35e4861fe4bf3";
const COHORT_SHA256: &str = "29e2fd5596ce3ed8e93933161e05b92229cd3065a892bc8ee59d6265f6fa2d99";
const TARGET_SEEDS: [u64; 5] = [116001, 116002, 116003, 116004, 116005];
const TARGET_HEADER: &str = "h116-targets/v1";
const LABEL_HEADER: &str = "h116-target-labels/v1";

#[derive(Clone, Copy, Debug)]
struct CurveSpec {
    slug: &'static str,
    p: u64,
    a: u64,
    b: u64,
    order: u64,
    trace: i64,
    degree: usize,
}

const CURVES: [CurveSpec; 5] = [
    CurveSpec { slug: "h116-fp107-a1-b25-r127-d3", p: 107, a: 1, b: 25, order: 127, trace: -19, degree: 3 },
    CurveSpec { slug: "h116-fp211-a1-b148-r197-d4", p: 211, a: 1, b: 148, order: 197, trace: 15, degree: 4 },
    CurveSpec { slug: "h116-fp241-a1-b57-r257-d4", p: 241, a: 1, b: 57, order: 257, trace: -15, degree: 4 },
    CurveSpec { slug: "h116-fp431-a1-b111-r397-d3", p: 431, a: 1, b: 111, order: 397, trace: 35, degree: 3 },
    CurveSpec { slug: "h116-fp587-a1-b204-r547-d3", p: 587, a: 1, b: 204, order: 547, trace: 41, degree: 3 },
];

#[derive(Clone, Debug)]
struct BlindedTarget {
    generator_label: u64,
    digest: String,
    x: u64,
    y: u64,
}

#[derive(Clone, Copy, Debug)]
struct PlantedControl {
    case_id: usize,
    candidate: Moments,
    direct: Moments,
    tamper_rejected: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Moments {
    x: [u64; J],
    y: [u64; J],
}

#[derive(Clone, Copy, Debug, Default)]
struct BaseOps {
    add: u64,
    mul: u64,
    inv: u64,
}

struct BaseField {
    p: u64,
    ops: BaseOps,
}

impl BaseField {
    fn new(p: u64) -> Self { Self { p, ops: BaseOps::default() } }
    fn add(&mut self, a: u64, b: u64) -> u64 { self.ops.add += 1; (a + b) % self.p }
    fn neg(&self, a: u64) -> u64 { if a == 0 { 0 } else { self.p - a } }
    fn sub(&mut self, a: u64, b: u64) -> u64 { self.add(a, self.neg(b)) }
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.ops.mul += 1;
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    fn inv(&mut self, a: u64) -> Result<u64, String> {
        self.ops.inv += 1;
        mod_inverse(a, self.p)
    }
    fn div(&mut self, a: u64, b: u64) -> Result<u64, String> {
        let inverse = self.inv(b)?;
        Ok(self.mul(a, inverse))
    }
    fn constant(&self, value: usize) -> u64 { value as u64 % self.p }
}

fn inverse_and_log(polynomial: &[u64; J + 1], f: &mut BaseField) -> ([u64; J], [u64; J]) {
    let mut inverse = [0_u64; J];
    inverse[0] = 1;
    for n in 1..J {
        let mut sum = 0;
        for i in 1..=n {
            let product = f.mul(polynomial[i], inverse[n - i]);
            sum = f.add(sum, product);
        }
        inverse[n] = f.neg(sum);
    }
    let mut log_derivative = [0_u64; J];
    for n in 0..J {
        let mut sum = 0;
        for i in 1..=(n + 1) {
            let derivative_coefficient = f.mul(f.constant(i), polynomial[i]);
            let product = f.mul(derivative_coefficient, inverse[n + 1 - i]);
            sum = f.add(sum, product);
        }
        log_derivative[n] = sum;
    }
    (inverse, log_derivative)
}

#[derive(Clone, Copy, Debug)]
struct Jet {
    p: u64,
    h: [u64; J],
    differential_squared: u64,
    differential_cubed: u64,
    setup_ops: BaseOps,
}

impl Jet {
    fn new(p: u64, curve_a: u64, curve_b: u64, trace: i64) -> Result<Self, String> {
        if p <= 2 * J as u64 + 3 { return Err(format!("characteristic {p} outside p>19")); }
        let mut f = BaseField::new(p);
        let differential = (trace - 1).rem_euclid(p as i64) as u64;
        if differential == 0 { return Err("zero differential for V-1".to_string()); }
        let d2 = f.mul(differential, differential);
        let d3 = f.mul(d2, differential);
        let d4 = f.mul(d2, d2);
        let d6 = f.mul(d3, d3);
        let target_a = f.mul(d4, curve_a);
        let target_b = f.mul(d6, curve_b);
        let mut h = [0_u64; J];
        let delta_a = f.sub(curve_a, target_a);
        let delta_b = f.sub(curve_b, target_b);
        h[1] = f.div(delta_a, 5)?;
        h[2] = f.div(delta_b, 7)?;
        for k in 3..J {
            let mut convolution = 0;
            for i in 1..=(k - 2) {
                let product = f.mul(h[i], h[k - 1 - i]);
                convolution = f.add(convolution, product);
            }
            let two_k_plus_three = 2 * k + 3;
            let denominator = (k - 2) * two_k_plus_three;
            let first_num = f.mul(3, convolution);
            let first = f.div(first_num, f.constant(denominator))?;
            let second_inner = f.mul(curve_a, h[k - 2]);
            let second_num = f.mul(f.constant(2 * k - 3), second_inner);
            let second = f.div(second_num, f.constant(two_k_plus_three))?;
            let third_inner = f.mul(curve_b, h[k - 3]);
            let third_num = f.mul(f.constant(2 * (k - 3)), third_inner);
            let third = f.div(third_num, f.constant(two_k_plus_three))?;
            let first_minus_second = f.sub(first, second);
            h[k] = f.sub(first_minus_second, third);
        }
        Ok(Self { p, h, differential_squared: d2, differential_cubed: d3, setup_ops: f.ops })
    }

    fn online(&self, target_x: u64, target_y: u64, reference_x: u64) -> Result<(Moments, BaseOps), String> {
        fn polynomial(jet: &Jet, u: u64, f: &BaseField) -> [u64; J + 1] {
            let mut polynomial = [0_u64; J + 1];
            polynomial[0] = 1;
            polynomial[1] = f.neg(u);
            polynomial[2..=J].copy_from_slice(&jet.h[1..J]);
            polynomial
        }

        let mut f = BaseField::new(self.p);
        let normalized_target_x = f.mul(self.differential_squared, target_x);
        let normalized_reference_x = f.mul(self.differential_squared, reference_x);
        let target_polynomial = polynomial(self, normalized_target_x, &f);
        let reference_polynomial = polynomial(self, normalized_reference_x, &f);
        let (inverse, target_log) = inverse_and_log(&target_polynomial, &mut f);
        let (_, reference_log) = inverse_and_log(&reference_polynomial, &mut f);
        let y_scale = f.mul(self.differential_cubed, target_y);
        let mut result = Moments::default();
        for index in 0..J {
            result.x[index] = f.sub(reference_log[index], target_log[index]);
            result.y[index] = f.mul(y_scale, inverse[index]);
        }
        Ok((result, f.ops))
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 { while b != 0 { (a, b) = (b, a % b); } a }

fn first_base_point(curve: &Curve<'_>, p: u64) -> Result<Point, String> {
    for x in 0..p {
        for y in 0..p {
            let point = Point::Affine(curve.field.from_base(x), curve.field.from_base(y));
            if curve.is_on_curve(point) { return Ok(point); }
        }
    }
    Err("base point not found".to_string())
}

fn canonical_sqrt(field: &Field, value: Felt) -> Result<Option<Felt>, String> {
    let Some(root) = field.sqrt(value)? else { return Ok(None); };
    let negative = field.neg(root);
    Ok(Some(if field.to_index(root) <= field.to_index(negative) { root } else { negative }))
}

struct DirectSetup {
    base_generator: Point,
    kernel_generator: Point,
    group_order: u64,
    cofactor: u64,
    states_examined: u64,
}

fn direct_setup(field: &Field, curve: &Curve<'_>, spec: CurveSpec) -> Result<DirectSetup, String> {
    let group_order = trace_group_order(spec.p, spec.trace, spec.degree)?;
    let r2 = spec.order.checked_mul(spec.order).ok_or("r^2 overflow")?;
    if group_order % r2 != 0 { return Err("r^2 does not divide extension order".to_string()); }
    let cofactor = group_order / r2;
    if gcd(cofactor, spec.order) != 1 { return Err("r divides torsion cofactor".to_string()); }
    let base_generator = first_base_point(curve, spec.p)?;
    if curve.scalar_mul(spec.order, base_generator)? != Point::Infinity {
        return Err("base point does not have order r".to_string());
    }
    for x_index in 0..field.q {
        let x = field.from_index(x_index);
        let rhs = field.add(field.add(field.mul(field.square(x), x), field.mul(curve.a, x)), curve.b);
        let Some(y) = canonical_sqrt(field, rhs)? else { continue; };
        let projected = curve.scalar_mul(cofactor, Point::Affine(x, y))?;
        if projected == Point::Infinity || curve.scalar_mul(spec.order, projected)? != Point::Infinity { continue; }
        let kernel = curve.sub(curve.frobenius(projected), projected)?;
        if kernel == Point::Infinity || curve.scalar_mul(spec.order, kernel)? != Point::Infinity { continue; }
        let inverse_frobenius = curve.frobenius_power(kernel, spec.degree - 1);
        let verschiebung = curve.scalar_mul(spec.p, inverse_frobenius)?;
        if curve.sub(verschiebung, kernel)? != Point::Infinity { continue; }
        return Ok(DirectSetup { base_generator, kernel_generator: kernel, group_order, cofactor, states_examined: x_index + 1 });
    }
    Err("kernel generator not found".to_string())
}

fn point_base_coordinates(field: &Field, point: Point) -> Result<(u64, u64), String> {
    let Point::Affine(x, y) = point else { return Err("unexpected infinity".to_string()); };
    Ok((field.to_base(x)?, field.to_base(y)?))
}

fn coordinate_map(field: &Field, point: Point, lambda: u64) -> Point {
    match point {
        Point::Infinity => Point::Infinity,
        Point::Affine(x, y) => {
            let l = field.from_base(lambda);
            let l2 = field.mul(l, l);
            let l3 = field.mul(l2, l);
            Point::Affine(field.mul(l2, x), field.mul(l3, y))
        }
    }
}

fn absolute_direct_moments(curve: &Curve<'_>, shift: Point, kernel: Point, order: u64, lambda: u64) -> Result<Moments, String> {
    let mut x_sum = [curve.field.zero(); J];
    let mut y_sum = [curve.field.zero(); J];
    let mut current = shift;
    for _ in 0..order {
        let mapped = coordinate_map(curve.field, current, lambda);
        let Point::Affine(x, y) = mapped else { return Err("infinity in affine fiber".to_string()); };
        let mut x_power = curve.field.one();
        for index in 0..J {
            y_sum[index] = curve.field.add(y_sum[index], curve.field.mul(y, x_power));
            x_power = curve.field.mul(x_power, x);
            x_sum[index] = curve.field.add(x_sum[index], x_power);
        }
        current = curve.add(current, kernel)?;
    }
    if current != shift { return Err("fiber walk did not close".to_string()); }
    let mut out = Moments::default();
    for i in 0..J {
        out.x[i] = curve.field.to_base(x_sum[i])?;
        out.y[i] = curve.field.to_base(y_sum[i])?;
    }
    Ok(out)
}

fn direct_relative(target: Moments, reference: Moments, p: u64) -> Moments {
    let mut out = target;
    for i in 0..J { out.x[i] = (target.x[i] + p - reference.x[i]) % p; }
    out
}

fn kernel_velu_h(curve: &Curve<'_>, kernel: Point, order: u64) -> Result<[u64; J], String> {
    let mut power_sums = [curve.field.zero(); J + 1];
    let mut point = kernel;
    for _ in 1..order {
        let Point::Affine(x, _) = point else { return Err("early infinity in kernel".to_string()); };
        let mut power = curve.field.one();
        for sum in &mut power_sums {
            *sum = curve.field.add(*sum, power);
            power = curve.field.mul(power, x);
        }
        point = curve.add(point, kernel)?;
    }
    if point != Point::Infinity { return Err("kernel did not close".to_string()); }
    let mut h = [0_u64; J];
    for i in 1..J {
        let first = curve.field.mul(curve.field.from_base((2 * i + 1) as u64), power_sums[i + 1]);
        let second = curve.field.mul(
            curve.field.from_base((2 * i - 1) as u64),
            curve.field.mul(curve.a, power_sums[i - 1]),
        );
        let mut value = curve.field.add(first, second);
        if i >= 2 {
            let third = curve.field.mul(
                curve.field.from_base((2 * (i - 1)) as u64),
                curve.field.mul(curve.b, power_sums[i - 2]),
            );
            value = curve.field.add(value, third);
        }
        h[i] = curve.field.to_base(value)?;
    }
    Ok(h)
}

fn section(curve: &Curve<'_>, spec: CurveSpec, target: Point) -> Result<Point, String> {
    let scalar = mod_inverse((spec.p + spec.order - 1) % spec.order, spec.order)?;
    curve.scalar_mul(scalar, target)
}

fn sha256_hex(bytes: &[u8]) -> String {
    sha256::sha256(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

fn target_digest(spec: CurveSpec, generator_label: u64, x: u64, y: u64) -> String {
    sha256_hex(format!(
        "H116-TARGET\0{PROTOCOL_SHA256}\0{COHORT_SHA256}\0{}\0{generator_label}\0{x}\0{y}",
        spec.slug
    ).as_bytes())
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut value = *state;
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    value ^ (value >> 31)
}

fn shuffled_scalars(order: u64, seed: u64) -> Vec<u64> {
    let mut values: Vec<u64> = (1..order).collect();
    let mut state = seed;
    for index in (1..values.len()).rev() {
        let swap = (splitmix64(&mut state) % (index as u64 + 1)) as usize;
        values.swap(index, swap);
    }
    values
}

fn prepare_targets(target_path: &Path, label_path: &Path) -> Result<(), String> {
    if target_path == label_path { return Err("target and label paths must differ".to_string()); }
    let target_file = File::create(target_path).map_err(|e| format!("create {}: {e}", target_path.display()))?;
    let label_file = File::create(label_path).map_err(|e| format!("create {}: {e}", label_path.display()))?;
    let mut targets = BufWriter::new(target_file);
    let mut labels = BufWriter::new(label_file);
    writeln!(targets, "{TARGET_HEADER}\t{PROTOCOL_SHA256}\t{COHORT_SHA256}").map_err(|e| e.to_string())?;
    writeln!(labels, "{LABEL_HEADER}\t{PROTOCOL_SHA256}\t{COHORT_SHA256}").map_err(|e| e.to_string())?;
    let mut rows = 0_u64;
    for (curve_index, spec) in CURVES.iter().copied().enumerate() {
        let modulus = first_irreducible(spec.p, spec.degree)?;
        let field = Field::new(spec.p, spec.degree, &modulus)?;
        let curve = Curve { field: &field, a: field.from_base(spec.a), b: field.from_base(spec.b) };
        let base_generator = first_base_point(&curve, spec.p)?;
        if curve.scalar_mul(spec.order, base_generator)? != Point::Infinity {
            return Err(format!("{} base generator has wrong order", spec.slug));
        }
        let scalar_order = shuffled_scalars(spec.order, TARGET_SEEDS[curve_index]);
        for generator_label in 1..=3_u64 {
            let generator = curve.scalar_mul(generator_label, base_generator)?;
            for &scalar in &scalar_order {
                let point = curve.scalar_mul(scalar, generator)?;
                let (x, y) = point_base_coordinates(&field, point)?;
                let digest = target_digest(spec, generator_label, x, y);
                writeln!(targets, "{}\t{}\t{}\t{}\t{}", spec.slug, generator_label, digest, x, y)
                    .map_err(|e| e.to_string())?;
                writeln!(labels, "{}\t{}\t{}\t{}", spec.slug, generator_label, digest, scalar)
                    .map_err(|e| e.to_string())?;
                rows += 1;
            }
        }
    }
    targets.flush().map_err(|e| e.to_string())?;
    labels.flush().map_err(|e| e.to_string())?;
    println!("h116 target preparation: rows={rows}");
    Ok(())
}

fn read_blinded_targets(path: &Path) -> Result<(Vec<Vec<BlindedTarget>>, String), String> {
    let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let manifest_sha256 = sha256_hex(&bytes);
    let text = std::str::from_utf8(&bytes).map_err(|e| format!("target manifest is not UTF-8: {e}"))?;
    let mut lines = text.lines();
    let expected_header = format!("{TARGET_HEADER}\t{PROTOCOL_SHA256}\t{COHORT_SHA256}");
    if lines.next() != Some(expected_header.as_str()) { return Err("target manifest header mismatch".to_string()); }
    let mut groups = vec![Vec::new(); CURVES.len() * 3];
    let mut digests = HashSet::new();
    for (line_index, line) in lines.enumerate() {
        let columns: Vec<&str> = line.split('\t').collect();
        if columns.len() != 5 { return Err(format!("target line {} has {} columns", line_index + 2, columns.len())); }
        let curve_index = CURVES.iter().position(|spec| spec.slug == columns[0])
            .ok_or_else(|| format!("unknown target curve {}", columns[0]))?;
        let spec = CURVES[curve_index];
        let generator_label: u64 = columns[1].parse().map_err(|_| format!("bad generator label at line {}", line_index + 2))?;
        if !(1..=3).contains(&generator_label) { return Err(format!("generator label out of range at line {}", line_index + 2)); }
        let x: u64 = columns[3].parse().map_err(|_| format!("bad x at line {}", line_index + 2))?;
        let y: u64 = columns[4].parse().map_err(|_| format!("bad y at line {}", line_index + 2))?;
        if x >= spec.p || y >= spec.p { return Err(format!("noncanonical coordinate at line {}", line_index + 2)); }
        let digest = columns[2].to_string();
        if digest != target_digest(spec, generator_label, x, y) {
            return Err(format!("target digest mismatch at line {}", line_index + 2));
        }
        if !digests.insert(digest.clone()) { return Err(format!("duplicate target digest at line {}", line_index + 2)); }
        groups[curve_index * 3 + generator_label as usize - 1].push(BlindedTarget { generator_label, digest, x, y });
    }
    for (curve_index, spec) in CURVES.iter().enumerate() {
        for generator_label in 1..=3_usize {
            let count = groups[curve_index * 3 + generator_label - 1].len();
            if count != spec.order as usize - 1 {
                return Err(format!("{} label {} has {} targets, expected {}", spec.slug, generator_label, count, spec.order - 1));
            }
        }
    }
    Ok((groups, manifest_sha256))
}

fn mul_mod_u64(left: u64, right: u64, modulus: u64) -> u64 {
    ((left as u128 * right as u128) % modulus as u128) as u64
}

fn pow_mod_u64(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1_u64;
    while exponent != 0 {
        if exponent & 1 == 1 { result = mul_mod_u64(result, base, modulus); }
        exponent >>= 1;
        if exponent != 0 { base = mul_mod_u64(base, base, modulus); }
    }
    result
}

fn planted_controls() -> Result<Vec<PlantedControl>, String> {
    let p = 1009_u64;
    let mut controls = Vec::new();
    for case_id in 0..32_usize {
        let degree = 9 + case_id % 8;
        let roots: Vec<u64> = (0..degree)
            .map(|index| 1 + ((case_id as u64 * 29 + index as u64 * 37) % (p - 1)))
            .collect();
        if roots.iter().copied().collect::<HashSet<_>>().len() != degree {
            return Err(format!("planted case {case_id} has repeated roots"));
        }
        let mut f = BaseField::new(p);
        let mut polynomial = [0_u64; J + 1];
        polynomial[0] = 1;
        for &root in &roots {
            for index in (1..=J).rev() {
                let product = f.mul(root, polynomial[index - 1]);
                polynomial[index] = f.sub(polynomial[index], product);
            }
        }
        let (inverse, log_derivative) = inverse_and_log(&polynomial, &mut f);
        let mut candidate = Moments::default();
        let mut direct = Moments::default();
        for index in 0..J {
            candidate.x[index] = if log_derivative[index] == 0 { 0 } else { p - log_derivative[index] };
            candidate.y[index] = inverse[index];
            direct.x[index] = roots.iter().fold(0_u64, |sum, &root| {
                (sum + pow_mod_u64(root, index as u64 + 1, p)) % p
            });
        }
        for (root_index, &root) in roots.iter().enumerate() {
            let denominator = roots.iter().enumerate().filter(|(other, _)| *other != root_index)
                .fold(1_u64, |product, (_, &other)| mul_mod_u64(product, (root + p - other) % p, p));
            let weight = pow_mod_u64(denominator, p - 2, p);
            for index in 0..J {
                let term = mul_mod_u64(weight, pow_mod_u64(root, (degree - 1 + index) as u64, p), p);
                direct.y[index] = (direct.y[index] + term) % p;
            }
        }
        if candidate != direct { return Err(format!("planted case {case_id} failed exact comparison")); }
        let mut tampered = candidate;
        if case_id % 16 < J {
            let index = case_id % J;
            tampered.x[index] = (tampered.x[index] + 1) % p;
        } else {
            let index = case_id % J;
            tampered.y[index] = (tampered.y[index] + 1) % p;
        }
        controls.push(PlantedControl { case_id, candidate, direct, tamper_rejected: tampered != direct });
    }
    Ok(controls)
}

fn array_json(values: &[u64]) -> String {
    format!("[{}]", values.iter().map(u64::to_string).collect::<Vec<_>>().join(","))
}

fn moments_json(value: Moments) -> String {
    format!("{{\"x\":{},\"y\":{}}}", array_json(&value.x), array_json(&value.y))
}

fn base_ops_json(value: BaseOps) -> String {
    format!("{{\"add\":{},\"mul\":{},\"inv\":{}}}", value.add, value.mul, value.inv)
}

fn extension_ops_json(value: CountSnapshot) -> String {
    format!(concat!("{{\"base_additions\":{},\"base_multiplications\":{},\"base_inversions\":{},",
        "\"extension_additions\":{},\"extension_multiplications\":{},\"extension_inversions\":{},",
        "\"frobenius_maps\":{},\"point_additions\":{},\"scalar_multiplications\":{}}}"),
        value.base_additions, value.base_multiplications, value.base_inversions,
        value.extension_additions, value.extension_multiplications, value.extension_inversions,
        value.frobenius_maps, value.point_additions, value.scalar_multiplications)
}

fn mapped_base_inputs(spec: CurveSpec, target: Point, reference: Point, field: &Field, lambda: u64) -> Result<(u64, u64, u64, u64, u64), String> {
    let (u, v) = point_base_coordinates(field, coordinate_map(field, target, lambda))?;
    let (u0, _) = point_base_coordinates(field, coordinate_map(field, reference, lambda))?;
    let l2 = lambda * lambda % spec.p;
    let l4 = l2 * l2 % spec.p;
    let l6 = l4 * l2 % spec.p;
    Ok((spec.a * l4 % spec.p, spec.b * l6 % spec.p, u, v, u0))
}

fn run_curve<W: Write>(writer: &mut W, spec: CurveSpec, target_groups: &[Vec<BlindedTarget>]) -> Result<(u64, u64), String> {
    if target_groups.len() != 3 { return Err("run_curve requires three blinded target groups".to_string()); }
    let modulus = first_irreducible(spec.p, spec.degree)?;
    let field = Field::new(spec.p, spec.degree, &modulus)?;
    let curve = Curve { field: &field, a: field.from_base(spec.a), b: field.from_base(spec.b) };
    let setup = direct_setup(&field, &curve, spec)?;
    let transverse = curve.scalar_mul(mod_inverse(spec.p - 1, spec.order)?, setup.kernel_generator)?;
    let torsion = torsion_table(&curve, setup.base_generator, transverse, spec.order)?;
    assert_eq!(torsion.len(), (spec.order * spec.order) as usize);
    writeln!(writer, "{{\"type\":\"setup\",\"curve\":\"{}\",\"modulus\":{},\"group_order\":{},\"cofactor\":{},\"states_examined\":{}}}",
        spec.slug, array_json(&modulus), setup.group_order, setup.cofactor, setup.states_examined).map_err(|e| e.to_string())?;
    let mut rows = 0_u64;
    let mut mismatches = 0_u64;
    for label in 1..=3_u64 {
        let generator = curve.scalar_mul(label, setup.base_generator)?;
        let targets = &target_groups[label as usize - 1];
        let mut target_points = Vec::with_capacity(targets.len());
        let mut point_keys = HashSet::new();
        for target in targets {
            if target.generator_label != label { return Err("blinded target label/group mismatch".to_string()); }
            if !point_keys.insert((target.x, target.y)) { return Err("duplicate blinded target point".to_string()); }
            let point = Point::Affine(field.from_base(target.x), field.from_base(target.y));
            if !curve.is_on_curve(point) || point == Point::Infinity {
                return Err(format!("{} target {} is not a nonidentity curve point", spec.slug, target.digest));
            }
            if curve.scalar_mul(spec.order, point)? != Point::Infinity {
                return Err(format!("{} target {} is outside the order-r subgroup", spec.slug, target.digest));
            }
            target_points.push(point);
        }
        if target_points.len() != spec.order as usize - 1 {
            return Err(format!("{} label {label} target census is incomplete", spec.slug));
        }
        for &lambda in &[1_u64, 2] {
            let (model_a, model_b, _, _, reference_x) = mapped_base_inputs(spec, generator, generator, &field, lambda)?;
            let jet = Jet::new(spec.p, model_a, model_b, spec.trace)?;
            let reference_section = section(&curve, spec, generator)?;
            field.reset_counters();
            let reference_direct = absolute_direct_moments(&curve, reference_section, setup.kernel_generator, spec.order, lambda)?;
            for (blinded, &target) in targets.iter().zip(&target_points) {
                let (_, _, target_x, target_y, _) = mapped_base_inputs(spec, target, generator, &field, lambda)?;
                let candidate_start = Instant::now();
                let (candidate, online_ops) = jet.online(target_x, target_y, reference_x)?;
                let candidate_ns = candidate_start.elapsed().as_nanos();
                let target_section = section(&curve, spec, target)?;
                field.reset_counters();
                let direct_start = Instant::now();
                let direct_absolute = absolute_direct_moments(&curve, target_section, setup.kernel_generator, spec.order, lambda)?;
                let direct_ns = direct_start.elapsed().as_nanos();
                let direct_ops = field.snapshot();
                let direct = direct_relative(direct_absolute, reference_direct, spec.p);
                field.reset_counters();
                let aa_absolute = absolute_direct_moments(&curve, target_section, setup.kernel_generator, spec.order, lambda)?;
                let aa = direct_relative(aa_absolute, reference_direct, spec.p);
                let aa_ops = field.snapshot();
                if direct != aa { return Err("direct A/A mismatch".to_string()); }
                let matches = candidate == direct;
                rows += 1;
                if !matches { mismatches += 1; }
                writeln!(writer, concat!("{{\"type\":\"row\",\"curve\":\"{}\",\"generator_label\":{},",
                    "\"target_digest\":\"{}\",\"lambda\":{},\"candidate\":{},\"direct\":{},\"direct_aa\":{},",
                    "\"match\":{},\"jet_setup_ops\":{},\"candidate_online_ops\":{},",
                    "\"direct_ops\":{},\"direct_aa_ops\":{},\"candidate_dev_ns\":{},\"direct_dev_ns\":{}}}"),
                    spec.slug, label, blinded.digest, lambda, moments_json(candidate), moments_json(direct), moments_json(aa),
                    matches, base_ops_json(jet.setup_ops), base_ops_json(online_ops),
                    extension_ops_json(direct_ops), extension_ops_json(aa_ops), candidate_ns, direct_ns).map_err(|e| e.to_string())?;
                if !matches { return Ok((rows, mismatches)); }
            }
        }
    }
    Ok((rows, mismatches))
}

fn run(target_path: &Path, output_path: &Path) -> Result<(), String> {
    let (target_groups, target_manifest_sha256) = read_blinded_targets(target_path)?;
    let file = File::create(output_path).map_err(|e| format!("create {}: {e}", output_path.display()))?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{{\"type\":\"header\",\"schema\":\"h116.component/v2\",\"protocol_sha256\":\"{PROTOCOL_SHA256}\",\"cohort_sha256\":\"{COHORT_SHA256}\",\"target_manifest_sha256\":\"{target_manifest_sha256}\",\"labels_available_to_process\":false}}")
        .map_err(|e| e.to_string())?;
    for control in planted_controls()? {
        writeln!(writer, "{{\"type\":\"positive_control\",\"case_id\":{},\"candidate\":{},\"direct\":{},\"match\":{},\"tamper_rejected\":{}}}",
            control.case_id, moments_json(control.candidate), moments_json(control.direct), control.candidate == control.direct, control.tamper_rejected)
            .map_err(|e| e.to_string())?;
    }
    for prime in [5_u64, 7, 13] {
        writeln!(writer, "{{\"type\":\"negative_control\",\"prime\":{prime},\"rejected\":{}}}", Jet::new(prime, 2 % prime, 4 % prime, -3).is_err())
            .map_err(|e| e.to_string())?;
    }
    let mut rows = 0;
    let mut mismatches = 0;
    for (curve_index, spec) in CURVES.iter().copied().enumerate() {
        let start = curve_index * 3;
        let (curve_rows, curve_mismatches) = run_curve(&mut writer, spec, &target_groups[start..start + 3])?;
        rows += curve_rows;
        mismatches += curve_mismatches;
        if curve_mismatches != 0 { break; }
    }
    writeln!(writer, "{{\"type\":\"summary\",\"rows\":{rows},\"mismatches\":{mismatches},\"status\":\"{}\"}}", if mismatches == 0 { "exact" } else { "first_mismatch" })
        .map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())?;
    println!("h116 component run: rows={rows} mismatches={mismatches}");
    Ok(())
}

fn selftest() -> Result<(), String> {
    h116_field::selftest()?;
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    for prime in [5_u64, 7, 13] { assert!(Jet::new(prime, 2 % prime, 4 % prime, -3).is_err()); }
    let controls = planted_controls()?;
    assert_eq!(controls.len(), 32);
    assert!(controls.iter().all(|control| control.candidate == control.direct && control.tamper_rejected));
    assert_eq!(shuffled_scalars(11, 116001), shuffled_scalars(11, 116001));
    assert_ne!(shuffled_scalars(11, 116001), (1..11).collect::<Vec<_>>());
    let spec = CurveSpec { slug: "h115-control-fp31", p: 31, a: 1, b: 19, order: 37, trace: -5, degree: 4 };
    let modulus = first_irreducible(spec.p, spec.degree)?;
    let field = Field::new(spec.p, spec.degree, &modulus)?;
    let curve = Curve { field: &field, a: field.from_base(spec.a), b: field.from_base(spec.b) };
    let setup = direct_setup(&field, &curve, spec)?;
    let reference = setup.base_generator;
    let target = curve.scalar_mul(2, reference)?;
    for lambda in [1_u64, 2] {
        let (model_a, model_b, tx, ty, rx) = mapped_base_inputs(spec, target, reference, &field, lambda)?;
        let jet = Jet::new(spec.p, model_a, model_b, spec.trace)?;
        if lambda == 1 {
            assert_eq!(jet.h, kernel_velu_h(&curve, setup.kernel_generator, spec.order)?);
        }
        let (candidate, _) = jet.online(tx, ty, rx)?;
        let target_direct = absolute_direct_moments(&curve, section(&curve, spec, target)?, setup.kernel_generator, spec.order, lambda)?;
        let reference_direct = absolute_direct_moments(&curve, section(&curve, spec, reference)?, setup.kernel_generator, spec.order, lambda)?;
        assert_eq!(candidate, direct_relative(target_direct, reference_direct, spec.p));
    }
    println!("h116 producer selftest: pass");
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => selftest(),
        [_, flag, target_path, label_path] if flag == "--prepare-targets" => {
            prepare_targets(Path::new(target_path), Path::new(label_path))
        }
        [_, flag, target_path, output_path] if flag == "--run" => {
            run(Path::new(target_path), Path::new(output_path))
        }
        _ => Err(concat!("usage: h116_producer --selftest | ",
            "--prepare-targets TARGETS.tsv LABELS.tsv | --run TARGETS.tsv OUTPUT.jsonl").to_string()),
    };
    if let Err(error) = result { eprintln!("h116 producer error: {error}"); std::process::exit(1); }
}
