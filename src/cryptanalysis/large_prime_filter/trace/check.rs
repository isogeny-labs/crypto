//! Independent arithmetic and recorded-schedule verification. Never calls the
//! native filter, WorkRow::axpy, native normalization or native replay helper.
use super::*;

fn require(value: bool, reason: &str) -> Result<(), String> {
    if value {
        Ok(())
    } else {
        Err(reason.into())
    }
}
fn add(a: u64, b: u64, r: u64) -> u64 {
    ((u128::from(a) + u128::from(b)) % u128::from(r)) as u64
}
fn mul(a: u64, b: u64, r: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(r)) as u64
}
fn normalized<K: Copy + Ord>(terms: impl IntoIterator<Item = (K, u64)>, r: u64) -> Vec<(K, u64)> {
    let mut out = BTreeMap::new();
    for (k, v) in terms {
        let old = out.get(&k).copied().unwrap_or(0);
        out.insert(k, add(old, v % r, r));
    }
    out.into_iter().filter(|(_, v)| *v != 0).collect()
}
fn initial(row: &PartialRelation) -> RowSnapshot {
    RowSnapshot {
        small: row.small.clone(),
        large: row.large.clone(),
        rhs: row.rhs,
        sources: vec![(row.id, 1)],
    }
}
fn combined(a: &RowSnapshot, b: &RowSnapshot, s: u64, r: u64) -> RowSnapshot {
    fn terms<K: Copy + Ord>(a: &[(K, u64)], b: &[(K, u64)], s: u64, r: u64) -> Vec<(K, u64)> {
        normalized(
            a.iter()
                .copied()
                .chain(b.iter().map(|&(k, v)| (k, mul(s, v, r)))),
            r,
        )
    }
    RowSnapshot {
        small: terms(&a.small, &b.small, s, r),
        large: terms(&a.large, &b.large, s, r),
        rhs: add(a.rhs, mul(s, b.rhs, r), r),
        sources: terms(&a.sources, &b.sources, s, r),
    }
}

struct Replay<'a> {
    run: &'a FilterRun,
    index: usize,
    work: u64,
    ack_work: u64,
    inverses: u64,
    ack_inverses: u64,
    bytes: u64,
    attempted_bytes: u64,
    report: LargePrimeFilterReport,
    rows: Vec<CompleteRelation>,
    stopped: bool,
}
impl Replay<'_> {
    fn next(&self) -> Option<&FilterEvent> {
        self.run
            .events
            .get(self.index)
            .or(self.run.unacknowledged.as_ref())
    }
    fn admit(&mut self, plan: Reservation) -> Result<bool, String> {
        let limits = self.run.limits;
        let stop = if plan.state_terms > limits.row_terms {
            Some(FilterStop::StateLimit)
        } else if self.index as u64 >= limits.events {
            Some(FilterStop::EventLimit)
        } else if plan.work > limits.work - self.work {
            Some(FilterStop::WorkLimit)
        } else {
            None
        };
        if let Some(stop) = stop {
            require(
                self.index == self.run.events.len()
                    && self.run.unacknowledged.is_none()
                    && self.run.stop == stop
                    && self.run.next_reservation == Some(plan),
                "unjustified trace reservation stop",
            )?;
            self.stopped = true;
            return Ok(false);
        }
        require(
            self.next().is_some(),
            "unjustified truncated filter schedule",
        )?;
        Ok(true)
    }
    fn emit(
        &mut self,
        plan: Reservation,
        transition: Transition,
        report: LargePrimeFilterReport,
    ) -> Result<bool, String> {
        let expected = FilterEvent {
            sequence: self.index as u64,
            reservation: plan,
            transition,
            report_after: report,
        };
        require(
            self.next() == Some(&expected),
            "filter transition, arithmetic, source combination or report mismatch",
        )?;
        self.work += plan.work;
        let inv = u64::from(matches!(expected.transition, Transition::Reduction { .. }));
        self.inverses += inv;
        let bytes = serde_json::to_vec(&expected)
            .map_err(|e| e.to_string())?
            .len() as u64;
        let byte_stop = bytes > self.run.limits.bytes - self.bytes;
        if self.index == self.run.events.len() {
            require(
                self.run.next_reservation.is_none(),
                "unexpected reservation after computed event",
            )?;
            if byte_stop {
                require(
                    self.run.stop == FilterStop::ByteLimit && self.run.sink_error.is_none(),
                    "unjustified trace byte stop",
                )?;
            } else {
                require(
                    self.run.stop == FilterStop::SinkFailure
                        && self
                            .run
                            .sink_error
                            .as_ref()
                            .is_some_and(|s| s.len() <= 4096),
                    "unjustified trace sink stop",
                )?;
            }
            self.attempted_bytes = bytes;
            self.stopped = true;
            return Ok(false);
        }
        require(!byte_stop, "acknowledged filter bytes exceed cap")?;
        self.bytes += bytes;
        self.ack_work += plan.work;
        self.ack_inverses += inv;
        self.report = expected.report_after;
        if let Transition::CompleteRow { row, .. } = expected.transition {
            self.rows.push(row.complete());
        }
        self.index += 1;
        Ok(true)
    }
}

