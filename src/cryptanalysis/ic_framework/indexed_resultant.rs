//! Five-summand prime-field decomposition with an indexed quadratic S3 chart.

use std::collections::BTreeMap;

use super::plugins::table_counted;
use super::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::ic_boundary::{
    fast_map, invmod, CountedGroup, FactorBase, FastMap, FieldOps, GroupOps, OracleCounters,
    PrimeCurve, PrimePoint,
};
use crate::cryptanalysis::semaev_prime_poly::Curve;

#[derive(Default)]
pub struct IndexedResultantOracle {
    pairs: FastMap<Vec<[usize; 2]>>,
    identity_pair: Option<[usize; 2]>,
    setup_field_ops: FieldOps,
    setup_lookups: u64,
    setup_negations: u64,
    inserts: u64,
}

impl DecompositionOracle<PrimeCurve> for IndexedResultantOracle {
    fn name(&self) -> &str {
        "indexed-s3"
    }

    fn summands(&self) -> u32 {
        5
    }

    fn describe(&self, _: &Params) -> String {
        "quadratic S3 pair index joined to signed triples".into()
    }

    fn prepare(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        params: &Params,
        ops: &mut GroupOps,
    ) -> Result<(), String> {
        *self = Self::default();
        if params.u64_or("m", 5)? != 5 {
            return Err("indexed-s3 requires m=5".into());
        }
        let group = ctx.group;
        if group.p <= 3 || group.p >= 1 << 62 || group.p.is_multiple_of(2) {
            return Err("indexed-s3 requires an odd prime field with 3 < p < 2^62".into());
        }
        if fb.points.iter().any(|p| p.infinity) {
            return Err("indexed-s3 requires finite factor-base points".into());
        }

        let mut setup = Self {
            pairs: fast_map(fb.abscissae),
            ..Self::default()
        };
        for (i, &point) in fb.points.iter().enumerate() {
            setup.setup_lookups += 1;
            setup.setup_negations += 1;
            if let Some(j) = fb.index_of_key(group.key(&group.neg(point))) {
                setup.identity_pair = Some([i, j]);
                break;
            }
        }
        let curve = Curve::new(group.p, group.a, group.b);
        let mut representatives = Vec::with_capacity(fb.abscissae);
        for x in &fb.abscissa_list {
            setup.setup_lookups += 1;
            representatives.push(fb.x_index[x][0]);
        }
        for (offset, &i) in representatives.iter().enumerate() {
            for &j in &representatives[offset..] {
                for x in pair_roots(
                    curve,
                    fb.points[i],
                    fb.points[j],
                    &mut setup.setup_field_ops,
                )?
                .into_iter()
                .flatten()
                {
                    setup.pairs.entry(x).or_default().push([i, j]);
                    setup.inserts += 1;
                }
            }
        }
        ops.field_ops.merge(setup.setup_field_ops);
        *self = setup;
        Ok(())
    }

    fn setup_native(&self) -> BTreeMap<String, u64> {
        BTreeMap::from([
            ("inversions".into(), self.setup_field_ops.invs),
            ("lookups".into(), self.setup_lookups),
        ])
    }

    fn setup_work_native(&self) -> BTreeMap<String, u64> {
        table_counted(BTreeMap::from([
            ("table_insert_attempts".into(), self.inserts),
            (
                "native_field_muls_uncharged".into(),
                self.setup_field_ops.muls,
            ),
            (
                "native_field_sqrs_uncharged".into(),
                self.setup_field_ops.sqrs,
            ),
            ("point_negations_uncharged".into(), self.setup_negations),
        ]))
    }

    fn decompose(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        target: PrimePoint,
    ) -> Option<Vec<usize>> {
        let group = ctx.group;
        for (i, &first) in fb.points.iter().enumerate() {
            let rest = group.add(ops, target, group.neg(first));
            counters.point_negations += 1;
            for (j, &second) in fb.points.iter().enumerate().skip(i) {
                let rest = group.add(ops, rest, group.neg(second));
                counters.point_negations += 1;
                for (k, &third) in fb.points.iter().enumerate().skip(j) {
                    let residual = group.add(ops, rest, group.neg(third));
                    counters.point_negations += 1;
                    if let Some([u, v]) = self.lift_pair(group, fb, ops, counters, residual) {
                        return Some(vec![i, j, k, u, v]);
                    }
                }
            }
        }
        None
    }
}

impl IndexedResultantOracle {
    fn lift_pair(
        &self,
        group: &PrimeCurve,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        residual: PrimePoint,
    ) -> Option<[usize; 2]> {
        if residual.infinity {
            return self.identity_pair;
        }

        counters.lookups += 1;
        for &[i, j] in self.pairs.get(&residual.x)? {
            counters.lookups += 2;
            let left = &fb.x_index[&fb.points[i].x];
            let right = &fb.x_index[&fb.points[j].x];
            for &u in left {
                for &v in right {
                    if group.add(ops, fb.points[u], fb.points[v]) == residual {
                        return Some([u, v]);
                    }
                }
            }
        }
        None
    }
}

