//! Durable dataset artifacts and verified publication through the native store.
//!
//! Publication is deliberately stronger than `Store::verify(remote = true)`:
//! every object is downloaded and hashed, including objects found on a retry.
//! The caller owns the worker lease and gives each worker its own store index.
//! Local writes assume that lease excludes concurrent directory mutation.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::budget::BudgetScope;
use super::s3_budget::S3BudgetContext;
use super::types::Artifact;
use crate::hash::sha256::Sha256;
use crate::store::backend::{object_key, Backend, Meta, RemoteSpec, SINGLE_PUT_MAX};
use crate::store::index::{validate_name, validate_sha256};
use crate::store::{hash_file, Config, PushMeta, Store};

const RECEIPT_SCHEMA: &str = "ml-dataset.publication/v1";
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Every dataset S3 store uses the accounting transport, including nested
/// Store::push requests. The ordinary AWS CLI backend is never a fallback.
pub(super) fn dataset_store(
    cfg: Config,
    budget: Option<&Arc<S3BudgetContext>>,
    scope: BudgetScope,
) -> Result<Store, String> {
    match cfg.remote.as_ref() {
        Some(RemoteSpec::S3 { .. }) => super::s3_budget::open_store(
            cfg,
            budget
                .ok_or("dataset S3 operations require an active bound budget session")?
                .clone(),
            scope,
        ),
        _ => Store::open(cfg),
    }
}

pub(super) fn expect_remote_object(
    budget: Option<&Arc<S3BudgetContext>>,
    remote: &RemoteSpec,
    key: &str,
    artifact: &Artifact,
    scope: BudgetScope,
) -> Result<(), String> {
    if let RemoteSpec::S3 { bucket, prefix } = remote {
        let full_key = if prefix.is_empty() {
            key.to_owned()
        } else {
            format!("{prefix}/{key}")
        };
        budget
            .ok_or("dataset S3 object requires an active bound budget session")?
            .expect_object(bucket, &full_key, &artifact.sha256, artifact.bytes, scope)?;
    }
    Ok(())
}

pub(super) fn reserve_remote_output(
    budget: Option<&Arc<S3BudgetContext>>,
    remote: &RemoteSpec,
    key: &str,
    artifact: &Artifact,
    scope: BudgetScope,
) -> Result<(), String> {
    expect_remote_object(budget, remote, key, artifact, scope.clone())?;
    if let RemoteSpec::S3 { bucket, prefix } = remote {
        let key = if prefix.is_empty() {
            key.to_owned()
        } else {
            format!("{prefix}/{key}")
        };
        budget
            .ok_or("S3 output requires a budget")?
            .session()
            .reserve_object(bucket, &key, &artifact.sha256, artifact.bytes, &scope)?;
    }
    Ok(())
}

pub(super) fn verify_remote_output(
    budget: Option<&Arc<S3BudgetContext>>,
    remote: &RemoteSpec,
    key: &str,
    scope: BudgetScope,
) -> Result<(), String> {
    if let RemoteSpec::S3 { bucket, prefix } = remote {
        let key = if prefix.is_empty() {
            key.to_owned()
        } else {
            format!("{prefix}/{key}")
        };
        budget
            .ok_or("S3 output verification requires a budget")?
            .verify_registered(bucket, &key, scope)?;
    }
    Ok(())
}

