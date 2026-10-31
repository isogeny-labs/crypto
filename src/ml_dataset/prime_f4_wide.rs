//! Bounded wide-prime m=2 F4, with original-domain replay distinct from elimination.
use super::super::execution::{LaunchContext, SolverExecutionContext};
use super::*;
use crate::cryptanalysis::f4_fp_tower::{free_of, mask_of, mono, TowerF4Options};
use crate::cryptanalysis::f4_tower_generic::{
    f4_tower_generic_bounded, BoundedF4Limits, BoundedF4Stop, GPoly, GTowerRing,
};
use crate::cryptanalysis::fp_backend::{Fp64, PrimeField};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

pub(super) const STRATEGY: &str = "prime-f4-fp64-finite-domain-m2/v1";
const INPUT: &str = "prime-f4-input.json";
const RESULT: &str = "prime-f4-wide-result.json";
const EVENTS: &str = "prime-f4-wide-events.jsonl";
const EXTRACT: &str = "prime-f4-root-extraction.json";
const INPUT_ROLE: &str = "complete-prime-field-polynomial-system";
const RESULT_ROLE: &str = "restricted-wide-prime-f4-result/v1";
const EVENTS_ROLE: &str = "restricted-wide-prime-f4-boundary-events/v1";
const EXTRACT_ROLE: &str = "restricted-wide-prime-f4-original-domain-replay/v1";
const RESERVE: usize = 64 * 1024;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Term {
    exponents: [u8; 2],
    coefficient: String,
}
type Poly = Vec<Term>;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Input {
    schema: String,
    strategy: String,
    config: CaseConfig,
    modulus: String,
    a: String,
    b: String,
    target_x: String,
    factor_base_x: Vec<String>,
    polynomials: Vec<Poly>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct Limits {
    pub(super) work: u64,
    pub(super) bytes: u64,
    pub(super) rows: usize,
    pub(super) columns: usize,
    pub(super) basis: usize,
    pub(super) pairs: usize,
    pub(super) steps: usize,
    pub(super) events: usize,
    pub(super) encoded_bytes: usize,
    pub(super) domain_tests: u64,
    pub(super) group_adds: u64,
    pub(super) degree: u32,
}
impl Limits {
    pub(super) fn derive(config: &CaseConfig, launch: &LaunchContext) -> Result<Self, String> {
        let hard = BoundedF4Limits::default();
        let work = config
            .work_limit
            .checked_mul(1024)
            .ok_or("wide work limit overflow")?
            .min(hard.work);
        let bytes = (launch.memory_bytes / 4).min(hard.bytes);
        // Canonical and live copies share the outer allowance. Leave room for
        // prior worker artifacts, capture metadata and mandatory terminal state.
        let encoded_bytes = usize::try_from((launch.trace_bytes / 32).min(4 * 1024 * 1024))
            .map_err(|_| "wide trace limit overflow")?;
        if work == 0 || bytes == 0 || encoded_bytes <= 4 * RESERVE {
            return Err(
                "wide profile requires positive work/memory and terminal trace reserve".into(),
            );
        }
        Ok(Self {
            work,
            bytes,
            rows: hard.rows,
            columns: hard.columns,
            basis: hard.basis,
            pairs: hard.pairs,
            steps: hard.steps,
            events: hard.events,
            encoded_bytes,
            domain_tests: config.work_limit.min(1024),
            group_adds: config.work_limit,
            degree: config.degree_bound.min(12),
        })
    }
    pub(super) fn native(&self) -> BoundedF4Limits {
        BoundedF4Limits {
            work: self.work,
            bytes: self.bytes,
            rows: self.rows,
            columns: self.columns,
            basis: self.basis,
            pairs: self.pairs,
            steps: self.steps,
            events: self.events,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct Event {
    pub(super) boundary: String,
    pub(super) sequence: usize,
    pub(super) work_reserved: u64,
    pub(super) bytes_reserved: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct LiftCheck {
    indices: [usize; 2],
    matches_target: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct PairCheck {
    x: [String; 2],
    original_system_root: bool,
    lifts: Vec<LiftCheck>,
    lifts_complete: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Extraction {
    schema: String,
    pairs: Vec<PairCheck>,
    domain_complete: bool,
    group_adds: u64,
    witness_replay_adds: u64,
    stop: String,
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct NativeStep {
    pub(super) degree: u32,
    pub(super) critical_pairs: usize,
    pub(super) tower_pairs: usize,
    pub(super) s_rows: usize,
    pub(super) reducer_rows: usize,
    pub(super) promoted_rows: usize,
    pub(super) cols: usize,
    pub(super) nnz: u64,
    pub(super) residual_rows: usize,
    pub(super) residual_cols: usize,
    pub(super) full_rank_after: Option<usize>,
    pub(super) skipped_rows: usize,
    pub(super) dense_entries: u64,
    pub(super) fresh: usize,
    pub(super) fresh_min_degree: u32,
    pub(super) muladds: u64,
    pub(super) basis_active: usize,
    pub(super) basis_kept: usize,
    pub(super) basis_entries: u64,
    pub(super) pairs_left: usize,
    pub(super) ms: f64,
    pub(super) ms_rows: f64,
    pub(super) ms_reduce: f64,
    pub(super) ms_echelon: f64,
    pub(super) ms_update: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct NativeStatistics {
    pub(super) inconsistent: bool,
    pub(super) degree_reached: u32,
    pub(super) solving_degree_max: u32,
    pub(super) last_productive_degree: u32,
    pub(super) max_cols_to_solution: usize,
    pub(super) steps_to_solution: usize,
    pub(super) steps: usize,
    pub(super) max_rows: usize,
    pub(super) max_cols: usize,
    pub(super) max_nnz: u64,
    pub(super) max_residual_rows: usize,
    pub(super) rows_skipped: u64,
    pub(super) max_dense_entries: u64,
    pub(super) muladds: u64,
    pub(super) critical_pairs_reduced: u64,
    pub(super) tower_pairs_reduced: u64,
    pub(super) pairs_product_skipped: u64,
    pub(super) pairs_chain_skipped: u64,
    pub(super) reducer_rows: u64,
    pub(super) pairs_above_bound: usize,
    pub(super) staircase_at_stop: Option<usize>,
    pub(super) timed_out: bool,
    pub(super) oversize: bool,
    pub(super) ms: f64,
    pub(super) trace: Vec<NativeStep>,
    pub(super) stopped_step: Option<NativeStep>,
}

pub(super) fn number(s: &str) -> Result<u64, String> {
    let n = s.parse::<u64>().map_err(|_| "invalid canonical u64")?;
    if n.to_string() != s {
        return Err("noncanonical integer".into());
    }
    Ok(n)
}
fn canonical(terms: Vec<([u8; 2], BigUint)>, p: &BigUint) -> Poly {
    let mut map = std::collections::BTreeMap::<[u8; 2], BigUint>::new();
    for (e, c) in terms {
        *map.entry(e).or_default() += c;
    }
    map.into_iter()
        .filter_map(|(exponents, c)| {
            let c = c % p;
            (!c.is_zero()).then(|| Term {
                exponents,
                coefficient: c.to_string(),
            })
        })
        .collect()
}
fn input(
    config: &CaseConfig,
    group: &Group,
    domain: &[Element],
    target: &Element,
) -> Result<Input, String> {
    let Group::Prime(c) = group else {
        return Err("wide F4 requires prime curve".into());
    };
    let p = c.p.clone();
    let a = c.a.clone();
    let b = c.b.clone();
    let r = target.x().ok_or("wide F4 requires affine target")?;
    let mut xs = Vec::new();
    for point in domain {
        let x = point.x().ok_or("wide F4 domain contains infinity")?;
        if x >= p {
            return Err("noncanonical factor-base coordinate".into());
        }
        xs.push(x);
    }
    xs.sort();
    xs.dedup();
    if xs.is_empty() || xs.len() > 32 {
        return Err("wide F4 profile requires 1..32 distinct domain x values".into());
    }
    let neg = |x: BigUint| (&p - x % &p) % &p;
    let r2 = &r * &r % &p;
    let linear = neg(BigUint::from(2u8) * &a * &r + BigUint::from(4u8) * &b);
    let s3 = canonical(
        vec![
            ([2, 2], BigUint::from(1u8)),
            ([2, 1], neg(BigUint::from(2u8) * &r)),
            ([1, 2], neg(BigUint::from(2u8) * &r)),
            ([2, 0], r2.clone()),
            ([0, 2], r2.clone()),
            ([1, 1], neg(BigUint::from(2u8) * (&r2 + &a))),
            ([1, 0], linear.clone()),
            ([0, 1], linear),
            ([0, 0], &a * &a + neg(BigUint::from(4u8) * &b * &r)),
        ],
        &p,
    );
    let mut member = vec![BigUint::from(1u8)];
    for x in &xs {
        let mut next = vec![BigUint::zero(); member.len() + 1];
        for (i, v) in member.iter().enumerate() {
            next[i] = (&next[i] + neg(v * x)) % &p;
            next[i + 1] = (&next[i + 1] + v) % &p;
        }
        member = next;
    }
    let mut polynomials = vec![s3];
    for axis in 0..2 {
        polynomials.push(canonical(
            member
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let mut e = [0, 0];
                    e[axis] = i as u8;
                    (e, c.clone())
                })
                .collect(),
            &p,
        ));
    }
    Ok(Input {
        schema: "crypto.ml-dataset.wide-prime-f4-input/v1".into(),
        strategy: STRATEGY.into(),
        config: config.clone(),
        modulus: p.to_string(),
        a: a.to_string(),
        b: b.to_string(),
        target_x: r.to_string(),
        factor_base_x: xs.iter().map(ToString::to_string).collect(),
        polynomials,
    })
}
fn eval(poly: &Poly, x: &[u64; 2], p: u64) -> Result<u64, String> {
    let modulus = BigUint::from(p);
    let mut sum = BigUint::zero();
    for term in poly {
        let c = number(&term.coefficient)?;
        if c >= p {
            return Err("coefficient outside field".into());
        }
        sum = (sum
            + BigUint::from(c)
                * BigUint::from(x[0]).modpow(&BigUint::from(term.exponents[0]), &modulus)
                * BigUint::from(x[1]).modpow(&BigUint::from(term.exponents[1]), &modulus))
            % &modulus;
    }
    sum.to_u64().ok_or_else(|| "evaluation exceeds u64".into())
}
pub(super) fn encoded<T: Serialize>(value: &T, cap: usize) -> Result<Vec<u8>, String> {
    let mut writer = LimitedJsonWriter {
        bytes: vec![],
        limit: cap,
    };
    serde_json::to_writer(&mut writer, value).map_err(|e| e.to_string())?;
    Ok(writer.bytes)
}
pub(super) fn push<T: Serialize>(
    data: &mut SolverData,
    path: &str,
    role: &str,
    value: &T,
    cap: usize,
) -> Result<(), String> {
    let bytes = encoded(value, cap)?;
    data.artifacts.push(ArtifactPayload {
        path: path.into(),
        role: role.into(),
        encoding: "json".into(),
        bytes,
    });
    Ok(())
}
pub(super) fn stop_name(stop: &BoundedF4Stop) -> &'static str {
    match stop {
        BoundedF4Stop::Complete => "complete",
        BoundedF4Stop::UnitDerived => "unit_derived",
        BoundedF4Stop::DegreeLimit => "degree_limit",
        BoundedF4Stop::WorkLimit => "work_limit",
        BoundedF4Stop::StorageAdmission => "storage_admission",
        BoundedF4Stop::Deadline => "deadline",
        BoundedF4Stop::ObserverFailure(_) => "observer_failure",
        BoundedF4Stop::InvalidInput => "invalid_input",
    }
}
fn base_result() -> ResultRecord {
    ResultRecord {
        schema: "crypto.ml-dataset.wide-prime-f4-result/v1".into(),
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
    let original = input(config, group, domain, target)?;
    let mut data = SolverData {
        outcome: outcome(
            "partial",
            "Bounded wide-prime F4 and independently checked original-domain extraction",
        ),
        artifacts: vec![],
    };
    data.outcome["strategy"] = json!(STRATEGY);
    data.outcome["trace_gaps"]=json!(["Native boundaries and final basis do not expose complete matrices, pivots or a derivation certificate","Reserved work and storage are conservative admission quantities, not executed instruction counts or RSS","No first or last fall degree theorem or independent scalar recovery"]);
    push(&mut data, INPUT, INPUT_ROLE, &original, RESERVE)?;
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
    let p = number(&original.modulus)?;
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
        n_free: 2,
    };
    let native_input: Vec<_> = original
        .polynomials
        .iter()
        .map(|poly| {
            Ok(GPoly::from_terms(
                &ring.field,
                poly.iter()
                    .map(|t| {
                        Ok((
                            mono(0, [t.exponents[0], t.exponents[1], 0, 0]),
                            ring.field.from_u64(number(&t.coefficient)?),
                        ))
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
                live_event(&json!({"kind":"wide_prime_f4_boundary","record":record}))?;
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
                exponents: [e[0], e[1]],
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
    let mut extraction = Extraction {
        schema: "crypto.ml-dataset.wide-prime-f4-extraction/v1".into(),
        pairs: vec![],
        domain_complete: false,
        group_adds: 0,
        witness_replay_adds: 0,
        stop: "complete".into(),
    };
    let mut found = None;
    if data.outcome.get("live_persistence_errors").is_none() {
        let xs = original
            .factor_base_x
            .iter()
            .map(|s| number(s))
            .collect::<Result<Vec<_>, _>>()?;
        'pairs: for &x in &xs {
            for &y in &xs {
                if extraction.pairs.len() as u64 >= limits.domain_tests {
                    extraction.stop = "domain_budget".into();
                    break 'pairs;
                }
                if Instant::now() >= context.deadline {
                    extraction.stop = "deadline".into();
                    break 'pairs;
                }
                let used = encoded(&extraction, limits.encoded_bytes / 8)?.len();
                if (limits.encoded_bytes / 8).saturating_sub(used) < 2048 {
                    extraction.stop = "extraction_bytes".into();
                    break 'pairs;
                }
                let root = original
                    .polynomials
                    .iter()
                    .map(|f| eval(f, &[x, y], p))
                    .collect::<Result<Vec<_>, _>>()?
                    .iter()
                    .all(|v| *v == 0);
                let mut check = PairCheck {
                    x: [x.to_string(), y.to_string()],
                    original_system_root: root,
                    lifts: vec![],
                    lifts_complete: true,
                };
                if root {
                    let choices: Vec<Vec<usize>> = [x, y]
                        .iter()
                        .map(|x| {
                            domain
                                .iter()
                                .enumerate()
                                .filter(|(_, point)| point.x() == Some(BigUint::from(*x)))
                                .map(|(i, _)| i)
                                .collect()
                        })
                        .collect();
                    'lifts: for &i in &choices[0] {
                        for &j in &choices[1] {
                            if extraction.group_adds >= limits.group_adds
                                || Instant::now() >= context.deadline
                            {
                                check.lifts_complete = false;
                                extraction.stop = "lift_budget_or_deadline".into();
                                break 'lifts;
                            }
                            let sum = group.add(&domain[i], &domain[j]);
                            extraction.group_adds += 1;
                            let matched = &sum == target;
                            check.lifts.push(LiftCheck {
                                indices: [i, j],
                                matches_target: matched,
                            });
                            if matched {
                                if limits.group_adds.saturating_sub(extraction.group_adds) < 2 {
                                    check.lifts_complete = false;
                                    extraction.stop = "witness_replay_budget".into();
                                    break 'lifts;
                                }
                                let replay =
                                    group.add(&group.add(&group.zero(), &domain[i]), &domain[j]);
                                extraction.group_adds += 2;
                                extraction.witness_replay_adds = 2;
                                if &replay != target {
                                    return Err("wide PDP witness replay failed".into());
                                }
                                found = Some([i, j]);
                                check.lifts_complete = false;
                                extraction.stop = "witness_found".into();
                                break 'lifts;
                            }
                        }
                    }
                }
                // Admit each complete record before retaining it; no growing trace
                // beyond encoded allocation allowance.
                if encoded(&check, 1984).is_err() {
                    return Err("wide extraction record exceeded pre-admitted size".into());
                }
                extraction.pairs.push(check);
                if extraction.stop != "complete" {
                    break 'pairs;
                }
            }
        }
        extraction.domain_complete =
            extraction.stop == "complete" && extraction.pairs.len() == xs.len() * xs.len();
        result.extraction_complete = extraction.domain_complete;
        result.exhaustive_no_decomposition = extraction.domain_complete && found.is_none();
        if let Some([i, j]) = found {
            result.pdp_witness_verified = true;
            push(
                &mut data,
                "pdp-witness.json",
                "independently-verified-pdp",
                &json!({"summands":[domain[i].json(),domain[j].json()],"target":target.json(),"sum":target.json(),"replay_group_add_calls":2}),
                RESERVE,
            )?;
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

pub(super) fn artifact<'a>(
    artifacts: &'a [super::super::types::Artifact],
    path: &str,
    role: &str,
    encoding: &str,
    required: bool,
) -> Result<Option<&'a super::super::types::Artifact>, String> {
    let matches: Vec<_> = artifacts
        .iter()
        .filter(|a| a.path == path || a.role == role)
        .collect();
    if matches.is_empty() && !required {
        return Ok(None);
    }
    if matches.len() != 1
        || matches[0].path != path
        || matches[0].role != role
        || matches[0].encoding != encoding
    {
        return Err(format!("wide F4 reserved artifact mismatch: {path}"));
    }
    Ok(Some(matches[0]))
}
fn admitted_file_cap(path: &str, limits: Option<&Limits>) -> usize {
    match path {
        INPUT | "pdp-witness.json" => RESERVE,
        RESULT => limits.map_or(RESERVE, |l| l.encoded_bytes),
        EVENTS => limits.map_or(0, |l| (l.encoded_bytes / 8).min(256 * 1024)),
        EXTRACT => limits.map_or(0, |l| l.encoded_bytes / 8 + RESERVE),
        _ => 0,
    }
}
fn verify_byte_admission(
    dir: &Path,
    artifacts: &[super::super::types::Artifact],
    limits: Option<&Limits>,
) -> Result<(), String> {
    let mut total = 0u64;
    for a in artifacts
        .iter()
        .filter(|a| [INPUT, RESULT, EVENTS, EXTRACT, "pdp-witness.json"].contains(&a.path.as_str()))
    {
        let cap = admitted_file_cap(&a.path, limits) as u64;
        let metadata = fs::symlink_metadata(dir.join(&a.path)).map_err(|e| e.to_string())?;
        if !metadata.is_file()
            || metadata.file_type().is_symlink()
            || metadata.len() != a.bytes
            || a.bytes > cap
        {
            return Err(format!(
                "wide artifact exceeds admitted serialized bytes: {}",
                a.path
            ));
        }
        total = total.checked_add(a.bytes).ok_or("wide byte sum overflow")?;
    }
    if total > limits.map_or(2 * RESERVE, |l| l.encoded_bytes) as u64 {
        return Err("wide aggregate serialized byte allowance exceeded".into());
    }
    Ok(())
}
pub(super) fn read_bounded(dir: &Path, path: &str, cap: usize) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let path = dir.join(path);
    let m = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
    if !m.is_file() || m.file_type().is_symlink() || m.len() > cap as u64 {
        return Err("wide file is not regular/admitted".into());
    }
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(|e| e.to_string())?
        .take(cap as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > cap {
        return Err("wide file grew past serialized cap".into());
    }
    Ok(bytes)
}
fn read_typed<T: for<'de> Deserialize<'de>>(
    dir: &Path,
    path: &str,
    limits: Option<&Limits>,
) -> Result<T, String> {
    serde_json::from_slice(&read_bounded(dir, path, admitted_file_cap(path, limits))?)
        .map_err(|e| e.to_string())
}
pub(super) fn verify_censored_outcome(outcome: &Value) -> Result<(), String> {
    if outcome["solver_completed"] != false
        || [
            "pdp_witness_verified",
            "exhaustive_no_decomposition",
            "native_backend_invoked",
            "algebra_completed",
            "native_statistics_complete",
            "extraction_complete",
        ]
        .iter()
        .any(|key| !outcome[*key].is_null() && outcome[*key] != false)
    {
        return Err("censored supervisor promoted wide F4 completion".into());
    }
    Ok(())
}
pub(super) fn verify_unproved_claims(outcome: &Value) -> Result<(), String> {
    for key in [
        "trace_complete",
        "execution_trace_complete_verified",
        "total_operation_count_verified",
        "native_basis_derivation_verified",
        "scalar_recovered_without_witness",
        "scalar_replay_verified",
        "ecdlp_pipeline_completed",
    ] {
        if !outcome[key].is_null() && outcome[key] != false {
            return Err(format!("wide F4 cannot certify {key}"));
        }
    }
    for key in [
        "first_fall_degree",
        "last_fall_degree",
        "retired_hardware_instructions",
    ] {
        if !outcome[key].is_null() {
            return Err(format!("wide F4 has no verified {key}"));
        }
    }
    Ok(())
}
/// Checks canonical mathematical claims, not native basis derivation or runtime timing.
pub(super) fn verify(
    dir: &Path,
    config: &CaseConfig,
    outcome: &Value,
    artifacts: &[super::super::types::Artifact],
) -> Result<(), String> {
    if config.family == "prime" && config.solver == "f4" && config.m == 2 && config.n >= 33
        || outcome["strategy"] == STRATEGY
        || artifacts
            .iter()
            .any(|a| a.path == RESULT || a.role == RESULT_ROLE)
    {
        verify_unproved_claims(outcome)?;
    }
    let supervisor_censored = matches!(
        outcome["status"].as_str(),
        Some("timeout" | "trace_budget_exhausted" | "worker_failed")
    );
    // A killed worker may leave incomplete canonical files before publishing
    // its registry. Keep those hash-checked bytes as unregistered partial data,
    // never promote their mathematical or producer completion labels.
    let partial_paths = [INPUT, RESULT, EVENTS, EXTRACT];
    let partial: Vec<_> = artifacts
        .iter()
        .filter(|a| partial_paths.contains(&a.path.as_str()))
        .collect();
    if supervisor_censored
        && !partial.is_empty()
        && partial
            .iter()
            .all(|a| a.role == "unregistered-partial-artifact" && a.encoding == "native")
    {
        verify_censored_outcome(outcome)?;
        return Ok(());
    }
    let reserved = artifacts.iter().any(|a| {
        [RESULT, EVENTS].contains(&a.path.as_str())
            || [RESULT_ROLE, EVENTS_ROLE, EXTRACT_ROLE].contains(&a.role.as_str())
    });
    let context = super::super::execution::load(dir)?;
    let admitted_limits = context
        .as_ref()
        .and_then(|launch| Limits::derive(config, launch).ok());
    let bounded_profile = reserved
        || outcome["strategy"] == STRATEGY
        || context.is_some()
            && config.family == "prime"
            && config.solver == "f4"
            && config.m == 2
            && config.n >= 33;
    if bounded_profile {
        verify_byte_admission(dir, artifacts, admitted_limits.as_ref())?;
    }
    let input_value: Option<Value> = if artifacts.iter().any(|a| a.path == INPUT) {
        Some(if bounded_profile {
            read_typed(dir, INPUT, admitted_limits.as_ref())?
        } else {
            super::super::read_json(&dir.join(INPUT))?
        })
    } else {
        None
    };
    let declared = outcome["strategy"] == STRATEGY
        || input_value.as_ref().is_some_and(|v| {
            v["strategy"] == STRATEGY || v["schema"] == "crypto.ml-dataset.wide-prime-f4-input/v1"
        });
    let public: Option<Value> = if dir.join("public.json").exists() {
        Some(super::super::read_json(&dir.join("public.json"))?)
    } else {
        None
    };
    let wide_config = config.family == "prime"
        && config.solver == "f4"
        && config.m == 2
        && (config.n >= 33
            || public
                .as_ref()
                .and_then(|v| v["field"]["modulus"].as_str())
                .and_then(|p| p.parse::<u64>().ok())
                .is_some_and(|p| p >= 1u64 << 32));
    let claimed = [
        "native_backend_invoked",
        "native_statistics_complete",
        "algebra_completed",
        "extraction_complete",
        "exhaustive_no_decomposition",
        "solver_completed",
        "pdp_witness_verified",
    ]
    .iter()
    .any(|key| outcome[*key] == true);
    if wide_config
        && context.is_some()
        && outcome["status"] == "error"
        && !reserved
        && !declared
        && !claimed
        && partial.is_empty()
    {
        // Ordinary workers return exit0 with a bound error result after setup
        // failure. This exception cannot hide registered or unlisted wide data.
        if [
            INPUT,
            RESULT,
            EVENTS,
            EXTRACT,
            "pdp-witness.json",
            "solver-domain.json",
        ]
        .iter()
        .any(|p| fs::symlink_metadata(dir.join(p)).is_ok())
        {
            return Err("pre-solver error retains unlisted wide artifacts".into());
        }
        verify_censored_outcome(outcome)?;
        artifact(artifacts, "result.json", "solver-outcome", "json", true)?;
        let retained: Value = serde_json::from_slice(&read_bounded(dir, "result.json", RESERVE)?)
            .map_err(|e| e.to_string())?;
        verify_unproved_claims(&retained)?;
        verify_censored_outcome(&retained)?;
        let mut claimed_outcome = outcome.clone();
        claimed_outcome
            .as_object_mut()
            .ok_or("worker outcome is not an object")?
            .remove("supervisor");
        if retained != claimed_outcome
            || retained["status"] != "error"
            || retained["reason"].as_str().is_none()
        {
            return Err("pre-solver error differs from retained worker result".into());
        }
        return Ok(());
    }
    let expected = wide_config && (context.is_some() || claimed);
    if !reserved
        && !declared
        && (!expected || supervisor_censored && !claimed && partial.is_empty())
    {
        return Ok(());
    }
    if config.family != "prime" || config.solver != "f4" || config.m != 2 {
        return Err("wide F4 strategy/config mismatch".into());
    }
    artifact(artifacts, INPUT, INPUT_ROLE, "json", true)?;
    artifact(artifacts, RESULT, RESULT_ROLE, "json", true)?;
    let original: Input = serde_json::from_value(input_value.ok_or("wide F4 lacks equations")?)
        .map_err(|e| e.to_string())?;
    let case = CaseData {
        public_instance: public.ok_or("wide F4 lacks public instance")?,
        private_witness: Value::Null,
        factor_base: super::super::read_json(&dir.join("factor-base.json"))?,
    };
    artifact(artifacts, "public.json", "public-instance", "json", true)?;
    artifact(artifacts, "factor-base.json", "factor-base", "json", true)?;
    if case.public_instance["family"] != config.family
        || case.public_instance["n"] != json!(config.n)
        || case.public_instance["m"] != json!(config.m)
        || case.public_instance["twist"] != json!(config.twist)
    {
        return Err("wide public instance differs from frozen config".into());
    }
    curves::verify_public(&case.public_instance)?;
    curves::verify_factor_base_config(config, &case.public_instance, &case.factor_base)?;
    let group = Group::read(&case)?;
    let target = group.point(&case.public_instance["target"])?;
    let domain = domain(&group, &case)?;
    if original != input(config, &group, &domain, &target)? {
        return Err("wide F4 original equations/config/domain mismatch".into());
    }
    verify_byte_admission(dir, artifacts, admitted_limits.as_ref())?;
    let result: ResultRecord = read_typed(dir, RESULT, admitted_limits.as_ref())?;
    if result.schema != "crypto.ml-dataset.wide-prime-f4-result/v1"
        || result.strategy != STRATEGY
        || result.native_basis_derivation_verified
        || result.execution_trace_complete_verified
        || result.total_operation_count_verified
    {
        return Err("wide F4 unverified derivation/operation claim".into());
    }
    match (&result.launch, &result.limits) {
        (Some(launch), Some(limits)) => {
            if context.as_ref() != Some(launch) || *limits != Limits::derive(config, launch)? {
                return Err("wide F4 admitted limits/launch mismatch".into());
            }
        }
        (Some(launch), None) => {
            if context.as_ref() != Some(launch)
                || result.stop != "profile_admission"
                || Limits::derive(config, launch).is_ok()
            {
                return Err("wide F4 unexplained admission refusal".into());
            }
        }
        (None, None) => {
            if result.stop != "missing_execution_context" || context.is_some() {
                return Err("wide F4 missing context is unexplained".into());
            }
        }
        _ => return Err("wide F4 incomplete admission context".into()),
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
    let p = number(&original.modulus)?;
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
        let bytes = read_bounded(
            dir,
            EVENTS,
            admitted_file_cap(EVENTS, admitted_limits.as_ref()),
        )?;
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
    if complete && events.last().is_none_or(|e| e.boundary != "terminal") {
        return Err("complete wide F4 lacks terminal acknowledgement".into());
    }
    let extraction_descriptor = artifact(
        artifacts,
        EXTRACT,
        EXTRACT_ROLE,
        "json",
        result.extraction_retained,
    )?;
    if extraction_descriptor.is_some() != result.extraction_retained {
        return Err("wide F4 undeclared extraction artifact".into());
    }
    if result.extraction_retained {
        let extraction: Extraction = read_typed(dir, EXTRACT, admitted_limits.as_ref())?;
        let limits = result.limits.as_ref().ok_or("extraction without limits")?;
        verify_extraction(
            &original,
            &extraction,
            &group,
            &domain,
            &target,
            limits,
            &result,
        )?;
        if native_claim_contradicts_domain(&result, &extraction) {
            return Err("native refutation contradicts original-domain root".into());
        }
        if result.pdp_witness_verified {
            let pair = extraction
                .pairs
                .iter()
                .flat_map(|p| p.lifts.iter())
                .find(|l| l.matches_target)
                .ok_or("wide witness lacks matching extraction")?;
            let witness: Value = super::super::read_json(&dir.join("pdp-witness.json"))?;
            if witness["summands"]
                != json!([
                    domain[pair.indices[0]].json(),
                    domain[pair.indices[1]].json()
                ])
            {
                return Err("wide witness differs from checked extraction".into());
            }
        }
    } else if result.extraction_complete
        || result.exhaustive_no_decomposition
        || result.pdp_witness_verified
    {
        return Err("wide F4 extraction labels lack evidence".into());
    }
    let censored = matches!(
        outcome["status"].as_str(),
        Some("timeout" | "trace_budget_exhausted" | "worker_failed")
    );
    let retained_outcome: Value;
    let outcome = if censored {
        verify_censored_outcome(outcome)?;
        artifact(artifacts, "result.json", "solver-outcome", "json", true)?;
        retained_outcome = serde_json::from_slice(&read_bounded(dir, "result.json", RESERVE)?)
            .map_err(|e| e.to_string())?;
        verify_unproved_claims(&retained_outcome)?;
        &retained_outcome
    } else {
        outcome
    };
    for (key, value) in [
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
        ("native_basis_derivation_verified", false),
        ("execution_trace_complete_verified", false),
        ("total_operation_count_verified", false),
    ] {
        if outcome[key] != json!(value) {
            return Err(format!("wide F4 outcome mismatch: {key}"));
        }
    }
    if outcome["counters"]
        != json!({"native_work_reserved":result.work_reserved,"native_cumulative_bytes_reserved":result.bytes_reserved,"acknowledged_native_boundaries":result.acknowledged_events})
    {
        return Err("wide F4 outcome counters mismatch".into());
    }
    let persistence_failed = outcome.get("live_persistence_errors").is_some();
    let expected_status = if result.pdp_witness_verified && !persistence_failed {
        "verified"
    } else {
        "partial"
    };
    if outcome["status"] != expected_status {
        return Err("wide worker status differs from verified witness/persistence state".into());
    }

    if outcome["termination"] != result.stop
        || outcome["solver_completed"]
            != json!(
                !persistence_failed
                    && (result.pdp_witness_verified || result.exhaustive_no_decomposition)
            )
    {
        return Err("wide F4 terminal outcome mismatch".into());
    }
    if result.pdp_witness_verified {
        artifact(
            artifacts,
            "pdp-witness.json",
            "independently-verified-pdp",
            "json",
            true,
        )?;
        verify_solution_artifact(
            &case.public_instance,
            &case.factor_base,
            "independently-verified-pdp",
            &super::super::read_json(&dir.join("pdp-witness.json"))?,
        )?;
    }
    Ok(())
}
fn native_claim_contradicts_domain(result: &ResultRecord, extraction: &Extraction) -> bool {
    (result.stop == "unit_derived"
        || result
            .native_statistics
            .as_ref()
            .is_some_and(|s| s.inconsistent))
        && extraction.pairs.iter().any(|p| p.original_system_root)
}
fn verify_extraction(
    input: &Input,
    e: &Extraction,
    group: &Group,
    domain: &[Element],
    target: &Element,
    limits: &Limits,
    result: &ResultRecord,
) -> Result<(), String> {
    if e.schema != "crypto.ml-dataset.wide-prime-f4-extraction/v1"
        || e.pairs.len() as u64 > limits.domain_tests
        || e.group_adds > limits.group_adds
        || ![
            "complete",
            "domain_budget",
            "deadline",
            "lift_budget_or_deadline",
            "witness_replay_budget",
            "witness_found",
            "extraction_bytes",
        ]
        .contains(&e.stop.as_str())
    {
        return Err("invalid wide F4 extraction envelope".into());
    }
    let xs = input
        .factor_base_x
        .iter()
        .map(|s| number(s))
        .collect::<Result<Vec<_>, _>>()?;
    let p = number(&input.modulus)?;
    let mut adds = 0;
    let mut found = false;
    for (k, pair) in e.pairs.iter().enumerate() {
        if k >= xs.len() * xs.len()
            || pair.x != [xs[k / xs.len()].to_string(), xs[k % xs.len()].to_string()]
        {
            return Err("wide F4 extraction is not original ordered domain prefix".into());
        }
        let x = [number(&pair.x[0])?, number(&pair.x[1])?];
        let root = input
            .polynomials
            .iter()
            .map(|f| eval(f, &x, p))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .all(|v| *v == 0);
        if root != pair.original_system_root {
            return Err("wide F4 original equation evaluation mismatch".into());
        }
        let mut choices = vec![];
        if root {
            for (i, a) in domain.iter().enumerate() {
                if a.x() != Some(BigUint::from(x[0])) {
                    continue;
                }
                for (j, b) in domain.iter().enumerate() {
                    if b.x() == Some(BigUint::from(x[1])) {
                        choices.push([i, j]);
                    }
                }
            }
        }
        if pair.lifts.len() > choices.len()
            || pair.lifts_complete && pair.lifts.len() != choices.len()
            || !pair.lifts_complete && k + 1 != e.pairs.len()
        {
            return Err("wide F4 lift prefix completeness mismatch".into());
        }
        for (lift_index, (check, expected)) in pair.lifts.iter().zip(&choices).enumerate() {
            if check.indices != *expected {
                return Err("wide F4 lift domain mismatch".into());
            }
            let matches = group.add(&domain[expected[0]], &domain[expected[1]]) == *target;
            adds += 1;
            if matches != check.matches_target {
                return Err("wide F4 group lift replay mismatch".into());
            }
            if matches && (k + 1 != e.pairs.len() || lift_index + 1 != pair.lifts.len()) {
                return Err("wide extraction continued after first witness".into());
            }
            found |= matches;
        }
    }
    if e.witness_replay_adds != if result.pdp_witness_verified { 2 } else { 0 }
        || e.group_adds != adds + e.witness_replay_adds
        || result.pdp_witness_verified && !found
    {
        return Err("wide F4 extraction counters/witness mismatch".into());
    }
    let full =
        e.pairs.len() == xs.len() * xs.len() && e.pairs.iter().all(|p| p.lifts_complete) && !found;
    if e.domain_complete != (e.stop == "complete" && full)
        || result.extraction_complete != e.domain_complete
        || result.exhaustive_no_decomposition != e.domain_complete
    {
        return Err("wide F4 exhaustive domain claim not established".into());
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
        outcome: outcome(
            "partial",
            "Initial live stream failed before native entry; canonical equations retained",
        ),
        artifacts: vec![],
    };
    data.outcome["strategy"] = json!(STRATEGY);
    push(
        &mut data,
        INPUT,
        INPUT_ROLE,
        &input(config, group, domain, target)?,
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
    fn fixture(twist: bool) -> (CaseConfig, CaseData, SolverExecutionContext) {
        fixture_at(twist, 61)
    }
    fn fixture_at(twist: bool, n: u32) -> (CaseConfig, CaseData, SolverExecutionContext) {
        let cfg = CaseConfig {
            factor_base_policy: None,
            family: "prime".into(),
            solver: "f4".into(),
            n,
            m: 2,
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
        case.public_instance["target"] = group.add(first, first).json();
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
    fn wide_prime_fixed47_53_controls_invoke_backend_for_both_twists() {
        for n in [47, 53] {
            for twist in [false, true] {
                let (cfg, case, context) = fixture_at(twist, n);
                let data = super::super::solve_with_context(&cfg, &case, Some(&context)).unwrap();
                assert_eq!(
                    data.outcome["native_backend_invoked"], true,
                    "n={n},twist={twist}: {}",
                    data.outcome
                );
                assert_eq!(data.outcome["pdp_witness_verified"], true);
            }
        }
    }
    #[test]
    fn wide_prime_fixed_no_lift_completes_original_domain() {
        for twist in [false, true] {
            let (cfg, mut case, context) = fixture(twist);
            let group = Group::read(&case).unwrap();
            let points = domain(&group, &case).unwrap();
            // Fixed factor-base-size2 recipe retains exactly P,-P. Ordered sums
            // are 0,+2P,-2P; 3P is absent in the declared large prime subgroup.
            assert_eq!(points.len(), 2);
            assert_eq!(points[1], group.neg(&points[0]));
            assert!(group.order().unwrap() > 5);
            let target = group.mul(&points[0], 3);
            assert_ne!(target, group.zero());
            case.public_instance["target"] = target.json();
            let data = super::super::solve_with_context(&cfg, &case, Some(&context)).unwrap();
            assert_eq!(data.outcome["native_backend_invoked"], true);
            assert_eq!(data.outcome["extraction_complete"], true);
            assert_eq!(data.outcome["exhaustive_no_decomposition"], true);
            assert_eq!(data.outcome["pdp_witness_verified"], false);
            let original: Input = record(&data, INPUT);
            let extraction: Extraction = record(&data, EXTRACT);
            let result: ResultRecord = record(&data, RESULT);
            verify_extraction(
                &original,
                &extraction,
                &group,
                &points,
                &target,
                result.limits.as_ref().unwrap(),
                &result,
            )
            .unwrap();
        }
    }
    #[test]
    fn wide_prime_group_replay_budget_exact_boundary_and_one_less() {
        for twist in [false, true] {
            let (mut cfg, case, context) = fixture(twist);
            let group = Group::read(&case).unwrap();
            let points = domain(&group, &case).unwrap();
            let target = group.point(&case.public_instance["target"]).unwrap();
            for budget in [2, 3] {
                cfg.work_limit = budget;
                let data = solve(&cfg, &group, &points, &target, Some(&context)).unwrap();
                assert_eq!(data.outcome["pdp_witness_verified"], json!(budget == 3));
                assert_eq!(data.outcome["exhaustive_no_decomposition"], false);
                let result: ResultRecord = record(&data, RESULT);
                let extraction: Extraction = record(&data, EXTRACT);
                let original: Input = record(&data, INPUT);
                verify_extraction(
                    &original,
                    &extraction,
                    &group,
                    &points,
                    &target,
                    result.limits.as_ref().unwrap(),
                    &result,
                )
                .unwrap();
                assert!(extraction.group_adds <= budget);
                let mut forged = extraction.clone();
                forged.domain_complete = true;
                forged.stop = "complete".into();
                assert!(verify_extraction(
                    &original,
                    &forged,
                    &group,
                    &points,
                    &target,
                    result.limits.as_ref().unwrap(),
                    &result
                )
                .is_err());
            }
        }
    }
    #[test]
    fn wide_prime_expired_deadline_retains_equations_without_backend() {
        let (cfg, case, mut context) = fixture(false);
        context.deadline = Instant::now();
        let data = super::super::solve_with_context(&cfg, &case, Some(&context)).unwrap();
        let result: ResultRecord = record(&data, RESULT);
        assert_eq!(result.stop, "deadline");
        assert!(!result.native_backend_invoked);
        let original: Input = record(&data, INPUT);
        assert_eq!(original.modulus, "2305843009213693951");
        assert_eq!(data.outcome["solver_completed"], false);
    }
    #[test]
    fn wide_prime_payload_and_initial_stream_caps_preserve_canonical_input() {
        for begin in [false, true] {
            let (cfg, case, context) = fixture(false);
            let mut limits = super::super::super::capture::capture_limits(64).unwrap();
            if begin {
                limits.max_event_bytes = 512;
                limits.max_stream_bytes = 512;
            } else {
                limits.max_payload_bytes = 0;
            }
            let path = std::env::temp_dir().join(format!(
                "wide-f4-cap-{}-{}-{begin}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            let data = super::super::solve_with_bound_event_sink_context(
                &cfg,
                &case,
                &path,
                context.launch.binding.clone(),
                limits,
                Some(&context),
            )
            .unwrap();
            assert_eq!(data.outcome["native_backend_invoked"], false);
            assert_eq!(data.outcome["solver_completed"], false);
            assert_eq!(data.outcome["live_persistence_complete"], false);
            let original: Input = record(&data, INPUT);
            assert_eq!(original.modulus, "2305843009213693951");
            let result: ResultRecord = record(&data, RESULT);
            assert_eq!(result.stop, "observer_failure");
            assert!(LIVE_SINK.with(|slot| slot.borrow().is_none()));
            fs::remove_dir_all(path).unwrap();
        }
    }
    #[test]
    fn unit_stop_contradiction_does_not_depend_on_optional_statistics() {
        let mut result = base_result();
        result.stop = "unit_derived".into();
        assert!(result.native_statistics.is_none());
        let mut extraction = Extraction {
            schema: "crypto.ml-dataset.wide-prime-f4-extraction/v1".into(),
            pairs: vec![PairCheck {
                x: ["0".into(), "0".into()],
                original_system_root: true,
                lifts: vec![],
                lifts_complete: true,
            }],
            domain_complete: false,
            group_adds: 0,
            witness_replay_adds: 0,
            stop: "domain_budget".into(),
        };
        assert!(native_claim_contradicts_domain(&result, &extraction));
        extraction.pairs[0].original_system_root = false;
        assert!(!native_claim_contradicts_domain(&result, &extraction));
    }
    #[test]
    fn serialized_native_event_allowance_exact_and_one_over() {
        let (cfg, _, context) = fixture(false);
        let limits = Limits::derive(&cfg, &context.launch).unwrap();
        let cap = admitted_file_cap(EVENTS, Some(&limits));
        let dir = std::env::temp_dir().join(format!(
            "wide-byte-boundary-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&dir).unwrap();
        for extra in [0, 1] {
            fs::write(dir.join(EVENTS), vec![b' '; cap + extra]).unwrap();
            let a = super::super::super::types::Artifact {
                path: EVENTS.into(),
                role: EVENTS_ROLE.into(),
                encoding: "jsonl".into(),
                sha256: "0".repeat(64),
                bytes: (cap + extra) as u64,
            };
            assert_eq!(
                verify_byte_admission(&dir, &[a], Some(&limits)).is_ok(),
                extra == 0
            );
            assert_eq!(read_bounded(&dir, EVENTS, cap).is_ok(), extra == 0);
        }
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn wide_prime_limits_and_typed_records_reject_overflow_and_false_derivation() {
        let (mut cfg, _, context) = fixture(false);
        cfg.work_limit = u64::MAX;
        assert!(Limits::derive(&cfg, &context.launch).is_err());
        cfg.work_limit = 1;
        let mut launch = context.launch.clone();
        launch.trace_bytes = 1;
        assert!(Limits::derive(&cfg, &launch).is_err());
        let mut value = serde_json::to_value(base_result()).unwrap();
        value["unexpected"] = json!(true);
        assert!(serde_json::from_value::<ResultRecord>(value).is_err());
        assert!(number("01").is_err());
        assert!(number("18446744073709551616").is_err());
    }
}
