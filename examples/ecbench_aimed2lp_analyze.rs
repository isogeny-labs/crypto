//! Native, frozen read-out for the aimed-m=3 plus exact-2LP experiment.
//!
//! The independent units are the eight public targets at each field size.
//! Five measured rounds are repeated timings within a target, never forty
//! independent samples.  Complete L2 whole-call wall is the primary host-
//! specific outcome.  The 0/0.1/1 native-work band is emitted separately;
//! a missing native count makes pricing unrankable without erasing a valid
//! wall result.

use crypto_lib::cryptanalysis::ecbench::audit::{AuditReport, AUDIT_SCHEMA};
use crypto_lib::cryptanalysis::ecbench::band::unpriced_counts;
use crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex;
use crypto_lib::cryptanalysis::ecbench::compare::{
    compare_loaded, CompareOptions, Comparison,
};
use crypto_lib::cryptanalysis::ecbench::record::Record;
use crypto_lib::cryptanalysis::ecbench::runner::{read_records, read_session, Session};
use crypto_lib::cryptanalysis::ecbench::spec::{plan, Execution, Level, Plan, Spec};
use crypto_lib::cryptanalysis::ecbench::stats::cluster_bootstrap_ci;
use crypto_lib::ml_dataset::{protocol_canonical_bytes, protocol_unique_json};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const RESULT_SCHEMA: &str = "icb.aimed2lp.results/v2";
const VALIDATION_SCHEMA: &str = "icb.aimed2lp.validation/v2";
const EXPORT_SCHEMA: &str = "experiment.export/v1";
const ARMS: [&str; 6] = [
    "rho-v2",
    "aimed",
    "aimed-aa",
    "aimed-lp0",
    "aimed-lp2-c05",
    "aimed-lp2-c1",
];
const CONTROL_ARMS: [&str; 5] = [
    "aimed",
    "aimed-lp0",
    "aimed-aa",
    "aimed-lp2-c05",
    "aimed-lp2-c1",
];
const CANDIDATES: [(&str, &str); 2] = [
    ("C-c05", "aimed-lp2-c05"),
    ("C-c1", "aimed-lp2-c1"),
];
const TARGETS: usize = 8;
const ROUNDS: usize = 5;
const RESAMPLES: usize = 20_000;
const WALL_THRESHOLD: f64 = 0.95;
const PRICES: [f64; 3] = [0.0, 0.1, 1.0];
const TARGET_NOT_ATTEMPTED_REASON: &str =
    "positive control did not complete; target launch prohibited by protocol";
