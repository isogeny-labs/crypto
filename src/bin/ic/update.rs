//! macOS self-update for the bundled `ic` + `crax` CLI.
//!
//! The release job publishes an archive named
//! `crypto-cli-<tag>-macos-<arch>.tar.gz` (or `...-macos-universal.tar.gz`)
//! containing `bin/ic`, `bin/crax`, the ECC2K-130 Python engine under
//! `share/cryptocli/ecc2k130/codegen/`, and `share/cryptocli/version.json`.
//! A `SHA256SUMS` asset covers every published file. This command fetches the
//! latest GitHub release, verifies the archive, and replaces the binaries and
//! engine atomically. It never invokes a shell with downloaded data.
use super::VERSION;
use clap::Args;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs,
    io::{IsTerminal, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Args, Clone)]
pub struct UpdateArgs {
    /// Report whether a newer release exists without installing it.
    #[arg(long)]
    pub check: bool,
    /// Install a specific release tag instead of the latest.
    #[arg(long)]
    pub version: Option<String>,
    /// GitHub repository that publishes the macOS bundle.
    #[arg(long, default_value = "isogeny-labs/crypto")]
    pub repo: String,
    /// Directory containing ic and crax; defaults to this executable's directory.
    #[arg(long)]
    pub install_dir: Option<PathBuf>,
    /// Replace binaries even when the release is not newer.
    #[arg(long)]
    pub force: bool,
    /// Download, verify and unpack, but do not replace anything.
    #[arg(long)]
    pub dry_run: bool,
    /// Skip the confirmation prompt.
    #[arg(long)]
    pub yes: bool,
    /// Install from a local archive instead of downloading from GitHub.
    #[arg(long)]
    pub from_tarball: Option<PathBuf>,
    /// Suppress progress lines on stderr.
    #[arg(long)]
    pub quiet: bool,
}

#[derive(Clone, Debug)]
struct Asset {
    name: String,
    url: String,
}

#[derive(Clone, Debug)]
struct Release {
    tag: String,
    assets: Vec<Asset>,
}

pub fn run(args: UpdateArgs) -> Result<Value, String> {
    if !cfg!(target_os = "macos") {
        return Err(
            "ic update is macOS-only; install the release archive manually on this platform".into(),
        );
    }
    let executable = std::env::current_exe()
        .map_err(|error| format!("cannot locate the running executable: {error}"))?;
    let install_dir = args
        .install_dir
        .clone()
        .or_else(|| executable.parent().map(Path::to_path_buf))
        .ok_or("cannot determine the install directory; pass --install-dir")?;
    let (bin_dir, share_dir) = layout(&install_dir);

    let latest = if let Some(path) = &args.from_tarball {
        Release {
            tag: args.version.clone().unwrap_or_else(|| "local".to_string()),
            assets: vec![Asset {
                name: path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("local.tar.gz")
                    .to_string(),
                url: path.display().to_string(),
            }],
        }
    } else {
        fetch_release(&args.repo, args.version.as_deref())?
    };
    let arch = current_arch();
    let asset = choose_asset(&latest.assets, arch)
        .ok_or_else(|| format!("release {} has no macOS {arch} archive", latest.tag))?;

    let newer = args.from_tarball.is_some() || is_newer(VERSION, &latest.tag)?;
    if args.check && !newer {
        return Ok(report(
            "up_to_date",
            &latest.tag,
            &asset.name,
            &bin_dir,
            "already at the latest release",
        ));
    }
    if args.check {
        return Ok(report(
            "update_available",
            &latest.tag,
            &asset.name,
            &bin_dir,
            "a newer release is available; run `ic update`",
        ));
    }
    if !newer && !args.force {
        return Ok(report(
            "up_to_date",
            &latest.tag,
            &asset.name,
            &bin_dir,
            "already at the latest release; pass --force to reinstall",
        ));
    }
    if !args.yes && std::io::stdin().is_terminal() {
        eprint!(
            "Install {} (ic + crax) into {}? [y/N] ",
            latest.tag,
            bin_dir.display()
        );
        let _ = std::io::stderr().flush();
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .map_err(|error| format!("cannot read the confirmation: {error}"))?;
        if !matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes") {
            return Ok(report(
                "cancelled",
                &latest.tag,
                &asset.name,
                &bin_dir,
                "nothing was changed",
            ));
        }
    }

    let work = temporary_directory("cryptocli-update")?;
    let result = (|| {
        let archive = if let Some(path) = &args.from_tarball {
            path.clone()
        } else {
            let path = work.join(&asset.name);
            say(&args, &format!("downloading {}", asset.name));
            download(&asset.url, &path)?;
            let sums_path = work.join("SHA256SUMS");
            say(&args, "downloading SHA256SUMS");
            download(
                &download_url(&args.repo, &latest.tag, "SHA256SUMS"),
                &sums_path,
            )?;
            verify_archive(&path, &sums_path, &asset.name)?;
            path
        };
        say(&args, &format!("unpacking {}", archive.display()));
        let bundle = work.join("bundle");
        unpack(&archive, &bundle)?;
        install_bundle(&bundle, &bin_dir, &share_dir, &latest.tag, args.dry_run)
    })();
    let installed = result;
    let _ = fs::remove_dir_all(&work);
    let installed = installed?;
    let status = if args.dry_run { "dry_run" } else { "installed" };
    let message = if args.dry_run {
        "verified the archive; no files were replaced"
    } else {
        "installed ic and crax; rerun `ic --version` to confirm"
    };
    Ok(report_with_engine(
        status,
        &latest.tag,
        &asset.name,
        &bin_dir,
        message,
        installed,
    ))
}

