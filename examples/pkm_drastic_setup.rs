//! Demonstrate the drastic *setup* reduction from a supersingular PKM
//! auxiliary curve at the NIST P-192 prime.
//!
//! Petit--Kosters--Messeng's isogeny factor base needs an auxiliary curve
//! over the same field with a smooth subgroup; it does not require the
//! auxiliary curve to be ordinary.  For
//!
//! ```text
//! p = 2^192 - 2^64 - 1,
//! E': y^2 = x^3 + x,
//! #E'(F_p) = p + 1 = 2^64 (2^128 - 1),
//! ```
//!
//! a point of exact order `2^64` is obtained by one deterministic draw.  The
//! example truncates that point to order `2^16`, constructs the actual chain
//! of 16 degree-2 Vélu maps, and verifies its full `2^16`-element fibre.
//!
//! The comparison uses the repository's preregistered P-192 ordinary-search
//! measurement: the `X_1(2,8)` family succeeds at height `t` with probability
//! `2^(3-t)` and costs 10.06 microseconds per draw.  Thus `t = 48` needs
//! `2^45` draws in expectation, about 11.2 core-years.  This is a setup
//! comparison, not a new relation-solver or ECDLP complexity claim.

use std::collections::HashSet;
use std::time::Instant;

use crypto_lib::cryptanalysis::fp_backend::{FpP192, PrimeField};
use crypto_lib::cryptanalysis::two_sylow_search::{
    big_double, build_tower, find_p192_supersingular_point, p192, tower_eval,
    P192SupersingularPoint, Sw,
};
use num_bigint::BigUint;
use num_traits::{One, Zero};

const SOURCE_T: u32 = 64;
const DEMO_T: u32 = 16;
const TOWER_SEED: u64 = 1;
const ORDINARY_COMPARISON_T: u32 = 48;
const ORDINARY_US_PER_DRAW: f64 = 10.06;

#[derive(Debug)]
struct DemoResult {
    point: P192SupersingularPoint,
    point_seconds: f64,
    tower_seconds: f64,
    tower_height: u32,
    fibre_size: usize,
    lifting_abscissae: usize,
    ordinary_expected_draws: u128,
    ordinary_core_years: f64,
}

fn has_exact_pow2_order(
    p: &BigUint,
    a: &BigUint,
    b: &BigUint,
    qx: &BigUint,
    qy: &BigUint,
    t: u32,
) -> bool {
    if (qy * qy) % p != (qx * qx * qx + a * qx + b) % p {
        return false;
    }
    let mut q = Some((qx.clone(), qy.clone()));
    for _ in 1..t {
        q = big_double(p, a, &q);
        if q.is_none() {
            return false;
        }
    }
    big_double(p, a, &q).is_none()
}

fn run_demo(tower_height: u32) -> DemoResult {
    assert!((1..=20).contains(&tower_height));
    let p = p192();
    let a = BigUint::one();
    let b = BigUint::zero();

    let odd_cofactor = (&p + 1u32) >> SOURCE_T;
    assert_eq!(odd_cofactor, (BigUint::one() << 128u32) - 1u32);

    let point_start = Instant::now();
    let point = find_p192_supersingular_point(SOURCE_T, 0, 32)
        .expect("valid supersingular P-192 parameters")
        .expect("the frozen seed panel contains an exact-order point");
    let point_seconds = point_start.elapsed().as_secs_f64();
    assert!(has_exact_pow2_order(
        &p, &a, &b, &point.qx, &point.qy, SOURCE_T
    ));

    let mut q = Some((point.qx.clone(), point.qy.clone()));
    for _ in tower_height..SOURCE_T {
        q = big_double(&p, &a, &q);
    }
    let (qx, qy) = q.expect("truncated point is nonzero");
    assert!(has_exact_pow2_order(&p, &a, &b, &qx, &qy, tower_height));

    let field = FpP192::new();
    let auxiliary = Sw {
        f: field.clone(),
        a: field.one(),
        b: field.zero(),
    };
    let tower_start = Instant::now();
    let tower = build_tower(
        &auxiliary,
        (field.from_biguint(&qx), field.from_biguint(&qy)),
        tower_height,
        TOWER_SEED,
    )
    .expect("the truncated point builds a Vélu tower");
    let tower_seconds = tower_start.elapsed().as_secs_f64();

    let fibre: HashSet<_> = tower.v.iter().copied().collect();
    let expected_size = 1usize << tower_height;
    assert_eq!(tower.steps.len(), tower_height as usize);
    assert_eq!(tower.v.len(), expected_size);
    assert_eq!(fibre.len(), expected_size);
    assert!(tower
        .v
        .iter()
        .all(|&x| tower_eval(&field, &tower.steps, x) == Some(tower.c)));

    // Count the auxiliary fibre abscissae that lift to the actual NIST P-192
    // curve. These are the PKM factor-base columns before sign handling.
    let p192_b = BigUint::parse_bytes(b"64210519e59c80e70fa7e9ab72243049feb8deecc146b9b1", 16)
        .expect("P-192 b");
    let target_a = field.neg(field.from_u64(3));
    let target_b = field.from_biguint(&p192_b);
    let lifting_abscissae = tower
        .v
        .iter()
        .filter(|&&x| {
            let rhs = field.add(field.mul(field.add(field.sqr(x), target_a), x), target_b);
            field.legendre(rhs) >= 0
        })
        .count();

    let ordinary_expected_draws = 1u128 << (ORDINARY_COMPARISON_T - 3);
    let ordinary_seconds = ordinary_expected_draws as f64 * ORDINARY_US_PER_DRAW * 1e-6;
    let ordinary_core_years = ordinary_seconds / (365.25 * 24.0 * 60.0 * 60.0);

    DemoResult {
        point,
        point_seconds,
        tower_seconds,
        tower_height,
        fibre_size: fibre.len(),
        lifting_abscissae,
        ordinary_expected_draws,
        ordinary_core_years,
    }
}

