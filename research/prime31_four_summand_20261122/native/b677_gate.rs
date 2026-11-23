//! Frozen B677 correctness gate. The witness and independent integer arithmetic
//! below belong to this executable grader, never to the candidate library API.

use crypto_lib::cryptanalysis::f4_fp::F4ResourceLimits;
use prime31_algebraic_ic::{Curve, Point, PRIME31};
use prime31_four_summand::b677_fixture::Hybrid677;
use prime31_four_summand::extractor::{extract_cofactor, extract_hybrid677, Limits, Status};
use prime31_four_summand::relation_adapter::{
    AdapterError, Disposition, RelationAdapter, SignedRelation,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::io::Write;
use std::time::Instant;

#[derive(Clone, Copy, Default, Serialize)]
struct GraderWork {
    field_products: u64,
    field_inversions: u64,
    group_additions: u64,
}

#[derive(Default)]
struct IndependentGrader {
    work: GraderWork,
}

impl IndependentGrader {
    fn product(&mut self, a: u64, b: u64, p: u64) -> u64 {
        self.work.field_products += 1;
        ((a as u128 * b as u128) % p as u128) as u64
    }

    fn power(&mut self, mut a: u64, mut exponent: u64, p: u64) -> u64 {
        let mut result = 1;
        while exponent != 0 {
            if exponent & 1 != 0 {
                result = self.product(result, a, p);
            }
            a = self.product(a, a, p);
            exponent >>= 1;
        }
        result
    }

    fn on_curve(&mut self, e: Curve, point: Point) -> bool {
        let Some((x, y)) = point else {
            return true;
        };
        if x >= e.p || y >= e.p {
            return false;
        }
        let x2 = self.product(x, x, e.p);
        let x3 = self.product(x2, x, e.p);
        let ax = self.product(e.a, x, e.p);
        self.product(y, y, e.p) == (x3 + ax + e.b) % e.p
    }

    fn negative(e: Curve, point: Point) -> Point {
        point.map(|(x, y)| (x, (e.p - y) % e.p))
    }

    fn addition(&mut self, e: Curve, left: Point, right: Point) -> Result<Point, String> {
        self.work.group_additions += 1;
        let (Some((x1, y1)), Some((x2, y2))) = (left, right) else {
            return Ok(left.or(right));
        };
        if x1 == x2 && (y1 + y2) % e.p == 0 {
            return Ok(None);
        }
        let (numerator, denominator) = if x1 == x2 {
            if y1 != y2 {
                return Err("independent grader encountered incompatible equal-x points".into());
            }
            ((3 * self.product(x1, x1, e.p) + e.a) % e.p, 2 * y1 % e.p)
        } else {
            ((y2 + e.p - y1) % e.p, (x2 + e.p - x1) % e.p)
        };
        if denominator == 0 {
            return Err("independent grader encountered a zero affine denominator".into());
        }
        self.work.field_inversions += 1;
        let inverse = self.power(denominator, e.p - 2, e.p);
        let slope = self.product(numerator, inverse, e.p);
        let x3 = (self.product(slope, slope, e.p) + 2 * e.p - x1 - x2) % e.p;
        let y3 = (self.product(slope, (x1 + e.p - x3) % e.p, e.p) + e.p - y1) % e.p;
        let result = Some((x3, y3));
        if !self.on_curve(e, result) {
            return Err("independent grader produced an off-curve sum".into());
        }
        Ok(result)
    }

    fn scalar(&mut self, e: Curve, mut k: u64, mut point: Point) -> Result<Point, String> {
        let mut result = None;
        while k != 0 {
            if k & 1 != 0 {
                result = self.addition(e, result, point)?;
            }
            point = self.addition(e, point, point)?;
            k >>= 1;
        }
        Ok(result)
    }

    fn row(
        &mut self,
        e: Curve,
        base: &[(u64, u64)],
        row: &SignedRelation,
    ) -> Result<Point, String> {
        if row.terms.len() != 4 {
            return Err("RR5 grader requires exactly four distinct signed leaves".into());
        }
        let mut seen = BTreeSet::new();
        let mut result = None;
        for &(index, sign) in &row.terms {
            if index >= base.len() || !seen.insert(index) || !matches!(sign, -1 | 1) {
                return Err("invalid index, repeated x, or sign in extracted row".into());
            }
            let point = Some(base[index]);
            if !self.on_curve(e, point) {
                return Err("an extracted row refers to an off-curve base point".into());
            }
            let signed = if sign == 1 {
                point
            } else {
                Self::negative(e, point)
            };
            result = self.addition(e, result, signed)?;
        }
        Ok(result)
    }

    fn dickson1024(&mut self, mut x: u64) -> u64 {
        for _ in 0..10 {
            x = (self.product(x, x, PRIME31.p) + PRIME31.p - 2) % PRIME31.p;
        }
        x
    }

    fn positive(&mut self, fixture: &Hybrid677) -> Result<(Point, SignedRelation), String> {
        let mut dickson = Vec::new();
        let mut extras = Vec::new();
        for (index, &(x, _)) in fixture.points().iter().enumerate() {
            if self.dickson1024(x) == fixture.terminal() {
                dickson.push(index);
            } else {
                extras.push(index);
            }
        }
        if dickson.len() != 509 || extras.len() != 168 {
            return Err("independent grader cannot partition the frozen hybrid base".into());
        }
        let mut terms = vec![
            (dickson[0], 1),
            (dickson[1], -1),
            (extras[0], 1),
            (extras[1], 1),
        ];
        terms.sort_unstable_by_key(|term| term.0);
        let witness = SignedRelation { terms };
        let q = self.row(PRIME31, fixture.points(), &witness)?;
        Ok((q, witness))
    }
}

fn emit(label: &str, value: Value) -> Result<(), String> {
    println!("{label} {value}");
    std::io::stdout().flush().map_err(|err| err.to_string())
}

fn peak_rss_kib() -> Option<u64> {
    std::fs::read_to_string("/proc/self/status")
        .ok()?
        .lines()
        .find_map(|line| {
            line.strip_prefix("VmHWM:")?
                .split_whitespace()
                .next()?
                .parse()
                .ok()
        })
}

fn arguments() -> Result<(String, String), String> {
    let mut case = None;
    let mut presentation = None;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?;
        let destination = match flag.as_str() {
            "--case" => &mut case,
            "--presentation" => &mut presentation,
            _ => return Err(format!("unknown argument {flag}")),
        };
        if destination.replace(value).is_some() {
            return Err(format!("duplicate argument {flag}"));
        }
    }
    let case = case.ok_or("required: --case positive|h0|w0")?;
    let presentation = presentation.ok_or("required: --presentation structured|cofactor")?;
    if !matches!(case.as_str(), "positive" | "h0" | "w0") {
        return Err(format!("unregistered case {case}"));
    }
    if !matches!(presentation.as_str(), "structured" | "cofactor") {
        return Err(format!("unregistered presentation {presentation}"));
    }
    Ok((case, presentation))
}

