//! `rootfind-subtract-gpu`: the `rootfind-subtract` oracle (`m = 2` or `3`)
//! with its sweep run in batches on the `gpu/rootfind` kernel, through the
//! persistent worker `rootfind_serve` (`gpu/rootfind/rootfind_serve.cpp`).
//!
//! **`m = 3`.**  The CPU oracle probes every candidate `R ∓ F_i` against the
//! negation-folded pair table, keyed by the abscissa of the sum.  Here the
//! device looks every candidate up in the set of those abscissae (probe mode
//! of the kernel, no `j ≥ i` filter); a position it does not report is one
//! where the CPU probe misses its key, so the reported positions, confirmed
//! in order through the CPU probe, give the CPU's first hit, charge for
//! charge.
//!
//! **`worker=daemon`.**  A spawned worker creates its CUDA context in every
//! process (about 0.3 s on an A100, and ecbench runs each record in its own
//! process).  With `worker=daemon` the run connects to a running
//! `rootfind_serve --daemon SOCKET` whose context already exists; the
//! daemon's start is a one-time cost on the host, outside every record, and
//! is reported beside them.
//!
//! **Same search, same answer, same charge.**  For every target the CPU
//! oracle walks the abscissae of the base in order and, at each, the two
//! candidates `R − F_i` then `R + F_i`, returning the first that is a base
//! point.  The kernel computes every candidate of every target in a batch
//! and reports each hit as `R = ε·F_i + τ·F_j` with `j ≥ i` (ε = +1 is the
//! candidate `R − F_i`).  This oracle sorts a target's hits into the CPU's
//! order, recomputes each proposed candidate on the CPU, and confirms it
//! through the CPU oracle's own lookup (key lookup, then the re-addition in
//! the group on the ledger), so the returned indices are those
//! `rootfind-subtract` returns.  The first CPU hit always has `j ≥ i`: a hit
//! with `j < i` would have been found earlier, at abscissa `j`, because the
//! base holds both signs of every abscissa (checked in `prepare`).  Targets
//! whose abscissa is itself in the base (`R = ±F_i`, the degenerate rows)
//! are handed to the CPU oracle whole.
//!
//! **Charged** exactly as `rootfind-subtract` charges the same target: the
//! batched inversion, one group addition per candidate up to and including
//! the first hit (all `2|F_x|` on a refutation), every lookup, and the
//! confirming addition.  The tests demand equal ledgers.  What the GPU
//! computes beyond that (the rest of a decomposing target's sweep, targets a
//! lane round presented but the run never asked for) is not charged and is
//! reported as native counters (`gpu_*`), as `fes-f2-gpu` reports `fes-f2`'s
//! count.  Wall time is the only quantity the device changes.
//!
//! **Batches** come from [`DecompositionOracle::touch`]: the lane target
//! sources (`walk-prime-batch-*`) touch every point of a round before
//! decomposing them, so one round is one kernel launch.  Any other source
//! gives batches of one target.  No worker, no run: `prepare` fails rather
//! than fall back to the CPU, so a `-gpu` arm is never silently a CPU arm.
//! Worker: `ICX_ROOTFIND_WORKER`, else `rootfind_serve_cuda` or
//! `rootfind_serve` beside the running executable.

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

use super::plugins::{RootfindSubtractOracle, SweepRep};
use super::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::ic_boundary::{
    CountedGroup, FactorBase, GroupOps, OracleCounters, PrimeCurve, PrimeInstance, PrimePoint,
};
use crate::cryptanalysis::prime_fast::Fp64;

const MAGIC_INIT: u32 = 0x3353_4652; // "RFS3"
const MAGIC_BATCH: u32 = 0x3142_4652; // "RFB1"
/// Largest batch sent to the worker (and its device buffers' size).
pub const MAX_BATCH: usize = 4096;

/// One relation as the worker reports it (`rf_relation`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkerRelation {
    pub target: u32,
    pub i: u32,
    pub j: u32,
    pub eps: i8,
    pub tau: i8,
}

/// The worker's answer to one batch.
#[derive(Clone, Debug)]
pub struct BatchReply {
    pub relations: Vec<WorkerRelation>,
    /// Device (or emulation) time of the batch, as the worker measured it.
    pub kernel_seconds: f64,
}

/// Find the worker: `ICX_ROOTFIND_WORKER`, then the CUDA and the host build
/// next to the current executable.
pub fn discover_worker() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("ICX_ROOTFIND_WORKER") {
        let path = PathBuf::from(p);
        return path.is_file().then_some(path);
    }
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    ["rootfind_serve_cuda", "rootfind_serve"]
        .iter()
        .map(|n| dir.join(n))
        .find(|c| c.is_file())
}

/// Find the daemon's socket: `ICX_ROOTFIND_SOCKET`, else `rootfind_serve.sock`
/// beside the current executable (ecbench children get a fixed environment).
pub fn discover_socket() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("ICX_ROOTFIND_SOCKET") {
        let path = PathBuf::from(p);
        return path.exists().then_some(path);
    }
    let exe = std::env::current_exe().ok()?;
    let cand = exe.parent()?.join("rootfind_serve.sock");
    cand.exists().then_some(cand)
}

/// How a run reaches the worker: its own `rootfind_serve --serve-bin` child
/// (`worker=spawn`, the default: every process creates its CUDA context), or a
/// connection to a running `rootfind_serve --daemon SOCKET` (`worker=daemon`:
/// the context exists before the run; the daemon's start is a one-time host
/// cost outside every record and must be reported beside them).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerMode {
    Spawn,
    Daemon,
    /// No worker: the sweep runs in this process, [`NativeSweep`] over the
    /// batch's targets on every available thread (`worker=native`).
    Native,
}

impl WorkerMode {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "spawn" => Ok(Self::Spawn),
            "daemon" => Ok(Self::Daemon),
            "native" => Ok(Self::Native),
            other => Err(format!(
                "rootfind-subtract-gpu: worker must be spawn, daemon or native, got `{other}`"
            )),
        }
    }
}

/// A connection to the worker: a child on pipes, or the daemon on a socket.
pub struct RootfindWorker {
    child: Option<Child>,
    writer: Box<dyn Write + Send>,
    reader: BufReader<Box<dyn Read + Send>>,
    path: PathBuf,
    /// Backend the last `init` resolved to (1 emulation, 2 CUDA).
    pub backend: u32,
}

