//! Deterministic structural cohort selector for H116.
//!
//! This program does not construct a Frobenius fiber, evaluate a moment,
//! inspect a scalar label, score a predicate, or take a timing measurement.

use std::collections::BTreeSet;
use std::fmt::Write as _;

const MIN_P: u64 = 53;
const MAX_P: u64 = 2_000;
const MAX_EXTENSION_SIZE: u128 = 100_000_000;
const TARGET_CURVES: usize = 5;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Curve {
    p: u64,
    order: u64,
    torsion_degree: u64,
    a: u64,
    b: u64,
    trace: i64,
    extension_size: u128,
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3_u64;
    while d <= n / d {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1_u64;
    base %= modulus;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exponent >>= 1;
    }
    result
}

fn torsion_degree(p: u64, r: u64) -> Option<u64> {
    // The frozen cohort requires the *multiplicative order* to lie in 2..=6.
    // Without this guard an order-one residue would also satisfy p^2 = 1 and
    // be misreported as degree two.
    if p % r == 1 {
        return None;
    }
    (2_u64..=6).find(|&d| mod_pow(p, d, r) == 1)
}

fn bounded_pow(base: u64, exponent: u64) -> Option<u128> {
    let mut value = 1_u128;
    for _ in 0..exponent {
        value = value.checked_mul(base as u128)?;
        if value > MAX_EXTENSION_SIZE {
            return None;
        }
    }
    Some(value)
}

fn nonsingular(p: u64, a: u64, b: u64) -> bool {
    let p128 = p as u128;
    let a3 = (a as u128 * a as u128 % p128) * a as u128 % p128;
    let b2 = b as u128 * b as u128 % p128;
    (4_u128 * a3 + 27_u128 * b2) % p128 != 0
}

fn square_root_counts(p: u64) -> Vec<u64> {
    let mut counts = vec![0_u64; p as usize];
    for y in 0..p {
        let square = ((y as u128 * y as u128) % p as u128) as usize;
        counts[square] += 1;
    }
    counts
}

fn point_count(p: u64, a: u64, b: u64, roots: &[u64]) -> u64 {
    let p128 = p as u128;
    let mut total = 1_u64; // point at infinity
    for x in 0..p {
        let x128 = x as u128;
        let rhs = (x128 * x128 % p128 * x128 + a as u128 * x128 + b as u128)
            % p128;
        total += roots[rhs as usize];
    }
    total
}

fn excluded_h115(p: u64, a: u64, b: u64) -> bool {
    matches!(
        (p, a, b),
        (7, 1, 1) | (5, 2, 1) | (13, 2, 4) | (31, 1, 19) | (47, 1, 38)
    )
}

fn select() -> (Vec<Curve>, u64, u64) {
    let mut selected = Vec::new();
    let mut retained_orders = BTreeSet::new();
    let mut primes_scanned = 0_u64;
    let mut models_scanned = 0_u64;

    for p in MIN_P..=MAX_P {
        if !is_prime(p) {
            continue;
        }
        primes_scanned += 1;
        let roots = square_root_counts(p);
        let mut at_p = Vec::new();
        for a in 0..p {
            for b in 0..p {
                models_scanned += 1;
                if excluded_h115(p, a, b) || !nonsingular(p, a, b) {
                    continue;
                }
                let order = point_count(p, a, b, &roots);
                if !is_prime(order) {
                    continue;
                }
                let Some(d) = torsion_degree(p, order) else {
                    continue;
                };
                let Some(extension_size) = bounded_pow(p, d) else {
                    continue;
                };
                at_p.push(Curve {
                    p,
                    order,
                    torsion_degree: d,
                    a,
                    b,
                    trace: p as i64 + 1 - order as i64,
                    extension_size,
                });
            }
        }
        at_p.sort_by_key(|curve| {
            (
                curve.order,
                curve.torsion_degree,
                curve.a,
                curve.b,
            )
        });
        for curve in at_p {
            if retained_orders.insert(curve.order) {
                selected.push(curve);
                if selected.len() == TARGET_CURVES {
                    return (selected, primes_scanned, models_scanned);
                }
            }
        }
    }
    (selected, primes_scanned, models_scanned)
}

fn json(selected: &[Curve], primes_scanned: u64, models_scanned: u64) -> String {
    let mut output = String::new();
    writeln!(&mut output, "{{").unwrap();
    writeln!(&mut output, "  \"schema\": \"h116.cohort/v1\",").unwrap();
    writeln!(
        &mut output,
        "  \"rule\": \"COHORT_RULE.json at protocol freeze e1638152e26e31c5274828b9ed268d55c1cf2a7a489d0c98e43f69823539803b\","
    )
    .unwrap();
    writeln!(&mut output, "  \"primes_scanned\": {primes_scanned},").unwrap();
    writeln!(&mut output, "  \"models_scanned\": {models_scanned},").unwrap();
    writeln!(&mut output, "  \"curves\": [").unwrap();
    for (index, curve) in selected.iter().enumerate() {
        let comma = if index + 1 == selected.len() { "" } else { "," };
        writeln!(
            &mut output,
            "    {{\"slug\":\"h116-fp{}-a{}-b{}-r{}-d{}\",\"p\":{},\"a\":{},\"b\":{},\"order\":{},\"trace\":{},\"torsion_degree\":{},\"extension_size\":{}}}{}",
            curve.p,
            curve.a,
            curve.b,
            curve.order,
            curve.torsion_degree,
            curve.p,
            curve.a,
            curve.b,
            curve.order,
            curve.trace,
            curve.torsion_degree,
            curve.extension_size,
            comma
        )
        .unwrap();
    }
    writeln!(&mut output, "  ],").unwrap();
    writeln!(&mut output, "  \"generator_labels\": [1,2,3],").unwrap();
    writeln!(&mut output, "  \"coordinate_isomorphism_lambda\": 2").unwrap();
    writeln!(&mut output, "}}").unwrap();
    output
}

fn selftest() {
    assert!(is_prime(2));
    assert!(is_prime(61));
    assert!(!is_prime(1));
    assert!(!is_prime(91));
    assert_eq!(torsion_degree(11, 5), None);
    assert_eq!(torsion_degree(31, 37), Some(4));
    assert_eq!(bounded_pow(31, 4), Some(923_521));
    let roots = square_root_counts(31);
    assert_eq!(point_count(31, 1, 19, &roots), 37);
    assert!(nonsingular(31, 1, 19));
    assert!(excluded_h115(31, 1, 19));
    println!("h116 selector selftest: pass");
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--selftest") {
        selftest();
        return;
    }
    let (selected, primes_scanned, models_scanned) = select();
    assert_eq!(
        selected.len(),
        TARGET_CURVES,
        "selection rule found fewer than five curves"
    );
    print!("{}", json(&selected, primes_scanned, models_scanned));
}
