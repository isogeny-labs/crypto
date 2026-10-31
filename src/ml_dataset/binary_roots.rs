//! Dataset binding for bounded binary quadratic roots. Root completeness is
//! independent of event completeness, PDP lifting, and scalar recovery.

use super::*;
use crate::cryptanalysis::binary_root_trace::{
    self as native, FieldSpec, RootEvent, RootLimits, RootReport, RootStop,
};
use crate::cryptanalysis::binary_semaev::binary_semaev_s3_in_x3;
use serde::{Deserialize, Serialize};

pub(super) const CERTIFICATE_SCOPE: &str =
    "distinct roots per completed binary S3 call; incomplete calls prove no root absence";
const SCHEMA: &str = "ml-dataset.binary-root-certificates/v1";
const ROLE: &str = "binary-root-certificates/v1";
const EVENT_BYTE_CAP: usize = TRACE_BYTE_CAP / 4;
const BYTE_CAP_REASON: &str = "binary root native envelope byte cap reached";
const SETUP_WORK: u64 = 5; // 2 additions, 2 squares, 1 multiplication.
const COUNTER_SCOPE: &str = "binary S3 recipe field calls + native field add/mul/square and matrix bit tests/word XORs + original-root Horner calls + group lift additions; mixed admission units, excludes validation, replay, serialization, loop/control overhead and other group internals";
const BYTE_SCOPE: &str = "compact JSON native event envelopes, excluding live-sink wrapper, newline and artifact duplication";
const ALL_FIELD_POLICY: &str =
    "explicit all-field certificate, then bounded actual factor-base x enumeration";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Binding {
    public_sha256: String,
    factor_base_sha256: String,
    config_sha256: String,
}
fn binding(public: &Value, base: &Value, config: &CaseConfig) -> Result<Binding, String> {
    let digest = |v: &Value| -> Result<String, String> {
        Ok(crate::ml_dataset::digest(
            &serde_json::to_vec(v).map_err(|e| e.to_string())?,
        ))
    };
    Ok(Binding {
        public_sha256: digest(public)?,
        factor_base_sha256: digest(base)?,
        config_sha256: digest(&serde_json::to_value(config).map_err(|e| e.to_string())?)?,
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LiftCheck {
    x: String,
    evaluation_work: u64,
    group_lift_add_calls: u64,
    in_factor_base: bool,
    group_lift_verified: bool,
    lift_budget_exhausted: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Query {
    call_index: usize,
    fixed_x: String,
    coefficients_constant_first: Vec<u64>,
    work_before: u64,
    coefficient_setup_work: u64,
    work_after: u64,
    trace_byte_cap_reached: bool,
    report: RootReport,
    checks: Vec<LiftCheck>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Certificate {
    schema: String,
    binding: Binding,
    field: FieldSpec,
    target: Value,
    requested_work_limit: u64,
    effective_work_limit: u64,
    event_cap: u64,
    event_byte_cap: usize,
    charged_work: u64,
    acknowledged_native_events: u64,
    acknowledged_native_event_bytes: usize,
    queries: Vec<Query>,
    root_queries_completed: bool,
    censored: bool,
    found_pdp: bool,
    distinct_root_certificates_verified: bool,
    execution_trace_complete_verified: bool,
    total_operation_count_verified: bool,
    counter_scope: String,
    event_byte_scope: String,
    all_field_polynomial_policy: String,
}

fn field(curve: &BinaryCurve) -> FieldSpec {
    FieldSpec {
        degree: curve.m,
        modulus: curve
            .irreducible
            .low_terms
            .iter()
            .fold(1u64 << curve.m, |v, i| v | (1u64 << i)),
    }
}
fn codes(domain: &[Element]) -> Result<Vec<u64>, String> {
    let mut xs = domain
        .iter()
        .filter_map(Element::x)
        .map(|x| x.to_u64().ok_or("binary factor-base x exceeds u64"))
        .collect::<Result<Vec<_>, _>>()?;
    xs.sort_unstable();
    xs.dedup();
    Ok(xs)
}
fn coefficients(curve: &BinaryCurve, x: u64, r: u64) -> Vec<u64> {
    let (a, b, c) = binary_semaev_s3_in_x3(
        &F2mElement::from_biguint(&x.into(), curve.m),
        &F2mElement::from_biguint(&r.into(), curve.m),
        &curve.b,
        &curve.irreducible,
    );
    vec![
        c.to_biguint().to_u64().unwrap(),
        b.to_biguint().to_u64().unwrap(),
        a.to_biguint().to_u64().unwrap(),
    ]
}
fn evaluates_to_zero(curve: &BinaryCurve, coeff: &[u64], x: u64) -> bool {
    let x = F2mElement::from_biguint(&x.into(), curve.m);
    coeff
        .iter()
        .rev()
        .fold(F2mElement::zero(curve.m), |acc, &c| {
            acc.mul(&x, &curve.irreducible)
                .add(&F2mElement::from_biguint(&c.into(), curve.m))
        })
        .is_zero()
}
fn event_row(index: usize, x: u64, event: &RootEvent) -> Value {
    json!({"schema":"ml-dataset.binary-root-event/v1","call_index":index,
        "fixed_x":x.to_string(),"native":event})
}
fn event_size(index: usize, x: u64, event: &RootEvent) -> Result<usize, String> {
    Ok(serde_json::to_vec(&event_row(index, x, event))
        .map_err(|e| e.to_string())?
        .len())
}

// Only byte admission failure can be sealed by this adapter. Real live/state
// I/O failures return Err before saving a call or completion certificate.
fn verify_event_budget(
    row: &Query,
    index: usize,
    x: u64,
    byte_cap: usize,
    totals: &mut (u64, usize),
) -> Result<(), String> {
    for event in &row.report.events {
        totals.0 = totals
            .0
            .checked_add(1)
            .ok_or("binary event count overflow")?;
        totals.1 = totals
            .1
            .checked_add(event_size(index, x, event)?)
            .ok_or("binary event bytes overflow")?;
        if totals.0 > TRACE_EVENT_CAP || totals.1 > byte_cap {
            return Err("binary native event count or bytes exceed cap".into());
        }
    }
    let report = &row.report;
    if report.stop == RootStop::SinkFailure {
        let attempted = report
            .unacknowledged
            .as_ref()
            .ok_or("missing byte-capped event")?;
        if !row.trace_byte_cap_reached
            || report.sink_error.as_deref() != Some(BYTE_CAP_REASON)
            || totals.0 >= TRACE_EVENT_CAP
            || totals
                .1
                .checked_add(event_size(index, x, attempted)?)
                .ok_or("binary event bytes overflow")?
                <= byte_cap
        {
            return Err("unjustified binary byte-cap stop".into());
        }
    } else if row.trace_byte_cap_reached
        || report.unacknowledged.is_some()
        || report.sink_error.is_some()
    {
        return Err("binary byte-cap label or failed-sink state differs".into());
    }
    Ok(())
}

/// Check aggregate admission and the supplied canonical event envelopes. This
/// does not establish a complete native trajectory or full execution cost.
pub(super) fn trace_totals(artifact: &Value) -> Result<(u64, u64), String> {
    let cert: Certificate = serde_json::from_value(artifact.clone()).map_err(|e| e.to_string())?;
    if cert.event_cap != TRACE_EVENT_CAP || cert.event_byte_cap != EVENT_BYTE_CAP {
        return Err("binary root event admission caps differ".into());
    }
    let mut totals = (0, 0);
    for (index, row) in cert.queries.iter().enumerate() {
        let x = row.fixed_x.parse::<u64>().map_err(|e| e.to_string())?;
        if row.call_index != index || row.fixed_x != x.to_string() {
            return Err("binary root event query identity differs".into());
        }
        verify_event_budget(row, index, x, EVENT_BYTE_CAP, &mut totals)?;
        if row.trace_byte_cap_reached && index + 1 != cert.queries.len() {
            return Err("binary root queries continue after byte-cap refusal".into());
        }
    }
    if cert.acknowledged_native_events != totals.0
        || cert.acknowledged_native_event_bytes != totals.1
    {
        return Err("binary root acknowledged event aggregates differ".into());
    }
    Ok((totals.0, totals.1 as u64))
}

fn validate_config(public: &Value, base: &Value, config: &CaseConfig) -> Result<(), String> {
    if config.solver != "root"
        || !["binary", "koblitz"].contains(&config.family.as_str())
        || config.m != 2
        || public["family"] != config.family
        || public["m"] != config.m
        || public["n"] != config.n
        || public["twist"] != config.twist
        || (base["schema"] == "ml-ecdlp-factor-base/v2"
            && base["construction_status"] == "unsupported")
    {
        return Err("binary root public/config/domain binding differs".into());
    }
    Ok(())
}

pub(super) fn solve(
    config: &CaseConfig,
    case: &CaseData,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<SolverData, String> {
    validate_config(&case.public_instance, &case.factor_base, config)?;
    let Group::Binary(curve) = group else {
        return Err("binary root group mismatch".into());
    };
    let Some(xr) = target.x().and_then(|x| x.to_u64()) else {
        return Ok(unsupported("S3 root adapter requires affine target"));
    };
    let xs = codes(domain)?;
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    let (mut used, mut events, mut bytes) = (0u64, 0u64, 0usize);
    let mut queries = vec![];
    let mut found = None;
    let mut censored = false;
    let mut data = SolverData {
        outcome: outcome(
            "partial",
            "Bounded native binary roots with independent distinct-root certificates",
        ),
        artifacts: vec![],
    };
    for (index, &x) in xs.iter().enumerate() {
        if limit.saturating_sub(used) < SETUP_WORK {
            censored = true;
            break;
        }
        let before = used;
        let coeff = coefficients(curve, x, xr);
        used += SETUP_WORK;
        let mut byte_cap = false;
        let mut io_error = None;
        let report = native::solve(
            &coeff,
            field(curve),
            RootLimits {
                work: limit - used,
                events: TRACE_EVENT_CAP - events,
            },
            &mut |event| {
                let row = event_row(index, x, event);
                let size = event_size(index, x, event)?;
                if bytes.saturating_add(size) > EVENT_BYTE_CAP {
                    byte_cap = true;
                    return Err(BYTE_CAP_REASON.into());
                }
                if let Err(error) = live_event(&row) {
                    io_error = Some(error.clone());
                    return Err(error);
                }
                events += 1;
                bytes += size;
                if matches!(
                    event,
                    RootEvent::Normalized { .. }
                        | RootEvent::TraceEnd { .. }
                        | RootEvent::HalfTraceEnd { .. }
                        | RootEvent::MatrixEnd { .. }
                ) {
                    if let Err(error) = save(
                        &mut data,
                        &format!("binary-root-state-{index:04}-{events:06}.json"),
                        "native-binary-root-intermediate-state",
                        &row,
                    ) {
                        io_error = Some(error.clone());
                        return Err(error);
                    }
                }
                Ok(())
            },
        )?;
        used += report.counters.work();
        if let Some(error) = io_error {
            return Err(error);
        }
        native::verify(&report)?; // Excluded from solver-work accounting.
        if report.stop == RootStop::LinearSystemFailure {
            return Err("binary root native linear system failed".into());
        }
        let roots = if report.stop == RootStop::AllFieldElements {
            xs.clone()
        } else {
            report.roots.clone()
        };
        let mut checks = vec![];
        if report.complete() {
            for root in roots {
                let eval = 2 * coeff.len() as u64;
                if limit.saturating_sub(used) < eval {
                    censored = true;
                    break;
                }
                used += eval;
                if !evaluates_to_zero(curve, &coeff, root) {
                    return Err("native binary root failed original polynomial evaluation".into());
                }
                let (witness, adds, exhausted) = lift(
                    group,
                    domain,
                    &[BigUint::from(x), BigUint::from(root)],
                    target,
                    limit - used,
                );
                used += adds;
                checks.push(LiftCheck {
                    x: root.to_string(),
                    evaluation_work: eval,
                    group_lift_add_calls: adds,
                    in_factor_base: xs.contains(&root),
                    group_lift_verified: witness.is_some(),
                    lift_budget_exhausted: exhausted,
                });
                if witness.is_some() {
                    found = witness;
                    break;
                }
                if exhausted {
                    censored = true;
                    break;
                }
            }
        } else {
            censored = true;
        }
        let query = Query {
            call_index: index,
            fixed_x: x.to_string(),
            coefficients_constant_first: coeff,
            work_before: before,
            coefficient_setup_work: SETUP_WORK,
            work_after: used,
            trace_byte_cap_reached: byte_cap,
            report,
            checks,
        };
        save(
            &mut data,
            &format!("binary-root-call-{index:04}.json"),
            "native-binary-root-call",
            &serde_json::to_value(&query).map_err(|e| e.to_string())?,
        )?;
        queries.push(query);
        if found.is_some() || censored {
            break;
        }
    }
    let certificate = Certificate {
        schema: SCHEMA.into(),
        binding: binding(&case.public_instance, &case.factor_base, config)?,
        field: field(curve),
        target: target.json(),
        requested_work_limit: config.work_limit,
        effective_work_limit: limit,
        event_cap: TRACE_EVENT_CAP,
        event_byte_cap: EVENT_BYTE_CAP,
        charged_work: used,
        acknowledged_native_events: events,
        acknowledged_native_event_bytes: bytes,
        root_queries_completed: !censored && queries.len() == xs.len(),
        queries,
        censored,
        found_pdp: found.is_some(),
        distinct_root_certificates_verified: true,
        execution_trace_complete_verified: false,
        total_operation_count_verified: false,
        counter_scope: COUNTER_SCOPE.into(),
        event_byte_scope: BYTE_SCOPE.into(),
        all_field_polynomial_policy: ALL_FIELD_POLICY.into(),
    };
    save(
        &mut data,
        "binary-root-certificates.json",
        ROLE,
        &serde_json::to_value(&certificate).map_err(|e| e.to_string())?,
    )?;
    data.outcome["counters"] = json!({"aggregate_root_primitive_and_group_work":used,
        "root_solver_calls":certificate.queries.len(),"acknowledged_native_events":events,
        "acknowledged_native_event_bytes":bytes});
    data.outcome["root_certificate_scope"] = json!(CERTIFICATE_SCOPE);
    data.outcome["root_execution_trace_complete_verified"] = json!(false);
    data.outcome["root_total_operation_count_verified"] = json!(false);
    data.outcome["trace_gaps"] = json!(["Higher-m elimination unsupported",
        "Distinct-root and supplied-event checks do not establish complete execution or total operation cost",
        "Retained native states are artifacts, not a restorable process checkpoint"]);
    if let Some(witness) = found {
        set_witness(&mut data, group, &witness, target)?;
    } else if censored {
        data.outcome["status"] = json!("budget_exhausted");
        data.outcome["reason"] =
            json!("Aggregate binary-root/lift work or native trace cap reached");
    } else {
        data.outcome["solver_completed"] = json!(true);
    }
    Ok(data)
}

pub(super) fn verify(
    public: &Value,
    base: &Value,
    config: &CaseConfig,
    artifact: &Value,
) -> Result<(), String> {
    validate_config(public, base, config)?;
    let cert: Certificate = serde_json::from_value(artifact.clone()).map_err(|e| e.to_string())?;
    let limit = config.work_limit.min(TRACE_EVENT_CAP);
    if cert.schema != SCHEMA
        || cert.binding != binding(public, base, config)?
        || cert.target != public["target"]
        || cert.requested_work_limit != config.work_limit
        || cert.effective_work_limit != limit
        || cert.event_cap != TRACE_EVENT_CAP
        || cert.event_byte_cap != EVENT_BYTE_CAP
        || !cert.distinct_root_certificates_verified
        || cert.execution_trace_complete_verified
        || cert.total_operation_count_verified
        || cert.counter_scope != COUNTER_SCOPE
        || cert.event_byte_scope != BYTE_SCOPE
        || cert.all_field_polynomial_policy != ALL_FIELD_POLICY
    {
        return Err("binary root certificate binding or scope differs".into());
    }
    let case = CaseData {
        public_instance: public.clone(),
        private_witness: Value::Null,
        factor_base: base.clone(),
    };
    let group = Group::read(&case)?;
    let Group::Binary(curve) = &group else {
        return Err("binary curve required".into());
    };
    if cert.field != field(curve) {
        return Err("binary root field differs".into());
    }
    let target = group.point(&public["target"])?;
    if !group.on_curve(&target) {
        return Err("binary root target is off curve".into());
    }
    let xr = target
        .x()
        .and_then(|x| x.to_u64())
        .ok_or("affine target required")?;
    let domain = domain(&group, &case)?;
    let xs = codes(&domain)?;
    if cert.queries.len() > xs.len() {
        return Err("too many binary root queries".into());
    }
    let (mut used, mut events, mut bytes) = (0u64, 0u64, 0usize);
    let (mut found, mut censored) = (false, false);
    for (index, row) in cert.queries.iter().enumerate() {
        if found || censored {
            return Err("binary root queries continue past terminal state".into());
        }
        let x = xs[index];
        let coeff = coefficients(curve, x, xr);
        if row.call_index != index
            || row.fixed_x != x.to_string()
            || row.work_before != used
            || row.coefficient_setup_work != SETUP_WORK
            || row.coefficients_constant_first != coeff
            || limit.saturating_sub(used) < SETUP_WORK
        {
            return Err("binary root query identity or work prefix differs".into());
        }
        used += SETUP_WORK;
        let report = &row.report;
        let mut canonical = coeff.clone();
        while canonical.last() == Some(&0) {
            canonical.pop();
        }
        if report.field != cert.field
            || report.polynomial != canonical
            || report.limits
                != (RootLimits {
                    work: limit - used,
                    events: TRACE_EVENT_CAP - events,
                })
        {
            return Err("binary root report differs from frozen call".into());
        }
        native::verify(report)?;
        if report.stop == RootStop::LinearSystemFailure {
            return Err("failed binary linear system is not a dataset certificate".into());
        }
        used += report.counters.work();
        let mut totals = (events, bytes);
        verify_event_budget(row, index, x, EVENT_BYTE_CAP, &mut totals)?;
        (events, bytes) = totals;
        let roots = if report.stop == RootStop::AllFieldElements {
            xs.clone()
        } else {
            report.roots.clone()
        };
        if row.checks.len() > roots.len() || (!report.complete() && !row.checks.is_empty()) {
            return Err("invalid censored binary root lifts".into());
        }
        for (j, check) in row.checks.iter().enumerate() {
            if found || censored {
                return Err("binary lifts continue after terminal state".into());
            }
            let root = roots[j];
            let eval = 2 * coeff.len() as u64;
            if check.x != root.to_string()
                || check.evaluation_work != eval
                || limit.saturating_sub(used) < eval
                || !evaluates_to_zero(curve, &coeff, root)
            {
                return Err("binary root evaluation identity or work differs".into());
            }
            used += eval;
            let (w, adds, exhausted) = lift(
                &group,
                &domain,
                &[BigUint::from(x), BigUint::from(root)],
                &target,
                limit - used,
            );
            if check.group_lift_add_calls != adds
                || check.in_factor_base != xs.contains(&root)
                || check.group_lift_verified != w.is_some()
                || check.lift_budget_exhausted != exhausted
            {
                return Err("binary root group lift replay differs".into());
            }
            used += adds;
            found = w.is_some();
            censored = exhausted && !found;
        }
        if report.complete() && !found && row.checks.len() < roots.len() {
            if !censored && limit.saturating_sub(used) >= 2 * coeff.len() as u64 {
                return Err("unjustified omitted binary root lift".into());
            }
            censored = true;
        }
        if !report.complete() {
            censored = true;
        }
        if row.work_after != used || used > limit || events > TRACE_EVENT_CAP {
            return Err("aggregate binary root work/events differ".into());
        }
    }
    if !found && !censored && cert.queries.len() < xs.len() {
        if limit.saturating_sub(used) >= SETUP_WORK {
            return Err("unjustified omitted binary root query".into());
        }
        censored = true;
    }
    if cert.charged_work != used
        || cert.acknowledged_native_events != events
        || cert.acknowledged_native_event_bytes != bytes
        || cert.found_pdp != found
        || cert.censored != censored
        || cert.root_queries_completed != (!censored && cert.queries.len() == xs.len())
    {
        return Err("binary root aggregate completion differs".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(family: &str, n: u32, twist: bool) -> (CaseConfig, CaseData) {
        let config = CaseConfig {
            factor_base_policy: None,
            family: family.into(),
            n,
            m: 2,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 8,
            degree_bound: 4,
            work_limit: TRACE_EVENT_CAP,
            solver: "root".into(),
        };
        let mut case = curves::generate(&config).unwrap();
        case.private_witness = Value::Null;
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let point = points.iter().min_by_key(|p| p.x()).unwrap();
        let target = group.add(point, point);
        assert_ne!(target, group.zero());
        case.public_instance["target"] = target.json();
        case.public_instance["population"] = json!("software-public-planted-binary-root-fixture");
        (config, case)
    }
    fn certificate(result: &SolverData) -> Value {
        serde_json::from_slice(
            &result
                .artifacts
                .iter()
                .find(|a| a.role == ROLE)
                .unwrap()
                .bytes,
        )
        .unwrap()
    }

    #[test]
    fn odd_even_binary_families_and_twists_bind_and_lift_without_private_witness() {
        for family in ["koblitz", "binary"] {
            for n in [11, 12] {
                for twist in [false, true] {
                    let (cfg, case) = fixture(family, n, twist);
                    let result = super::super::solve(&cfg, &case).unwrap();
                    assert_eq!(
                        result.outcome["pdp_witness_verified"], true,
                        "{family}/{n}/{twist}: {}",
                        result.outcome
                    );
                    let cert = certificate(&result);
                    verify(&case.public_instance, &case.factor_base, &cfg, &cert).unwrap();
                    crate::ml_dataset::verify_binary_root_outcome(&result.outcome, &cert).unwrap();
                    assert!(result
                        .artifacts
                        .iter()
                        .any(|a| a.role == "native-binary-root-intermediate-state"));
                    let label = result
                        .artifacts
                        .iter()
                        .find(|a| a.role == "independently-verified-pdp")
                        .unwrap();
                    verify_solution_artifact(
                        &case.public_instance,
                        &case.factor_base,
                        &label.role,
                        &serde_json::from_slice(&label.bytes).unwrap(),
                    )
                    .unwrap();
                    let encoded = serde_json::to_string(&cert).unwrap();
                    assert!(!encoded.contains("private_witness"));
                    assert_eq!(cert["field"]["degree"], n);
                }
            }
        }
    }

    #[test]
    fn aggregate_caps_preserve_replayable_prefix_and_exact_completion_boundary() {
        for n in [11, 12] {
            let (mut cfg, case) = fixture("koblitz", n, false);
            let full = super::super::solve(&cfg, &case).unwrap();
            let exact = certificate(&full)["charged_work"].as_u64().unwrap();
            for cap in [
                0,
                SETUP_WORK - 1,
                SETUP_WORK,
                SETUP_WORK + 1,
                SETUP_WORK + 20,
                exact - 1,
                exact,
            ] {
                cfg.work_limit = cap;
                let result = super::super::solve(&cfg, &case).unwrap();
                let cert = certificate(&result);
                verify(&case.public_instance, &case.factor_base, &cfg, &cert).unwrap();
                crate::ml_dataset::verify_binary_root_outcome(&result.outcome, &cert).unwrap();
                assert!(cert["charged_work"].as_u64().unwrap() <= cap);
                assert_eq!(
                    result.outcome["pdp_witness_verified"],
                    cap == exact,
                    "n={n}, cap={cap}"
                );
                assert_eq!(cert["censored"], cap != exact);
            }
        }
    }

    #[test]
    fn canonical_envelope_caps_bind_failed_finish_and_accumulate_across_calls() {
        let limits = RootLimits {
            work: 10_000,
            events: 10_000,
        };
        let field = FieldSpec {
            degree: 3,
            modulus: 0b1011,
        };
        let report = native::solve(&[0, 1, 1], field, limits, &mut |_| Ok(())).unwrap();
        native::verify(&report).unwrap();
        assert!(report.complete());
        assert_eq!(report.roots, vec![0, 1]);
        let mut query = Query {
            call_index: 0,
            fixed_x: "7".into(),
            coefficients_constant_first: vec![0, 1, 1],
            work_before: 0,
            coefficient_setup_work: SETUP_WORK,
            work_after: report.counters.work() + SETUP_WORK,
            trace_byte_cap_reached: false,
            report,
            checks: vec![],
        };
        let exact = query
            .report
            .events
            .iter()
            .map(|e| event_size(0, 7, e).unwrap())
            .sum::<usize>();
        let mut totals = (0, 0);
        verify_event_budget(&query, 0, 7, exact, &mut totals).unwrap();
        assert_eq!(totals, (query.report.events.len() as u64, exact));
        assert!(verify_event_budget(&query, 0, 7, exact - 1, &mut (0, 0)).is_err());
        // Both individual calls fit; the second call crosses the shared cap.
        let mut second = (0, 0);
        verify_event_budget(&query, 1, 7, usize::MAX, &mut second).unwrap();
        let mut cumulative = (0, 0);
        verify_event_budget(&query, 0, 7, exact + second.1, &mut cumulative).unwrap();
        verify_event_budget(&query, 1, 7, exact + second.1, &mut cumulative).unwrap();
        assert_eq!(
            cumulative,
            (2 * query.report.events.len() as u64, exact + second.1)
        );
        let mut cumulative = (0, 0);
        verify_event_budget(&query, 0, 7, exact + second.1 - 1, &mut cumulative).unwrap();
        assert!(verify_event_budget(&query, 1, 7, exact + second.1 - 1, &mut cumulative).is_err());
        query.trace_byte_cap_reached = true;
        assert!(verify_event_budget(&query, 0, 7, exact, &mut (0, 0)).is_err());

        let mut acknowledged = 0;
        query.report = native::solve(&[0, 1, 1], field, limits, &mut |event| {
            let size = event_size(0, 7, event)?;
            if acknowledged + size > exact - 1 {
                return Err(BYTE_CAP_REASON.into());
            }
            acknowledged += size;
            Ok(())
        })
        .unwrap();
        native::verify(&query.report).unwrap();
        assert_eq!(query.report.stop, RootStop::SinkFailure);
        assert!(matches!(
            query.report.unacknowledged.as_ref(),
            Some(RootEvent::Finish { .. })
        ));
        assert!(query.report.roots.is_empty());
        let mut totals = (0, 0);
        verify_event_budget(&query, 0, 7, exact - 1, &mut totals).unwrap();
        assert_eq!(totals, (query.report.events.len() as u64, acknowledged));
        assert!(verify_event_budget(&query, 0, 7, exact, &mut (0, 0)).is_err());
        query.trace_byte_cap_reached = false;
        assert!(verify_event_budget(&query, 0, 7, exact - 1, &mut (0, 0)).is_err());
        query.trace_byte_cap_reached = true;
        query.report.sink_error = Some("selected-state I/O failure".into());
        native::verify(&query.report).unwrap();
        assert!(verify_event_budget(&query, 0, 7, exact - 1, &mut (0, 0)).is_err());
    }

    #[test]
    fn exact_identity_native_events_and_aggregate_tampering_are_rejected() {
        let (cfg, case) = fixture("binary", 12, true);
        let result = super::super::solve(&cfg, &case).unwrap();
        let cert = certificate(&result);
        let rejects = |value: &Value| {
            assert!(verify(&case.public_instance, &case.factor_base, &cfg, value).is_err())
        };
        for key in [
            "aggregate_root_primitive_and_group_work",
            "root_solver_calls",
            "acknowledged_native_events",
            "acknowledged_native_event_bytes",
        ] {
            let mut bad = result.outcome.clone();
            bad["counters"][key] = json!(result.outcome["counters"][key].as_u64().unwrap() + 1);
            assert!(crate::ml_dataset::verify_binary_root_outcome(&bad, &cert).is_err());
        }
        for key in [
            "charged_work",
            "acknowledged_native_events",
            "acknowledged_native_event_bytes",
            "event_cap",
            "event_byte_cap",
        ] {
            let mut bad = cert.clone();
            bad[key] = json!(cert[key].as_u64().unwrap() + 1);
            rejects(&bad);
        }
        for key in [
            "execution_trace_complete_verified",
            "total_operation_count_verified",
        ] {
            let mut bad = cert.clone();
            bad[key] = json!(true);
            rejects(&bad);
        }
        let mut bad = cert.clone();
        bad["unexpected"] = json!(true);
        rejects(&bad);
        let mut bad = cert.clone();
        bad["field"]["modulus"] = json!(0);
        rejects(&bad);
        let mut bad = cert.clone();
        bad["queries"][0]["fixed_x"] = json!("00");
        rejects(&bad);
        let mut bad = cert.clone();
        bad["queries"][0]["trace_byte_cap_reached"] = json!(true);
        rejects(&bad);
        let mut bad = cert.clone();
        bad["queries"][0]["report"]["events"]
            .as_array_mut()
            .unwrap()
            .remove(0);
        rejects(&bad);
        let mut bad = cert.clone();
        bad["queries"] = json!([]);
        rejects(&bad);
        let mut wrong_cfg = cfg.clone();
        wrong_cfg.seed += 1;
        assert!(verify(&case.public_instance, &case.factor_base, &wrong_cfg, &cert).is_err());
        let mut wrong_cfg = cfg.clone();
        wrong_cfg.degree_bound += 1;
        assert!(verify(&case.public_instance, &case.factor_base, &wrong_cfg, &cert).is_err());
        for (key, value) in [
            ("twist", json!(false)),
            ("population", json!("other")),
            ("generator", Value::Null),
        ] {
            let mut public = case.public_instance.clone();
            public[key] = value;
            assert!(verify(&public, &case.factor_base, &cfg, &cert).is_err());
        }
        let mut public = case.public_instance.clone();
        public["curve"]["a"] = json!("0");
        assert!(verify(&public, &case.factor_base, &cfg, &cert).is_err());
        let mut base = case.factor_base.clone();
        base["points"].as_array_mut().unwrap().reverse();
        assert!(verify(&case.public_instance, &base, &cfg, &cert).is_err());
        let mut base = case.factor_base.clone();
        base["construction_status"] = json!("other");
        assert!(verify(&case.public_instance, &base, &cfg, &cert).is_err());
    }

    #[test]
    fn live_states_and_event_prefix_survive_payload_sink_failure() {
        let (cfg, case) = fixture("koblitz", 12, false);
        let expected = super::super::solve(&cfg, &case).unwrap();
        let first = expected
            .artifacts
            .iter()
            .find(|a| a.role == "native-binary-root-intermediate-state")
            .unwrap();
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("binary-root-live-{}-{nonce}", std::process::id()));
        fs::create_dir_all(root.join("payloads")).unwrap();
        fs::write(root.join("payloads").join(&first.path), b"collision").unwrap();
        assert!(solve_with_event_sink(&cfg, &case, &root)
            .unwrap_err()
            .contains("collision"));
        let rows: Vec<Value> = fs::read_to_string(root.join("events.jsonl"))
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        let native: Vec<_> = rows
            .iter()
            .filter(|r| r["event"]["schema"] == "ml-dataset.binary-root-event/v1")
            .collect();
        assert!(!native.is_empty());
        assert_eq!(
            native.last().unwrap()["event"],
            serde_json::from_slice::<Value>(&first.bytes).unwrap()
        );
        assert!(!root.join("payloads/binary-root-certificates.json").exists());
        assert_eq!(
            fs::read(root.join("payloads").join(&first.path)).unwrap(),
            b"collision"
        );
        fs::remove_dir_all(&root).unwrap();
        let result = solve_with_event_sink(&cfg, &case, &root).unwrap();
        for payload in &result.artifacts {
            assert_eq!(
                fs::read(root.join("payloads").join(&payload.path)).unwrap(),
                payload.bytes
            );
        }
        let cert = certificate(&result);
        verify(&case.public_instance, &case.factor_base, &cfg, &cert).unwrap();
        let rows: Vec<Value> = fs::read_to_string(root.join("events.jsonl"))
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        let native: Vec<_> = rows
            .iter()
            .filter(|r| r["event"]["schema"] == "ml-dataset.binary-root-event/v1")
            .collect();
        assert_eq!(
            native.len() as u64,
            cert["acknowledged_native_events"].as_u64().unwrap()
        );
        let bytes: usize = native
            .iter()
            .map(|row| serde_json::to_vec(&row["event"]).unwrap().len())
            .sum();
        assert_eq!(
            bytes as u64,
            cert["acknowledged_native_event_bytes"].as_u64().unwrap()
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn higher_arity_remains_explicitly_unsupported() {
        let (mut cfg, mut case) = fixture("binary", 11, false);
        cfg.m = 3;
        case.public_instance["m"] = json!(3);
        let result = super::super::solve(&cfg, &case).unwrap();
        assert_eq!(result.outcome["status"], "unsupported");
        assert!(!result.artifacts.iter().any(|a| a.role == ROLE));
    }
}
