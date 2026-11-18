//! Prime31-only exact inverse of the merged degree-three Vélu isogeny.
//! Uses Frobenius gcd on the rational inverse cubic; no ECDLP scalar or
//! generic four-sum point table is input to the inverse.
//! This is a reusable algebraic transfer component, NOT an ECDLP break.
//! Protocol: PRIME31_FROBENIUS_CUBIC_INVERSE_PROTOCOL_20261118.md
//! [pcc:unverified]
use prime31_algebraic_ic::{check_curve, Arithmetic, Curve, Point, PRIME31};
use serde_json::json;
use std::time::Instant;

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const U: u64 = 533_591_203;
const TAU: u64 = 1_555_600_864;
const V: u64 = 259_447_350;
const DUAL_U: u64 = 546_710_038;
const INV3: u64 = 1_431_631_221;
const E2: Curve = Curve {
    p: P,
    a: 323_891_735,
    b: 1_985_248_359,
    n: N,
    g: (1_767_020_261, 634_125_456),
};

type Poly = Vec<u64>;

#[derive(Default, Clone, Copy, Debug)]
struct FieldMeter {
    multiplications: u64,
    inversions: u64,
}
impl FieldMeter {
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.multiplications += 1;
        ((u128::from(a) * u128::from(b)) % u128::from(P)) as u64
    }
    fn pow(&mut self, mut a: u64, mut exponent: u64) -> u64 {
        let mut result = 1;
        while exponent > 0 {
            if exponent & 1 != 0 {
                result = self.mul(result, a);
            }
            a = self.mul(a, a);
            exponent >>= 1;
        }
        result
    }
    fn inv(&mut self, a: u64) -> u64 {
        assert_ne!(a, 0, "internal inverse of zero");
        self.inversions += 1;
        self.pow(a, P - 2)
    }
}
fn add(a: u64, b: u64) -> u64 {
    (a + b) % P
}
fn sub(a: u64, b: u64) -> u64 {
    (a + P - b) % P
}
fn trim(mut a: Poly) -> Poly {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn remainder(mut a: Poly, b: &[u64], m: &mut FieldMeter) -> Poly {
    a = trim(a);
    assert!(!b.is_empty(), "zero polynomial divisor");
    let b_inverse = if *b.last().unwrap() == 1 {
        1
    } else {
        m.inv(*b.last().unwrap())
    };
    while a.len() >= b.len() {
        let leading = *a.last().unwrap();
        let coeff = m.mul(leading, b_inverse);
        let shift = a.len() - b.len();
        for (j, &bj) in b.iter().enumerate() {
            let product = m.mul(coeff, bj);
            a[j + shift] = sub(a[j + shift], product);
        }
        a = trim(a);
    }
    a
}
fn mul_mod(a: &[u64], b: &[u64], modulus: &[u64], m: &mut FieldMeter) -> Poly {
    assert!(!a.is_empty() && !b.is_empty());
    let mut product = vec![0; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        for (j, &bj) in b.iter().enumerate() {
            let term = m.mul(ai, bj);
            product[i + j] = add(product[i + j], term);
        }
    }
    remainder(product, modulus, m)
}
fn pow_mod(mut a: Poly, mut exponent: u64, modulus: &[u64], m: &mut FieldMeter) -> Poly {
    let mut result = vec![1];
    while exponent > 0 {
        if exponent & 1 != 0 {
            result = mul_mod(&result, &a, modulus, m);
        }
        exponent >>= 1;
        if exponent > 0 {
            a = mul_mod(&a, &a, modulus, m);
        }
    }
    result
}
fn gcd(mut a: Poly, mut b: Poly, m: &mut FieldMeter) -> Poly {
    a = trim(a);
    b = trim(b);
    while !b.is_empty() {
        let next = remainder(a, &b, m);
        a = b;
        b = next;
    }
    if a.is_empty() {
        return a;
    }
    let unit = if *a.last().unwrap() == 1 {
        1
    } else {
        m.inv(*a.last().unwrap())
    };
    for coefficient in &mut a {
        *coefficient = m.mul(*coefficient, unit);
    }
    a
}
fn inverse_cubic(x_image: u64, m: &mut FieldMeter) -> Poly {
    // F_X(t)=t(t-u)^2+2*tau*(t-u)+4*v-X*(t-u)^2.
    // Every product is reduced before combining to avoid u64 overflow.
    let u2 = m.mul(U, U);
    let two_tau_u = m.mul((2 * TAU) % P, U);
    let xu2 = m.mul(x_image, u2);
    let c0 = sub(sub((4 * V) % P, two_tau_u), xu2);
    let two_x_u = m.mul((2 * x_image) % P, U);
    let c1 = add(add(u2, two_x_u), (2 * TAU) % P);
    let c2 = sub(0, (2 * U + x_image) % P);
    vec![c0, c1, c2, 1]
}
fn on_target(q: Point, m: &mut FieldMeter) -> bool {
    let Some((x, y)) = q else { return true; };
    if x >= P || y >= P {
        return false;
    }
    let x2 = m.mul(x, x);
    let x3 = m.mul(x2, x);
    let ax = m.mul(E2.a, x);
    let y2 = m.mul(y, y);
    y2 == add(add(x3, ax), E2.b)
}
fn invert_public(q: Point, m: &mut FieldMeter) -> Result<Point, String> {
    let Some((x_image, y_image)) = q else { return Ok(None); };
    if !on_target(q, m) {
        return Err("off-curve or out-of-range codomain point".into());
    }
    let f = inverse_cubic(x_image, m);
    let mut x_to_p = pow_mod(vec![0, 1], P, &f, m);
    if x_to_p.len() < 2 {
        x_to_p.resize(2, 0);
    }
    x_to_p[1] = sub(x_to_p[1], 1);
    let common = gcd(f.clone(), x_to_p, m);
    if common.len() != 2 || common[1] != 1 {
        return Err("inverse fibre did not have exactly one rational x root".into());
    }
    let x = sub(0, common[0]);
    let h = sub(x, U);
    if h == 0 {
        return Err("rational inverse landed on a geometric kernel pole".into());
    }
    let h2 = m.mul(h, h);
    let h3 = m.mul(h2, h);
    let first = m.mul(x, h2);
    let tau_term = m.mul((2 * TAU) % P, h);
    let xh2 = m.mul(x_image, h2);
    if sub(add(add(first, tau_term), (4 * V) % P), xh2) != 0 {
        return Err("recovered x did not satisfy the original cubic".into());
    }
    let derivative_numerator = sub(sub(h3, tau_term), (8 * V) % P);
    if derivative_numerator == 0 {
        return Err("zero inverse ordinate denominator".into());
    }
    let den_inverse = m.inv(derivative_numerator);
    let factor = m.mul(h3, den_inverse);
    let y = m.mul(y_image, factor);
    Ok(Some((x, y)))
}
fn forward(q: Point, u: u64, tau: u64, v: u64, m: &mut FieldMeter) -> Point {
    q.map(|(x, y)| {
        let h_inverse = m.inv(sub(x, u));
        let h2_inverse = m.mul(h_inverse, h_inverse);
        let h3_inverse = m.mul(h2_inverse, h_inverse);
        let term1 = m.mul((2 * tau) % P, h_inverse);
        let term2 = m.mul((4 * v) % P, h2_inverse);
        let xx = add(x, add(term1, term2));
        let derivative1 = m.mul((2 * tau) % P, h2_inverse);
        let derivative2 = m.mul((8 * v) % P, h3_inverse);
        let yy = m.mul(y, sub(sub(1, derivative1), derivative2));
        (xx, yy)
    })
}
fn quadratic_nonsquare(f: &[u64], r: u64, m: &mut FieldMeter) -> bool {
    // Divide monic cubic f by (t-r): remaining t^2 + b*t + c.
    let b = add(f[2], r);
    let rb = m.mul(r, b);
    let c = add(f[1], rb);
    let rc = m.mul(r, c);
    if f[0] != sub(0, rc) {
        return false;
    }
    let bb = m.mul(b, b);
    let disc = sub(bb, m.mul(4, c));
    m.pow(disc, (P - 1) / 2) == P - 1
}
fn panel() -> serde_json::Value {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("original ordinary cofactor-one prime31");
    check_curve(E2, &mut ar).expect("degree-three codomain prime-order check");
    assert_eq!((3 * INV3) % N, 1);
    let targets: Vec<Point> = (1..=256)
        .map(|k| ar.scalar(E2, k, Some(E2.g)))
        .collect();
    assert_eq!(targets[0], Some(E2.g));

    let mut direct_meter = FieldMeter::default();
    let start = Instant::now();
    let source_points: Vec<Point> = targets
        .iter()
        .map(|&q| invert_public(q, &mut direct_meter).expect("unique linear factor"))
        .collect();
    let direct_ns = start.elapsed().as_nanos();

    // Dual inverse reference: normalized dual gives (9*x([3]P),27*y([3]P)).
    // Undo scaling then multiply by 1/3 modulo n using original EC arithmetic.
    let mut setup_meter = FieldMeter::default();
    let du2 = setup_meter.mul(DUAL_U, DUAL_U);
    let du3 = setup_meter.mul(du2, DUAL_U);
    let three_u2 = setup_meter.mul(3, du2);
    let dual_tau = add(three_u2, E2.a);
    let aux = setup_meter.mul(E2.a, DUAL_U);
    let dual_v = add(add(du3, aux), E2.b);
    let inv9 = setup_meter.inv(9);
    let inv27 = setup_meter.inv(27);
    let before = ar.counts;
    let mut dual_meter = FieldMeter::default();
    let start_dual = Instant::now();
    let dual_points: Vec<Point> = targets
        .iter()
        .map(|&q| {
            let scaled = forward(q, DUAL_U, dual_tau, dual_v, &mut dual_meter);
            let back3 = scaled.map(|(x, y)| {
                (dual_meter.mul(x, inv9), dual_meter.mul(y, inv27))
            });
            ar.scalar(PRIME31, INV3, back3)
        })
        .collect();
    let dual_ns = start_dual.elapsed().as_nanos();
    let after = ar.counts;
    assert_eq!(dual_points, source_points, "dual-plus-scalar inverse comparator");

    let mut certificate = FieldMeter::default();
    for (index, (&q, &source)) in targets.iter().zip(source_points.iter()).enumerate() {
        assert!(ar.on_curve(PRIME31, source), "inverse outside source curve");
        assert_eq!(
            source,
            ar.scalar(PRIME31, index as u64 + 1, Some(PRIME31.g)),
            "source known-answer is caller-only and never passed to invert_public"
        );
        assert_eq!(forward(source, U, TAU, V, &mut certificate), q);
        let (x, _) = source.expect("nonidentity public panel");
        let inverse_poly = inverse_cubic(q.unwrap().0, &mut certificate);
        assert!(
            quadratic_nonsquare(&inverse_poly, x, &mut certificate),
            "nonlinear conjugate factor must be irreducible"
        );
    }
    assert_eq!(invert_public(None, &mut certificate).unwrap(), None);
    let neg_q = ar.neg(E2, targets[0]);
    let neg_source = invert_public(neg_q, &mut certificate).unwrap();
    assert_eq!(neg_source, ar.neg(PRIME31, source_points[0]));
    let wrong_y = (E2.g.1 + 1) % P;
    assert!(invert_public(Some((E2.g.0, wrong_y)), &mut certificate).is_err());
    let (direct_fp_multiplications, direct_fp_inversions) =
        (direct_meter.multiplications, direct_meter.inversions);
    let dual_native_muls = after.fp_multiplications - before.fp_multiplications;
    let dual_native_invs = after.fp_inversions - before.fp_inversions;
    let transcript: Vec<_> = targets.iter().zip(source_points.iter()).enumerate()
        .map(|(case, (&public_q, &source_preimage))| json!({
            "case":case + 1, "public_q":public_q, "source_preimage":source_preimage
        })).collect();
    json!({
        "schema":"prime31.frobenius_inverse_cubic.v1",
        "standing":"[pcc:unverified]",
        "algorithm":"exact nonenumerative degree-3 isogeny inversion by unique Fp root; not a fresh ECDLP relation extractor",
        "p":P, "n":N, "cofactor":1,
        "source_curve":[PRIME31.a,PRIME31.b],
        "codomain_curve":[E2.a,E2.b],
        "public_targets":targets.len(),
        "public_transcript":transcript,
        "unique_linear_gcd":targets.len(),
        "nontrivial_quadratic_irreducible":targets.len(),
        "forward_roundtrip_ok":targets.len(),
        "caller_only_scalar_checks":targets.len(),
        "dual_agreement":targets.len(),
        "negation_identity_bad_point_controls":3,
        "direct_inverse_fp_multiplications":direct_fp_multiplications,
        "direct_inverse_fp_inversions":direct_fp_inversions,
        "dual_setup_fp_multiplications":setup_meter.multiplications,
        "dual_setup_fp_inversions":setup_meter.inversions,
        "dual_component_fp_multiplications":dual_meter.multiplications,
        "dual_component_fp_inversions":dual_meter.inversions,
        "dual_native_scalar_fp_multiplications":dual_native_muls,
        "dual_native_scalar_fp_inversions":dual_native_invs,
        "dual_native_group_additions":after.group_additions - before.group_additions,
        "direct_stage_ns_diagnostic":direct_ns.to_string(),
        "dual_stage_ns_diagnostic":dual_ns.to_string(),
        "full_index_calculus_logs_solved":0,
        "full_cost_speedup_claim":false,
        "independent_signed_pcc":false
    })
}
fn main() {
    println!("{}", panel());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_256_public_targets_and_frobenius_quadratic() {
        let v = panel();
        assert_eq!(v["public_targets"], 256);
        assert_eq!(v["dual_agreement"], 256);
        assert_eq!(v["full_index_calculus_logs_solved"], 0);
    }
    #[test]
    fn sample_generator_inverse_and_gcd() {
        let mut m = FieldMeter::default();
        assert_eq!(invert_public(Some(E2.g), &mut m).unwrap(), Some(PRIME31.g));
        assert_eq!(
            inverse_cubic(E2.g.0, &mut m),
            vec![1_622_785_654, 1_444_346_180, 1_460_764_627, 1]
        );
    }
}