//! Exact Euclidean reciprocity floor sums and paired hash cancellation (`REPORT.md` §1).
//!
//! For two Bochum affine hash slots sharing prime modulus `sigma` in `[2p, 4p]`,
//! testing `F_1 ∩ (-F_2)` at the identity target reduces (per ordinate parity
//! `e in {0, 1}`) to finding all `t in [0, tau_1)` such that
//! `(a_rel * t + d_e) mod sigma < tau_2`.
//!
//! Counting raw hits on any subinterval `[L, R)` is the difference of two
//! lattice floor sums, evaluated in `O(log sigma)` steps via Euclidean
//! reciprocity. Recursive interval splitting visits only nonempty subintervals,
//! giving deterministic `poly(log p) * (1 + C)` bit operations where `C` is the
//! number of raw hash intersections (`E[C] = O(1)` at the two-slot threshold).

use crate::arithmetic::{inverse, RandomSource};
use crate::curve::{Curve, Point};
use crate::reduction::FactorBase;
use crate::{integrity, invalid, overflow, BigInt, BigUint, Meter, Result};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

/// Compute the exact lattice floor sum `S(n; a, d, v) = sum_{t = 0}^{n - 1} floor((a * t + d) / v)`
/// by iterative Euclidean reciprocity (`REPORT.md`, "Exact cancellation by floor sums").
///
/// Handles arbitrary signed `a` and `d` using mathematical floor division (`div_mod_floor`)
/// and positive modulus `v > 0`, terminating in `O(log min(|a mod v|, v))` steps.
pub fn floor_sum(
    n: &BigUint,
    a: &BigInt,
    d: &BigInt,
    v: &BigUint,
    meter: &mut Meter,
) -> Result<BigInt> {
    if v.is_zero() {
        return Err(invalid("floor_sum modulus must be positive"));
    }
    let width = n
        .bits()
        .max(a.bits())
        .max(d.bits())
        .max(v.bits())
        .saturating_add(2);
    meter.bits(
        usize::try_from(width).map_err(|_| overflow("floor_sum bit width"))?,
        "floor_sum arithmetic width",
    )?;
    if n.is_zero() {
        return Ok(BigInt::zero());
    }

    let mut cur_n = BigInt::from(n.clone());
    let mut cur_a = a.clone();
    let mut cur_d = d.clone();
    let mut cur_v = BigInt::from(v.clone());
    let mut sign = BigInt::one();
    let mut total = BigInt::zero();
    let one = BigInt::one();

    while cur_n > BigInt::zero() {
        meter.charge(1, "floor_sum euclidean step")?;
        let (q_a, r_a) = cur_a.div_mod_floor(&cur_v);
        let (q_d, r_d) = cur_d.div_mod_floor(&cur_v);

        let n_minus_one = &cur_n - &one;
        let triangular = (&cur_n * &n_minus_one) >> 1usize;
        let quotient_term = &q_a * triangular + &q_d * &cur_n;
        total += &sign * quotient_term;

        if r_a.is_zero() {
            break;
        }
        let y = (&r_a * &n_minus_one + &r_d).div_floor(&cur_v);
        if y.is_zero() {
            break;
        }

        total += &sign * (&cur_n * &y);
        sign = -sign;

        let next_d = &cur_v - &r_d + &r_a - &one;
        cur_n = y;
        cur_a = cur_v;
        cur_d = next_d;
        cur_v = r_a;
    }

    Ok(total)
}

/// Count integers `t in [start, start + len)` satisfying `(a * t + d) mod sigma < tau`.
///
/// Uses the exact identity `[(u mod sigma) < tau] = floor(u / sigma) - floor((u - tau) / sigma)`
/// for `0 <= tau <= sigma` and `sigma > 0`. Returns `(count, floor_sum_calls_used)`.
pub fn count_modular_interval(
    start: &BigUint,
    len: &BigUint,
    a: &BigUint,
    d: &BigUint,
    sigma: &BigUint,
    tau: &BigUint,
    meter: &mut Meter,
) -> Result<(BigUint, u64)> {
    if sigma.is_zero() || tau > sigma {
        return Err(invalid("modular interval parameters"));
    }
    if len.is_zero() || tau.is_zero() {
        return Ok((BigUint::zero(), 0));
    }
    if tau == sigma {
        return Ok((len.clone(), 0));
    }

    let a_mod = a % sigma;
    let d_shifted = (a_mod.clone() * (start % sigma) + d) % sigma;
    let a_int = BigInt::from(a_mod);
    let d_int = BigInt::from(d_shifted);
    let d_minus_tau = &d_int - BigInt::from(tau.clone());

    let upper = floor_sum(len, &a_int, &d_int, sigma, meter)?;
    let lower = floor_sum(len, &a_int, &d_minus_tau, sigma, meter)?;
    let diff = upper - lower;
    if diff.is_negative() {
        return Err(integrity("negative modular hit count from floor_sum"));
    }
    let count = diff
        .to_biguint()
        .ok_or_else(|| integrity("floor_sum sign conversion"))?;
    if &count > len {
        return Err(integrity("modular hit count exceeds interval length"));
    }
    Ok((count, 2))
}

