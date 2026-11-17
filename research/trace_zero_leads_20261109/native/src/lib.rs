//! Known fixtures and structural checks; all arithmetic/model code is production.
use full_trace_zero_backend::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
use num_bigint::BigUint;
use num_traits::One;

pub fn fixture(n: u32) -> BinaryCurve {
    if n == 131 {
        return BinaryCurve::certicom_challenge("ECC2K-130").unwrap().0;
    }
    let (terms, order) = match n {
        3 => (vec![0, 1], BigUint::from(1u32)),
        5 => (vec![0, 2], BigUint::from(11u32)),
        7 => (vec![0, 1], BigUint::from(29u32)),
        9 => (vec![0, 4], BigUint::from(127u32)),
        83 => (
            vec![0, 1, 2, 45],
            BigUint::parse_bytes(b"2417851639230796216685689", 10).unwrap(),
        ),
        _ => panic!("unsupported fixture"),
    };
    let generator = if n == 83 {
        BinaryPoint::Affine {
            x: F2mElement::from_hex("477f77103dfad59850800", n),
            y: F2mElement::from_hex("2fa5e737d542c4e4fd5c3", n),
        }
    } else {
        BinaryPoint::Infinity
    };
    BinaryCurve {
        m: n,
        irreducible: IrreduciblePoly {
            degree: n,
            low_terms: terms,
        },
        a: F2mElement::zero(n),
        b: F2mElement::one(n),
        generator,
        order,
        cofactor: BigUint::from(4u32),
    }
}

/// Trace-zero projection of polynomial basis powers; does not change dimension.
pub fn window(curve: &BinaryCurve, d: usize) -> Vec<F2mElement> {
    assert!(d < curve.m as usize);
    (1..=d)
        .map(|i| {
            let b = F2mElement::from_biguint(&(BigUint::one() << i), curve.m);
            if b.trace(&curve.irreducible) == 0 {
                b
            } else {
                b.add(&F2mElement::one(curve.m))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests;
