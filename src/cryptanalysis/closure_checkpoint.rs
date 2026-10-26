//! Application checkpoints for the MutantXL closure scan
//! ([`closure_refutation_scan`](super::koblitz_bench::closure_refutation_scan)).
//!
//! A closure at degree 4 or 5 runs for hours to days on a matrix of tens to
//! hundreds of gigabytes.  On Spot capacity it is cut short again and again,
//! so the scan saves the mathematical state and a later process continues
//! from it.  Between rounds that is the echelon rows in the current column
//! layout, the columns, the queue of pending degree falls and the cost
//! counters; inside a round's elimination (the long step) it is the partly
//! reduced matrix and the elimination's position at a packed-word boundary.
//! It is not a process image: any x86-64 host can resume it, and every file
//! is hashed.
//!
//! The on-disk contract is the managed worker's `manifest-sha256-v1`
//! ([`NATIVE_CHECKPOINTS.md`](../../../infra/spot-system/NATIVE_CHECKPOINTS.md)):
//! immutable `gen-<n>` directories, files streamed through SHA-256 and
//! fsynced, and `checkpoint-manifest.json` written last by an atomic rename
//! followed by a directory fsync.  A generation without the manifest is
//! partial and is never restored.  Two committed generations are kept; the
//! older of two is deleted before the next is written, so the peak on disk is
//! two states, and a failed periodic save is logged and retried.
//! Copying a committed generation to object storage is the caller's job
//! (`scripts/ckpt-s3-sync.sh`), because a generation is immutable.
//!
//! What a checkpoint does not hold: the plain (XL) basis's elimination and the
//! Macaulay matrix build and first elimination of the closure (before round
//! 0).  A loss inside either repeats that step.  Once round 0 is saved, a loss
//! costs at most the save interval plus the work from the last save to its
//! copy in object storage.

use super::inherited_f4::{CloseFlow, ClosureSink, InheritCost, ReducedBasis, RoundJob, RoundView};
use super::koblitz_bench::ClosureDegree;
use crate::cryptanalysis::pq_groebner_f2::F2BoolPoly;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

const MAGIC: &[u8; 8] = b"DRGCKPT1";
const SCHEMA: &str = "spot.checkpoint/v1";
const KEEP: usize = 2;

static SIGNALLED: AtomicBool = AtomicBool::new(false);
extern "C" fn on_signal(_: libc::c_int) {
    SIGNALLED.store(true, Ordering::Relaxed);
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct CostRec {
    reduce_word_ops: u64,
    rref_word_ops: u64,
    specialise_word_ops: u64,
    displaced_rows: u64,
    completion_rows: u64,
    closure_products: u64,
}
impl From<&InheritCost> for CostRec {
    fn from(c: &InheritCost) -> Self {
        Self {
            reduce_word_ops: c.reduce_word_ops,
            rref_word_ops: c.rref_word_ops,
            specialise_word_ops: c.specialise_word_ops,
            displaced_rows: c.displaced_rows,
            completion_rows: c.completion_rows,
            closure_products: c.closure_products,
        }
    }
}
impl From<&CostRec> for InheritCost {
    fn from(c: &CostRec) -> Self {
        Self {
            reduce_word_ops: c.reduce_word_ops,
            rref_word_ops: c.rref_word_ops,
            specialise_word_ops: c.specialise_word_ops,
            displaced_rows: c.displaced_rows,
            completion_rows: c.completion_rows,
            closure_products: c.closure_products,
        }
    }
}

/// One finished degree of the scan.
#[derive(Clone, Serialize, Deserialize)]
struct DegreeRec {
    degree: u32,
    xl_refuted: bool,
    closure_refuted: bool,
    saturated: bool,
    xl_rank: usize,
    closure_rank: usize,
    columns: usize,
    closure_products: u64,
    closure_word_ops: u64,
    secs: f64,
}

/// The degree in progress: the plain basis is done, the closure is between
/// rounds.  `rounds_done == 0` is the seeded basis before round 0.
#[derive(Clone, Serialize, Deserialize)]
struct Active {
    degree: u32,
    xl_refuted: bool,
    xl_rank: usize,
    /// `None` until the closure has been seeded (only the XL answer is held).
    rounds_done: Option<u32>,
    cost: CostRec,
    /// Compute seconds spent on this degree in earlier segments.
    secs: f64,
}

#[derive(Default, Serialize, Deserialize)]
struct Catalog {
    version: u32,
    /// Identity of the system and the scan: a store is for one system.
    system_sha256: String,
    /// Process starts that worked on this store, counting the first.
    segments: u32,
    completed: BTreeMap<u32, DegreeRec>,
    active: Option<Active>,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    path: String,
    sha256: String,
    bytes: u64,
}
#[derive(Serialize, Deserialize)]
struct Manifest {
    schema: String,
    job_id: String,
    spec_sha256: String,
    generation: u64,
    committed: bool,
    files: Vec<Entry>,
}

struct HashWriter {
    file: File,
    hash: Sha256,
    bytes: u64,
}
impl Write for HashWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let n = self.file.write(bytes)?;
        self.hash.update(&bytes[..n]);
        self.bytes += n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

fn write_file(
    dir: &Path,
    name: &str,
    write: impl FnOnce(&mut BufWriter<HashWriter>) -> io::Result<()>,
) -> io::Result<Entry> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let hash_writer = HashWriter {
        file: options.open(dir.join(name))?,
        hash: Sha256::new(),
        bytes: 0,
    };
    let mut writer = BufWriter::with_capacity(8 << 20, hash_writer);
    write(&mut writer)?;
    writer.flush()?;
    let writer = writer.into_inner().map_err(|e| e.into_error())?;
    writer.file.sync_all()?;
    Ok(Entry {
        path: name.into(),
        sha256: format!("{:x}", writer.hash.finalize()),
        bytes: writer.bytes,
    })
}

