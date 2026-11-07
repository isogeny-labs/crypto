//! Read-only S3 checks (ports of s3check.py, s3requests.py and c2_identity.py).
//! Like the Python, these shell out to the `aws` CLI (no SDK); they list and
//! hash remote objects and compare them with the repository copies. Nothing is
//! written to S3.

use crate::j2::{frozen_spec_name, ECB, REV};
use crate::py::{self, py_eq, split_ws, splitlines, Obj};
use crate::v;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

pub const BUCKET: &str = "s3://isolab-results-104186811093-us-east-2/runs";
pub const JOBS: [(u32, &str, &str); 6] = [
    (24, "wall", "ic-dgbf-p24-20261029"),
    (24, "counted", "ic-dgbf-rho-p24-20261029"),
    (32, "wall", "ic-dgbf-p32-20261029"),
    (32, "counted", "ic-dgbf-rho-p32-20261029"),
    (28, "counted", "ic-dgbf-rho-p28-20261029-r2"),
    (28, "wall", "ic-dgbf-p28-20261029"),
];
const IMAGE: &str = "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b";

pub fn is_uuid(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    let lens = [8, 4, 4, 4, 12];
    parts.len() == 5
        && parts.iter().zip(lens).all(|(p, n)| p.len() == n && p.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)))
}

/// `aws ...`: stdout; an empty-prefix `aws s3 ls` (exit 1, no output) reads as "".
fn aws(args: &[&str]) -> Vec<u8> {
    let o = Command::new("aws").args(args).output().unwrap_or_else(|e| v::stop(format!("aws: {e}")));
    let code = o.status.code().unwrap_or(-1);
    if code == 1 && args.len() >= 2 && args[0] == "s3" && args[1] == "ls" && o.stdout.is_empty() && String::from_utf8_lossy(&o.stderr).trim().is_empty() {
        return Vec::new();
    }
    if code != 0 {
        v::stop(format!("aws {}: {}", args.join(" "), String::from_utf8_lossy(&o.stderr).chars().take(400).collect::<String>()));
    }
    o.stdout
}

fn s3_sha(uri: &str) -> Option<String> {
    let mut ch = Command::new("aws").args(["s3", "cp", uri, "-"]).stdout(Stdio::piped()).spawn().ok()?;
    let mut out = ch.stdout.take().unwrap();
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = out.read(&mut buf).ok()?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    if !ch.wait().ok()?.success() {
        return None;
    }
    Some(v::hex(&h.finalize()))
}

/// Keys of a recursive listing, relative to `runs/{job}/`.
fn rel_keys(listing: &str, job: &str) -> Vec<String> {
    let pre = format!("runs/{job}/");
    splitlines(listing)
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| split_ws(l, 3)[3].chars().skip(pre.chars().count()).collect())
        .collect()
}

fn pods_of(rel: &[String]) -> Vec<String> {
    rel.iter().map(|r| r.split('/').next().unwrap().to_string()).filter(|x| is_uuid(x)).collect::<BTreeSet<_>>().into_iter().collect()
}

fn sums_names(eks: &Path) -> Vec<String> {
    v::sums_lines(eks).into_iter().map(|(_, n)| n).collect()
}

