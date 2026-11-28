//! Same-base, first-witness m=3 control using the pinned production PairTable.
//!
//! The table builders, table probe and batched prime-curve addition are exact
//! selected upstream code. This module is study glue: a bounded probe loop,
//! explicit cancellation handling, signed-row replay and accounting. It does
//! not enumerate all witnesses and does not substitute a new arithmetic solver.
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use crate::collector::group_ledger;
use crate::cryptanalysis::ic_boundary::PairTable;
use crate::oracle::{
    OracleCounters, OracleLimits, OracleResult, OracleStatus, Presentation, SignedRelation,
};
use crate::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};

/// Matches the production prime adder's maximum batch. Passing at most one
/// such block allows a deadline check before each call without changing its
/// within-block enumeration or field accounting.
pub const PROBE_BLOCK: usize = 128;
const MAX_SIGNED_POINTS: usize = 4096;
const FIRST_WITNESS_POLICY: &str = "cancellation_first_then_signed_index_order";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PairMode {
    /// Differential correctness control using the production scalar builder.
    Scalar,
    /// Production batched table builder and production batched complements.
    Batched,
}

#[derive(Clone, Debug, Serialize)]
pub struct PairSetupReport {
    pub backend: &'static str,
    pub mode: PairMode,
    pub columns: usize,
    pub signed_points: usize,
    pub abscissae: usize,
    pub table_kind: &'static str,
    pub table_entries: u64,
    pub insert_attempts: u64,
    pub index_lookups: u64,
    pub production_native: BTreeMap<String, u64>,
    pub table_group_and_field: Value,
    pub validation_group_and_field: Value,
    pub validation_point_checks: u64,
    pub validation_index_lookups: u64,
    pub validation_point_negations: u64,
    pub requested_hash_entry_capacity: u64,
    pub logical_entry_payload_bytes: u64,
    pub requested_payload_bytes: u64,
    pub memory_accounting: &'static str,
    pub table_elapsed_ns: u64,
    pub elapsed_ns: u64,
    pub probe_block_size: usize,
    pub first_witness_policy: &'static str,
    pub accounting: &'static str,
}

#[derive(Clone, Debug, Default, Serialize)]
struct QueryCounts {
    target_point_checks: u64,
    cancellation_lookups: u64,
    cancellation_hits: u64,
    deadline_checks: u64,
    blocks_started: u64,
    /// Inputs submitted to the adder, including a partly visited final batch.
    prepared_input_points: u64,
    /// Completed R-P_i values actually passed to the probe callback.
    candidate_complements: u64,
    identity_complements: u64,
    table_probe_calls: u64,
    pair_witness_hits: u64,
    replay_additions: u64,
}

pub struct PairControl<'a> {
    curve: PrimeCurve,
    order: u64,
    base: &'a FactorBase<PrimePoint>,
    table: PairTable,
    mode: PairMode,
    setup: PairSetupReport,
}

fn elapsed_ns(start: Instant) -> u64 {
    start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64
}

/// Delta of the executed production ledger, including its fields that are
/// intentionally skipped by GroupOps' legacy Serialize implementation.
fn ops_delta(after: &GroupOps, before: &GroupOps) -> GroupOps {
    let mut out = *after;
    out.adds -= before.adds;
    out.doubles -= before.doubles;
    out.scalar_mults -= before.scalar_mults;
    out.field_ops.muls -= before.field_ops.muls;
    out.field_ops.sqrs -= before.field_ops.sqrs;
    out.field_ops.invs -= before.field_ops.invs;
    out.montgomery_reductions -= before.montgomery_reductions;
    out
}

fn checked_point(curve: &PrimeCurve, point: PrimePoint, ops: &mut GroupOps) -> bool {
    if point.infinity {
        return point == PrimePoint::INFINITY;
    }
    if point.x >= curve.p || point.y >= curve.p {
        return false;
    }
    // is_on_curve executes four mulmod calls, including the two squares.
    ops.field_ops.muls += 4;
    curve.is_on_curve(point)
}

fn counted_mul(a: u64, b: u64, p: u64, ops: &mut GroupOps) -> u64 {
    ops.field_ops.muls += 1;
    (u128::from(a) * u128::from(b) % u128::from(p)) as u64
}

