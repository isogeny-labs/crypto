//! Native full-map oracle for the preregistered H116 formal-scalar OOS panel.
//!
//! This source deliberately does not evaluate the formal-scalar candidate. It
//! selects the five alternative models fixed by FORMAL_SCALAR_OOS_PROTOCOL,
//! constructs ker(V-1), emits the full reversed Velu map, and checks the
//! cleared curve identity plus the nine-coefficient recurrence control.

#[allow(dead_code)]
mod h116_field;

use h116_field::{first_irreducible, trace_group_order, CountSnapshot, Curve, Felt, Field, Point};
use std::collections::HashSet;

const JET_PRECISION: usize = 11;
const RECURRENCE_PRECISION: usize = 9;

#[derive(Clone, Copy)]
struct CurveSpec {
    slug: &'static str,
    p: u64,
    admitted_a: u64,
    admitted_b: u64,
    order: u64,
    trace: i64,
    degree: usize,
}

const PANEL: [CurveSpec; 5] = [
    CurveSpec {
        slug: "fp107-r127-d3",
        p: 107,
        admitted_a: 1,
        admitted_b: 25,
        order: 127,
        trace: -19,
        degree: 3,
    },
    CurveSpec {
        slug: "fp211-r197-d4",
        p: 211,
        admitted_a: 1,
        admitted_b: 148,
        order: 197,
        trace: 15,
        degree: 4,
    },
    CurveSpec {
        slug: "fp241-r257-d4",
        p: 241,
        admitted_a: 1,
        admitted_b: 57,
        order: 257,
        trace: -15,
        degree: 4,
    },
    CurveSpec {
        slug: "fp431-r397-d3",
        p: 431,
        admitted_a: 1,
        admitted_b: 111,
        order: 397,
        trace: 35,
        degree: 3,
    },
    CurveSpec {
        slug: "fp587-r547-d3",
        p: 587,
        admitted_a: 1,
        admitted_b: 204,
        order: 547,
        trace: 41,
        degree: 3,
    },
];

fn add_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}

fn sub_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + p as u128 - b as u128) % p as u128) as u64
}

fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn inverse_mod(value: u64, p: u64) -> u64 {
    let mut base = value % p;
    let mut exponent = p - 2;
    let mut out = 1;
    while exponent != 0 {
        if exponent & 1 == 1 {
            out = mul_mod(out, base, p);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = mul_mod(base, base, p);
        }
    }
    out
}

fn nonsingular(p: u64, a: u64, b: u64) -> bool {
    add_mod(
        mul_mod(4, mul_mod(mul_mod(a, a, p), a, p), p),
        mul_mod(27, mul_mod(b, b, p), p),
        p,
    ) != 0
}

fn j_invariant(p: u64, a: u64, b: u64) -> Result<u64, String> {
    let four_a3 = mul_mod(4, mul_mod(mul_mod(a, a, p), a, p), p);
    let denominator = add_mod(four_a3, mul_mod(27, mul_mod(b, b, p), p), p);
    if denominator == 0 {
        return Err("j-invariant requested for a singular model".to_string());
    }
    Ok(mul_mod(
        mul_mod(1728 % p, four_a3, p),
        inverse_mod(denominator, p),
        p,
    ))
}

fn point_count(p: u64, a: u64, b: u64) -> u64 {
    let mut square_counts = vec![0_u64; p as usize];
    for y in 0..p {
        square_counts[mul_mod(y, y, p) as usize] += 1;
    }
    let mut count = 1;
    for x in 0..p {
        let rhs = add_mod(
            add_mod(mul_mod(mul_mod(x, x, p), x, p), mul_mod(a, x, p), p),
            b,
            p,
        );
        count += square_counts[rhs as usize];
    }
    count
}

fn excluded_h115(p: u64, a: u64, b: u64) -> bool {
    matches!(
        (p, a, b),
        (7, 1, 1) | (5, 2, 1) | (13, 2, 4) | (31, 1, 19) | (47, 1, 38)
    )
}

