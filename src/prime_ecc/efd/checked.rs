use super::*;
use crate::ecc::curve::CurveParams;
use crate::ecc::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionPath {
    Formula,
    AffineExceptional,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CheckedPoint {
    pub point: Point,
    pub path: ExecutionPath,
}

/// Interoperable, checked, finite-chart EFD ADD/DBL over short Weierstrass
/// curves. Conversion and validation are charged to this API; it is a reference
/// adapter, not a faster walk. Infinity, equal/opposite inputs and xz-chart
/// exceptions use the existing affine law and expose that fallback in `path`.
pub fn weierstrass_operation(
    curve: &CurveParams,
    id: &str,
    p: &Point,
    q: Option<&Point>,
) -> Result<CheckedPoint> {
    let formula = catalogue()
        .formula(id)
        .ok_or_else(|| EfdError::Unsupported(id.into()))?;
    if !formula.representation.starts_with("shortw/")
        || formula.representation == "shortw/xz"
        || !matches!(formula.operation.as_str(), "addition" | "doubling")
    {
        return Err(EfdError::Unsupported(id.into()));
    }
    if (formula.operation == "addition") != q.is_some() {
        return Err(EfdError::Unsupported(
            "ADD takes two points, DBL one".into(),
        ));
    }
    let modulus = curve.p.clone();
    let a = curve.a_fe();
    let inputs = Bindings::from([
        ("a".into(), a.clone()),
        (
            "b".into(),
            FieldElement::new(curve.b.clone(), modulus.clone()),
        ),
    ]);
    check_field(&modulus, &inputs)?;
    for point in std::iter::once(p).chain(q) {
        if let Point::Affine { x, y } = point {
            if &x.modulus != &modulus || &y.modulus != &modulus || !curve.is_on_curve(point) {
                return Err(EfdError::InvalidPoint);
            }
        }
    }
    let rep = catalogue()
        .representation(&formula.representation)
        .ok_or_else(|| EfdError::Unsupported(formula.representation.clone()))?;
    // `add-1986-cc*` upstream programs are incomplete; the pinned record still
    // states the computation, so use that fallback instead of refusing.
    let program = formula.compiled()?;
    // Do not allow an exceptional-case fallback to bypass specialization checks.
    for assumption in &rep.assumptions {
        let (left, right) = program::equality(assumption)?;
        if left.eval(&modulus, &inputs)? != right.eval(&modulus, &inputs)? {
            return Err(EfdError::Assumption(assumption.clone()));
        }
    }
    let fallback = || CheckedPoint {
        point: if let Some(q) = q {
            p.add(q, &a)
        } else {
            p.double(&a)
        },
        path: ExecutionPath::AffineExceptional,
    };
    let Point::Affine { x, y } = p else {
        return Ok(fallback());
    };
    let z = FieldElement::one(modulus.clone());
    let mut encoded = Bindings::new();
    encoded.extend(rep.bind_point(&rep.encode_affine(&modulus, &inputs, x, y, &z)?, 1));
    if let Some(Point::Affine { x, y }) = q {
        encoded.extend(rep.bind_point(&rep.encode_affine(&modulus, &inputs, x, y, &z)?, 2));
    }
    let mut all = inputs.clone();
    all.extend(encoded);
    let output = match program.evaluate(&modulus, &all) {
        Ok(v) => v,
        Err(EfdError::ZeroDenominator) => return Ok(fallback()),
        Err(e) => return Err(e),
    };
    let (x, y) = match rep.project(&inputs, &output, 3) {
        Ok(Projection::Full { x, y }) => (x, y),
        Err(EfdError::OutsideChart | EfdError::ZeroDenominator) => return Ok(fallback()),
        Err(e) => return Err(e),
        _ => return Err(EfdError::Unsupported(id.into())),
    };
    let point = Point::Affine { x, y };
    if !curve.is_on_curve(&point) {
        return Err(EfdError::InvalidPoint);
    }
    Ok(CheckedPoint {
        point,
        path: ExecutionPath::Formula,
    })
}