/// Audit checkpoints are independent immutable control objects. They never
/// change an existing cell completion or catalog receipt on a later grant.
pub(super) fn publish_budget_checkpoint(
    root: &Path,
    artifact: &Artifact,
    cfg: Config,
    budget: &Arc<S3BudgetContext>,
) -> Result<Artifact, String> {
    let remote = cfg
        .remote
        .clone()
        .ok_or("budget checkpoint has no remote")?;
    let key = format!("budget-checkpoints/{}.sqlite", artifact.sha256);
    reserve_remote_output(
        Some(budget),
        &remote,
        &key,
        artifact,
        BudgetScope::Finalization,
    )?;
    let store = dataset_store(cfg, Some(budget), BudgetScope::Finalization)?;
    verify_manifest(root, std::slice::from_ref(artifact))?;
    prepare_s3_publication(
        store.remote()?,
        &remote,
        &key,
        &root.join(&artifact.path),
        artifact,
        root,
    )?;
    let temp = TempFile::new(root)?;
    store.remote()?.get(&key, &temp.path)?;
    check_file(&temp.path, &artifact.sha256, artifact.bytes)?;
    verify_remote_output(Some(budget), &remote, &key, BudgetScope::Finalization)?;
    write_json(
        root,
        &format!("budget-checkpoints/verified-{}.json", artifact.sha256),
        "restricted-budget-checkpoint-readback",
        &serde_json::json!({
            "remote":remote.display(),"object_key":key,"sha256":artifact.sha256,"bytes":artifact.bytes,
            "roundtrip_verified":true,"journal_scope":"prefix before checkpoint publication requests",
            "account_wide_billing_cutoff":false}),
    )
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PublishedObject {
    pub name: String,
    pub artifact: Artifact,
    pub object_key: String,
    pub roundtrip_verified: bool,
}

/// A local completion record's content. Its own digest is returned separately
/// in `PublishReceipt`, avoiding a self-referential digest.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct CompletionRecord {
    schema: String,
    remote: String,
    namespace: String,
    manifest_sha256: String,
    objects: Vec<PublishedObject>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishReceipt {
    pub schema: String,
    pub remote: String,
    pub namespace: String,
    pub manifest_sha256: String,
    /// Includes the manifest, after its dependent objects.
    pub objects: Vec<PublishedObject>,
    /// Uploaded and read back after every object in `objects`.
    pub completion: PublishedObject,
    /// Local acknowledgement written only after the completion's round trip.
    pub confirmation: Artifact,
    /// Machine-local bookkeeping is not part of a portable receipt's bytes.
    #[serde(default, skip_serializing)]
    pub index_path: PathBuf,
    /// The discoverable catalog stores this receipt with this field omitted,
    /// so neither the catalog nor its acknowledgement hashes itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog: Option<CatalogLocator>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CatalogLocator {
    /// Relative to the receipt's remote, appended after its configured prefix.
    pub key: String,
    pub sha256: String,
    pub bytes: u64,
    pub roundtrip_verified: bool,
    pub local_receipt: Artifact,
    pub confirmation: Artifact,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogConfirmation {
    pub schema: String,
    pub remote: String,
    pub namespace: String,
    pub manifest_sha256: String,
    pub key: String,
    pub sha256: String,
    pub bytes: u64,
    pub roundtrip_verified: bool,
    pub receipt: Artifact,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationConfirmation {
    pub schema: String,
    pub remote: String,
    pub namespace: String,
    pub manifest_sha256: String,
    pub completion: PublishedObject,
}

fn digest(bytes: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(bytes);
    hex::encode(hash.finalize())
}

fn check_relative(path: &str) -> Result<(), String> {
    if path.is_empty()
        || path.contains('\\')
        || path.bytes().any(|b| b.is_ascii_control())
        || path
            .split('/')
            .any(|p| p.is_empty() || p == "." || p == "..")
        || Path::new(path)
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(format!("unsafe artifact path {path:?}"));
    }
    Ok(())
}

pub(super) fn root_dir(root: &Path, create: bool) -> Result<PathBuf, String> {
    if create {
        fs::create_dir_all(root).map_err(|e| format!("{}: {e}", root.display()))?;
    }
    let meta = fs::symlink_metadata(root).map_err(|e| format!("{}: {e}", root.display()))?;
    if !meta.is_dir() || meta.file_type().is_symlink() {
        return Err(format!("{} is not an ordinary directory", root.display()));
    }
    root.canonicalize()
        .map_err(|e| format!("{}: {e}", root.display()))
}

/// Reject symlinks at every component, including symlinks whose targets remain
/// inside the root. This also rejects aliases between artifact paths.
pub(super) fn contained_path(
    root: &Path,
    relative: &str,
    create_parents: bool,
) -> Result<PathBuf, String> {
    check_relative(relative)?;
    let mut path = root.to_path_buf();
    let parts: Vec<&str> = relative.split('/').collect();
    for (i, part) in parts.iter().enumerate() {
        path.push(part);
        let last = i + 1 == parts.len();
        match fs::symlink_metadata(&path) {
            Ok(meta) => {
                if meta.file_type().is_symlink()
                    || (!last && !meta.is_dir())
                    || (last && !meta.is_file())
                {
                    return Err(format!(
                        "{} is not an ordinary {}",
                        path.display(),
                        if last { "file" } else { "directory" }
                    ));
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound && last => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound && create_parents => {
                match fs::create_dir(&path) {
                    Ok(()) => {
                        sync_directory(path.parent().expect("child has parent"))?;
                    }
                    Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                        let meta = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
                        if !meta.is_dir() || meta.file_type().is_symlink() {
                            return Err(format!("{} is not an ordinary directory", path.display()));
                        }
                    }
                    Err(e) => return Err(format!("{}: {e}", path.display())),
                }
            }
            Err(e) => return Err(format!("{}: {e}", path.display())),
        }
    }
    // The parent exists even when the file is new. Do not rely on a lexical
    // starts_with check alone when deciding containment.
    let parent = path
        .parent()
        .ok_or_else(|| "artifact has no parent".to_string())?;
    let actual_parent = parent
        .canonicalize()
        .map_err(|e| format!("{}: {e}", parent.display()))?;
    if !actual_parent.starts_with(root) {
        return Err(format!("{} escapes artifact root", path.display()));
    }
    Ok(path)
}

#[cfg(unix)]
pub(super) fn sync_directory(path: &Path) -> Result<(), String> {
    File::open(path)
        .and_then(|file| file.sync_all())
        .map_err(|e| format!("sync directory {}: {e}", path.display()))
}

#[cfg(not(unix))]
pub(super) fn sync_directory(_path: &Path) -> Result<(), String> {
    // Rust does not expose a portable directory flush on Windows. File data
    // is still synced and the link installation is create-if-absent.
    Ok(())
}

struct TempFile {
    path: PathBuf,
    file: File,
}

impl TempFile {
    fn new(parent: &Path) -> Result<Self, String> {
        for _ in 0..100 {
            let seq = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!(".dataset-part-{}-{seq}", std::process::id()));
            let mut opts = OpenOptions::new();
            opts.read(true).write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                opts.mode(0o600);
            }
            match opts.open(&path) {
                Ok(file) => return Ok(Self { path, file }),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(format!("{}: {e}", path.display())),
            }
        }
        Err("could not allocate a unique dataset temporary file".into())
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

pub(super) fn check_file(path: &Path, sha256: &str, bytes: u64) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(format!("{} is not an ordinary file", path.display()));
    }
    let (actual_sha, actual_bytes) = hash_file(path)?;
    if actual_sha != sha256 || actual_bytes != bytes {
        return Err(format!("{}: artifact mismatch: expected sha256 {sha256}, {bytes} bytes; got {actual_sha}, {actual_bytes} bytes", path.display()));
    }
    Ok(())
}

fn install(temp: &TempFile, destination: &Path, sha256: &str, bytes: u64) -> Result<(), String> {
    temp.file
        .sync_all()
        .map_err(|e| format!("{}: {e}", temp.path.display()))?;
    // A rename could replace existing data. Linking a fully synced same-device
    // temporary file publishes complete bytes without replacing any name.
    match fs::hard_link(&temp.path, destination) {
        Ok(()) => sync_directory(destination.parent().expect("artifact has parent"))?,
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            check_file(destination, sha256, bytes)?;
        }
        Err(e) => return Err(format!("install {}: {e}", destination.display())),
    }
    Ok(())
}

/// Write complete bytes once. Identical retries are idempotent; a differing
/// retry returns an error and preserves the original artifact.
pub fn write_bytes(
    root: &Path,
    path: &str,
    role: &str,
    encoding: &str,
    bytes: &[u8],
) -> Result<Artifact, String> {
    if role.trim().is_empty() || encoding.trim().is_empty() {
        return Err("artifact role and encoding must be nonempty".into());
    }
    check_relative(path)?;
    let root = root_dir(root, true)?;
    let destination = contained_path(&root, path, true)?;
    let artifact = Artifact {
        role: role.into(),
        path: path.into(),
        sha256: digest(bytes),
        bytes: bytes.len() as u64,
        encoding: encoding.into(),
    };
    if destination.exists() {
        check_file(&destination, &artifact.sha256, artifact.bytes)?;
        return Ok(artifact);
    }
    let mut temp = TempFile::new(destination.parent().expect("artifact has parent"))?;
    temp.file
        .write_all(bytes)
        .map_err(|e| format!("{}: {e}", temp.path.display()))?;
    install(&temp, &destination, &artifact.sha256, artifact.bytes)?;
    Ok(artifact)
}

pub fn write_json<T: Serialize>(
    root: &Path,
    path: &str,
    role: &str,
    value: &T,
) -> Result<Artifact, String> {
    // Value maps are sorted by serde_json's default BTreeMap representation.
    // This canonicalizes map order, but does not claim RFC 8785 compatibility.
    let value = serde_json::to_value(value).map_err(|e| format!("serialize {path}: {e}"))?;
    let mut bytes =
        serde_json::to_vec_pretty(&value).map_err(|e| format!("serialize {path}: {e}"))?;
    bytes.push(b'\n');
    write_bytes(root, path, role, "json", &bytes)
}