fn report(status: &str, latest: &str, asset: &str, bin_dir: &Path, message: &str) -> Value {
    report_with_engine(status, latest, asset, bin_dir, message, false)
}

fn report_with_engine(
    status: &str,
    latest: &str,
    asset: &str,
    bin_dir: &Path,
    message: &str,
    engine_installed: bool,
) -> Value {
    json!({
        "schema_version": 1,
        "operation": "update",
        "status": status,
        "current": VERSION,
        "latest": latest,
        "asset": asset,
        "install_dir": bin_dir.display().to_string(),
        "engine_installed": engine_installed,
        "message": message,
    })
}

fn current_arch() -> &'static str {
    if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else {
        "unknown"
    }
}

fn choose_asset(assets: &[Asset], arch: &str) -> Option<Asset> {
    let archive = |asset: &Asset| {
        asset.name.starts_with("crypto-cli-")
            && asset.name.ends_with(".tar.gz")
            && !asset.name.contains("SHA256SUMS")
    };
    assets
        .iter()
        .find(|asset| archive(asset) && asset.name.contains("macos-universal"))
        .or_else(|| {
            assets.iter().find(|asset| {
                archive(asset)
                    && (asset.name.contains(&format!("macos-{arch}"))
                        || (arch == "aarch64" && asset.name.contains("macos-arm64"))
                        || (arch == "x86_64" && asset.name.contains("macos-amd64")))
            })
        })
        .cloned()
}

fn fetch_release(repo: &str, version: Option<&str>) -> Result<Release, String> {
    let url = match version {
        Some(version) => format!("https://api.github.com/repos/{repo}/releases/tags/{version}"),
        None => format!("https://api.github.com/repos/{repo}/releases/latest"),
    };
    let body = curl(&url, None)?;
    let document: Value = serde_json::from_slice(&body)
        .map_err(|error| format!("GitHub returned invalid release JSON: {error}"))?;
    let tag = document["tag_name"]
        .as_str()
        .ok_or("GitHub release has no tag_name")?
        .to_string();
    let assets = document["assets"]
        .as_array()
        .ok_or("GitHub release has no assets")?
        .iter()
        .filter_map(|asset| {
            Some(Asset {
                name: asset["name"].as_str()?.to_string(),
                url: asset["browser_download_url"].as_str()?.to_string(),
            })
        })
        .collect();
    Ok(Release { tag, assets })
}

fn download_url(repo: &str, tag: &str, asset: &str) -> String {
    format!("https://github.com/{repo}/releases/download/{tag}/{asset}")
}

fn curl(url: &str, output: Option<&Path>) -> Result<Vec<u8>, String> {
    let mut command = Command::new("curl");
    command.args(["-fsSL", "--retry", "3", "--max-time", "120"]);
    if let Some(token) = github_token() {
        command
            .arg("-H")
            .arg(format!("Authorization: Bearer {token}"));
    }
    if let Some(path) = output {
        command.arg("-o").arg(path);
    }
    command.arg(url);
    command.stdin(Stdio::null());
    let result = command
        .output()
        .map_err(|error| format!("cannot run curl: {error}"))?;
    if !result.status.success() {
        return Err(format!(
            "curl failed for {url}: {}",
            String::from_utf8_lossy(&result.stderr).trim()
        ));
    }
    Ok(result.stdout)
}

