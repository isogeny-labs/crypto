//! Independent distinct-root acceptance, never a producer root-finder import.
//! Shoup 2nd ed., Thm 17.2, 19.6 and section 20.4.1 justify gcd(f,X^q-X).
//! Supplied event tallies are consistency checks, not execution/cost proofs.
use crate::arithmetic::{decimal, public_curve, Curve, Field, Point};
use crate::common::digest;
use anyhow::{bail, ensure, Context, Result};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const PRIME_ROLE: &str = "prime-root-certificates/v1";
pub const BINARY_ROLE: &str = "binary-root-certificates/v1";
const EVENT_CAP: u64 = 100_000;
const BYTE_CAP: u64 = 16 * 1024 * 1024;
const MAX_WORK: u64 = 1_000_000;
const ALL_FIELD_POLICY: &str =
    "explicit all-field certificate, then bounded actual factor-base x enumeration";
const PRIME_SCOPE: &str =
    "distinct roots per completed prime S3 call; incomplete calls prove no root absence";
const BINARY_SCOPE: &str =
    "distinct roots per completed binary S3 call; incomplete calls prove no root absence";
const BINARY_WORK_SCOPE: &str = "field add/mul/square calls plus matrix pivot bit tests and word XORs; inverse calls informational; excludes input validation, serialization, replay, loop/control overhead and adapter setup/lifts";

fn fields(v: &Value, expected: &[&str]) -> Result<()> {
    let object = v.as_object().context("root record must be an object")?;
    ensure!(
        object.len() == expected.len() && expected.iter().all(|k| object.contains_key(*k)),
        "unknown or missing root record fields"
    );
    Ok(())
}
fn uint(v: &Value) -> Result<u64> {
    v.as_u64().context("root value must be an unsigned integer")
}
fn flag(v: &Value) -> Result<bool> {
    v.as_bool().context("root flag must be boolean")
}
fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().context("root value must be an array")
}
fn word(v: &Value, field: &Field) -> Result<u64> {
    let x = uint(v)?;
    ensure!(x < field.q(), "root field element is noncanonical");
    Ok(x)
}
fn polynomial(v: &Value, field: &Field, max_len: usize) -> Result<Vec<u64>> {
    let a = array(v)?;
    ensure!(a.len() <= max_len, "root polynomial exceeds degree bound");
    let p = a
        .iter()
        .map(|x| word(x, field))
        .collect::<Result<Vec<_>>>()?;
    ensure!(
        p.last() != Some(&0),
        "root polynomial has trailing zero coefficients"
    );
    Ok(p)
}
fn trim(p: &mut Vec<u64>) {
    while p.last() == Some(&0) {
        p.pop();
    }
}
fn subtract(a: &[u64], b: &[u64], field: &Field) -> Vec<u64> {
    let mut result = vec![0; a.len().max(b.len())];
    for (i, x) in result.iter_mut().enumerate() {
        *x = field.sub(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0));
    }
    trim(&mut result);
    result
}
fn product(a: &[u64], b: &[u64], field: &Field) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut result = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            result[i + j] = field.add(result[i + j], field.mul(x, y));
        }
    }
    trim(&mut result);
    result
}
fn remainder(mut a: Vec<u64>, b: &[u64], field: &Field) -> Result<Vec<u64>> {
    ensure!(!b.is_empty(), "zero root polynomial divisor");
    let inverse = field.inverse(*b.last().unwrap())?;
    while a.len() >= b.len() && !a.is_empty() {
        let shift = a.len() - b.len();
        let scale = field.mul(*a.last().unwrap(), inverse);
        for (j, &value) in b.iter().enumerate() {
            a[shift + j] = field.sub(a[shift + j], field.mul(scale, value));
        }
        trim(&mut a);
    }
    Ok(a)
}
fn power_x(modulus: &[u64], field: &Field) -> Result<Vec<u64>> {
    let mut exponent = field.q();
    let mut base = remainder(vec![0, 1], modulus, field)?;
    let mut acc = remainder(vec![1], modulus, field)?;
    while exponent > 0 {
        if exponent & 1 != 0 {
            acc = remainder(product(&acc, &base, field), modulus, field)?;
        }
        exponent >>= 1;
        if exponent > 0 {
            base = remainder(product(&base, &base, field), modulus, field)?;
        }
    }
    Ok(acc)
}
fn gcd(mut a: Vec<u64>, mut b: Vec<u64>, field: &Field) -> Result<Vec<u64>> {
    while !b.is_empty() {
        let r = remainder(a, &b, field)?;
        a = b;
        b = r;
    }
    if let Some(&leading) = a.last() {
        let inverse = field.inverse(leading)?;
        for x in &mut a {
            *x = field.mul(*x, inverse);
        }
    }
    Ok(a)
}
fn evaluate(p: &[u64], x: u64, field: &Field) -> u64 {
    p.iter()
        .rev()
        .fold(0, |a, &b| field.add(field.mul(a, x), b))
}
fn root_identity(p: &[u64], roots: &[u64], field: &Field) -> Result<(Vec<u64>, Vec<u64>)> {
    ensure!(
        !p.is_empty(),
        "zero polynomial requires all-field semantics"
    );
    ensure!(
        roots.len() < p.len()
            && roots.windows(2).all(|w| w[0] < w[1])
            && roots.iter().all(|r| *r < field.q()),
        "noncanonical distinct root list"
    );
    let xp = power_x(p, field)?;
    let linear = gcd(p.to_vec(), subtract(&xp, &[0, 1], field), field)?;
    let mut factors = vec![1];
    for &root in roots {
        factors = product(&factors, &[field.neg(root), 1], field);
    }
    ensure!(
        linear == factors,
        "distinct root gcd/product identity failed"
    );
    Ok((xp, linear))
}

fn field_spec(field: &Field) -> Value {
    match field {
        Field::Binary { degree, polynomial } => json!({"degree":degree,"modulus":polynomial}),
        Field::Prime { p } => json!({"p":p}),
    }
}

struct ReportCheck {
    complete: bool,
    all_field: bool,
    roots: Vec<u64>,
    work: u64,
    events: u64,
}