fn put_u64(w: &mut impl Write, n: u64) -> io::Result<()> {
    w.write_all(&n.to_le_bytes())
}
fn get_u64(r: &mut impl Read) -> io::Result<u64> {
    let mut b = [0; 8];
    r.read_exact(&mut b)?;
    Ok(u64::from_le_bytes(b))
}
fn put_words(w: &mut impl Write, words: &[u64]) -> io::Result<()> {
    #[cfg(target_endian = "little")]
    {
        // SAFETY: u64 has no padding and any byte pattern is a valid u8.
        let bytes =
            unsafe { std::slice::from_raw_parts(words.as_ptr().cast::<u8>(), words.len() * 8) };
        w.write_all(bytes)
    }
    #[cfg(not(target_endian = "little"))]
    {
        for x in words {
            w.write_all(&x.to_le_bytes())?;
        }
        Ok(())
    }
}
fn get_words(r: &mut impl Read, n: usize) -> io::Result<Vec<u64>> {
    let mut out = vec![0u64; n];
    #[cfg(target_endian = "little")]
    {
        // SAFETY: as above, the buffer is fully initialised and owned.
        let bytes = unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr().cast::<u8>(), n * 8) };
        r.read_exact(bytes)?;
    }
    #[cfg(not(target_endian = "little"))]
    for x in out.iter_mut() {
        *x = get_u64(r)?;
    }
    Ok(out)
}

/// A single-writer checkpoint store in one directory.
pub struct Store {
    root: PathBuf,
    interval: Duration,
    generation: u64,
    catalog: Catalog,
    /// The basis file of the generation that was loaded, not yet consumed.
    resumed_basis: Option<PathBuf>,
    last_save: Instant,
    /// Test hook, also `DREG_CKPT_STOP_AFTER_ROUNDS`: behave as if SIGTERM
    /// arrived after this many rounds of the current process.
    pub stop_after_rounds: Option<u32>,
    /// Test hook, also `DREG_CKPT_STOP_AFTER_WORDS`: stop inside an
    /// elimination at its next word boundary once this many words have been
    /// processed in this process (so every process makes progress).
    pub stop_after_words: Option<u64>,
    rounds_this_process: u32,
    words_this_process: u64,
    /// Committed generations written by this process.
    pub saves: u32,
}

/// How a checkpointed scan ended.
pub enum ScanEnd {
    Done(Vec<ClosureDegree>),
    /// A stop was requested; the state is saved and a new process continues.
    Interrupted,
}

impl Store {
    /// A store from `DREG_CKPT_DIR` (and `DREG_CKPT_SECONDS`, default 900);
    /// `None` when no directory is set, which keeps the original path.
    pub fn from_env() -> io::Result<Option<Self>> {
        let Some(dir) = std::env::var_os("DREG_CKPT_DIR") else {
            return Ok(None);
        };
        let secs = std::env::var("DREG_CKPT_SECONDS")
            .ok()
            .map(|s| s.parse::<u64>().map_err(|_| invalid("DREG_CKPT_SECONDS")))
            .transpose()?
            .unwrap_or(900);
        let mut store = Self::open(Path::new(&dir), Duration::from_secs(secs.max(1)))?;
        store.stop_after_rounds = std::env::var("DREG_CKPT_STOP_AFTER_ROUNDS")
            .ok()
            .and_then(|s| s.parse().ok());
        store.stop_after_words = std::env::var("DREG_CKPT_STOP_AFTER_WORDS")
            .ok()
            .and_then(|s| s.parse().ok());
        Ok(Some(store))
    }

    /// SIGTERM and SIGINT set a flag that the scan reads between rounds.
    pub fn install_signal_handlers() {
        // SAFETY: the handler only stores to an atomic.
        unsafe {
            for sig in [libc::SIGTERM, libc::SIGINT] {
                libc::signal(sig, on_signal as *const () as libc::sighandler_t);
            }
        }
    }

