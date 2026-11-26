use anyhow::{ensure, Result};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::poly::{Poly, PrimeField};

#[derive(Debug, PartialEq, Eq)]
pub struct RootRun {
    pub roots: Vec<u64>,
    pub extraction_degree: i64,
    pub reflection_center: Option<u64>,
}

/// Find the distinct roots of a nonzero polynomial over the given prime field.
/// Coefficients start with the constant term.
pub fn find_roots(
    field: PrimeField,
    coefficients: &[i128],
    fold: bool,
    seed: u64,
) -> Result<RootRun> {
    roots_of(&Poly::from_signed(field, coefficients), fold, seed)
}

fn split_linear_factors(poly: Poly, rng: &mut StdRng) -> Vec<u64> {
    let field = poly.field;
    let p = field.modulus();
    let one = Poly::new(field, &[1]);
    let mut pending = vec![poly];
    let mut roots = Vec::new();

    while let Some(factor) = pending.pop() {
        if factor.degree() <= 5 {
            roots.extend(factor.native_roots());
            continue;
        }

        // The quadratic characters of X + shift split distinct linear roots.
        let shift = rng.random_range(0..p);
        let character = factor.linear_power_mod(shift, (p - 1) / 2);
        let part = factor.gcd(&character.sub(&one));
        if part.degree() > 0 && part.degree() < factor.degree() {
            pending.push(factor.quotient(&part));
            pending.push(part);
        } else {
            pending.push(factor);
        }
    }
    roots
}

fn reflection_center(poly: &Poly, d: usize) -> Option<u64> {
    let field = poly.field;
    let degree = d as u64 % field.modulus();
    if degree == 0 {
        return Some(0);
    }
    let center = field.mul(
        field.neg(poly.coeff(d - 1)),
        field.inv(field.mul(degree, poly.coeff(d))),
    );
    if center != 0 && d >= 4 {
        // A second odd coefficient rejects most asymmetric inputs before shifting.
        let mut coefficient = 0;
        for k in 0..=3 {
            let i = d - 3 + k;
            let choose = match k {
                0 => 1,
                1 => i as u128,
                2 => i as u128 * (i - 1) as u128 / 2,
                _ => i as u128 * (i - 1) as u128 * (i - 2) as u128 / 6,
            };
            let term = field.mul(poly.coeff(i), (choose % u128::from(field.modulus())) as u64);
            coefficient = field.add(coefficient, field.mul(term, field.pow(center, k)));
        }
        if coefficient != 0 {
            return None;
        }
    }
    Some(center)
}

pub(crate) fn roots_of(poly: &Poly, fold: bool, seed: u64) -> Result<RootRun> {
    let degree = poly.degree();
    ensure!(
        degree >= 0,
        "the zero polynomial vanishes on the entire field"
    );

    let field = poly.field;
    let p = field.modulus();
    if p == 2 || degree < 2 {
        return Ok(RootRun {
            roots: poly.native_roots(),
            extraction_degree: degree,
            reflection_center: None,
        });
    }

    let mut rng = StdRng::seed_from_u64(seed);
    if fold && degree % 2 == 0 {
        if let Some(center) = reflection_center(poly, degree as usize) {
            let shifted;
            let at_center = if center == 0 {
                poly
            } else {
                shifted = poly.shift(center);
                &shifted
            };
            if (1..degree as usize)
                .step_by(2)
                .all(|i| at_center.coeff(i) == 0)
            {
                let coefficients: Vec<_> = (0..=degree as usize)
                    .step_by(2)
                    .map(|i| at_center.coeff(i))
                    .collect();
                let compressed = Poly::new(field, &coefficients);
                let character = compressed.linear_power_mod(0, (p - 1) / 2);
                let squares = compressed.gcd(&character.sub(&Poly::new(field, &[1])));
                let mut roots = Vec::new();
                if coefficients[0] == 0 {
                    roots.push(center);
                }
                for value in split_linear_factors(squares, &mut rng) {
                    let offset = field.sqrt(value).expect("the gcd selects nonzero squares");
                    roots.push(field.add(center, offset));
                    roots.push(field.add(center, field.neg(offset)));
                }
                roots.sort_unstable();
                return Ok(RootRun {
                    roots,
                    extraction_degree: compressed.degree(),
                    reflection_center: Some(center),
                });
            }
        }
    }

    let frobenius = poly.linear_power_mod(0, p);
    let linear = poly.gcd(&frobenius.sub(&Poly::new(field, &[0, 1])));
    let mut roots = split_linear_factors(linear, &mut rng);
    roots.sort_unstable();
    Ok(RootRun {
        roots,
        extraction_degree: degree,
        reflection_center: None,
    })
}