const SELF_TEST_CHECKS: [&str; 6] = [
    "clustered-sign-flip",
    "holm-four-cell",
    "canonical-json",
    "attempt-envelope-adversarial",
    "result-schema-v2-adversarial",
    "export-636-adversarial",
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: String,
    protocol: String,
    result_schema: String,
    receipt_schema: String,
    request_schema: String,
    focused_gates: String,
    checkpoint_receipt: String,
    image_receipt: String,
    provider_preflight: String,
    native_analyzer: NativeAnalyzerInput,
    native_provider_preflight: NativeProviderPreflightInput,
    provider_request: Value,
    request_templates: BTreeMap<String, String>,
    positive_control: ControlInput,
    sizes: Vec<SizeInput>,
    outputs: OutputInput,
    #[serde(default)]
    attempt_receipts: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeAnalyzerInput {
    source: String,
    source_sha256: String,
    binary_sha256: String,
    self_test_receipt: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeProviderPreflightInput {
    source: String,
    source_sha256: String,
    binary_sha256: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ControlInput {
    session: String,
    receipt: String,
    replay_receipt: String,
    attempt_receipt: String,
    export: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SizeInput {
    n: u32,
    session: String,
    replay_receipt: String,
    attempt_receipt: String,
    export: String,
    comparisons: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OutputInput {
    results: String,
    validation: String,
    canonical_export: String,
    export_schema: String,
    expected_rows: usize,
}

struct Identity {
    protocol_id: String,
    protocol_sha256: String,
    source_commit: String,
    binary_sha256: String,
    image_digest: String,
    analysis_source_sha256: String,
    analysis_binary_sha256: String,
    provider_preflight_source_sha256: String,
    provider_preflight_binary_sha256: String,
    image: Value,
    protocol: Value,
}

struct Bundle {
    cell: String,
    dir: PathBuf,
    session: Session,
    records: Vec<Record>,
    audit: Option<AuditReport>,
}

struct AttemptReceipt {
    relative: String,
    class: TerminalClass,
    value: Value,
}

#[derive(Clone)]
struct Estimate {
    point: f64,
    ci95: (f64, f64),
    raw_p: f64,
    adjusted_p: f64,
    clusters: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TerminalClass {
    Complete,
    ScientificCensored,
    InfrastructureBlocked,
    NotAttempted,
}

fn bytes(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn json_value(path: &Path) -> Result<Value, String> {
    let text = String::from_utf8(bytes(path)?)
        .map_err(|e| format!("{}: invalid UTF-8: {e}", path.display()))?;
    protocol_unique_json(&text, 0).map_err(|e| format!("{}: {e}", path.display()))
}

fn read_typed<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    serde_json::from_value(json_value(path)?).map_err(|e| format!("{}: {e}", path.display()))
}

fn now_string() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("unix:{seconds}")
}

fn ratio(pairs: &[(f64, f64)]) -> Option<f64> {
    let (a, b) = pairs
        .iter()
        .fold((0.0, 0.0), |(sa, sb), (x, y)| (sa + x, sb + y));
    (!pairs.is_empty() && a > 0.0).then_some(b / a)
}

fn protocol_identity(path: &Path) -> Result<(Value, String), String> {
    let text = String::from_utf8(bytes(path)?).map_err(|e| format!("protocol UTF-8: {e}"))?;
    let value = protocol_unique_json(&text, 0)?;
    if value["schema"] != "experiment.protocol/v1" {
        return Err("protocol schema is not experiment.protocol/v1".into());
    }
    let freeze = &value["freeze"];
    if freeze["status"] != "frozen" || freeze["filed_before_affected_data"] != true {
        return Err("protocol is not frozen before affected data".into());
    }
    let expected = freeze["protocol_sha256"]
        .as_str()
        .ok_or("frozen protocol has no hash")?;
    let mut body = value.clone();
    body.as_object_mut()
        .ok_or("protocol is not an object")?
        .remove("freeze");
    let actual = sha256_hex(&protocol_canonical_bytes(&body)?);
    if actual != expected {
        return Err(format!("frozen protocol hash differs: {actual} != {expected}"));
    }
    Ok((value, actual))
}

fn load_identity(base: &Path, manifest_path: &Path, manifest: &Manifest) -> Result<Identity, String> {
    let protocol_path = base.join(&manifest.protocol);
    let (protocol, protocol_sha256) = protocol_identity(&protocol_path)?;
    let protocol_id = protocol["protocol_id"]
        .as_str()
        .ok_or("protocol_id missing")?
        .to_owned();
    let focused_path = base.join(&manifest.focused_gates);
    let focused = json_value(&focused_path)?;
    if focused["schema"] != "icb.aimed2lp.focused_gates/v1" || focused["status"] != "pass" {
        return Err("focused gates are not a passing v1 receipt".into());
    }
    let image = json_value(&base.join(&manifest.image_receipt))?;
    exact_keys(
        &image,
        &[
            "schema", "created", "source_commit", "cargo_lock_sha256", "binary_sha256",
            "image_digest", "image_ref", "build_command", "rustflags", "cargo_build_jobs",
            "focused_gate_receipt_sha256", "protocol_sha256", "checkpoint_receipt_sha256",
            "analysis_manifest_sha256", "result_schema_sha256", "receipt_schema_sha256",
            "request_schema_sha256", "native_analyzer_source_sha256",
            "native_analyzer_binary_sha256", "native_provider_preflight_source_sha256",
            "native_provider_preflight_binary_sha256", "specs",
        ],
        "immutable image receipt",
    )?;
    if image["schema"] != "icb.aimed2lp.image/v1" {
        return Err("image receipt has the wrong schema".into());
    }
    let source_commit = image["source_commit"]
        .as_str()
        .ok_or("image source_commit missing")?
        .to_owned();
    if image["created"].as_str().is_none_or(str::is_empty)
        || source_commit.len() != 40
        || !lowercase_hex(&source_commit)
    {
        return Err("image receipt has malformed creation/source identity".into());
    }
    exact_keys(
        &image["specs"],
        &[
            "research/icb_aimed_large_prime_n53_n61_20261101/specs/control-n13.json",
            "research/icb_aimed_large_prime_n53_n61_20261101/specs/n53.json",
            "research/icb_aimed_large_prime_n53_n61_20261101/specs/n61.json",
        ],
        "image spec bindings",
    )?;
    for relative in [expected_spec_path(13), expected_spec_path(53), expected_spec_path(61)] {
        let spec = &image["specs"][relative.as_str()];
        exact_keys(spec, &["file_sha256", "identity_sha256", "spec_id"], "image spec")?;
        if sha256_string(&spec["file_sha256"]).is_none()
            || sha256_string(&spec["identity_sha256"]).is_none()
            || !spec["spec_id"].as_str().is_some_and(|id| {
                id.len() == 17
                    && id.starts_with("ECS1h")
                    && lowercase_hex(&id[5..])
            })
        {
            return Err(format!("image spec binding is malformed for {relative}"));
        }
    }
    let binary_sha256 = image["binary_sha256"]
        .as_str()
        .ok_or("image binary_sha256 missing")?
        .to_owned();
    let image_digest = image["image_digest"]
        .as_str()
        .ok_or("image digest missing")?
        .to_owned();
    if sha256_string(&image["binary_sha256"]).is_none()
        || image_digest.len() != 71
        || !image_digest.starts_with("sha256:")
        || !lowercase_hex(&image_digest[7..])
        || !image["image_ref"]
        .as_str()
        .is_some_and(|reference| reference.ends_with(&format!("@{image_digest}")))
        || image["protocol_sha256"] != protocol_sha256
        || image["analysis_manifest_sha256"] != sha256_hex(&bytes(manifest_path)?)
        || image["cargo_lock_sha256"] != sha256_hex(&bytes(&base.join("../../Cargo.lock"))?)
        || image["build_command"]
            != json!([
                "cargo",
                "build",
                "--locked",
                "--release",
                "--no-default-features",
                "--bin",
                "ecbench",
                "--example",
                "ecbench_aimed2lp_analyze",
                "--example",
                "ecbench_aimed2lp_preflight"
            ])
        || image["rustflags"] != "-C target-cpu=x86-64"
        || image["cargo_build_jobs"] != 1
        || protocol["extensions"]["native_analysis"]["build"] != image["build_command"]
        || protocol["extensions"]["native_analysis"]["cargo_build_jobs"] != 1
    {
        return Err("immutable image receipt has an unbound build or digest identity".into());
    }
    let script = &protocol["analysis_plan"]["script"];
    let protocol_source_path = base.join(
        script["path"]
            .as_str()
            .ok_or("native analysis source path missing")?,
    );
    let manifest_source_path = base.join(&manifest.native_analyzer.source);
    if fs::canonicalize(&protocol_source_path).map_err(|e| e.to_string())?
        != fs::canonicalize(&manifest_source_path).map_err(|e| e.to_string())?
    {
        return Err("manifest and protocol name different native analyzer sources".into());
    }
    let extension_source_path = base.join("../..").join(
        protocol["extensions"]["native_analysis"]["source_path"]
            .as_str()
            .ok_or("native-analysis extension source path missing")?,
    );
    if fs::canonicalize(&protocol_source_path).map_err(|error| error.to_string())?
        != fs::canonicalize(&extension_source_path).map_err(|error| error.to_string())?
    {
        return Err("analysis plan and native-analysis extension name different sources".into());
    }
    let source_sha256 = sha256_hex(&bytes(&protocol_source_path)?);
    if script["sha256"] != source_sha256
        || protocol["extensions"]["native_analysis"]["source_sha256"] != source_sha256
        || protocol["extensions"]["native_analyzer_source_sha256"] != source_sha256
        || manifest.native_analyzer.source_sha256 != source_sha256
    {
        return Err("native analysis source differs from the frozen protocol".into());
    }
    if image["native_analyzer_source_sha256"] != source_sha256 {
        return Err("image does not bind the native analyzer source".into());
    }
    let analysis_binary_sha256 = sha256_hex(&bytes(
        &std::env::current_exe().map_err(|e| format!("current analyzer: {e}"))?,
    )?);
    if protocol["extensions"]["native_analyzer_binary_sha256"] != analysis_binary_sha256
        || protocol["extensions"]["native_analysis"]["binary_sha256"]
            != analysis_binary_sha256
        || manifest.native_analyzer.binary_sha256 != analysis_binary_sha256
        || image["native_analyzer_binary_sha256"] != analysis_binary_sha256
    {
        return Err("native analyzer binary differs from protocol/image identities".into());
    }
    let preflight_source_path = base.join(&manifest.native_provider_preflight.source);
    let protocol_preflight_source_path = base.join("../..").join(
        protocol["extensions"]["provider_preflight"]["source_path"]
            .as_str()
            .ok_or("provider-preflight source path missing")?,
    );
    if fs::canonicalize(&preflight_source_path).map_err(|error| error.to_string())?
        != fs::canonicalize(&protocol_preflight_source_path)
            .map_err(|error| error.to_string())?
    {
        return Err("manifest and protocol name different provider-preflight sources".into());
    }
    let provider_preflight_source_sha256 = sha256_hex(&bytes(&preflight_source_path)?);
    if manifest.native_provider_preflight.source_sha256 != provider_preflight_source_sha256
        || protocol["extensions"]["provider_preflight_source_sha256"]
            != provider_preflight_source_sha256
        || protocol["extensions"]["provider_preflight"]["source_sha256"]
            != provider_preflight_source_sha256
        || image["native_provider_preflight_source_sha256"]
            != provider_preflight_source_sha256
    {
        return Err("native provider-preflight source differs from the frozen protocol".into());
    }
    let preflight_binary_path = base.join("../..").join(
        protocol["extensions"]["provider_preflight"]["binary_path"]
            .as_str()
            .ok_or("provider-preflight binary path missing")?,
    );
    let provider_preflight_binary_sha256 = sha256_hex(&bytes(&preflight_binary_path)?);
    if manifest.native_provider_preflight.binary_sha256 != provider_preflight_binary_sha256
        || protocol["extensions"]["provider_preflight_binary_sha256"]
            != provider_preflight_binary_sha256
        || protocol["extensions"]["provider_preflight"]["binary_sha256"]
            != provider_preflight_binary_sha256
        || protocol["extensions"]["provider_preflight"]["binary_sha256"]
            != provider_preflight_binary_sha256
        || image["native_provider_preflight_binary_sha256"]
            != provider_preflight_binary_sha256
    {
        return Err("native provider-preflight binary differs from protocol/image identities".into());
    }
    let self_test = json_value(&base.join(&manifest.native_analyzer.self_test_receipt))?;
    exact_keys(
        &self_test,
        &[
            "schema", "created", "status", "source_sha256", "binary_sha256",
            "analysis_manifest_sha256", "result_schema_sha256", "receipt_schema_sha256",
            "request_schema_sha256", "command", "checks",
        ],
        "native analyzer self-test receipt",
    )?;
    if self_test["schema"] != "icb.aimed2lp.native_analyzer_tests/v1"
        || self_test["status"] != "pass"
        || self_test["source_sha256"] != source_sha256
        || self_test["binary_sha256"] != analysis_binary_sha256
        || self_test["analysis_manifest_sha256"] != sha256_hex(&bytes(manifest_path)?)
        || self_test["result_schema_sha256"]
            != sha256_hex(&bytes(&base.join(&manifest.result_schema))?)
        || self_test["receipt_schema_sha256"]
            != sha256_hex(&bytes(&base.join(&manifest.receipt_schema))?)
        || self_test["request_schema_sha256"]
            != sha256_hex(&bytes(&base.join(&manifest.request_schema))?)
    {
        return Err("native analyzer self-test receipt is absent, stale, or failing".into());
    }
    let self_test_command = self_test["command"]
        .as_array()
        .filter(|command| {
            !command.is_empty()
                && command.iter().all(|arg| arg.as_str().is_some())
                && command.last().and_then(Value::as_str) == Some("--self-test")
        })
        .ok_or("native analyzer self-test command is malformed")?;
    if self_test_command.len() < 2 {
        return Err("native analyzer self-test command does not name its binary".into());
    }
    let checks = self_test["checks"]
        .as_array()
        .filter(|checks| !checks.is_empty())
        .ok_or("native analyzer self-test checks are absent")?;
    let mut check_ids = BTreeSet::new();
    for check in checks {
        exact_keys(check, &["id", "outcome"], "native analyzer self-test check")?;
        let id = check["id"].as_str().filter(|id| !id.is_empty());
        if check["outcome"] != "pass" || id.is_none_or(|id| !check_ids.insert(id)) {
            return Err("native analyzer self-test check is duplicate or failing".into());
        }
    }
    for required in SELF_TEST_CHECKS {
        if !check_ids.contains(required) {
            return Err(format!("native analyzer self-test receipt omits {required}"));
        }
    }
    if focused["source_commit"] != source_commit
        || image["focused_gate_receipt_sha256"] != sha256_hex(&bytes(&focused_path)?)
        || focused["target_data"]["n53_runs"] != 0
        || focused["target_data"]["n61_runs"] != 0
    {
        return Err("focused receipt, source commit, and image are not bound".into());
    }
    let focused_checks = focused["checks"]
        .as_array()
        .filter(|checks| !checks.is_empty())
        .ok_or("focused gate receipt has no checks")?;
    if focused_checks.iter().any(|check| check["outcome"] != "pass") {
        return Err("focused gate receipt contains a non-passing check".into());
    }
    let focused_names = focused_checks
        .iter()
        .flat_map(|check| [check["id"].as_str(), check["test"].as_str()])
        .flatten()
        .collect::<Vec<_>>()
        .join("\n");
    for required in [
        "projected_large_prime_rows_map_to_the_frozen_log_columns_and_fail_closed",
        "aimed_lp_adapter_preserves_the_frozen_ordinary_three_sum_opportunities",
        "lp_zero_is_the_frozen_pipeline_and_two_lp_recovers_a_toy_target",
        "aimed_lp_report_control_and_native_ownership_are_exact",
        "exported_aimed_window_indices_are_the_collectors_exact_scan",
        "aimed_decomposition_scans_only_the_named_third_summands",
    ] {
        if !focused_names.contains(required) {
            return Err(format!("focused gate receipt omits {required}"));
        }
    }
    let extensions = &protocol["extensions"];
    for (relative, field) in [
        (&manifest.result_schema, "result_schema_sha256"),
        (&manifest.receipt_schema, "receipt_schema_sha256"),
        (&manifest.request_schema, "request_schema_sha256"),
    ] {
        let digest = sha256_hex(&bytes(&base.join(relative))?);
        if extensions[field] != digest || image[field] != digest {
            return Err(format!("{relative} differs from frozen {field}"));
        }
    }
    let result_contract = json_value(&base.join(&manifest.result_schema))?;
    if result_contract["$id"] != RESULT_SCHEMA
        || result_contract["properties"]["schema"]["const"] != RESULT_SCHEMA
    {
        return Err("bound result schema is not the native v2 result contract".into());
    }
    if extensions["analysis_manifest_sha256"] != sha256_hex(&bytes(manifest_path)?) {
        return Err("analysis manifest differs from the frozen protocol".into());
    }
    let provider_preflight = json_value(&base.join(&manifest.provider_preflight))?;
    exact_keys(
        &provider_preflight,
        &[
            "schema", "created", "status", "source_sha256", "binary_sha256", "wrapper",
            "compare_implementation", "request", "specs", "gate",
        ],
        "provider preflight receipt",
    )?;
    if provider_preflight["schema"] != "icb.aimed2lp.provider_preflight/v1"
        || provider_preflight["status"] != "pass"
        || provider_preflight["created"].as_str().is_none_or(str::is_empty)
        || provider_preflight["gate"].as_str().is_none_or(str::is_empty)
        || provider_preflight["source_sha256"] != provider_preflight_source_sha256
        || provider_preflight["binary_sha256"] != provider_preflight_binary_sha256
    {
        return Err("native provider topology preflight is absent or failing".into());
    }
    for field in ["wrapper", "compare_implementation"] {
        exact_keys(
            &provider_preflight[field],
            &["path", "sha256"],
            &format!("provider preflight {field}"),
        )?;
        let relative = provider_preflight[field]["path"]
            .as_str()
            .ok_or_else(|| format!("provider preflight {field} path is absent"))?;
        let source_path = base.join("../..").join(relative);
        if provider_preflight[field]["sha256"] != sha256_hex(&bytes(&source_path)?) {
            return Err(format!("provider preflight {field} source binding is stale"));
        }
    }
    for (key, expected) in manifest
        .provider_request
        .as_object()
        .ok_or("manifest provider_request is not an object")?
    {
        if provider_preflight["request"][key.as_str()] != expected.clone() {
            return Err(format!("provider preflight request drifts at {key}"));
        }
    }
    exact_keys(
        &provider_preflight["request"],
        &[
            "instance_type",
            "capacity",
            "node_selector",
            "exclusive_vcpus",
            "cpu_request",
            "cpu_limit",
            "memory",
            "memory_bytes",
            "memory_request",
            "memory_limit",
            "restart_policy",
            "child_timeout_max_seconds",
            "outer_job_deadline_seconds",
            "estimated_session_seconds",
            "artifact_upload_grace_seconds",
            "provider_retries",
            "scientific_retries",
            "infrastructure_attempts_max",
        ],
        "provider preflight request",
    )?;
    exact_keys(
        &provider_preflight["request"]["node_selector"],
        &["node.kubernetes.io/instance-type", "eks.amazonaws.com/capacityType"],
        "provider preflight node selector",
    )?;
    if provider_preflight["request"]["node_selector"]["node.kubernetes.io/instance-type"]
        != "c8a.2xlarge"
        || provider_preflight["request"]["node_selector"]["eks.amazonaws.com/capacityType"]
            != "SPOT"
        || provider_preflight["request"]["cpu_request"] != "4"
        || provider_preflight["request"]["cpu_limit"] != "4"
        || provider_preflight["request"]["memory_request"] != "8Gi"
        || provider_preflight["request"]["memory_limit"] != "8Gi"
        || provider_preflight["request"]["restart_policy"] != "Never"
        || provider_preflight["request"]["provider_retries"] != 0
        || provider_preflight["request"]["scientific_retries"] != 0
        || provider_preflight["request"]["infrastructure_attempts_max"] != 2
    {
        return Err("provider preflight pod/resource contract drifts".into());
    }
    let preflight_specs = provider_preflight["specs"]
        .as_array()
        .filter(|specs| specs.len() == 3)
        .ok_or("provider preflight must bind exactly three specs")?;
    let mut preflight_cells = BTreeSet::new();
    for spec in preflight_specs {
        exact_keys(
            spec,
            &[
                "path", "file_sha256", "spec_id", "spec_sha256", "request_template_path",
                "request_template_sha256", "payload_sha256", "payload_bytes", "argv",
                "provider_identity_sha256", "pod_spec_sha256", "durable_prefix",
                "first_reference", "first_control", "aa_denominator", "aa_numerator",
                "aa_selection", "scheduled_aa_pairs", "measured_rows", "rounds", "warmup",
                "targets", "child_timeout_seconds", "isolation_required",
            ],
            "provider preflight spec",
        )?;
        let path = spec["path"]
            .as_str()
            .ok_or("provider preflight spec path is absent")?;
        let cell = path
            .strip_prefix("research/icb_aimed_large_prime_n53_n61_20261101/specs/")
            .and_then(|name| name.strip_suffix(".json"))
            .ok_or("provider preflight spec path is outside the study")?;
        if !matches!(cell, "control-n13" | "n53" | "n61")
            || !preflight_cells.insert(cell)
        {
            return Err("provider preflight repeats or invents a cell".into());
        }
        let template_relative = manifest
            .request_templates
            .get(cell)
            .ok_or("provider preflight cell has no request template")?;
        let template = json_value(&base.join(template_relative))?;
        let spec_receipt = &image["specs"][path];
        let expected_request_path = format!(
            "research/icb_aimed_large_prime_n53_n61_20261101/{template_relative}"
        );
        if spec["file_sha256"] != spec_receipt["file_sha256"]
            || spec["spec_id"] != spec_receipt["spec_id"]
            || spec["spec_sha256"] != spec_receipt["identity_sha256"]
            || spec["request_template_path"] != expected_request_path
            || spec["request_template_sha256"] != sha256_hex(&bytes(&base.join(template_relative))?)
            || spec["payload_sha256"] != template["payload_sha256"]
            || spec["payload_bytes"] != template["payload_bytes"]
            || spec["argv"] != template["argv"]
            || spec["provider_identity_sha256"] != template["provider_identity_sha256"]
            || spec["pod_spec_sha256"] != template["pod_spec_sha256"]
            || spec["durable_prefix"] != template["durable_prefix"]
            || spec["first_reference"]
                != if cell == "control-n13" { "aimed" } else { "rho-v2" }
            || spec["first_control"]
                != if cell == "control-n13" { "aimed-lp0" } else { "aimed-aa" }
            || spec["aa_denominator"] != "aimed"
            || spec["aa_numerator"] != "aimed-aa"
            || spec["aa_selection"].as_str().is_none_or(str::is_empty)
            || spec["scheduled_aa_pairs"] != if cell == "control-n13" { 10 } else { 40 }
            || spec["measured_rows"] != if cell == "control-n13" { 50 } else { 240 }
            || spec["rounds"] != 5
            || spec["warmup"] != 1
            || spec["targets"] != if cell == "control-n13" { 2 } else { 8 }
            || spec["child_timeout_seconds"]
                != if cell == "control-n13" { 600 } else { 3_600 }
            || spec["isolation_required"] != "L2"
        {
            return Err(format!("provider preflight binding drifts for {cell}"));
        }
    }
    let checkpoint = json_value(&base.join(&manifest.checkpoint_receipt))?;
    exact_keys(
        &checkpoint,
        &[
            "schema", "created", "status", "source_commit", "protocol_sha256",
            "filed_before_affected_data", "resume_sha256", "focused_gate_receipt_sha256",
            "analysis_manifest_sha256", "result_schema_sha256", "receipt_schema_sha256",
            "request_schema_sha256", "request_templates", "native_analyzer_source_sha256",
            "native_analyzer_binary_sha256", "native_provider_preflight_source_sha256",
            "native_provider_preflight_binary_sha256", "provider_preflight_receipt_sha256",
            "target_rows",
        ],
        "prospective checkpoint receipt",
    )?;
    exact_keys(
        &checkpoint["request_templates"],
        &["control-n13", "n53", "n61"],
        "checkpoint request-template bindings",
    )?;
    if checkpoint["schema"] != "icb.aimed2lp.checkpoint/v1"
        || checkpoint["status"] != "pass"
        || checkpoint["created"].as_str().is_none_or(str::is_empty)
        || checkpoint["protocol_sha256"] != protocol_sha256
        || checkpoint["source_commit"] != source_commit
        || checkpoint["filed_before_affected_data"] != true
        || checkpoint["focused_gate_receipt_sha256"] != sha256_hex(&bytes(&focused_path)?)
        || checkpoint["analysis_manifest_sha256"] != sha256_hex(&bytes(manifest_path)?)
        || checkpoint["result_schema_sha256"]
            != sha256_hex(&bytes(&base.join(&manifest.result_schema))?)
        || checkpoint["receipt_schema_sha256"]
            != sha256_hex(&bytes(&base.join(&manifest.receipt_schema))?)
        || checkpoint["request_schema_sha256"]
            != sha256_hex(&bytes(&base.join(&manifest.request_schema))?)
        || checkpoint["resume_sha256"]
            != sha256_hex(&bytes(&base.join("../../.resume/RESUME.md"))?)
        || checkpoint["native_analyzer_source_sha256"] != source_sha256
        || checkpoint["native_analyzer_binary_sha256"] != analysis_binary_sha256
        || checkpoint["native_provider_preflight_source_sha256"]
            != provider_preflight_source_sha256
        || checkpoint["native_provider_preflight_binary_sha256"]
            != provider_preflight_binary_sha256
        || checkpoint["native_provider_preflight_source_sha256"]
            != provider_preflight_source_sha256
        || checkpoint["native_provider_preflight_binary_sha256"]
            != provider_preflight_binary_sha256
        || checkpoint["provider_preflight_receipt_sha256"]
            != sha256_hex(&bytes(&base.join(&manifest.provider_preflight))?)
        || checkpoint["target_rows"] != 0
        || image["checkpoint_receipt_sha256"]
            != sha256_hex(&bytes(&base.join(&manifest.checkpoint_receipt))?)
    {
        return Err("prospective checkpoint receipt is absent, stale, or post-data".into());
    }
    for (cell, relative) in &manifest.request_templates {
        if checkpoint["request_templates"][cell.as_str()]
            != sha256_hex(&bytes(&base.join(relative))?)
        {
            return Err(format!("checkpoint does not bind the {cell} request template"));
        }
    }
    for arm in protocol["arms"].as_array().ok_or("protocol arms missing")? {
        if arm["instrument"]["commit"] != source_commit
            || arm["instrument"]["binary_sha256"] != binary_sha256
            || arm["instrument"]["image_digest"] != image_digest
        {
            return Err(format!("arm {} has identity drift", arm["id"]));
        }
    }
    Ok(Identity {
        protocol_id,
        protocol_sha256,
        source_commit,
        binary_sha256,
        image_digest,
        analysis_source_sha256: source_sha256,
        analysis_binary_sha256,
        provider_preflight_source_sha256,
        provider_preflight_binary_sha256,
        image,
        protocol,
    })
}

fn expected_spec_path(n: u32) -> String {
    if n == 13 {
        "research/icb_aimed_large_prime_n53_n61_20261101/specs/control-n13.json".into()
    } else {
        format!("research/icb_aimed_large_prime_n53_n61_20261101/specs/n{n}.json")
    }
}

fn verified(record: &Record) -> bool {
    record.outcome.status == "verified" && record.outcome.matches_target == Some(true)
}

fn classify_records(records: &[Record]) -> Result<TerminalClass, String> {
    let mut class = TerminalClass::Complete;
    for record in records.iter().filter(|record| !record.warmup) {
        match record.outcome.status.as_str() {
            "verified" if record.outcome.matches_target == Some(true) => {}
            "wrong_answer" | "verified" => {
                return Err(format!(
                    "{} has a correctness failure ({})",
                    record.run_id, record.outcome.status
                ));
            }
            "exhausted" | "timeout" if class == TerminalClass::Complete => {
                class = TerminalClass::ScientificCensored
            }
            "exhausted" | "timeout" => {}
            "error" | "crashed" => class = TerminalClass::InfrastructureBlocked,
            other => return Err(format!("{} has unknown outcome {other}", record.run_id)),
        }
    }
    Ok(class)
}

fn checked_audit(path: &Path, session: &Session, measured: usize) -> Result<AuditReport, String> {
    let audit: AuditReport = read_typed(path)?;
    if audit.schema != AUDIT_SCHEMA
        || !audit.ok
        || audit.session_id != session.session_id
        || audit.session_status != "complete"
        || audit.records_bound != Some(true)
        || audit.records != session.records_written
        || audit.planned_executions != session.executions_planned
        || audit.replay_selection.as_deref() != Some("all")
        || audit.replay_candidates != Some(audit.replays.len() as u64)
        || audit.replays.len() > measured
        || audit.replays.iter().any(|replay| !replay.reproduced)
        || !audit.problems.is_empty()
        || !audit.regraded
    {
        return Err(format!("{} is not a complete replay-all audit", path.display()));
    }
    Ok(audit)
}

fn load_bundle(
    base: &Path,
    session_rel: &str,
    audit_rel: &str,
    identity: &Identity,
    n: u32,
    expected_records: usize,
    expected_measured: usize,
) -> Result<Bundle, String> {
    let dir = base.join(session_rel);
    let session = read_session(&dir)?;
    if session.status != "complete"
        || session.executions_planned != expected_records as u64
        || session.records_written != expected_records as u64
        || session.git_commit.as_deref() != Some(&identity.source_commit)
        || session.binary_sha256.as_deref() != Some(&identity.binary_sha256)
    {
        return Err(format!("{} is not the expected complete frozen session", dir.display()));
    }
    let spec_path = expected_spec_path(n);
    let spec_receipt = &identity.image["specs"][spec_path.as_str()];
    if spec_receipt["identity_sha256"] != session.spec_sha256
        || spec_receipt["spec_id"] != session.spec_id
    {
        return Err(format!("{} does not use the image-bound n={n} spec", dir.display()));
    }
    let records = read_records(&dir)?;
    if records.len() != expected_records
        || records.iter().filter(|record| !record.warmup).count() != expected_measured
    {
        return Err(format!(
            "{} has {} rows, expected {expected_records} with {expected_measured} measured",
            dir.display(),
            records.len()
        ));
    }
    for record in &records {
        if record.session_id != session.session_id
            || record.binary_sha256.as_deref() != Some(&identity.binary_sha256)
            || record.isolation.level < Level::L2
        {
            return Err(format!("{} has identity or L2 drift", record.run_id));
        }
    }
    let audit = checked_audit(&base.join(audit_rel), &session, expected_measured)?;
    if records.iter().filter(|record| !record.warmup).all(verified)
        && audit.replays.len() != expected_measured
    {
        return Err(format!("{} did not replay every eligible measured row", dir.display()));
    }
    Ok(Bundle {
        cell: if n == 13 { "control-n13".into() } else { format!("n{n}") },
        dir,
        session,
        records,
        audit: Some(audit),
    })
}

fn load_retained_bundle(
    base: &Path,
    session_rel: &str,
    audit_rel: &str,
    identity: &Identity,
    n: u32,
    expected_records: usize,
) -> Result<Option<Bundle>, String> {
    let dir = base.join(session_rel);
    if !dir.exists() {
        return Ok(None);
    }
    let session = read_session(&dir)?;
    if session.executions_planned != expected_records as u64
        || session.git_commit.as_deref() != Some(&identity.source_commit)
        || session.binary_sha256.as_deref() != Some(&identity.binary_sha256)
    {
        return Err(format!("{} retained session has identity drift", dir.display()));
    }
    let spec_path = expected_spec_path(n);
    let spec_receipt = &identity.image["specs"][spec_path.as_str()];
    if spec_receipt["identity_sha256"] != session.spec_sha256
        || spec_receipt["spec_id"] != session.spec_id
    {
        return Err(format!("{} retained session has spec drift", dir.display()));
    }
    let records = read_records(&dir)?;
    if records.len() > expected_records || session.records_written != records.len() as u64 {
        return Err(format!("{} retained rows disagree with session flow", dir.display()));
    }
    for record in &records {
        if record.session_id != session.session_id
            || record.binary_sha256.as_deref() != Some(&identity.binary_sha256)
        {
            return Err(format!("{} retained row has identity drift", record.run_id));
        }
        if matches!(record.outcome.status.as_str(), "wrong_answer" | "verified")
            && !verified(record)
        {
            return Err(format!("{} retained row is a correctness failure", record.run_id));
        }
    }
    let audit_path = base.join(audit_rel);
    let audit = if audit_path.exists() {
        let audit: AuditReport = read_typed(&audit_path)?;
        if audit.schema != AUDIT_SCHEMA
            || audit.session_id != session.session_id
            || audit.replays.iter().any(|replay| !replay.reproduced)
        {
            return Err(format!("{} retained audit has identity/replay failure", audit_path.display()));
        }
        Some(audit)
    } else {
        None
    };
    Ok(Some(Bundle {
        cell: if n == 13 { "control-n13".into() } else { format!("n{n}") },
        dir,
        session,
        records,
        audit,
    }))
}

fn validate_retained_terminal_shape(
    session_status: &str,
    planned: u64,
    written: u64,
    stop_kind: Option<&str>,
    stop_seq: Option<u64>,
    stop_message: Option<&str>,
    observed: TerminalClass,
    declared: TerminalClass,
) -> Result<(), String> {
    if matches!(declared, TerminalClass::Complete | TerminalClass::NotAttempted) {
        return Err("retained noncomplete validation received an inapplicable class".into());
    }
    match session_status {
        "complete" => {
            if written != planned
                || stop_kind.is_some()
                || stop_seq.is_some()
                || stop_message.is_some()
                || observed != declared
            {
                return Err(
                    "complete retained session does not substantiate its noncomplete terminal class"
                        .into(),
                );
            }
        }
        "interrupted" => {
            let expected_stop_seq = match stop_kind {
                Some("interrupt" | "spawn_error") => Some(written),
                Some("error") if written == 0 => None,
                Some("error") => Some(written - 1),
                _ => return Err("interrupted retained session has an unknown stop kind".into()),
            };
            if written >= planned
                || stop_seq != expected_stop_seq
                || stop_message.is_none_or(str::is_empty)
                || (observed != declared
                    && !(observed == TerminalClass::Complete
                        && declared == TerminalClass::InfrastructureBlocked))
            {
                return Err(
                    "interrupted retained session does not substantiate its terminal class".into(),
                );
            }
        }
        _ => return Err("retained session is neither complete nor interrupted".into()),
    }
    Ok(())
}

fn validate_retained_terminal(
    bundle: &Bundle,
    declared: TerminalClass,
) -> Result<(), String> {
    let stop = bundle.session.stop.as_ref();
    validate_retained_terminal_shape(
        &bundle.session.status,
        bundle.session.executions_planned,
        bundle.session.records_written,
        stop.map(|stop| stop.kind.as_str()),
        stop.and_then(|stop| stop.seq),
        stop.map(|stop| stop.message.as_str()),
        classify_records(&bundle.records)?,
        declared,
    )
}

type Blocks<'a> = BTreeMap<(String, u32), BTreeMap<String, &'a Record>>;

fn paired_blocks<'a>(
    records: &'a [Record],
    expected_arms: &[&str],
    expected_targets: usize,
) -> Result<Blocks<'a>, String> {
    let mut blocks = Blocks::new();
    for record in records.iter().filter(|record| !record.warmup) {
        let key = (record.workload.workload_id.clone(), record.round);
        if blocks
            .entry(key.clone())
            .or_default()
            .insert(record.arm.clone(), record)
            .is_some()
        {
            return Err(format!("duplicate {} round {} arm {}", key.0, key.1, record.arm));
        }
    }
    if blocks.len() != expected_targets * ROUNDS {
        return Err(format!(
            "found {} measured target/round blocks, expected {}",
            blocks.len(),
            expected_targets * ROUNDS
        ));
    }
    let expected: BTreeSet<&str> = expected_arms.iter().copied().collect();
    for ((workload, round), arms) in &blocks {
        if arms.keys().map(String::as_str).collect::<BTreeSet<_>>() != expected {
            return Err(format!("{workload} round {round} lacks a frozen arm"));
        }
        let first = arms.values().next().ok_or("empty arm block")?;
        for record in arms.values() {
            if record.workload.target != first.workload.target
                || record.algorithm_seed != first.algorithm_seed
                || record.workload.target_index != first.workload.target_index
            {
                return Err(format!("{workload} round {round} is not exactly paired"));
            }
            if verified(first)
                && verified(record)
                && record.outcome.recovered != first.outcome.recovered
            {
                return Err(format!("{workload} round {round} verified answers disagree"));
            }
        }
    }
    Ok(blocks)
}

fn remove_clocks(value: &mut Value) {
    match value {
        Value::Object(fields) => {
            for key in ["wall_ns", "solve_wall_ns", "method_call_wall_ns", "phases_ns"] {
                fields.remove(key);
            }
            for child in fields.values_mut() {
                remove_clocks(child);
            }
        }
        Value::Array(items) => {
            for child in items {
                remove_clocks(child);
            }
        }
        _ => {}
    }
}

fn operation_signature(record: &Record) -> Value {
    let mut signature = json!({
        "recovered": record.outcome.recovered,
        "total_gae": record.cost.total_gae,
        "unpriced": record.cost.unpriced,
        "phases": record.phases.iter().map(|phase| json!({
            "name": phase.name,
            "adds": phase.adds,
            "doubles": phase.doubles,
            "scalar_mults": phase.scalar_mults,
            "gae": phase.gae,
            "native": phase.native,
            "work_native": phase.work_native,
        })).collect::<Vec<_>>(),
        "counters": record.counters,
        "factor_base": record.factor_base,
        "field_ops": record.field_ops,
        "detail": record.detail,
    });
    remove_clocks(&mut signature);
    signature
}

fn exact_pair_gate(blocks: &Blocks<'_>, a: &str, b: &str) -> Result<usize, String> {
    let mut pairs = 0usize;
    for ((workload, round), arms) in blocks {
        let (ra, rb) = (arms[a], arms[b]);
        if !verified(ra) || !verified(rb) {
            continue;
        }
        if operation_signature(ra) != operation_signature(rb) {
            return Err(format!("{a}/{b} operations differ at {workload} round {round}"));
        }
        pairs += 1;
    }
    Ok(pairs)
}

fn lp_counter_gate(
    run_id: &str,
    counters: &BTreeMap<String, u64>,
    require_activity: bool,
) -> Result<(), String> {
    let counter = |name: &str| counters.get(name).copied();
    if counter("aimed_lp_direct_two_sum") != Some(0)
        || counter("aimed_lp_verification_failures") != Some(0)
    {
        return Err(format!("{run_id} violates the fail-closed LP invariant"));
    }
    if require_activity {
        let partial_trials = counter("aimed_lp_partial_scan_trials").unwrap_or(0);
        let partial_summands = counter("aimed_lp_partial_summands_scanned").unwrap_or(0);
        let total_summands = counter("summands_scanned").unwrap_or(0);
        if counter("aimed_lp_partials_fed").unwrap_or(0) == 0
            || counter("lp_projected_relations").unwrap_or(0) == 0
            || counter("aimed_lp_merged_relations").unwrap_or(0) == 0
            || partial_trials == 0
            || partial_summands == 0
            || total_summands < partial_summands
        {
            return Err(format!("{run_id} lacks active merged 2LP flow"));
        }
    }
    Ok(())
}

fn lp_gate(records: &[Record], require_activity: bool) -> Result<(), String> {
    for record in records.iter().filter(|record| {
        !record.warmup && matches!(record.arm.as_str(), "aimed-lp2-c05" | "aimed-lp2-c1")
    }) {
        lp_counter_gate(
            &record.run_id,
            &record.counters,
            require_activity && verified(record),
        )?;
    }
    Ok(())
}

fn solve_wall(record: &Record) -> Option<f64> {
    record
        .time
        .solve_wall_ns
        .filter(|value| *value > 0)
        .map(|value| value as f64)
}

fn target_clusters(
    blocks: &Blocks<'_>,
    a: &str,
    b: &str,
    value: impl Fn(&Record) -> Option<f64>,
) -> Result<Vec<(f64, f64)>, String> {
    let mut by_target = BTreeMap::<String, Vec<(f64, f64)>>::new();
    for ((workload, _round), arms) in blocks {
        let (ra, rb) = (arms[a], arms[b]);
        if !verified(ra) || !verified(rb) {
            continue;
        }
        let (Some(x), Some(y)) = (value(ra), value(rb)) else {
            return Err(format!("{workload} lacks a finite {a}/{b} measurement"));
        };
        by_target.entry(workload.clone()).or_default().push((x, y));
    }
    if by_target.len() != TARGETS
        || by_target.values().any(|rounds| rounds.len() != ROUNDS)
    {
        return Err(format!(
            "{a}/{b} has {} complete target clusters, expected {TARGETS} x {ROUNDS}",
            by_target.len()
        ));
    }
    Ok(by_target
        .values()
        .map(|rounds| {
            rounds
                .iter()
                .fold((0.0, 0.0), |(x, y), (a, b)| (x + a, y + b))
        })
        .collect())
}

/// Exact paired target-level sign-flip test for H0: B/A >= `threshold`.
fn sign_flip_p(clusters: &[(f64, f64)], threshold: f64) -> Option<f64> {
    if clusters.is_empty() || clusters.len() >= usize::BITS as usize {
        return None;
    }
    let differences: Vec<f64> = clusters
        .iter()
        .map(|(a, b)| b - threshold * a)
        .collect();
    let observed: f64 = differences.iter().sum();
    let total = 1usize << differences.len();
    let tolerance = 1e-12 * observed.abs().max(1.0);
    let favorable = (0..total)
        .filter(|mask| {
            let permuted: f64 = differences
                .iter()
                .enumerate()
                .map(|(index, difference)| {
                    if mask & (1usize << index) == 0 {
                        *difference
                    } else {
                        -*difference
                    }
                })
                .sum();
            permuted <= observed + tolerance
        })
        .count();
    Some(favorable as f64 / total as f64)
}

fn estimate(clusters: &[(f64, f64)], threshold: f64, seed: u64) -> Result<Estimate, String> {
    if clusters.len() != TARGETS {
        return Err(format!("inference needs exactly {TARGETS} target clusters"));
    }
    let point = ratio(clusters).ok_or("ratio is undefined")?;
    let strata: Vec<Vec<(f64, f64)>> = clusters.iter().map(|pair| vec![*pair]).collect();
    let ci95 = cluster_bootstrap_ci(&strata, RESAMPLES, seed, |sample| {
        let flat: Vec<(f64, f64)> = sample.iter().flatten().copied().collect();
        ratio(&flat)
    })
    .ok_or("target-cluster bootstrap interval is undefined")?;
    let raw_p = sign_flip_p(clusters, threshold).ok_or("sign-flip test is undefined")?;
    Ok(Estimate {
        point,
        ci95,
        raw_p,
        adjusted_p: f64::NAN,
        clusters: clusters.len(),
    })
}

fn holm(estimates: &mut BTreeMap<(u32, String), Estimate>, family_size: usize) {
    let mut ordered: Vec<((u32, String), f64)> = estimates
        .iter()
        .map(|(key, estimate)| (key.clone(), estimate.raw_p))
        .collect();
    ordered.sort_by(|left, right| left.1.total_cmp(&right.1));
    let mut prior: f64 = 0.0;
    assert!(ordered.len() <= family_size);
    for (index, (key, p)) in ordered.into_iter().enumerate() {
        let adjusted = (p * (family_size - index) as f64).min(1.0).max(prior);
        prior = adjusted;
        estimates.get_mut(&key).unwrap().adjusted_p = adjusted;
    }
}

fn native_count(record: &Record, weight: f64) -> Option<f64> {
    let gae = record.cost.total_gae?;
    if weight == 0.0 {
        return Some(gae);
    }
    let counts = unpriced_counts(record);
    let native = counts
        .values()
        .try_fold(0u128, |sum, count| Some(sum + u128::from((*count)?)))?;
    Some(gae + weight * native as f64)
}

fn pricing_band(blocks: &Blocks<'_>, a: &str, b: &str, seed: u64) -> Value {
    let mut ratios = Vec::new();
    let mut intervals = Vec::new();
    let mut missing = Vec::new();
    for (index, weight) in PRICES.into_iter().enumerate() {
        let clusters = target_clusters(blocks, a, b, |record| native_count(record, weight));
        match clusters.and_then(|clusters| estimate(&clusters, 1.0, seed + index as u64)) {
            Ok(row) => {
                ratios.push(json!(row.point));
                intervals.push(json!([row.ci95.0, row.ci95.1]));
                missing.push(false);
            }
            Err(_) => {
                ratios.push(Value::Null);
                intervals.push(Value::Null);
                missing.push(true);
            }
        }
    }
    let status = if missing.iter().skip(1).any(|value| *value) {
        "UNRANKABLE PRICING"
    } else {
        "DESCRIPTIVE COMPLETE"
    };
    let missing_units = blocks
        .values()
        .flat_map(|arms| [arms[a], arms[b]])
        .flat_map(|record| unpriced_counts(record).into_iter())
        .filter_map(|(name, count)| count.is_none().then_some(name))
        .collect::<BTreeSet<_>>();
    json!({
        "status": status,
        "role": "descriptive_not_decisive",
        "weights": PRICES,
        "ratios": ratios,
        "ci95": intervals,
        "missing_native_count": missing,
        "missing_unpriced_units": missing_units,
        "note": "This band does not alter the finite-host whole-wall verdict. A null weight is never interpreted as zero work."
    })
}

fn compare_saved(path: &Path, bundle: &Bundle, a: &str, b: &str) -> Result<Comparison, String> {
    let saved: Comparison = read_typed(path)?;
    let options = CompareOptions {
        resamples: saved.ops.resamples,
        seed: saved
            .resample_seed
            .ok_or("saved comparison lacks resample seed")?,
        allow_interrupted: false,
    };
    let fresh = compare_loaded(
        &bundle.session,
        &bundle.records,
        &bundle.session,
        &bundle.records,
        a,
        b,
        Level::L2,
        &options,
    )?;
    if serde_json::to_value(&fresh).map_err(|e| e.to_string())?
        != serde_json::to_value(&saved).map_err(|e| e.to_string())?
    {
        return Err(format!("{} differs from native recomputation", path.display()));
    }
    Ok(saved)
}

fn canonical_sha256(value: &Value) -> Result<String, String> {
    Ok(sha256_hex(&protocol_canonical_bytes(value)?))
}

fn lowercase_hex(text: &str) -> bool {
    text.bytes()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn sha256_string(value: &Value) -> Option<&str> {
    value
        .as_str()
        .filter(|hash| {
            hash.len() == 64 && lowercase_hex(hash)
        })
}

fn valid_argv(value: &Value) -> bool {
    value.as_array().is_some_and(|argv| {
        !argv.is_empty()
            && argv
                .iter()
                .all(|arg| arg.as_str().is_some_and(|arg| !arg.is_empty()))
    })
}

fn local_artifact(base: &Path, relative: &str) -> Result<PathBuf, String> {
    use std::path::Component;
    let path = Path::new(relative);
    if path.is_absolute()
        || path.components().any(|component| {
            !matches!(component, Component::Normal(_) | Component::CurDir)
        })
    {
        return Err(format!("attempt artifact is not a safe relative path: {relative}"));
    }
    Ok(base.join(path))
}

fn prospective_path(path: &Path) -> Result<PathBuf, String> {
    let name = path
        .file_name()
        .ok_or_else(|| format!("{} has no output file name", path.display()))?;
    let parent = path
        .parent()
        .ok_or_else(|| format!("{} has no output parent", path.display()))?;
    let parent = if parent.as_os_str().is_empty() {
        Path::new(".")
    } else {
        parent
    };
    Ok(fs::canonicalize(parent)
        .map_err(|error| format!("{}: {error}", parent.display()))?
        .join(name))
}

fn terminal_class(value: &Value) -> Result<TerminalClass, String> {
    match value.as_str() {
        Some("complete") => Ok(TerminalClass::Complete),
        Some("scientific_censored") => Ok(TerminalClass::ScientificCensored),
        Some("infrastructure_blocked") => Ok(TerminalClass::InfrastructureBlocked),
        Some("not_attempted") => Ok(TerminalClass::NotAttempted),
        Some("invalid") => Err("attempt aggregate reports INVALID".into()),
        _ => Err("attempt aggregate has an unknown final_class".into()),
    }
}

fn expected_pod_job_spec(request: &Value) -> Value {
    json!({
        "schema":"icb.aimed2lp.pod_job_spec/v1",
        "provider_tool":request["provider_tool"],
        "job_name":request["job_name"],
        "image_uri":request["image_uri"],
        "image_digest":request["image_digest"],
        "argv":request["argv"],
        "payload_sha256":request["payload_sha256"],
        "payload_bytes":request["payload_bytes"],
        "instance_type":request["instance_type"],
        "capacity":request["capacity"],
        "node_selector":request["node_selector"],
        "resources":{
            "exclusive_vcpus":request["exclusive_vcpus"],
            "cpu_request":request["cpu_request"],
            "cpu_limit":request["cpu_limit"],
            "memory":request["memory"],
            "memory_bytes":request["memory_bytes"],
            "memory_request":request["memory_request"],
            "memory_limit":request["memory_limit"],
        },
        "restart_policy":request["restart_policy"],
        "child_timeout_seconds":request["child_timeout_seconds"],
        "outer_deadline_seconds":request["outer_deadline_seconds"],
    })
}

fn validate_pod_job_spec(request: &Value, observed: &Value) -> Result<(), String> {
    if observed != &expected_pod_job_spec(request) {
        return Err(
            "observed Pod/Job contract differs from exact frozen argv/resources/deadline".into(),
        );
    }
    Ok(())
}

fn canonical_artifact_json(path: &Path) -> Result<(Value, Vec<u8>), String> {
    let content = bytes(path)?;
    let text = std::str::from_utf8(&content)
        .map_err(|error| format!("{}: invalid UTF-8: {error}", path.display()))?;
    let value = protocol_unique_json(text, 0)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    if protocol_canonical_bytes(&value)? != content {
        return Err(format!("{} is not exact canonical JSON bytes", path.display()));
    }
    Ok((value, content))
}

fn validate_provider_and_pod_artifacts(
    base: &Path,
    template: &Value,
    attempt: &Value,
) -> Result<(), String> {
    let provider_relative = attempt["provider_identity_artifact"]
        .as_str()
        .ok_or("provider identity artifact path is absent")?;
    let pod_relative = attempt["pod_spec_artifact"]
        .as_str()
        .ok_or("Pod/Job spec artifact path is absent")?;
    let (provider, provider_bytes) =
        canonical_artifact_json(&local_artifact(base, provider_relative)?)?;
    let (pod, pod_bytes) = canonical_artifact_json(&local_artifact(base, pod_relative)?)?;
    if sha256_hex(&provider_bytes) != template["provider_identity_sha256"]
        || sha256_hex(&pod_bytes) != template["pod_spec_sha256"]
    {
        return Err("provider or Pod/Job artifact differs from its frozen anchor".into());
    }
    exact_keys(
        &provider,
        &[
            "schema", "provider_tool", "cloud_account_id", "region", "cluster", "namespace",
            "service_account", "admission_controller",
        ],
        "provider identity artifact",
    )?;
    if provider["schema"] != "icb.aimed2lp.provider_identity/v1"
        || provider["provider_tool"] != template["provider_tool"]
        || [
            "cloud_account_id",
            "region",
            "cluster",
            "namespace",
            "service_account",
            "admission_controller",
        ]
        .iter()
        .any(|field| provider[*field].as_str().is_none_or(str::is_empty))
    {
        return Err("provider identity artifact has an invalid semantic contract".into());
    }
    validate_pod_job_spec(template, &pod)?;
    Ok(())
}

fn require_artifact_path(
    artifacts: &[Value],
    required: &str,
    what: &str,
) -> Result<(), String> {
    let count = artifacts
        .iter()
        .filter(|artifact| artifact["path"] == required)
        .count();
    if count != 1 {
        return Err(format!(
            "{what} must custody {required} exactly once, found {count}"
        ));
    }
    Ok(())
}

fn require_unique_attempt_owner(
    attempts: &[Value],
    required: &str,
    what: &str,
) -> Result<(), String> {
    let owners = attempts
        .iter()
        .filter(|attempt| {
            attempt["artifacts"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|artifact| artifact["path"] == required)
        })
        .count();
    if owners != 1 {
        return Err(format!(
            "{what} must have exactly one authenticated attempt owner, found {owners}"
        ));
    }
    Ok(())
}

fn validate_global_artifact_owners(
    consumed: &BTreeSet<String>,
    owners: &BTreeMap<String, usize>,
) -> Result<(), String> {
    for path in consumed {
        let count = owners.get(path).copied().unwrap_or(0);
        if count != 1 {
            return Err(format!(
                "consumed artifact {path} must have exactly one owner across all cells, found {count}"
            ));
        }
    }
    Ok(())
}

fn require_artifact_basename(
    artifacts: &[Value],
    basename: &str,
    what: &str,
) -> Result<(), String> {
    let count = artifacts
        .iter()
        .filter(|artifact| {
            artifact["path"]
                .as_str()
                .and_then(|path| Path::new(path).file_name())
                .and_then(|name| name.to_str())
                == Some(basename)
        })
        .count();
    if count != 1 {
        return Err(format!(
            "{what} must custody one {basename} artifact, found {count}"
        ));
    }
    Ok(())
}

fn cell_artifact_contract(
    manifest: &Manifest,
    cell: &str,
) -> Result<(String, String, String, Vec<String>), String> {
    if cell == "control-n13" {
        return Ok((
            manifest.positive_control.session.clone(),
            manifest.positive_control.replay_receipt.clone(),
            manifest.positive_control.export.clone(),
            vec![manifest.positive_control.receipt.clone()],
        ));
    }
    let n = cell
        .strip_prefix('n')
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or_else(|| format!("unknown attempt cell {cell}"))?;
    let input = manifest
        .sizes
        .iter()
        .find(|input| input.n == n)
        .ok_or_else(|| format!("manifest has no input for {cell}"))?;
    Ok((
        input.session.clone(),
        input.replay_receipt.clone(),
        input.export.clone(),
        input.comparisons.values().cloned().collect(),
    ))
}

fn require_session_core_custody(
    artifacts: &[Value],
    session: &str,
    what: &str,
) -> Result<(), String> {
    for name in ["session.json", "host.json", "plan.json", "spec.json", "records.jsonl"] {
        require_artifact_path(artifacts, &format!("{session}/{name}"), what)?;
    }
    Ok(())
}

fn require_complete_session_custody(
    artifacts: &[Value],
    session: &str,
    replay: &str,
    export: &str,
    decisive: &[String],
    what: &str,
) -> Result<(), String> {
    require_session_core_custody(artifacts, session, what)?;
    require_artifact_path(artifacts, replay, what)?;
    require_artifact_path(artifacts, export, what)?;
    for path in decisive {
        require_artifact_path(artifacts, path, what)?;
    }
    Ok(())
}

fn validate_attempt_envelope(
    template: &Value,
    attempt: &Value,
    durable_prefix: &str,
) -> Result<(), String> {
    if &attempt["request"] != template {
        return Err("attempt request differs from the frozen request".into());
    }
    if attempt["lineage"]["restart_count"] != 0 {
        return Err("scientific pod restart_count is not zero".into());
    }
    let authentication = &attempt["authentication"];
    if authentication["provider_identity_sha256"] != template["provider_identity_sha256"]
        || authentication["pod_spec_sha256"] != template["pod_spec_sha256"]
        || authentication["lineage_object_sha256"] != canonical_sha256(&attempt["lineage"])?
        || authentication["request_object_sha256"] != canonical_sha256(&attempt["request"])?
        || authentication["terminal_object_sha256"] != canonical_sha256(&attempt["terminal"])?
        || authentication["artifact_listing_sha256"] != canonical_sha256(&attempt["artifacts"])?
    {
        return Err("attempt envelope authentication differs".into());
    }
    let artifacts = attempt["artifacts"]
        .as_array()
        .filter(|artifacts| !artifacts.is_empty())
        .ok_or("attempt envelope has no artifact custody")?;
    let provider_path = attempt["provider_identity_artifact"]
        .as_str()
        .filter(|path| !path.is_empty())
        .ok_or("attempt has no provider-identity artifact reference")?;
    let pod_path = attempt["pod_spec_artifact"]
        .as_str()
        .filter(|path| !path.is_empty())
        .ok_or("attempt has no Pod-spec artifact reference")?;
    if provider_path == pod_path {
        return Err("provider identity and Pod spec name the same artifact".into());
    }
    for (path, anchor, name) in [
        (provider_path, &template["provider_identity_sha256"], "provider identity"),
        (pod_path, &template["pod_spec_sha256"], "Pod spec"),
    ] {
        let matching = artifacts
            .iter()
            .filter(|artifact| artifact["path"] == path)
            .collect::<Vec<_>>();
        if matching.len() != 1 || &matching[0]["sha256"] != anchor {
            return Err(format!("attempt does not custody the anchored {name} artifact"));
        }
    }
    for artifact in artifacts {
        if !artifact["object_uri"]
            .as_str()
            .is_some_and(|uri| uri.starts_with(durable_prefix))
            || artifact["object_version"].as_str().is_none_or(str::is_empty)
            || artifact["readback_sha256"] != artifact["sha256"]
            || artifact["readback_bytes"] != artifact["bytes"]
            || artifact["readback_verified"] != true
        {
            return Err("attempt artifact is not durably versioned and read back".into());
        }
    }
    Ok(())
}

fn register_lineage_ids(
    lineage: &Value,
    request_ids: &mut BTreeSet<String>,
    job_uids: &mut BTreeSet<String>,
    pod_uids: &mut BTreeSet<String>,
) -> Result<(), String> {
    let request_id = lineage["request_id"]
        .as_str()
        .filter(|value| !value.is_empty())
        .ok_or("attempt lineage lacks request_id")?;
    if !request_ids.insert(request_id.to_string()) {
        return Err("provider request id is reused across attempts or cells".into());
    }
    for (field, seen) in [("job_uid", job_uids), ("pod_uid", pod_uids)] {
        if let Some(value) = lineage[field].as_str().filter(|value| !value.is_empty()) {
            if !seen.insert(value.to_string()) {
                return Err(format!("{field} is reused across attempts or cells"));
            }
        }
    }
    Ok(())
}

fn check_attempt_receipts(
    base: &Path,
    manifest: &Manifest,
    identity: &Identity,
) -> Result<BTreeMap<String, AttemptReceipt>, String> {
    let mut expected = BTreeMap::from([(
        "control-n13".to_string(),
        manifest.positive_control.attempt_receipt.clone(),
    )]);
    for size in &manifest.sizes {
        expected.insert(format!("n{}", size.n), size.attempt_receipt.clone());
    }
    if expected.len() != 3
        || manifest.attempt_receipts.iter().cloned().collect::<BTreeSet<_>>()
            != expected.values().cloned().collect::<BTreeSet<_>>()
    {
        return Err("manifest must bind exactly the control-n13, n53 and n61 aggregates".into());
    }
    if manifest.request_templates.keys().cloned().collect::<BTreeSet<_>>()
        != expected.keys().cloned().collect::<BTreeSet<_>>()
    {
        return Err("manifest must bind exactly three prospective request templates".into());
    }

    let mut checked = BTreeMap::new();
    let mut request_ids = BTreeSet::new();
    let mut job_uids = BTreeSet::new();
    let mut pod_uids = BTreeSet::new();
    let mut global_artifact_owners = BTreeMap::<String, usize>::new();
    let mut globally_consumed = BTreeSet::<String>::new();
    for (cell, relative) in expected {
        let template_relative = &manifest.request_templates[&cell];
        let template_path = base.join(template_relative);
        let template_bytes = bytes(&template_path)?;
        let frozen_request_sha256 = sha256_hex(&template_bytes);
        if identity.protocol["extensions"]["request_template_sha256"]
            [template_relative.as_str()]
            != frozen_request_sha256
        {
            return Err(format!("{} differs from the frozen request hash", template_path.display()));
        }
        let template = json_value(&template_path)?;
        if protocol_canonical_bytes(&template)? != template_bytes {
            return Err(format!(
                "{} is not exact canonical request JSON bytes",
                template_path.display()
            ));
        }
        exact_keys(
            &template,
            &[
                "schema", "status", "cell", "provider_tool", "job_name", "spec_path",
                "spec_file_sha256", "source_revision", "image_uri", "image_digest",
                "ecbench_sha256", "argv", "payload_sha256", "payload_bytes",
                "provider_identity_sha256", "pod_spec_sha256", "instance_type", "capacity",
                "node_selector", "exclusive_vcpus", "cpu_request", "cpu_limit", "memory",
                "memory_bytes", "memory_request", "memory_limit", "restart_policy",
                "child_timeout_seconds", "outer_deadline_seconds", "durable_prefix",
                "provider_retries", "scientific_retries", "infrastructure_attempts_max",
                "retain_failed_artifacts",
            ],
            &format!("{cell} frozen request"),
        )?;
        exact_keys(
            &template["node_selector"],
            &["node.kubernetes.io/instance-type", "eks.amazonaws.com/capacityType"],
            &format!("{cell} node selector"),
        )?;
        let n = cell.strip_prefix('n').and_then(|value| value.parse::<u32>().ok());
        let expected_n = n.unwrap_or(13);
        let spec_path = expected_spec_path(expected_n);
        let spec_content = bytes(&base.join("../..").join(&spec_path))?;
        let spec_file_sha256 = sha256_hex(&spec_content);
        let expected_prefix = format!(
            "s3://setec-astronomy-artifacts/ecbench/icb-aimed-large-prime-n53-n61-20261101/{cell}/"
        );
        let digest_suffix = format!("@{}", identity.image_digest);
        let expected_pod_spec_sha256 = canonical_sha256(&expected_pod_job_spec(&template))?;
        if template["schema"] != "icb.aimed2lp.request/v1"
            || template["status"] != "frozen"
            || template["cell"] != cell
            || template["provider_tool"] != "isolab_k8s_benchmark"
            || template["spec_path"] != spec_path
            || template["spec_file_sha256"]
                != identity.image["specs"][spec_path.as_str()]["file_sha256"]
            || template["spec_file_sha256"] != spec_file_sha256
            || template["source_revision"] != identity.source_commit
            || template["image_uri"] != identity.image["image_ref"]
            || template["image_digest"] != identity.image_digest
            || !template["image_uri"]
                .as_str()
                .is_some_and(|value| value.ends_with(&digest_suffix))
            || template["ecbench_sha256"] != identity.binary_sha256
            || !valid_argv(&template["argv"])
            || sha256_string(&template["payload_sha256"]).is_none()
            || template["payload_sha256"] != spec_file_sha256
            || template["payload_bytes"] != spec_content.len() as u64
            || sha256_string(&template["provider_identity_sha256"]).is_none()
            || sha256_string(&template["pod_spec_sha256"]).is_none()
            || template["pod_spec_sha256"] != expected_pod_spec_sha256
            || template["instance_type"] != "c8a.2xlarge"
            || template["capacity"] != "spot"
            || template["node_selector"]["node.kubernetes.io/instance-type"]
                != "c8a.2xlarge"
            || template["node_selector"]["eks.amazonaws.com/capacityType"] != "SPOT"
            || template["exclusive_vcpus"] != 4
            || template["cpu_request"] != "4"
            || template["cpu_limit"] != "4"
            || template["memory"] != "8Gi"
            || template["memory_bytes"] != 8_589_934_592u64
            || template["memory_request"] != "8Gi"
            || template["memory_limit"] != "8Gi"
            || template["restart_policy"] != "Never"
            || template["child_timeout_seconds"] != if expected_n == 13 { 600 } else { 3_600 }
            || template["outer_deadline_seconds"] != 7_800
            || template["durable_prefix"] != expected_prefix
            || template["provider_retries"] != 0
            || template["scientific_retries"] != 0
            || template["infrastructure_attempts_max"] != 2
            || template["retain_failed_artifacts"] != true
        {
            return Err(format!("{} is not the frozen cell request", template_path.display()));
        }
        let path = base.join(&relative);
        let receipt = json_value(&path)?;
        exact_keys(
            &receipt,
            &[
                "schema", "created", "cell", "final_class", "not_attempted_reason",
                "frozen_request_sha256", "protocol_sha256", "source_commit", "binary_sha256",
                "image_digest", "provider_identity_sha256", "attempts",
            ],
            &format!("{cell} attempt aggregate"),
        )?;
        if receipt["schema"] != "icb.aimed2lp.attempts/v1"
            || receipt["created"].as_str().is_none_or(str::is_empty)
            || receipt["cell"] != cell
            || receipt["frozen_request_sha256"] != frozen_request_sha256
            || receipt["protocol_sha256"] != identity.protocol_sha256
            || receipt["source_commit"] != identity.source_commit
            || receipt["binary_sha256"] != identity.binary_sha256
            || receipt["image_digest"] != identity.image_digest
            || receipt["provider_identity_sha256"] != template["provider_identity_sha256"]
        {
            return Err(format!("{} has an unbound aggregate identity", path.display()));
        }
        let class = terminal_class(&receipt["final_class"])?;
        let attempts = receipt["attempts"]
            .as_array()
            .filter(|attempts| attempts.len() <= 2)
            .ok_or_else(|| format!("{} must contain an attempts array", path.display()))?;
        if class == TerminalClass::NotAttempted {
            if !attempts.is_empty()
                || receipt["not_attempted_reason"]
                    .as_str()
                    .is_none_or(str::is_empty)
            {
                return Err(format!(
                    "{} not-attempted aggregate has attempts or no reason",
                    path.display()
                ));
            }
        } else if attempts.is_empty() || !receipt["not_attempted_reason"].is_null() {
            return Err(format!(
                "{} terminal aggregate needs one or two attempts and no not-attempted reason",
                path.display()
            ));
        }
        let (session_relative, replay_relative, export_relative, decisive_paths) =
            cell_artifact_contract(manifest, &cell)?;
        for (index, attempt) in attempts.iter().enumerate() {
            exact_keys(
                attempt,
                &[
                    "schema", "created", "status", "protocol_sha256", "source_commit",
                    "binary_sha256", "image_digest", "lineage", "request", "terminal",
                    "authentication", "provider_identity_artifact", "pod_spec_artifact",
                    "artifacts",
                ],
                &format!("{cell} attempt {index}"),
            )?;
            if attempt["schema"] != "icb.aimed2lp.attempt/v1"
                || attempt["created"].as_str().is_none_or(str::is_empty)
                || attempt["protocol_sha256"] != identity.protocol_sha256
                || attempt["source_commit"] != identity.source_commit
                || attempt["binary_sha256"] != identity.binary_sha256
                || attempt["image_digest"] != identity.image_digest
            {
                return Err(format!("{} attempt {index} has identity drift", path.display()));
            }
            let status = attempt["status"]
                .as_str()
                .ok_or_else(|| format!("{} attempt {index} lacks status", path.display()))?;
            let terminal = attempt["terminal"]["classification"]
                .as_str()
                .ok_or_else(|| format!("{} attempt {index} lacks terminal class", path.display()))?;
            let consistent = matches!(
                (status, terminal),
                ("succeeded", "scientific_complete")
                    | ("scientific_censored", "scientific_censored")
                    | ("infrastructure_failed", "infrastructure_blocked")
                    | ("resource_refused", "infrastructure_blocked")
                    | ("invalid", "identity_or_correctness_invalid")
            );
            if !consistent || status == "invalid" {
                return Err(format!("{} attempt {index} is invalid or inconsistent", path.display()));
            }
            let terminal_value = &attempt["terminal"];
            if terminal_value["reason"].as_str().is_none_or(str::is_empty)
                || terminal_value["finished"].as_str().is_none_or(str::is_empty)
                || (!terminal_value["started"].is_null()
                    && terminal_value["started"].as_str().is_none_or(str::is_empty))
                || (!terminal_value["exit_code"].is_null()
                    && terminal_value["exit_code"].as_i64().is_none())
            {
                return Err(format!("{} attempt {index} has an invalid terminal object", path.display()));
            }
            if index + 1 < attempts.len() && terminal != "infrastructure_blocked" {
                return Err(format!("{} retries a non-infrastructure outcome", path.display()));
            }
            let lineage = &attempt["lineage"];
            exact_keys(
                lineage,
                &[
                    "request_id", "job_name", "job_uid", "pod_name", "pod_uid", "node_name",
                    "instance_id", "instance_type", "capacity", "attempt", "restart_count",
                ],
                &format!("{cell} attempt {index} lineage"),
            )?;
            let request_id = lineage["request_id"]
                .as_str()
                .filter(|value| !value.is_empty());
            if request_id.is_none()
                || lineage["job_name"] != template["job_name"]
                || lineage["attempt"].as_u64() != Some(index as u64)
                || lineage["restart_count"] != 0
            {
                return Err(format!("{} attempt {index} has incomplete lineage", path.display()));
            }
            register_lineage_ids(
                lineage,
                &mut request_ids,
                &mut job_uids,
                &mut pod_uids,
            )
            .map_err(|error| format!("{} attempt {index}: {error}", path.display()))?;
            let job_uid = lineage["job_uid"].as_str().filter(|value| !value.is_empty());
            let pod_name = lineage["pod_name"].as_str().filter(|value| !value.is_empty());
            let pod_uid = lineage["pod_uid"].as_str().filter(|value| !value.is_empty());
            let node_name = lineage["node_name"].as_str().filter(|value| !value.is_empty());
            let instance_id = lineage["instance_id"].as_str().filter(|value| !value.is_empty());
            if status != "resource_refused"
                && (job_uid.is_none()
                    || pod_name.is_none()
                    || pod_uid.is_none()
                    || node_name.is_none()
                    || instance_id.is_none()
                    || lineage["instance_type"] != template["instance_type"]
                    || lineage["capacity"] != template["capacity"])
            {
                return Err(format!(
                    "{} attempt {index} lacks exact Job/Pod/node/instance identity",
                    path.display()
                ));
            }
            if status == "resource_refused"
                && (["job_uid", "pod_name", "pod_uid", "node_name", "instance_id"]
                    .iter()
                    .any(|field| {
                        !lineage[*field].is_null()
                            && lineage[*field].as_str().is_none_or(str::is_empty)
                    })
                    || !matches!(lineage["instance_type"].as_str(), None | Some("c8a.2xlarge"))
                    || !matches!(lineage["capacity"].as_str(), None | Some("spot")))
            {
                return Err(format!("{} refused attempt {index} has malformed lineage", path.display()));
            }

            validate_attempt_envelope(&template, attempt, &expected_prefix)
                .map_err(|error| format!("{} attempt {index}: {error}", path.display()))?;
            exact_keys(
                &attempt["terminal"],
                &["classification", "reason", "exit_code", "started", "finished"],
                &format!("{cell} attempt {index} terminal"),
            )?;
            exact_keys(
                &attempt["authentication"],
                &[
                    "provider_identity_sha256", "pod_spec_sha256", "lineage_object_sha256",
                    "request_object_sha256", "terminal_object_sha256", "artifact_listing_sha256",
                ],
                &format!("{cell} attempt {index} authentication"),
            )?;
            let artifacts = attempt["artifacts"]
                .as_array()
                .filter(|artifacts| !artifacts.is_empty())
                .ok_or_else(|| format!("{} attempt {index} lacks artifact custody", path.display()))?;
            let mut artifact_paths = BTreeSet::new();
            let mut object_uris = BTreeSet::new();
            for artifact in artifacts {
                exact_keys(
                    artifact,
                    &[
                        "path", "sha256", "bytes", "object_uri", "object_version",
                        "readback_sha256", "readback_bytes", "readback_verified",
                    ],
                    &format!("{cell} attempt {index} artifact"),
                )?;
                let relative_artifact = artifact["path"]
                    .as_str()
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| format!("{} attempt {index} has an unnamed artifact", path.display()))?;
                let artifact_path = local_artifact(base, relative_artifact)?;
                let content = bytes(&artifact_path)?;
                let digest = sha256_hex(&content);
                if artifact["sha256"] != sha256_hex(&content)
                    || artifact["bytes"] != content.len() as u64
                    || artifact["readback_sha256"] != digest
                    || artifact["readback_bytes"] != content.len() as u64
                    || !artifact_paths.insert(relative_artifact.to_string())
                    || !object_uris.insert(artifact["object_uri"].as_str().unwrap_or_default())
                {
                    return Err(format!("{} has stale artifact {relative_artifact}", path.display()));
                }
                *global_artifact_owners
                    .entry(relative_artifact.to_string())
                    .or_default() += 1;
            }
            let custody = format!("{cell} attempt {index}");
            require_artifact_path(artifacts, template_relative, &custody)?;
            for basename in ["stdout.log", "stderr.log", "job-exit.json", "SHA256SUMS"] {
                require_artifact_basename(artifacts, basename, &custody)?;
            }
            validate_provider_and_pod_artifacts(base, &template, attempt)
                .map_err(|error| format!("{} attempt {index}: {error}", path.display()))?;

            let records_path = format!("{session_relative}/records.jsonl");
            let owns_retained_session = artifacts
                .iter()
                .any(|artifact| artifact["path"] == records_path);
            if status == "succeeded" {
                require_complete_session_custody(
                    artifacts,
                    &session_relative,
                    &replay_relative,
                    &export_relative,
                    &decisive_paths,
                    &custody,
                )?;
            } else if status == "scientific_censored" || owns_retained_session {
                require_session_core_custody(artifacts, &session_relative, &custody)?;
                for optional in std::iter::once(&replay_relative)
                    .chain(std::iter::once(&export_relative))
                    .chain(decisive_paths.iter())
                {
                    if base.join(optional).exists() {
                        require_artifact_path(artifacts, optional, &custody)?;
                    }
                }
            }
        }
        let complete_attempt = attempts.iter().any(|attempt| attempt["status"] == "succeeded");
        if complete_attempt || base.join(&session_relative).exists() {
            for name in ["session.json", "host.json", "plan.json", "spec.json", "records.jsonl"] {
                let consumed = format!("{session_relative}/{name}");
                globally_consumed.insert(consumed.clone());
                require_unique_attempt_owner(attempts, &consumed, &consumed)?;
            }
        }
        for consumed in std::iter::once(&replay_relative)
            .chain(std::iter::once(&export_relative))
            .chain(decisive_paths.iter())
        {
            if complete_attempt || base.join(consumed).exists() {
                globally_consumed.insert(consumed.to_string());
                require_unique_attempt_owner(attempts, consumed, consumed)?;
            }
        }
        let final_terminal = attempts
            .last()
            .and_then(|attempt| attempt["terminal"]["classification"].as_str());
        let final_matches = matches!(
            (class, final_terminal),
            (TerminalClass::Complete, Some("scientific_complete"))
                | (TerminalClass::ScientificCensored, Some("scientific_censored"))
                | (TerminalClass::InfrastructureBlocked, Some("infrastructure_blocked"))
                | (TerminalClass::NotAttempted, None)
        );
        if !final_matches {
            return Err(format!("{} final_class disagrees with its last attempt", path.display()));
        }
        checked.insert(
            cell,
            AttemptReceipt {
                relative,
                class,
                value: receipt,
            },
        );
    }
    validate_global_artifact_owners(&globally_consumed, &global_artifact_owners)?;
    Ok(checked)
}

fn control_gate(
    base: &Path,
    input: &ControlInput,
    identity: &Identity,
    attempt: &AttemptReceipt,
) -> Result<Option<Bundle>, String> {
    if attempt.class != TerminalClass::Complete {
        let retained = load_retained_bundle(
            base,
            &input.session,
            &input.replay_receipt,
            identity,
            13,
            60,
        )?;
        if attempt.class == TerminalClass::NotAttempted && retained.is_some() {
            return Err("not-attempted n13 control unexpectedly has retained session data".into());
        }
        if let Some(bundle) = &retained {
            validate_retained_terminal(bundle, attempt.class)
                .map_err(|error| format!("n13 retained terminal mismatch: {error}"))?;
            lp_gate(&bundle.records, false)?;
        }
        return Ok(retained);
    }
    let bundle = load_bundle(
        base,
        &input.session,
        &input.replay_receipt,
        identity,
        13,
        60,
        50,
    )?;
    if classify_records(&bundle.records)? != TerminalClass::Complete {
        return Err("n13 positive control did not complete every row".into());
    }
    if bundle
        .records
        .iter()
        .any(|record| record.isolation.level < Level::L2)
    {
        return Err("n13 positive control contains a row below L2".into());
    }
    let blocks = paired_blocks(&bundle.records, &CONTROL_ARMS, 2)?;
    if exact_pair_gate(&blocks, "aimed", "aimed-aa")? != 10
        || exact_pair_gate(&blocks, "aimed", "aimed-lp0")? != 10
    {
        return Err("n13 A/A or LP=0 does not contain ten exact pairs".into());
    }
    lp_gate(&bundle.records, true)?;
    let receipt = json_value(&base.join(&input.receipt))?;
    exact_keys(
        &receipt,
        &[
            "schema", "created", "status", "session_id", "source_commit", "binary_sha256",
            "spec_identity_sha256", "measured_rows", "verified_rows", "aa_pairs_identical",
            "lp0_pairs_identical", "lp2_rows_active", "lp2_rows_with_merged_relation",
            "lp_direct_two_sum", "lp_verification_failures", "replay_required", "note",
        ],
        "n13 control receipt",
    )?;
    if receipt["schema"] != "icb.aimed2lp.control/v1"
        || receipt["created"].as_str().is_none_or(str::is_empty)
        || receipt["status"] != "pass"
        || receipt["session_id"] != bundle.session.session_id
        || receipt["source_commit"] != identity.source_commit
        || receipt["binary_sha256"] != identity.binary_sha256
        || receipt["spec_identity_sha256"] != bundle.session.spec_sha256
        || receipt["measured_rows"] != 50
        || receipt["verified_rows"] != 50
        || receipt["aa_pairs_identical"] != 10
        || receipt["lp0_pairs_identical"] != 10
        || receipt["lp2_rows_active"] != 20
        || receipt["lp2_rows_with_merged_relation"] != 20
        || receipt["lp_direct_two_sum"] != 0
        || receipt["lp_verification_failures"] != 0
        || receipt["replay_required"] != true
        || receipt["note"].as_str().is_none_or(str::is_empty)
    {
        return Err("n13 control receipt does not bind all positive-control gates".into());
    }
    Ok(Some(bundle))
}

fn comparison_arms(id: &str) -> Option<(&'static str, &'static str)> {
    match id {
        "C-aa" => Some(("aimed", "aimed-aa")),
        "C-lp0" => Some(("aimed", "aimed-lp0")),
        "C-c05" => Some(("aimed", "aimed-lp2-c05")),
        "C-c1" => Some(("aimed", "aimed-lp2-c1")),
        "C-rho-c05" => Some(("rho-v2", "aimed-lp2-c05")),
        "C-rho-c1" => Some(("rho-v2", "aimed-lp2-c1")),
        _ => None,
    }
}

fn target_gate(
    base: &Path,
    input: &SizeInput,
    identity: &Identity,
    attempt: &AttemptReceipt,
) -> Result<Option<Bundle>, String> {
    if attempt.class != TerminalClass::Complete {
        let retained = load_retained_bundle(
            base,
            &input.session,
            &input.replay_receipt,
            identity,
            input.n,
            288,
        )?;
        if attempt.class == TerminalClass::NotAttempted && retained.is_some() {
            return Err(format!(
                "not-attempted n={} target unexpectedly has retained session data",
                input.n
            ));
        }
        if let Some(bundle) = &retained {
            validate_retained_terminal(bundle, attempt.class).map_err(|error| {
                format!("n={} retained terminal mismatch: {error}", input.n)
            })?;
            lp_gate(&bundle.records, false)?;
        }
        return Ok(retained);
    }
    let bundle = load_bundle(
        base,
        &input.session,
        &input.replay_receipt,
        identity,
        input.n,
        288,
        240,
    )?;
    if bundle
        .records
        .iter()
        .any(|record| record.isolation.level < Level::L2)
    {
        return Err(format!("n={} contains a row below L2", input.n));
    }
    let blocks = paired_blocks(&bundle.records, &ARMS, TARGETS)?;
    let class = classify_records(&bundle.records)?;
    if class != TerminalClass::Complete {
        return Err(format!(
            "n={} aggregate says complete but measured rows do not",
            input.n
        ));
    }
    let aa_pairs = exact_pair_gate(&blocks, "aimed", "aimed-aa")?;
    let lp0_pairs = exact_pair_gate(&blocks, "aimed", "aimed-lp0")?;
    if aa_pairs != TARGETS * ROUNDS || lp0_pairs != TARGETS * ROUNDS {
        return Err(format!("n={} fails A/A or LP=0 exact identity", input.n));
    }
    lp_gate(&bundle.records, false)?;
    for (id, relative) in &input.comparisons {
        let (a, b) = comparison_arms(id)
            .ok_or_else(|| format!("n={} has unknown comparison {id}", input.n))?;
        let _ = compare_saved(&base.join(relative), &bundle, a, b)?;
    }
    Ok(Some(bundle))
}

fn verdict(estimate: &Estimate, aa: &Estimate) -> &'static str {
    let aa_stable = aa.ci95.0 >= 0.9 && aa.ci95.1 <= 1.1 && aa.ci95.0 <= 1.0 && aa.ci95.1 >= 1.0;
    if !aa_stable {
        "INVALID"
    } else if estimate.adjusted_p <= 0.05
        && estimate.point <= WALL_THRESHOLD
        && estimate.ci95.1 <= WALL_THRESHOLD
    {
        "IMPROVEMENT"
    } else if estimate.ci95.0 > 1.0 {
        "WORSE"
    } else {
        "NULL"
    }
}

fn file_row(path: &Path, display: String, kind: &str) -> Result<Value, String> {
    let content = bytes(path)?;
    Ok(json!({
        "path": display,
        "sha256": sha256_hex(&content),
        "bytes": content.len(),
        "kind": kind,
    }))
}

fn push_file_if_exists(
    files: &mut Vec<Value>,
    path: &Path,
    display: String,
    kind: &str,
) -> Result<(), String> {
    if path.exists() {
        files.push(file_row(path, display, kind)?);
    }
    Ok(())
}

fn load_cell_plan(
    base: &Path,
    cell: &str,
    n: u32,
    identity: &Identity,
) -> Result<Plan, String> {
    let relative = expected_spec_path(n);
    let path = base.join("../..").join(&relative);
    let content = bytes(&path)?;
    if identity.image["specs"][relative.as_str()]["file_sha256"] != sha256_hex(&content) {
        return Err(format!("{cell} spec bytes differ from the immutable image receipt"));
    }
    let text = std::str::from_utf8(&content)
        .map_err(|error| format!("{} is not UTF-8: {error}", path.display()))?;
    let parsed = protocol_unique_json(text, 0)?;
    let spec = Spec::from_json(
        &serde_json::to_string(&parsed).map_err(|error| format!("{cell} spec: {error}"))?,
    )?;
    let expanded = plan(spec)?;
    if identity.image["specs"][relative.as_str()]["identity_sha256"] != expanded.spec_sha256
        || identity.image["specs"][relative.as_str()]["spec_id"] != expanded.spec_id
    {
        return Err(format!("{cell} expanded plan differs from the immutable image receipt"));
    }
    let expected = if n == 13 { 60 } else { 288 };
    if expanded.executions.len() != expected {
        return Err(format!(
            "{cell} expands to {} rows, expected {expected}",
            expanded.executions.len()
        ));
    }
    Ok(expanded)
}

fn attempt_backend(attempt: &Value) -> Result<String, String> {
    let lineage = &attempt["lineage"];
    let request_id = lineage["request_id"]
        .as_str()
        .filter(|value| !value.is_empty())
        .ok_or("attempt lineage lacks request_id")?;
    let job_uid = lineage["job_uid"].as_str().unwrap_or("resource-refused");
    let pod_uid = lineage["pod_uid"].as_str().unwrap_or("resource-refused");
    let number = lineage["attempt"].as_u64().ok_or("attempt lineage lacks attempt")?;
    Ok(format!(
        "request={request_id};job_uid={job_uid};pod_uid={pod_uid};attempt={number}"
    ))
}

fn bundle_attempt<'a>(
    base: &Path,
    bundle: &Bundle,
    receipt: &'a AttemptReceipt,
) -> Result<&'a Value, String> {
    let relative = bundle
        .dir
        .strip_prefix(base)
        .map_err(|_| format!("{} is outside the study directory", bundle.dir.display()))?;
    let records_path = relative.join("records.jsonl").display().to_string();
    let owners: Vec<&Value> = receipt.value["attempts"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|attempt| {
            attempt["artifacts"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|artifact| artifact["path"] == records_path)
        })
        .collect();
    if owners.len() != 1 {
        return Err(format!(
            "{} must be owned by exactly one authenticated provider attempt, found {}",
            records_path,
            owners.len()
        ));
    }
    Ok(owners[0])
}