    /// Open `root`, creating it, and restore its newest valid generation.
    /// A directory holding generations none of which verifies is an error:
    /// a long run is never silently restarted.
    pub fn open(root: &Path, interval: Duration) -> io::Result<Self> {
        fs::create_dir_all(root)?;
        let root = fs::canonicalize(root)?;
        let mut store = Self {
            root,
            interval,
            generation: 0,
            catalog: Catalog {
                version: 1,
                segments: 1,
                ..Default::default()
            },
            resumed_basis: None,
            last_save: Instant::now(),
            stop_after_rounds: None,
            stop_after_words: None,
            rounds_this_process: 0,
            words_this_process: 0,
            saves: 0,
        };
        let mut gens = Vec::new();
        for entry in fs::read_dir(&store.root)? {
            let entry = entry?;
            if let Some(n) = entry
                .file_name()
                .to_str()
                .and_then(|s| s.strip_prefix("gen-"))
                .and_then(|s| s.parse::<u64>().ok())
            {
                store.generation = store.generation.max(n);
                if entry.path().join("checkpoint-manifest.json").is_file() {
                    gens.push((n, entry.path()));
                } else {
                    // A save that was cut off (SIGKILL after the grace period,
                    // a lost node) can be hundreds of gigabytes of a state that
                    // can never be restored; the local disk is finite.
                    eprintln!("checkpoint: removing uncommitted generation {n}");
                    fs::remove_dir_all(entry.path())?;
                }
            }
        }
        gens.sort();
        let mut last_err = None;
        for (n, dir) in gens.into_iter().rev() {
            match Self::verify(&dir) {
                Ok(()) => {
                    let mut catalog: Catalog = serde_json::from_reader(BufReader::new(
                        File::open(dir.join("catalog.json"))?,
                    ))?;
                    if catalog.version != 1 {
                        return Err(invalid("unsupported checkpoint catalog"));
                    }
                    catalog.segments += 1;
                    eprintln!(
                        "checkpoint: restoring generation {n}: {} degrees done, active {:?}, segment {}",
                        catalog.completed.len(),
                        catalog.active.as_ref().map(|a| (a.degree, a.rounds_done)),
                        catalog.segments
                    );
                    store.catalog = catalog;
                    if store
                        .catalog
                        .active
                        .as_ref()
                        .is_some_and(|a| a.rounds_done.is_some())
                    {
                        store.resumed_basis = Some(dir.join("basis.bin"));
                    }
                    return Ok(store);
                }
                Err(e) => {
                    eprintln!(
                        "checkpoint: generation {n} does not verify ({e}); trying the previous one"
                    );
                    last_err = Some(e);
                }
            }
        }
        if let Some(e) = last_err {
            return Err(invalid(format!(
                "{} holds checkpoint generations but none verifies (last: {e}); refusing to restart",
                store.root.display()
            )));
        }
        Ok(store)
    }

    fn verify(dir: &Path) -> io::Result<()> {
        let manifest: Manifest =
            serde_json::from_reader(File::open(dir.join("checkpoint-manifest.json"))?)?;
        if manifest.schema != SCHEMA || !manifest.committed {
            return Err(invalid("manifest is not a committed spot.checkpoint/v1"));
        }
        for entry in &manifest.files {
            if entry.path.contains('/') || entry.path.contains("..") {
                return Err(invalid("manifest path escapes the generation"));
            }
            let path = dir.join(&entry.path);
            if fs::symlink_metadata(&path)?.file_type().is_symlink() {
                return Err(invalid("checkpoint symlink"));
            }
            let mut f = File::open(path)?;
            let mut hash = Sha256::new();
            let mut buf = vec![0u8; 8 << 20];
            let mut total = 0u64;
            loop {
                let n = f.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                hash.update(&buf[..n]);
                total += n as u64;
            }
            if total != entry.bytes || format!("{:x}", hash.finalize()) != entry.sha256 {
                return Err(invalid(format!("{} checksum disagrees", entry.path)));
            }
        }
        Ok(())
    }

    /// Process starts that have worked on this store (1 on a fresh one).
    pub fn segments(&self) -> u32 {
        self.catalog.segments
    }

    fn stop_requested(&self) -> bool {
        SIGNALLED.load(Ordering::Relaxed)
            || self
                .stop_after_rounds
                .is_some_and(|n| self.rounds_this_process >= n)
            || self
                .stop_after_words
                .is_some_and(|n| self.words_this_process > n)
    }

    /// After a failed save: delete the generation directory it left, which has
    /// no manifest and can never be restored, and wait a full interval before
    /// trying again.
    fn discard_partial(&mut self) {
        let dir = self.root.join(format!("gen-{:020}", self.generation));
        if !dir.join("checkpoint-manifest.json").is_file() {
            let _ = fs::remove_dir_all(dir);
        }
        self.last_save = Instant::now();
    }

