//! Card-bound one-use registration for the fresh-paired n17 rho arm.
//! Freeze/publish/replay bind the public card to the archived ecbench binary
//! without launching it. Execute consumes the claim once and refuses unless
//! given an explicit separate authorization token.
use super::{ordinary_build, sat_control::native, target_math};
use crypto_lib::cryptanalysis::{
    ecbench::workload::{CurveSpec, TargetKind, Workload, PUBLIC_TARGET_LAW},
    prepared_control_archive::verify_tar_inventory,
    prepared_sat_control::{canonical_sha, sha256},
};
use flate2::read::MultiGzDecoder;
use native::{load, read, require, save};
use serde_json::{json, Map, Value};
use std::{fs, io::Read, path::Path, process::Command, time::Instant};

const QUESTION: &str = "fresh-paired-n17-rho-target-registration-v1";
const PUBLISH_QUESTION: &str = "fresh-paired-n17-rho-target-scientific-registration-custody-v1";
const SOURCE_PUB_QUESTION: &str = "fresh-paired-n17-rho-source-publication-v1";
const STATUS_REGISTERED: &str = "RHO_CARD_REGISTERED_NOT_DISPATCHED";
const STATUS_PUBLISH: &str = "PASS_DATA_ONLY_RHO_TARGET_SCIENTIFIC_REGISTRATION_CUSTODY";
const CURVE_ID: &str = "EC1N17Ckb1hbbe2b5b6b1e6";
const SOURCE_QUESTION: &str = "fresh-paired-n17-source-publication-v1";
const CARD_SHA: &str = "bd969a4092a329bf093688b9d4715c86d77850c11f115ff47852c9235d7c2379";
const SOURCE_SEAL: &str = "f74601e7be2ef1c7141fee5a997b38c090e0f19eadbe0d7e843577b59918cf59";
const SOURCE_ARCHIVE: &str = "b0f0983a01ad3d7fc113f4e9f7b394890dcf019647c423263c5b2e61639eb05a";
const SOURCE_DESCRIPTOR: &str = "24d65abf45d33aa566882ec83f0bd73a676af5c2cb431f7a4841dd621b327256";
const ECBENCH_SHA: &str = "60fa3f7a042de090faabffcc09af28a59970c6d112320564b0f0621ecd413d47";
const FREEZE_SHA: &str = "a94df27cd3ed73974787e4cf87d755bbb062f5431645704a61b8c4d45f761aa2";
const TARGET: [u64; 2] = [87633, 27];
const TARGET_SEED: u64 = 13942624875975887236;
const WORKLOAD_ID: &str = "Wf3040f4f5554";
const WORKLOAD_SHA: &str = "f3040f4f5554a12cd8c8f04bfcf38f021a34e5de5f8248da6f23ee5e88d07927";
const DISPATCH_TOKEN: &str = "separately_authorized_rho_arm_v1";