/// Verify input binding, every recorded row/source identity and the bounded
/// algorithm schedule using independent coefficient arithmetic. A successful
/// check of a censored run proves only its supplied prefix, never UNSAT or a
/// complete dependency basis. Sink failure is a reported failure boundary,
/// not proof of external durable storage. No producer algorithm is executed.
pub fn verify_filter_trace(
    inputs: &[PartialRelation],
    options: &LargePrimeFilterOptions,
    run: &FilterRun,
) -> Result<(), String> {
    require(
        (3..1u64 << 63).contains(&options.modulus) && options.modulus % 2 == 1,
        "invalid filter modulus",
    )?;
    require(
        run.schema == SCHEMA
            && run.options == *options
            && run.limits.valid()
            && run.work_scope == WORK_SCOPE
            && run.byte_scope == BYTE_SCOPE
            && !run.execution_trace_complete_verified
            && !run.total_operation_count_verified
            && !run.dependency_basis_complete_verified,
        "filter binding, limits or scope mismatch",
    )?;
    let terms = raw_terms(inputs).unwrap_or(usize::MAX);
    require(
        run.input_rows == inputs.len() && run.input_terms == terms,
        "filter raw input size mismatch",
    )?;
    let initial_report = LargePrimeFilterReport {
        input_relations: inputs.len(),
        ..Default::default()
    };
    let input_capped = inputs.len() > run.limits.input_rows || terms > run.limits.input_terms;
    if input_capped {
        require(
            run.stop == FilterStop::InputLimit
                && run.input_sha256.is_none()
                && run.events.is_empty()
                && run.unacknowledged.is_none()
                && run.next_reservation.is_none()
                && run.sink_error.is_none()
                && run.computed_work == 0
                && run.acknowledged_work == 0
                && run.inverse_calls == 0
                && run.acknowledged_inverse_calls == 0
                && run.acknowledged_bytes == 0
                && run.unacknowledged_bytes == 0
                && run.rows.is_empty()
                && run.report == initial_report
                && !run.report_final,
            "invalid raw input admission stop",
        )?;
        return Ok(());
    }
    require(
        run.input_sha256.as_deref() == Some(input_hash(inputs).as_str()),
        "filter raw ordered input digest mismatch",
    )?;
    let mut ids = BTreeSet::new();
    require(
        inputs.iter().all(|r| ids.insert(r.id)),
        "duplicate input source id",
    )?;
    require(
        run.events.len() as u64 + u64::from(run.unacknowledged.is_some()) <= run.limits.events,
        "too many supplied filter events",
    )?;
    let mut replay = Replay {
        run,
        index: 0,
        work: 0,
        ack_work: 0,
        inverses: 0,
        ack_inverses: 0,
        bytes: 0,
        attempted_bytes: 0,
        report: initial_report,
        rows: vec![],
        stopped: false,
    };
    schedule(inputs, &mut replay)?;
    require(
        replay.index == run.events.len()
            && replay.work == run.computed_work
            && replay.ack_work == run.acknowledged_work
            && replay.inverses == run.inverse_calls
            && replay.ack_inverses == run.acknowledged_inverse_calls
            && replay.bytes == run.acknowledged_bytes
            && replay.attempted_bytes == run.unacknowledged_bytes
            && replay.rows == run.rows
            && replay.report == run.report
            && run.report_final == !replay.stopped,
        "filter prefix aggregate mismatch",
    )?;
    if !replay.stopped {
        require(
            run.stop == FilterStop::Completed
                && run.unacknowledged.is_none()
                && run.next_reservation.is_none()
                && run.sink_error.is_none(),
            "invalid final filter state",
        )?;
    } else if run.unacknowledged.is_none() {
        require(run.sink_error.is_none(), "unexpected sink failure message")?;
    }
    Ok(())
}

