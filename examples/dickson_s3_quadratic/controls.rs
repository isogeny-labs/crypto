//! Independent, bounded ground truth for signed target extraction.
//!
//! This module enumerates the explicitly supplied signed point-base and uses
//! only the production group law. It does not call a candidate solver, reuse
//! solver assignments, cache partial sums, or produce a timing result. A full
//! group sum is computed before the separate finite-prefix chart check, so an
//! intermediate infinity never silently removes a genuine group relation.

use crate::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};
use std::collections::BTreeSet;

const MAX_SIGNED_BASE: usize = 32;
const MAX_TUPLES: usize = 1_048_576;

#[derive(Debug, Default)]
pub(super) struct Control {
    /// Ordered signed target relations whose nonroot prefix sums are finite.
    pub(super) ordered_signed: BTreeSet<Vec<(u64, u64)>>,
    /// The same finite-chart relations, sorted by (x, y), retaining repeats.
    /// A multiset is retained if at least one of its orders fits the chart.
    pub(super) signed_multisets: BTreeSet<Vec<(u64, u64)>>,
    /// Leaf abscissae followed by x(P1+P2), ..., x(P1+...+P_(m-1)).
    /// Distinct signed relations can have the same complete x-chain.
    pub(super) x_chains: BTreeSet<Vec<u64>>,
    /// All ordered signed tuples summing to the exact signed target, including
    /// those that pass through infinity before the final sum.
    pub(super) total_group_matches: u64,
    /// Target matches excluded because at least one nonroot prefix is infinity.
    /// A tuple is counted once even if more than one prefix is infinity.
    pub(super) infinity_prefix_matches: u64,
    /// All tuples in the Cartesian power, including tuples that miss the target.
    pub(super) visited_tuples: u64,
    /// Native group ledger for the literal sum of every tuple. This is control
    /// work, separate from the candidate's work and from complete cost pricing.
    pub(super) full_sum_group_ops: GroupOps,
    /// Native group ledger for the independent chart pass on target matches.
    /// Point validation, sign-closure checks, sorting and set operations are not
    /// covered by either group ledger.
    pub(super) prefix_chart_group_ops: GroupOps,
}

