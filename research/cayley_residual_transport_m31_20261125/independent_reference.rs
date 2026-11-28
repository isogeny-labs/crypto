//! Independent M31 / Fp^2 reference for residual chord labels.
//!
//! Written from the mathematical specification without reading the candidate
//! implementation. This file imports neither candidate nor upstream arithmetic.
//! No randomness, timing, fixture generation, or test execution lives here.
#![allow(dead_code)]

pub const P: u64 = 2_147_483_647;
pub type Fp2 = (u64, u64); // a + i*b, where i^2 = -1.
pub type Projective = (u64, u64); // (X:Z), with C=(X+iZ)/(X-iZ).
pub type Affine = (u64, u64);
pub type Point = Option<Affine>;

pub fn fp_add(a: u64, b: u64) -> u64 {
    (a + b) % P
}

pub fn fp_sub(a: u64, b: u64) -> u64 {
    (a + P - b) % P
}

pub fn fp_mul(a: u64, b: u64) -> u64 {
    // Both operands are canonical M31 residues; their product fits in u64.
    (a * b) % P
}

pub fn fp_pow(mut a: u64, mut exponent: u64) -> u64 {
    let mut accumulator = 1;
    while exponent != 0 {
        if exponent & 1 == 1 {
            accumulator = fp_mul(accumulator, a);
        }
        a = fp_mul(a, a);
        exponent >>= 1;
    }
    accumulator
}

pub fn fp_inverse(a: u64) -> Result<u64, String> {
    if a == 0 || a >= P {
        return Err("reference inverse requires a canonical nonzero residue".into());
    }
    Ok(fp_pow(a, P - 2))
}

pub fn fp_sqrt(a: u64) -> Result<Option<u64>, String> {
    if a >= P {
        return Err("reference square root requires a canonical residue".into());
    }
    let candidate = fp_pow(a, (P + 1) / 4);
    Ok((fp_mul(candidate, candidate) == a).then_some(candidate))
}

fn canonical_fp2(z: Fp2) -> bool {
    z.0 < P && z.1 < P
}

pub fn extension_mul(z: Fp2, t: Fp2) -> Fp2 {
    (
        fp_sub(fp_mul(z.0, t.0), fp_mul(z.1, t.1)),
        fp_add(fp_mul(z.0, t.1), fp_mul(z.1, t.0)),
    )
}

pub fn extension_div(z: Fp2, t: Fp2) -> Result<Fp2, String> {
    if !canonical_fp2(z) || !canonical_fp2(t) {
        return Err("reference extension division requires canonical inputs".into());
    }
    let denominator = fp_add(fp_mul(t.0, t.0), fp_mul(t.1, t.1));
    let inverse = fp_inverse(denominator)?;
    // Multiply numerator by the conjugate of the denominator.
    let conjugate_product = extension_mul(z, (t.0, fp_sub(0, t.1)));
    Ok((
        fp_mul(conjugate_product.0, inverse),
        fp_mul(conjugate_product.1, inverse),
    ))
}

pub fn norm_one(z: Fp2) -> bool {
    canonical_fp2(z) && fp_add(fp_mul(z.0, z.0), fp_mul(z.1, z.1)) == 1
}

/// Evaluate C(X:Z) directly in Fp^2. Every valid projective point maps to T2.
/// Because -1 is a nonsquare in M31, X-iZ is nonzero whenever (X,Z)!=(0,0).
pub fn cayley(parameter: Projective) -> Result<Fp2, String> {
    let (x, z) = parameter;
    if x >= P || z >= P || (x == 0 && z == 0) {
        return Err("reference Cayley input is not a canonical projective point".into());
    }
    extension_div((x, z), (x, fp_sub(0, z)))
}

/// Recover the unique canonical projective parameter: (t,1), or (1,0).
/// For C(t)=a+ib with C(t)!=1, t=b/(1-a).
pub fn canonical_parameter(label: Fp2) -> Result<Projective, String> {
    if !norm_one(label) {
        return Err("reference label is not a canonical norm-one element".into());
    }
    if label == (1, 0) {
        return Ok((1, 0));
    }
    let inverse = fp_inverse(fp_sub(1, label.0))?;
    Ok((fp_mul(label.1, inverse), 1))
}

fn check_label_parameters(
    w: u64,
    v: u64,
    s: u64,
    delta: u64,
    eta: Fp2,
) -> Result<u64, String> {
    if w >= P || v >= P || s >= P || delta == 0 || delta >= P {
        return Err("reference residual data are noncanonical or delta is zero".into());
    }
    if !norm_one(eta) {
        return Err("reference prefix label is not a norm-one element".into());
    }
    let h = fp_sub(s, w);
    if h == 0 {
        return Err("reference residual map degenerates when s=w".into());
    }
    Ok(h)
}