impl RootfindWorker {
    pub fn spawn(worker: &Path) -> Result<Self, String> {
        let mut child = Command::new(worker)
            .arg("--serve-bin")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| format!("failed to start {} --serve-bin: {e}", worker.display()))?;
        let stdin = child.stdin.take().ok_or("no stdin on rootfind worker")?;
        let stdout = child.stdout.take().ok_or("no stdout on rootfind worker")?;
        Ok(Self {
            child: Some(child),
            writer: Box::new(stdin),
            reader: BufReader::new(Box::new(stdout)),
            path: worker.to_path_buf(),
            backend: 0,
        })
    }

    pub fn connect(socket: &Path) -> Result<Self, String> {
        let stream = std::os::unix::net::UnixStream::connect(socket)
            .map_err(|e| format!("rootfind daemon {}: {e}", socket.display()))?;
        let read_half = stream
            .try_clone()
            .map_err(|e| format!("rootfind daemon socket: {e}"))?;
        Ok(Self {
            child: None,
            writer: Box::new(stream),
            reader: BufReader::new(Box::new(read_half)),
            path: socket.to_path_buf(),
            backend: 0,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn send(&mut self, buf: &[u8]) -> Result<(), String> {
        self.writer
            .write_all(buf)
            .and_then(|_| self.writer.flush())
            .map_err(|e| format!("rootfind worker write: {e}"))
    }

    fn read(&mut self, n: usize) -> Result<Vec<u8>, String> {
        let mut b = vec![0u8; n];
        self.reader
            .read_exact(&mut b)
            .map_err(|e| format!("rootfind worker read: {e}"))?;
        Ok(b)
    }

    /// Load a factor base (canonical coordinates) and, for probe mode, the
    /// abscissae candidates are looked up in instead (`probe` non-empty: a
    /// pair table's sums).  Lookup and chunk 0 let the library choose.
    /// `backend`: 0 any, 1 host emulation, 2 CUDA (`rf_backend`); `lookup`: 0 the
    /// library's choice, 3 a hash table built on the device (`rf_lookup`).
    pub fn init(
        &mut self,
        p: u64,
        xs: &[u64],
        ys: &[u64],
        probe: &[u64],
        backend: u32,
        lookup: u32,
        curve_a: u64,
    ) -> Result<u32, String> {
        let mut req = Vec::with_capacity(40 + 16 * xs.len() + 8 * probe.len());
        req.extend_from_slice(&MAGIC_INIT.to_le_bytes());
        req.extend_from_slice(&(xs.len() as u32).to_le_bytes());
        req.extend_from_slice(&p.to_le_bytes());
        for w in [backend, lookup, 0, MAX_BATCH as u32] {
            req.extend_from_slice(&w.to_le_bytes());
        }
        for v in xs.iter().chain(ys) {
            req.extend_from_slice(&v.to_le_bytes());
        }
        req.extend_from_slice(&(probe.len() as u32).to_le_bytes());
        for v in probe {
            req.extend_from_slice(&v.to_le_bytes());
        }
        req.extend_from_slice(&curve_a.to_le_bytes());
        self.send(&req)?;
        let r = self.read(24)?;
        let w = |i: usize| u32::from_le_bytes(r[4 * i..4 * i + 4].try_into().unwrap());
        if w(0) != MAGIC_INIT {
            return Err(format!("rootfind worker: bad init reply magic {:#x}", w(0)));
        }
        if w(1) != 0 {
            return Err(format!("rootfind worker: init status {}", w(1) as i32));
        }
        self.backend = w(2);
        Ok(w(5))
    }

    /// Decompose one batch (canonical coordinates, at most [`MAX_BATCH`]).
    /// `Ok(None)`: the batch's relations overflowed the worker's buffer, so
    /// its list is incomplete; the caller splits the batch.
    pub fn batch(&mut self, xs: &[u64], ys: &[u64]) -> Result<Option<BatchReply>, String> {
        let mut req = Vec::with_capacity(8 + 16 * xs.len());
        req.extend_from_slice(&MAGIC_BATCH.to_le_bytes());
        req.extend_from_slice(&(xs.len() as u32).to_le_bytes());
        for v in xs.iter().chain(ys) {
            req.extend_from_slice(&v.to_le_bytes());
        }
        self.send(&req)?;
        let h = self.read(24)?;
        let w = |i: usize| u32::from_le_bytes(h[4 * i..4 * i + 4].try_into().unwrap());
        if w(0) != MAGIC_BATCH {
            return Err(format!(
                "rootfind worker: bad batch reply magic {:#x}",
                w(0)
            ));
        }
        let (status, found, written) = (w(1) as i32, w(2), w(3));
        let kernel_seconds = f64::from_le_bytes(h[16..24].try_into().unwrap());
        let body = self.read(16 * written as usize)?;
        if status == 1 {
            return Ok(None); // RF_ERR_OVERFLOW
        }
        if status != 0 || found != written {
            return Err(format!(
                "rootfind worker: batch status {status}, {written} of {found} relations"
            ));
        }
        let relations = body
            .chunks_exact(16)
            .map(|c| WorkerRelation {
                target: u32::from_le_bytes(c[0..4].try_into().unwrap()),
                i: u32::from_le_bytes(c[4..8].try_into().unwrap()),
                j: u32::from_le_bytes(c[8..12].try_into().unwrap()),
                eps: c[12] as i8,
                tau: c[13] as i8,
            })
            .collect();
        Ok(Some(BatchReply {
            relations,
            kernel_seconds,
        }))
    }
}

impl Drop for RootfindWorker {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

thread_local! {
    /// One worker connection per thread, kept across runs (a spawned
    /// worker's CUDA context is created once per process); each run
    /// re-`init`s it.  Per thread, so concurrent pipelines in one process
    /// never re-initialise each other's worker between calls.
    static SHARED: RefCell<Option<(WorkerMode, RootfindWorker)>> = const { RefCell::new(None) };
}

fn with_worker<T>(
    mode: WorkerMode,
    f: impl FnOnce(&mut RootfindWorker) -> Result<T, String>,
) -> Result<T, String> {
    SHARED.with(|cell| {
        let mut guard = cell.borrow_mut();
        if guard.as_ref().is_some_and(|(m, _)| *m != mode) {
            *guard = None;
        }
        if guard.is_none() {
            let worker = match mode {
                WorkerMode::Spawn => {
                    let path = discover_worker().ok_or(
                        "rootfind-subtract-gpu: no worker (set ICX_ROOTFIND_WORKER or build \
                         gpu/rootfind `make serve` / `make serve-cuda`)",
                    )?;
                    RootfindWorker::spawn(&path)?
                }
                WorkerMode::Daemon => {
                    let sock = discover_socket().ok_or(
                        "rootfind-subtract-gpu: worker=daemon but no socket (ICX_ROOTFIND_SOCKET, \
                         or rootfind_serve.sock beside the executable)",
                    )?;
                    RootfindWorker::connect(&sock)?
                }
                WorkerMode::Native => {
                    return Err("rootfind-subtract-gpu: worker=native has no worker process".into())
                }
            };
            *guard = Some((mode, worker));
        }
        let out = f(&mut guard.as_mut().expect("worker present").1);
        if out.is_err() {
            *guard = None;
        }
        out
    })
}

/// Sum of the worker's kernel seconds in this process (diagnostic only; not
/// a counter, so records replay).
static KERNEL_SECONDS: Mutex<f64> = Mutex::new(0.0);

pub fn kernel_seconds_total() -> f64 {
    *KERNEL_SECONDS.lock().unwrap_or_else(|p| p.into_inner())
}

/// `n` candidates that are not base points, as `rootfind-subtract` charges
/// them one by one: an addition (two multiplications, a squaring and two
/// conversions) and a lookup each.
fn charge_misses(ops: &mut GroupOps, counters: &mut OracleCounters, n: usize) {
    let n = n as u64;
    ops.adds += n;
    ops.field_ops.muls += 4 * n;
    ops.field_ops.sqrs += n;
    counters.lookups += n;
}

/// A target's hits in the CPU sweep's order: `(position, j, tau)` with
/// `position = 2·i + (0 for ε = +1, 1 for ε = −1)`.
type Hits = Vec<(usize, u32, i8)>;

/// `worker=native`: the batch sweep on the CPU, in this process, with
/// `rootfind-subtract`'s own arithmetic (one Montgomery batched inversion a
/// target, two candidates per abscissa) and the targets of a batch spread
/// over a thread pool.  A candidate is reported when its abscissa is in the
/// probe set (`m = 2`: the base's abscissae; `m = 3`: the pair table's), a
/// superset of the positions where the CPU lookup succeeds; the caller
/// confirms them in order through that lookup, as it does a worker's.
struct NativeSweep {
    f: Fp64,
    xs: Vec<u64>,
    ys: Vec<u64>,
    /// Open-addressing set of Montgomery-form abscissae (`u64::MAX` empty).
    slots: Vec<u64>,
    shift: u32,
    pool: rayon::ThreadPool,
    /// `simd=ifma`: eight targets a step in AVX-512 IFMA lanes; the probe
    /// set then holds canonical abscissae instead of Montgomery forms.
    #[cfg(target_arch = "x86_64")]
    ifma: Option<ifma::Ifma52>,
}

impl NativeSweep {
    /// `probe`: the probe set's abscissae in Montgomery form, or canonical
    /// when `simd` (then the AVX-512 IFMA backend sweeps).
    fn new(
        f: Fp64,
        reps: &[SweepRep],
        probe_mont: &[u64],
        threads: usize,
        simd: bool,
    ) -> Result<Self, String> {
        #[cfg(not(target_arch = "x86_64"))]
        if simd {
            return Err(
                "rootfind-subtract-gpu: simd=ifma needs AVX-512 IFMA on this host and p < 2^52"
                    .into(),
            );
        }
        #[cfg(target_arch = "x86_64")]
        let ifma = if simd {
            let xs: Vec<u64> = reps.iter().map(|r| f.from_mont(r.x)).collect();
            let ys: Vec<u64> = reps.iter().map(|r| f.from_mont(r.y)).collect();
            Some(ifma::Ifma52::new(f.p, &xs, &ys).ok_or(
                "rootfind-subtract-gpu: simd=ifma needs AVX-512 IFMA on this host and p < 2^52",
            )?)
        } else {
            None
        };
        let n = probe_mont.len().max(1);
        let bits = (usize::BITS - (2 * n - 1).leading_zeros()).max(4);
        let mut slots = vec![u64::MAX; 1usize << bits];
        let shift = 64 - bits;
        let mask = slots.len() - 1;
        for &x in probe_mont {
            let mut h = Self::hash(x, shift);
            while slots[h] != u64::MAX && slots[h] != x {
                h = (h + 1) & mask;
            }
            slots[h] = x;
        }
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .map_err(|e| format!("rootfind-subtract-gpu: worker=native thread pool: {e}"))?;
        Ok(Self {
            f,
            xs: reps.iter().map(|r| r.x).collect(),
            ys: reps.iter().map(|r| r.y).collect(),
            slots,
            shift,
            pool,
            #[cfg(target_arch = "x86_64")]
            ifma,
        })
    }

    #[inline(always)]
    fn hash(x: u64, shift: u32) -> usize {
        (x.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> shift) as usize
    }

    #[inline(always)]
    fn contains(&self, x: u64) -> bool {
        let mask = self.slots.len() - 1;
        let mut h = Self::hash(x, self.shift);
        loop {
            let v = self.slots[h];
            if v == x {
                return true;
            }
            if v == u64::MAX {
                return false;
            }
            h = (h + 1) & mask;
        }
    }

    /// One target's reported positions, in the CPU sweep's order.  Rows with
    /// `x_i = x_R` are skipped (such targets go to the CPU oracle whole).
    fn sweep_one(&self, x: u64, y: u64, diffs: &mut Vec<u64>, scratch: &mut Vec<u64>) -> Hits {
        let f = self.f;
        let (xr, yr) = (f.to_mont(x), f.to_mont(y));
        diffs.clear();
        diffs.extend(self.xs.iter().map(|&xi| {
            let d = f.sub(xr, xi);
            if d == 0 {
                f.one
            } else {
                d
            }
        }));
        f.batch_inv(diffs, scratch);
        let mut hits = Hits::new();
        for i in 0..self.xs.len() {
            let (xi, yi) = (self.xs[i], self.ys[i]);
            if xi == xr {
                continue;
            }
            let inv = diffs[i];
            let lm = f.mul(f.add(yr, yi), inv);
            let lp = f.mul(f.sub(yr, yi), inv);
            let base = f.add(xr, xi);
            if self.contains(f.sub(f.sqr(lm), base)) {
                hits.push((2 * i, 0, 1));
            }
            if self.contains(f.sub(f.sqr(lp), base)) {
                hits.push((2 * i + 1, 0, 1));
            }
        }
        hits
    }

    fn sweep(&self, pts: &[PrimePoint]) -> Vec<Hits> {
        use rayon::prelude::*;
        #[cfg(target_arch = "x86_64")]
        if let Some(v) = &self.ifma {
            return self.pool.install(|| {
                pts.par_chunks(ifma::LANES)
                    .map_init(Vec::new, |c, chunk| {
                        v.sweep_lanes(chunk, c, |x| self.contains(x))
                    })
                    .flatten()
                    .collect()
            });
        }
        self.pool.install(|| {
            pts.par_iter()
                .map_init(
                    || (Vec::new(), Vec::new()),
                    |(d, s), p| self.sweep_one(p.x, p.y, d, s),
                )
                .collect()
        })
    }
}

/// The 8-lane sweep of `simd=ifma`: Montgomery arithmetic with `R = 2^52`
/// in AVX-512 IFMA lanes, one target a lane, each with its own batched
/// inversion over the base's abscissae; the same candidates, in the same
/// order, as [`NativeSweep::sweep_one`].
#[cfg(target_arch = "x86_64")]
mod ifma {
    use super::{Hits, PrimePoint};
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    pub(super) const LANES: usize = 8;
    const M52: u64 = (1 << 52) - 1;

    pub(super) struct Ifma52 {
        p: u64,
        /// `−p⁻¹ mod 2^52`.
        pinv: u64,
        /// `2^52 mod p`, `2^104 mod p`.
        r: u64,
        r2: u64,
        /// The base's abscissae and ordinates, times `2^52` mod `p`.
        xs: Vec<u64>,
        ys: Vec<u64>,
    }

    fn mulmod(a: u64, b: u64, p: u64) -> u64 {
        (a as u128 * b as u128 % p as u128) as u64
    }

    fn powmod(mut a: u64, mut e: u64, p: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = mulmod(r, a, p);
            }
            a = mulmod(a, a, p);
            e >>= 1;
        }
        r
    }

    impl Ifma52 {
        /// `None` without AVX-512 IFMA or for `p ≥ 2^52`.
        pub(super) fn new(p: u64, xs: &[u64], ys: &[u64]) -> Option<Self> {
            #[cfg(target_arch = "x86_64")]
            let ok = std::arch::is_x86_feature_detected!("avx512f")
                && std::arch::is_x86_feature_detected!("avx512ifma");
            #[cfg(not(target_arch = "x86_64"))]
            let ok = false;
            if !ok || !(5..1 << 52).contains(&p) || p & 1 == 0 {
                return None;
            }
            let mut inv = 1u64;
            for _ in 0..6 {
                inv = inv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(inv)));
            }
            let r = ((1u128 << 52) % p as u128) as u64;
            let r2 = mulmod(r, r, p);
            Some(Self {
                p,
                pinv: inv.wrapping_neg() & M52,
                r,
                r2,
                xs: xs.iter().map(|&x| mulmod(x, r, p)).collect(),
                ys: ys.iter().map(|&y| mulmod(y, r, p)).collect(),
            })
        }

        /// Each target's reported positions (as [`NativeSweep::sweep_one`]
        /// reports them); `contains` answers for canonical abscissae.
        pub(super) fn sweep_lanes(
            &self,
            pts: &[PrimePoint],
            prefix: &mut Vec<[u64; LANES]>,
            contains: impl Fn(u64) -> bool,
        ) -> Vec<Hits> {
            debug_assert!(!pts.is_empty() && pts.len() <= LANES);
            let mut xr = [0u64; LANES];
            let mut yr = [0u64; LANES];
            for l in 0..LANES {
                let q = pts[l.min(pts.len() - 1)];
                xr[l] = mulmod(q.x, self.r, self.p);
                yr[l] = mulmod(q.y, self.r, self.p);
            }
            let mut hits = vec![Hits::new(); LANES];
            // SAFETY: `new` returned `Some` only where the CPU has
            // avx512f and avx512ifma.
            #[cfg(target_arch = "x86_64")]
            unsafe {
                self.sweep_avx(&xr, &yr, prefix, &contains, &mut hits)
            };
            hits.truncate(pts.len());
            hits
        }

        /// `a⁻¹·2^52` from `a·2^52`, per lane (scalar).
        fn inv_lanes(&self, v: [u64; LANES]) -> [u64; LANES] {
            let mut o = [0u64; LANES];
            for l in 0..LANES {
                o[l] = mulmod(self.r2, powmod(v[l], self.p - 2, self.p), self.p);
            }
            o
        }

        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "avx512f,avx512ifma")]
        unsafe fn sweep_avx(
            &self,
            xr: &[u64; LANES],
            yr: &[u64; LANES],
            prefix: &mut Vec<[u64; LANES]>,
            contains: &impl Fn(u64) -> bool,
            hits: &mut [Hits],
        ) {
            let ld = |a: &[u64; LANES]| _mm512_loadu_si512(a.as_ptr() as *const _);
            let st = |v: __m512i| {
                let mut a = [0u64; LANES];
                _mm512_storeu_si512(a.as_mut_ptr() as *mut _, v);
                a
            };
            let p = _mm512_set1_epi64(self.p as i64);
            let pinv = _mm512_set1_epi64(self.pinv as i64);
            let z = _mm512_setzero_si512();
            let one_r = _mm512_set1_epi64(self.r as i64);
            let one = _mm512_set1_epi64(1);
            // Montgomery product, R = 2^52, inputs and output in [0, p).
            let mm = |a: __m512i, b: __m512i| {
                let lo = _mm512_madd52lo_epu64(z, a, b);
                let hi = _mm512_madd52hi_epu64(z, a, b);
                let m = _mm512_madd52lo_epu64(z, lo, pinv);
                let c = _mm512_srli_epi64::<52>(_mm512_madd52lo_epu64(lo, m, p));
                let u = _mm512_add_epi64(_mm512_madd52hi_epu64(hi, m, p), c);
                _mm512_min_epu64(u, _mm512_sub_epi64(u, p))
            };
            let add = |a: __m512i, b: __m512i| {
                let s = _mm512_add_epi64(a, b);
                _mm512_min_epu64(s, _mm512_sub_epi64(s, p))
            };
            let sub = |a: __m512i, b: __m512i| {
                let d = _mm512_sub_epi64(a, b);
                _mm512_min_epu64(d, _mm512_add_epi64(d, p))
            };
            let vxr = ld(xr);
            let vyr = ld(yr);
            let k = self.xs.len();
            // x_R − x_i, with 1 (in Montgomery form) where it is 0.
            let diff = |i: usize| {
                let d = sub(vxr, _mm512_set1_epi64(self.xs[i] as i64));
                let zm = _mm512_cmpeq_epi64_mask(d, z);
                (_mm512_mask_blend_epi64(zm, d, one_r), zm)
            };
            prefix.clear();
            prefix.reserve(k);
            let mut acc = one_r;
            for i in 0..k {
                acc = mm(acc, diff(i).0);
                prefix.push(st(acc));
            }
            let mut inv = ld(&self.inv_lanes(st(acc)));
            for i in (1..k).rev() {
                let inv_i = mm(inv, ld(&prefix[i - 1]));
                inv = mm(inv, diff(i).0);
                prefix[i] = st(inv_i);
            }
            prefix[0] = st(inv);
            for i in 0..k {
                let (_, zm) = diff(i);
                let xi = _mm512_set1_epi64(self.xs[i] as i64);
                let yi = _mm512_set1_epi64(self.ys[i] as i64);
                let inv_i = ld(&prefix[i]);
                let lm = mm(add(vyr, yi), inv_i);
                let lp = mm(sub(vyr, yi), inv_i);
                let base = add(vxr, xi);
                let xm = st(mm(sub(mm(lm, lm), base), one));
                let xp = st(mm(sub(mm(lp, lp), base), one));
                for (l, h) in hits.iter_mut().enumerate() {
                    if zm & (1 << l) != 0 {
                        continue;
                    }
                    if contains(xm[l]) {
                        h.push((2 * i, 0, 1));
                    }
                    if contains(xp[l]) {
                        h.push((2 * i + 1, 0, 1));
                    }
                }
            }
        }
    }
}