    /// Remove committed generations beyond the newest `keep`, except the one a
    /// pending resume still has to read.
    fn prune(&mut self, keep: usize) -> io::Result<()> {
        let mut committed: Vec<PathBuf> = fs::read_dir(&self.root)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| {
                p.file_name()
                    .and_then(|s| s.to_str())
                    .is_some_and(|s| s.starts_with("gen-"))
                    && p.join("checkpoint-manifest.json").is_file()
            })
            .collect();
        committed.sort();
        let excess = committed.len().saturating_sub(keep);
        let held = self.resumed_basis.as_deref().and_then(Path::parent);
        for p in committed.into_iter().take(excess) {
            if Some(p.as_path()) != held {
                fs::remove_dir_all(p)?;
            }
        }
        File::open(&self.root)?.sync_all()
    }

    fn save(&mut self, payload: Payload<'_>) -> io::Result<()> {
        let started = Instant::now();
        // Make room first: keep only the newest committed generation while the
        // next is written, so the peak is two states on disk, not three, and a
        // valid generation survives a loss at any point of the write.
        self.prune(KEEP - 1)?;
        self.generation += 1;
        let dir = self.root.join(format!("gen-{:020}", self.generation));
        fs::create_dir(&dir)?;
        let mut files = vec![write_file(&dir, "catalog.json", |w| {
            serde_json::to_writer(w, &self.catalog)?;
            Ok(())
        })?];
        match payload {
            Payload::Catalog => {}
            Payload::Between(basis) => {
                let view = basis
                    .closure_view()
                    .ok_or_else(|| invalid("closure is not between rounds"))?;
                files.push(write_file(&dir, "basis.bin", |w| {
                    w.write_all(MAGIC)?;
                    for n in [
                        KIND_BETWEEN,
                        view.degree as u64,
                        view.n_vars as u64,
                        view.support_local as u64,
                        view.min_fall_degree as u64,
                        view.closure_rounds as u64,
                        view.words as u64,
                        view.columns.len() as u64,
                        view.n_rows() as u64,
                        view.pending.len() as u64,
                    ] {
                        put_u64(w, n)?;
                    }
                    put_words(w, view.columns)?;
                    for &p in view.pending {
                        w.write_all(&p.to_le_bytes())?;
                    }
                    for r in 0..view.n_rows() {
                        put_words(w, view.row(r))?;
                    }
                    Ok(())
                })?);
            }
            Payload::Elimination(view) => {
                let words = view.columns.len().div_ceil(64).max(1);
                files.push(write_file(&dir, "basis.bin", |w| {
                    w.write_all(MAGIC)?;
                    for n in [
                        KIND_ELIMINATION,
                        view.degree as u64,
                        view.n_vars as u64,
                        view.support_local as u64,
                        view.min_fall_degree as u64,
                        view.closure_rounds as u64,
                        words as u64,
                        view.columns.len() as u64,
                        view.matrix.len() as u64,
                        0,
                        view.pivot_row as u64,
                        view.word as u64,
                    ] {
                        put_u64(w, n)?;
                    }
                    put_words(w, view.columns)?;
                    let bits: Vec<u8> = view.was_pivot.iter().map(|&b| b as u8).collect();
                    w.write_all(&bits)?;
                    for row in view.matrix {
                        put_words(w, row)?;
                    }
                    Ok(())
                })?);
            }
        }
        let manifest = Manifest {
            schema: SCHEMA.into(),
            job_id: "dreg-closure".into(),
            spec_sha256: self.catalog.system_sha256.clone(),
            generation: self.generation,
            committed: true,
            files,
        };
        write_file(&dir, "manifest.tmp", |w| {
            serde_json::to_writer(w, &manifest)?;
            Ok(())
        })?;
        File::open(&dir)?.sync_all()?;
        fs::rename(
            dir.join("manifest.tmp"),
            dir.join("checkpoint-manifest.json"),
        )?;
        File::open(&dir)?.sync_all()?;
        File::open(&self.root)?.sync_all()?;
        self.last_save = Instant::now();
        self.saves += 1;
        eprintln!(
            "checkpoint: generation {} committed, {:.2} GB in {:.1}s",
            self.generation,
            manifest.files.iter().map(|e| e.bytes).sum::<u64>() as f64 / 1e9,
            started.elapsed().as_secs_f64()
        );
        File::open(&self.root)?.sync_all()?;
        Ok(())
    }

    fn read_state(
        path: &Path,
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
    ) -> io::Result<Restored> {
        let mut r = BufReader::with_capacity(8 << 20, File::open(path)?);
        let mut magic = [0u8; 8];
        r.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(invalid("not a closure checkpoint"));
        }
        let kind = get_u64(&mut r)?;
        let mut h = [0u64; 9];
        for x in &mut h {
            *x = get_u64(&mut r)?;
        }
        let [d, nv, local, minfall, rounds, words, ncols, nrows, npend] = h;
        let (pivot_row, word) = if kind == KIND_ELIMINATION {
            (get_u64(&mut r)? as usize, get_u64(&mut r)? as usize)
        } else {
            (0, 0)
        };
        if d as u32 != degree || nv as usize != n_vars {
            return Err(invalid("checkpoint is for another degree or system"));
        }
        let (words, ncols, nrows, npend) = (
            words as usize,
            ncols as usize,
            nrows as usize,
            npend as usize,
        );
        if words != ncols.div_ceil(64).max(1) {
            return Err(invalid("checkpoint layout is inconsistent"));
        }
        let columns = get_words(&mut r, ncols)?;
        match kind {
            KIND_BETWEEN => {
                let mut pending = Vec::with_capacity(npend);
                for _ in 0..npend {
                    let mut b = [0u8; 4];
                    r.read_exact(&mut b)?;
                    pending.push(u32::from_le_bytes(b));
                }
                let mut rows = Vec::with_capacity(nrows);
                for _ in 0..nrows {
                    rows.push(get_words(&mut r, words)?);
                }
                ReducedBasis::from_closure_state(
                    system,
                    n_vars,
                    degree,
                    local != 0,
                    minfall as u32,
                    rounds as u32,
                    columns,
                    rows,
                    pending,
                )
                .map(Restored::Between)
                .ok_or_else(|| invalid("checkpoint rows do not rebuild a basis"))
            }
            KIND_ELIMINATION => {
                let mut bits = vec![0u8; ncols];
                r.read_exact(&mut bits)?;
                let was_pivot = bits.into_iter().map(|b| b != 0).collect();
                let mut matrix = Vec::with_capacity(nrows);
                for _ in 0..nrows {
                    matrix.push(get_words(&mut r, words)?);
                }
                ReducedBasis::from_round_state(
                    system,
                    n_vars,
                    degree,
                    local != 0,
                    minfall as u32,
                    rounds as u32,
                    columns,
                    was_pivot,
                    matrix,
                    pivot_row,
                    word,
                )
                .map(|(b, j)| Restored::Elimination(b, j))
                .ok_or_else(|| invalid("checkpoint matrix does not rebuild a round"))
            }
            _ => Err(invalid("unknown checkpoint kind")),
        }
    }
}