/// Direct extension-field evaluation of
/// eta * (v-m*(s-w)+i*delta)/(v-m*(s-w)-i*delta).
///
/// None is the projective infinite slope. Its normalized label is 1 and its
/// RAW label is eta. This is distinct from the raw identity unless eta=1.
/// The caller separately certifies K and S lie on the intended curve; this
/// function checks the algebraic nondegeneracy needed by the rational map.
pub fn evaluate_label(
    w: u64,
    v: u64,
    s: u64,
    delta: u64,
    eta: Fp2,
    slope: Option<u64>,
) -> Result<Fp2, String> {
    let h = check_label_parameters(w, v, s, delta, eta)?;
    match slope {
        None => Ok(eta),
        Some(m) if m < P => {
            let line_value = fp_sub(v, fp_mul(m, h));
            let quotient = extension_div(
                (line_value, delta),
                (line_value, fp_sub(0, delta)),
            )?;
            Ok(extension_mul(eta, quotient))
        }
        Some(_) => Err("reference slope is not canonical".into()),
    }
}

/// Invert the raw label through direct Fp^2 division before recovering m.
/// Normalizing by eta gives C(t), with t=(v-m*(s-w))/delta.
pub fn recover_slope(
    w: u64,
    v: u64,
    s: u64,
    delta: u64,
    eta: Fp2,
    label: Fp2,
) -> Result<Option<u64>, String> {
    let h = check_label_parameters(w, v, s, delta, eta)?;
    if !norm_one(label) {
        return Err("reference raw label is not a norm-one element".into());
    }
    let normalized = extension_div(label, eta)?;
    if normalized == (1, 0) {
        return Ok(None);
    }
    let (t, denominator) = canonical_parameter(normalized)?;
    if denominator != 1 {
        return Err("reference internal normalization failed".into());
    }
    let m = fp_mul(fp_sub(v, fp_mul(delta, t)), fp_inverse(h)?);
    Ok(Some(m))
}

pub fn nonsingular(a: u64, b: u64) -> bool {
    a < P
        && b < P
        && fp_add(fp_mul(4, fp_mul(fp_mul(a, a), a)), fp_mul(27, fp_mul(b, b)))
            != 0
}

pub fn on_curve(a: u64, b: u64, point: Point) -> bool {
    if a >= P || b >= P {
        return false;
    }
    match point {
        None => true,
        Some((x, y)) if x < P && y < P => {
            fp_mul(y, y) == fp_add(fp_add(fp_mul(fp_mul(x, x), x), fp_mul(a, x)), b)
        }
        Some(_) => false,
    }
}

/// S=(s,i*delta) is a nonrational trace-zero curve point precisely when
/// delta!=0 and -delta^2=s^3+a*s+b. Nonrationality uses -1 nonsquare in M31.
pub fn trace_zero_anchor(a: u64, b: u64, s: u64, delta: u64) -> bool {
    a < P
        && b < P
        && s < P
        && delta > 0
        && delta < P
        && fp_sub(0, fp_mul(delta, delta))
            == fp_add(fp_add(fp_mul(fp_mul(s, s), s), fp_mul(a, s)), b)
}

pub fn point_neg(point: Point) -> Point {
    point.map(|(x, y)| (x, fp_sub(0, y)))
}

/// Independent ordinary affine elliptic group law, including identity,
/// opposite points and doubling at a zero ordinate.
pub fn point_add(a: u64, b: u64, left: Point, right: Point) -> Result<Point, String> {
    if !nonsingular(a, b) || !on_curve(a, b, left) || !on_curve(a, b, right) {
        return Err("reference group addition requires a nonsingular curve and curve points".into());
    }
    let (Some((x1, y1)), Some((x2, y2))) = (left, right) else {
        return Ok(left.or(right));
    };
    if x1 == x2 && fp_add(y1, y2) == 0 {
        return Ok(None);
    }
    let slope = if x1 == x2 {
        fp_mul(
            fp_add(fp_mul(3, fp_mul(x1, x1)), a),
            fp_inverse(fp_add(y1, y1))?,
        )
    } else {
        fp_mul(fp_sub(y2, y1), fp_inverse(fp_sub(x2, x1))?)
    };
    let x3 = fp_sub(fp_sub(fp_mul(slope, slope), x1), x2);
    let y3 = fp_sub(fp_mul(slope, fp_sub(x1, x3)), y1);
    let result = Some((x3, y3));
    if !on_curve(a, b, result) {
        return Err("reference addition produced a point off the curve".into());
    }
    Ok(result)
}

