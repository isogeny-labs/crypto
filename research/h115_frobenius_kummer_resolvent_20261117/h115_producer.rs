mod h115_field;
mod h115_sha256;

use h115_field::{
    first_irreducible, mod_inverse, torsion_table, trace_group_order, CountSnapshot, Curve, Felt,
    Field, Point,
};
use h115_sha256::{digest, hex, Sha256};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

const CURVES_SHA256: &str = "babd2556951ecfcaee66a5c6966364ca136bf1f62fb0a51511f391768255d295";
const PROTOCOL_SHA256: &str = "c8412d94f3e6cc928c13c93572a8796cea3a983b5fcd4e6b043a75bc5b1a2363";
const PREDICATE_COUNT: usize = 648;
const CHANNEL_COUNT: usize = 54;
const MOMENT_COUNT: usize = 8;

#[derive(Clone, Copy, Debug)]
struct CurveSpec {
    slug: &'static str,
    p: u64,
    a: u64,
    b: u64,
    order: u64,
    trace: i64,
    degree: usize,
    extension_size: u64,
}

const CURVES: [CurveSpec; 5] = [
    CurveSpec {
        slug: "h115-fp7-a1-b1-r5-d4",
        p: 7,
        a: 1,
        b: 1,
        order: 5,
        trace: 3,
        degree: 4,
        extension_size: 2401,
    },
    CurveSpec {
        slug: "h115-fp5-a2-b1-r7-d6",
        p: 5,
        a: 2,
        b: 1,
        order: 7,
        trace: -1,
        degree: 6,
        extension_size: 15625,
    },
    CurveSpec {
        slug: "h115-fp13-a2-b4-r17-d4",
        p: 13,
        a: 2,
        b: 4,
        order: 17,
        trace: -3,
        degree: 4,
        extension_size: 28561,
    },
    CurveSpec {
        slug: "h115-fp31-a1-b19-r37-d4",
        p: 31,
        a: 1,
        b: 19,
        order: 37,
        trace: -5,
        degree: 4,
        extension_size: 923521,
    },
    CurveSpec {
        slug: "h115-fp47-a1-b38-r61-d3",
        p: 47,
        a: 1,
        b: 38,
        order: 61,
        trace: -13,
        degree: 3,
        extension_size: 103823,
    },
];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Moments {
    x: [u64; MOMENT_COUNT],
    y: [u64; MOMENT_COUNT],
}

#[derive(Clone, Debug)]
struct Setup {
    modulus: Vec<u64>,
    group_order: u64,
    base_generator: Point,
    transverse: Point,
    kernel_generator: Point,
    torsion_count: usize,
    kernel_count: usize,
    cofactor: u64,
    enumerated_extension_states: u64,
    counters: CountSnapshot,
}

fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

fn point_json(field: &Field, point: Point) -> String {
    match point {
        Point::Infinity => "null".to_string(),
        Point::Affine(x, y) => format!("[{},{}]", field.to_index(x), field.to_index(y)),
    }
}

fn u64_array_json(values: &[u64]) -> String {
    let mut out = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        out.push_str(&value.to_string());
    }
    out.push(']');
    out
}

fn bool_array_json(values: &[bool]) -> String {
    let mut out = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            out.push(',');
        }
        out.push_str(if *value { "true" } else { "false" });
    }
    out.push(']');
    out
}

fn moments_json(moment: Moments) -> String {
    format!(
        "{{\"x\":{},\"y\":{}}}",
        u64_array_json(&moment.x),
        u64_array_json(&moment.y),
    )
}

fn counters_json(counters: CountSnapshot) -> String {
    format!(
        concat!(
            "{{\"base_additions\":{},\"base_multiplications\":{},\"base_inversions\":{},",
            "\"extension_additions\":{},\"extension_multiplications\":{},",
            "\"extension_inversions\":{},\"frobenius_maps\":{},",
            "\"point_additions\":{},\"scalar_multiplications\":{}}}"
        ),
        counters.base_additions,
        counters.base_multiplications,
        counters.base_inversions,
        counters.extension_additions,
        counters.extension_multiplications,
        counters.extension_inversions,
        counters.frobenius_maps,
        counters.point_additions,
        counters.scalar_multiplications,
    )
}

fn verify_input_hash(path: &Path, expected: &str) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let actual = hex(&digest(&bytes));
    if actual != expected {
        return Err(format!(
            "input digest mismatch for {}: {actual} != {expected}",
            path.display()
        ));
    }
    Ok(())
}

fn point_mul_signed(curve: &Curve<'_>, scalar: i64, point: Point) -> Result<Point, String> {
    if scalar >= 0 {
        curve.scalar_mul(scalar.unsigned_abs(), point)
    } else {
        Ok(curve.neg(curve.scalar_mul(scalar.unsigned_abs(), point)?))
    }
}

fn find_base_generator(curve: &Curve<'_>, p: u64) -> Result<Point, String> {
    for x in 0..p {
        for y in 0..p {
            let point = Point::Affine(curve.field.from_base(x), curve.field.from_base(y));
            if curve.is_on_curve(point) {
                return Ok(point);
            }
        }
    }
    Err("base generator not found".to_string())
}

fn canonical_sqrt(field: &Field, value: Felt) -> Result<Option<Felt>, String> {
    let Some(root) = field.sqrt(value)? else {
        return Ok(None);
    };
    let negative = field.neg(root);
    Ok(Some(if field.to_index(root) <= field.to_index(negative) {
        root
    } else {
        negative
    }))
}

