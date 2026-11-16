//! Recover completed cells from immutable publication receipts on a new host.
//!
//! Downloads go into retained attempt directories, outside the active cells
//! tree. A cell becomes active only after remote byte checks and native replay.
//! This is a reader: it never requires the producer executable to be installed,
//! and it never uploads or changes a remote object. S3 reads require an active
//! budget journal; the separate activation command first writes and verifies
//! its admitted ownership control object. Supplied job/pod identities are exact
//! bindings, not an automatic Kubernetes ownership attestation.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::path::Path;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::budget::BudgetScope;
use super::s3_budget::S3BudgetContext;
use super::storage::{self, PublicationConfirmation, PublishReceipt, PublishedObject};
use super::types::Artifact;
use crate::store::backend::{object_key, RemoteSpec};
use crate::store::index::{validate_name, validate_sha256};
use crate::store::{Config, Store};

/// Bound restore I/O before trusting a downloaded producer run binding. The
/// producer's trace_mib may describe a much larger job and is not an authority
/// to allocate local resources. Larger restores fail explicitly, never truncate.
const RESTORE_MAX_BYTES: u64 = 1024 * 1024 * 1024;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct CompletionRecord {
    schema: String,
    remote: String,
    namespace: String,
    manifest_sha256: String,
    objects: Vec<PublishedObject>,
}

fn canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
    let value = serde_json::to_value(value).map_err(|e| e.to_string())?;
    let mut bytes = serde_json::to_vec_pretty(&value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn checked_receipt(path: &Path) -> Result<PublishReceipt, String> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let parent = super::checked_directory(parent, false)?;
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("receipt needs a UTF-8 filename")?;
    let path = storage::contained_path(&parent, name, false)?;
    let metadata = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err("publication receipt must be a regular file".into());
    }
    if metadata.len() > super::JSON_INPUT_MAX_BYTES {
        return Err(format!(
            "publication receipt exceeds {}-byte JSON input limit",
            super::JSON_INPUT_MAX_BYTES
        ));
    }
    super::read_json(&path)
}

fn replay_parses_json(artifact: &Artifact) -> bool {
    [
        "manifest.json",
        "plan.json",
        "run.json",
        "protocol.json",
        "config.json",
        "public.json",
        "witness.json",
        "factor-base.json",
    ]
    .contains(&artifact.path.as_str())
        || [
            "independently-verified-pdp",
            "restricted-independent-scalar-label",
            "restricted-large-prime-membership-provenance",
            "large-prime-elimination-transcript",
        ]
        .contains(&artifact.role.as_str())
}

fn validate_object(object: &PublishedObject, namespace: &str) -> Result<(), String> {
    let artifact = &object.artifact;
    validate_sha256(&artifact.sha256)?;
    validate_name(&object.name)?;
    // Path safety is checked here, before anything is downloaded. The actual
    // filesystem containment check is repeated before every installation.
    if artifact.path.is_empty()
        || artifact.path.contains('\\')
        || artifact.path.bytes().any(|b| b.is_ascii_control())
        || artifact
            .path
            .split('/')
            .any(|p| p.is_empty() || p == "." || p == "..")
        || Path::new(&artifact.path)
            .components()
            .any(|p| !matches!(p, std::path::Component::Normal(_)))
    {
        return Err(format!("unsafe receipt artifact path {:?}", artifact.path));
    }
    if artifact.role.trim().is_empty()
        || artifact.encoding.trim().is_empty()
        || object.name != format!("{namespace}/{}", artifact.path)
        || object.object_key != object_key(&artifact.sha256)?
        || !object.roundtrip_verified
    {
        return Err(format!(
            "invalid publication object binding for {}",
            artifact.path
        ));
    }
    Ok(())
}