pub struct RootfindSubtractGpuOracle<'i> {
    cpu: RootfindSubtractOracle<'i>,
    m: u32,
    mode: WorkerMode,
    backend: u32,
    lookup: u32,
    /// `probe_build=pairs`: the abscissa of each base representative, by
    /// position in the sweep (for the degenerate rows' pair search).
    abscissa_pos: HashMap<u64, usize>,
    pending: RefCell<Vec<PrimePoint>>,
    swept: HashMap<(u64, u64), Hits>,
    counters: BTreeMap<String, u64>,
    native: Option<NativeSweep>,
}

impl<'i> RootfindSubtractGpuOracle<'i> {
    pub fn new(m: u32, instance: &'i PrimeInstance) -> Self {
        Self {
            cpu: RootfindSubtractOracle::new(m, instance),
            m,
            mode: WorkerMode::Spawn,
            backend: 0,
            lookup: 0,
            abscissa_pos: HashMap::new(),
            pending: RefCell::new(Vec::new()),
            swept: HashMap::new(),
            counters: BTreeMap::new(),
            native: None,
        }
    }

    fn bump(&mut self, name: &str, by: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += by;
    }

    /// `probe_build=pairs`: charge the negation-folded pair table the worker
    /// builds exactly as `PairTable::build_negation_folded` would: every visit
    /// is one group addition (`a(a + 1)` of them over `a` abscissae, each with
    /// both signs), whose field work depends only on its case: `a(a − 1)`
    /// general additions, `a` doublings (`P + P`) and `a` identities (`P − P`).
    /// One addition of each case, made here on the real ledger type, gives
    /// the per-case charge.  Work counters: `a²` insert attempts (the
    /// non-identity sums) and `a` index reads.
    fn charge_pair_table_build(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
    ) -> Result<(), String> {
        let g = ctx.group;
        let reps: Vec<SweepRep> = self
            .cpu
            .sweep()
            .ok_or("rootfind-subtract did not prepare")?
            .1
            .to_vec();
        let a = reps.len() as u64;
        for rep in &reps {
            let p = fb.points[rep.plus];
            if rep.plus == rep.minus || p.y == 0 || fb.points[rep.minus] != g.neg(p) {
                return Err(
                    "rootfind-subtract-gpu: probe_build=pairs needs both signs of every abscissa \
                     and no point of order two"
                        .into(),
                );
            }
        }
        let p0 = fb.points[reps[0].plus];
        let one = |q: PrimePoint| {
            let mut o = GroupOps::default();
            g.add(&mut o, p0, q);
            o
        };
        let dbl = one(p0);
        let ident = one(fb.points[reps[0].minus]);
        let gen = if a >= 2 {
            one(fb.points[reps[1].plus])
        } else {
            GroupOps::default()
        };
        for (case, n) in [(gen, a * a.saturating_sub(1)), (dbl, a), (ident, a)] {
            ops.adds += case.adds * n;
            ops.doubles += case.doubles * n;
            ops.scalar_mults += case.scalar_mults * n;
            ops.field_ops.muls += case.field_ops.muls * n;
            ops.field_ops.sqrs += case.field_ops.sqrs * n;
            ops.field_ops.invs += case.field_ops.invs * n;
            ops.montgomery_reductions += case.montgomery_reductions * n;
        }
        let mut work = BTreeMap::new();
        if a * a > 0 {
            work.insert("table_insert_attempts".to_string(), a * a);
            work.insert("lookups".to_string(), a);
        }
        self.cpu.set_deferred_table_work(work);
        self.abscissa_pos = reps
            .iter()
            .enumerate()
            .map(|(i, r)| (fb.points[r.plus].x, i))
            .collect();
        Ok(())
    }

