//! One-shot, fail-closed submission of a suspended dataset Job.
//! The spendable worker grant is deliberately issued later, after observed UIDs.

use clap::Parser;
use crypto_lib::ml_dataset::budget::{CampaignLedger, CampaignPolicy, LaunchReservation};
use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

type Result<T> = std::result::Result<T, String>;

#[derive(Parser)]
#[command(about = "Submit one suspended, budget-reserved ECDLP dataset Job")]
struct Args {
    #[arg(long)]
    policy: PathBuf,
    #[arg(long)]
    ledger: PathBuf,
    #[arg(long)]
    reservation: PathBuf,
    #[arg(long)]
    access_approval: PathBuf,
    /// Root-owned, non-writable trust anchor provisioned by the approving operator.
    #[arg(long)]
    approval_trust_anchor: PathBuf,
    /// Independently reviewed SHA-256 of the access approval's exact bytes.
    #[arg(long)]
    trusted_approval_sha256: String,
    #[arg(long)]
    job: PathBuf,
    #[arg(long)]
    plan: PathBuf,
    #[arg(long)]
    protocol: PathBuf,
    #[arg(long)]
    startup_config: PathBuf,
    /// Independently priced, exact Job resource evidence bound by the native policy.
    #[arg(long)]
    resource_envelope: PathBuf,
    #[arg(long)]
    crypto_bin: PathBuf,
    /// A new, durable directory on the authoritative controller host.
    #[arg(long)]
    intent_dir: PathBuf,
    #[arg(long)]
    kubectl: PathBuf,
    /// Root-owned protected kubeconfig pinned for every Kubernetes call.
    #[arg(long)]
    kubeconfig: PathBuf,
    /// Exact Job UID observed independently after an ambiguous create.
    #[arg(long)]
    reconcile_uid: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AccessApproval {
    schema: String,
    campaign_id: String,
    cluster_arn: String,
    kubectl_context: String,
    cluster_server: String,
    cluster_ca_sha256: String,
    namespace: String,
    job_name: String,
    service_account: String,
    credential_role_arn: String,
    configmap_name: String,
    configmap_uid: String,
    config_data_sha256: String,
    mailbox_name: String,
    mailbox_uid: String,
    image: String,
    destination_bucket: String,
    destination_prefix: String,
    policy_sha256: String,
    reservation_sha256: String,
    job_sha256: String,
    expires_unix_seconds: u64,
    max_active_deadline_seconds: u64,
    issuer_evidence_sha256: String,
    allow_suspended_submit: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TrustAnchor {
    schema: String,
    approval_sha256: String,
}

fn protected_root_file(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let mut current = Some(path);
        while let Some(p) = current {
            let meta = fs::symlink_metadata(p).map_err(|e| e.to_string())?;
            if meta.uid() != 0
                || meta.permissions().mode() & 0o022 != 0
                || meta.file_type().is_symlink()
            {
                return Err("approval trust anchor and every ancestor must be root-owned, non-writable directories or a regular file".into());
            }
            if p == path && !meta.is_file() || p != path && !meta.is_dir() {
                return Err("invalid trust anchor path type".into());
            }
            current = p.parent().filter(|parent| *parent != p);
        }
    }
    #[cfg(not(unix))]
    {
        return Err("protected approval trust anchor requires Unix ownership checks".into());
    }
    if !path.is_absolute() {
        return Err("protected path must be absolute".into());
    }
    Ok(())
}
fn trusted_approval_digest(path: &Path) -> Result<String> {
    protected_root_file(path)?;
    let bytes = read_bounded(path, 4096)?;
    let anchor: TrustAnchor = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if anchor.schema != "crypto.ml-dataset.submit-trust-anchor/v1" {
        return Err("unknown approval trust anchor schema".into());
    }
    check_sha(&anchor.approval_sha256)?;
    Ok(anchor.approval_sha256)
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PricedEnvelope {
    schema: String,
    job_resources: Value,
    node_selector: Value,
    active_deadline_seconds: u64,
    billable_resource_seconds: u64,
    billable_resource_count: u32,
    compute_rate_micro_usd_per_hour: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Intent {
    schema: String,
    status: String,
    campaign_id: String,
    launch_id: String,
    cluster_arn: String,
    namespace: String,
    job_name: String,
    policy_sha256: String,
    reservation_sha256: String,
    approval_sha256: String,
    job_sha256: String,
    job_uid: Option<String>,
}

fn sha(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn check_sha(value: &str) -> Result<()> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err("expected lowercase SHA-256 digest".into());
    }
    Ok(())
}
fn read_bounded(path: &Path, max: u64) -> Result<Vec<u8>> {
    let meta = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !meta.file_type().is_file() || meta.len() > max {
        return Err(format!("invalid bounded input: {}", path.display()));
    }
    fs::read(path).map_err(|e| e.to_string())
}
fn value(path: &Path, max: u64) -> Result<(Value, Vec<u8>)> {
    let bytes = read_bounded(path, max)?;
    let parsed = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    Ok((parsed, bytes))
}
fn required<'a>(v: &'a Value, pointer: &str) -> Result<&'a Value> {
    v.pointer(pointer)
        .ok_or_else(|| format!("missing {pointer}"))
}
fn equal(v: &Value, pointer: &str, expected: Value) -> Result<()> {
    if required(v, pointer)? != &expected {
        return Err(format!("Job {pointer} differs from approval"));
    }
    Ok(())
}
fn exact_keys(v: &Value, pointer: &str, keys: &[&str]) -> Result<()> {
    let obj = required(v, pointer)?
        .as_object()
        .ok_or(format!("{pointer} must be object"))?;
    if obj.keys().any(|key| !keys.contains(&key.as_str())) {
        return Err(format!("{pointer} has unapproved fields"));
    }
    Ok(())
}
fn job_guard(
    job: &Value,
    approval: &AccessApproval,
    reservation: &LaunchReservation,
) -> Result<()> {
    exact_keys(job, "", &["apiVersion", "kind", "metadata", "spec"])?;
    equal(job, "/apiVersion", json!("batch/v1"))?;
    equal(job, "/kind", json!("Job"))?;
    exact_keys(
        job,
        "/metadata",
        &["name", "namespace", "labels", "annotations"],
    )?;
    equal(job, "/metadata/name", json!(approval.job_name))?;
    equal(job, "/metadata/namespace", json!(approval.namespace))?;
    exact_keys(
        job,
        "/spec",
        &[
            "suspend",
            "completions",
            "parallelism",
            "backoffLimit",
            "activeDeadlineSeconds",
            "template",
        ],
    )?;
    equal(job, "/spec/suspend", json!(true))?;
    equal(job, "/spec/completions", json!(1))?;
    equal(job, "/spec/parallelism", json!(1))?;
    equal(job, "/spec/backoffLimit", json!(0))?;
    let deadline = required(job, "/spec/activeDeadlineSeconds")?
        .as_u64()
        .ok_or("invalid Job deadline")?;
    if deadline == 0 || deadline > approval.max_active_deadline_seconds {
        return Err("Job deadline exceeds approval".into());
    }
    exact_keys(job, "/spec/template", &["metadata", "spec"])?;
    exact_keys(
        job,
        "/spec/template/spec",
        &[
            "restartPolicy",
            "serviceAccountName",
            "automountServiceAccountToken",
            "terminationGracePeriodSeconds",
            "nodeSelector",
            "tolerations",
            "affinity",
            "securityContext",
            "containers",
            "volumes",
        ],
    )?;
    equal(job, "/spec/template/spec/restartPolicy", json!("Never"))?;
    equal(
        job,
        "/spec/template/spec/serviceAccountName",
        json!(approval.service_account),
    )?;
    equal(
        job,
        "/spec/template/spec/automountServiceAccountToken",
        json!(false),
    )?;
    let containers = required(job, "/spec/template/spec/containers")?
        .as_array()
        .ok_or("containers must be array")?;
    if containers.len() != 1 {
        return Err("expected one container".into());
    }
    exact_keys(
        job,
        "/spec/template/spec/containers/0",
        &[
            "name",
            "image",
            "imagePullPolicy",
            "command",
            "workingDir",
            "securityContext",
            "resources",
            "env",
            "volumeMounts",
        ],
    )?;
    equal(job, "/spec/template/spec/containers/0/name", json!("task"))?;
    equal(
        job,
        "/spec/template/spec/containers/0/image",
        json!(approval.image),
    )?;
    equal(
        job,
        "/spec/template/spec/containers/0/command",
        json!(["/usr/local/bin/crypto", "dataset", "start"]),
    )?;
    equal(
        job,
        "/spec/template/spec/containers/0/securityContext/allowPrivilegeEscalation",
        json!(false),
    )?;
    equal(
        job,
        "/spec/template/spec/containers/0/securityContext/readOnlyRootFilesystem",
        json!(true),
    )?;
    equal(
        job,
        "/spec/template/spec/containers/0/securityContext/capabilities/drop",
        json!(["ALL"]),
    )?;
    let resources = required(job, "/spec/template/spec/containers/0/resources")?;
    exact_keys(resources, "", &["requests", "limits"])?;
    for side in ["requests", "limits"] {
        exact_keys(
            resources,
            &format!("/{side}"),
            &["cpu", "memory", "ephemeral-storage"],
        )?;
        for field in ["cpu", "memory", "ephemeral-storage"] {
            let s = required(resources, &format!("/{side}/{field}"))?
                .as_str()
                .ok_or("resource must be string")?;
            if s.is_empty() || s.contains("__") {
                return Err("unresolved resource placeholder".into());
            }
        }
    }
    let env = required(job, "/spec/template/spec/containers/0/env")?
        .as_array()
        .ok_or("env must be array")?;
    let permitted = [
        "HOME",
        "AWS_REGION",
        "AWS_DEFAULT_REGION",
        "RAYON_NUM_THREADS",
        "OMP_NUM_THREADS",
        "POD_UID",
        "JOB_CONTROLLER_UID",
        "POD_NAMESPACE",
        "POD_NAME",
        "NODE_NAME",
    ];
    if env.len() != permitted.len() {
        return Err("unexpected environment set".into());
    }
    for (item, name) in env.iter().zip(permitted) {
        if item.get("name") != Some(&json!(name)) {
            return Err("environment order/name differs".into());
        }
        exact_keys(item, "", &["name", "value", "valueFrom"])?;
    }
    let image_digest = approval
        .image
        .split_once("@sha256:")
        .map(|(_, digest)| digest)
        .ok_or("image must be digest-pinned")?;
    if approval.image.contains("__") || check_sha(image_digest).is_err() {
        return Err("image must be digest-pinned".into());
    }
    let volumes = required(job, "/spec/template/spec/volumes")?
        .as_array()
        .ok_or("volumes must be array")?;
    let mounts = required(job, "/spec/template/spec/containers/0/volumeMounts")?
        .as_array()
        .ok_or("mounts must be array")?;
    if volumes.len() != 4 || mounts.len() != 4 {
        return Err("expected four reviewed volumes and mounts".into());
    }
    let names = ["work", "tmp", "startup-config", "late-grant"];
    for (idx, name) in names.iter().enumerate() {
        if volumes[idx]["name"] != *name || mounts[idx]["name"] != *name {
            return Err("volume/mount set differs".into());
        }
    }
    equal(
        job,
        "/spec/template/spec/containers/0/volumeMounts/2/mountPath",
        json!("/opt/crypto/dataset/startup-gate.json"),
    )?;
    equal(
        job,
        "/spec/template/spec/containers/0/volumeMounts/3/mountPath",
        json!("/run/dataset-admission"),
    )?;
    for (idx, name) in ["work", "tmp"].iter().enumerate() {
        exact_keys(&volumes[idx], "", &["name", "emptyDir"])?;
        if volumes[idx]["emptyDir"]["sizeLimit"]
            .as_str()
            .unwrap_or("")
            .is_empty()
        {
            return Err(format!("{name} has no size limit"));
        }
    }
    for idx in [2, 3] {
        exact_keys(&volumes[idx], "", &["name", "configMap"])?;
    }
    equal(
        job,
        "/spec/template/spec/volumes/2/configMap/name",
        json!(approval.configmap_name),
    )?;
    equal(
        job,
        "/spec/template/spec/volumes/3/configMap/name",
        json!(approval.mailbox_name),
    )?;
    if reservation.request.target.namespace != approval.namespace
        || reservation.request.target.job_name != approval.job_name
        || reservation.request.target.cluster_identity != approval.cluster_arn
    {
        return Err("reservation target differs".into());
    }
    Ok(())
}