/// One raw affine-hash intersection before curve quadratic-residuosity filtering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawHashIntersection {
    /// Assumed ordinate parity `e in {0, 1}` of the first-slot point.
    pub parity: u8,
    /// First-slot hash value `t in [0, tau_1)`.
    pub first_hash: BigUint,
    /// First-slot candidate compressed encoding `z = (a_1 * t + b_1) mod sigma`.
    pub first_encoding: BigUint,
}

/// Complete output and operation counters for floor-sum paired cancellation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CancellationReport {
    /// Total calls made to `floor_sum`.
    pub floor_sum_calls: u64,
    /// Total recursive subintervals visited across both parities.
    pub intervals_visited: u64,
    /// All raw modular hash intersections `(e, t, z)` before range/parity/curve filtering.
    pub raw_intersections: Vec<RawHashIntersection>,
    /// Raw intersections passing `0 <= z < 2p` and `z mod 2 == e`.
    pub canonical_candidates: u64,
    /// Certified points `P in F_1` such that `-P in F_2` (sorted canonically).
    pub points: Vec<Point>,
}

/// Find all raw affine-hash intersections `(a_rel * t + d_e) mod sigma < tau_2`
/// for `t in [0, tau_1)` without curve decoding, in `poly(log sigma) * (1 + C)` work.
pub fn find_raw_intersections(
    sigma: &BigUint,
    tau_1: &BigUint,
    alpha_1: &BigUint,
    beta_1: &BigUint,
    tau_2: &BigUint,
    alpha_2: &BigUint,
    beta_2: &BigUint,
    meter: &mut Meter,
) -> Result<(Vec<RawHashIntersection>, u64, u64)> {
    if sigma <= &BigUint::from(2u8)
        || tau_1.is_zero()
        || tau_2.is_zero()
        || tau_1 > sigma
        || tau_2 > sigma
        || alpha_1.is_zero()
        || alpha_1 >= sigma
        || alpha_2.is_zero()
        || alpha_2 >= sigma
        || beta_1 >= sigma
        || beta_2 >= sigma
    {
        return Err(invalid("raw intersection hash parameters"));
    }

    let a_1 = inverse(alpha_1, sigma)?;
    let b_1 = (&a_1 * (sigma - beta_1)) % sigma;
    let a_rel = (alpha_2 * &a_1) % sigma;
    let one = BigUint::one();

    let mut floor_sum_calls = 0u64;
    let mut intervals_visited = 0u64;
    let mut raw = Vec::new();

    for parity in [0u8, 1u8] {
        let b_shifted = if parity == 0 {
            (&b_1 + &one) % sigma
        } else {
            (&b_1 + sigma - &one) % sigma
        };
        let d_e = (alpha_2 * b_shifted + beta_2) % sigma;

        let (root_count, calls) =
            count_modular_interval(&BigUint::zero(), tau_1, &a_rel, &d_e, sigma, tau_2, meter)?;
        floor_sum_calls = floor_sum_calls
            .checked_add(calls)
            .ok_or_else(|| overflow("floor_sum call counter"))?;
        if root_count.is_zero() {
            continue;
        }

        let mut stack = vec![(BigUint::zero(), tau_1.clone(), root_count)];
        while let Some((lo, len, expected)) = stack.pop() {
            meter.charge(1, "floor_sum interval split")?;
            intervals_visited = intervals_visited
                .checked_add(1)
                .ok_or_else(|| overflow("interval split counter"))?;

            if len == one {
                if expected != one {
                    return Err(integrity("unit interval had count != 1"));
                }
                if (&a_rel * &lo + &d_e) % sigma >= *tau_2 {
                    return Err(integrity("floor_sum leaf failed second hash bound"));
                }
                let z = (&a_1 * &lo + &b_1) % sigma;
                meter.vector(raw.len() + 1, "raw hash intersections")?;
                raw.push(RawHashIntersection {
                    parity,
                    first_hash: lo,
                    first_encoding: z,
                });
                continue;
            }

            let left_len = &len >> 1usize;
            let right_len = &len - &left_len;
            let (left_count, calls) =
                count_modular_interval(&lo, &left_len, &a_rel, &d_e, sigma, tau_2, meter)?;
            floor_sum_calls = floor_sum_calls
                .checked_add(calls)
                .ok_or_else(|| overflow("floor_sum call counter"))?;
            if left_count > expected {
                return Err(integrity("subinterval count exceeded parent count"));
            }
            let right_count = &expected - &left_count;

            if !right_count.is_zero() {
                meter.vector(stack.len() + 1, "floor_sum split stack")?;
                stack.push((&lo + &left_len, right_len, right_count));
            }
            if !left_count.is_zero() {
                meter.vector(stack.len() + 1, "floor_sum split stack")?;
                stack.push((lo, left_len, left_count));
            }
        }
    }

    Ok((raw, floor_sum_calls, intervals_visited))
}