    /// The pair `(i, j)` of fb indices at build index `b` (`rf_pair_offset`).
    fn pair_at(&self, b: u64) -> Option<(usize, usize)> {
        let (_, reps) = self.cpu.sweep()?;
        let n = reps.len() as u64;
        let off = |ai: u64| 2 * (ai * n - ai * ai.saturating_sub(1) / 2);
        let (mut lo, mut hi) = (0u64, n); // the largest ai with off(ai) <= b
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if off(mid) <= b {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let rem = b - off(lo);
        let bi = (lo + rem / 2) as usize;
        let rb = reps.get(bi)?;
        let j = if rem.is_multiple_of(2) {
            rb.plus
        } else {
            rb.minus
        };
        Some((reps[lo as usize].plus, j))
    }

    /// The least build index whose sum has the abscissa of `t` (the pair the
    /// CPU table keeps for that key), by enumeration: `P_ai + Q = ±t` with `Q`
    /// a base point above an abscissa `bi ≥ ai`.  Only the degenerate rows
    /// need this; their extra arithmetic is lookup work, not charged.
    fn first_pair_for(
        &self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        t: PrimePoint,
    ) -> Option<u64> {
        let g = ctx.group;
        let (_, reps) = self.cpu.sweep()?;
        let n = reps.len() as u64;
        let mut scratch = GroupOps::default();
        let mut best: Option<u64> = None;
        for (ai, rep) in reps.iter().enumerate() {
            let p = fb.points[rep.plus];
            for target in [t, g.neg(t)] {
                let q = g.add(&mut scratch, target, g.neg(p));
                if q.infinity {
                    continue;
                }
                let Some(&bi) = self.abscissa_pos.get(&q.x) else {
                    continue;
                };
                if bi < ai {
                    continue;
                }
                let s = if q == fb.points[reps[bi].plus] {
                    0
                } else if q == fb.points[reps[bi].minus] {
                    1
                } else {
                    continue;
                };
                if bi == ai && s == 1 {
                    continue; // P − P
                }
                let ai = ai as u64;
                let b = 2 * (ai * n - ai * ai.saturating_sub(1) / 2) + 2 * (bi as u64 - ai) + s;
                best = Some(best.map_or(b, |x: u64| x.min(b)));
            }
        }
        best
    }

    /// `rootfind-subtract`'s `m = 3` lookup of `t = R − F_summand`, with the
    /// pair table's answer for `t`'s abscissa given as a build index (from the
    /// device, or searched for a degenerate row): one lookup, then the
    /// probe's confirming addition on the ledger, exactly as
    /// `PairTable::probe`.
    #[allow(clippy::too_many_arguments)]
    fn lookup_pairs(
        &self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        summand: usize,
        t: PrimePoint,
        b: Option<u64>,
    ) -> Option<Vec<usize>> {
        let g = ctx.group;
        counters.lookups += 1;
        if g.is_identity(&t) {
            return None;
        }
        let b = match b {
            Some(b) => b,
            None => self.first_pair_for(ctx, fb, t)?,
        };
        let (i, j) = self.pair_at(b)?;
        let s = g.add(ops, fb.points[i], fb.points[j]);
        if s == t {
            Some(vec![summand, i, j])
        } else if s == g.neg(t) {
            Some(vec![summand, fb.neg_index[i], fb.neg_index[j]])
        } else {
            None
        }
    }

    /// Sweep `point` and every touched point not yet swept, in batches.
    fn sweep_batch(&mut self, point: PrimePoint) -> Result<(), String> {
        let mut batch = vec![point];
        let mut seen: std::collections::HashSet<(u64, u64)> = [(point.x, point.y)].into();
        for p in self.pending.borrow_mut().drain(..) {
            let key = (p.x, p.y);
            if !p.infinity && !self.swept.contains_key(&key) && seen.insert(key) {
                batch.push(p);
            }
        }
        if let Some(native) = &self.native {
            let hits = native.sweep(&batch);
            for (p, h) in batch.iter().zip(hits) {
                self.swept.insert((p.x, p.y), h);
            }
            self.bump("gpu_batches", 1);
            self.bump("gpu_targets_swept", batch.len() as u64);
            return Ok(());
        }
        // A batch whose relations overflow the worker's buffer is split in
        // halves until each fits (only dense m = 3 tables on tiny curves).
        let mut work: Vec<&[PrimePoint]> = batch.chunks(MAX_BATCH).rev().collect();
        let mode = self.mode;
        while let Some(chunk) = work.pop() {
            let xs: Vec<u64> = chunk.iter().map(|p| p.x).collect();
            let ys: Vec<u64> = chunk.iter().map(|p| p.y).collect();
            let Some(reply) = with_worker(mode, |w| w.batch(&xs, &ys))? else {
                if chunk.len() == 1 {
                    return Err("rootfind worker: one target overflows the relation buffer".into());
                }
                self.bump("gpu_batch_splits", 1);
                let (lo, hi) = chunk.split_at(chunk.len() / 2);
                work.push(hi);
                work.push(lo);
                continue;
            };
            *KERNEL_SECONDS.lock().unwrap_or_else(|p| p.into_inner()) += reply.kernel_seconds;
            let mut hits: Vec<Hits> = vec![Vec::new(); chunk.len()];
            for r in &reply.relations {
                if r.tau == 0 {
                    continue; // degenerate row: such targets go to the CPU oracle
                }
                let k = r.target as usize;
                if k >= chunk.len() {
                    return Err(format!("rootfind worker: target index {k} out of range"));
                }
                let pos = 2 * r.i as usize + usize::from(r.eps < 0);
                hits[k].push((pos, r.j, r.tau));
            }
            for (p, mut h) in chunk.iter().zip(hits) {
                h.sort_unstable();
                self.swept.insert((p.x, p.y), h);
            }
            self.bump("gpu_batches", 1);
            self.bump("gpu_targets_swept", chunk.len() as u64);
        }
        Ok(())
    }
}

impl DecompositionOracle<PrimeCurve> for RootfindSubtractGpuOracle<'_> {
    fn name(&self) -> &str {
        "rootfind-subtract-gpu"
    }