const KIND_BETWEEN: u64 = 0;
const KIND_ELIMINATION: u64 = 1;

enum Payload<'a> {
    Catalog,
    Between(&'a ReducedBasis),
    Elimination(&'a RoundView<'a>),
}

enum Restored {
    Between(ReducedBasis),
    Elimination(ReducedBasis, RoundJob),
}

/// SHA-256 of the system and its variable count: what a store is bound to.
pub fn system_digest(system: &[F2BoolPoly], n_vars: usize) -> String {
    let mut h = Sha256::new();
    h.update(b"closure-refutation-scan/1");
    h.update((n_vars as u64).to_le_bytes());
    h.update((system.len() as u64).to_le_bytes());
    for p in system {
        h.update((p.terms.len() as u64).to_le_bytes());
        for t in &p.terms {
            h.update(t.mask.to_le_bytes());
        }
    }
    format!("{:x}", h.finalize())
}

fn to_degree(rec: &DegreeRec) -> ClosureDegree {
    ClosureDegree {
        degree: rec.degree,
        xl_refuted: rec.xl_refuted,
        closure_refuted: rec.closure_refuted,
        saturated: rec.saturated,
        xl_rank: rec.xl_rank,
        closure_rank: rec.closure_rank,
        columns: rec.columns,
        closure_products: rec.closure_products,
        closure_word_ops: rec.closure_word_ops,
        secs: rec.secs,
    }
}

/// The sink the scan hands to the closure: it writes a generation between
/// rounds and inside a round's elimination when the interval has passed or a
/// stop is requested, and stops the closure when one is.
struct Saver<'a> {
    store: &'a mut Store,
    degree: u32,
    xl_refuted: bool,
    xl_rank: usize,
    secs_before: f64,
    started: Instant,
    error: Option<io::Error>,
}

impl Saver<'_> {
    fn active(&self, rounds: u32, cost: &InheritCost) -> Active {
        Active {
            degree: self.degree,
            xl_refuted: self.xl_refuted,
            xl_rank: self.xl_rank,
            rounds_done: Some(rounds),
            cost: CostRec::from(cost),
            secs: self.secs_before + self.started.elapsed().as_secs_f64(),
        }
    }
    /// A failed periodic save is not fatal: the run still holds its state in
    /// memory, so the partial generation is dropped and the save is retried
    /// after another interval.  A save that fails when a stop was requested
    /// is the last chance to keep the state, so it is an error.
    fn after_save(&mut self, saved: io::Result<()>, stop: bool) {
        if let Err(e) = saved {
            if stop {
                self.error = Some(e);
            } else {
                eprintln!("checkpoint: periodic save failed ({e}); continuing, will retry");
                self.store.discard_partial();
            }
        }
    }

    fn flow(&self, stop: bool) -> CloseFlow {
        if stop || self.error.is_some() {
            CloseFlow::Stop
        } else {
            CloseFlow::Continue
        }
    }
}

