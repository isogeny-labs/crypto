//! Repository configuration, wrapped data keys and the local key cache.
//!
//! * `.vault/vault.json` (committed): repo id, KMS key, region, each realm's
//!   current epoch, and the globs that must never be committed in plaintext.
//! * `.vault/keys/<realm>.<epoch>.kms` (committed): the realm's 64-byte data
//!   key wrapped by AWS KMS under the encryption context
//!   `{repo, realm, epoch}`. Safe to publish; only KMS can unwrap it, and the
//!   key policy can grant each realm to different principals.
//! * `<git-common-dir>/vault/<realm>.<epoch>.key` (never committed, mode 0600):
//!   the unwrapped key, written by `vault unlock`.
//!
//! KMS is reached through the `aws` CLI (`$VAULT_AWS` overrides the program),
//! which keeps the AWS SDK out of the binary and reuses the caller's identity:
//! the session's role in Claude sessions, the OIDC role in GitHub Actions.

use crate::format::{Key, KEY_LEN};
use anyhow::{bail, ensure, Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub const CONFIG_REL: &str = ".vault/vault.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub version: u32,
    /// `owner/name`; bound into every blob's associated data.
    pub repo: String,
    pub kms_key_id: String,
    pub region: String,
    /// Realm name to current epoch.
    #[serde(default)]
    pub realms: BTreeMap<String, u32>,
    /// Paths that must be vault blobs in every commit, independently of
    /// `.gitattributes`, so deleting an attribute line cannot silently turn
    /// encryption off.
    #[serde(default)]
    pub protect: Vec<String>,
}

pub struct Repo {
    pub root: PathBuf,
    pub common_dir: PathBuf,
}

impl Repo {
    pub fn discover() -> Result<Self> {
        let root = git_out(&["rev-parse", "--show-toplevel"])?;
        let common = git_out(&["rev-parse", "--path-format=absolute", "--git-common-dir"])?;
        Ok(Repo {
            root: PathBuf::from(root.trim()),
            common_dir: PathBuf::from(common.trim()),
        })
    }
    pub fn config_path(&self) -> PathBuf {
        self.root.join(CONFIG_REL)
    }
    pub fn load_config(&self) -> Result<Config> {
        let p = self.config_path();
        let text = std::fs::read_to_string(&p)
            .with_context(|| format!("vault: reading {}", p.display()))?;
        let c: Config = serde_json::from_str(&text)
            .with_context(|| format!("vault: parsing {}", p.display()))?;
        ensure!(
            c.version == 1,
            "vault: unsupported config version {}",
            c.version
        );
        for r in c.realms.keys() {
            crate::format::validate_realm(r)?;
        }
        Ok(c)
    }
    pub fn save_config(&self, c: &Config) -> Result<()> {
        let p = self.config_path();
        std::fs::create_dir_all(p.parent().unwrap())?;
        std::fs::write(&p, serde_json::to_string_pretty(c)? + "\n")?;
        Ok(())
    }
    pub fn wrapped_path(&self, realm: &str, epoch: u32) -> PathBuf {
        self.root
            .join(".vault/keys")
            .join(format!("{realm}.{epoch}.kms"))
    }
    pub fn cache_dir(&self) -> PathBuf {
        self.common_dir.join("vault")
    }
    pub fn cache_path(&self, realm: &str, epoch: u32) -> PathBuf {
        self.cache_dir().join(format!("{realm}.{epoch}.key"))
    }

    pub fn cached_key(&self, realm: &str, epoch: u32) -> Option<Key> {
        let bytes = zeroize::Zeroizing::new(std::fs::read(self.cache_path(realm, epoch)).ok()?);
        if bytes.len() != KEY_LEN {
            return None;
        }
        let mut k = zeroize::Zeroizing::new([0u8; KEY_LEN]);
        k.copy_from_slice(&bytes);
        Some(k)
    }

    pub fn store_cached_key(&self, realm: &str, epoch: u32, key: &Key) -> Result<()> {
        let dir = self.cache_dir();
        std::fs::create_dir_all(&dir)?;
        set_mode(&dir, 0o700)?;
        let p = self.cache_path(realm, epoch);
        let tmp = p.with_extension("tmp");
        {
            let mut f = open_private(&tmp)?;
            f.write_all(key.as_slice())?;
            f.sync_all()?;
        }
        std::fs::rename(&tmp, &p)?;
        Ok(())
    }

    /// Every (realm, epoch) with a committed wrapped key.
    pub fn wrapped_epochs(&self, realm: &str) -> Result<Vec<u32>> {
        let dir = self.root.join(".vault/keys");
        let mut out = vec![];
        if let Ok(rd) = std::fs::read_dir(&dir) {
            for e in rd {
                let name = e?.file_name().to_string_lossy().into_owned();
                if let Some(rest) = name.strip_prefix(&format!("{realm}.")) {
                    if let Some(ep) = rest.strip_suffix(".kms").and_then(|s| s.parse().ok()) {
                        out.push(ep);
                    }
                }
            }
        }
        out.sort();
        Ok(out)
    }
}

fn open_private(p: &Path) -> Result<std::fs::File> {
    let mut o = std::fs::OpenOptions::new();
    o.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600);
    }
    Ok(o.open(p)?)
}