    fn summands(&self) -> u32 {
        self.m
    }

    fn describe(&self, params: &Params) -> String {
        format!(
            "{}; the sweep runs in batches on the gpu/rootfind kernel (rootfind_serve worker), \
             every hit confirmed on the CPU in rootfind-subtract's order and charge \
             (m = 3: candidates probed against the pair table's abscissae on the device)",
            self.cpu.describe(params)
        )
    }

    fn parameters(&self) -> &[(&str, &str)] {
        &[
            (
                "m",
                "summands, 2 (candidates looked up in the base) or 3 (in the pair table's \
                 abscissae on the device, then probed on the CPU)",
            ),
            (
                "worker",
                "spawn (default: a worker process per run, which creates its CUDA context), \
                 daemon (connect to rootfind_serve --daemon; its start is a one-time host cost) or \
                 native (no worker: rootfind-subtract's own sweep over each batch's targets on a \
                 thread pool in this process)",
            ),
            (
                "backend",
                "any (default: the device if the worker has one), cuda (fail without one) or \
                 emulate (the same kernel code on every host thread: a multi-threaded CPU sweep)",
            ),
            (
                "probe_build",
                "device (default for m = 3: the lookup table built on the device) or host",
            ),
            (
                "threads",
                "worker=native only: the thread pool's size (default: every thread the process \
                 may use)",
            ),
            (
                "simd",
                "worker=native only: off (default, scalar 64-bit Montgomery) or ifma (eight targets \
                 at once in AVX-512 IFMA lanes; refused without IFMA or for p >= 2^52)",
            ),
        ]
    }

    fn prepare(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        params: &Params,
        ops: &mut GroupOps,
    ) -> Result<(), String> {
        if !(2..=3).contains(&self.m) {
            return Err(format!(
                "rootfind-subtract-gpu takes m = 2 or 3, got {}",
                self.m
            ));
        }
        for key in params.0.keys() {
            if !["m", "worker", "backend", "probe_build", "threads", "simd"].contains(&key.as_str())
            {
                return Err(format!(
                    "rootfind-subtract-gpu reads only `m`, `worker`, `backend`, `probe_build`, \
                     `threads` and `simd`; `{key}` is unknown"
                ));
            }
        }
        self.mode = WorkerMode::parse(params.0.get("worker").map_or("spawn", |s| s.as_str()))?;
        // `worker=native` has no backend or device table; `threads` belongs to it
        // alone (0 or absent: every thread the process may use).
        let threads = match params.0.get("threads") {
            None => 0,
            Some(t) => t.parse::<usize>().map_err(|_| {
                format!("rootfind-subtract-gpu: threads must be a count, got `{t}`")
            })?,
        };
        if self.mode == WorkerMode::Native {
            for k in ["backend", "probe_build"] {
                if params.0.contains_key(k) {
                    return Err(format!(
                        "rootfind-subtract-gpu: worker=native takes no `{k}` (it sweeps on the \
                         CPU against the probe set the CPU oracle builds)"
                    ));
                }
            }
        } else if params.0.contains_key("threads") || params.0.contains_key("simd") {
            return Err("rootfind-subtract-gpu: `threads` and `simd` need worker=native".into());
        }
        let simd = match params.0.get("simd").map_or("off", |s| s.as_str()) {
            "off" => false,
            "ifma" => true,
            other => {
                return Err(format!(
                    "rootfind-subtract-gpu: simd must be off or ifma, got `{other}`"
                ))
            }
        };
        // `backend=cuda` fails closed when the worker has no device; `emulate`
        // runs the same kernel code on every host thread (a multi-threaded CPU
        // sweep); `any` (the default) takes the device when there is one.
        self.backend = match params.0.get("backend").map_or("any", |s| s.as_str()) {
            "any" => 0,
            "emulate" => 1,
            "cuda" => 2,
            other => {
                return Err(format!(
                    "rootfind-subtract-gpu: backend must be any, cuda or emulate, got `{other}`"
                ))
            }
        };
        // `probe_build=device` (the default for m = 3): the probe set's hash
        // table is built on the device; `host`: the library's host-built table
        // (sorted for large sets), the first m = 3 measurement's path;
        // `pairs` (m = 3): the pair table itself is built by the worker (on the
        // device under CUDA) from the base, and charged here as the CPU build.
        let default_build = if self.m == 3 && self.mode != WorkerMode::Native {
            "device"
        } else {
            "host"
        };
        self.lookup = match params
            .0
            .get("probe_build")
            .map_or(default_build, |s| s.as_str())
        {
            "host" => 0,
            "device" => 3,
            "pairs" if self.m == 3 => 4,
            other => {
                return Err(format!(
                "rootfind-subtract-gpu: probe_build must be host or device (or pairs at m = 3), \
                     got `{other}`"
            ))
            }
        };
        if self.lookup == 4 {
            self.cpu.defer_pair_table();
        }
        // The CPU oracle sees only `m`: the worker changes where the sweep
        // runs, not the search.
        let mut cpu_params = params.clone();
        for k in ["worker", "backend", "probe_build", "threads", "simd"] {
            cpu_params.0.remove(k);
        }
        self.cpu.prepare(ctx, fb, &cpu_params, ops)?;
        let (f, reps) = self
            .cpu
            .sweep()
            .ok_or("rootfind-subtract did not prepare")?;
        // Both signs of every abscissa (or a 2-torsion point) are in the
        // base: what makes the kernel's `j ≥ i` hits contain the CPU's first.
        let mut xs = Vec::with_capacity(reps.len());
        let mut ys = Vec::with_capacity(reps.len());
        for rep in reps {
            let (x, y) = (f.from_mont(rep.x), f.from_mont(rep.y));
            if self.m == 2 && rep.plus == rep.minus && y != 0 {
                return Err(
                    "rootfind-subtract-gpu needs a negation-closed base (both signs of every \
                     abscissa)"
                        .into(),
                );
            }
            xs.push(x);
            ys.push(y);
        }
        if xs.is_empty() {
            return Err("rootfind-subtract-gpu: empty base".into());
        }
        let p = ctx.group.p;
        // The backend the worker resolved is not a counter: with `backend=any`
        // or `emulate` a replay on a host without a GPU reads the same record.
        // A `backend=cuda` record names its backend in the method parameters
        // and replays only where a device answers (it fails closed elsewhere).
        // m = 3: the device looks candidates up in the abscissae of the pair
        // table the CPU oracle built (and charged); a hit there is a position
        // where the CPU probe finds a key, so every position the CPU sweep
        // could stop at is reported, with no `j ≥ i` filter.
        let probe = if self.m == 3 && self.lookup != 4 {
            let px = self
                .cpu
                .pair_abscissae()
                .ok_or("rootfind-subtract-gpu: the pair table has no plain map")?;
            if px.is_empty() {
                return Err("rootfind-subtract-gpu: empty pair table".into());
            }
            px
        } else {
            Vec::new()
        };
        self.native = None;
        if self.mode == WorkerMode::Native {
            let (f, reps) = self
                .cpu
                .sweep()
                .ok_or("rootfind-subtract did not prepare")?;
            // The IFMA sweep probes canonical abscissae; the scalar one
            // Montgomery forms.
            let set: Vec<u64> = match (self.m == 3, simd) {
                (true, true) => probe.clone(),
                (true, false) => probe.iter().map(|&x| f.to_mont(x)).collect(),
                (false, true) => reps.iter().map(|r| f.from_mont(r.x)).collect(),
                (false, false) => reps.iter().map(|r| r.x).collect(),
            };
            let threads = if threads == 0 {
                std::thread::available_parallelism().map_or(1, |n| n.get())
            } else {
                threads
            };
            self.native = Some(NativeSweep::new(f, reps, &set, threads, simd)?);
            self.swept.clear();
            self.pending.borrow_mut().clear();
            self.counters.clear();
            return Ok(());
        }
        let (backend, lookup) = (self.backend, self.lookup);
        let curve_a = ctx.group.a;
        if lookup == 4 {
            self.charge_pair_table_build(ctx, fb, ops)?;
        }
        let entries = with_worker(self.mode, |w| {
            w.init(p, &xs, &ys, &probe, backend, lookup, curve_a)
        })?;
        if lookup == 4 && entries == 0 {
            return Err("rootfind-subtract-gpu: the worker built an empty pair table".into());
        }
        self.swept.clear();
        self.pending.borrow_mut().clear();
        self.counters.clear();
        Ok(())
    }

    fn setup_native(&self) -> BTreeMap<String, u64> {
        self.cpu.setup_native()
    }

    fn setup_work_native(&self) -> BTreeMap<String, u64> {
        self.cpu.setup_work_native()
    }

    fn relation_work_native(&self) -> BTreeMap<String, u64> {
        self.cpu.relation_work_native()
    }

    fn relation_counters(&self) -> BTreeMap<String, u64> {
        let mut out = self.cpu.relation_counters();
        out.extend(self.counters.clone());
        out.insert("gpu_targets_unused".into(), self.swept.len() as u64);
        out
    }