/// Certify all paired cancellations `P in F_1 ∩ (-F_2)` at the identity target
/// using Euclidean reciprocity floor sums and recursive interval splitting.
///
/// Every returned point `P` is independently replayed through `base_1.contains`,
/// `base_2.contains(-P)`, and `P + (-P) = O`.
pub fn find_cancellations(
    curve: &Curve,
    base_1: &FactorBase,
    base_2: &FactorBase,
    rng: &mut impl RandomSource,
    meter: &mut Meter,
) -> Result<CancellationReport> {
    if base_1.prime() != base_2.prime() {
        return Err(invalid("cancellation slots must share hash prime"));
    }
    let sigma = base_1.prime();
    if sigma < &(curve.modulus() << 1) || sigma > &(curve.modulus() << 2) {
        return Err(invalid("factor-base hash prime outside [2p, 4p]"));
    }

    let (raw_intersections, floor_sum_calls, intervals_visited) = find_raw_intersections(
        sigma,
        base_1.threshold(),
        &base_1.seed().alpha,
        &base_1.seed().beta,
        base_2.threshold(),
        &base_2.seed().alpha,
        &base_2.seed().beta,
        meter,
    )?;

    let two_p = curve.modulus() << 1;
    let mut decoder = curve.decoder()?;
    let mut canonical_candidates = 0u64;
    let mut points = Vec::new();

    for hit in &raw_intersections {
        if hit.first_encoding >= two_p || u8::from(hit.first_encoding.bit(0)) != hit.parity {
            continue;
        }
        canonical_candidates = canonical_candidates
            .checked_add(1)
            .ok_or_else(|| overflow("canonical candidate counter"))?;
        if let Some(point) = decoder.decode(&hit.first_encoding, rng, meter)? {
            if !curve.in_subgroup(&point, meter)? {
                continue;
            }
            let negated = curve.negate(&point)?;
            if !base_1.contains(curve, &point, meter)?
                || !base_2.contains(curve, &negated, meter)?
                || curve.add(&point, &negated, meter)? != Point::Infinity
            {
                return Err(integrity("floor_sum cancellation witness replay failed"));
            }
            meter.vector(points.len() + 1, "verified cancellation points")?;
            points.push(point);
        }
    }

    points.sort();
    if points.windows(2).any(|w| w[0] == w[1]) {
        return Err(integrity("duplicate cancellation point emitted"));
    }

    Ok(CancellationReport {
        floor_sum_calls,
        intervals_visited,
        raw_intersections,
        canonical_candidates,
        points,
    })
}