fn download(url: &str, path: &Path) -> Result<(), String> {
    curl(url, Some(path))?;
    Ok(())
}

fn github_token() -> Option<String> {
    std::env::var("GH_TOKEN")
        .or_else(|_| std::env::var("GITHUB_TOKEN"))
        .ok()
        .filter(|token| !token.trim().is_empty())
}

fn verify_archive(path: &Path, sums: &Path, name: &str) -> Result<(), String> {
    let text = fs::read_to_string(sums)
        .map_err(|error| format!("cannot read {}: {error}", sums.display()))?;
    let expected = parse_sums(&text)
        .remove(name)
        .ok_or_else(|| format!("SHA256SUMS has no entry for {name}"))?;
    let actual = sha256(path)?;
    if actual != expected {
        return Err(format!(
            "checksum mismatch for {name}: expected {expected}, got {actual}"
        ));
    }
    Ok(())
}

fn parse_sums(text: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for line in text.lines() {
        let mut fields = line.split_whitespace();
        let (Some(hash), Some(name)) = (fields.next(), fields.next()) else {
            continue;
        };
        let name = name.trim_start_matches('*').to_string();
        if hash.len() == 64 && hash.chars().all(|ch| ch.is_ascii_hexdigit()) {
            result.insert(name, hash.to_ascii_lowercase());
        }
    }
    result
}

fn sha256(path: &Path) -> Result<String, String> {
    let mut file =
        fs::File::open(path).map_err(|error| format!("cannot open {}: {error}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 1 << 20];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn unpack(archive: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination)
        .map_err(|error| format!("cannot create {}: {error}", destination.display()))?;
    let status = Command::new("tar")
        .arg("-xzf")
        .arg(archive)
        .arg("-C")
        .arg(destination)
        .status()
        .map_err(|error| format!("cannot run tar: {error}"))?;
    if !status.success() {
        return Err(format!("tar failed for {}", archive.display()));
    }
    Ok(())
}

fn find_named(root: &Path, name: &str) -> Option<PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let Ok(entries) = fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(kind) = entry.file_type() else {
                continue;
            };
            if kind.is_symlink() {
                continue;
            }
            if kind.is_dir() {
                pending.push(path);
            } else if kind.is_file()
                && path.file_name().and_then(|name| name.to_str()) == Some(name)
            {
                return Some(path);
            }
        }
    }
    None
}

fn layout(install_dir: &Path) -> (PathBuf, PathBuf) {
    let bin = install_dir.to_path_buf();
    let share = if bin.file_name().and_then(|name| name.to_str()) == Some("bin") {
        bin.parent()
            .unwrap_or_else(|| Path::new("/"))
            .join("share/cryptocli")
    } else {
        bin.join(".cryptocli")
    };
    (bin, share)
}

fn install_bundle(
    bundle: &Path,
    bin_dir: &Path,
    share_dir: &Path,
    tag: &str,
    dry_run: bool,
) -> Result<bool, String> {
    let ic = find_named(bundle, "ic").ok_or("archive does not contain ic")?;
    let crax = find_named(bundle, "crax").ok_or("archive does not contain crax")?;
    let share =
        find_named(bundle, "version.json").and_then(|path| path.parent().map(Path::to_path_buf));
    let engine = find_named(bundle, "indexcalc_fixed.py")
        .and_then(|path| path.parent().map(Path::to_path_buf));
    if dry_run {
        return Ok(engine.is_some() || share.is_some());
    }
    fs::create_dir_all(bin_dir)
        .map_err(|error| format!("cannot create {}: {error}", bin_dir.display()))?;
    if !writable(bin_dir) {
        return Err(format!(
            "{} is not writable; rerun with sudo: sudo {} update",
            bin_dir.display(),
            std::env::current_exe()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|_| "ic".to_string())
        ));
    }
    copy_atomic(&ic, bin_dir, "ic")?;
    copy_atomic(&crax, bin_dir, "crax")?;
    let engine_installed = if let Some(share) = share {
        copy_directory(&share, share_dir)?;
        share_dir
            .join("ecc2k130/codegen/indexcalc_fixed.py")
            .is_file()
    } else if let Some(engine) = engine {
        copy_directory(&engine, &share_dir.join("ecc2k130/codegen"))?;
        true
    } else {
        false
    };
    write_manifest(share_dir, tag)?;
    Ok(engine_installed)
}

