//! Minimal library use of the EFD representation layer.
//!
//! ```sh
//! cargo run --release --example efd_ic
//! ```
//!
//! The example builds a Montgomery curve in the `xz` chart, decodes its
//! generator, runs the prime-field index-calculus measurement on the derived
//! short-Weierstrass curve, and encodes a target back in the chart.

use crypto_lib::cryptanalysis::efd_representations::EfdRepresentation;
use crypto_lib::cryptanalysis::prime_small_x::{measure, MontField, Options};
use crypto_lib::ecc::field::FieldElement;
use crypto_lib::prime_ecc::efd::Bindings;
use num_bigint::BigUint;

fn fe(value: u64, p: &BigUint) -> FieldElement {
    FieldElement::new(BigUint::from(value), p.clone())
}

fn main() -> Result<(), String> {
    let p = BigUint::from(1019u32);
    let params = Bindings::from([("a".to_owned(), fe(131, &p)), ("b".to_owned(), fe(1, &p))]);

    // Montgomery curve b y^2 = x^3 + a x^2 + x, EFD xz chart.
    let representation = EfdRepresentation::new("montgom", "xz", &p, &params)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&representation.describe()).unwrap()
    );

    let field = MontField::<3>::new(&p)?;
    let curve = representation.curve(
        field,
        "EFD montgom/xz over F_1019".into(),
        BigUint::from(269u32),
        true,
        4,
    )?;

    // X/Z = 604 is the x-coordinate of the published generator.
    let generator = representation.decode_point(&curve.field, &[fe(604, &p), fe(1, &p)], 0)?;
    let mut field_copy = curve.field.clone();
    assert!(curve.on_curve(&mut field_copy, &generator));

    let report = measure(
        &curve,
        &Options {
            size: 6,
            trials: 200,
            controls: 2,
            seed: 1,
        },
    )?;
    println!(
        "exact yield {:.6}, controls {}/{}",
        report.exact_yield, report.controls_recovered, report.controls
    );

    let target = curve
        .mul_big(&mut field_copy, &generator, &BigUint::from(137u32))
        .ok_or("137*P is infinity")?;
    let encoded = representation.encode_point(&curve.field, &target)?;
    println!("137*P in xz = ({}, {})", encoded[0].value, encoded[1].value);
    Ok(())
}