/// Reference `O(tau_1)` full-enumeration oracle for `F_1 ∩ (-F_2)`, used to verify
/// zero disagreements against [`find_cancellations`].
pub fn enumerate_cancellations_reference(
    curve: &Curve,
    base_1: &FactorBase,
    base_2: &FactorBase,
    rng: &mut impl RandomSource,
    meter: &mut Meter,
) -> Result<Vec<Point>> {
    if base_1.prime() != base_2.prime() {
        return Err(invalid("cancellation slots must share hash prime"));
    }
    let members_1 = base_1.enumerate(curve, rng, meter)?;
    let mut out = Vec::new();
    for p in members_1 {
        let neg = curve.negate(&p)?;
        if base_2.contains(curve, &neg, meter)? {
            out.push(p);
        }
    }
    out.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arithmetic::Seeded;
    use crate::reduction::HashSeed;
    use crate::Limits;

    fn naive_floor_sum(n: u64, a: i64, d: i64, v: i64) -> i128 {
        let mut s = 0i128;
        let v128 = i128::from(v);
        for t in 0..n {
            let num = i128::from(a) * i128::from(t) + i128::from(d);
            s += num.div_euclid(v128);
        }
        s
    }

    #[test]
    fn floor_sum_matches_naive_across_signed_and_boundary_inputs() {
        let mut meter = Meter::new(Limits::default());
        for n in [0u64, 1, 2, 3, 7, 16, 31] {
            for v in [1u64, 2, 3, 5, 7, 13, 29] {
                for a in [-37i64, -13, -5, -1, 0, 1, 2, 5, 13, 29, 64] {
                    for d in [-40i64, -13, -7, -1, 0, 1, 6, 13, 35] {
                        let got = floor_sum(
                            &BigUint::from(n),
                            &BigInt::from(a),
                            &BigInt::from(d),
                            &BigUint::from(v),
                            &mut meter,
                        )
                        .unwrap();
                        let expected = BigInt::from(naive_floor_sum(n, a, d, v as i64));
                        assert_eq!(got, expected, "n={n} a={a} d={d} v={v}");
                    }
                }
            }
        }
    }

    #[test]
    fn floor_sum_cancellations_match_full_tau_enumeration_zero_disagreements() {
        let mut meter = Meter::new(Limits {
            steps: 500_000_000,
            ..Limits::default()
        });
        // Certified prime-order curve p = 827, r = 823 (icv1-fp10-t5-192cb216).
        let curve = Curve::new(
            BigUint::from(827u32),
            BigUint::from(1u32),
            BigUint::from(15u32),
            BigUint::from(823u32),
            Point::Affine {
                x: BigUint::from(2u32),
                y: BigUint::from(822u32),
            },
            &mut meter,
        )
        .unwrap();
        let sigma = BigUint::from(1657u32);
        let mut rng = Seeded(20261119);
        let mut total_nonempty = 0usize;

        for trial in 0..40u64 {
            let tau_1 = BigUint::from(20u64 + (trial * 7) % 120);
            let tau_2 = BigUint::from(20u64 + (trial * 11) % 120);
            let alpha_1 = BigUint::from(1u64 + (trial * 37 + 3) % 1656);
            let beta_1 = BigUint::from((trial * 101 + 7) % 1657);
            // Every fourth trial aligns slopes so F_1 and -F_2 collide densely (`REPORT.md` §1).
            let (alpha_2, beta_2) = if trial % 4 == 0 {
                let b2 = (&beta_1 + &sigma - &alpha_1) % &sigma;
                (alpha_1.clone(), b2)
            } else {
                (
                    BigUint::from(1u64 + (trial * 53 + 11) % 1656),
                    BigUint::from((trial * 197 + 19) % 1657),
                )
            };

            let base_1 = FactorBase::new(
                &curve,
                sigma.clone(),
                tau_1,
                HashSeed {
                    alpha: alpha_1,
                    beta: beta_1,
                },
                &mut meter,
            )
            .unwrap();
            let base_2 = FactorBase::new(
                &curve,
                sigma.clone(),
                tau_2,
                HashSeed {
                    alpha: alpha_2,
                    beta: beta_2,
                },
                &mut meter,
            )
            .unwrap();

            let report = find_cancellations(&curve, &base_1, &base_2, &mut rng, &mut meter).unwrap();
            let reference =
                enumerate_cancellations_reference(&curve, &base_1, &base_2, &mut rng, &mut meter)
                    .unwrap();
            assert_eq!(report.points, reference, "trial={trial}");
            if !report.points.is_empty() {
                total_nonempty += 1;
            }
        }
        assert!(total_nonempty >= 10);
    }
}
