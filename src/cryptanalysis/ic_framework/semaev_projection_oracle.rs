//! `semaev-projection`: wrap [`PairProjection`] as an `ic.pipeline`
//! [`DecompositionOracle`] so batch-inv Semaev joins (#4846 / #4819 P0-1a)
//! can be remasured end-to-end against `rho.negation`.
//!
//! The library decomposes an abscissa into factor-base abscissae. This
//! oracle lifts a hit through [`lift_abscissae`] so the pipeline's
//! `[d]G = Q` check still applies.

use std::collections::BTreeMap;

use super::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::ic_boundary::{
    lift_abscissae, FactorBase, GroupOps, OracleCounters, PrimeCurve, PrimePoint,
};
use crate::cryptanalysis::pkm_smooth_tower::Curve;
use crate::cryptanalysis::semaev_projection::{FpOpCounts, PairProjection};

/// Exact Semaev pair-projection join as a prime-curve decomposition oracle.
pub struct SemaevProjectionOracle {
    m: u32,
    projection: Option<PairProjection>,
    build_fp: FpOpCounts,
    build_native: BTreeMap<String, u64>,
    query_native: BTreeMap<String, u64>,
}

impl SemaevProjectionOracle {
    pub fn new(m: u32) -> Self {
        Self {
            m,
            projection: None,
            build_fp: FpOpCounts::default(),
            build_native: BTreeMap::new(),
            query_native: BTreeMap::new(),
        }
    }

    fn merge_fp(ops: &mut GroupOps, fp: &FpOpCounts) {
        ops.field_ops.muls += fp.muls;
        ops.field_ops.sqrs += fp.sqrs;
        ops.field_ops.invs += fp.invs;
    }
}

impl DecompositionOracle<PrimeCurve> for SemaevProjectionOracle {
    fn name(&self) -> &str {
        "semaev-projection"
    }

    fn summands(&self) -> u32 {
        self.m
    }