fn schedule(inputs: &[PartialRelation], a: &mut Replay<'_>) -> Result<(), String> {
    let options = a.run.options;
    let r = options.modulus;
    let mut report = a.report.clone();
    macro_rules! admit {
        ($p:expr) => {
            if !a.admit($p)? {
                return Ok(());
            }
        };
    }
    macro_rules! emit {
        ($p:expr,$t:expr) => {
            if !a.emit($p, $t, report.clone())? {
                return Ok(());
            }
        };
    }
    let p = Reservation::new(Phase::Begin, 0, 0);
    admit!(p);
    emit!(
        p,
        Transition::Begin {
            input_sha256: input_hash(inputs),
            input_rows: inputs.len(),
            input_terms: raw_terms(inputs).unwrap(),
            options,
            limits: a.run.limits
        }
    );
    let mut rows: Vec<PartialRelation> = Vec::new();
    for (position, input) in inputs.iter().enumerate() {
        let p = Reservation::new(
            Phase::Normalize,
            input.small.len() + input.large.len() + 1,
            input.small.len() + input.large.len() + 2,
        );
        admit!(p);
        let row = PartialRelation {
            id: input.id,
            small: normalized(input.small.iter().copied(), r),
            large: normalized(input.large.iter().copied(), r),
            rhs: input.rhs % r,
        };
        emit!(
            p,
            Transition::Normalized {
                position,
                row: row.clone()
            }
        );
        if let Some(old) = rows
            .iter()
            .find(|old| old.small == row.small && old.large == row.large && old.rhs == row.rhs)
        {
            let p = Reservation::new(Phase::Duplicate, 0, 0);
            admit!(p);
            report.duplicate_relations += 1;
            emit!(
                p,
                Transition::DuplicateDropped {
                    input_id: row.id,
                    representative: old.id
                }
            );
        } else {
            rows.push(row);
        }
    }
    let mut active: BTreeSet<_> = rows.iter().map(|row| row.id).collect();
    let mut degrees = BTreeMap::<u64, usize>::new();
    for row in &rows {
        for &(k, _) in &row.large {
            *degrees.entry(k).or_default() += 1;
        }
    }
    report.input_large_vertices = degrees.len();
    while degrees.values().any(|&d| d == 1) {
        let plan = a
            .next()
            .map(|e| e.reservation)
            .or(a.run.next_reservation)
            .ok_or("missing peel or censored reservation")?;
        let column = plan.choice.ok_or("missing peel column")?;
        require(
            degrees.get(&column) == Some(&1),
            "peel column is not singleton",
        )?;
        let row = rows
            .iter()
            .find(|row| active.contains(&row.id) && row.large.iter().any(|t| t.0 == column))
            .ok_or("peel has no active row")?;
        let mut p = Reservation::new(
            Phase::Peel,
            row.large.len(),
            row.small.len() + row.large.len() + 2,
        );
        p.choice = Some(column);
        admit!(p);
        let before: Vec<_> = row.large.iter().map(|t| (t.0, degrees[&t.0])).collect();
        require(
            before.iter().all(|t| t.1 > 0),
            "invalid active incidence degree",
        )?;
        let after: Vec<_> = before.iter().map(|&(k, d)| (k, d - 1)).collect();
        report.peeled_relations += 1;
        emit!(
            p,
            Transition::Peel {
                column,
                input_id: row.id,
                degrees_before: before,
                degrees_after: after.clone()
            }
        );
        active.remove(&row.id);
        for (k, d) in after {
            degrees.insert(k, d);
        }
    }
    let smooth: Vec<_> = rows.iter().filter(|row| row.large.is_empty()).collect();
    let mut core: Vec<_> = rows
        .iter()
        .filter(|row| active.contains(&row.id) && !row.large.is_empty())
        .collect();
    core.sort_by_key(|row| (row.large.len(), row.small.len(), row.id));
    let p = Reservation::new(Phase::Core, core.iter().map(|row| row.large.len()).sum(), 0);
    admit!(p);
    let mut core_degrees = BTreeMap::<u64, usize>::new();
    for row in &core {
        for &(k, _) in &row.large {
            *core_degrees.entry(k).or_default() += 1;
        }
    }
    report.core_relations = core.len();
    report.core_large_vertices = core_degrees.len();
    emit!(
        p,
        Transition::CoreReady {
            row_ids: core.iter().map(|row| row.id).collect(),
            smooth_ids: smooth.iter().map(|row| row.id).collect(),
            degrees: core_degrees.iter().map(|(&k, &v)| (k, v)).collect()
        }
    );
    for input in smooth {
        let p = Reservation::new(Phase::Complete, 0, input.small.len() + 2);
        admit!(p);
        report.complete_relations += 1;
        emit!(
            p,
            Transition::CompleteRow {
                reference: RowRef {
                    input_id: input.id,
                    version: 0
                },
                row: initial(input)
            }
        );
    }
    let mut pivots = BTreeMap::<u64, (RowRef, RowSnapshot)>::new();
    for input in core {
        let mut reference = RowRef {
            input_id: input.id,
            version: 0,
        };
        let p = Reservation::new(Phase::Row, 0, input.small.len() + input.large.len() + 2);
        admit!(p);
        let mut row = initial(input);
        emit!(
            p,
            Transition::RowBegin {
                reference,
                row: row.clone()
            }
        );
        loop {
            if row.large.is_empty() {
                let p = Reservation::new(Phase::Complete, 0, row.terms());
                admit!(p);
                report.complete_relations += 1;
                report.cycles_closed += usize::from(row.sources.len() > 1);
                emit!(
                    p,
                    Transition::CompleteRow {
                        reference,
                        row: row.clone()
                    }
                );
                break;
            }
            let p = Reservation::new(Phase::Select, row.large.len(), row.terms());
            admit!(p);
            let mut choices: Vec<_> = row
                .large
                .iter()
                .map(|&(k, v)| {
                    let score = core_degrees.get(&k).copied().unwrap_or(1).saturating_sub(1)
                        * (row.large.len() - 1);
                    (usize::from(!pivots.contains_key(&k)), score, k, v)
                })
                .collect();
            choices.sort_unstable();
            let (_, score, column, coefficient) = choices[0];
            let pivot_ref = pivots.get(&column).map(|p| p.0);
            emit!(
                p,
                Transition::PivotSelected {
                    reference,
                    column,
                    pivot: pivot_ref,
                    score
                }
            );
            if let Some((pivot_ref, pivot)) = pivots.get(&column) {
                let p = Reservation::new(
                    Phase::Reduce,
                    row.terms() + pivot.terms() - 1,
                    row.terms() + pivot.terms() - 1,
                );
                admit!(p);
                // Inverse is supplied and checked multiplicatively; no producer
                // Euclidean inverse routine is reused.
                let next = a.next().ok_or("missing reduction")?;
                let Transition::Reduction { inverse, scale, .. } = &next.transition else {
                    return Err("expected reduction".into());
                };
                let (inverse, scale) = (*inverse, *scale);
                let pc = pivot
                    .large
                    .iter()
                    .find(|t| t.0 == column)
                    .ok_or("missing pivot coefficient")?
                    .1;
                require(
                    inverse < r
                        && scale < r
                        && mul(inverse, pc, r) == 1
                        && add(coefficient, mul(scale, pc, r), r) == 0,
                    "invalid inverse or cancellation scale",
                )?;
                let candidate = combined(&row, pivot, scale, r);
                let accepted = candidate.small.len() <= options.max_small_weight
                    && candidate.sources.len() <= options.max_sources;
                if accepted {
                    report.merges += 1;
                } else {
                    report.rejected_fill += 1;
                }
                emit!(
                    p,
                    Transition::Reduction {
                        reference,
                        column,
                        pivot: *pivot_ref,
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
                emit!(
                    p,
                    Transition::PivotStored {
                        reference,
                        column,
                        row: row.clone()
                    }
                );
                pivots.insert(column, (reference, row));
                break;
            }
        }
    }
    let p = Reservation::new(Phase::Finish, 0, 0);
    admit!(p);
    emit!(
        p,
        Transition::Finish {
            rows: a.rows.clone()
        }
    );
    Ok(())
}
