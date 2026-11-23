use std::collections::BTreeMap;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::PrimeBatchMitmOracle;
use crate::cryptanalysis::ic_boundary::{
    decompose_mitm, CountedGroup, FactorBase, GroupOps, OracleCounters, PrimeCurve, PrimePoint,
};
use crate::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecializedConfig {
    pub setup_chunk: usize,
    pub probe_chunk: usize,
}

impl Default for SpecializedConfig {
    fn default() -> Self {
        Self {
            setup_chunk: 1024,
            probe_chunk: 16,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SignedPoint {
    pub x: u64,
    pub y: u64,
    pub column: usize,
    pub coefficient: u64,
}

pub struct Prepared {
    curve: PrimeCurve,
    order: u64,
    base: FactorBase<PrimePoint>,
    oracle: PrimeBatchMitmOracle,
    setup: Value,
}

#[derive(Debug)]
pub struct NativeQuery {
    pub indices: Option<Vec<usize>>,
    pub scalar_identity_fallback: bool,
    pub group_ops: GroupOps,
    pub lookups: u64,
    pub native: BTreeMap<String, u64>,
}

fn context<'a>(
    curve: &'a PrimeCurve,
    base: &FactorBase<PrimePoint>,
    order: u64,
    target: PrimePoint,
) -> InstanceCtx<'a, PrimeCurve> {
    // This oracle consults ctx.group only. The remaining fields stay public:
    // a signed base point as reference and the actual decomposition query.
    InstanceCtx {
        group: curve,
        generator: base.points[0],
        target,
        r: order,
        cofactor: 1,
        group_order: order,
        name: "prime31-specialized-incumbent".into(),
        field_degree: None,
    }
}

impl Prepared {
    /// Own a checked copy of the exact signed base. No reordering, new base
    /// choice, known logarithms or target-dependent base construction occurs.
    pub fn prepare(
        p: u64,
        a: u64,
        b: u64,
        order: u64,
        columns: usize,
        points: &[SignedPoint],
        config: SpecializedConfig,
    ) -> Result<Self, String> {
        let start = Instant::now();
        if !(5..(1u64 << 32)).contains(&p)
            || p & 1 == 0
            || a >= p
            || b >= p
            || !(3..(1u64 << 32)).contains(&order)
            || order & 1 == 0
            || !(1..=4096).contains(&columns)
            || points.len() != 2 * columns
            || !(1..=1 << 20).contains(&config.setup_chunk)
            || !(1..=4096).contains(&config.probe_chunk)
        {
            return Err(
                "unsupported specialized prime, signed-base dimensions or batch limits".into(),
            );
        }
        let curve = PrimeCurve { p, a, b };
        let mut base = FactorBase::empty("unchanged caller signed factor base".into());
        let mut column_x = vec![None; columns];
        let mut column_signs = vec![0u8; columns];
        for (i, value) in points.iter().enumerate() {
            let point = PrimePoint::affine(value.x, value.y);
            if value.x >= p
                || value.y == 0
                || value.y >= p
                || value.column >= columns
                || !curve.is_on_curve(point)
                || (value.coefficient != 1 && value.coefficient != order - 1)
            {
                return Err(
                    "specialized signed base has noncanonical point, column or sign".into(),
                );
            }
            let mask = if value.coefficient == 1 { 1 } else { 2 };
            if column_signs[value.column] & mask != 0 {
                return Err("specialized signed base repeats a column sign".into());
            }
            column_signs[value.column] |= mask;
            match column_x[value.column] {
                Some(x) if x != value.x => {
                    return Err("one specialized column has different abscissae".into())
                }
                None => column_x[value.column] = Some(value.x),
                _ => {}
            }
            if base.point_index.insert(curve.key(&point), i).is_some() {
                return Err("specialized signed base repeats a point".into());
            }
            base.points.push(point);
            base.col_of.push(value.column);
            base.coef_of.push(value.coefficient);
            base.x_index.entry(value.x).or_default().push(i);
        }
        if column_signs.iter().any(|&mask| mask != 3) || base.x_index.len() != columns {
            return Err("specialized columns do not biject with complete signed abscissae".into());
        }
        base.columns = columns;
        base.abscissae = columns;
        base.abscissa_list = base.x_index.keys().copied().collect();
        base.abscissa_list.sort_unstable();
        for x in &base.abscissa_list {
            let group = &base.x_index[x];
            if group.len() != 2 {
                return Err("specialized abscissa must have exactly two signed indices".into());
            }
            let left = base.points[group[0]];
            if curve.neg(left) != base.points[group[1]]
                || base.col_of[group[0]] != base.col_of[group[1]]
            {
                return Err("specialized signed pair is not the exact negation involution".into());
            }
        }
        base.neg_index = base
            .points
            .iter()
            .map(|point| {
                base.point_index
                    .get(&curve.key(&curve.neg(*point)))
                    .copied()
                    .ok_or_else(|| "specialized base not closed under negation".to_string())
            })
            .collect::<Result<_, _>>()?;
        let mut params = Params::default();
        params.set("negation_folded", "1");
        params.set("native_setup_pricing", "1");
        params.set("native_probe_pricing", "1");
        params.set("chunk", config.setup_chunk.to_string());
        params.set("probe_chunk", config.probe_chunk.to_string());
        let mut oracle = PrimeBatchMitmOracle::new(3);
        let mut setup_ops = GroupOps::default();
        oracle.prepare(
            &context(&curve, &base, order, base.points[0]),
            &base,
            &params,
            &mut setup_ops,
        )?;
        let native = oracle.setup_native();
        let table = oracle
            .table
            .as_ref()
            .ok_or("maintained oracle did not prepare a table")?;
        let setup = json!({
            "schema":"prime31-specialized-pair-setup/v1", "backend":"specialized_pair_table",
            "repository":"isogeny-labs/cryptanalysis", "source_revision":"457ae1818aaf8134bd833ad9c1b9d57015264510",
            "columns":columns, "signed_points":points.len(), "abscissae":columns, "summands":3,
            "config":config, "native":native, "group_ops":setup_ops,
            "finite_probe_order":"sorted_abscissa_then_original_index_within_abscissa",
            "identity_probe_order":"original_signed_index_scalar_fallback",
            "native_group_operations_are_complete_field_ledger":false,
            "field_operation_counters":null,
            "table_kind":table.kind, "retained_entries":table.entries,
            "logical_key_witness_payload_bytes":table.entries * table.witness_entry_payload_bytes() as u64,
            "payload_is_total_allocation_or_rss":false,
            "adapter_setup_seconds_diagnostic":start.elapsed().as_secs_f64(),
            "subgroup_certification":"supplied by caller; adapter checks points and signed metadata",
            "outer_process_envelope_required":true,
            "timings_are_admitted_performance_evidence":false
        });
        Ok(Self {
            curve,
            order,
            base,
            oracle,
            setup,
        })
    }