fn validate_receipt(receipt: &PublishReceipt, remote: Option<&str>) -> Result<RemoteSpec, String> {
    if receipt.schema != "ml-dataset.publication/v1" {
        return Err("unknown publication receipt schema".into());
    }
    validate_name(&receipt.namespace)?;
    validate_sha256(&receipt.manifest_sha256)?;
    let original = RemoteSpec::parse(&receipt.remote)?;
    let selected = RemoteSpec::parse(remote.unwrap_or(&receipt.remote))?;
    for spec in [&original, &selected] {
        if let RemoteSpec::S3 { bucket, .. } = spec {
            if bucket != "setec-astronomy-artifacts" {
                return Err(
                    "dataset restore may only read S3 bucket setec-astronomy-artifacts".into(),
                );
            }
        }
    }
    if selected != original {
        return Err("restore remote override must match the receipt remote; a mirror cannot attest the original remote".into());
    }
    let mut paths = BTreeSet::new();
    if receipt.completion.artifact.bytes > super::JSON_INPUT_MAX_BYTES {
        return Err("completion receipt exceeds JSON input limit".into());
    }
    let mut total = receipt.completion.artifact.bytes;
    for object in &receipt.objects {
        validate_object(object, &receipt.namespace)?;
        if replay_parses_json(&object.artifact)
            && object.artifact.bytes > super::JSON_INPUT_MAX_BYTES
        {
            return Err(format!(
                "parsed artifact {} exceeds {}-byte JSON input limit",
                object.artifact.path,
                super::JSON_INPUT_MAX_BYTES
            ));
        }
        if !paths.insert(object.artifact.path.as_str()) {
            return Err("duplicate publication artifact path".into());
        }
        if object.artifact.path.starts_with("publication/") {
            return Err("cell artifact overlaps publication metadata".into());
        }
        total = total
            .checked_add(object.artifact.bytes)
            .ok_or("receipt byte count overflow")?;
        if total > RESTORE_MAX_BYTES {
            return Err(format!("restore exceeds {RESTORE_MAX_BYTES}-byte aggregate budget; larger restores require an explicitly configured implementation, no data was truncated"));
        }
    }
    let manifest = receipt
        .objects
        .iter()
        .find(|o| o.artifact.path == "manifest.json")
        .ok_or("receipt has no cell manifest")?;
    if manifest.artifact.sha256 != receipt.manifest_sha256
        || manifest.artifact.role != "cell-manifest"
        || manifest.artifact.encoding != "json"
    {
        return Err("receipt manifest digest, role or encoding mismatch".into());
    }
    validate_object(&receipt.completion, &receipt.namespace)?;
    let record = CompletionRecord {
        schema: receipt.schema.clone(),
        remote: receipt.remote.clone(),
        namespace: receipt.namespace.clone(),
        manifest_sha256: receipt.manifest_sha256.clone(),
        objects: receipt.objects.clone(),
    };
    let receipt_id = super::digest(&serde_json::to_vec(&record).map_err(|e| e.to_string())?);
    let completion = &receipt.completion.artifact;
    if completion.path != format!("publication/{receipt_id}.json")
        || completion.role != "publication_completion"
        || completion.encoding != "json"
    {
        return Err("completion path is not bound to the receipt contents".into());
    }
    let expected = confirmation(receipt);
    let bytes = canonical_json(&expected)?;
    if receipt.confirmation
        != (Artifact {
            path: format!("publication/verified-{receipt_id}.json"),
            role: "publication_confirmation".into(),
            encoding: "json".into(),
            sha256: super::digest(&bytes),
            bytes: bytes.len() as u64,
        })
    {
        return Err("receipt confirmation does not bind the expected completion".into());
    }
    Ok(selected)
}

fn confirmation(receipt: &PublishReceipt) -> PublicationConfirmation {
    PublicationConfirmation {
        schema: "ml-dataset.publication-confirmation/v1".into(),
        remote: receipt.remote.clone(),
        namespace: receipt.namespace.clone(),
        manifest_sha256: receipt.manifest_sha256.clone(),
        completion: receipt.completion.clone(),
    }
}

/// Install an already verified same-filesystem file without replacing any name.
fn link_checked(source: &Path, root: &Path, artifact: &Artifact) -> Result<(), String> {
    storage::check_file(source, &artifact.sha256, artifact.bytes)?;
    let destination = storage::contained_path(root, &artifact.path, true)?;
    match fs::hard_link(source, &destination) {
        Ok(()) => storage::sync_directory(destination.parent().ok_or("artifact has no parent")?)?,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            storage::check_file(&destination, &artifact.sha256, artifact.bytes)?;
        }
        Err(e) => return Err(format!("install {}: {e}", destination.display())),
    }
    Ok(())
}

/// Backend::get streams to a fresh attempt-local destination. Even an identical
/// retry fetches again: local cache bytes cannot establish current remote proof.
fn download(
    store: &Store,
    scratch: &Path,
    cell: &Path,
    index: usize,
    object: &PublishedObject,
) -> Result<(), String> {
    let name = format!("object-{index:08}.download");
    let destination = storage::contained_path(scratch, &name, false)?;
    if fs::symlink_metadata(&destination).is_ok() {
        return Err("fresh download destination unexpectedly exists".into());
    }
    let remote_meta = store
        .remote()?
        .head(&object.object_key)?
        .ok_or_else(|| format!("remote restore object missing: {}", object.object_key))?;
    if remote_meta.bytes != object.artifact.bytes {
        return Err(format!(
            "remote restore size differs from receipt for {}; refusing download",
            object.object_key
        ));
    }
    store.remote()?.get(&object.object_key, &destination)?;
    storage::check_file(&destination, &object.artifact.sha256, object.artifact.bytes)
        .map_err(|e| format!("remote restore verification failed: {e}"))?;
    File::open(&destination)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())?;
    link_checked(&destination, cell, &object.artifact)
}

