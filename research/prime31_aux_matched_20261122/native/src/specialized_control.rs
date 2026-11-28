//! Adapter for the isolated, exact maintained prime-batch MITM source closure.
//! The caller's base and signed-column map are preserved. Finite targets use
//! the maintained batched oracle; infinity uses its exact scalar reference on
//! the same cached table. Setup and boundary conversion are paid cold.

use std::time::{Duration, Instant};

pub use prime31_specialized_incumbent::SpecializedConfig;
use prime31_specialized_incumbent::{Prepared, SignedPoint};
use serde_json::{json, Value};

use crate::collector::group_ledger;
use crate::oracle::{
    OracleCounters, OracleLimits, OracleResult, OracleStatus, Presentation, SignedRelation,
};
use crate::replay;
use crate::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};

pub const SPECIALIZED_SOURCE_MANIFEST: &str = prime31_specialized_incumbent::SOURCE_MANIFEST;

pub struct SpecializedControl<'a> {
    curve: PrimeCurve,
    order: u64,
    base: &'a FactorBase<PrimePoint>,
    native: Prepared,
    setup: Value,
}

fn independent(curve: &PrimeCurve) -> replay::Curve {
    replay::Curve {
        p: curve.p,
        a: curve.a,
        b: curve.b,
    }
}

fn canonical(curve: &PrimeCurve, point: PrimePoint) -> bool {
    if point.infinity {
        point == PrimePoint::INFINITY
    } else {
        point.x < curve.p && point.y < curve.p && curve.is_on_curve(point)
    }
}

impl<'a> SpecializedControl<'a> {
    pub fn prepare(
        curve: &PrimeCurve,
        order: u64,
        base: &'a FactorBase<PrimePoint>,
    ) -> Result<Self, String> {
        Self::prepare_with_config(curve, order, base, SpecializedConfig::default())
    }

    pub fn prepare_with_config(
        curve: &PrimeCurve,
        order: u64,
        base: &'a FactorBase<PrimePoint>,
        config: SpecializedConfig,
    ) -> Result<Self, String> {
        let start = Instant::now();
        if !independent(curve).valid_parameters()
            || !replay::prime(curve.p)
            || !(3..(1u64 << 32)).contains(&order)
            || !replay::prime(order)
            || !(1..=4096).contains(&base.columns)
            || !(1..=1 << 20).contains(&config.setup_chunk)
            || !(1..=4096).contains(&config.probe_chunk)
        {
            return Err("specialized control requires a nonsingular prime curve, odd prime order and bounded base/batches".into());
        }
        let count = base.points.len();
        if count != 2 * base.columns
            || base.negations_deferred
            || base.neg_index.len() != count
            || base.col_of.len() != count
            || base.coef_of.len() != count
            || base.point_index.len() != count
            || base.abscissae != base.columns
            || base.abscissa_list.len() != base.columns
            || base.x_index.len() != base.columns
            || !base.abscissa_list.windows(2).all(|xs| xs[0] < xs[1])
        {
            return Err(
                "specialized control needs a complete explicit signed base and column map".into(),
            );
        }
        for (i, &point) in base.points.iter().enumerate() {
            if point.infinity
                || point.y == 0
                || !canonical(curve, point)
                || base.col_of[i] >= base.columns
                || (base.coef_of[i] != 1 && base.coef_of[i] != order - 1)
                || base.index_of_key(curve.key(&point)) != Some(i)
                || base.index_of_key(curve.key(&curve.neg(point))) != Some(base.neg_index[i])
                || base.neg_index[i] >= count
            {
                return Err("malformed specialized point/sign/point index/negation map".into());
            }
            let group = base
                .x_index
                .get(&point.x)
                .ok_or("specialized point is missing its abscissa index")?;
            if group.len() != 2 || !group.contains(&i) || !base.abscissa_list.contains(&point.x) {
                return Err("malformed specialized abscissa index".into());
            }
        }
        for x in &base.abscissa_list {
            let group = base
                .x_index
                .get(x)
                .ok_or("specialized abscissa list is inconsistent")?;
            if group.len() != 2
                || group[0] >= count
                || group[1] >= count
                || group[0] >= group[1]
                || base.points[group[0]].x != *x
                || base.points[group[1]].x != *x
            {
                return Err(
                    "specialized abscissa indices must preserve original signed-point order".into(),
                );
            }
        }
        let points: Vec<_> = base
            .points
            .iter()
            .enumerate()
            .map(|(i, point)| SignedPoint {
                x: point.x,
                y: point.y,
                column: base.col_of[i],
                coefficient: base.coef_of[i],
            })
            .collect();
        let native = Prepared::prepare(
            curve.p,
            curve.a,
            curve.b,
            order,
            base.columns,
            &points,
            config,
        )?;
        let mut setup = native.setup_report().clone();
        setup["caller_metadata_validation"] = json!({
            "point_checks":count, "column_sign_checks":count,
            "point_index_checks":count, "negation_index_checks":count,
            "abscissa_groups_checked":base.columns,
            "group_order_certificate_supplied_by_common_target_and_base_setup":true,
            "validation_operation_counters":null
        });
        setup["adapter_setup_seconds_diagnostic"] = json!(start.elapsed().as_secs_f64());
        setup["first_witness_policy"] = json!(
            "cancellation_first_then_native_abscissa_groups; scalar_signed_index_order_at_infinity"
        );
        Ok(Self {
            curve: curve.clone(),
            order,
            base,
            native,
            setup,
        })
    }

