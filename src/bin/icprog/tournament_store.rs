//! Where a tournament round's files are read from: its directory, or the
//! committed evidence archive it was packed into (`evidence/pack.py`).
//!
//! A packed round stores each gzip Callgrind profile expanded, with the
//! original 10-byte header, 8-byte trailer and SHA-256 in PAX fields.  The
//! archive reader rebuilds every such stream with zlib's level-9 raw
//! DEFLATE, exactly as `evidence/restore.py` does, and refuses a member
//! whose rebuilt stream does not hash to its recorded identity.  The round
//! is then held in memory, so a 69,000-file round is verified without being
//! written to disk; its digests are those of the files a restore would
//! write.

use std::collections::BTreeMap;
use std::io::Read;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use sha2::{Digest, Sha256};

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

pub fn digest_file(path: &Path) -> Result<String, String> {
    let mut f = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f
            .read(&mut buf)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(hex(&h.finalize()))
}

// ── zlib, as CPython's `zlib` module drives it ──────────────────────

#[repr(C)]
struct ZStream {
    next_in: *const u8,
    avail_in: c_uint,
    total_in: c_ulong,
    next_out: *mut u8,
    avail_out: c_uint,
    total_out: c_ulong,
    msg: *const c_char,
    state: *mut c_void,
    zalloc: *const c_void,
    zfree: *const c_void,
    opaque: *mut c_void,
    data_type: c_int,
    adler: c_ulong,
    reserved: c_ulong,
}