#[cfg(target_os = "linux")]
fn activate(source: &Path, destination: &Path) -> Result<(), String> {
    use std::os::unix::ffi::OsStrExt;
    let source_c =
        std::ffi::CString::new(source.as_os_str().as_bytes()).map_err(|e| e.to_string())?;
    let destination_c =
        std::ffi::CString::new(destination.as_os_str().as_bytes()).map_err(|e| e.to_string())?;
    let result = unsafe {
        libc::renameat2(
            libc::AT_FDCWD,
            source_c.as_ptr(),
            libc::AT_FDCWD,
            destination_c.as_ptr(),
            libc::RENAME_NOREPLACE,
        )
    };
    if result != 0 {
        return Err(format!(
            "activate restored cell without replacement: {}",
            std::io::Error::last_os_error()
        ));
    }
    storage::sync_directory(destination.parent().ok_or("cell has no parent")?)?;
    storage::sync_directory(source.parent().ok_or("staging cell has no parent")?)
}

#[cfg(not(target_os = "linux"))]
fn activate(_source: &Path, _destination: &Path) -> Result<(), String> {
    Err(
        "atomic dataset restore activation currently requires Linux renameat2(RENAME_NOREPLACE)"
            .into(),
    )
}

/// Restore one published cell into a run directory. Repeated calls accumulate
/// cells only when the entire run binding (including source, binary, caps and
/// shard axes) matches. Partial downloads are retained for inspection/retry;
/// each retry performs fresh remote reads before creating a local confirmation.
pub fn restore(receipt_path: &Path, out: &Path, remote: Option<&str>) -> Result<Value, String> {
    restore_budgeted(receipt_path, out, remote, None)
}