fn validate_record_against_execution(
    cell: &str,
    plan: &Plan,
    execution: &Execution,
    record: &Record,
) -> Result<(), String> {
    let arm = plan
        .arms
        .get(execution.arm)
        .ok_or_else(|| format!("{cell} execution {} has invalid arm index", execution.seq))?;
    let workload = plan
        .workloads
        .get(execution.workload)
        .ok_or_else(|| format!("{cell} execution {} has invalid workload index", execution.seq))?;
    if record.seq != execution.seq
        || record.round != execution.round
        || record.warmup != execution.warmup
        || record.arm != arm.name
        || &record.workload != workload
        || record.algorithm_seed != execution.algorithm_seed.to_string()
    {
        return Err(format!(
            "{cell} record {} does not match frozen execution {}",
            record.run_id, execution.seq
        ));
    }
    Ok(())
}

fn cell_exclusion(class: TerminalClass) -> Option<&'static str> {
    match class {
        TerminalClass::Complete => None,
        TerminalClass::ScientificCensored => Some(
            "cell ended scientific_censored; partial rows are retained outside decisive inference",
        ),
        TerminalClass::InfrastructureBlocked => Some(
            "cell ended infrastructure_blocked; partial rows are retained outside decisive inference",
        ),
        TerminalClass::NotAttempted => Some(
            "cell was not attempted after a prerequisite gate failed",
        ),
    }
}

