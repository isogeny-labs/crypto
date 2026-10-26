//! # Checkpoints for the long `u128` runs (`ic_wide`).
//!
//! `ecbench` cannot resume a session, and a run of 7–24 hours does not
//! survive a host that restarts every few hours.  This module lets the two
//! long methods save their state and continue it, **without changing a
//! single count**: a resumed run records exactly the counters of an
//! uninterrupted one (`ic_wide`'s tests compare every counter), and
//! `ecbench verify --replay-all` re-executes from scratch with no
//! checkpoint and must agree.
//!
//! Opt-in by directory: the measured child looks for `./ecbench-ckpt/`
//! (its working directory is the repository root, and its environment is
//! fixed by the harness, so a variable cannot carry the setting).  A
//! checkpoint is keyed by the method, the workload and the seed, is
//! written atomically (temporary file, then rename), and is deleted when
//! the run completes, so a later replay never resumes.
//!
//! What a resume does not preserve is wall time: the solver's wall clock
//! is restored from the checkpoint, the rest of a run's clock covers the
//! resumed stretch only.  Wall time was never a claim of these runs.

use std::cell::Cell;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// A growing byte buffer, little-endian.
#[derive(Default)]
pub struct Wr(pub Vec<u8>);

impl Wr {
    pub fn u8(&mut self, v: u8) {
        self.0.push(v);
    }
    pub fn u64(&mut self, v: u64) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }
    pub fn u128(&mut self, v: u128) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }
    /// LEB128.
    pub fn varint(&mut self, mut v: u64) {
        loop {
            let b = (v & 0x7f) as u8;
            v >>= 7;
            if v == 0 {
                self.0.push(b);
                return;
            }
            self.0.push(b | 0x80);
        }
    }
    pub fn bytes(&mut self, b: &[u8]) {
        self.varint(b.len() as u64);
        self.0.extend_from_slice(b);
    }
    pub fn str(&mut self, s: &str) {
        self.bytes(s.as_bytes());
    }
}

/// A reader over a byte slice; every read can fail.
pub struct Rd<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> Rd<'a> {
    pub fn new(b: &'a [u8]) -> Self {
        Rd { b, p: 0 }
    }
    pub fn pos(&self) -> usize {
        self.p
    }
    pub fn at_end(&self) -> bool {
        self.p >= self.b.len()
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let s = self
            .b
            .get(self.p..self.p + n)
            .ok_or_else(|| "checkpoint truncated".to_string())?;
        self.p += n;
        Ok(s)
    }
    pub fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    pub fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    pub fn u128(&mut self) -> Result<u128, String> {
        Ok(u128::from_le_bytes(self.take(16)?.try_into().unwrap()))
    }
    pub fn varint(&mut self) -> Result<u64, String> {
        let (mut v, mut shift) = (0u64, 0u32);
        loop {
            let b = self.u8()?;
            v |= u64::from(b & 0x7f) << shift;
            if b & 0x80 == 0 {
                return Ok(v);
            }
            shift += 7;
            if shift > 63 {
                return Err("checkpoint: varint too long".into());
            }
        }
    }
    pub fn bytes(&mut self) -> Result<&'a [u8], String> {
        let n = self.varint()? as usize;
        self.take(n)
    }
    pub fn str(&mut self) -> Result<String, String> {
        String::from_utf8(self.bytes()?.to_vec()).map_err(|e| e.to_string())
    }
}

/// Where and how often a run checkpoints.
pub struct Ckpt {
    pub dir: PathBuf,
    pub key: String,
    pub interval: Duration,
    /// Test hook: fail with `simulated crash` after this many snapshots.
    pub stop_after_snapshots: Option<u64>,
    snapshots: Cell<u64>,
    last: Cell<Instant>,
    /// Test hook: snapshot every this many units instead of by time.
    pub every_units: Option<u64>,
}

impl Ckpt {
    /// The checkpoint of `key` in `dir`, if `dir` exists.
    pub fn in_dir(dir: &Path, key: &str, interval: Duration) -> Option<Ckpt> {
        dir.is_dir().then(|| Ckpt {
            dir: dir.to_path_buf(),
            key: key.chars().filter(|c| c.is_ascii_alphanumeric()).collect(),
            interval,
            stop_after_snapshots: None,
            snapshots: Cell::new(0),
            last: Cell::new(Instant::now()),
            every_units: None,
        })
    }

    pub fn path(&self, ext: &str) -> PathBuf {
        self.dir.join(format!("{}.{ext}", self.key))
    }

    /// Whether a snapshot is due after `units` units of work (calls, walks).
    pub fn due(&self, units: u64) -> bool {
        match self.every_units {
            Some(n) => n > 0 && units > 0 && units % n == 0,
            None => self.last.get().elapsed() >= self.interval,
        }
    }

    /// Write `bytes` to `<key>.<ext>` atomically.
    pub fn write_atomic(&self, ext: &str, bytes: &[u8]) -> Result<(), String> {
        let tmp = self.path(&format!("{ext}.tmp"));
        let mut f = fs::File::create(&tmp).map_err(|e| format!("{}: {e}", tmp.display()))?;
        f.write_all(bytes).map_err(|e| e.to_string())?;
        f.sync_data().map_err(|e| e.to_string())?;
        fs::rename(&tmp, self.path(ext)).map_err(|e| e.to_string())
    }

    pub fn read(&self, ext: &str) -> Option<Vec<u8>> {
        fs::read(self.path(ext)).ok()
    }

    /// Note a finished snapshot; `Err` when the test hook says to crash.
    pub fn snapshotted(&self) -> Result<(), String> {
        self.last.set(Instant::now());
        let n = self.snapshots.get() + 1;
        self.snapshots.set(n);
        match self.stop_after_snapshots {
            Some(k) if n >= k => Err("simulated crash".into()),
            _ => Ok(()),
        }
    }

    /// Delete every file of this checkpoint (the run completed).
    pub fn remove_all(&self) {
        for ext in ["snap", "log", "snap.tmp"] {
            let _ = fs::remove_file(self.path(ext));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_codec_round_trips() {
        let mut w = Wr::default();
        w.u8(7);
        w.u64(u64::MAX - 3);
        w.u128(u128::MAX - 5);
        for v in [0u64, 1, 127, 128, 300, u64::MAX] {
            w.varint(v);
        }
        w.str("héllo");
        let mut r = Rd::new(&w.0);
        assert_eq!(r.u8().unwrap(), 7);
        assert_eq!(r.u64().unwrap(), u64::MAX - 3);
        assert_eq!(r.u128().unwrap(), u128::MAX - 5);
        for v in [0u64, 1, 127, 128, 300, u64::MAX] {
            assert_eq!(r.varint().unwrap(), v);
        }
        assert_eq!(r.str().unwrap(), "héllo");
        assert!(r.at_end());
        assert!(Rd::new(&w.0[..3]).u64().is_err());
    }
}
