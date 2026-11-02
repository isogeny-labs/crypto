//! External reconciler. All durable transitions use Kubernetes resourceVersion;
//! all effects have deterministic names and are checked before being repeated.
use crate::{
    contract::*,
    io::*,
    model::*,
    supervisor::{Operation, Status},
};
use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::Duration,
};

const SUPERVISOR: &str = "/opt/isolab/recovery/isolab-eks-recovery";
fn aws() -> Command {
    let mut c = Command::new("timeout");
    c.args([
        "--kill-after=5s",
        "180s",
        "aws",
        "--region",
        REGION,
        "--no-cli-pager",
    ])
    .env("AWS_MAX_ATTEMPTS", "3");
    c
}
fn kube(namespace: &str) -> Command {
    let mut c = Command::new("timeout");
    c.args([
        "--kill-after=5s",
        "180s",
        "kubectl",
        "--context",
        "isolab",
        "--namespace",
        namespace,
        "--request-timeout=30s",
    ]);
    c
}
fn stdin_json(c: &mut Command, v: &Value) -> Result<Value, String> {
    let mut child = c
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(error)?;
    child
        .stdin
        .take()
        .ok_or("stdin missing")?
        .write_all(&serde_json::to_vec(v).map_err(error)?)
        .map_err(error)?;
    let out = child.wait_with_output().map_err(error)?;
    if !out.status.success() {
        return Err(format!(
            "Kubernetes write failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    serde_json::from_slice(&out.stdout).map_err(error)
}
fn get(ns: &str, kind: &str, name: &str) -> Result<Option<Value>, String> {
    let o = output(kube(ns).args(["get", kind, name, "--ignore-not-found", "-o", "json"]))?;
    if o.stdout.is_empty() {
        Ok(None)
    } else {
        Ok(Some(serde_json::from_slice(&o.stdout).map_err(error)?))
    }
}
fn create(ns: &str, v: &Value) -> Result<Value, String> {
    stdin_json(kube(ns).args(["create", "-f", "-", "-o", "json"]), v)
}
fn field<'a>(v: &'a Value, pointer: &str) -> Result<&'a str, String> {
    v.pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing {pointer}"))
}
fn identity(v: &Value, req: &Request) -> Result<(), String> {
    if v["metadata"]["annotations"]["isolab.dev/recovery-request"] != req.hash() {
        return Err("existing resource belongs to another request".into());
    }
    Ok(())
}
fn ensure(ns: &str, v: &Value, req: &Request) -> Result<Value, String> {
    let kind = field(v, "/kind")?;
    let name = field(v, "/metadata/name")?;
    if let Some(existing) = get(ns, kind, name)? {
        identity(&existing, req)?;
        Ok(existing)
    } else {
        create(ns, v)
    }
}
fn metadata(name: &str, req: &Request) -> Value {
    json!({"name":name,"namespace":NAMESPACE,"labels":{"isolab.dev/recovery-run":req.run_name},"annotations":{"isolab.dev/recovery-request":req.hash(),"karpenter.sh/do-not-disrupt":"true"}})
}
fn basename(s: &State) -> String {
    format!("{}-a{}", s.request.run_name, s.attempt)
}
fn prefix(req: &Request) -> String {
    format!("recovery/runs/{}/", req.run_name)
}
fn build_status(id: &str) -> Result<Option<String>, String> {
    let v = json(aws().args(["codebuild", "batch-get-builds", "--ids", id]))?;
    Ok(v["builds"]
        .as_array()
        .and_then(|a| a.iter().find(|b| b["id"] == id))
        .and_then(|v| v["buildStatus"].as_str())
        .map(str::to_owned))
}
fn instance_state(id: &str) -> Result<Option<String>, String> {
    let v = json(aws().args(["ec2", "describe-instances", "--instance-ids", id]))?;
    Ok(v["Reservations"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|r| r["Instances"].as_array().into_iter().flatten())
        .find(|i| i["InstanceId"] == id)
        .and_then(|i| i["State"]["Name"].as_str())
        .map(str::to_owned))
}
struct Stored {
    object: Value,
    state: State,
}
impl Stored {
    fn save(&mut self) -> Result<(), String> {
        self.object["data"]["state.json"] =
            json!(serde_json::to_string(&self.state).map_err(error)?);
        self.object = stdin_json(
            kube(NAMESPACE).args(["replace", "-f", "-", "-o", "json"]),
            &self.object,
        )?;
        Ok(())
    }
}
fn load(req: &Request) -> Result<Option<Stored>, String> {
    let Some(object) = get(NAMESPACE, "configmap", &format!("{}-state", req.run_name))? else {
        return Ok(None);
    };
    identity(&object, req)?;
    let state: State = serde_json::from_str(field(&object, "/data/state.json")?).map_err(error)?;
    state.bind_request(req)?;
    Ok(Some(Stored { object, state }))
}
fn begin(req: Request, owner: &str) -> Result<Stored, String> {
    if let Some(mut stored) = load(&req)? {
        if stored.state.owner_build != owner {
            let status = build_status(&stored.state.owner_build)?;
            stored.state.claim(owner, status.as_deref())?;
            stored.save()?;
        }
        return Ok(stored);
    }
    let state = State::new(req.clone(), owner.into(), now())?;
    // A permanent S3 tombstone prevents restarting a deleted run. A crash between
    // this write and ConfigMap creation requires operator reconciliation, never an
    // automatic fresh computation. CodeBuild retries use the existing ConfigMap.
    let registry = PathBuf::from("recovery-registry.json");
    save(&registry, &req)?;
    output(
        aws()
            .args([
                "s3api",
                "put-object",
                "--bucket",
                BUCKET,
                "--key",
                &format!("{}REQUEST.json", prefix(&req)),
                "--body",
            ])
            .arg(&registry)
            .args(["--if-none-match", "*", "--server-side-encryption", "AES256"]),
    )?;
    let object = create(
        NAMESPACE,
        &json!({"apiVersion":"v1","kind":"ConfigMap","metadata":metadata(&format!("{}-state",req.run_name),&req),"data":{"state.json":serde_json::to_string(&state).map_err(error)?}}),
    )?;
    Ok(Stored { object, state })
}
fn rpc(s: &State, action: &str, generation: u64, hash: Option<String>) -> Result<(), String> {
    let op = Operation {
        request_sha256: s.request_sha256.clone(),
        attempt: s.attempt,
        action: action.into(),
        generation,
        manifest_sha256: hash,
    };
    output(kube(NAMESPACE).args([
        "exec",
        &basename(s),
        "--",
        SUPERVISOR,
        "rpc",
        &s.attempt.to_string(),
        &serde_json::to_string(&op).map_err(error)?,
    ]))?;
    Ok(())
}
fn status(s: &State) -> Result<Status, String> {
    let o = output(kube(NAMESPACE).args([
        "exec",
        &basename(s),
        "--",
        SUPERVISOR,
        "status",
        &s.attempt.to_string(),
    ]))?;
    let status: Status = serde_json::from_slice(&o.stdout).map_err(error)?;
    if status.request_sha256 != s.request_sha256 || status.attempt != s.attempt {
        return Err("supervisor identity mismatch".into());
    }
    Ok(status)
}
// Owner-observation collection only. This does not promote an external closure.
const OWNER_RECEIPT_CAP: usize = 256 * 1024;
const OWNER_STDERR_CAP: usize = 16 * 1024;

fn bounded_owner_stream(mut stream: impl std::io::Read, cap: usize) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let mut bytes = Vec::new();
    stream
        .by_ref()
        .take((cap + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(error)?;
    // Drain without retaining excess so a noisy child cannot deadlock its pipe.
    std::io::copy(&mut stream, &mut std::io::sink()).map_err(error)?;
    Ok(bytes)
}

fn owner_receipt_bytes(s: &State) -> Result<(bool, Vec<u8>, Vec<u8>), String> {
    let path = format!("/work/control/attempt-{}/owner-exit.json", s.attempt);
    // The path is derived only from the already-bound integer attempt. No shell,
    // receipt-provided path, new Kubernetes permission or payload credential.
    let mut child = kube(NAMESPACE)
        .args(["exec", &basename(s), "--", "head", "-c", "262145", &path])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(error)?;
    let stdout = child.stdout.take().ok_or("owner receipt stdout absent")?;
    let stderr = child.stderr.take().ok_or("owner receipt stderr absent")?;
    let out = thread::spawn(move || bounded_owner_stream(stdout, OWNER_RECEIPT_CAP));
    let err = thread::spawn(move || bounded_owner_stream(stderr, OWNER_STDERR_CAP));
    let status = child.wait().map_err(error);
    let stdout = out.join().map_err(|_| "owner stdout reader panicked")??;
    let stderr = err.join().map_err(|_| "owner stderr reader panicked")??;
    Ok((status?.success(), stdout, stderr))
}

// Existing controller save() replaces targets. Owner captures instead use an
// exact-byte idempotent exclusive publication; conflicts never overwrite bytes.
fn retain_owner_bytes(path: &Path, bytes: &[u8]) -> Result<(), String> {
    use std::io::Read;
    if let Ok(meta) = fs::symlink_metadata(path) {
        if !meta.is_file() || meta.file_type().is_symlink() || meta.len() != bytes.len() as u64 {
            return Err("owner receipt existing-path conflict".into());
        }
        let mut existing = Vec::new();
        let file = File::open(path).map_err(error)?;
        (&file)
            .take((bytes.len() + 1) as u64)
            .read_to_end(&mut existing)
            .map_err(error)?;
        if existing != bytes {
            return Err("owner receipt existing bytes differ".into());
        }
        file.sync_all().map_err(error)?;
        return File::open(path.parent().ok_or("owner receipt parent absent")?)
            .and_then(|parent| parent.sync_all())
            .map_err(error);
    }
    let temp = path.with_extension("owner-collection-tmp");
    let mut opts = fs::OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let mut file = opts.open(&temp).map_err(error)?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(error)?;
    drop(file);
    fs::hard_link(&temp, path).map_err(error)?;
    fs::remove_file(&temp).map_err(error)?;
    File::open(path.parent().ok_or("owner receipt parent absent")?)
        .and_then(|parent| parent.sync_all())
        .map_err(error)
}

#[derive(serde::Deserialize)]
struct OwnerReceiptBinding {
    schema: String,
    request_sha256: String,
    attempt: u32,
    accepted_external_closure: Value,
    total_active_worker_ns: Value,
}
fn check_owner_binding(bytes: &[u8], request: &str, attempt: u32) -> Result<(), String> {
    if bytes.len() > OWNER_RECEIPT_CAP {
        return Err("owner receipt exceeds cap".into());
    }
    // Derive rejects duplicate known binding fields. Other observation fields
    // remain exact raw custody; their semantics are not certified by this fetch.
    let receipt: OwnerReceiptBinding = serde_json::from_slice(bytes).map_err(error)?;
    if receipt.schema != "isolab.recovery.owner-exit/v1"
        || receipt.request_sha256 != request
        || receipt.attempt != attempt
        || !receipt.accepted_external_closure.is_null()
        || !receipt.total_active_worker_ns.is_null()
    {
        return Err("owner receipt schema/request/attempt/null-closure binding mismatch".into());
    }
    Ok(())
}

fn collect_owner_exit(s: &State, pod: &Value, st: &Status, dir: &Path) -> Result<(), String> {
    let capture = owner_receipt_bytes(s);
    let mut summary = json!({"schema":"isolab.recovery.owner-exit-collection/v1",
        "request_sha256":s.request_sha256,"attempt":s.attempt,"pod":basename(s),
        "pod_uid":pod["metadata"]["uid"],"supervisor_phase":st.phase,
        "transport":"existing_kubectl_exec","accepted_external_closure":null,
        "total_active_worker_ns":null,"collection_status":"unavailable",
        "exact_receipt_sha256":null,"raw_capture_file":null});
    match capture {
        Err(error) => {
            summary["collection_error"] = json!(error);
        }
        Ok((success, bytes, stderr)) => {
            let oversized = bytes.len() > OWNER_RECEIPT_CAP;
            if !bytes.is_empty() {
                let hash = digest(&bytes);
                let name = format!("owner-exit-attempt-{}-{}.json", s.attempt, hash);
                retain_owner_bytes(&dir.join(&name), &bytes)?;
                summary["raw_capture_file"] = json!(name);
                summary["raw_capture_sha256"] = json!(hash);
                summary["raw_capture_bytes"] = json!(bytes.len());
            }
            summary["transport_success"] = json!(success);
            summary["stdout_truncated"] = json!(oversized);
            summary["stderr_truncated"] = json!(stderr.len() > OWNER_STDERR_CAP);
            summary["stderr_prefix"] = json!(String::from_utf8_lossy(
                &stderr[..stderr.len().min(OWNER_STDERR_CAP)]
            ));
            let binding = if !success {
                Err("owner receipt transport failed or file missing".into())
            } else {
                check_owner_binding(&bytes, &s.request_sha256, s.attempt)
            };
            match binding {
                Ok(()) => {
                    summary["collection_status"] = json!("fetched_binding_checked_only");
                    summary["exact_receipt_sha256"] = json!(digest(&bytes));
                }
                Err(error) => {
                    summary["collection_error"] = json!(error);
                }
            }
        }
    }
    // Digest-addressed captures preserve differing retries without replacing the
    // old failure. Existing run-controller EXIT upload includes top-level *.json.
    let bytes = serde_json::to_vec_pretty(&summary).map_err(error)?;
    let name = format!(
        "owner-exit-collection-attempt-{}-{}.json",
        s.attempt,
        digest(&bytes)
    );
    retain_owner_bytes(&dir.join(name), &bytes)
}

// Existing resourceVersion CAS is the admission boundary. No campaign is
// auto-created: prior charged history must be reconciled before this path runs.
fn campaign_admit(
    s: &State,
    epoch: crate::campaign::Epoch,
) -> Result<Option<crate::campaign::Reservation>, String> {
    let Some(binding) = &s.request.worker_campaign else {
        return Ok(None);
    };
    let name = binding.policy.name();
    let mut object = get(NAMESPACE, "configmap", &name)?
        .ok_or("campaign admission ledger missing; refusing zero-balance initialization")?;
    let policy_hash = binding.policy.hash()?;
    if object["metadata"]["uid"] != binding.ledger_uid
        || object["metadata"]["annotations"]["isolab.dev/worker-policy"] != policy_hash
        || object["metadata"]["resourceVersion"]
            .as_str()
            .is_none_or(|s| s.is_empty())
        || object["metadata"]["uid"]
            .as_str()
            .is_none_or(|s| s.is_empty())
    {
        return Err("campaign ConfigMap identity/resourceVersion missing or mismatched".into());
    }
    let raw = field(&object, "/data/ledger.json")?;
    let mut ledger = crate::campaign::Ledger::from_bytes(raw.as_bytes())?;
    let before = ledger.clone();
    let spec_hash = digest(&serde_json::to_vec(&s.request.spec).map_err(error)?);
    let reserved = ledger.reserve(
        binding,
        &spec_hash,
        &s.request_sha256,
        s.attempt,
        epoch,
        crate::campaign::AdmissionWindow {
            request_deadline: s.request.deadline_unix_seconds,
            now: now(),
        },
    )?;
    if ledger != before {
        object["data"]["ledger.json"] = json!(serde_json::to_string(&ledger).map_err(error)?);
        // A conflict or ambiguous response aborts before worker creation. A
        // later reconciliation reloads the permanent reservation; no refund.
        let committed = stdin_json(
            kube(NAMESPACE).args(["replace", "-f", "-", "-o", "json"]),
            &object,
        )?;
        validate_campaign_cas_response(&object, &committed)?;
    }
    reserved.remaining_payload_seconds(now())?;
    Ok(Some(reserved))
}
fn validate_campaign_cas_response(submitted: &Value, committed: &Value) -> Result<(), String> {
    if committed["metadata"]["uid"] != submitted["metadata"]["uid"]
        || committed["data"]["ledger.json"] != submitted["data"]["ledger.json"]
        || committed["metadata"]["resourceVersion"]
            .as_str()
            .is_none_or(|s| s.is_empty())
        || committed["metadata"]["resourceVersion"] == submitted["metadata"]["resourceVersion"]
    {
        return Err("campaign CAS response binding mismatch; charge retained".into());
    }
    Ok(())
}
// Collection is not an admission to continue payload work. Retain failure
// evidence even when a grant expires, without converting the failure to success.
fn retain_then_reject<T>(
    reason: String,
    collect: impl FnOnce() -> Result<(), String>,
) -> Result<T, String> {
    match collect() {
        Ok(()) => Err(reason),
        Err(error) => Err(format!(
            "{reason}; terminal evidence retention failed: {error}"
        )),
    }
}
fn collection_call_fits(deadline: u64, observed_now: u64) -> Result<(), String> {
    // Existing kube transport has180s timeout plus5s forced-kill grace.
    if deadline
        .checked_sub(observed_now)
        .is_none_or(|left| left < 185)
    {
        return Err("insufficient absolute-deadline reserve for bounded evidence call".into());
    }
    Ok(())
}
fn retain_campaign_terminal_evidence(
    s: &State,
    dir: &Path,
    epoch: crate::campaign::Epoch,
) -> Result<(), String> {
    let binding = s
        .request
        .worker_campaign
        .as_ref()
        .ok_or("campaign binding absent")?;
    if now() >= binding.policy.absolute_deadline_unix_seconds {
        return Err("absolute campaign collection deadline reached".into());
    }
    let deadline = binding.policy.absolute_deadline_unix_seconds;
    collection_call_fits(deadline, now())?;
    match epoch {
        crate::campaign::Epoch::Computation => {
            let Some(pod) = get(NAMESPACE, "pod", &basename(s))? else {
                return Ok(());
            };
            identity(&pod, &s.request)?;
            collection_call_fits(deadline, now())?;
            let status = status(s)?;
            save(&dir.join("supervisor-status.json"), &status)?;
            if ["failed", "completed", "verified"].contains(&status.phase.as_str()) {
                collection_call_fits(deadline, now())?;
                collect_owner_exit(s, &pod, &status, dir)?;
            }
        }
        crate::campaign::Epoch::Fresh => {
            let Some(name) = &s.benchmark_job else {
                return Ok(());
            };
            let Some(job) = get("isolab", "job", name)? else {
                return Ok(());
            };
            identity(&job, &s.request)?;
            if s.benchmark_uid
                .as_ref()
                .is_some_and(|uid| job["metadata"]["uid"] != *uid)
            {
                return Err("terminal fresh Job UID mismatch".into());
            }
            let bytes = serde_json::to_vec_pretty(&job).map_err(error)?;
            let name = format!("benchmark-expired-job-{}.json", digest(&bytes));
            retain_owner_bytes(&dir.join(name), &bytes)?;
            if s.request.fresh_owner.is_some() {
                collection_call_fits(deadline, now())?;
                let job_name = s.benchmark_job.as_deref().ok_or("fresh Job name missing")?;
                let list = json(kube("isolab").args([
                    "get",
                    "pods",
                    "-l",
                    &format!("job-name={job_name}"),
                    "-o",
                    "json",
                ]))?;
                let items = list["items"].as_array().ok_or("fresh Pod list absent")?;
                if items.len() > 16 {
                    return Err("fresh Pod identity cap".into());
                }
                let mut pods = Vec::new();
                for pod in items {
                    let expected = job["metadata"]["uid"].as_str().ok_or("Job UID absent")?;
                    if !pod["metadata"]["ownerReferences"]
                        .as_array()
                        .is_some_and(|refs| {
                            refs.iter()
                                .any(|r| r["kind"] == "Job" && r["uid"] == expected)
                        })
                    {
                        return Err("fresh Pod/Job owner mismatch".into());
                    }
                    pods.push(json!({"uid":field(pod,"/metadata/uid")?}));
                }
                let status =
                    json!({"pods":pods,"source":"existing Kubernetes Pod ownerReferences"});
                collect_fresh_owner_evidence(s, &status, dir)?;
            }
        }
    }
    Ok(())
}

fn campaign_bind_manifest(
    v: &mut Value,
    reservation: &crate::campaign::Reservation,
    job: bool,
) -> Result<(), String> {
    let raw = serde_json::to_string(reservation).map_err(error)?;
    let hash = digest(raw.as_bytes());
    v["metadata"]["annotations"]["isolab.dev/worker-reservation"] = json!(hash);
    if !job {
        v["spec"]["hostPID"] = json!(false);
        v["spec"]["shareProcessNamespace"] = json!(false);
        let container = &mut v["spec"]["containers"][0];
        if container.get("env").is_none() {
            container["env"] = json!([]);
        }
        let env = container["env"]
            .as_array_mut()
            .ok_or("supervisor env not array")?;
        if env.iter().any(|v| {
            matches!(
                v["name"].as_str(),
                Some(
                    "ISOLAB_COMPUTATION_RESERVATION"
                        | "ISOLAB_COMPUTATION_RESERVATION_SHA256"
                        | "ISOLAB_COMPUTATION_PRIVATE_PID_NAMESPACE"
                )
            )
        }) {
            return Err("duplicate computation reservation transport".into());
        }
        env.push(json!({"name":"ISOLAB_COMPUTATION_PRIVATE_PID_NAMESPACE","value":"controller-owned-private-pid-namespace/v1"}));
        env.push(json!({"name":"ISOLAB_COMPUTATION_RESERVATION","value":raw}));
        env.push(json!({"name":"ISOLAB_COMPUTATION_RESERVATION_SHA256","valueFrom":{"fieldRef":{"fieldPath":"metadata.annotations['isolab.dev/worker-reservation']"}}}));
    }
    if job {
        v["spec"]["template"]["metadata"]["annotations"]["isolab.dev/worker-reservation"] =
            json!(hash);
    }
    Ok(())
}

// ObjectFieldSelector.apiVersion has the Kubernetes-documented default v1.
// Normalize ONLY omission of that field on an actual fieldRef object. Preserve
// every other key, value, ordering, selector, namespace and owner identity.
fn env_with_fieldref_default(value: &Value) -> Value {
    let mut normalized = value.clone();
    if let Some(entries) = normalized.as_array_mut() {
        for entry in entries {
            if let Some(reference) = entry
                .get_mut("valueFrom")
                .and_then(|v| v.get_mut("fieldRef"))
                .and_then(Value::as_object_mut)
            {
                reference.entry("apiVersion").or_insert_with(|| json!("v1"));
            }
        }
    }
    normalized
}

fn computation_owner_matches(observed: &Value, intended: &Value) -> Result<(), String> {
    if ["initContainers", "ephemeralContainers"].iter().any(|key| {
        observed["spec"]
            .get(*key)
            .is_some_and(|v| !v.as_array().is_some_and(Vec::is_empty))
    }) {
        return Err("unexpected computation init/ephemeral container".into());
    }
    if observed["spec"]["hostPID"].as_bool().unwrap_or(false)
        || observed["spec"]["shareProcessNamespace"]
            .as_bool()
            .unwrap_or(false)
        || observed["spec"]["containers"].as_array().map(Vec::len) != Some(1)
        || observed["metadata"]["annotations"]["isolab.dev/worker-reservation"]
            != intended["metadata"]["annotations"]["isolab.dev/worker-reservation"]
    {
        return Err("computation namespace/reservation differs from intended owner".into());
    }
    for key in [
        "name",
        "image",
        "command",
        "args",
        "env",
        "volumeMounts",
        "securityContext",
    ] {
        let actual = &observed["spec"]["containers"][0][key];
        let expected = &intended["spec"]["containers"][0][key];
        let same = if key == "env" {
            env_with_fieldref_default(actual) == env_with_fieldref_default(expected)
        } else {
            actual == expected
        };
        if !same {
            return Err(format!(
                "computation owner {key} differs from intended manifest"
            ));
        }
    }
    // Kubernetes may default volume mode; compare mount-source identities rather
    // than rejecting its harmless API-default fields.
    let volumes = observed["spec"]["volumes"]
        .as_array()
        .ok_or("computation volumes missing")?;
    let expected = intended["spec"]["volumes"]
        .as_array()
        .ok_or("intended volumes missing")?;
    if volumes.len() != expected.len() {
        return Err("unexpected computation volume".into());
    }
    for e in expected {
        let found: Vec<_> = volumes.iter().filter(|v| v["name"] == e["name"]).collect();
        if found.len() != 1 {
            return Err("computation volume identity missing/duplicate".into());
        }
        for key in ["persistentVolumeClaim", "secret", "hostPath", "emptyDir"] {
            if found[0][key] != e[key] {
                return Err("computation volume source differs".into());
            }
        }
        if found[0]["configMap"]["name"] != e["configMap"]["name"] {
            return Err("computation request volume differs".into());
        }
    }
    Ok(())
}

fn provision(s: &State) -> Result<Value, String> {
    let reservation = campaign_admit(s, crate::campaign::Epoch::Computation)?;
    let req = &s.request;
    let name = basename(s);
    ensure(
        NAMESPACE,
        &json!({"apiVersion":"v1","kind":"ConfigMap","metadata":metadata(&format!("{}-request",req.run_name),req),"immutable":true,"data":{"request.json":serde_json::to_string(req).map_err(error)?}}),
        req,
    )?;
    let mut pvc = json!({"apiVersion":"v1","kind":"PersistentVolumeClaim","metadata":metadata(&name,req),"spec":{"accessModes":["ReadWriteOnce"],"storageClassName":"gp3","resources":{"requests":{"storage":"8Gi"}}}});
    if let Some(number) = s.selected_generation {
        let g = s
            .generations
            .iter()
            .find(|g| g.number == number)
            .ok_or("missing selected generation")?;
        pvc["spec"]["dataSource"] = json!({"name":g.snapshot_name,"kind":"VolumeSnapshot","apiGroup":"snapshot.storage.k8s.io"});
    }
    ensure(NAMESPACE, &pvc, req)?;
    let remaining = req
        .deadline_unix_seconds
        .checked_sub(now())
        .filter(|n| *n > 0)
        .ok_or("original deadline expired")?;
    let remaining = match &reservation {
        Some(r) => remaining.min(r.remaining_payload_seconds(now())?),
        None => remaining,
    };
    let mut manifest = computation_pod(s, remaining);
    if let Some(r) = &reservation {
        campaign_bind_manifest(&mut manifest, r, false)?;
    }
    let observed = ensure(NAMESPACE, &manifest, req)?;
    if reservation.is_some() {
        computation_owner_matches(&observed, &manifest)?;
    }
    Ok(observed)
}
fn computation_pod(s: &State, remaining: u64) -> Value {
    let req = &s.request;
    let name = basename(s);
    json!({"apiVersion":"v1","kind":"Pod","metadata":metadata(&name,req),"spec":{"restartPolicy":"Never","activeDeadlineSeconds":remaining,"automountServiceAccountToken":false,"enableServiceLinks":false,"nodeSelector":{"isolab.dev/pool":"cpu-spot","node.kubernetes.io/instance-type":req.instance_type},"tolerations":[{"key":"isolab.dev/workload","operator":"Equal","value":"cpu","effect":"NoSchedule"}],"containers":[{"name":"supervisor","image":req.runtime_image_uri,"command":[SUPERVISOR,"supervise","/request/request.json",s.attempt.to_string()],"securityContext":{"privileged":true,"runAsUser":0,"runAsGroup":0},"resources":{"requests":{"cpu":"4","memory":req.memory.as_str()},"limits":{"cpu":"4","memory":req.memory.as_str()}},"volumeMounts":[{"name":"work","mountPath":"/work"},{"name":"request","mountPath":"/request","readOnly":true}]}],"volumes":[{"name":"work","persistentVolumeClaim":{"claimName":name}},{"name":"request","configMap":{"name":format!("{}-request",req.run_name)}}]}})
}
fn bind_placement(stored: &mut Stored, pod: &Value) -> Result<bool, String> {
    let Some(node_name) = pod["spec"]["nodeName"].as_str() else {
        return Ok(false);
    };
    let node = get(NAMESPACE, "node", node_name)?.ok_or("assigned node missing")?;
    let runtime = node_runtime(&node)?;
    match &stored.state.original_node_runtime {
        Some(original) if original != &runtime => {
            return Err(
                "replacement kernel/container runtime/OS class differs from original worker".into(),
            )
        }
        None if stored.state.attempt != 0 => {
            return Err("original worker runtime was not recorded; restore refused".into())
        }
        None => {
            stored.state.original_node_runtime = Some(runtime);
            stored.save()?;
        }
        _ => {}
    }
    if node["metadata"]["labels"]["node.kubernetes.io/instance-type"]
        != stored.state.request.instance_type
        || node["metadata"]["labels"]["karpenter.sh/capacity-type"] != "spot"
    {
        return Err("worker is not the qualified Spot instance class".into());
    }
    let instance = field(&node, "/spec/providerID")?
        .rsplit('/')
        .next()
        .filter(|x| x.starts_with("i-"))
        .ok_or("node instance identity missing")?
        .to_owned();
    let uid = field(pod, "/metadata/uid")?.to_owned();
    if stored
        .state
        .current_pod_uid
        .as_ref()
        .is_some_and(|s| s != &uid)
        || stored
            .state
            .current_instance
            .as_ref()
            .is_some_and(|s| s != &instance)
    {
        return Err("attempt placement identity changed".into());
    }
    if stored.state.current_pod_uid.is_none() {
        stored.state.current_pod_uid = Some(uid);
        stored.state.current_instance = Some(instance);
        stored.save()?;
    }
    let runtime = &stored.state.request.runtime_image_uri;
    if pod["spec"]["containers"][0]["image"] != *runtime {
        return Err("supervisor Pod image differs from request".into());
    }
    Ok(pod["status"]["phase"] == "Running"
        && pod["status"]["containerStatuses"]
            .as_array()
            .is_some_and(|cs| {
                cs.iter().any(|c| {
                    c["name"] == "supervisor"
                        && c["ready"] == true
                        && c["imageID"].as_str().is_some_and(|id| {
                            id.ends_with(runtime.split('@').nth(1).unwrap_or("INVALID"))
                        })
                })
            }))
}
fn snapshot(s: &State, n: u64) -> Result<Option<(String, String)>, String> {
    let name = format!("{}-g{n}", s.request.run_name);
    let v = ensure(
        NAMESPACE,
        &json!({"apiVersion":"snapshot.storage.k8s.io/v1","kind":"VolumeSnapshot","metadata":metadata(&name,&s.request),"spec":{"volumeSnapshotClassName":"isolab-ebs-checkpoints","source":{"persistentVolumeClaimName":basename(s)}}}),
        &s.request,
    )?;
    if v["status"]["error"].is_object() {
        return Err(format!("EBS snapshot failed: {}", v["status"]["error"]));
    }
    if v["status"]["readyToUse"] != true {
        return Ok(None);
    }
    let content = get(
        NAMESPACE,
        "volumesnapshotcontent",
        field(&v, "/status/boundVolumeSnapshotContentName")?,
    )?
    .ok_or("snapshot content missing")?;
    if content["spec"]["volumeSnapshotRef"]["uid"] != v["metadata"]["uid"] {
        return Err("snapshot content binding mismatch".into());
    }
    let id = field(&content, "/status/snapshotHandle")?.to_owned();
    let aws = json(aws().args(["ec2", "describe-snapshots", "--snapshot-ids", &id]))?;
    let snap = &aws["Snapshots"][0];
    if snap["SnapshotId"] != id
        || snap["OwnerId"] != ACCOUNT
        || snap["Encrypted"] != true
        || snap["State"] != "completed"
    {
        return Err("snapshot is not an encrypted completed snapshot owned by this account".into());
    }
    Ok(Some((name, id)))
}
fn capture(s: &State, n: u64, directory: &Path) -> Result<(PathBuf, String), String> {
    fs::create_dir_all(directory).map_err(error)?;
    let manifest = output(kube(NAMESPACE).args([
        "exec",
        &basename(s),
        "--",
        "cat",
        &format!("/work/generations/g{n}/manifest.json"),
    ]))?
    .stdout;
    let m: crate::supervisor::Manifest = serde_json::from_slice(&manifest).map_err(error)?;
    if m.request_sha256 != s.request_sha256 || m.generation != n {
        return Err("manifest not bound to this generation".into());
    }
    fs::write(directory.join("manifest.json"), &manifest).map_err(error)?;
    let archive = directory.join("checkpoint.tar");
    let file = File::create(&archive).map_err(error)?;
    let result = kube(NAMESPACE)
        .args([
            "exec",
            &basename(s),
            "--",
            SUPERVISOR,
            "archive",
            &n.to_string(),
        ])
        .stdout(file)
        .stderr(Stdio::piped())
        .output()
        .map_err(error)?;
    if !result.status.success() {
        return Err(format!(
            "checkpoint export failed: {}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    Ok((archive, digest(&manifest)))
}
fn put_verified(path: &Path, key: &str, dir: &Path) -> Result<(String, String, u64), String> {
    let (sha, size) = sha_file(path)?;
    let result = json(
        aws()
            .args([
                "s3api",
                "put-object",
                "--bucket",
                BUCKET,
                "--key",
                key,
                "--body",
            ])
            .arg(path)
            .args(["--server-side-encryption", "AES256"]),
    )?;
    let version = field(&result, "/VersionId")?.to_owned();
    if version == "null" {
        return Err("checkpoint bucket is not versioned".into());
    }
    let readback = dir.join("s3-readback");
    let obj = json(
        aws()
            .args([
                "s3api",
                "get-object",
                "--bucket",
                BUCKET,
                "--key",
                key,
                "--version-id",
                &version,
            ])
            .arg(&readback),
    )?;
    if obj["VersionId"] != version
        || obj["ServerSideEncryption"] != "AES256"
        || sha_file(&readback)? != (sha.clone(), size)
    {
        return Err("complete S3 checkpoint readback failed".into());
    }
    fs::remove_file(readback).map_err(error)?;
    Ok((version, sha, size))
}
fn verify_archive(g: &Generation, dir: &Path) -> Result<(), String> {
    let path = dir.join(format!("restore-g{}.tar", g.number));
    let v = json(
        aws()
            .args([
                "s3api",
                "get-object",
                "--bucket",
                BUCKET,
                "--key",
                &g.archive_key,
                "--version-id",
                &g.archive_version,
            ])
            .arg(&path),
    )?;
    if v["VersionId"] != g.archive_version
        || v["ServerSideEncryption"] != "AES256"
        || sha_file(&path)? != (g.archive_sha256.clone(), g.archive_bytes)
    {
        return Err("committed checkpoint archive is corrupt or has wrong identity".into());
    }
    let manifest = output(
        Command::new("tar")
            .arg("-xOf")
            .arg(&path)
            .arg(format!("generations/g{}/manifest.json", g.number)),
    )?
    .stdout;
    if digest(&manifest) != g.manifest_sha256 {
        return Err("archive manifest is not committed manifest".into());
    }
    Ok(())
}
fn checkpoint(stored: &mut Stored, st: &Status, dir: &Path) -> Result<(), String> {
    let s = &mut stored.state;
    if st.phase == "completed" {
        s.phase = Phase::VerifyingComputation;
        stored.save()?;
        return Ok(());
    }
    if st.phase == "running" {
        if s.checkpoint_resumed(st.generation)? {
            s.finish_checkpoint();
            stored.save()?;
            return Ok(());
        }
        rpc(s, "checkpoint", s.intended_checkpoint()?, None)?;
        return Ok(());
    }
    if st.phase == "checkpointing" {
        return Ok(());
    }
    if st.phase != "stopped" {
        return Err(format!(
            "unexpected checkpoint supervisor state: {}",
            st.phase
        ));
    }
    let n = st
        .generation
        .ok_or("stopped checkpoint generation missing")?;
    if n != s.intended_checkpoint()? {
        return Err("stopped generation is not the intended generation".into());
    }
    if let Some(g) = s.generations.iter().find(|g| g.number == n) {
        rpc(s, "continue", n, Some(g.manifest_sha256.clone()))?;
        s.finish_checkpoint();
        stored.save()?;
        return Ok(());
    }
    if n != s.next_generation {
        return Err("stopped generation is not the intended generation".into());
    }
    let Some((snapshot_name, snapshot_id)) = snapshot(s, n)? else {
        return Ok(());
    };
    let gen_dir = dir.join(format!("g{n}"));
    let (archive, manifest_sha256) = capture(s, n, &gen_dir)?;
    let archive_key = format!("{}generations/g{n}/checkpoint.tar", prefix(&s.request));
    let (archive_version, archive_sha256, archive_bytes) =
        put_verified(&archive, &archive_key, &gen_dir)?;
    s.commit(Generation {
        number: n,
        request_sha256: s.request_sha256.clone(),
        snapshot_name,
        snapshot_id,
        archive_key,
        archive_version,
        archive_sha256,
        archive_bytes,
        manifest_sha256,
        source_instance: s.current_instance.clone().ok_or("missing worker")?,
        source_pod_uid: s.current_pod_uid.clone().ok_or("missing pod")?,
        committed_unix_seconds: now(),
        snapshot_ready: true,
        encrypted: true,
        readback_verified: true,
    })?;
    stored.save()?; // Commit BEFORE resuming the stopped writer.
    Ok(())
}
// Pin only the Kubernetes ObjectFieldSelector API default. This keeps exact
// owner env/volume comparisons valid after API round-trip without dropping keys.
fn pin_fresh_selector_defaults(manifest: &mut Value) {
    let spec = &mut manifest["spec"]["template"]["spec"];
    if let Some(containers) = spec["containers"].as_array_mut() {
        for container in containers {
            if let Some(env) = container["env"].as_array_mut() {
                for entry in env {
                    if let Some(selector) = entry
                        .get_mut("valueFrom")
                        .and_then(|v| v.get_mut("fieldRef"))
                        .and_then(Value::as_object_mut)
                    {
                        selector.entry("apiVersion").or_insert_with(|| json!("v1"));
                    }
                }
            }
        }
    }
    if let Some(volumes) = spec["volumes"].as_array_mut() {
        for volume in volumes {
            if let Some(items) = volume
                .get_mut("downwardAPI")
                .and_then(|v| v.get_mut("items"))
                .and_then(Value::as_array_mut)
            {
                for item in items {
                    if let Some(selector) = item.get_mut("fieldRef").and_then(Value::as_object_mut)
                    {
                        selector.entry("apiVersion").or_insert_with(|| json!("v1"));
                    }
                }
            }
        }
    }
}
fn manifest_job(s: &State, remaining: u64) -> Result<Value, String> {
    let deploy = std::env::var("ISOLAB_DEPLOY_DIR").map_err(error)?;
    let name = s.benchmark_job.as_deref().ok_or("benchmark name missing")?;
    let output = output(Command::new(Path::new(&deploy).join("submit-job")).args([
        "--isolation-profile",
        "l2",
        "--instance-type",
        &s.request.instance_type,
        "--image",
        &s.request.build.image_uri,
        "--name",
        name,
        "--cpu",
        "4",
        "--memory",
        s.request.memory.as_str(),
        "--seconds",
        &remaining.to_string(),
        "--retries",
        "0",
        "--dry-run",
        "--",
        "bash",
        "-c",
        include_str!("../../../run-ecbench"),
        "isolab-ecbench",
        &serde_json::to_string(&s.request.spec).map_err(error)?,
        &s.request.build.source_revision,
        &s.request.build.ecbench_sha256,
    ]))?;
    let mut v: Value = serde_json::from_slice(&output.stdout).map_err(error)?;
    v["metadata"]["annotations"]["isolab.dev/recovery-request"] = json!(s.request_sha256);
    v["metadata"]["annotations"]["isolab.dev/logical-run"] = json!(s.request.run_name);
    if let Some(transport) = &s.request.diagnostic_transport {
        transport.validate(&s.request.spec)?;
        let annotations = &mut v["spec"]["template"]["metadata"]["annotations"];
        annotations["isolab.dev/diagnostic-fresh-launch"] = json!(transport.fresh.raw_json);
        // Kubelet publishes this annotation as a root-owned read-only file. The
        // fixed subPath mount does not track later annotation updates.
        let volume = json!({"name":"diagnostic-input","downwardAPI":{"defaultMode":292,
            "items":[{"path":"fresh-launch.json","fieldRef":{"fieldPath":"metadata.annotations['isolab.dev/diagnostic-fresh-launch']"}}]}});
        v["spec"]["template"]["spec"]["volumes"]
            .as_array_mut()
            .ok_or("job volumes absent")?
            .push(volume);
        let task = &mut v["spec"]["template"]["spec"]["containers"][0];
        if task["name"] != "task" {
            return Err("unexpected fresh task container".into());
        }
        task["volumeMounts"].as_array_mut().ok_or("job mounts absent")?.push(json!({"name":"diagnostic-input",
            "mountPath":"/opt/isolab/diagnostic-input/fresh-launch.json","subPath":"fresh-launch.json","readOnly":true}));
        task["command"]
            .as_array_mut()
            .ok_or("job command absent")?
            .extend([
                json!("--diagnostic-launch"),
                json!("/opt/isolab/diagnostic-input/fresh-launch.json"),
                json!("--diagnostic-launch-sha256"),
                json!(transport.fresh.sha256),
                json!("--diagnostic-spec-sha256"),
                json!(transport.fresh.spec_bytes_sha256),
            ]);
    }
    if let Some(owner) = &s.request.fresh_owner {
        if s.request.diagnostic_transport.is_none()
            || owner.stop_utc_seconds <= now()
            || owner.stop_utc_seconds > s.request.deadline_unix_seconds
            || !hex(&owner.binary_sha256, 64)
        {
            return Err("fresh owner admission unavailable or expired".into());
        }
        let image = s
            .request
            .build
            .image_uri
            .rsplit_once("@sha256:")
            .ok_or("fresh image digest missing")?
            .1;
        let task = &mut v["spec"]["template"]["spec"]["containers"][0];
        let env = task["env"].as_array_mut().ok_or("fresh env absent")?;
        for (name, value) in [
            ("ISOLAB_FRESH_OWNER_SHA256", owner.binary_sha256.clone()),
            ("ISOLAB_OWNER_REQUEST_SHA256", s.request_sha256.clone()),
            ("ISOLAB_OWNER_IMAGE_SHA256", image.to_string()),
            ("ISOLAB_OWNER_STOP_UTC", owner.stop_utc_seconds.to_string()),
        ] {
            if env.iter().any(|v| v["name"] == name) {
                return Err("duplicate owner env".into());
            }
            env.push(json!({"name":name,"value":value}));
        }
        env.push(json!({"name":"ISOLAB_OWNER_JOB_UID","valueFrom":{"fieldRef":{"fieldPath":"metadata.labels['batch.kubernetes.io/controller-uid']"}}}));
    }
    if s.request.fresh_owner.is_some() {
        pin_fresh_selector_defaults(&mut v);
    }
    Ok(v)
}
fn mcp(name: &str, job: &str) -> Result<Value, String> {
    json(Command::new("isolab-eks-mcp").args(["call", name, &json!({"job_name":job}).to_string()]))
}
fn verify_benchmark(s: &State, pod_uid: &str, status: &Value, dir: &Path) -> Result<Value, String> {
    let job = s.benchmark_job.as_deref().ok_or("missing epoch name")?;
    let artifact = dir.join("benchmark-artifacts");
    fs::create_dir_all(&artifact).map_err(error)?;
    let uri = format!("s3://{BUCKET}/runs/{job}/{pod_uid}/");
    output(
        aws()
            .args(["s3", "cp", &uri])
            .arg(&artifact)
            .args(["--recursive", "--only-show-errors"]),
    )?;
    // Validate path names and completeness before invoking the maintained verifier.
    let sums = fs::read_to_string(artifact.join("SHA256SUMS")).map_err(error)?;
    let mut named = std::collections::BTreeSet::new();
    for line in sums.lines() {
        let (hash, name) = line.split_once("  ").ok_or("invalid S3 hash manifest")?;
        let name = name.trim_start_matches("./");
        if !hex(hash, 64)
            || Path::new(name)
                .components()
                .any(|c| !matches!(c, std::path::Component::Normal(_)))
            || !named.insert(name.to_owned())
            || sha_file(&artifact.join(name))?.0 != hash
        {
            return Err("S3 payload hash or path verification failed".into());
        }
    }
    fn actual(
        dir: &Path,
        base: &Path,
        out: &mut std::collections::BTreeSet<String>,
    ) -> Result<(), String> {
        for e in fs::read_dir(dir).map_err(error)? {
            let e = e.map_err(error)?;
            if e.file_type().map_err(error)?.is_dir() {
                actual(&e.path(), base, out)?;
            } else {
                let name = e
                    .path()
                    .strip_prefix(base)
                    .map_err(error)?
                    .to_str()
                    .ok_or("invalid artifact path")?
                    .to_owned();
                if name != "SHA256SUMS" {
                    out.insert(name);
                }
            }
        }
        Ok(())
    }
    let mut names = std::collections::BTreeSet::new();
    actual(&artifact, &artifact, &mut names)?;
    if named != names || named.is_empty() {
        return Err("S3 manifest does not cover every payload".into());
    }
    let request = json!({"schema":"isolab.eks.request/v1","job":{"job_name":job,"build":s.request.build,"spec":s.request.spec}});
    save(&dir.join("benchmark-request.json"), &request)?;
    save(&dir.join("benchmark-status.json"), status)?;
    let deploy = std::env::var("ISOLAB_DEPLOY_DIR").map_err(error)?;
    output(
        Command::new(Path::new(&deploy).join("remote/verify-build"))
            .arg(dir.join("benchmark-request.json"))
            .arg(dir.join("benchmark-status.json"))
            .arg(pod_uid)
            .arg(&artifact),
    )?;
    let owner_check = if s.request.fresh_owner.is_some() {
        Some(crate::fresh_owner::check(s, pod_uid, &artifact)?)
    } else {
        None
    };
    if let Some(check) = &owner_check {
        let bytes = serde_json::to_vec_pretty(check).map_err(error)?;
        retain_owner_bytes(
            &dir.join(format!(
                "fresh-owner-controller-check-{}.json",
                digest(&bytes)
            )),
            &bytes,
        )?;
    }
    let gate: Value = read(&artifact.join("l2-gate.json"))?;
    let audit: Value = read(&artifact.join("audit.json"))?;
    let control: Value = read(&artifact.join("control-comparison.json"))?;
    let exit: Value = read(&artifact.join("job-exit.json"))?;
    if exit["exit_code"] != 0
        || gate["accepted"] != true
        || gate["executions"].as_u64().unwrap_or(0) == 0
        || gate["below_l2"] != json!([])
        || gate["invalid_outcomes"] != json!([])
        || audit["ok"] != true
        || audit["regraded"] != true
        || audit["records_bound"] != true
        || audit["records"].as_u64().unwrap_or(0) == 0
        || !audit["replays"]
            .as_array()
            .is_some_and(|r| !r.is_empty() && r.iter().all(|r| r["reproduced"] == true))
        || control["wall"]["status"] != "admitted"
        || control["wall"]["aa_pairs"].as_u64().unwrap_or(0) < 5
        || !control["wall"]["aa_ci95"].is_array()
    {
        return Err("fresh epoch L2/replay/A-A admission failed".into());
    }
    Ok(
        json!({"job_name":job,"job_uid":s.benchmark_uid,"pod_uid":pod_uid,"results_uri":uri,"payload_hashes_verified":named.len(),"fresh_owner_identity_check":owner_check,"l2":gate,"native_audit":audit,"control":control}),
    )
}
fn benchmark(stored: &mut Stored, dir: &Path) -> Result<(), String> {
    let reservation = match campaign_admit(&stored.state, crate::campaign::Epoch::Fresh) {
        Ok(reservation) => reservation,
        Err(error) => {
            return retain_then_reject(error, || {
                retain_campaign_terminal_evidence(&stored.state, dir, crate::campaign::Epoch::Fresh)
            })
        }
    };
    if stored.state.benchmark_manifest.is_none() {
        let remaining = stored
            .state
            .request
            .deadline_unix_seconds
            .checked_sub(now() + 120)
            .filter(|x| *x >= 60)
            .ok_or("insufficient original deadline for fresh epoch")?;
        let remaining = match &reservation {
            Some(r) => remaining.min(r.remaining_payload_seconds(now())?),
            None => remaining,
        };
        let mut manifest = manifest_job(&stored.state, remaining)?;
        if let Some(r) = &reservation {
            campaign_bind_manifest(&mut manifest, r, true)?;
            if let Some(owner) = &stored.state.request.fresh_owner {
                let stop = owner.stop_utc_seconds.min(r.deadline_unix_seconds);
                let env = manifest["spec"]["template"]["spec"]["containers"][0]["env"]
                    .as_array_mut()
                    .ok_or("fresh env missing")?;
                let value = env
                    .iter_mut()
                    .find(|v| v["name"] == "ISOLAB_OWNER_STOP_UTC")
                    .ok_or("fresh owner cutoff env absent")?;
                value["value"] = json!(stop.to_string());
            }
        }
        stored.state.benchmark_manifest = Some(manifest);
        stored.save()?;
    }
    let name = stored.state.benchmark_job.clone().ok_or("epoch missing")?;
    let existing = get("isolab", "job", &name)?;
    if existing.is_none() {
        if stored.state.benchmark_create_attempted {
            return Err("previous epoch submission is ambiguous or disappeared; refusing a second submission".into());
        }
        stored.state.benchmark_create_attempted = true;
        stored.save()?;
    }
    let s = &mut stored.state;
    let name = s.benchmark_job.clone().ok_or("epoch missing")?;
    let intended = s
        .benchmark_manifest
        .as_ref()
        .ok_or("epoch manifest missing")?;
    if let Some(r) = &reservation {
        let hash = digest(&serde_json::to_vec(r).map_err(error)?);
        if intended["metadata"]["annotations"]["isolab.dev/worker-reservation"] != hash {
            return Err(
                "retained fresh manifest differs from admitted campaign reservation".into(),
            );
        }
    }
    let current = match existing {
        Some(v) => v,
        None => {
            if s.benchmark_uid.is_some() {
                return Err("previous epoch disappeared; refusing to rerun timing".into());
            }
            create("isolab", intended)?
        }
    };
    identity(&current, &s.request)?;
    if reservation.is_some()
        && current["metadata"]["annotations"]["isolab.dev/worker-reservation"]
            != intended["metadata"]["annotations"]["isolab.dev/worker-reservation"]
    {
        return Err("existing fresh Job lacks matching campaign reservation".into());
    }
    if current["spec"]["template"]["spec"]["containers"][0]["command"]
        != intended["spec"]["template"]["spec"]["containers"][0]["command"]
        || current["spec"]["template"]["spec"]["containers"][0]["image"]
            != s.request.build.image_uri
    {
        return Err("existing benchmark differs from immutable intent".into());
    }
    if s.request.fresh_owner.is_some()
        && (current["spec"]["template"]["spec"]["containers"][0]["env"]
            != intended["spec"]["template"]["spec"]["containers"][0]["env"]
            || current["spec"]["template"]["spec"]["volumes"]
                != intended["spec"]["template"]["spec"]["volumes"]
            || current["spec"]["template"]["spec"]["containers"][0]["volumeMounts"]
                != intended["spec"]["template"]["spec"]["containers"][0]["volumeMounts"])
    {
        return Err(
            "existing fresh owner environment or mounts differ from immutable intent".into(),
        );
    }
    let uid = field(&current, "/metadata/uid")?.to_owned();
    if s.benchmark_uid.as_ref().is_some_and(|x| x != &uid) {
        return Err("epoch job identity changed".into());
    }
    if s.benchmark_uid.is_none() {
        s.benchmark_uid = Some(uid);
        stored.save()?;
        return Ok(());
    }
    // Queue the fresh epoch before releasing its predecessor's CPU allocation.
    // This lets the scheduler reuse that worker and avoids an empty-node
    // consolidation gap while EC2 still charges its terminating Spot vCPUs.
    output(kube(NAMESPACE).args([
        "delete",
        "pod",
        &basename(s),
        "--ignore-not-found",
        "--wait=false",
    ]))?;
    let status = mcp("isolab_k8s_status", &name)?;
    save(&dir.join("benchmark-status.json"), &status)?;
    if s.request.fresh_owner.is_some()
        && matches!(
            status["job"]["state"].as_str(),
            Some("failed" | "succeeded")
        )
    {
        // Evidence collection precedes interpretation, and never executes a verifier or replay.
        if let Err(e) = collect_fresh_owner_evidence(s, &status, dir) {
            if let Err(retention) = save(
                &dir.join("fresh-owner-collection-error.json"),
                &json!({"error":e,"scientific_acceptance":null}),
            ) {
                eprintln!("fresh owner collection and local retention failed: {e}; {retention}");
            }
        }
    }
    if status["job"]["state"] == "failed" {
        return Err("fresh benchmark epoch failed; measurement is not retried".into());
    }
    if status["job"]["state"] != "succeeded" {
        return Ok(());
    }
    let pods = status["pods"].as_array().ok_or("epoch pod list missing")?;
    if pods.len() != 1 {
        return Err("fresh epoch must have exactly one attempt".into());
    }
    let uid = field(&pods[0], "/uid")?;
    let receipt = verify_benchmark(s, uid, &status, dir)?;
    save(&dir.join("benchmark-result.json"), &receipt)?;
    output(
        aws()
            .args(["s3", "cp"])
            .arg(dir.join("benchmark-result.json"))
            .arg(format!(
                "s3://{BUCKET}/{}benchmark-result.json",
                prefix(&s.request)
            ))
            .arg("--only-show-errors"),
    )?;
    s.enforce_deadline(now())?;
    s.phase = Phase::Succeeded;
    stored.save()?;
    Ok(())
}
fn node_runtime(node: &Value) -> Result<Value, String> {
    let mut result = serde_json::Map::new();
    for key in [
        "kernelVersion",
        "containerRuntimeVersion",
        "osImage",
        "architecture",
    ] {
        let value = field(node, &format!("/status/nodeInfo/{key}"))?;
        if value.is_empty() {
            return Err("empty node runtime field".into());
        }
        result.insert(key.into(), json!(value));
    }
    if result["architecture"] != "amd64" {
        return Err("recovery requires amd64".into());
    }
    Ok(Value::Object(result))
}
fn tick(stored: &mut Stored, dir: &Path) -> Result<(), String> {
    if stored.state.request.worker_campaign.is_some()
        && now() >= stored.state.request.deadline_unix_seconds
    {
        let epoch = if stored.state.phase == Phase::Benchmark {
            crate::campaign::Epoch::Fresh
        } else {
            crate::campaign::Epoch::Computation
        };
        return retain_then_reject(
            "original request deadline expired; no new work admitted".into(),
            || retain_campaign_terminal_evidence(&stored.state, dir, epoch),
        );
    }
    stored.state.enforce_deadline(now())?;
    if stored.state.phase == Phase::Benchmark {
        return benchmark(stored, dir);
    }
    // EC2 outlives Kubernetes Node records. Check the durable instance identity
    // before attempting to read a Node that Karpenter may already have removed.
    if stored.state.phase != Phase::Fencing {
        if let Some(id) = &stored.state.current_instance {
            if instance_state(id)?.as_deref() == Some("terminated") {
                stored.state.phase = Phase::Fencing;
                stored.save()?;
                return Ok(());
            }
        }
    }
    if stored.state.phase == Phase::Fencing {
        let id = stored
            .state
            .current_instance
            .clone()
            .ok_or("no previous writer identity; cannot fence")?;
        let state = instance_state(&id)?;
        if state.as_deref() != Some("terminated") {
            return Ok(());
        }
        stored.state.confirm_terminated(&id, state.as_deref())?;
        if stored.state.request.worker_campaign.is_some() {
            // Reserve the next restoration before archive verification work.
            // This remains charged if verification later fails or no restore runs.
            let mut restoration = stored.state.clone();
            restoration.attempt = restoration
                .attempt
                .checked_add(1)
                .ok_or("attempt overflow")?;
            campaign_admit(&restoration, crate::campaign::Epoch::Computation)?;
        }
        let mut selected = None;
        for g in stored.state.generations.iter().rev() {
            match verify_archive(g, dir) {
                Ok(()) => {
                    selected = Some(g.number);
                    break;
                }
                Err(e) => {
                    save(
                        &dir.join(format!("rejected-generation-{}.json", g.number)),
                        &json!({"error":e,"generation":g,"at":now()}),
                    )?;
                }
            }
        }
        let number =
            selected.ok_or("no valid committed generation; refusing to restart from zero")?;
        stored.state.begin_restore(number, now())?;
        stored.save()?;
        return Ok(());
    }
    let pod = if matches!(stored.state.phase, Phase::Provisioning | Phase::Restoring) {
        // Reconcile an already-created Pod before attempting fresh admission.
        // An expired grant must not hide its terminal owner receipt.
        match get(NAMESPACE, "pod", &basename(&stored.state))? {
            Some(pod) => pod,
            None => provision(&stored.state)?,
        }
    } else {
        match get(NAMESPACE, "pod", &basename(&stored.state))? {
            Some(p) => p,
            None => {
                stored.state.phase = Phase::Fencing;
                stored.save()?;
                return Ok(());
            }
        }
    };
    identity(&pod, &stored.state.request)?;
    if !bind_placement(stored, &pod)? {
        if stored.state.current_instance.is_some()
            && (pod["status"]["phase"] == "Failed" || pod["status"]["phase"] == "Unknown")
        {
            stored.state.phase = Phase::Fencing;
            stored.save()?;
        }
        return Ok(());
    }
    let st = match status(&stored.state) {
        Ok(s) => s,
        Err(e) => {
            save(
                &dir.join("last-contact-error.json"),
                &json!({"error":e,"at":now()}),
            )?;
            if let Some(id) = &stored.state.current_instance {
                if instance_state(id)?.as_deref() == Some("terminated") {
                    stored.state.phase = Phase::Fencing;
                    stored.save()?;
                }
            }
            return Ok(());
        }
    };
    save(&dir.join("supervisor-status.json"), &st)?;
    // Collect protected owner observations before either terminal branch. Missing
    // receipts remain explicit unknown evidence, never fabricated exit/closure.
    let owner_collection = if ["failed", "completed", "verified"].contains(&st.phase.as_str()) {
        collect_owner_exit(&stored.state, &pod, &st, dir)
    } else {
        Ok(())
    };
    if st.phase == "failed" {
        let original = st
            .error
            .clone()
            .unwrap_or_else(|| format!("computation exited {:?}", st.exit_code));
        return Err(match owner_collection {
            Ok(()) => original,
            Err(collection) => format!("{original}; owner receipt retention failed: {collection}"),
        });
    }
    owner_collection.map_err(|error| format!("owner receipt retention failed: {error}"))?;
    // All read-only terminal custody precedes this gate. Expiry still blocks
    // start/restore/continue/checkpoint/verification or further work below.
    if let Some(reservation) = campaign_admit(&stored.state, crate::campaign::Epoch::Computation)? {
        let expected = digest(&serde_json::to_vec(&reservation).map_err(error)?);
        if pod["metadata"]["annotations"]["isolab.dev/worker-reservation"] != expected {
            return Err("existing computation Pod lacks matching campaign reservation".into());
        }
        let mut intended = computation_pod(&stored.state, 1);
        campaign_bind_manifest(&mut intended, &reservation, false)?;
        computation_owner_matches(&pod, &intended)?;
    }
    match stored.state.phase {
        Phase::Provisioning => {
            if st.phase == "waiting" {
                rpc(&stored.state, "start", 0, None)?;
            } else if ["running", "completed"].contains(&st.phase.as_str()) {
                stored.state.phase = Phase::Running;
                stored.save()?;
            }
        }
        Phase::Restoring => {
            let n = stored
                .state
                .selected_generation
                .ok_or("restore generation missing")?;
            let g = stored
                .state
                .generations
                .iter()
                .find(|g| g.number == n)
                .ok_or("generation missing")?;
            if st.phase == "waiting" {
                rpc(&stored.state, "restore", n, Some(g.manifest_sha256.clone()))?;
            } else if st.phase == "stopped" {
                rpc(
                    &stored.state,
                    "continue",
                    n,
                    Some(g.manifest_sha256.clone()),
                )?;
            } else if st.phase == "running" || st.phase == "completed" {
                stored.state.phase = Phase::Running;
                stored.state.last_checkpoint_at = now();
                stored.save()?;
            }
        }
        Phase::Running => {
            if st.phase == "completed" {
                stored.state.phase = Phase::VerifyingComputation;
                stored.save()?;
            } else if st.phase == "running"
                && now()
                    >= stored.state.last_checkpoint_at
                        + stored.state.request.checkpoint_interval_seconds
            {
                stored.state.begin_checkpoint()?;
                stored.save()?;
            }
        }
        Phase::Checkpointing => checkpoint(stored, &st, dir)?,
        Phase::VerifyingComputation => {
            if st.phase == "completed" {
                rpc(&stored.state, "verify", 0, None)?;
            } else if st.phase == "verified" {
                let archive = dir.join("computation.tar");
                let result = kube(NAMESPACE)
                    .args([
                        "exec",
                        &basename(&stored.state),
                        "--",
                        SUPERVISOR,
                        "archive-computation",
                    ])
                    .stdout(File::create(&archive).map_err(error)?)
                    .stderr(Stdio::piped())
                    .output()
                    .map_err(error)?;
                if !result.status.success() {
                    return Err("computation evidence export failed".into());
                }
                let key = format!("{}computation.tar", prefix(&stored.state.request));
                let (version, hash, bytes) = put_verified(&archive, &key, dir)?;
                save(
                    &dir.join("computation-result.json"),
                    &json!({"schema":"isolab.eks.computation-result/v1","key":key,"version":version,"sha256":hash,"bytes":bytes,"timing_valid":false,"native_replay":true,"supervisor":st}),
                )?;
                output(
                    aws()
                        .args(["s3", "cp"])
                        .arg(dir.join("computation-result.json"))
                        .arg(format!(
                            "s3://{BUCKET}/{}computation-result.json",
                            prefix(&stored.state.request)
                        ))
                        .arg("--only-show-errors"),
                )?;
                stored.state.begin_benchmark(true)?;
                stored.save()?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn publish(stored: &Stored, dir: &Path) -> Result<(), String> {
    save(&dir.join("state.json"), &stored.state)?;
    output(
        aws()
            .args(["s3", "cp"])
            .arg(dir.join("state.json"))
            .arg(format!(
                "s3://{BUCKET}/{}state.json",
                prefix(&stored.state.request)
            ))
            .arg("--only-show-errors"),
    )?;
    Ok(())
}
pub fn run(req: Request, dir: &Path) -> Result<(), String> {
    fs::create_dir_all(dir).map_err(error)?;
    let owner = std::env::var("CODEBUILD_BUILD_ID")
        .map_err(|_| "controller requires CODEBUILD_BUILD_ID; no local takeover identity")?;
    if !owner.starts_with("isolab-recovery-controller:") {
        return Err("unexpected controller project identity".into());
    }
    let identity = json(aws().args(["sts", "get-caller-identity"]))?;
    if identity["Account"] != ACCOUNT {
        return Err("wrong AWS account".into());
    }
    let mut stored = begin(req.clone(), &owner)?;
    loop {
        if stored.state.terminal() {
            publish(&stored, dir)?;
            return if stored.state.phase == Phase::Succeeded {
                Ok(())
            } else {
                Err(stored
                    .state
                    .failure
                    .clone()
                    .unwrap_or_else(|| "run failed".into()))
            };
        }
        // Check ownership at every reconciliation boundary. AWS terminal-owner check
        // plus CAS prevents two controllers advancing one logical run.
        stored = load(&req)?.ok_or("durable state disappeared")?;
        if stored.state.owner_build != owner {
            return Err("controller no longer owns this run".into());
        }
        if let Err(e) = tick(&mut stored, dir) {
            // Transport/API errors preserve the last durable state for a
            // CodeBuild infrastructure retry. They never grant a new epoch.
            if e.starts_with("command ") || e.starts_with("Kubernetes write failed:") {
                save(
                    &dir.join("controller-error.json"),
                    &json!({"error":e,"at":now()}),
                )?;
                return Err(e);
            }
            stored.state.fail(&e);
            stored.save()?;
            let _ = rpc(&stored.state, "stop", 0, None);
            save(&dir.join("failure.json"), &json!({"error":e,"at":now()}))?;
        }
        publish(&stored, dir)?;
        thread::sleep(Duration::from_secs(5));
    }
}

/// AWS-only ingress. The caller needs no network path to the private EKS API.
pub fn submit(req: Request, dir: &Path) -> Result<Value, String> {
    req.validate_new_deadline(now())?;
    fs::create_dir_all(dir).map_err(error)?;
    let caller = json(aws().args(["sts", "get-caller-identity"]))?;
    if caller["Account"] != ACCOUNT {
        return Err("wrong AWS account".into());
    }
    let bytes = serde_json::to_vec(&req).map_err(error)?;
    let path = dir.join("request.json");
    atomic(&path, &bytes)?;
    let key = format!("inputs/recovery-requests/{}.json", digest(&bytes));
    output(
        aws()
            .args([
                "s3api",
                "put-object",
                "--bucket",
                BUCKET,
                "--key",
                &key,
                "--body",
            ])
            .arg(&path)
            .args(["--server-side-encryption", "AES256"]),
    )?;
    let input = json!({"projectName":"isolab-recovery-controller","idempotencyToken":req.hash(),"environmentVariablesOverride":[{"name":"ISOLAB_RECOVERY_REQUEST_KEY","value":key,"type":"PLAINTEXT"}]});
    save(&dir.join("start-request.json"), &input)?;
    // AWS's token covers transport retries; the durable controller additionally
    // binds run_name forever, including retries after the token window expires.
    let started = json(
        aws()
            .args(["codebuild", "start-build", "--cli-input-json"])
            .arg(format!(
                "file://{}",
                dir.join("start-request.json").display()
            )),
    )?;
    save(&dir.join("build-start.json"), &started)?;
    let receipt = json!({"schema":"isolab.eks.recovery-submission/v1","run_name":req.run_name,"request_sha256":req.hash(),"controller_build":started["build"]["id"],"state_uri":format!("s3://{BUCKET}/{}state.json",prefix(&req)),"deadline_unix_seconds":req.deadline_unix_seconds});
    save(&dir.join("submission.json"), &receipt)?;
    Ok(receipt)
}
pub fn remote_status(req: &Request, dir: &Path) -> Result<Value, String> {
    fs::create_dir_all(dir).map_err(error)?;
    let path = dir.join("remote-state.json");
    let mut command = aws();
    command
        .args([
            "s3api",
            "get-object",
            "--bucket",
            BUCKET,
            "--key",
            &format!("{}state.json", prefix(req)),
        ])
        .arg(&path);
    let out = command.output().map_err(error)?;
    if !out.status.success() {
        let e = String::from_utf8_lossy(&out.stderr);
        if e.contains("NoSuchKey") || e.contains("(404)") {
            return Ok(
                json!({"schema":"isolab.eks.recovery-status/v1","run_name":req.run_name,"request_sha256":req.hash(),"phase":"awaiting_controller_receipt"}),
            );
        }
        return Err(e.into());
    }
    let state: State = read(&path)?;
    state.bind_request(req)?;
    serde_json::to_value(state).map_err(error)
}
pub fn wait(req: Request, dir: &Path) -> Result<Value, String> {
    loop {
        let status = remote_status(&req, dir)?;
        match status["phase"].as_str() {
            Some("succeeded") => {
                let key = format!("{}benchmark-result.json", prefix(&req));
                let path = dir.join("benchmark-result.json");
                output(
                    aws()
                        .args(["s3api", "get-object", "--bucket", BUCKET, "--key", &key])
                        .arg(&path),
                )?;
                let benchmark: Value = read(&path)?;
                let key = format!("{}computation-result.json", prefix(&req));
                let path = dir.join("computation-result.json");
                output(
                    aws()
                        .args(["s3api", "get-object", "--bucket", BUCKET, "--key", &key])
                        .arg(&path),
                )?;
                let computation: Value = read(&path)?;
                let result = json!({"schema":"isolab.eks.recovery-result/v1","state":status,"computation":computation,"benchmark":benchmark});
                save(&dir.join("result.json"), &result)?;
                return Ok(result);
            }
            Some("failed") => return Err(format!("recovery run failed: {}", status["failure"])),
            _ => {}
        }
        if now() > req.deadline_unix_seconds + 180 {
            return Err("original deadline plus receipt grace expired; inspect retained state, never resubmit as a fresh run".into());
        }
        thread::sleep(Duration::from_secs(15));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn memory_envelope_reaches_computation_and_fresh_l2_manifests() {
        // Isolate environment changes; submit-job is run ONLY in --dry-run mode.
        const CHILD: &str = "ISOLAB_MEMORY_MANIFEST_TEST_CHILD";
        if std::env::var_os(CHILD).is_none() {
            let status = Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "cloud::tests::memory_envelope_reaches_computation_and_fresh_l2_manifests",
                ])
                .env(CHILD, "1")
                .status()
                .unwrap();
            assert!(status.success());
            return;
        }
        use std::os::unix::fs::PermissionsExt;
        let dir =
            std::env::temp_dir().join(format!("isolab-memory-manifest-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let script = dir.join("submit-job");
        fs::write(&script, include_str!("../../../submit-job")).unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).unwrap();
        std::env::set_var("ISOLAB_DEPLOY_DIR", &dir);
        for memory in [crate::contract::Memory::Gi4, crate::contract::Memory::Gi8] {
            let mut req = crate::contract::fixture();
            req.memory = memory;
            let mut state = State::new(req, "test".into(), 1000).unwrap();
            state.benchmark_job = Some("memory-envelope-test".into());
            let pod = computation_pod(&state, 3600);
            let job = manifest_job(&state, 3600).unwrap();
            for resources in [
                &pod["spec"]["containers"][0]["resources"],
                &job["spec"]["template"]["spec"]["containers"][0]["resources"],
            ] {
                assert_eq!(resources["requests"]["memory"], memory.as_str());
                assert_eq!(resources["limits"]["memory"], memory.as_str());
                assert_eq!(resources["requests"]["cpu"], "4");
                assert_eq!(resources["limits"]["cpu"], "4");
            }
            assert_eq!(pod["spec"]["activeDeadlineSeconds"], 3600);
            assert_eq!(job["spec"]["activeDeadlineSeconds"], 3600);
            assert_eq!(job["spec"]["backoffLimit"], 0);
            assert_eq!(
                job["metadata"]["annotations"]["isolab.dev/recovery-request"],
                state.request_sha256
            );
            assert_eq!(
                job["spec"]["template"]["spec"]["nodeSelector"]["isolab.dev/pool"],
                "cpu-l2-spot"
            );
        }
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn checkpoint_readback_rejects_corruption_version_and_manifest_changes() {
        // Isolate PATH changes in a child test process; the parent test suite may
        // run other tests concurrently. No AWS credentials or network are used.
        const CHILD: &str = "ISOLAB_ARCHIVE_TEST_CHILD";
        if std::env::var_os(CHILD).is_none() {
            let status = Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "cloud::tests::checkpoint_readback_rejects_corruption_version_and_manifest_changes"])
                .env(CHILD, "1")
                .status().unwrap();
            assert!(status.success());
            return;
        }
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("isolab-archive-test-{}", std::process::id()));
        fs::create_dir_all(dir.join("generations/g1")).unwrap();
        let manifest = b"{\"committed\":true}\n";
        fs::write(dir.join("generations/g1/manifest.json"), manifest).unwrap();
        let archive = dir.join("source.tar");
        assert!(Command::new("tar")
            .arg("-cf")
            .arg(&archive)
            .arg("-C")
            .arg(&dir)
            .arg("generations")
            .status()
            .unwrap()
            .success());
        fs::write(dir.join("aws"), b"#!/bin/bash\nset -eu\ncp \"$ISOLAB_TEST_ARCHIVE\" \"${@: -1}\"\nprintf '%s\\n' '{\"VersionId\":\"v1\",\"ServerSideEncryption\":\"AES256\"}'\n").unwrap();
        fs::set_permissions(dir.join("aws"), fs::Permissions::from_mode(0o755)).unwrap();
        std::env::set_var(
            "PATH",
            format!("{}:{}", dir.display(), std::env::var("PATH").unwrap()),
        );
        std::env::set_var("ISOLAB_TEST_ARCHIVE", &archive);
        let (sha, bytes) = sha_file(&archive).unwrap();
        let good = Generation {
            number: 1,
            request_sha256: "a".repeat(64),
            snapshot_name: "snapshot".into(),
            snapshot_id: "snap-test".into(),
            archive_key: "test/checkpoint.tar".into(),
            archive_version: "v1".into(),
            archive_sha256: sha,
            archive_bytes: bytes,
            manifest_sha256: digest(manifest),
            source_instance: "i-old".into(),
            source_pod_uid: "pod-old".into(),
            committed_unix_seconds: 1000,
            snapshot_ready: true,
            encrypted: true,
            readback_verified: true,
        };
        verify_archive(&good, &dir).unwrap();
        let mut wrong_version = good.clone();
        wrong_version.archive_version = "v2".into();
        assert!(verify_archive(&wrong_version, &dir).is_err());
        let mut wrong_manifest = good.clone();
        wrong_manifest.manifest_sha256 = "f".repeat(64);
        assert!(verify_archive(&wrong_manifest, &dir).is_err());
        let original = fs::read(&archive).unwrap();
        let mut corrupt = original.clone();
        corrupt[100] ^= 1;
        fs::write(&archive, corrupt).unwrap();
        assert!(verify_archive(&good, &dir).is_err());
        fs::write(&archive, &original[..original.len() / 2]).unwrap();
        assert!(verify_archive(&good, &dir).is_err());
        // An intact older committed archive remains eligible after rejecting
        // the failed readbacks; no generation is relabelled or repaired.
        fs::write(&archive, original).unwrap();
        verify_archive(&good, &dir).unwrap();
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn runtime_class_requires_all_fields_and_exact_container_runtime() {
        let good = json!({"status":{"nodeInfo":{"kernelVersion":"kernel1","containerRuntimeVersion":"containerd://2.3.5","osImage":"AL2023","architecture":"amd64"}}});
        let class = node_runtime(&good).unwrap();
        let mut other = good.clone();
        other["status"]["nodeInfo"]["containerRuntimeVersion"] = json!("containerd://2.4.0");
        assert_ne!(node_runtime(&other).unwrap(), class);
        other = good;
        other["status"]["nodeInfo"]
            .as_object_mut()
            .unwrap()
            .remove("kernelVersion");
        assert!(node_runtime(&other).is_err());
    }
}

#[cfg(test)]
mod owner_exit_collection_tests {
    use super::*;
    #[test]
    fn owner_binding_is_not_closure_and_rejects_wrong_attempt_or_duplicate() {
        let value = json!({"schema":"isolab.recovery.owner-exit/v1","request_sha256":"a".repeat(64),
            "attempt":2,"reap_observed":false,"accepted_external_closure":null,"total_active_worker_ns":null});
        let bytes = serde_json::to_vec(&value).unwrap();
        assert!(check_owner_binding(&bytes, &"a".repeat(64), 2).is_ok());
        assert!(check_owner_binding(&bytes, &"a".repeat(64), 3).is_err());
        let duplicate =
            String::from_utf8(bytes.clone())
                .unwrap()
                .replacen('{', "{\"attempt\":2,", 1);
        assert!(check_owner_binding(duplicate.as_bytes(), &"a".repeat(64), 2).is_err());
        let mut bad = value;
        bad["accepted_external_closure"] = json!({"claimed":true});
        assert!(
            check_owner_binding(&serde_json::to_vec(&bad).unwrap(), &"a".repeat(64), 2).is_err()
        );
    }
    #[test]
    fn owner_capture_is_bounded_and_drains_without_unbounded_storage() {
        let bytes = vec![b'x'; OWNER_RECEIPT_CAP * 2];
        let observed =
            bounded_owner_stream(std::io::Cursor::new(bytes), OWNER_RECEIPT_CAP).unwrap();
        assert_eq!(observed.len(), OWNER_RECEIPT_CAP + 1);
        assert!(check_owner_binding(&observed, "unused", 0).is_err());
    }
    #[test]
    fn owner_publication_preserves_exact_bytes_and_conflicts() {
        let dir =
            std::env::temp_dir().join(format!("owner-collection-fixture-{}", std::process::id()));
        fs::create_dir(&dir).unwrap();
        let path = dir.join("receipt.json");
        let bytes = br#"{ "exact" : true }"#;
        retain_owner_bytes(&path, bytes).unwrap();
        retain_owner_bytes(&path, bytes).unwrap();
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert!(retain_owner_bytes(&path, b"different").is_err());
        assert_eq!(fs::read(&path).unwrap(), bytes);
        fs::remove_dir_all(dir).unwrap();
    }
}

/// Bounded evidence-only collection; never launches verification, replay or another Job.
/// Missing files/identity/remaining time are findings, not fabricated completed observations.
pub(crate) fn collect_fresh_owner_evidence(
    s: &State,
    status: &Value,
    dir: &Path,
) -> Result<Value, String> {
    let mut findings = Vec::<String>::new();
    let mut objects = Vec::<Value>::new();
    let job = s.benchmark_job.as_deref().ok_or("fresh job name missing")?;
    let pods = status["pods"].as_array();
    if pods.is_none_or(|p| p.is_empty()) {
        findings.push("missing existing Pod identities".into());
    }
    if pods.is_some_and(|p| p.len() > 16) {
        return Err("fresh owner pod collection cap".into());
    }
    let custody = dir.to_path_buf();
    fs::create_dir_all(&custody).map_err(error)?;
    for pod in pods.into_iter().flatten() {
        let Some(uid) = pod["uid"].as_str().filter(|v| {
            !v.is_empty()
                && v.len() <= 128
                && v.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        }) else {
            findings.push("invalid Pod identity".into());
            continue;
        };
        for name in [
            "owner-run-config.json",
            "owner-run.json",
            "owner-replay-config.json",
            "owner-replay.json",
        ] {
            // Existing aws() permits180 seconds plus5 kill grace. Reserve that full bound
            // before every call; campaign/root deadline is never extended for collection.
            if s.request
                .worker_campaign
                .as_ref()
                .map_or(s.request.deadline_unix_seconds, |v| {
                    v.policy.absolute_deadline_unix_seconds
                })
                .saturating_sub(now())
                <= 185
            {
                findings.push(format!(
                    "collection deadline reserve exhausted: {uid}/{name}"
                ));
                continue;
            }
            let cap = if name.ends_with("config.json") {
                65536
            } else {
                262144
            };
            let tmp = custody.join(format!("{uid}-{name}.download-{}", std::process::id()));
            if tmp.exists() {
                findings.push(format!("retained ambiguous download exists: {uid}/{name}"));
                continue;
            }
            let key = format!("runs/{job}/{uid}/{name}");
            let out = output(
                aws()
                    .args([
                        "s3api",
                        "get-object",
                        "--bucket",
                        BUCKET,
                        "--key",
                        &key,
                        "--range",
                        &format!("bytes=0-{cap}"),
                    ])
                    .arg(&tmp),
            );
            let meta = match out {
                Ok(v) => v,
                Err(e) => {
                    findings.push(format!("{uid}/{name}: {e}"));
                    continue;
                }
            };
            let bytes = fs::read(&tmp).map_err(error)?;
            if bytes.len() > cap {
                findings.push(format!("object cap: {uid}/{name}"));
                continue;
            }
            let sha = digest(&bytes);
            let retained = custody.join(format!("fresh-owner-{uid}-{name}-{sha}.json"));
            if retained.exists() {
                if fs::read(&retained).map_err(error)? != bytes {
                    return Err("custody digest collision".into());
                }
            } else {
                fs::hard_link(&tmp, &retained).map_err(error)?;
            }
            File::open(&retained)
                .and_then(|v| v.sync_all())
                .map_err(error)?;
            fs::remove_file(&tmp).map_err(error)?;
            objects.push(json!({"pod_uid":uid,"job_uid":s.benchmark_uid,"request_sha256":s.request_sha256,"image_uri":s.request.build.image_uri,"key":key,"bytes":bytes.len(),"sha256":sha,"retained_file":retained.file_name().and_then(|v|v.to_str()),"get_metadata":serde_json::from_slice::<Value>(&meta.stdout).ok()}));
        }
    }
    File::open(&custody)
        .and_then(|v| v.sync_all())
        .map_err(error)?;
    let result = json!({"schema":"isolab.fresh-owner-custody/v1","request_sha256":s.request_sha256,"job_uid":s.benchmark_uid,"objects":objects,"findings":findings,"owner_receipts_authenticated":false,"accepted_external_closure":null,"total_active_worker_ns":null,"scope":"exact bounded bytes collected from expected S3 keys; metadata is not semantic acceptance"});
    let raw = serde_json::to_vec_pretty(&result).map_err(error)?;
    retain_owner_bytes(
        &dir.join(format!("fresh-owner-custody-{}.json", digest(&raw))),
        &raw,
    )?;
    Ok(result)
}

#[cfg(test)]
mod campaign_cas_tests {
    use super::*;
    #[test]
    fn expired_campaign_retains_terminal_evidence_without_admitting_work() {
        let reservation = crate::campaign::Reservation {
            request_sha256: "a".repeat(64),
            attempt: 0,
            epoch: crate::campaign::Epoch::Computation,
            policy_sha256: "b".repeat(64),
            admitted_unix_seconds: 100,
            deadline_unix_seconds: 200,
            ceiling_seconds: 160,
            shutdown_reserve_seconds: 60,
        };
        let expiry = reservation.remaining_payload_seconds(200).unwrap_err();
        assert!(collection_call_fits(200, 15).is_ok());
        assert!(collection_call_fits(200, 16).is_err());
        assert!(collection_call_fits(200, 200).is_err());
        let retained = std::cell::Cell::new(false);
        let result: Result<(), String> = retain_then_reject(expiry.clone(), || {
            retained.set(true);
            Ok(())
        });
        assert!(retained.get());
        assert_eq!(result.unwrap_err(), expiry);
        let result: Result<(), String> =
            retain_then_reject("expired grant".into(), || Err("receipt failure".into()));
        let message = result.unwrap_err();
        assert!(message.contains("expired grant") && message.contains("receipt failure"));
    }
    #[test]
    fn campaign_commit_requires_new_revision_same_uid_and_exact_ledger() {
        let input = json!({"metadata":{"uid":"u","resourceVersion":"41"},"data":{"ledger.json":"exact bytes"}});
        let mut committed = input.clone();
        assert!(validate_campaign_cas_response(&input, &committed).is_err());
        committed["metadata"]["resourceVersion"] = json!("42");
        assert!(validate_campaign_cas_response(&input, &committed).is_ok());
        committed["metadata"]["uid"] = json!("replacement");
        assert!(validate_campaign_cas_response(&input, &committed).is_err());
        committed["metadata"]["uid"] = json!("u");
        committed["data"]["ledger.json"] = json!("changed bytes");
        assert!(validate_campaign_cas_response(&input, &committed).is_err());
    }
}

#[cfg(test)]
mod computation_owner_tests {
    use super::*;
    #[test]
    fn matching_annotation_cannot_hide_wrong_namespace_or_owner_transport() {
        let v = json!({"metadata":{"annotations":{"isolab.dev/worker-reservation":"a"}},"spec":{"hostPID":false,"shareProcessNamespace":false,"containers":[{"name":"supervisor","image":"pinned","command":["owner"],"env":[{"name":"ISOLAB_COMPUTATION_RESERVATION","value":"exact"}],"volumeMounts":[],"securityContext":{}}],"volumes":[]}});
        computation_owner_matches(&v, &v).unwrap();
        let mut selector = v.clone();
        selector["spec"]["containers"][0]["env"][0] = json!({"name":"BINDING","valueFrom":{"fieldRef":{"fieldPath":"metadata.annotations['isolab.dev/worker-reservation']"}}});
        let mut api_defaulted = selector.clone();
        api_defaulted["spec"]["containers"][0]["env"][0]["valueFrom"]["fieldRef"]["apiVersion"] =
            json!("v1");
        computation_owner_matches(&api_defaulted, &selector).unwrap();
        api_defaulted["spec"]["containers"][0]["env"][0]["valueFrom"]["fieldRef"]["fieldPath"] =
            json!("metadata.name");
        assert!(computation_owner_matches(&api_defaulted, &selector).is_err());

        for key in ["initContainers", "ephemeralContainers"] {
            let mut wrong = v.clone();
            wrong["spec"][key] = json!([{"name":"injected","targetContainerName":"supervisor"}]);
            assert!(computation_owner_matches(&wrong, &v).is_err());
            wrong["spec"][key] = json!([]);
            computation_owner_matches(&wrong, &v).unwrap();
        }

        for key in ["hostPID", "shareProcessNamespace"] {
            let mut wrong = v.clone();
            wrong["spec"][key] = json!(true);
            assert!(computation_owner_matches(&wrong, &v).is_err());
        }
        for key in ["command", "env", "image"] {
            let mut wrong = v.clone();
            wrong["spec"]["containers"][0][key] = Value::Null;
            assert!(computation_owner_matches(&wrong, &v).is_err());
        }
        let mut sidecar = v.clone();
        sidecar["spec"]["containers"]
            .as_array_mut()
            .unwrap()
            .push(json!({"name":"other"}));
        assert!(computation_owner_matches(&sidecar, &v).is_err());
    }
}

#[cfg(test)]
mod fieldref_default_tests {
    use super::*;
    #[test]
    fn only_omitted_fieldref_api_version_defaults_to_v1() {
        let expected = json!([{"name":"BINDING","valueFrom":{"fieldRef":{"fieldPath":"metadata.annotations['isolab.dev/worker-reservation']"}}}]);
        let mut defaulted = expected.clone();
        defaulted[0]["valueFrom"]["fieldRef"]["apiVersion"] = json!("v1");
        assert_eq!(
            env_with_fieldref_default(&expected),
            env_with_fieldref_default(&defaulted)
        );
        for wrong in [json!("v2"), Value::Null, json!("")] {
            let mut value = defaulted.clone();
            value[0]["valueFrom"]["fieldRef"]["apiVersion"] = wrong;
            assert_ne!(
                env_with_fieldref_default(&expected),
                env_with_fieldref_default(&value)
            );
        }
        let mut wrong = defaulted.clone();
        wrong[0]["valueFrom"]["fieldRef"]["fieldPath"] = json!("metadata.name");
        assert_ne!(
            env_with_fieldref_default(&expected),
            env_with_fieldref_default(&wrong)
        );
        let mut extra = defaulted.clone();
        extra
            .as_array_mut()
            .unwrap()
            .push(json!({"name":"OTHER","value":"x"}));
        assert_ne!(
            env_with_fieldref_default(&expected),
            env_with_fieldref_default(&extra)
        );
    }
    #[test]
    fn fresh_api_roundtrip_preserves_exact_identity_comparison() {
        let mut intended = json!({"spec":{"template":{"spec":{
            "containers":[{"env":[
                {"name":"POD_UID","valueFrom":{"fieldRef":{"fieldPath":"metadata.uid"}}},
                {"name":"NODE_NAME","valueFrom":{"fieldRef":{"fieldPath":"spec.nodeName"}}},
                {"name":"ISOLAB_OWNER_JOB_UID","valueFrom":{"fieldRef":{"fieldPath":"metadata.labels['batch.kubernetes.io/controller-uid']"}}},
                {"name":"ISOLAB_OWNER_REQUEST_SHA256","value":"pinned-request"}
            ]}],
            "volumes":[{"name":"work","emptyDir":{"sizeLimit":"8Gi"}},
                {"name":"diagnostic-input","downwardAPI":{"defaultMode":292,"items":[
                    {"path":"fresh-launch.json","fieldRef":{"fieldPath":"metadata.annotations['isolab.dev/diagnostic-fresh-launch']"}}
                ]}}]
        }}}});
        let mut observed = intended.clone();
        // Explicit shape produced by ObjectFieldSelector API defaulting.
        for i in 0..3 {
            observed["spec"]["template"]["spec"]["containers"][0]["env"][i]["valueFrom"]
                ["fieldRef"]["apiVersion"] = json!("v1");
        }
        observed["spec"]["template"]["spec"]["volumes"][1]["downwardAPI"]["items"][0]["fieldRef"]
            ["apiVersion"] = json!("v1");
        assert_ne!(intended, observed);
        pin_fresh_selector_defaults(&mut intended);
        assert_eq!(intended, observed);
        let once = intended.clone();
        pin_fresh_selector_defaults(&mut intended);
        assert_eq!(intended, once);
        for pointer in [
            "/spec/template/spec/containers/0/env/0/valueFrom/fieldRef/fieldPath",
            "/spec/template/spec/containers/0/env/0/valueFrom/fieldRef/apiVersion",
            "/spec/template/spec/containers/0/env/3/value",
            "/spec/template/spec/volumes/1/downwardAPI/defaultMode",
            "/spec/template/spec/volumes/1/downwardAPI/items/0/fieldRef/fieldPath",
        ] {
            let mut changed = observed.clone();
            *changed.pointer_mut(pointer).unwrap() = json!("different");
            pin_fresh_selector_defaults(&mut changed);
            assert_ne!(changed, intended, "mutation must remain visible: {pointer}");
        }
        let mut extra = observed;
        extra["spec"]["template"]["spec"]["volumes"]
            .as_array_mut()
            .unwrap()
            .push(json!({"name":"injected","emptyDir":{}}));
        pin_fresh_selector_defaults(&mut extra);
        assert_ne!(extra, intended);
    }
    #[test]
    fn fresh_selector_defaults_preserve_resource_field_items() {
        let mut manifest = json!({"spec":{"template":{"spec":{
            "containers":[{"env":[{"name":"CPU_LIMIT","valueFrom":{"resourceFieldRef":{"resource":"limits.cpu","divisor":"1"}}}]}],
            "volumes":[{"name":"resources","downwardAPI":{"items":[
                {"path":"cpu","resourceFieldRef":{"containerName":"task","resource":"limits.cpu","divisor":"1"}}
            ]}}]
        }}}});
        let original = manifest.clone();
        pin_fresh_selector_defaults(&mut manifest);
        assert_eq!(manifest, original);
        assert!(
            manifest["spec"]["template"]["spec"]["volumes"][0]["downwardAPI"]["items"][0]
                .get("fieldRef")
                .is_none()
        );
    }
}
