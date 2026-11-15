//! Clean/smudge logic and git's long-running filter protocol
//! (gitprotocol-long-running-process(5), version 2).
//!
//! Policy:
//! * clean (worktree -> index): plaintext is encrypted under the realm's
//!   current epoch; no key is a hard error, so with `filter.<d>.required`
//!   git refuses to stage plaintext. A vault blob passes through unchanged:
//!   that is what a locked worktree holds.
//! * smudge (index -> worktree): a vault blob is decrypted when the key is
//!   present and its embedded path matches; otherwise it is written as is
//!   (locked), with a warning when authentication or the path check failed.
//!   Checkout never fails for lack of a key.

use crate::format::{self, Key};
use crate::keys::{Config, Repo};
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Read, Write};

pub struct Keyring {
    pub repo: Repo,
    pub cfg: Config,
    cache: HashMap<(String, u32), Option<Key>>,
}

impl Keyring {
    pub fn open() -> Result<Self> {
        let repo = Repo::discover()?;
        let cfg = repo.load_config()?;
        Ok(Keyring {
            repo,
            cfg,
            cache: HashMap::new(),
        })
    }
    pub fn get(&mut self, realm: &str, epoch: u32) -> Option<&Key> {
        let repo = &self.repo;
        self.cache
            .entry((realm.to_string(), epoch))
            .or_insert_with(|| repo.cached_key(realm, epoch))
            .as_ref()
    }
    pub fn current_epoch(&self, realm: &str) -> Result<u32> {
        self.cfg
            .realms
            .get(realm)
            .copied()
            .with_context(|| format!("vault: realm {realm:?} is not in .vault/vault.json"))
    }
}

pub fn clean(kr: &mut Keyring, realm: &str, path: &str, input: &[u8]) -> Result<Vec<u8>> {
    if format::is_ciphertext(input) {
        return Ok(input.to_vec());
    }
    let epoch = kr.current_epoch(realm)?;
    let repo_id = kr.cfg.repo.clone();
    let Some(key) = kr.get(realm, epoch) else {
        bail!(
            "vault: refusing to stage {path} in plaintext: no key for realm {realm} epoch {epoch} \
             (run `vault unlock`)"
        );
    };
    format::encrypt(key, &repo_id, realm, epoch, path, input)
}

pub enum Smudged {
    Plain(Vec<u8>),
    Locked(Vec<u8>),
}

pub fn smudge(kr: &mut Keyring, path: &str, input: Vec<u8>) -> Smudged {
    if !format::is_ciphertext(&input) {
        return Smudged::Plain(input);
    }
    match try_open(kr, Some(path), &input) {
        Ok(Some((_, content))) => Smudged::Plain(content),
        Ok(None) => Smudged::Locked(input),
        Err(e) => {
            eprintln!("vault: {path}: {e:#}; leaving it encrypted");
            Smudged::Locked(input)
        }
    }
}

/// Decrypts a vault blob. `Ok(None)` means the key is not available.
/// With `expect_path`, a blob whose embedded path differs is an error.
pub fn try_open(
    kr: &mut Keyring,
    expect_path: Option<&str>,
    blob: &[u8],
) -> Result<Option<(String, Vec<u8>)>> {
    let (h, body) = format::parse(blob)?;
    let repo_id = kr.cfg.repo.clone();
    let Some(key) = kr.get(&h.realm, h.epoch) else {
        return Ok(None);
    };
    let (p, content) = format::decrypt(key, &repo_id, &h, body)?;
    if let Some(want) = expect_path {
        if p != want {
            bail!("blob was encrypted for path {p:?}, not {want:?} (moved without re-staging, or swapped)");
        }
    }
    Ok(Some((p, content)))
}

// ---- pkt-line -------------------------------------------------------------

const MAX_DATA: usize = 65516;

pub enum Pkt {
    Flush,
    Data(Vec<u8>),
}

pub fn read_pkt(r: &mut impl Read) -> Result<Option<Pkt>> {
    let mut len = [0u8; 4];
    match r.read_exact(&mut len) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e.into()),
    }
    let n = usize::from_str_radix(std::str::from_utf8(&len)?, 16)
        .context("vault: bad pkt-line length")?;
    if n == 0 {
        return Ok(Some(Pkt::Flush));
    }
    if !(4..=MAX_DATA + 4).contains(&n) {
        bail!("vault: bad pkt-line length {n}");
    }
    let mut buf = vec![0u8; n - 4];
    r.read_exact(&mut buf)?;
    Ok(Some(Pkt::Data(buf)))
}

pub fn write_data(w: &mut impl Write, data: &[u8]) -> Result<()> {
    for chunk in data.chunks(MAX_DATA) {
        write!(w, "{:04x}", chunk.len() + 4)?;
        w.write_all(chunk)?;
    }
    Ok(())
}

pub fn write_text(w: &mut impl Write, line: &str) -> Result<()> {
    write_data(w, format!("{line}\n").as_bytes())
}

