//! Finite event replay from independently reconstructed coefficient maps.
//! No producer implementation or checker is linked or invoked.
use super::{budget::Budget, schema::*};
use crate::common::digest;
use anyhow::{ensure, Context, Result};
use std::collections::{BTreeMap, BTreeSet};

pub fn add(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 + b as u128) % r as u128) as u64
}
pub fn mul(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}
pub fn normalize<K: Ord + Copy>(
    input: &[(K, u64)],
    r: u64,
    b: &mut Budget,
) -> Result<Vec<(K, u64)>> {
    b.work(input.len())?;
    let mut map = BTreeMap::new();
    for &(k, v) in input {
        let e = map.entry(k).or_insert(0);
        *e = add(*e, v, r);
    }
    Ok(map.into_iter().filter(|(_, v)| *v != 0).collect())
}
fn sum<K: Ord + Copy>(
    a: &[(K, u64)],
    s: u64,
    c: &[(K, u64)],
    r: u64,
    b: &mut Budget,
) -> Result<Vec<(K, u64)>> {
    b.work(a.len() + c.len())?;
    let mut m: BTreeMap<K, u64> = a.iter().copied().collect();
    for &(k, v) in c {
        let e = m.entry(k).or_insert(0);
        *e = add(*e, mul(s, v, r), r);
    }
    Ok(m.into_iter().filter(|(_, v)| *v != 0).collect())
}
fn axpy(a: &Row, s: u64, c: &Row, r: u64, b: &mut Budget) -> Result<Row> {
    b.work(1)?;
    Ok(Row {
        small: sum(&a.small, s, &c.small, r, b)?,
        large: sum(&a.large, s, &c.large, r, b)?,
        rhs: add(a.rhs, mul(s, c.rhs, r), r),
        sources: sum(&a.sources, s, &c.sources, r, b)?,
    })
}
fn initial(a: &Partial) -> Row {
    Row {
        small: a.small.clone(),
        large: a.large.clone(),
        rhs: a.rhs,
        sources: vec![(a.id, 1)],
    }
}
fn canonical<K: Ord>(terms: &[(K, u64)], r: u64) -> bool {
    terms.iter().all(|(_, v)| *v > 0 && *v < r) && terms.windows(2).all(|w| w[0].0 < w[1].0)
}

struct Cursor<'a> {
    run: &'a Run,
    index: usize,
    work: u64,
    ack_work: u64,
    inv: u64,
    ack_inv: u64,
    bytes: u64,
    refused: u64,
    report: Report,
    rows: Vec<Complete>,
    completed_terms: usize,
    stopped: bool,
}
impl<'a> Cursor<'a> {
    fn new(run: &'a Run, count: usize) -> Self {
        Self {
            run,
            index: 0,
            work: 0,
            ack_work: 0,
            inv: 0,
            ack_inv: 0,
            bytes: 0,
            refused: 0,
            report: Report {
                input_relations: count,
                ..Report::default()
            },
            rows: vec![],
            completed_terms: 0,
            stopped: false,
        }
    }
    fn peek(&self) -> Option<&Event> {
        self.run
            .events
            .get(self.index)
            .or(self.run.unacknowledged.as_ref())
    }
    fn admit(&mut self, p: &Reservation) -> Result<bool> {
        let stop = if p.state_terms > self.run.limits.row_terms {
            Some(Stop::StateLimit)
        } else if self.index as u64 >= self.run.limits.events {
            Some(Stop::EventLimit)
        } else if p.work > self.run.limits.work.saturating_sub(self.work) {
            Some(Stop::WorkLimit)
        } else {
            None
        };
        if let Some(stop) = stop {
            ensure!(
                self.index == self.run.events.len()
                    && self.run.stop == stop
                    && self.run.next_reservation.as_ref() == Some(p)
                    && self.run.unacknowledged.is_none(),
                "filter reservation refusal"
            );
            self.stopped = true;
            Ok(false)
        } else {
            Ok(true)
        }
    }
    fn event(
        &mut self,
        p: Reservation,
        t: Transition,
        report: Report,
        b: &mut Budget,
    ) -> Result<bool> {
        let expected = Event {
            sequence: self.index as u64,
            reservation: p,
            transition: t,
            report_after: report,
        };
        let actual = self.peek().context("missing expected filter event")?;
        b.work(1 + expected.reservation.state_terms)?;
        ensure!(
            *actual == expected,
            "filter transition/report at {}",
            self.index
        );
        let n = serde_json::to_vec(&expected)?.len() as u64;
        let inv = u64::from(matches!(expected.transition, Transition::Reduction { .. }));
        self.work = self
            .work
            .checked_add(expected.reservation.work)
            .context("filter work overflow")?;
        self.inv += inv;
        if self.index == self.run.events.len() {
            ensure!(self.run.next_reservation.is_none(), "mixed native refusal");
            if n > self.run.limits.bytes.saturating_sub(self.bytes) {
                ensure!(
                    self.run.stop == Stop::ByteLimit && self.run.sink_error.is_none(),
                    "filter byte refusal"
                );
            } else {
                ensure!(
                    self.run.stop == Stop::SinkFailure
                        && self
                            .run
                            .sink_error
                            .as_ref()
                            .is_some_and(|s| s.chars().count() <= 1024),
                    "filter unacknowledged event without refusal"
                );
            }
            self.refused = n;
            self.stopped = true;
            return Ok(false);
        }
        ensure!(
            n <= self.run.limits.bytes.saturating_sub(self.bytes),
            "acknowledged filter bytes exceed cap"
        );
        self.bytes += n;
        self.ack_work = self.work;
        self.ack_inv = self.inv;
        self.report = expected.report_after;
        if let Transition::CompleteRow { row, .. } = expected.transition {
            self.completed_terms += row.terms();
            self.rows.push(row.complete());
        }
        self.index += 1;
        Ok(true)
    }
    fn finish(&self, completed: bool) -> Result<()> {
        let r = self.run;
        ensure!(
            self.index == r.events.len() && r.report == self.report && r.rows == self.rows,
            "filter final prefix differs"
        );
        ensure!(
            r.computed_work == self.work
                && r.acknowledged_work == self.ack_work
                && r.inverse_calls == self.inv
                && r.acknowledged_inverse_calls == self.ack_inv
                && r.acknowledged_bytes == self.bytes
                && r.unacknowledged_bytes == self.refused,
            "filter native totals differ"
        );
        ensure!(r.report_final == completed, "filter report final");
        if completed {
            ensure!(
                r.stop == Stop::Completed
                    && r.next_reservation.is_none()
                    && r.unacknowledged.is_none()
                    && r.sink_error.is_none(),
                "false native completion"
            );
        } else {
            ensure!(
                self.stopped && r.stop != Stop::Completed,
                "unjustified native censor"
            );
            if r.unacknowledged.is_none() {
                ensure!(r.sink_error.is_none(), "orphan sink error");
            }
        }
        Ok(())
    }
}

