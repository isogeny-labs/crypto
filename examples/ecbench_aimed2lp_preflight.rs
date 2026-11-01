//! Native pre-data provider/topology preflight for ICB-AIMED-2LP-N53-N61-20261101.
//!
//! This resolves each ecbench spec through the production planner, binds the
//! exact provider wrapper source, and reproduces the wrapper's first-reference /
//! first-control selection.  It then calls production `choose_aa`, which is the
//! path that populates `wall.aa_pairs` before run-ecbench's exit-43 gate.

use crypto_lib::cryptanalysis::ecbench::compare::choose_aa;
use crypto_lib::cryptanalysis::ecbench::spec::{plan, Level, Role, Spec};
use crypto_lib::ml_dataset::{protocol_canonical_bytes, protocol_unique_json};
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const SCHEMA: &str = "icb.aimed2lp.provider_preflight/v1";
const STUDY: &str = "research/icb_aimed_large_prime_n53_n61_20261101";
const WRAPPER: &str = "taskq/deploy/eks-karpenter/run-ecbench";
const COMPARE: &str = "src/cryptanalysis/ecbench/compare.rs";
const SELF: &str = "examples/ecbench_aimed2lp_preflight.rs";

#[derive(Serialize)]
struct SourceBinding {
    path: String,
    sha256: String,
}

#[derive(Serialize)]
struct RequestContract {
    instance_type: &'static str,
    capacity: &'static str,
    node_selector: BTreeMap<&'static str, &'static str>,
    exclusive_vcpus: u64,
    cpu_request: &'static str,
    cpu_limit: &'static str,
    memory: &'static str,
    memory_bytes: u64,
    memory_request: &'static str,
    memory_limit: &'static str,
    restart_policy: &'static str,
    child_timeout_max_seconds: u64,
    outer_job_deadline_seconds: u64,
    estimated_session_seconds: u64,
    artifact_upload_grace_seconds: u64,
    provider_retries: u64,
    scientific_retries: u64,
    infrastructure_attempts_max: u64,
}

#[derive(Serialize)]
struct SpecReceipt {
    #[serde(skip)]
    request_status: String,
    path: String,
    file_sha256: String,
    spec_id: String,
    spec_sha256: String,
    request_template_path: String,
    request_template_sha256: String,
    payload_sha256: Option<String>,
    payload_bytes: Option<u64>,
    argv: Option<Vec<String>>,
    provider_identity_sha256: Option<String>,
    pod_spec_sha256: Option<String>,
    durable_prefix: String,
    first_reference: String,
    first_control: String,
    aa_denominator: String,
    aa_numerator: String,
    aa_selection: String,
    scheduled_aa_pairs: u64,
    measured_rows: u64,
    rounds: u32,
    warmup: u32,
    targets: u64,
    child_timeout_seconds: u64,
    isolation_required: &'static str,
}