/// Validate a claimed affine residual pair by the independent group law.
pub fn residual_pair_valid(a: u64, b: u64, target: Affine, pair: (Affine, Affine)) -> bool {
    point_add(a, b, Some(pair.0), Some(pair.1)) == Ok(Some(target))
}

/// Evaluate f_{U,V}(S)/f_{U,V}(-S) independently, where
/// f_{U,V}=ell_{U,V}/vertical_{U+V} and S=(s,i*delta).
///
/// Every vertical factor is Fp-valued at S and -S and therefore cancels.
/// For a nonvertical chord use ell=y-y(U)-m*(x-x(U)); the line is defined
/// over Fp, so the values at the two anchors are conjugates. Its imaginary
/// coefficient is delta!=0, hence neither value vanishes. If a summand is O,
/// f=1. If U=-V, f is vertical and the ratio is likewise 1.
///
/// This is a principal-divisor label, not a product of Cayley x-coordinates.
pub fn chord_ratio(
    a: u64,
    b: u64,
    left: Point,
    right: Point,
    s: u64,
    delta: u64,
) -> Result<Fp2, String> {
    if !trace_zero_anchor(a, b, s, delta) {
        return Err("reference chord ratio requires a nonrational trace-zero curve anchor".into());
    }
    // This independently validates the curve and both summands, including
    // identity and vertical cases, before their evaluation shortcuts.
    let sum = point_add(a, b, left, right)?;
    let (Some((x1, y1)), Some((x2, y2))) = (left, right) else {
        return Ok((1, 0));
    };
    if sum.is_none() {
        return Ok((1, 0));
    }
    let slope = if x1 == x2 {
        fp_mul(
            fp_add(fp_mul(3, fp_mul(x1, x1)), a),
            fp_inverse(fp_add(y1, y1))?,
        )
    } else {
        fp_mul(fp_sub(y2, y1), fp_inverse(fp_sub(x2, x1))?)
    };
    let real_value = fp_sub(fp_sub(0, y1), fp_mul(slope, fp_sub(s, x1)));
    extension_div((real_value, delta), (real_value, fp_sub(0, delta)))
}

/// Prefix factors for a four-summand target relation. With A=P1+P2 and
/// K=R-A, eta_j is ratio(f_{P1,P2})*ratio(f_{A,K}) at anchor j.
/// For any residual P3+P4=K, the complete four-point ratio is
/// eta_j*ratio(f_{P3,P4}). No hidden scalar or expected residual pair is used.
pub fn prefix_factors(
    a: u64,
    b: u64,
    r: Point,
    p1: Point,
    p2: Point,
    anchors: [(u64, u64); 2],
) -> Result<(Point, [Fp2; 2]), String> {
    let prefix_sum = point_add(a, b, p1, p2)?;
    let k = point_add(a, b, r, point_neg(prefix_sum))?;
    let mut factors = [(1, 0); 2];
    for (index, (s, delta)) in anchors.into_iter().enumerate() {
        let first = chord_ratio(a, b, p1, p2, s, delta)?;
        let joining = chord_ratio(a, b, prefix_sum, k, s, delta)?;
        factors[index] = extension_mul(first, joining);
    }
    Ok((k, factors))
}

/// Direct principal-divisor ratio for four points, using the pairing
/// (P1,P2),(P3,P4). If A=P1+P2, K=P3+P4 and R=A+K, the product
/// f_{P1,P2}*f_{P3,P4}*f_{A,K} has divisor
/// (P1)+(P2)+(P3)+(P4)-(R)-3(O).
///
/// Different association orders define the same divisor and differ only by
/// an Fp scalar, which cancels from the ratio at conjugate anchors. Identity
/// summands and cancelling/vertical pairs are permitted and checked.
pub fn four_point_label(
    a: u64,
    b: u64,
    points: [Point; 4],
    anchor: (u64, u64),
) -> Result<Fp2, String> {
    let [p1, p2, p3, p4] = points;
    let (s, delta) = anchor;
    let prefix_sum = point_add(a, b, p1, p2)?;
    let residual_sum = point_add(a, b, p3, p4)?;
    let first = chord_ratio(a, b, p1, p2, s, delta)?;
    let second = chord_ratio(a, b, p3, p4, s, delta)?;
    let joining = chord_ratio(a, b, prefix_sum, residual_sum, s, delta)?;
    Ok(extension_mul(extension_mul(first, second), joining))
}