fn record_export(record: &Record, class: TerminalClass, attempt: &Value) -> Result<Value, String> {
    let (status, outcome_class, verification) = match record.outcome.status.as_str() {
        "verified" if record.outcome.matches_target == Some(true) => {
            ("succeeded", "completed", "verified")
        }
        "wrong_answer" | "verified" => ("failed", "completed", "refuted"),
        "exhausted" => ("failed", "completed", "no_claim"),
        "timeout" => ("timeout", "not_completed", "no_claim"),
        "error" | "crashed" => ("infra_error", "not_completed", "error"),
        _ => ("failed", "not_completed", "error"),
    };
    let counted = record
        .cost
        .total_gae
        .map(|gae| json!({"cold_gae": gae}))
        .unwrap_or_else(|| json!({}));
    let wall_valid = class == TerminalClass::Complete
        && verified(record)
        && record.isolation.level >= Level::L2;
    let (excluded, exclusion_rule) = if let Some(rule) = cell_exclusion(class) {
        (true, Some(rule))
    } else if record.warmup {
        (true, Some("preregistered warmup"))
    } else if !verified(record) {
        (true, Some("non-verified terminal row retained outside decisive inference"))
    } else if record.isolation.level < Level::L2 {
        (true, Some("row below preregistered L2 wall admission"))
    } else {
        (false, None)
    };
    let lineage = &attempt["lineage"];
    Ok(json!({
        "run_id": record.run_id,
        "arm": record.arm,
        "instance_id": record.workload.workload_id,
        "seed": record.algorithm_seed,
        "replicate": record.round,
        "warmup": record.warmup,
        "status": status,
        "outcome_class": outcome_class,
        "attempts": 1,
        "resumed": 0,
        "checkpoint": {"policy":"none","generations_written":0,"restored_from_generation":null,"manifest_sha256":null},
        "measurements": {
            "counted": counted,
            "wall_seconds": record.time.solve_wall_ns.map(|ns| ns as f64 / 1e9),
            "wall_valid": wall_valid,
            "cpu_user_seconds": record.time.user_ns as f64 / 1e9,
            "cpu_sys_seconds": record.time.sys_ns as f64 / 1e9,
            "max_rss_bytes": record.time.max_rss_kib.saturating_mul(1024),
            "counters": record.counters,
        },
        "isolation": {
            "tier": record.isolation.level.name(),
            "contended": record.isolation.level < Level::L2,
            "blockers": record.isolation.blockers,
        },
        "verification": {"status": verification, "verifier":"ecbench parent and replay-all audit"},
        "exclusion": {"excluded":excluded,"rule":exclusion_rule},
        "artifacts": [],
        "started_at": null,
        "finished_at": null,
        "worker": lineage["node_name"].as_str().map(str::to_owned),
        "backend_ref": attempt_backend(attempt)?,
    }))
}