    pub fn setup_report(&self) -> &Value {
        &self.setup
    }

    fn signed_relation(
        &self,
        indices: Vec<usize>,
        target: PrimePoint,
        ops: &mut GroupOps,
    ) -> Result<SignedRelation, String> {
        if indices.len() != 3 {
            return Err("specialized oracle returned the wrong arity".into());
        }
        let mut row = vec![0i64; self.base.columns];
        let mut sum = PrimePoint::INFINITY;
        for &i in &indices {
            let point = *self
                .base
                .points
                .get(i)
                .ok_or("specialized witness index is outside the caller's base")?;
            row[self.base.col_of[i]] += if self.base.coef_of[i] == 1 {
                1
            } else if self.base.coef_of[i] == self.order - 1 {
                -1
            } else {
                return Err("specialized witness coefficient is not a sign".into());
            };
            sum = self.curve.add(ops, sum, point);
        }
        if sum != target {
            return Err("specialized witness failed signed public-point replay".into());
        }
        Ok(SignedRelation { indices, row })
    }

    /// Positive-cap first-witness mode only. The whole maintained native call
    /// is one cooperative block: the wrapper checks the deadline before it,
    /// then records any overrun without manufacturing a timeout or negative.
    /// The caller's outer process cap remains authoritative.
    pub fn decompose(
        &mut self,
        target: PrimePoint,
        limits: &OracleLimits,
        presentation: Presentation,
        ops: &mut GroupOps,
    ) -> Result<OracleResult, String> {
        let start = Instant::now();
        let budget = Duration::from_millis(limits.timeout_ms);
        let mut replay_ops = GroupOps::default();
        let mut native = Value::Null;
        let mut cancellation_checked = false;
        let mut cancellation_hit = false;
        let (status, relation, reason) = if !limits.stop_after_first || limits.max_solutions == 0 {
            (
                OracleStatus::UnknownLimit,
                None,
                Some(
                    "specialized pair table supports positive-cap first-witness mode only"
                        .to_string(),
                ),
            )
        } else if start.elapsed() >= budget {
            (
                OracleStatus::UnknownTimeout,
                None,
                Some("specialized query deadline before validation".to_string()),
            )
        } else {
            if !canonical(&self.curve, target) {
                return Err("specialized target is not a canonical curve point".into());
            }
            cancellation_checked = true;
            if let Some(i) = self.base.index_of_key(self.curve.key(&target)) {
                cancellation_hit = true;
                let relation = self.signed_relation(
                    vec![i, 0, self.base.neg_index[0]],
                    target,
                    &mut replay_ops,
                )?;
                (OracleStatus::FoundFirst, Some(relation), None)
            } else if start.elapsed() >= budget {
                (
                    OracleStatus::UnknownTimeout,
                    None,
                    Some("specialized query deadline before native call".to_string()),
                )
            } else {
                let query = self.native.decompose(if target.infinity {
                    None
                } else {
                    Some((target.x, target.y))
                })?;
                native = json!({
                    "native_called":true, "scalar_identity_fallback":query.scalar_identity_fallback,
                    "group_ops":query.group_ops, "lookups":query.lookups,
                    "counters":query.native, "field_operation_counters":null,
                    "group_counts_are_complete_native_arithmetic":false
                });
                match query.indices {
                    Some(indices) => (
                        OracleStatus::FoundFirst,
                        Some(self.signed_relation(indices, target, &mut replay_ops)?),
                        None,
                    ),
                    None => (OracleStatus::CompleteNegative, None, None),
                }
            }
        };
        // Only this counted replay uses the harness arithmetic. The foreign
        // native ledger remains separately named, so its absent field work is
        // never converted to zero or silently mixed into this field ledger.
        ops.merge(replay_ops);
        let mut counters = OracleCounters::default();
        if relation
            .as_ref()
            .is_some_and(|r| r.row.iter().all(|&v| v == 0))
        {
            counters.zero_relation_rows = 1;
        }
        Ok(OracleResult {
            status, complete:status == OracleStatus::CompleteNegative, positive:relation.is_some(),
            presentation, summands:3, relations:relation.into_iter().collect(), charts:vec![], counters,
            elapsed_ns:start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64,
            polynomial_build_ns:0, reason,
            accounting:"Exact maintained specialized MITM with paid setup; foreign native/group diagnostics are separate from adapter replay. Native field work is unavailable; polynomial counters are zero because no polynomial solver ran. The collector independently replays accepted rows.".into(),
            backend_work:Some(json!({
                "backend":"specialized_pair_table", "source_revision":"457ae1818aaf8134bd833ad9c1b9d57015264510",
                "native":native, "cancellation_checked":cancellation_checked, "cancellation_hit":cancellation_hit,
                "adapter_replay_group_and_field":group_ledger(&replay_ops),
                "native_ledger_merged_into_collector":false, "complete_operation_ledger":false,
                "first_witness_policy":"cancellation_first_then_native_abscissa_groups; scalar_signed_index_order_at_infinity",
                "cooperative_block":"whole maintained oracle call", "deadline_overrun":start.elapsed() > budget,
                "outer_process_cap_required":true,
                "completeness":"first witness only; complete negative requires cancellation check and exhaustive maintained finite scan or exact scalar identity fallback"
            })),
        })
    }
}
