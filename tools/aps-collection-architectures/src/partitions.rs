//! Pilot-only rule selection. Exact retained and discarded relations are reported.
//! This is not a learned GB-cost predictor and has no target-generalization claim.
use crate::recursive::{direct, Tuple};
use crate::{key, Affine, Fixture, FixtureId, PointKey, Work};
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize)]
pub struct Labeled {
    pub point: Affine,
    pub a: u64,
    pub b: u64,
}

pub fn labeled_base(
    f: Fixture,
    q: Affine,
    seed: u64,
    w: &mut Work,
) -> Result<Vec<Labeled>, String> {
    if q.is_none() || !crate::valid_points(f, &[q]) {
        return Err("invalid target".into());
    }
    let mut out = vec![];
    let mut seen = BTreeSet::new();
    for i in 0..256 {
        w.draws += 1;
        let a = (i * 7 + seed % f.order * 3) % f.order;
        let b = (i * i + i + seed % f.order) % f.order;
        let left = w.mul(f, Some(f.generator), a);
        let right = w.mul(f, q, b);
        let point = w.add(f, left, right);
        if point.is_some() && seen.insert(key(point)) {
            out.push(Labeled { point, a, b });
        }
        if out.len() == 8 {
            return Ok(out);
        }
    }
    Err("base draw cap reached".into())
}

fn color(rule: usize, i: usize, p: Affine) -> usize {
    let (x, y) = key(p).unwrap_or((0, 0));
    (match rule {
        0 => i as u64,
        1 => x,
        2 => y,
        3 => x + 3 * y,
        _ => unreachable!(),
    } % 4) as usize
}
fn admitted(rule: usize, base: &[Labeled], tuple: Tuple) -> bool {
    let colors: BTreeSet<_> = tuple.map(|i| color(rule, i, base[i].point)).into();
    colors.len() == 4
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Score {
    pub tuples: u64,
    pub useful: u64,
}
#[derive(Clone, Debug, Serialize)]
pub struct FrozenRule {
    fixture: FixtureId,
    rule: usize,
    pilot_targets: BTreeSet<PointKey>,
    pub pilot_scores: [Score; 4],
    pub all_pilot_scores_uninformative: bool,
}
#[derive(Clone, Debug, Serialize)]
pub struct Evaluation {
    pub target: Affine,
    pub seed: u64,
    pub base: Vec<Labeled>,
    pub all_roots: BTreeSet<Tuple>,
    pub admitted_roots: BTreeSet<Tuple>,
    pub useful: u64,
    pub zero_b: u64,
    pub cell_tuples: u64,
}
pub fn evaluate(
    f: Fixture,
    q: Affine,
    seed: u64,
    rule: usize,
    w: &mut Work,
) -> Result<Evaluation, String> {
    if rule >= 4 {
        return Err("unknown partition rule".into());
    }
    let base = labeled_base(f, q, seed, w)?;
    let points: Vec<_> = base.iter().map(|v| v.point).collect();
    let all_roots = direct(f, &points, None, w);
    let admitted_roots: BTreeSet<_> = all_roots
        .iter()
        .copied()
        .filter(|&t| admitted(rule, &base, t))
        .collect();
    let mut useful = 0;
    let mut zero_b = 0;
    for t in &admitted_roots {
        let a = t.iter().map(|&i| base[i].a).sum::<u64>() % f.order;
        let b = t.iter().map(|&i| base[i].b).sum::<u64>() % f.order;
        if b == 0 {
            zero_b += 1;
            continue;
        }
        let inv = (1..f.order)
            .find(|v| v * b % f.order == 1)
            .ok_or("nonunit b")?;
        let k = (f.order - a) * inv % f.order;
        w.witness_checks += 1;
        if w.mul(f, Some(f.generator), k) != q {
            return Err("partition scalar witness failed".into());
        }
        useful += 1;
    }
    let mut cell_tuples = 0;
    for i in 0..base.len() {
        for j in i + 1..base.len() {
            for k in j + 1..base.len() {
                for l in k + 1..base.len() {
                    if admitted(rule, &base, [i, j, k, l]) {
                        cell_tuples += 1;
                    }
                }
            }
        }
    }
    Ok(Evaluation {
        target: q,
        seed,
        base,
        all_roots,
        admitted_roots,
        useful,
        zero_b,
        cell_tuples,
    })
}

impl FrozenRule {
    /// Optimize pilot cell volume per useful root, not just sign-safe coverage.
    /// Pilot scanning and all four failed/zero-yield candidate scores are retained.
    pub fn train(f: Fixture, pilot: &[(Affine, u64)], w: &mut Work) -> Result<Self, String> {
        if pilot.is_empty() {
            return Err("empty pilot".into());
        }
        let mut keys = BTreeSet::new();
        let mut scores: [Score; 4] = std::array::from_fn(|_| Score::default());
        for &(q, seed) in pilot {
            if !keys.insert(key(q)) {
                return Err("duplicate pilot target".into());
            }
            for (rule, score) in scores.iter_mut().enumerate() {
                let result = evaluate(f, q, seed, rule, w)?;
                score.tuples += result.cell_tuples;
                score.useful += result.useful;
            }
        }
        let mut best = 0;
        for i in 1..4 {
            if scores[i].useful > 0
                && (scores[best].useful == 0
                    || scores[i].tuples * scores[best].useful
                        < scores[best].tuples * scores[i].useful)
            {
                best = i;
            }
        }
        Ok(Self {
            fixture: f.id,
            rule: best,
            pilot_targets: keys,
            all_pilot_scores_uninformative: scores.iter().all(|s| s.useful == 0),
            pilot_scores: scores,
        })
    }
    pub fn test(
        &self,
        f: Fixture,
        q: Affine,
        seed: u64,
        w: &mut Work,
    ) -> Result<Evaluation, String> {
        if f.id != self.fixture || self.pilot_targets.contains(&key(q)) {
            return Err("held-out curve mismatch or pilot-target overlap".into());
        }
        evaluate(f, q, seed, self.rule, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_pilot_then_disjoint_targets_and_all_losses_retained() {
        for id in [FixtureId::P17, FixtureId::P101] {
            let f = Fixture::new(id);
            let mut w = Work::default();
            let model =
                FrozenRule::train(f, &[(f.point(3), 11), (f.point(5), 13)], &mut w).unwrap();
            let frozen = serde_json::to_string(&model).unwrap();
            for k in [7, 11, 16] {
                let e = model.test(f, f.point(k), 17, &mut w).unwrap();
                assert!(e.admitted_roots.is_subset(&e.all_roots));
                assert_eq!(e.useful + e.zero_b, e.admitted_roots.len() as u64);
                for v in &e.base {
                    assert_eq!(
                        f.curve.add(f.point(v.a), f.curve.mul(f.point(k), v.b)),
                        v.point
                    );
                }
                assert_eq!(serde_json::to_string(&model).unwrap(), frozen);
            }
            assert!(model.test(f, f.point(3), 999, &mut w).is_err());
            assert!(w.draws > 0 && w.direct_tuples > 0);
        }
    }
    #[test]
    fn all_four_rules_include_uninformative_and_empty_candidates() {
        let f = Fixture::new(FixtureId::P17);
        for rule in 0..4 {
            let e = evaluate(f, f.point(7), 23, rule, &mut Work::default()).unwrap();
            assert_eq!(e.base.len(), 8);
            assert_eq!(e.useful + e.zero_b, e.admitted_roots.len() as u64);
        }
        assert!(evaluate(f, f.point(7), 0, 4, &mut Work::default()).is_err());
        assert!(FrozenRule::train(f, &[], &mut Work::default()).is_err());
    }
}
