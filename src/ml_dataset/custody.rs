//! Restricted, chunked, content-addressed local custody for an entire run.
//! No cloud transport is implied by a locally verified bundle.

use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use super::storage;

const CHUNK_BYTES: usize = 4 * 1024 * 1024;
const DEFAULT_MAX_BYTES: u64 = 1024 * 1024 * 1024 * 1024;
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Member {
    path: String,
    bytes: u64,
    sha256: String,
    chunks: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
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
        } => {
            let bundle = super::checked_directory(bundle, false)?;
            let manifest = read_manifest(&bundle)?;
            replay_members(&bundle, &manifest, None)?;
            let parent =
                super::checked_directory(destination.parent().unwrap_or(Path::new(".")), false)?;
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
            let replay = super::verify_run(&destination.join("run"), true)?;
            Ok(
                json!({"schema":"crypto.ml-dataset.restricted-custody-restore/v1",
                "destination":destination,"members":manifest.members.len(),"bytes":bytes,
                "native_run_replay":replay,"cloud_requests":0}),
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
