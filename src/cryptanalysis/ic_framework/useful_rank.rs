//! Group-verified row accounting. Rank includes the target column; base rank does not.
//! Receipts are bounded; counters remain exact after the transcript fills, except
//! duplicate counts, whose checked prefix is explicitly reported.
use super::stages::InstanceCtx;
use crate::cryptanalysis::ic_boundary::{
    invmod, CountedGroup, FactorBase, GroupOps, IncrementalGauss, PhaseCost, RelationSolver,
    RowStatus,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, time::Instant};

const RECEIPT_BYTES: usize = 8 * 1024 * 1024;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RowReceipt {
    pub row: Vec<u64>,
    pub rhs: u64,
    pub group_verified: bool,
    pub duplicate: bool,
    pub rank_before: usize,
    pub rank_after: usize,
    pub base_rank_after: usize,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct RankAudit {
    pub schema: String,
    pub instance: String,
    pub column_keys: Vec<u64>,
    pub modulus: u64,
    pub cofactor: u64,
    pub generator_key: u64,
    pub target_key: u64,
    pub rows_seen: u64,
    pub verified_rows: u64,
    pub invalid_rows: u64,
    pub independent_rows: u64,
    pub dependent_rows: u64,
    /// Exact over `duplicate_checks` rows; a lower bound if checks were omitted.
    pub duplicate_rows: u64,
    pub duplicate_checks: u64,
    pub duplicate_checks_omitted: u64,
    pub rank: usize,
    pub base_rank: usize,
    pub transcript_complete: bool,
    pub receipts: Vec<RowReceipt>,
}

/// Replay includes base-column and subgroup checks. The caller supplies the
/// exact archived base and workload (G,Q,r,h); a transcript is not self-authenticating.
pub fn replay<G: CountedGroup>(
    ctx: &InstanceCtx<G>,
    fb: &FactorBase<G::Elt>,
    report: &RankAudit,
) -> Result<(), String> {
    if !report.transcript_complete {
        return Err("rank transcript is truncated".into());
    }
    let inner = Box::new(IncrementalGauss::new(fb.columns + 1, ctx.r));
    let mut matrix = AuditedMatrix::new(ctx, fb, inner)?;
    for receipt in &report.receipts {
        matrix.add_row(receipt.row.clone(), receipt.rhs);
    }
    if &matrix.audit != report {
        return Err("rank transcript replay mismatch".into());
    }
    if matrix.audit.invalid_rows != 0 {
        return Err("rank transcript contains invalid group equations".into());
    }
    Ok(())
}

pub struct AuditedMatrix<'a, G: CountedGroup> {
    ctx: &'a InstanceCtx<'a, G>,
    inner: Box<dyn RelationSolver>,
    columns: Vec<G::Elt>,
    base_matrix: IncrementalGauss,
    seen: BTreeSet<Vec<u64>>,
    receipt_bytes: usize,
    pub audit: RankAudit,
    /// Row wall time is already inside the collector's LA interval.
    pub cost: PhaseCost,
    /// Construction occurs before the collector's interval and must be added.
    pub setup_ns: u64,
}
impl<'a, G: CountedGroup> AuditedMatrix<'a, G> {
    pub fn new(
        ctx: &'a InstanceCtx<'a, G>,
        fb: &FactorBase<G::Elt>,
        inner: Box<dyn RelationSolver>,
    ) -> Result<Self, String> {
        let started = Instant::now();
        if fb.columns == 0 || fb.columns > 4096 {
            return Err("rank audit requires 1..4096 columns".into());
        }
        if inner.work().1 != "row_ops" || inner.rank() != 0 {
            return Err("rank audit requires a fresh incremental row_ops solver".into());
        }
        if ctx.r <= 2 || !crate::cryptanalysis::challenge_ladder::is_prime_u64(ctx.r) {
            return Err("rank audit requires an odd prime modulus".into());
        }
        if fb.points.len() != fb.col_of.len() || fb.points.len() != fb.coef_of.len() {
            return Err("invalid factor-base column map".into());
        }
        if ctx.cofactor == 0
            || ctx.cofactor.is_multiple_of(ctx.r)
            || ctx.group.is_identity(&ctx.generator)
        {
            return Err(
                "rank audit requires nonzero subgroup generator and invertible cofactor".into(),
            );
        }
        let mut ops = GroupOps::default();
        let mut columns = vec![None; fb.columns];
        for (i, &point) in fb.points.iter().enumerate() {
            let col = fb.col_of[i];
            if col >= fb.columns {
                return Err("factor-base column out of range".into());
            }
            let coef = fb.coef_of[i] % ctx.r;
            let inverse = invmod(coef, ctx.r).ok_or("zero factor-base coefficient")?;
            let cleared = ctx.group.mul(&mut ops, point, ctx.cofactor);
            let canonical = ctx.group.mul(&mut ops, cleared, inverse);
            if let Some(prior) = columns[col] {
                if prior != canonical {
                    return Err("inconsistent factor-base quotient map".into());
                }
            } else {
                columns[col] = Some(canonical);
            }
        }
        let mut columns = columns
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or("unused factor-base column")?;
        columns.push(ctx.target);
        for point in columns.iter().chain(std::iter::once(&ctx.generator)) {
            if !ctx
                .group
                .is_identity(&ctx.group.mul(&mut ops, *point, ctx.r))
            {
                return Err("rank audit point outside subgroup".into());
            }
        }
        Ok(Self {
            ctx,
            inner,
            columns: columns.clone(),
            base_matrix: IncrementalGauss::new(fb.columns, ctx.r),
            seen: BTreeSet::new(),
            receipt_bytes: 0,
            audit: RankAudit {
                schema: "ic-useful-rank/v1".into(),
                instance: ctx.name.clone(),
                column_keys: columns.iter().map(|p| ctx.group.key(p)).collect(),
                modulus: ctx.r,
                cofactor: ctx.cofactor,
                generator_key: ctx.group.key(&ctx.generator),
                target_key: ctx.group.key(&ctx.target),
                transcript_complete: true,
                ..Default::default()
            },
            cost: PhaseCost {
                group_ops: ops,
                ..Default::default()
            },
            setup_ns: started.elapsed().as_nanos() as u64,
        })
    }
}
impl<G: CountedGroup> RelationSolver for AuditedMatrix<'_, G> {
    fn add_row(&mut self, mut row: Vec<u64>, rhs: u64) -> RowStatus {
        self.audit.rows_seen += 1;
        let r = self.ctx.r;
        for v in &mut row {
            *v %= r;
        }
        let rhs = rhs % r;
        let before = self.inner.rank();
        let mut sum = self.ctx.group.identity();
        let mut valid = row.len() == self.columns.len();
        if valid {
            for (&v, &point) in row.iter().zip(&self.columns) {
                if v != 0 {
                    let term = self.ctx.group.mul(&mut self.cost.group_ops, point, v);
                    sum = self.ctx.group.add(&mut self.cost.group_ops, sum, term);
                }
            }
            valid = sum
                == self
                    .ctx
                    .group
                    .mul(&mut self.cost.group_ops, self.ctx.generator, rhs);
        }
        let bytes = row.len().saturating_add(8).saturating_mul(16);
        let retain = self.audit.transcript_complete
            && self.receipt_bytes.saturating_add(bytes) <= RECEIPT_BYTES;
        let mut duplicate = false;
        if valid && retain {
            let mut key = row.clone();
            key.push(rhs);
            if let Some(&first) = key.iter().find(|&&v| v != 0) {
                let inv = invmod(first, r).expect("prime nonzero coefficient");
                for v in &mut key {
                    *v = (*v as u128 * inv as u128 % r as u128) as u64;
                }
            }
            duplicate = !self.seen.insert(key);
            self.audit.duplicate_checks += 1;
            self.audit.duplicate_rows += u64::from(duplicate);
            self.cost
                .count("rank_normalized_entries_unpriced", row.len() as u64 + 1);
        } else if valid {
            self.audit.duplicate_checks_omitted += 1;
        }
        let status = if valid {
            self.audit.verified_rows += 1;
            // Projection onto base columns is a coefficient-space rank, not
            // a claim that target-dependent rows supply reusable base logs.
            self.base_matrix.add_row(row[..row.len() - 1].to_vec(), 0);
            self.inner.add_row(row.clone(), rhs)
        } else {
            self.audit.invalid_rows += 1;
            RowStatus::Inconsistent
        };
        let after = self.inner.rank();
        if valid {
            if after > before {
                self.audit.independent_rows += 1;
            } else {
                self.audit.dependent_rows += 1;
            }
        }
        self.audit.rank = after;
        self.audit.base_rank = self.base_matrix.rank();
        if retain {
            self.receipt_bytes += bytes;
            self.audit.receipts.push(RowReceipt {
                row,
                rhs,
                group_verified: valid,
                duplicate,
                rank_before: before,
                rank_after: after,
                base_rank_after: self.audit.base_rank,
            });
        } else {
            self.audit.transcript_complete = false;
        }
        status
    }
    fn pinned(&self, col: usize) -> Option<u64> {
        self.inner.pinned(col)
    }
    fn rank(&self) -> usize {
        self.inner.rank()
    }
    fn dependent(&self) -> u64 {
        self.inner.dependent()
    }
    fn work(&self) -> (u64, &'static str) {
        (self.inner.work().0 + self.base_matrix.row_ops, "row_ops")
    }
}