fn select_alternative(spec: CurveSpec) -> Result<(u64, u64, u64), String> {
    let admitted_j = j_invariant(spec.p, spec.admitted_a, spec.admitted_b)?;
    let mut models_examined = 0_u64;
    for a in 0..spec.p {
        for b in 0..spec.p {
            models_examined += 1;
            if !nonsingular(spec.p, a, b)
                || excluded_h115(spec.p, a, b)
                || j_invariant(spec.p, a, b)? == admitted_j
            {
                continue;
            }
            if point_count(spec.p, a, b) == spec.order {
                return Ok((a, b, models_examined));
            }
        }
    }
    Err(format!("selection_failure for {}", spec.slug))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn first_base_point(curve: &Curve<'_>, p: u64) -> Result<Point, String> {
    for x in 0..p {
        for y in 0..p {
            let point = Point::Affine(curve.field.from_base(x), curve.field.from_base(y));
            if curve.is_on_curve(point) {
                return Ok(point);
            }
        }
    }
    Err("base point not found".to_string())
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

fn kernel_generator(
    field: &Field,
    curve: &Curve<'_>,
    spec: CurveSpec,
) -> Result<(Point, u64), String> {
    let group_order = trace_group_order(spec.p, spec.trace, spec.degree)?;
    let r2 = spec.order.checked_mul(spec.order).ok_or("r^2 overflow")?;
    if group_order % r2 != 0 {
        return Err(format!(
            "r^2 does not divide extension order for {}",
            spec.slug
        ));
    }
    let cofactor = group_order / r2;
    if gcd(cofactor, spec.order) != 1 {
        return Err(format!("r divides torsion cofactor for {}", spec.slug));
    }
    let base = first_base_point(curve, spec.p)?;
    if curve.scalar_mul(spec.order, base)? != Point::Infinity {
        return Err(format!(
            "base point does not have order r for {}",
            spec.slug
        ));
    }
    for x_index in 0..field.q {
        let x = field.from_index(x_index);
        let rhs = field.add(
            field.add(field.mul(field.square(x), x), field.mul(curve.a, x)),
            curve.b,
        );
        let Some(y) = canonical_sqrt(field, rhs)? else {
            continue;
        };
        let projected = curve.scalar_mul(cofactor, Point::Affine(x, y))?;
        if projected == Point::Infinity
            || curve.scalar_mul(spec.order, projected)? != Point::Infinity
        {
            continue;
        }
        let kernel = curve.sub(curve.frobenius(projected), projected)?;
        if kernel == Point::Infinity || curve.scalar_mul(spec.order, kernel)? != Point::Infinity {
            continue;
        }
        let inverse_frobenius = curve.frobenius_power(kernel, spec.degree - 1);
        let verschiebung = curve.scalar_mul(spec.p, inverse_frobenius)?;
        if curve.sub(verschiebung, kernel)? == Point::Infinity {
            return Ok((kernel, x_index + 1));
        }
    }
    Err(format!("kernel generator not found for {}", spec.slug))
}

type ExtPoly = Vec<Felt>;

fn ext_mul(field: &Field, left: &[Felt], right: &[Felt]) -> ExtPoly {
    let mut out = vec![field.zero(); left.len() + right.len() - 1];
    for (i, &a) in left.iter().enumerate() {
        for (j, &b) in right.iter().enumerate() {
            out[i + j] = field.add(out[i + j], field.mul(a, b));
        }
    }
    out
}

fn ext_add_scaled(field: &Field, target: &mut ExtPoly, source: &[Felt], scale: Felt) {
    if target.len() < source.len() {
        target.resize(source.len(), field.zero());
    }
    for (slot, &coefficient) in target.iter_mut().zip(source) {
        *slot = field.add(*slot, field.mul(scale, coefficient));
    }
}

fn ext_divide_x_minus(field: &Field, polynomial: &[Felt], root: Felt) -> Result<ExtPoly, String> {
    if polynomial.len() < 2 {
        return Err("linear division needs positive degree".to_string());
    }
    let degree = polynomial.len() - 1;
    let mut quotient = vec![field.zero(); degree];
    quotient[degree - 1] = polynomial[degree];
    for k in (1..degree).rev() {
        quotient[k - 1] = field.add(polynomial[k], field.mul(root, quotient[k]));
    }
    let remainder = field.add(polynomial[0], field.mul(root, quotient[0]));
    if remainder != field.zero() {
        return Err("nonzero synthetic-division remainder".to_string());
    }
    Ok(quotient)
}

struct VeluPolynomials {
    numerator: Vec<u64>,
    denominator: Vec<u64>,
    target_a: u64,
    target_b: u64,
    half_size: usize,
}

fn velu_polynomials(
    field: &Field,
    curve: &Curve<'_>,
    kernel: Point,
    order: u64,
) -> Result<VeluPolynomials, String> {
    let mut half = Vec::new();
    let mut seen_x = HashSet::new();
    let mut point = kernel;
    for _ in 1..order {
        let Point::Affine(x, y) = point else {
            return Err("early infinity in kernel walk".to_string());
        };
        if seen_x.insert(x) {
            half.push((x, y));
        }
        point = curve.add(point, kernel)?;
    }
    if point != Point::Infinity || half.len() * 2 + 1 != order as usize {
        return Err("kernel half-set has wrong cardinality".to_string());
    }

    let two = field.from_base(2);
    let three = field.from_base(3);
    let mut factors = Vec::with_capacity(half.len());
    let mut sum_v = field.zero();
    let mut sum_uxv = field.zero();
    for &(x, y) in &half {
        let gx = field.add(field.mul(three, field.square(x)), curve.a);
        let v = field.mul(two, gx);
        let gy = field.neg(field.mul(two, y));
        let u = field.square(gy);
        sum_v = field.add(sum_v, v);
        sum_uxv = field.add(sum_uxv, field.add(u, field.mul(x, v)));
        factors.push((x, v, u));
    }

    let mut denominator = vec![field.one()];
    for &(x, _, _) in &factors {
        let factor = [field.square(x), field.neg(field.mul(two, x)), field.one()];
        denominator = ext_mul(field, &denominator, &factor);
    }
    let mut numerator = vec![field.zero(); denominator.len() + 1];
    numerator[1..].copy_from_slice(&denominator);
    for &(x, v, u) in &factors {
        let once = ext_divide_x_minus(field, &denominator, x)?;
        let twice = ext_divide_x_minus(field, &once, x)?;
        ext_add_scaled(field, &mut numerator, &once, v);
        ext_add_scaled(field, &mut numerator, &twice, u);
    }

    let target_a = field.sub(curve.a, field.mul(field.from_base(5), sum_v));
    let target_b = field.sub(curve.b, field.mul(field.from_base(7), sum_uxv));
    let numerator = numerator
        .into_iter()
        .rev()
        .map(|value| field.to_base(value))
        .collect::<Result<Vec<_>, _>>()?;
    let denominator = denominator
        .into_iter()
        .rev()
        .map(|value| field.to_base(value))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(VeluPolynomials {
        numerator,
        denominator,
        target_a: field.to_base(target_a)?,
        target_b: field.to_base(target_b)?,
        half_size: half.len(),
    })
}

type Poly = Vec<u64>;

fn trim(mut value: Poly) -> Poly {
    while value.len() > 1 && value.last() == Some(&0) {
        value.pop();
    }
    value
}

fn poly_add(left: &[u64], right: &[u64], p: u64) -> Poly {
    let mut out = vec![0; left.len().max(right.len())];
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = add_mod(
            left.get(i).copied().unwrap_or(0),
            right.get(i).copied().unwrap_or(0),
            p,
        );
    }
    trim(out)
}

fn poly_sub(left: &[u64], right: &[u64], p: u64) -> Poly {
    let mut out = vec![0; left.len().max(right.len())];
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = sub_mod(
            left.get(i).copied().unwrap_or(0),
            right.get(i).copied().unwrap_or(0),
            p,
        );
    }
    trim(out)
}

