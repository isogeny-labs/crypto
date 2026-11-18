//! Independent structural verifier for the H116 v2 cohort.
//!
//! Unlike the selector, this verifier counts curve points with Euler's
//! criterion rather than a square-root multiplicity table.

use std::collections::BTreeSet;
use std::fs;

const MIN_P: u64 = 53;
const MAX_P: u64 = 2_000;
const MAX_EXTENSION_SIZE: u128 = 10_000_000_000;
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
    let mut divisor = 3_u64;
    while divisor <= n / divisor {
        if n % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1_u64;
    base %= modulus;
    while exponent != 0 {
        if exponent & 1 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exponent >>= 1;
    }
    result
}

fn multiplicative_order_2_to_6(p: u64, r: u64) -> Option<u64> {
    if p % r == 1 {
        return None;
    }
    (2_u64..=6).find(|&degree| mod_pow(p, degree, r) == 1)
}

fn bounded_power(base: u64, exponent: u64) -> Option<u128> {
    let mut result = 1_u128;
    for _ in 0..exponent {
        result = result.checked_mul(base as u128)?;
        if result > MAX_EXTENSION_SIZE {
            return None;
        }
    }
    Some(result)
}

fn hasse_bound(p: u64) -> u64 {
    let mut bound = 0_u64;
    while (bound + 1) <= 4 * p / (bound + 1) {
        bound += 1;
    }
    bound
}

fn nonsingular(p: u64, a: u64, b: u64) -> bool {
    let modulus = p as u128;
    let a3 = (a as u128 * a as u128 % modulus) * a as u128 % modulus;
    let b2 = b as u128 * b as u128 % modulus;
    (4 * a3 + 27 * b2) % modulus != 0
}

fn point_count_legendre(p: u64, a: u64, b: u64) -> u64 {
    let modulus = p as u128;
    let mut count = 1_u64;
    for x in 0..p {
        let x128 = x as u128;
        let rhs = ((x128 * x128 % modulus) * x128 + a as u128 * x128 + b as u128)
            % modulus;
        if rhs == 0 {
            count += 1;
        } else if mod_pow(rhs as u64, (p - 1) / 2, p) == 1 {
            count += 2;
        }
    }
    count
}

fn excluded_h115(p: u64, a: u64, b: u64) -> bool {
    matches!(
        (p, a, b),
        (7, 1, 1) | (5, 2, 1) | (13, 2, 4) | (31, 1, 19) | (47, 1, 38)
    )
}