fn admitted_guard(admitted: &Value, reviewed: &Value, approval: &AccessApproval) -> Result<()> {
    exact_keys(
        &admitted["spec"],
        "",
        &[
            "suspend",
            "completions",
            "parallelism",
            "backoffLimit",
            "activeDeadlineSeconds",
            "template",
            "selector",
            "manualSelector",
            "completionMode",
        ],
    )?;
    if admitted["spec"].get("manualSelector").is_some()
        && admitted["spec"]["manualSelector"] != false
        || admitted["spec"].get("completionMode").is_some()
            && admitted["spec"]["completionMode"] != "NonIndexed"
    {
        return Err("admitted Job selector or completion mode differs".into());
    }
    if admitted["metadata"]["labels"] != reviewed["metadata"]["labels"]
        || admitted["metadata"]["annotations"] != reviewed["metadata"]["annotations"]
    {
        return Err("admitted Job labels or annotations differ".into());
    }
    let job_uid = admitted["metadata"]["uid"]
        .as_str()
        .filter(|uid| !uid.is_empty());
    if job_uid.is_some() && admitted["spec"].get("selector").is_none() {
        return Err("live Job lacks controller-UID selector".into());
    }
    if let Some(selector) = admitted["spec"].get("selector") {
        let uid = job_uid.ok_or("selector present without observed Job UID")?;
        exact_keys(selector, "", &["matchLabels"])?;
        let labels = selector["matchLabels"]
            .as_object()
            .ok_or("admitted Job selector lacks labels")?;
        if labels.is_empty() {
            return Err("empty admitted Job selector".into());
        }
        if !["batch.kubernetes.io/controller-uid", "controller-uid"]
            .iter()
            .any(|key| labels.get(*key).and_then(Value::as_str) == Some(uid))
        {
            return Err("admitted Job selector lacks matching controller UID".into());
        }
        for (key, value) in labels {
            match key.as_str() {
                "batch.kubernetes.io/job-name" | "job-name"
                    if value == &json!(approval.job_name) => {}
                "batch.kubernetes.io/controller-uid" | "controller-uid"
                    if admitted["metadata"]["uid"]
                        .as_str()
                        .map(|uid| value == uid)
                        .unwrap_or(false) => {}
                _ => return Err("admitted Job selector differs from owner".into()),
            }
        }
    }
    let template_meta = &admitted["spec"]["template"]["metadata"];
    exact_keys(
        template_meta,
        "",
        &["labels", "annotations", "creationTimestamp"],
    )?;
    if template_meta["annotations"] != reviewed["spec"]["template"]["metadata"]["annotations"] {
        return Err("admitted Pod template annotations differ".into());
    }
    let approved_labels = reviewed["spec"]["template"]["metadata"]["labels"].as_object();
    let observed_labels = template_meta["labels"]
        .as_object()
        .ok_or("admitted Pod labels missing")?;
    if let Some(uid) = job_uid {
        if !["batch.kubernetes.io/controller-uid", "controller-uid"]
            .iter()
            .any(|key| observed_labels.get(*key).and_then(Value::as_str) == Some(uid))
        {
            return Err("admitted Pod template lacks matching controller UID".into());
        }
    }
    for (key, expected) in approved_labels.into_iter().flatten() {
        if observed_labels.get(key) != Some(expected) {
            return Err("admitted Pod template label differs".into());
        }
    }
    for (key, actual) in observed_labels {
        if approved_labels
            .map(|m| m.contains_key(key))
            .unwrap_or(false)
        {
            continue;
        }
        match key.as_str() {
            "batch.kubernetes.io/job-name" | "job-name" if actual == &json!(approval.job_name) => {}
            "batch.kubernetes.io/controller-uid" | "controller-uid"
                if admitted["metadata"]["uid"]
                    .as_str()
                    .map(|uid| actual == uid)
                    .unwrap_or(false) => {}
            _ => return Err("unapproved Pod template label".into()),
        }
    }
    if admitted["kind"] != "Job"
        || admitted["metadata"]["name"] != approval.job_name
        || admitted["metadata"]["namespace"] != approval.namespace
        || admitted["spec"]["suspend"] != true
        || admitted["spec"]["completions"] != 1
        || admitted["spec"]["parallelism"] != 1
        || admitted["spec"]["backoffLimit"] != 0
        || admitted["spec"]["activeDeadlineSeconds"] != reviewed["spec"]["activeDeadlineSeconds"]
    {
        return Err("admitted Job identity or lifetime differs".into());
    }
    let actual = &admitted["spec"]["template"]["spec"];
    let expected = &reviewed["spec"]["template"]["spec"];
    exact_keys(
        actual,
        "",
        &[
            "restartPolicy",
            "serviceAccountName",
            "automountServiceAccountToken",
            "terminationGracePeriodSeconds",
            "nodeSelector",
            "tolerations",
            "affinity",
            "securityContext",
            "containers",
            "volumes",
            "dnsPolicy",
            "schedulerName",
            "enableServiceLinks",
            "serviceAccount",
            "priority",
            "preemptionPolicy",
        ],
    )?;
    if actual.get("dnsPolicy").is_some() && actual["dnsPolicy"] != "ClusterFirst"
        || actual.get("schedulerName").is_some() && actual["schedulerName"] != "default-scheduler"
        || actual.get("serviceAccount").is_some()
            && actual["serviceAccount"] != approval.service_account
        || actual.get("priority").is_some() && actual["priority"] != 0
        || actual.get("preemptionPolicy").is_some()
            && actual["preemptionPolicy"] != "PreemptLowerPriority"
    {
        return Err("admitted Pod default changed execution routing".into());
    }
    for field in [
        "serviceAccountName",
        "automountServiceAccountToken",
        "restartPolicy",
        "nodeSelector",
        "tolerations",
        "affinity",
        "securityContext",
        "volumes",
        "terminationGracePeriodSeconds",
    ] {
        if actual[field] != expected[field] {
            return Err(format!("admitted Pod {field} differs"));
        }
    }
    for forbidden in [
        "initContainers",
        "ephemeralContainers",
        "hostNetwork",
        "hostPID",
        "hostIPC",
        "hostUsers",
        "shareProcessNamespace",
    ] {
        if actual.get(forbidden).is_some()
            && actual[forbidden] != false
            && actual[forbidden] != json!([])
        {
            return Err(format!("unapproved admitted Pod field {forbidden}"));
        }
    }
    let containers = actual["containers"]
        .as_array()
        .ok_or("admitted containers missing")?;
    if containers.len() != 1 {
        return Err("admitted sidecar or missing task container".into());
    }
    let observed = &containers[0];
    let requested = &expected["containers"][0];
    exact_keys(
        observed,
        "",
        &[
            "name",
            "image",
            "imagePullPolicy",
            "command",
            "workingDir",
            "securityContext",
            "resources",
            "env",
            "volumeMounts",
            "terminationMessagePath",
            "terminationMessagePolicy",
        ],
    )?;
    if observed.get("terminationMessagePath").is_some()
        && observed["terminationMessagePath"] != "/dev/termination-log"
        || observed.get("terminationMessagePolicy").is_some()
            && observed["terminationMessagePolicy"] != "File"
    {
        return Err("admitted termination message settings differ".into());
    }
    for field in [
        "name",
        "image",
        "command",
        "resources",
        "env",
        "volumeMounts",
        "securityContext",
        "workingDir",
        "imagePullPolicy",
    ] {
        if observed[field] != requested[field] {
            return Err(format!("admitted container {field} differs"));
        }
    }
    if observed.get("args").is_some() || observed.get("envFrom").is_some() {
        return Err("admitted container has extra args or environment source".into());
    }
    Ok(())
}

