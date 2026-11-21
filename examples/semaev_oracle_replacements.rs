//! Exact experiment for projection-only and compiled-target Semaev oracles.
//!
//! Protocol: `research/semaev_oracle_replacements_20261117/PROTOCOL.md`.
//! The projection arm is independently checked with signed elliptic-curve
//! group additions.  The compression arm interpolates the complete online
//! target function and tests whether a recurrence learned on a prefix
//! predicts an untouched suffix.

use crypto_lib::cryptanalysis::pkm_smooth_tower::{
    addm, invm, is_prime, mulm, sqrtm, subm, Curve, Pt,
};
use crypto_lib::cryptanalysis::semaev_projection::{
    interval_factor_base, PairProjection, QueryStats,
};
use serde::Serialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Serialize)]
struct PolynomialProfile {
    degree: Option<usize>,
    nonzero_coefficients: usize,
    density: f64,
    exact_replay: bool,
}

#[derive(Clone, Debug, Serialize)]
struct RecurrenceProfile {
    train_values: usize,
    order: usize,
    holdout_values: usize,
    holdout_correct: usize,
}

fn eval_poly(coefficients: &[u64], x: u64, p: u64) -> u64 {
    coefficients
        .iter()
        .rev()
        .fold(0, |acc, &coefficient| addm(mulm(acc, x, p), coefficient, p))
}

/// Interpolate a function given at every element `0..p-1`.
///
/// Over `F_p`, the indicator at `a` is `1 - (X-a)^(p-1)`.  Since
/// `binom(p-1,k) = (-1)^k`, its nonconstant coefficient at degree `k` is
/// `-a^(p-1-k)`.  This gives an exact O(p^2) compiler without solving a
/// Vandermonde system.
fn interpolate_complete(values: &[u64], p: u64) -> Vec<u64> {
    assert_eq!(values.len(), p as usize);
    let mut coefficients = vec![0u64; p as usize];
    for (a, &value) in values.iter().enumerate() {
        let value = value % p;
        if value == 0 {
            continue;
        }
        let a = a as u64;
        for (k, coefficient) in coefficients.iter_mut().enumerate() {
            let power = crypto_lib::cryptanalysis::pkm_smooth_tower::powm(a, p - 1 - k as u64, p);
            let delta = if k == 0 {
                subm(1, power, p)
            } else {
                subm(0, power, p)
            };
            *coefficient = addm(*coefficient, mulm(value, delta, p), p);
        }
    }
    coefficients
}

fn polynomial_profile(values: &[u64], p: u64) -> PolynomialProfile {
    let coefficients = interpolate_complete(values, p);
    let degree = coefficients
        .iter()
        .rposition(|&coefficient| coefficient != 0);
    let nonzero_coefficients = coefficients
        .iter()
        .filter(|&&coefficient| coefficient != 0)
        .count();
    PolynomialProfile {
        degree,
        nonzero_coefficients,
        density: nonzero_coefficients as f64 / p as f64,
        exact_replay: values
            .iter()
            .enumerate()
            .all(|(x, &value)| eval_poly(&coefficients, x as u64, p) == value % p),
    }
}

/// Berlekamp--Massey. The result is `C` with
/// `s[n] + sum(C[i]*s[n-i], i=1..L) = 0` on the training prefix.
fn berlekamp_massey(sequence: &[u64], p: u64) -> Vec<u64> {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let mut length = 0usize;
    let mut shift = 1usize;
    let mut last_discrepancy = 1u64;
    for n in 0..sequence.len() {
        let mut discrepancy = sequence[n] % p;
        for i in 1..=length {
            discrepancy = addm(discrepancy, mulm(c[i], sequence[n - i] % p, p), p);
        }
        if discrepancy == 0 {
            shift += 1;
            continue;
        }
        let previous = c.clone();
        let scale = mulm(discrepancy, invm(last_discrepancy, p), p);
        if c.len() < b.len() + shift {
            c.resize(b.len() + shift, 0);
        }
        for (i, &coefficient) in b.iter().enumerate() {
            c[i + shift] = subm(c[i + shift], mulm(scale, coefficient, p), p);
        }
        if 2 * length <= n {
            length = n + 1 - length;
            b = previous;
            last_discrepancy = discrepancy;
            shift = 1;
        } else {
            shift += 1;
        }
    }
    c.resize(length + 1, 0);
    c
}