fn expected_cohort() -> (Vec<Curve>, u64, u64) {
    let mut selected = Vec::new();
    let mut retained_orders = BTreeSet::new();
    let mut primes_scanned = 0_u64;
    let mut models_scanned = 0_u64;

    for p in MIN_P..=MAX_P {
        if !is_prime(p) {
            continue;
        }
        primes_scanned += 1;
        let center = p + 1;
        let bound = hasse_bound(p);
        let mut possible = Vec::new();
        for order in center.saturating_sub(bound)..=center + bound {
            let trace = center as i128 - order as i128;
            if trace * trace > 4_i128 * p as i128
                || retained_orders.contains(&order)
                || !is_prime(order)
            {
                continue;
            }
            let Some(degree) = multiplicative_order_2_to_6(p, order) else {
                continue;
            };
            let Some(extension_size) = bounded_power(p, degree) else {
                continue;
            };
            possible.push((order, degree, extension_size));
        }
        possible.sort_unstable();
        if possible.is_empty() {
            continue;
        }

        let mut at_p = Vec::new();
        'models: for a in 0..p {
            for b in 0..p {
                models_scanned += 1;
                if excluded_h115(p, a, b) || !nonsingular(p, a, b) {
                    continue;
                }
                let order = point_count_legendre(p, a, b);
                let Some(&(_, degree, extension_size)) = possible
                    .iter()
                    .find(|&&(candidate, _, _)| candidate == order)
                else {
                    continue;
                };
                if at_p.iter().any(|curve: &Curve| curve.order == order) {
                    continue;
                }
                at_p.push(Curve {
                    p,
                    order,
                    torsion_degree: degree,
                    a,
                    b,
                    trace: p as i64 + 1 - order as i64,
                    extension_size,
                });
                if at_p.len() == possible.len() {
                    break 'models;
                }
            }
        }
        at_p.sort_by_key(|curve| (curve.order, curve.torsion_degree, curve.a, curve.b));
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

fn parse_integer(line: &str, key: &str) -> i128 {
    let needle = format!("\"{key}\":");
    let start = line.find(&needle).unwrap_or_else(|| panic!("missing field {key}")) + needle.len();
    let bytes = line.as_bytes();
    let mut end = start;
    if bytes.get(end) == Some(&b'-') {
        end += 1;
    }
    while bytes.get(end).is_some_and(u8::is_ascii_digit) {
        end += 1;
    }
    line[start..end]
        .parse::<i128>()
        .unwrap_or_else(|_| panic!("invalid integer field {key}"))
}

fn parse_cohort(input: &str) -> Vec<Curve> {
    input
        .lines()
        .filter(|line| line.contains("\"slug\""))
        .map(|line| Curve {
            p: parse_integer(line, "p") as u64,
            a: parse_integer(line, "a") as u64,
            b: parse_integer(line, "b") as u64,
            order: parse_integer(line, "order") as u64,
            trace: parse_integer(line, "trace") as i64,
            torsion_degree: parse_integer(line, "torsion_degree") as u64,
            extension_size: parse_integer(line, "extension_size") as u128,
        })
        .collect()
}

fn parse_top_level(input: &str, key: &str) -> u64 {
    input
        .lines()
        .find(|line| line.contains(&format!("\"{key}\"")))
        .map(|line| parse_integer(line, key) as u64)
        .unwrap_or_else(|| panic!("missing top-level field {key}"))
}

fn selftest() {
    assert!(is_prime(587));
    assert!(!is_prime(589));
    assert_eq!(multiplicative_order_2_to_6(107, 127), Some(3));
    assert_eq!(bounded_power(107, 3), Some(1_225_043));
    assert!(nonsingular(107, 1, 25));
    assert_eq!(point_count_legendre(107, 1, 25), 127);
    println!("h116 independent cohort verifier selftest: pass");
}

fn main() {
    let argument = std::env::args().nth(1);
    if argument.as_deref() == Some("--selftest") {
        selftest();
        return;
    }
    let path = argument.expect("usage: h116_cohort_verify COHORT_V2.json");
    let input = fs::read_to_string(path).expect("read cohort");
    assert!(input.contains("\"schema\": \"h116.cohort/v2\""));
    assert!(input.contains("42b510bd0f67d23ad49e5827d947510de5e1c68bf82d29c78fa35e4861fe4bf3"));
    assert!(input.contains("\"generator_labels\": [1,2,3]"));
    assert!(input.contains("\"coordinate_isomorphism_lambda\": 2"));

    let (expected, primes_scanned, models_scanned) = expected_cohort();
    assert_eq!(expected.len(), TARGET_CURVES);
    assert_eq!(parse_cohort(&input), expected);
    assert_eq!(parse_top_level(&input, "primes_scanned"), primes_scanned);
    assert_eq!(parse_top_level(&input, "models_scanned"), models_scanned);
    for curve in &expected {
        let slug = format!(
            "h116-fp{}-a{}-b{}-r{}-d{}",
            curve.p, curve.a, curve.b, curve.order, curve.torsion_degree
        );
        assert!(input.contains(&slug));
    }
    println!(
        "{{\"schema\":\"h116.cohort-verification/v1\",\"status\":\"pass\",\"curves\":{},\"primes_scanned\":{},\"models_scanned\":{}}}",
        expected.len(), primes_scanned, models_scanned
    );
}