/// Exhaust the Cartesian power of a complete, explicitly signed affine base.
///
/// Both signs must be supplied for a point with nonzero ordinate; a point with
/// ordinate zero appears once. Duplicate points, noncanonical coordinates,
/// off-curve points, and infinity are rejected. Empty bases are allowed and
/// have no tuples. The target must be finite and canonical. The caller owns
/// the prime-field and nonsingularity qualification of `curve`; this helper
/// does not certify the curve or its subgroup order.
///
/// This is a ground-truth control, bounded to at most 32 signed base points,
/// 2 <= m <= 4, and 1,048,576 ordered tuples. It returns no partial result.
pub(super) fn exhaustive(
    curve: &PrimeCurve,
    base_points: &[PrimePoint],
    m: usize,
    target: PrimePoint,
) -> Result<Control, String> {
    if !(2..=4).contains(&m) {
        return Err("exhaustive control requires 2 <= m <= 4".into());
    }
    if base_points.len() > MAX_SIGNED_BASE {
        return Err(format!(
            "exhaustive control signed base exceeds {MAX_SIGNED_BASE} points"
        ));
    }
    let tuple_count = base_points
        .len()
        .checked_pow(m as u32)
        .ok_or("exhaustive control tuple count overflow")?;
    if tuple_count > MAX_TUPLES {
        return Err(format!(
            "exhaustive control exceeds {MAX_TUPLES} ordered tuples"
        ));
    }
    if curve.p < 5 || curve.p >= (1_u64 << 63) || curve.a >= curve.p || curve.b >= curve.p {
        return Err(
            "exhaustive control requires canonical short-Weierstrass parameters with 5 <= p < 2^63"
                .into(),
        );
    }
    validate_affine(curve, target, "target")?;
    let mut signed_base = BTreeSet::new();
    for (index, point) in base_points.iter().copied().enumerate() {
        validate_affine(curve, point, &format!("base point {index}"))?;
        if !signed_base.insert((point.x, point.y)) {
            return Err(format!("exhaustive control duplicate base point {index}"));
        }
    }
    for point in base_points.iter().copied() {
        let negative = curve.neg(point);
        if !signed_base.contains(&(negative.x, negative.y)) {
            return Err("exhaustive control requires the explicitly signed base to be closed under negation".into());
        }
    }

    let mut control = Control::default();
    let mut tuple = vec![curve.identity(); m];
    for ordinal in 0..tuple_count {
        // An odometer over the complete Cartesian power: no prefix pruning,
        // root/assignment lookup, group-sum cache, or candidate-solver input.
        let mut digits = ordinal;
        for point in tuple.iter_mut().rev() {
            *point = base_points[digits % base_points.len()];
            digits /= base_points.len();
        }
        control.visited_tuples += 1;
        let mut full_sum = curve.identity();
        for point in tuple.iter().copied() {
            full_sum = curve.add(&mut control.full_sum_group_ops, full_sum, point);
        }
        if full_sum != target {
            continue;
        }
        control.total_group_matches += 1;

        // Start afresh. The full group relation above survives an intermediate
        // infinity; the affine solver chart below excludes that relation's
        // particular order. Only nonroot prefixes belong in the x-chain.
        let mut x_chain: Vec<u64> = tuple.iter().map(|point| point.x).collect();
        let mut prefix = tuple[0];
        let mut has_infinity_prefix = false;
        for point in tuple[1..m - 1].iter().copied() {
            prefix = curve.add(&mut control.prefix_chart_group_ops, prefix, point);
            if prefix.infinity {
                has_infinity_prefix = true;
                break;
            }
            x_chain.push(prefix.x);
        }
        if has_infinity_prefix {
            control.infinity_prefix_matches += 1;
            continue;
        }

        let signed: Vec<(u64, u64)> = tuple.iter().map(|point| (point.x, point.y)).collect();
        control.ordered_signed.insert(signed.clone());
        let mut multiset = signed;
        multiset.sort_unstable();
        control.signed_multisets.insert(multiset);
        control.x_chains.insert(x_chain);
    }
    debug_assert_eq!(control.visited_tuples, tuple_count as u64);
    debug_assert_eq!(
        control.total_group_matches,
        control.ordered_signed.len() as u64 + control.infinity_prefix_matches
    );
    Ok(control)
}