fn find_transverse(
    curve: &Curve<'_>,
    spec: CurveSpec,
    group_order: u64,
) -> Result<(Point, Point, u64, u64), String> {
    let r_squared = spec
        .order
        .checked_mul(spec.order)
        .ok_or_else(|| "r^2 overflow".to_string())?;
    if group_order % r_squared != 0 {
        return Err(format!(
            "r^2 does not divide extension order for {}",
            spec.slug
        ));
    }
    let cofactor = group_order / r_squared;
    if gcd(cofactor, spec.order) != 1 {
        return Err(format!("r divides torsion cofactor for {}", spec.slug));
    }
    for x_index in 0..curve.field.q {
        let x = curve.field.from_index(x_index);
        let rhs = curve.field.add(
            curve.field.add(
                curve.field.mul(curve.field.square(x), x),
                curve.field.mul(curve.a, x),
            ),
            curve.b,
        );
        let Some(y) = canonical_sqrt(curve.field, rhs)? else {
            continue;
        };
        let point = Point::Affine(x, y);
        let projected = curve.scalar_mul(cofactor, point)?;
        if projected == Point::Infinity {
            continue;
        }
        if curve.scalar_mul(spec.order, projected)? != Point::Infinity {
            continue;
        }
        let kernel = curve.sub(curve.frobenius(projected), projected)?;
        if kernel == Point::Infinity {
            continue;
        }
        if curve.scalar_mul(spec.order, kernel)? != Point::Infinity {
            return Err("kernel generator not killed by r".to_string());
        }
        return Ok((projected, kernel, cofactor, x_index + 1));
    }
    Err(format!(
        "transverse torsion point not found for {}",
        spec.slug
    ))
}

fn verschiebung(curve: &Curve<'_>, spec: CurveSpec, point: Point) -> Result<Point, String> {
    curve.scalar_mul(
        spec.p,
        curve.frobenius_power(point, spec.degree.saturating_sub(1)),
    )
}

fn setup_curve(field: &Field, curve: &Curve<'_>, spec: CurveSpec) -> Result<Setup, String> {
    if field.q != spec.extension_size {
        return Err(format!("extension-size mismatch for {}", spec.slug));
    }
    let group_order = trace_group_order(spec.p, spec.trace, spec.degree)?;
    let base_generator = find_base_generator(curve, spec.p)?;
    if curve.scalar_mul(spec.order, base_generator)? != Point::Infinity {
        return Err("base generator order mismatch".to_string());
    }
    let (transverse, kernel_generator, cofactor, enumerated_extension_states) =
        find_transverse(curve, spec, group_order)?;
    let torsion = torsion_table(curve, base_generator, transverse, spec.order)?;
    let expected_torsion = usize::try_from(spec.order * spec.order)
        .map_err(|_| "torsion cardinality conversion".to_string())?;
    if torsion.len() != expected_torsion {
        return Err("torsion table cardinality mismatch".to_string());
    }
    let mut kernel_line = HashSet::new();
    for scalar in 0..spec.order {
        kernel_line.insert(curve.scalar_mul(scalar, kernel_generator)?);
    }
    if kernel_line.len() != usize::try_from(spec.order).expect("small order") {
        return Err("kernel line cardinality mismatch".to_string());
    }
    let frobenius_kernel = curve.frobenius(kernel_generator);
    if frobenius_kernel != curve.scalar_mul(spec.p % spec.order, kernel_generator)? {
        return Err("transverse Frobenius eigenvalue mismatch".to_string());
    }
    let mut checked_kernel = HashSet::new();
    for &point in &torsion {
        let pi = curve.frobenius(point);
        let pi2 = curve.frobenius(pi);
        let characteristic = curve.add(
            curve.sub(pi2, point_mul_signed(curve, spec.trace, pi)?)?,
            curve.scalar_mul(spec.p, point)?,
        )?;
        if characteristic != Point::Infinity {
            return Err("Frobenius characteristic equation failed".to_string());
        }
        let cocycle = curve.sub(pi, point)?;
        let factor = curve.sub(verschiebung(curve, spec, cocycle)?, cocycle)?;
        if factor != curve.scalar_mul(spec.order, point)? {
            return Err("(V-1)(pi-1) factorization failed".to_string());
        }
        if curve.sub(verschiebung(curve, spec, point)?, point)? == Point::Infinity {
            checked_kernel.insert(point);
        }
    }
    if checked_kernel != kernel_line {
        return Err("image/kernel line mismatch".to_string());
    }
    Ok(Setup {
        modulus: field.modulus[..=field.d].to_vec(),
        group_order,
        base_generator,
        transverse,
        kernel_generator,
        torsion_count: torsion.len(),
        kernel_count: checked_kernel.len(),
        cofactor,
        enumerated_extension_states,
        counters: field.snapshot(),
    })
}

fn raw_moment_sum(
    curve: &Curve<'_>,
    shift: Point,
    line_generator: Point,
    order: u64,
    omit_infinity: bool,
) -> Result<([Felt; MOMENT_COUNT], [Felt; MOMENT_COUNT], u64), String> {
    let mut x_sum = [curve.field.zero(); MOMENT_COUNT];
    let mut y_sum = [curve.field.zero(); MOMENT_COUNT];
    let mut terms = 0_u64;
    for scalar in 0..order {
        let point = curve.add(shift, curve.scalar_mul(scalar, line_generator)?)?;
        let Point::Affine(x, y) = point else {
            if omit_infinity {
                continue;
            }
            return Err("infinity in an affine moment fiber".to_string());
        };
        terms += 1;
        let mut x_power = curve.field.one();
        for index in 0..MOMENT_COUNT {
            y_sum[index] = curve.field.add(y_sum[index], curve.field.mul(y, x_power));
            x_power = curve.field.mul(x_power, x);
            x_sum[index] = curve.field.add(x_sum[index], x_power);
        }
    }
    Ok((x_sum, y_sum, terms))
}