fn recurrence_profile(values: &[u64], p: u64) -> RecurrenceProfile {
    let train_values = values.len() * 2 / 3;
    let recurrence = berlekamp_massey(&values[..train_values], p);
    let order = recurrence.len().saturating_sub(1);
    let holdout_correct = (train_values..values.len())
        .filter(|&n| {
            if n < order {
                return false;
            }
            let sum = (1..=order).fold(0, |acc, i| {
                addm(acc, mulm(recurrence[i], values[n - i] % p, p), p)
            });
            subm(0, sum, p) == values[n] % p
        })
        .count();
    RecurrenceProfile {
        train_values,
        order,
        holdout_values: values.len() - train_values,
        holdout_correct,
    }
}

fn signed_points(curve: &Curve, x: u64) -> Vec<Pt> {
    let y = sqrtm(curve.rhs(x), curve.p).expect("factor-base point");
    let point = Some((x, y));
    let negated = curve.neg(point);
    if point == negated {
        vec![point]
    } else {
        vec![point, negated]
    }
}

/// Independent exact control: build reachable x-coordinates with the group
/// law, without evaluating a summation polynomial or consulting the join.
fn group_control(curve: &Curve, base: &[u64]) -> (HashSet<u64>, HashSet<u64>) {
    let signed: Vec<Pt> = base.iter().flat_map(|&x| signed_points(curve, x)).collect();
    let mut pair_sums = HashSet::new();
    for &left in &signed {
        for &right in &signed {
            pair_sums.insert(curve.add(left, right));
        }
    }
    let mut three = HashSet::new();
    for &pair in &pair_sums {
        for &point in &signed {
            if let Some((x, _)) = curve.add(pair, point) {
                three.insert(x);
            }
        }
    }
    let mut four = HashSet::new();
    for &left in &pair_sums {
        for &right in &pair_sums {
            if let Some((x, _)) = curve.add(left, right) {
                four.insert(x);
            }
        }
    }
    (three, four)
}

fn tuple_reaches_target(curve: &Curve, xs: &[u64], target: u64) -> bool {
    fn visit(curve: &Curve, xs: &[u64], target: u64, depth: usize, sum: Pt) -> bool {
        if depth == xs.len() {
            return sum.is_some_and(|(x, _)| x == target);
        }
        signed_points(curve, xs[depth])
            .into_iter()
            .any(|point| visit(curve, xs, target, depth + 1, curve.add(sum, point)))
    }
    visit(curve, xs, target, 0, None)
}

fn add_query(total: &mut QueryStats, value: QueryStats) {
    total.quadratics += value.quadratics;
    total.root_keys += value.root_keys;
    total.lookups += value.lookups;
    total.compatible_keys += value.compatible_keys;
    total.cancellation_checks += value.cancellation_checks;
}

fn parse_args() -> HashMap<String, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut parsed = HashMap::new();
    let mut i = 0;
    while i < args.len() {
        let key = args[i].trim_start_matches("--").to_string();
        let value = args.get(i + 1).cloned().unwrap_or_default();
        parsed.insert(key, value);
        i += 2;
    }
    parsed
}

