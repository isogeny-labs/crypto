//! Exact finite counting gate behind the Curve25519 subexponential pivot.
//!
//! The pair-split model is the standard uniform-occupancy surrogate
//! `B^2 + ceil(r/B)` before linear algebra.  The coverage bound is rigorous:
//! at arity at most `m`, no more than `sum_{j=0}^m B^j` residues can be hit.
//! This program is a cost certificate, not an ECDLP solver.

use hashed_ecdl_tensor::BigUint;
use num_traits::{One, Zero};
use serde::Serialize;

const CURVE25519_SUBGROUP_TAIL: &str = "27742317777372353535851937790883648493";

fn ceil_div(n: &BigUint, d: &BigUint) -> BigUint {
    assert!(!d.is_zero());
    (n + d - BigUint::one()) / d
}

fn coverage_cap(order: &BigUint, base: &BigUint, arity: u32) -> BigUint {
    let mut sum = BigUint::one();
    let mut power = BigUint::one();
    for _ in 0..arity {
        power *= base;
        sum += &power;
        if sum >= *order {
            return order.clone();
        }
    }
    sum
}

/// Lower bound on uniform target attempts needed to obtain `B` successes.
fn target_attempts_lower(order: &BigUint, base: &BigUint, arity: u32) -> BigUint {
    let covered = coverage_cap(order, base, arity);
    ceil_div(&(base * order), &covered)
}

fn pair_split_work(order: &BigUint, base: &BigUint) -> BigUint {
    base * base + ceil_div(order, base)
}

fn ceil_sqrt(n: &BigUint) -> BigUint {
    if n <= &BigUint::one() {
        return n.clone();
    }
    let mut x = BigUint::one() << n.bits().div_ceil(2) as usize;
    loop {
        let next = (&x + n / &x) >> 1usize;
        if next >= x {
            break;
        }
        x = next;
    }
    if &x * &x < *n {
        x + BigUint::one()
    } else {
        x
    }
}

fn best_power_of_two_pair_split(order: &BigUint) -> (u64, BigUint, BigUint) {
    let mut best = None;
    for k in 0..order.bits() {
        let base = BigUint::one() << k as usize;
        if base > *order {
            break;
        }
        let work = pair_split_work(order, &base);
        let improves = match &best {
            Some((_, _, previous)) => work < *previous,
            None => true,
        };
        if improves {
            best = Some((k, base, work));
        }
    }
    best.expect("positive group order")
}

#[derive(Serialize)]
struct GateRow {
    id: &'static str,
    order: String,
    order_bits: u64,
    pollard_sqrt_ceiling: String,
    pair_split_best_base_power: u64,
    pair_split_best_base: String,
    pair_split_work_before_linear_algebra: String,
    pair_split_work_bits: u64,
    pair_split_over_sqrt_ceiling: String,
    fixed_arity_probe_base: String,
    fixed_arity_probe_m3_coverage: String,
    fixed_arity_probe_m3_target_attempts_lower: String,
    fixed_arity_probe_m3_target_attempts_lower_bits: u64,
}

fn row(id: &'static str, order: BigUint, probe_base: BigUint) -> GateRow {
    let (power, base, work) = best_power_of_two_pair_split(&order);
    let sqrt = ceil_sqrt(&order);
    let coverage = coverage_cap(&order, &probe_base, 3);
    let attempts = target_attempts_lower(&order, &probe_base, 3);
    GateRow {
        id,
        order: order.to_string(),
        order_bits: order.bits(),
        pollard_sqrt_ceiling: sqrt.to_string(),
        pair_split_best_base_power: power,
        pair_split_best_base: base.to_string(),
        pair_split_work_before_linear_algebra: work.to_string(),
        pair_split_work_bits: work.bits(),
        pair_split_over_sqrt_ceiling: ceil_div(&work, &sqrt).to_string(),
        fixed_arity_probe_base: probe_base.to_string(),
        fixed_arity_probe_m3_coverage: coverage.to_string(),
        fixed_arity_probe_m3_target_attempts_lower_bits: attempts.bits(),
        fixed_arity_probe_m3_target_attempts_lower: attempts.to_string(),
    }
}

fn curve25519_order() -> BigUint {
    (BigUint::one() << 252usize)
        + BigUint::parse_bytes(CURVE25519_SUBGROUP_TAIL.as_bytes(), 10)
            .expect("valid subgroup tail")
}

fn main() {
    let report = serde_json::json!({
        "schema": "curve25519-subex-counting-gate/v1",
        "scope": "exact coverage bound plus uniform-occupancy m=3 pair-split surrogate; no ECDLP speed claim",
        "rows": [
            row("curve25519_family_miniature", BigUint::from(131_203u64), BigUint::from(19u8)),
            row("curve25519_prime_subgroup", curve25519_order(), BigUint::one() << 16usize)
        ]
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage_is_capped_by_the_group() {
        let order = BigUint::from(1_009u16);
        assert_eq!(coverage_cap(&order, &BigUint::from(10u8), 2), 111u8.into());
        assert_eq!(coverage_cap(&order, &BigUint::from(10u8), 3), order);
    }

    #[test]
    fn target_attempt_bound_uses_requested_row_count() {
        let order = BigUint::from(101u8);
        let base = BigUint::from(2u8);
        // Coverage is 1+2+4+8=15, so ceil(2*101/15)=14.
        assert_eq!(target_attempts_lower(&order, &base, 3), 14u8.into());
    }

    #[test]
    fn curve25519_pair_split_optimum_is_at_two_to_84() {
        let order = curve25519_order();
        assert_eq!(order.bits(), 253);
        assert_eq!(best_power_of_two_pair_split(&order).0, 84);
    }

    #[test]
    fn integer_square_root_rounds_up() {
        assert_eq!(ceil_sqrt(&16u8.into()), 4u8.into());
        assert_eq!(ceil_sqrt(&17u8.into()), 5u8.into());
    }
}