pub fn write_flush(w: &mut impl Write) -> Result<()> {
    w.write_all(b"0000")?;
    Ok(())
}

/// Reads text packets up to a flush. `None` at a clean end of stream.
fn read_list(r: &mut impl Read) -> Result<Option<Vec<String>>> {
    let mut out = vec![];
    loop {
        match read_pkt(r)? {
            None if out.is_empty() => return Ok(None),
            None => bail!("vault: unexpected end of filter stream"),
            Some(Pkt::Flush) => return Ok(Some(out)),
            Some(Pkt::Data(d)) => {
                let s = String::from_utf8(d)?;
                out.push(s.strip_suffix('\n').unwrap_or(&s).to_string());
            }
        }
    }
}

fn read_content(r: &mut impl Read) -> Result<Vec<u8>> {
    let mut out = vec![];
    loop {
        match read_pkt(r)? {
            None => bail!("vault: unexpected end of filter stream"),
            Some(Pkt::Flush) => return Ok(out),
            Some(Pkt::Data(d)) => out.extend_from_slice(&d),
        }
    }
}

pub fn filter_process(realm: &str) -> Result<()> {
    let mut kr = Keyring::open()?;
    format::validate_realm(realm)?;
    let stdin = std::io::stdin();
    let mut r = BufReader::new(stdin.lock());
    let stdout = std::io::stdout();
    let mut w = BufWriter::new(stdout.lock());

    let hello = read_list(&mut r)?.context("vault: no handshake")?;
    if hello.first().map(String::as_str) != Some("git-filter-client")
        || !hello.iter().any(|l| l == "version=2")
    {
        bail!("vault: unsupported filter handshake {hello:?}");
    }
    write_text(&mut w, "git-filter-server")?;
    write_text(&mut w, "version=2")?;
    write_flush(&mut w)?;
    w.flush()?;

    let caps = read_list(&mut r)?.context("vault: no capabilities")?;
    for c in ["capability=clean", "capability=smudge"] {
        if caps.iter().any(|l| l == c) {
            write_text(&mut w, c)?;
        }
    }
    write_flush(&mut w)?;
    w.flush()?;

    while let Some(meta) = read_list(&mut r)? {
        let get = |k: &str| {
            meta.iter()
                .find_map(|l| l.strip_prefix(k).map(str::to_string))
        };
        let command = get("command=").unwrap_or_default();
        let path = get("pathname=").unwrap_or_default();
        let content = read_content(&mut r)?;
        let result = match command.as_str() {
            "clean" => clean(&mut kr, realm, &path, &content),
            "smudge" => Ok(match smudge(&mut kr, &path, content) {
                Smudged::Plain(v) | Smudged::Locked(v) => v,
            }),
            other => Err(anyhow::anyhow!(
                "vault: unsupported filter command {other:?}"
            )),
        };
        match result {
            Ok(out) => {
                write_text(&mut w, "status=success")?;
                write_flush(&mut w)?;
                write_data(&mut w, &out)?;
                write_flush(&mut w)?;
                write_flush(&mut w)?; // empty list: keep status=success
            }
            Err(e) => {
                eprintln!("{e:#}");
                write_text(&mut w, "status=error")?;
                write_flush(&mut w)?;
            }
        }
        w.flush()?;
    }
    Ok(())
}

pub fn read_stdin() -> Result<Vec<u8>> {
    let mut v = vec![];
    std::io::stdin().lock().read_to_end(&mut v)?;
    Ok(v)
}

/// For `vault cat` (diff textconv): decrypt if possible, else show a stub so a
/// locked diff stays readable and binary noise stays out of the terminal.
pub fn textconv(kr: &mut Keyring, blob: &[u8]) -> Vec<u8> {
    if !format::is_ciphertext(blob) {
        return blob.to_vec();
    }
    match try_open(kr, None, blob) {
        Ok(Some((_, c))) => c,
        Ok(None) => match format::parse(blob) {
            Ok((h, _)) => format!(
                "[vault: {} bytes, realm {} epoch {}, locked]\n",
                blob.len(),
                h.realm,
                h.epoch
            )
            .into_bytes(),
            Err(_) => b"[vault: malformed blob]\n".to_vec(),
        },
        Err(e) => format!("[vault: {e:#}]\n").into_bytes(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pkt_roundtrip_large() {
        let data: Vec<u8> = (0..200_000u32).map(|i| i as u8).collect();
        let mut buf = vec![];
        write_data(&mut buf, &data).unwrap();
        write_flush(&mut buf).unwrap();
        let got = read_content(&mut &buf[..]).unwrap();
        assert_eq!(got, data);
    }

    #[test]
    fn pkt_rejects_bad_length() {
        assert!(read_pkt(&mut &b"0002"[..]).is_err());
        assert!(read_pkt(&mut &b"zzzz"[..]).is_err());
        assert!(read_pkt(&mut &b""[..]).unwrap().is_none());
    }
}