fn report_check(report: &Value, field: &Field, algebra: bool) -> Result<ReportCheck> {
    let binary = field.binary_model();
    let mut keys = vec![
        "schema",
        "polynomial",
        "limits",
        "stop",
        "counters",
        "events",
        "unacknowledged",
        "sink_error",
        "roots",
    ];
    if binary {
        keys.extend([
            "field",
            "execution_trace_complete_verified",
            "total_operation_count_verified",
            "work_scope",
        ]);
    } else {
        keys.extend(["p", "seed"]);
    }
    fields(report, &keys)?;
    let max_degree = if binary { 2 } else { 8 };
    let p = polynomial(&report["polynomial"], field, max_degree + 1)?;
    let limits = &report["limits"];
    let limit_work;
    if binary {
        ensure!(
            report["schema"] == "binary-root-trace/v1" && report["field"] == field_spec(field),
            "binary root report field/schema mismatch"
        );
        ensure!(
            report["execution_trace_complete_verified"] == false
                && report["total_operation_count_verified"] == false
                && report["work_scope"] == BINARY_WORK_SCOPE,
            "binary root report overclaims execution or cost"
        );
        fields(limits, &["work", "events"])?;
        limit_work = uint(&limits["work"])?;
    } else {
        ensure!(
            report["schema"] == "prime-root-trace/v1" && uint(&report["p"])? == field.q(),
            "prime root report field/schema mismatch"
        );
        uint(&report["seed"])?;
        fields(
            limits,
            &["field_work", "split_attempts", "events", "degree"],
        )?;
        limit_work = uint(&limits["field_work"])?;
        ensure!(
            uint(&limits["degree"])? <= 8
                && p.len() <= uint(&limits["degree"])? as usize + 1
                && uint(&limits["split_attempts"])? <= EVENT_CAP,
            "invalid prime root limits"
        );
    }
    let limit_events = uint(&limits["events"])?;
    ensure!(
        limit_work <= MAX_WORK && limit_events <= EVENT_CAP,
        "root limits exceed hard bounds"
    );
    let stop = report["stop"].as_str().context("missing root stop")?;
    let complete = matches!(stop, "complete" | "all_field_elements");
    ensure!(
        complete
            || matches!(stop, "work_limit" | "event_limit" | "sink_failure")
            || (!binary && stop == "split_limit")
            || (binary && stop == "linear_system_failure"),
        "unknown root stop"
    );
    let unack = &report["unacknowledged"];
    if stop == "sink_failure" {
        ensure!(
            unack.is_object()
                && report["sink_error"]
                    .as_str()
                    .is_some_and(|s| s.len() <= 4096),
            "missing failed-sink root state"
        );
    } else {
        ensure!(
            unack.is_null() && report["sink_error"].is_null(),
            "unexpected failed-sink root state"
        );
    }
    let events = array(&report["events"])?;
    ensure!(
        events.len() as u64 + u64::from(!unack.is_null()) <= limit_events,
        "root event bound exceeded"
    );
    let counter_keys = if binary {
        vec![
            "additions",
            "multiplications",
            "squares",
            "inverse_calls",
            "matrix_bit_tests",
            "matrix_word_xors",
        ]
    } else {
        vec![
            "additions",
            "subtractions",
            "multiplications",
            "squares",
            "inverse_calls",
            "completed_poly_products",
            "completed_divisions",
            "gcd_steps",
            "split_attempts",
            "rejected_splits",
        ]
    };
    fields(&report["counters"], &counter_keys)?;
    let mut tally: BTreeMap<&str, u64> = counter_keys.iter().map(|k| (*k, 0)).collect();
    let mut begin_seen = false;
    let mut finish_seen = false;
    for (index, event) in events
        .iter()
        .chain((!unack.is_null()).then_some(unack))
        .enumerate()
    {
        ensure!(!finish_seen, "events after root finish");
        let kind = event["kind"].as_str().context("invalid root event tag")?;
        let count = match kind {
            "begin" => {
                ensure!(index == 0 && !begin_seen, "root begin is not unique/first");
                begin_seen = true;
                if binary {
                    fields(event, &["kind", "field", "polynomial"])?;
                    ensure!(
                        event["field"] == report["field"],
                        "root begin field mismatch"
                    );
                } else {
                    fields(event, &["kind", "p", "polynomial", "seed"])?;
                    ensure!(
                        event["p"] == report["p"] && event["seed"] == report["seed"],
                        "root begin field/seed mismatch"
                    );
                }
                ensure!(
                    event["polynomial"] == report["polynomial"],
                    "root begin polynomial mismatch"
                );
                None
            }
            "finish" => {
                ensure!(begin_seen, "root finish before begin");
                finish_seen = true;
                if binary {
                    fields(event, &["kind", "roots", "all_field_elements"])?;
                } else {
                    fields(
                        event,
                        &[
                            "kind",
                            "roots",
                            "xp_mod_f",
                            "linear_part",
                            "all_field_elements",
                        ],
                    )?;
                }
                if index < events.len() {
                    ensure!(
                        complete && event["roots"] == report["roots"],
                        "root finish/report mismatch"
                    );
                } else {
                    ensure!(stop == "sink_failure", "unacknowledged completion claim");
                }
                let roots = array(&event["roots"])?
                    .iter()
                    .map(|r| word(r, field))
                    .collect::<Result<Vec<_>>>()?;
                ensure!(
                    roots.len() <= max_degree && roots.windows(2).all(|w| w[0] < w[1]),
                    "root list is not strictly sorted/distinct"
                );
                let all = flag(&event["all_field_elements"])?;
                ensure!(
                    all == p.is_empty(),
                    "zero/all-field root semantics mismatch"
                );
                if index < events.len() {
                    ensure!(
                        all == (stop == "all_field_elements"),
                        "root terminal kind mismatch"
                    );
                }
                if all {
                    ensure!(
                        roots.is_empty(),
                        "all-field certificate must not enumerate a partial root list"
                    );
                    if !binary {
                        ensure!(
                            array(&event["xp_mod_f"])?.is_empty()
                                && array(&event["linear_part"])?.is_empty(),
                            "zero polynomial has residue certificate"
                        );
                    }
                } else if algebra {
                    let (xp, linear) = root_identity(&p, &roots, field)?;
                    if !binary {
                        ensure!(
                            polynomial(&event["xp_mod_f"], field, 9)? == xp
                                && polynomial(&event["linear_part"], field, 9)? == linear,
                            "prime residue/gcd certificate mismatch"
                        );
                    }
                } else if !binary {
                    polynomial(&event["xp_mod_f"], field, 9)?;
                    polynomial(&event["linear_part"], field, 9)?;
                }
                None
            }
            "field" => {
                fields(event, &["kind", "op", "a", "b", "value"])?;
                let a = word(&event["a"], field)?;
                let b = word(&event["b"], field)?;
                let value = word(&event["value"], field)?;
                let (key, expected) = match event["op"].as_str() {
                    Some("add") => ("additions", field.add(a, b)),
                    Some("sub") if !binary => ("subtractions", field.sub(a, b)),
                    Some("mul") => ("multiplications", field.mul(a, b)),
                    Some("square") => {
                        ensure!(a == b, "root square operands mismatch");
                        ("squares", field.mul(a, a))
                    }
                    _ => bail!("unknown root field operation"),
                };
                ensure!(
                    !algebra || value == expected,
                    "root field event arithmetic mismatch"
                );
                Some(key)
            }
            "inverse_begin" => Some("inverse_calls"),
            "product" if !binary => Some("completed_poly_products"),
            "division" if !binary => Some("completed_divisions"),
            "gcd_step" if !binary => Some("gcd_steps"),
            "split_begin" if !binary => Some("split_attempts"),
            "split_end" if !binary => {
                if flag(&event["accepted"])? {
                    None
                } else {
                    Some("rejected_splits")
                }
            }
            "matrix_bit_test" if binary => Some("matrix_bit_tests"),
            "matrix_word_xor" if binary => Some("matrix_word_xors"),
            "inverse_end" => None,
            "power_begin" | "power_step" | "power_end" | "gcd_begin" | "gcd_end"
            | "linear_root"
                if !binary =>
            {
                None
            }
            "branch" | "inverse_power" | "normalized" | "trace_begin" | "trace_step"
            | "trace_end" | "half_trace_begin" | "half_trace_step" | "half_trace_end"
            | "matrix_begin" | "matrix_column" | "matrix_reduction" | "matrix_pivot"
            | "matrix_end" | "square_root_step" | "scale" | "extracted_root"
                if binary =>
            {
                None
            }
            _ => bail!("unknown root event"),
        };
        ensure!(begin_seen, "root event before begin");
        if let Some(key) = count {
            *tally.get_mut(key).context("inapplicable root counter")? += 1;
        }
    }
    let mut work = 0;
    for (key, &count) in &tally {
        ensure!(
            uint(&report["counters"][*key])? == count,
            "root supplied-event counter mismatch"
        );
        if matches!(
            *key,
            "additions"
                | "subtractions"
                | "multiplications"
                | "squares"
                | "matrix_bit_tests"
                | "matrix_word_xors"
        ) {
            work += count;
        }
    }
    ensure!(work <= limit_work, "root work exceeds declared cap");
    ensure!(
        complete == events.last().is_some_and(|e| e["kind"] == "finish"),
        "missing or censored root finish"
    );
    let roots = array(&report["roots"])?
        .iter()
        .map(|r| word(r, field))
        .collect::<Result<Vec<_>>>()?;
    ensure!(
        roots.len() <= max_degree && (complete || roots.is_empty()),
        "censored report claims final roots"
    );
    if stop == "work_limit" {
        ensure!(work == limit_work, "root work cap not reached");
    }
    if stop == "event_limit" {
        ensure!(
            events.len() as u64 == limit_events,
            "root event cap not reached"
        );
    }
    if !binary {
        ensure!(
            tally["split_attempts"] <= uint(&limits["split_attempts"])?,
            "root split cap exceeded"
        );
        if stop == "split_limit" {
            ensure!(
                tally["split_attempts"] == uint(&limits["split_attempts"])?,
                "root split cap not reached"
            );
        }
    }
    Ok(ReportCheck {
        complete,
        all_field: stop == "all_field_elements",
        roots,
        work,
        events: events.len() as u64,
    })
}

