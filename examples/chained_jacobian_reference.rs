//! Differential development test against the pre-existing BigUint arithmetic.
//! This is not an independent signed validation or a timing benchmark.
#[allow(dead_code)]
#[path = "../research/chained_jacobian_transfers_20261110/native/src/lib.rs"]
mod chain;

use crypto_lib::prime_hyperelliptic::{FpPoly, HyperellipticCurveP, MumfordDivisorP};
use num_bigint::BigUint;

fn polynomial(c: &[i64]) -> FpPoly {
    FpPoly::from_coeffs(
        c.iter().map(|x| BigUint::from(*x as u64)).collect(),
        BigUint::from(101_u32),
    )
}

// t=a*z, Y=a²*Z, a=1/leading(F). Then F(a*z)/a^4 is monic.
// No square root or quadratic twist of the source curve is introduced.
fn curve() -> HyperellipticCurveP {
    let f = chain::quintic();
    let a = chain::inv(*f.last().unwrap());
    let scale = chain::inv(chain::pow(a, 4));
    let monic: Vec<_> = f
        .iter()
        .enumerate()
        .map(|(i, c)| chain::fp(c * chain::pow(a, i as u64) * scale))
        .collect();
    HyperellipticCurveP::try_new(BigUint::from(101_u32), polynomial(&monic), 2).unwrap()
}

fn convert(d: &chain::Divisor, c: &HyperellipticCurveP) -> MumfordDivisorP {
    let a = chain::inv(*chain::quintic().last().unwrap());
    let leading = chain::pow(a, (d.u.len() - 1) as u64);
    let u: Vec<_> =
        d.u.iter()
            .enumerate()
            .map(|(i, x)| chain::fp(x * chain::pow(a, i as u64) * chain::inv(leading)))
            .collect();
    let v: Vec<_> =
        d.v.iter()
            .enumerate()
            .map(|(i, x)| chain::fp(x * chain::pow(a, i as u64) * chain::inv(chain::pow(a, 2))))
            .collect();
    MumfordDivisorP::try_from_polynomials(c, polynomial(&u), polynomial(&v)).unwrap()
}

fn check() {
    let c = curve();
    let classes = chain::rational_classes();
    for (i, d) in classes.iter().enumerate() {
        let h = &classes[(7919 * i + 37) % classes.len()];
        let a = convert(d, &c);
        let b = convert(h, &c);
        let got = a.checked_add(&b, &c).unwrap();
        assert_eq!(got, convert(&d.add(h), &c), "class pair {i}");
        assert_eq!(a.checked_neg(&c).unwrap(), convert(&d.neg(), &c));
    }
    println!(
        "PASS differential canonical additions and inverses for all {} F101 classes",
        classes.len()
    );
    println!("Reference: pre-existing crypto::prime_hyperelliptic; no signed independent verdict");
}

fn main() {
    check();
}

#[test]
fn native_vs_existing_biguint_arithmetic() {
    check();
}