fn missing_export(
    cell: &str,
    plan: &Plan,
    execution: &Execution,
    class: TerminalClass,
    attempt: Option<&Value>,
    stopped_here: bool,
) -> Result<Value, String> {
    if stopped_here && attempt.is_none() {
        return Err(format!("{cell} stopped execution has no authenticated provider attempt"));
    }
    let arm = &plan.arms[execution.arm];
    let workload = &plan.workloads[execution.workload];
    let (status, blocker, attempted) = if stopped_here {
        match class {
            TerminalClass::ScientificCensored => (
                "interrupted",
                "scientific stopping rule censored this execution and every later scheduled row",
                true,
            ),
            TerminalClass::InfrastructureBlocked => (
                "infra_error",
                "infrastructure failure stopped this execution and every later scheduled row",
                true,
            ),
            TerminalClass::Complete => {
                return Err(format!("complete {cell} has a missing stopped execution"))
            }
            TerminalClass::NotAttempted => {
                return Err(format!("not-attempted {cell} cannot have a stopped execution"))
            }
        }
    } else {
        (
            "cancelled",
            match class {
                TerminalClass::ScientificCensored => {
                    "not attempted after the preregistered scientific stopping rule fired"
                }
                TerminalClass::InfrastructureBlocked => {
                    "not attempted after the infrastructure-blocked terminal decision"
                }
                TerminalClass::Complete => return Err(format!("complete {cell} has a missing row")),
                TerminalClass::NotAttempted => {
                    "not attempted because the preregistered prerequisite gate did not pass"
                }
            },
            false,
        )
    };
    Ok(json!({
        "run_id": format!("planned-{cell}-{:03}", execution.seq),
        "arm": arm.name,
        "instance_id": workload.workload_id,
        "seed": execution.algorithm_seed.to_string(),
        "replicate": execution.round,
        "warmup": execution.warmup,
        "status": status,
        "outcome_class": "not_completed",
        // experiment.export/v1 requires every scheduled row to carry one
        // row-level scheduling attempt, including a cancelled row whose child
        // never began. `flow.attempted` below is the execution-start count;
        // the cell attempt receipt separately has attempts=[] when no provider
        // request existed.
        "attempts": 1,
        "resumed": 0,
        "checkpoint": {"policy":"none","generations_written":0,"restored_from_generation":null,"manifest_sha256":null},
        "measurements": {
            "counted": {},
            "wall_seconds": null,
            "wall_valid": false,
            "cpu_user_seconds": null,
            "cpu_sys_seconds": null,
            "max_rss_bytes": null,
            "counters": {},
        },
        "isolation": {"tier":null,"contended":null,"blockers":[blocker]},
        "verification": {"status":"not_run","verifier":null},
        "exclusion": {"excluded":true,"rule":blocker},
        "artifacts": [],
        "started_at": if attempted { attempt.map_or(Value::Null, |value| value["terminal"]["started"].clone()) } else { Value::Null },
        "finished_at": if attempted { attempt.map_or(Value::Null, |value| value["terminal"]["finished"].clone()) } else { Value::Null },
        "worker": attempt.and_then(|value| value["lineage"]["node_name"].as_str()).map(str::to_owned),
        "backend_ref": attempt.map(attempt_backend).transpose()?,
    }))
}

fn build_export(
    base: &Path,
    manifest_path: &Path,
    manifest: &Manifest,
    identity: &Identity,
    attempts: &BTreeMap<String, AttemptReceipt>,
    bundles: &[&Bundle],
    result_bytes: &[u8],
    validation_bytes: &[u8],
    result_path: &Path,
    validation_path: &Path,
) -> Result<Value, String> {
    let source_path = base.join(&manifest.native_analyzer.source);
    let mut files = vec![
        file_row(&base.join(&manifest.protocol), manifest.protocol.clone(), "protocol")?,
        file_row(manifest_path, "analysis-manifest.json".into(), "analysis")?,
        file_row(&base.join(&manifest.result_schema), manifest.result_schema.clone(), "other")?,
        file_row(&base.join(&manifest.receipt_schema), manifest.receipt_schema.clone(), "other")?,
        file_row(&base.join(&manifest.request_schema), manifest.request_schema.clone(), "other")?,
        file_row(&base.join(&manifest.focused_gates), manifest.focused_gates.clone(), "receipt")?,
        file_row(&base.join(&manifest.checkpoint_receipt), manifest.checkpoint_receipt.clone(), "receipt")?,
        file_row(&base.join(&manifest.image_receipt), manifest.image_receipt.clone(), "receipt")?,
        file_row(&base.join(&manifest.provider_preflight), manifest.provider_preflight.clone(), "receipt")?,
        file_row(
            &base.join(&manifest.native_analyzer.self_test_receipt),
            manifest.native_analyzer.self_test_receipt.clone(),
            "receipt",
        )?,
    ];
    for request in manifest.request_templates.values() {
        files.push(file_row(&base.join(request), request.clone(), "other")?);
    }
    for receipt in &manifest.attempt_receipts {
        files.push(file_row(&base.join(receipt), receipt.clone(), "receipt")?);
        let aggregate = json_value(&base.join(receipt))?;
        for artifact in aggregate["attempts"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(|attempt| attempt["artifacts"].as_array().into_iter().flatten())
        {
            let relative = artifact["path"]
                .as_str()
                .ok_or("attempt artifact path missing during export")?;
            files.push(file_row(
                &local_artifact(base, relative)?,
                relative.to_string(),
                "raw",
            )?);
        }
    }
    push_file_if_exists(
        &mut files,
        &base.join(&manifest.positive_control.receipt),
        manifest.positive_control.receipt.clone(),
        "receipt",
    )?;
    push_file_if_exists(
        &mut files,
        &base.join(&manifest.positive_control.replay_receipt),
        manifest.positive_control.replay_receipt.clone(),
        "receipt",
    )?;
    push_file_if_exists(
        &mut files,
        &base.join(&manifest.positive_control.export),
        manifest.positive_control.export.clone(),
        "summary",
    )?;
    for size in &manifest.sizes {
        push_file_if_exists(
            &mut files,
            &base.join(&size.replay_receipt),
            size.replay_receipt.clone(),
            "receipt",
        )?;
        push_file_if_exists(
            &mut files,
            &base.join(&size.export),
            size.export.clone(),
            "summary",
        )?;
        for comparison in size.comparisons.values() {
            push_file_if_exists(
                &mut files,
                &base.join(comparison),
                comparison.clone(),
                "summary",
            )?;
        }
    }
    for bundle in bundles {
        let prefix = bundle
            .dir
            .strip_prefix(base)
            .unwrap_or(&bundle.dir)
            .display()
            .to_string();
        for name in ["host.json", "plan.json", "records.jsonl", "session.json", "spec.json"] {
            push_file_if_exists(
                &mut files,
                &bundle.dir.join(name),
                format!("{prefix}/{name}"),
                if name == "host.json" { "host" } else { "raw" },
            )?;
        }
    }
    let mut bundle_by_cell = BTreeMap::new();
    for bundle in bundles {
        if bundle_by_cell.insert(bundle.cell.as_str(), *bundle).is_some() {
            return Err(format!("duplicate retained bundle for {}", bundle.cell));
        }
    }
    let mut runs = Vec::with_capacity(636);
    let mut execution_attempted = 0usize;
    for (cell, n) in [("control-n13", 13), ("n53", 53), ("n61", 61)] {
        let receipt = attempts
            .get(cell)
            .ok_or_else(|| format!("canonical export lacks {cell} attempt aggregate"))?;
        let expanded = load_cell_plan(base, cell, n, identity)?;
        let bundle = bundle_by_cell.get(cell).copied();
        let owner = bundle
            .map(|retained| bundle_attempt(base, retained, receipt))
            .transpose()?;
        let mut actual = BTreeMap::<u64, &Record>::new();
        if let Some(retained) = bundle {
            if !matches!(retained.session.status.as_str(), "complete" | "interrupted") {
                return Err(format!("{cell} retained session is not terminal"));
            }
            for record in &retained.records {
                if actual.insert(record.seq, record).is_some() {
                    return Err(format!("{cell} contains duplicate sequence {}", record.seq));
                }
            }
            if actual
                .keys()
                .copied()
                .enumerate()
                .any(|(index, seq)| seq != index as u64)
            {
                return Err(format!("{cell} retained records are not a contiguous prefix"));
            }
            if retained.session.last_seq != actual.keys().next_back().copied()
                && !actual.is_empty()
            {
                return Err(format!("{cell} last_seq disagrees with retained records"));
            }
        }
        for execution in &expanded.executions {
            if let Some(record) = actual.remove(&execution.seq) {
                validate_record_against_execution(cell, &expanded, execution, record)?;
                runs.push(record_export(
                    record,
                    receipt.class,
                    owner.ok_or_else(|| format!("{cell} record has no provider owner"))?,
                )?);
                execution_attempted += 1;
                continue;
            }
            let stopped_here = bundle
                .and_then(|retained| retained.session.stop.as_ref())
                .and_then(|stop| stop.seq)
                == Some(execution.seq);
            runs.push(missing_export(
                cell,
                &expanded,
                execution,
                receipt.class,
                if stopped_here { owner } else { None },
                stopped_here,
            )?);
            execution_attempted += usize::from(stopped_here);
        }
        if let Some((seq, _)) = actual.first_key_value() {
            return Err(format!("{cell} contains out-of-plan sequence {seq}"));
        }
        if receipt.class == TerminalClass::Complete
            && runs.iter().rev().take(expanded.executions.len()).any(|run| {
                run["status"] == "cancelled" || run["outcome_class"] == "not_completed"
            })
        {
            return Err(format!("complete {cell} has an incomplete canonical row"));
        }
        if receipt.class == TerminalClass::NotAttempted && bundle.is_some() {
            return Err(format!("not-attempted {cell} unexpectedly has a session"));
        }
    }
    if runs.len() != 636 {
        return Err(format!("canonical export has {} rows, expected 636", runs.len()));
    }
    let run_count = runs.len();
    let completed = runs
        .iter()
        .filter(|run| run["outcome_class"] == "completed")
        .count();
    let interrupted = runs
        .iter()
        .filter(|run| run["status"] == "interrupted")
        .count();
    let mut excluded_by_rule = BTreeMap::<String, Vec<String>>::new();
    for run in &runs {
        if run["exclusion"]["excluded"] == true {
            let rule = run["exclusion"]["rule"]
                .as_str()
                .unwrap_or("unspecified exclusion")
                .to_string();
            if let Some(run_id) = run["run_id"].as_str() {
                excluded_by_rule.entry(rule).or_default().push(run_id.to_string());
            }
        }
    }
    let excluded: Vec<Value> = excluded_by_rule
        .into_iter()
        .map(|(rule, run_ids)| json!({"rule":rule,"run_ids":run_ids}))
        .collect();
    let isolation_tier_max = bundles
        .iter()
        .flat_map(|bundle| bundle.records.iter().map(|record| record.isolation.level))
        .max()
        .map(Level::name);
    let arms: Vec<Value> = identity.protocol["arms"]
        .as_array()
        .ok_or("protocol arms missing")?
        .iter()
        .map(|arm| json!({
            "id": arm["id"],
            "role": arm["role"],
            "binary_sha256": identity.binary_sha256,
            "commit": identity.source_commit,
            "image_digest": identity.image_digest,
        }))
        .collect();
    let output_row = |path: &Path, content: &[u8]| {
        json!({
            "path": path.file_name().and_then(|name| name.to_str()).unwrap_or("output.json"),
            "sha256": sha256_hex(content),
            "bytes": content.len(),
            "kind": "analysis",
        })
    };
    Ok(json!({
        "schema": EXPORT_SCHEMA,
        "export_id": format!("{}-export", identity.protocol_id),
        "created": now_string(),
        "protocol": {
            "path": manifest.protocol,
            "sha256": identity.protocol_sha256,
            "protocol_id": identity.protocol_id,
            "version": identity.protocol["version"],
        },
        "producer": {"runtime":"ecbench-native-analyzer","model":null,"session":null,"role":"analysis_exporter"},
        "source": {"repo":"crypto","commit":identity.source_commit,"dirty":false},
        "backend": "isolab-k8s",
        "host": {
            "path": null,
            "sha256": null,
            "class_id": null,
            "summary": {
                "cpu_model": null,
                "logical_cpus": null,
                "memory_bytes": null,
                "kernel": null,
                "hypervisor": null,
                "isolation_tier_max": isolation_tier_max,
                "instance_type": "c8a.2xlarge",
                "spot": true,
            }
        },
        "arms": arms,
        "runs": runs,
        "flow": {
            "planned": 60 + 288 + 288,
            "attempted": execution_attempted,
            "completed": completed,
            "not_completed": 636 - completed,
            "interrupted": interrupted,
            "resumed": 0,
            "excluded": excluded,
        },
        "files": files,
        "analysis": {
            "script": file_row(&source_path, "examples/ecbench_aimed2lp_analyze.rs".into(), "analysis")?,
            "blinded": false,
            "blinding_key_sha256": null,
            "outputs": [output_row(result_path, result_bytes), output_row(validation_path, validation_bytes)],
            "unmasked_at": null,
        },
        "notes": [
            {"kind":"observation","text":"Whole-cold paired wall is valid only for the frozen c8a.2xlarge L2 workload; pricing bands are descriptive.","by":"native analyzer"},
            {"kind":"observation","text":"Every produced row and authenticated attempt artifact is retained; all planned-but-never-started executions are explicit cancelled rows with the one scheduling attempt required by experiment.export/v1, remain absent from flow.attempted, and stay visible in flow.","by":"native analyzer"}
        ],
        "reviews_dir": "reviews",
    }))
}

fn analyse(
    manifest_path: &Path,
    out_path: &Path,
    validation_path: &Path,
    export_path: &Path,
) -> Result<(
    Value,
    Value,
    Vec<Bundle>,
    Manifest,
    Identity,
    BTreeMap<String, AttemptReceipt>,
), String> {
    let manifest: Manifest = read_typed(manifest_path)?;
    if manifest.schema != "icb.aimed2lp.analysis_manifest/v1" {
        return Err("analysis manifest has the wrong schema".into());
    }
    let base = manifest_path.parent().ok_or("manifest has no parent")?;
    if manifest.outputs.export_schema != EXPORT_SCHEMA || manifest.outputs.expected_rows != 636 {
        return Err("analysis manifest does not require the 636-row canonical export".into());
    }
    for (actual, relative, label) in [
        (out_path, manifest.outputs.results.as_str(), "result"),
        (
            validation_path,
            manifest.outputs.validation.as_str(),
            "validation",
        ),
        (
            export_path,
            manifest.outputs.canonical_export.as_str(),
            "canonical export",
        ),
    ] {
        if prospective_path(actual)? != prospective_path(&base.join(relative))? {
            return Err(format!("{label} output path differs from the frozen manifest"));
        }
    }
    if manifest.sizes.len() != 2
        || manifest.sizes.iter().map(|size| size.n).collect::<BTreeSet<_>>()
            != BTreeSet::from([53, 61])
    {
        return Err("manifest must contain exactly n=53 and n=61".into());
    }
    let identity = load_identity(base, manifest_path, &manifest)?;
    let attempts = check_attempt_receipts(base, &manifest, &identity)?;
    let control_attempt = &attempts["control-n13"];
    if control_attempt.class == TerminalClass::NotAttempted {
        return Err(
            "the positive control cannot be not_attempted after all frozen prerequisite gates pass"
                .into(),
        );
    }
    let control = control_gate(
        base,
        &manifest.positive_control,
        &identity,
        control_attempt,
    )?;
    if control_attempt.class != TerminalClass::Complete
        && manifest.sizes.iter().any(|size| {
            attempts[&format!("n{}", size.n)].class != TerminalClass::NotAttempted
        })
    {
        return Err("a target cell was attempted despite a non-passing positive-control gate".into());
    }
    if control_attempt.class != TerminalClass::Complete {
        for size in &manifest.sizes {
            let receipt = &attempts[&format!("n{}", size.n)];
            if receipt.value["not_attempted_reason"] != TARGET_NOT_ATTEMPTED_REASON {
                return Err(format!(
                    "n={} not-attempted reason does not name the failed positive-control gate",
                    size.n
                ));
            }
        }
    } else if manifest.sizes.iter().any(|size| {
        attempts[&format!("n{}", size.n)].class == TerminalClass::NotAttempted
    }) {
        return Err(
            "a target cell is not_attempted after the positive control passed; authenticated resource refusal must be infrastructure_blocked"
                .into(),
        );
    }
    let mut targets = Vec::new();
    for size in &manifest.sizes {
        targets.push(target_gate(
            base,
            size,
            &identity,
            &attempts[&format!("n{}", size.n)],
        )?);
    }

    let mut estimates = BTreeMap::<(u32, String), Estimate>::new();
    let mut aa = BTreeMap::<u32, Estimate>::new();
    let classes: BTreeMap<u32, TerminalClass> = manifest
        .sizes
        .iter()
        .map(|size| (size.n, attempts[&format!("n{}", size.n)].class))
        .collect();
    for (input, bundle) in manifest.sizes.iter().zip(&targets) {
        if classes[&input.n] != TerminalClass::Complete {
            continue;
        }
        let bundle = bundle
            .as_ref()
            .ok_or_else(|| format!("n={} complete attempt lacks a session", input.n))?;
        let blocks = paired_blocks(&bundle.records, &ARMS, TARGETS)?;
        let aa_clusters = target_clusters(&blocks, "aimed", "aimed-aa", solve_wall)?;
        aa.insert(input.n, estimate(&aa_clusters, 1.0, 2026110100 + input.n as u64)?);
        for (index, (id, arm)) in CANDIDATES.into_iter().enumerate() {
            let clusters = target_clusters(&blocks, "aimed", arm, solve_wall)?;
            estimates.insert(
                (input.n, id.to_string()),
                estimate(
                    &clusters,
                    WALL_THRESHOLD,
                    2026111000 + input.n as u64 * 10 + index as u64,
                )?,
            );
        }
    }
    holm(&mut estimates, 4);

    let mut size_rows = Vec::new();
    let mut candidate_verdicts = BTreeMap::<String, BTreeMap<u32, String>>::new();
    for (input, bundle) in manifest.sizes.iter().zip(&targets) {
        let class = classes[&input.n];
        let blocks = if class == TerminalClass::Complete {
            Some(paired_blocks(
                &bundle.as_ref().ok_or("complete size has no bundle")?.records,
                &ARMS,
                TARGETS,
            )?)
        } else {
            None
        };
        let mut comparisons = Vec::new();
        let aa_row = if let Some(row) = aa.get(&input.n) {
            let stable = row.ci95.0 >= 0.9
                && row.ci95.1 <= 1.1
                && row.ci95.0 <= 1.0
                && row.ci95.1 >= 1.0;
            json!({
                "status": if stable { "pass" } else { "fail" },
                "ratio_b_over_a": row.point,
                "ci95": [row.ci95.0, row.ci95.1],
                "target_clusters": row.clusters,
                "rounds_per_target": ROUNDS,
                "required_interval": [0.9, 1.1],
                "must_contain": 1.0,
            })
        } else {
            json!({
                "status": "not_evaluated",
                "ratio_b_over_a": null,
                "ci95": null,
                "target_clusters": TARGETS,
                "rounds_per_target": ROUNDS,
            })
        };
        for (id, arm) in CANDIDATES {
            let (wall, wall_verdict, pricing) = match class {
                TerminalClass::Complete => {
                    let row = estimates
                        .get(&(input.n, id.to_string()))
                        .ok_or("missing complete candidate estimate")?;
                    let aa_estimate = aa.get(&input.n).ok_or("missing complete A/A estimate")?;
                    let v = verdict(row, aa_estimate);
                    (
                        json!({
                            "status": "complete",
                            "metric": "solve_wall_ns",
                            "ratio_b_over_a": row.point,
                            "ci95": [row.ci95.0, row.ci95.1],
                            "target_clusters": row.clusters,
                            "rounds_per_target": ROUNDS,
                            "ci_method": "target_cluster_percentile_bootstrap",
                            "exact_sign_flip": {
                                "statistic": "sum_over_targets(sum_candidate_rounds - 0.95 * sum_baseline_rounds)",
                                "alternative": "candidate_below_null_ratio",
                                "null_ratio": WALL_THRESHOLD,
                                "sign_assignments": 1usize << TARGETS,
                                "raw_one_sided_p": row.raw_p,
                                "holm_family_size": 4,
                                "holm_adjusted_p": row.adjusted_p,
                            },
                        }),
                        v,
                        pricing_band(
                            blocks.as_ref().unwrap(),
                            "aimed",
                            arm,
                            2026112000 + input.n as u64 + if arm.ends_with("c1") { 1 } else { 0 },
                        ),
                    )
                }
                TerminalClass::ScientificCensored => (
                    json!({
                        "status":"scientific_censored",
                        "metric":"solve_wall_ns",
                        "ratio_b_over_a":null,
                        "ci95":null,
                        "target_clusters":TARGETS,
                        "rounds_per_target":ROUNDS,
                        "exact_sign_flip":null,
                    }),
                    "CENSORED NON-IMPROVEMENT",
                    json!({"status":"NOT_EVALUATED","role":"descriptive_not_decisive"}),
                ),
                TerminalClass::InfrastructureBlocked => (
                    json!({
                        "status":"infrastructure_blocked",
                        "metric":"solve_wall_ns",
                        "ratio_b_over_a":null,
                        "ci95":null,
                        "target_clusters":TARGETS,
                        "rounds_per_target":ROUNDS,
                        "exact_sign_flip":null,
                    }),
                    "BLOCKED",
                    json!({"status":"NOT_EVALUATED","role":"descriptive_not_decisive"}),
                ),
                TerminalClass::NotAttempted => (
                    json!({
                        "status":"not_attempted",
                        "metric":"solve_wall_ns",
                        "ratio_b_over_a":null,
                        "ci95":null,
                        "target_clusters":TARGETS,
                        "rounds_per_target":ROUNDS,
                        "exact_sign_flip":null,
                    }),
                    "NOT ATTEMPTED",
                    json!({"status":"NOT_EVALUATED","role":"descriptive_not_decisive"}),
                ),
            };
            let pricing_status = pricing["status"].clone();
            candidate_verdicts
                .entry(id.to_string())
                .or_default()
                .insert(input.n, wall_verdict.to_string());
            comparisons.push(json!({
                "id": id,
                "a": "aimed",
                "b": arm,
                "wall": wall,
                "wall_verdict": wall_verdict,
                "pricing_band": pricing,
                "pricing_status": pricing_status,
            }));
        }
        let status = match class {
            TerminalClass::Complete => "complete",
            TerminalClass::ScientificCensored => "scientific_censored",
            TerminalClass::InfrastructureBlocked => "infrastructure_blocked",
            TerminalClass::NotAttempted => "not_attempted",
        };
        size_rows.push(json!({
            "n": input.n,
            "status": status,
            "attempt_receipt": input.attempt_receipt,
            "session_export": input.export,
            "session_id": bundle.as_ref().map(|bundle| bundle.session.session_id.clone()),
            "records": bundle.as_ref().map_or(0, |bundle| bundle.records.len()),
            "measured_rows": bundle.as_ref().map_or(0, |bundle| bundle.records.iter().filter(|record| !record.warmup).count()),
            "target_clusters": TARGETS,
            "rounds_per_target": ROUNDS,
            "aa": aa_row,
            "comparisons": comparisons,
        }));
    }
    let overall = if candidate_verdicts
        .values()
        .flat_map(BTreeMap::values)
        .any(|verdict| verdict == "INVALID")
    {
        "INVALID"
    } else if control_attempt.class == TerminalClass::NotAttempted {
        "NOT ATTEMPTED"
    } else if control_attempt.class != TerminalClass::Complete {
        "BLOCKED"
    } else if classes.values().any(|class| *class == TerminalClass::NotAttempted) {
        "NOT ATTEMPTED"
    } else if classes
        .values()
        .any(|class| *class == TerminalClass::InfrastructureBlocked)
    {
        "BLOCKED"
    } else if classes.values().any(|class| *class == TerminalClass::ScientificCensored) {
        "CENSORED NON-IMPROVEMENT"
    } else if candidate_verdicts.values().any(|by_size| {
        by_size.get(&53).map(String::as_str) == Some("IMPROVEMENT")
            && by_size.get(&61).map(String::as_str) == Some("IMPROVEMENT")
    }) {
        "IMPROVEMENT"
    } else {
        "NULL"
    };
    let result_status = match overall {
        "INVALID" => "invalid",
        "BLOCKED" => "blocked",
        "NOT ATTEMPTED" => "not_attempted",
        "CENSORED NON-IMPROVEMENT" => "scientific_censored",
        _ => "complete",
    };
    let mut attempt_rows = Map::new();
    for (cell, receipt) in &attempts {
        attempt_rows.insert(
            cell.clone(),
            json!({
                "path": receipt.relative,
                "sha256": sha256_hex(&bytes(&base.join(&receipt.relative))?),
                "final_class": receipt.value["final_class"],
                "attempts": receipt.value["attempts"].as_array().map_or(0, Vec::len),
            }),
        );
    }
    let created = now_string();
    let result = json!({
        "schema": RESULT_SCHEMA,
        "protocol_id": identity.protocol_id,
        "protocol_sha256": identity.protocol_sha256,
        "native_analyzer_source_sha256": identity.analysis_source_sha256,
        "native_analyzer_binary_sha256": identity.analysis_binary_sha256,
        "native_provider_preflight_source_sha256": identity.provider_preflight_source_sha256,
        "native_provider_preflight_binary_sha256": identity.provider_preflight_binary_sha256,
        "source_commit": identity.source_commit,
        "binary_sha256": identity.binary_sha256,
        "image_digest": identity.image_digest,
        "created": created,
        "status": result_status,
        "controls": {
            "focused_gates": {"status":"pass","receipt":manifest.focused_gates},
            "checkpoint": {"status":"pass","receipt":manifest.checkpoint_receipt},
            "provider_preflight": {"status":"pass","receipt":manifest.provider_preflight},
            "native_analyzer_self_test": {"status":"pass","receipt":manifest.native_analyzer.self_test_receipt},
            "positive_n13": {
                "status": match control_attempt.class {
                    TerminalClass::Complete => "pass",
                    TerminalClass::ScientificCensored => "scientific_censored",
                    TerminalClass::InfrastructureBlocked => "blocked",
                    TerminalClass::NotAttempted => "not_attempted",
                },
                "receipt": manifest.positive_control.receipt,
                "attempt_receipt": manifest.positive_control.attempt_receipt,
            },
            "lp0_identity": if control_attempt.class == TerminalClass::Complete { "pass" } else { "not_evaluated" },
            "aa_operation_identity": if control_attempt.class == TerminalClass::Complete { "pass" } else { "not_evaluated" },
            "direct_two_sum_zero": if control_attempt.class == TerminalClass::Complete { "pass" } else { "not_evaluated" },
            "replay_all": if control_attempt.class == TerminalClass::Complete { "pass" } else { "not_evaluated" },
            "provider_attempt_receipts": "pass",
        },
        "attempt_receipts": attempt_rows,
        "inference": {
            "primary": "whole-cold paired L2 wall on frozen c8a.2xlarge",
            "independent_unit": "public target",
            "target_clusters_per_size": TARGETS,
            "repeated_rounds_per_target": ROUNDS,
            "multiple_testing": "Holm over four candidate cells (2 configurations x 2 sizes)",
            "sensitivity": sensitivity(),
        },
        "sizes": size_rows,
        "overall_verdict": overall,
        "canonical_export": {"schema":EXPORT_SCHEMA,"path":export_path.display().to_string(),"expected_rows":636},
        "limitations": [
            "Finite-host result for the frozen c8a.2xlarge L2 workload only; no cross-hardware or asymptotic claim.",
            "Only eight independent target clusters per size; five rounds are repeated timings within target.",
            "0/0.1/1 pricing is descriptive and remains UNRANKABLE PRICING wherever a named native unit has no numeric count.",
            "One-target complete-cold Table A scope only; no Table C or shared-preprocessing claim.",
        ],
    });
    validate_result_contract(&result)?;
    let validation = json!({
        "schema": VALIDATION_SCHEMA,
        "created": created,
        "valid": overall != "INVALID",
        "status": result_status,
        "findings": if overall == "INVALID" { vec!["A/A wall stability gate failed"] } else { Vec::<&str>::new() },
        "protocol_sha256": identity.protocol_sha256,
        "manifest_sha256": sha256_hex(&bytes(manifest_path)?),
        "native_analyzer_source": manifest.native_analyzer.source,
        "native_analyzer_binary_sha256": identity.analysis_binary_sha256,
        "result_target": out_path.display().to_string(),
        "validation_target": validation_path.display().to_string(),
        "canonical_export": {"schema":EXPORT_SCHEMA,"path":export_path.display().to_string(),"expected_rows":636},
        "attempt_receipts": attempt_rows,
        "exact_sign_flip_raw_p": estimates.iter().map(|((n,id),row)| (format!("n{n}:{id}"), json!(row.raw_p))).collect::<Map<_,_>>(),
        "holm_adjusted_p": estimates.iter().map(|((n,id),row)| (format!("n{n}:{id}"), json!(row.adjusted_p))).collect::<Map<_,_>>(),
        "replay_receipts": control.iter().chain(targets.iter().flatten()).filter_map(|bundle| bundle.audit.as_ref().map(|audit| json!({
            "session_id": bundle.session.session_id,
            "replays": audit.replays.len(),
            "records_bound": audit.records_bound,
            "audit_ok": audit.ok,
        }))).collect::<Vec<_>>(),
    });
    let mut bundles = Vec::new();
    bundles.extend(control);
    bundles.extend(targets.into_iter().flatten());
    Ok((result, validation, bundles, manifest, identity, attempts))
}

fn json_bytes(value: &Value) -> Result<Vec<u8>, String> {
    let mut out = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    out.push(b'\n');
    Ok(out)
}

fn exact_keys(value: &Value, expected: &[&str], what: &str) -> Result<(), String> {
    let actual = value
        .as_object()
        .ok_or_else(|| format!("{what} is not an object"))?
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected = expected
        .iter()
        .map(|key| (*key).to_string())
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(format!("{what} keys differ: {actual:?} != {expected:?}"));
    }
    Ok(())
}

