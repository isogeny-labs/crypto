use num_bigint::BigUint;
use num_traits::{One, ToPrimitive};
use rand::Rng;

use crate::prime_hyperelliptic::{
    FpPoly, HyperellipticCurveP, MumfordDivisorP, PrimeHyperellipticError,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum FactorBaseError {
    BudgetExceeded,
    RootSearchExhausted,
    InvalidDivisor(PrimeHyperellipticError),
}

pub(super) fn build(
    curve: &HyperellipticCurveP,
    degree: u32,
    polynomial_budget: usize,
    rng: &mut impl Rng,
) -> Result<Vec<MumfordDivisorP>, FactorBaseError> {
    curve.validate().map_err(FactorBaseError::InvalidDivisor)?;
    let degree = degree.min(curve.genus);
    if degree == 0 {
        return Ok(Vec::new());
    }
    let p = curve.p.to_usize().ok_or(FactorBaseError::BudgetExceeded)?;
    let mut counts = Vec::new();
    let mut at_degree = 1usize;
    let mut total = 0usize;
    for _ in 0..degree {
        at_degree = at_degree
            .checked_mul(p)
            .ok_or(FactorBaseError::BudgetExceeded)?;
        total = total
            .checked_add(at_degree)
            .ok_or(FactorBaseError::BudgetExceeded)?;
        if total > polynomial_budget {
            return Err(FactorBaseError::BudgetExceeded);
        }
        counts.push(at_degree);
    }

    let mut base = Vec::new();
    for (index, count) in counts.into_iter().enumerate() {
        let degree = index + 1;
        for code in 0..count {
            let mut coefficients = decode_coefficients(code, degree, p);
            coefficients.push(BigUint::one());
            let u = FpPoly::from_coeffs(coefficients, curve.p.clone());
            if !is_irreducible(&u, degree) {
                continue;
            }
            let value = curve.f.rem(&u);
            let Some(v) = residue_square_root(&value, &u, degree, p, count, rng)? else {
                continue;
            };
            let negated = v.neg();
            let v = if negated.coeffs < v.coeffs {
                negated
            } else {
                v
            };
            let place = MumfordDivisorP::try_from_polynomials(curve, u, v)
                .map_err(FactorBaseError::InvalidDivisor)?;
            base.push(place);
        }
    }
    Ok(base)
}

fn decode_coefficients(mut code: usize, degree: usize, p: usize) -> Vec<BigUint> {
    (0..degree)
        .map(|_| {
            let digit = code % p;
            code /= p;
            BigUint::from(digit)
        })
        .collect()
}

fn is_irreducible(u: &FpPoly, degree: usize) -> bool {
    // A reducible polynomial has an irreducible factor of degree at most
    // half its degree. Factors of degree d divide x^(p^d) - x.
    let x = FpPoly::x(u.p.clone());
    let mut power = x.clone();
    for _ in 0..degree / 2 {
        power = power.pow_mod(&u.p, u);
        if u.gcd(&power.sub(&x)).degree() != Some(0) {
            return false;
        }
    }
    true
}

/// Solve the square-root equation in the residue field F_p[x]/(u).
fn residue_square_root(
    value: &FpPoly,
    u: &FpPoly,
    degree: usize,
    p: usize,
    order: usize,
    rng: &mut impl Rng,
) -> Result<Option<FpPoly>, FactorBaseError> {
    if value.is_zero() {
        return Ok(Some(value.clone()));
    }
    let one = FpPoly::one(u.p.clone());
    let minus_one = one.neg();
    let order_minus_one = BigUint::from(order - 1);
    let half = &order_minus_one >> 1;
    if value.pow_mod(&half, u) != one {
        return Ok(None);
    }

    // Tonelli–Shanks starts with the odd part of the residue-field order
    // minus one. For order = 3 mod 4 the first exponent is already enough.
    let mut two_adicity = (order - 1).trailing_zeros();
    let odd_part = order_minus_one >> two_adicity;
    let mut root = value.pow_mod(&((&odd_part + 1u32) >> 1), u);
    if two_adicity == 1 {
        return Ok(Some(root));
    }
    let mut power = value.pow_mod(&odd_part, u);
    if power == one {
        return Ok(Some(root));
    }
    let nonresidue = (0..64)
        .map(|_| {
            let code = rng.gen_range(1..order);
            FpPoly::from_coeffs(decode_coefficients(code, degree, p), u.p.clone())
        })
        .find(|candidate| candidate.pow_mod(&half, u) == minus_one)
        .ok_or(FactorBaseError::RootSearchExhausted)?;
    let mut correction = nonresidue.pow_mod(&odd_part, u);
    while power != one {
        let mut squared = power.mul(&power).rem(u);
        let mut index = 1;
        while squared != one && index < two_adicity {
            squared = squared.mul(&squared).rem(u);
            index += 1;
        }
        if index >= two_adicity {
            return Err(FactorBaseError::RootSearchExhausted);
        }
        let exponent = BigUint::one() << (two_adicity - index - 1);
        let step = correction.pow_mod(&exponent, u);
        correction = step.mul(&step).rem(u);
        root = root.mul(&step).rem(u);
        power = power.mul(&correction).rem(u);
        two_adicity = index;
    }
    Ok(Some(root))
}

/// Signed coordinates for a reduced divisor over the enumerated places.
/// The sign comes from its local value of v, including at repeated factors.
/// The base produced by build is in increasing degree order.
pub(super) fn smooth_coordinates(
    divisor: &MumfordDivisorP,
    base: &[MumfordDivisorP],
) -> Option<Vec<i32>> {
    let mut remaining = divisor.u.clone();
    let mut entries = Vec::new();
    for (index, place) in base.iter().enumerate() {
        let degree = place.u.degree()?;
        if degree == 0 {
            return None;
        }
        if remaining.degree()? < degree {
            break;
        }
        let mut multiplicity = 0i32;
        loop {
            let (quotient, remainder) = remaining.divrem(&place.u);
            if !remainder.is_zero() {
                break;
            }
            multiplicity = multiplicity.checked_add(1)?;
            remaining = quotient;
        }
        if multiplicity != 0 {
            let value = divisor.v.rem(&place.u);
            let coefficient = if value == place.v {
                multiplicity
            } else if value == place.v.neg() {
                -multiplicity
            } else {
                return None;
            };
            entries.push((index, coefficient));
        }
    }
    if remaining != FpPoly::one(divisor.u.p.clone()) {
        return None;
    }
    let mut coordinates = vec![0; base.len()];
    for (index, coefficient) in entries {
        coordinates[index] = coefficient;
    }
    Some(coordinates)
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn includes_split_extension_places_and_tracks_signed_multiplicity() {
        let p = BigUint::from(3u32);
        let polynomial = |coefficients: &[u32]| {
            FpPoly::from_coeffs(
                coefficients.iter().copied().map(BigUint::from).collect(),
                p.clone(),
            )
        };
        let curve = HyperellipticCurveP::new(p.clone(), polynomial(&[1, 0, 0, 2, 0, 0, 0, 1]), 3);
        let mut rng = StdRng::seed_from_u64(14);
        assert_eq!(
            build(&curve, 2, 11, &mut rng),
            Err(FactorBaseError::BudgetExceeded)
        );
        let base = build(&curve, 2, 12, &mut rng).unwrap();
        assert_eq!(base.len(), 5);
        let extension = base
            .iter()
            .find(|place| place.u == polynomial(&[2, 2, 1]))
            .unwrap();
        assert_eq!(extension.v.degree(), Some(1));
        assert!(smooth_coordinates(extension, &base[..3]).is_none());

        let repeated =
            MumfordDivisorP::try_from_polynomials(&curve, polynomial(&[0, 0, 1]), polynomial(&[2]))
                .unwrap();
        assert_eq!(
            smooth_coordinates(&repeated, &base),
            Some(vec![-2, 0, 0, 0, 0])
        );
    }
}
