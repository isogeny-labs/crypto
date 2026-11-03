//! Restricted, chunked, content-addressed local custody for an entire run.
//! No cloud transport is implied by a locally verified bundle.

use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::budget::BudgetScope;
use super::s3_budget::S3BudgetContext;
use super::storage::{self, PublishReceipt};
use super::types::Artifact;
use super::BudgetArgs;
use crate::store::backend::{object_key as remote_object_key, RemoteSpec};

const CHUNK_BYTES: usize = 4 * 1024 * 1024;
const DEFAULT_MAX_BYTES: u64 = 1024 * 1024 * 1024 * 1024;
const MAX_REMOTE_OBJECTS: usize = 1_000_002;
const DEFAULT_MAX_FILES: usize = 1_000_000;

#[derive(Args)]
pub struct CustodyArgs {
    #[command(subcommand)]
    pub command: CustodyCommand,
}

#[derive(Subcommand)]
pub enum CustodyCommand {
    /// Snapshot a quiescent run and optional report/export directories.
    Create {
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        bundle: PathBuf,
        /// Extra directory in the form LABEL=PATH; appears under extra/LABEL.
        #[arg(long = "include")]
        includes: Vec<String>,
        #[arg(long, default_value_t = DEFAULT_MAX_BYTES)]
        max_bytes: u64,
        #[arg(long, default_value_t = DEFAULT_MAX_FILES)]
        max_files: usize,
    },
    /// Rehash the manifest, every chunk and every reconstructed file.
    Verify {
        #[arg(long)]
        bundle: PathBuf,
    },
    /// Materialize a verified bundle into a fresh directory and replay the run.
    Restore {
        #[arg(long)]
        bundle: PathBuf,
        #[arg(long)]
        destination: PathBuf,
    },
    /// Publish a verified local bundle with an active grant-bound S3 budget.
    Publish {
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        bundle: PathBuf,
        #[arg(long)]
        remote: String,
        #[command(flatten)]
        budget: BudgetArgs,
    },
    /// Fetch every exact remote object and replay a fresh restored run.
    RestoreRemote {
        #[arg(long)]
        receipt: PathBuf,
        #[arg(long)]
        bundle: PathBuf,
        #[arg(long)]
        destination: PathBuf,
        #[command(flatten)]
        budget: BudgetArgs,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Member {
    path: String,
    bytes: u64,
    sha256: String,
    chunks: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: String,
    chunk_bytes: usize,
    plan_file_sha256: String,
    protocol_file_sha256: String,
    run_file_sha256: String,
    source_commit: String,
    binary_sha256: String,
    known_answer_replay_performed: bool,
    members: Vec<Member>,
}

fn safe_name(name: &str) -> Result<(), String> {
    if name.is_empty()
        || name.contains('\\')
        || name.bytes().any(|b| b.is_ascii_control())
        || name
            .split('/')
            .any(|p| p.is_empty() || p == "." || p == "..")
        || Path::new(name)
            .components()
            .any(|p| !matches!(p, std::path::Component::Normal(_)))
    {
        return Err(format!("unsafe custody path {name:?}"));
    }
    Ok(())
}

fn sha(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn object_path(digest: &str) -> Result<String, String> {
    crate::store::index::validate_sha256(digest)?;
    Ok(format!("objects/sha256/{digest}"))
}

fn collect(
    root: &Path,
    prefix: &str,
    bundle: &Path,
    members: &mut Vec<Member>,
    total: &mut u64,
    max_bytes: u64,
    max_files: usize,
) -> Result<(), String> {
    let mut children = fs::read_dir(root)
        .map_err(|e| format!("{}: {e}", root.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    children.sort_by_key(|e| e.file_name());
    for child in children {
        let name = child
            .file_name()
            .into_string()
            .map_err(|_| "custody requires UTF-8 path names")?;
        safe_name(&name)?;
        let relative = format!("{prefix}/{name}");
        safe_name(&relative)?;
        let path = child.path();
        let meta = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
        if meta.is_dir() && !meta.file_type().is_symlink() {
            collect(
                &path, &relative, bundle, members, total, max_bytes, max_files,
            )?;
            continue;
        }
        if !meta.is_file() || meta.file_type().is_symlink() {
            return Err(format!(
                "custody refuses nonregular member {}",
                path.display()
            ));
        }
        if members.len() >= max_files {
            return Err("custody file cap exceeded".into());
        }
        *total = total
            .checked_add(meta.len())
            .ok_or("custody byte count overflow")?;
        if *total > max_bytes {
            return Err("custody byte cap exceeded".into());
        }
        let mut input = File::open(&path).map_err(|e| e.to_string())?;
        let mut file_hash = Sha256::new();
        let mut chunks = Vec::new();
        let mut read_total = 0u64;
        loop {
            let mut buf = vec![0u8; CHUNK_BYTES];
            let mut n = 0;
            while n < CHUNK_BYTES {
                let read = input.read(&mut buf[n..]).map_err(|e| e.to_string())?;
                if read == 0 {
                    break;
                }
                n += read;
            }
            if n == 0 {
                break;
            }
            buf.truncate(n);
            read_total += n as u64;
            if read_total > meta.len() {
                return Err(format!("member grew during capture: {}", path.display()));
            }
            file_hash.update(&buf);
            let digest = sha(&buf);
            let object = object_path(&digest)?;
            storage::write_bytes(bundle, &object, "restricted-custody-chunk", "binary", &buf)?;
            chunks.push(digest);
        }
        if read_total != meta.len() {
            return Err(format!("member changed during capture: {}", path.display()));
        }
        let final_hash = hex::encode(file_hash.finalize());
        if crate::store::hash_file(&path)?.0 != final_hash {
            return Err(format!("member changed during capture: {}", path.display()));
        }
        members.push(Member {
            path: relative,
            bytes: read_total,
            sha256: final_hash,
            chunks,
        });
    }
    Ok(())
}

fn read_manifest(bundle: &Path) -> Result<Manifest, String> {
    let path = storage::contained_path(bundle, "manifest.json", false)?;
    let meta = fs::metadata(&path).map_err(|e| e.to_string())?;
    if meta.len() > 256 * 1024 * 1024 {
        return Err("custody manifest exceeds 256 MiB".into());
    }
    serde_json::from_reader(File::open(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

fn check_manifest(m: &Manifest) -> Result<(), String> {
    if m.schema != "crypto.ml-dataset.restricted-custody/v1" || m.chunk_bytes != CHUNK_BYTES {
        return Err("unsupported custody manifest".into());
    }
    for hash in [
        &m.plan_file_sha256,
        &m.protocol_file_sha256,
        &m.run_file_sha256,
        &m.binary_sha256,
    ] {
        crate::store::index::validate_sha256(hash)?;
    }
    if m.source_commit.len() != 40 || !m.source_commit.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("invalid custody source commit".into());
    }
    let mut seen = BTreeSet::new();
    for member in &m.members {
        safe_name(&member.path)?;
        if !(member.path.starts_with("run/") || member.path.starts_with("extra/"))
            || !seen.insert(&member.path)
        {
            return Err("duplicate or unscoped custody member".into());
        }
        crate::store::index::validate_sha256(&member.sha256)?;
        let expected = member.bytes.div_ceil(CHUNK_BYTES as u64);
        if member.chunks.len() as u64 != expected {
            return Err(format!("wrong chunk count for {}", member.path));
        }
        for hash in &member.chunks {
            crate::store::index::validate_sha256(hash)?;
        }
    }
    for (path, hash) in [
        ("run/plan.json", &m.plan_file_sha256),
        ("run/protocol.json", &m.protocol_file_sha256),
        ("run/run.json", &m.run_file_sha256),
    ] {
        if seen.iter().filter(|p| p.as_str() == path).count() != 1
            || m.members
                .iter()
                .find(|entry| entry.path == path)
                .map(|v| &v.sha256)
                != Some(hash)
        {
            return Err(format!("custody missing or changed binding {path}"));
        }
    }
    Ok(())
}

fn replay_members(
    bundle: &Path,
    manifest: &Manifest,
    destination: Option<&Path>,
) -> Result<u64, String> {
    check_manifest(manifest)?;
    let mut total = 0u64;
    for member in &manifest.members {
        total = total
            .checked_add(member.bytes)
            .ok_or("custody total overflow")?;
        let mut file_hash = Sha256::new();
        let mut written = 0u64;
        let mut output = if let Some(root) = destination {
            let target = storage::contained_path(root, &member.path, true)?;
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            Some(options.open(target).map_err(|e| e.to_string())?)
        } else {
            None
        };
        for (index, hash) in member.chunks.iter().enumerate() {
            let object = storage::contained_path(bundle, &object_path(hash)?, false)?;
            let mut file = File::open(&object).map_err(|e| e.to_string())?;
            if file.metadata().map_err(|e| e.to_string())?.len() > CHUNK_BYTES as u64 {
                return Err(format!("oversized custody chunk {hash}"));
            }
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
            let remaining = member.bytes - written;
            let expected = if index + 1 == member.chunks.len() {
                remaining
            } else {
                CHUNK_BYTES as u64
            };
            if bytes.len() as u64 != expected || sha(&bytes) != *hash {
                return Err(format!("custody chunk mismatch {hash}"));
            }
            file_hash.update(&bytes);
            if let Some(out) = &mut output {
                out.write_all(&bytes).map_err(|e| e.to_string())?;
            }
            written += bytes.len() as u64;
        }
        if written != member.bytes || hex::encode(file_hash.finalize()) != member.sha256 {
            return Err(format!("custody file mismatch {}", member.path));
        }
        if let Some(out) = output {
            out.sync_all().map_err(|e| e.to_string())?;
        }
    }
    Ok(total)
}

fn bundle_objects(bundle: &Path, manifest: &Manifest) -> Result<Vec<Artifact>, String> {
    check_manifest(manifest)?;
    let mut chunks = BTreeMap::<String, u64>::new();
    for member in &manifest.members {
        for (index, digest) in member.chunks.iter().enumerate() {
            let size = if index + 1 == member.chunks.len() {
                member.bytes - CHUNK_BYTES as u64 * index as u64
            } else {
                CHUNK_BYTES as u64
            };
            if let Some(previous) = chunks.insert(digest.clone(), size) {
                if previous != size {
                    return Err(format!("custody chunk {digest} has conflicting lengths"));
                }
            }
        }
    }
    let mut artifacts = Vec::with_capacity(chunks.len() + 1);
    for (digest, bytes) in chunks {
        let path = object_path(&digest)?;
        storage::check_file(
            &storage::contained_path(bundle, &path, false)?,
            &digest,
            bytes,
        )?;
        artifacts.push(Artifact {
            role: "restricted-full-run-custody-chunk".into(),
            path,
            sha256: digest,
            bytes,
            encoding: "binary".into(),
        });
    }
    let (sha256, bytes) = crate::store::hash_file(&bundle.join("manifest.json"))?;
    artifacts.push(Artifact {
        role: "restricted-full-run-custody-manifest".into(),
        path: "manifest.json".into(),
        sha256,
        bytes,
        encoding: "json".into(),
    });
    Ok(artifacts)
}

fn approved_remote(uri: &str) -> Result<RemoteSpec, String> {
    let remote = RemoteSpec::parse(uri)?;
    if !matches!(&remote, RemoteSpec::S3 { bucket, prefix }
        if bucket == "setec-astronomy-artifacts" && !prefix.is_empty())
    {
        return Err("custody requires an approved nonempty S3 prefix".into());
    }
    Ok(remote)
}

fn source_files(
    root: &Path,
    prefix: &str,
    files: &mut BTreeMap<String, (String, u64)>,
) -> Result<(), String> {
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| "custody requires UTF-8 path names")?;
        safe_name(&name)?;
        let relative = format!("{prefix}/{name}");
        let path = entry.path();
        let meta = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
        if meta.is_dir() && !meta.file_type().is_symlink() {
            source_files(&path, &relative, files)?;
        } else if meta.is_file() && !meta.file_type().is_symlink() {
            files.insert(relative, crate::store::hash_file(&path)?);
        } else {
            return Err(format!(
                "custody refuses nonregular source member {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn compare_source_run(root: &Path, manifest: &Manifest) -> Result<(), String> {
    let mut actual = BTreeMap::new();
    source_files(root, "run", &mut actual)?;
    let expected: BTreeMap<_, _> = manifest
        .members
        .iter()
        .filter(|m| m.path.starts_with("run/"))
        .map(|m| (m.path.clone(), (m.sha256.clone(), m.bytes)))
        .collect();
    if actual != expected {
        return Err("custody bundled run differs from locked source run".into());
    }
    Ok(())
}

fn verify_custody_run_identity(root: &Path, manifest: &Manifest) -> Result<(), String> {
    let (_, binding, _) = super::bound_run(root)?;
    for (name, expected) in [
        ("plan.json", &manifest.plan_file_sha256),
        ("protocol.json", &manifest.protocol_file_sha256),
        ("run.json", &manifest.run_file_sha256),
    ] {
        if crate::store::hash_file(&root.join(name))?.0 != *expected {
            return Err(format!("custody {name} differs from bound run"));
        }
    }
    if binding["source_commit"].as_str() != Some(manifest.source_commit.as_str())
        || binding["binary_sha256"].as_str() != Some(manifest.binary_sha256.as_str())
    {
        return Err("custody producer differs from bound run".into());
    }
    Ok(())
}

fn publish_bundle(
    out: &Path,
    bundle: &Path,
    remote: &RemoteSpec,
    context: &Arc<S3BudgetContext>,
) -> Result<Value, String> {
    let root = super::checked_directory(out, false)?;
    let bundle = super::checked_directory(bundle, false)?;
    let _lease = super::Lease::acquire(&root)?;
    #[cfg(target_os = "linux")]
    super::assert_no_workers(&root)?;
    let manifest = read_manifest(&bundle)?;
    replay_members(&bundle, &manifest, None)?;
    compare_source_run(&root, &manifest)?;
    verify_custody_run_identity(&root, &manifest)?;
    super::verify_run(&root, true)?;
    let objects = bundle_objects(&bundle, &manifest)?;
    let custody_manifest = objects
        .iter()
        .find(|a| a.path == "manifest.json")
        .ok_or("custody manifest missing")?;
    let binding = storage::write_json(
        &bundle,
        "publication/custody-objects.json",
        "restricted-custody-publication-binding",
        &json!({"schema":"crypto.ml-dataset.custody-publication/v1",
            "custody_manifest_sha256":custody_manifest.sha256,
            "artifacts":objects}),
    )?;
    let mut cfg = crate::store::Config::discover()?;
    cfg.remote = Some(remote.clone());
    cfg.index_path = bundle.join("store-index.json");
    cfg.cache_dir = bundle.join("store-cache");
    let namespace = format!("ml-dataset/custody/{}", custody_manifest.sha256);
    let receipt = storage::publish_budgeted(
        &bundle,
        &objects,
        &binding,
        cfg,
        &namespace,
        Some(context),
        BudgetScope::Work,
    )?;
    let receipt_path = format!(
        "publication/full-run-{}-{}.json",
        sha(remote.display().as_bytes()),
        custody_manifest.sha256
    );
    let local_receipt = storage::write_json(
        &bundle,
        &receipt_path,
        "restricted-custody-publication-receipt",
        &receipt,
    )?;
    Ok(
        json!({"schema":"crypto.ml-dataset.custody-remote-publication/v1",
        "remote":remote.display(),"custody_manifest_sha256":custody_manifest.sha256,
        "custody_manifest_bytes":custody_manifest.bytes,"objects":objects.len()+1,
        "receipt":local_receipt,"publication":receipt,"all_remote_objects_read_back":true}),
    )
}

fn download_exact(
    bundle: &Path,
    remote: &RemoteSpec,
    context: &Arc<S3BudgetContext>,
    artifact: &Artifact,
) -> Result<(), String> {
    crate::store::index::validate_sha256(&artifact.sha256)?;
    let key = remote_object_key(&artifact.sha256)?;
    storage::expect_remote_object(Some(context), remote, &key, artifact, BudgetScope::Work)?;
    let mut cfg = crate::store::Config::discover()?;
    cfg.remote = Some(remote.clone());
    cfg.index_path = bundle.join("readback-index.json");
    cfg.cache_dir = bundle.join("readback-cache");
    let store = storage::dataset_store(cfg, Some(context), BudgetScope::Work)?;
    let backend = store.remote()?;
    let meta = backend
        .head(&key)?
        .ok_or_else(|| format!("remote custody object absent: {key}"))?;
    if meta.bytes != artifact.bytes || meta.sha256.as_ref().is_some_and(|v| v != &artifact.sha256) {
        return Err(format!("remote custody HEAD mismatch: {key}"));
    }
    let target = storage::contained_path(bundle, &artifact.path, true)?;
    let temporary = target.with_extension(format!("remote-{}", std::process::id()));
    if temporary.exists() || target.exists() {
        return Err(format!(
            "remote custody destination exists: {}",
            target.display()
        ));
    }
    backend.get(&key, &temporary)?;
    storage::check_file(&temporary, &artifact.sha256, artifact.bytes)?;
    File::open(&temporary)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())?;
    fs::hard_link(&temporary, &target).map_err(|e| e.to_string())?;
    storage::sync_directory(target.parent().ok_or("missing custody object parent")?)?;
    fs::remove_file(&temporary).map_err(|e| e.to_string())?;
    Ok(())
}

fn verify_receipt_records(
    bundle: &Path,
    remote: &RemoteSpec,
    context: &Arc<S3BudgetContext>,
    receipt: &PublishReceipt,
) -> Result<(), String> {
    let completion = &receipt.completion;
    if !completion.roundtrip_verified
        || !completion.artifact.path.starts_with("publication/")
        || !completion.artifact.path.ends_with(".json")
        || completion.object_key != remote_object_key(&completion.artifact.sha256)?
    {
        return Err("invalid custody completion locator".into());
    }
    // Publication names the completion by the serialized record digest.
    // Validate the safe path and its downloaded content.
    safe_name(&completion.artifact.path)?;
    download_exact(bundle, remote, context, &completion.artifact)?;
    let completion_record: Value = serde_json::from_reader(
        File::open(bundle.join(&completion.artifact.path)).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if completion_record
        != json!({"schema":receipt.schema,"remote":receipt.remote,
        "namespace":receipt.namespace,"manifest_sha256":receipt.manifest_sha256,
        "objects":receipt.objects})
    {
        return Err("remote completion does not bind custody receipt".into());
    }
    let catalog = receipt.catalog.as_ref().ok_or("custody catalog missing")?;
    if !catalog.roundtrip_verified
        || catalog.key
            != format!(
                "ml-dataset/catalog/{}/{}.json",
                receipt.namespace, receipt.manifest_sha256
            )
    {
        return Err("invalid custody catalog locator".into());
    }
    let artifact = Artifact {
        role: "publication_portable_receipt".into(),
        path: "publication/remote-catalog.json".into(),
        sha256: catalog.sha256.clone(),
        bytes: catalog.bytes,
        encoding: "json".into(),
    };
    storage::expect_remote_object(
        Some(context),
        remote,
        &catalog.key,
        &artifact,
        BudgetScope::Work,
    )?;
    let mut cfg = crate::store::Config::discover()?;
    cfg.remote = Some(remote.clone());
    cfg.index_path = bundle.join("readback-index.json");
    cfg.cache_dir = bundle.join("readback-cache");
    let store = storage::dataset_store(cfg, Some(context), BudgetScope::Work)?;
    let backend = store.remote()?;
    let meta = backend
        .head(&catalog.key)?
        .ok_or("remote custody catalog absent")?;
    if meta.bytes != catalog.bytes || meta.sha256.as_ref().is_some_and(|v| v != &catalog.sha256) {
        return Err("remote custody catalog HEAD mismatch".into());
    }
    let path = storage::contained_path(bundle, &artifact.path, true)?;
    backend.get(&catalog.key, &path)?;
    storage::check_file(&path, &catalog.sha256, catalog.bytes)?;
    let portable: Value = serde_json::from_reader(File::open(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let mut expected = receipt.clone();
    expected.catalog = None;
    if portable != serde_json::to_value(&expected).map_err(|e| e.to_string())? {
        return Err("remote catalog does not bind custody receipt".into());
    }
    Ok(())
}

fn restore_local(bundle: &Path, destination: &Path) -> Result<Value, String> {
    let bundle = super::checked_directory(bundle, false)?;
    let manifest = read_manifest(&bundle)?;
    replay_members(&bundle, &manifest, None)?;
    let parent = super::checked_directory(destination.parent().unwrap_or(Path::new(".")), false)?;
    let target = parent.join(
        destination
            .file_name()
            .ok_or("invalid restore destination")?,
    );
    if target.starts_with(&bundle) || bundle.starts_with(&target) {
        return Err("custody restore destination must be separate from bundle".into());
    }
    if destination.exists() {
        return Err("custody restore destination already exists".into());
    }
    fs::create_dir(destination).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(destination, fs::Permissions::from_mode(0o700))
            .map_err(|e| e.to_string())?;
    }
    let bytes = replay_members(&bundle, &manifest, Some(destination))?;
    verify_custody_run_identity(&destination.join("run"), &manifest)?;
    let replay = super::verify_run(&destination.join("run"), true)?;
    Ok(
        json!({"schema":"crypto.ml-dataset.restricted-custody-restore/v1",
        "destination":destination,"members":manifest.members.len(),"bytes":bytes,
        "native_run_replay":replay,"cloud_requests":0}),
    )
}

fn restore_remote(
    receipt_path: &Path,
    bundle: &Path,
    destination: &Path,
    context: &Arc<S3BudgetContext>,
) -> Result<Value, String> {
    let receipt: PublishReceipt =
        serde_json::from_reader(File::open(receipt_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if receipt.schema != "ml-dataset.publication/v1"
        || receipt.objects.len() < 2
        || receipt.objects.len() > MAX_REMOTE_OBJECTS
    {
        return Err("invalid custody publication receipt".into());
    }
    let remote = approved_remote(&receipt.remote)?;
    if bundle.exists() || destination.exists() {
        return Err("remote custody requires fresh bundle and restore destinations".into());
    }
    let mut seen = BTreeSet::new();
    let mut original_manifest = None;
    let mut binding_manifest = None;
    for object in &receipt.objects {
        if !object.roundtrip_verified
            || object.object_key != remote_object_key(&object.artifact.sha256)?
            || !seen.insert(&object.artifact.path)
            || object.artifact.bytes > DEFAULT_MAX_BYTES
        {
            return Err("invalid or duplicate custody publication object".into());
        }
        match object.artifact.path.as_str() {
            "manifest.json" => original_manifest = Some(&object.artifact),
            "publication/custody-objects.json" => binding_manifest = Some(&object.artifact),
            path if path.starts_with("objects/sha256/") => {
                if object_path(&object.artifact.sha256)? != path {
                    return Err("custody chunk path and digest differ".into());
                }
            }
            _ => return Err("unexpected custody publication path".into()),
        }
    }
    let original_manifest = original_manifest.ok_or("custody manifest missing from receipt")?;
    let binding_manifest = binding_manifest.ok_or("publication binding missing from receipt")?;
    if binding_manifest.sha256 != receipt.manifest_sha256 {
        return Err("publication binding digest differs from receipt".into());
    }
    if receipt.namespace != format!("ml-dataset/custody/{}", original_manifest.sha256) {
        return Err("publication namespace differs from custody manifest".into());
    }
    if receipt.completion.roundtrip_verified == false
        || receipt
            .catalog
            .as_ref()
            .is_none_or(|c| !c.roundtrip_verified)
    {
        return Err("custody publication lacks verified completion and catalog".into());
    }
    let parent = super::checked_directory(bundle.parent().unwrap_or(Path::new(".")), false)?;
    if parent.join(bundle.file_name().ok_or("invalid bundle path")?) == destination {
        return Err("bundle and restore destinations must differ".into());
    }
    fs::create_dir(bundle).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(bundle, fs::Permissions::from_mode(0o700))
            .map_err(|e| e.to_string())?;
    }
    // Fetch the binding and native manifest first, then every referenced chunk.
    download_exact(bundle, &remote, context, binding_manifest)?;
    download_exact(bundle, &remote, context, original_manifest)?;
    let binding: Value = serde_json::from_reader(
        File::open(bundle.join(&binding_manifest.path)).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let expected: Vec<Artifact> = serde_json::from_value(
        binding
            .get("artifacts")
            .cloned()
            .ok_or("publication artifact list missing")?,
    )
    .map_err(|e| e.to_string())?;
    let actual: Vec<Artifact> = receipt
        .objects
        .iter()
        .filter(|o| o.artifact.path != binding_manifest.path)
        .map(|o| o.artifact.clone())
        .collect();
    if binding["schema"] != "crypto.ml-dataset.custody-publication/v1"
        || binding["custody_manifest_sha256"].as_str() != Some(original_manifest.sha256.as_str())
        || expected != actual
    {
        return Err("publication binding does not match receipt objects".into());
    }
    let manifest = read_manifest(bundle)?;
    let expected_chunks: BTreeSet<String> = manifest
        .members
        .iter()
        .flat_map(|m| m.chunks.iter().cloned())
        .collect();
    let receipt_chunks: BTreeSet<String> = receipt
        .objects
        .iter()
        .filter(|o| o.artifact.path.starts_with("objects/sha256/"))
        .map(|o| o.artifact.sha256.clone())
        .collect();
    if expected_chunks != receipt_chunks {
        return Err("custody manifest chunks differ from publication receipt".into());
    }
    let provenance: BTreeSet<String> = manifest
        .members
        .iter()
        .filter(|m| {
            matches!(
                m.path.as_str(),
                "run/plan.json" | "run/protocol.json" | "run/run.json"
            )
        })
        .flat_map(|m| m.chunks.iter().cloned())
        .collect();
    for digest in &provenance {
        let object = receipt
            .objects
            .iter()
            .find(|o| o.artifact.path == object_path(digest).unwrap())
            .ok_or("custody provenance chunk absent from receipt")?;
        download_exact(bundle, &remote, context, &object.artifact)?;
    }
    let preflight = bundle.join("preflight");
    fs::create_dir(&preflight).map_err(|e| e.to_string())?;
    let preflight_manifest = Manifest {
        members: manifest
            .members
            .iter()
            .filter(|m| {
                matches!(
                    m.path.as_str(),
                    "run/plan.json" | "run/protocol.json" | "run/run.json"
                )
            })
            .cloned()
            .collect(),
        ..manifest.clone()
    };
    replay_members(bundle, &preflight_manifest, Some(&preflight))?;
    verify_custody_run_identity(&preflight.join("run"), &preflight_manifest)?;
    super::validate_budget_run(context.session(), &preflight.join("run"))?;
    fs::remove_dir_all(&preflight).map_err(|e| e.to_string())?;
    verify_receipt_records(bundle, &remote, context, &receipt)?;
    for object in &receipt.objects {
        if object.artifact.path.starts_with("objects/sha256/")
            && !provenance.contains(&object.artifact.sha256)
        {
            download_exact(bundle, &remote, context, &object.artifact)?;
        }
    }
    let replay = restore_local(bundle, destination)?;
    super::validate_budget_run(context.session(), &destination.join("run"))?;
    Ok(
        json!({"schema":"crypto.ml-dataset.custody-remote-restore/v1",
        "remote":remote.display(),"bundle":bundle,"custody_manifest_sha256":original_manifest.sha256,
        "objects_read_back":receipt.objects.len(),"native_restore":replay}),
    )
}

pub fn run(args: &CustodyArgs) -> Result<Value, String> {
    match &args.command {
        CustodyCommand::Create {
            out,
            bundle,
            includes,
            max_bytes,
            max_files,
        } => {
            if *max_bytes == 0 || *max_files == 0 {
                return Err("custody caps must be positive".into());
            }
            let root = super::checked_directory(out, false)?;
            let parent =
                super::checked_directory(bundle.parent().unwrap_or(Path::new(".")), false)?;
            if bundle.exists() {
                return Err("custody bundle destination already exists".into());
            }
            if parent.starts_with(&root) {
                return Err("custody bundle cannot be inside the run".into());
            }
            let bundle_absolute = parent.join(bundle.file_name().ok_or("invalid bundle name")?);
            let mut labels = BTreeSet::new();
            let mut checked_includes = Vec::new();
            for item in includes {
                let (label, path) = item
                    .split_once('=')
                    .ok_or("--include requires LABEL=PATH")?;
                safe_name(label)?;
                if label.contains('/') || !labels.insert(label) {
                    return Err("include labels must be unique single path components".into());
                }
                let include = super::checked_directory(Path::new(path), false)?;
                if include.starts_with(&root)
                    || root.starts_with(&include)
                    || include.starts_with(&bundle_absolute)
                    || bundle_absolute.starts_with(&include)
                {
                    return Err("include directory must be separate from run and bundle".into());
                }
                checked_includes.push((label.to_owned(), include));
            }
            let _lease = super::Lease::acquire(&root)?;
            #[cfg(target_os = "linux")]
            super::assert_no_workers(&root)?;
            let (_, binding, _) = super::bound_run(&root)?;
            super::verify_run(&root, true)?;
            fs::create_dir(bundle).map_err(|e| e.to_string())?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(bundle, fs::Permissions::from_mode(0o700))
                    .map_err(|e| e.to_string())?;
            }
            let mut members = Vec::new();
            let mut total = 0;
            collect(
                &root,
                "run",
                bundle,
                &mut members,
                &mut total,
                *max_bytes,
                *max_files,
            )?;
            for (label, include) in checked_includes {
                collect(
                    &include,
                    &format!("extra/{label}"),
                    bundle,
                    &mut members,
                    &mut total,
                    *max_bytes,
                    *max_files,
                )?;
            }
            members.sort_by(|a, b| a.path.cmp(&b.path));
            let manifest = Manifest {
                schema: "crypto.ml-dataset.restricted-custody/v1".into(),
                chunk_bytes: CHUNK_BYTES,
                plan_file_sha256: crate::store::hash_file(&root.join("plan.json"))?.0,
                protocol_file_sha256: crate::store::hash_file(&root.join("protocol.json"))?.0,
                run_file_sha256: crate::store::hash_file(&root.join("run.json"))?.0,
                source_commit: binding["source_commit"]
                    .as_str()
                    .ok_or("missing source commit")?
                    .into(),
                binary_sha256: binding["binary_sha256"]
                    .as_str()
                    .ok_or("missing binary hash")?
                    .into(),
                known_answer_replay_performed: true,
                members,
            };
            check_manifest(&manifest)?;
            let artifact = storage::write_json(
                bundle,
                "manifest.json",
                "restricted-full-run-custody-manifest",
                &manifest,
            )?;
            let bytes = replay_members(bundle, &manifest, None)?;
            storage::sync_directory(bundle)?;
            Ok(
                json!({"schema":manifest.schema,"bundle":bundle,"manifest_sha256":artifact.sha256,
                "members":manifest.members.len(),"bytes":bytes,"chunk_bytes":CHUNK_BYTES,
                "known_answer_replay_performed":true,"cloud_requests":0,"remote_published":false}),
            )
        }
        CustodyCommand::Verify { bundle } => {
            let bundle = super::checked_directory(bundle, false)?;
            let manifest = read_manifest(&bundle)?;
            let bytes = replay_members(&bundle, &manifest, None)?;
            Ok(
                json!({"schema":"crypto.ml-dataset.restricted-custody-verify/v1",
                "manifest_sha256":crate::store::hash_file(&bundle.join("manifest.json"))?.0,
                "members":manifest.members.len(),"bytes":bytes,"all_member_hashes_verified":true,
                "native_run_replay_performed":false,"cloud_requests":0}),
            )
        }
        CustodyCommand::Restore {
            bundle,
            destination,
        } => restore_local(bundle, destination),
        CustodyCommand::Publish {
            out,
            bundle,
            remote,
            budget,
        } => {
            let remote = approved_remote(remote)?;
            let session = super::open_budget(budget)?
                .ok_or("custody S3 publication requires an active bound budget session")?;
            super::validate_budget_run(&session, out)?;
            let context = S3BudgetContext::from_env(session)?;
            let result = publish_bundle(out, bundle, &remote, &context);
            super::combine_finalization(
                result,
                super::finalize_budget(bundle, Some(&context), Some(&remote), true),
            )
        }
        CustodyCommand::RestoreRemote {
            receipt,
            bundle,
            destination,
            budget,
        } => {
            let session = super::open_budget(budget)?
                .ok_or("custody remote restore requires an active bound budget session")?;
            let context = S3BudgetContext::from_env(session)?;
            let result = restore_remote(receipt, bundle, destination, &context);
            super::combine_finalization(
                result,
                super::finalize_budget(bundle, Some(&context), None, false),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "ml-custody-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn chunk_manifest_roundtrip_and_tamper_rejection() {
        let scratch = Scratch::new();
        let source = scratch.0.join("source");
        let bundle = scratch.0.join("bundle");
        let restored = scratch.0.join("restored");
        fs::create_dir(&source).unwrap();
        fs::create_dir(&bundle).unwrap();
        fs::create_dir(&restored).unwrap();
        let data = vec![0x5a; CHUNK_BYTES + 9];
        fs::write(source.join("trace.bin"), &data).unwrap();
        let mut members = Vec::new();
        let mut total = 0;
        collect(
            &source,
            "run",
            &bundle,
            &mut members,
            &mut total,
            10_000_000,
            2,
        )
        .unwrap();
        assert_eq!(total, data.len() as u64);
        assert_eq!(members[0].chunks.len(), 2);
        let file_hash = members[0].sha256.clone();
        let manifest = Manifest {
            schema: "crypto.ml-dataset.restricted-custody/v1".into(),
            chunk_bytes: CHUNK_BYTES,
            plan_file_sha256: file_hash.clone(),
            protocol_file_sha256: file_hash.clone(),
            run_file_sha256: file_hash.clone(),
            source_commit: "0".repeat(40),
            binary_sha256: file_hash,
            known_answer_replay_performed: true,
            members: members.clone(),
        };
        // The root provenance files are mandatory, even for a syntactically
        // valid collection of content-addressed objects.
        assert!(check_manifest(&manifest).is_err());
        for name in ["plan.json", "protocol.json", "run.json"] {
            let bytes = name.as_bytes();
            let hash = sha(bytes);
            storage::write_bytes(
                &bundle,
                &object_path(&hash).unwrap(),
                "chunk",
                "binary",
                bytes,
            )
            .unwrap();
            members.push(Member {
                path: format!("run/{name}"),
                bytes: bytes.len() as u64,
                sha256: hash.clone(),
                chunks: vec![hash.clone()],
            });
        }
        let manifest = Manifest {
            plan_file_sha256: sha(b"plan.json"),
            protocol_file_sha256: sha(b"protocol.json"),
            run_file_sha256: sha(b"run.json"),
            members,
            ..manifest
        };
        for name in ["plan.json", "protocol.json", "run.json"] {
            fs::write(source.join(name), name).unwrap();
        }
        compare_source_run(&source, &manifest).unwrap();
        fs::write(source.join("trace.bin"), b"changed after capture").unwrap();
        assert!(compare_source_run(&source, &manifest).is_err());
        fs::write(source.join("trace.bin"), &data).unwrap();
        fs::write(source.join("extra.txt"), b"not in bundle").unwrap();
        assert!(compare_source_run(&source, &manifest).is_err());
        fs::remove_file(source.join("extra.txt")).unwrap();
        storage::write_json(&bundle, "manifest.json", "custody-test", &manifest).unwrap();
        let objects = bundle_objects(&bundle, &manifest).unwrap();
        assert_eq!(objects.len(), 6);
        assert_eq!(objects.last().unwrap().path, "manifest.json");
        assert_eq!(
            replay_members(&bundle, &manifest, Some(&restored)).unwrap(),
            total + 30
        );
        assert_eq!(fs::read(restored.join("run/trace.bin")).unwrap(), data);
        let object = storage::contained_path(
            &bundle,
            &object_path(&manifest.members[0].chunks[0]).unwrap(),
            false,
        )
        .unwrap();
        fs::write(object, b"tampered").unwrap();
        assert!(replay_members(&bundle, &manifest, None).is_err());
        assert!(bundle_objects(&bundle, &manifest).is_err());
    }

    #[test]
    fn remote_must_use_approved_s3_prefix() {
        assert!(approved_remote("file:///tmp/custody").is_err());
        assert!(approved_remote("s3://setec-astronomy-artifacts").is_err());
        assert!(approved_remote("s3://other-bucket/custody").is_err());
        assert!(approved_remote("s3://setec-astronomy-artifacts/custody").is_ok());
    }

    #[test]
    fn capture_rejects_symlinks_and_caps() {
        let scratch = Scratch::new();
        let source = scratch.0.join("source");
        let bundle = scratch.0.join("bundle");
        fs::create_dir(&source).unwrap();
        fs::create_dir(&bundle).unwrap();
        fs::write(source.join("trace.bin"), b"data").unwrap();
        let mut members = Vec::new();
        let mut total = 0;
        assert!(collect(&source, "run", &bundle, &mut members, &mut total, 3, 1).is_err());
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(source.join("trace.bin"), source.join("alias")).unwrap();
            let mut members = Vec::new();
            let mut total = 0;
            assert!(collect(&source, "run", &bundle, &mut members, &mut total, 10, 2).is_err());
        }
    }
}