fn nonempty_string(value: &Value) -> bool {
    value.as_str().is_some_and(|text| !text.is_empty())
}

fn numeric_interval(value: &Value) -> bool {
    value
        .as_array()
        .is_some_and(|items| items.len() == 2 && items.iter().all(|item| item.as_f64().is_some()))
}

fn validate_aa_contract(value: &Value) -> Result<(), String> {
    match value["status"].as_str() {
        Some("pass" | "fail") => {
            exact_keys(
                value,
                &[
                    "status", "ratio_b_over_a", "ci95", "target_clusters",
                    "rounds_per_target", "required_interval", "must_contain",
                ],
                "evaluated A/A row",
            )?;
            if value["ratio_b_over_a"].as_f64().is_none()
                || !numeric_interval(&value["ci95"])
                || value["target_clusters"] != 8
                || value["rounds_per_target"] != 5
                || value["required_interval"] != json!([0.9, 1.1])
                || value["must_contain"] != 1.0
            {
                return Err("evaluated A/A row violates the result schema".into());
            }
        }
        Some("not_evaluated") => {
            exact_keys(
                value,
                &["status", "ratio_b_over_a", "ci95", "target_clusters", "rounds_per_target"],
                "unevaluated A/A row",
            )?;
            if !value["ratio_b_over_a"].is_null()
                || !value["ci95"].is_null()
                || value["target_clusters"] != 8
                || value["rounds_per_target"] != 5
            {
                return Err("unevaluated A/A row violates the result schema".into());
            }
        }
        _ => return Err("A/A row has an invalid status".into()),
    }
    Ok(())
}

fn validate_exact_sign_flip(value: &Value) -> Result<(), String> {
    exact_keys(
        value,
        &[
            "statistic", "alternative", "null_ratio", "sign_assignments",
            "raw_one_sided_p", "holm_family_size", "holm_adjusted_p",
        ],
        "exact sign-flip row",
    )?;
    let probability = |field: &str| {
        value[field]
            .as_f64()
            .is_some_and(|number| (0.0..=1.0).contains(&number))
    };
    if value["statistic"]
        != "sum_over_targets(sum_candidate_rounds - 0.95 * sum_baseline_rounds)"
        || value["alternative"] != "candidate_below_null_ratio"
        || value["null_ratio"] != 0.95
        || value["sign_assignments"] != 256
        || !probability("raw_one_sided_p")
        || value["holm_family_size"] != 4
        || !probability("holm_adjusted_p")
    {
        return Err("exact sign-flip row violates the result schema".into());
    }
    Ok(())
}

fn validate_pricing_contract(value: &Value) -> Result<(), String> {
    match value["status"].as_str() {
        Some("NOT_EVALUATED") => {
            exact_keys(value, &["status", "role"], "unevaluated pricing row")?;
            if value["role"] != "descriptive_not_decisive" {
                return Err("unevaluated pricing row has the wrong role".into());
            }
        }
        Some("DESCRIPTIVE COMPLETE" | "UNRANKABLE PRICING") => {
            exact_keys(
                value,
                &[
                    "status", "role", "weights", "ratios", "ci95",
                    "missing_native_count", "missing_unpriced_units", "note",
                ],
                "evaluated pricing row",
            )?;
            let ratios = value["ratios"].as_array();
            let intervals = value["ci95"].as_array();
            let missing = value["missing_native_count"].as_array();
            let units = value["missing_unpriced_units"].as_array();
            if value["role"] != "descriptive_not_decisive"
                || value["weights"] != json!([0.0, 0.1, 1.0])
                || ratios.is_none_or(|items| {
                    items.len() != 3
                        || items.iter().any(|item| !item.is_null() && item.as_f64().is_none())
                })
                || intervals.is_none_or(|items| {
                    items.len() != 3
                        || items.iter().any(|item| !item.is_null() && !numeric_interval(item))
                })
                || missing.is_none_or(|items| {
                    items.len() != 3 || items.iter().any(|item| item.as_bool().is_none())
                })
                || units.is_none_or(|items| {
                    items.iter().any(|item| item.as_str().is_none())
                        || items
                            .iter()
                            .filter_map(Value::as_str)
                            .collect::<BTreeSet<_>>()
                            .len()
                            != items.len()
                })
                || !nonempty_string(&value["note"])
            {
                return Err("evaluated pricing row violates the result schema".into());
            }
        }
        _ => return Err("pricing row has an invalid status".into()),
    }
    Ok(())
}

fn validate_comparison_contract(value: &Value, size_status: &str) -> Result<(), String> {
    exact_keys(
        value,
        &["id", "a", "b", "wall", "wall_verdict", "pricing_band", "pricing_status"],
        "candidate comparison",
    )?;
    let id = value["id"].as_str().ok_or("candidate comparison lacks id")?;
    let expected_arm = match id {
        "C-c05" => "aimed-lp2-c05",
        "C-c1" => "aimed-lp2-c1",
        _ => return Err("candidate comparison has an invalid id".into()),
    };
    if value["a"] != "aimed" || value["b"] != expected_arm {
        return Err(format!("{id} comparison has the wrong arms"));
    }
    let wall = &value["wall"];
    if size_status == "complete" {
        exact_keys(
            wall,
            &[
                "status", "metric", "ratio_b_over_a", "ci95", "target_clusters",
                "rounds_per_target", "ci_method", "exact_sign_flip",
            ],
            "complete wall row",
        )?;
        if wall["status"] != "complete"
            || wall["metric"] != "solve_wall_ns"
            || wall["ratio_b_over_a"].as_f64().is_none()
            || !numeric_interval(&wall["ci95"])
            || wall["target_clusters"] != 8
            || wall["rounds_per_target"] != 5
            || wall["ci_method"] != "target_cluster_percentile_bootstrap"
        {
            return Err("complete wall row violates the result schema".into());
        }
        validate_exact_sign_flip(&wall["exact_sign_flip"])?;
        if !matches!(
            value["wall_verdict"].as_str(),
            Some("IMPROVEMENT" | "NULL" | "WORSE" | "INVALID")
        ) {
            return Err("complete comparison has an invalid verdict".into());
        }
    } else {
        exact_keys(
            wall,
            &[
                "status", "metric", "ratio_b_over_a", "ci95", "target_clusters",
                "rounds_per_target", "exact_sign_flip",
            ],
            "terminal wall row",
        )?;
        let expected_verdict = match size_status {
            "scientific_censored" => "CENSORED NON-IMPROVEMENT",
            "infrastructure_blocked" => "BLOCKED",
            "not_attempted" => "NOT ATTEMPTED",
            _ => return Err("size row has an invalid terminal status".into()),
        };
        if wall["status"] != size_status
            || wall["metric"] != "solve_wall_ns"
            || !wall["ratio_b_over_a"].is_null()
            || !wall["ci95"].is_null()
            || wall["target_clusters"] != 8
            || wall["rounds_per_target"] != 5
            || !wall["exact_sign_flip"].is_null()
            || value["wall_verdict"] != expected_verdict
        {
            return Err("terminal wall row violates the result schema".into());
        }
    }
    validate_pricing_contract(&value["pricing_band"])?;
    if value["pricing_status"] != value["pricing_band"]["status"] {
        return Err("pricing status duplicates the wrong value".into());
    }
    if size_status != "complete" && value["pricing_status"] != "NOT_EVALUATED" {
        return Err("terminal comparison evaluates descriptive pricing".into());
    }
    Ok(())
}

/// A fail-closed native validator for the exact v2 result shape emitted here.
/// The frozen JSON Schema is separately byte-bound by `load_identity`; this
/// check prevents the analyzer from publishing an output it cannot itself
/// account for even when no general-purpose JSON-Schema crate is available.
fn validate_result_contract(result: &Value) -> Result<(), String> {
    exact_keys(
        result,
        &[
            "schema",
            "protocol_id",
            "protocol_sha256",
            "native_analyzer_source_sha256",
            "native_analyzer_binary_sha256",
            "native_provider_preflight_source_sha256",
            "native_provider_preflight_binary_sha256",
            "source_commit",
            "binary_sha256",
            "image_digest",
            "created",
            "status",
            "controls",
            "attempt_receipts",
            "inference",
            "sizes",
            "overall_verdict",
            "canonical_export",
            "limitations",
        ],
        "result",
    )?;
    if result["schema"] != RESULT_SCHEMA
        || result["protocol_id"] != "ICB-AIMED-2LP-N53-N61-20261101"
        || !result["source_commit"].as_str().is_some_and(|commit| {
            commit.len() == 40 && lowercase_hex(commit)
        })
        || !result["image_digest"].as_str().is_some_and(|digest| {
            digest.len() == 71
                && digest.starts_with("sha256:")
                && lowercase_hex(&digest[7..])
        })
        || !nonempty_string(&result["created"])
        || !matches!(
            result["status"].as_str(),
            Some("complete" | "scientific_censored" | "blocked" | "not_attempted" | "invalid")
        )
        || !matches!(
            result["overall_verdict"].as_str(),
            Some(
                "IMPROVEMENT"
                    | "NULL"
                    | "WORSE"
                    | "CENSORED NON-IMPROVEMENT"
                    | "BLOCKED"
                    | "NOT ATTEMPTED"
                    | "INVALID"
            )
        )
    {
        return Err("result has an invalid schema/status/verdict".into());
    }
    for field in [
        "protocol_sha256",
        "native_analyzer_source_sha256",
        "native_analyzer_binary_sha256",
        "native_provider_preflight_source_sha256",
        "native_provider_preflight_binary_sha256",
        "binary_sha256",
    ] {
        if sha256_string(&result[field]).is_none() {
            return Err(format!("result {field} is not a SHA-256"));
        }
    }
    let expected_status = match result["overall_verdict"].as_str().unwrap_or_default() {
        "INVALID" => "invalid",
        "BLOCKED" => "blocked",
        "NOT ATTEMPTED" => "not_attempted",
        "CENSORED NON-IMPROVEMENT" => "scientific_censored",
        _ => "complete",
    };
    if result["status"] != expected_status {
        return Err("result status and overall verdict disagree".into());
    }
    exact_keys(
        &result["controls"],
        &[
            "focused_gates",
            "checkpoint",
            "provider_preflight",
            "native_analyzer_self_test",
            "positive_n13",
            "lp0_identity",
            "aa_operation_identity",
            "direct_two_sum_zero",
            "replay_all",
            "provider_attempt_receipts",
        ],
        "result controls",
    )?;
    for gate in [
        "focused_gates",
        "checkpoint",
        "provider_preflight",
        "native_analyzer_self_test",
    ] {
        let value = &result["controls"][gate];
        exact_keys(value, &["status", "receipt"], gate)?;
        if value["status"] != "pass" || !nonempty_string(&value["receipt"]) {
            return Err(format!("result control {gate} is not a passing receipt"));
        }
    }
    let positive = &result["controls"]["positive_n13"];
    exact_keys(
        positive,
        &["status", "receipt", "attempt_receipt"],
        "positive n13 control",
    )?;
    if !matches!(
        positive["status"].as_str(),
        Some("pass" | "scientific_censored" | "blocked" | "not_attempted")
    ) || !nonempty_string(&positive["receipt"])
        || !nonempty_string(&positive["attempt_receipt"])
    {
        return Err("positive n13 result control is malformed".into());
    }
    for gate in ["lp0_identity", "aa_operation_identity", "direct_two_sum_zero", "replay_all"] {
        if !matches!(
            result["controls"][gate].as_str(),
            Some("pass" | "not_evaluated")
        ) {
            return Err(format!("result control {gate} has an invalid state"));
        }
    }
    if result["controls"]["provider_attempt_receipts"] != "pass" {
        return Err("provider attempt receipts are not passing".into());
    }
    let evaluated = positive["status"] == "pass";
    if ["lp0_identity", "aa_operation_identity", "direct_two_sum_zero", "replay_all"]
        .iter()
        .any(|gate| {
            result["controls"][*gate]
                != if evaluated { "pass" } else { "not_evaluated" }
        })
    {
        return Err("positive-control state disagrees with its dependent gates".into());
    }
    exact_keys(
        &result["attempt_receipts"],
        &["control-n13", "n53", "n61"],
        "result attempt receipts",
    )?;
    for cell in ["control-n13", "n53", "n61"] {
        let summary = &result["attempt_receipts"][cell];
        exact_keys(summary, &["path", "sha256", "final_class", "attempts"], cell)?;
        let class = summary["final_class"].as_str();
        let count = summary["attempts"].as_u64();
        if !nonempty_string(&summary["path"])
            || sha256_string(&summary["sha256"]).is_none()
            || !matches!(
                class,
                Some("complete" | "scientific_censored" | "infrastructure_blocked" | "not_attempted")
            )
            || count.is_none_or(|count| count > 2)
            || (class == Some("not_attempted")) != (count == Some(0))
            || (class != Some("not_attempted") && count.is_none_or(|count| count == 0))
        {
            return Err(format!("result attempt summary {cell} is invalid"));
        }
    }
    let control_summary = &result["attempt_receipts"]["control-n13"];
    let expected_positive_status = match control_summary["final_class"].as_str() {
        Some("complete") => "pass",
        Some("scientific_censored") => "scientific_censored",
        Some("infrastructure_blocked") => "blocked",
        Some("not_attempted") => "not_attempted",
        _ => return Err("control attempt summary has invalid class".into()),
    };
    if positive["attempt_receipt"] != control_summary["path"]
        || positive["status"] != expected_positive_status
    {
        return Err("positive n13 result disagrees with its attempt receipt".into());
    }
    let expected_inference = json!({
        "primary":"whole-cold paired L2 wall on frozen c8a.2xlarge",
        "independent_unit":"public target",
        "target_clusters_per_size":8,
        "repeated_rounds_per_target":5,
        "multiple_testing":"Holm over four candidate cells (2 configurations x 2 sizes)",
        "sensitivity":sensitivity(),
    });
    if result["inference"] != expected_inference {
        return Err("result inference/sensitivity differs from the frozen v2 contract".into());
    }
    let sizes = result["sizes"]
        .as_array()
        .filter(|sizes| sizes.len() == 2)
        .ok_or("result must contain exactly two size rows")?;
    let mut seen = BTreeSet::new();
    let mut size_statuses = Vec::new();
    let mut comparison_verdicts = BTreeMap::<String, BTreeMap<u64, String>>::new();
    for size in sizes {
        exact_keys(
            size,
            &[
                "n",
                "status",
                "attempt_receipt",
                "session_export",
                "session_id",
                "records",
                "measured_rows",
                "target_clusters",
                "rounds_per_target",
                "aa",
                "comparisons",
            ],
            "result size row",
        )?;
        let n = size["n"].as_u64().ok_or("result size lacks n")?;
        if !matches!(n, 53 | 61) || !seen.insert(n) {
            return Err("result size rows are not exactly n53/n61".into());
        }
        let status = size["status"]
            .as_str()
            .filter(|status| {
                matches!(
                    *status,
                    "complete" | "scientific_censored" | "infrastructure_blocked" | "not_attempted"
                )
            })
            .ok_or("result size has invalid status")?;
        size_statuses.push(status.to_string());
        let records = size["records"].as_u64().filter(|count| *count <= 288);
        let measured = size["measured_rows"].as_u64().filter(|count| *count <= 240);
        let session_id_valid = size["session_id"].as_str().is_some_and(|session| {
            session.len() == 18
                && session.starts_with("ECBS1h")
                && lowercase_hex(&session[6..])
        });
        if !nonempty_string(&size["attempt_receipt"])
            || !nonempty_string(&size["session_export"])
            || (!size["session_id"].is_null() && !session_id_valid)
            || records.is_none()
            || measured.is_none()
            || size["target_clusters"] != 8
            || size["rounds_per_target"] != 5
            || (status == "complete"
                && (!session_id_valid || records != Some(288) || measured != Some(240)))
            || (status == "not_attempted"
                && (!size["session_id"].is_null()
                    || records != Some(0)
                    || measured != Some(0)))
        {
            return Err(format!("n{n} result size flow violates the v2 schema"));
        }
        validate_aa_contract(&size["aa"])?;
        if (status == "complete")
            != matches!(size["aa"]["status"].as_str(), Some("pass" | "fail"))
        {
            return Err(format!("n{n} A/A evaluation disagrees with size status"));
        }
        let cell = format!("n{n}");
        let summary = &result["attempt_receipts"][&cell];
        if size["attempt_receipt"] != summary["path"]
            || size["status"] != summary["final_class"]
        {
            return Err(format!("n{n} size row disagrees with its attempt receipt"));
        }
        let comparisons = size["comparisons"]
            .as_array()
            .filter(|rows| rows.len() == 2)
            .ok_or("result size must contain two candidate comparisons")?;
        let ids = comparisons
            .iter()
            .filter_map(|row| row["id"].as_str())
            .collect::<BTreeSet<_>>();
        if ids != BTreeSet::from(["C-c05", "C-c1"]) {
            return Err(format!("n{n} result has the wrong candidate comparisons"));
        }
        for comparison in comparisons {
            validate_comparison_contract(comparison, status)?;
            comparison_verdicts
                .entry(comparison["id"].as_str().unwrap_or_default().to_string())
                .or_default()
                .insert(n, comparison["wall_verdict"].as_str().unwrap_or_default().to_string());
        }
    }
    let expected_overall = if comparison_verdicts
        .values()
        .flat_map(BTreeMap::values)
        .any(|verdict| verdict == "INVALID")
    {
        "INVALID"
    } else if positive["status"] == "not_attempted" {
        "NOT ATTEMPTED"
    } else if positive["status"] != "pass" {
        "BLOCKED"
    } else if size_statuses.iter().any(|status| status == "not_attempted") {
        "NOT ATTEMPTED"
    } else if size_statuses
        .iter()
        .any(|status| status == "infrastructure_blocked")
    {
        "BLOCKED"
    } else if size_statuses
        .iter()
        .any(|status| status == "scientific_censored")
    {
        "CENSORED NON-IMPROVEMENT"
    } else if comparison_verdicts.values().any(|by_size| {
        by_size.get(&53).map(String::as_str) == Some("IMPROVEMENT")
            && by_size.get(&61).map(String::as_str) == Some("IMPROVEMENT")
    }) {
        "IMPROVEMENT"
    } else {
        "NULL"
    };
    if result["overall_verdict"] != expected_overall {
        return Err("overall verdict disagrees with cell-level terminal states".into());
    }
    exact_keys(
        &result["canonical_export"],
        &["schema", "path", "expected_rows"],
        "result canonical export",
    )?;
    if result["canonical_export"]["schema"] != EXPORT_SCHEMA
        || result["canonical_export"]["expected_rows"] != 636
        || result["canonical_export"]["path"]
            .as_str()
            .is_none_or(str::is_empty)
    {
        return Err("result canonical export contract is invalid".into());
    }
    if result["limitations"].as_array().is_none_or(|limitations| {
        limitations.is_empty() || limitations.iter().any(|item| !nonempty_string(item))
    })
    {
        return Err("result limitations are absent".into());
    }
    Ok(())
}

