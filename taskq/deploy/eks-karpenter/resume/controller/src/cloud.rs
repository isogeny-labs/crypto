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
fn provision(s: &State) -> Result<Value, String> {
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
    ensure(
        NAMESPACE,
        &json!({"apiVersion":"v1","kind":"Pod","metadata":metadata(&name,req),"spec":{"restartPolicy":"Never","activeDeadlineSeconds":remaining,"automountServiceAccountToken":false,"enableServiceLinks":false,"nodeSelector":{"isolab.dev/pool":"cpu-spot","node.kubernetes.io/instance-type":req.instance_type},"tolerations":[{"key":"isolab.dev/workload","operator":"Equal","value":"cpu","effect":"NoSchedule"}],"containers":[{"name":"supervisor","image":req.runtime_image_uri,"command":[SUPERVISOR,"supervise","/request/request.json",s.attempt.to_string()],"securityContext":{"privileged":true,"runAsUser":0,"runAsGroup":0},"resources":{"requests":{"cpu":"4","memory":"4Gi"},"limits":{"cpu":"4","memory":"4Gi"}},"volumeMounts":[{"name":"work","mountPath":"/work"},{"name":"request","mountPath":"/request","readOnly":true}]}],"volumes":[{"name":"work","persistentVolumeClaim":{"claimName":name}},{"name":"request","configMap":{"name":format!("{}-request",req.run_name)}}]}}),
        req,
    )
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
        if st
            .generation
            .is_some_and(|n| s.generations.iter().any(|g| g.number == n))
        {
            s.phase = Phase::Running;
            stored.save()?;
            return Ok(());
        }
        rpc(s, "checkpoint", s.next_generation, None)?;
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
    if let Some(g) = s.generations.iter().find(|g| g.number == n) {
        rpc(s, "continue", n, Some(g.manifest_sha256.clone()))?;
        s.phase = Phase::Running;
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
        "4Gi",
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
        || audit["records"] != audit["verified_records"]
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
        json!({"job_name":job,"job_uid":s.benchmark_uid,"pod_uid":pod_uid,"results_uri":uri,"payload_hashes_verified":named.len(),"l2":gate,"native_audit":audit,"control":control}),
    )
}
fn benchmark(stored: &mut Stored, dir: &Path) -> Result<(), String> {
    if stored.state.benchmark_manifest.is_none() {
        let remaining = stored
            .state
            .request
            .deadline_unix_seconds
            .checked_sub(now() + 120)
            .filter(|x| *x >= 60)
            .ok_or("insufficient original deadline for fresh epoch")?;
        stored.state.benchmark_manifest = Some(manifest_job(&stored.state, remaining)?);
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
    if current["spec"]["template"]["spec"]["containers"][0]["command"]
        != intended["spec"]["template"]["spec"]["containers"][0]["command"]
        || current["spec"]["template"]["spec"]["containers"][0]["image"]
            != s.request.build.image_uri
    {
        return Err("existing benchmark differs from immutable intent".into());
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
        provision(&stored.state)?
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
    if st.phase == "failed" {
        return Err(st
            .error
            .unwrap_or_else(|| format!("computation exited {:?}", st.exit_code)));
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
                stored.state.phase = Phase::Checkpointing;
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