fn writable(directory: &Path) -> bool {
    let probe = directory.join(format!(".ic-write-probe-{}", std::process::id()));
    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&probe)
    {
        Ok(_) => {
            let _ = fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}

fn copy_atomic(source: &Path, directory: &Path, name: &str) -> Result<(), String> {
    let temporary = directory.join(format!(".{name}.new.{}", std::process::id()));
    fs::copy(source, &temporary).map_err(|error| format!("cannot stage {name}: {error}"))?;
    set_executable(&temporary)?;
    fs::rename(&temporary, directory.join(name))
        .map_err(|error| format!("cannot replace {name}: {error}"))?;
    Ok(())
}

fn set_executable(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755))
            .map_err(|error| format!("cannot chmod {}: {error}", path.display()))?;
    }
    Ok(())
}

fn copy_directory(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination)
        .map_err(|error| format!("cannot create {}: {error}", destination.display()))?;
    let entries = fs::read_dir(source)
        .map_err(|error| format!("cannot read {}: {error}", source.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("cannot read {}: {error}", source.display()))?;
        let kind = entry
            .file_type()
            .map_err(|error| format!("cannot inspect {}: {error}", entry.path().display()))?;
        let target = destination.join(entry.file_name());
        if kind.is_dir() {
            copy_directory(&entry.path(), &target)?;
        } else if kind.is_file() {
            fs::copy(entry.path(), &target)
                .map_err(|error| format!("cannot copy {}: {error}", entry.path().display()))?;
        }
    }
    Ok(())
}

fn write_manifest(share_dir: &Path, tag: &str) -> Result<(), String> {
    fs::create_dir_all(share_dir)
        .map_err(|error| format!("cannot create {}: {error}", share_dir.display()))?;
    if !writable(share_dir) {
        return Err(format!(
            "{} is not writable; rerun with sudo",
            share_dir.display()
        ));
    }
    let document = json!({
        "schema_version": 1,
        "operation": "cryptocli-install",
        "version": tag,
        "binaries": ["ic", "crax"],
        "engine": "ecc2k130/codegen",
    });
    let temporary = share_dir.join(format!(".version.json.new.{}", std::process::id()));
    fs::write(
        &temporary,
        serde_json::to_vec_pretty(&document).map_err(|error| error.to_string())?,
    )
    .map_err(|error| format!("cannot write the version manifest: {error}"))?;
    fs::rename(&temporary, share_dir.join("version.json"))
        .map_err(|error| format!("cannot replace the version manifest: {error}"))?;
    Ok(())
}

fn parse_version(text: &str) -> Option<(u64, u64, u64)> {
    let text = text.trim();
    let text = text.strip_prefix('v').unwrap_or(text);
    let core = text.split(['-', '+']).next()?;
    let mut parts = core.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().unwrap_or("0").parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

fn is_newer(current: &str, latest: &str) -> Result<bool, String> {
    let current = parse_version(current)
        .ok_or_else(|| format!("cannot compare the current version {current:?}; pass --force"))?;
    let latest = parse_version(latest)
        .ok_or_else(|| format!("cannot compare the release version {latest:?}; pass --force"))?;
    Ok(latest > current)
}

fn temporary_directory(prefix: &str) -> Result<PathBuf, String> {
    let base = std::env::temp_dir();
    for attempt in 0..100u32 {
        let path = base.join(format!("{prefix}-{}-{attempt}", std::process::id()));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(format!("cannot create {}: {error}", path.display())),
        }
    }
    Err("cannot create a temporary directory".into())
}