    fn describe(&self, _params: &Params) -> String {
        format!(
            "Semaev pair-projection join (rational-discriminant batch-inv, #4819 P0-1a): \
             exact m = {} abscissa decomposition then signed summand recovery",
            self.m
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[("m", "summands a relation: 3 or 4")]
    }

    fn prepare(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        params: &Params,
        ops: &mut GroupOps,
    ) -> Result<(), String> {
        if !matches!(self.m, 3 | 4) {
            return Err(format!(
                "semaev-projection runs at m = 3 or 4, not {}",
                self.m
            ));
        }
        if params.u64_or("m", self.m as u64)? != self.m as u64 {
            return Err("semaev-projection: m in the oracle string must match".into());
        }
        if params.0.keys().any(|k| k != "m") {
            return Err("semaev-projection accepts only m=3|4".into());
        }
        if fb.abscissa_list.is_empty() {
            return Err("semaev-projection needs a non-empty factor base".into());
        }
        let curve = Curve {
            p: ctx.group.p,
            a: ctx.group.a,
            b: ctx.group.b,
        };
        let mut fp = FpOpCounts::default();
        let projection = PairProjection::build_counted(curve, &fb.abscissa_list, &mut fp)?;
        Self::merge_fp(ops, &fp);
        self.build_fp = fp;
        let stats = projection.build_stats();
        let mut native = BTreeMap::new();
        native.insert(
            "projection_unordered_pairs".into(),
            stats.unordered_pairs,
        );
        native.insert("projection_root_keys".into(), stats.root_keys);
        native.insert(
            "projection_distinct_keys".into(),
            stats.distinct_keys as u64,
        );
        native.insert("counted:projection_build".into(), 1);
        self.build_native = native;
        self.projection = Some(projection);
        self.query_native.clear();
        Ok(())
    }

    fn setup_native(&self) -> BTreeMap<String, u64> {
        let mut out = self.build_native.clone();
        out.insert("build_fp_muls".into(), self.build_fp.muls);
        out.insert("build_fp_sqrs".into(), self.build_fp.sqrs);
        out.insert("build_fp_invs".into(), self.build_fp.invs);
        out
    }

    fn setup_work_native(&self) -> BTreeMap<String, u64> {
        self.build_native.clone()
    }

    fn decompose(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: PrimePoint,
    ) -> Option<Vec<usize>> {
        let projection = self.projection.as_ref()?;
        if point.infinity {
            return None;
        }
        let mut fp = FpOpCounts::default();
        let (witness, stats) = match self.m {
            3 => {
                let (w, s) = projection.decompose_three_counted(point.x, &mut fp);
                (w.map(|w| w.x.to_vec()), s)
            }
            4 => {
                let (w, s) = projection.decompose_four_counted(point.x, &mut fp);
                (w.map(|w| w.x.to_vec()), s)
            }
            _ => return None,
        };
        Self::merge_fp(ops, &fp);
        *self
            .query_native
            .entry("projection_quadratics".into())
            .or_insert(0) += stats.quadratics;
        *self
            .query_native
            .entry("projection_lookups".into())
            .or_insert(0) += stats.lookups;
        *self
            .query_native
            .entry("projection_root_keys".into())
            .or_insert(0) += stats.root_keys;
        counters.lookups += stats.lookups;
        let xs = witness?;
        lift_abscissae(ctx.group, fb, ops, &xs, point)
    }

    fn relation_counters(&self) -> BTreeMap<String, u64> {
        self.query_native.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::{
        prime_factor_base, CountedGroup, PrimeInstance,
    };
    use crate::cryptanalysis::ic_framework::stages::Params;

    fn tiny_inst() -> PrimeInstance {
        let curve = PrimeCurve {
            p: 113,
            a: 19,
            b: 2,
        };
        let mut ops = GroupOps::default();
        for x in 1..113 {
            if let Some(y) = curve.sqrt(curve.rhs(x)) {
                let g = curve.mul(&mut ops, PrimePoint::affine(x, y), 16);
                if !g.infinity && curve.mul(&mut ops, g, 7).infinity {
                    return PrimeInstance {
                        name: "semaev-projection-control".into(),
                        curve,
                        group_order: 112,
                        r: 7,
                        cofactor: 16,
                        generator: (g.x, g.y),
                        representation: None,
                    };
                }
            }
        }
        panic!("control generator missing");
    }

    #[test]
    fn recovers_planted_three_sum_on_control_curve() {
        let inst = tiny_inst();
        let fb = prime_factor_base(&inst, 4);
        assert!(fb.abscissae >= 3, "need enough abscissae");
        let p0 = fb.points[0];
        let p1 = fb.points[2];
        let p2 = fb.points[4.min(fb.points.len().saturating_sub(1))];
        let mut ops = GroupOps::default();
        let target = inst
            .curve
            .add(&mut ops, inst.curve.add(&mut ops, p0, p1), p2);
        assert!(!target.infinity);

        let ctx = InstanceCtx {
            group: &inst.curve,
            generator: inst.generator_point(),
            target,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: None,
        };
        let (_, params) = Params::parse_spec("semaev-projection:m=3").unwrap();
        let mut oracle = SemaevProjectionOracle::new(3);
        let mut prep = GroupOps::default();
        DecompositionOracle::prepare(&mut oracle, &ctx, &fb, &params, &mut prep).unwrap();
        let mut counters = OracleCounters::default();
        let mut qops = GroupOps::default();
        let got = DecompositionOracle::decompose(
            &mut oracle,
            &ctx,
            &fb,
            &mut qops,
            &mut counters,
            target,
        );
        let idxs = got.expect("planted three-sum should hit");
        assert_eq!(idxs.len(), 3);
        let mut sum = PrimePoint::INFINITY;
        for i in idxs {
            sum = if sum.infinity {
                fb.points[i]
            } else {
                inst.curve.add(&mut qops, sum, fb.points[i])
            };
        }
        assert_eq!(sum.x, target.x);
        assert_eq!(sum.y, target.y);
    }
}