/// Check every manifest member, rejecting missing, changed, or repeated paths.
/// Unlisted worker state (leases and publication indexes) is allowed.
pub fn verify_manifest(root: &Path, artifacts: &[Artifact]) -> Result<(), String> {
    let root = root_dir(root, false)?;
    let mut paths = BTreeSet::new();
    for artifact in artifacts {
        if !paths.insert(&artifact.path) {
            return Err(format!("duplicate artifact path {:?}", artifact.path));
        }
        if artifact.role.trim().is_empty() || artifact.encoding.trim().is_empty() {
            return Err(format!("{}: empty role or encoding", artifact.path));
        }
        validate_sha256(&artifact.sha256)?;
        let path = contained_path(&root, &artifact.path, false)?;
        check_file(&path, &artifact.sha256, artifact.bytes)?;
    }
    Ok(())
}

fn verify_manifest_binding(
    root: &Path,
    artifacts: &[Artifact],
    manifest: &Artifact,
) -> Result<(), String> {
    let path = contained_path(root, &manifest.path, false)?;
    let file = File::open(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let value: serde_json::Value =
        serde_json::from_reader(file).map_err(|e| format!("manifest {}: {e}", path.display()))?;
    let members: Vec<Artifact> = serde_json::from_value(
        value
            .get("artifacts")
            .cloned()
            .ok_or_else(|| "publication manifest must have an artifacts array".to_string())?,
    )
    .map_err(|e| format!("manifest artifacts: {e}"))?;
    let expected: BTreeMap<_, _> = artifacts.iter().map(|a| (&a.path, a)).collect();
    let actual: BTreeMap<_, _> = members.iter().map(|a| (&a.path, a)).collect();
    if actual.len() != members.len() || expected != actual {
        return Err(
            "publication manifest artifacts do not exactly match the supplied artifact set".into(),
        );
    }
    Ok(())
}

fn roundtrip(store: &Store, artifact: &Artifact, scratch: &Path) -> Result<(), String> {
    let temp = TempFile::new(scratch)?;
    store
        .remote()?
        .get(&object_key(&artifact.sha256)?, &temp.path)?;
    check_file(&temp.path, &artifact.sha256, artifact.bytes)
        .map_err(|e| format!("remote round-trip failed: {e}"))
}

/// A denied S3 HEAD is ambiguous when ListBucket is unavailable. It is never
/// evidence of absence. Only this authorized dataset S3 path may instead try
/// the backend's immutable single PUT, then require fresh metadata and bytes.
/// Other backends/errors and shared Store behavior retain their existing rules.
fn publication_head(
    backend: &dyn Backend,
    remote: Option<&RemoteSpec>,
    key: &str,
    source: &Path,
    artifact: &Artifact,
    scratch: &Path,
) -> Result<Option<Meta>, String> {
    match backend.head(key) {
        Ok(meta) => Ok(meta),
        Err(error)
            if matches!(remote, Some(RemoteSpec::S3 { bucket, .. }) if bucket == "setec-astronomy-artifacts")
                && ((error.starts_with("aws head-object failed")
                    && (error.contains("(403)")
                        || error.contains("(Forbidden)")
                        || error.contains("(AccessDenied)")))
                    || error.starts_with(super::s3_budget::AMBIGUOUS_HEAD_DENIED)) =>
        {
            if artifact.bytes > SINGLE_PUT_MAX {
                return Err("HEAD-denied publication exceeds immutable single-PUT limit".into());
            }
            check_file(source, &artifact.sha256, artifact.bytes)?;
            backend.put(key, source, &artifact.sha256).map_err(|e| {
                format!(
                    "conditional dataset PUT after denied HEAD failed: {e}; initial HEAD: {error}"
                )
            })?;
            let meta = backend
                .head(key)?
                .ok_or("dataset object absent after conditional PUT")?;
            if meta.bytes != artifact.bytes
                || meta
                    .sha256
                    .as_ref()
                    .is_some_and(|sha| sha != &artifact.sha256)
            {
                return Err(
                    "dataset metadata differs after conditional PUT; refusing acknowledgement"
                        .into(),
                );
            }
            let temp = TempFile::new(scratch)?;
            backend.get(key, &temp.path)?;
            check_file(&temp.path, &artifact.sha256, artifact.bytes)
                .map_err(|e| format!("conditional dataset PUT readback failed: {e}"))?;
            Ok(Some(meta))
        }
        Err(error) => Err(error),
    }
}

/// Prepare an authorized immutable S3 object for the caller's final readback.
/// Success here is not a publication acknowledgement: the caller must always
/// download and verify bytes before writing its durable completion receipt.
pub(super) fn prepare_s3_publication(
    backend: &dyn Backend,
    remote: &RemoteSpec,
    key: &str,
    source: &Path,
    artifact: &Artifact,
    scratch: &Path,
) -> Result<(), String> {
    if !matches!(remote, RemoteSpec::S3 { bucket, .. } if bucket == "setec-astronomy-artifacts") {
        return Err("dataset S3 publication requires the approved bucket".into());
    }
    check_relative(key)?;
    if artifact.bytes > SINGLE_PUT_MAX {
        return Err("dataset receipt exceeds immutable single-PUT limit".into());
    }
    check_file(source, &artifact.sha256, artifact.bytes)?;
    match publication_head(backend, Some(remote), key, source, artifact, scratch)? {
        Some(meta) => {
            if meta.bytes != artifact.bytes
                || meta
                    .sha256
                    .as_ref()
                    .is_some_and(|sha| sha != &artifact.sha256)
            {
                return Err(format!(
                    "{key}: existing size or digest mismatch; refusing overwrite"
                ));
            }
        }
        None => {
            backend.put(key, source, &artifact.sha256)?;
        }
    }
    Ok(())
}

fn publish_object(
    store: &mut Store,
    root: &Path,
    artifact: &Artifact,
    namespace: &str,
    budget: Option<&Arc<S3BudgetContext>>,
    scope: BudgetScope,
) -> Result<PublishedObject, String> {
    if matches!(store.cfg.remote, Some(RemoteSpec::S3 { .. })) && artifact.bytes > SINGLE_PUT_MAX {
        return Err(format!("dataset S3 artifacts must be chunked to at most {SINGLE_PUT_MAX} bytes for immutable conditional PUT"));
    }
    let source = contained_path(root, &artifact.path, false)?;
    check_file(&source, &artifact.sha256, artifact.bytes)?;
    let name = format!("{namespace}/{}", artifact.path);
    validate_name(&name)?;
    let key = object_key(&artifact.sha256)?;
    reserve_remote_output(
        budget,
        store
            .cfg
            .remote
            .as_ref()
            .ok_or("missing publication remote")?,
        &key,
        artifact,
        scope.clone(),
    )?;
    if let Some(existing) = store.index.get(&name) {
        if existing.sha256 != artifact.sha256 || existing.bytes != artifact.bytes {
            return Err(format!(
                "{name}: refusing to repoint a published dataset name"
            ));
        }
    }
    if let Some(meta) = publication_head(
        store.remote()?,
        store.cfg.remote.as_ref(),
        &key,
        &source,
        artifact,
        root,
    )? {
        if meta.bytes != artifact.bytes
            || meta
                .sha256
                .as_ref()
                .is_some_and(|sha| sha != &artifact.sha256)
        {
            return Err(format!(
                "{key}: existing remote object has mismatched size or digest; refusing overwrite"
            ));
        }
        roundtrip(store, artifact, root)?;
    }
    // The ordinary directory backend's put replaces files. Install its object
    // ourselves with create-if-absent semantics; Store::push then only indexes
    // it. S3 uses the existing backend's conditional single-object PUT.
    if let Some(RemoteSpec::Dir(directory)) = store.cfg.remote.as_ref() {
        let directory = root_dir(directory, true)?;
        let destination = contained_path(&directory, &key, true)?;
        if destination.exists() {
            check_file(&destination, &artifact.sha256, artifact.bytes)?;
        } else {
            let mut temp = TempFile::new(destination.parent().expect("object has parent"))?;
            let mut input = File::open(&source).map_err(|e| e.to_string())?;
            io::copy(&mut input, &mut temp.file).map_err(|e| e.to_string())?;
            check_file(&temp.path, &artifact.sha256, artifact.bytes)?;
            install(&temp, &destination, &artifact.sha256, artifact.bytes)?;
        }
    }
    let report = store.push(
        &name,
        &source,
        PushMeta {
            description: format!("ML dataset {} ({})", artifact.role, artifact.encoding),
            tags: vec!["ml-dataset".into()],
            provenance: format!(
                "immutable dataset artifact {}; sha256 {}",
                artifact.path, artifact.sha256
            ),
            replace: false,
            no_cache: true,
        },
    )?;
    if report.sha256 != artifact.sha256 || report.bytes != artifact.bytes || report.replaced {
        return Err(format!("{name}: source changed during publication"));
    }
    roundtrip(store, artifact, root)?;
    verify_remote_output(
        budget,
        store
            .cfg
            .remote
            .as_ref()
            .ok_or("missing publication remote")?,
        &key,
        scope,
    )?;
    File::open(&store.cfg.index_path)
        .and_then(|file| file.sync_all())
        .map_err(|e| format!("sync store index {}: {e}", store.cfg.index_path.display()))?;
    sync_directory(
        store
            .cfg
            .index_path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new(".")),
    )?;
    Ok(PublishedObject {
        name,
        artifact: artifact.clone(),
        object_key: key,
        roundtrip_verified: true,
    })
}