fn required_allowed_keys(
    value: &Value,
    required: &[&str],
    allowed: &[&str],
    what: &str,
) -> Result<(), String> {
    let actual = value
        .as_object()
        .ok_or_else(|| format!("{what} is not an object"))?
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let required = required.iter().copied().collect::<BTreeSet<_>>();
    let allowed = allowed.iter().copied().collect::<BTreeSet<_>>();
    if !required.is_subset(&actual) || !actual.is_subset(&allowed) {
        return Err(format!("{what} keys violate experiment.export/v1"));
    }
    Ok(())
}

fn export_id(value: &Value) -> bool {
    value.as_str().is_some_and(|text| {
        let bytes = text.as_bytes();
        !bytes.is_empty()
            && bytes.len() <= 121
            && bytes[0].is_ascii_alphanumeric()
            && bytes
                .iter()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(*byte, b'.' | b'_' | b'-'))
    })
}

fn nullable_string(value: &Value) -> bool {
    value.is_null() || value.as_str().is_some()
}

fn nullable_sha256(value: &Value) -> bool {
    value.is_null() || sha256_string(value).is_some()
}

fn number_or_string(value: &Value) -> bool {
    value.is_number() || value.as_str().is_some()
}

fn validate_export_file(value: &Value, what: &str) -> Result<(), String> {
    required_allowed_keys(
        value,
        &["path", "sha256", "bytes"],
        &["path", "sha256", "bytes", "kind"],
        what,
    )?;
    if !nonempty_string(&value["path"])
        || sha256_string(&value["sha256"]).is_none()
        || value["bytes"].as_u64().is_none()
        || (!value["kind"].is_null()
            && !matches!(
                value["kind"].as_str(),
                Some(
                    "raw"
                        | "log"
                        | "summary"
                        | "host"
                        | "checkpoint_manifest"
                        | "analysis"
                        | "protocol"
                        | "report"
                        | "receipt"
                        | "other"
                )
            ))
    {
        return Err(format!("{what} violates the experiment.export/v1 file shape"));
    }
    Ok(())
}

fn validate_export_top_level(export: &Value) -> Result<(), String> {
    if !export_id(&export["export_id"])
        || export["created"].as_str().is_none_or(|created| created.len() < 10)
        || !matches!(
            export["backend"].as_str(),
            Some(
                "local"
                    | "taskq"
                    | "isolab"
                    | "isolab-k8s"
                    | "spotlab"
                    | "spot-system"
                    | "ephemeral"
                    | "modal"
                    | "runpod"
                    | "cairn"
                    | "slurm"
                    | "manual"
                    | "ecbench"
                    | "runbook"
                    | "csd-lab"
                    | "mixed"
            )
        )
        || !nonempty_string(&export["reviews_dir"])
    {
        return Err("canonical export has invalid identity/backend metadata".into());
    }

    required_allowed_keys(
        &export["protocol"],
        &["path", "sha256", "protocol_id", "version"],
        &["path", "sha256", "protocol_id", "version", "seed_nonce"],
        "canonical protocol binding",
    )?;
    if !nonempty_string(&export["protocol"]["path"])
        || sha256_string(&export["protocol"]["sha256"]).is_none()
        || !export_id(&export["protocol"]["protocol_id"])
        || export["protocol"]["version"].as_u64().is_none_or(|version| version < 1)
        || (!export["protocol"]["seed_nonce"].is_null()
            && !nonempty_string(&export["protocol"]["seed_nonce"]))
    {
        return Err("canonical protocol binding violates experiment.export/v1".into());
    }

    exact_keys(
        &export["producer"],
        &["runtime", "model", "session", "role"],
        "canonical producer",
    )?;
    if ["runtime", "model", "session", "role"]
        .iter()
        .any(|field| !nullable_string(&export["producer"][*field]))
    {
        return Err("canonical producer violates experiment.export/v1".into());
    }

    exact_keys(
        &export["source"],
        &["repo", "commit", "dirty"],
        "canonical source",
    )?;
    if !nonempty_string(&export["source"]["repo"])
        || (!export["source"]["commit"].is_null()
            && !export["source"]["commit"].as_str().is_some_and(|commit| {
                commit.len() == 40 && lowercase_hex(commit)
            }))
        || (!export["source"]["dirty"].is_null()
            && export["source"]["dirty"].as_bool().is_none())
    {
        return Err("canonical source violates experiment.export/v1".into());
    }

    exact_keys(
        &export["host"],
        &["path", "sha256", "class_id", "summary"],
        "canonical host",
    )?;
    if !nullable_string(&export["host"]["path"])
        || !nullable_sha256(&export["host"]["sha256"])
        || !nullable_string(&export["host"]["class_id"])
    {
        return Err("canonical host identity violates experiment.export/v1".into());
    }
    required_allowed_keys(
        &export["host"]["summary"],
        &[
            "cpu_model",
            "logical_cpus",
            "memory_bytes",
            "kernel",
            "hypervisor",
            "isolation_tier_max",
        ],
        &[
            "cpu_model",
            "logical_cpus",
            "memory_bytes",
            "kernel",
            "hypervisor",
            "isolation_tier_max",
            "gpu",
            "instance_type",
            "spot",
        ],
        "canonical host summary",
    )?;
    let summary = &export["host"]["summary"];
    if ["cpu_model", "kernel", "hypervisor", "isolation_tier_max", "gpu", "instance_type"]
        .iter()
        .any(|field| !summary[*field].is_null() && summary[*field].as_str().is_none())
        || ["logical_cpus", "memory_bytes"]
            .iter()
            .any(|field| !summary[*field].is_null() && summary[*field].as_u64().is_none())
        || (!summary["spot"].is_null() && summary["spot"].as_bool().is_none())
    {
        return Err("canonical host summary violates experiment.export/v1".into());
    }

    let arms = export["arms"]
        .as_array()
        .filter(|arms| !arms.is_empty())
        .ok_or("canonical export has no arms")?;
    for arm in arms {
        required_allowed_keys(
            arm,
            &["id", "role", "binary_sha256", "commit"],
            &["id", "role", "binary_sha256", "commit", "image_digest"],
            "canonical arm",
        )?;
        if !export_id(&arm["id"])
            || !matches!(
                arm["role"].as_str(),
                Some(
                    "reference"
                        | "baseline"
                        | "candidate"
                        | "control_aa"
                        | "control_positive"
                        | "control_negative"
                )
            )
            || !nullable_sha256(&arm["binary_sha256"])
            || (!arm["commit"].is_null()
                && !arm["commit"].as_str().is_some_and(|commit| {
                    commit.len() == 40 && lowercase_hex(commit)
                }))
            || !nullable_string(&arm["image_digest"])
        {
            return Err("canonical arm violates experiment.export/v1".into());
        }
    }

    let files = export["files"]
        .as_array()
        .ok_or("canonical export files is not an array")?;
    for file in files {
        validate_export_file(file, "canonical file")?;
    }
    exact_keys(
        &export["analysis"],
        &["script", "blinded", "blinding_key_sha256", "outputs", "unmasked_at"],
        "canonical analysis",
    )?;
    if !export["analysis"]["script"].is_null() {
        validate_export_file(&export["analysis"]["script"], "canonical analysis script")?;
    }
    if export["analysis"]["blinded"].as_bool().is_none()
        || !nullable_sha256(&export["analysis"]["blinding_key_sha256"])
        || !nullable_string(&export["analysis"]["unmasked_at"])
    {
        return Err("canonical analysis metadata violates experiment.export/v1".into());
    }
    for output in export["analysis"]["outputs"]
        .as_array()
        .ok_or("canonical analysis outputs is not an array")?
    {
        validate_export_file(output, "canonical analysis output")?;
    }
    for note in export["notes"]
        .as_array()
        .ok_or("canonical export notes is not an array")?
    {
        exact_keys(note, &["kind", "text", "by"], "canonical note")?;
        if note["kind"] != "observation"
            || !nonempty_string(&note["text"])
            || !nonempty_string(&note["by"])
        {
            return Err("canonical note violates experiment.export/v1".into());
        }
    }
    Ok(())
}

fn validate_export_contract(export: &Value) -> Result<(), String> {
    exact_keys(
        export,
        &[
            "schema", "export_id", "created", "protocol", "producer", "source", "backend",
            "host", "arms", "runs", "flow", "files", "analysis", "notes", "reviews_dir",
        ],
        "canonical export",
    )?;
    if export["schema"] != EXPORT_SCHEMA {
        return Err("canonical export has the wrong schema".into());
    }
    validate_export_top_level(export)?;
    let runs = export["runs"]
        .as_array()
        .filter(|runs| runs.len() == 636)
        .ok_or("canonical export must contain exactly 636 scheduled rows")?;
    let arm_ids = export["arms"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|arm| arm["id"].as_str())
        .collect::<BTreeSet<_>>();
    let mut ids = BTreeSet::new();
    let mut completed = 0usize;
    let mut attempted = 0usize;
    let mut interrupted = 0usize;
    let mut expected_excluded = BTreeSet::<String>::new();
    for run in runs {
        exact_keys(
            run,
            &[
                "run_id", "arm", "instance_id", "seed", "replicate", "warmup", "status",
                "outcome_class", "attempts", "resumed", "checkpoint", "measurements",
                "isolation", "verification", "exclusion", "artifacts", "started_at",
                "finished_at", "worker", "backend_ref",
            ],
            "canonical run row",
        )?;
        let id = run["run_id"]
            .as_str()
            .filter(|id| !id.is_empty())
            .ok_or("canonical row lacks run_id")?;
        if !ids.insert(id) {
            return Err(format!("canonical export repeats run_id {id}"));
        }
        let status = run["status"].as_str().ok_or("canonical row lacks status")?;
        if !matches!(
            status,
            "succeeded" | "failed" | "timeout" | "cancelled" | "infra_error" | "interrupted"
        ) || !matches!(run["outcome_class"].as_str(), Some("completed" | "not_completed"))
            || !nonempty_string(&run["arm"])
            || !arm_ids.contains(run["arm"].as_str().unwrap_or_default())
            || !nonempty_string(&run["instance_id"])
            || (!run["seed"].is_null()
                && run["seed"].as_str().is_none()
                && run["seed"].as_u64().is_none())
            || run["replicate"].as_u64().is_none()
            || run["warmup"].as_bool().is_none()
            || run["attempts"].as_u64().is_none_or(|count| count > 1)
            || run["resumed"] != 0
            || (!run["started_at"].is_null() && !nonempty_string(&run["started_at"]))
            || (!run["finished_at"].is_null() && !nonempty_string(&run["finished_at"]))
            || (status == "succeeded" && run["outcome_class"] != "completed")
            || (matches!(status, "timeout" | "cancelled" | "infra_error" | "interrupted")
                && run["outcome_class"] != "not_completed")
        {
            return Err(format!("canonical row {id} has malformed core fields"));
        }
        exact_keys(
            &run["checkpoint"],
            &[
                "policy", "generations_written", "restored_from_generation", "manifest_sha256",
            ],
            "canonical checkpoint row",
        )?;
        if run["checkpoint"]["policy"] != "none"
            || run["checkpoint"]["generations_written"] != 0
            || !run["checkpoint"]["restored_from_generation"].is_null()
            || !run["checkpoint"]["manifest_sha256"].is_null()
        {
            return Err(format!("canonical row {id} claims a forbidden restore/checkpoint"));
        }
        exact_keys(
            &run["measurements"],
            &[
                "counted", "wall_seconds", "wall_valid", "cpu_user_seconds", "cpu_sys_seconds",
                "max_rss_bytes", "counters",
            ],
            "canonical measurements",
        )?;
        exact_keys(
            &run["isolation"],
            &["tier", "contended", "blockers"],
            "canonical isolation",
        )?;
        exact_keys(
            &run["verification"],
            &["status", "verifier"],
            "canonical verification",
        )?;
        exact_keys(
            &run["exclusion"],
            &["excluded", "rule"],
            "canonical exclusion",
        )?;
        if run["measurements"]["wall_valid"].as_bool().is_none()
            || run["measurements"]["counted"].as_object().is_none()
            || run["measurements"]["counters"].as_object().is_none()
            || (!run["measurements"]["wall_seconds"].is_null()
                && run["measurements"]["wall_seconds"].as_f64().is_none())
            || (!run["measurements"]["cpu_user_seconds"].is_null()
                && run["measurements"]["cpu_user_seconds"].as_f64().is_none())
            || (!run["measurements"]["cpu_sys_seconds"].is_null()
                && run["measurements"]["cpu_sys_seconds"].as_f64().is_none())
            || (!run["measurements"]["max_rss_bytes"].is_null()
                && run["measurements"]["max_rss_bytes"].as_u64().is_none())
            || (!run["isolation"]["tier"].is_null()
                && !nonempty_string(&run["isolation"]["tier"]))
            || (!run["isolation"]["contended"].is_null()
                && run["isolation"]["contended"].as_bool().is_none())
            || run["isolation"]["blockers"].as_array().is_none_or(|items| {
                items.iter().any(|item| item.as_str().is_none())
            })
            || run["artifacts"].as_array().is_none()
            || !matches!(
                run["verification"]["status"].as_str(),
                Some("verified" | "refuted" | "no_claim" | "error" | "not_run")
            )
            || (!run["verification"]["verifier"].is_null()
                && !nonempty_string(&run["verification"]["verifier"]))
            || run["exclusion"]["excluded"].as_bool().is_none()
            || (run["exclusion"]["excluded"] == true
                && !nonempty_string(&run["exclusion"]["rule"]))
            || (run["exclusion"]["excluded"] == false
                && !run["exclusion"]["rule"].is_null())
            || (!run["worker"].is_null() && !nonempty_string(&run["worker"]))
            || (!run["backend_ref"].is_null() && !nonempty_string(&run["backend_ref"]))
        {
            return Err(format!("canonical row {id} has malformed evidence fields"));
        }
        for field in ["counted", "counters"] {
            if run["measurements"][field]
                .as_object()
                .is_some_and(|values| values.values().any(|value| !number_or_string(value)))
            {
                return Err(format!("canonical row {id} has a malformed {field} value"));
            }
        }
        for artifact in run["artifacts"].as_array().into_iter().flatten() {
            validate_export_file(artifact, &format!("canonical row {id} artifact"))?;
        }
        if status != "cancelled" {
            attempted += 1;
        }
        if run["outcome_class"] == "completed" {
            completed += 1;
        }
        interrupted += usize::from(status == "interrupted");
        if run["exclusion"]["excluded"] == true {
            expected_excluded.insert(id.to_string());
        }
        if matches!(status, "cancelled" | "timeout" | "infra_error" | "interrupted")
            && (run["measurements"]["wall_valid"] != false
                || run["exclusion"]["excluded"] != true)
        {
            return Err(format!("non-completed canonical row {id} admits a wall clock"));
        }
        if status == "cancelled"
            && (run["attempts"] != 1
                || run["verification"]["status"] != "not_run"
                || !run["measurements"]["wall_seconds"].is_null()
                || !run["started_at"].is_null()
                || !run["finished_at"].is_null())
        {
            return Err(format!("cancelled canonical row {id} looks executed"));
        }
        if status != "cancelled" && run["attempts"].as_u64().is_none_or(|count| count == 0) {
            return Err(format!("attempted canonical row {id} has zero attempts"));
        }
    }
    let flow = &export["flow"];
    exact_keys(
        flow,
        &[
            "planned", "attempted", "completed", "not_completed", "interrupted", "resumed",
            "excluded",
        ],
        "canonical export flow",
    )?;
    let excluded_groups = flow["excluded"]
        .as_array()
        .ok_or("canonical export exclusion ledger is not an array")?;
    let mut listed_excluded = BTreeSet::<String>::new();
    for group in excluded_groups {
        exact_keys(group, &["rule", "run_ids"], "canonical exclusion group")?;
        if !nonempty_string(&group["rule"]) {
            return Err("canonical exclusion group has no rule".into());
        }
        let run_ids = group["run_ids"]
            .as_array()
            .ok_or("canonical exclusion group has no run_ids")?;
        for value in run_ids {
            let id = value
                .as_str()
                .filter(|id| !id.is_empty())
                .ok_or("canonical exclusion group contains an invalid run_id")?;
            if !listed_excluded.insert(id.to_string()) {
                return Err(format!("canonical exclusion ledger repeats {id}"));
            }
        }
    }
    if listed_excluded != expected_excluded {
        return Err("canonical exclusion ledger disagrees with run rows".into());
    }
    if flow["planned"] != 636
        || flow["attempted"] != attempted as u64
        || flow["completed"] != completed as u64
        || flow["not_completed"] != (636 - completed) as u64
        || flow["interrupted"] != interrupted as u64
        || flow["resumed"] != 0
    {
        return Err("canonical export flow disagrees with its 636 rows".into());
    }
    Ok(())
}