fn command(bin: &Path, args: &[&str]) -> Result<Value> {
    let out = Command::new(bin)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!("command failed (status {})", out.status));
    }
    serde_json::from_slice(&out.stdout).map_err(|e| format!("invalid command JSON: {e}"))
}
fn command_stdin(bin: &Path, args: &[&str], bytes: &[u8]) -> Result<Value> {
    let mut child = Command::new(bin)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    child
        .stdin
        .take()
        .ok_or("missing child stdin")?
        .write_all(bytes)
        .map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!("command failed (status {})", out.status));
    }
    serde_json::from_slice(&out.stdout).map_err(|e| format!("invalid command JSON: {e}"))
}
fn kubectl(args: &Args, approval: &AccessApproval, extra: &[&str]) -> Result<Value> {
    let mut argv = vec![
        "--request-timeout=20s",
        "--kubeconfig",
        args.kubeconfig.to_str().ok_or("kubeconfig path encoding")?,
        "--context",
        approval.kubectl_context.as_str(),
    ];
    argv.extend_from_slice(extra);
    command(&args.kubectl, &argv)
}
fn kubectl_stdin(
    args: &Args,
    approval: &AccessApproval,
    extra: &[&str],
    bytes: &[u8],
) -> Result<Value> {
    let mut argv = vec![
        "--request-timeout=20s",
        "--kubeconfig",
        args.kubeconfig.to_str().ok_or("kubeconfig path encoding")?,
        "--context",
        approval.kubectl_context.as_str(),
    ];
    argv.extend_from_slice(extra);
    command_stdin(&args.kubectl, &argv, bytes)
}
fn named_jobs(args: &Args, approval: &AccessApproval) -> Result<Vec<Value>> {
    let selector = format!("metadata.name={}", approval.job_name);
    let response = kubectl(
        args,
        approval,
        &[
            "-n",
            &approval.namespace,
            "get",
            "jobs",
            "--field-selector",
            &selector,
            "-o",
            "json",
        ],
    )?;
    if response["kind"] != "JobList" {
        return Err("unexpected Kubernetes Job query response".into());
    }
    let jobs = response["items"].as_array().ok_or("JobList lacks items")?;
    if jobs.len() > 1 {
        return Err("multiple Jobs matched exact name".into());
    }
    Ok(jobs.clone())
}
fn sync_dir(path: &Path) -> Result<()> {
    File::open(path)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())
}
fn attempt_marker(
    ledger_path: &Path,
    launch_id: &str,
    intent_sha: &str,
    expected_reservation: &Value,
    expected_policy: &Value,
    create: bool,
) -> Result<()> {
    let mut connection = Connection::open_with_flags(
        ledger_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|e| e.to_string())?;
    connection
        .busy_timeout(std::time::Duration::from_secs(10))
        .map_err(|e| e.to_string())?;
    connection.execute_batch("PRAGMA journal_mode=DELETE; PRAGMA synchronous=EXTRA; CREATE TABLE IF NOT EXISTS ml_dataset_submit_attempts (launch_id TEXT PRIMARY KEY, intent_sha256 TEXT NOT NULL);").map_err(|e| e.to_string())?;
    let tx = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|e| e.to_string())?;
    let (kind, state_text, state_sha): (String, String, String) = tx
        .query_row("SELECT kind,json,sha FROM state WHERE id=1", [], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(|e| e.to_string())?;
    if kind != "campaign" || sha(state_text.as_bytes()) != state_sha {
        return Err("allocator state kind or digest changed".into());
    }
    let state: Value = serde_json::from_str(&state_text).map_err(|e| e.to_string())?;
    if state["policy"] != *expected_policy
        || state["launches"][launch_id]["reservation"] != *expected_reservation
        || expected_reservation["request"]["launch_id"] != launch_id
    {
        return Err("allocator policy or launch reservation changed before create".into());
    }
    if create && !state["launches"][launch_id]["binding"].is_null() {
        return Err("launch became bound before create".into());
    }
    let prior: Option<String> = tx
        .query_row(
            "SELECT intent_sha256 FROM ml_dataset_submit_attempts WHERE launch_id=?1",
            params![launch_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    match (create, prior) {
        (true, None) => {
            tx.execute(
                "INSERT INTO ml_dataset_submit_attempts (launch_id, intent_sha256) VALUES (?1, ?2)",
                params![launch_id, intent_sha],
            )
            .map_err(|e| e.to_string())?;
            let previous: Option<(u64, String)> = tx
                .query_row(
                    "SELECT seq,sha FROM events ORDER BY seq DESC LIMIT 1",
                    [],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .optional()
                .map_err(|e| e.to_string())?;
            let (sequence, previous_sha) = match previous {
                Some((n, digest)) => (n.checked_add(1).ok_or("event sequence overflow")?, digest),
                None => (1, "0".repeat(64)),
            };
            let body = json!({"schema":"crypto.ml-dataset.budget-event/v1","sequence":sequence,"previous":previous_sha,"kind":"submit_attempt_committed","state_sha256":state_sha,"details":{"launch_id":launch_id,"intent_sha256":intent_sha}});
            let body_text = serde_json::to_string(&body).map_err(|e| e.to_string())?;
            tx.execute(
                "INSERT INTO events(seq,previous,sha,body) VALUES(?1,?2,?3,?4)",
                params![sequence, previous_sha, sha(body_text.as_bytes()), body_text],
            )
            .map_err(|e| e.to_string())?;
        }
        (false, Some(previous)) if previous == intent_sha => {}
        (true, Some(_)) => {
            return Err("allocator already records a create attempt for this launch ID".into())
        }
        _ => return Err("intent and allocator one-shot marker disagree".into()),
    }
    let mut statement = tx
        .prepare("SELECT body,sha FROM events ORDER BY seq")
        .map_err(|e| e.to_string())?;
    let mut rows = statement.query([]).map_err(|e| e.to_string())?;
    let mut matches = 0u32;
    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let body_text: String = row.get(0).map_err(|e| e.to_string())?;
        let event_sha: String = row.get(1).map_err(|e| e.to_string())?;
        if sha(body_text.as_bytes()) != event_sha {
            return Err("allocator event hash changed".into());
        }
        let body: Value = serde_json::from_str(&body_text).map_err(|e| e.to_string())?;
        if body["kind"] == "submit_attempt_committed" && body["details"]["launch_id"] == launch_id {
            if body["details"]["intent_sha256"] != intent_sha {
                return Err("allocator marker event differs from intent".into());
            }
            matches = matches
                .checked_add(1)
                .ok_or("marker event count overflow")?;
        }
    }
    drop(rows);
    drop(statement);
    if matches != 1 {
        return Err("allocator marker lacks exactly one hash-chained event".into());
    }
    tx.commit().map_err(|e| e.to_string())
}
fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
    }
    let mut f = options.open(path).map_err(|e| e.to_string())?;
    f.write_all(bytes)
        .and_then(|_| f.sync_all())
        .map_err(|e| e.to_string())?;
    sync_dir(path.parent().ok_or("missing intent parent")?)
}
fn run(args: Args) -> Result<()> {
    protected_root_file(&args.kubeconfig)?;
    protected_root_file(&args.kubectl)?;
    check_sha(&args.trusted_approval_sha256)?;
    if trusted_approval_digest(&args.approval_trust_anchor)? != args.trusted_approval_sha256 {
        return Err("operator trust anchor differs from approval pin".into());
    }
    let approval_bytes = read_bounded(&args.access_approval, 65536)?;
    if sha(&approval_bytes) != args.trusted_approval_sha256 {
        return Err("worker-access approval differs from independent trusted digest".into());
    }
    let approval: AccessApproval =
        serde_json::from_slice(&approval_bytes).map_err(|e| e.to_string())?;
    if approval.schema != "crypto.ml-dataset.worker-access-approval/v1"
        || !approval.allow_suspended_submit
    {
        return Err("no authorized suspended Job approval".into());
    }
    for digest in [
        &approval.cluster_ca_sha256,
        &approval.policy_sha256,
        &approval.reservation_sha256,
        &approval.job_sha256,
        &approval.config_data_sha256,
        &approval.issuer_evidence_sha256,
    ] {
        check_sha(digest)?;
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();
    if now >= approval.expires_unix_seconds {
        return Err("worker-access approval expired".into());
    }
    let (policy_value, _) = value(&args.policy, 1024 * 1024)?;
    let policy: CampaignPolicy = serde_json::from_value(policy_value).map_err(|e| e.to_string())?;
    policy.validate()?;
    if policy.sha256()? != approval.policy_sha256
        || policy.campaign_id != approval.campaign_id
        || now >= policy.deadline_unix_seconds
    {
        return Err("original campaign policy missing, changed or expired".into());
    }
    let (reservation_value, _) = value(&args.reservation, 1024 * 1024)?;
    let reservation: LaunchReservation =
        serde_json::from_value(reservation_value).map_err(|e| e.to_string())?;
    if reservation.sha256()? != approval.reservation_sha256
        || reservation.policy_sha256 != approval.policy_sha256
        || reservation.campaign_id != approval.campaign_id
        || now >= reservation.request.expires_unix_seconds
    {
        return Err("launch reservation changed or expired".into());
    }
    if reservation.request.credential_role_arn.as_deref() != Some(&approval.credential_role_arn) {
        return Err("worker IRSA role differs".into());
    }
    if !policy.allowed_remotes.iter().any(|r| {
        r.bucket == approval.destination_bucket
            && approval.destination_prefix.starts_with(&r.prefix)
    }) {
        return Err("destination outside original policy".into());
    }
    let ledger = CampaignLedger::open(&args.ledger, &policy)?;
    let snapshot = ledger.snapshot()?;
    let stored = &snapshot["launches"][&reservation.request.launch_id]["reservation"];
    if stored != &serde_json::to_value(&reservation).map_err(|e| e.to_string())? {
        return Err("reservation is absent from original allocator".into());
    }
    if !snapshot["launches"][&reservation.request.launch_id]["binding"].is_null() {
        return Err("launch is already bound".into());
    }
    let preflight = command(
        &args.crypto_bin,
        &[
            "dataset",
            "preflight",
            "--plan",
            args.plan.to_str().ok_or("plan path encoding")?,
            "--protocol",
            args.protocol.to_str().ok_or("protocol path encoding")?,
        ],
    )?;
    let binding = &reservation.request.binding;
    if preflight["source_commit"] != binding.source_commit
        || preflight["binary_sha256"] != binding.binary_sha256
        || preflight["plan_sha256"] != binding.plan_sha256
        || preflight["protocol_file_sha256"] != binding.protocol_sha256
    {
        return Err(
            "native plan/protocol/source/binary preflight differs from reserved binding".into(),
        );
    }
    let (job, job_bytes) = value(&args.job, 1024 * 1024)?;
    if sha(&job_bytes) != approval.job_sha256 {
        return Err("Job bytes differ from approval".into());
    }
    job_guard(&job, &approval, &reservation)?;
    let envelope_bytes = read_bounded(&args.resource_envelope, 65536)?;
    if sha(&envelope_bytes) != policy.resource_envelope_sha256
        || policy.resource_envelope_sha256 != reservation.request.resource_envelope_sha256
    {
        return Err("priced resource envelope differs from native policy or reservation".into());
    }
    let envelope: PricedEnvelope =
        serde_json::from_slice(&envelope_bytes).map_err(|e| e.to_string())?;
    if envelope.schema != "crypto.ml-dataset.priced-job-envelope/v1"
        || envelope.job_resources != job["spec"]["template"]["spec"]["containers"][0]["resources"]
        || envelope.node_selector != job["spec"]["template"]["spec"]["nodeSelector"]
        || envelope.active_deadline_seconds != job["spec"]["activeDeadlineSeconds"]
        || envelope.billable_resource_seconds != policy.billable_resource_seconds
        || envelope.billable_resource_count != policy.billable_resource_count
        || envelope.compute_rate_micro_usd_per_hour != policy.compute_rate_micro_usd_per_hour
    {
        return Err("actual Job resources or priced lifetime differ from approved envelope".into());
    }
    let config_bytes = read_bounded(&args.startup_config, 65536)?;
    if sha(&config_bytes) != approval.config_data_sha256 {
        return Err("startup config differs from approval".into());
    }
    let startup: Value = serde_json::from_slice(&config_bytes).map_err(|e| e.to_string())?;
    let startup_deadline = startup["startup_deadline_unix_seconds"]
        .as_u64()
        .ok_or("missing startup deadline")?;
    if startup["schema"] != "crypto.ml-dataset.startup-config/v1"
        || startup["policy_sha256"] != approval.policy_sha256
        || startup["reservation_sha256"] != approval.reservation_sha256
        || startup["generate"]["remote"]
            != format!(
                "s3://{}/{}",
                approval.destination_bucket, approval.destination_prefix
            )
        || startup_deadline <= now
        || startup_deadline > approval.expires_unix_seconds
        || startup_deadline > reservation.request.expires_unix_seconds
    {
        return Err("startup config binding, destination or deadline differs".into());
    }
    // The explicit context is checked against the independent server and CA pins.
    let ctx = kubectl(
        &args,
        &approval,
        &["config", "view", "--minify", "-o", "json"],
    )?;
    if ctx["current-context"] != approval.kubectl_context
        || ctx["clusters"][0]["cluster"]["server"] != approval.cluster_server
    {
        return Err("Kubernetes context differs from approved endpoint".into());
    }
    let ca = ctx["clusters"][0]["cluster"]["certificate-authority-data"]
        .as_str()
        .ok_or("context CA data missing")?;
    if sha(ca.as_bytes()) != approval.cluster_ca_sha256
        || ctx["clusters"][0]["cluster"]["insecure-skip-tls-verify"] == true
    {
        return Err("Kubernetes CA differs from approval".into());
    }
    let sa = kubectl(
        &args,
        &approval,
        &[
            "-n",
            &approval.namespace,
            "get",
            "serviceaccount",
            &approval.service_account,
            "-o",
            "json",
        ],
    )?;
    if sa["metadata"]["annotations"]["eks.amazonaws.com/role-arn"] != approval.credential_role_arn {
        return Err("service account IRSA role differs".into());
    }
    let config = kubectl(
        &args,
        &approval,
        &[
            "-n",
            &approval.namespace,
            "get",
            "configmap",
            &approval.configmap_name,
            "-o",
            "json",
        ],
    )?;
    if config["metadata"]["uid"] != approval.configmap_uid
        || config["immutable"] != true
        || config["data"]["startup-gate.json"]
            .as_str()
            .map(|s| sha(s.as_bytes()))
            != Some(approval.config_data_sha256.clone())
    {
        return Err("immutable startup ConfigMap differs from approval".into());
    }
    let mailbox = kubectl(
        &args,
        &approval,
        &[
            "-n",
            &approval.namespace,
            "get",
            "configmap",
            &approval.mailbox_name,
            "-o",
            "json",
        ],
    )?;
    if mailbox["metadata"]["uid"] != approval.mailbox_uid
        || !mailbox["data"]
            .as_object()
            .map(|o| o.is_empty())
            .unwrap_or(false)
    {
        return Err("preprovisioned empty mailbox differs from approval".into());
    }
    let intent = Intent {
        schema: "crypto.ml-dataset.submit-intent/v1".into(),
        status: "create_attempt_committed".into(),
        campaign_id: approval.campaign_id.clone(),
        launch_id: reservation.request.launch_id.clone(),
        cluster_arn: approval.cluster_arn.clone(),
        namespace: approval.namespace.clone(),
        job_name: approval.job_name.clone(),
        policy_sha256: approval.policy_sha256.clone(),
        reservation_sha256: approval.reservation_sha256.clone(),
        approval_sha256: args.trusted_approval_sha256.clone(),
        job_sha256: approval.job_sha256.clone(),
        job_uid: None,
    };
    let intent_path = args.intent_dir.join("intent.json");
    let existing_intent = intent_path.exists();
    let intent_bytes = serde_json::to_vec_pretty(&intent).map_err(|e| e.to_string())?;
    let intent_sha = sha(&intent_bytes);
    let expected_uid = if existing_intent {
        let old: Intent = serde_json::from_slice(&read_bounded(&intent_path, 65536)?)
            .map_err(|e| e.to_string())?;
        if serde_json::to_value(&old).map_err(|e| e.to_string())?
            != serde_json::to_value(&intent).map_err(|e| e.to_string())?
        {
            return Err("saved one-shot intent differs; no new Job will be created".into());
        }
        attempt_marker(
            &args.ledger,
            &reservation.request.launch_id,
            &intent_sha,
            &serde_json::to_value(&reservation).map_err(|e| e.to_string())?,
            &serde_json::to_value(&policy).map_err(|e| e.to_string())?,
            false,
        )?;
        args.reconcile_uid
            .clone()
            .ok_or("reconciliation requires independently observed --reconcile-uid")?
    } else {
        if args.reconcile_uid.is_some() {
            return Err("reconcile UID requires an existing intent".into());
        }
        if !named_jobs(&args, &approval)?.is_empty() {
            return Err("Job name already exists before intent; reconcile manually".into());
        }
        let dry = kubectl_stdin(
            &args,
            &approval,
            &[
                "-n",
                &approval.namespace,
                "create",
                "--dry-run=server",
                "-f",
                "-",
                "-o",
                "json",
            ],
            &job_bytes,
        )?;
        admitted_guard(&dry, &job, &approval)?;
        attempt_marker(
            &args.ledger,
            &reservation.request.launch_id,
            &intent_sha,
            &serde_json::to_value(&reservation).map_err(|e| e.to_string())?,
            &serde_json::to_value(&policy).map_err(|e| e.to_string())?,
            true,
        )?;
        let parent = args.intent_dir.parent().ok_or("intent parent missing")?;
        let parent_meta = fs::symlink_metadata(parent).map_err(|e| e.to_string())?;
        if !parent_meta.file_type().is_dir() {
            return Err("intent parent must be a real directory".into());
        }
        fs::create_dir(&args.intent_dir)
            .map_err(|e| format!("intent directory must be new: {e}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&args.intent_dir, fs::Permissions::from_mode(0o700))
                .map_err(|e| e.to_string())?;
        }
        sync_dir(args.intent_dir.parent().ok_or("intent parent missing")?)?;
        write_new(&intent_path, &intent_bytes)?;
        // Any ambiguous outcome is permanently charged. This command never retries creation.
        let created = kubectl_stdin(
            &args,
            &approval,
            &["-n", &approval.namespace, "create", "-f", "-", "-o", "json"],
            &job_bytes,
        )
        .map_err(|_| "create outcome unknown; intent retained, no second create permitted")?;
        created["metadata"]["uid"]
            .as_str()
            .ok_or("create response lacks Job UID")?
            .to_owned()
    };
    let observed = named_jobs(&args, &approval)?.into_iter().next().ok_or(
        "create outcome unknown; retain intent and reconcile later without another create",
    )?;
    let uid = observed["metadata"]["uid"]
        .as_str()
        .ok_or("created Job has no UID")?;
    if uid != expected_uid {
        return Err("live Job UID differs from create response or reconciliation UID".into());
    }
    admitted_guard(&observed, &job, &approval)?;
    if observed["spec"]["suspend"] != true
        || observed["metadata"]["name"] != approval.job_name
        || observed["metadata"]["namespace"] != approval.namespace
        || observed["spec"]["template"]["spec"]["serviceAccountName"] != approval.service_account
        || observed["spec"]["template"]["spec"]["containers"][0]["image"] != approval.image
        || observed["spec"]["template"]["spec"]["containers"][0]["command"]
            != json!(["/usr/local/bin/crypto", "dataset", "start"])
        || observed["spec"]["template"]["spec"]["volumes"][2]["configMap"]["name"]
            != approval.configmap_name
        || observed["spec"]["template"]["spec"]["volumes"][3]["configMap"]["name"]
            != approval.mailbox_name
    {
        return Err("observed Job differs; do not unsuspend".into());
    }
    let receipt = json!({"schema":"crypto.ml-dataset.submit-observation/v1","intent_sha256":sha(&fs::read(args.intent_dir.join("intent.json")).map_err(|e|e.to_string())?),"job_uid":uid,"job":observed,"worker_grant_issued":false,"job_unsuspended":false});
    let observation_path = args.intent_dir.join("observation.json");
    let receipt_bytes = serde_json::to_vec_pretty(&receipt).map_err(|e| e.to_string())?;
    if observation_path.exists() {
        let previous: Value =
            serde_json::from_slice(&read_bounded(&observation_path, 1024 * 1024)?)
                .map_err(|e| e.to_string())?;
        if previous["job_uid"] != uid
            || previous["intent_sha256"] != receipt["intent_sha256"]
            || previous["job"]["metadata"]["name"] != observed["metadata"]["name"]
        {
            return Err(
                "saved observation differs from live Job identity; do not unsuspend".into(),
            );
        }
    } else {
        write_new(&observation_path, &receipt_bytes)?;
    }
    println!(
        "{}",
        json!({"status":"suspended_job_observed","job_uid":uid,"intent_dir":args.intent_dir})
    );
    Ok(())
}
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("dataset campaign submit: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::ml_dataset::budget::{LaunchRequest, LaunchTarget, Resources, RunBinding};

    fn fixture() -> (AccessApproval, LaunchReservation) {
        let approval: AccessApproval = serde_json::from_value(json!({
            "schema":"crypto.ml-dataset.worker-access-approval/v1","campaign_id":"test",
            "cluster_arn":"arn:aws:eks:us-east-2:1:cluster/test","kubectl_context":"test",
            "cluster_server":"https://example.invalid","cluster_ca_sha256":"a".repeat(64),
            "namespace":"test","job_name":"test","service_account":"worker",
            "credential_role_arn":"arn:aws:iam::1:role/test","configmap_name":"config",
            "configmap_uid":"uid-config","config_data_sha256":"a".repeat(64),
            "mailbox_name":"mailbox","mailbox_uid":"uid-mailbox","image":format!("example@sha256:{}", "a".repeat(64)),
            "destination_bucket":"bucket","destination_prefix":"prefix/","policy_sha256":"a".repeat(64),
            "reservation_sha256":"b".repeat(64),"job_sha256":"c".repeat(64),
            "expires_unix_seconds":9999999999u64,"max_active_deadline_seconds":3600,
            "issuer_evidence_sha256":"d".repeat(64),"allow_suspended_submit":true
        })).unwrap();
        let reservation = LaunchReservation {
            schema: "crypto.ml-dataset.budget-launch-reservation/v1".into(),
            campaign_id: "test".into(),
            policy_sha256: "a".repeat(64),
            request: LaunchRequest {
                schema: "crypto.ml-dataset.budget-launch-request/v1".into(),
                launch_id: "test".into(),
                target: LaunchTarget {
                    cluster_identity: approval.cluster_arn.clone(),
                    namespace: "test".into(),
                    job_name: "test".into(),
                },
                worker_journal_path: "/work/budget/worker.sqlite".into(),
                binding: RunBinding {
                    source_commit: "a".repeat(40),
                    binary_sha256: "a".repeat(64),
                    plan_sha256: "a".repeat(64),
                    protocol_sha256: "a".repeat(64),
                    shard_index: 0,
                    shard_count: 1,
                },
                expires_unix_seconds: 9999999999,
                resource_envelope_sha256: "a".repeat(64),
                resources: Resources::default(),
                finalization: Resources::default(),
                credential_role_arn: Some(approval.credential_role_arn.clone()),
            },
        };
        (approval, reservation)
    }
    #[test]
    fn reject_bad_digest() {
        assert!(check_sha("abc").is_err());
    }
    #[test]
    fn no_empty_job_fields() {
        let a = serde_json::from_value::<AccessApproval>(
            json!({"schema":"crypto.ml-dataset.worker-access-approval/v1"}),
        );
        assert!(a.is_err());
    }
    #[test]
    fn rejects_unsuspended_or_extended_job() {
        let (approval, reservation) = fixture();
        let job = json!({"apiVersion":"batch/v1","kind":"Job","metadata":{"name":"test","namespace":"test"},"spec":{"suspend":false}});
        assert!(job_guard(&job, &approval, &reservation)
            .unwrap_err()
            .contains("suspend"));
        let extended = json!({"apiVersion":"batch/v1","kind":"Job","metadata":{"name":"test","namespace":"test"},"spec":{"suspend":true,"hostNetwork":true}});
        assert!(job_guard(&extended, &approval, &reservation)
            .unwrap_err()
            .contains("unapproved fields"));
    }
    #[test]
    fn reviewed_job_shape_rejects_host_path() {
        let (approval, reservation) = fixture();
        let env_names = [
            "HOME",
            "AWS_REGION",
            "AWS_DEFAULT_REGION",
            "RAYON_NUM_THREADS",
            "OMP_NUM_THREADS",
            "POD_UID",
            "JOB_CONTROLLER_UID",
            "POD_NAMESPACE",
            "POD_NAME",
            "NODE_NAME",
        ];
        let env: Vec<Value> = env_names
            .into_iter()
            .map(|name| json!({"name":name}))
            .collect();
        let mut job = json!({
            "apiVersion":"batch/v1", "kind":"Job",
            "metadata":{"name":"test","namespace":"test"},
            "spec":{"suspend":true,"completions":1,"parallelism":1,"backoffLimit":0,"activeDeadlineSeconds":3600,
                "template":{"metadata":{"labels":{},"annotations":{}},"spec":{"restartPolicy":"Never","serviceAccountName":"worker","automountServiceAccountToken":false,
                    "containers":[{"name":"task","image":format!("example@sha256:{}", "a".repeat(64)),"command":["/usr/local/bin/crypto","dataset","start"],
                        "securityContext":{"allowPrivilegeEscalation":false,"readOnlyRootFilesystem":true,"capabilities":{"drop":["ALL"]}},
                        "resources":{"requests":{"cpu":"1","memory":"2Gi","ephemeral-storage":"4Gi"},"limits":{"cpu":"2","memory":"4Gi","ephemeral-storage":"8Gi"}},
                        "env":env,"volumeMounts":[{"name":"work"},{"name":"tmp"},{"name":"startup-config","mountPath":"/opt/crypto/dataset/startup-gate.json"},{"name":"late-grant","mountPath":"/run/dataset-admission"}]}],
                    "volumes":[{"name":"work","emptyDir":{"sizeLimit":"4Gi"}},{"name":"tmp","emptyDir":{"sizeLimit":"4Gi"}},{"name":"startup-config","configMap":{"name":"config"}},{"name":"late-grant","configMap":{"name":"mailbox"}}]
                }}}
        });
        assert!(job_guard(&job, &approval, &reservation).is_ok());
        assert!(admitted_guard(&job, &job, &approval).is_ok());
        let mut mutated = job.clone();
        mutated["spec"]["template"]["spec"]["containers"][0]["resources"]["limits"]["memory"] =
            json!("8Gi");
        assert!(admitted_guard(&mutated, &job, &approval)
            .unwrap_err()
            .contains("resources"));
        let mut sidecar = job.clone();
        sidecar["spec"]["template"]["spec"]["containers"]
            .as_array_mut()
            .unwrap()
            .push(json!({"name":"sidecar"}));
        assert!(admitted_guard(&sidecar, &job, &approval)
            .unwrap_err()
            .contains("sidecar"));
        let mut selector_without_uid = job.clone();
        selector_without_uid["metadata"]["uid"] = json!("uid-1");
        selector_without_uid["spec"]["selector"] =
            json!({"matchLabels":{"batch.kubernetes.io/job-name":"test"}});
        selector_without_uid["spec"]["template"]["metadata"]["labels"] =
            json!({"batch.kubernetes.io/job-name":"test"});
        assert!(admitted_guard(&selector_without_uid, &job, &approval)
            .unwrap_err()
            .contains("controller UID"));
        selector_without_uid["spec"]["selector"]["matchLabels"]
            ["batch.kubernetes.io/controller-uid"] = json!("uid-1");
        selector_without_uid["spec"]["template"]["metadata"]["labels"]
            ["batch.kubernetes.io/controller-uid"] = json!("uid-1");
        assert!(admitted_guard(&selector_without_uid, &job, &approval).is_ok());
        job["spec"]["template"]["spec"]["volumes"][0]["hostPath"] = json!({"path":"/"});
        assert!(job_guard(&job, &approval, &reservation)
            .unwrap_err()
            .contains("unapproved fields"));
    }
    #[test]
    fn allocator_marker_charges_first_attempt_and_blocks_duplicate() {
        let path = std::env::temp_dir().join(format!(
            "dataset-submit-marker-{}-{}.sqlite",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let connection = Connection::open(&path).unwrap();
        connection.execute_batch("CREATE TABLE state(id INTEGER PRIMARY KEY,kind TEXT,json TEXT,sha TEXT); CREATE TABLE events(seq INTEGER PRIMARY KEY,previous TEXT,sha TEXT,body TEXT);").unwrap();
        let (_, reservation) = fixture();
        let reservation = serde_json::to_value(reservation).unwrap();
        let policy = json!({"test":true});
        let state =
            json!({"policy":policy,"launches":{"test":{"reservation":reservation,"binding":null}}});
        let state_text = serde_json::to_string(&state).unwrap();
        connection
            .execute(
                "INSERT INTO state(id,kind,json,sha) VALUES(1,'campaign',?1,?2)",
                params![state_text, sha(state_text.as_bytes())],
            )
            .unwrap();
        assert!(
            attempt_marker(&path, "test", &"a".repeat(64), &reservation, &policy, true).is_ok()
        );
        assert!(
            attempt_marker(&path, "test", &"a".repeat(64), &reservation, &policy, true).is_err()
        );
        assert!(
            attempt_marker(&path, "test", &"a".repeat(64), &reservation, &policy, false).is_ok()
        );
        assert!(
            attempt_marker(&path, "test", &"b".repeat(64), &reservation, &policy, false).is_err()
        );
        std::fs::remove_file(path).unwrap();
    }
}