fn catalog_key(namespace: &str, manifest_sha256: &str) -> Result<String, String> {
    validate_name(namespace)?;
    validate_sha256(manifest_sha256)?;
    let key = format!("ml-dataset/catalog/{namespace}/{manifest_sha256}.json");
    check_relative(&key)?;
    Ok(key)
}

/// Publish a discoverable, portable receipt under an immutable non-CAS key.
/// The full receipt is kept locally before upload; only the separate catalog
/// acknowledgement records a successful remote download and digest check.
fn publish_catalog(
    root: &Path,
    receipt: &PublishReceipt,
    store: &Store,
    budget: Option<&Arc<S3BudgetContext>>,
) -> Result<CatalogLocator, String> {
    let key = catalog_key(&receipt.namespace, &receipt.manifest_sha256)?;
    let mut portable = receipt.clone();
    portable.catalog = None;
    let receipt_id = digest(&serde_json::to_vec(&portable).map_err(|e| e.to_string())?);
    let artifact = write_json(
        root,
        &format!("publication/receipt-{receipt_id}.json"),
        "publication_portable_receipt",
        &portable,
    )?;
    let source = contained_path(root, &artifact.path, false)?;
    reserve_remote_output(
        budget,
        store.cfg.remote.as_ref().ok_or("missing catalog remote")?,
        &key,
        &artifact,
        BudgetScope::Finalization,
    )?;
    match store.cfg.remote.as_ref() {
        Some(RemoteSpec::Dir(directory)) => {
            // DirBackend::put replaces existing names. Use the same synced,
            // create-if-absent installation as immutable local artifacts.
            let directory = root_dir(directory, true)?;
            let destination = contained_path(&directory, &key, true)?;
            if destination.exists() {
                check_file(&destination, &artifact.sha256, artifact.bytes)
                    .map_err(|e| format!("catalog refuses overwrite: {e}"))?;
            } else {
                let mut temp = TempFile::new(destination.parent().expect("catalog has parent"))?;
                let mut input = File::open(&source).map_err(|e| e.to_string())?;
                io::copy(&mut input, &mut temp.file).map_err(|e| e.to_string())?;
                check_file(&temp.path, &artifact.sha256, artifact.bytes)?;
                install(&temp, &destination, &artifact.sha256, artifact.bytes)?;
            }
        }
        Some(RemoteSpec::S3 { bucket, .. }) if bucket == "setec-astronomy-artifacts" => {
            prepare_s3_publication(
                store.remote()?,
                store.cfg.remote.as_ref().expect("matched S3 remote"),
                &key,
                &source,
                &artifact,
                root,
            )?;
        }
        _ => return Err("catalog publication requires an authorized S3 or file remote".into()),
    }
    // Do not trust HEAD metadata alone, or DirBackend's key-derived checksum:
    // this key intentionally contains the manifest digest, not receipt digest.
    let temp = TempFile::new(root)?;
    store.remote()?.get(&key, &temp.path)?;
    check_file(&temp.path, &artifact.sha256, artifact.bytes)
        .map_err(|e| format!("catalog remote round-trip failed: {e}"))?;
    verify_remote_output(
        budget,
        store.cfg.remote.as_ref().ok_or("missing catalog remote")?,
        &key,
        BudgetScope::Finalization,
    )?;
    let confirmation = write_json(
        root,
        &format!("publication/catalog-verified-{receipt_id}.json"),
        "publication_catalog_confirmation",
        &CatalogConfirmation {
            schema: "ml-dataset.publication-catalog-confirmation/v1".into(),
            remote: receipt.remote.clone(),
            namespace: receipt.namespace.clone(),
            manifest_sha256: receipt.manifest_sha256.clone(),
            key: key.clone(),
            sha256: artifact.sha256.clone(),
            bytes: artifact.bytes,
            roundtrip_verified: true,
            receipt: artifact.clone(),
        },
    )?;
    Ok(CatalogLocator {
        key,
        sha256: artifact.sha256.clone(),
        bytes: artifact.bytes,
        roundtrip_verified: true,
        local_receipt: artifact,
        confirmation,
    })
}

/// Publish artifact objects first, their binding manifest second, and a
/// completion receipt next, and its discoverable catalog locator last. Every
/// successful object has a download/hash check.
/// Interrupted calls may leave verified objects and index entries; retrying the
/// exact manifest is safe and rechecks them before producing completion.
pub fn publish(
    root: &Path,
    artifacts: &[Artifact],
    manifest: &Artifact,
    cfg: Config,
    namespace: &str,
) -> Result<PublishReceipt, String> {
    publish_budgeted(
        root,
        artifacts,
        manifest,
        cfg,
        namespace,
        None,
        BudgetScope::Work,
    )
}