fn descend_value(field: &Field, value: Felt) -> Result<Felt, String> {
    let mut trace = field.zero();
    let mut conjugate = value;
    for _ in 0..field.d {
        trace = field.add(trace, conjugate);
        conjugate = field.frobenius(conjugate);
    }
    let degree_mod_p = u64::try_from(field.d).expect("degree fits u64") % field.p;
    let inverse_degree = mod_inverse(degree_mod_p, field.p)?;
    Ok(field.mul(trace, field.from_base(inverse_degree)))
}

fn moments_for_line(
    curve: &Curve<'_>,
    shift: Point,
    line_generator: Point,
    order: u64,
    descend: bool,
    omit_infinity: bool,
) -> Result<(Moments, u64), String> {
    let (mut x_sum, mut y_sum, terms) =
        raw_moment_sum(curve, shift, line_generator, order, omit_infinity)?;
    if descend {
        for index in 0..MOMENT_COUNT {
            x_sum[index] = descend_value(curve.field, x_sum[index])?;
            y_sum[index] = descend_value(curve.field, y_sum[index])?;
        }
    }
    let mut result = Moments::default();
    for index in 0..MOMENT_COUNT {
        result.x[index] = curve.field.to_base(x_sum[index])?;
        result.y[index] = curve.field.to_base(y_sum[index])?;
    }
    Ok((result, terms))
}

fn base_ratio(numerator: u64, denominator: u64, p: u64) -> Result<u64, String> {
    if denominator == 0 {
        Ok(0)
    } else {
        Ok((numerator * mod_inverse(denominator, p)?) % p)
    }
}

fn channels(target: Moments, generator: Moments, p: u64) -> Result<[u64; CHANNEL_COUNT], String> {
    let mut result = [0_u64; CHANNEL_COUNT];
    let mut cursor = 0_usize;
    for values in [target.x, target.y] {
        for value in values {
            result[cursor] = value;
            cursor += 1;
        }
    }
    for (target_values, generator_values) in [(target.x, generator.x), (target.y, generator.y)] {
        for index in 0..MOMENT_COUNT {
            result[cursor] = base_ratio(target_values[index], generator_values[index], p)?;
            cursor += 1;
        }
    }
    for (target_values, generator_values) in [(target.x, generator.x), (target.y, generator.y)] {
        for index in 0..(MOMENT_COUNT - 1) {
            let numerator = target_values[index] * generator_values[index + 1] % p;
            let denominator = target_values[index + 1] * generator_values[index] % p;
            result[cursor] = base_ratio(numerator, denominator, p)?;
            cursor += 1;
        }
    }
    for index in 0..MOMENT_COUNT {
        let numerator = target.y[index] * generator.x[index] % p;
        let denominator = target.x[index] * generator.y[index] % p;
        result[cursor] = base_ratio(numerator, denominator, p)?;
        cursor += 1;
    }
    if cursor != CHANNEL_COUNT {
        return Err(format!("channel count {cursor} != {CHANNEL_COUNT}"));
    }
    Ok(result)
}

#[derive(Clone, Copy, Debug, Default)]
struct ChannelCost {
    base_multiplications: u64,
    base_inversions: u64,
}

fn channel_cost(target: Moments, generator: Moments) -> ChannelCost {
    let mut result = ChannelCost::default();
    for denominator in generator.x.into_iter().chain(generator.y) {
        if denominator != 0 {
            result.base_inversions += 1;
            result.base_multiplications += 1;
        }
    }
    for (target_values, generator_values) in [(target.x, generator.x), (target.y, generator.y)] {
        for index in 0..(MOMENT_COUNT - 1) {
            result.base_multiplications += 2;
            if target_values[index + 1] != 0 && generator_values[index] != 0 {
                result.base_inversions += 1;
                result.base_multiplications += 1;
            }
        }
    }
    for index in 0..MOMENT_COUNT {
        result.base_multiplications += 2;
        if target.x[index] != 0 && generator.y[index] != 0 {
            result.base_inversions += 1;
            result.base_multiplications += 1;
        }
    }
    result
}

