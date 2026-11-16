//! Four-variable affine encoding with independent signed-group triple replay.
use super::super::execution::{LaunchContext, SolverExecutionContext};
use super::prime_f4_wide::{
    artifact, encoded, number, push, read_bounded, stop_name, verify_censored_outcome,
    verify_unproved_claims, Event, Limits, NativeStatistics,
};
use super::*;
use crate::cryptanalysis::f4_fp_tower::{free_of, mask_of, mono, TowerF4Options};
use crate::cryptanalysis::f4_tower_generic::{f4_tower_generic_bounded, GPoly, GTowerRing};
use crate::cryptanalysis::fp_backend::{Fp64, PrimeField};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
pub(super) const STRATEGY: &str = "prime-f4-fp64-affine-chain-m3-group-replay/v1";
const INPUT: &str = "prime-f4-input.json";
const RESULT: &str = "prime-f4-wide-m3-result.json";
const EVENTS: &str = "prime-f4-wide-m3-events.jsonl";
const EXTRACT: &str = "prime-f4-wide-m3-group-extraction.json";
const INPUT_ROLE: &str = "complete-prime-field-polynomial-system";
const RESULT_ROLE: &str = "restricted-wide-prime-f4-m3-result/v1";
const EVENTS_ROLE: &str = "restricted-wide-prime-f4-m3-boundary-events/v1";
const EXTRACT_ROLE: &str = "restricted-wide-prime-f4-m3-group-replay/v1";
const RESERVE: usize = 64 * 1024;
const POLICY: &str = "ordered-signed-triples-continue-after-first-witness/v1";
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Term {
    exponents: [u8; 4],
    coefficient: String,
}
type Poly = Vec<Term>;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "branch", deny_unknown_fields)]
enum Prefix {
    Affine {
        coordinates: [String; 4],
        residues: Vec<String>,
    },
    ExceptionalInfinityPrefix,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Triple {
    indices: [usize; 3],
    prefix: Prefix,
    total: Option<Value>,
    matches_target: Option<bool>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Extraction {
    schema: String,
    policy: String,
    signed_domain: Vec<Value>,
    rows: Vec<Triple>,
    domain_complete: bool,
    stop: String,
    persistence_error: Option<String>,
    group_adds: u64,
    witness_replay_adds: u64,
    original_equation_evaluations: u64,
    witness_equation_replays: u64,
    first_verified_cursor: Option<usize>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct ResultRecord {
    schema: String,
    strategy: String,
    launch: Option<LaunchContext>,
    limits: Option<Limits>,
    stop: String,
    native_backend_invoked: bool,
    native_statistics_complete: bool,
    work_reserved: u64,
    bytes_reserved: u64,
    acknowledged_events: usize,
    native_statistics: Option<NativeStatistics>,
    basis: Vec<Poly>,
    basis_retained: bool,
    native_basis_derivation_verified: bool,
    execution_trace_complete_verified: bool,
    total_operation_count_verified: bool,
    extraction_complete: bool,
    exhaustive_no_decomposition: bool,
    pdp_witness_verified: bool,
    extraction_retained: bool,
}
fn base_result() -> ResultRecord {
    ResultRecord {
        schema: "crypto.ml-dataset.wide-prime-f4-m3-result/v1".into(),
        strategy: STRATEGY.into(),
        launch: None,
        limits: None,
        stop: "missing_execution_context".into(),
        native_backend_invoked: false,
        native_statistics_complete: false,
        work_reserved: 0,
        bytes_reserved: 0,
        acknowledged_events: 0,
        native_statistics: None,
        basis: vec![],
        basis_retained: false,
        native_basis_derivation_verified: false,
        execution_trace_complete_verified: false,
        total_operation_count_verified: false,
        extraction_complete: false,
        exhaustive_no_decomposition: false,
        pdp_witness_verified: false,
        extraction_retained: false,
    }
}
fn finish(data: &mut SolverData, result: &ResultRecord, cap: usize) -> Result<(), String> {
    push(data, RESULT, RESULT_ROLE, result, cap)?;
    data.outcome["native_backend_invoked"] = json!(result.native_backend_invoked);
    data.outcome["algebra_completed"] = json!(result.native_statistics_complete);
    data.outcome["native_statistics_complete"] = json!(result.native_statistics_complete);
    data.outcome["termination"] = json!(result.stop);
    data.outcome["extraction_complete"] = json!(result.extraction_complete);
    data.outcome["exhaustive_no_decomposition"] = json!(result.exhaustive_no_decomposition);
    data.outcome["pdp_witness_verified"] = json!(result.pdp_witness_verified);
    data.outcome["solver_completed"] =
        json!(result.exhaustive_no_decomposition || result.pdp_witness_verified);
    data.outcome["native_basis_derivation_verified"] = json!(false);
    data.outcome["execution_trace_complete_verified"] = json!(false);
    data.outcome["total_operation_count_verified"] = json!(false);
    data.outcome["counters"] = json!({"native_work_reserved":result.work_reserved,"native_cumulative_bytes_reserved":result.bytes_reserved,"acknowledged_native_boundaries":result.acknowledged_events});
    if result.pdp_witness_verified {
        data.outcome["status"] = json!("verified");
    }
    flush_prime_chain_payloads(data);
    if data.outcome.get("live_persistence_errors").is_some() {
        data.outcome["solver_completed"] = json!(false);
        data.outcome["status"] = json!("partial");
    }
    Ok(())
}

pub(super) fn solve(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
    context: Option<&SolverExecutionContext>,
) -> Result<SolverData, String> {
    let original = prime_chain_input(3, group, domain, target)?;
    let mut data = SolverData {
        outcome: outcome(
            "partial",
            "Bounded wide-prime F4 and independently checked original-domain extraction",
        ),
        artifacts: vec![],
    };
    data.outcome["strategy"] = json!(STRATEGY);
    data.outcome["trace_gaps"]=json!(["Native boundaries and final basis do not expose complete matrices, pivots or a derivation certificate","Reserved work and storage are conservative admission quantities, not executed instruction counts or RSS","No first or last fall degree theorem or independent scalar recovery"]);
    push(&mut data, INPUT, INPUT_ROLE, &original.artifact, RESERVE)?;
    let mut result = base_result();
    let Some(context) = context else {
        finish(&mut data, &result, RESERVE)?;
        return Ok(data);
    };
    result.launch = Some(context.launch.clone());
    let limits = match Limits::derive(config, &context.launch) {
        Ok(l) => l,
        Err(e) => {
            result.stop = "profile_admission".into();
            data.outcome["reason"] = json!(e);
            finish(&mut data, &result, RESERVE)?;
            return Ok(data);
        }
    };
    result.limits = Some(limits.clone());
    flush_prime_chain_payloads(&mut data);
    if data.outcome.get("live_persistence_errors").is_some() {
        result.stop = "observer_failure".into();
        finish(&mut data, &result, RESERVE)?;
        return Ok(data);
    }
    if Instant::now() >= context.deadline {
        result.stop = "deadline".into();
        finish(&mut data, &result, RESERVE)?;
        return Ok(data);
    }
    let p = original.modulus;
    if !((1u64 << 32)..=(1u64 << 61) - 1).contains(&p)
        || !crate::cryptanalysis::residual_walk::is_prime_u64(p)
    {
        result.stop = "invalid_input".into();
        finish(&mut data, &result, RESERVE)?;
        return Ok(data);
    }
    let field = Fp64::new(p);
    let ring = GTowerRing {
        field,
        rules: vec![],
        n_free: 4,
    };
    let native_input: Vec<_> = original
        .polynomials
        .iter()
        .map(|poly| {
            Ok(GPoly::from_terms(
                &ring.field,
                poly.iter()
                    .map(|(e, c)| {
                        let exponents: [u8; 4] = e
                            .iter()
                            .map(|v| {
                                u8::try_from(*v).map_err(|_| {
                                    "chain exponent exceeds packed coordinate".to_string()
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?
                            .try_into()
                            .map_err(|_| "chain variable count")?;
                        Ok((mono(0, exponents), ring.field.from_u64(*c)))
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let mut opts = TowerF4Options::new(limits.degree);
    opts.full_rank_exit = false;
    opts.deadline = Some(context.deadline);
    let mut event_bytes = Vec::new();
    let mut first_error: Option<String> = None;
    let event_cap = (limits.encoded_bytes / 8).min(256 * 1024);
    let report = f4_tower_generic_bounded(
        &native_input,
        &ring,
        &opts,
        &limits.native(),
        &mut |event| {
            if let Some(error) = &first_error {
                return Err(error.clone());
            }
            let record = Event {
                boundary: event.boundary.into(),
                sequence: event.sequence,
                work_reserved: event.work_reserved,
                bytes_reserved: event.bytes_peak_reserved,
            };
            let accepted: Result<(), String> = (|| {
                let bytes = encoded(&record, 64 * 1024)?;
                if bytes.len() + 1 > event_cap.saturating_sub(event_bytes.len()) {
                    return Err("wide native boundary byte cap".into());
                }
                live_event(&json!({"kind":"wide_prime_f4_m3_boundary","record":record}))?;
                event_bytes.extend_from_slice(&bytes);
                event_bytes.push(b'\n');
                Ok(())
            })();
            if let Err(error) = &accepted {
                first_error = Some(error.clone());
            }
            accepted
        },
    );
    result.stop = stop_name(&report.stop).into();
    result.native_backend_invoked = report.native_backend_invoked;
    result.native_statistics_complete = report.native_statistics_complete;
    result.work_reserved = report.work_reserved;
    result.bytes_reserved = report.bytes_peak_reserved;
    result.acknowledged_events = report.acknowledged_events;
    #[derive(Serialize)]
    struct StatsView<'a> {
        inconsistent: bool,
        degree_reached: u32,
        solving_degree_max: u32,
        last_productive_degree: u32,
        max_cols_to_solution: usize,
        steps_to_solution: usize,
        steps: usize,
        max_rows: usize,
        max_cols: usize,
        max_nnz: u64,
        max_residual_rows: usize,
        rows_skipped: u64,
        max_dense_entries: u64,
        muladds: u64,
        critical_pairs_reduced: u64,
        tower_pairs_reduced: u64,
        pairs_product_skipped: u64,
        pairs_chain_skipped: u64,
        reducer_rows: u64,
        pairs_above_bound: usize,
        staircase_at_stop: Option<usize>,
        timed_out: bool,
        oversize: bool,
        ms: f64,
        trace: &'a [crate::cryptanalysis::f4_fp_tower::StepTrace],
        stopped_step: &'a Option<crate::cryptanalysis::f4_fp_tower::StepTrace>,
    }
    let stats = &report.native.stats;
    let view = StatsView {
        inconsistent: stats.inconsistent,
        degree_reached: stats.degree_reached,
        solving_degree_max: stats.solving_degree_max,
        last_productive_degree: stats.last_productive_degree,
        max_cols_to_solution: stats.max_cols_to_solution,
        steps_to_solution: stats.steps_to_solution,
        steps: stats.steps,
        max_rows: stats.max_rows,
        max_cols: stats.max_cols,
        max_nnz: stats.max_nnz,
        max_residual_rows: stats.max_residual_rows,
        rows_skipped: stats.rows_skipped,
        max_dense_entries: stats.max_dense_entries,
        muladds: stats.muladds,
        critical_pairs_reduced: stats.critical_pairs_reduced,
        tower_pairs_reduced: stats.tower_pairs_reduced,
        pairs_product_skipped: stats.pairs_product_skipped,
        pairs_chain_skipped: stats.pairs_chain_skipped,
        reducer_rows: stats.reducer_rows,
        pairs_above_bound: stats.pairs_above_bound,
        staircase_at_stop: stats.staircase_at_stop,
        timed_out: stats.timed_out,
        oversize: stats.oversize,
        ms: stats.ms,
        trace: &stats.trace,
        stopped_step: &stats.stopped_step,
    };
    result.native_statistics = encoded(&view, RESERVE)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok());

    data.artifacts.push(ArtifactPayload {
        path: EVENTS.into(),
        role: EVENTS_ROLE.into(),
        encoding: "jsonl".into(),
        bytes: event_bytes,
    });
    if let Some(error) = first_error {
        record_prime_chain_persistence_error(&mut data, "wide_native_observer", &error);
    }
    // Encode only within the protected adapter budget; native basis may be much
    // larger. No unbounded JSON Value construction before this check.
    let mut basis = Vec::new();
    let mut basis_bytes = 0usize;
    let basis_cap = limits.encoded_bytes / 8;
    for poly in &report.native.basis {
        if poly.monos.len() > basis_cap.saturating_sub(basis_bytes) / 96 {
            basis.clear();
            break;
        }
        let mut row = Vec::new();
        for (&m, &c) in poly.monos.iter().zip(&poly.coefs) {
            if mask_of(m) != 0 {
                return Err("wide native basis contains tower monomial".into());
            }
            let e = free_of(m);
            row.push(Term {
                exponents: e,
                coefficient: ring
                    .field
                    .to_u64(c)
                    .ok_or("wide coefficient conversion")?
                    .to_string(),
            });
        }
        row.sort_by_key(|t| t.exponents);
        let bytes = encoded(&row, basis_cap.saturating_sub(basis_bytes));
        match bytes {
            Ok(b) => {
                basis_bytes += b.len() + 1;
                basis.push(row);
            }
            Err(_) => {
                basis.clear();
                break;
            }
        }
    }
    result.basis_retained = basis.len() == report.native.basis.len();
    result.basis = basis;
    if data.outcome.get("live_persistence_errors").is_none() {
        let extraction = extract(
            &original,
            group,
            domain,
            target,
            &limits,
            Some(context.deadline),
        )?;
        if let Some(error) = &extraction.persistence_error {
            record_prime_chain_persistence_error(&mut data, "m3_group_extraction", error);
        }
        result.extraction_complete = extraction.domain_complete;
        result.exhaustive_no_decomposition = extraction.domain_complete
            && !extraction
                .rows
                .iter()
                .any(|r| r.matches_target == Some(true));
        if let Some(cursor) = extraction.first_verified_cursor {
            let indices = extraction.rows[cursor].indices;
            data.artifacts
                .push(witness_payload(domain, target, indices)?);
            result.pdp_witness_verified = true;
        }
        if result.stop == "unit_derived" && extraction.rows.iter().any(affine_root) {
            return Err("native unit contradicts independently checked affine root".into());
        }
        push(
            &mut data,
            EXTRACT,
            EXTRACT_ROLE,
            &extraction,
            limits.encoded_bytes / 8 + RESERVE,
        )?;
        result.extraction_retained = true;
    }
    let cap = limits
        .encoded_bytes
        .saturating_sub(data.artifacts.iter().map(|a| a.bytes.len()).sum::<usize>());
    if encoded(&result, cap).is_err() {
        result.basis.clear();
        result.basis_retained = false;
    }
    finish(&mut data, &result, cap)?;
    Ok(data)
}
fn residues(input: &PrimeChainInput, coordinates: &[String; 4]) -> Result<Vec<String>, String> {
    let p = BigUint::from(input.modulus);
    let xs: Vec<BigUint> = coordinates
        .iter()
        .map(|s| {
            s.parse()
                .map_err(|_| "invalid affine coordinate".to_string())
        })
        .collect::<Result<_, _>>()?;
    if xs.iter().any(|x| x >= &p) {
        return Err("affine coordinate outside field".into());
    }
    input
        .polynomials
        .iter()
        .map(|poly| {
            let mut sum = BigUint::zero();
            for (e, c) in poly {
                if e.len() != 4 || *c >= input.modulus {
                    return Err("invalid original polynomial".into());
                }
                let mut v = BigUint::from(*c);
                for (x, power) in xs.iter().zip(e) {
                    v = (v * x.modpow(&BigUint::from(*power), &p)) % &p;
                }
                sum = (sum + v) % &p;
            }
            Ok(sum.to_string())
        })
        .collect()
}
fn prefix_record(
    input: &PrimeChainInput,
    domain: &[Element],
    indices: [usize; 3],
    prefix: &Element,
) -> Result<Prefix, String> {
    match prefix.x() {
        None => Ok(Prefix::ExceptionalInfinityPrefix),
        Some(x) => {
            let coordinates = [
                domain[indices[0]]
                    .x()
                    .ok_or("nonaffine domain")?
                    .to_string(),
                domain[indices[1]]
                    .x()
                    .ok_or("nonaffine domain")?
                    .to_string(),
                domain[indices[2]]
                    .x()
                    .ok_or("nonaffine domain")?
                    .to_string(),
                x.to_string(),
            ];
            let residues = residues(input, &coordinates)?;
            if residues.first().map(String::as_str) != Some("0") {
                return Err("group prefix violates first S3 equation".into());
            }
            Ok(Prefix::Affine {
                coordinates,
                residues,
            })
        }
    }
}
fn affine_root(row: &Triple) -> bool {
    matches!(&row.prefix,Prefix::Affine{residues,..} if residues.len()==5 && residues.iter().all(|r|r=="0"))
}
fn expired(deadline: Option<Instant>) -> bool {
    deadline.is_some_and(|d| Instant::now() >= d)
}
fn extract(
    input: &PrimeChainInput,
    group: &Group,
    domain: &[Element],
    target: &Element,
    limits: &Limits,
    deadline: Option<Instant>,
) -> Result<Extraction, String> {
    extract_with_clock(input, group, domain, target, limits, &mut || {
        expired(deadline)
    })
}
fn witness_payload(
    domain: &[Element],
    target: &Element,
    indices: [usize; 3],
) -> Result<ArtifactPayload, String> {
    let value = json!({"summands":indices.map(|i|domain[i].json()),"target":target.json(),"sum":target.json(),"replay_group_add_calls":3});
    Ok(ArtifactPayload {
        path: "pdp-witness.json".into(),
        role: "independently-verified-pdp".into(),
        encoding: "json".into(),
        bytes: encoded(&value, RESERVE)?,
    })
}
trait ExtractionPersistence {
    fn witness(&mut self, payload: &ArtifactPayload) -> Result<(), String>;
    fn event(&mut self, event: &Value) -> Result<(), String>;
}
struct LiveExtraction;
impl ExtractionPersistence for LiveExtraction {
    fn witness(&mut self, payload: &ArtifactPayload) -> Result<(), String> {
        live_payload(payload)
    }
    fn event(&mut self, event: &Value) -> Result<(), String> {
        live_event(event)
    }
}
fn extract_with_clock(
    input: &PrimeChainInput,
    group: &Group,
    domain: &[Element],
    target: &Element,
    limits: &Limits,
    deadline_expired: &mut dyn FnMut() -> bool,
) -> Result<Extraction, String> {
    extract_with_persistence(
        input,
        group,
        domain,
        target,
        limits,
        deadline_expired,
        &mut LiveExtraction,
    )
}
fn extract_with_persistence(
    input: &PrimeChainInput,
    group: &Group,
    domain: &[Element],
    target: &Element,
    limits: &Limits,
    deadline_expired: &mut dyn FnMut() -> bool,
    persistence: &mut dyn ExtractionPersistence,
) -> Result<Extraction, String> {
    let mut out = Extraction {
        schema: "crypto.ml-dataset.wide-prime-f4-m3-extraction/v1".into(),
        policy: POLICY.into(),
        signed_domain: domain.iter().map(Element::json).collect(),
        rows: vec![],
        domain_complete: false,
        stop: "complete".into(),
        persistence_error: None,
        group_adds: 0,
        witness_replay_adds: 0,
        original_equation_evaluations: 0,
        witness_equation_replays: 0,
        first_verified_cursor: None,
    };
    let d = domain.len();
    let total = d.checked_pow(3).ok_or("triple count overflow")?;
    for cursor in 0..total {
        if cursor as u64 >= limits.domain_tests {
            out.stop = "domain_budget".into();
            break;
        }
        if deadline_expired() {
            out.stop = "deadline".into();
            break;
        }
        if limits.group_adds.saturating_sub(out.group_adds) < 2 {
            out.stop = "group_budget".into();
            break;
        }
        let cap = limits.encoded_bytes / 8;
        if cap.saturating_sub(encoded(&out, cap)?.len()) < 4096 {
            out.stop = "extraction_bytes".into();
            break;
        }
        let indices = [cursor / (d * d), (cursor / d) % d, cursor % d];
        let prefix = group.add(&domain[indices[0]], &domain[indices[1]]);
        out.group_adds += 1;
        let branch = prefix_record(input, domain, indices, &prefix)?;
        if matches!(branch, Prefix::Affine { .. }) {
            out.original_equation_evaluations += 5;
        }
        let mut row = Triple {
            indices,
            prefix: branch,
            total: None,
            matches_target: None,
        };
        if deadline_expired() {
            out.rows.push(row);
            out.stop = "deadline_between_adds".into();
            break;
        }
        let total_point = group.add(&prefix, &domain[indices[2]]);
        out.group_adds += 1;
        let matched = &total_point == target;
        row.total = Some(total_point.json());
        row.matches_target = Some(matched);
        if matched && matches!(row.prefix, Prefix::Affine { .. }) && !affine_root(&row) {
            return Err("matched affine group triple violates original equations".into());
        }
        if encoded(&row, 4032).is_err() {
            return Err("triple exceeded reserved bytes".into());
        }
        out.rows.push(row);
        if matched && out.first_verified_cursor.is_none() {
            if limits.group_adds.saturating_sub(out.group_adds) < 3 {
                out.stop = "witness_replay_budget".into();
                break;
            }
            let mut sum = group.zero();
            for i in indices {
                if deadline_expired() {
                    out.stop = "witness_replay_deadline".into();
                    break;
                }
                sum = group.add(&sum, &domain[i]);
                out.group_adds += 1;
                out.witness_replay_adds += 1;
            }
            if out.stop != "complete" {
                break;
            }
            if &sum != target {
                return Err("independent triple replay failed".into());
            }
            if let Prefix::Affine {
                coordinates,
                residues: saved,
            } = &out.rows[cursor].prefix
            {
                if residues(input, coordinates)? != *saved {
                    return Err("witness equation replay mismatch".into());
                }
                out.witness_equation_replays += 5;
            }
            out.first_verified_cursor = Some(cursor);
            // Publish the dedicated certificate before another triple can run.
            // Later canonical staging uses the same compact bounded bytes.
            let payload = witness_payload(domain, target, indices)?;
            if let Err(error) = persistence.witness(&payload) {
                out.persistence_error = Some(error.chars().take(512).collect());
                out.stop = "observer_failure".into();
                break;
            }
        }
        if let Err(error) = persistence.event(
            &json!({"kind":"wide_prime_f4_m3_group_triple","cursor":cursor,"record":out.rows.last(),"group_adds":out.group_adds,"witness_replay_adds":out.witness_replay_adds,"first_verified_cursor":out.first_verified_cursor}),
        ) {
            out.persistence_error = Some(error.chars().take(512).collect());
            out.stop = "observer_failure".into();
            break;
        }
    }
    if out.persistence_error.is_none() && out.stop != "complete" {
        if let Err(error) = persistence.event(
            &json!({"kind":"wide_prime_f4_m3_group_stop","stop":out.stop,"last_record":out.rows.last(),"group_adds":out.group_adds,"witness_replay_adds":out.witness_replay_adds,"first_verified_cursor":out.first_verified_cursor}),
        ) {
            out.persistence_error = Some(error.chars().take(512).collect());
        }
    }
    out.domain_complete = out.stop == "complete"
        && out.rows.len() == total
        && out.rows.iter().all(|r| r.total.is_some());
    Ok(out)
}
fn file_cap(path: &str, limits: Option<&Limits>) -> usize {
    match path {
        INPUT | "pdp-witness.json" => RESERVE,
        RESULT => limits.map_or(RESERVE, |l| l.encoded_bytes),
        EVENTS => limits.map_or(0, |l| (l.encoded_bytes / 8).min(256 * 1024)),
        EXTRACT => limits.map_or(0, |l| l.encoded_bytes / 8 + RESERVE),
        _ => 0,
    }
}
fn read<T: for<'de> Deserialize<'de>>(
    dir: &Path,
    path: &str,
    limits: Option<&Limits>,
) -> Result<T, String> {
    serde_json::from_slice(&read_bounded(dir, path, file_cap(path, limits))?)
        .map_err(|e| e.to_string())
}
fn verify_m3_claims(outcome: &Value) -> Result<(), String> {
    verify_unproved_claims(outcome)?;
    // These proposed aliases are not emitted by this version. The typed result
    // and extraction record own their meanings; arbitrary outer labels cannot.
    for key in [
        "group_domain_complete",
        "group_prefix_complete",
        "group_infinity_prefixes_covered",
        "first_certificate_retained",
    ] {
        if !outcome[key].is_null() && outcome[key] != false {
            return Err(format!("unregistered m3 summary claim: {key}"));
        }
    }
    Ok(())
}
fn verify_bytes(
    dir: &Path,
    artifacts: &[super::super::types::Artifact],
    limits: Option<&Limits>,
) -> Result<(), String> {
    let mut total = 0u64;
    for a in artifacts
        .iter()
        .filter(|a| [INPUT, RESULT, EVENTS, EXTRACT, "pdp-witness.json"].contains(&a.path.as_str()))
    {
        let meta = fs::symlink_metadata(dir.join(&a.path)).map_err(|e| e.to_string())?;
        if !meta.is_file()
            || meta.file_type().is_symlink()
            || meta.len() != a.bytes
            || a.bytes > file_cap(&a.path, limits) as u64
        {
            return Err("m3 serialized byte admission".into());
        }
        total = total.checked_add(a.bytes).ok_or("m3 byte overflow")?;
    }
    if total > limits.map_or(2 * RESERVE, |l| l.encoded_bytes) as u64 {
        return Err("m3 aggregate byte admission".into());
    }
    Ok(())
}
pub(super) fn verify(
    dir: &Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[super::super::types::Artifact],
) -> Result<WidePrimeReplay, String> {
    let applicable =
        config.family == "prime" && config.solver == "f4" && config.m == 3 && config.n >= 33;
    let reserved = artifacts.iter().any(|a| {
        [RESULT, EVENTS, EXTRACT].contains(&a.path.as_str())
            || [RESULT_ROLE, EVENTS_ROLE, EXTRACT_ROLE].contains(&a.role.as_str())
    });
    let context = super::super::execution::load(dir)?;
    let claimed = [
        "solver_completed",
        "native_backend_invoked",
        "native_statistics_complete",
        "algebra_completed",
        "extraction_complete",
        "pdp_witness_verified",
        "exhaustive_no_decomposition",
    ]
    .iter()
    .any(|k| outcome[*k] == true);
    if !reserved
        && outcome["strategy"] != STRATEGY
        && !(applicable && (context.is_some() || claimed))
    {
        return Ok(WidePrimeReplay::Ordinary);
    }
    verify_m3_claims(outcome)?;
    if !applicable {
        return Err("m3 strategy/config mismatch".into());
    }
    let censored = matches!(
        outcome["status"].as_str(),
        Some("timeout" | "trace_budget_exhausted" | "worker_failed")
    );
    let parts: Vec<_> = artifacts
        .iter()
        .filter(|a| [INPUT, RESULT, EVENTS, EXTRACT].contains(&a.path.as_str()))
        .collect();
    if censored {
        verify_censored_outcome(outcome)?;
    }
    if censored
        && !artifacts
            .iter()
            .any(|a| [RESULT_ROLE, EVENTS_ROLE, EXTRACT_ROLE].contains(&a.role.as_str()))
        && (parts.is_empty()
            || parts
                .iter()
                .all(|a| a.role == "unregistered-partial-artifact" && a.encoding == "native"))
    {
        if artifacts.iter().any(|a| {
            [
                "prime-f4-wide-result.json",
                "prime-f4-wide-events.jsonl",
                "prime-f4-root-extraction.json",
            ]
            .contains(&a.path.as_str())
                || [
                    "restricted-wide-prime-f4-result/v1",
                    "restricted-wide-prime-f4-boundary-events/v1",
                    "restricted-wide-prime-f4-original-domain-replay/v1",
                ]
                .contains(&a.role.as_str())
        }) {
            return Err("m3 raw prefix contains foreign m2 artifacts".into());
        }
        if artifacts.iter().any(|a| {
            ["pdp-witness.json", "solver-domain.json", "result.json"].contains(&a.path.as_str())
                && a.role != "unregistered-partial-artifact"
        }) {
            return Err(
                "registered mathematical evidence cannot bypass m3 replay as raw prefix".into(),
            );
        }
        if context.is_some() {
            artifact(
                artifacts,
                super::super::execution::PATH,
                super::super::execution::ROLE,
                "json",
                true,
            )?;
        }
        let limits = context
            .as_ref()
            .and_then(|c| Limits::derive(config, c).ok());
        verify_bytes(dir, artifacts, limits.as_ref())?;
        return Ok(WidePrimeReplay::CensoredM3Prefix);
    }
    if outcome["status"] == "error"
        && !claimed
        && !reserved
        && parts.is_empty()
        && outcome["strategy"] != STRATEGY
    {
        for path in [
            INPUT,
            RESULT,
            EVENTS,
            EXTRACT,
            "pdp-witness.json",
            "solver-domain.json",
        ] {
            if fs::symlink_metadata(dir.join(path)).is_ok() {
                return Err("setup error has solver artifacts".into());
            }
        }
        verify_censored_outcome(outcome)?;
        artifact(artifacts, "result.json", "solver-outcome", "json", true)?;
        let retained: Value = serde_json::from_slice(&read_bounded(dir, "result.json", RESERVE)?)
            .map_err(|e| e.to_string())?;
        let mut outer = outcome.clone();
        outer
            .as_object_mut()
            .ok_or("outcome object")?
            .remove("supervisor");
        if retained != outer || retained["reason"].as_str().is_none() {
            return Err("setup error differs from retained outcome".into());
        }
        return Ok(WidePrimeReplay::Ordinary);
    }
    if context.is_some() {
        artifact(
            artifacts,
            super::super::execution::PATH,
            super::super::execution::ROLE,
            "json",
            true,
        )?;
    }
    let limits = context
        .as_ref()
        .and_then(|c| Limits::derive(config, c).ok());
    verify_bytes(dir, artifacts, limits.as_ref())?;
    for (path, role) in [
        (INPUT, INPUT_ROLE),
        (RESULT, RESULT_ROLE),
        ("public.json", "public-instance"),
        ("factor-base.json", "factor-base"),
    ] {
        artifact(artifacts, path, role, "json", true)?;
    }
    let case = CaseData {
        public_instance: super::super::read_json(&dir.join("public.json"))?,
        factor_base: super::super::read_json(&dir.join("factor-base.json"))?,
        private_witness: Value::Null,
    };
    let public = &case.public_instance;
    if public["family"] != config.family
        || public["n"] != json!(config.n)
        || public["m"] != json!(config.m)
        || public["twist"] != json!(config.twist)
    {
        return Err("m3 public/config mismatch".into());
    }
    curves::verify_public(public)?;
    curves::verify_factor_base(public, &case.factor_base)?;
    let group = Group::read(&case)?;
    let target = group.point(&public["target"])?;
    let domain = domain(&group, &case)?;
    let original = prime_chain_input(3, &group, &domain, &target)?;
    let saved: Value = read(dir, INPUT, limits.as_ref())?;
    if saved != original.artifact {
        return Err("m3 canonical equations mismatch".into());
    }
    let result: ResultRecord = read(dir, RESULT, limits.as_ref())?;
    let domain_descriptor = artifact(
        artifacts,
        "solver-domain.json",
        "exact-signed-factor-base",
        "json",
        result.native_backend_invoked
            || result.extraction_retained
            || result.work_reserved != 0
            || result.acknowledged_events != 0,
    )?;
    if domain_descriptor.is_some() {
        let saved_domain: Value =
            serde_json::from_slice(&read_bounded(dir, "solver-domain.json", RESERVE)?)
                .map_err(|e| e.to_string())?;
        let expected_domain = json!({"points":domain.iter().map(Element::json).collect::<Vec<_>>(),"negation_closed":true,"repetitions_allowed":true});
        if saved_domain != expected_domain {
            return Err("m3 signed solver domain differs from canonical domain".into());
        }
    } else if fs::symlink_metadata(dir.join("solver-domain.json")).is_ok() {
        return Err("m3 solver domain lacks manifest binding".into());
    }
    if result.schema != "crypto.ml-dataset.wide-prime-f4-m3-result/v1"
        || result.strategy != STRATEGY
        || result.launch != context
        || result.limits != limits
        || result.native_basis_derivation_verified
        || result.execution_trace_complete_verified
        || result.total_operation_count_verified
    {
        return Err("m3 result context/schema/unproved claim".into());
    }
    if limits.is_none()
        && result.stop
            != if context.is_some() {
                "profile_admission"
            } else {
                "missing_execution_context"
            }
    {
        return Err("m3 admission refusal mismatch".into());
    }
    if limits.is_some()
        && matches!(
            result.stop.as_str(),
            "missing_execution_context" | "profile_admission"
        )
    {
        return Err("m3 admitted context cannot claim admission refusal".into());
    }
    let complete = matches!(result.stop.as_str(), "complete" | "unit_derived");
    if result.native_statistics_complete != complete || complete && !result.native_backend_invoked {
        return Err("wide F4 native completion/stop mismatch".into());
    }
    if ![
        "complete",
        "unit_derived",
        "degree_limit",
        "work_limit",
        "storage_admission",
        "deadline",
        "observer_failure",
        "invalid_input",
        "missing_execution_context",
        "profile_admission",
    ]
    .contains(&result.stop.as_str())
    {
        return Err("unknown wide F4 stop".into());
    }
    if let Some(l) = &result.limits {
        if result.work_reserved > l.work
            || result.bytes_reserved > l.bytes
            || result.acknowledged_events > l.events
            || result.basis.len() > l.basis
        {
            return Err("wide F4 counters exceed admission".into());
        }
    } else if result.native_backend_invoked
        || result.native_statistics.is_some()
        || result.basis_retained
        || result.work_reserved != 0
        || result.bytes_reserved != 0
        || result.acknowledged_events != 0
        || !result.basis.is_empty()
        || result.extraction_retained
    {
        return Err("wide F4 work without admission".into());
    }
    if let Some(stats) = &result.native_statistics {
        if !stats.ms.is_finite()
            || stats.ms < 0.0
            || stats.trace.len() > 64
            || stats
                .trace
                .iter()
                .chain(stats.stopped_step.iter())
                .any(|step| {
                    [
                        step.ms,
                        step.ms_rows,
                        step.ms_reduce,
                        step.ms_echelon,
                        step.ms_update,
                    ]
                    .iter()
                    .any(|v| !v.is_finite() || *v < 0.0)
                })
        {
            return Err("invalid native diagnostic timing/trace".into());
        }
    }
    let p = original.modulus;
    if !result.basis_retained && !result.basis.is_empty() {
        return Err("wide F4 partial basis mislabeled".into());
    }
    let mut terms = 0usize;
    for poly in &result.basis {
        terms = terms.checked_add(poly.len()).ok_or("basis term overflow")?;
        if terms > 131072 || !poly.windows(2).all(|w| w[0].exponents < w[1].exponents) {
            return Err("wide F4 noncanonical basis".into());
        }
        for t in poly {
            if t.exponents.iter().any(|e| *e > 64)
                || number(&t.coefficient)? == 0
                || number(&t.coefficient)? >= p
            {
                return Err("wide F4 invalid basis term".into());
            }
        }
    }
    let event_descriptor = artifact(
        artifacts,
        EVENTS,
        EVENTS_ROLE,
        "jsonl",
        result.native_backend_invoked || result.acknowledged_events > 0,
    )?;
    let mut events = Vec::new();
    if event_descriptor.is_some() {
        let bytes = read_bounded(dir, EVENTS, file_cap(EVENTS, limits.as_ref()))?;
        if !bytes.is_empty() && bytes.last() != Some(&b'\n') {
            return Err("wide boundary stream lacks final newline".into());
        }
        let content = bytes.strip_suffix(b"\n").unwrap_or(&bytes);
        if !content.is_empty() {
            for line in content.split(|b| *b == b'\n') {
                if line.is_empty() || line.len() > 64 * 1024 {
                    return Err("wide boundary line exceeds admission or is empty".into());
                }
                events.push(serde_json::from_slice::<Event>(line).map_err(|e| e.to_string())?);
            }
        } else if !bytes.is_empty() {
            return Err("wide boundary stream contains only empty lines".into());
        }
    }
    if events.len() != result.acknowledged_events {
        return Err("wide F4 acknowledged boundary mismatch".into());
    }
    let mut work = 0;
    let mut bytes = 0;
    for (i, event) in events.iter().enumerate() {
        if event.sequence != i + 1
            || event.work_reserved < work
            || event.bytes_reserved < bytes
            || event.work_reserved > result.work_reserved
            || event.bytes_reserved > result.bytes_reserved
            || !matches!(
                event.boundary.as_str(),
                "begin" | "initial_echelon" | "step" | "terminal"
            )
            || i == 0 && event.boundary != "begin"
            || i > 0 && event.boundary == "begin"
            || event.boundary == "terminal" && i + 1 != events.len()
            || event.boundary == "initial_echelon" && i != 1
        {
            return Err("wide F4 boundary prefix mismatch".into());
        }
        work = event.work_reserved;
        bytes = event.bytes_reserved;
    }
    if events.last().is_some_and(|e| {
        e.boundary == "terminal"
            && (e.work_reserved != result.work_reserved
                || e.bytes_reserved != result.bytes_reserved)
    }) {
        return Err("m3 acknowledged terminal totals differ from result".into());
    }
    if complete && events.last().is_none_or(|e| e.boundary != "terminal") {
        return Err("complete wide F4 lacks terminal acknowledgement".into());
    }
    let descriptor = artifact(
        artifacts,
        EXTRACT,
        EXTRACT_ROLE,
        "json",
        result.extraction_retained,
    )?;
    if descriptor.is_some() != result.extraction_retained {
        return Err("m3 extraction descriptor mismatch".into());
    }
    if result.extraction_retained {
        let extraction: Extraction = read(dir, EXTRACT, limits.as_ref())?;
        verify_extraction(
            &original,
            &extraction,
            &group,
            &domain,
            &target,
            limits.as_ref().ok_or("extraction without limits")?,
        )?;
        if extraction.persistence_error.is_some()
            && outcome.get("live_persistence_errors").is_none()
            && !censored
        {
            return Err("m3 lost sticky persistence failure".into());
        }
        let positive = extraction.first_verified_cursor.is_some();
        let negative = extraction.domain_complete
            && !extraction
                .rows
                .iter()
                .any(|r| r.matches_target == Some(true));
        if positive != result.pdp_witness_verified
            || negative != result.exhaustive_no_decomposition
            || extraction.domain_complete != result.extraction_complete
        {
            return Err("m3 extraction claims mismatch".into());
        }
        if (result.stop == "unit_derived"
            || result
                .native_statistics
                .as_ref()
                .is_some_and(|s| s.inconsistent))
            && extraction.rows.iter().any(affine_root)
        {
            return Err("unit contradicts affine domain root".into());
        }
        let witness = artifact(
            artifacts,
            "pdp-witness.json",
            "independently-verified-pdp",
            "json",
            positive,
        )?;
        if witness.is_some() != positive {
            return Err("m3 witness descriptor mismatch".into());
        }
        if let Some(cursor) = extraction.first_verified_cursor {
            let got: Value = read(dir, "pdp-witness.json", limits.as_ref())?;
            let expected = json!({"summands":extraction.rows[cursor].indices.map(|i|domain[i].json()),"target":target.json(),"sum":target.json(),"replay_group_add_calls":3});
            if got != expected {
                return Err("m3 certificate differs from first verified triple".into());
            }
        }
    } else if result.extraction_complete
        || result.exhaustive_no_decomposition
        || result.pdp_witness_verified
    {
        return Err("m3 extraction claims lack evidence".into());
    }
    let retained;
    let effective = if censored {
        artifact(artifacts, "result.json", "solver-outcome", "json", true)?;
        retained = serde_json::from_slice::<Value>(&read_bounded(dir, "result.json", RESERVE)?)
            .map_err(|e| e.to_string())?;
        &retained
    } else {
        outcome
    };
    verify_m3_claims(effective)?;
    if effective["counters"]
        != json!({"native_work_reserved":result.work_reserved,"native_cumulative_bytes_reserved":result.bytes_reserved,"acknowledged_native_boundaries":result.acknowledged_events})
    {
        return Err("m3 worker counters differ from retained native result".into());
    }
    let persistence_failed = effective.get("live_persistence_errors").is_some();
    if result.extraction_retained {
        let extraction: Extraction = read(dir, EXTRACT, limits.as_ref())?;
        if extraction.persistence_error.is_some() && !persistence_failed {
            return Err("m3 retained outcome lost persistence failure".into());
        }
    }
    for (key, expected) in [
        ("native_backend_invoked", result.native_backend_invoked),
        ("algebra_completed", result.native_statistics_complete),
        (
            "native_statistics_complete",
            result.native_statistics_complete,
        ),
        ("extraction_complete", result.extraction_complete),
        (
            "exhaustive_no_decomposition",
            result.exhaustive_no_decomposition,
        ),
        ("pdp_witness_verified", result.pdp_witness_verified),
        (
            "solver_completed",
            !persistence_failed
                && (result.exhaustive_no_decomposition || result.pdp_witness_verified),
        ),
    ] {
        if effective[key] != json!(expected) {
            return Err(format!("m3 worker outcome mismatch: {key}"));
        }
    }
    if effective["strategy"] != STRATEGY
        || effective["termination"] != result.stop
        || effective["status"]
            != if result.pdp_witness_verified && !persistence_failed {
                "verified"
            } else {
                "partial"
            }
    {
        return Err("m3 worker classification mismatch".into());
    }
    Ok(WidePrimeReplay::Ordinary)
}
fn verify_extraction(
    input: &PrimeChainInput,
    saved: &Extraction,
    group: &Group,
    domain: &[Element],
    target: &Element,
    limits: &Limits,
) -> Result<(), String> {
    let d = domain.len();
    let total = d.checked_pow(3).ok_or("triple count overflow")?;
    if saved.stop == "observer_failure" && saved.persistence_error.is_none()
        || saved
            .persistence_error
            .as_ref()
            .is_some_and(|e| e.chars().count() > 512)
    {
        return Err("m3 persistence stop mismatch".into());
    }
    if saved.schema != "crypto.ml-dataset.wide-prime-f4-m3-extraction/v1"
        || saved.policy != POLICY
        || saved.signed_domain != domain.iter().map(Element::json).collect::<Vec<_>>()
        || saved.rows.len() > total
        || saved.rows.len() as u64 > limits.domain_tests
        || saved.group_adds > limits.group_adds
        || saved.witness_replay_adds > 3
    {
        return Err("m3 extraction header/caps".into());
    }
    let mut adds = 0;
    let mut evaluations = 0;
    let mut first_match = None;
    for (cursor, row) in saved.rows.iter().enumerate() {
        let prior_witness = saved.first_verified_cursor.is_some_and(|i| i < cursor);
        let prior_replay = if prior_witness { 3 } else { 0 };
        let prefix_state = Extraction {
            schema: saved.schema.clone(),
            policy: POLICY.into(),
            signed_domain: saved.signed_domain.clone(),
            rows: saved.rows[..cursor].to_vec(),
            domain_complete: false,
            stop: "complete".into(),
            persistence_error: None,
            group_adds: adds + prior_replay,
            witness_replay_adds: prior_replay,
            original_equation_evaluations: evaluations,
            witness_equation_replays: if prior_witness {
                saved.witness_equation_replays
            } else {
                0
            },
            first_verified_cursor: saved.first_verified_cursor.filter(|i| *i < cursor),
        };
        let cap = limits.encoded_bytes / 8;
        if limits.group_adds.saturating_sub(adds + prior_replay) < 2
            || cap.saturating_sub(encoded(&prefix_state, cap)?.len()) < 4096
            || encoded(row, 4032).is_err()
        {
            return Err("m3 triple lacks preadmitted work/record envelope".into());
        }
        let indices = [cursor / (d * d), (cursor / d) % d, cursor % d];
        if row.indices != indices {
            return Err("m3 ordered triple prefix differs".into());
        }
        let prefix = group.add(&domain[indices[0]], &domain[indices[1]]);
        adds += 1;
        if row.prefix != prefix_record(input, domain, indices, &prefix)? {
            return Err("m3 prefix or original equations differ".into());
        }
        if matches!(row.prefix, Prefix::Affine { .. }) {
            evaluations += 5;
        }
        match (&row.total, row.matches_target) {
            (Some(total_value), Some(matched)) => {
                let sum = group.add(&prefix, &domain[indices[2]]);
                adds += 1;
                if *total_value != sum.json() || matched != (&sum == target) {
                    return Err("m3 group total or match differs".into());
                }
                if matched {
                    if matches!(row.prefix, Prefix::Affine { .. }) && !affine_root(row) {
                        return Err("matched affine triple not root".into());
                    }
                    if first_match.is_none() {
                        first_match = Some(cursor);
                    }
                }
            }
            (None, None)
                if cursor + 1 == saved.rows.len() && saved.stop == "deadline_between_adds" => {}
            _ => return Err("m3 incomplete triple stage".into()),
        }
    }
    if saved.group_adds != adds + saved.witness_replay_adds
        || saved.original_equation_evaluations != evaluations
    {
        return Err("m3 arithmetic counters differ".into());
    }
    match saved.first_verified_cursor {
        Some(cursor) => {
            if Some(cursor) != first_match || saved.witness_replay_adds != 3 {
                return Err("m3 first certificate selection".into());
            }
            if limits.group_adds.saturating_sub(2 * (cursor as u64 + 1)) < 3 {
                return Err("m3 certificate lacks three-add permit".into());
            }
            let row = &saved.rows[cursor];
            let mut sum = group.zero();
            for i in row.indices {
                sum = group.add(&sum, &domain[i]);
            }
            if &sum != target {
                return Err("m3 independent certificate replay".into());
            }
            let expected = if matches!(row.prefix, Prefix::Affine { .. }) {
                5
            } else {
                0
            };
            if saved.witness_equation_replays != expected {
                return Err("m3 witness equation counter".into());
            }
        }
        None => {
            if saved.witness_equation_replays != 0 || saved.witness_replay_adds == 3 {
                return Err("m3 missing completed certificate".into());
            }
            if let Some(cursor) = first_match {
                // The producer reserves all three calls before checking any
                // certificate-replay deadline, even if zero calls finish.
                if saved.stop == "witness_replay_deadline"
                    && limits.group_adds.saturating_sub(2 * (cursor as u64 + 1)) < 3
                {
                    return Err("m3 partial certificate lacks three-add permit".into());
                }
                if cursor + 1 != saved.rows.len()
                    || !matches!(
                        saved.stop.as_str(),
                        "witness_replay_budget" | "witness_replay_deadline"
                    )
                {
                    return Err("m3 ignored matching candidate".into());
                }
            } else if saved.witness_replay_adds != 0 {
                return Err("m3 replay without candidate".into());
            }
        }
    }
    let full = saved.rows.len() == total && saved.rows.iter().all(|r| r.total.is_some());
    if saved.domain_complete != (saved.stop == "complete" && full) {
        return Err("m3 false full-domain label".into());
    }
    match saved.stop.as_str() {
        "complete" if full => {}
        "domain_budget" if saved.rows.len() as u64 == limits.domain_tests && !full => {}
        "group_budget" if limits.group_adds.saturating_sub(saved.group_adds) < 2 && !full => {}
        "witness_replay_budget"
            if first_match.is_some()
                && saved.first_verified_cursor.is_none()
                && saved.witness_replay_adds == 0
                && limits.group_adds.saturating_sub(saved.group_adds) < 3 => {}
        "witness_replay_deadline"
            if first_match.is_some()
                && saved.first_verified_cursor.is_none()
                && saved.witness_replay_adds < 3 => {}
        "deadline_between_adds" if saved.rows.last().is_some_and(|r| r.total.is_none()) => {}
        "deadline" if !full => {}
        "extraction_bytes" if !full => {
            let cap = limits.encoded_bytes / 8;
            let mut pre_next = saved.clone();
            pre_next.stop = "complete".into();
            pre_next.persistence_error = None;
            pre_next.domain_complete = false;
            if saved.rows.len() as u64 >= limits.domain_tests
                || limits.group_adds.saturating_sub(saved.group_adds) < 2
                || cap.saturating_sub(encoded(&pre_next, cap)?.len()) >= 4096
            {
                return Err("m3 extraction byte stop lacks pre-next-row shortage".into());
            }
        }
        "observer_failure" if saved.persistence_error.is_some() => {}
        _ => return Err("m3 extraction stop inconsistent".into()),
    }
    Ok(())
}
pub(super) fn begin_failure(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
    context: &SolverExecutionContext,
    error: &str,
) -> Result<SolverData, String> {
    let mut data = SolverData {
        outcome: outcome("partial", "Initial stream refused before native m3 entry"),
        artifacts: vec![],
    };
    data.outcome["strategy"] = json!(STRATEGY);
    push(
        &mut data,
        INPUT,
        INPUT_ROLE,
        &prime_chain_input(3, group, domain, target)?.artifact,
        RESERVE,
    )?;
    record_prime_chain_persistence_error(&mut data, "solver_begin", error);
    let mut result = base_result();
    result.launch = Some(context.launch.clone());
    result.limits = Limits::derive(config, &context.launch).ok();
    result.stop = if result.limits.is_some() {
        "observer_failure"
    } else {
        "profile_admission"
    }
    .into();
    finish(&mut data, &result, RESERVE)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture_at(twist: bool, n: u32) -> (CaseConfig, CaseData, SolverExecutionContext) {
        let cfg = CaseConfig {
            family: "prime".into(),
            solver: "f4".into(),
            n,
            m: 3,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 2,
            degree_bound: 8,
            work_limit: 50_000,
        };
        let mut case = curves::generate(&cfg).unwrap();
        case.private_witness = Value::Null;
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let first = points.iter().min_by_key(|p| p.x()).unwrap();
        case.public_instance["target"] = group.mul(first, 3).json();
        let binding = AttemptBinding {
            schema: "crypto.ml-dataset.attempt-binding/v1".into(),
            run_sha256: "1".repeat(64),
            plan_sha256: "2".repeat(64),
            protocol_sha256: "3".repeat(64),
            config_sha256: "4".repeat(64),
            source_commit: "5".repeat(40),
            binary_sha256: "6".repeat(64),
            cell_index: 0,
            attempt_sha256: "7".repeat(64),
        };
        let launch = LaunchContext {
            schema: "crypto.ml-dataset.execution-context/v1".into(),
            binding,
            boot_id: "00000000-0000-0000-0000-000000000000".into(),
            start_monotonic_ns: 1,
            deadline_monotonic_ns: 30_000_000_001,
            seconds_per_case: 30,
            memory_bytes: 512 * 1024 * 1024,
            trace_bytes: 64 * 1024 * 1024,
        };
        (
            cfg,
            case,
            SolverExecutionContext {
                launch,
                deadline: Instant::now() + Duration::from_secs(30),
            },
        )
    }
    fn record<T: for<'de> Deserialize<'de>>(data: &SolverData, path: &str) -> T {
        serde_json::from_slice(
            &data
                .artifacts
                .iter()
                .find(|a| a.path == path)
                .unwrap()
                .bytes,
        )
        .unwrap()
    }

    #[test]
    fn fixed_m3_native_positive_negative_and_infinity_both_twists() {
        for n in [47, 53, 61] {
            for twist in [false, true] {
                let (cfg, mut case, context) = fixture_at(twist, n);
                let group = Group::read(&case).unwrap();
                let points = domain(&group, &case).unwrap();
                assert_eq!(points.len(), 2);
                assert_eq!(points[1], group.neg(&points[0]));
                for k in 1..=7 {
                    assert_ne!(group.mul(&points[0], k), group.zero());
                }
                for scalar in [3, 4, 1] {
                    let target = group.mul(&points[0], scalar);
                    case.public_instance["target"] = target.json();
                    let data =
                        super::super::solve_with_context(&cfg, &case, Some(&context)).unwrap();
                    assert_eq!(data.outcome["native_backend_invoked"], true);
                    let result: ResultRecord = record(&data, RESULT);
                    let extraction: Extraction = record(&data, EXTRACT);
                    let original = prime_chain_input(3, &group, &points, &target).unwrap();
                    verify_extraction(
                        &original,
                        &extraction,
                        &group,
                        &points,
                        &target,
                        result.limits.as_ref().unwrap(),
                    )
                    .unwrap();
                    assert!(extraction.domain_complete);
                    assert_eq!(extraction.rows.len(), 8);
                    assert_eq!(result.pdp_witness_verified, scalar != 4);
                    assert_eq!(result.exhaustive_no_decomposition, scalar == 4);
                    if scalar == 1 {
                        let first = extraction.first_verified_cursor.unwrap();
                        assert!(matches!(
                            extraction.rows[first].prefix,
                            Prefix::Affine { .. }
                        ));
                        assert!(extraction.rows.iter().skip(first + 1).any(|r| matches!(
                            r.prefix,
                            Prefix::ExceptionalInfinityPrefix
                        ) && r
                            .matches_target
                            == Some(true)));
                    }
                }
            }
        }
    }
    #[test]
    fn m3_group_caps_keep_positive_and_never_promote_incomplete_negative() {
        let (cfg, case, context) = fixture_at(false, 61);
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let mut limits = Limits::derive(&cfg, &context.launch).unwrap();
        for scalar in [3, 4] {
            let target = group.mul(&points[0], scalar);
            let input = prime_chain_input(3, &group, &points, &target).unwrap();
            for cap in [2, 4, 5, 15, 16, 19] {
                limits.group_adds = cap;
                let output = extract(&input, &group, &points, &target, &limits, None).unwrap();
                verify_extraction(&input, &output, &group, &points, &target, &limits).unwrap();
                if scalar == 3 {
                    assert_eq!(output.first_verified_cursor.is_some(), cap >= 5);
                }
                if scalar == 4 {
                    assert_eq!(output.domain_complete, cap >= 16);
                }
            }
        }
    }
    #[test]
    fn m3_typed_replay_rejects_branch_counter_cursor_and_certificate_mutations() {
        let (cfg, case, context) = fixture_at(false, 61);
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let target = points[0].clone();
        let limits = Limits::derive(&cfg, &context.launch).unwrap();
        let input = prime_chain_input(3, &group, &points, &target).unwrap();
        let good = extract(&input, &group, &points, &target, &limits, None).unwrap();
        verify_extraction(&input, &good, &group, &points, &target, &limits).unwrap();
        let mut bad = good.clone();
        bad.group_adds += 1;
        assert!(verify_extraction(&input, &bad, &group, &points, &target, &limits).is_err());
        bad = good.clone();
        bad.rows[0].indices.swap(0, 1);
        bad.rows[0].indices[0] = 1;
        assert!(verify_extraction(&input, &bad, &group, &points, &target, &limits).is_err());
        bad = good.clone();
        bad.rows[0].prefix = Prefix::ExceptionalInfinityPrefix;
        assert!(verify_extraction(&input, &bad, &group, &points, &target, &limits).is_err());
        bad = good.clone();
        bad.first_verified_cursor = Some(7);
        assert!(verify_extraction(&input, &bad, &group, &points, &target, &limits).is_err());
        bad = good.clone();
        bad.rows.pop();
        assert!(verify_extraction(&input, &bad, &group, &points, &target, &limits).is_err());
        let mut v = serde_json::to_value(good).unwrap();
        v["rows"][0]["prefix"]["injected"] = json!(true);
        assert!(serde_json::from_value::<Extraction>(v).is_err());
    }
    #[test]
    fn m3_original_deadline_between_adds_and_before_certificate_is_censored() {
        let (cfg, case, context) = fixture_at(false, 61);
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let target = group.mul(&points[0], 3);
        let input = prime_chain_input(3, &group, &points, &target).unwrap();
        let limits = Limits::derive(&cfg, &context.launch).unwrap();
        for stop_check in [1, 2, 3, 4, 5] {
            let mut checks = 0;
            let output = extract_with_clock(&input, &group, &points, &target, &limits, &mut || {
                checks += 1;
                checks >= stop_check
            })
            .unwrap();
            verify_extraction(&input, &output, &group, &points, &target, &limits).unwrap();
            assert!(!output.domain_complete);
            assert!(output.first_verified_cursor.is_none());
            if stop_check == 2 {
                assert_eq!(output.group_adds, 1);
                assert!(output.rows[0].total.is_none());
            }
            if stop_check >= 3 {
                assert_eq!(output.rows[0].matches_target, Some(true));
                assert_eq!(output.witness_replay_adds, stop_check - 3);
            }
        }
    }
    #[test]
    fn m3_initial_persistence_refusal_stages_canonical_equations() {
        let (cfg, case, context) = fixture_at(false, 61);
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let target = group.point(&case.public_instance["target"]).unwrap();
        let output =
            begin_failure(&cfg, &group, &points, &target, &context, "injected refusal").unwrap();
        assert_eq!(output.outcome["native_backend_invoked"], false);
        assert_eq!(output.outcome["solver_completed"], false);
        assert!(output.outcome.get("live_persistence_errors").is_some());
        let canonical: Value = record(&output, INPUT);
        assert_eq!(
            canonical,
            prime_chain_input(3, &group, &points, &target)
                .unwrap()
                .artifact
        );
    }
    #[test]
    fn m3_byte_admission_exact_and_one_over_event_and_aggregate() {
        let (cfg, _, context) = fixture_at(false, 61);
        let limits = Limits::derive(&cfg, &context.launch).unwrap();
        let dir = std::env::temp_dir().join(format!(
            "m3-byte-admission-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&dir).unwrap();
        for extra in [0, 1] {
            for path in [EVENTS, INPUT, RESULT] {
                if dir.join(path).exists() {
                    fs::remove_file(dir.join(path)).unwrap();
                }
            }
            let len = file_cap(EVENTS, Some(&limits)) + extra;
            let a = super::super::super::storage::write_bytes(
                &dir,
                EVENTS,
                EVENTS_ROLE,
                "jsonl",
                &vec![b' '; len],
            )
            .unwrap();
            assert_eq!(verify_bytes(&dir, &[a], Some(&limits)).is_ok(), extra == 0);
            let input =
                super::super::super::storage::write_bytes(&dir, INPUT, INPUT_ROLE, "json", b"{}")
                    .unwrap();
            let result = super::super::super::storage::write_bytes(
                &dir,
                RESULT,
                RESULT_ROLE,
                "json",
                &vec![b' '; limits.encoded_bytes - 2 + extra],
            )
            .unwrap();
            assert_eq!(
                verify_bytes(&dir, &[input, result], Some(&limits)).is_ok(),
                extra == 0
            );
        }
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn m3_false_byte_stop_on_valid_deadline_prefix_is_rejected() {
        let (cfg, case, context) = fixture_at(false, 61);
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let target = group.mul(&points[0], 4);
        let input = prime_chain_input(3, &group, &points, &target).unwrap();
        let limits = Limits::derive(&cfg, &context.launch).unwrap();
        let mut checks = 0;
        let baseline = extract_with_clock(&input, &group, &points, &target, &limits, &mut || {
            checks += 1;
            checks >= 3
        })
        .unwrap();
        assert_eq!(baseline.stop, "deadline");
        assert_eq!(baseline.rows.len(), 1);
        verify_extraction(&input, &baseline, &group, &points, &target, &limits).unwrap();
        let mut forged = baseline.clone();
        forged.stop = "extraction_bytes".into();
        assert!(verify_extraction(&input, &forged, &group, &points, &target, &limits).is_err());
        let mut empty = baseline;
        empty.rows.clear();
        empty.group_adds = 0;
        empty.original_equation_evaluations = 0;
        empty.stop = "complete".into();
        let mut tight = limits.clone();
        tight.encoded_bytes = (encoded(&empty, limits.encoded_bytes).unwrap().len() + 4096) * 8;
        let actual = extract(&input, &group, &points, &target, &tight, None).unwrap();
        assert_eq!(actual.stop, "extraction_bytes");
        assert_eq!(actual.rows.len(), 1);
        verify_extraction(&input, &actual, &group, &points, &target, &tight).unwrap();
    }
    #[test]
    fn m3_partial_certificate_deadline_requires_full_three_add_reservation() {
        let (cfg, case, context) = fixture_at(false, 61);
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let target = group.mul(&points[0], 3);
        let input = prime_chain_input(3, &group, &points, &target).unwrap();
        let mut limits = Limits::derive(&cfg, &context.launch).unwrap();
        limits.group_adds = 4;
        let baseline = extract(&input, &group, &points, &target, &limits, None).unwrap();
        assert_eq!(baseline.stop, "witness_replay_budget");
        assert_eq!(baseline.group_adds, 2);
        assert_eq!(baseline.rows[0].matches_target, Some(true));
        verify_extraction(&input, &baseline, &group, &points, &target, &limits).unwrap();
        for replay_adds in [0, 1, 2] {
            let mut forged = baseline.clone();
            forged.stop = "witness_replay_deadline".into();
            forged.witness_replay_adds = replay_adds;
            forged.group_adds = 2 + replay_adds;
            assert!(verify_extraction(&input, &forged, &group, &points, &target, &limits).is_err());
        }
    }
    #[test]
    fn m3_bound_capture_persists_first_certificate_before_later_callback_failure() {
        struct Sink {
            emitter: CaptureEmitter,
            path: PathBuf,
            fail_later: bool,
            witness_calls: usize,
            event_calls: usize,
        }
        impl ExtractionPersistence for Sink {
            fn witness(&mut self, payload: &ArtifactPayload) -> Result<(), String> {
                self.witness_calls += 1;
                self.emitter.payload(payload).map_err(|e| e.to_string())
            }
            fn event(&mut self, event: &Value) -> Result<(), String> {
                self.event_calls += 1;
                if self.fail_later
                    && event["kind"] == "wide_prime_f4_m3_group_triple"
                    && event["cursor"] == 1
                {
                    assert!(self.path.join("payloads/pdp-witness.json").exists());
                    return Err("controlled later triple callback failure".into());
                }
                self.emitter.event(event).map_err(|e| e.to_string())
            }
        }
        let (cfg, case, context) = fixture_at(false, 61);
        let group = Group::read(&case).unwrap();
        let points = domain(&group, &case).unwrap();
        let target = group.mul(&points[0], 3);
        let input = prime_chain_input(3, &group, &points, &target).unwrap();
        let limits = Limits::derive(&cfg, &context.launch).unwrap();
        for refuse_first in [false, true] {
            let path = std::env::temp_dir().join(format!(
                "m3-first-certificate-{}-{}-{refuse_first}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            let mut capture = super::super::super::capture::capture_limits(64).unwrap();
            if refuse_first {
                capture.max_payload_bytes = 0;
            }
            let emitter =
                CaptureEmitter::create(&path, context.launch.binding.clone(), capture).unwrap();
            let mut sink = Sink {
                emitter,
                path: path.clone(),
                fail_later: !refuse_first,
                witness_calls: 0,
                event_calls: 0,
            };
            let output = extract_with_persistence(
                &input,
                &group,
                &points,
                &target,
                &limits,
                &mut || false,
                &mut sink,
            )
            .unwrap();
            verify_extraction(&input, &output, &group, &points, &target, &limits).unwrap();
            assert_eq!(output.first_verified_cursor, Some(0));
            assert_eq!(output.stop, "observer_failure");
            assert!(output.persistence_error.is_some());
            assert!(!output.domain_complete);
            assert_eq!(sink.witness_calls, 1);
            let canonical = witness_payload(&points, &target, output.rows[0].indices).unwrap();
            if refuse_first {
                assert_eq!(output.rows.len(), 1);
                assert_eq!(sink.event_calls, 0);
                assert!(!path.join("payloads/pdp-witness.json").exists());
                assert!(sink.emitter.payload(&canonical).is_err());
            } else {
                assert_eq!(output.rows.len(), 2);
                assert_eq!(sink.event_calls, 2);
                assert_eq!(
                    fs::read(path.join("payloads/pdp-witness.json")).unwrap(),
                    canonical.bytes
                );
                let before = fs::read(path.join("events.jsonl")).unwrap();
                sink.emitter.payload(&canonical).unwrap();
                assert_eq!(
                    fs::read(path.join("events.jsonl")).unwrap(),
                    before,
                    "identical canonical staging must not double-charge live capture"
                );
            }
            drop(sink);
            fs::remove_dir_all(path).unwrap();
        }
    }
}