fn desc(bytes: &[u8]) -> Value {
    json!({"bytes":bytes.len(),"sha256":sha256(bytes)})
}
fn outside(path: &Path, roots: &[&Path]) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or("rho target output has no parent")?
        .canonicalize()
        .map_err(|e| e.to_string())?;
    require(
        roots.iter().all(|root| !parent.starts_with(root)),
        "rho target output must stay outside sealed inputs and capsule",
    )
}
fn controller_sha() -> Result<String, String> {
    let own = std::env::current_exe().map_err(|e| e.to_string())?;
    Ok(sha256(&read(&own, 128 * 1024 * 1024)?))
}
fn unconsumed(root: &Path) -> Result<(), String> {
    require(
        !root
            .join("consumed.json")
            .try_exists()
            .map_err(|e| e.to_string())?,
        "rho target registration already consumed; never retry",
    )
}
fn policy() -> Value {
    json!({
        "method":"rho.signed_frobenius_strong",
        "lanes":32,
        "dp_bits":4,
        "step_cap_factor":2000
    })
}
fn ecbench_spec() -> Value {
    json!({
        "schema":"ecbench.spec/v1",
        "label":"fresh-paired-n17-rho-arm-v1",
        "description":"Card-bound one-use strong-rho arm for the fresh-paired n17 campaign. Not authorized by the four-arm freeze alone.",
        "workloads":{
            "curves":[{"kind":"koblitz","a":1,"n":17}],
            "targets_per_curve":1,
            "target_seed":TARGET_SEED,
            "target_kind":"public"
        },
        "arms":[{
            "name":"rho-strong",
            "role":"reference",
            "method":{
                "id":"rho.signed_frobenius_strong",
                "params":{"lanes":"32","dp_bits":"4","step_cap_factor":"2000"}
            }
        }],
        "measurement":{
            "rounds":1,
            "warmup":0,
            "order":"alternate",
            "seed":2026111901,
            "isolation_required":"L0",
            "timeout_seconds":900
        }
    })
}
fn rederive_card_point() -> Result<(), String> {
    let spec = CurveSpec::Koblitz { a: 1, n: 17 };
    let instance = spec.build()?;
    let workload = Workload::on(&spec, &instance, TARGET_SEED, 0, TargetKind::Public)?;
    let parse = |hex: &str| -> Result<u64, String> {
        u64::from_str_radix(
            hex.strip_prefix("0x")
                .ok_or_else(|| "noncanonical point hex".to_string())?,
            16,
        )
        .map_err(|_| "invalid point coordinate hex".to_string())
    };
    let target = [parse(&workload.target[0])?, parse(&workload.target[1])?];
    require(
        workload.target_law == PUBLIC_TARGET_LAW
            && workload.workload_id == WORKLOAD_ID
            && workload.workload_sha256 == WORKLOAD_SHA
            && target == TARGET,
        "card seed does not rederive freeze-pinned public point/workload",
    )
}
fn check_card(card_path: &Path) -> Result<(String, Vec<u8>), String> {
    let bytes = read(card_path, 65536)?;
    let sha = sha256(&bytes);
    require(
        sha == CARD_SHA,
        "rho target card SHA-256 differs from freeze pin",
    )?;
    let card = target_math::parse(&bytes)?;
    require(
        card["schema_version"] == 1
            && card["curve_id"] == CURVE_ID
            && card["target"] == json!(TARGET)
            && card["target_seed"] == TARGET_SEED
            && card["target_index"] == 0
            && card["target_counter"] == 1
            && card["ecbench_workload_id"] == WORKLOAD_ID
            && card["ecbench_workload_sha256"] == WORKLOAD_SHA
            && card["source_publications"]["rho"] == SOURCE_DESCRIPTOR
            && card["scalar_generated"] == false,
        "rho target card fields differ from freeze pins",
    )?;
    rederive_card_point()?;
    Ok((sha, bytes))
}
fn check_source(publication: &Path, descriptor: &Path) -> Result<Vec<u8>, String> {
    let header = load(&publication.join("SOURCE_PUBLICATION.json"))?;
    let seal = load(&publication.join("SOURCE_SEAL.json"))?;
    require(
        header["schema_version"] == 1
            && header["question"] == SOURCE_PUB_QUESTION
            && header["role"] == "rho"
            && header["curve_id"] == CURVE_ID
            && seal["source_registration_sha256"] == SOURCE_SEAL
            && header["archive"]["sha256"] == SOURCE_ARCHIVE
            && header["immutable_files"]["immutable/bin/ecbench"]["sha256"] == ECBENCH_SHA
            && header["source_bound_execution_admitted"] == false
            && header["target_card_created"] == false
            && header["target_worker_calls"] == 0
            && header["online_speedup"].is_null()
            && canonical_sha(&header)? == SOURCE_SEAL,
        "rho source publication differs from freeze pin",
    )?;
    let descriptor_bytes = read(descriptor, 65536)?;
    require(
        sha256(&descriptor_bytes) == SOURCE_DESCRIPTOR,
        "rho source descriptor SHA-256 differs from card pin",
    )?;
    let descriptor_value = target_math::parse(&descriptor_bytes)?;
    require(
        descriptor_value["schema_version"] == 1
            && descriptor_value["question"] == SOURCE_QUESTION
            && descriptor_value["role"] == "rho"
            && descriptor_value["curve_id"] == CURVE_ID
            && descriptor_value["registration_sha256"] == SOURCE_SEAL
            && descriptor_value["archive_sha256"] == SOURCE_ARCHIVE
            && descriptor_value["target_free"] == true
            && descriptor_value["source_bound_execution_admitted"] == false,
        "rho source descriptor fields differ from freeze pin",
    )?;
    let binary = read(
        &publication.join("immutable/bin/ecbench"),
        128 * 1024 * 1024,
    )?;
    require(
        sha256(&binary) == ECBENCH_SHA,
        "archived ecbench bytes differ from freeze pin",
    )?;
    Ok(descriptor_bytes)
}
fn check_freeze(freeze_path: &Path) -> Result<Vec<u8>, String> {
    let bytes = read(freeze_path, 1024 * 1024)?;
    require(
        sha256(&bytes) == FREEZE_SHA,
        "four-arm freeze SHA-256 differs from archived pin",
    )?;
    let freeze = target_math::parse(&bytes)?;
    require(
        freeze["status"] == "FOUR_ARM_FROZEN_NOT_DISPATCHED"
            && freeze["dispatch_authorization"] == "not_granted_freeze_only"
            && freeze["arm_order"] == json!(["rho", "f5", "sat", "incumbent"])
            && freeze["card_sha256"] == CARD_SHA
            && freeze["public_target"] == json!(TARGET)
            && freeze["pins"]["source_descriptors"]["rho"]["registration_sha256"] == SOURCE_SEAL
            && freeze["pins"]["source_descriptors"]["rho"]["archive_sha256"] == SOURCE_ARCHIVE
            && freeze["pins"]["source_descriptors"]["rho"]["descriptor_sha256"]
                == SOURCE_DESCRIPTOR
            && freeze["policies"]["rho"] == policy()
            && freeze["claims"]["execution_admitted"] == false
            && freeze["claims"]["four_arm_dispatch_created"] == false,
        "four-arm freeze pins do not admit this rho registration",
    )?;
    Ok(bytes)
}
fn immutable_inventory(root: &Path) -> Result<Map<String, Value>, String> {
    let mut files = Map::new();
    for (name, value) in native::inventory(&root.join("immutable"))?
        .as_object()
        .ok_or("rho target immutable inventory missing")?
    {
        files.insert(format!("immutable/{name}"), value.clone());
    }
    Ok(files)
}
fn registration_claims(reg: &Value) -> Result<(), String> {
    require(
        reg["schema_version"] == 1
            && reg["question"] == QUESTION
            && reg["role"] == "rho"
            && reg["status"] == STATUS_REGISTERED
            && reg["curve_id"] == CURVE_ID
            && reg["card_sha256"] == CARD_SHA
            && reg["public_target"] == json!(TARGET)
            && reg["source_registration_sha256"] == SOURCE_SEAL
            && reg["source_archive_sha256"] == SOURCE_ARCHIVE
            && reg["source_descriptor_sha256"] == SOURCE_DESCRIPTOR
            && reg["ecbench_sha256"] == ECBENCH_SHA
            && reg["four_arm_freeze_sha256"] == FREEZE_SHA
            && reg["arm"] == "rho"
            && reg["arm_order_position"] == 0
            && reg["one_use"] == true
            && reg["resume_forbidden"] == true
            && reg["retry_forbidden"] == true
            && reg["execution_admitted"] == false
            && reg["source_bound_execution_admitted"] == false
            && reg["online_speedup"].is_null()
            && reg["dispatch_authorization_recorded"] == "not_granted_freeze_only"
            && reg["scientific_worker_calls"] == 0,
        "rho target registration overstates custody or diverges from freeze pins",
    )
}
fn retained_files(root: &Path) -> Result<Map<String, Value>, String> {
    let mut files = Map::new();
    for name in [
        "registration.json",
        "seal.json",
        "config.json",
        "card.json",
        "source-descriptor.json",
        "four-arm-freeze.json",
        "ecbench.spec.json",
        "host-context.json",
    ] {
        files.insert(
            name.into(),
            desc(&read(&root.join(name), 16 * 1024 * 1024)?),
        );
    }
    files.extend(immutable_inventory(root)?);
    Ok(files)
}