fn validate_affine(curve: &PrimeCurve, point: PrimePoint, label: &str) -> Result<(), String> {
    if point.infinity || point.x >= curve.p || point.y >= curve.p {
        return Err(format!(
            "exhaustive control {label} must be a canonical finite point"
        ));
    }
    if !curve.is_on_curve(point) {
        return Err(format!("exhaustive control {label} is not on the curve"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf_x_tuples(control: &Control) -> BTreeSet<Vec<u64>> {
        control
            .ordered_signed
            .iter()
            .map(|tuple| tuple.iter().map(|point| point.0).collect())
            .collect()
    }

    #[test]
    fn hand_derived_counts_separate_finite_chains_from_group_relations() {
        // Fixed predictions, not observations: the execution record must
        // establish these assertions before any report calls them verified.
        let curve = PrimeCurve { p: 7, a: 0, b: 3 };
        let base = [
            PrimePoint::affine(3, 3),
            PrimePoint::affine(3, 4),
            PrimePoint::affine(4, 2),
            PrimePoint::affine(4, 5),
        ];
        let target = PrimePoint::affine(1, 2);
        // (m, finite signed orders, signed multisets, full x-chains,
        //  leaf-x tuples, all group matches, excluded infinity orders).
        for (m, finite, multisets, chains, leaves, all, infinity) in [
            (2, 2, 1, 2, 2, 2, 0),
            (3, 4, 2, 4, 4, 4, 0),
            (4, 16, 2, 16, 8, 24, 8),
        ] {
            let control = exhaustive(&curve, &base, m, target).unwrap();
            assert_eq!(control.visited_tuples, 4_u64.pow(m as u32));
            assert_eq!(control.ordered_signed.len(), finite, "m={m}");
            assert_eq!(control.signed_multisets.len(), multisets, "m={m}");
            assert_eq!(control.x_chains.len(), chains, "m={m}");
            assert_eq!(leaf_x_tuples(&control).len(), leaves, "m={m}");
            assert_eq!(control.total_group_matches, all, "m={m}");
            assert_eq!(control.infinity_prefix_matches, infinity, "m={m}");
            assert!(control
                .x_chains
                .iter()
                .all(|chain| chain.len() == 2 * m - 2));
            assert_eq!(
                control.full_sum_group_ops.adds,
                control.visited_tuples * m as u64
            );
            if m == 2 {
                assert_eq!(
                    control.ordered_signed,
                    BTreeSet::from([vec![(3, 3), (4, 2)], vec![(4, 2), (3, 3)]])
                );
            }
            if m == 4 {
                // This exact signed tuple reaches infinity after two leaves,
                // then reaches the target. Its order is outside the chart.
                assert!(!control
                    .ordered_signed
                    .contains(&vec![(3, 3), (3, 4), (3, 3), (4, 2),]));
            }
        }
    }

    #[test]
    fn zero_y_target_retains_both_sign_choices_for_each_leaf_order() {
        let curve = PrimeCurve { p: 7, a: 3, b: 3 };
        let base = [
            PrimePoint::affine(3, 2),
            PrimePoint::affine(3, 5),
            PrimePoint::affine(4, 3),
            PrimePoint::affine(4, 4),
        ];
        let control = exhaustive(&curve, &base, 2, PrimePoint::affine(1, 0)).unwrap();
        assert_eq!(control.visited_tuples, 16);
        assert_eq!(control.total_group_matches, 4);
        assert_eq!(control.infinity_prefix_matches, 0);
        assert_eq!(
            control.ordered_signed,
            BTreeSet::from([
                vec![(3, 2), (4, 3)],
                vec![(4, 3), (3, 2)],
                vec![(3, 5), (4, 4)],
                vec![(4, 4), (3, 5)],
            ])
        );
        assert_eq!(control.signed_multisets.len(), 2);
        assert_eq!(control.x_chains, BTreeSet::from([vec![3, 4], vec![4, 3]]));
        assert_eq!(leaf_x_tuples(&control), control.x_chains);
    }

    #[test]
    fn empty_signed_base_has_no_tuples_or_matches() {
        let curve = PrimeCurve { p: 7, a: 0, b: 3 };
        let control = exhaustive(&curve, &[], 4, PrimePoint::affine(1, 2)).unwrap();
        assert_eq!(control.visited_tuples, 0);
        assert_eq!(control.total_group_matches, 0);
        assert_eq!(control.infinity_prefix_matches, 0);
        assert!(control.ordered_signed.is_empty());
        assert!(control.signed_multisets.is_empty());
        assert!(control.x_chains.is_empty());
    }

    #[test]
    fn rejects_ambiguous_or_out_of_scope_inputs() {
        let curve = PrimeCurve { p: 7, a: 0, b: 3 };
        let point = PrimePoint::affine(3, 3);
        let negative = PrimePoint::affine(3, 4);
        let target = PrimePoint::affine(1, 2);
        assert!(exhaustive(&curve, &[point, negative], 1, target).is_err());
        assert!(exhaustive(&curve, &[point, negative], 5, target).is_err());
        assert!(exhaustive(&curve, &[point; 33], 2, target).is_err());
        assert!(exhaustive(&curve, &[point, negative, point], 2, target).is_err());
        assert!(exhaustive(&curve, &[point], 2, target).is_err());
        assert!(exhaustive(&curve, &[PrimePoint::INFINITY], 2, target).is_err());
        assert!(exhaustive(&curve, &[PrimePoint::affine(10, 3)], 2, target).is_err());
        assert!(exhaustive(&curve, &[PrimePoint::affine(0, 0)], 2, target).is_err());
        assert!(exhaustive(&curve, &[point, negative], 2, PrimePoint::INFINITY).is_err());
    }
}
