//! Known-answer structural control, not a timed APS or DLP solver.
use crate::{
    coloring,
    curve::{Curve, Point},
};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct LabeledPoint {
    pub point: Point,
    pub a: u64,
    pub b: u64,
    pub original_slot: usize,
}

#[derive(Serialize)]
pub struct LabeledCoverage {
    pub family_size: usize,
    pub represented_relation_incidents: usize,
    pub missing_relation_incidents: usize,
    pub represented_informative_incidents: usize,
    pub represented_zero_b_incidents: usize,
}

#[derive(Serialize)]
pub struct LabelReport {
    pub curve: Curve,
    pub generator: Point,
    pub target: Point,
    pub subgroup_order: u64,
    pub arity: usize,
    pub sampling_draws: usize,
    pub points: Vec<LabeledPoint>,
    pub relation_incidents: usize,
    pub informative_incidents: usize,
    pub zero_b_incidents: usize,
    pub family_coverage: Vec<LabeledCoverage>,
}

pub fn check_labels(
    curve: Curve,
    g: Point,
    q: Point,
    points: &[LabeledPoint],
) -> Result<(), String> {
    for p in points {
        if curve.add(curve.mul(Some(g), p.a), curve.mul(Some(q), p.b)) != Some(p.point) {
            return Err("point label differs from its group representation".into());
        }
    }
    Ok(())
}

pub fn experiment(arity: usize) -> Result<LabelReport, String> {
    if !(3..=4).contains(&arity) {
        return Err("label control arity must be 3/4".into());
    }
    let curve = Curve::new(17, 2, 2)?;
    let g = Point { x: 5, y: 1 };
    // Public known-answer fixture; no target scalar enters the relation checker.
    let q = Point { x: 0, y: 6 };
    let order = 19;
    if curve.mul(Some(g), order).is_some() || curve.mul(Some(q), order).is_some() {
        return Err("invalid fixture subgroup".into());
    }
    let mut points: Vec<LabeledPoint> = Vec::new();
    let mut draws = 0;
    for slot in 0..arity {
        let size = 8 / arity + usize::from(slot < 8 % arity);
        let mut in_slot = 0;
        for u in 0..order {
            if in_slot == size {
                break;
            }
            draws += 1;
            let v = 1 << slot;
            let Some(mut point) = curve.add(curve.mul(Some(g), u), curve.mul(Some(q), v)) else {
                continue;
            };
            if point.y == 0 || points.iter().any(|old| old.point.x == point.x) {
                continue;
            }
            let (a, b) = if point.y > curve.neg(point).y {
                point = curve.neg(point);
                ((order - u) % order, (order - v) % order)
            } else {
                (u, v)
            };
            if curve.add(curve.mul(Some(g), a), curve.mul(Some(q), b)) != Some(point) {
                return Err("canonical sign broke a point label".into());
            }
            points.push(LabeledPoint {
                point,
                a,
                b,
                original_slot: slot,
            });
            in_slot += 1;
        }
        if in_slot != size {
            return Err("fixed labeled pool exhausted".into());
        }
    }
    check_labels(curve, g, q, &points)?;
    let mut incidents = Vec::new();
    for tuple in coloring::tuples(points.len(), arity) {
        for mask in 0..1usize << (arity - 1) {
            let (sum, a, b) =
                tuple
                    .iter()
                    .enumerate()
                    .fold((None, 0, 0), |(sum, a, b), (i, &id)| {
                        let p = &points[id];
                        let negative = i > 0 && mask & (1 << (i - 1)) != 0;
                        let (point, aa, bb) = if negative {
                            (
                                curve.neg(p.point),
                                (order - p.a) % order,
                                (order - p.b) % order,
                            )
                        } else {
                            (p.point, p.a, p.b)
                        };
                        (
                            curve.add(sum, Some(point)),
                            (a + aa) % order,
                            (b + bb) % order,
                        )
                    });
            if sum.is_some() {
                continue;
            }
            if curve
                .add(curve.mul(Some(g), a), curve.mul(Some(q), b))
                .is_some()
            {
                return Err("zero relation has inconsistent known labels".into());
            }
            if b != 0 {
                // Tiny independent exhaustive scalar check; not supplied a log.
                let recovered = (0..order)
                    .find(|k| (a + b * k) % order == 0)
                    .ok_or("no scalar for informative relation")?;
                if curve.mul(Some(g), recovered) != Some(q) {
                    return Err("informative relation failed scalar check".into());
                }
            }
            incidents.push((tuple.clone(), b));
        }
    }
    let informative = incidents.iter().filter(|(_, b)| *b != 0).count();
    let mut coverage = Vec::new();
    for size in [1, 8, 32] {
        let family = coloring::candidate_family(points.len(), arity, size)?;
        let audit = coloring::audit(points.len(), arity, &family)?;
        let represented: std::collections::BTreeSet<_> =
            audit.owners.into_iter().map(|(t, _)| t).collect();
        let kept: Vec<_> = incidents
            .iter()
            .filter(|(t, _)| represented.contains(t))
            .collect();
        let good = kept.iter().filter(|(_, b)| *b != 0).count();
        coverage.push(LabeledCoverage {
            family_size: size,
            represented_relation_incidents: kept.len(),
            missing_relation_incidents: incidents.len() - kept.len(),
            represented_informative_incidents: good,
            represented_zero_b_incidents: kept.len() - good,
        });
    }
    Ok(LabelReport {
        curve,
        generator: g,
        target: q,
        subgroup_order: order,
        arity,
        sampling_draws: draws,
        points,
        relation_incidents: incidents.len(),
        informative_incidents: informative,
        zero_b_incidents: incidents.len() - informative,
        family_coverage: coverage,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recoloring_preserves_actual_point_labels_and_explicit_relation_population() {
        for m in [3, 4] {
            let report = experiment(m).unwrap();
            assert_eq!(report.points.len(), 8);
            assert!(report.informative_incidents > 0);
            assert_eq!(
                report.relation_incidents,
                report.informative_incidents + report.zero_b_incidents
            );
            for row in &report.family_coverage {
                assert_eq!(
                    row.represented_relation_incidents + row.missing_relation_incidents,
                    report.relation_incidents
                );
                assert_eq!(
                    row.represented_relation_incidents,
                    row.represented_informative_incidents + row.represented_zero_b_incidents
                );
            }
            for pair in report.family_coverage.windows(2) {
                assert!(
                    pair[0].represented_relation_incidents
                        <= pair[1].represented_relation_incidents
                );
            }
        }
        assert!(experiment(5).is_err());
    }

    #[test]
    fn corrupted_label_is_rejected_before_relations_can_be_interpreted() {
        let report = experiment(3).unwrap();
        let mut poisoned = report.points.clone();
        poisoned[0].a = (poisoned[0].a + 1) % report.subgroup_order;
        assert!(check_labels(report.curve, report.generator, report.target, &poisoned).is_err());
    }
}