fn set_mode(p: &Path, mode: u32) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(p, std::fs::Permissions::from_mode(mode))?;
    }
    let _ = (p, mode);
    Ok(())
}

pub fn git_out(args: &[&str]) -> Result<String> {
    let o = Command::new("git")
        .args(args)
        .stderr(Stdio::inherit())
        .output()?;
    ensure!(o.status.success(), "vault: git {} failed", args.join(" "));
    Ok(String::from_utf8(o.stdout)?)
}

fn context_json(cfg: &Config, realm: &str, epoch: u32) -> String {
    serde_json::json!({ "repo": cfg.repo, "realm": realm, "epoch": epoch.to_string() }).to_string()
}

fn aws() -> Command {
    Command::new(std::env::var_os("VAULT_AWS").unwrap_or_else(|| "aws".into()))
}

fn run_aws(mut cmd: Command, stdin: Option<&[u8]>) -> Result<serde_json::Value> {
    cmd.stdin(if stdin.is_some() {
        Stdio::piped()
    } else {
        Stdio::null()
    })
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
    let mut child = cmd
        .spawn()
        .context("vault: cannot run the aws CLI (set VAULT_AWS or install awscli)")?;
    if let Some(data) = stdin {
        child.stdin.take().unwrap().write_all(data)?;
    }
    let o = child.wait_with_output()?;
    if !o.status.success() {
        bail!(
            "vault: aws kms failed: {}",
            String::from_utf8_lossy(&o.stderr).trim()
        );
    }
    serde_json::from_slice(&o.stdout).context("vault: aws returned non-JSON")
}

fn b64_field(v: &serde_json::Value, name: &str) -> Result<zeroize::Zeroizing<Vec<u8>>> {
    let s = v
        .get(name)
        .and_then(|x| x.as_str())
        .with_context(|| format!("vault: aws output lacks {name}"))?;
    Ok(zeroize::Zeroizing::new(B64.decode(s)?))
}

/// Asks KMS for a fresh 64-byte data key; returns (plaintext key, wrapped blob).
pub fn kms_generate(cfg: &Config, realm: &str, epoch: u32) -> Result<(Key, Vec<u8>)> {
    let mut cmd = aws();
    cmd.args([
        "kms",
        "generate-data-key",
        "--output",
        "json",
        "--region",
        &cfg.region,
    ])
    .args([
        "--key-id",
        &cfg.kms_key_id,
        "--number-of-bytes",
        &KEY_LEN.to_string(),
    ])
    .args(["--encryption-context", &context_json(cfg, realm, epoch)]);
    let v = run_aws(cmd, None)?;
    let pt = b64_field(&v, "Plaintext")?;
    let blob = b64_field(&v, "CiphertextBlob")?;
    ensure!(
        pt.len() == KEY_LEN,
        "vault: KMS returned a {}-byte key",
        pt.len()
    );
    let mut k = zeroize::Zeroizing::new([0u8; KEY_LEN]);
    k.copy_from_slice(&pt);
    Ok((k, blob.to_vec()))
}

/// Unwraps a committed key with KMS. The encryption context must match, so a
/// wrapped key copied to another realm, epoch or repository does not unwrap.
pub fn kms_unwrap(cfg: &Config, realm: &str, epoch: u32, wrapped: &[u8]) -> Result<Key> {
    let mut cmd = aws();
    cmd.args([
        "kms",
        "decrypt",
        "--output",
        "json",
        "--region",
        &cfg.region,
    ])
    .args([
        "--key-id",
        &cfg.kms_key_id,
        "--ciphertext-blob",
        "fileb:///dev/stdin",
    ])
    .args(["--encryption-context", &context_json(cfg, realm, epoch)]);
    let v = run_aws(cmd, Some(wrapped))?;
    let pt = b64_field(&v, "Plaintext")?;
    ensure!(
        pt.len() == KEY_LEN,
        "vault: KMS returned a {}-byte key",
        pt.len()
    );
    let mut k = zeroize::Zeroizing::new([0u8; KEY_LEN]);
    k.copy_from_slice(&pt);
    Ok(k)
}

pub fn read_wrapped(p: &Path) -> Result<Vec<u8>> {
    let text =
        std::fs::read_to_string(p).with_context(|| format!("vault: reading {}", p.display()))?;
    Ok(B64.decode(text.trim())?)
}

pub fn write_wrapped(p: &Path, blob: &[u8]) -> Result<()> {
    std::fs::create_dir_all(p.parent().unwrap())?;
    std::fs::write(p, B64.encode(blob) + "\n")?;
    Ok(())
}

/// Break-glass key files are hex, one key per file.
pub fn read_key_file(p: &Path) -> Result<Key> {
    let text = zeroize::Zeroizing::new(std::fs::read_to_string(p)?);
    let t = text.trim();
    ensure!(
        t.len() == 2 * KEY_LEN,
        "vault: key file must hold {} hex characters",
        2 * KEY_LEN
    );
    let mut k = zeroize::Zeroizing::new([0u8; KEY_LEN]);
    for i in 0..KEY_LEN {
        k[i] =
            u8::from_str_radix(&t[2 * i..2 * i + 2], 16).context("vault: key file is not hex")?;
    }
    Ok(k)
}

pub fn key_hex(k: &Key) -> zeroize::Zeroizing<String> {
    zeroize::Zeroizing::new(k.iter().map(|b| format!("{b:02x}")).collect())
}