pub fn restore_budgeted(
    receipt_path: &Path,
    out: &Path,
    remote: Option<&str>,
    budget: Option<&Arc<S3BudgetContext>>,
) -> Result<Value, String> {
    let receipt = checked_receipt(receipt_path)?;
    let remote = validate_receipt(&receipt, remote)?;
    // Refuse the ordinary CLI-backed store before creating a restore attempt.
    if matches!(&remote, RemoteSpec::S3 { .. }) && budget.is_none() {
        return Err("S3 restore requires an active bound budget session".into());
    }
    let _lease = super::Lease::acquire(out)?;
    let out = super::checked_directory(out, false)?;
    super::assert_no_workers(&out)?;
    let staging = super::checked_directory(
        &out.join(".restore")
            .join(&receipt.completion.artifact.sha256),
        true,
    )?;
    let attempt = staging.join(format!(
        "attempt-{}-{}",
        std::process::id(),
        super::unix_nanos()?
    ));
    fs::create_dir(&attempt).map_err(|e| e.to_string())?;
    storage::sync_directory(&staging)?;
    let attempt = super::checked_directory(&attempt, false)?;
    let scratch = super::checked_directory(&attempt.join("downloads"), true)?;
    let cell = super::checked_directory(&attempt.join("cell"), true)?;
    storage::write_json(&attempt, "receipt.json", "restore-input-receipt", &receipt)?;
    let mut cfg = Config::discover()?;
    cfg.remote = Some(remote.clone());
    cfg.index_path = attempt.join("read-only-store-index.json");
    cfg.cache_dir = attempt.join("unused-cache");
    let store = storage::dataset_store(cfg, budget, BudgetScope::Work)?;
    for object in std::iter::once(&receipt.completion).chain(&receipt.objects) {
        storage::expect_remote_object(
            budget,
            &remote,
            &object.object_key,
            &object.artifact,
            BudgetScope::Work,
        )?;
    }

    // Fetch the final publication marker first, and require it to bind the
    // complete object list supplied in the externally retained receipt.
    download(&store, &scratch, &cell, 0, &receipt.completion)?;
    let completion: CompletionRecord =
        super::read_json(&cell.join(&receipt.completion.artifact.path))?;
    if completion
        != (CompletionRecord {
            schema: receipt.schema.clone(),
            remote: receipt.remote.clone(),
            namespace: receipt.namespace.clone(),
            manifest_sha256: receipt.manifest_sha256.clone(),
            objects: receipt.objects.clone(),
        })
    {
        return Err("remote completion does not exactly match the supplied receipt".into());
    }
    for (i, object) in receipt.objects.iter().enumerate() {
        download(&store, &scratch, &cell, i + 1, object)?;
    }
    let manifest = super::verify_cell(&cell, true)?;
    let declared: BTreeMap<_, _> = manifest.artifacts.iter().map(|a| (&a.path, a)).collect();
    let downloaded: BTreeMap<_, _> = receipt
        .objects
        .iter()
        .filter(|o| o.artifact.path != "manifest.json")
        .map(|o| (&o.artifact.path, &o.artifact))
        .collect();
    if declared != downloaded || declared.len() != manifest.artifacts.len() {
        return Err("remote manifest artifacts do not exactly match the receipt object set".into());
    }
    let (plan, binding, _) = super::bound_run(&cell)?;
    if let Some(context) = budget {
        super::validate_budget_run(context.session(), &cell)?;
    }
    let shard_count = binding["shard_count"]
        .as_u64()
        .filter(|n| *n > 0)
        .ok_or("invalid restore shard count")?;
    let shard_index = binding["shard_index"]
        .as_u64()
        .filter(|n| *n < shard_count)
        .ok_or("invalid restore shard index")?;
    if manifest.cell_index % shard_count != shard_index
        || binding["seconds_per_case"].as_u64().is_none_or(|n| n == 0)
        || binding["memory_mib"].as_u64().is_none_or(|n| n < 128)
        || binding["trace_mib"].as_u64().is_none_or(|n| n == 0)
        || receipt.namespace != super::publication_namespace(&binding, manifest.cell_index)?
        || manifest.config != plan.cell(manifest.cell_index)?
    {
        return Err("restored cell has an invalid shard, resource or namespace binding".into());
    }
    // Compare every existing root provenance file before installing any root
    // file. Exact bytes preserve the originally frozen protocol and caps.
    let roots: Vec<&Artifact> = ["plan.json", "run.json", "protocol.json"]
        .iter()
        .map(|name| {
            manifest
                .artifacts
                .iter()
                .find(|a| a.path == *name)
                .ok_or("missing root provenance artifact")
        })
        .collect::<Result<_, _>>()?;
    for artifact in &roots {
        let path = storage::contained_path(&out, &artifact.path, false)?;
        if fs::symlink_metadata(&path).is_ok() {
            storage::check_file(&path, &artifact.sha256, artifact.bytes)
                .map_err(|e| format!("restored run/source/binary/caps conflict: {e}"))?;
        }
    }
    // Existing completed cells must all share the exact binding. This also
    // rejects a directory assembled from unrelated source builds or caps.
    for existing in super::completed_cells(&out)? {
        // Existing cells still receive full byte/provenance checks. Their
        // arithmetic was replayed on activation; replaying every unrelated
        // cell on every insertion makes N restores perform O(N²) replays.
        let existing_manifest = super::verify_cell(&existing, false)?;
        if super::read_json::<Value>(&existing.join("run.json"))? != binding
            || existing_manifest.cell_index % shard_count != shard_index
            || existing.file_name().and_then(|n| n.to_str())
                != Some(format!("{:012}", existing_manifest.cell_index).as_str())
        {
            return Err("existing completed cell has a different source/run binding".into());
        }
    }
    let cells = super::checked_directory(&out.join("cells"), true)?;
    let destination = cells.join(format!("{:012}", manifest.cell_index));
    let already_present = match fs::symlink_metadata(&destination) {
        Ok(metadata) => {
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                return Err("restored cell destination is not an ordinary directory".into());
            }
            if !destination.join("manifest.json").exists() {
                return Err(
                    "restore overlaps an incomplete local attempt; preserve and reconcile it first"
                        .into(),
                );
            }
            let original = super::verify_cell(&destination, true)?;
            storage::check_file(
                &destination.join("manifest.json"),
                &receipt.manifest_sha256,
                receipt
                    .objects
                    .iter()
                    .find(|o| o.artifact.path == "manifest.json")
                    .expect("validated manifest")
                    .artifact
                    .bytes,
            )?;
            if original.cell_index != manifest.cell_index
                || super::read_json::<Value>(&destination.join("run.json"))? != binding
            {
                return Err("restore overlaps a different completed cell".into());
            }
            true
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
        Err(e) => return Err(e.to_string()),
    };
    // A confirmation exists only after every object was freshly downloaded,
    // hash checked and replayed during this invocation.
    let ack = storage::write_json(
        &cell,
        &receipt.confirmation.path,
        "publication_confirmation",
        &confirmation(&receipt),
    )?;
    if ack != receipt.confirmation {
        return Err("restored confirmation bytes differ from publication receipt".into());
    }
    for artifact in roots {
        link_checked(&cell.join(&artifact.path), &out, artifact)?;
    }
    super::bound_run(&out)?;
    if already_present {
        link_checked(
            &cell.join(&receipt.completion.artifact.path),
            &destination,
            &receipt.completion.artifact,
        )?;
        link_checked(
            &cell.join(&receipt.confirmation.path),
            &destination,
            &receipt.confirmation,
        )?;
    } else {
        activate(&cell, &destination)?;
    }
    let confirmations = super::publication_confirmations(&destination, &receipt.manifest_sha256)?;
    let report = json!({"schema":"crypto.ml-dataset.restore/v1","cell_index":manifest.cell_index,
        "output":out,"cell_directory":destination,"already_present":already_present,
        "remote":receipt.remote,"manifest_sha256":receipt.manifest_sha256,
        "completion_sha256":receipt.completion.artifact.sha256,"objects_downloaded_and_verified":receipt.objects.len()+1,
        "remote_rechecked":true,"native_replay_performed":true,"publication_confirmations":confirmations,
        "restore_aggregate_byte_limit":RESTORE_MAX_BYTES,"parsed_json_byte_limit":super::JSON_INPUT_MAX_BYTES,
        "source_binary_required_for_read_replay":false,"run_binding_preserved":true,"attempt_directory":attempt});
    storage::write_json(
        &attempt,
        "restored.json",
        "restore-verification-receipt",
        &report,
    )?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct Fixture {
        root: PathBuf,
        plan: super::super::DatasetPlan,
        binding: Value,
        protocol: Vec<u8>,
    }

    impl Fixture {
        fn new(memory_mib: u64) -> Self {
            let root = std::env::temp_dir().join(format!(
                "crypto-dataset-restore-test-{}-{}",
                std::process::id(),
                super::super::unix_nanos().unwrap()
            ));
            fs::create_dir(&root).unwrap();
            let mut protocol = json!({"schema":"experiment.protocol/v1","purpose":"restore unit correctness fixture"});
            let protocol_hash =
                super::super::digest(&super::super::protocol_canonical_bytes(&protocol).unwrap());
            protocol["freeze"] = json!({"status":"frozen","protocol_sha256":protocol_hash,
                "filed_before_affected_data":true,"frozen_at":"2026-10-28T00:00:00Z"});
            let protocol = serde_json::to_vec(&protocol).unwrap();
            let plan = super::super::DatasetPlan {
                schema: "crypto.ml-dataset.plan/v1".into(),
                field_sizes: vec![7],
                arities: vec![2],
                families: vec!["koblitz".into()],
                solvers: vec!["bsgs".into()],
                curve_seeds: 1,
                generators: 1,
                keys: 1,
                seed: 19,
                factor_base_size: 4,
                degree_bound: 3,
                work_limit: 10000,
                twists: true,
                protocol_sha256: Some(super::super::digest(&protocol)),
                split_rule: "curve-twist-component-sha256-v1".into(),
            };
            let binding = json!({"schema":"crypto.ml-dataset.run/v1",
                "plan_sha256":super::super::digest(&serde_json::to_vec(&plan).unwrap()),
                "source_commit":"a".repeat(40),"binary_sha256":"b".repeat(64),
                "shard_index":0,"shard_count":1,"seconds_per_case":10,
                "memory_mib":memory_mib,"trace_mib":64,"clock_valid_for_comparison":false});
            Self {
                root,
                plan,
                binding,
                protocol,
            }
        }

        fn publish(&self, cell_index: u64) -> (PathBuf, PublishReceipt) {
            let dir = self.root.join(format!("source-{cell_index}"));
            fs::create_dir(&dir).unwrap();
            let config = self.plan.cell(cell_index).unwrap();
            let case = super::super::curves::generate(&config).unwrap();
            let result = super::super::solvers::solve(&config, &case).unwrap();
            assert_eq!(result.outcome["scalar_recovered_without_witness"], true);
            let mut artifacts = vec![
                storage::write_json(&dir, "plan.json", "restricted-frozen-plan", &self.plan)
                    .unwrap(),
                storage::write_json(&dir, "run.json", "restricted-run-binding", &self.binding)
                    .unwrap(),
                storage::write_bytes(
                    &dir,
                    "protocol.json",
                    "restricted-frozen-protocol",
                    "json",
                    &self.protocol,
                )
                .unwrap(),
                storage::write_json(&dir, "config.json", "restricted-config", &config).unwrap(),
                storage::write_json(
                    &dir,
                    "public.json",
                    "public-instance",
                    &case.public_instance,
                )
                .unwrap(),
                storage::write_json(
                    &dir,
                    "witness.json",
                    "restricted-known-answer",
                    &case.private_witness,
                )
                .unwrap(),
                storage::write_json(&dir, "factor-base.json", "factor-base", &case.factor_base)
                    .unwrap(),
            ];
            for path in ["attempt.json", "worker-process.json"] {
                artifacts.push(
                    storage::write_json(
                        &dir,
                        path,
                        "restricted-provenance",
                        &json!({"unit_test":true}),
                    )
                    .unwrap(),
                );
            }
            for path in ["stdout.log", "stderr.log"] {
                artifacts.push(
                    storage::write_bytes(&dir, path, "log", "utf8", b"unit fixture\n").unwrap(),
                );
            }
            for artifact in result.artifacts {
                artifacts.push(
                    storage::write_bytes(
                        &dir,
                        &artifact.path,
                        &artifact.role,
                        &artifact.encoding,
                        &artifact.bytes,
                    )
                    .unwrap(),
                );
            }
            let mut outcome = result.outcome;
            outcome["known_answer_verified"] = json!(true);
            outcome["factor_base_verified"] = json!(true);
            let manifest = super::super::CellManifest {
                schema: "crypto.ml-dataset.cell/v1".into(),
                cell_index,
                plan_sha256: self.binding["plan_sha256"].as_str().unwrap().into(),
                config,
                outcome,
                artifacts,
            };
            let artifact =
                storage::write_json(&dir, "manifest.json", "cell-manifest", &manifest).unwrap();
            super::super::verify_cell(&dir, true).unwrap();
            let cfg = Config {
                index_path: self.root.join(format!("index-{cell_index}.json")),
                cache_dir: self.root.join("cache"),
                remote: Some(RemoteSpec::Dir(self.root.join("remote"))),
                endpoint: None,
                aws: "aws".into(),
            };
            let receipt = storage::publish(
                &dir,
                &manifest.artifacts,
                &artifact,
                cfg,
                &super::super::publication_namespace(&self.binding, cell_index).unwrap(),
            )
            .unwrap();
            let receipt_path = self.root.join(format!("receipt-{cell_index}.json"));
            fs::write(&receipt_path, serde_json::to_vec_pretty(&receipt).unwrap()).unwrap();
            (receipt_path, receipt)
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn native_budgeted_restore_retains_failed_attempt_and_retries_with_read_debits() {
        let fixture = Fixture::new(256);
        fixture.publish(0);
        let source = fixture.root.join("source-0");
        let manifest: super::super::CellManifest =
            super::super::read_json(&source.join("manifest.json")).unwrap();
        let (sha256, bytes) = crate::store::hash_file(&source.join("manifest.json")).unwrap();
        let artifact = Artifact {
            path: "manifest.json".into(),
            role: "cell-manifest".into(),
            encoding: "json".into(),
            sha256,
            bytes,
        };
        let journal = fixture.root.join("budget");
        fs::create_dir(&journal).unwrap();
        let binding = super::super::budget::RunBinding {
            source_commit: "c".repeat(40),
            binary_sha256: "d".repeat(64),
            plan_sha256: fixture.binding["plan_sha256"].as_str().unwrap().into(),
            protocol_sha256: fixture.plan.protocol_sha256.clone().unwrap(),
            shard_index: 0,
            shard_count: 1,
        };
        let (context, wire) = super::super::budget_test_support::context(&journal, Some(binding));
        let remote = RemoteSpec::parse("s3://setec-astronomy-artifacts/ml-dataset/test").unwrap();
        let cfg = Config {
            index_path: fixture.root.join("native-index.json"),
            cache_dir: fixture.root.join("native-cache"),
            remote: Some(remote),
            endpoint: None,
            aws: "aws".into(),
        };
        let receipt = storage::publish_budgeted(
            &source,
            &manifest.artifacts,
            &artifact,
            cfg,
            &super::super::publication_namespace(&fixture.binding, 0).unwrap(),
            Some(&context),
            BudgetScope::Work,
        )
        .unwrap();
        let receipt_path = fixture.root.join("native-receipt.json");
        fs::write(&receipt_path, serde_json::to_vec(&receipt).unwrap()).unwrap();
        let out = fixture.root.join("native-restored");
        let before = context.session().snapshot().unwrap();
        *wire.fail_next_get.lock().unwrap() = true;
        assert!(restore_budgeted(&receipt_path, &out, None, Some(&context)).is_err());
        assert!(!out.join("cells/000000000000/manifest.json").exists());
        let attempts = out
            .join(".restore")
            .join(&receipt.completion.artifact.sha256);
        assert_eq!(fs::read_dir(&attempts).unwrap().count(), 1);
        let failed = context.session().snapshot().unwrap();
        assert!(
            failed["get_attempts"].as_u64().unwrap() > before["get_attempts"].as_u64().unwrap()
        );
        let restored = restore_budgeted(&receipt_path, &out, None, Some(&context)).unwrap();
        assert_eq!(restored["native_replay_performed"], true);
        assert_eq!(fs::read_dir(&attempts).unwrap().count(), 2);
        assert_eq!(
            super::super::read_json::<Value>(&out.join("run.json")).unwrap(),
            fixture.binding
        );
        let after = context.session().snapshot().unwrap();
        assert_eq!(after["put_attempts"], before["put_attempts"]);
        assert_eq!(
            after["reserved_unique_output_bytes"],
            before["reserved_unique_output_bytes"]
        );
        assert!(after["get_attempts"].as_u64().unwrap() > failed["get_attempts"].as_u64().unwrap());
        assert_eq!(
            after["charged_attempts"].as_u64().unwrap(),
            wire.calls.lock().unwrap().len() as u64
        );
    }

    #[test]
    fn cross_host_roundtrip_replays_solved_cells_and_preserves_binding() {
        let fixture = Fixture::new(256);
        let (path, receipt) = fixture.publish(0);
        let out = fixture.root.join("restored");
        let first = restore(&path, &out, None).unwrap();
        assert_eq!(first["already_present"], false);
        assert_eq!(first["remote_rechecked"], true);
        assert_eq!(first["publication_confirmations"], 1);
        // A new host discovers this portable receipt from the remote catalog;
        // it has neither the producer's local index path nor its self locator.
        let catalog_path = fixture
            .root
            .join("remote")
            .join(&receipt.catalog.as_ref().expect("published catalog").key);
        let portable: PublishReceipt = super::super::read_json(&catalog_path).unwrap();
        assert!(portable.index_path.as_os_str().is_empty());
        assert!(portable.catalog.is_none());
        let catalog_restore =
            restore(&catalog_path, &fixture.root.join("from-catalog"), None).unwrap();
        assert_eq!(catalog_restore["native_replay_performed"], true);
        assert_eq!(catalog_restore["manifest_sha256"], receipt.manifest_sha256);
        assert_eq!(
            super::super::read_json::<Value>(&out.join("run.json")).unwrap(),
            fixture.binding
        );
        let second = restore(&path, &out, Some(&receipt.remote)).unwrap();
        assert_eq!(second["already_present"], true);
        assert_ne!(first["attempt_directory"], second["attempt_directory"]);
        let (twist_path, _) = fixture.publish(1);
        restore(&twist_path, &out, None).unwrap();
        let status = super::super::verify_run(&out, true).unwrap();
        assert_eq!(status["completed_receipts"], 2);
        assert_eq!(status["published_cells"], 2);
        assert_eq!(status["independently_solved_ecdlp_cells"], 2);
        // The synthetic source/binary digests intentionally differ from this
        // test executable: read-only replay must not require the old binary.
        assert_eq!(status["shard_attempts_complete"], true);
    }

    #[test]
    fn remote_corruption_retains_attempt_without_activating_manifest_then_retry_succeeds() {
        let fixture = Fixture::new(256);
        let (path, receipt) = fixture.publish(0);
        let object = receipt
            .objects
            .iter()
            .find(|o| o.artifact.path == "stdout.log")
            .unwrap();
        let remote = fixture.root.join("remote").join(&object.object_key);
        let original = fs::read(&remote).unwrap();
        fs::write(&remote, vec![b'x'; original.len()]).unwrap();
        let out = fixture.root.join("retry");
        let error = restore(&path, &out, None).unwrap_err();
        assert!(
            error.contains("remote restore verification failed"),
            "{error}"
        );
        assert!(!out.join("cells/000000000000/manifest.json").exists());
        let attempts = out
            .join(".restore")
            .join(&receipt.completion.artifact.sha256);
        assert_eq!(fs::read_dir(&attempts).unwrap().count(), 1);
        fs::write(&remote, original).unwrap();
        restore(&path, &out, None).unwrap();
        assert_eq!(fs::read_dir(&attempts).unwrap().count(), 2);
    }

    #[test]
    fn rejects_receipt_substitution_disallowed_remote_and_different_run_caps() {
        let fixture = Fixture::new(256);
        let (path, receipt) = fixture.publish(0);
        let out = fixture.root.join("restored");
        restore(&path, &out, None).unwrap();
        assert!(restore(&path, &out, Some("s3://other-bucket"))
            .unwrap_err()
            .contains("only read S3"));
        assert!(
            restore(&path, &out, Some("s3://setec-astronomy-artifacts/another"))
                .unwrap_err()
                .contains("must match")
        );
        let mut changed = receipt.clone();
        changed.objects[0].artifact.path = "../outside".into();
        changed.objects[0].name = format!("{}/../outside", changed.namespace);
        fs::write(
            fixture.root.join("bad.json"),
            serde_json::to_vec(&changed).unwrap(),
        )
        .unwrap();
        assert!(restore(&fixture.root.join("bad.json"), &out, None).is_err());
        let conflicting = Fixture::new(512);
        let (conflict_path, _) = conflicting.publish(1);
        assert!(restore(&conflict_path, &out, None)
            .unwrap_err()
            .contains("caps conflict"));
        assert!(!out.join("cells/000000000001").exists());
        assert_eq!(
            super::super::read_json::<Value>(&out.join("run.json")).unwrap(),
            fixture.binding
        );
    }

    #[cfg(unix)]
    #[test]
    fn refuses_symlink_output_and_preserves_incomplete_local_attempt() {
        let fixture = Fixture::new(256);
        let (path, _) = fixture.publish(0);
        let target = fixture.root.join("target");
        fs::create_dir(&target).unwrap();
        let alias = fixture.root.join("alias");
        std::os::unix::fs::symlink(&target, &alias).unwrap();
        assert!(restore(&path, &alias, None)
            .unwrap_err()
            .contains("ordinary directory"));
        let out = fixture.root.join("incomplete");
        fs::create_dir_all(out.join("cells/000000000000")).unwrap();
        fs::write(out.join("cells/000000000000/original.bin"), b"preserve").unwrap();
        assert!(restore(&path, &out, None)
            .unwrap_err()
            .contains("incomplete local attempt"));
        assert_eq!(
            fs::read(out.join("cells/000000000000/original.bin")).unwrap(),
            b"preserve"
        );
        assert!(!out.join("cells/000000000000/manifest.json").exists());
    }

    #[test]
    fn rejects_oversized_receipts_parsed_artifacts_and_aggregate_before_downloads() {
        let fixture = Fixture::new(256);
        let oversized = fixture.root.join("oversized-receipt.json");
        File::create(&oversized)
            .unwrap()
            .set_len(super::super::JSON_INPUT_MAX_BYTES + 1)
            .unwrap();
        let out = fixture.root.join("oversized-output");
        assert!(restore(&oversized, &out, None)
            .unwrap_err()
            .contains("JSON input limit"));
        assert!(!out.exists());

        let (_, receipt) = fixture.publish(0);
        let mut oversized = receipt.clone();
        oversized
            .objects
            .iter_mut()
            .find(|o| o.artifact.path == "public.json")
            .unwrap()
            .artifact
            .bytes = super::super::JSON_INPUT_MAX_BYTES + 1;
        let path = fixture.root.join("oversized-public.json");
        fs::write(&path, serde_json::to_vec(&oversized).unwrap()).unwrap();
        assert!(restore(&path, &out, None)
            .unwrap_err()
            .contains("parsed artifact public.json"));
        assert!(!out.exists());

        oversized = receipt.clone();
        oversized.completion.artifact.bytes = super::super::JSON_INPUT_MAX_BYTES + 1;
        fs::write(&path, serde_json::to_vec(&oversized).unwrap()).unwrap();
        assert!(restore(&path, &out, None)
            .unwrap_err()
            .contains("completion receipt exceeds"));
        assert!(!out.exists());

        oversized = receipt;
        oversized
            .objects
            .iter_mut()
            .find(|o| o.artifact.path == "stdout.log")
            .unwrap()
            .artifact
            .bytes = RESTORE_MAX_BYTES;
        fs::write(&path, serde_json::to_vec(&oversized).unwrap()).unwrap();
        assert!(restore(&path, &out, None)
            .unwrap_err()
            .contains("aggregate budget"));
        assert!(!out.exists());
    }

    #[test]
    fn oversized_remote_object_is_rejected_before_download_and_existing_hash_audit_remains() {
        let fixture = Fixture::new(256);
        let (path, receipt) = fixture.publish(0);
        let object_index = receipt
            .objects
            .iter()
            .position(|o| o.artifact.path == "stdout.log")
            .unwrap();
        let remote = fixture
            .root
            .join("remote")
            .join(&receipt.objects[object_index].object_key);
        let original = fs::read(&remote).unwrap();
        fs::OpenOptions::new()
            .write(true)
            .open(&remote)
            .unwrap()
            .set_len(RESTORE_MAX_BYTES + 1)
            .unwrap();
        let out = fixture.root.join("oversized-remote-output");
        assert!(restore(&path, &out, None)
            .unwrap_err()
            .contains("remote restore size differs"));
        let attempts = out
            .join(".restore")
            .join(&receipt.completion.artifact.sha256);
        let attempt = fs::read_dir(attempts)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        assert!(!attempt
            .join("downloads")
            .join(format!("object-{:08}.download", object_index + 1))
            .exists());
        assert!(!out.join("cells/000000000000/manifest.json").exists());
        fs::write(&remote, original).unwrap();
        restore(&path, &out, None).unwrap();

        // Removing repeated arithmetic replay must retain the full local hash
        // audit when a later cell is appended to this run.
        fs::write(out.join("cells/000000000000/stdout.log"), b"tampered").unwrap();
        let (next, _) = fixture.publish(1);
        assert!(restore(&next, &out, None)
            .unwrap_err()
            .contains("artifact mismatch"));
        assert!(!out.join("cells/000000000001/manifest.json").exists());
    }

    #[test]
    fn hash_consistent_forged_scalar_is_rejected_by_native_replay() {
        let fixture = Fixture::new(256);
        fixture.publish(0);
        let source = fixture.root.join("source-0");
        // Simulate a corrupt producer that publishes internally consistent
        // hashes for a false known answer. CAS integrity alone is insufficient.
        let mut witness: Value = super::super::read_json(&source.join("witness.json")).unwrap();
        witness["scalar"] = json!("0");
        let witness_bytes = canonical_json(&witness).unwrap();
        fs::write(source.join("witness.json"), &witness_bytes).unwrap();
        let mut manifest: super::super::CellManifest =
            super::super::read_json(&source.join("manifest.json")).unwrap();
        let record = manifest
            .artifacts
            .iter_mut()
            .find(|a| a.path == "witness.json")
            .unwrap();
        record.sha256 = super::super::digest(&witness_bytes);
        record.bytes = witness_bytes.len() as u64;
        let bytes = canonical_json(&manifest).unwrap();
        fs::write(source.join("manifest.json"), &bytes).unwrap();
        let manifest_artifact = Artifact {
            path: "manifest.json".into(),
            role: "cell-manifest".into(),
            encoding: "json".into(),
            sha256: super::super::digest(&bytes),
            bytes: bytes.len() as u64,
        };
        let cfg = Config {
            index_path: fixture.root.join("forged-index.json"),
            cache_dir: fixture.root.join("cache"),
            remote: Some(RemoteSpec::Dir(fixture.root.join("remote"))),
            endpoint: None,
            aws: "aws".into(),
        };
        let receipt = storage::publish(
            &source,
            &manifest.artifacts,
            &manifest_artifact,
            cfg,
            &super::super::publication_namespace(&fixture.binding, 0).unwrap(),
        )
        .unwrap();
        let path = fixture.root.join("forged-receipt.json");
        fs::write(&path, serde_json::to_vec(&receipt).unwrap()).unwrap();
        let out = fixture.root.join("forged-restored");
        let error = restore(&path, &out, None).unwrap_err();
        assert!(
            !error.contains("artifact mismatch"),
            "hashes should pass and native replay should fail: {error}"
        );
        assert!(!out.join("cells/000000000000/manifest.json").exists());
    }
}