/// Check the materialized sign/index contract needed by a folded pair table.
/// Field primality and the declared subgroup order are certified by the common
/// PublicTarget validation and factor-base preparation, outside this adapter.
/// In particular, `order` must annihilate the supplied base points.
fn validate_base(
    curve: &PrimeCurve,
    order: u64,
    base: &FactorBase<PrimePoint>,
    ops: &mut GroupOps,
) -> Result<(u64, u64, u64), String> {
    if curve.p <= 3
        || curve.p >= (1u64 << 32)
        || curve.a >= curve.p
        || curve.b >= curve.p
        || !(2..(1u64 << 32)).contains(&order)
    {
        return Err("pair control requires canonical p<2^32 parameters and order>=2".into());
    }
    let a2 = counted_mul(curve.a, curve.a, curve.p, ops);
    let a3 = counted_mul(a2, curve.a, curve.p, ops);
    let b2 = counted_mul(curve.b, curve.b, curve.p, ops);
    let disc_a = counted_mul(4, a3, curve.p, ops);
    let disc_b = counted_mul(27 % curve.p, b2, curve.p, ops);
    if (disc_a + disc_b) % curve.p == 0 {
        return Err("pair control requires a nonsingular curve".into());
    }
    let n = base.points.len();
    if n == 0 || n > MAX_SIGNED_POINTS || base.columns == 0 {
        return Err(format!(
            "pair control requires 1..={MAX_SIGNED_POINTS} materialized signed points"
        ));
    }
    if base.negations_deferred
        || base.neg_index.len() != n
        || base.col_of.len() != n
        || base.coef_of.len() != n
        || base.point_index.len() != n
        || base.abscissae != base.columns
        || base.abscissa_list.len() != base.columns
        || base.x_index.len() != base.columns
        || !base.abscissa_list.windows(2).all(|xs| xs[0] < xs[1])
    {
        return Err("pair control needs a complete explicit one-column-per-abscissa base".into());
    }
    let mut point_checks = 0;
    let mut index_lookups = 0;
    let mut point_negations = 0;
    for (i, &point) in base.points.iter().enumerate() {
        point_checks += 1;
        if point.infinity || !checked_point(curve, point, ops) {
            return Err(format!(
                "pair factor-base point {i} is not finite and on curve"
            ));
        }
        index_lookups += 1;
        if base.index_of_key(curve.key(&point)) != Some(i) {
            return Err("pair factor-base point index is inconsistent".into());
        }
        let ni = base.neg_index[i];
        point_negations += 1;
        if ni >= n || base.neg_index[ni] != i || base.points[ni] != curve.neg(point) {
            return Err("pair factor-base negation index is inconsistent".into());
        }
        if base.col_of[i] >= base.columns
            || base.col_of[ni] != base.col_of[i]
            || (base.coef_of[i] != 1 && base.coef_of[i] != order - 1)
        {
            return Err("pair factor-base column/sign metadata is inconsistent".into());
        }
    }
    let mut seen_points = vec![false; n];
    let mut seen_columns = vec![false; base.columns];
    for &x in &base.abscissa_list {
        index_lookups += 1;
        let group = base
            .x_index
            .get(&x)
            .ok_or("pair factor-base abscissa index is inconsistent")?;
        if group.is_empty() || group.len() > 2 {
            return Err("pair factor-base abscissa must have one or two points".into());
        }
        let first = *group.first().ok_or("empty abscissa group")?;
        if first >= n {
            return Err("pair factor-base abscissa contains an invalid point index".into());
        }
        let column = base.col_of[first];
        if seen_columns[column] {
            return Err("pair factor-base column spans different abscissae".into());
        }
        seen_columns[column] = true;
        for &i in group {
            if i >= n || seen_points[i] || base.points[i].x != x || base.col_of[i] != column {
                return Err("pair factor-base abscissa membership is inconsistent".into());
            }
            seen_points[i] = true;
        }
        if group.len() == 1 {
            if base.points[first].y != 0
                || base.neg_index[first] != first
                || base.coef_of[first] != 1
            {
                return Err("a singleton abscissa must be a canonical order-two column".into());
            }
        } else {
            let second = group[1];
            if base.neg_index[first] != second
                || base.coef_of[first] == base.coef_of[second]
                || order == 2
            {
                return Err("a paired abscissa must have opposite column signs".into());
            }
        }
    }
    if seen_points.contains(&false) || seen_columns.contains(&false) {
        return Err("pair factor-base indexes omit points or columns".into());
    }
    Ok((point_checks, index_lookups, point_negations))
}

