use crate::sampler;
use aps_partition_lab::curve::{Curve, Point};
use crypto_lib::cryptanalysis::aps_single_relation::LabeledBase;
use serde::Serialize;

/// A sufficient, conservative label condition. Uncertain never means useless.
pub fn guaranteed_nonzero(labels: &[u64], order: u64) -> bool {
    if !(3..=4).contains(&labels.len()) || order < 3 || labels.iter().any(|&b| b >= order) {
        return false;
    }
    let centered: Vec<_> = labels.iter().map(|&b| b.min(order - b)).collect();
    if centered.iter().map(|&b| u128::from(b)).sum::<u128>() >= u128::from(order) {
        return false;
    }
    let Some(minimum) = centered
        .iter()
        .filter(|&&b| b != 0)
        .map(|b| b.trailing_zeros())
        .min()
    else {
        return false;
    };
    centered
        .iter()
        .filter(|&&b| b != 0 && b.trailing_zeros() == minimum)
        .count()
        & 1
        == 1
}

fn signed_b(labels: &[u64], mask: usize, order: u64) -> u64 {
    let mut out = 0u128;
    for (i, &b) in labels.iter().enumerate() {
        out =
            (out + u128::from(if mask & (1 << i) == 0 {
                b
            } else {
                (order - b) % order
            })) % u128::from(order);
    }
    out as u64
}

#[derive(Serialize)]
pub struct ProfileRow {
    pub coordinates: Vec<u64>,
    pub guaranteed_informative: bool,
    pub has_signed_zero_b: bool,
    pub informative_group_incidents: usize,
    pub zero_b_group_incidents: usize,
}

#[derive(Serialize)]
pub struct ProfileReport {
    pub seed: u64,
    pub base: LabeledBase,
    pub rows: Vec<ProfileRow>,
    pub safe_tuples: usize,
    pub uncertain_tuples: usize,
    pub polynomial_integration_tested: bool,
    pub all_safe_sign_checks_hold: bool,
}

pub fn run(seed: u64) -> Result<ProfileReport, String> {
    let f = sampler::even_fixture(seed)?;
    let curve = Curve::new(101, 59, 6)?;
    let points: Vec<_> = f.base.points.values().collect();
    let mut rows = Vec::new();
    let mut safe_count = 0;
    let mut correct = true;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for k in j + 1..points.len() {
                for l in k + 1..points.len() {
                    let tuple = [points[i], points[j], points[k], points[l]];
                    let labels: Vec<_> = tuple.iter().map(|p| p.v).collect();
                    let safe = guaranteed_nonzero(&labels, f.curve.n);
                    let has_zero = (0..16).any(|mask| signed_b(&labels, mask, f.curve.n) == 0);
                    correct &= !safe || !has_zero;
                    safe_count += usize::from(safe);
                    let mut informative = 0;
                    let mut zero = 0;
                    for mask in 0..8usize {
                        let mut sum = None;
                        let mut b = 0;
                        for (at, p) in tuple.iter().enumerate() {
                            let point = Point { x: p.x, y: p.y };
                            if !curve.contains(point) {
                                return Err(
                                    "valuation fixture point is not on the reference curve".into(),
                                );
                            }
                            let negative = at > 0 && mask & (1 << (at - 1)) != 0;
                            sum = curve
                                .add(sum, Some(if negative { curve.neg(point) } else { point }));
                            b = (b + if negative {
                                (f.curve.n - p.v) % f.curve.n
                            } else {
                                p.v
                            }) % f.curve.n;
                        }
                        if sum.is_none() {
                            if b == 0 {
                                zero += 1;
                            } else {
                                informative += 1;
                            }
                        }
                    }
                    rows.push(ProfileRow {
                        coordinates: tuple.iter().map(|p| p.x).collect(),
                        guaranteed_informative: safe,
                        has_signed_zero_b: has_zero,
                        informative_group_incidents: informative,
                        zero_b_group_incidents: zero,
                    });
                }
            }
        }
    }
    Ok(ProfileReport {
        seed,
        base: f.base,
        safe_tuples: safe_count,
        uncertain_tuples: rows.len() - safe_count,
        rows,
        polynomial_integration_tested: false,
        all_safe_sign_checks_hold: correct,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sufficient_condition_survives_complete_small_modulus_sign_checks() {
        for order in [5, 7, 11] {
            for a in 0..order {
                for b in 0..order {
                    for c in 0..order {
                        for d in 0..order {
                            let labels = [a, b, c, d];
                            if guaranteed_nonzero(&labels, order) {
                                assert!((0..16).all(|mask| signed_b(&labels, mask, order) != 0));
                            }
                        }
                    }
                }
            }
        }
        assert!(
            !guaranteed_nonzero(&[1, 2, 2, 2], 7),
            "odd integer sum can wrap to zero modulo the subgroup order"
        );
        assert!(!guaranteed_nonzero(&[0, 0, 0, 0], 107));
        assert!(!guaranteed_nonzero(&[107, 1, 2, 4], 107));
        assert!(!guaranteed_nonzero(&[1, 2], 107));
    }

    #[test]
    fn actual_even_arity_sampler_keeps_all_safe_and_uncertain_profiles() {
        for seed in sampler::SEEDS {
            let report = run(seed).unwrap();
            assert_eq!(report.rows.len(), 70);
            assert_eq!(
                report.safe_tuples, 48,
                "prediction from two samples of each absolute weight1/2/4/8"
            );
            assert_eq!(report.uncertain_tuples, 22);
            assert!(report.all_safe_sign_checks_hold);
            assert!(!report.polynomial_integration_tested);
        }
    }
}
