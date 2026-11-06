//! Small shared pieces: the deterministic generator, hashing, and the
//! append-only JSON-lines record.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// SplitMix64.  The schedule's shuffle and the bootstrap use it, so a seed
/// reproduces both on any host and any toolchain; `rand`'s generators are
/// allowed to change between versions, a recorded plan is not.
pub struct SplitMix64(u64);

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        SplitMix64(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `0..n` up to the modulo bias of a 64-bit draw, which for
    /// the sizes used here (hundreds) is below 2^-55.
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }

    /// Fisher–Yates, from the back.
    pub fn shuffle<T>(&mut self, xs: &mut [T]) {
        for i in (1..xs.len()).rev() {
            let j = self.below(i + 1);
            xs.swap(i, j);
        }
    }
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex(&h.finalize())
}

pub fn sha256_file(path: &Path) -> io::Result<String> {
    let mut h = Sha256::new();
    let mut f = File::open(path)?;
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(hex(&h.finalize()))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn unix_time() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

/// Appends one JSON value as a line and forces it to disk, so a crash keeps
/// every row that was written before it.
pub fn append_jsonl(file: &mut File, v: &Value) -> io::Result<()> {
    let mut line = serde_json::to_string(v).map_err(io::Error::other)?;
    line.push('\n');
    file.write_all(line.as_bytes())?;
    file.flush()?;
    file.sync_all()
}

/// Reads a JSON-lines file.  A torn last line (a crash in the middle of a
/// write) is dropped; a bad line anywhere else is an error.
pub fn read_jsonl(path: &Path) -> Result<Vec<Value>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::with_capacity(lines.len());
    for (i, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<Value>(line) {
            Ok(v) => out.push(v),
            Err(e) if i + 1 == lines.len() => {
                let _ = e; // torn final line
            }
            Err(e) => {
                return Err(format!(
                    "{}: line {} is not JSON and is not the last line ({e})",
                    path.display(),
                    i + 1
                ))
            }
        }
    }
    Ok(out)
}

pub fn read_json(path: &Path) -> Result<Value, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

pub fn write_json(path: &Path, v: &Value) -> Result<(), String> {
    let mut text = serde_json::to_string_pretty(v).map_err(|e| e.to_string())?;
    text.push('\n');
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Numeric field of a JSON object as `f64`.
pub fn num(v: &Value, key: &str) -> Option<f64> {
    v.get(key).and_then(Value::as_f64)
}

#[cfg(test)]
mod tests {
    use super::tests_support::tempdir;
    use super::*;

    #[test]
    fn splitmix_matches_the_published_stream() {
        // Seed 1: the first outputs of the reference implementation.
        let mut r = SplitMix64::new(1);
        assert_eq!(r.next_u64(), 10451216379200822465);
        assert_eq!(r.next_u64(), 13757245211066428519);
        assert_eq!(r.next_u64(), 17911839290282890590);
    }

    #[test]
    fn shuffle_is_the_fixed_permutation_the_oracle_gives() {
        let mut r = SplitMix64::new(0xdeadbeef);
        let mut xs: Vec<u32> = (0..10).collect();
        r.shuffle(&mut xs);
        assert_eq!(xs, vec![4, 2, 1, 3, 6, 0, 9, 5, 8, 7]);
    }

    #[test]
    fn sha256_known_answer() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn a_torn_last_line_is_dropped_but_a_bad_middle_line_is_not() {
        let dir = tempdir("jsonl");
        let p = dir.join("a.jsonl");
        std::fs::write(&p, "{\"a\":1}\n{\"b\":2}\n{\"c\"").unwrap();
        assert_eq!(read_jsonl(&p).unwrap().len(), 2);
        std::fs::write(&p, "{\"broken\n{\"a\":1}\n").unwrap();
        assert!(read_jsonl(&p).is_err());
        assert!(read_jsonl(&dir.join("missing")).unwrap().is_empty());
    }

    #[test]
    fn append_is_line_per_value() {
        let dir = tempdir("append");
        let p = dir.join("r.jsonl");
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&p)
            .unwrap();
        append_jsonl(&mut f, &serde_json::json!({"x": 1})).unwrap();
        append_jsonl(&mut f, &serde_json::json!({"x": 2})).unwrap();
        assert_eq!(read_jsonl(&p).unwrap().len(), 2);
    }
}

#[cfg(test)]
pub mod tests_support {
    use std::path::PathBuf;

    /// A fresh directory under the system temp dir, removed by the OS later.
    pub fn tempdir(tag: &str) -> PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        let d = std::env::temp_dir().join(format!(
            "perfindex-{tag}-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&d).unwrap();
        d
    }
}