const PERMUTATIONS_THREE: [[u64; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

fn predicate_components(id: usize) -> Result<(usize, usize, usize), String> {
    if id >= PREDICATE_COUNT {
        return Err(format!("predicate ID out of range: {id}"));
    }
    let channel = id / 12;
    let remainder = id % 12;
    Ok((channel, remainder / 6, remainder % 6))
}

fn predicate(id: usize, values: &[u64; CHANNEL_COUNT]) -> Result<u64, String> {
    let (channel, permutation_two, permutation_three) = predicate_components(id)?;
    let value = values[channel];
    let residue_two = value % 2;
    let mapped_two = if permutation_two == 0 {
        residue_two
    } else {
        1 - residue_two
    };
    let residue_three = usize::try_from(value % 3).expect("small residue");
    let mapped_three = PERMUTATIONS_THREE[permutation_three][residue_three];
    Ok((3 * mapped_two + 4 * mapped_three) % 6)
}

fn predicate_name(id: usize) -> Result<String, String> {
    let (channel, permutation_two, permutation_three) = predicate_components(id)?;
    Ok(format!(
        "ch{channel:02}-p2{permutation_two}-p3{permutation_three}"
    ))
}

fn verify_predicate_family() -> Result<(), String> {
    let mut names = HashSet::new();
    for id in 0..PREDICATE_COUNT {
        let name = predicate_name(id)?;
        if !names.insert(name) {
            return Err("duplicate predicate name".to_string());
        }
    }
    if names.len() != PREDICATE_COUNT {
        return Err("predicate family cardinality mismatch".to_string());
    }
    Ok(())
}

fn structural_selftest() -> Result<(), String> {
    h115_sha256::selftest()?;
    h115_field::selftest()?;
    verify_predicate_family()?;
    for spec in CURVES {
        let modulus = first_irreducible(spec.p, spec.degree)?;
        let field = Field::new(spec.p, spec.degree, &modulus)?;
        let curve = Curve {
            field: &field,
            a: field.from_base(spec.a),
            b: field.from_base(spec.b),
        };
        let setup = setup_curve(&field, &curve, spec)?;
        let inverse = mod_inverse((spec.p + spec.order - 1) % spec.order, spec.order)?;
        let q = setup.base_generator;
        let section = curve.scalar_mul(inverse, q)?;
        let mut fiber = HashSet::new();
        for scalar in 0..spec.order {
            let point = curve.add(section, curve.scalar_mul(scalar, setup.kernel_generator)?)?;
            if !fiber.insert(point) {
                return Err("candidate fiber cardinality mismatch".to_string());
            }
            let image = curve.sub(verschiebung(&curve, spec, point)?, point)?;
            if image != q {
                return Err("candidate fiber failed (V-1)C=Q".to_string());
            }
        }
        let (moment, terms) = moments_for_line(
            &curve,
            section,
            setup.kernel_generator,
            spec.order,
            false,
            false,
        )?;
        if terms != spec.order {
            return Err("candidate moment term count mismatch".to_string());
        }
        let values = channels(moment, moment, spec.p)?;
        for id in 0..PREDICATE_COUNT {
            if predicate(id, &values)? >= 6 {
                return Err("predicate digit outside radix".to_string());
            }
        }
        println!(
            "{} modulus={} N={} cofactor={} states={} G={} T={} H={} torsion={} kernel={} moment_terms={}",
            spec.slug,
            u64_array_json(&setup.modulus),
            setup.group_order,
            setup.cofactor,
            setup.enumerated_extension_states,
            point_json(&field, setup.base_generator),
            point_json(&field, setup.transverse),
            point_json(&field, setup.kernel_generator),
            setup.torsion_count,
            setup.kernel_count,
            terms,
        );
    }
    Ok(())
}

struct RecordWriter {
    writer: BufWriter<File>,
    digest: Sha256,
    records: u64,
    bytes: u64,
}

impl RecordWriter {
    fn create(path: &Path) -> Result<Self, String> {
        let file =
            File::create(path).map_err(|error| format!("create {}: {error}", path.display()))?;
        Ok(Self {
            writer: BufWriter::new(file),
            digest: Sha256::new(),
            records: 0,
            bytes: 0,
        })
    }

    fn data_record(&mut self, record: &str) -> Result<(), String> {
        let mut line = record.as_bytes().to_vec();
        line.push(b'\n');
        self.writer
            .write_all(&line)
            .map_err(|error| format!("write raw record: {error}"))?;
        self.digest.update(&line);
        self.records += 1;
        self.bytes += u64::try_from(line.len()).expect("record length fits u64");
        Ok(())
    }

    fn summary_record(mut self, record_without_digest: &str) -> Result<String, String> {
        let raw_digest = hex(&self.digest.finalize());
        let record = format!(
            "{{\"type\":\"summary\",\"raw_sha256\":\"{raw_digest}\",{record_without_digest}}}"
        );
        self.writer
            .write_all(record.as_bytes())
            .and_then(|()| self.writer.write_all(b"\n"))
            .and_then(|()| self.writer.flush())
            .map_err(|error| format!("write summary: {error}"))?;
        Ok(raw_digest)
    }
}

#[derive(Clone)]
struct RowCache {
    spec: CurveSpec,
    multiplier: u64,
    channels_by_scalar: Vec<Option<[u64; CHANNEL_COUNT]>>,
}

#[derive(Clone, Copy, Debug)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
        value ^ (value >> 31)
    }
}

