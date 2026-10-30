//! Read-only intake; no solver, replay, cloud access or receipt fabrication.
//! Expected custody SHA/Pod/request/binary must come from independently
//! authenticated controller evidence, never copied from an untrusted bundle.
use crypto_lib::cryptanalysis::ecbench::{
    canonical,
    diagnostic_resources::{FreshOwnerPins, SessionResourceJournal},
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs::File, io::Read, path::Path};
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn read(path: &str, cap: usize) -> Result<Vec<u8>, String> {
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file: File = options.open(Path::new(path)).map_err(|e| e.to_string())?;
    if !file.metadata().map_err(|e| e.to_string())?.is_file() {
        return Err("intake requires regular files".into());
    }
    let mut bytes = Vec::new();
    file.take(cap as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > cap {
        return Err("intake byte cap".into());
    }
    Ok(bytes)
}
fn custody(
    bytes: &[u8],
    expected_hash: &str,
    owner: &[u8],
    request: &str,
    pod: &str,
) -> Result<Value, String> {
    if hash(bytes) != expected_hash {
        return Err("externally pinned custody digest mismatch".into());
    }
    canonical::reject_duplicate_keys(std::str::from_utf8(bytes).map_err(|e| e.to_string())?)?;
    let v: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    if v["schema"] != "isolab.recovery.owner-exit-collection/v1"
        || v["collection_status"] != "fetched_binding_checked_only"
        || v["request_sha256"] != request
        || v["attempt"].as_u64() != Some(0)
        || v["pod_uid"] != pod
        || v["exact_receipt_sha256"] != hash(owner)
        || pod.is_empty()
    {
        return Err("controller custody association mismatch".into());
    }
    Ok(v)
}
fn fresh_custody(
    bytes: &[u8],
    expected_hash: &str,
    owner: &[u8],
    pins: &FreshOwnerPins,
) -> Result<Value, String> {
    if hash(bytes) != expected_hash {
        return Err("externally pinned fresh custody digest mismatch".into());
    }
    canonical::reject_duplicate_keys(std::str::from_utf8(bytes).map_err(|e| e.to_string())?)?;
    let v: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    let runs: Vec<_> = v["records"]
        .as_array()
        .ok_or("fresh custody records absent")?
        .iter()
        .filter(|r| r["phase"] == "run")
        .collect();
    if v["schema"] != "isolab.fresh-owner-controller-check/v1"
        || v["request_sha256"] != pins.request_sha256
        || v["pod_uid"] != pins.pod_uid
        || v["job_uid"] != pins.job_uid
        || runs.len() != 1
        || runs[0]["receipt_sha256"] != hash(owner)
        || runs[0]["config_sha256"] != pins.config_sha256
        || v.get("accepted_external_closure") != Some(&Value::Null)
        || v.get("total_active_worker_ns") != Some(&Value::Null)
    {
        return Err("fresh controller custody association mismatch".into());
    }
    Ok(v)
}
fn run_fresh(args: &[String]) -> Result<Value, String> {
    if args.len() != 14 {
        return Err("usage: ecbench_owner_closure --fresh CUSTODY CUSTODY_SHA OWNER CONFIG CONFIG_SHA PREPARATION JOURNAL EXPECTED_REQUEST_SHA EXPECTED_BINARY_SHA EXPECTED_POD_UID EXPECTED_JOB_UID EXPECTED_IMAGE_SHA".into());
    }
    let c = read(&args[2], 256 * 1024)?;
    let o = read(&args[4], 256 * 1024)?;
    let config = read(&args[5], 65536)?;
    let p = read(&args[7], 256 * 1024)?;
    let j = read(&args[8], 8 * 1024 * 1024)?;
    let pins = FreshOwnerPins {
        config_sha256: args[6].clone(),
        request_sha256: args[9].clone(),
        binary_sha256: args[10].clone(),
        pod_uid: args[11].clone(),
        job_uid: args[12].clone(),
        image_sha256: args[13].clone(),
    };
    fresh_custody(&c, &args[3], &o, &pins)?;
    let journal: SessionResourceJournal = serde_json::from_slice(&j).map_err(|e| e.to_string())?;
    let closure = journal.close_fresh_enclosing_evidence(&j, &p, &o, &config, &pins)?;
    Ok(
        json!({"schema":"ecbench.owner-closure-intake/v1","status":"closed_fresh_enclosing_evidence","closure":closure,
        "trust_context":{"controller_collection_sha256":hash(&c),"pins":pins,"origin":"caller-supplied independently authenticated controller pins; this program verifies exact byte and semantic binding, not external authentication","owner_model":"trusted same-UID fresh instrumentation, not root-private"},
        "replay_session_closure":null,"mathematical_result":null,"timing_qualification":null,"worker_allocation_ns":null}),
    )
}
fn run(args: &[String]) -> Result<Value, String> {
    if args.get(1).is_some_and(|v| v == "--fresh") {
        return run_fresh(args);
    }
    if args.len() != 9 {
        return Err("usage: ecbench_owner_closure CUSTODY CUSTODY_SHA OWNER PREPARATION JOURNAL EXPECTED_REQUEST_SHA EXPECTED_BINARY_SHA EXPECTED_POD_UID".into());
    }
    let c = read(&args[1], 256 * 1024)?;
    let o = read(&args[3], 256 * 1024)?;
    let p = read(&args[4], 256 * 1024)?;
    let j = read(&args[5], 8 * 1024 * 1024)?;
    let _ = custody(&c, &args[2], &o, &args[6], &args[8])?;
    let journal: SessionResourceJournal = serde_json::from_slice(&j).map_err(|e| e.to_string())?;
    let closure = journal.close_enclosing_evidence(&j, &p, &o, &args[6], &args[7])?;
    Ok(
        json!({"schema":"ecbench.owner-closure-intake/v1","status":"closed_enclosing_evidence","closure":closure,
        "trust_context":{"controller_collection_sha256":hash(&c),"pod_uid":args[8],"request_sha256":args[6],"binary_sha256":args[7],
        "origin":"caller-supplied independently authenticated controller pins; this program verifies their byte/content binding, not external authentication"},
        "mathematical_result":null,"timing_qualification":null,"worker_allocation_ns":null}),
    )
}
fn main() {
    match run(&std::env::args().collect::<Vec<_>>()) {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).expect("JSON value")),
        Err(error) => {
            println!(
                "{}",
                json!({"schema":"ecbench.owner-closure-intake/v1","status":"unresolved","closure":null,"error":error})
            );
            std::process::exit(1);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn self_asserted_custody_is_not_an_external_pin() {
        let bytes = br#"{"schema":"isolab.recovery.owner-exit-collection/v1"}"#;
        assert!(
            custody(bytes, &"a".repeat(64), b"{}", &"b".repeat(64), "pod")
                .unwrap_err()
                .contains("pinned custody")
        );
    }
    #[test]
    fn known_custody_still_requires_exact_owner_and_pod() {
        let o = b"{}";
        let v = json!({"schema":"isolab.recovery.owner-exit-collection/v1","collection_status":"fetched_binding_checked_only","request_sha256":"a".repeat(64),"attempt":0,"pod_uid":"pod","exact_receipt_sha256":hash(o)});
        let bytes = serde_json::to_vec(&v).unwrap();
        let pin = hash(&bytes);
        assert!(custody(&bytes, &pin, o, &"a".repeat(64), "pod").is_ok());
        assert!(custody(&bytes, &pin, b"{ }", &"a".repeat(64), "pod").is_err());
        assert!(custody(&bytes, &pin, o, &"a".repeat(64), "other").is_err());
    }
    #[test]
    fn fresh_controller_pin_binds_exact_run_receipt_config_pod_and_job() {
        let owner = b"{}";
        let pins = FreshOwnerPins {
            config_sha256: "a".repeat(64),
            request_sha256: "b".repeat(64),
            binary_sha256: "c".repeat(64),
            image_sha256: "d".repeat(64),
            pod_uid: "pod".into(),
            job_uid: "job".into(),
        };
        let custody_value = json!({"schema":"isolab.fresh-owner-controller-check/v1","request_sha256":pins.request_sha256,"pod_uid":"pod","job_uid":"job","records":[{"phase":"run","receipt_sha256":hash(owner),"config_sha256":pins.config_sha256}],"accepted_external_closure":null,"total_active_worker_ns":null});
        let bytes = serde_json::to_vec(&custody_value).unwrap();
        let digest = hash(&bytes);
        assert!(fresh_custody(&bytes, &digest, owner, &pins).is_ok());
        assert!(fresh_custody(&bytes, &"0".repeat(64), owner, &pins).is_err());
        assert!(fresh_custody(&bytes, &digest, b"{ }", &pins).is_err());
        let mut wrong = pins.clone();
        wrong.job_uid = "different".into();
        assert!(fresh_custody(&bytes, &digest, owner, &wrong).is_err());
        let mut duplicate = custody_value.clone();
        let record = duplicate["records"][0].clone();
        duplicate["records"].as_array_mut().unwrap().push(record);
        let db = serde_json::to_vec(&duplicate).unwrap();
        assert!(fresh_custody(&db, &hash(&db), owner, &pins).is_err());
    }
    #[test]
    fn fresh_cli_dispatch_closes_only_pinned_run_files() {
        let dir = std::env::temp_dir().join(format!("fresh-full-closure-{}", std::process::id()));
        std::fs::create_dir(&dir).unwrap();
        let p = json!({"schema":"ecbench.session_preparation/v1","session_id":"s","runner_id":"r","clock_type":"linux_CLOCK_MONOTONIC","origin_ns":20,"boot_id":"b","time_namespace":"n","time_namespace_offsets":"o"});
        let pb = serde_json::to_vec(&p).unwrap();
        let ph = hash(&pb);
        let cid = format!("r-preparation-sha256-{ph}");
        let j = SessionResourceJournal::begin("s".into(), "r".into(), cid.clone(), 0).unwrap();
        let jb = serde_json::to_vec(&j).unwrap();
        let config = json!({"schema":"isolab.fresh-owner-config/v1","phase":"run","request_sha256":"a".repeat(64),"binary_sha256":"b".repeat(64),"image_digest":"c".repeat(64),"pod_uid":"pod","job_uid":"job"});
        let cb = serde_json::to_vec(&config).unwrap();
        let clock = |raw| json!({"raw_ns":raw,"boot_id":"b","time_namespace":"n","time_namespace_offsets":"o"});
        let o = json!({"schema":"isolab.fresh-owner-observation/v1","config_sha256":hash(&cb),"binding":config,"reap_observed":true,"exit_code":0,"findings":[],"accepted_external_closure":null,"total_active_worker_ns":null,"exact_kernel_exit_ns":null,"payload_pid":12,"owner_begin_clock":clock(10),"postexec_runner_observation":{"pid":12,"starttime":"17","binary_sha256":"b".repeat(64),"clock":clock(25)},"reap_observed_clock":clock(30),"session_binding":{"session_id":"s","runner_id":"r","clock_id":cid,"preparation_sha256":ph,"journal_sha256":hash(&jb),"estimand":"run_session_origin_to_owner_post_reap_evidence_sync","reap_observed_offset_ns":10,"enclosing_evidence_wall_ns":20,"journal_sync_clock":clock(40)}});
        let ob = serde_json::to_vec(&o).unwrap();
        let c = json!({"schema":"isolab.fresh-owner-controller-check/v1","request_sha256":"a".repeat(64),"pod_uid":"pod","job_uid":"job","records":[{"phase":"run","receipt_sha256":hash(&ob),"config_sha256":hash(&cb)}],"accepted_external_closure":null,"total_active_worker_ns":null});
        let custody = serde_json::to_vec(&c).unwrap();
        for (name, bytes) in [
            ("custody", &custody),
            ("owner", &ob),
            ("config", &cb),
            ("prep", &pb),
            ("journal", &jb),
        ] {
            std::fs::write(dir.join(name), bytes).unwrap();
        }
        let path = |name| dir.join(name).to_str().unwrap().to_string();
        let args = vec![
            "ecbench_owner_closure".into(),
            "--fresh".into(),
            path("custody"),
            hash(&custody),
            path("owner"),
            path("config"),
            hash(&cb),
            path("prep"),
            path("journal"),
            "a".repeat(64),
            "b".repeat(64),
            "pod".into(),
            "job".into(),
            "c".repeat(64),
        ];
        let v = run(&args).unwrap();
        assert_eq!(v["status"], "closed_fresh_enclosing_evidence");
        assert_eq!(v["closure"]["enclosing_session_evidence_wall_ns"], 20);
        assert!(v["worker_allocation_ns"].is_null());
        assert!(v["closure"]["exact_runner_exit_ns"].is_null());
        let mut wrong = args;
        wrong[13] = "d".repeat(64);
        assert!(run(&wrong).is_err());
        std::fs::remove_dir_all(dir).unwrap();
    }
}