pub fn run_s3check(out: &Path) {
    let mut o = Obj::new();
    o.set("bucket", BUCKET);
    let lst = String::from_utf8(aws(&["s3", "ls", "--recursive", &format!("{BUCKET}/ic-dgbf-rho-p28-20261029/")])).unwrap();
    let orig = splitlines(&lst).iter().filter(|l| !l.trim().is_empty()).count();
    let mut jobs = Obj::new();
    for (b, kind, job) in JOBS {
        let eks = v::session_dir(b, kind);
        let je = v::jload(&eks.join("job-exit.json"));
        let lst = String::from_utf8(aws(&["s3", "ls", "--recursive", &format!("{BUCKET}/{job}/")])).unwrap();
        let rel = rel_keys(&lst, job);
        let pods = pods_of(&rel);
        let launchers: Vec<String> = rel
            .iter()
            .filter(|r| r.starts_with("launcher/"))
            .map(|r| r.split('/').nth(1).unwrap().to_string())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let gj = |k: &str| je.get(k).cloned().unwrap_or(Value::Null);
        let mut rec = Obj::new();
        rec.set("job", job)
            .set("objects", rel.len() as u64)
            .set("pod_prefixes", pods.clone())
            .set("job_exit_pod_uid", gj("pod_uid"))
            .set("job_exit_code", gj("exit_code"))
            .set("job_exit_job_name", gj("job_name"))
            .set("one_pod_prefix_equal_to_job_exit", pods.len() == 1 && py_eq(&Value::from(pods[0].clone()), &gj("pod_uid")))
            .set("launchers", launchers.clone());
        let mut files = Map::new();
        let mut all_eq = None;
        if pods.len() == 1 {
            let mut names = sums_names(&eks);
            names.push("SHA256SUMS".into());
            let mut all = true;
            for name in names {
                let local = v::sha256_file(&eks.join(&name));
                let remote = s3_sha(&format!("{BUCKET}/{job}/{}/{name}", pods[0]));
                let equal = remote.as_deref() == Some(local.as_str());
                all &= equal;
                files.insert(name, serde_json::json!({"local": local, "s3": remote, "equal": equal}));
            }
            all_eq = Some(all);
        }
        rec.set("files", Value::Object(files));
        let mut lres = Vec::new();
        for lid in &launchers {
            if rel.iter().any(|r| r == &format!("launcher/{lid}/result.json")) {
                let r: Value = serde_json::from_slice(&aws(&["s3", "cp", &format!("{BUCKET}/{job}/launcher/{lid}/result.json"), "-"])).unwrap();
                let gr = |k: &str| r.get(k).cloned().unwrap_or(Value::Null);
                lres.push(serde_json::json!({"launcher": lid, "exit_code": gr("exit_code"), "state": gr("state"),
                    "build": gr("build"), "pod_uid": gr("pod_uid")}));
            }
        }
        rec.set("launcher_results", lres);
        if let Some(a) = all_eq {
            rec.set("all_files_equal", a);
        }
        println!("{job} {} {:?} {:?}", rel.len(), pods, all_eq);
        jobs.set(format!("P{b}-{kind}"), rec.val());
    }
    o.set("jobs", jobs.val());
    o.set("original_p28_counted_prefix_objects", orig as u64);
    py::dump(&o.val(), out);
}

fn check_request(req: &Value, frozen: &Value) -> Value {
    let j = &req["job"];
    let mut o = Obj::new();
    o.set("spec_json_equal_frozen", py_eq(&j["spec"], frozen))
        .set("image_uri", j["build"]["image_uri"].as_str() == Some(IMAGE))
        .set("source_revision", j["build"]["source_revision"].as_str() == Some(REV))
        .set("ecbench_sha256", j["build"]["ecbench_sha256"].as_str() == Some(ECB))
        .set("instance_type", j.get("instance_type").cloned().unwrap_or(Value::Null))
        .set("seconds", j.get("seconds").cloned().unwrap_or(Value::Null))
        .set("harness", req["provenance"]["harness"].clone())
        .set("job_name", j["job_name"].clone());
    o.val()
}

