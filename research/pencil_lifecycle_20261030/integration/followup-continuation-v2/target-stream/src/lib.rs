//! Offline validation and replay of one pencil target/attempt.
//! These ranks describe supplied equations; they do not verify geometric relations
//! or group scalars. Never call this from the metered producer.
use pencil_relation_information_adapter::from_augmented_rows;
use pencil_symbolic_graph::{
    collector::{Admission, Collector},
    residual::Limits,
};
use serde::Serialize;

#[derive(Clone, Copy, Debug)]
pub struct ReplayLimits {
    pub max_events: usize,
    pub max_columns: usize,
    pub max_terms: usize,
    /// Sum of dense cells submitted to all prefix diagnostics, across both spaces.
    pub max_prefix_cells: usize,
}
impl Default for ReplayLimits {
    fn default() -> Self {
        Self {
            max_events: 10_000,
            max_columns: 4096,
            max_terms: 1_000_000,
            max_prefix_cells: 20_000_000,
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct PrefixInformation {
    pub rows: usize,
    pub nuisance_rank: usize,
    pub nuisance_and_target_rank: usize,
    pub full_rank: usize,
    pub consistent: bool,
    pub target_identifiable: bool,
}
fn prefix(p: u64, n: usize, rows: &[Vec<u64>]) -> Result<PrefixInformation, String> {
    let (_, i) = from_augmented_rows(p, n, rows)?;
    Ok(PrefixInformation {
        rows: rows.len(),
        nuisance_rank: i.nuisance_rank,
        nuisance_and_target_rank: i.nuisance_and_target_rank,
        full_rank: i.full_rank,
        consistent: i.consistent,
        target_identifiable: i.target_determined,
    })
}
fn dense(p: u64, width: usize, terms: &[(usize, u64)]) -> Result<Vec<u64>, String> {
    let mut row = vec![0; width];
    for &(i, c) in terms {
        if i >= width {
            return Err("term column outside fixed map".into());
        }
        if c >= p {
            return Err("noncanonical submitted coefficient".into());
        }
        row[i] = ((row[i] as u128 + c as u128) % p as u128) as u64;
    }
    Ok(row)
}
fn incorporated(a: Admission) -> bool {
    matches!(
        a,
        Admission::Joined | Admission::Anchored | Admission::Independent | Admission::Dependent
    )
}
fn actual_pending(c: &Collector) -> Option<Vec<(usize, u64)>> {
    c.pending_constraint()
        .map(|row| row.iter().map(|(&i, &v)| (i, v)).collect())
}
fn candidate_scalar(c: &Collector, a: Admission) -> Result<Option<u64>, String> {
    if a != Admission::Independent {
        return Ok(None);
    }
    let target = c
        .small_columns()
        .len()
        .checked_sub(2)
        .ok_or("missing label columns")?;
    if c.basis().pivot(target + 1).is_some() {
        return Err("inconsistent independent relation: native assertion would fail".into());
    }
    Ok(c.basis()
        .pivot(target)
        .map(|row| row.get(&(target + 1)).copied().unwrap_or(0)))
}

use pencil_symbolic_graph::relation_stream::{
    Candidate, Convention, Disposition, Event, Header, Record, Terminal,
};

#[derive(Debug, Serialize)]
pub struct Prefix {
    pub sequence: u64,
    pub walk_index: u64,
    pub lambda: u64,
    pub admission: pencil_symbolic_graph::relation_stream::Admission,
    pub generated_reported_verified_inputs: PrefixInformation,
    pub incorporated_inputs: PrefixInformation,
    pub retained_residual_rank: usize,
    pub pending_constraint_unincorporated: Option<Vec<(usize, u64)>>,
    pub candidate_scalar: Option<u64>,
}
#[derive(Debug, Serialize)]
pub struct ReplayReport {
    pub schema: &'static str,
    pub header: Header,
    pub syntactically_complete: bool,
    pub records: usize,
    pub candidates: usize,
    pub prefixes: Vec<Prefix>,
    pub terminal: Terminal,
    pub scalar_independently_verified: bool,
    pub geometric_relations_independently_verified: bool,
    pub scope: &'static str,
}

fn validate_header(h: &Header, limits: ReplayLimits) -> Result<(), String> {
    let hex = |s: &str, n| s.len() == n && s.bytes().all(|b| b.is_ascii_hexdigit());
    if h.schema != "pencil-relation-stream/v1" || h.convention != Convention::PencilAugmentedRhs {
        return Err(
            "unsupported stream schema/convention; signed and Kummer adapters are not implemented"
                .into(),
        );
    }
    if !hex(&h.identity.source_revision, 40)
        || !hex(&h.identity.source_sha256, 64)
        || h.identity.workload_id.is_empty()
        || h.identity.run_id.is_empty()
    {
        return Err("invalid source/workload/run identity".into());
    }
    if h.p <= 13
        || h.p >= 1 << 31
        || h.p % 4 != 3
        || h.r <= 2
        || h.order == 0
        || h.order % h.r != 0
        || h.cofactor != h.order / h.r
    {
        return Err("invalid Scholten base/scalar/order domain".into());
    }
    pencil_target_information::information(h.p, 0, &[])?;
    pencil_target_information::information(h.r, 0, &[])?;
    if h.source_coefficients.len() != 4
        || h.polynomial.len() != 6
        || h.polynomial[5] == 0
        || h.source_coefficients.iter().any(|&v| v >= h.p * h.p)
        || h.polynomial.iter().any(|&v| v >= h.p)
        || h.generator.infinity
        || [&h.generator, &h.target]
            .iter()
            .any(|v| v.x >= h.p * h.p || v.y >= h.p * h.p)
    {
        return Err("invalid curve/point encoding".into());
    }
    let width = h.nuisance_columns.checked_add(2).ok_or("column overflow")?;
    if width > limits.max_columns
        || h.x_to_column.len() != h.p as usize
        || h.x_to_column.len() > limits.max_terms
    {
        return Err("fixed column-map dimensions or replay limit".into());
    }
    let mut next = 0;
    for col in h.x_to_column.iter().flatten() {
        if *col != next {
            return Err("column map must enumerate unique identity-order nodes".into());
        }
        next += 1;
    }
    if next != h.nuisance_columns
        || h.column_map_version != 0
        || ![2, 4, 8, 16].contains(&h.denominator)
    {
        return Err("column-map version/count/partition policy mismatch".into());
    }
    let small: Vec<_> = (0..width)
        .filter(|&i| i >= h.nuisance_columns || i % h.denominator == 0)
        .collect();
    if small != h.small_columns {
        return Err("small-column map disagrees with fixed partition".into());
    }
    Ok(())
}
fn expected_row(h: &Header, c: &Candidate) -> Result<Vec<(usize, u64)>, String> {
    let mut row = Vec::new();
    for &[x, y] in &c.points {
        if x >= h.p || y >= h.p {
            return Err("noncanonical geometric point encoding".into());
        }
        if y == 0 {
            continue;
        }
        let col = h.x_to_column[x as usize].ok_or("nonzero point has no column in fixed map")?;
        row.push((col, if y <= h.p / 2 { 1 } else { h.r - 1 }));
    }
    row.push((h.nuisance_columns, c.b));
    row.push((h.nuisance_columns + 1, if c.a == 0 { 0 } else { h.r - c.a }));
    Ok(row)
}

/// The expected header must come from the caller's separately pinned workload,
/// not be copied from an untrusted stream. Equality binds every header field.
pub fn replay(
    records: &[Record],
    expected: &Header,
    limits: ReplayLimits,
) -> Result<ReplayReport, String> {
    if records.len() < 2 || records.len() > limits.max_events {
        return Err("missing header/terminal or event limit".into());
    }
    let h = match &records[0].event {
        Event::Header(h) => h,
        _ => return Err("first event must be Header".into()),
    };
    if h != expected {
        return Err("stream header differs from independently expected workload/source/curve/target binding".into());
    }
    validate_header(h, limits)?;
    let width = h.nuisance_columns + 2;
    let partition: Vec<_> = (0..width)
        .map(|i| h.small_columns.binary_search(&i).is_ok())
        .collect();
    let mut collector = Collector::new(
        h.r,
        &partition,
        Limits {
            retained_coefficients: h.coefficient_cap,
            working_coefficients: h.coefficient_cap,
        },
    )
    .map_err(|e| format!("collector: {e:?}"))?;
    let mut generated = Vec::new();
    let mut admitted = Vec::new();
    let mut prefixes = Vec::new();
    let mut pending: Option<Candidate> = None;
    let mut terminal = None;
    let mut candidate_count = 0;
    let mut last_candidate: Option<Candidate> = None;
    let mut stop_admission = None;
    let mut scalar = None;
    let mut term_count = 0usize;
    let mut charged_cells = 0usize;
    let mut attempted_lambdas = 0usize;
    let mut rejected_candidates = 0usize;
    let mut verified_relations = 0usize;
    for (index, record) in records.iter().enumerate() {
        if record.sequence != index as u64 {
            return Err(format!("sequence mismatch at record {index}"));
        }
        if index == 0 {
            continue;
        }
        if terminal.is_some() {
            return Err("event after terminal".into());
        }
        if (stop_admission.is_some() || scalar.is_some())
            && !matches!(record.event, Event::Terminal(_))
        {
            return Err("nonterminal event after cap or scalar candidate".into());
        }
        match &record.event {
            Event::Header(_) => return Err("repeated header".into()),
            Event::Candidate(c) => {
                if pending.is_some() {
                    return Err("verified candidate missing adjacent relation".into());
                }
                if c.walk_index >= 16
                    || c.a >= h.r
                    || c.b == 0
                    || c.b >= h.r
                    || c.divisor.degree > 2
                    || c.divisor.u.iter().chain(&c.divisor.v).any(|&v| v >= h.p)
                    || c.divisor.u[c.divisor.degree] != 1
                {
                    return Err("invalid candidate labels/divisor/walk encoding".into());
                }
                if let Some(old) = &last_candidate {
                    if c.walk_index < old.walk_index
                        || (c.walk_index == old.walk_index
                            && (old.lambda.is_none()
                                || c.lambda.is_none()
                                || c.lambda <= old.lambda))
                    {
                        return Err("candidate walk/lambda order is not strictly increasing".into());
                    }
                    if c.walk_index == old.walk_index
                        && (c.a != old.a || c.b != old.b || c.divisor != old.divisor)
                    {
                        return Err("candidate changed fixed walk labels/divisor".into());
                    }
                }
                match c.disposition {
                    Disposition::DegreeSkip => {
                        if c.divisor.degree == 2 || c.lambda.is_some() || !c.points.is_empty() {
                            return Err("invalid degree-skip disposition".into());
                        }
                    }
                    Disposition::Rejected | Disposition::Verified => {
                        if c.divisor.degree != 2 || c.lambda.is_none() || c.lambda.unwrap() >= h.p {
                            return Err("invalid lambda candidate disposition".into());
                        }
                        if c.disposition == Disposition::Rejected && !c.points.is_empty() {
                            return Err("rejected candidate carries relation points".into());
                        }
                        if c.disposition == Disposition::Verified {
                            if c.points.len() != 3 {
                                return Err(
                                    "verified pencil complement must contain three points".into()
                                );
                            }
                            let _ = expected_row(h, c)?;
                            pending = Some(c.clone());
                        }
                    }
                }
                candidate_count += 1;
                attempted_lambdas += usize::from(c.lambda.is_some());
                rejected_candidates += usize::from(c.disposition == Disposition::Rejected);
                verified_relations += usize::from(c.disposition == Disposition::Verified);
                last_candidate = Some(c.clone());
            }
            Event::Relation(r) => {
                let c = pending
                    .take()
                    .ok_or("relation without adjacent verified candidate")?;
                if r.walk_index != c.walk_index || Some(r.lambda) != c.lambda {
                    return Err("relation candidate identity mismatch".into());
                }
                if r.submitted != expected_row(h, &c)? {
                    return Err(
                        "submitted terms disagree with point signs/labels/pencil RHS convention"
                            .into(),
                    );
                }
                term_count = term_count
                    .checked_add(r.submitted.len())
                    .ok_or("term count overflow")?;
                if term_count > limits.max_terms {
                    return Err("term limit".into());
                }
                let row = dense(h.r, width, &r.submitted)?;
                let actual = collector
                    .insert(&r.submitted)
                    .map_err(|e| format!("collector replay: {e:?}"))?;
                if pencil_symbolic_graph::relation_stream::Admission::from(actual) != r.admission {
                    return Err("collector admission mismatch".into());
                }
                if actual_pending(&collector) != r.pending_constraint {
                    return Err(
                        "pending-constraint mismatch (small-column coordinates required)".into(),
                    );
                }
                if collector.basis().rank() != r.residual_rank {
                    return Err("retained residual rank mismatch".into());
                }
                let found = candidate_scalar(&collector, actual)?;
                if found != r.candidate_scalar {
                    return Err("scalar candidate differs from replayed target pivot".into());
                }
                scalar = found;
                if matches!(
                    actual,
                    Admission::StoppedWorkingCap | Admission::StoppedRetainedCap
                ) {
                    stop_admission = Some(actual);
                }
                if actual == Admission::AlreadyStopped {
                    return Err("already-stopped admission is not a legal producer event".into());
                }
                generated.push(row.clone());
                if incorporated(actual) {
                    admitted.push(row);
                }
                let cells = generated
                    .len()
                    .checked_add(admitted.len())
                    .and_then(|n| n.checked_mul(width))
                    .ok_or("prefix accounting overflow")?;
                charged_cells = charged_cells
                    .checked_add(cells)
                    .ok_or("prefix accounting overflow")?;
                if charged_cells > limits.max_prefix_cells {
                    return Err("offline prefix diagnostic work limit".into());
                }
                prefixes.push(Prefix {
                    sequence: record.sequence,
                    walk_index: r.walk_index,
                    lambda: r.lambda,
                    admission: r.admission,
                    generated_reported_verified_inputs: prefix(
                        h.r,
                        h.nuisance_columns,
                        &generated,
                    )?,
                    incorporated_inputs: prefix(h.r, h.nuisance_columns, &admitted)?,
                    retained_residual_rank: r.residual_rank,
                    pending_constraint_unincorporated: r.pending_constraint.clone(),
                    candidate_scalar: found,
                });
            }
            Event::Terminal(t) => {
                if pending.is_some() {
                    return Err("terminal truncates pending verified candidate".into());
                }
                let minimum_walks = last_candidate
                    .as_ref()
                    .map(|c| c.walk_index + 1)
                    .unwrap_or(0);
                if t.walks > 16
                    || t.walks < minimum_walks
                    || t.candidate_lambdas < attempted_lambdas
                    || t.verified_relations != verified_relations
                    || t.rejected_candidates != rejected_candidates
                    || u128::from(t.submitted_rows) != collector.counts().submitted_rows
                    || u128::from(t.admitted_rows) != collector.counts().admitted_rows
                {
                    return Err("terminal counts disagree with captured prefix".into());
                }
                if (scalar.is_some() || stop_admission.is_some()) && t.walks != minimum_walks {
                    return Err("terminal walk count after early stop".into());
                }
                if scalar.is_none()
                    && stop_admission.is_none()
                    && (t.walks != 16 || t.candidate_lambdas != attempted_lambdas)
                {
                    return Err("walk-cap terminal is not exhausted".into());
                }
                if t.recovered != scalar
                    || t.recovered.is_some_and(|k| k >= h.r)
                    || (t.source_verified && t.recovered.is_none())
                {
                    return Err("terminal scalar/verification mismatch".into());
                }
                let reason = match stop_admission {
                    Some(Admission::StoppedWorkingCap) => Some("symbolic_working_cap"),
                    Some(Admission::StoppedRetainedCap) => Some("symbolic_retained_cap"),
                    _ => {
                        if scalar.is_none() {
                            Some("walk_cap")
                        } else {
                            None
                        }
                    }
                };
                if t.failure.as_deref() != reason {
                    return Err("terminal reason does not match replay state".into());
                }
                terminal = Some(t.clone());
            }
        }
    }
    let terminal = terminal.ok_or("truncated stream: terminal record required")?;
    Ok(ReplayReport{schema:"pencil-target-prefix-information/v1",header:h.clone(),syntactically_complete:true,records:records.len(),candidates:candidate_count,prefixes,terminal,
        scalar_independently_verified:false,geometric_relations_independently_verified:false,
        scope:"syntactic terminal only, not authenticated successful capture; offline fixed pencil input-row prefixes; generated producer-attested verified rows and successfully incorporated inputs are separate; cap pending is excluded; source group verification is reported only"})
}

/// Native producer JSONL or an explicit JSON array packet. Byte limits belong to
/// the caller; this decoder rejects empty/truncated records and extra JSON data.
pub fn decode(input: &str) -> Result<Vec<Record>, String> {
    if input.trim_start().starts_with('[') {
        return serde_json::from_str(input).map_err(|e| format!("JSON packet: {e}"));
    }
    if !input.ends_with('\n') {
        return Err("JSONL lacks final record newline (possibly truncated)".into());
    }
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if line.trim().is_empty() {
                return Err(format!("empty JSONL record {i}"));
            }
            serde_json::from_str(line).map_err(|e| format!("JSONL record {i}: {e}"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pencil_symbolic_graph::relation_stream::{Divisor, Identity, Point, Relation};
    fn header(n: usize, cap: usize) -> Header {
        let mut map = vec![None; 19];
        for (i, v) in map.iter_mut().take(n).enumerate() {
            *v = Some(i);
        }
        Header {
            schema: "pencil-relation-stream/v1".into(),
            identity: Identity {
                source_revision: "a".repeat(40),
                source_sha256: "b".repeat(64),
                workload_id: "algebra-fixture".into(),
                run_id: "fixed-1".into(),
            },
            convention: Convention::PencilAugmentedRhs,
            p: 19,
            r: 7,
            order: 7,
            cofactor: 1,
            seed: 0,
            source_coefficients: vec![1, 0, 0, 1],
            polynomial: vec![1, 0, 0, 0, 0, 1],
            generator: Point {
                x: 1,
                y: 2,
                infinity: false,
            },
            target: Point {
                x: 3,
                y: 4,
                infinity: false,
            },
            x_to_column: map,
            nuisance_columns: n,
            small_columns: (0..n + 2).filter(|&i| i >= n || i % 2 == 0).collect(),
            column_map_version: 0,
            denominator: 2,
            coefficient_cap: cap,
        }
    }
    fn candidate(points: Vec<[u64; 2]>, a: u64, b: u64, lambda: u64) -> Candidate {
        Candidate {
            walk_index: 0,
            lambda: Some(lambda),
            a,
            b,
            divisor: Divisor {
                degree: 2,
                u: [0, 0, 1],
                v: [0, 0],
            },
            points,
            disposition: Disposition::Verified,
        }
    }
    fn fixture(h: &Header, cs: Vec<Candidate>) -> Vec<Record> {
        let partition: Vec<_> = (0..h.nuisance_columns + 2)
            .map(|i| h.small_columns.contains(&i))
            .collect();
        let mut collector = Collector::new(
            h.r,
            &partition,
            Limits {
                retained_coefficients: h.coefficient_cap,
                working_coefficients: h.coefficient_cap,
            },
        )
        .unwrap();
        let mut events = vec![Event::Header(h.clone())];
        let mut recovered = None;
        let mut failure = Some("walk_cap".into());
        for c in cs {
            let row = expected_row(h, &c).unwrap();
            let actual = collector.insert(&row).unwrap();
            recovered = candidate_scalar(&collector, actual).unwrap();
            events.push(Event::Candidate(c.clone()));
            events.push(Event::Relation(Relation {
                walk_index: c.walk_index,
                lambda: c.lambda.unwrap(),
                submitted: row,
                admission: actual.into(),
                pending_constraint: actual_pending(&collector),
                residual_rank: collector.basis().rank(),
                candidate_scalar: recovered,
            }));
            if recovered.is_some() {
                failure = None;
                break;
            }
            if actual == Admission::StoppedWorkingCap {
                failure = Some("symbolic_working_cap".into());
                break;
            }
            if actual == Admission::StoppedRetainedCap {
                failure = Some("symbolic_retained_cap".into());
                break;
            }
        }
        let candidate_events: Vec<_> = events
            .iter()
            .filter_map(|e| {
                if let Event::Candidate(c) = e {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();
        let walks = if recovered.is_some() || collector.stopped().is_some() {
            candidate_events
                .last()
                .map(|c| c.walk_index + 1)
                .unwrap_or(0)
        } else {
            16
        };
        events.push(Event::Terminal(Terminal {
            walks,
            candidate_lambdas: candidate_events.len(),
            verified_relations: candidate_events.len(),
            rejected_candidates: 0,
            submitted_rows: collector.counts().submitted_rows.try_into().unwrap(),
            admitted_rows: collector.counts().admitted_rows.try_into().unwrap(),
            recovered,
            source_verified: recovered.is_some(),
            failure,
        }));
        events
            .into_iter()
            .enumerate()
            .map(|(i, event)| Record {
                sequence: i as u64,
                event,
            })
            .collect()
    }
    #[test]
    fn pencil_signs_duplicates_zero_and_target_candidate() {
        let h = header(1, 100);
        let c = candidate(vec![[0, 1], [0, 18], [0, 0]], 2, 1, 0);
        let records = fixture(&h, vec![c]);
        let report = replay(&records, &h, ReplayLimits::default()).unwrap();
        assert_eq!(report.prefixes[0].candidate_scalar, Some(5));
        assert!(report.prefixes[0].incorporated_inputs.target_identifiable);
        assert!(!report.scalar_independently_verified);
        assert!(!report.geometric_relations_independently_verified);
        if let Event::Relation(r) = &records[2].event {
            assert_eq!(r.submitted, vec![(0, 1), (0, 6), (1, 1), (2, 5)]);
        } else {
            panic!();
        }
    }
    #[test]
    fn cap_pending_never_enters_admitted_space() {
        let h = header(1, 0);
        let rows = fixture(&h, vec![candidate(vec![[0, 1], [0, 18], [0, 0]], 2, 1, 0)]);
        let report = replay(&rows, &h, ReplayLimits::default()).unwrap();
        let p = &report.prefixes[0];
        assert_eq!(p.generated_reported_verified_inputs.rows, 1);
        assert!(p.generated_reported_verified_inputs.target_identifiable);
        assert_eq!(p.incorporated_inputs.rows, 0);
        assert!(!p.incorporated_inputs.target_identifiable);
        assert_eq!(p.retained_residual_rank, 0);
        assert!(p.pending_constraint_unincorporated.is_some());
    }
    #[test]
    fn rejected_large_row_has_generated_but_no_admitted_prefix() {
        let h = header(6, 100);
        let rows = fixture(&h, vec![candidate(vec![[1, 1], [3, 1], [5, 1]], 0, 1, 0)]);
        let report = replay(&rows, &h, ReplayLimits::default()).unwrap();
        assert_eq!(
            report.prefixes[0].admission,
            pencil_symbolic_graph::relation_stream::Admission::RejectedThreeLarge
        );
        assert_eq!(
            report.prefixes[0].generated_reported_verified_inputs.rows,
            1
        );
        assert_eq!(report.prefixes[0].incorporated_inputs.rows, 0);
    }
    #[test]
    fn corruption_controls() {
        let h = header(1, 100);
        let good = fixture(&h, vec![candidate(vec![[0, 1], [0, 18], [0, 0]], 2, 1, 0)]);
        let bad = |rows: &[Record]| assert!(replay(rows, &h, ReplayLimits::default()).is_err());
        let mut x = good.clone();
        x[2].sequence = 1;
        bad(&x);
        let mut x = good.clone();
        x.pop();
        bad(&x);
        let mut x = good.clone();
        x.push(Record {
            sequence: 4,
            event: good[1].event.clone(),
        });
        bad(&x);
        let mut x = good.clone();
        if let Event::Relation(r) = &mut x[2].event {
            r.submitted.last_mut().unwrap().1 = 2;
        }
        bad(&x);
        let mut x = good.clone();
        if let Event::Relation(r) = &mut x[2].event {
            r.submitted[0].1 = 8;
        }
        bad(&x);
        let mut x = good.clone();
        if let Event::Relation(r) = &mut x[2].event {
            r.admission = pencil_symbolic_graph::relation_stream::Admission::Dependent;
        }
        bad(&x);
        let mut x = good.clone();
        if let Event::Relation(r) = &mut x[2].event {
            r.pending_constraint = Some(vec![]);
        }
        bad(&x);
        let mut x = good.clone();
        if let Event::Relation(r) = &mut x[2].event {
            r.candidate_scalar = Some(1);
        }
        bad(&x);
        let mut x = good.clone();
        if let Event::Terminal(t) = &mut x[3].event {
            t.recovered = None;
        }
        bad(&x);
        let mut x = good.clone();
        x.remove(2);
        for (i, r) in x.iter_mut().enumerate() {
            r.sequence = i as u64;
        }
        bad(&x);
        let mut expected = h.clone();
        expected.target.x += 1;
        assert!(replay(&good, &expected, ReplayLimits::default()).is_err());
        let mut expected = h.clone();
        expected.identity.workload_id = "other".into();
        assert!(replay(&good, &expected, ReplayLimits::default()).is_err());
    }
    #[test]
    fn bad_domains_maps_limits_and_unknown_contracts() {
        for mutate in [0, 1, 2, 3, 4] {
            let mut h = header(1, 100);
            match mutate {
                0 => h.r = 9,
                1 => h.column_map_version = 1,
                2 => h.x_to_column[0] = Some(1),
                3 => h.small_columns = vec![1, 2],
                _ => h.identity.source_revision = "missing".into(),
            };
            let rows = vec![
                Record {
                    sequence: 0,
                    event: Event::Header(h.clone()),
                },
                Record {
                    sequence: 1,
                    event: Event::Terminal(Terminal {
                        walks: 16,
                        candidate_lambdas: 0,
                        verified_relations: 0,
                        rejected_candidates: 0,
                        submitted_rows: 0,
                        admitted_rows: 0,
                        recovered: None,
                        source_verified: false,
                        failure: Some("walk_cap".into()),
                    }),
                },
            ];
            assert!(replay(&rows, &h, ReplayLimits::default()).is_err());
        }
        let h = header(1, 100);
        let rows = fixture(&h, vec![candidate(vec![[0, 1], [0, 18], [0, 0]], 2, 1, 0)]);
        let mut lim = ReplayLimits::default();
        lim.max_prefix_cells = 0;
        assert!(replay(&rows, &h, lim).is_err());
        let json = serde_json::to_string(&rows).unwrap();
        assert_eq!(decode(&json).unwrap(), rows);
        let lines = rows
            .iter()
            .map(|r| serde_json::to_string(r).unwrap() + "\n")
            .collect::<String>();
        assert_eq!(decode(&lines).unwrap(), rows);
        assert!(decode(lines.trim_end()).is_err());
        assert!(decode(&(lines + "{}\n")).is_err());
        assert!(decode(&json.replace("PencilAugmentedRhs", "SignedGraph")).is_err());
    }
    #[test]
    fn dispositions_and_candidate_order_are_checked() {
        let h = header(1, 100);
        let skip = Candidate {
            walk_index: 0,
            lambda: None,
            a: 1,
            b: 1,
            divisor: Divisor {
                degree: 0,
                u: [1, 0, 0],
                v: [0, 0],
            },
            points: vec![],
            disposition: Disposition::DegreeSkip,
        };
        let rejected = Candidate {
            walk_index: 1,
            lambda: Some(2),
            a: 2,
            b: 1,
            divisor: Divisor {
                degree: 2,
                u: [0, 0, 1],
                v: [0, 0],
            },
            points: vec![],
            disposition: Disposition::Rejected,
        };
        let t = Terminal {
            walks: 16,
            candidate_lambdas: 1,
            verified_relations: 0,
            rejected_candidates: 1,
            submitted_rows: 0,
            admitted_rows: 0,
            recovered: None,
            source_verified: false,
            failure: Some("walk_cap".into()),
        };
        let rows: Vec<_> = vec![
            Event::Header(h.clone()),
            Event::Candidate(skip),
            Event::Candidate(rejected),
            Event::Terminal(t),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, event)| Record {
            sequence: i as u64,
            event,
        })
        .collect();
        let report = replay(&rows, &h, ReplayLimits::default()).unwrap();
        assert_eq!(report.candidates, 2);
        assert!(report.prefixes.is_empty());
        let mut x = rows.clone();
        if let Event::Candidate(c) = &mut x[2].event {
            c.walk_index = 0;
        }
        assert!(replay(&x, &h, ReplayLimits::default()).is_err());
        let mut x = rows.clone();
        if let Event::Terminal(t) = &mut x[3].event {
            t.rejected_candidates = 0;
        }
        assert!(replay(&x, &h, ReplayLimits::default()).is_err());
        let mut x = rows.clone();
        if let Event::Candidate(c) = &mut x[1].event {
            c.disposition = Disposition::Verified;
        }
        assert!(replay(&x, &h, ReplayLimits::default()).is_err());
    }
}