fn seeded_rng(domain: &str, spec: CurveSpec, multiplier: u64, replicate: u64) -> SplitMix64 {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(domain.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(spec.slug.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(multiplier.to_string().as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(replicate.to_string().as_bytes());
    let hash = digest(&bytes);
    SplitMix64::new(u64::from_be_bytes(
        hash[..8].try_into().expect("eight-byte seed"),
    ))
}

fn shuffled_labels(spec: CurveSpec, multiplier: u64, replicate: u64) -> Vec<u64> {
    let mut labels: Vec<u64> = (1..spec.order).map(|scalar| scalar % 6).collect();
    let mut rng = seeded_rng("matched-label", spec, multiplier, replicate);
    for upper in (1..labels.len()).rev() {
        let index =
            usize::try_from(rng.next() % u64::try_from(upper + 1).expect("length fits u64"))
                .expect("shuffle index fits usize");
        labels.swap(upper, index);
    }
    labels
}

fn random_control_line(
    curve: &Curve<'_>,
    spec: CurveSpec,
    generator: Point,
    transverse: Point,
    multiplier: u64,
    replicate: u64,
    used: &mut HashSet<u64>,
) -> Result<(u64, Point), String> {
    let mut rng = seeded_rng("random-line", spec, multiplier, replicate);
    for _ in 0..(spec.order * 4) {
        let a = rng.next() % spec.order;
        let b = rng.next() % spec.order;
        if a == 0 || b == 0 {
            continue;
        }
        let normalized = a * mod_inverse(b, spec.order)? % spec.order;
        if !used.insert(normalized) {
            continue;
        }
        let line = curve.add(curve.scalar_mul(normalized, generator)?, transverse)?;
        let pi_line = curve.frobenius(line);
        let mut eigen = false;
        for scalar in 0..spec.order {
            if curve.scalar_mul(scalar, line)? == pi_line {
                eigen = true;
                break;
            }
        }
        if !eigen {
            return Ok((normalized, line));
        }
    }
    Err("failed to draw a distinct non-eigen control line".to_string())
}

fn coordinate_map(field: &Field, point: Point, lambda: u64) -> Point {
    match point {
        Point::Infinity => Point::Infinity,
        Point::Affine(x, y) => {
            let scale = field.from_base(lambda);
            let lambda2 = field.square(scale);
            let lambda3 = field.mul(lambda2, scale);
            Point::Affine(field.mul(lambda2, x), field.mul(lambda3, y))
        }
    }
}

fn row_score(
    values: &[Option<[u64; CHANNEL_COUNT]>],
    labels: &[u64],
) -> Result<[u64; PREDICATE_COUNT], String> {
    let mut score = [0_u64; PREDICATE_COUNT];
    if values.len() != labels.len() + 1 {
        return Err("row score length mismatch".to_string());
    }
    for (offset, &label) in labels.iter().enumerate() {
        let scalar = offset + 1;
        let channels = values[scalar]
            .as_ref()
            .ok_or_else(|| "missing row channel vector".to_string())?;
        for (id, slot) in score.iter_mut().enumerate() {
            if predicate(id, channels)? == label {
                *slot += 1;
            }
        }
    }
    Ok(score)
}

fn best_score(scores: &[u64; PREDICATE_COUNT]) -> (u64, Vec<usize>) {
    let best = scores.iter().copied().max().unwrap_or(0);
    let ids = scores
        .iter()
        .enumerate()
        .filter_map(|(id, &score)| (score == best).then_some(id))
        .collect();
    (best, ids)
}

fn ids_json(ids: &[usize]) -> String {
    let values: Vec<u64> = ids
        .iter()
        .map(|&id| u64::try_from(id).expect("predicate ID fits u64"))
        .collect();
    u64_array_json(&values)
}

fn verify_fiber(
    curve: &Curve<'_>,
    spec: CurveSpec,
    target: Point,
    section: Point,
    line: Point,
) -> Result<(), String> {
    let mut points = HashSet::new();
    for scalar in 0..spec.order {
        let point = curve.add(section, curve.scalar_mul(scalar, line)?)?;
        if !points.insert(point) {
            return Err("fiber contains a duplicate".to_string());
        }
        if curve.sub(verschiebung(curve, spec, point)?, point)? != target {
            return Err("fiber point fails (V-1)C=Q".to_string());
        }
    }
    if points.len() != usize::try_from(spec.order).expect("small order") {
        return Err("fiber cardinality mismatch".to_string());
    }
    Ok(())
}

fn emit_moment_record(
    writer: &mut RecordWriter,
    class: &str,
    spec: CurveSpec,
    multiplier: u64,
    scalar: u64,
    replicate: Option<u64>,
    moment: Moments,
    channel_values: &[u64; CHANNEL_COUNT],
    label: u64,
    terms: u64,
) -> Result<(), String> {
    let replicate_json = replicate.map_or_else(|| "null".to_string(), |value| value.to_string());
    writer.data_record(&format!(
        concat!(
            "{{\"type\":\"moment\",\"class\":\"{}\",\"curve\":\"{}\",",
            "\"multiplier\":{},\"scalar\":{},\"replicate\":{},",
            "\"moment\":{},\"channels\":{},\"label\":{},\"terms\":{}}}"
        ),
        class,
        spec.slug,
        multiplier,
        scalar,
        replicate_json,
        moments_json(moment),
        u64_array_json(channel_values),
        label,
        terms,
    ))
}

fn construct_curve(spec: CurveSpec) -> Result<(Field, Vec<u64>), String> {
    let modulus = first_irreducible(spec.p, spec.degree)?;
    let field = Field::new(spec.p, spec.degree, &modulus)?;
    Ok((field, modulus))
}

fn run_experiment(output: &Path) -> Result<(), String> {
    let started = Instant::now();
    verify_predicate_family()?;
    let mut writer = RecordWriter::create(output)?;
    writer.data_record(&format!(
        concat!(
            "{{\"type\":\"meta\",\"schema\":\"h115.raw/v1\",",
            "\"curves_sha256\":\"{}\",\"protocol_sha256\":\"{}\",",
            "\"predicate_count\":{},\"channel_count\":{},\"radix\":6}}"
        ),
        CURVES_SHA256, PROTOCOL_SHA256, PREDICATE_COUNT, CHANNEL_COUNT,
    ))?;

    let mut global_exact = [true; PREDICATE_COUNT];
    let mut global_scores = [0_u64; PREDICATE_COUNT];
    let mut rows = Vec::<RowCache>::new();
    let mut target_count = 0_u64;
    let mut random_control_records = 0_u64;
    let mut rational_control_records = 0_u64;
    let mut coordinate_records = 0_u64;
    let mut moment_terms = 0_u64;
    let mut predicate_evaluations = 0_u64;
    let mut channel_base_multiplications = 0_u64;
    let mut channel_base_inversions = 0_u64;

    for (curve_index, spec) in CURVES.into_iter().enumerate() {
        let (field, modulus) = construct_curve(spec)?;
        let curve = Curve {
            field: &field,
            a: field.from_base(spec.a),
            b: field.from_base(spec.b),
        };
        let setup = setup_curve(&field, &curve, spec)?;
        if setup.modulus != modulus {
            return Err("setup modulus handoff mismatch".to_string());
        }
        writer.data_record(&format!(
            concat!(
                "{{\"type\":\"curve\",\"curve\":\"{}\",\"modulus\":{},",
                "\"extension_order\":{},\"base_generator\":{},\"transverse\":{},",
                "\"kernel_generator\":{},\"torsion_count\":{},\"kernel_count\":{},",
                "\"cofactor\":{},\"enumerated_extension_states\":{},",
                "\"setup_counters\":{}}}"
            ),
            spec.slug,
            u64_array_json(&modulus),
            setup.group_order,
            point_json(&field, setup.base_generator),
            point_json(&field, setup.transverse),
            point_json(&field, setup.kernel_generator),
            setup.torsion_count,
            setup.kernel_count,
            setup.cofactor,
            setup.enumerated_extension_states,
            counters_json(setup.counters),
        ))?;
        let inverse_p_minus_one = mod_inverse((spec.p + spec.order - 1) % spec.order, spec.order)?;
        for multiplier in [1_u64, 2, 3] {
            field.reset_counters();
            let generator = curve.scalar_mul(multiplier, setup.base_generator)?;
            let row_len = usize::try_from(spec.order).expect("small row order");
            let mut moments = vec![None; row_len];
            let mut row_terms = vec![0_u64; row_len];
            for scalar in 1..spec.order {
                let target = curve.scalar_mul(scalar, generator)?;
                let section = curve.scalar_mul(inverse_p_minus_one, target)?;
                verify_fiber(&curve, spec, target, section, setup.kernel_generator)?;
                let (moment, terms) = moments_for_line(
                    &curve,
                    section,
                    setup.kernel_generator,
                    spec.order,
                    false,
                    false,
                )?;
                moments[usize::try_from(scalar).expect("small scalar")] = Some(moment);
                row_terms[usize::try_from(scalar).expect("small scalar")] = terms;
                moment_terms += terms;
            }
            let generator_moment =
                moments[1].ok_or_else(|| "missing generator moment".to_string())?;
            let mut channels_by_scalar = vec![None; row_len];
            for scalar in 1..spec.order {
                let index = usize::try_from(scalar).expect("small scalar");
                let moment = moments[index].ok_or_else(|| "missing target moment".to_string())?;
                let channel_values = channels(moment, generator_moment, spec.p)?;
                let cost = channel_cost(moment, generator_moment);
                channel_base_multiplications += cost.base_multiplications;
                channel_base_inversions += cost.base_inversions;
                emit_moment_record(
                    &mut writer,
                    "candidate",
                    spec,
                    multiplier,
                    scalar,
                    None,
                    moment,
                    &channel_values,
                    scalar % 6,
                    row_terms[index],
                )?;
                channels_by_scalar[index] = Some(channel_values);
            }
            let labels: Vec<u64> = (1..spec.order).map(|scalar| scalar % 6).collect();
            let scores = row_score(&channels_by_scalar, &labels)?;
            predicate_evaluations +=
                (spec.order - 1) * u64::try_from(PREDICATE_COUNT).expect("count");
            let expected = spec.order - 1;
            let exact: Vec<bool> = scores.iter().map(|&score| score == expected).collect();
            for id in 0..PREDICATE_COUNT {
                global_exact[id] &= exact[id];
                global_scores[id] += scores[id];
            }
            let (row_best, row_best_ids) = best_score(&scores);
            writer.data_record(&format!(
                concat!(
                    "{{\"type\":\"row_summary\",\"curve\":\"{}\",\"multiplier\":{},",
                    "\"targets\":{},\"best_score\":{},\"best_ids\":{},",
                    "\"exact\":{},\"counters\":{}}}"
                ),
                spec.slug,
                multiplier,
                expected,
                row_best,
                ids_json(&row_best_ids),
                bool_array_json(&exact),
                counters_json(field.snapshot()),
            ))?;
            target_count += expected;

            if curve_index < 2 {
                let mut used_lines = HashSet::new();
                for replicate in 0..8_u64 {
                    let (line_id, line) = random_control_line(
                        &curve,
                        spec,
                        generator,
                        setup.transverse,
                        multiplier,
                        replicate,
                        &mut used_lines,
                    )?;
                    let mut control_moments = vec![None; row_len];
                    let mut control_terms = vec![0_u64; row_len];
                    for scalar in 1..spec.order {
                        let target = curve.scalar_mul(scalar, generator)?;
                        let section = curve.scalar_mul(inverse_p_minus_one, target)?;
                        let (moment, terms) =
                            moments_for_line(&curve, section, line, spec.order, true, false)?;
                        let index = usize::try_from(scalar).expect("small scalar");
                        control_moments[index] = Some(moment);
                        control_terms[index] = terms;
                        moment_terms += terms;
                    }
                    let control_generator = control_moments[1]
                        .ok_or_else(|| "missing random-control generator moment".to_string())?;
                    let mut control_channels = vec![None; row_len];
                    for scalar in 1..spec.order {
                        let index = usize::try_from(scalar).expect("small scalar");
                        let moment = control_moments[index]
                            .ok_or_else(|| "missing random-control moment".to_string())?;
                        let values = channels(moment, control_generator, spec.p)?;
                        let cost = channel_cost(moment, control_generator);
                        channel_base_multiplications += cost.base_multiplications;
                        channel_base_inversions += cost.base_inversions;
                        emit_moment_record(
                            &mut writer,
                            "random-line",
                            spec,
                            multiplier,
                            scalar,
                            Some(replicate),
                            moment,
                            &values,
                            scalar % 6,
                            control_terms[index],
                        )?;
                        control_channels[index] = Some(values);
                        random_control_records += 1;
                    }
                    let control_scores = row_score(&control_channels, &labels)?;
                    predicate_evaluations +=
                        expected * u64::try_from(PREDICATE_COUNT).expect("count");
                    let (control_best, control_ids) = best_score(&control_scores);
                    writer.data_record(&format!(
                        concat!(
                            "{{\"type\":\"control_summary\",\"class\":\"random-line\",",
                            "\"curve\":\"{}\",\"multiplier\":{},\"replicate\":{},",
                            "\"line_id\":{},\"best_score\":{},\"best_ids\":{}}}"
                        ),
                        spec.slug,
                        multiplier,
                        replicate,
                        line_id,
                        control_best,
                        ids_json(&control_ids),
                    ))?;
                }
                for replicate in 0..8_u64 {
                    let shuffled = shuffled_labels(spec, multiplier, replicate);
                    let shuffled_scores = row_score(&channels_by_scalar, &shuffled)?;
                    predicate_evaluations +=
                        expected * u64::try_from(PREDICATE_COUNT).expect("count");
                    let (control_best, control_ids) = best_score(&shuffled_scores);
                    writer.data_record(&format!(
                        concat!(
                            "{{\"type\":\"label_control\",\"curve\":\"{}\",",
                            "\"multiplier\":{},\"replicate\":{},\"labels\":{},",
                            "\"best_score\":{},\"best_ids\":{}}}"
                        ),
                        spec.slug,
                        multiplier,
                        replicate,
                        u64_array_json(&shuffled),
                        control_best,
                        ids_json(&control_ids),
                    ))?;
                }
            }

            let rational_shift = Point::Infinity;
            let (rational_moment, rational_terms) =
                moments_for_line(&curve, rational_shift, generator, spec.order, true, true)?;
            if rational_terms != spec.order - 1 {
                return Err("rational control affine-term count mismatch".to_string());
            }
            let rational_channels = channels(rational_moment, rational_moment, spec.p)?;
            let rational_cost = channel_cost(rational_moment, rational_moment);
            for scalar in 1..spec.order {
                channel_base_multiplications += rational_cost.base_multiplications;
                channel_base_inversions += rational_cost.base_inversions;
                emit_moment_record(
                    &mut writer,
                    "rational-line",
                    spec,
                    multiplier,
                    scalar,
                    None,
                    rational_moment,
                    &rational_channels,
                    scalar % 6,
                    rational_terms,
                )?;
                rational_control_records += 1;
                moment_terms += rational_terms;
            }

            if multiplier == 1 {
                let lambda = field.from_base(2);
                let lambda2 = field.square(lambda);
                let lambda4 = field.square(lambda2);
                let lambda6 = field.mul(lambda4, lambda2);
                let iso_curve = Curve {
                    field: &field,
                    a: field.mul(lambda4, curve.a),
                    b: field.mul(lambda6, curve.b),
                };
                let iso_generator = coordinate_map(&field, generator, 2);
                let iso_kernel = coordinate_map(&field, setup.kernel_generator, 2);
                let mut iso_moments = vec![None; row_len];
                let mut iso_terms = vec![0_u64; row_len];
                for scalar in 1..spec.order {
                    let target = iso_curve.scalar_mul(scalar, iso_generator)?;
                    let section = iso_curve.scalar_mul(inverse_p_minus_one, target)?;
                    let (moment, terms) = moments_for_line(
                        &iso_curve, section, iso_kernel, spec.order, false, false,
                    )?;
                    let index = usize::try_from(scalar).expect("small scalar");
                    iso_moments[index] = Some(moment);
                    iso_terms[index] = terms;
                    moment_terms += terms;
                }
                let iso_generator_moment = iso_moments[1]
                    .ok_or_else(|| "missing isomorphic generator moment".to_string())?;
                for scalar in 1..spec.order {
                    let index = usize::try_from(scalar).expect("small scalar");
                    let moment = iso_moments[index]
                        .ok_or_else(|| "missing isomorphic target moment".to_string())?;
                    let iso_channels = channels(moment, iso_generator_moment, spec.p)?;
                    let cost = channel_cost(moment, iso_generator_moment);
                    channel_base_multiplications += cost.base_multiplications;
                    channel_base_inversions += cost.base_inversions;
                    let base_channels = channels_by_scalar[index]
                        .as_ref()
                        .ok_or_else(|| "missing base channels for isomorphism".to_string())?;
                    if iso_channels[16..] != base_channels[16..] {
                        return Err("normalized channel failed coordinate transport".to_string());
                    }
                    emit_moment_record(
                        &mut writer,
                        "coordinate-isomorphism",
                        spec,
                        multiplier,
                        scalar,
                        None,
                        moment,
                        &iso_channels,
                        scalar % 6,
                        iso_terms[index],
                    )?;
                    coordinate_records += 1;
                }
            }

            rows.push(RowCache {
                spec,
                multiplier,
                channels_by_scalar,
            });
        }
    }

    if rows.len() != 15 || target_count != 366 {
        return Err(format!(
            "cohort cardinality mismatch rows={} targets={target_count}",
            rows.len()
        ));
    }
    let exact_ids: Vec<usize> = global_exact
        .iter()
        .enumerate()
        .filter_map(|(id, &exact)| exact.then_some(id))
        .collect();
    let (global_best, global_best_ids) = best_score(&global_scores);
    let raw_best = global_scores[..(16 * 12)]
        .iter()
        .copied()
        .max()
        .unwrap_or(0);
    let normalized_best = global_scores[(16 * 12)..]
        .iter()
        .copied()
        .max()
        .unwrap_or(0);

    let mut decoder_queries = 0_u64;
    let mut decoder_targets = 0_u64;
    let mut forbidden_targets = 0_u64;
    for row in &rows {
        let (field, _) = construct_curve(row.spec)?;
        let curve = Curve {
            field: &field,
            a: field.from_base(row.spec.a),
            b: field.from_base(row.spec.b),
        };
        let setup = setup_curve(&field, &curve, row.spec)?;
        let generator = curve.scalar_mul(row.multiplier, setup.base_generator)?;
        let inverse_six = mod_inverse(6 % row.spec.order, row.spec.order)?;
        let mut by_point = HashMap::new();
        for scalar in 1..row.spec.order {
            let point = curve.scalar_mul(scalar, generator)?;
            let values = row.channels_by_scalar[usize::try_from(scalar).expect("small scalar")]
                .ok_or_else(|| "decoder channel table gap".to_string())?;
            by_point.insert(point, values);
        }
        for &id in &exact_ids {
            for scalar in 1..row.spec.order {
                let target = curve.scalar_mul(scalar, generator)?;
                let mut current = target;
                let mut reconstructed = 0_u64;
                let mut place = 1_u64;
                let mut row_queries = 0_u64;
                while current != Point::Infinity {
                    let values = by_point
                        .get(&current)
                        .ok_or_else(|| "decoder queried a noncohort point".to_string())?;
                    let digit = predicate(id, values)?;
                    let residual = curve.sub(current, curve.scalar_mul(digit, generator)?)?;
                    current = curve.scalar_mul(inverse_six, residual)?;
                    reconstructed = reconstructed
                        .checked_add(
                            digit
                                .checked_mul(place)
                                .ok_or_else(|| "decoder product overflow".to_string())?,
                        )
                        .ok_or_else(|| "decoder sum overflow".to_string())?;
                    place = place
                        .checked_mul(6)
                        .ok_or_else(|| "decoder place overflow".to_string())?;
                    row_queries += 1;
                    if row_queries > 64 {
                        return Err("decoder failed to terminate".to_string());
                    }
                }
                if reconstructed != scalar || curve.scalar_mul(reconstructed, generator)? != target
                {
                    return Err("independent decoder verification failed".to_string());
                }
                decoder_queries += row_queries;
                decoder_targets += 1;
            }
        }
        for scalar in 1..row.spec.order {
            let target = curve.scalar_mul(scalar, generator)?;
            let mut current = target;
            let mut remaining = scalar;
            let mut reconstructed = 0_u64;
            let mut place = 1_u64;
            while current != Point::Infinity {
                let digit = remaining % 6;
                let residual = curve.sub(current, curve.scalar_mul(digit, generator)?)?;
                current = curve.scalar_mul(inverse_six, residual)?;
                reconstructed += digit * place;
                place *= 6;
                remaining /= 6;
                if current != curve.scalar_mul(remaining, generator)? {
                    return Err("forbidden-oracle group update mismatch".to_string());
                }
            }
            if reconstructed != scalar
                || remaining != 0
                || curve.scalar_mul(reconstructed, generator)? != target
            {
                return Err("forbidden-oracle decoder failed".to_string());
            }
            forbidden_targets += 1;
        }
    }

    if forbidden_targets != 366 {
        return Err(format!(
            "forbidden-oracle target count {forbidden_targets} != 366"
        ));
    }

    let records = writer.records;
    let raw_bytes = writer.bytes;
    let elapsed_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    let summary_tail = format!(
        concat!(
            "\"records\":{},\"raw_bytes\":{},\"rows\":15,\"targets\":366,",
            "\"global_best_score\":{},\"global_best_ids\":{},\"exact_ids\":{},",
            "\"raw_only_best\":{},\"normalized_only_best\":{},",
            "\"random_control_records\":{},\"rational_control_records\":{},",
            "\"coordinate_records\":{},\"moment_terms\":{},",
            "\"channel_base_multiplications\":{},\"channel_base_inversions\":{},",
            "\"predicate_evaluations\":{},\"decoder_targets\":{},",
            "\"decoder_queries\":{},\"forbidden_positive\":true,",
            "\"forbidden_targets\":{},",
            "\"wall_ms\":{}"
        ),
        records,
        raw_bytes,
        global_best,
        ids_json(&global_best_ids),
        ids_json(&exact_ids),
        raw_best,
        normalized_best,
        random_control_records,
        rational_control_records,
        coordinate_records,
        moment_terms,
        channel_base_multiplications,
        channel_base_inversions,
        predicate_evaluations,
        decoder_targets,
        decoder_queries,
        forbidden_targets,
        elapsed_ms,
    );
    let raw_digest = writer.summary_record(&summary_tail)?;
    println!(
        concat!(
            "h115 run complete rows=15 targets=366 best={}/366 exact_ids={} ",
            "decoder_targets={} raw_sha256={} wall_ms={}"
        ),
        global_best,
        exact_ids.len(),
        decoder_targets,
        raw_digest,
        elapsed_ms,
    );
    Ok(())
}

fn usage(program: &str) {
    eprintln!("usage: {program} --selftest | --run OUTPUT_JSONL [--inputs DIR]");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => structural_selftest(),
        [_, flag, output] if flag == "--run" => {
            let input_dir = PathBuf::from(".");
            verify_input_hash(&input_dir.join("CURVES.json"), CURVES_SHA256)
                .and_then(|()| verify_input_hash(&input_dir.join("PROTOCOL.json"), PROTOCOL_SHA256))
                .and_then(|()| run_experiment(Path::new(output)))
        }
        [_, flag, output, inputs_flag, inputs] if flag == "--run" && inputs_flag == "--inputs" => {
            let input_dir = PathBuf::from(inputs);
            verify_input_hash(&input_dir.join("CURVES.json"), CURVES_SHA256)
                .and_then(|()| verify_input_hash(&input_dir.join("PROTOCOL.json"), PROTOCOL_SHA256))
                .and_then(|()| run_experiment(Path::new(output)))
        }
        _ => {
            usage(args.first().map(String::as_str).unwrap_or("h115_producer"));
            Err("invalid arguments".to_string())
        }
    };
    if let Err(error) = result {
        eprintln!("h115 producer error: {error}");
        std::process::exit(1);
    }
}