/// Independently verify a standalone report. Completed means distinct field
/// roots only. No adapter binding, execution, operation-cost or UNSAT claim.
pub fn verify_report(report: &Value) -> Result<Value> {
    let field = if report["schema"] == "prime-root-trace/v1" {
        let p = uint(&report["p"])?;
        ensure!(p < 1u64 << 61, "prime root field exceeds word bound");
        if p == 3 {
            Field::Prime { p }
        } else {
            Field::prime(p)?
        }
    } else {
        ensure!(
            report["schema"] == "binary-root-trace/v1",
            "unknown root report schema"
        );
        fields(&report["field"], &["degree", "modulus"])?;
        let degree = uint(&report["field"]["degree"])?;
        ensure!(
            (3..=61).contains(&degree),
            "binary root field degree unsupported"
        );
        let modulus = uint(&report["field"]["modulus"])?;
        ensure!(modulus >> degree == 1, "noncanonical binary root modulus");
        let terms = (0..degree)
            .filter(|i| modulus & (1u64 << i) != 0)
            .map(|i| i as u32)
            .collect::<Vec<_>>();
        Field::binary(degree as u32, &terms)?
    };
    let checked = report_check(report, &field, true)?;
    Ok(
        json!({"completed_distinct_root_set_verified":checked.complete,"all_field_elements":checked.all_field,
        "censored":!checked.complete,"execution_trace_complete_verified":false,"all_event_identities_verified":false,
        "total_operation_count_verified":false,"unsat_verified":false,"dataset_binding_verified":false}),
    )
}

// Producer Value uses recursively sorted maps; this consumer enables
// preserve_order for canonical plan serialization, so sort explicitly here.
fn sorted_value(value: &Value) -> Value {
    match value {
        Value::Object(object) => {
            let ordered: BTreeMap<_, _> = object
                .iter()
                .map(|(k, v)| (k.clone(), sorted_value(v)))
                .collect();
            Value::Object(ordered.into_iter().collect::<Map<_, _>>())
        }
        Value::Array(rows) => Value::Array(rows.iter().map(sorted_value).collect()),
        _ => value.clone(),
    }
}
fn value_hash(value: &Value) -> Result<String> {
    Ok(digest(&serde_json::to_vec(&sorted_value(value))?))
}