    pub fn setup_report(&self) -> &Value {
        &self.setup
    }

    pub fn decompose(&mut self, point: Option<(u64, u64)>) -> Result<NativeQuery, String> {
        let target = match point {
            Some((x, y)) if x < self.curve.p && y < self.curve.p => PrimePoint::affine(x, y),
            Some(_) => return Err("noncanonical specialized target".into()),
            None => PrimePoint::INFINITY,
        };
        if !self.curve.is_on_curve(target) {
            return Err("specialized target is off curve".into());
        }
        let before = self.oracle.relation_counters();
        let mut ops = GroupOps::default();
        let mut counters = OracleCounters::default();
        let indices = if target.infinity {
            decompose_mitm(
                &self.curve,
                &self.base,
                self.oracle.table.as_ref().ok_or("missing native table")?,
                3,
                &mut ops,
                &mut counters,
                target,
            )
        } else {
            self.oracle.decompose(
                &context(&self.curve, &self.base, self.order, target),
                &self.base,
                &mut ops,
                &mut counters,
                target,
            )
        };
        let native = self
            .oracle
            .relation_counters()
            .into_iter()
            .map(|(key, after)| {
                let old = before.get(&key).copied().unwrap_or(0);
                after
                    .checked_sub(old)
                    .map(|delta| (key, delta))
                    .ok_or_else(|| "native counter moved backwards".to_string())
            })
            .collect::<Result<_, _>>()?;
        Ok(NativeQuery {
            indices,
            scalar_identity_fallback: target.infinity,
            group_ops: ops,
            lookups: counters.lookups,
            native,
        })
    }
}
