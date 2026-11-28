//! Run65: exact public-shift autocorrelation of the original 509-point Dickson factor base.
//! Arithmetic is the existing prime31_algebraic_ic crate: this is NOT an ECDLP solver.
//! Independent C++ and Python reference results were frozen before this Rust example.
//! cargo run --locked --release --example prime31_run65_shift_correlations
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, PRIME31};
use std::collections::HashSet;

const LAGS: [u64; 5] = [1, 2, 4, 8, 16];
const EXPECTED: [usize; 5] = [124, 124, 136, 114, 96];
const EXPECTED_UNION8: usize = 4_141_238;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    assert_eq!((PRIME31.p, PRIME31.a, PRIME31.b, PRIME31.n),
        (2_147_483_647, 847_514_846, 2_087_444_724, 2_147_446_831));
    let base = Base::build(PRIME31, 10, &mut ar)?;
    assert_eq!(base.points.len(), 509, "depth-10 public base changed");
    let mut signed = Vec::with_capacity(2 * base.points.len());
    for &p in &base.points {
        signed.push(Some(p));
        signed.push(ar.neg(PRIME31, Some(p)));
    }
    let mut support = HashSet::with_capacity(520_000);
    let mut pair_records = 0_usize;
    for i in 0..signed.len() {
        for j in i..signed.len() {
            let sum = ar.add(PRIME31, signed[i], signed[j]);
            assert!(ar.on_curve(PRIME31, sum), "off-curve pair sum");
            support.insert(sum);
            pair_records += 1;
        }
    }
    assert_eq!(pair_records, 518_671);
    assert_eq!(support.len(), 518_085);
    assert!(support.contains(&None));

    for (&lag, &want) in LAGS.iter().zip(EXPECTED.iter()) {
        let delta = ar.scalar(PRIME31, lag, Some(PRIME31.g));
        assert!(ar.on_curve(PRIME31, delta));
        let overlap = support.iter()
            .filter(|&&point| support.contains(&ar.add(PRIME31, point, delta)))
            .count();
        assert_eq!(overlap, want, "public-shift intersection changed at lag={lag}");
        println!("run65 lag={lag} intersection={overlap} exact=PASS");
    }

    let minus_g = ar.neg(PRIME31, Some(PRIME31.g));
    let mut union = HashSet::with_capacity(4_200_000);
    for &point in &support {
        let mut q = point;
        for _ in 0..8 {
            union.insert(q);
            q = ar.add(PRIME31, q, minus_g);
        }
    }
    assert_eq!(union.len(), EXPECTED_UNION8, "eight-consecutive-shift union changed");
    let n = PRIME31.n as f64;
    let density = support.len() as f64 / n;
    let independent = n * (1.0 - (1.0 - density).powi(8));
    let relative = (union.len() as f64 / independent - 1.0) * 100.0;
    println!(
        "run65 size={} eight_shift_union={} independent_eight_expected={independent:.6} \
         relative_delta_pct={relative:.9} exact=PASS \
         classification=public_support_diagnostic_not_relation_extraction",
        support.len(), union.len()
    );
    Ok(())
}