impl ClosureSink for Saver<'_> {
    fn after_round(&mut self, basis: &ReducedBasis, cost: &InheritCost, done: u32) -> CloseFlow {
        self.store.rounds_this_process += 1;
        eprintln!(
            "checkpoint: degree {} round {done}: {} rows, {} columns, {} falls pending, {:.1}s",
            self.degree,
            basis.rank(),
            basis.columns(),
            basis.pending_falls(),
            self.secs_before + self.started.elapsed().as_secs_f64()
        );
        let stop = self.store.stop_requested();
        if stop || self.store.last_save.elapsed() >= self.store.interval {
            self.store.catalog.active = Some(self.active(done, cost));
            let saved = self.store.save(Payload::Between(basis));
            self.after_save(saved, stop);
        }
        self.flow(stop)
    }

    fn in_round(&mut self, view: &RoundView<'_>, cost: &InheritCost, done: u32) -> CloseFlow {
        self.store.words_this_process += 1;
        let stop = self.store.stop_requested();
        if stop || self.store.last_save.elapsed() >= self.store.interval {
            self.store.catalog.active = Some(self.active(done, cost));
            eprintln!(
                "checkpoint: degree {} round {}: elimination at word {} of {}, {} of {} rows pivoted",
                self.degree,
                done + 1,
                view.word,
                view.columns.len().div_ceil(64),
                view.pivot_row,
                view.matrix.len()
            );
            let saved = self.store.save(Payload::Elimination(view));
            self.after_save(saved, stop);
        }
        self.flow(stop)
    }
}

