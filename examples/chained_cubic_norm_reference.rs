//! Replay producer norm witnesses with the pre-existing crypto BigUint
//! Jacobian and affine elliptic engines. No producer norm code is imported.
//! Both Jacobian implementations use Cantor; this is not a signed verdict.
use crypto_lib::ecc::{field::FieldElement, point::Point};
use crypto_lib::prime_hyperelliptic::{
    FpPoly, HyperellipticCurveP as Curve, MumfordDivisorP as Divisor,
};
use num_bigint::BigUint;
use num_traits::{One, Zero};

fn p() -> BigUint {
    BigUint::from(101_u32)
}
fn polynomial(c: &[u64]) -> FpPoly {
    FpPoly::from_coeffs(c.iter().map(|x| BigUint::from(*x)).collect(), p())
}
fn curve(degree: usize, genus: u32) -> Curve {
    let mut f = vec![0; degree + 1];
    f[0] = 46;
    f[degree / 3] = 1;
    f[degree] = 1;
    Curve::try_new(p(), polynomial(&f), genus).unwrap()
}
fn affine(x: u64, y: u64) -> Point {
    Point::Affine {
        x: FieldElement::new(BigUint::from(x), p()),
        y: FieldElement::new(BigUint::from(y), p()),
    }
}
fn decode(c: &str) -> FpPoly {
    let coeffs = if c.is_empty() {
        vec![]
    } else {
        c.split(',')
            .map(|v| BigUint::from(v.parse::<u64>().unwrap()))
            .collect()
    };
    FpPoly::from_coeffs(coeffs, p())
}
fn expand(f: &FpPoly) -> FpPoly {
    if f.is_zero() {
        return FpPoly::zero(p());
    }
    let mut c = vec![BigUint::zero(); 3 * f.degree().unwrap() + 1];
    for (i, v) in f.coeffs.iter().enumerate() {
        c[3 * i] = v.clone();
    }
    FpPoly::from_coeffs(c, p())
}
fn pull(d: &Divisor, target: &Curve) -> Divisor {
    Divisor::try_from_polynomials(target, expand(&d.u), expand(&d.v)).unwrap()
}
fn modular_power(mut a: FpPoly, mut n: u64, modulus: &FpPoly) -> FpPoly {
    a = a.divrem(modulus).1;
    let mut out = FpPoly::one(p());
    while n != 0 {
        if n & 1 != 0 {
            out = out.mul(&a).divrem(modulus).1;
        }
        a = a.mul(&a).divrem(modulus).1;
        n >>= 1;
    }
    out
}
// Independent degree classification: rational roots, then a Frobenius gcd.
// It does not enumerate the producer's monic quadratic candidates.
fn closed_support(mut u: FpPoly) -> (bool, bool) {
    for x in 0_u64..101 {
        let q = polynomial(&[(101 - x) % 101, 1]);
        while u.degree().unwrap() > 0 && u.eval(&BigUint::from(x)).is_zero() {
            u = u.divrem(&q).0;
        }
    }
    if u.degree() == Some(3) {
        return (true, false);
    }
    if u.degree() != Some(4) {
        return (false, false);
    }
    let x = polynomial(&[0, 1]);
    let quadratic_roots = modular_power(x.clone(), 101 * 101, &u).sub(&x).gcd(&u);
    (false, quadratic_roots == FpPoly::one(p()))
}
fn verify() {
    let base = curve(3, 1);
    let cover = curve(9, 4);
    let next = curve(27, 13);
    let a =
        Divisor::try_from_point(&cover, &BigUint::from(26_u32), &BigUint::from(37_u32)).unwrap();
    let b = Divisor::try_from_point(&cover, &BigUint::from(4_u32), &BigUint::from(19_u32)).unwrap();
    let pa = pull(&a, &next);
    let pb = pull(&b, &next);
    let g = affine(2, 37);
    let q = affine(64, 19);
    let coefficient = FieldElement::new(BigUint::one(), p());
    assert_eq!(
        g.scalar_mul_vartime(&BigUint::from(109_u32), &coefficient),
        Point::Infinity
    );
    let data = include_str!(
        "../research/chained_jacobian_transfers_20261110/evidence/cubic-norm-witness-01.tsv"
    );
    let mut seen = std::collections::BTreeSet::new();
    let (mut cubic, mut quartic) = (false, false);
    for line in data.lines().skip(2) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 7);
        let offset = fields[0].parse::<u8>().unwrap();
        let k = fields[1].parse::<u64>().unwrap();
        assert!(offset <= 1 && k <= 64);
        assert!(seen.insert((offset, k)));
        let d =
            Divisor::try_from_polynomials(&cover, decode(fields[2]), decode(fields[3])).unwrap();
        let norm =
            Divisor::try_from_polynomials(&base, decode(fields[4]), decode(fields[5])).unwrap();
        let mut expected = a.checked_scalar_mul(&BigUint::from(k), &cover).unwrap();
        let mut expected_next = pa.checked_scalar_mul(&BigUint::from(k), &next).unwrap();
        let mut expected_norm = g.scalar_mul_vartime(&BigUint::from(k), &coefficient);
        if offset == 1 {
            expected = expected.checked_add(&b, &cover).unwrap();
            expected_next = expected_next.checked_add(&pb, &next).unwrap();
            expected_norm = expected_norm.add_vartime(&q, &coefficient);
        }
        assert_eq!(d, expected);
        assert_eq!(pull(&d, &next), expected_next);
        let observed = if norm.u.degree() == Some(0) {
            Point::Infinity
        } else {
            assert_eq!(norm.u.degree(), Some(1));
            Point::Affine {
                x: FieldElement::new((&p() - &norm.u.coeffs[0]) % p(), p()),
                y: FieldElement::new(norm.v.coeffs.first().cloned().unwrap_or_default(), p()),
            }
        };
        assert_eq!(observed, expected_norm, "norm witness offset{offset},k{k}");
        let (c3, c4) = closed_support(d.u);
        cubic |= c3;
        quartic |= c4;
    }
    assert_eq!(seen.len(), 130);
    assert!(cubic && quartic);
    println!("PASS130 exact norm/input witnesses, genus4→13 pullback relations and independently classified irreducible degree3/4 support, using pre-existing crypto Jacobian and affine elliptic arithmetic; unsigned");
}
fn main() {
    verify();
}
#[test]
fn replay_all_closed_point_norm_witnesses() {
    verify();
}