impl<'a> PairControl<'a> {
    /// Prepare exactly this factor base. No extra points, known logarithms or
    /// target-dependent selection are used. The caller must include this cold
    /// setup and its separate ledger in the algorithm total.
    pub fn prepare(
        curve: &PrimeCurve,
        order: u64,
        base: &'a FactorBase<PrimePoint>,
        mode: PairMode,
    ) -> Result<Self, String> {
        let start = Instant::now();
        let mut validation = GroupOps::default();
        let (point_checks, index_lookups, point_negations) =
            validate_base(curve, order, base, &mut validation)?;
        let capacity = (base.abscissae as u64)
            .checked_mul(base.abscissae as u64)
            .and_then(|n| n.checked_add(1))
            .ok_or("pair table capacity overflow")?;
        let table = match mode {
            PairMode::Scalar => PairTable::build_negation_folded(curve, base),
            PairMode::Batched => PairTable::build_negation_folded_batched(curve, base),
        };
        let setup = PairSetupReport {
            backend: "pair_table",
            mode,
            columns: base.columns,
            signed_points: base.points.len(),
            abscissae: base.abscissae,
            table_kind: table.kind,
            table_entries: table.entries,
            insert_attempts: table.insert_attempts,
            index_lookups: table.index_lookups,
            production_native: table.work_counters(),
            table_group_and_field: group_ledger(&table.build_ops),
            validation_group_and_field: group_ledger(&validation),
            validation_point_checks: point_checks,
            validation_index_lookups: index_lookups,
            validation_point_negations: point_negations,
            requested_hash_entry_capacity: capacity,
            logical_entry_payload_bytes: table.entries * 16,
            requested_payload_bytes: capacity * 16,
            memory_accounting: "16 bytes per logical (u64 key,u32 i,u32 j); requested capacity is the production reserve argument, not allocator capacity, total allocation or peak RSS; hash metadata, row buffers and retained thread-local arithmetic scratch are excluded",
            table_elapsed_ns: table.build_wall_ns,
            elapsed_ns: elapsed_ns(start),
            probe_block_size: PROBE_BLOCK,
            first_witness_policy: FIRST_WITNESS_POLICY,
            accounting: "production table native counters and complete returned group/field ledger; adapter validation counted separately; wall durations are diagnostics unless an admitted matched run supplies them; no complete CPU/allocator count is claimed",
        };
        Ok(Self {
            curve: curve.clone(),
            order,
            base,
            table,
            mode,
            setup,
        })
    }

    pub fn setup_report(&self) -> &PairSetupReport {
        &self.setup
    }

    fn relation(
        &self,
        indices: Vec<usize>,
        target: PrimePoint,
        ops: &mut GroupOps,
        counts: &mut QueryCounts,
    ) -> Result<SignedRelation, String> {
        let mut row = vec![0i64; self.base.columns];
        let mut sum = PrimePoint::INFINITY;
        for &i in &indices {
            let point = *self
                .base
                .points
                .get(i)
                .ok_or("pair witness index outside base")?;
            let coefficient = self.base.coef_of[i];
            let sign = if coefficient == 1 {
                1
            } else if coefficient == self.order - 1 {
                -1
            } else {
                return Err("pair witness coefficient is not a sign".into());
            };
            row[self.base.col_of[i]] += sign;
            sum = self.curve.add(ops, sum, point);
            counts.replay_additions += 1;
        }
        if indices.len() != 3 || sum != target {
            return Err("production pair witness failed literal signed-point replay".into());
        }
        Ok(SignedRelation { indices, row })
    }

    #[allow(clippy::too_many_arguments)]
    fn result(
        &self,
        status: OracleStatus,
        relation: Option<SignedRelation>,
        reason: Option<&str>,
        presentation: Presentation,
        start: Instant,
        before: &GroupOps,
        ops: &GroupOps,
        counts: &QueryCounts,
    ) -> OracleResult {
        let complete = status == OracleStatus::CompleteNegative;
        let positive = relation.is_some();
        let mut counters = OracleCounters::default();
        if relation
            .as_ref()
            .is_some_and(|r| r.row.iter().all(|&x| x == 0))
        {
            counters.zero_relation_rows = 1;
        }
        OracleResult {
            status,
            complete,
            positive,
            presentation,
            summands: 3,
            relations: relation.into_iter().collect(),
            charts: vec![],
            counters,
            elapsed_ns: elapsed_ns(start),
            polynomial_build_ns: 0,
            reason: reason.map(str::to_owned),
            accounting: "pair-table adapter: group/field work, native probe counts and validation are in backend_work; the F4-specific counters are zero because no polynomial solver ran; setup is a separate choice_pair_setup receipt; the collector independently replays every accepted row".into(),
            backend_work: Some(json!({
                "backend":"pair_table",
                "mode":self.mode,
                "table_kind":self.table.kind,
                "first_witness_policy":FIRST_WITNESS_POLICY,
                "probe_block_size":PROBE_BLOCK,
                "native":counts,
                "group_and_field":group_ledger(&ops_delta(ops,before)),
                "completeness":"first-witness only; complete negative requires cancellation check and all signed complements; infinity pair sums are covered by the cancellation prefix",
                "accounting":"executed production group/field ledger plus explicit adapter native counters; hash internals, memory allocation, comparisons and full machine instructions are not priced here"
            })),
        }
    }

