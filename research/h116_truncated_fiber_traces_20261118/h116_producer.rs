mod h116_field;

use h116_field::{
    first_irreducible, mod_inverse, torsion_table, trace_group_order, CountSnapshot, Curve, Felt,
    Field, Point,
};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;

const J: usize = 8;
const PROTOCOL_SHA256: &str = "42b510bd0f67d23ad49e5827d947510de5e1c68bf82d29c78fa35e4861fe4bf3";
const COHORT_SHA256: &str = "29e2fd5596ce3ed8e93933161e05b92229cd3065a892bc8ee59d6265f6fa2d99";

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
        fn inverse_and_log(jet: &Jet, u: u64, f: &mut BaseField) -> ([u64; J], [u64; J]) {
            let mut polynomial = [0_u64; J + 1];
            polynomial[0] = 1;
            polynomial[1] = f.neg(u);
            polynomial[2..=J].copy_from_slice(&jet.h[1..J]);
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

        let mut f = BaseField::new(self.p);
        let normalized_target_x = f.mul(self.differential_squared, target_x);
        let normalized_reference_x = f.mul(self.differential_squared, reference_x);
        let (inverse, target_log) = inverse_and_log(self, normalized_target_x, &mut f);
        let (_, reference_log) = inverse_and_log(self, normalized_reference_x, &mut f);
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

fn run_curve<W: Write>(writer: &mut W, spec: CurveSpec) -> Result<(u64, u64), String> {
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
        for &lambda in &[1_u64, 2] {
            let (model_a, model_b, _, _, reference_x) = mapped_base_inputs(spec, generator, generator, &field, lambda)?;
            let jet = Jet::new(spec.p, model_a, model_b, spec.trace)?;
            let reference_section = section(&curve, spec, generator)?;
            field.reset_counters();
            let reference_direct = absolute_direct_moments(&curve, reference_section, setup.kernel_generator, spec.order, lambda)?;
            for scalar in 1..spec.order {
                let target = curve.scalar_mul(scalar, generator)?;
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
                if direct != aa { return Err("direct A/A mismatch".to_string()); }
                let matches = candidate == direct;
                rows += 1;
                if !matches { mismatches += 1; }
                writeln!(writer, concat!("{{\"type\":\"row\",\"curve\":\"{}\",\"generator_label\":{},",
                    "\"target_index\":{},\"lambda\":{},\"candidate\":{},\"direct\":{},",
                    "\"match\":{},\"jet_setup_ops\":{},\"candidate_online_ops\":{},",
                    "\"direct_ops\":{},\"candidate_dev_ns\":{},\"direct_dev_ns\":{}}}"),
                    spec.slug, label, scalar, lambda, moments_json(candidate), moments_json(direct),
                    matches, base_ops_json(jet.setup_ops), base_ops_json(online_ops),
                    extension_ops_json(direct_ops), candidate_ns, direct_ns).map_err(|e| e.to_string())?;
                if !matches { return Ok((rows, mismatches)); }
            }
        }
    }
    Ok((rows, mismatches))
}

fn run(path: &Path) -> Result<(), String> {
    let file = File::create(path).map_err(|e| format!("create {}: {e}", path.display()))?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{{\"type\":\"header\",\"schema\":\"h116.component/v1\",\"protocol_sha256\":\"{PROTOCOL_SHA256}\",\"cohort_sha256\":\"{COHORT_SHA256}\"}}")
        .map_err(|e| e.to_string())?;
    let mut rows = 0;
    let mut mismatches = 0;
    for spec in CURVES {
        let (curve_rows, curve_mismatches) = run_curve(&mut writer, spec)?;
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
    assert!(Jet::new(13, 2, 4, -3).is_err());
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
        [_, flag, path] if flag == "--run" => run(Path::new(path)),
        _ => Err("usage: h116_producer --selftest | --run OUTPUT.jsonl".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 producer error: {error}"); std::process::exit(1); }
}