pub(super) fn freeze(
    source_publication: &Path,
    source_descriptor: &Path,
    card: &Path,
    freeze_path: &Path,
    capsule: &Path,
    out: &Path,
) -> Result<String, String> {
    let source_publication = source_publication
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let source_descriptor = source_descriptor
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let card = card.canonicalize().map_err(|e| e.to_string())?;
    let freeze_path = freeze_path.canonicalize().map_err(|e| e.to_string())?;
    require(
        !capsule.exists(),
        "rho target capsule path already exists; refuse overwrite",
    )?;
    outside(out, &[&source_publication, &card, &freeze_path, capsule])?;
    let descriptor_bytes = check_source(&source_publication, &source_descriptor)?;
    let (card_sha, card_bytes) = check_card(&card)?;
    let freeze_bytes = check_freeze(&freeze_path)?;
    fs::create_dir(capsule).map_err(|e| e.to_string())?;
    let capsule = capsule.canonicalize().map_err(|e| e.to_string())?;
    let immutable = capsule.join("immutable");
    fs::create_dir(&immutable).map_err(|e| e.to_string())?;
    fs::create_dir(immutable.join("bin")).map_err(|e| e.to_string())?;
    fs::create_dir(immutable.join("build")).map_err(|e| e.to_string())?;
    for (src, dst) in [
        ("immutable/bin/ecbench", "bin/ecbench"),
        ("immutable/build/build.log", "build/build.log"),
        (
            "immutable/build/build-record.json",
            "build/build-record.json",
        ),
    ] {
        native::create(
            &immutable.join(dst),
            &read(&source_publication.join(src), 128 * 1024 * 1024)?,
        )?;
    }
    native::create(&capsule.join("card.json"), &card_bytes)?;
    native::create(&capsule.join("source-descriptor.json"), &descriptor_bytes)?;
    native::create(&capsule.join("four-arm-freeze.json"), &freeze_bytes)?;
    let spec = ecbench_spec();
    save(&capsule.join("ecbench.spec.json"), &spec)?;
    save(
        &capsule.join("host-context.json"),
        &json!({
            "schema_version":1,
            "host_class":"physical-macos-arm64-exploratory",
            "isolation":"not_isolated_host_receipt",
            "recorded_at_stage":"rho-target-freeze"
        }),
    )?;
    save(
        &capsule.join("config.json"),
        &json!({
            "schema_version":1,
            "question":QUESTION,
            "worker":"ecbench",
            "worker_timeout_ms":900000,
            "policy":policy(),
            "allow_busy":true,
            "cpus":"none"
        }),
    )?;
    let immutable_files = immutable_inventory(&capsule)?;
    require(
        immutable_files["immutable/bin/ecbench"]["sha256"] == ECBENCH_SHA,
        "capsule ecbench inventory differs from freeze pin",
    )?;
    let registration = json!({
        "schema_version":1,
        "question":QUESTION,
        "role":"rho",
        "status":STATUS_REGISTERED,
        "curve_id":CURVE_ID,
        "card_sha256":card_sha,
        "public_target":TARGET,
        "target_seed":TARGET_SEED,
        "ecbench_workload_id":WORKLOAD_ID,
        "ecbench_workload_sha256":WORKLOAD_SHA,
        "source_registration_sha256":SOURCE_SEAL,
        "source_archive_sha256":SOURCE_ARCHIVE,
        "source_descriptor_sha256":SOURCE_DESCRIPTOR,
        "ecbench_sha256":ECBENCH_SHA,
        "four_arm_freeze_sha256":FREEZE_SHA,
        "arm":"rho",
        "arm_order_position":0,
        "policy":policy(),
        "ecbench_spec":spec,
        "one_use":true,
        "resume_forbidden":true,
        "retry_forbidden":true,
        "execution_admitted":false,
        "source_bound_execution_admitted":false,
        "dispatch_authorization_recorded":"not_granted_freeze_only",
        "scientific_worker_calls":0,
        "online_speedup":null,
        "controller_sha256":controller_sha()?,
        "immutable_files":immutable_files
    });
    registration_claims(&registration)?;
    save(&capsule.join("registration.json"), &registration)?;
    let seal = canonical_sha(&registration)?;
    save(
        &capsule.join("seal.json"),
        &json!({"registration_sha256":seal}),
    )?;
    let result = json!({
        "schema_version":1,
        "status":STATUS_REGISTERED,
        "registration_sha256":seal,
        "card_sha256":CARD_SHA,
        "public_target":TARGET,
        "source_registration_sha256":SOURCE_SEAL,
        "ecbench_sha256":ECBENCH_SHA,
        "four_arm_freeze_sha256":FREEZE_SHA,
        "capsule":capsule,
        "scientific_worker_calls":0,
        "execution_admitted":false,
        "source_bound_execution_admitted":false,
        "online_speedup":null,
        "dispatch_authorization_recorded":"not_granted_freeze_only"
    });
    save(out, &result)?;
    serde_json::to_string_pretty(&result).map_err(|e| e.to_string())
}
fn load_registration(root: &Path, seal: &str) -> Result<Value, String> {
    let reg = load(&root.join("registration.json"))?;
    registration_claims(&reg)?;
    require(
        canonical_sha(&reg)? == seal
            && load(&root.join("seal.json"))?["registration_sha256"] == seal,
        "rho target registration seal differs",
    )?;
    require(
        sha256(&read(
            &root.join("immutable/bin/ecbench"),
            128 * 1024 * 1024,
        )?) == ECBENCH_SHA,
        "capsule ecbench changed after freeze",
    )?;
    unconsumed(root)?;
    Ok(reg)
}