    /// Find the first m=3 witness or establish absence after a full scan.
    /// Repetitions, doubling, negative signs and an infinite target are allowed.
    ///
    /// The production table omits identity pair sums. Such a witness can only
    /// occur when R itself is in the base. The common cancellation prefix then
    /// returns R + P_0 - P_0 before either scalar or batched probing. Otherwise,
    /// folded pair coverage and the full signed-complement scan are exhaustive.
    /// The explicit deadline is cooperative, checked between blocks; a started
    /// block (and a found witness replay) may finish after it.
    pub fn decompose(
        &self,
        target: PrimePoint,
        limits: &OracleLimits,
        presentation: Presentation,
        ops: &mut GroupOps,
    ) -> Result<OracleResult, String> {
        let start = Instant::now();
        let before = *ops;
        let mut counts = QueryCounts::default();
        if !limits.stop_after_first || limits.max_solutions == 0 {
            return Ok(self.result(
                OracleStatus::UnknownLimit,
                None,
                Some("pair_table supports a positive-cap first-witness query only"),
                presentation,
                start,
                &before,
                ops,
                &counts,
            ));
        }
        let budget = Duration::from_millis(limits.timeout_ms);
        counts.deadline_checks += 1;
        if start.elapsed() >= budget {
            return Ok(self.result(
                OracleStatus::UnknownTimeout,
                None,
                Some("pair query deadline before validation"),
                presentation,
                start,
                &before,
                ops,
                &counts,
            ));
        }
        counts.target_point_checks += 1;
        if !checked_point(&self.curve, target, ops) {
            return Err("pair query target is not a canonical point on the curve".into());
        }
        counts.cancellation_lookups += 1;
        if let Some(i) = self.base.index_of_key(self.curve.key(&target)) {
            counts.cancellation_hits += 1;
            let relation =
                self.relation(vec![i, 0, self.base.neg_index[0]], target, ops, &mut counts)?;
            return Ok(self.result(
                OracleStatus::FoundFirst,
                Some(relation),
                None,
                presentation,
                start,
                &before,
                ops,
                &counts,
            ));
        }

        for begin in (0..self.base.points.len()).step_by(PROBE_BLOCK) {
            counts.deadline_checks += 1;
            if start.elapsed() >= budget {
                return Ok(self.result(
                    OracleStatus::UnknownTimeout,
                    None,
                    Some("pair query deadline before completing the signed-complement scan"),
                    presentation,
                    start,
                    &before,
                    ops,
                    &counts,
                ));
            }
            let end = (begin + PROBE_BLOCK).min(self.base.points.len());
            counts.blocks_started += 1;
            let mut found = None;
            let mut visit = |ops: &mut GroupOps, offset: usize, complement: PrimePoint| {
                counts.candidate_complements += 1;
                if complement.infinity {
                    counts.identity_complements += 1;
                    return false;
                }
                if let Some((j, k)) = self.table.probe(
                    &self.curve,
                    self.base,
                    ops,
                    &mut counts.table_probe_calls,
                    complement,
                ) {
                    counts.pair_witness_hits += 1;
                    found = Some(vec![begin + offset, j, k]);
                    return true;
                }
                false
            };
            match self.mode {
                PairMode::Scalar => {
                    for (offset, &point) in self.base.points[begin..end].iter().enumerate() {
                        // Scalar mode only prepares the point it visits.
                        // Batched mode prepares the entire submitted block.
                        // Account below through a separate local, since the
                        // callback mutably owns the query counters here.
                        let complement = self.curve.add(ops, target, self.curve.neg(point));
                        if visit(ops, offset, complement) {
                            break;
                        }
                    }
                }
                PairMode::Batched => {
                    self.curve.add_each_batched(
                        ops,
                        target,
                        &self.base.points[begin..end],
                        true,
                        &mut visit,
                    );
                }
            }
            counts.prepared_input_points += match self.mode {
                PairMode::Scalar => found.as_ref().map_or(end - begin, |w| w[0] - begin + 1),
                PairMode::Batched => end - begin,
            } as u64;
            if let Some(indices) = found {
                let relation = self.relation(indices, target, ops, &mut counts)?;
                return Ok(self.result(
                    OracleStatus::FoundFirst,
                    Some(relation),
                    None,
                    presentation,
                    start,
                    &before,
                    ops,
                    &counts,
                ));
            }
        }
        Ok(self.result(
            OracleStatus::CompleteNegative,
            None,
            None,
            presentation,
            start,
            &before,
            ops,
            &counts,
        ))
    }
}