pub fn verify(inputs: &[Partial], options: &Options, run: &Run, b: &mut Budget) -> Result<()> {
    let r = options.modulus;
    ensure!(
        (3..1u64 << 63).contains(&r) && r % 2 == 1,
        "invalid scalar modulus"
    );
    let h = Limits::policy();
    let l = &run.limits;
    ensure!(
        l.input_rows <= h.input_rows
            && l.input_terms <= h.input_terms
            && l.row_terms <= h.row_terms
            && l.events <= h.events
            && l.bytes <= h.bytes
            && l.work <= h.work,
        "native limits"
    );
    ensure!(
        run.schema == "large-prime-filter-trace/v1"
            && run.options == *options
            && run.work_scope == WORK_SCOPE
            && run.byte_scope == BYTE_SCOPE,
        "native contract"
    );
    ensure!(
        !run.execution_trace_complete_verified
            && !run.total_operation_count_verified
            && !run.dependency_basis_complete_verified,
        "native scope overclaim"
    );
    let terms = inputs
        .iter()
        .try_fold(0usize, |n, p| {
            n.checked_add(p.small.len())?.checked_add(p.large.len())
        })
        .context("input term overflow")?;
    ensure!(
        run.input_rows == inputs.len() && run.input_terms == terms,
        "native input counts"
    );
    b.work(terms + inputs.len())?;
    b.scratch(
        terms
            .checked_mul(512)
            .and_then(|n| n.checked_add(inputs.len() * 1024))
            .context("consumer_resource_limit: sparse scratch")?,
    )?;
    let mut c = Cursor::new(run, inputs.len());
    if inputs.len() > l.input_rows || terms > l.input_terms {
        ensure!(
            run.stop == Stop::InputLimit
                && run.input_sha256.is_none()
                && run.events.is_empty()
                && run.unacknowledged.is_none()
                && run.next_reservation.is_none()
                && run.sink_error.is_none(),
            "input refusal"
        );
        c.stopped = true;
        return c.finish(false);
    }
    let mut ids = BTreeSet::new();
    ensure!(
        inputs.iter().all(|p| ids.insert(p.id)),
        "duplicate source id"
    );
    let sha = digest(&serde_json::to_vec(inputs)?);
    ensure!(
        run.input_sha256.as_ref() == Some(&sha),
        "native ordered input commitment"
    );
    let mut report = c.report.clone();
    macro_rules! admit {
        ($p:expr) => {
            if !c.admit(&$p)? {
                return c.finish(false);
            }
        };
    }
    macro_rules! event {
        ($p:expr,$t:expr) => {
            if !c.event($p, $t, report.clone(), b)? {
                return c.finish(false);
            }
        };
    }
    let p = Reservation::new(Phase::Begin, 0, 0);
    admit!(p);
    event!(
        p,
        Transition::Begin {
            input_sha256: sha,
            input_rows: inputs.len(),
            input_terms: terms,
            options: options.clone(),
            limits: l.clone()
        }
    );
    let mut retained = Vec::new();
    let mut duplicates = BTreeMap::new();
    let mut normalized = BTreeMap::new();
    for (position, raw) in inputs.iter().enumerate() {
        let p = Reservation::new(
            Phase::Normalize,
            raw.small.len() + raw.large.len() + 1,
            raw.small.len() + raw.large.len() + 2,
        );
        admit!(p);
        let row = Partial {
            id: raw.id,
            small: normalize(&raw.small, r, b)?,
            large: normalize(&raw.large, r, b)?,
            rhs: raw.rhs % r,
        };
        event!(
            p,
            Transition::Normalized {
                position,
                row: row.clone()
            }
        );
        normalized.insert(row.id, row.clone());
        let key = (row.small.clone(), row.large.clone(), row.rhs);
        if let Some(&representative) = duplicates.get(&key) {
            let p = Reservation::new(Phase::Duplicate, 0, 0);
            admit!(p);
            report.duplicate_relations += 1;
            event!(
                p,
                Transition::DuplicateDropped {
                    input_id: row.id,
                    representative
                }
            );
        } else {
            duplicates.insert(key, row.id);
            retained.push(row);
        }
    }
    let mut incidence: BTreeMap<u64, BTreeSet<usize>> = BTreeMap::new();
    for (i, row) in retained.iter().enumerate() {
        b.work(row.large.len())?;
        for &(col, _) in &row.large {
            incidence.entry(col).or_default().insert(i);
        }
    }
    report.input_large_vertices = incidence.len();
    let mut active = vec![true; retained.len()];
    while incidence.values().any(|v| v.len() == 1) {
        b.work(incidence.len())?;
        let choice = c
            .peek()
            .map(|e| &e.reservation)
            .or(run.next_reservation.as_ref())
            .and_then(|p| p.choice)
            .context("peel choice absent")?;
        let incident = incidence.get(&choice).context("peel unknown column")?;
        ensure!(incident.len() == 1, "peel is not singleton");
        let i = *incident.first().unwrap();
        let row = &retained[i];
        let mut p = Reservation::new(
            Phase::Peel,
            row.large.len(),
            row.small.len() + row.large.len() + 2,
        );
        p.choice = Some(choice);
        admit!(p);
        let before: Vec<_> = row
            .large
            .iter()
            .map(|(k, _)| (*k, incidence[k].len()))
            .collect();
        let after: Vec<_> = before.iter().map(|(k, n)| (*k, n - 1)).collect();
        report.peeled_relations += 1;
        event!(
            p,
            Transition::Peel {
                column: choice,
                input_id: row.id,
                degrees_before: before,
                degrees_after: after
            }
        );
        active[i] = false;
        for (col, _) in &row.large {
            incidence.get_mut(col).unwrap().remove(&i);
        }
    }
    let smooth: Vec<_> = retained.iter().filter(|p| p.large.is_empty()).collect();
    let mut core: Vec<_> = retained
        .iter()
        .enumerate()
        .filter(|(i, p)| active[*i] && !p.large.is_empty())
        .map(|(_, p)| p)
        .collect();
    core.sort_by_key(|p| (p.large.len(), p.small.len(), p.id));
    let core_terms = core.iter().map(|p| p.large.len()).sum();
    let p = Reservation::new(Phase::Core, core_terms, 0);
    admit!(p);
    b.work(core_terms)?;
    let mut degrees = BTreeMap::new();
    for row in &core {
        for (col, _) in &row.large {
            *degrees.entry(*col).or_insert(0) += 1usize;
        }
    }
    report.core_relations = core.len();
    report.core_large_vertices = degrees.len();
    event!(
        p,
        Transition::CoreReady {
            row_ids: core.iter().map(|p| p.id).collect(),
            smooth_ids: smooth.iter().map(|p| p.id).collect(),
            degrees: degrees.iter().map(|(k, v)| (*k, *v)).collect()
        }
    );
    for raw in smooth {
        b.scratch(
            (terms * 4 + c.completed_terms * 3 + raw.small.len() * 3) * 128 + inputs.len() * 1024,
        )?;
        let p = Reservation::new(Phase::Complete, 0, raw.small.len() + 2);
        admit!(p);
        report.complete_relations += 1;
        event!(
            p,
            Transition::CompleteRow {
                reference: RowRef {
                    input_id: raw.id,
                    version: 0
                },
                row: initial(raw)
            }
        );
    }
    let mut pivots: BTreeMap<u64, (RowRef, Row)> = BTreeMap::new();
    let mut pivot_terms = 0usize;
    for raw in core {
        let p = Reservation::new(Phase::Row, 0, raw.small.len() + raw.large.len() + 2);
        admit!(p);
        let mut row = initial(raw);
        let mut reference = RowRef {
            input_id: raw.id,
            version: 0,
        };
        event!(
            p,
            Transition::RowBegin {
                reference: reference.clone(),
                row: row.clone()
            }
        );
        loop {
            b.scratch(
                (terms * 4 + pivot_terms * 2 + c.completed_terms * 3 + row.terms() * 4) * 128
                    + inputs.len() * 1024,
            )?;
            if row.large.is_empty() {
                let p = Reservation::new(Phase::Complete, 0, row.terms());
                admit!(p);
                check_combination(&row, &normalized, r, b)?;
                report.complete_relations += 1;
                report.cycles_closed += usize::from(row.sources.len() > 1);
                event!(
                    p,
                    Transition::CompleteRow {
                        reference: reference.clone(),
                        row: row.clone()
                    }
                );
                break;
            }
            let p = Reservation::new(Phase::Select, row.large.len(), row.terms());
            admit!(p);
            b.work(row.large.len())?;
            let (column, coefficient) = *row
                .large
                .iter()
                .min_by_key(|(k, v)| {
                    (
                        !pivots.contains_key(k),
                        (degrees.get(k).copied().unwrap_or(1) - 1) * (row.large.len() - 1),
                        *k,
                        *v,
                    )
                })
                .unwrap();
            let score = (degrees.get(&column).copied().unwrap_or(1) - 1) * (row.large.len() - 1);
            event!(
                p,
                Transition::PivotSelected {
                    reference: reference.clone(),
                    column,
                    pivot: pivots.get(&column).map(|p| p.0.clone()),
                    score
                }
            );
            if let Some((pr, pivot)) = pivots.get(&column) {
                let units = row.terms() + pivot.terms() - 1;
                let p = Reservation::new(Phase::Reduce, units, units);
                admit!(p);
                let (inverse, scale) = match &c.peek().context("missing reduction")?.transition {
                    Transition::Reduction { inverse, scale, .. } => (*inverse, *scale),
                    _ => anyhow::bail!("expected reduction"),
                };
                let pc = pivot.large.iter().find(|(k, _)| *k == column).unwrap().1;
                ensure!(
                    inverse < r
                        && scale < r
                        && mul(inverse, pc, r) == 1
                        && add(coefficient, mul(scale, pc, r), r) == 0,
                    "invalid inverse/cancellation"
                );
                let candidate = axpy(&row, scale, pivot, r, b)?;
                let accepted = candidate.small.len() <= options.max_small_weight
                    && candidate.sources.len() <= options.max_sources;
                if accepted {
                    report.merges += 1;
                } else {
                    report.rejected_fill += 1;
                }
                event!(
                    p,
                    Transition::Reduction {
                        reference: reference.clone(),
                        column,
                        pivot: pr.clone(),
                        inverse,
                        scale,
                        candidate: candidate.clone(),
                        accepted
                    }
                );
                if !accepted {
                    break;
                }
                row = candidate;
                reference.version += 1;
            } else {
                let p = Reservation::new(Phase::Store, 0, row.terms());
                admit!(p);
                report.pivots += 1;
                event!(
                    p,
                    Transition::PivotStored {
                        reference: reference.clone(),
                        column,
                        row: row.clone()
                    }
                );
                pivot_terms += row.terms();
                pivots.insert(column, (reference, row));
                break;
            }
        }
    }
    let p = Reservation::new(Phase::Finish, 0, 0);
    admit!(p);
    b.scratch((terms * 4 + pivot_terms * 2 + c.completed_terms * 3) * 128 + inputs.len() * 1024)?;
    let rows = c.rows.clone();
    event!(p, Transition::Finish { rows });
    c.finish(true)
}
fn check_combination(
    row: &Row,
    inputs: &BTreeMap<u64, Partial>,
    r: u64,
    b: &mut Budget,
) -> Result<()> {
    ensure!(
        canonical(&row.sources, r) && !row.sources.is_empty(),
        "noncanonical source combination"
    );
    let mut total = Row::default();
    for (id, k) in &row.sources {
        let input = inputs.get(id).context("unknown combination source")?;
        total = axpy(&total, *k, &initial(input), r, b)?;
    }
    ensure!(total == *row, "source combination does not reproduce row");
    Ok(())
}