fn poly_mul(left: &[u64], right: &[u64], p: u64) -> Poly {
    let mut out = vec![0; left.len() + right.len() - 1];
    for (i, &a) in left.iter().enumerate() {
        for (j, &b) in right.iter().enumerate() {
            out[i + j] = add_mod(out[i + j], mul_mod(a, b, p), p);
        }
    }
    trim(out)
}

fn poly_scale_shift(value: &[u64], scale: u64, shift: usize, p: u64) -> Poly {
    let mut out = vec![0; value.len() + shift];
    for (i, &coefficient) in value.iter().enumerate() {
        out[i + shift] = mul_mod(scale, coefficient, p);
    }
    trim(out)
}

fn poly_pow(value: &[u64], exponent: usize, p: u64) -> Poly {
    let mut out = vec![1];
    for _ in 0..exponent {
        out = poly_mul(&out, value, p);
    }
    out
}

fn poly_derivative(value: &[u64], p: u64) -> Poly {
    if value.len() <= 1 {
        return vec![0];
    }
    trim(
        value
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &coefficient)| mul_mod(i as u64 % p, coefficient, p))
            .collect(),
    )
}

fn certificate_identity(
    p: u64,
    source_a: u64,
    source_b: u64,
    target_a: u64,
    target_b: u64,
    numerator: &[u64],
    denominator: &[u64],
) -> bool {
    let np = poly_derivative(numerator, p);
    let dp = poly_derivative(denominator, p);
    let n_minus_wnp = poly_sub(numerator, &poly_scale_shift(&np, 1, 1, p), p);
    let h = poly_add(
        &poly_mul(denominator, &n_minus_wnp, p),
        &poly_scale_shift(&poly_mul(&dp, numerator, p), 1, 1, p),
        p,
    );
    let f = vec![1, 0, source_a % p, source_b % p];
    let left = poly_mul(&f, &poly_pow(&h, 2, p), p);
    let d_n3 = poly_mul(denominator, &poly_pow(numerator, 3, p), p);
    let at_term = poly_scale_shift(
        &poly_mul(numerator, &poly_pow(denominator, 3, p), p),
        target_a,
        2,
        p,
    );
    let bt_term = poly_scale_shift(&poly_pow(denominator, 4, p), target_b, 3, p);
    left == poly_add(&poly_add(&d_n3, &at_term, p), &bt_term, p)
}