pub fn publish_budgeted(
    root: &Path,
    artifacts: &[Artifact],
    manifest: &Artifact,
    cfg: Config,
    namespace: &str,
    budget: Option<&Arc<S3BudgetContext>>,
    scope: BudgetScope,
) -> Result<PublishReceipt, String> {
    validate_name(namespace)?;
    catalog_key(namespace, &manifest.sha256)?;
    match cfg.remote.as_ref() {
        Some(RemoteSpec::S3 { bucket, .. }) if bucket != "setec-astronomy-artifacts" => {
            return Err(format!("dataset publication may only write S3 bucket setec-astronomy-artifacts, not {bucket}"));
        }
        Some(_) => {}
        None => return Err("dataset publication requires an explicit S3 or file remote".into()),
    }
    // The existing store's multipart path does not enforce If-None-Match.
    // Dataset producers must chunk large objects before publication.
    if matches!(cfg.remote, Some(RemoteSpec::S3 { .. }))
        && artifacts
            .iter()
            .chain(std::iter::once(manifest))
            .any(|a| a.bytes > SINGLE_PUT_MAX)
    {
        return Err(format!("dataset S3 artifacts must be chunked to at most {SINGLE_PUT_MAX} bytes for immutable conditional PUT"));
    }
    let root = root_dir(root, false)?;
    let mut all = artifacts.to_vec();
    all.push(manifest.clone());
    verify_manifest(&root, &all)?;
    verify_manifest_binding(&root, artifacts, manifest)?;
    let remote = cfg.remote.as_ref().expect("validated remote").display();
    let index_path = cfg.index_path.clone();
    let mut store = dataset_store(cfg.clone(), budget, scope.clone())?;
    // Validate all logical names/conflicts before causing any remote writes.
    for artifact in &all {
        let name = format!("{namespace}/{}", artifact.path);
        validate_name(&name)?;
        if let Some(previous) = store.index.get(&name) {
            if previous.sha256 != artifact.sha256 || previous.bytes != artifact.bytes {
                return Err(format!(
                    "{name}: refusing to repoint a published dataset name"
                ));
            }
        }
    }
    let mut objects = Vec::with_capacity(all.len());
    for artifact in &all {
        objects.push(publish_object(
            &mut store,
            &root,
            artifact,
            namespace,
            budget,
            scope.clone(),
        )?);
    }
    let record = CompletionRecord {
        schema: RECEIPT_SCHEMA.into(),
        remote: remote.clone(),
        namespace: namespace.into(),
        manifest_sha256: manifest.sha256.clone(),
        objects: objects.clone(),
    };
    // Namespace and destination are included so publishing the same immutable
    // shard to a second explicitly selected remote does not repoint a receipt.
    let receipt_id = digest(&serde_json::to_vec(&record).map_err(|e| e.to_string())?);
    let receipt_path = format!("publication/{receipt_id}.json");
    let receipt = write_json(&root, &receipt_path, "publication_completion", &record)?;
    // Reopen after the work store persisted its index. Completion/catalog
    // requests consume the protected finalization partition of the same grant.
    let mut final_store = dataset_store(cfg, budget, BudgetScope::Finalization)?;
    let completion = publish_object(
        &mut final_store,
        &root,
        &receipt,
        namespace,
        budget,
        BudgetScope::Finalization,
    )?;
    let confirmation = write_json(
        &root,
        &format!("publication/verified-{receipt_id}.json"),
        "publication_confirmation",
        &PublicationConfirmation {
            schema: "ml-dataset.publication-confirmation/v1".into(),
            remote: remote.clone(),
            namespace: namespace.into(),
            manifest_sha256: manifest.sha256.clone(),
            completion: completion.clone(),
        },
    )?;
    let mut receipt = PublishReceipt {
        schema: RECEIPT_SCHEMA.into(),
        remote,
        namespace: namespace.into(),
        manifest_sha256: manifest.sha256.clone(),
        objects,
        completion,
        confirmation,
        index_path,
        catalog: None,
    };
    receipt.catalog = Some(publish_catalog(&root, &receipt, &final_store, budget)?);
    Ok(receipt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::cell::RefCell;

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let seq = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let root = std::env::temp_dir().join(format!(
                "crypto-ml-storage-{}-{nanos}-{seq}",
                std::process::id()
            ));
            fs::create_dir(&root).unwrap();
            Self(root)
        }
        fn cfg(&self) -> Config {
            Config {
                index_path: self.0.join("index.json"),
                cache_dir: self.0.join("cache"),
                remote: Some(RemoteSpec::Dir(self.0.join("remote"))),
                endpoint: None,
                aws: "aws".into(),
            }
        }
        fn shard(&self) -> (Vec<Artifact>, Artifact) {
            let artifact =
                write_bytes(&self.0, "cases/one.bin", "test_input", "binary", b"abcdef").unwrap();
            let manifest = write_json(
                &self.0,
                "manifest.json",
                "manifest",
                &json!({"artifacts": [artifact.clone()]}),
            )
            .unwrap();
            (vec![artifact], manifest)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    /// Models an immutable S3 object with an ambiguous initial HEAD response.
    /// Existing bytes are never replaced, even when a conditional PUT races.
    struct DeniedHeadBackend {
        first_error: RefCell<Option<String>>,
        object: RefCell<Option<Vec<u8>>>,
        put_error: Option<String>,
        get_error: Option<String>,
        calls: RefCell<Vec<&'static str>>,
    }
    impl DeniedHeadBackend {
        fn new(error: &str) -> Self {
            Self {
                first_error: RefCell::new(Some(error.into())),
                object: RefCell::new(None),
                put_error: None,
                get_error: None,
                calls: RefCell::new(Vec::new()),
            }
        }
    }
    impl Backend for DeniedHeadBackend {
        fn describe(&self) -> String {
            "mock immutable S3".into()
        }
        fn head(&self, _key: &str) -> Result<Option<Meta>, String> {
            self.calls.borrow_mut().push("head");
            if let Some(error) = self.first_error.borrow_mut().take() {
                return Err(error);
            }
            Ok(self.object.borrow().as_ref().map(|bytes| Meta {
                bytes: bytes.len() as u64,
                sha256: None,
            }))
        }
        fn put(&self, _key: &str, source: &Path, _sha256: &str) -> Result<bool, String> {
            self.calls.borrow_mut().push("conditional-put");
            if let Some(error) = &self.put_error {
                return Err(error.clone());
            }
            let mut object = self.object.borrow_mut();
            let created = object.is_none();
            if created {
                *object = Some(fs::read(source).map_err(|e| e.to_string())?);
            }
            // A same-size existing object with unavailable checksum is accepted
            // by the native backend after 412; only full readback proves bytes.
            Ok(created)
        }
        fn get(&self, _key: &str, destination: &Path) -> Result<(), String> {
            self.calls.borrow_mut().push("get");
            if let Some(error) = &self.get_error {
                return Err(error.clone());
            }
            fs::write(
                destination,
                self.object.borrow().as_ref().ok_or("missing object")?,
            )
            .map_err(|e| e.to_string())
        }
        fn list(&self, _prefix: &str) -> Result<Vec<(String, u64)>, String> {
            panic!("dataset fallback must not list a bucket")
        }
    }

    fn dataset_s3_remote() -> RemoteSpec {
        RemoteSpec::S3 {
            bucket: "setec-astronomy-artifacts".into(),
            prefix: "ml-dataset/test".into(),
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn native_budgeted_publication_resume_catalog_lab_receipt_and_fetch_share_debits() {
        let f = Fixture::new();
        let (context, wire) = super::super::budget_test_support::context(&f.0, None);
        let (artifacts, manifest) = f.shard();
        let mut cfg = f.cfg();
        cfg.remote = Some(dataset_s3_remote());
        let first = publish_budgeted(
            &f.0,
            &artifacts,
            &manifest,
            cfg.clone(),
            "datasets/test",
            Some(&context),
            BudgetScope::Work,
        )
        .unwrap();
        let calls = wire.calls.lock().unwrap().clone();
        assert!(calls.windows(3).any(|rows| rows[0].0 == "HEAD"
            && rows[1].0 == "GET"
            && rows[2].0 == "PUT"
            && rows[0].1 == rows[1].1
            && rows[1].1 == rows[2].1));
        let before = context.session().snapshot().unwrap();
        assert!(before["verified_unique_output_bytes"].as_u64().unwrap() > 0);
        let second = publish_budgeted(
            &f.0,
            &artifacts,
            &manifest,
            cfg.clone(),
            "datasets/test",
            Some(&context),
            BudgetScope::Work,
        )
        .unwrap();
        let after = context.session().snapshot().unwrap();
        assert_eq!(first.catalog, second.catalog);
        assert_eq!(
            before["reserved_unique_output_bytes"],
            after["reserved_unique_output_bytes"]
        );
        assert_eq!(before["put_attempts"], after["put_attempts"]);
        assert!(after["get_attempts"].as_u64().unwrap() > before["get_attempts"].as_u64().unwrap());
        let receipt = write_json(
            &f.0,
            "portable-receipt.json",
            "restricted-publication-receipt",
            &second,
        )
        .unwrap();
        let lab = RemoteSpec::parse("s3://setec-astronomy-artifacts/ml-dataset/test/lab").unwrap();
        super::super::publish_lab_receipt_to(&f.0, &receipt, &lab, Some(&context)).unwrap();
        assert!(wire
            .calls
            .lock()
            .unwrap()
            .iter()
            .any(|(method, key)| method == "PUT" && key.contains("/lab/dataset-receipts/")));
        let catalog = first.catalog.unwrap();
        let fetched = f.0.join("fetched-catalog.json");
        super::super::fetch_budget_object(
            &context,
            &dataset_s3_remote(),
            &catalog.key,
            &catalog.sha256,
            catalog.bytes,
            &fetched,
        )
        .unwrap();
        check_file(&fetched, &catalog.sha256, catalog.bytes).unwrap();
        let portable: PublishReceipt = serde_json::from_slice(&fs::read(fetched).unwrap()).unwrap();
        assert_eq!(portable.manifest_sha256, manifest.sha256);
        assert!(portable.catalog.is_none());
        super::super::finalize_budget(&f.0, Some(&context), Some(&dataset_s3_remote()), true)
            .unwrap();
        assert!(wire
            .calls
            .lock()
            .unwrap()
            .iter()
            .any(|(method, key)| method == "PUT" && key.contains("/budget-checkpoints/")));
        let snapshot = context.session().snapshot().unwrap();
        assert_eq!(
            snapshot["charged_attempts"].as_u64().unwrap(),
            wire.calls.lock().unwrap().len() as u64
        );
        assert!(
            snapshot["finalization_charged"]["put_attempts"]
                .as_u64()
                .unwrap()
                >= 4
        );
        assert!(!snapshot["local_byte_allocation_is_hard_write_limit"]
            .as_bool()
            .unwrap());
    }

    #[test]
    fn denied_s3_head_conditionally_creates_cas_and_catalog_then_reads_back() {
        let f = Fixture::new();
        let (artifacts, _) = f.shard();
        let artifact = &artifacts[0];
        let source = f.0.join(&artifact.path);
        for key in [
            object_key(&artifact.sha256).unwrap(),
            catalog_key("dataset/test", &artifact.sha256).unwrap(),
        ] {
            let backend = DeniedHeadBackend::new("aws head-object failed: (403) Forbidden");
            let result = publication_head(
                &backend,
                Some(&dataset_s3_remote()),
                &key,
                &source,
                artifact,
                &f.0,
            )
            .unwrap();
            assert_eq!(result.unwrap().bytes, artifact.bytes);
            assert_eq!(
                *backend.calls.borrow(),
                ["head", "conditional-put", "head", "get"]
            );
            assert_eq!(
                backend.object.borrow().as_deref(),
                Some(b"abcdef".as_slice())
            );
        }
    }

    #[test]
    fn denied_s3_head_never_acknowledges_denied_writes_or_reads() {
        let f = Fixture::new();
        let (artifacts, _) = f.shard();
        let artifact = &artifacts[0];
        let source = f.0.join(&artifact.path);
        let key = object_key(&artifact.sha256).unwrap();
        let mut backend = DeniedHeadBackend::new("aws head-object failed: (AccessDenied)");
        backend.put_error = Some("put-object AccessDenied".into());
        assert!(publication_head(
            &backend,
            Some(&dataset_s3_remote()),
            &key,
            &source,
            artifact,
            &f.0
        )
        .unwrap_err()
        .contains("conditional dataset PUT after denied HEAD failed"));
        assert_eq!(*backend.calls.borrow(), ["head", "conditional-put"]);
        assert!(backend.object.borrow().is_none());

        let mut backend = DeniedHeadBackend::new("aws head-object failed: (403) Forbidden");
        backend.get_error = Some("get-object AccessDenied".into());
        assert!(publication_head(
            &backend,
            Some(&dataset_s3_remote()),
            &key,
            &source,
            artifact,
            &f.0
        )
        .unwrap_err()
        .contains("get-object AccessDenied"));
        assert_eq!(
            *backend.calls.borrow(),
            ["head", "conditional-put", "head", "get"]
        );
    }

    #[test]
    fn lab_receipt_preparation_uses_conditional_fallback_and_preserves_conflicts() {
        let f = Fixture::new();
        let artifact = write_json(
            &f.0,
            "receipt.json",
            "publication-receipt",
            &json!({"fixture":true}),
        )
        .unwrap();
        let source = f.0.join(&artifact.path);
        let key = format!("dataset-receipts/{}.json", artifact.sha256);
        let backend = DeniedHeadBackend::new("aws head-object failed: (403) Forbidden");
        prepare_s3_publication(
            &backend,
            &dataset_s3_remote(),
            &key,
            &source,
            &artifact,
            &f.0,
        )
        .unwrap();
        assert_eq!(
            *backend.calls.borrow(),
            ["head", "conditional-put", "head", "get"]
        );
        let verified = f.0.join("lab-final-readback.json");
        backend.get(&key, &verified).unwrap();
        check_file(&verified, &artifact.sha256, artifact.bytes).unwrap();

        let corrupt = vec![b'x'; artifact.bytes as usize];
        let backend = DeniedHeadBackend::new("aws head-object failed: (403) Forbidden");
        *backend.object.borrow_mut() = Some(corrupt.clone());
        assert!(prepare_s3_publication(
            &backend,
            &dataset_s3_remote(),
            &key,
            &source,
            &artifact,
            &f.0
        )
        .unwrap_err()
        .contains("readback failed"));
        assert_eq!(backend.object.borrow().as_ref(), Some(&corrupt));

        let mut backend = DeniedHeadBackend::new("aws head-object failed: (403) Forbidden");
        backend.put_error = Some("put-object AccessDenied".into());
        assert!(prepare_s3_publication(
            &backend,
            &dataset_s3_remote(),
            &key,
            &source,
            &artifact,
            &f.0
        )
        .is_err());
        assert!(backend.object.borrow().is_none());
    }

    #[test]
    fn denied_s3_head_preserves_conflicts_and_rejects_forged_existing_bytes() {
        let f = Fixture::new();
        let (artifacts, _) = f.shard();
        let artifact = &artifacts[0];
        let source = f.0.join(&artifact.path);
        let key = object_key(&artifact.sha256).unwrap();
        let backend = DeniedHeadBackend::new("aws head-object failed: (403) Forbidden");
        *backend.object.borrow_mut() = Some(b"ABCDEF".to_vec());
        assert!(publication_head(
            &backend,
            Some(&dataset_s3_remote()),
            &key,
            &source,
            artifact,
            &f.0
        )
        .unwrap_err()
        .contains("readback failed"));
        assert_eq!(
            backend.object.borrow().as_deref(),
            Some(b"ABCDEF".as_slice())
        );
        assert_eq!(
            *backend.calls.borrow(),
            ["head", "conditional-put", "head", "get"]
        );

        let mut backend = DeniedHeadBackend::new("aws head-object failed: (403) Forbidden");
        *backend.object.borrow_mut() = Some(b"other".to_vec());
        backend.put_error = Some("PreconditionFailed: conflicting immutable object".into());
        assert!(publication_head(
            &backend,
            Some(&dataset_s3_remote()),
            &key,
            &source,
            artifact,
            &f.0
        )
        .unwrap_err()
        .contains("PreconditionFailed"));
        assert_eq!(
            backend.object.borrow().as_deref(),
            Some(b"other".as_slice())
        );
        assert_eq!(*backend.calls.borrow(), ["head", "conditional-put"]);
    }

    #[test]
    fn publication_fallback_rejects_non403_errors_other_remotes_and_oversize() {
        let f = Fixture::new();
        let (artifacts, _) = f.shard();
        let artifact = &artifacts[0];
        let source = f.0.join(&artifact.path);
        let key = object_key(&artifact.sha256).unwrap();
        for (error, remote) in [
            (
                "aws head-object failed: (500) InternalError",
                dataset_s3_remote(),
            ),
            (
                "aws head-object failed: (403) Forbidden",
                RemoteSpec::Dir(f.0.join("remote")),
            ),
            (
                "aws head-object failed: (403) Forbidden",
                RemoteSpec::S3 {
                    bucket: "another-bucket".into(),
                    prefix: String::new(),
                },
            ),
        ] {
            let backend = DeniedHeadBackend::new(error);
            assert_eq!(
                publication_head(&backend, Some(&remote), &key, &source, artifact, &f.0)
                    .unwrap_err(),
                error
            );
            assert_eq!(*backend.calls.borrow(), ["head"]);
        }
        let mut oversized = artifact.clone();
        oversized.bytes = SINGLE_PUT_MAX + 1;
        let backend = DeniedHeadBackend::new("aws head-object failed: (403) Forbidden");
        assert!(publication_head(
            &backend,
            Some(&dataset_s3_remote()),
            &key,
            &source,
            &oversized,
            &f.0
        )
        .unwrap_err()
        .contains("single-PUT limit"));
        assert_eq!(*backend.calls.borrow(), ["head"]);
    }

    #[test]
    fn immutable_retry_preserves_original_and_verifier_rejects_tampering() {
        let f = Fixture::new();
        let first = write_bytes(&f.0, "nested/a.bin", "input", "binary", b"correct").unwrap();
        assert_eq!(
            write_bytes(&f.0, "nested/a.bin", "input", "binary", b"correct").unwrap(),
            first
        );
        assert!(write_bytes(&f.0, "nested/a.bin", "input", "binary", b"changed").is_err());
        verify_manifest(&f.0, std::slice::from_ref(&first)).unwrap();
        fs::write(f.0.join(&first.path), b"tampered").unwrap();
        assert!(verify_manifest(&f.0, &[first])
            .unwrap_err()
            .contains("artifact mismatch"));
    }

    #[test]
    fn rejects_traversal_duplicates_missing_and_invalid_digests() {
        let f = Fixture::new();
        for path in [
            "../outside",
            "/absolute",
            "a/../b",
            "a//b",
            "a/./b",
            "a\\b",
            "",
        ] {
            assert!(
                write_bytes(&f.0, path, "input", "binary", b"data").is_err(),
                "{path}"
            );
        }
        let (artifacts, _) = f.shard();
        assert!(
            verify_manifest(&f.0, &[artifacts[0].clone(), artifacts[0].clone()])
                .unwrap_err()
                .contains("duplicate")
        );
        let mut invalid = artifacts[0].clone();
        invalid.sha256 = "0".into();
        assert!(verify_manifest(&f.0, &[invalid]).is_err());
        fs::remove_file(f.0.join(&artifacts[0].path)).unwrap();
        assert!(verify_manifest(&f.0, &artifacts).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_files_and_directories_even_within_root() {
        use std::os::unix::fs::symlink;
        let f = Fixture::new();
        let (artifacts, _) = f.shard();
        symlink(f.0.join("cases"), f.0.join("alias")).unwrap();
        assert!(write_bytes(&f.0, "alias/new", "input", "binary", b"data").is_err());
        symlink(f.0.join(&artifacts[0].path), f.0.join("linked")).unwrap();
        let mut linked = artifacts[0].clone();
        linked.path = "linked".into();
        assert!(verify_manifest(&f.0, &[linked]).is_err());
    }

    #[test]
    fn publication_roundtrip_and_retry_are_complete_and_identical() {
        let f = Fixture::new();
        let (artifacts, manifest) = f.shard();
        let first = publish(&f.0, &artifacts, &manifest, f.cfg(), "datasets/test").unwrap();
        assert_eq!(first.objects.len(), 2);
        assert!(first.objects.iter().all(|o| o.roundtrip_verified));
        let second = publish(&f.0, &artifacts, &manifest, f.cfg(), "datasets/test").unwrap();
        assert_eq!(first.completion, second.completion);
        assert_eq!(first.catalog, second.catalog);
        let catalog = first.catalog.as_ref().unwrap();
        assert!(catalog.roundtrip_verified);
        assert_eq!(
            catalog.key,
            catalog_key("datasets/test", &manifest.sha256).unwrap()
        );
        let catalog_path = f.0.join("remote").join(&catalog.key);
        check_file(&catalog_path, &catalog.sha256, catalog.bytes).unwrap();
        let portable: PublishReceipt =
            serde_json::from_slice(&fs::read(&catalog_path).unwrap()).unwrap();
        assert_eq!(portable.completion, first.completion);
        assert_eq!(portable.objects, first.objects);
        assert!(portable.catalog.is_none());
        assert!(portable.index_path.as_os_str().is_empty());
        let mut legacy = serde_json::to_value(&portable).unwrap();
        legacy["index_path"] = serde_json::json!("/old/workspace/index.json");
        let legacy: PublishReceipt = serde_json::from_value(legacy).unwrap();
        assert_eq!(
            legacy.index_path,
            PathBuf::from("/old/workspace/index.json")
        );
        let ack: CatalogConfirmation =
            serde_json::from_slice(&fs::read(f.0.join(&catalog.confirmation.path)).unwrap())
                .unwrap();
        assert_eq!(ack.key, catalog.key);
        assert_eq!(ack.sha256, catalog.sha256);
        assert_eq!(ack.receipt, catalog.local_receipt);
        let mut moved_index = f.cfg();
        moved_index.index_path = f.0.join("another-index.json");
        let third = publish(&f.0, &artifacts, &manifest, moved_index, "datasets/test").unwrap();
        assert_eq!(first.catalog, third.catalog);
        let record: CompletionRecord =
            serde_json::from_slice(&fs::read(f.0.join(&first.completion.artifact.path)).unwrap())
                .unwrap();
        assert_eq!(record.objects, first.objects);
        let store = Store::open(f.cfg()).unwrap();
        assert_eq!(store.index.entries.len(), 3);
        roundtrip(&store, &first.completion.artifact, &f.0).unwrap();
    }

    #[test]
    fn failed_catalog_is_preserved_and_never_acknowledged() {
        let f = Fixture::new();
        let (artifacts, manifest) = f.shard();
        let key = catalog_key("datasets/test", &manifest.sha256).unwrap();
        let remote = root_dir(&f.0.join("remote"), true).unwrap();
        let destination = contained_path(&remote, &key, true).unwrap();
        fs::write(&destination, b"unexpected catalog content").unwrap();
        let err = publish(&f.0, &artifacts, &manifest, f.cfg(), "datasets/test").unwrap_err();
        assert!(err.contains("catalog refuses overwrite"), "{err}");
        assert_eq!(
            fs::read(destination).unwrap(),
            b"unexpected catalog content"
        );
        let names: Vec<_> = fs::read_dir(f.0.join("publication"))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert!(names.iter().any(|name| name.starts_with("receipt-")));
        assert!(names.iter().any(|name| name.starts_with("verified-")));
        assert!(!names
            .iter()
            .any(|name| name.starts_with("catalog-verified-")));
    }

    #[test]
    fn catalog_corruption_is_rejected_without_replacement_on_retry() {
        let f = Fixture::new();
        let (artifacts, manifest) = f.shard();
        let first = publish(&f.0, &artifacts, &manifest, f.cfg(), "datasets/test").unwrap();
        let catalog = first.catalog.unwrap();
        let destination = f.0.join("remote").join(catalog.key);
        let mut bytes = fs::read(&destination).unwrap();
        bytes[0] ^= 1;
        fs::write(&destination, &bytes).unwrap();
        let err = publish(&f.0, &artifacts, &manifest, f.cfg(), "datasets/test").unwrap_err();
        assert!(err.contains("catalog refuses overwrite"), "{err}");
        assert_eq!(fs::read(destination).unwrap(), bytes);
    }

    #[test]
    fn resumed_publication_rejects_same_size_remote_corruption_without_overwrite() {
        let f = Fixture::new();
        let (artifacts, manifest) = f.shard();
        publish(&f.0, &artifacts, &manifest, f.cfg(), "datasets/test").unwrap();
        let remote_path =
            f.0.join("remote")
                .join(object_key(&artifacts[0].sha256).unwrap());
        fs::write(&remote_path, b"ghijkl").unwrap();
        let err = publish(&f.0, &artifacts, &manifest, f.cfg(), "datasets/test").unwrap_err();
        assert!(err.contains("round-trip"), "{err}");
        assert_eq!(fs::read(remote_path).unwrap(), b"ghijkl");
    }

    #[test]
    fn interrupted_publication_resumes_without_repointing_or_duplicate_entries() {
        let f = Fixture::new();
        let (artifacts, manifest) = f.shard();
        let mut store = Store::open(f.cfg()).unwrap();
        publish_object(
            &mut store,
            &f.0,
            &artifacts[0],
            "datasets/test",
            None,
            BudgetScope::Work,
        )
        .unwrap();
        assert_eq!(store.index.entries.len(), 1);
        assert!(!f.0.join("publication").exists());
        let receipt = publish(&f.0, &artifacts, &manifest, f.cfg(), "datasets/test").unwrap();
        assert!(receipt.completion.roundtrip_verified);
        assert_eq!(Store::open(f.cfg()).unwrap().index.entries.len(), 3);
    }

    #[test]
    fn publication_rejects_unbound_manifest_and_disallowed_bucket_before_writes() {
        let f = Fixture::new();
        let (artifacts, _) = f.shard();
        let manifest = write_json(
            &f.0,
            "bad-manifest.json",
            "manifest",
            &json!({"artifacts": []}),
        )
        .unwrap();
        assert!(
            publish(&f.0, &artifacts, &manifest, f.cfg(), "datasets/test")
                .unwrap_err()
                .contains("exactly match")
        );
        assert!(!f.0.join("remote").exists());
        let mut cfg = f.cfg();
        cfg.remote = Some(RemoteSpec::S3 {
            bucket: "wrong-bucket".into(),
            prefix: String::new(),
        });
        assert!(publish(&f.0, &artifacts, &manifest, cfg, "datasets/test")
            .unwrap_err()
            .contains("only write S3 bucket"));
        assert!(!f.0.join("index.json").exists());
    }
}