fn main() -> Result<(), String> {
    let args = parse_args();
    let value = |key: &str, default: &str| {
        args.get(key)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    };
    let p: u64 = value("p", "251").parse().map_err(|_| "invalid p")?;
    let a: u64 = value("a", "2").parse().map_err(|_| "invalid a")?;
    let b: u64 = value("b", "3").parse().map_err(|_| "invalid b")?;
    let bounds: Vec<u64> = value("bounds", "8,16,24,32")
        .split(',')
        .map(|part| part.parse().map_err(|_| format!("invalid bound {part}")))
        .collect::<Result<_, _>>()?;
    if !is_prime(p) || p <= 2 {
        return Err("p must be an odd prime".into());
    }
    let curve = Curve {
        p,
        a: a % p,
        b: b % p,
    };
    if curve.is_singular() {
        return Err("singular curve".into());
    }

    for bound in bounds {
        let base = interval_factor_base(&curve, bound);
        let projection = PairProjection::build(curve, &base)?;
        let (control_three, control_four) = group_control(&curve, &base);
        let mut existence = Vec::with_capacity(p as usize);
        let mut compatibility = Vec::with_capacity(p as usize);
        let mut three_mismatches = 0usize;
        let mut four_mismatches = 0usize;
        let mut three_bad_witnesses = 0usize;
        let mut four_bad_witnesses = 0usize;
        let mut three_total = QueryStats::default();
        let mut four_total = QueryStats::default();
        for target in 0..p {
            let (three, three_stats) = projection.decompose_three(target);
            let (four, four_stats) = projection.decompose_four(target);
            three_mismatches += usize::from(three.is_some() != control_three.contains(&target));
            four_mismatches += usize::from(four.is_some() != control_four.contains(&target));
            three_bad_witnesses += three
                .as_ref()
                .is_some_and(|witness| !tuple_reaches_target(&curve, &witness.x, target))
                as usize;
            four_bad_witnesses += four
                .as_ref()
                .is_some_and(|witness| !tuple_reaches_target(&curve, &witness.x, target))
                as usize;
            existence.push(u64::from(four.is_some()));
            compatibility.push(four_stats.compatible_keys % p);
            add_query(&mut three_total, three_stats);
            add_query(&mut four_total, four_stats);
        }

        println!(
            "{}",
            json!({
                "schema": "semaev-oracle-replacements/v1",
                "p": p,
                "curve": {"a": curve.a, "b": curve.b},
                "interval_bound": bound,
                "factor_base_size": base.len(),
                "build": projection.build_stats(),
                "independent_group_control": {
                    "three_reachable_x": control_three.len(),
                    "four_reachable_x": control_four.len(),
                    "three_mismatches": three_mismatches,
                    "four_mismatches": four_mismatches,
                    "three_bad_witnesses": three_bad_witnesses,
                    "four_bad_witnesses": four_bad_witnesses,
                    "passed": three_mismatches == 0 && four_mismatches == 0
                        && three_bad_witnesses == 0 && four_bad_witnesses == 0
                },
                "online_totals_all_targets": {
                    "three": three_total,
                    "four": four_total
                },
                "compiled_target_function": {
                    "four_sum_existence": {
                        "interpolant": polynomial_profile(&existence, p),
                        "recurrence": recurrence_profile(&existence, p)
                    },
                    "finite_root_compatibility_count_mod_p": {
                        "interpolant": polynomial_profile(&compatibility, p),
                        "recurrence": recurrence_profile(&compatibility, p)
                    }
                },
                "cost_class": "exact projection join; O(B^2) build/table and O(min(p,B^2)) four-sum query",
                "claim_scope": "toy oracle-stage experiment; no ECDLP or rho speedup claim"
            })
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_interpolation_replays_arbitrary_function() {
        let p = 101;
        let values: Vec<_> = (0..p)
            .map(|x| (x * x * x + 17 * x + u64::from(x % 7 == 0)) % p)
            .collect();
        assert!(polynomial_profile(&values, p).exact_replay);
    }

    #[test]
    fn berlekamp_massey_recovers_a_recurrence() {
        let p = 101;
        let mut values = vec![3, 7];
        while values.len() < p as usize {
            let n = values.len();
            values.push(addm(
                mulm(2, values[n - 1], p),
                mulm(5, values[n - 2], p),
                p,
            ));
        }
        let profile = recurrence_profile(&values, p);
        assert_eq!(profile.order, 2);
        assert_eq!(profile.holdout_correct, profile.holdout_values);
    }
}