fn quotient_prefix(numerator: &[u64], denominator: &[u64], p: u64, precision: usize) -> Vec<u64> {
    assert_eq!(denominator[0], 1);
    let mut out = vec![0; precision];
    for n in 0..precision {
        let mut value = numerator.get(n).copied().unwrap_or(0);
        for i in 1..=n.min(denominator.len() - 1) {
            value = sub_mod(value, mul_mod(denominator[i], out[n - i], p), p);
        }
        out[n] = value;
    }
    out
}

fn recurrence_prefix(
    p: u64,
    source_a: u64,
    source_b: u64,
    target_a: u64,
    target_b: u64,
) -> Vec<u64> {
    let mut out = vec![0; RECURRENCE_PRECISION];
    out[0] = 1;
    out[2] = mul_mod(sub_mod(source_a, target_a, p), inverse_mod(5, p), p);
    out[3] = mul_mod(sub_mod(source_b, target_b, p), inverse_mod(7, p), p);
    for k in 3..8 {
        let mut convolution = 0;
        for i in 1..=(k - 2) {
            convolution = add_mod(convolution, mul_mod(out[i + 1], out[k - i], p), p);
        }
        let step = 2 * k + 3;
        let first = mul_mod(
            mul_mod(3, convolution, p),
            inverse_mod(((k - 2) * step) as u64 % p, p),
            p,
        );
        let second = mul_mod(
            mul_mod((2 * k - 3) as u64 % p, source_a, p),
            mul_mod(out[k - 1], inverse_mod(step as u64 % p, p), p),
            p,
        );
        let third = mul_mod(
            mul_mod((2 * (k - 3)) as u64 % p, source_b, p),
            mul_mod(out[k - 2], inverse_mod(step as u64 % p, p), p),
            p,
        );
        out[k + 1] = sub_mod(sub_mod(first, second, p), third, p);
    }
    out
}

fn snapshot_json(value: CountSnapshot) -> String {
    format!(
        concat!(
            "{{\"base_additions\":{},\"base_multiplications\":{},",
            "\"base_inversions\":{},\"extension_additions\":{},",
            "\"extension_multiplications\":{},\"extension_inversions\":{},",
            "\"frobenius_maps\":{},\"point_additions\":{},",
            "\"scalar_multiplications\":{}}}"
        ),
        value.base_additions,
        value.base_multiplications,
        value.base_inversions,
        value.extension_additions,
        value.extension_multiplications,
        value.extension_inversions,
        value.frobenius_maps,
        value.point_additions,
        value.scalar_multiplications,
    )
}