#[derive(Serialize)]
struct Receipt {
    schema: &'static str,
    created: String,
    status: String,
    source_sha256: String,
    binary_sha256: String,
    wrapper: SourceBinding,
    compare_implementation: SourceBinding,
    request: RequestContract,
    specs: Vec<SpecReceipt>,
    gate: &'static str,
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
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

fn fixed_hex(value: &serde_json::Value, length: usize) -> bool {
    value.as_str().is_some_and(|text| {
        text.len() == length
            && text
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

fn now_string() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    format!("unix:{seconds}")
}

fn read(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn bind(root: &Path, rel: &str) -> Result<SourceBinding, String> {
    let bytes = read(&root.join(rel))?;
    Ok(SourceBinding {
        path: rel.into(),
        sha256: sha256(&bytes),
    })
}

fn role_name(role: Role) -> &'static str {
    match role {
        Role::Reference => "reference",
        Role::Baseline => "baseline",
        Role::Candidate => "candidate",
        Role::Control => "control",
    }
}

fn preflight_spec(root: &Path, name: &str) -> Result<SpecReceipt, String> {
    let rel = format!("{STUDY}/specs/{name}.json");
    let bytes = read(&root.join(&rel))?;
    let text = std::str::from_utf8(&bytes).map_err(|e| format!("{rel}: {e}"))?;
    protocol_unique_json(text, 0).map_err(|e| format!("{rel}: {e}"))?;
    let p = plan(Spec::from_json(text)?)?;

    let request_rel = format!("{STUDY}/requests/{name}.json");
    let request_bytes = read(&root.join(&request_rel))?;
    let request_text =
        std::str::from_utf8(&request_bytes).map_err(|e| format!("{request_rel}: {e}"))?;
    let request = protocol_unique_json(request_text, 0)
        .map_err(|e| format!("{request_rel}: {e}"))?;
    let request_keys = request
        .as_object()
        .ok_or_else(|| format!("{request_rel}: request is not an object"))?
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected_keys = [
        "schema", "status", "cell", "provider_tool", "job_name", "spec_path",
        "spec_file_sha256", "source_revision", "image_uri", "image_digest",
        "ecbench_sha256", "argv", "payload_sha256", "payload_bytes",
        "provider_identity_sha256", "pod_spec_sha256", "instance_type", "capacity",
        "node_selector", "exclusive_vcpus", "cpu_request", "cpu_limit", "memory",
        "memory_bytes", "memory_request", "memory_limit", "restart_policy",
        "child_timeout_seconds", "outer_deadline_seconds", "durable_prefix",
        "provider_retries", "scientific_retries", "infrastructure_attempts_max",
        "retain_failed_artifacts",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    if request_keys != expected_keys {
        return Err(format!("{request_rel}: request keys drift"));
    }
    let request_status = request["status"]
        .as_str()
        .ok_or_else(|| format!("{request_rel}: status is absent"))?;
    let frozen_identity = fixed_hex(&request["source_revision"], 40)
        && request["image_uri"]
            .as_str()
            .is_some_and(|value| value.contains("@sha256:"))
        && request["image_digest"].as_str().is_some_and(|value| {
            value.len() == 71
                && value.starts_with("sha256:")
                && value[7..]
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        })
        && fixed_hex(&request["ecbench_sha256"], 64)
        && request["argv"].as_array().is_some_and(|value| {
            !value.is_empty()
                && value
                    .iter()
                    .all(|arg| arg.as_str().is_some_and(|arg| !arg.is_empty()))
        })
        && fixed_hex(&request["payload_sha256"], 64)
        && request["payload_bytes"].as_u64().is_some_and(|value| value > 0)
        && fixed_hex(&request["provider_identity_sha256"], 64)
        && fixed_hex(&request["pod_spec_sha256"], 64);
    let digest_bound_image = request["image_uri"]
        .as_str()
        .zip(request["image_digest"].as_str())
        .is_some_and(|(image, digest)| image.ends_with(&format!("@{digest}")));
    let expected_pod_spec_sha256 = sha256(&protocol_canonical_bytes(
        &expected_pod_job_spec(&request),
    )?);
    let draft_identity = [
        "source_revision",
        "image_uri",
        "image_digest",
        "ecbench_sha256",
        "argv",
        "payload_sha256",
        "payload_bytes",
        "provider_identity_sha256",
        "pod_spec_sha256",
    ]
    .iter()
    .all(|field| request[*field].is_null());
    let node_selector_exact = request["node_selector"].as_object().is_some_and(|selector| {
        selector.keys().map(String::as_str).collect::<BTreeSet<_>>()
            == BTreeSet::from([
                "node.kubernetes.io/instance-type",
                "eks.amazonaws.com/capacityType",
            ])
    });
    if !matches!(request_status, "draft_identity_blocked" | "frozen")
        || (request_status == "frozen") != frozen_identity
        || (request_status == "draft_identity_blocked") != draft_identity
        || (request_status == "frozen" && !digest_bound_image)
        || (request_status == "frozen"
            && protocol_canonical_bytes(&request)? != request_bytes)
        || (request_status == "frozen"
            && (request["payload_sha256"] != sha256(&bytes)
                || request["payload_bytes"] != bytes.len() as u64
                || request["pod_spec_sha256"] != expected_pod_spec_sha256))
        || request["schema"] != "icb.aimed2lp.request/v1"
        || request["cell"] != name
        || request["provider_tool"] != "isolab_k8s_benchmark"
        || request["spec_path"] != rel
        || request["spec_file_sha256"] != sha256(&bytes)
        || request["instance_type"] != "c8a.2xlarge"
        || request["capacity"] != "spot"
        || !node_selector_exact
        || request["node_selector"]["node.kubernetes.io/instance-type"] != "c8a.2xlarge"
        || request["node_selector"]["eks.amazonaws.com/capacityType"] != "SPOT"
        || request["exclusive_vcpus"] != 4
        || request["cpu_request"] != "4"
        || request["cpu_limit"] != "4"
        || request["memory"] != "8Gi"
        || request["memory_bytes"] != 8_589_934_592u64
        || request["memory_request"] != "8Gi"
        || request["memory_limit"] != "8Gi"
        || request["restart_policy"] != "Never"
        || request["child_timeout_seconds"] != p.spec.measurement.timeout_seconds
        || request["outer_deadline_seconds"] != 7_800
        || request["durable_prefix"]
            != format!(
                "s3://setec-astronomy-artifacts/ecbench/icb-aimed-large-prime-n53-n61-20261101/{name}/"
            )
        || request["provider_retries"] != 0
        || request["scientific_retries"] != 0
        || request["infrastructure_attempts_max"] != 2
        || request["retain_failed_artifacts"] != true
    {
        return Err(format!("{request_rel}: request/template drift"));
    }

    if p.spec.measurement.isolation_required != Level::L2
        || p.spec.measurement.rounds < 5
        || p.spec.measurement.warmup < 1
        || p.spec.measurement.timeout_seconds > 3_600
    {
        return Err(format!(
            "{rel}: provider requires L2, >=5 rounds, >=1 warmup and child timeout <=3600"
        ));
    }

    // These two `find`s intentionally match run-ecbench's `jq ... | head -1`.
    let reference = p
        .arms
        .iter()
        .find(|a| a.role == Role::Reference)
        .ok_or_else(|| format!("{rel}: no reference"))?;
    let control = p
        .arms
        .iter()
        .find(|a| a.role == Role::Control)
        .ok_or_else(|| format!("{rel}: no control"))?;

    let arms: Vec<(String, String, String)> = p
        .arms
        .iter()
        .map(|a| {
            (
                a.name.clone(),
                a.method.method_id.clone(),
                role_name(a.role).into(),
            )
        })
        .collect();
    let (aa_den, aa_num, why) = choose_aa(&arms, &reference.name, &control.name)
        .ok_or_else(|| format!("{rel}: chosen comparison has no distinct same-method A/A pair"))?;

    let targets = p.spec.workloads.targets_per_curve
        * u64::try_from(p.spec.workloads.curves.len()).map_err(|e| e.to_string())?;
    let scheduled_aa_pairs = targets * u64::from(p.spec.measurement.rounds);
    if scheduled_aa_pairs < 5 {
        return Err(format!(
            "{rel}: {scheduled_aa_pairs} scheduled A/A pairs cannot satisfy exit 43"
        ));
    }

    let expected = match name {
        "control-n13" => ("aimed", "aimed-lp0", "aimed", "aimed-aa"),
        "n53" | "n61" => ("rho-v2", "aimed-aa", "aimed", "aimed-aa"),
        _ => return Err(format!("unexpected spec {name}")),
    };
    if (
        reference.name.as_str(),
        control.name.as_str(),
        aa_den.as_str(),
        aa_num.as_str(),
    ) != expected
    {
        return Err(format!(
            "{rel}: topology drift: selected ({}, {}) with A/A ({}, {}), expected {expected:?}",
            reference.name, control.name, aa_den, aa_num
        ));
    }

    let measured_rows = targets
        * u64::from(p.spec.measurement.rounds)
        * u64::try_from(p.arms.len()).map_err(|e| e.to_string())?;
    let argv = request["argv"].as_array().map(|values| {
        values
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| format!("{request_rel}: argv contains a non-string"))
            })
            .collect::<Result<Vec<_>, _>>()
    }).transpose()?;
    Ok(SpecReceipt {
        request_status: request_status.into(),
        path: rel,
        file_sha256: sha256(&bytes),
        spec_id: p.spec_id,
        spec_sha256: p.spec_sha256,
        request_template_path: request_rel.clone(),
        request_template_sha256: sha256(&request_bytes),
        payload_sha256: request["payload_sha256"].as_str().map(str::to_owned),
        payload_bytes: request["payload_bytes"].as_u64(),
        argv,
        provider_identity_sha256: request["provider_identity_sha256"].as_str().map(str::to_owned),
        pod_spec_sha256: request["pod_spec_sha256"].as_str().map(str::to_owned),
        durable_prefix: request["durable_prefix"]
            .as_str()
            .ok_or_else(|| format!("{request_rel}: durable_prefix is absent"))?
            .into(),
        first_reference: reference.name.clone(),
        first_control: control.name.clone(),
        aa_denominator: aa_den,
        aa_numerator: aa_num,
        aa_selection: why.into(),
        scheduled_aa_pairs,
        measured_rows,
        rounds: p.spec.measurement.rounds,
        warmup: p.spec.measurement.warmup,
        targets,
        child_timeout_seconds: p.spec.measurement.timeout_seconds,
        isolation_required: p.spec.measurement.isolation_required.name(),
    })
}

fn parse_args() -> Result<(PathBuf, Option<PathBuf>), String> {
    let mut root = None;
    let mut out = None;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--repo" => root = Some(PathBuf::from(args.next().ok_or("--repo needs a path")?)),
            "--out" => out = Some(PathBuf::from(args.next().ok_or("--out needs a path")?)),
            "--help" | "-h" => {
                return Err("usage: ecbench_aimed2lp_preflight --repo REPO [--out RECEIPT.json]".into())
            }
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    Ok((root.ok_or("--repo is required")?, out))
}

fn run() -> Result<(), String> {
    let (root, out) = parse_args()?;
    let wrapper_bytes = read(&root.join(WRAPPER))?;
    let wrapper_text = std::str::from_utf8(&wrapper_bytes).map_err(|e| e.to_string())?;
    for required in [
        "select(.role==\"reference\")",
        "select(.role==\"control\")",
        ".wall.aa_pairs>=5",
        ".wall.aa_ci95!=null",
    ] {
        if !wrapper_text.contains(required) {
            return Err(format!("provider wrapper no longer contains `{required}`"));
        }
    }

    let specs = ["control-n13", "n53", "n61"]
        .iter()
        .map(|name| preflight_spec(&root, name))
        .collect::<Result<Vec<_>, _>>()?;
    let status = if specs
        .iter()
        .all(|spec| spec.request_status == "frozen")
    {
        "pass"
    } else {
        "draft_pass"
    };
    let receipt = Receipt {
        schema: SCHEMA,
        created: now_string(),
        status: status.into(),
        source_sha256: sha256(&read(&root.join(SELF))?),
        binary_sha256: sha256(&read(
            &env::current_exe().map_err(|error| format!("current preflight binary: {error}"))?,
        )?),
        wrapper: bind(&root, WRAPPER)?,
        compare_implementation: bind(&root, COMPARE)?,
        request: RequestContract {
            instance_type: "c8a.2xlarge",
            capacity: "spot",
            node_selector: BTreeMap::from([
                ("node.kubernetes.io/instance-type", "c8a.2xlarge"),
                ("eks.amazonaws.com/capacityType", "SPOT"),
            ]),
            exclusive_vcpus: 4,
            cpu_request: "4",
            cpu_limit: "4",
            memory: "8Gi",
            memory_bytes: 8_589_934_592,
            memory_request: "8Gi",
            memory_limit: "8Gi",
            restart_policy: "Never",
            child_timeout_max_seconds: 3_600,
            outer_job_deadline_seconds: 7_800,
            estimated_session_seconds: 6_600,
            artifact_upload_grace_seconds: 1_200,
            provider_retries: 0,
            scientific_retries: 0,
            infrastructure_attempts_max: 2,
        },
        specs,
        gate: "run-ecbench first reference/control resolve; production choose_aa returns the frozen aimed/aimed-aa pair; scheduled aa_pairs >= 5. Runtime L2 admission and aa_ci95 remain control/target gates.",
    };
    let bytes = serde_json::to_vec_pretty(&receipt).map_err(|e| e.to_string())?;
    if let Some(path) = out {
        fs::write(&path, &bytes).map_err(|e| format!("{}: {e}", path.display()))?;
    } else {
        println!("{}", std::str::from_utf8(&bytes).map_err(|e| e.to_string())?);
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ecbench_aimed2lp_preflight: {e}");
        std::process::exit(2);
    }
}