/// s3requests.py: each launcher's request.json must embed the frozen spec and
/// name the image digest, revision and ecbench sha256 of PROTOCOL section 4.
pub fn run_s3requests(s3check: &Path, out: &Path) {
    let s3 = v::jload(s3check);
    let mut o = Obj::new();
    for (b, kind, job) in JOBS {
        let frozen = v::jload(&v::backlog().join(frozen_spec_name(b, kind)));
        let mut rec = Obj::new();
        for lid in s3["jobs"][format!("P{b}-{kind}")]["launchers"].as_array().unwrap() {
            let lid = lid.as_str().unwrap();
            let p = Command::new("aws")
                .args(["s3", "cp", &format!("{BUCKET}/{job}/launcher/{lid}/request.json"), "-"])
                .stderr(Stdio::null())
                .output()
                .unwrap_or_else(|e| v::stop(format!("aws: {e}")));
            let val = if p.status.success() {
                check_request(&serde_json::from_slice(&p.stdout).unwrap(), &frozen)
            } else {
                serde_json::json!({"error": "absent"})
            };
            rec.set(lid, val);
        }
        if b == 28 && kind == "counted" {
            let local = v::jload(&v::root().join("P28").join("receipts-P28-rho-r2").join("request.json"));
            rec.set("local:P28/receipts-P28-rho-r2/request.json", check_request(&local, &frozen));
        }
        println!("{job} {}", rec.0.len());
        o.set(format!("P{b}-{kind}"), rec.val());
    }
    py::dump(&o.val(), out);
}

/// c2_identity.py (RULING-P28 C6): the C2 identity facts for every job.
pub fn run_c2(out: &Path) {
    let mut o = Obj::new();
    for (b, kind, job) in JOBS {
        let eks = v::session_dir(b, kind);
        let p = Command::new("aws")
            .args(["s3", "ls", "--recursive", &format!("{BUCKET}/{job}/")])
            .output()
            .unwrap_or_else(|e| v::stop(format!("aws: {e}")));
        let rel = rel_keys(&String::from_utf8(p.stdout).unwrap(), job);
        let pods = pods_of(&rel);
        let mut pod_files: Vec<String> = if pods.is_empty() {
            Vec::new()
        } else {
            rel.iter()
                .filter(|r| r.starts_with(&format!("{}/", pods[0])))
                .map(|r| r.split_once('/').unwrap().1.to_string())
                .collect()
        };
        pod_files.sort();
        let mut listed = sums_names(&eks);
        listed.sort();
        let je = v::jload(&eks.join("job-exit.json"));
        let bi = v::jload(&eks.join("build-info.json"));
        let frozen = v::jload(&v::backlog().join(frozen_spec_name(b, kind)));
        let (ok_sums, _, _) = v::check_sums(&eks);
        let binsha = split_ws(&v::read_text(&eks.join("binary.sha256")), -1)[0].clone();
        let pod_count = pods.len();
        let present = pod_files.iter().any(|x| x == "job-exit.json");
        let identity = je["job_name"].as_str() == Some(job) && pods.len() == 1 && py_eq(&Value::from(pods[0].clone()), &je["pod_uid"]);
        let manifest = pod_files.iter().filter(|x| *x != "SHA256SUMS").cloned().collect::<Vec<_>>() == listed;
        let build = bi["source_revision"].as_str() == Some(REV) && bi["ecbench_sha256"].as_str() == Some(ECB);
        let bsha = binsha == ECB;
        let ispec = py_eq(&v::jload(&eks.join("input-spec.json")), &frozen);
        let mut f = Obj::new();
        f.set("POD_UID_COUNT", pod_count as u64)
            .set("JOB_EXIT_PRESENT", present)
            .set("JOB_EXIT_IDENTITY", identity)
            .set("EXIT_CODE", je["exit_code"].clone())
            .set("SHA256SUMS_OK", ok_sums)
            .set("MANIFEST_COMPLETE", manifest)
            .set("BUILD_INFO", build)
            .set("BINARY_SHA", bsha)
            .set("INPUT_SPEC", ispec)
            .set("IMAGE_DIGEST", "unverifiable_without_pod_status")
            .set("S3_TERMINAL", pod_count == 1 && present && identity && ok_sums && manifest && build && bsha && ispec);
        println!("{job} S3_TERMINAL {}", f.0["S3_TERMINAL"]);
        o.set(format!("P{b}-{kind}"), f.val());
    }
    py::dump(&o.val(), out);
}