#[link(name = "z")]
extern "C" {
    fn zlibVersion() -> *const c_char;
    fn deflateInit2_(
        strm: *mut ZStream,
        level: c_int,
        method: c_int,
        window_bits: c_int,
        mem_level: c_int,
        strategy: c_int,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int;
    fn deflate(strm: *mut ZStream, flush: c_int) -> c_int;
    fn deflateEnd(strm: *mut ZStream) -> c_int;
    fn crc32(crc: c_ulong, buf: *const u8, len: c_uint) -> c_ulong;
}

const Z_NO_FLUSH: c_int = 0;
const Z_FINISH: c_int = 4;
const Z_OK: c_int = 0;
const Z_STREAM_END: c_int = 1;
const Z_BUF_ERROR: c_int = -5;

/// `zlib.compressobj(9, zlib.DEFLATED, -15)`, then `compress(data)` and
/// `flush()`: raw DEFLATE at level 9, memory level 8, default strategy.
pub fn deflate_raw9(data: &[u8]) -> Result<Vec<u8>, String> {
    unsafe {
        let mut s: ZStream = std::mem::zeroed();
        let rc = deflateInit2_(
            &mut s,
            9,
            8,
            -15,
            8,
            0,
            zlibVersion(),
            std::mem::size_of::<ZStream>() as c_int,
        );
        if rc != Z_OK {
            return Err(format!("deflateInit2 failed: {rc}"));
        }
        let mut out: Vec<u8> = Vec::with_capacity(data.len() / 4 + 64);
        let mut chunk = vec![0u8; 1 << 16];
        let mut run = |s: &mut ZStream, flush: c_int, out: &mut Vec<u8>| -> Result<(), String> {
            loop {
                s.next_out = chunk.as_mut_ptr();
                s.avail_out = chunk.len() as c_uint;
                let rc = deflate(s, flush);
                if rc != Z_OK && rc != Z_STREAM_END && rc != Z_BUF_ERROR {
                    return Err(format!("deflate failed: {rc}"));
                }
                let produced = chunk.len() - s.avail_out as usize;
                out.extend_from_slice(&chunk[..produced]);
                if flush == Z_FINISH {
                    if rc == Z_STREAM_END {
                        return Ok(());
                    }
                } else if s.avail_out != 0 && s.avail_in == 0 {
                    return Ok(());
                }
            }
        };
        let mut offset = 0usize;
        let mut result = Ok(());
        // CPython feeds at most UINT_MAX bytes per call; profiles are smaller.
        while result.is_ok() && offset < data.len() {
            let take = (data.len() - offset).min(u32::MAX as usize);
            s.next_in = data.as_ptr().add(offset);
            s.avail_in = take as c_uint;
            result = run(&mut s, Z_NO_FLUSH, &mut out);
            offset += take;
        }
        if result.is_ok() {
            s.next_in = std::ptr::null();
            s.avail_in = 0;
            result = run(&mut s, Z_FINISH, &mut out);
        }
        deflateEnd(&mut s);
        result.map(|_| out)
    }
}

fn crc32_of(data: &[u8]) -> u32 {
    let mut crc: c_ulong = 0;
    for part in data.chunks(1 << 30) {
        crc = unsafe { crc32(crc, part.as_ptr(), part.len() as c_uint) };
    }
    crc as u32
}

/// `gzip.compress(data, mtime=0)` as CPython 3.12 writes it: zlib's own
/// level-9 gzip stream with the modification time zeroed and the OS byte
/// 255.  The DEFLATE body is `deflate_raw9`, so a profile this writes is
/// the stream the archive reader rebuilds.
pub fn gzip_mtime0(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = vec![0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 2, 255];
    out.extend(deflate_raw9(data)?);
    out.extend(crc32_of(data).to_le_bytes());
    out.extend((data.len() as u32).to_le_bytes());
    Ok(out)
}

pub fn gunzip(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    flate2::read::MultiGzDecoder::new(bytes)
        .read_to_end(&mut out)
        .map_err(|e| format!("gzip: {e}"))?;
    Ok(out)
}

// ── the store ───────────────────────────────────────────────────────

pub enum Member {
    Plain(Vec<u8>),
    /// A gzip stream held expanded, with the SHA-256 of the stream itself.
    Gzip {
        expanded: Vec<u8>,
        sha256: String,
    },
}

pub enum Store {
    Dir(PathBuf),
    Archive {
        label: String,
        files: BTreeMap<String, Member>,
    },
}

impl Store {
    pub fn label(&self) -> String {
        match self {
            Store::Dir(p) => p.display().to_string(),
            Store::Archive { label, .. } => label.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn root(&self) -> Option<&Path> {
        match self {
            Store::Dir(p) => Some(p),
            Store::Archive { .. } => None,
        }
    }

    pub fn exists(&self, rel: &str) -> bool {
        match self {
            Store::Dir(p) => p.join(rel).is_file(),
            Store::Archive { files, .. } => files.contains_key(rel),
        }
    }

    /// The file's bytes as they are stored on disk (a gzip member is
    /// rebuilt).
    pub fn read(&self, rel: &str) -> Result<Vec<u8>, String> {
        match self {
            Store::Dir(p) => {
                std::fs::read(p.join(rel)).map_err(|e| format!("{}: {e}", p.join(rel).display()))
            }
            Store::Archive { files, label } => match files.get(rel) {
                Some(Member::Plain(b)) => Ok(b.clone()),
                Some(Member::Gzip { .. }) => Err(format!(
                    "{label}: {rel} is a gzip member; read it with gunzip"
                )),
                None => Err(format!("{label}: no file {rel}")),
            },
        }
    }

    pub fn read_text(&self, rel: &str) -> Result<String, String> {
        String::from_utf8(self.read(rel)?).map_err(|e| format!("{rel}: {e}"))
    }

    /// A gzip file's expanded content (`gzip.decompress(path.read_bytes())`).
    pub fn gunzip(&self, rel: &str) -> Result<Vec<u8>, String> {
        match self {
            Store::Dir(p) => gunzip(
                &self
                    .read(rel)
                    .map_err(|e| format!("{}: {e}", p.display()))?,
            ),
            Store::Archive { files, label } => match files.get(rel) {
                Some(Member::Gzip { expanded, .. }) => Ok(expanded.clone()),
                Some(Member::Plain(b)) => gunzip(b),
                None => Err(format!("{label}: no file {rel}")),
            },
        }
    }

    pub fn digest(&self, rel: &str) -> Result<String, String> {
        match self {
            Store::Dir(p) => digest_file(&p.join(rel)),
            Store::Archive { files, label } => match files.get(rel) {
                Some(Member::Plain(b)) => Ok(sha256_hex(b)),
                Some(Member::Gzip { sha256, .. }) => Ok(sha256.clone()),
                None => Err(format!("{label}: no file {rel}")),
            },
        }
    }

    /// Every file under `prefix` (a directory, relative to the round),
    /// sorted, relative to the round.
    pub fn files_under(&self, prefix: &str) -> Vec<String> {
        let prefix = if prefix.is_empty() || prefix.ends_with('/') {
            prefix.to_string()
        } else {
            format!("{prefix}/")
        };
        match self {
            Store::Dir(p) => {
                let mut out = Vec::new();
                let base = p.join(&prefix);
                let mut stack = vec![base];
                while let Some(d) = stack.pop() {
                    let Ok(rd) = std::fs::read_dir(&d) else {
                        continue;
                    };
                    for e in rd.flatten() {
                        let path = e.path();
                        if path.is_dir() {
                            stack.push(path);
                        } else if path.is_file() {
                            if let Ok(rel) = path.strip_prefix(p) {
                                out.push(rel.to_string_lossy().into_owned());
                            }
                        }
                    }
                }
                out.sort();
                out
            }
            Store::Archive { files, .. } => files
                .range(prefix.clone()..)
                .take_while(|(k, _)| k.starts_with(&prefix))
                .map(|(k, _)| k.clone())
                .collect(),
        }
    }
}

/// One archive member's PAX records.
fn pax_records(data: &[u8]) -> Result<BTreeMap<String, String>, String> {
    let mut out = BTreeMap::new();
    let mut i = 0;
    while i < data.len() {
        let space = data[i..]
            .iter()
            .position(|&b| b == b' ')
            .ok_or("bad PAX record")?;
        let len: usize = std::str::from_utf8(&data[i..i + space])
            .ok()
            .and_then(|s| s.parse().ok())
            .ok_or("bad PAX length")?;
        if len == 0 || i + len > data.len() {
            return Err("bad PAX length".into());
        }
        let rec = &data[i + space + 1..i + len - 1];
        let eq = rec
            .iter()
            .position(|&b| b == b'=')
            .ok_or("bad PAX record")?;
        out.insert(
            String::from_utf8_lossy(&rec[..eq]).into_owned(),
            String::from_utf8_lossy(&rec[eq + 1..]).into_owned(),
        );
        i += len;
    }
    Ok(out)
}

fn octal(field: &[u8]) -> Result<u64, String> {
    let s: String = field
        .iter()
        .take_while(|&&b| b != 0)
        .map(|&b| b as char)
        .collect();
    let s = s.trim();
    if s.is_empty() {
        return Ok(0);
    }
    u64::from_str_radix(s, 8).map_err(|_| format!("bad tar number {s:?}"))
}

fn cstr(field: &[u8]) -> String {
    String::from_utf8_lossy(&field[..field.iter().position(|&b| b == 0).unwrap_or(field.len())])
        .into_owned()
}

fn unhex(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err("odd hex".into());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

#[allow(dead_code)]
pub struct ArchiveSummary {
    pub files: u64,
    pub bytes: u64,
}

/// Load the members of `archive` under `member_prefix` (e.g.
/// `runs/round-0020`) into memory.  zstd runs as a child process (thin
/// orchestration; it is the archive's own codec).  When `expected` names a
/// manifest entry, the archive's SHA-256, member count and restored byte
/// count must match it, as `restore.py` requires.
pub fn load_archive(
    archive: &Path,
    member_prefix: &str,
    expected: Option<(&str, u64, u64)>,
) -> Result<(Store, ArchiveSummary), String> {
    if let Some((sha, _, _)) = expected {
        let actual = digest_file(archive)?;
        if actual != sha {
            return Err(format!("archive hash mismatch: {}", archive.display()));
        }
    }
    let mut child = Command::new("zstd")
        .args(["-d", "-q", "-c"])
        .arg(archive)
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("zstd: {e}"))?;
    let mut stream = child.stdout.take().expect("piped");
    let prefix = format!("{}/", member_prefix.trim_end_matches('/'));
    let mut files = BTreeMap::new();
    let (mut count, mut total) = (0u64, 0u64);
    let mut pending_pax: Option<BTreeMap<String, String>> = None;
    let mut header = [0u8; 512];
    let mut zero_blocks = 0;
    let result: Result<(), String> = (|| loop {
        stream
            .read_exact(&mut header)
            .map_err(|e| format!("tar: {e}"))?;
        if header.iter().all(|&b| b == 0) {
            zero_blocks += 1;
            if zero_blocks == 2 {
                return Ok(());
            }
            continue;
        }
        zero_blocks = 0;
        let size = octal(&header[124..136])? as usize;
        let kind = header[156];
        let mut data = vec![0u8; size];
        stream
            .read_exact(&mut data)
            .map_err(|e| format!("tar: {e}"))?;
        let pad = (512 - size % 512) % 512;
        if pad > 0 {
            let mut skip = vec![0u8; pad];
            stream
                .read_exact(&mut skip)
                .map_err(|e| format!("tar: {e}"))?;
        }
        match kind {
            b'x' => {
                pending_pax = Some(pax_records(&data)?);
                continue;
            }
            b'g' => continue,
            b'0' | 0 => {}
            other => {
                return Err(format!("unsafe archive member type {}", other as char));
            }
        }
        let pax = pending_pax.take().unwrap_or_default();
        let mut name = cstr(&header[0..100]);
        let ustar_prefix = cstr(&header[345..500]);
        if &header[257..262] == b"ustar" && !ustar_prefix.is_empty() {
            name = format!("{ustar_prefix}/{name}");
        }
        if let Some(p) = pax.get("path") {
            name = p.clone();
        }
        if name.starts_with('/') || name.split('/').any(|p| p == "..") {
            return Err(format!("unsafe archive member: {name}"));
        }
        let content_len;
        let member = if let Some(sha) = pax.get("ic.gzip.sha256") {
            let head = unhex(pax.get("ic.gzip.header").ok_or("missing gzip header")?)?;
            let tail = unhex(pax.get("ic.gzip.trailer").ok_or("missing gzip trailer")?)?;
            let mut rebuilt = head;
            rebuilt.extend(deflate_raw9(&data)?);
            rebuilt.extend(tail);
            if &sha256_hex(&rebuilt) != sha {
                return Err(format!(
                    "gzip reconstruction differs with this zlib: {name}"
                ));
            }
            content_len = rebuilt.len() as u64;
            Member::Gzip {
                expanded: data,
                sha256: sha.clone(),
            }
        } else {
            content_len = data.len() as u64;
            Member::Plain(data)
        };
        count += 1;
        total += content_len;
        if let Some(rel) = name.strip_prefix(&prefix) {
            files.insert(rel.to_string(), member);
        }
    })();
    // Read the end-of-archive padding too, so zstd is not cut off by a
    // closed pipe.
    let _ = std::io::copy(&mut stream, &mut std::io::sink());
    drop(stream);
    let status = child.wait().map_err(|e| e.to_string())?;
    result?;
    if !status.success() {
        return Err(format!("zstd failed: {}", archive.display()));
    }
    if let Some((_, n, bytes)) = expected {
        if count != n || total != bytes {
            return Err(format!(
                "archive member count or length mismatch: {}",
                archive.display()
            ));
        }
    }
    if files.is_empty() {
        return Err(format!(
            "no members under {prefix} in {}",
            archive.display()
        ));
    }
    Ok((
        Store::Archive {
            label: format!("{}:{}", archive.display(), member_prefix),
            files,
        },
        ArchiveSummary {
            files: count,
            bytes: total,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gzip_round_trips_and_matches_cpythons_header() {
        let data = b"events: Ir\nsummary: 12\ntotals: 12\n".repeat(50);
        let gz = gzip_mtime0(&data).unwrap();
        assert_eq!(&gz[..10], &[0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 2, 255]);
        assert_eq!(gunzip(&gz).unwrap(), data);
        // The rebuilt-stream identity the archive reader checks.
        let mut rebuilt = gz[..10].to_vec();
        rebuilt.extend(deflate_raw9(&data).unwrap());
        rebuilt.extend(&gz[gz.len() - 8..]);
        assert_eq!(rebuilt, gz);
    }

    #[test]
    fn raw_deflate_inflates_back() {
        let data: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8).collect();
        let raw = deflate_raw9(&data).unwrap();
        let mut out = Vec::new();
        flate2::read::DeflateDecoder::new(&raw[..])
            .read_to_end(&mut out)
            .unwrap();
        assert_eq!(out, data);
        assert!(deflate_raw9(b"").unwrap().len() <= 2);
    }

    #[test]
    fn pax_records_parse() {
        let rec = b"30 ic.gzip.sha256=abcdef01234\n12 path=a/b\n";
        let m = pax_records(rec).unwrap();
        assert_eq!(m["ic.gzip.sha256"], "abcdef01234");
        assert_eq!(m["path"], "a/b");
    }
}