    fn line_guard(&self) -> bool {
        // The cache-line guard is the one that touches lane rounds ahead of
        // their decomposition; membership answers are identical.
        true
    }

    fn touch(&self, _ctx: &InstanceCtx<PrimeCurve>, point: &PrimePoint) {
        self.pending.borrow_mut().push(*point);
    }

    fn decompose(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: PrimePoint,
    ) -> Option<Vec<usize>> {
        if point.infinity {
            return None;
        }
        let (f, k) = {
            let (f, reps) = self.cpu.sweep()?;
            (f, reps.len())
        };
        let key = (point.x, point.y);
        if !self.swept.contains_key(&key) {
            if let Err(e) = self.sweep_batch(point) {
                // Fail closed: a run with a broken worker must not report
                // relations it did not find, nor silently become a CPU run.
                panic!("rootfind-subtract-gpu: {e}");
            }
        }
        let hits = self.swept.remove(&key).unwrap_or_default();
        if self.lookup == 4 {
            return self.decompose_pairs(ctx, fb, ops, counters, point, hits);
        }
        if fb.x_index.contains_key(&point.x) {
            // R = ±F_i: the CPU oracle takes the degenerate rows its own way.
            self.bump("gpu_cpu_fallback_targets", 1);
            return self.cpu.decompose(ctx, fb, ops, counters, point);
        }
        self.cpu.charge_sweep_setup(ops, counters);
        let xr = f.to_mont(point.x);
        let yr = f.to_mont(point.y);
        let mut next = 0usize; // first candidate position not yet charged
        for (pos, _j, _tau) in hits {
            if pos < next {
                continue; // a second hit at one candidate: already examined
            }
            charge_misses(ops, counters, pos - next);
            next = pos + 1;
            let (summand, t) = {
                let (_, reps) = self.cpu.sweep()?;
                let rep = reps[pos / 2];
                let inv = f.inv(f.sub(xr, rep.x));
                let lam = if pos % 2 == 0 {
                    f.mul(f.add(yr, rep.y), inv)
                } else {
                    f.mul(f.sub(yr, rep.y), inv)
                };
                let x3 = f.sub(f.sub(f.sqr(lam), xr), rep.x);
                let y3 = f.sub(f.mul(lam, f.sub(xr, x3)), yr);
                let summand = if pos % 2 == 0 { rep.plus } else { rep.minus };
                (
                    summand,
                    PrimePoint::affine(f.from_mont(x3), f.from_mont(y3)),
                )
            };
            RootfindSubtractOracle::charge_candidate(ops);
            self.bump("gpu_hits_confirmed_on_cpu", 1);
            if let Some(found) = self.cpu.lookup(ctx, fb, ops, counters, point, summand, t) {
                return Some(found);
            }
        }
        charge_misses(ops, counters, 2 * k - next);
        None
    }
}

impl RootfindSubtractGpuOracle<'_> {
    /// `probe_build=pairs`: `rootfind-subtract:m=3`'s sweep, candidate for
    /// candidate.  Positions the device reported are confirmed through
    /// [`Self::lookup_pairs`] with the device's build index; the two rows of an
    /// abscissa equal to `x_R` (if any) are taken as the CPU takes them (one
    /// charged addition each, no candidate charge) with the pair searched for.
    fn decompose_pairs(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        counters: &mut OracleCounters,
        point: PrimePoint,
        hits: Hits,
    ) -> Option<Vec<usize>> {
        let g = ctx.group;
        let (f, k) = {
            let (f, reps) = self.cpu.sweep()?;
            (f, reps.len())
        };
        let degenerate = self.abscissa_pos.get(&point.x).copied();
        if degenerate.is_some() {
            self.bump("gpu_degenerate_rows", 1);
        }
        self.cpu.charge_sweep_setup(ops, counters);
        let xr = f.to_mont(point.x);
        let yr = f.to_mont(point.y);
        // Misses in [from, to), skipping the degenerate row's two positions.
        let misses = |from: usize, to: usize| -> usize {
            let n = to.saturating_sub(from);
            match degenerate {
                Some(d) => n - (from..to).filter(|&q| q / 2 == d).count(),
                None => n,
            }
        };
        let mut events: Vec<(usize, Option<u64>)> = hits
            .iter()
            .filter(|(pos, _, _)| Some(pos / 2) != degenerate)
            .map(|&(pos, j, _)| (pos, Some(j as u64)))
            .collect();
        if let Some(d) = degenerate {
            events.push((2 * d, None));
            events.push((2 * d + 1, None));
        }
        events.sort_unstable();
        events.dedup_by_key(|e| e.0);
        let mut next = 0usize;
        for (pos, b) in events {
            charge_misses(ops, counters, misses(next, pos));
            next = pos + 1;
            let found = if Some(pos / 2) == degenerate {
                let (_, reps) = self.cpu.sweep()?;
                let rep = reps[pos / 2];
                let summand = if pos % 2 == 0 { rep.plus } else { rep.minus };
                let t = g.add(ops, point, g.neg(fb.points[summand]));
                self.lookup_pairs(ctx, fb, ops, counters, summand, t, None)
            } else {
                let (summand, t) = {
                    let (_, reps) = self.cpu.sweep()?;
                    let rep = reps[pos / 2];
                    let inv = f.inv(f.sub(xr, rep.x));
                    let lam = if pos % 2 == 0 {
                        f.mul(f.add(yr, rep.y), inv)
                    } else {
                        f.mul(f.sub(yr, rep.y), inv)
                    };
                    let x3 = f.sub(f.sub(f.sqr(lam), xr), rep.x);
                    let y3 = f.sub(f.mul(lam, f.sub(xr, x3)), yr);
                    let summand = if pos % 2 == 0 { rep.plus } else { rep.minus };
                    (
                        summand,
                        PrimePoint::affine(f.from_mont(x3), f.from_mont(y3)),
                    )
                };
                RootfindSubtractOracle::charge_candidate(ops);
                self.bump("gpu_hits_confirmed_on_cpu", 1);
                self.lookup_pairs(ctx, fb, ops, counters, summand, t, b)
            };
            if found.is_some() {
                return found;
            }
        }
        charge_misses(ops, counters, misses(next, 2 * k));
        None
    }
}