fn run() -> Result<bool, String> {
    let (case, presentation) = arguments()?;
    let cold_start = Instant::now();
    let fixture_start = Instant::now();
    let fixture = Hybrid677::build()?;
    let fixture_seconds = fixture_start.elapsed().as_secs_f64();
    let mut grader = IndependentGrader::default();
    let query_start = Instant::now();
    let (q, expected) = match case.as_str() {
        "positive" => {
            let (q, witness) = grader.positive(&fixture)?;
            (q, Some(witness))
        }
        "h0" => (Some((18, 221_278_090)), None),
        "w0" => (Some((101_000_006, 1_077_058_598)), None),
        _ => unreachable!(),
    };
    let query_seconds = query_start.elapsed().as_secs_f64();
    let input_work = grader.work;
    let limits = Limits {
        max_degree: 10,
        seconds: 30,
        max_polynomial_terms: 20_000,
        max_assignments: 4096,
    };
    let resources = F4ResourceLimits {
        max_pairs: 20_000,
        max_symbolic_monomials: 8_192,
        max_matrix_entries: 2_000_000,
    };
    let input_valid = grader.on_curve(PRIME31, q);
    let chart_supported =
        q.is_some_and(|(x, _)| !fixture.points().iter().any(|point| point.0 == x));
    emit(
        "B677_INPUT",
        json!({
            "schema": "prime31-rr5-b677-input/v1",
            "scope": "bounded native correctness qualification; no ECDLP or speedup claim",
            "case": &case,
            "presentation": &presentation,
            "curve": PRIME31,
            "public_base": fixture.points(),
            "public_target": q,
            "fixture_certificate": fixture.certificate(),
            "dickson_terminal": fixture.terminal(),
            "public_coset_squares": fixture.coset_squares(),
            "fixture_setup_counts": fixture.counts(),
            "fixture_setup_seconds": fixture_seconds,
            "query_harness_seconds": query_seconds,
            "query_harness_work": input_work,
            "control_expected_row": &expected,
            "control_rule": expected.as_ref().map(|_| "first two Dickson and first two Y7 points, signs (+,-,+,+), sorted canonical base"),
            "public_walk_source": (case != "positive").then_some("prime31_run66_three_sum_support.rs: H0/W0 native 256-point walks"),
            "public_target_on_curve": input_valid,
            "supported_target_chart": chart_supported,
            "candidate_received_expected_row": false,
            "candidate_received_target_scalar": false,
            "candidate_received_factor_base_logs": false,
            "test_oracle_tuple_evaluations": 0,
            "limits": {
                "max_degree": limits.max_degree,
                "seconds": limits.seconds,
                "max_polynomial_terms": limits.max_polynomial_terms,
                "max_assignments": limits.max_assignments,
            },
            "resource_limits": {
                "max_pairs": resources.max_pairs,
                "max_symbolic_monomials": resources.max_symbolic_monomials,
                "max_matrix_entries": resources.max_matrix_entries,
            },
        }),
    )?;
    // The frozen input is flushed before even an unsupported-chart outcome.
    // The sole decomposition witness remains in this executable's grader.
    let candidate_start = Instant::now();
    let actual = if presentation == "structured" {
        extract_hybrid677(&fixture, q, limits, resources)
    } else {
        extract_cofactor(PRIME31, fixture.points(), q, limits, resources)
    };
    let candidate_seconds = candidate_start.elapsed().as_secs_f64();

    let mut failures = Vec::<String>::new();
    if !input_valid || !chart_supported {
        failures.push("the frozen public input fails its chart preflight".into());
    }
    if !matches!(actual.status, Status::Complete | Status::Empty) {
        failures.push("candidate did not complete the algebraic extraction gate".into());
    }
    if actual.diagnostics.rejected_assignments != 0 {
        failures.push("an algebraic assignment failed the candidate signed replay".into());
    }
    if actual.diagnostics.forbidden_branch_attempts != 0
        || actual.diagnostics.forbidden_domain_evaluations != 0
        || actual.diagnostics.forbidden_fallbacks_refused != 0
    {
        failures.push("candidate attempted a forbidden enumeration fallback".into());
    }
    let traversal_complete = actual.traversal.as_ref().is_some_and(|trace| {
        trace.complete
            && !trace.timed_out
            && !trace.trace_stopped
            && !trace.stopped_early
            && trace.resource_limit.is_none()
            && trace.full_field_branches == 0
            && trace.domain_evaluations == 0
    });
    if !traversal_complete {
        failures.push("native traversal does not certify all algebraic branches exhausted".into());
    }
    let expected_present = expected.as_ref().map(|witness| {
        actual
            .relations
            .iter()
            .any(|row| row.terms == witness.terms)
    });
    if expected_present == Some(false) {
        failures.push("the frozen positive control row was not recovered".into());
    }
    if expected.is_some() && actual.status != Status::Complete {
        failures.push("a known positive control cannot satisfy an empty-result gate".into());
    }
    if (actual.status == Status::Empty) != actual.relations.is_empty()
        && matches!(actual.status, Status::Complete | Status::Empty)
    {
        failures.push("candidate completion status disagrees with its returned rows".into());
    }

    let adapter_start = Instant::now();
    let mut adapter = RelationAdapter::new(PRIME31, fixture.points().to_vec(), q)
        .map_err(|err| format!("adapter setup failed: {err}"))?;
    let mut receipts = Vec::new();
    let mut row_replays = Vec::new();
    for row in &actual.relations {
        let replay = grader.row(PRIME31, fixture.points(), row);
        let valid = replay.as_ref().is_ok_and(|sum| *sum == q);
        row_replays.push(json!({"row": row, "valid": valid, "sum_or_error": replay}));
        if !valid {
            failures.push("independent integer replay rejected an extracted row".into());
            continue;
        }
        // Each actual extraction query is Q = [0]G + [1]Q. No invented RHS,
        // homogeneous cache, expected row, or eight-leaf difference is inserted.
        match adapter.accept(row, 0, 1) {
            Ok(receipt) => receipts.push(receipt),
            Err(error) => failures.push(format!("adapter rejected an authentic row: {error}")),
        }
    }
    let authentic_adapter_work = adapter.work();
    let authentic_adapter_arithmetic = adapter.arithmetic_counts();
    let authentic_adapter_seconds = adapter_start.elapsed().as_secs_f64();

    let mutations = if let Some(row) = actual.relations.first().filter(|row| {
        row.terms.len() == 4 && row.terms.iter().all(|(_, sign)| matches!(sign, -1 | 1))
    }) {
        let rank_before = adapter.rank();
        let scalar_before = adapter.scalar();
        let wrong_rhs = adapter.accept(row, 1, 1);
        let mut wrong_sign_row = row.clone();
        wrong_sign_row.terms[0].1 = -wrong_sign_row.terms[0].1;
        let wrong_sign = adapter.accept(&wrong_sign_row, 0, 1);
        let rejected_without_mutation = matches!(wrong_rhs, Err(AdapterError::GroupEqualityFailed))
            && matches!(wrong_sign, Err(AdapterError::GroupEqualityFailed))
            && adapter.rank() == rank_before
            && adapter.scalar() == scalar_before;
        let duplicate = adapter.accept(row, 0, 1);
        let negative_row = SignedRelation {
            terms: row
                .terms
                .iter()
                .map(|&(index, sign)| (index, -sign))
                .collect(),
        };
        let negative_duplicate = adapter.accept(&negative_row, 0, PRIME31.n - 1);
        let duplicate_checks = duplicate
            .as_ref()
            .is_ok_and(|receipt| receipt.disposition == Disposition::Duplicate)
            && negative_duplicate
                .as_ref()
                .is_ok_and(|receipt| receipt.disposition == Disposition::Duplicate);
        if !rejected_without_mutation || !duplicate_checks {
            failures.push("adapter mutation or duplicate control failed".into());
        }
        json!({
            "executed": true,
            "wrong_rhs": wrong_rhs,
            "wrong_sign": wrong_sign,
            "rejected_without_rank_or_scalar_mutation": rejected_without_mutation,
            "duplicate": duplicate,
            "negative_duplicate": negative_duplicate,
            "duplicate_controls_passed": duplicate_checks,
            "rank_before": rank_before,
            "rank_after": adapter.rank(),
        })
    } else {
        json!({"executed": false, "reason": "no extracted four-leaf row"})
    };
    let recovered = adapter.scalar();
    let recovered_scalar_replay = match recovered {
        Some(d) => Some(grader.scalar(PRIME31, d, Some(PRIME31.g))? == q),
        None => None,
    };
    if recovered_scalar_replay == Some(false) {
        failures.push("independent [d]G=Q replay failed".into());
    }
    let passed = failures.is_empty();
    emit(
        "B677_RESULT",
        json!({
            "schema": "prime31-rr5-b677-result/v1",
            "case": case,
            "presentation": presentation,
            "candidate": actual,
            "candidate_seconds": candidate_seconds,
            "expected_control_row_present": expected_present,
            "independent_row_replays": row_replays,
            "authentic_public_rhs": {"a": 0, "b": 1},
            "adapter_receipts": receipts,
            "adapter_rank": adapter.rank(),
            "adapter_setup_counts": adapter.setup_arithmetic_counts(),
            "authentic_adapter_counts": authentic_adapter_arithmetic,
            "authentic_adapter_work": authentic_adapter_work,
            "authentic_adapter_and_replay_seconds": authentic_adapter_seconds,
            "adapter_mutation_controls": mutations,
            "adapter_counts_including_mutation_controls": adapter.arithmetic_counts(),
            "adapter_work_including_mutation_controls": adapter.work(),
            "recovered_scalar": recovered,
            "independent_recovered_scalar_replay": recovered_scalar_replay,
            "independent_grader_work_including_query": grader.work,
            "cold_wall_seconds_including_grader_and_mutations": cold_start.elapsed().as_secs_f64(),
            "peak_rss_kib": peak_rss_kib(),
            "complete_gate_passed": passed,
            "gate_failures": failures,
            "scope": "one frozen query; no target-recovery campaign or matched performance claim",
        }),
    )?;
    Ok(passed)
}

fn main() {
    match run() {
        Ok(true) => {}
        Ok(false) => std::process::exit(1),
        Err(error) => {
            let _ = emit(
                "B677_ERROR",
                json!({"error": error, "complete_gate_passed": false}),
            );
            std::process::exit(2);
        }
    }
}