fn write_new(path: &Path, content: &[u8]) -> Result<(), String> {
    if path.exists() {
        return Err(format!("{} already exists", path.display()));
    }
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("output has no UTF-8 file name")?;
    let temporary = path.with_file_name(format!(".{name}.tmp.{}", std::process::id()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| format!("{}: {e}", temporary.display()))?;
    file.write_all(content)
        .and_then(|_| file.sync_all())
        .map_err(|e| format!("{}: {e}", temporary.display()))?;
    drop(file);
    let linked = fs::hard_link(&temporary, path)
        .map_err(|e| format!("{}: atomic publish: {e}", path.display()));
    let removed = fs::remove_file(&temporary)
        .map_err(|e| format!("{}: temporary cleanup: {e}", temporary.display()));
    linked?;
    removed
}

fn self_test() -> Result<(), String> {
    let clusters = vec![(100.0, 80.0); TARGETS];
    let row = estimate(&clusters, WALL_THRESHOLD, 7)?;
    if (row.point - 0.8).abs() > 1e-12
        || row.ci95 != (0.8, 0.8)
        || row.raw_p != 1.0 / 256.0
    {
        return Err(format!(
            "exact target-cluster fixture differs: point={} ci={:?} p={}",
            row.point, row.ci95, row.raw_p
        ));
    }
    let mut family = BTreeMap::new();
    for (index, p) in [0.01, 0.03, 0.2, 0.5].into_iter().enumerate() {
        family.insert(
            (53 + index as u32, format!("c{index}")),
            Estimate {
                point: 1.0,
                ci95: (0.9, 1.1),
                raw_p: p,
                adjusted_p: f64::NAN,
                clusters: TARGETS,
            },
        );
    }
    holm(&mut family, 4);
    if (family[&(53, "c0".into())].adjusted_p - 0.04).abs() > 1e-12
        || family[&(54, "c1".into())].adjusted_p < 0.09
    {
        return Err("Holm self-test differs".into());
    }
    if protocol_unique_json(r#"{"a":1,"a":2}"#, 0).is_ok() {
        return Err("duplicate protocol keys were accepted".into());
    }
    let parsed = protocol_unique_json(r#"{"b":0.1,"a":1}"#, 0)?;
    if protocol_canonical_bytes(&parsed)? != br#"{"a":1,"b":0.1}"# {
        return Err("canonical protocol bytes differ".into());
    }
    if sha256_string(&json!("A".repeat(64))).is_some() {
        return Err("uppercase SHA-256 passed the lowercase schema contract".into());
    }
    if valid_argv(&json!(["ecbench", ""])) {
        return Err("empty frozen argv element passed request validation".into());
    }

    let mut lp_activity = BTreeMap::from([
        ("aimed_lp_direct_two_sum".into(), 0),
        ("aimed_lp_verification_failures".into(), 0),
        ("aimed_lp_partials_fed".into(), 1),
        ("lp_projected_relations".into(), 1),
        ("aimed_lp_merged_relations".into(), 1),
        ("aimed_lp_partial_scan_trials".into(), 2),
        ("aimed_lp_partial_summands_scanned".into(), 17),
        ("summands_scanned".into(), 49),
    ]);
    lp_counter_gate("lp-activity-self-test", &lp_activity, true)?;
    for required in [
        "aimed_lp_partial_scan_trials",
        "aimed_lp_partial_summands_scanned",
    ] {
        let saved = lp_activity.insert(required.into(), 0).unwrap();
        if lp_counter_gate("lp-activity-self-test", &lp_activity, true).is_ok() {
            return Err(format!("zero {required} passed the active LP gate"));
        }
        lp_activity.insert(required.into(), saved);
    }
    lp_activity.insert("summands_scanned".into(), 16);
    if lp_counter_gate("lp-activity-self-test", &lp_activity, true).is_ok() {
        return Err("partial LP scan exceeded total summands without rejection".into());
    }

    let digest = "0".repeat(64);
    let template = json!({
        "argv":["ecbench","run","--spec","frozen.json"],
        "provider_identity_sha256":digest,
        "pod_spec_sha256":digest,
    });
    let terminal = json!({
        "classification":"scientific_complete",
        "reason":"fixture",
        "exit_code":0,
        "started":"unix:1",
        "finished":"unix:2",
    });
    let artifact = |path: &str| {
        json!({
            "path":format!("sessions/fixture/{path}"),
            "sha256":digest,
            "bytes":1,
            "object_uri":format!("s3://setec-astronomy-artifacts/ecbench/icb-aimed-large-prime-n53-n61-20261101/n53/{path}"),
            "object_version":"fixture-version",
            "readback_sha256":digest,
            "readback_bytes":1,
            "readback_verified":true,
        })
    };
    let artifacts = json!([
        artifact("provider-identity.json"),
        artifact("pod-spec.json"),
        artifact("records.jsonl"),
    ]);
    let lineage = json!({"restart_count":0});
    let mut envelope = json!({
        "lineage":lineage,
        "provider_identity_artifact":"sessions/fixture/provider-identity.json",
        "pod_spec_artifact":"sessions/fixture/pod-spec.json",
        "request":template,
        "terminal":terminal,
        "artifacts":artifacts,
        "authentication":{
            "provider_identity_sha256":digest,
            "pod_spec_sha256":digest,
            "lineage_object_sha256":canonical_sha256(&lineage)?,
            "request_object_sha256":canonical_sha256(&template)?,
            "terminal_object_sha256":canonical_sha256(&terminal)?,
            "artifact_listing_sha256":canonical_sha256(&artifacts)?,
        },
    });
    let prefix = "s3://setec-astronomy-artifacts/ecbench/icb-aimed-large-prime-n53-n61-20261101/n53/";
    validate_attempt_envelope(&template, &envelope, prefix)?;
    envelope["lineage"]["restart_count"] = json!(1);
    if validate_attempt_envelope(&template, &envelope, prefix).is_ok() {
        return Err("nonzero pod restart passed the attempt-envelope test".into());
    }
    envelope["lineage"]["restart_count"] = json!(0);
    envelope["request"]["argv"][0] = json!("tampered");
    envelope["authentication"]["request_object_sha256"] =
        json!(canonical_sha256(&envelope["request"])?);
    if validate_attempt_envelope(&template, &envelope, prefix).is_ok() {
        return Err("tampered argv passed the attempt-envelope test".into());
    }
    envelope["request"] = template.clone();
    envelope["authentication"]["request_object_sha256"] =
        json!(canonical_sha256(&envelope["request"])?);
    envelope["artifacts"][0]["object_uri"] = json!("s3://wrong-bucket/records.jsonl");
    envelope["authentication"]["artifact_listing_sha256"] =
        json!(canonical_sha256(&envelope["artifacts"])?);
    if validate_attempt_envelope(&template, &envelope, prefix).is_ok() {
        return Err("wrong durable URI passed the attempt-envelope test".into());
    }
    envelope["artifacts"][0]["object_uri"] = json!(format!("{prefix}records.jsonl"));
    envelope["artifacts"][0]["readback_bytes"] = json!(2);
    envelope["authentication"]["artifact_listing_sha256"] =
        json!(canonical_sha256(&envelope["artifacts"])?);
    if validate_attempt_envelope(&template, &envelope, prefix).is_ok() {
        return Err("bad durable readback passed the attempt-envelope test".into());
    }

    if validate_retained_terminal_shape(
        "complete",
        288,
        288,
        None,
        None,
        None,
        TerminalClass::Complete,
        TerminalClass::ScientificCensored,
    )
    .is_ok()
    {
        return Err("complete all-verified session accepted a censored terminal class".into());
    }
    validate_retained_terminal_shape(
        "interrupted",
        288,
        12,
        Some("interrupt"),
        Some(12),
        Some("provider signal"),
        TerminalClass::Complete,
        TerminalClass::InfrastructureBlocked,
    )?;
    if validate_retained_terminal_shape(
        "interrupted",
        288,
        12,
        None,
        None,
        None,
        TerminalClass::Complete,
        TerminalClass::InfrastructureBlocked,
    )
    .is_ok()
    {
        return Err("interrupted session without an authenticated stop passed".into());
    }
    if validate_retained_terminal_shape(
        "interrupted",
        288,
        12,
        Some("interrupt"),
        Some(12),
        Some("provider signal"),
        TerminalClass::Complete,
        TerminalClass::ScientificCensored,
    )
    .is_ok()
    {
        return Err("clean interrupted prefix was relabeled scientific_censored".into());
    }
    if validate_retained_terminal_shape(
        "interrupted",
        288,
        12,
        Some("spawn_error"),
        Some(13),
        Some("spawn failed"),
        TerminalClass::Complete,
        TerminalClass::InfrastructureBlocked,
    )
    .is_ok()
    {
        return Err("interrupted session with a skipped stop sequence passed".into());
    }
    let duplicate_lineage = json!({
        "request_id":"request-1",
        "job_uid":"job-1",
        "pod_uid":"pod-1",
    });
    let mut fixture_request_ids = BTreeSet::new();
    let mut fixture_job_uids = BTreeSet::new();
    let mut fixture_pod_uids = BTreeSet::new();
    register_lineage_ids(
        &duplicate_lineage,
        &mut fixture_request_ids,
        &mut fixture_job_uids,
        &mut fixture_pod_uids,
    )?;
    if register_lineage_ids(
        &duplicate_lineage,
        &mut fixture_request_ids,
        &mut fixture_job_uids,
        &mut fixture_pod_uids,
    )
    .is_ok()
    {
        return Err("duplicate cross-cell provider lineage passed".into());
    }
    let semantic_provider = json!({
        "schema":"icb.aimed2lp.provider_identity/v1",
        "provider_tool":"isolab_k8s_benchmark",
        "cloud_account_id":"123456789012",
        "region":"us-east-2",
        "cluster":"isolab",
        "namespace":"isolab",
        "service_account":"runner",
        "admission_controller":"isolab-eks-mcp",
    });
    let mut semantic_template = json!({
        "provider_tool":"isolab_k8s_benchmark",
        "job_name":"fixture-job",
        "image_uri":format!("registry.example/tasks@sha256:{digest}"),
        "image_digest":format!("sha256:{digest}"),
        "argv":["ecbench","run","--spec","frozen.json"],
        "payload_sha256":digest,
        "payload_bytes":1,
        "provider_identity_sha256":null,
        "pod_spec_sha256":null,
        "instance_type":"c8a.2xlarge",
        "capacity":"spot",
        "node_selector":{
            "node.kubernetes.io/instance-type":"c8a.2xlarge",
            "eks.amazonaws.com/capacityType":"SPOT",
        },
        "exclusive_vcpus":4,
        "cpu_request":"4",
        "cpu_limit":"4",
        "memory":"8Gi",
        "memory_bytes":8_589_934_592u64,
        "memory_request":"8Gi",
        "memory_limit":"8Gi",
        "restart_policy":"Never",
        "child_timeout_seconds":3_600,
        "outer_deadline_seconds":7_800,
    });
    let provider_bytes = protocol_canonical_bytes(&semantic_provider)?;
    semantic_template["provider_identity_sha256"] = json!(sha256_hex(&provider_bytes));
    let mut semantic_pod = expected_pod_job_spec(&semantic_template);
    for (name, drifted) in [
        (
            "argv",
            {
                let mut value = semantic_pod.clone();
                value["argv"][0] = json!("tampered");
                value
            },
        ),
        (
            "outer deadline",
            {
                let mut value = semantic_pod.clone();
                value["outer_deadline_seconds"] = json!(7_801);
                value
            },
        ),
    ] {
        if validate_pod_job_spec(&semantic_template, &drifted).is_ok() {
            return Err(format!("Pod/Job {name} drift passed semantic validation"));
        }
    }
    let pod_bytes = protocol_canonical_bytes(&semantic_pod)?;
    semantic_template["pod_spec_sha256"] = json!(sha256_hex(&pod_bytes));
    let semantic_root = std::env::temp_dir().join(format!(
        "ecbench-aimed2lp-selftest-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| error.to_string())?
            .as_nanos()
    ));
    fs::create_dir(&semantic_root)
        .map_err(|error| format!("{}: {error}", semantic_root.display()))?;
    let semantic_check = (|| -> Result<(), String> {
        fs::write(semantic_root.join("provider.json"), &provider_bytes)
            .map_err(|error| error.to_string())?;
        fs::write(semantic_root.join("pod.json"), &pod_bytes)
            .map_err(|error| error.to_string())?;
        let semantic_attempt = json!({
            "provider_identity_artifact":"provider.json",
            "pod_spec_artifact":"pod.json",
        });
        validate_provider_and_pod_artifacts(
            &semantic_root,
            &semantic_template,
            &semantic_attempt,
        )?;
        semantic_pod["resources"]["cpu_limit"] = json!("5");
        let mutated = protocol_canonical_bytes(&semantic_pod)?;
        fs::write(semantic_root.join("pod.json"), &mutated)
            .map_err(|error| error.to_string())?;
        semantic_template["pod_spec_sha256"] = json!(sha256_hex(&mutated));
        if validate_provider_and_pod_artifacts(
            &semantic_root,
            &semantic_template,
            &semantic_attempt,
        )
        .is_ok()
        {
            return Err("resource-drifted canonical Pod artifact passed semantic validation".into());
        }
        Ok(())
    })();
    let cleanup = fs::remove_dir_all(&semantic_root)
        .map_err(|error| format!("{}: {error}", semantic_root.display()));
    semantic_check?;
    cleanup?;

    let incomplete_custody = ["session.json", "plan.json", "spec.json", "records.jsonl"]
        .into_iter()
        .map(|name| json!({"path":format!("sessions/n53/{name}")}))
        .collect::<Vec<_>>();
    if require_session_core_custody(
        &incomplete_custody,
        "sessions/n53",
        "adversarial fixture",
    )
    .is_ok()
    {
        return Err("session custody without host.json passed".into());
    }
    let censored_core_custody = [
        "session.json",
        "host.json",
        "plan.json",
        "spec.json",
        "records.jsonl",
    ]
    .into_iter()
    .map(|name| json!({"path":format!("sessions/control-n13/{name}")}))
    .collect::<Vec<_>>();
    require_session_core_custody(
        &censored_core_custody,
        "sessions/control-n13",
        "censored control fixture",
    )?;
    if require_complete_session_custody(
        &censored_core_custody,
        "sessions/control-n13",
        "sessions/control-n13/replay-all.json",
        "sessions/control-n13/export.json",
        &["sessions/control-n13/CONTROL.json".into()],
        "succeeded control fixture",
    )
    .is_ok()
    {
        return Err("succeeded control without decisive receipts passed custody".into());
    }
    let duplicate_owners = vec![
        json!({"artifacts":[{"path":"sessions/n53/replay-all.json"}]}),
        json!({"artifacts":[{"path":"sessions/n53/replay-all.json"}]}),
    ];
    if require_unique_attempt_owner(
        &duplicate_owners,
        "sessions/n53/replay-all.json",
        "replay fixture",
    )
    .is_ok()
    {
        return Err("decisive artifact with two attempt owners passed custody".into());
    }
    let cross_cell_consumed = BTreeSet::from(["sessions/n53/replay-all.json".to_string()]);
    let cross_cell_owners = BTreeMap::from([(
        "sessions/n53/replay-all.json".to_string(),
        2usize,
    )]);
    if validate_global_artifact_owners(&cross_cell_consumed, &cross_cell_owners).is_ok() {
        return Err("cross-cell duplicate owner passed global custody".into());
    }

    let attempt_summary = |cell: &str| {
        json!({
            "path":format!("attempts/{cell}.json"),
            "sha256":digest,
            "final_class":"complete",
            "attempts":1,
        })
    };
    let comparison_fixture = |id: &str, arm: &str| {
        json!({
            "id":id,
            "a":"aimed",
            "b":arm,
            "wall":{
                "status":"complete",
                "metric":"solve_wall_ns",
                "ratio_b_over_a":1.0,
                "ci95":[0.9,1.1],
                "target_clusters":8,
                "rounds_per_target":5,
                "ci_method":"target_cluster_percentile_bootstrap",
                "exact_sign_flip":{
                    "statistic":"sum_over_targets(sum_candidate_rounds - 0.95 * sum_baseline_rounds)",
                    "alternative":"candidate_below_null_ratio",
                    "null_ratio":0.95,
                    "sign_assignments":256,
                    "raw_one_sided_p":0.5,
                    "holm_family_size":4,
                    "holm_adjusted_p":1.0,
                },
            },
            "wall_verdict":"NULL",
            "pricing_band":{
                "status":"DESCRIPTIVE COMPLETE",
                "role":"descriptive_not_decisive",
                "weights":[0.0,0.1,1.0],
                "ratios":[1.0,1.0,1.0],
                "ci95":[[1.0,1.0],[1.0,1.0],[1.0,1.0]],
                "missing_native_count":[false,false,false],
                "missing_unpriced_units":[],
                "note":"fixture",
            },
            "pricing_status":"DESCRIPTIVE COMPLETE",
        })
    };
    let size_fixture = |n: u32| {
        json!({
            "n":n,
            "status":"complete",
            "attempt_receipt":format!("attempts/n{n}.json"),
            "session_export":format!("sessions/n{n}/export.json"),
            "session_id":"ECBS1h000000000000",
            "records":288,
            "measured_rows":240,
            "target_clusters":8,
            "rounds_per_target":5,
            "aa":{
                "status":"pass","ratio_b_over_a":1.0,"ci95":[0.9,1.1],
                "target_clusters":8,"rounds_per_target":5,
                "required_interval":[0.9,1.1],"must_contain":1.0
            },
            "comparisons":[
                comparison_fixture("C-c05","aimed-lp2-c05"),
                comparison_fixture("C-c1","aimed-lp2-c1")
            ],
        })
    };
    let mut result_fixture = json!({
        "schema":RESULT_SCHEMA,
        "protocol_id":"ICB-AIMED-2LP-N53-N61-20261101",
        "protocol_sha256":digest,
        "native_analyzer_source_sha256":digest,
        "native_analyzer_binary_sha256":digest,
        "native_provider_preflight_source_sha256":digest,
        "native_provider_preflight_binary_sha256":digest,
        "source_commit":"0".repeat(40),
        "binary_sha256":digest,
        "image_digest":format!("sha256:{digest}"),
        "created":"unix:1",
        "status":"complete",
        "controls":{
            "focused_gates":{"status":"pass","receipt":"receipts/focused-gates.json"},
            "checkpoint":{"status":"pass","receipt":"receipts/checkpoint.json"},
            "provider_preflight":{"status":"pass","receipt":"receipts/provider-preflight.json"},
            "native_analyzer_self_test":{"status":"pass","receipt":"receipts/native-analyzer-tests.json"},
            "positive_n13":{
                "status":"pass","receipt":"sessions/control-n13/CONTROL.json",
                "attempt_receipt":"attempts/control-n13.json"
            },
            "lp0_identity":"pass","aa_operation_identity":"pass",
            "direct_two_sum_zero":"pass","replay_all":"pass",
            "provider_attempt_receipts":"pass"
        },
        "attempt_receipts":{
            "control-n13":attempt_summary("control-n13"),
            "n53":attempt_summary("n53"),
            "n61":attempt_summary("n61"),
        },
        "inference":{
            "primary":"whole-cold paired L2 wall on frozen c8a.2xlarge",
            "independent_unit":"public target",
            "target_clusters_per_size":8,
            "repeated_rounds_per_target":5,
            "multiple_testing":"Holm over four candidate cells (2 configurations x 2 sizes)",
            "sensitivity":sensitivity()
        },
        "sizes":[size_fixture(53),size_fixture(61)],
        "overall_verdict":"NULL",
        "canonical_export":{"schema":EXPORT_SCHEMA,"path":"export.json","expected_rows":636},
        "limitations":["fixture"],
    });
    validate_result_contract(&result_fixture)?;
    result_fixture["source_commit"] = json!("A".repeat(40));
    if validate_result_contract(&result_fixture).is_ok() {
        return Err("uppercase result commit passed the v2 schema contract".into());
    }
    result_fixture["source_commit"] = json!("0".repeat(40));
    result_fixture["unexpected"] = json!(true);
    if validate_result_contract(&result_fixture).is_ok() {
        return Err("extra result field passed the v2 contract test".into());
    }
    result_fixture.as_object_mut().unwrap().remove("unexpected");
    result_fixture["canonical_export"]["expected_rows"] = json!(635);
    if validate_result_contract(&result_fixture).is_ok() {
        return Err("wrong result export denominator passed the v2 contract test".into());
    }
    result_fixture["canonical_export"]["expected_rows"] = json!(636);
    result_fixture["sizes"][0]["comparisons"][0]["wall"]["exact_sign_flip"]
        ["sign_assignments"] = json!(255);
    if validate_result_contract(&result_fixture).is_ok() {
        return Err("nested sign-flip drift passed the v2 contract test".into());
    }

    let runs = (0..636)
        .map(|seq| {
            json!({
                "run_id":format!("planned-fixture-{seq:03}"),
                "arm":"aimed",
                "instance_id":format!("W{seq:012x}"),
                "seed":"1",
                "replicate":0,
                "warmup":false,
                "status":"cancelled",
                "outcome_class":"not_completed",
                "attempts":1,
                "resumed":0,
                "checkpoint":{"policy":"none","generations_written":0,"restored_from_generation":null,"manifest_sha256":null},
                "measurements":{
                    "counted":{},"wall_seconds":null,"wall_valid":false,
                    "cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,
                    "counters":{}
                },
                "isolation":{"tier":null,"contended":null,"blockers":["not attempted"]},
                "verification":{"status":"not_run","verifier":null},
                "exclusion":{"excluded":true,"rule":"not attempted"},
                "artifacts":[],
                "started_at":null,
                "finished_at":null,
                "worker":null,
                "backend_ref":null,
            })
        })
        .collect::<Vec<_>>();
    let excluded_run_ids = (0..636)
        .map(|seq| json!(format!("planned-fixture-{seq:03}")))
        .collect::<Vec<_>>();
    let mut export_fixture = json!({
        "schema":EXPORT_SCHEMA,
        "export_id":"fixture",
        "created":"unix:000001",
        "protocol":{
            "path":"protocol/protocol.json",
            "sha256":digest,
            "protocol_id":"fixture-protocol",
            "version":1,
        },
        "producer":{
            "runtime":"ecbench-native-analyzer",
            "model":null,
            "session":null,
            "role":"analysis_exporter",
        },
        "source":{"repo":"crypto","commit":"0".repeat(40),"dirty":false},
        "backend":"isolab-k8s",
        "host":{
            "path":null,
            "sha256":null,
            "class_id":null,
            "summary":{
                "cpu_model":null,
                "logical_cpus":null,
                "memory_bytes":null,
                "kernel":null,
                "hypervisor":null,
                "isolation_tier_max":null,
                "instance_type":"c8a.2xlarge",
                "spot":true,
            },
        },
        "arms":[{
            "id":"aimed",
            "role":"baseline",
            "binary_sha256":digest,
            "commit":"0".repeat(40),
            "image_digest":format!("sha256:{digest}"),
        }],
        "runs":runs,
        "flow":{
            "planned":636,"attempted":0,"completed":0,"not_completed":636,
            "interrupted":0,"resumed":0,
            "excluded":[{"rule":"not attempted","run_ids":excluded_run_ids}]
        },
        "files":[],
        "analysis":{
            "script":null,
            "blinded":false,
            "blinding_key_sha256":null,
            "outputs":[],
            "unmasked_at":null,
        },
        "notes":[],
        "reviews_dir":"reviews",
    });
    validate_export_contract(&export_fixture)?;
    export_fixture["protocol"]["sha256"] = json!("A".repeat(64));
    if validate_export_contract(&export_fixture).is_ok() {
        return Err("uppercase canonical export SHA-256 passed v1".into());
    }
    export_fixture["protocol"]["sha256"] = json!(digest.clone());
    export_fixture["backend"] = json!("fixture");
    if validate_export_contract(&export_fixture).is_ok() {
        return Err("invalid canonical backend passed the v1 contract test".into());
    }
    export_fixture["backend"] = json!("isolab-k8s");
    export_fixture["producer"]["unexpected"] = json!(true);
    if validate_export_contract(&export_fixture).is_ok() {
        return Err("extra canonical producer field passed the v1 contract test".into());
    }
    export_fixture["producer"].as_object_mut().unwrap().remove("unexpected");
    export_fixture["files"] = json!([{
        "path":"fixture.bin",
        "sha256":digest,
        "bytes":1,
        "kind":"invented",
    }]);
    if validate_export_contract(&export_fixture).is_ok() {
        return Err("invalid canonical file kind passed the v1 contract test".into());
    }
    export_fixture["files"] = json!([]);
    validate_export_contract(&export_fixture)?;
    export_fixture["runs"][0]["measurements"]["wall_valid"] = json!(true);
    if validate_export_contract(&export_fixture).is_ok() {
        return Err("cancelled row with valid wall passed the export contract test".into());
    }
    export_fixture["runs"][0]["measurements"]["wall_valid"] = json!(false);
    export_fixture["runs"][0]["attempts"] = json!(0);
    if validate_export_contract(&export_fixture).is_ok() {
        return Err("cancelled row with zero canonical scheduling attempts passed v1".into());
    }
    export_fixture["runs"][0]["attempts"] = json!(1);
    export_fixture["runs"].as_array_mut().unwrap().pop();
    if validate_export_contract(&export_fixture).is_ok() {
        return Err("635-row export passed the fixed denominator test".into());
    }
    Ok(())
}

struct Args {
    manifest: PathBuf,
    out: PathBuf,
    validation: PathBuf,
    export: PathBuf,
    self_test: bool,
}

fn args() -> Result<Args, String> {
    let default = PathBuf::from("research/icb_aimed_large_prime_n53_n61_20261101");
    let mut out = Args {
        manifest: default.join("analysis-manifest.json"),
        out: default.join("results.json"),
        validation: default.join("VALIDATION.json"),
        export: default.join("export.json"),
        self_test: false,
    };
    let values: Vec<String> = std::env::args().skip(1).collect();
    let mut index = 0usize;
    while index < values.len() {
        match values[index].as_str() {
            "--self-test" => {
                out.self_test = true;
                index += 1;
            }
            "--manifest" | "--out" | "--validation" | "--export" => {
                let flag = values[index].clone();
                let value = values
                    .get(index + 1)
                    .ok_or_else(|| format!("{flag} needs a path"))?;
                match flag.as_str() {
                    "--manifest" => out.manifest = value.into(),
                    "--out" => out.out = value.into(),
                    "--validation" => out.validation = value.into(),
                    "--export" => out.export = value.into(),
                    _ => unreachable!(),
                }
                index += 2;
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    Ok(out)
}

fn run() -> Result<(), String> {
    let args = args()?;
    if args.self_test {
        self_test()?;
        for check in SELF_TEST_CHECKS {
            println!("PASS: {check}");
        }
        println!("OK: native aimed-2LP analyzer self-test");
        return Ok(());
    }
    for path in [&args.out, &args.validation, &args.export] {
        if path.exists() {
            return Err(format!("{} already exists", path.display()));
        }
    }
    let (result, validation, bundles, manifest, identity, attempts) =
        analyse(&args.manifest, &args.out, &args.validation, &args.export)?;
    let result_bytes = json_bytes(&result)?;
    let validation_bytes = json_bytes(&validation)?;
    let base = args.manifest.parent().ok_or("manifest has no parent")?;
    let refs: Vec<&Bundle> = bundles.iter().collect();
    let export = build_export(
        base,
        &args.manifest,
        &manifest,
        &identity,
        &attempts,
        &refs,
        &result_bytes,
        &validation_bytes,
        &args.out,
        &args.validation,
    )?;
    validate_export_contract(&export)?;
    let export_bytes = json_bytes(&export)?;
    write_new(&args.out, &result_bytes)?;
    write_new(&args.validation, &validation_bytes)?;
    write_new(&args.export, &export_bytes)?;
    println!("OK: {} ({})", args.out.display(), result["overall_verdict"]);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("REFUSED: {error}");
        std::process::exit(2);
    }
}

fn sensitivity() -> Value {
    json!({
        "independent_target_clusters": TARGETS,
        "repeated_rounds_per_cluster": ROUNDS,
        "exact_sign_patterns": 1usize << TARGETS,
        "minimum_raw_p": 1.0 / (1usize << TARGETS) as f64,
        "minimum_four_test_holm_p": 4.0 / (1usize << TARGETS) as f64,
        "equal_magnitude_power_if_each_target_favors_candidate": {
            "p_0.80": 0.8f64.powi(TARGETS as i32),
            "p_0.90": 0.9f64.powi(TARGETS as i32),
            "p_0.95": 0.95f64.powi(TARGETS as i32)
        },
        "limitation": "The displayed power is the exact all-favorable probability for equal-magnitude target contrasts under the Holm worst rank; heterogeneous effect magnitudes can change power."
    })
}