fn array_json(values: &[u64]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn run_cell(spec: CurveSpec) -> Result<String, String> {
    let (source_a, source_b, models_examined) = select_alternative(spec)?;
    let selected_j = j_invariant(spec.p, source_a, source_b)?;
    let admitted_j = j_invariant(spec.p, spec.admitted_a, spec.admitted_b)?;
    let modulus = first_irreducible(spec.p, spec.degree)?;
    let field = Field::new(spec.p, spec.degree, &modulus)?;
    let curve = Curve {
        field: &field,
        a: field.from_base(source_a),
        b: field.from_base(source_b),
    };
    let (kernel, states_examined) = kernel_generator(&field, &curve, spec)?;
    field.reset_counters();
    let map = velu_polynomials(&field, &curve, kernel, spec.order)?;
    let construction_ops = field.snapshot();

    let differential = (spec.trace - 1).rem_euclid(spec.p as i64) as u64;
    let d2 = mul_mod(differential, differential, spec.p);
    let expected_a = mul_mod(mul_mod(d2, d2, spec.p), source_a, spec.p);
    let d3 = mul_mod(d2, differential, spec.p);
    let expected_b = mul_mod(mul_mod(d3, d3, spec.p), source_b, spec.p);
    let codomain_match = map.target_a == expected_a && map.target_b == expected_b;
    let identity = certificate_identity(
        spec.p,
        source_a,
        source_b,
        map.target_a,
        map.target_b,
        &map.numerator,
        &map.denominator,
    );
    let quotient = quotient_prefix(&map.numerator, &map.denominator, spec.p, JET_PRECISION);
    let recurrence = recurrence_prefix(spec.p, source_a, source_b, map.target_a, map.target_b);
    let prefix_match = quotient[..RECURRENCE_PRECISION] == recurrence;
    let mut tampered = map.numerator.clone();
    tampered[2] = add_mod(tampered[2], 1, spec.p);
    let tamper_rejected = !certificate_identity(
        spec.p,
        source_a,
        source_b,
        map.target_a,
        map.target_b,
        &tampered,
        &map.denominator,
    );
    let normalized =
        map.numerator[0] == 1 && map.denominator[0] == 1 && quotient[0] == 1 && quotient[1] == 0;
    if !(selected_j != admitted_j
        && codomain_match
        && identity
        && prefix_match
        && tamper_rejected
        && normalized)
    {
        return Err(format!("oracle control failed for {}", spec.slug));
    }

    Ok(format!(
        concat!(
            "{{\"slug\":\"{}\",\"p\":{},\"r\":{},\"trace\":{},",
            "\"extension_degree\":{},\"admitted_a\":{},\"admitted_b\":{},",
            "\"admitted_j\":{},\"source_a\":{},\"source_b\":{},",
            "\"source_j\":{},\"models_examined\":{},\"states_examined\":{},",
            "\"half_size\":{},\"target_a\":{},\"target_b\":{},",
            "\"codomain_match\":{},\"identity\":{},\"prefix_match\":{},",
            "\"normalized\":{},\"tamper_rejected\":{},",
            "\"numerator\":{},\"denominator\":{},",
            "\"quotient_prefix\":{},\"recurrence_prefix\":{},",
            "\"construction_ops\":{}}}"
        ),
        spec.slug,
        spec.p,
        spec.order,
        spec.trace,
        spec.degree,
        spec.admitted_a,
        spec.admitted_b,
        admitted_j,
        source_a,
        source_b,
        selected_j,
        models_examined,
        states_examined,
        map.half_size,
        map.target_a,
        map.target_b,
        codomain_match,
        identity,
        prefix_match,
        normalized,
        tamper_rejected,
        array_json(&map.numerator),
        array_json(&map.denominator),
        array_json(&quotient),
        array_json(&recurrence),
        snapshot_json(construction_ops),
    ))
}

fn run() -> Result<(), String> {
    let mut cells = Vec::new();
    for spec in PANEL {
        cells.push(run_cell(spec)?);
    }
    println!(
        "{{\"schema\":\"h116.formal-scalar-oos-oracle/v1\",\"protocol_id\":\"H116-FORMAL-SCALAR-OOS-001\",\"cells\":[{}]}}",
        cells.join(",")
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("h116 formal-scalar OOS oracle error: {error}");
        std::process::exit(1);
    }
}