#[cfg(test)]
mod tests {
    //! Needs a worker: `make -C gpu/rootfind serve` and
    //! `ICX_ROOTFIND_WORKER=$PWD/gpu/rootfind/rootfind_serve`.  Without one
    //! the tests print why and pass, as the root crate builds no native code.
    use super::*;
    use crate::cryptanalysis::ic_boundary::{
        find_prime_order_curve, prime_instance_for, Calibration,
    };
    use crate::cryptanalysis::ic_framework::plugins::PrimeAbscissaBase;
    use crate::cryptanalysis::ic_framework::stages::Targets;
    use crate::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec, RunReport};

    fn worker_present() -> bool {
        let ok = std::env::var("ICX_ROOTFIND_WORKER").is_ok() && discover_worker().is_some();
        if !ok {
            eprintln!("skipped: set ICX_ROOTFIND_WORKER to a built rootfind_serve");
        }
        ok
    }

    fn ctx(inst: &PrimeInstance, target: PrimePoint) -> InstanceCtx<'_, PrimeCurve> {
        InstanceCtx {
            group: &inst.curve,
            generator: inst.generator_point(),
            target,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: None,
        }
    }

    fn without_gpu(m: &BTreeMap<String, u64>) -> BTreeMap<String, u64> {
        m.iter()
            .filter(|(k, _)| !k.starts_with("gpu_"))
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }

    fn assert_same(cpu: &RunReport, gpu: &RunReport, label: &str) {
        assert_eq!(cpu.recovered, gpu.recovered, "{label}: recovered");
        assert_eq!(cpu.verified, gpu.verified, "{label}: verified");
        assert_eq!(cpu.exhausted, gpu.exhausted, "{label}: exhausted");
        let (dc, dg) = (&cpu.decomposition, &gpu.decomposition);
        assert_eq!(dc.targets_tried, dg.targets_tried, "{label}: targets");
        assert_eq!(dc.relations_found, dg.relations_found, "{label}: relations");
        assert_eq!(dc.setup.group_ops, dg.setup.group_ops, "{label}: setup ops");
        assert_eq!(
            dc.setup.work_native, dg.setup.work_native,
            "{label}: setup work"
        );
        assert_eq!(
            dc.cost.group_ops, dg.cost.group_ops,
            "{label}: relation ops"
        );
        assert_eq!(
            without_gpu(&dc.cost.native),
            without_gpu(&dg.cost.native),
            "{label}: relation native"
        );
        assert_eq!(
            dc.cost.work_native, dg.cost.work_native,
            "{label}: relation work"
        );
        let (lc, lg) = (&cpu.linear_algebra, &gpu.linear_algebra);
        assert_eq!(
            (lc.rows, lc.rank, lc.work),
            (lg.rows, lg.rank, lg.work),
            "{label}: LA"
        );
        assert_eq!(cpu.total_gae, gpu.total_gae, "{label}: total gae");
        assert_eq!(cpu.s, gpu.s, "{label}: S");
    }

    /// Whole pipeline runs: the GPU oracle reaches the same relations, the
    /// same logarithm and the same counted cost in every phase as
    /// `rootfind-subtract`, on lane targets (batched) and walk targets
    /// (batches of one).
    #[test]
    fn gpu_oracle_matches_rootfind_subtract_end_to_end() {
        if !worker_present() {
            return;
        }
        run_equality(
            2,
            "spawn",
            &[
                Targets::WalkPrimeBatch16,
                Targets::WalkPrimeBatch256,
                Targets::Walk,
            ],
        );
    }

    /// `m = 3`: candidates probed against the pair table's abscissae on the
    /// device give the CPU probe's first hit and charge.
    #[test]
    fn gpu_oracle_m3_matches_rootfind_subtract_end_to_end() {
        if !worker_present() {
            return;
        }
        run_equality(3, "spawn", &[Targets::WalkPrimeBatch256, Targets::Walk]);
    }

    /// `worker=daemon`: the same runs through a daemon on a socket.
    #[test]
    fn gpu_oracle_daemon_matches_rootfind_subtract() {
        if !worker_present() {
            return;
        }
        let worker = discover_worker().expect("worker");
        let dir = std::env::temp_dir().join(format!("rf-daemon-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let sock = dir.join("rootfind_serve.sock");
        let mut daemon = Command::new(&worker)
            .arg("--daemon")
            .arg(&sock)
            .spawn()
            .expect("daemon");
        for _ in 0..200 {
            if sock.exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        std::env::set_var("ICX_ROOTFIND_SOCKET", &sock);
        let out = std::panic::catch_unwind(|| {
            run_equality(2, "daemon", &[Targets::WalkPrimeBatch256]);
            run_equality(3, "daemon", &[Targets::WalkPrimeBatch256]);
            run_equality_with(
                3,
                "daemon",
                &[Targets::WalkPrimeBatch256],
                &[("probe_build", "host")],
            );
            run_equality_with(
                3,
                "daemon",
                &[Targets::WalkPrimeBatch256],
                &[("probe_build", "pairs")],
            );
        });
        let _ = daemon.kill();
        let _ = daemon.wait();
        let _ = std::fs::remove_dir_all(&dir);
        if let Err(e) = out {
            std::panic::resume_unwind(e);
        }
    }

    /// `worker=native` (no worker process): the in-process multi-threaded sweep
    /// equals `rootfind-subtract`, `m = 2` and `3`, one thread and three, on
    /// single targets and lane batches.
    #[test]
    fn native_sweep_matches_rootfind_subtract() {
        for m in [2, 3] {
            for threads in ["1", "3"] {
                run_equality_with(
                    m,
                    "native",
                    &[Targets::Walk, Targets::WalkPrimeBatch256],
                    &[("threads", threads)],
                );
            }
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    #[test]
    fn ifma_request_refuses_before_allocating_a_native_sweep() {
        let result = NativeSweep::new(Fp64::new(1_000_003), &[], &[], 1, true);
        assert!(matches!(result, Err(message) if message.contains("needs AVX-512 IFMA")));
    }

    fn ifma_present() -> bool {
        #[cfg(target_arch = "x86_64")]
        let ok = ifma::Ifma52::new(1_000_003, &[1], &[1]).is_some();
        #[cfg(not(target_arch = "x86_64"))]
        let ok = false;
        if !ok {
            eprintln!("skipped: no AVX-512 IFMA on this host");
        }
        ok
    }

    /// `simd=ifma` against the scalar sweep, hit for hit: targets in and out
    /// of the base (degenerate rows), negatives, and a partial last chunk.
    #[test]
    fn ifma_lanes_match_scalar_sweep() {
        if !ifma_present() {
            return;
        }
        for (inst, n) in [
            (prime_instance_for(20, 59297), 40u64),
            (prime_instance_for(14, 1), 9),
        ] {
            let f = Fp64::new(inst.curve.p);
            let g = inst.generator_point();
            let pt = |k: u64| inst.curve.mul(&mut GroupOps::default(), g, k);
            let reps: Vec<SweepRep> = (1..=n)
                .map(&pt)
                .map(|q| SweepRep {
                    x: f.to_mont(q.x),
                    y: f.to_mont(q.y),
                    plus: 0,
                    minus: 0,
                })
                .collect();
            // A probe set holding the base's abscissae and some others.
            let canon: Vec<u64> = (1..=3 * n).map(|k| pt(k).x).collect();
            let mont: Vec<u64> = canon.iter().map(|&x| f.to_mont(x)).collect();
            let scalar = NativeSweep::new(f, &reps, &mont, 2, false).unwrap();
            let lanes = NativeSweep::new(f, &reps, &canon, 2, true).unwrap();
            let mut targets: Vec<PrimePoint> = (1..=29).map(|k| pt(7 * k + 3)).collect();
            targets.push(pt(5)); // R = F_5: a degenerate row
            targets.push(inst.curve.neg(pt(2)));
            for len in [targets.len(), 1, 8, 9] {
                let a = scalar.sweep(&targets[..len]);
                let b = lanes.sweep(&targets[..len]);
                assert_eq!(a, b, "{} len {len}", inst.name);
                assert!(
                    a.iter().any(|h| !h.is_empty()),
                    "{}: no hits to compare",
                    inst.name
                );
            }
        }
    }

    /// `simd=ifma` on primes up to just below 2^52 (the curves above use
    /// p < 2^36; the Montgomery product's final reduction only matters past
    /// about 2^51): random field values, a probe set holding about a third of
    /// the candidates' abscissae (computed with u128), lanes against the
    /// scalar sweep and against that expectation.
    #[test]
    fn ifma_lanes_match_scalar_sweep_near_2_52() {
        if !ifma_present() {
            return;
        }
        fn mulm(a: u64, b: u64, p: u64) -> u64 {
            (a as u128 * b as u128 % p as u128) as u64
        }
        fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
            let mut r = 1;
            while e > 0 {
                if e & 1 == 1 {
                    r = mulm(r, a, p);
                }
                a = mulm(a, a, p);
                e >>= 1;
            }
            r
        }
        fn is_prime(n: u64) -> bool {
            if n < 4 || n.is_multiple_of(2) {
                return n == 2 || n == 3;
            }
            let (mut d, mut s) = (n - 1, 0);
            while d.is_multiple_of(2) {
                d /= 2;
                s += 1;
            }
            'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
                let mut x = powm(a % n, d, n);
                if x <= 1 || x == n - 1 {
                    continue;
                }
                for _ in 1..s {
                    x = mulm(x, x, n);
                    if x == n - 1 {
                        continue 'w;
                    }
                }
                return false;
            }
            true
        }
        let mut seed = 0x1234_5678_9abc_def0u64;
        let mut rnd = |n: u64| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed % n
        };
        let starts = [(1u64 << 52) - 1, (1 << 52) - 5000, 3 << 50, (1 << 51) + 777];
        for start in starts {
            let mut p = start | 1;
            while !is_prime(p) {
                p -= 2;
            }
            let f = Fp64::new(p);
            let k = 37usize;
            let xs: Vec<u64> = (0..k).map(|_| rnd(p)).collect();
            let ys: Vec<u64> = (0..k).map(|_| rnd(p)).collect();
            let reps: Vec<SweepRep> = xs
                .iter()
                .zip(&ys)
                .map(|(&x, &y)| SweepRep {
                    x: f.to_mont(x),
                    y: f.to_mont(y),
                    plus: 0,
                    minus: 0,
                })
                .collect();
            let mut targets: Vec<PrimePoint> = (0..19)
                .map(|_| PrimePoint::affine(rnd(p), rnd(p)))
                .collect();
            targets.push(PrimePoint::affine(xs[3], ys[3])); // a degenerate row
            targets.push(PrimePoint::affine(p - 1, p - 1)); // values at the edge
                                                            // The candidates' abscissae with u128 arithmetic; keep every third.
            let mut canon = Vec::new();
            let mut expected = vec![0usize; targets.len()];
            for (t, q) in targets.iter().enumerate() {
                for i in 0..k {
                    if xs[i] == q.x {
                        continue;
                    }
                    let inv = powm((q.x + p - xs[i]) % p, p - 2, p);
                    let num = [(q.y + ys[i]) % p, (q.y + p - ys[i]) % p];
                    for (e, yy) in num.into_iter().enumerate() {
                        let lam = mulm(yy, inv, p);
                        let x3 = (mulm(lam, lam, p) + 2 * p - q.x - xs[i]) % p;
                        if (t + i + e) % 3 == 0 {
                            canon.push(x3);
                            expected[t] += 1;
                        }
                    }
                }
            }
            canon.extend((0..50).map(|_| rnd(p)));
            let mont: Vec<u64> = canon.iter().map(|&x| f.to_mont(x)).collect();
            let scalar = NativeSweep::new(f, &reps, &mont, 2, false).unwrap();
            let lanes = NativeSweep::new(f, &reps, &canon, 2, true).unwrap();
            for len in [targets.len(), 1, 8, 9] {
                let a = scalar.sweep(&targets[..len]);
                let b = lanes.sweep(&targets[..len]);
                assert_eq!(a, b, "p = {p:#x}, len {len}");
                for t in 0..len {
                    assert!(b[t].len() >= expected[t], "p = {p:#x}, target {t}");
                }
            }
        }
    }

    /// `worker=native,simd=ifma` equals `rootfind-subtract` over whole pipelines.
    #[test]
    fn native_ifma_matches_rootfind_subtract() {
        if !ifma_present() {
            return;
        }
        for m in [2, 3] {
            run_equality_with(
                m,
                "native",
                &[Targets::Walk, Targets::WalkPrimeBatch256],
                &[("threads", "3"), ("simd", "ifma")],
            );
        }
    }

    /// The emulation backend asked for by name, and both probe-table builds.
    #[test]
    fn gpu_oracle_backend_and_probe_build_match() {
        if !worker_present() {
            return;
        }
        run_equality_with(
            2,
            "spawn",
            &[Targets::WalkPrimeBatch256],
            &[("backend", "emulate")],
        );
        for build in ["host", "device"] {
            run_equality_with(
                3,
                "spawn",
                &[Targets::WalkPrimeBatch256],
                &[("backend", "emulate"), ("probe_build", build)],
            );
        }
    }

    /// `probe_build=pairs`: the worker builds the pair table; relations, every
    /// phase's ledger (the table build's charge included) and S equal
    /// `rootfind-subtract:m=3`'s.
    #[test]
    fn gpu_oracle_pairs_build_matches() {
        if !worker_present() {
            return;
        }
        run_equality_with(
            3,
            "spawn",
            &[Targets::WalkPrimeBatch256, Targets::Walk],
            &[("probe_build", "pairs")],
        );
    }

    /// The degenerate rows (`x_R` a base abscissa) under `probe_build=pairs`:
    /// every base point and its double as targets, decomposed by both oracles
    /// with fresh ledgers; same answer, same ledger, same counters.
    #[test]
    fn gpu_oracle_pairs_degenerate_rows_match() {
        use crate::cryptanalysis::ic_framework::stages::FactorBaseBuilder;
        if !worker_present() {
            return;
        }
        for (inst, size) in [
            (prime_instance_for(14, 1), 16usize),
            (find_prime_order_curve(17, 3116), 28),
        ] {
            let g = inst.generator_point();
            let c = ctx(&inst, g);
            let mut fparams = Params::default();
            fparams.set("size", size.to_string());
            let base = PrimeAbscissaBase { instance: &inst };
            let fb = base.build(&c, &fparams, &mut GroupOps::default()).unwrap();
            let mut cpu_params = Params::default();
            cpu_params.set("m", "3");
            let mut gpu_params = cpu_params.clone();
            gpu_params.set("probe_build", "pairs");
            let mut cpu_o = RootfindSubtractOracle::new(3, &inst);
            let mut gpu_o = RootfindSubtractGpuOracle::new(3, &inst);
            let (mut po, mut pg) = (GroupOps::default(), GroupOps::default());
            cpu_o.prepare(&c, &fb, &cpu_params, &mut po).unwrap();
            gpu_o.prepare(&c, &fb, &gpu_params, &mut pg).unwrap();
            assert_eq!(po, pg, "{}: table build charge", inst.name);
            assert_eq!(
                cpu_o.setup_work_native(),
                gpu_o.setup_work_native(),
                "{}",
                inst.name
            );
            let mut found = 0;
            for &p in &fb.points {
                let doubled = inst.curve.add(&mut GroupOps::default(), p, p);
                for point in [p, doubled] {
                    let (mut oc, mut og) = (GroupOps::default(), GroupOps::default());
                    let (mut cc, mut cg) = (OracleCounters::default(), OracleCounters::default());
                    let a = cpu_o.decompose(&c, &fb, &mut oc, &mut cc, point);
                    let b = gpu_o.decompose(&c, &fb, &mut og, &mut cg, point);
                    assert_eq!(a, b, "{}: relation", inst.name);
                    assert_eq!(oc, og, "{}: ledger", inst.name);
                    assert_eq!(
                        format!("{cc:?}"),
                        format!("{cg:?}"),
                        "{}: counters",
                        inst.name
                    );
                    found += usize::from(a.is_some());
                }
            }
            assert!(found > 0, "{}: no target decomposed", inst.name);
            assert!(
                gpu_o
                    .relation_counters()
                    .get("gpu_degenerate_rows")
                    .copied()
                    .unwrap_or(0)
                    > 0,
                "{}: no degenerate row exercised",
                inst.name
            );
        }
    }

    fn run_equality(m: u32, worker: &str, sources: &[Targets]) {
        run_equality_with(m, worker, sources, &[]);
    }

    fn run_equality_with(m: u32, worker: &str, sources: &[Targets], extra: &[(&str, &str)]) {
        let calib = Calibration::default();
        let mut solved = 0;
        for (inst, size) in [
            (prime_instance_for(14, 1), 16usize),
            (prime_instance_for(18, 1), 32),
            (prime_instance_for(20, 59297), 48),
            (find_prime_order_curve(17, 3116), 28),
        ] {
            let g = inst.generator_point();
            for planted_seed in [11u64, 20_261_114] {
                let planted = planted_seed % inst.r;
                let q = inst.curve.mul(&mut GroupOps::default(), g, planted);
                let c = ctx(&inst, q);
                for &targets in sources {
                    for seed in [7u64, 3116] {
                        let mut spec = PipelineSpec {
                            factor_base: "prime-abscissa".into(),
                            oracle: "rootfind-subtract".into(),
                            targets,
                            max_trials: 4_000_000,
                            seed,
                            ..Default::default()
                        };
                        spec.factor_base_params.set("size", size.to_string());
                        spec.oracle_params.set("m", m.to_string());
                        let base = PrimeAbscissaBase { instance: &inst };
                        let mut cpu_o = RootfindSubtractOracle::new(m, &inst);
                        let cpu = run_pipeline(&c, &spec, &base, &mut cpu_o, planted, &calib, None)
                            .unwrap();
                        spec.oracle = "rootfind-subtract-gpu".into();
                        spec.oracle_params.set("worker", worker);
                        for (k, v) in extra {
                            spec.oracle_params.set(k, *v);
                        }
                        let mut gpu_o = RootfindSubtractGpuOracle::new(m, &inst);
                        let gpu = run_pipeline(&c, &spec, &base, &mut gpu_o, planted, &calib, None)
                            .unwrap();
                        let label = format!(
                            "m={m} worker={worker} {} {} seed {seed} planted {planted}",
                            inst.name,
                            targets.name()
                        );
                        assert_same(&cpu, &gpu, &label);
                        let gn = &gpu.decomposition.cost.native;
                        assert!(
                            gn.get("gpu_targets_swept").copied().unwrap_or(0) > 0,
                            "{label}"
                        );
                        if targets != Targets::Walk {
                            assert!(
                                gn["gpu_targets_swept"] > gn["gpu_batches"],
                                "{label}: lane rounds should batch"
                            );
                        }
                        if cpu.verified {
                            assert_eq!(cpu.recovered, Some(planted), "{label}");
                            solved += 1;
                        }
                    }
                }
            }
        }
        assert!(solved > 0, "no run solved");
    }

    /// The fail-closed checks: m = 4 and unknown parameters are refused.
    #[test]
    fn gpu_oracle_refuses_bad_parameters() {
        let inst = prime_instance_for(14, 1);
        let q = inst.generator_point();
        let c = ctx(&inst, q);
        let mut spec = PipelineSpec {
            factor_base: "prime-abscissa".into(),
            oracle: "rootfind-subtract-gpu".into(),
            ..Default::default()
        };
        spec.factor_base_params.set("size", "16");
        let base = PrimeAbscissaBase { instance: &inst };
        let mut o = RootfindSubtractGpuOracle::new(4, &inst);
        let r = run_pipeline(&c, &spec, &base, &mut o, 1, &Calibration::default(), None);
        assert!(r.is_err());
        spec.oracle_params.set("worker", "elsewhere");
        let mut o = RootfindSubtractGpuOracle::new(2, &inst);
        let r = run_pipeline(&c, &spec, &base, &mut o, 1, &Calibration::default(), None);
        assert!(r.is_err());
        for (k, v) in [("backend", "tpu"), ("probe_build", "elsewhere")] {
            spec.oracle_params.0.remove("worker");
            spec.oracle_params.set(k, v);
            let mut o = RootfindSubtractGpuOracle::new(2, &inst);
            let r = run_pipeline(&c, &spec, &base, &mut o, 1, &Calibration::default(), None);
            assert!(r.is_err(), "{k}={v}");
            spec.oracle_params.0.remove(k);
        }
        // `threads` only with worker=native; native takes no backend or build.
        for extra in [
            &[("threads", "2")][..],
            &[("worker", "native"), ("backend", "emulate")],
            &[("worker", "native"), ("probe_build", "host")],
            &[("worker", "native"), ("threads", "two")],
            &[("worker", "native"), ("simd", "avx9")],
            &[("worker", "spawn"), ("simd", "ifma")],
        ] {
            let mut sp = spec.clone();
            for (k, v) in extra {
                sp.oracle_params.set(k, *v);
            }
            let mut o = RootfindSubtractGpuOracle::new(2, &inst);
            let r = run_pipeline(&c, &sp, &base, &mut o, 1, &Calibration::default(), None);
            assert!(r.is_err(), "{extra:?}");
        }
    }
}