/// [`closure_refutation_scan`](super::koblitz_bench::closure_refutation_scan)
/// that saves its state into `store` between closure rounds and at word
/// boundaries inside a round's elimination, and continues from it.  It
/// returns the same `ClosureDegree` rows (the mathematics and the cost
/// counters are identical; `secs` is the compute time summed over segments).
/// The store is bound to `system`: another system is an error.
pub fn closure_refutation_scan_checkpointed(
    system: &[F2BoolPoly],
    n_vars: usize,
    d_min: u32,
    d_max: u32,
    store: &mut Store,
) -> io::Result<ScanEnd> {
    const ROUNDS: u32 = 1_000_000;
    let digest = system_digest(system, n_vars);
    if store.catalog.system_sha256.is_empty() {
        store.catalog.system_sha256 = digest;
    } else if store.catalog.system_sha256 != digest {
        return Err(invalid("checkpoint store belongs to a different system"));
    }
    let mut out = Vec::new();
    let (mut xl_done, mut closure_done) = (false, false);
    for degree in d_min..=d_max {
        if let Some(rec) = store.catalog.completed.get(&degree).cloned() {
            xl_done |= rec.xl_refuted;
            closure_done |= rec.closure_refuted;
            out.push(to_degree(&rec));
            if xl_done && closure_done {
                break;
            }
            continue;
        }
        let started = Instant::now();
        let carried = store
            .catalog
            .active
            .as_ref()
            .filter(|a| a.degree == degree)
            .cloned();
        let secs_before = carried.as_ref().map_or(0.0, |a| a.secs);
        let (xl_refuted, xl_rank) = match &carried {
            Some(a) => (a.xl_refuted, a.xl_rank),
            None => {
                let Some(x) = ReducedBasis::from_system(system, n_vars, degree)
                    .map(|(xl, _)| (xl.is_refuted(), xl.rank()))
                else {
                    break;
                };
                // The plain elimination is long at high degree: keep its answer.
                store.catalog.active = Some(Active {
                    degree,
                    xl_refuted: x.0,
                    xl_rank: x.1,
                    rounds_done: None,
                    cost: CostRec::default(),
                    secs: started.elapsed().as_secs_f64(),
                });
                store.save(Payload::Catalog)?;
                x
            }
        };
        // The state to continue from: the loaded one, or the seeded closure.
        let resumed = match (&carried, store.resumed_basis.take()) {
            (Some(a), Some(path)) if a.rounds_done.is_some() => {
                let restored = Store::read_state(&path, system, n_vars, degree)?;
                eprintln!(
                    "checkpoint: degree {degree} resumed after {} rounds",
                    a.rounds_done.unwrap_or(0)
                );
                Some((
                    restored,
                    InheritCost::from(&a.cost),
                    a.rounds_done.unwrap_or(0),
                ))
            }
            _ => None,
        };
        let (mut basis, mut cost, rounds, job) = match resumed {
            Some((Restored::Between(b), c, r)) => (b, c, r, None),
            Some((Restored::Elimination(b, j), c, r)) => (b, c, r, Some(j)),
            None => {
                let Some((basis, cost)) =
                    ReducedBasis::seed_closure_full(system, n_vars, degree, ROUNDS)
                else {
                    break;
                };
                eprintln!(
                    "checkpoint: degree {degree} seeded: {} rows, {} columns, {} falls pending, {:.1}s",
                    basis.rank(),
                    basis.columns(),
                    basis.pending_falls(),
                    secs_before + started.elapsed().as_secs_f64()
                );
                // Round 0: the seeded basis is the first state worth keeping.
                if basis.closure_view().is_some() {
                    store.catalog.active = Some(Active {
                        degree,
                        xl_refuted,
                        xl_rank,
                        rounds_done: Some(0),
                        cost: CostRec::from(&cost),
                        secs: secs_before + started.elapsed().as_secs_f64(),
                    });
                    store.save(Payload::Between(&basis))?;
                }
                (basis, cost, 0, None)
            }
        };
        if job.is_none() && store.stop_requested() && basis.pending_falls() > 0 {
            eprintln!("checkpoint: stop requested; degree {degree} saved after {rounds} rounds");
            return Ok(ScanEnd::Interrupted);
        }
        let mut saver = Saver {
            store,
            degree,
            xl_refuted,
            xl_rank,
            secs_before,
            started,
            error: None,
        };
        let end = basis.close_with(&mut cost, rounds, job, &mut saver);
        if let Some(e) = saver.error {
            return Err(e);
        }
        if end.stopped {
            eprintln!(
                "checkpoint: stop requested; degree {degree} saved after {} rounds",
                end.rounds
            );
            return Ok(ScanEnd::Interrupted);
        }
        let rec = DegreeRec {
            degree,
            xl_refuted,
            closure_refuted: basis.is_refuted(),
            saturated: basis.is_refuted() || basis.pending_falls() == 0,
            xl_rank,
            closure_rank: basis.rank(),
            columns: basis.columns(),
            closure_products: cost.closure_products,
            closure_word_ops: cost.word_ops(),
            secs: secs_before + started.elapsed().as_secs_f64(),
        };
        out.push(to_degree(&rec));
        store.catalog.completed.insert(degree, rec);
        store.catalog.active = None;
        store.save(Payload::Catalog)?;
        xl_done |= xl_refuted;
        closure_done |= basis.is_refuted();
        if xl_done && closure_done {
            break;
        }
    }
    Ok(ScanEnd::Done(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::F2mElement;
    use crate::cryptanalysis::koblitz_bench::{
        chained_solution_count, closure_refutation_scan, ladder_sample,
    };
    use crate::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
    use crate::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("dreg-ckpt-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    /// Chained draws, unsatisfiable and satisfiable, whose closure takes
    /// several rounds at degrees 3 and 4.
    fn systems() -> Vec<(String, Vec<F2BoolPoly>, usize)> {
        let mut out = Vec::new();
        for (m, n, ell, seed) in [(3usize, 7u32, 2usize, 12u64), (4, 4, 1, 13), (3, 5, 2, 11)] {
            let irr = find_irreducible_sparse(n).unwrap();
            let st = FieldStructure::new(n, &irr);
            let b = F2mElement::one(n);
            let mut rng = StdRng::seed_from_u64(seed);
            let (mut sat, mut unsat) = (0, 0);
            while sat < 1 || unsat < 2 {
                let (basis, x_r) = ladder_sample(n, ell, &mut rng);
                let satisfiable = chained_solution_count(m, &basis, &x_r, &b, &irr) > 0;
                if (satisfiable && sat == 1) || (!satisfiable && unsat == 2) {
                    continue;
                }
                let sys = build_decomposition_system(&basis, &x_r, &b, m, &st).unwrap();
                out.push((
                    format!("m{m}n{n}l{ell}s{seed}#{}", out.len()),
                    sys.equations,
                    sys.n_vars,
                ));
                if satisfiable {
                    sat += 1
                } else {
                    unsat += 1
                }
            }
        }
        out
    }

    fn same(a: &[ClosureDegree], b: &[ClosureDegree], what: &str) {
        assert_eq!(a.len(), b.len(), "{what}: degrees");
        for (x, y) in a.iter().zip(b) {
            // Everything but the time.
            assert_eq!(
                (
                    x.degree,
                    x.xl_refuted,
                    x.closure_refuted,
                    x.saturated,
                    x.xl_rank,
                    x.closure_rank,
                    x.columns,
                    x.closure_products,
                    x.closure_word_ops
                ),
                (
                    y.degree,
                    y.xl_refuted,
                    y.closure_refuted,
                    y.saturated,
                    y.xl_rank,
                    y.closure_rank,
                    y.columns,
                    y.closure_products,
                    y.closure_word_ops
                ),
                "{what}: degree {}",
                x.degree
            );
        }
    }

    /// Run the scan to completion, stopping after `every` rounds each
    /// process and reopening the store from disk each time, as a Spot loss
    /// and a new runner would.  Returns the result and how many stops.
    fn run_with_stops(
        dir: &Path,
        sys: &[F2BoolPoly],
        n_vars: usize,
        every: u32,
    ) -> (Vec<ClosureDegree>, u32) {
        run_with(dir, sys, n_vars, Some(every), None)
    }

    /// As [`run_with_stops`], also able to stop every `words` word
    /// boundaries inside an elimination.
    fn run_with(
        dir: &Path,
        sys: &[F2BoolPoly],
        n_vars: usize,
        rounds: Option<u32>,
        words: Option<u64>,
    ) -> (Vec<ClosureDegree>, u32) {
        let mut stops = 0;
        for _ in 0..1_000_000 {
            let mut store = Store::open(dir, Duration::from_secs(3600)).unwrap();
            store.stop_after_rounds = rounds;
            store.stop_after_words = words;
            match closure_refutation_scan_checkpointed(sys, n_vars, 3, 4, &mut store).unwrap() {
                ScanEnd::Done(v) => return (v, stops),
                ScanEnd::Interrupted => stops += 1,
            }
        }
        panic!("the scan never finished");
    }

    #[test]
    fn a_checkpointed_scan_equals_the_plain_scan() {
        for (name, sys, n_vars) in systems() {
            let plain = closure_refutation_scan(&sys, n_vars, 3, 4);
            let dir = scratch("plain");
            let mut store = Store::open(&dir, Duration::from_secs(3600)).unwrap();
            let ScanEnd::Done(got) =
                closure_refutation_scan_checkpointed(&sys, n_vars, 3, 4, &mut store).unwrap()
            else {
                panic!("interrupted without a stop")
            };
            same(&plain, &got, &name);
            let _ = fs::remove_dir_all(&dir);
        }
    }

    #[test]
    fn a_scan_stopped_and_resumed_at_every_round_equals_the_plain_scan() {
        let mut stops_total = 0;
        for (name, sys, n_vars) in systems() {
            let plain = closure_refutation_scan(&sys, n_vars, 3, 4);
            for every in [1, 2] {
                let dir = scratch("resume");
                let (got, stops) = run_with_stops(&dir, &sys, n_vars, every);
                same(&plain, &got, &format!("{name}, stop every {every}"));
                stops_total += stops;
                let _ = fs::remove_dir_all(&dir);
            }
        }
        // The resume path ran: closures took several rounds, so stops happened.
        assert!(
            stops_total >= 4,
            "only {stops_total} stops: the test is vacuous"
        );
    }

    /// The same, with the stops landing inside the rounds' eliminations:
    /// the matrix of basis rows and products is saved partly reduced and the
    /// elimination continues from its word.
    #[test]
    fn a_scan_stopped_inside_its_eliminations_equals_the_plain_scan() {
        let mut stops_total = 0;
        for (name, sys, n_vars) in systems() {
            let plain = closure_refutation_scan(&sys, n_vars, 3, 4);
            for every in [1u64, 2, 5] {
                let dir = scratch("elim");
                let (got, stops) = run_with(&dir, &sys, n_vars, None, Some(every));
                same(&plain, &got, &format!("{name}, stop every {every} words"));
                stops_total += stops;
                let _ = fs::remove_dir_all(&dir);
            }
        }
        assert!(
            stops_total >= 40,
            "only {stops_total} stops inside eliminations: the test is vacuous"
        );
    }

    #[test]
    fn a_corrupt_newest_generation_falls_back_to_the_previous_one() {
        let (name, sys, n_vars) = systems()
            .into_iter()
            .find(|(_, s, n)| {
                closure_refutation_scan(s, *n, 3, 4)
                    .iter()
                    .any(|d| d.closure_products > 0)
            })
            .unwrap();
        let plain = closure_refutation_scan(&sys, n_vars, 3, 4);
        let dir = scratch("corrupt");
        // Two stops leave two committed generations.
        for _ in 0..2 {
            let mut store = Store::open(&dir, Duration::from_secs(3600)).unwrap();
            store.stop_after_rounds = Some(1);
            let _ = closure_refutation_scan_checkpointed(&sys, n_vars, 3, 4, &mut store).unwrap();
        }
        let mut gens: Vec<_> = fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        gens.sort();
        let newest = gens.last().unwrap().clone();
        let victim = ["basis.bin", "catalog.json"]
            .iter()
            .map(|f| newest.join(f))
            .find(|p| p.is_file())
            .unwrap();
        let mut bytes = fs::read(&victim).unwrap();
        let mid = bytes.len() / 2;
        bytes[mid] ^= 0x55;
        fs::write(&victim, bytes).unwrap();
        let (got, _) = run_with_stops(&dir, &sys, n_vars, u32::MAX);
        same(&plain, &got, &name);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_store_refuses_another_system_and_a_directory_of_bad_generations() {
        let all = systems();
        let (_, a, na) = &all[0];
        let (_, b, nb) = &all[all.len() - 1];
        let dir = scratch("other");
        let mut store = Store::open(&dir, Duration::from_secs(3600)).unwrap();
        store.stop_after_rounds = Some(1);
        let _ = closure_refutation_scan_checkpointed(a, *na, 3, 4, &mut store).unwrap();
        let mut again = Store::open(&dir, Duration::from_secs(3600)).unwrap();
        let err = closure_refutation_scan_checkpointed(b, *nb, 3, 4, &mut again).err();
        assert!(err.is_some_and(|e| e.to_string().contains("different system")));
        // A save cut off before its manifest is deleted when the store opens:
        // it can never be restored and may be hundreds of gigabytes.
        let partial = dir.join("gen-00000000000000000099");
        fs::create_dir(&partial).unwrap();
        fs::write(partial.join("basis.bin"), b"cut off").unwrap();
        drop(Store::open(&dir, Duration::from_secs(3600)).unwrap());
        assert!(!partial.exists(), "an uncommitted generation was kept");
        // Damage every generation: opening must refuse, never restart.
        for e in fs::read_dir(&dir).unwrap() {
            let p = e.unwrap().path().join("catalog.json");
            if p.is_file() {
                fs::write(p, b"{}").unwrap();
            }
        }
        assert!(Store::open(&dir, Duration::from_secs(3600)).is_err());
        let _ = fs::remove_dir_all(&dir);
    }
}