fn say(args: &UpdateArgs, message: &str) {
    if !args.quiet {
        eprintln!("ic: {message}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asset(name: &str) -> Asset {
        Asset {
            name: name.to_string(),
            url: format!("https://example.invalid/{name}"),
        }
    }

    #[test]
    fn versions_compare_without_a_v_prefix() {
        assert!(is_newer("0.1.0", "v0.1.6201").unwrap());
        assert!(!is_newer("0.1.6201", "v0.1.6201").unwrap());
        assert!(!is_newer("0.1.6202", "v0.1.6201").unwrap());
        assert!(is_newer("1.0.0", "v1.0.1-rc1").unwrap());
        assert!(parse_version("v0.1.6201").is_some());
        assert!(parse_version("not-a-version").is_none());
    }

    #[test]
    fn versions_compare_exhaustively_as_tuples() {
        for major_a in 0..=4u64 {
            for minor_a in 0..=3u64 {
                for patch_a in 0..=3u64 {
                    for major_b in 0..=4u64 {
                        for minor_b in 0..=3u64 {
                            for patch_b in 0..=3u64 {
                                let a = format!("{major_a}.{minor_a}.{patch_a}");
                                let b = format!("v{major_b}.{minor_b}.{patch_b}");
                                assert_eq!(
                                    is_newer(&a, &b).unwrap(),
                                    (major_b, minor_b, patch_b) > (major_a, minor_a, patch_a),
                                    "{a} -> {b}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn layout_uses_the_prefix_share_tree_only_for_bin() {
        let (bin, share) = layout(Path::new("/usr/local/bin"));
        assert_eq!(bin, PathBuf::from("/usr/local/bin"));
        assert_eq!(share, PathBuf::from("/usr/local/share/cryptocli"));
        let (bin, share) = layout(Path::new("/tmp/target/release"));
        assert_eq!(bin, PathBuf::from("/tmp/target/release"));
        assert_eq!(share, PathBuf::from("/tmp/target/release/.cryptocli"));
    }

    #[test]
    fn selects_the_native_or_universal_archive() {
        let assets = vec![
            asset("crypto-cli-v1-macos-x86_64.tar.gz"),
            asset("crypto-cli-v1-macos-aarch64.tar.gz"),
            asset("SHA256SUMS"),
        ];
        assert_eq!(
            choose_asset(&assets, "aarch64").unwrap().name,
            "crypto-cli-v1-macos-aarch64.tar.gz"
        );
        assert_eq!(
            choose_asset(&assets, "x86_64").unwrap().name,
            "crypto-cli-v1-macos-x86_64.tar.gz"
        );
        let universal = vec![
            asset("crypto-cli-v1-macos-aarch64.tar.gz"),
            asset("crypto-cli-v1-macos-universal.tar.gz"),
        ];
        assert_eq!(
            choose_asset(&universal, "aarch64").unwrap().name,
            "crypto-cli-v1-macos-universal.tar.gz"
        );
        let rejected = vec![
            asset("SHA256SUMS"),
            asset("crypto-cli-v1-macos-aarch64.zip"),
            asset("not-crypto-cli-macos-aarch64.tar.gz"),
            asset("crypto-cli-v1-linux-aarch64.tar.gz"),
        ];
        assert!(choose_asset(&rejected, "aarch64").is_none());
    }

    #[test]
    fn parses_checksum_manifest_lines() {
        let sums = parse_sums(&format!(
            "{}  crypto-cli-v1-macos-aarch64.tar.gz\n{} *ic\n",
            "a".repeat(64),
            "b".repeat(64)
        ));
        assert_eq!(
            sums.get("crypto-cli-v1-macos-aarch64.tar.gz").unwrap(),
            &"a".repeat(64)
        );
        assert_eq!(sums.get("ic").unwrap(), &"b".repeat(64));
    }

    #[test]
    fn installs_binaries_and_engine_from_a_bundle() {
        let root = temporary_directory("cryptocli-update-test").unwrap();
        let bundle = root.join("bundle");
        let bin = bundle.join("bin");
        let engine = bundle.join("share/cryptocli/ecc2k130/codegen");
        fs::create_dir_all(&bin).unwrap();
        fs::create_dir_all(&engine).unwrap();
        fs::write(bin.join("ic"), b"#!/bin/sh\necho ic\n").unwrap();
        fs::write(bin.join("crax"), b"#!/bin/sh\necho crax\n").unwrap();
        fs::write(engine.join("indexcalc_fixed.py"), b"# engine\n").unwrap();
        let install = root.join("install/bin");
        let (bin_dir, share_dir) = layout(&install);
        assert!(install_bundle(&bundle, &bin_dir, &share_dir, "v0.1.9", false).unwrap());
        assert_eq!(
            fs::read(bin_dir.join("ic")).unwrap(),
            b"#!/bin/sh\necho ic\n"
        );
        assert_eq!(
            fs::read(bin_dir.join("crax")).unwrap(),
            b"#!/bin/sh\necho crax\n"
        );
        assert!(share_dir
            .join("ecc2k130/codegen/indexcalc_fixed.py")
            .is_file());
        assert!(share_dir.join("version.json").is_file());
        let _ = fs::remove_dir_all(root);
    }
}