fn main() {
    let result = run_demo(DEMO_T);
    let p = p192();
    let odd_cofactor = (&p + 1u32) >> SOURCE_T;

    println!("PKM drastic auxiliary-curve setup (NIST P-192)");
    println!("  p                 = {p}");
    println!("  auxiliary curve   = y^2 = x^3 + x (supersingular)");
    println!("  #E'(F_p)          = 2^64 * {odd_cofactor}");
    println!("  exact point order = 2^{}", result.point.t);
    println!("  Qx                = {}", result.point.qx);
    println!("  Qy                = {}", result.point.qy);
    println!(
        "  deterministic draw = seed {}, draw {} ({:.3} ms, locally)",
        result.point.seed,
        result.point.draws,
        result.point_seconds * 1e3
    );
    println!();
    println!("Ordinary auxiliary-curve route at t=48 (committed measurement)");
    println!(
        "  expected curves   = 2^45 = {}",
        result.ordinary_expected_draws
    );
    println!("  measured/draw      = {ORDINARY_US_PER_DRAW:.2} us");
    println!(
        "  expected search    = {:.2} core-years",
        result.ordinary_core_years
    );
    println!();
    println!("Supersingular PKM tower actually constructed");
    println!("  Vélu steps         = {}", result.tower_height);
    println!("  |V|                = {}", result.fibre_size);
    println!("  all L(x)=c         = yes ({} checked)", result.fibre_size);
    println!(
        "  lift to P-192      = {} factor-base abscissae",
        result.lifting_abscissae
    );
    println!(
        "  tower build        = {:.3} s (locally)",
        result.tower_seconds
    );
    println!();
    println!(
        "Conclusion: the auxiliary search drops from an expected 2^45 ordinary curves at t=48 to one deterministic supersingular draw, while producing a checked 2^16 PKM Vélu fibre. This demonstrates the setup reduction only."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_point_has_exact_order_2_64() {
        let point = find_p192_supersingular_point(64, 0, 32).unwrap().unwrap();
        assert_eq!(point.seed, 0);
        assert_eq!(point.draws, 1);
        assert_eq!(
            point.qx.to_string(),
            "540754667722598565332377032173227842192984538550307798180"
        );
        assert!(has_exact_pow2_order(
            &p192(),
            &BigUint::one(),
            &BigUint::zero(),
            &point.qx,
            &point.qy,
            64
        ));
    }

    #[test]
    fn builds_and_checks_a_real_velu_fibre() {
        let result = run_demo(8);
        assert_eq!(result.tower_height, 8);
        assert_eq!(result.fibre_size, 256);
        assert!(result.lifting_abscissae > 0);
        assert_eq!(result.ordinary_expected_draws, 1u128 << 45);
        assert!((11.0..11.5).contains(&result.ordinary_core_years));
    }
}