pub(super) fn publish(
    capsule: &Path,
    registration_sha256: &str,
    publication: &Path,
    out: &Path,
) -> Result<String, String> {
    let capsule = capsule.canonicalize().map_err(|e| e.to_string())?;
    require(
        !publication.exists(),
        "rho target publication already exists",
    )?;
    outside(out, &[&capsule, publication])?;
    let reg = load_registration(&capsule, registration_sha256)?;
    fs::create_dir(publication).map_err(|e| e.to_string())?;
    let publication = publication.canonicalize().map_err(|e| e.to_string())?;
    let files = retained_files(&capsule)?;
    for name in files.keys() {
        let target = publication.join(name);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        native::create(&target, &read(&capsule.join(name), 128 * 1024 * 1024)?)?;
    }
    ordinary_build::archive(&publication, &publication.join("capsule.tar.gz"), &files)?;
    let archive = read(&publication.join("capsule.tar.gz"), 96 * 1024 * 1024)?;
    let header = json!({
        "schema_version":1,
        "question":PUBLISH_QUESTION,
        "role":"rho",
        "status":STATUS_PUBLISH,
        "curve_id":CURVE_ID,
        "registration_sha256":registration_sha256,
        "card_sha256":CARD_SHA,
        "public_target":TARGET,
        "source_registration_sha256":SOURCE_SEAL,
        "ecbench_sha256":ECBENCH_SHA,
        "four_arm_freeze_sha256":FREEZE_SHA,
        "files":files,
        "archive":{"path":"capsule.tar.gz","bytes":archive.len(),"sha256":sha256(&archive)},
        "scientific_registration_published":true,
        "scientific_worker_calls":0,
        "execution_admitted":false,
        "source_bound_execution_admitted":false,
        "online_speedup":null,
        "dispatch_authorization_recorded":"not_granted_freeze_only",
        "registration":reg
    });
    save(&publication.join("PUBLICATION.json"), &header)?;
    let result = verify_publication(&publication, registration_sha256)?;
    save(out, &result)?;
    serde_json::to_string_pretty(&result).map_err(|e| e.to_string())
}
fn verify_publication(publication: &Path, seal: &str) -> Result<Value, String> {
    let header = load(&publication.join("PUBLICATION.json"))?;
    require(
        header["schema_version"] == 1
            && header["question"] == PUBLISH_QUESTION
            && header["role"] == "rho"
            && header["status"] == STATUS_PUBLISH
            && header["registration_sha256"] == seal
            && header["card_sha256"] == CARD_SHA
            && header["public_target"] == json!(TARGET)
            && header["scientific_worker_calls"] == 0
            && header["execution_admitted"] == false
            && header["source_bound_execution_admitted"] == false
            && header["online_speedup"].is_null()
            && header["dispatch_authorization_recorded"] == "not_granted_freeze_only",
        "rho target publication overstates scientific custody",
    )?;
    let files = header["files"]
        .as_object()
        .ok_or("rho target publication file inventory missing")?;
    require(
        files.contains_key("immutable/bin/ecbench")
            && files["immutable/bin/ecbench"]["sha256"] == ECBENCH_SHA
            && files.contains_key("registration.json")
            && files.contains_key("card.json")
            && files.contains_key("ecbench.spec.json"),
        "rho target publication missing bound ecbench or card sidecars",
    )?;
    for (name, meta) in files {
        let bytes = read(&publication.join(name), 128 * 1024 * 1024)?;
        require(
            desc(&bytes) == *meta,
            "rho target publication member bytes differ from inventory",
        )?;
    }
    let archive = read(&publication.join("capsule.tar.gz"), 96 * 1024 * 1024)?;
    require(
        header["archive"]
            == json!({"path":"capsule.tar.gz","bytes":archive.len(),"sha256":sha256(&archive)}),
        "rho target publication archive bytes differ",
    )?;
    let mut tar = Vec::new();
    MultiGzDecoder::new(archive.as_slice())
        .take(512 * 1024 * 1024 + 1)
        .read_to_end(&mut tar)
        .map_err(|e| e.to_string())?;
    require(
        tar.len() <= 512 * 1024 * 1024,
        "rho target publication archive exceeds decompression bound",
    )?;
    let count = verify_tar_inventory(&tar, &json!(files))?;
    Ok(json!({
        "schema_version":1,
        "status":STATUS_PUBLISH,
        "role":"rho",
        "registration_sha256":seal,
        "card_sha256":CARD_SHA,
        "public_target":TARGET,
        "source_registration_sha256":SOURCE_SEAL,
        "ecbench_sha256":ECBENCH_SHA,
        "archive_sha256":sha256(&archive),
        "archive_regular_files":count,
        "scientific_registration_published":true,
        "scientific_worker_calls":0,
        "archived_binaries_executed":0,
        "execution_admitted":false,
        "source_bound_execution_admitted":false,
        "online_speedup":null,
        "dispatch_authorization_recorded":"not_granted_freeze_only"
    }))
}
pub(super) fn replay(
    publication: &Path,
    registration_sha256: &str,
    out: &Path,
) -> Result<String, String> {
    let publication = publication.canonicalize().map_err(|e| e.to_string())?;
    outside(out, &[&publication])?;
    let result = verify_publication(&publication, registration_sha256)?;
    save(out, &result)?;
    serde_json::to_string_pretty(&result).map_err(|e| e.to_string())
}