/// Specialize the S3 chart at two rational abscissae. For distinct x the
/// discriminant is (4*y1*y2)^2, so the base's ordinates supply its root.
fn pair_roots(
    curve: Curve,
    p: PrimePoint,
    q: PrimePoint,
    field_ops: &mut FieldOps,
) -> Result<[Option<u64>; 2], String> {
    let f = curve.f;
    let [constant, linear, quadratic] = curve.s3_in_last(p.x, q.x);
    field_ops.muls += 6;
    field_ops.sqrs += 2;
    let (numerator, denominator, offset) = if quadratic != 0 {
        field_ops.muls += 1;
        let product = f.mul(p.y, q.y);
        let twice = f.add(product, product);
        (
            f.neg(linear),
            f.add(quadratic, quadratic),
            f.add(twice, twice),
        )
    } else if linear != 0 {
        (f.neg(constant), linear, 0)
    } else {
        // A rational 2-torsion point doubles to the identity. On a
        // nonsingular curve its diagonal S3 chart is a nonzero constant.
        return if constant == 0 {
            Err("indexed-s3 encountered an identically zero S3 chart".into())
        } else {
            Ok([None, None])
        };
    };
    field_ops.invs += 1;
    let inverse = invmod(denominator, f.p)
        .ok_or_else(|| "indexed-s3 encountered a noninvertible S3 coefficient".to_owned())?;
    field_ops.muls += 1;
    let first = f.mul(f.add(numerator, offset), inverse);
    let second = if offset == 0 {
        None
    } else {
        field_ops.muls += 1;
        Some(f.mul(f.sub(numerator, offset), inverse))
    };
    Ok([Some(first), second])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn replays_m31_single_target_hit_and_miss() {
        let group = PrimeCurve {
            p: 2_147_483_647,
            a: 847_514_846,
            b: 2_087_444_724,
        };
        let points: Vec<_> = (154_076..154_092)
            .flat_map(|x| {
                let point = PrimePoint::affine(x, group.sqrt(group.rhs(x)).unwrap());
                [point, group.neg(point)]
            })
            .collect();
        let fb = FactorBase::from_column_map(
            "M31 matched factor base".into(),
            points.clone(),
            (0..points.len()).collect(),
            vec![1; points.len()],
            points.len(),
            |p| group.key(p),
            |p| group.key(&group.neg(*p)),
            |p| p.x,
        )
        .unwrap();
        // The planted and unplanted targets from m31-f4-matched.jsonl,
        // both at seed 11781661.
        let target = PrimePoint::affine(16_739_214, 978_802_724);
        let miss = PrimePoint::affine(1_501_288_703, 1_200_940_150);
        let ctx = InstanceCtx {
            group: &group,
            generator: points[0],
            target,
            r: 1,
            cofactor: 1,
            group_order: 1,
            name: "M31 oracle fixture".into(),
            field_degree: None,
        };
        let mut oracle = IndexedResultantOracle::default();
        let mut ops = GroupOps::default();
        let mut counters = OracleCounters::default();
        oracle
            .prepare(&ctx, &fb, &Params::default(), &mut ops)
            .unwrap();
        let witness = oracle
            .decompose(&ctx, &fb, &mut ops, &mut counters, target)
            .unwrap();
        assert_eq!(witness.len(), 5);
        let sum = witness.iter().fold(PrimePoint::INFINITY, |sum, &i| {
            group.add(&mut ops, sum, fb.points[i])
        });
        assert_eq!(sum, target);
        assert_eq!(
            oracle.decompose(&ctx, &fb, &mut ops, &mut counters, miss),
            None
        );
    }

    #[test]
    fn pair_charts_match_signed_group_law_including_two_torsion() {
        let group = PrimeCurve { p: 17, a: 1, b: 0 };
        let curve = Curve::new(group.p, group.a, group.b);
        let points: Vec<_> = (0..group.p)
            .filter_map(|x| group.sqrt(group.rhs(x)).map(|y| PrimePoint::affine(x, y)))
            .collect();
        assert!(points.contains(&PrimePoint::affine(0, 0)));
        let mut ops = GroupOps::default();
        for &p in &points {
            for &q in &points {
                let roots: BTreeSet<_> = pair_roots(curve, p, q, &mut ops.field_ops)
                    .unwrap()
                    .into_iter()
                    .flatten()
                    .collect();
                let sums: BTreeSet<_> = [q, group.neg(q)]
                    .into_iter()
                    .map(|point| group.add(&mut ops, p, point))
                    .filter(|sum| !sum.infinity)
                    .map(|sum| sum.x)
                    .collect();
                assert_eq!(roots, sums, "{p:?} {q:?}");
            }
        }
    }
}
