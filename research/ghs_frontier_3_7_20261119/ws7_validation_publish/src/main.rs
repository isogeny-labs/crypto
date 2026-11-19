use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path};

const SCHEMA: &str = "ghs.frontier.validation-bundle/v1";
const AUTH_SCHEMA: &str = "ghs.frontier.publication-authorization/v1";
const REQUIRED_LANES: [&str; 4] = [
    "ws3_parallel_rho",
    "ws4_composition",
    "ws5_matched_ghs",
    "ws6_solver_coverage",
];
const REQUIRED_RHO_PHASES: [&str; 5] =
    ["setup", "walk", "dp_collision", "recovery", "verification"];
const REQUIRED_IC_PHASES: [&str; 5] = [
    "setup",
    "relation_generation",
    "filtering_deduplication",
    "solving",
    "target_recovery_verification",
];
const REQUIRED_EVIDENCE_ROLES: [&str; 12] = [
    "source_binding",
    "protocol",
    "spec",
    "session_manifest",
    "audit_receipt",
    "replay_receipt",
    "independent_receipt",
    "validation_json",
    "benchmark_record",
    "experiment_export",
    "report_pdf",
    "dashboard_dry_run",
];

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Bundle {
    schema: String,
    claim_id: String,
    test_only: bool,
    promotion_requested: bool,
    scope: Scope,
    source: SourceGate,
    lanes: Vec<LaneGate>,
    measurement: MeasurementGate,
    validation: ValidationGate,
    publication: PublicationGate,
    evidence_files: Vec<EvidenceFile>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Scope {
    family: String,
    target_count: u64,
    interval: String,
    unit: String,
    matched_reference: String,
    restricted_family_wording: bool,
    generic_curve_claim: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SourceGate {
    revision: Option<String>,
    revision_published: bool,
    clean_tree: bool,
    binary_sha256: Option<String>,
    image_digest: Option<String>,
    spec_sha256: Option<String>,
    protocol_sha256: Option<String>,
    source_binary_identity_verified: bool,
    no_post_result_code_change: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct LaneGate {
    id: String,
    status: String,
    evidence_count: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct MeasurementGate {
    session_status: String,
    cold_one_target: bool,
    same_target_both_arms: bool,
    strongest_applicable_reference: bool,
    all_phases_charged: bool,
    all_failures_retained: bool,
    warmup_rounds: u64,
    measured_rounds: u64,
    automatic_measurement_retries: u64,
    isolation_level: String,
    aa: AaGate,
    pricing: PricingGate,
    rho_phases: Vec<String>,
    ic_phases: Vec<String>,
    correctness: CorrectnessGate,
    replay: ReplayGate,
    independent: IndependentGate,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AaGate {
    admitted_pairs: u64,
    ratio: Option<f64>,
    ci_low: Option<f64>,
    ci_high: Option<f64>,
    outside_noise_for_claimed_wall_result: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PricingGate {
    band_weights_gae: Vec<f64>,
    every_unpriced_counter_emitted: bool,
    table_work_priced: bool,
    non_operation_bytes_not_priced_as_ops: bool,
    calibration_complete: bool,
    same_calibration_both_arms: bool,
    calibration_triples_per_seed: u64,
    calibrated_weight: Option<f64>,
    complete_cold_cost: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CorrectnessGate {
    completed_runs: u64,
    recovered_scalars_verified: u64,
    wrong_answers: u64,
    invalid_records_accepted: u64,
    phase_sums_exact: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ReplayGate {
    audit_status: String,
    replay_all: bool,
    deterministic_runs: u64,
    identical_replays: u64,
    receipt_sha256: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct IndependentGate {
    original_host_class: Option<String>,
    validator_host_class: Option<String>,
    both_arms_replayed: bool,
    every_recovered_scalar_checked: bool,
    receipt_sha256: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ValidationGate {
    raw_evidence_sealed: bool,
    producer_distinct_validator: bool,
    headlines_recomputed_from_raw: bool,
    no_post_result_tuning: bool,
    validation_json_status: String,
    pcc_standing: String,
    claim_scope_exact: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PublicationGate {
    benchmark_record_closed: bool,
    experiment_export_verified: bool,
    run_report_present: bool,
    result_report_present: bool,
    report_pdf_present: bool,
    figures_svg_and_png_present: bool,
    failures_timeouts_regressions_included: bool,
    frontier_rebuilt_and_checked: bool,
    leaderboard_generated_and_checked: bool,
    lab_browser_generated_and_checked: bool,
    dashboard_coverage_checked: bool,
    progress_timeline_updated_if_numeric: bool,
    no_manual_dashboard_edits: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct EvidenceFile {
    role: String,
    path: String,
    sha256: String,
    bytes: u64,
}

#[derive(Debug, Serialize)]
struct CheckReport {
    schema: &'static str,
    claim_id: String,
    ready_for_publication: bool,
    blocker_count: usize,
    blockers: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Authorization {
    schema: &'static str,
    claim_id: String,
    manifest_sha256: String,
    source_revision: String,
    evidence_files: usize,
    gate: &'static str,
}

fn is_lower_hex(value: &str, width: usize) -> bool {
    value.len() == width
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn hash_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn hash_file(path: &Path) -> Result<(String, u64), String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    Ok((hash_bytes(&bytes), bytes.len() as u64))
}

fn safe_relative(path: &str) -> bool {
    let path = Path::new(path);
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}

fn contains_all(actual: &[String], required: &[&str]) -> bool {
    let actual: BTreeSet<&str> = actual.iter().map(String::as_str).collect();
    required.iter().all(|item| actual.contains(item))
}

fn push_if(blockers: &mut Vec<String>, condition: bool, message: &str) {
    if condition {
        blockers.push(message.to_string());
    }
}

fn evaluate(bundle: &Bundle, root: &Path, verify_files: bool, allow_test: bool) -> Vec<String> {
    let mut blockers = Vec::new();
    push_if(&mut blockers, bundle.schema != SCHEMA, "schema is not v1");
    push_if(
        &mut blockers,
        bundle.claim_id.is_empty() || !bundle.claim_id.is_ascii(),
        "claim_id must be nonempty ASCII",
    );
    push_if(
        &mut blockers,
        bundle.test_only && !allow_test,
        "test_only bundle cannot authorize publication",
    );
    push_if(
        &mut blockers,
        !bundle.promotion_requested,
        "promotion_requested is false",
    );

    push_if(
        &mut blockers,
        bundle.scope.family != "restricted-ghs-weak-e0-fq3",
        "scope family is not the restricted GHS-weak E0/Fq3 family",
    );
    push_if(
        &mut blockers,
        bundle.scope.target_count != 1,
        "primary comparison is not one target",
    );
    push_if(
        &mut blockers,
        bundle.scope.interval != "cold-end-to-end",
        "primary interval is not cold end to end",
    );
    push_if(
        &mut blockers,
        bundle.scope.unit != "M-eq-and-S",
        "unit is not M-eq-and-S",
    );
    push_if(
        &mut blockers,
        bundle.scope.matched_reference != "strongest-applicable-strong-rho",
        "reference is not strongest applicable strong rho",
    );
    push_if(
        &mut blockers,
        !bundle.scope.restricted_family_wording,
        "restricted-family wording is absent",
    );
    push_if(
        &mut blockers,
        bundle.scope.generic_curve_claim,
        "bundle attempts a generic-curve claim",
    );

    match bundle.source.revision.as_deref() {
        Some(revision) if is_lower_hex(revision, 40) => {}
        _ => blockers.push("source revision is missing or malformed".to_string()),
    }
    for (label, value) in [
        ("binary", bundle.source.binary_sha256.as_deref()),
        ("spec", bundle.source.spec_sha256.as_deref()),
        ("protocol", bundle.source.protocol_sha256.as_deref()),
    ] {
        if !value.is_some_and(|digest| is_lower_hex(digest, 64)) {
            blockers.push(format!("{label} SHA-256 is missing or malformed"));
        }
    }
    if !bundle.source.image_digest.as_deref().is_some_and(|digest| {
        digest
            .strip_prefix("sha256:")
            .is_some_and(|hex| is_lower_hex(hex, 64))
    }) {
        blockers.push("image digest is missing or malformed".to_string());
    }
    push_if(
        &mut blockers,
        !bundle.source.revision_published,
        "source revision is not published",
    );
    push_if(
        &mut blockers,
        !bundle.source.clean_tree,
        "source tree was not clean",
    );
    push_if(
        &mut blockers,
        !bundle.source.source_binary_identity_verified,
        "source/binary/image identity was not verified",
    );
    push_if(
        &mut blockers,
        !bundle.source.no_post_result_code_change,
        "post-result code identity is not frozen",
    );

    let lane_ids: BTreeSet<&str> = bundle.lanes.iter().map(|lane| lane.id.as_str()).collect();
    let expected_lanes: BTreeSet<&str> = REQUIRED_LANES.into_iter().collect();
    push_if(
        &mut blockers,
        lane_ids != expected_lanes || lane_ids.len() != bundle.lanes.len(),
        "lane census is missing, duplicated, or unexpected",
    );
    for lane in &bundle.lanes {
        if lane.status != "verified-complete" {
            blockers.push(format!("{} is not verified-complete", lane.id));
        }
        if lane.evidence_count == 0 {
            blockers.push(format!("{} has no evidence", lane.id));
        }
    }

    let measurement = &bundle.measurement;
    push_if(
        &mut blockers,
        measurement.session_status != "complete",
        "measurement session is not complete",
    );
    push_if(
        &mut blockers,
        !measurement.cold_one_target,
        "cold one-target gate failed",
    );
    push_if(
        &mut blockers,
        !measurement.same_target_both_arms,
        "arms did not solve the same target",
    );
    push_if(
        &mut blockers,
        !measurement.strongest_applicable_reference,
        "strongest applicable reference gate failed",
    );
    push_if(
        &mut blockers,
        !measurement.all_phases_charged,
        "not all phases were charged",
    );
    push_if(
        &mut blockers,
        !measurement.all_failures_retained,
        "failures were not retained",
    );
    push_if(
        &mut blockers,
        measurement.warmup_rounds < 1,
        "fewer than one warmup round",
    );
    push_if(
        &mut blockers,
        measurement.measured_rounds < 5,
        "fewer than five measured rounds",
    );
    push_if(
        &mut blockers,
        measurement.automatic_measurement_retries != 0,
        "automatic measurement retries were nonzero",
    );
    push_if(
        &mut blockers,
        !matches!(measurement.isolation_level.as_str(), "L2" | "L3"),
        "wall measurement did not earn L2 or L3",
    );
    push_if(
        &mut blockers,
        !contains_all(&measurement.rho_phases, &REQUIRED_RHO_PHASES),
        "rho phase census is incomplete",
    );
    push_if(
        &mut blockers,
        !contains_all(&measurement.ic_phases, &REQUIRED_IC_PHASES),
        "IC phase census is incomplete",
    );

    let aa = &measurement.aa;
    push_if(
        &mut blockers,
        aa.admitted_pairs < 5,
        "fewer than five admitted A/A pairs",
    );
    match (aa.ratio, aa.ci_low, aa.ci_high) {
        (Some(ratio), Some(low), Some(high))
            if ratio.is_finite()
                && low.is_finite()
                && high.is_finite()
                && (0.9..=1.1).contains(&low)
                && (0.9..=1.1).contains(&high)
                && low <= 1.0
                && high >= 1.0
                && low <= ratio
                && ratio <= high => {}
        _ => blockers.push("A/A interval is absent or outside the frozen gate".to_string()),
    }
    push_if(
        &mut blockers,
        !aa.outside_noise_for_claimed_wall_result,
        "claimed wall result is not outside A/A noise",
    );

    let pricing = &measurement.pricing;
    push_if(
        &mut blockers,
        pricing.band_weights_gae != [0.0, 0.1, 1.0],
        "pricing band is not exactly 0, 0.1, 1 GAE",
    );
    push_if(
        &mut blockers,
        !pricing.every_unpriced_counter_emitted,
        "an unpriced counter is absent",
    );
    push_if(
        &mut blockers,
        !pricing.table_work_priced,
        "table work was not priced",
    );
    push_if(
        &mut blockers,
        !pricing.non_operation_bytes_not_priced_as_ops,
        "byte counters were priced as operations",
    );
    push_if(
        &mut blockers,
        !pricing.calibration_complete,
        "calibration is incomplete",
    );
    push_if(
        &mut blockers,
        !pricing.same_calibration_both_arms,
        "arms did not use the same calibration",
    );
    push_if(
        &mut blockers,
        pricing.calibration_triples_per_seed < 600,
        "fewer than 600 calibration triples per seed",
    );
    push_if(
        &mut blockers,
        !pricing
            .calibrated_weight
            .is_some_and(|value| value.is_finite() && value > 0.0),
        "calibrated native-operation weight is absent or invalid",
    );
    push_if(
        &mut blockers,
        !pricing.complete_cold_cost,
        "cold cost is incomplete",
    );

    let correctness = &measurement.correctness;
    push_if(
        &mut blockers,
        correctness.completed_runs == 0,
        "no completed runs",
    );
    push_if(
        &mut blockers,
        correctness.recovered_scalars_verified != correctness.completed_runs,
        "not every completed scalar was verified",
    );
    push_if(
        &mut blockers,
        correctness.wrong_answers != 0,
        "wrong answers are present",
    );
    push_if(
        &mut blockers,
        correctness.invalid_records_accepted != 0,
        "invalid records were accepted",
    );
    push_if(
        &mut blockers,
        !correctness.phase_sums_exact,
        "phase sums are not exact",
    );

    let replay = &measurement.replay;
    push_if(
        &mut blockers,
        replay.audit_status != "OK",
        "native audit is not OK",
    );
    push_if(&mut blockers, !replay.replay_all, "replay-all was not run");
    push_if(
        &mut blockers,
        replay.deterministic_runs == 0 || replay.identical_replays != replay.deterministic_runs,
        "not every deterministic run replayed identically",
    );
    if !replay
        .receipt_sha256
        .as_deref()
        .is_some_and(|digest| is_lower_hex(digest, 64))
    {
        blockers.push("replay receipt SHA-256 is absent or malformed".to_string());
    }

    let independent = &measurement.independent;
    match (
        independent.original_host_class.as_deref(),
        independent.validator_host_class.as_deref(),
    ) {
        (Some(original), Some(validator)) if !original.is_empty() && original != validator => {}
        _ => blockers.push("independent host class is absent or not distinct".to_string()),
    }
    push_if(
        &mut blockers,
        !independent.both_arms_replayed,
        "independent runner did not replay both arms",
    );
    push_if(
        &mut blockers,
        !independent.every_recovered_scalar_checked,
        "independent runner did not check every recovered scalar",
    );
    if !independent
        .receipt_sha256
        .as_deref()
        .is_some_and(|digest| is_lower_hex(digest, 64))
    {
        blockers.push("independent receipt SHA-256 is absent or malformed".to_string());
    }

    let validation = &bundle.validation;
    push_if(
        &mut blockers,
        !validation.raw_evidence_sealed,
        "raw evidence is not sealed",
    );
    push_if(
        &mut blockers,
        !validation.producer_distinct_validator,
        "validator is not producer-distinct",
    );
    push_if(
        &mut blockers,
        !validation.headlines_recomputed_from_raw,
        "headlines were not recomputed from raw evidence",
    );
    push_if(
        &mut blockers,
        !validation.no_post_result_tuning,
        "post-result tuning gate failed",
    );
    push_if(
        &mut blockers,
        validation.validation_json_status != "verified",
        "VALIDATION.json is not verified",
    );
    push_if(
        &mut blockers,
        validation.pcc_standing != "verified-ci-main",
        "PCC does not have verified CI-main standing",
    );
    push_if(
        &mut blockers,
        !validation.claim_scope_exact,
        "claim scope is not exact",
    );

    let publication = &bundle.publication;
    for (condition, message) in [
        (
            publication.benchmark_record_closed,
            "benchmark record is not closed",
        ),
        (
            publication.experiment_export_verified,
            "experiment export is not verified",
        ),
        (publication.run_report_present, "run report is absent"),
        (publication.result_report_present, "result report is absent"),
        (publication.report_pdf_present, "report PDF is absent"),
        (
            publication.figures_svg_and_png_present,
            "SVG/PNG figure pair is absent",
        ),
        (
            publication.failures_timeouts_regressions_included,
            "failures, timeouts, or regressions were omitted",
        ),
        (
            publication.frontier_rebuilt_and_checked,
            "frontier was not rebuilt and checked",
        ),
        (
            publication.leaderboard_generated_and_checked,
            "leaderboard was not generated and checked",
        ),
        (
            publication.lab_browser_generated_and_checked,
            "lab browser was not generated and checked",
        ),
        (
            publication.dashboard_coverage_checked,
            "dashboard coverage check did not pass",
        ),
        (
            publication.progress_timeline_updated_if_numeric,
            "numeric result is absent from the progress timeline",
        ),
        (
            publication.no_manual_dashboard_edits,
            "dashboard output was manually edited",
        ),
    ] {
        push_if(&mut blockers, !condition, message);
    }

    let roles: BTreeSet<&str> = bundle
        .evidence_files
        .iter()
        .map(|evidence| evidence.role.as_str())
        .collect();
    push_if(
        &mut blockers,
        roles.len() != bundle.evidence_files.len(),
        "evidence roles are duplicated",
    );
    for role in REQUIRED_EVIDENCE_ROLES {
        if !roles.contains(role) {
            blockers.push(format!("required evidence role {role} is absent"));
        }
    }
    for evidence in &bundle.evidence_files {
        if !safe_relative(&evidence.path) {
            blockers.push(format!("{} has unsafe evidence path", evidence.role));
            continue;
        }
        if !is_lower_hex(&evidence.sha256, 64) {
            blockers.push(format!("{} has malformed SHA-256", evidence.role));
        }
        if evidence.bytes == 0 {
            blockers.push(format!("{} has zero bytes", evidence.role));
        }
        if verify_files {
            let path = root.join(&evidence.path);
            match hash_file(&path) {
                Ok((digest, bytes)) => {
                    if digest != evidence.sha256 {
                        blockers.push(format!("{} file SHA-256 mismatch", evidence.role));
                    }
                    if bytes != evidence.bytes {
                        blockers.push(format!("{} file byte count mismatch", evidence.role));
                    }
                }
                Err(error) => blockers.push(format!("{} evidence: {error}", evidence.role)),
            }
        }
    }
    blockers
}

fn report(bundle: &Bundle, blockers: Vec<String>) -> CheckReport {
    CheckReport {
        schema: "ghs.frontier.validation-check/v1",
        claim_id: bundle.claim_id.clone(),
        ready_for_publication: blockers.is_empty(),
        blocker_count: blockers.len(),
        blockers,
    }
}

fn load(path: &Path) -> Result<(Vec<u8>, Bundle), String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let bundle = serde_json::from_slice(&bytes)
        .map_err(|error| format!("parse {}: {error}", path.display()))?;
    Ok((bytes, bundle))
}

fn root_for(path: &Path) -> &Path {
    path.parent().unwrap_or_else(|| Path::new("."))
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("create {}: {error}", path.display()))?;
    file.write_all(bytes)
        .map_err(|error| format!("write {}: {error}", path.display()))?;
    file.sync_all()
        .map_err(|error| format!("sync {}: {error}", path.display()))
}

fn synthetic_ready(root: &Path) -> Result<Bundle, String> {
    fs::create_dir_all(root).map_err(|error| format!("create {}: {error}", root.display()))?;
    let evidence_files = REQUIRED_EVIDENCE_ROLES
        .iter()
        .map(|role| {
            let path = format!("{role}.txt");
            let bytes = format!("synthetic self-test evidence for {role}\n").into_bytes();
            fs::write(root.join(&path), &bytes)
                .map_err(|error| format!("write self-test {role}: {error}"))?;
            Ok(EvidenceFile {
                role: (*role).to_string(),
                path,
                sha256: hash_bytes(&bytes),
                bytes: bytes.len() as u64,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let hash = "a".repeat(64);
    Ok(Bundle {
        schema: SCHEMA.to_string(),
        claim_id: "SYNTHETIC-SELFTEST".to_string(),
        test_only: true,
        promotion_requested: true,
        scope: Scope {
            family: "restricted-ghs-weak-e0-fq3".to_string(),
            target_count: 1,
            interval: "cold-end-to-end".to_string(),
            unit: "M-eq-and-S".to_string(),
            matched_reference: "strongest-applicable-strong-rho".to_string(),
            restricted_family_wording: true,
            generic_curve_claim: false,
        },
        source: SourceGate {
            revision: Some("b".repeat(40)),
            revision_published: true,
            clean_tree: true,
            binary_sha256: Some(hash.clone()),
            image_digest: Some(format!("sha256:{hash}")),
            spec_sha256: Some(hash.clone()),
            protocol_sha256: Some(hash.clone()),
            source_binary_identity_verified: true,
            no_post_result_code_change: true,
        },
        lanes: REQUIRED_LANES
            .iter()
            .map(|id| LaneGate {
                id: (*id).to_string(),
                status: "verified-complete".to_string(),
                evidence_count: 1,
            })
            .collect(),
        measurement: MeasurementGate {
            session_status: "complete".to_string(),
            cold_one_target: true,
            same_target_both_arms: true,
            strongest_applicable_reference: true,
            all_phases_charged: true,
            all_failures_retained: true,
            warmup_rounds: 1,
            measured_rounds: 5,
            automatic_measurement_retries: 0,
            isolation_level: "L2".to_string(),
            aa: AaGate {
                admitted_pairs: 5,
                ratio: Some(1.0),
                ci_low: Some(0.98),
                ci_high: Some(1.02),
                outside_noise_for_claimed_wall_result: true,
            },
            pricing: PricingGate {
                band_weights_gae: vec![0.0, 0.1, 1.0],
                every_unpriced_counter_emitted: true,
                table_work_priced: true,
                non_operation_bytes_not_priced_as_ops: true,
                calibration_complete: true,
                same_calibration_both_arms: true,
                calibration_triples_per_seed: 600,
                calibrated_weight: Some(0.01),
                complete_cold_cost: true,
            },
            rho_phases: REQUIRED_RHO_PHASES
                .iter()
                .map(|phase| phase.to_string())
                .collect(),
            ic_phases: REQUIRED_IC_PHASES
                .iter()
                .map(|phase| phase.to_string())
                .collect(),
            correctness: CorrectnessGate {
                completed_runs: 10,
                recovered_scalars_verified: 10,
                wrong_answers: 0,
                invalid_records_accepted: 0,
                phase_sums_exact: true,
            },
            replay: ReplayGate {
                audit_status: "OK".to_string(),
                replay_all: true,
                deterministic_runs: 10,
                identical_replays: 10,
                receipt_sha256: Some(hash.clone()),
            },
            independent: IndependentGate {
                original_host_class: Some("host-a".to_string()),
                validator_host_class: Some("host-b".to_string()),
                both_arms_replayed: true,
                every_recovered_scalar_checked: true,
                receipt_sha256: Some(hash),
            },
        },
        validation: ValidationGate {
            raw_evidence_sealed: true,
            producer_distinct_validator: true,
            headlines_recomputed_from_raw: true,
            no_post_result_tuning: true,
            validation_json_status: "verified".to_string(),
            pcc_standing: "verified-ci-main".to_string(),
            claim_scope_exact: true,
        },
        publication: PublicationGate {
            benchmark_record_closed: true,
            experiment_export_verified: true,
            run_report_present: true,
            result_report_present: true,
            report_pdf_present: true,
            figures_svg_and_png_present: true,
            failures_timeouts_regressions_included: true,
            frontier_rebuilt_and_checked: true,
            leaderboard_generated_and_checked: true,
            lab_browser_generated_and_checked: true,
            dashboard_coverage_checked: true,
            progress_timeline_updated_if_numeric: true,
            no_manual_dashboard_edits: true,
        },
        evidence_files,
    })
}

fn run_self_test() -> Result<usize, String> {
    let root = env::temp_dir().join(format!(
        "ghs-frontier-publication-gate-{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root)
            .map_err(|error| format!("remove stale {}: {error}", root.display()))?;
    }
    let base = synthetic_ready(&root)?;
    let ready_blockers = evaluate(&base, &root, true, true);
    if !ready_blockers.is_empty() {
        return Err(format!(
            "synthetic complete bundle rejected: {ready_blockers:?}"
        ));
    }
    let mut rejected = 0usize;
    macro_rules! corrupt {
        ($label:expr, $change:expr) => {{
            let mut candidate = base.clone();
            $change(&mut candidate);
            let blockers = evaluate(&candidate, &root, true, true);
            if blockers.is_empty() {
                return Err(format!("corruption accepted: {}", $label));
            }
            rejected += 1;
        }};
    }
    corrupt!("schema", |b: &mut Bundle| b.schema = "v0".to_string());
    corrupt!("promotion", |b: &mut Bundle| b.promotion_requested = false);
    corrupt!("family", |b: &mut Bundle| b.scope.family =
        "generic".to_string());
    corrupt!("generic claim", |b: &mut Bundle| b
        .scope
        .generic_curve_claim =
        true);
    corrupt!("revision", |b: &mut Bundle| b.source.revision =
        Some("bad".to_string()));
    corrupt!("unpublished", |b: &mut Bundle| b
        .source
        .revision_published =
        false);
    corrupt!("binary identity", |b: &mut Bundle| b
        .source
        .source_binary_identity_verified =
        false);
    corrupt!("lane status", |b: &mut Bundle| b.lanes[0].status =
        "partial".to_string());
    corrupt!("duplicate lane", |b: &mut Bundle| b.lanes[1].id =
        b.lanes[0].id.clone());
    corrupt!("cold", |b: &mut Bundle| b.measurement.cold_one_target =
        false);
    corrupt!("same target", |b: &mut Bundle| b
        .measurement
        .same_target_both_arms =
        false);
    corrupt!("rounds", |b: &mut Bundle| b.measurement.measured_rounds = 4);
    corrupt!("warmup", |b: &mut Bundle| b.measurement.warmup_rounds = 0);
    corrupt!("retries", |b: &mut Bundle| b
        .measurement
        .automatic_measurement_retries =
        1);
    corrupt!("isolation", |b: &mut Bundle| b
        .measurement
        .isolation_level =
        "L1".to_string());
    corrupt!("A/A pairs", |b: &mut Bundle| b
        .measurement
        .aa
        .admitted_pairs = 4);
    corrupt!("A/A interval", |b: &mut Bundle| b.measurement.aa.ci_high =
        Some(1.2));
    corrupt!("pricing weights", |b: &mut Bundle| b
        .measurement
        .pricing
        .band_weights_gae =
        vec![0.0, 1.0]);
    corrupt!("unpriced counter", |b: &mut Bundle| b
        .measurement
        .pricing
        .every_unpriced_counter_emitted =
        false);
    corrupt!("calibration", |b: &mut Bundle| b
        .measurement
        .pricing
        .calibration_complete =
        false);
    corrupt!("wrong answer", |b: &mut Bundle| b
        .measurement
        .correctness
        .wrong_answers = 1);
    corrupt!("scalar replay", |b: &mut Bundle| b
        .measurement
        .correctness
        .recovered_scalars_verified =
        9);
    corrupt!("replay all", |b: &mut Bundle| b
        .measurement
        .replay
        .replay_all = false);
    corrupt!("replay mismatch", |b: &mut Bundle| b
        .measurement
        .replay
        .identical_replays =
        9);
    corrupt!("same host", |b: &mut Bundle| b
        .measurement
        .independent
        .validator_host_class =
        Some("host-a".to_string()));
    corrupt!("one arm", |b: &mut Bundle| b
        .measurement
        .independent
        .both_arms_replayed = false);
    corrupt!("unsealed", |b: &mut Bundle| b
        .validation
        .raw_evidence_sealed =
        false);
    corrupt!("producer validator", |b: &mut Bundle| b
        .validation
        .producer_distinct_validator =
        false);
    corrupt!("PCC", |b: &mut Bundle| b.validation.pcc_standing =
        "unverified".to_string());
    corrupt!("benchmark record", |b: &mut Bundle| b
        .publication
        .benchmark_record_closed =
        false);
    corrupt!("dashboard coverage", |b: &mut Bundle| b
        .publication
        .dashboard_coverage_checked =
        false);
    corrupt!("missing evidence", |b: &mut Bundle| {
        b.evidence_files.pop();
    });
    corrupt!("unsafe path", |b: &mut Bundle| b.evidence_files[0].path =
        "../escape".to_string());
    corrupt!("evidence digest", |b: &mut Bundle| b.evidence_files[0]
        .sha256 =
        "0".repeat(64));
    let test_only_blockers = evaluate(&base, &root, true, false);
    if !test_only_blockers
        .iter()
        .any(|blocker| blocker.contains("test_only"))
    {
        return Err("test_only bundle was not refused by the CLI policy".to_string());
    }
    rejected += 1;
    fs::remove_dir_all(&root).map_err(|error| format!("remove {}: {error}", root.display()))?;
    Ok(rejected)
}

fn check_command(path: &Path) -> Result<i32, String> {
    let (_, bundle) = load(path)?;
    let blockers = evaluate(&bundle, root_for(path), true, false);
    let report = report(&bundle, blockers);
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?
    );
    Ok(if report.ready_for_publication { 0 } else { 3 })
}

fn authorize_command(manifest: &Path, output: &Path) -> Result<i32, String> {
    if output.exists() {
        return Err(format!("refusing to overwrite {}", output.display()));
    }
    let (bytes, bundle) = load(manifest)?;
    let blockers = evaluate(&bundle, root_for(manifest), true, false);
    if !blockers.is_empty() {
        let report = report(&bundle, blockers);
        eprintln!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?
        );
        return Ok(3);
    }
    let authorization = Authorization {
        schema: AUTH_SCHEMA,
        claim_id: bundle.claim_id,
        manifest_sha256: hash_bytes(&bytes),
        source_revision: bundle.source.revision.expect("validated revision"),
        evidence_files: bundle.evidence_files.len(),
        gate: "all-required-gates-passed",
    };
    let mut encoded =
        serde_json::to_vec_pretty(&authorization).map_err(|error| error.to_string())?;
    encoded.push(b'\n');
    write_new(output, &encoded)?;
    println!("authorization written: {}", output.display());
    Ok(0)
}

fn usage(program: &str) {
    eprintln!(
        "usage:\n  {program} check MANIFEST.json\n  {program} authorize MANIFEST.json OUTPUT.json\n  {program} self-test"
    );
}

fn run() -> Result<i32, String> {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, command, path] if command == "check" => check_command(Path::new(path)),
        [_, command, manifest, output] if command == "authorize" => {
            authorize_command(Path::new(manifest), Path::new(output))
        }
        [_, command] if command == "self-test" => {
            let rejected = run_self_test()?;
            println!("publication gate self-test PASS");
            println!("synthetic_complete_bundles_accepted=1");
            println!("corruptions_rejected={rejected}");
            println!("timing_observations=0 frontier_promotions=0");
            Ok(0)
        }
        _ => {
            usage(
                args.first()
                    .map(String::as_str)
                    .unwrap_or("publication-gate"),
            );
            Ok(2)
        }
    }
}

fn main() {
    match run() {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            eprintln!("publication gate ERROR: {error}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corruption_matrix_is_fail_closed() {
        assert_eq!(run_self_test().expect("self-test"), 35);
    }

    #[test]
    fn unknown_fields_are_rejected() {
        let value = serde_json::json!({"schema": SCHEMA, "unknown": true});
        let error = serde_json::from_value::<Bundle>(value).expect_err("must reject");
        assert!(error.to_string().contains("unknown field"));
    }
}