pub(super) fn execute(
    capsule: &Path,
    publication: &Path,
    execution: &Path,
    registration_sha256: &str,
    dispatch_authorization: &str,
) -> Result<String, String> {
    require(
        dispatch_authorization == DISPATCH_TOKEN,
        "rho target execute refused: four-arm freeze is not_granted_freeze_only; supply separately_authorized_rho_arm_v1 only when externally authorized",
    )?;
    let capsule = capsule.canonicalize().map_err(|e| e.to_string())?;
    let publication = publication.canonicalize().map_err(|e| e.to_string())?;
    require(!execution.exists(), "rho execution path already exists")?;
    outside(execution, &[&capsule, &publication])?;
    let reg = load_registration(&capsule, registration_sha256)?;
    let pub_preflight = verify_publication(&publication, registration_sha256)?;
    require(
        controller_sha()? == reg["controller_sha256"],
        "rho target execute requires the same tip controller that froze the registration",
    )?;
    let ecbench = capsule.join("immutable/bin/ecbench");
    fs::create_dir(execution).map_err(|e| e.to_string())?;
    let execution = execution.canonicalize().map_err(|e| e.to_string())?;
    save(
        &execution.join("publication-preflight.json"),
        &pub_preflight,
    )?;
    native::create(
        &execution.join("card.json"),
        &read(&capsule.join("card.json"), 65536)?,
    )?;
    native::create(
        &execution.join("ecbench.spec.json"),
        &read(&capsule.join("ecbench.spec.json"), 65536)?,
    )?;
    save(
        &capsule.join("consumed.json"),
        &json!({
            "schema_version":1,
            "question":QUESTION,
            "registration_sha256":registration_sha256,
            "execution":execution,
            "ecbench_sha256":ECBENCH_SHA,
            "card_sha256":CARD_SHA,
            "consumed_before_worker_launch":true
        }),
    )?;
    let started = Instant::now();
    let out_dir = execution.join("ecbench-session");
    let spec = execution.join("ecbench.spec.json");
    let output = Command::new(&ecbench)
        .args([
            "run",
            "--spec",
            spec.to_str().ok_or("spec path")?,
            "--out",
            out_dir.to_str().ok_or("session path")?,
            "--cpus",
            "none",
            "--allow-busy",
        ])
        .env_clear()
        .env("PATH", "/opt/homebrew/bin:/usr/bin:/bin")
        .env(
            "TMPDIR",
            std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".into()),
        )
        .current_dir(&execution)
        .output()
        .map_err(|e| e.to_string())?;
    let wall_ms = started.elapsed().as_millis();
    native::create(&execution.join("ecbench.stdout"), &output.stdout)?;
    native::create(&execution.join("ecbench.stderr"), &output.stderr)?;
    let terminal = json!({
        "schema_version":1,
        "question":QUESTION,
        "registration_sha256":registration_sha256,
        "card_sha256":CARD_SHA,
        "ecbench_sha256":ECBENCH_SHA,
        "exit_code":output.status.code(),
        "timed_out":false,
        "wall_ms":wall_ms,
        "session":out_dir,
        "source_bound_execution_admitted":output.status.success(),
        "fresh_paired_qualification":false,
        "headline_eligible":false,
        "online_speedup":null,
        "dispatch_authorization":dispatch_authorization
    });
    save(&execution.join("terminal.json"), &terminal)?;
    require(
        output.status.success(),
        "rho target ecbench failed or interrupted; terminal retained; never retry",
    )?;
    serde_json::to_string_pretty(&terminal).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_matches_four_arm_freeze_pin() {
        assert_eq!(
            policy(),
            json!({
                "method":"rho.signed_frobenius_strong",
                "lanes":32,
                "dp_bits":4,
                "step_cap_factor":2000
            })
        );
    }

    #[test]
    fn execute_token_is_fail_closed() {
        let err = execute(
            Path::new("/nonexistent-a"),
            Path::new("/nonexistent-b"),
            Path::new("/nonexistent-c"),
            "deadbeef",
            "not_granted_freeze_only",
        )
        .unwrap_err();
        assert!(err.contains("separately_authorized") || err.contains("not_granted_freeze_only"));
    }
}