fn ordered_domain(curve: &Curve, base: &Value) -> Result<Vec<Point>> {
    let rows = array(&base["points"])?;
    ensure!(rows.len() <= 4096, "root domain exceeds bound");
    let mut original = BTreeSet::new();
    let mut result = Vec::new();
    for row in rows {
        let p = curve.point(row, "root factor-base point", true)?;
        ensure!(original.insert(p), "duplicate root factor-base point");
        for signed in [p, curve.neg(p)?] {
            if !result.contains(&signed) {
                result.push(signed);
            }
        }
    }
    Ok(result)
}
fn coefficients(curve: &Curve, x: u64, target: u64) -> Vec<u64> {
    let f = &curve.field;
    let xy = f.mul(x, target);
    let sum = f.add(x, target);
    if f.binary_model() {
        vec![f.add(f.mul(xy, xy), curve.b), xy, f.mul(sum, sum)]
    } else {
        let difference = f.sub(x, target);
        let t = f.sub(xy, curve.a);
        vec![
            f.sub(f.mul(t, t), f.mul(4, f.mul(curve.b, sum))),
            f.neg(f.add(f.mul(2, f.mul(sum, f.add(xy, curve.a))), f.mul(4, curve.b))),
            f.mul(difference, difference),
        ]
    }
}
// Independent two-level finite-domain replay, preserving input/sign order.
// Counts below check a declared bounded prefix, never machine instructions.
fn lift(
    curve: &Curve,
    domain: &[Point],
    x: u64,
    y: u64,
    target: Point,
    limit: u64,
) -> Result<(bool, u64, bool)> {
    let left: Vec<_> = domain
        .iter()
        .copied()
        .filter(|p| p.is_some_and(|p| p.0 == x))
        .collect();
    let right: Vec<_> = domain
        .iter()
        .copied()
        .filter(|p| p.is_some_and(|p| p.0 == y))
        .collect();
    let mut used = 0;
    for &a in &left {
        if used == limit {
            break;
        }
        used += 1; // O + first summand.
        for &b in &right {
            if used == limit {
                break;
            }
            used += 1;
            if curve.add(a, b)? == target {
                return Ok((true, used, false));
            }
        }
    }
    Ok((
        false,
        used,
        used == limit && !left.is_empty() && !right.is_empty(),
    ))
}

fn outcome_binding(
    outcome: &Value,
    cert: &Value,
    binary: bool,
    events: u64,
    bytes: u64,
) -> Result<()> {
    let found = flag(&cert["found_pdp"])?;
    let censored = flag(&cert["censored"])?;
    let all = flag(&cert["root_queries_completed"])?;
    ensure!(
        !(found && censored) && !(censored && all) && (found || censored || all),
        "inconsistent root aggregate termination"
    );
    let status = if found {
        "verified"
    } else if censored {
        "budget_exhausted"
    } else {
        "partial"
    };
    ensure!(
        outcome["status"] == status
            && flag(&outcome["pdp_witness_verified"])? == found
            && flag(&outcome["solver_completed"])? == (found || !censored),
        "root outcome completion differs from certificate"
    );
    for key in [
        "root_execution_trace_complete_verified",
        "root_total_operation_count_verified",
        "trace_complete",
        "scalar_recovered_without_witness",
        "scalar_replay_verified",
        "ecdlp_pipeline_completed",
    ] {
        ensure!(outcome[key] == false, "root outcome overclaims {key}");
    }
    ensure!(
        outcome["root_certificate_scope"] == if binary { BINARY_SCOPE } else { PRIME_SCOPE },
        "root outcome scope mismatch"
    );
    ensure!(
        outcome["exhaustive_no_decomposition"] != true,
        "root certificate cannot establish UNSAT"
    );
    let key = if binary {
        "aggregate_root_primitive_and_group_work"
    } else {
        "aggregate_root_field_and_group_work"
    };
    ensure!(
        outcome["counters"][key] == cert["charged_work"]
            && uint(&outcome["counters"]["root_solver_calls"])?
                == array(&cert["queries"])?.len() as u64
            && uint(&outcome["counters"]["acknowledged_native_events"])? == events
            && uint(&outcome["counters"]["acknowledged_native_event_bytes"])? == bytes,
        "root outcome aggregate counters mismatch"
    );
    Ok(())
}

/// Bind a recognized saved dataset certificate. No producer implementation is
/// linked. Algebra is opt-in; structural checks and outcome binding always run.
pub fn verify_dataset(
    public: &Value,
    base: &Value,
    config: &Value,
    outcome: &Value,
    cert: &Value,
    role: &str,
    algebra: bool,
) -> Result<Value> {
    let binary = role == BINARY_ROLE;
    ensure!(binary || role == PRIME_ROLE, "unknown dataset root role");
    ensure!(
        config["solver"] == "root"
            && config["m"] == 2
            && (if binary {
                matches!(config["family"].as_str(), Some("binary" | "koblitz"))
            } else {
                config["family"] == "prime"
            }),
        "root certificate on ineligible cell"
    );
    crate::policy::bind_base(public, base, config)?;
    ensure!(
        base["construction_status"] != "unsupported",
        "root certificate uses unsupported factor base"
    );
    let context = public_curve(public, config)?;
    let curve = &context.curve;
    let field = &curve.field;
    ensure!(field.binary_model() == binary, "root family/field mismatch");
    let mut keys = vec![
        "schema",
        "target",
        "requested_work_limit",
        "effective_work_limit",
        "charged_work",
        "queries",
        "root_queries_completed",
        "censored",
        "found_pdp",
        "distinct_root_certificates_verified",
        "execution_trace_complete_verified",
        "total_operation_count_verified",
        "counter_scope",
        "all_field_polynomial_policy",
    ];
    if binary {
        keys.extend([
            "binding",
            "field",
            "event_cap",
            "event_byte_cap",
            "acknowledged_native_events",
            "acknowledged_native_event_bytes",
            "event_byte_scope",
        ]);
    } else {
        keys.push("p");
    }
    fields(cert, &keys)?;
    let schema = if binary {
        "ml-dataset.binary-root-certificates/v1"
    } else {
        "ml-dataset.prime-root-certificates/v1"
    };
    ensure!(
        cert["schema"] == schema
            && cert["target"] == public["target"]
            && cert["requested_work_limit"] == config["work_limit"],
        "root certificate public/config binding mismatch"
    );
    ensure!(
        cert["distinct_root_certificates_verified"] == true
            && cert["execution_trace_complete_verified"] == false
            && cert["total_operation_count_verified"] == false
            && cert["all_field_polynomial_policy"] == ALL_FIELD_POLICY,
        "root certificate scope mismatch"
    );
    let expected_scope = if binary {
        "binary S3 recipe field calls + native field add/mul/square and matrix bit tests/word XORs + original-root Horner calls + group lift additions; mixed admission units, excludes validation, replay, serialization, loop/control overhead and other group internals"
    } else {
        "coefficient recipe field operations + native field add/sub/mul/square including inverse exponentiation + original-root evaluation + group lift additions; excludes validation, serialization and replay"
    };
    ensure!(
        cert["counter_scope"] == expected_scope,
        "root certificate counter scope mismatch"
    );
    if binary {
        fields(
            &cert["binding"],
            &["public_sha256", "factor_base_sha256", "config_sha256"],
        )?;
        for (key, value) in [
            ("public_sha256", public),
            ("factor_base_sha256", base),
            ("config_sha256", config),
        ] {
            ensure!(
                cert["binding"][key] == value_hash(value)?,
                "binary root full-object binding mismatch"
            );
        }
        ensure!(cert["field"]==field_spec(field) && uint(&cert["event_cap"])?==EVENT_CAP
            && uint(&cert["event_byte_cap"])?==BYTE_CAP && cert["event_byte_scope"]=="compact JSON native event envelopes, excluding live-sink wrapper, newline and artifact duplication", "binary root field/event scope mismatch");
    } else {
        ensure!(
            decimal(&cert["p"], "root modulus")? == field.q(),
            "prime root modulus mismatch"
        );
    }
    let limit = uint(&config["work_limit"])?.min(EVENT_CAP);
    ensure!(
        uint(&cert["effective_work_limit"])? == limit,
        "root budget mismatch"
    );
    let domain = ordered_domain(curve, base)?;
    let xs: Vec<_> = domain
        .iter()
        .filter_map(|p| p.map(|p| p.0))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let target_x = context.target.context("root target must be affine")?.0;
    let queries = array(&cert["queries"])?;
    ensure!(queries.len() <= xs.len(), "root query count exceeds domain");
    let setup = if binary { 5 } else { 15 };
    let (mut used, mut events, mut bytes) = (0u64, 0u64, 0u64);
    let (mut found, mut censored) = (false, false);
    let mut results = Vec::new();
    for (index, row) in queries.iter().enumerate() {
        ensure!(!found && !censored, "root query after terminal state");
        fields(
            row,
            &[
                "call_index",
                "fixed_x",
                "coefficients_constant_first",
                "work_before",
                "coefficient_setup_work",
                "work_after",
                "trace_byte_cap_reached",
                "report",
                "checks",
            ],
        )?;
        let x = xs[index];
        let coeff = coefficients(curve, x, target_x);
        ensure!(
            uint(&row["call_index"])? == index as u64
                && decimal(&row["fixed_x"], "fixed root coordinate")? == x
                && row["coefficients_constant_first"] == json!(coeff)
                && uint(&row["work_before"])? == used
                && uint(&row["coefficient_setup_work"])? == setup
                && limit.saturating_sub(used) >= setup,
            "root query input/work prefix mismatch"
        );
        used += setup;
        let report = &row["report"];
        let mut canonical = coeff.clone();
        trim(&mut canonical);
        ensure!(
            report["polynomial"] == json!(canonical),
            "root report polynomial mismatch"
        );
        if binary {
            ensure!(
                report["limits"] == json!({"work":limit-used,"events":EVENT_CAP-events})
                    && report["stop"] != "linear_system_failure",
                "binary root report limits/failure mismatch"
            );
        } else {
            ensure!(
                uint(&report["seed"])? == (uint(&config["seed"])? ^ index as u64)
                    && report["limits"]
                        == json!({"field_work":limit-used,"split_attempts":1000,"events":EVENT_CAP-events,"degree":2}),
                "prime root report frozen limits/seed mismatch"
            );
        }
        let checked = report_check(report, field, algebra)?;
        used += checked.work;
        events += checked.events;
        for event in array(&report["events"])? {
            let envelope = json!({"schema":if binary {"ml-dataset.binary-root-event/v1"} else {"ml-dataset.prime-root-event/v1"},
                "call_index":index,"fixed_x":x.to_string(),"native":event});
            bytes += serde_json::to_vec(&envelope)?.len() as u64;
            ensure!(bytes <= BYTE_CAP, "root native event bytes exceed cap");
        }
        let stopped_at_bytes = report["stop"] == "sink_failure";
        ensure!(
            flag(&row["trace_byte_cap_reached"])? == stopped_at_bytes,
            "root byte cap flag mismatch"
        );
        if stopped_at_bytes {
            let reason = if binary {
                "binary root native envelope byte cap reached"
            } else {
                "prime root event byte cap reached"
            };
            ensure!(
                report["sink_error"] == reason,
                "root sink failure is not declared byte censorship"
            );
            let envelope = json!({"schema":if binary {"ml-dataset.binary-root-event/v1"} else {"ml-dataset.prime-root-event/v1"},
                "call_index":index,"fixed_x":x.to_string(),"native":report["unacknowledged"]});
            ensure!(
                bytes + serde_json::to_vec(&envelope)?.len() as u64 > BYTE_CAP,
                "unjustified root byte cap"
            );
        }
        let roots = if checked.all_field {
            &xs
        } else {
            &checked.roots
        };
        let checks = array(&row["checks"])?;
        ensure!(
            checks.len() <= roots.len() && (checked.complete || checks.is_empty()),
            "censored or excessive root lift checks"
        );
        for (j, check) in checks.iter().enumerate() {
            ensure!(!found && !censored, "root lift after terminal state");
            fields(
                check,
                &[
                    "x",
                    "evaluation_work",
                    "group_lift_add_calls",
                    "in_factor_base",
                    "group_lift_verified",
                    "lift_budget_exhausted",
                ],
            )?;
            let root = roots[j];
            let eval = 2 * coeff.len() as u64;
            ensure!(
                decimal(&check["x"], "root lift coordinate")? == root
                    && uint(&check["evaluation_work"])? == eval
                    && limit.saturating_sub(used) >= eval
                    && check["in_factor_base"] == xs.contains(&root),
                "root lift input/budget/domain mismatch"
            );
            used += eval;
            let claimed = (
                flag(&check["group_lift_verified"])?,
                uint(&check["group_lift_add_calls"])?,
                flag(&check["lift_budget_exhausted"])?,
            );
            ensure!(
                claimed.1 <= limit - used && !(claimed.0 && claimed.2),
                "invalid root lift accounting"
            );
            if algebra {
                ensure!(
                    evaluate(&coeff, root, field) == 0
                        && lift(curve, &domain, x, root, context.target, limit - used)? == claimed,
                    "independent root group lift mismatch"
                );
            }
            used += claimed.1;
            found = claimed.0;
            censored = claimed.2 && !found;
        }
        if checked.complete && !found && checks.len() < roots.len() {
            ensure!(
                censored || limit.saturating_sub(used) < 2 * coeff.len() as u64,
                "unjustified omitted root lift"
            );
            censored = true;
        }
        if !checked.complete {
            censored = true;
        }
        ensure!(
            uint(&row["work_after"])? == used && used <= limit,
            "root aggregate work mismatch"
        );
        results.push(json!({"call_index":index,"completed_distinct_root_set_verified":algebra&&checked.complete,
            "all_field_elements":checked.all_field,"censored":!checked.complete,
            "finite_domain_lift_prefix_verified":algebra,"root_count":if checked.all_field {Value::Null} else {json!(checked.roots.len())}}));
    }
    if !found && !censored && queries.len() < xs.len() {
        ensure!(
            limit.saturating_sub(used) < setup,
            "unjustified omitted root query"
        );
        censored = true;
    }
    ensure!(
        uint(&cert["charged_work"])? == used
            && flag(&cert["found_pdp"])? == found
            && flag(&cert["censored"])? == censored
            && flag(&cert["root_queries_completed"])? == (!censored && queries.len() == xs.len()),
        "root aggregate completion mismatch"
    );
    if binary {
        ensure!(
            uint(&cert["acknowledged_native_events"])? == events
                && uint(&cert["acknowledged_native_event_bytes"])? == bytes,
            "binary root aggregate event mismatch"
        );
    }
    outcome_binding(outcome, cert, binary, events, bytes)?;
    let verified = results
        .iter()
        .filter(|r| r["completed_distinct_root_set_verified"] == true)
        .count();
    Ok(
        json!({"schema":"ml-dataset.independent-root-check/v1","status":if algebra {"checked"} else {"arithmetic_not_requested"},
        "public_config_domain_outcome_bound":true,"full_object_hash_binding_verified":binary,
        "independent_arithmetic_requested":algebra,"completed_root_sets_checked":verified,"calls":results,
        "all_supplied_root_sets_verified":algebra && !queries.is_empty() && verified==queries.len(),
        "cell_censored":censored,"all_domain_queries_completed":cert["root_queries_completed"],
        "reported_accounting_consistent":true,"execution_trace_complete_verified":false,"all_event_identities_verified":false,
        "total_operation_count_verified":false,"unsat_verified":false,"factor_base_construction_verified":false,
        "subgroup_order_verified":false,"fall_degree_verified":false}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    fn report(field: &Field, polynomial: Vec<u64>, roots: Vec<u64>) -> Value {
        let mut polynomial = polynomial;
        trim(&mut polynomial);
        let all = polynomial.is_empty();
        let begin;
        let finish;
        let counters;
        let limits;
        if field.binary_model() {
            begin = json!({"kind":"begin","field":field_spec(field),"polynomial":polynomial});
            finish = json!({"kind":"finish","roots":roots,"all_field_elements":all});
            counters = json!({"additions":0,"multiplications":0,"squares":0,"inverse_calls":0,"matrix_bit_tests":0,"matrix_word_xors":0});
            limits = json!({"work":1000,"events":1000});
        } else {
            begin = json!({"kind":"begin","p":field.q(),"polynomial":polynomial,"seed":0});
            let (xp, g) = if all {
                (vec![], vec![])
            } else {
                let xp = power_x(&polynomial, field).unwrap();
                let g = gcd(polynomial.clone(), subtract(&xp, &[0, 1], field), field).unwrap();
                (xp, g)
            };
            finish = json!({"kind":"finish","roots":roots,"all_field_elements":all,"xp_mod_f":xp,"linear_part":g});
            counters = json!({"additions":0,"subtractions":0,"multiplications":0,"squares":0,"inverse_calls":0,
                "completed_poly_products":0,"completed_divisions":0,"gcd_steps":0,"split_attempts":0,"rejected_splits":0});
            limits = json!({"field_work":1000,"split_attempts":1000,"events":1000,"degree":8});
        }
        let mut value = json!({"schema":if field.binary_model() {"binary-root-trace/v1"} else {"prime-root-trace/v1"},
            "polynomial":polynomial,"limits":limits,"stop":if all {"all_field_elements"} else {"complete"},
            "counters":counters,"events":[begin,finish],"unacknowledged":null,"sink_error":null,"roots":roots});
        if field.binary_model() {
            value["field"] = field_spec(field);
            value["execution_trace_complete_verified"] = json!(false);
            value["total_operation_count_verified"] = json!(false);
            value["work_scope"] = json!(BINARY_WORK_SCOPE);
        } else {
            value["p"] = json!(field.q());
            value["seed"] = json!(0);
        }
        value
    }
    fn oracle(p: &[u64], field: &Field) -> Vec<u64> {
        (0..field.q())
            .filter(|&x| evaluate(p, x, field) == 0)
            .collect()
    }

    #[test]
    fn root_identity_matches_exhaustive_small_fields_in_both_characteristics() {
        let fields = [
            Field::Prime { p: 3 },
            Field::prime(5).unwrap(),
            Field::prime(17).unwrap(),
            Field::binary(3, &[0, 1]).unwrap(),
            Field::binary(4, &[0, 1]).unwrap(),
            Field::binary(5, &[0, 2]).unwrap(),
        ];
        for f in &fields {
            for a in 0..f.q() {
                for b in [0, 1, 2, f.q() - 1] {
                    for c in [0, 1, 2, f.q() - 1] {
                        let mut p = vec![a, b, c];
                        trim(&mut p);
                        if p.is_empty() {
                            continue;
                        }
                        let roots = oracle(&p, f);
                        root_identity(&p, &roots, f).unwrap();
                        let value = report(f, p.clone(), roots.clone());
                        assert_eq!(
                            verify_report(&value).unwrap()["completed_distinct_root_set_verified"],
                            true
                        );
                        if !roots.is_empty() {
                            assert!(root_identity(&p, &roots[1..], f).is_err());
                        }
                        let mut extra = roots;
                        extra.push(f.q());
                        assert!(root_identity(&p, &extra, f).is_err());
                    }
                }
            }
        }
    }

    #[test]
    fn repeated_roots_constants_and_zero_have_distinct_semantics() {
        for f in [
            Field::prime(17).unwrap(),
            Field::binary(4, &[0, 1]).unwrap(),
        ] {
            let p = product(&[f.neg(3), 1], &[f.neg(3), 1], &f);
            let good = report(&f, p, vec![3]);
            verify_report(&good).unwrap();
            let mut repeated = good.clone();
            repeated["roots"] = json!([3, 3]);
            repeated["events"][1]["roots"] = json!([3, 3]);
            assert!(verify_report(&repeated).is_err());
            verify_report(&report(&f, vec![1], vec![])).unwrap();
            let zero = report(&f, vec![], vec![]);
            assert_eq!(verify_report(&zero).unwrap()["all_field_elements"], true);
            let mut bad = zero;
            bad["stop"] = json!("complete");
            bad["events"][1]["all_field_elements"] = json!(false);
            assert!(verify_report(&bad).is_err());
        }
    }

    #[test]
    fn censored_and_unacknowledged_finishes_never_promote_roots() {
        let field = Field::prime(17).unwrap();
        let original = report(&field, vec![16, 0, 1], vec![1, 16]);
        let mut capped = original.clone();
        capped["events"].as_array_mut().unwrap().pop();
        capped["stop"] = json!("work_limit");
        capped["limits"]["field_work"] = json!(0);
        capped["roots"] = json!([]);
        let checked = verify_report(&capped).unwrap();
        assert_eq!(checked["completed_distinct_root_set_verified"], false);
        assert_eq!(checked["censored"], true);
        capped["roots"] = json!([1]);
        assert!(verify_report(&capped).is_err());
        let mut unack = original;
        unack["unacknowledged"] = unack["events"].as_array_mut().unwrap().pop().unwrap();
        unack["roots"] = json!([]);
        unack["stop"] = json!("sink_failure");
        unack["sink_error"] = json!("synthetic failed sink");
        assert_eq!(
            verify_report(&unack).unwrap()["completed_distinct_root_set_verified"],
            false
        );
        unack["stop"] = json!("complete");
        assert!(verify_report(&unack).is_err());
    }

    #[test]
    fn certificate_acceptance_does_not_claim_execution_or_cost() {
        let value = report(&Field::prime(17).unwrap(), vec![16, 0, 1], vec![1, 16]);
        // The minimal begin/finish fixture deliberately omits a solving trace.
        let result = verify_report(&value).unwrap();
        assert_eq!(result["completed_distinct_root_set_verified"], true);
        for key in [
            "execution_trace_complete_verified",
            "all_event_identities_verified",
            "total_operation_count_verified",
            "unsat_verified",
            "dataset_binding_verified",
        ] {
            assert_eq!(result[key], false);
        }
        for (key, replacement) in [
            ("p", json!(15)),
            ("roots", json!([1])),
            ("polynomial", json!([16, 0, 1, 0])),
            ("unknown", json!(true)),
        ] {
            let mut bad = value.clone();
            bad[key] = replacement;
            assert!(verify_report(&bad).is_err(), "accepted {key}");
        }
        let mut bad = value;
        bad["events"][1]["linear_part"] = json!([1]);
        assert!(verify_report(&bad).is_err());
        let mut bad = report(&Field::binary(3, &[0, 1]).unwrap(), vec![1, 1, 1], vec![]);
        bad["field"]["modulus"] = json!(9);
        assert!(verify_report(&bad).is_err());
    }

    #[test]
    fn prime_degree_eight_and_word_bound_do_not_enumerate_field() {
        let field = Field::prime((1u64 << 61) - 1).unwrap();
        let roots = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let p = roots
            .iter()
            .fold(vec![1], |a, &r| product(&a, &[field.neg(r), 1], &field));
        verify_report(&report(&field, p, roots)).unwrap();
    }

    #[test]
    fn binding_hash_sorts_nested_objects_but_preserves_array_order() {
        let a: Value = serde_json::from_str(r#"{"z":[{"b":2,"a":1}],"a":0}"#).unwrap();
        let b: Value = serde_json::from_str(r#"{"a":0,"z":[{"a":1,"b":2}]}"#).unwrap();
        assert_eq!(value_hash(&a).unwrap(), value_hash(&b).unwrap());
        assert_ne!(
            value_hash(&json!([1, 2])).unwrap(),
            value_hash(&json!([2, 1])).unwrap()
        );
    }

    fn dataset_fixture(binary: bool) -> (Value, Value, Value, Value, Value) {
        let (family, n, generator, target, model, field) = if binary {
            (
                "binary",
                3,
                json!({"x":"1","y":"0"}),
                json!({"x":"0","y":"1"}),
                json!({"model":"binary-ordinary","a":"0","b":"1","subgroup_order":"4"}),
                json!({"characteristic":"2","degree":3,"irreducible_low_terms":[0,1],"basis":"polynomial-lsb-first"}),
            )
        } else {
            (
                "prime",
                5,
                json!({"x":"5","y":"1"}),
                json!({"x":"6","y":"3"}),
                json!({"model":"short-weierstrass","a":"2","b":"2","subgroup_order":"19"}),
                json!({"characteristic":"17","modulus":"17","degree":1,"basis":"canonical-residue"}),
            )
        };
        let public = json!({"schema":"ml-ecdlp-instance/v1","family":family,"n":n,"m":2,"twist":false,
            "field":field,"curve":model,"generator":generator,"target":target});
        let config = json!({"family":family,"n":n,"m":2,"twist":false,"solver":"root","seed":0,"work_limit":1000,"factor_base_size":1});
        let base =
            json!({"schema":"ml-ecdlp-factor-base/v1","points":[generator],"requested_size":1});
        let context = public_curve(&public, &config).unwrap();
        let curve = &context.curve;
        let domain = ordered_domain(curve, &base).unwrap();
        let x = decimal(&generator["x"], "fixture x").unwrap();
        let coeff = coefficients(curve, x, context.target.unwrap().0);
        let roots = oracle(&coeff, &curve.field);
        let mut native = report(&curve.field, coeff.clone(), roots.clone());
        let mut used = if binary { 5 } else { 15 };
        if binary {
            native["limits"] = json!({"work":1000-used,"events":EVENT_CAP});
        } else {
            native["limits"] =
                json!({"field_work":1000-used,"split_attempts":1000,"events":EVENT_CAP,"degree":2});
        }
        let mut checks = Vec::new();
        for root in roots {
            used += 6;
            let (found, adds, exhausted) =
                lift(curve, &domain, x, root, context.target, 1000 - used).unwrap();
            used += adds;
            checks.push(json!({"x":root.to_string(),"evaluation_work":6,"group_lift_add_calls":adds,
                "in_factor_base":root==x,"group_lift_verified":found,"lift_budget_exhausted":exhausted}));
            if found {
                break;
            }
        }
        assert_eq!(checks.last().unwrap()["group_lift_verified"], true);
        let bytes:usize=array(&native["events"]).unwrap().iter().map(|e|
            serde_json::to_vec(&json!({"schema":if binary {"ml-dataset.binary-root-event/v1"} else {"ml-dataset.prime-root-event/v1"},
                "call_index":0,"fixed_x":x.to_string(),"native":e})).unwrap().len()).sum();
        let mut cert = json!({"schema":if binary {"ml-dataset.binary-root-certificates/v1"} else {"ml-dataset.prime-root-certificates/v1"},
            "target":target,"requested_work_limit":1000,"effective_work_limit":1000,"charged_work":used,
            "queries":[{"call_index":0,"fixed_x":x.to_string(),"coefficients_constant_first":coeff,"work_before":0,
                "coefficient_setup_work":if binary {5} else {15},"work_after":used,"trace_byte_cap_reached":false,"report":native,"checks":checks}],
            "root_queries_completed":true,"censored":false,"found_pdp":true,"distinct_root_certificates_verified":true,
            "execution_trace_complete_verified":false,"total_operation_count_verified":false,"all_field_polynomial_policy":ALL_FIELD_POLICY});
        if binary {
            cert["binding"] = json!({"public_sha256":value_hash(&public).unwrap(),"factor_base_sha256":value_hash(&base).unwrap(),"config_sha256":value_hash(&config).unwrap()});
            cert["field"] = field_spec(&curve.field);
            cert["event_cap"] = json!(EVENT_CAP);
            cert["event_byte_cap"] = json!(BYTE_CAP);
            cert["acknowledged_native_events"] = json!(2);
            cert["acknowledged_native_event_bytes"] = json!(bytes);
            cert["event_byte_scope"]=json!("compact JSON native event envelopes, excluding live-sink wrapper, newline and artifact duplication");
            cert["counter_scope"]=json!("binary S3 recipe field calls + native field add/mul/square and matrix bit tests/word XORs + original-root Horner calls + group lift additions; mixed admission units, excludes validation, replay, serialization, loop/control overhead and other group internals");
        } else {
            cert["p"] = json!("17");
            cert["counter_scope"]=json!("coefficient recipe field operations + native field add/sub/mul/square including inverse exponentiation + original-root evaluation + group lift additions; excludes validation, serialization and replay");
        }
        let key = if binary {
            "aggregate_root_primitive_and_group_work"
        } else {
            "aggregate_root_field_and_group_work"
        };
        let outcome = json!({"status":"verified","pdp_witness_verified":true,"solver_completed":true,
            "root_execution_trace_complete_verified":false,"root_total_operation_count_verified":false,"trace_complete":false,
            "scalar_recovered_without_witness":false,"scalar_replay_verified":false,"ecdlp_pipeline_completed":false,
            "root_certificate_scope":if binary {BINARY_SCOPE} else {PRIME_SCOPE},
            "counters":{key:used,"root_solver_calls":1,"acknowledged_native_events":2,"acknowledged_native_event_bytes":bytes}});
        (public, base, config, outcome, cert)
    }

    #[test]
    fn dataset_binding_rejects_altered_public_config_domain_outcome_and_roots() {
        for binary in [false, true] {
            let (public, base, config, outcome, cert) = dataset_fixture(binary);
            let role = if binary { BINARY_ROLE } else { PRIME_ROLE };
            let checked =
                verify_dataset(&public, &base, &config, &outcome, &cert, role, true).unwrap();
            assert_eq!(checked["completed_root_sets_checked"], 1);
            assert_eq!(checked["all_supplied_root_sets_verified"], true);
            let unchecked =
                verify_dataset(&public, &base, &config, &outcome, &cert, role, false).unwrap();
            assert_eq!(unchecked["all_supplied_root_sets_verified"], false);
            for (key, value) in [
                ("target", public["generator"].clone()),
                ("requested_work_limit", json!(1001)),
                ("queries", json!([])),
                ("found_pdp", json!(false)),
                ("censored", json!(true)),
                ("execution_trace_complete_verified", json!(true)),
                ("unknown", json!(true)),
            ] {
                let mut bad = cert.clone();
                bad[key] = value;
                assert!(
                    verify_dataset(&public, &base, &config, &outcome, &bad, role, true).is_err(),
                    "accepted certificate {key}"
                );
            }
            let mut bad = cert.clone();
            bad["queries"][0]["report"]["roots"] = json!([]);
            bad["queries"][0]["report"]["events"][1]["roots"] = json!([]);
            assert!(verify_dataset(&public, &base, &config, &outcome, &bad, role, true).is_err());
            let mut bad = public.clone();
            bad["curve"]["b"] = json!("0");
            assert!(verify_dataset(&bad, &base, &config, &outcome, &cert, role, true).is_err());
            let mut bad = config.clone();
            bad["twist"] = json!(true);
            assert!(verify_dataset(&public, &base, &bad, &outcome, &cert, role, true).is_err());
            let mut bad = base.clone();
            bad["points"] = json!([]);
            assert!(verify_dataset(&public, &bad, &config, &outcome, &cert, role, true).is_err());
            for key in [
                "root_execution_trace_complete_verified",
                "root_total_operation_count_verified",
                "ecdlp_pipeline_completed",
                "exhaustive_no_decomposition",
            ] {
                let mut bad = outcome.clone();
                bad[key] = json!(true);
                assert!(
                    verify_dataset(&public, &base, &config, &bad, &cert, role, true).is_err(),
                    "accepted outcome {key}"
                );
            }
            if binary {
                let mut bad = public.clone();
                bad["population"] = json!("transplanted");
                assert!(verify_dataset(&bad, &base, &config, &outcome, &cert, role, true).is_err());
                verify_dataset(
                    &sorted_value(&public),
                    &sorted_value(&base),
                    &sorted_value(&config),
                    &outcome,
                    &cert,
                    role,
                    true,
                )
                .unwrap();
            }
        }
    }

    #[test]
    fn exhausted_before_first_query_retains_censored_status_without_root_claim() {
        for binary in [false, true] {
            let (public, base, mut config, mut outcome, mut cert) = dataset_fixture(binary);
            config["work_limit"] = json!(0);
            for key in [
                "requested_work_limit",
                "effective_work_limit",
                "charged_work",
            ] {
                cert[key] = json!(0);
            }
            cert["queries"] = json!([]);
            cert["root_queries_completed"] = json!(false);
            cert["censored"] = json!(true);
            cert["found_pdp"] = json!(false);
            if binary {
                cert["binding"]["config_sha256"] = json!(value_hash(&config).unwrap());
                cert["acknowledged_native_events"] = json!(0);
                cert["acknowledged_native_event_bytes"] = json!(0);
            }
            outcome["status"] = json!("budget_exhausted");
            outcome["pdp_witness_verified"] = json!(false);
            outcome["solver_completed"] = json!(false);
            for (_, value) in outcome["counters"].as_object_mut().unwrap() {
                *value = json!(0);
            }
            let checked = verify_dataset(
                &public,
                &base,
                &config,
                &outcome,
                &cert,
                if binary { BINARY_ROLE } else { PRIME_ROLE },
                true,
            )
            .unwrap();
            assert_eq!(checked["completed_root_sets_checked"], 0);
            assert_eq!(checked["cell_censored"], true);
            assert_eq!(checked["all_supplied_root_sets_verified"], false);
        }
    }
}
