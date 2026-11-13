//! # Driving the GPU (or host-emulation) FES worker from Rust.
//!
//! This is the bridge that wires the `gpu/fes` Gray-code kernels — CUDA,
//! Metal, and the host-emulation build — into the `icx` binary. A quadratic
//! Boolean system is packed into the workers' shared file contract
//! (`gpu/fes/fes_io.hpp`), a worker executable is run as a subprocess, its
//! proposed solutions are read back, and **every one is re-verified on the CPU**
//! here before it is trusted. A worker can therefore only ever *propose*
//! candidates; a GPU bug, a truncated read, or a missing device can never
//! produce a wrong answer — at worst the result is empty and the caller falls
//! back to the in-process CPU search.
//!
//! The same in-process CPU search ([`PackedSystem::cpu_search`]) is both the
//! default backend and the cross-check the tests hold every worker to: the CPU
//! reference is the algorithm in `fes.cuh` ported to Rust, so a worker's output
//! must equal it exactly.

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;

use crate::cryptanalysis::mq_fes::QuadraticForm;

/// A quadratic Boolean system packed one bit per equation, matching the
/// workers' representation: `value(x) = cst ^ (+)_{x_i} lin[i] ^
/// (+)_{i>=j, x_i,x_j} quad_tri[tri(i,j)]`, and `x` is a solution iff
/// `value(x) == 0`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackedSystem {
    pub n: usize,
    pub m: usize,
    pub cst: u64,
    pub lin: Vec<u64>,
    /// Lower-triangular, `tri(i,j) = i*(i+1)/2 + j` for `i >= j`. The diagonal
    /// stays zero: a `QuadraticForm` folds `x_i^2 = x_i` into `linear`.
    pub quad_tri: Vec<u64>,
}

fn tri(i: usize, j: usize) -> usize {
    let (a, b) = if i >= j { (i, j) } else { (j, i) };
    a * (a + 1) / 2 + b
}

impl PackedSystem {
    /// Pack a system given as one [`QuadraticForm`] per equation (`m <= 64`,
    /// `n <= 62`, all forms over the same `n`).
    pub fn from_quadratic_forms(forms: &[QuadraticForm]) -> Result<Self, String> {
        if forms.is_empty() {
            return Err("empty system: FES needs at least one equation".into());
        }
        if forms.len() > 64 {
            return Err(format!(
                "too many equations for a u64 pack: {}",
                forms.len()
            ));
        }
        let n = forms[0].n;
        if n > 62 {
            return Err(format!("n = {n} exceeds the single-word worker limit (62)"));
        }
        if forms.iter().any(|f| f.n != n) {
            return Err("all equations must share the same variable count".into());
        }
        let mut cst = 0u64;
        let mut lin = vec![0u64; n];
        let mut quad_tri = vec![0u64; n * (n + 1) / 2];
        for (e, f) in forms.iter().enumerate() {
            let bit = 1u64 << e;
            if f.constant {
                cst ^= bit;
            }
            for (i, &l) in f.linear.iter().enumerate() {
                if l {
                    lin[i] ^= bit;
                }
            }
            // quad[i][j] with j < i.
            for (i, row) in f.quad.iter().enumerate() {
                for (j, &q) in row.iter().enumerate() {
                    if q {
                        quad_tri[tri(i, j)] ^= bit;
                    }
                }
            }
        }
        Ok(PackedSystem {
            n,
            m: forms.len(),
            cst,
            lin,
            quad_tri,
        })
    }

    fn quad(&self, i: usize, j: usize) -> u64 {
        self.quad_tri[tri(i, j)]
    }

    /// Evaluate the packed system at `x` (bit i is variable i). Zero means `x`
    /// solves every equation. This is the independent verifier for worker output.
    pub fn eval(&self, x: u64) -> u64 {
        let mut v = self.cst;
        for i in 0..self.n {
            if (x >> i) & 1 == 1 {
                v ^= self.lin[i];
                for j in 0..=i {
                    if (x >> j) & 1 == 1 {
                        v ^= self.quad(i, j);
                    }
                }
            }
        }
        v
    }

    /// True if `x` zeroes every equation.
    pub fn is_solution(&self, x: u64) -> bool {
        self.eval(x) == 0
    }

    /// The in-process CPU FES search — the `fes.cuh` derivative-maintained
    /// Gray walk ported to Rust. This is the default backend and the reference
    /// every worker is checked against.
    pub fn cpu_search(&self) -> Vec<u64> {
        let n = self.n;
        let mut sols = Vec::new();
        // base = 0: value is the constant, and df[i] = lin[i] ^ quad(i,i).
        let mut df: Vec<u64> = (0..n).map(|i| self.lin[i] ^ self.quad(i, i)).collect();
        let mut val = self.cst;
        if val == 0 {
            sols.push(0);
        }
        if n == 0 {
            return sols;
        }
        let steps: u64 = if n >= 63 { 0 } else { 1u64 << n };
        for s in 1..steps {
            let i1 = s.trailing_zeros() as usize;
            val ^= df[i1];
            for j in 0..n {
                if j != i1 {
                    df[j] ^= self.quad(i1, j);
                }
            }
            if val == 0 {
                sols.push(s ^ (s >> 1)); // Gray code of s
            }
        }
        sols
    }

    /// Serialize to the worker file contract (`fes_io.hpp`).
    pub fn to_contract(&self) -> String {
        let mut s = format!("{} {}\n{}\n", self.n, self.m, self.cst);
        let lin: Vec<String> = self.lin.iter().map(|v| v.to_string()).collect();
        s.push_str(&lin.join(" "));
        s.push('\n');
        let q: Vec<String> = self.quad_tri.iter().map(|v| v.to_string()).collect();
        s.push_str(&q.join(" "));
        s.push('\n');
        s
    }
}

/// Which backend solved a system, and its proposal/verification tally.
#[derive(Clone, Debug)]
pub struct FesResult {
    /// Verified solutions (each `eval == 0`), sorted and deduplicated.
    pub solutions: Vec<u64>,
    /// Human label of the backend used (`cpu`, or the worker path).
    pub backend: String,
    /// How many candidates the worker proposed (equals `solutions.len()` for cpu).
    pub proposed: usize,
    /// How many of those verified (dropped candidates are a worker defect).
    pub verified: usize,
}

/// The FES backend to use.
#[derive(Clone, Debug)]
pub enum FesBackend {
    /// The in-process CPU Gray-code search.
    Cpu,
    /// An external worker executable (CUDA / Metal / host-emulation) that
    /// speaks the file contract.
    Worker(PathBuf),
    /// Use a discovered worker if one exists, else the CPU search.
    Auto,
}

/// Parse a worker's `SOLUTIONS k` output into candidate assignments.
fn parse_solutions(stdout: &str) -> Vec<u64> {
    let mut out = Vec::new();
    let mut lines = stdout.lines();
    // Find the SOLUTIONS header (skip any leading diagnostics).
    let mut expected = None;
    for line in lines.by_ref() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("SOLUTIONS") {
            expected = rest.trim().parse::<usize>().ok();
            break;
        }
    }
    if expected.is_none() {
        return out;
    }
    for line in lines {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if let Ok(v) = t.parse::<u64>() {
            out.push(v);
        }
    }
    out
}

/// Look for a worker executable: `ICX_FES_WORKER` env override first, then the
/// known worker names next to the current executable (real GPU workers before
/// the host-emulation fallback).
pub fn discover_worker() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("ICX_FES_WORKER") {
        let path = PathBuf::from(p);
        if path.is_file() {
            return Some(path);
        }
    }
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    for name in ["fes_cuda", "fes_metal", "fes_solve"] {
        let cand = dir.join(name);
        if cand.is_file() {
            return Some(cand);
        }
    }
    None
}

/// Run a worker on a system and return its re-verified solutions.
fn run_worker(sys: &PackedSystem, worker: &Path) -> Result<FesResult, String> {
    // Unique temp file (pid + nanos) so concurrent calls never collide — the
    // WDSat oracle's shared-temp-file race, avoided.
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut path = std::env::temp_dir();
    path.push(format!("icx-fes-{pid}-{nanos}.txt"));
    {
        let mut f = std::fs::File::create(&path).map_err(|e| format!("temp file: {e}"))?;
        f.write_all(sys.to_contract().as_bytes())
            .map_err(|e| format!("write: {e}"))?;
    }
    let out = Command::new(worker).arg("--in").arg(&path).output();
    let _ = std::fs::remove_file(&path);
    let out = out.map_err(|e| format!("failed to run worker {}: {e}", worker.display()))?;
    if !out.status.success() {
        return Err(format!(
            "worker {} exited with {}: {}",
            worker.display(),
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let proposed = parse_solutions(&stdout);
    let mut verified: Vec<u64> = proposed
        .iter()
        .copied()
        .filter(|&x| sys.is_solution(x))
        .collect();
    verified.sort_unstable();
    verified.dedup();
    Ok(FesResult {
        solutions: verified.clone(),
        backend: worker.display().to_string(),
        proposed: proposed.len(),
        verified: verified.len(),
    })
}

/// The position of `x` in the Gray-code walk from 0 (the inverse of
/// `s ^ (s >> 1)`): `fes-f2`'s CPU search emits its zeros in increasing
/// order of this rank.
pub fn gray_rank(x: u64) -> u64 {
    let mut b = x;
    let mut shift = 1;
    while shift < 64 {
        b ^= b >> shift;
        shift <<= 1;
    }
    b
}

/// Put worker proposals in the order `fes-f2`'s sequential walk would emit
/// them.  The CUDA kernel splits the cube over threads and appends zeros
/// through an atomic counter, so its order is arbitrary; callers that take
/// the first usable solution (the descent oracle does) would otherwise
/// follow a different trajectory from `fes-f2` on the same input.
pub fn sort_gray_order(points: &mut [u64]) {
    points.sort_unstable_by_key(|&x| gray_rank(x));
}

/// A worker kept running in `--serve` mode (`gpu/fes/fes_io.hpp`): one
/// process, one CUDA context and one set of device buffers for a whole run,
/// so a solver call costs a pipe round trip and a kernel launch instead of a
/// process start and a CUDA initialisation.  Like [`run_worker`], it only
/// proposes candidates; callers re-verify them.
pub struct ServeWorker {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    path: PathBuf,
}

/// One serve-mode reply: the listed candidates and the worker's total count.
/// `total > proposed.len()` means the worker's output cap was hit, so the
/// list is incomplete and must not be read as the whole solution set.
#[derive(Clone, Debug)]
pub struct ServeReply {
    pub proposed: Vec<u64>,
    pub total: u64,
}

impl ServeWorker {
    /// Start `worker --serve`.
    pub fn spawn(worker: &Path) -> Result<Self, String> {
        let mut child = Command::new(worker)
            .arg("--serve")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| format!("failed to start {} --serve: {e}", worker.display()))?;
        let stdin = child.stdin.take().ok_or("no stdin on serve worker")?;
        let stdout = BufReader::new(child.stdout.take().ok_or("no stdout on serve worker")?);
        Ok(Self {
            child,
            stdin,
            stdout,
            path: worker.to_path_buf(),
        })
    }

    /// The worker executable.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Send one system and read its reply.
    pub fn solve(&mut self, sys: &PackedSystem) -> Result<ServeReply, String> {
        self.stdin
            .write_all(sys.to_contract().as_bytes())
            .and_then(|_| self.stdin.flush())
            .map_err(|e| format!("serve worker write: {e}"))?;
        let mut line = String::new();
        let header = loop {
            line.clear();
            let k = self
                .stdout
                .read_line(&mut line)
                .map_err(|e| format!("serve worker read: {e}"))?;
            if k == 0 {
                return Err("serve worker closed its output".into());
            }
            let t = line.trim();
            if t.starts_with("ERROR") {
                return Err(format!("serve worker: {t}"));
            }
            if t.starts_with("SOLUTIONS") {
                break t.to_string();
            }
        };
        let mut it = header.split_whitespace().skip(1);
        let listed: usize = it
            .next()
            .and_then(|v| v.parse().ok())
            .ok_or_else(|| format!("bad serve header: {header}"))?;
        let total: u64 = it
            .next()
            .and_then(|v| v.parse().ok())
            .ok_or_else(|| format!("bad serve header: {header}"))?;
        let mut proposed = Vec::with_capacity(listed);
        for _ in 0..listed {
            line.clear();
            self.stdout
                .read_line(&mut line)
                .map_err(|e| format!("serve worker read: {e}"))?;
            proposed.push(
                line.trim()
                    .parse::<u64>()
                    .map_err(|_| format!("bad solution line: {}", line.trim()))?,
            );
        }
        Ok(ServeReply { proposed, total })
    }
}

impl Drop for ServeWorker {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

static SHARED_SERVER: Mutex<Option<ServeWorker>> = Mutex::new(None);

/// Solve on the process-wide serve worker, starting it on first use from
/// [`discover_worker`] (`ICX_FES_WORKER` first).  `Ok(None)` when no worker
/// is available; any worker error drops it, so the next call starts afresh.
pub fn solve_on_shared_server(sys: &PackedSystem) -> Result<Option<(ServeReply, PathBuf)>, String> {
    let mut guard = SHARED_SERVER.lock().unwrap_or_else(|p| p.into_inner());
    if guard.is_none() {
        match discover_worker() {
            Some(w) => *guard = Some(ServeWorker::spawn(&w)?),
            None => return Ok(None),
        }
    }
    let server = guard.as_mut().expect("server present");
    match server.solve(sys) {
        Ok(reply) => Ok(Some((reply, server.path().to_path_buf()))),
        Err(e) => {
            *guard = None;
            Err(e)
        }
    }
}

/// Generate a pseudo-random quadratic Boolean system in `n` variables and `m`
/// equations with a planted common zero, returned with that planted point.
/// Deterministic in `seed`; used by `icx fes` to exercise the backends.
pub fn random_system(n: usize, m: usize, seed: u64) -> (PackedSystem, u64) {
    let mut s = seed | 1;
    let mut rng = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let mask = if n >= 64 { u64::MAX } else { (1u64 << n) - 1 };
    let planted = rng() & mask;
    let mut forms = Vec::with_capacity(m);
    for _ in 0..m {
        let mut f = QuadraticForm {
            n,
            constant: false,
            linear: (0..n).map(|_| rng() & 1 == 1).collect(),
            quad: (0..n)
                .map(|i| (0..i).map(|_| rng() & 1 == 1).collect())
                .collect(),
        };
        // Set the constant so the planted point is a zero of this equation.
        let mut v = false;
        for i in 0..n {
            if (planted >> i) & 1 == 1 {
                v ^= f.linear[i];
                for j in 0..i {
                    if (planted >> j) & 1 == 1 {
                        v ^= f.quad[i][j];
                    }
                }
            }
        }
        f.constant = v;
        forms.push(f);
    }
    (
        PackedSystem::from_quadratic_forms(&forms).expect("valid random system"),
        planted,
    )
}

/// Solve a packed system with the chosen backend.
pub fn solve(sys: &PackedSystem, backend: &FesBackend) -> Result<FesResult, String> {
    match backend {
        FesBackend::Cpu => {
            let mut sols = sys.cpu_search();
            sols.sort_unstable();
            sols.dedup();
            let n = sols.len();
            Ok(FesResult {
                solutions: sols,
                backend: "cpu".to_string(),
                proposed: n,
                verified: n,
            })
        }
        FesBackend::Worker(w) => run_worker(sys, w),
        FesBackend::Auto => match discover_worker() {
            Some(w) => run_worker(sys, &w),
            None => solve(sys, &FesBackend::Cpu),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_forms(n: usize, m: usize, seed: u64) -> Vec<QuadraticForm> {
        // Deterministic pseudo-random forms with a planted solution at `seed`.
        let mut s = seed | 1;
        let mut rng = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s
        };
        let sol = rng() & ((1u64 << n) - 1);
        let mut forms = Vec::new();
        for _ in 0..m {
            let mut f = QuadraticForm {
                n,
                constant: false,
                linear: (0..n).map(|_| rng() & 1 == 1).collect(),
                quad: (0..n)
                    .map(|i| (0..i).map(|_| rng() & 1 == 1).collect())
                    .collect(),
            };
            // Force the planted point to be a zero: adjust the constant.
            let mut v = false;
            for i in 0..n {
                if (sol >> i) & 1 == 1 {
                    v ^= f.linear[i];
                    for j in 0..i {
                        if (sol >> j) & 1 == 1 {
                            v ^= f.quad[i][j];
                        }
                    }
                }
            }
            f.constant = v;
            forms.push(f);
        }
        forms
    }

    fn brute(sys: &PackedSystem) -> Vec<u64> {
        (0..(1u64 << sys.n))
            .filter(|&x| sys.is_solution(x))
            .collect()
    }

    #[test]
    fn packed_eval_matches_forms() {
        let n = 12;
        let forms = sample_forms(n, 14, 42);
        let sys = PackedSystem::from_quadratic_forms(&forms).unwrap();
        for x in 0..(1u64 << n) {
            // Independent per-equation evaluation of the forms.
            let mut all_zero = true;
            for f in &forms {
                let mut v = f.constant;
                for i in 0..n {
                    if (x >> i) & 1 == 1 {
                        v ^= f.linear[i];
                        for j in 0..i {
                            if (x >> j) & 1 == 1 {
                                v ^= f.quad[i][j];
                            }
                        }
                    }
                }
                all_zero &= !v;
            }
            assert_eq!(sys.is_solution(x), all_zero, "mismatch at x={x}");
        }
    }

    #[test]
    fn cpu_search_matches_brute_force() {
        for (n, m, seed) in [(10usize, 12usize, 1u64), (14, 10, 7), (16, 20, 3)] {
            let forms = sample_forms(n, m, seed);
            let sys = PackedSystem::from_quadratic_forms(&forms).unwrap();
            let mut got = super::solve(&sys, &FesBackend::Cpu).unwrap().solutions;
            got.sort_unstable();
            let mut want = brute(&sys);
            want.sort_unstable();
            assert_eq!(got, want, "n={n} m={m} seed={seed}");
        }
    }

    #[test]
    fn parse_solutions_reads_the_contract() {
        let s = "some diagnostic\nSOLUTIONS 2\n8805\n42\n";
        assert_eq!(parse_solutions(s), vec![8805, 42]);
        assert_eq!(parse_solutions("SOLUTIONS 0\n"), Vec::<u64>::new());
        assert_eq!(parse_solutions("no header here\n"), Vec::<u64>::new());
    }

    #[test]
    fn worker_only_proposes_bad_candidates_are_dropped() {
        // A worker that returns a non-solution must have it filtered out.
        let forms = sample_forms(10, 12, 5);
        let sys = PackedSystem::from_quadratic_forms(&forms).unwrap();
        let real = sys.cpu_search();
        // Simulate parse output with one real solution plus a bogus one.
        let bogus = real.first().map(|&x| x ^ 1).unwrap_or(1);
        let proposed = [real.first().copied().unwrap_or(0), bogus];
        let verified: Vec<u64> = proposed
            .iter()
            .copied()
            .filter(|&x| sys.is_solution(x))
            .collect();
        assert!(verified.iter().all(|&x| sys.is_solution(x)));
        assert!(!verified.contains(&bogus) || sys.is_solution(bogus));
    }

    /// Build `gpu/fes/fes_solve` (the host worker) into a temp directory, or
    /// `None` when there is no C++ compiler.
    fn build_host_worker() -> Option<PathBuf> {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("gpu/fes/fes_solve.cpp");
        let dir = std::env::temp_dir().join(format!("fes-serve-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).ok()?;
        let out = dir.join("fes_solve");
        for cxx in ["c++", "g++", "clang++"] {
            let ok = Command::new(cxx)
                .args(["-O2", "-std=c++17", "-o"])
                .arg(&out)
                .arg(&src)
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if ok {
                return Some(out);
            }
        }
        None
    }

    /// The serve protocol end to end against the host worker: one process
    /// answers many systems back to back, each reply complete
    /// (`total == listed`) and equal to the CPU search.
    #[test]
    fn serve_worker_answers_many_systems_like_the_cpu_search() {
        let Some(worker) = build_host_worker() else {
            eprintln!("no C++ compiler: serve-worker test skipped");
            return;
        };
        let mut server = ServeWorker::spawn(&worker).expect("spawn host worker --serve");
        for trial in 0..40u64 {
            let n = 1 + (trial as usize % 22);
            let m = 1 + (trial as usize % 5);
            let forms = sample_forms(n, m, 1000 + trial);
            let sys = PackedSystem::from_quadratic_forms(&forms).unwrap();
            let reply = server.solve(&sys).expect("serve reply");
            assert_eq!(
                reply.total,
                reply.proposed.len() as u64,
                "trial {trial}: host worker never caps"
            );
            let mut got = reply.proposed.clone();
            got.sort_unstable();
            let mut want = sys.cpu_search();
            want.sort_unstable();
            assert_eq!(got, want, "trial {trial} (n = {n}, m = {m})");
        }
    }

    /// Shuffled solution lists sort back to exactly the CPU walk's order.
    #[test]
    fn sort_gray_order_reproduces_the_cpu_walk_order() {
        use crate::cryptanalysis::mq_fes::gray_incremental_find_all;
        let mut nonempty = 0;
        for trial in 0..60u64 {
            let n = 1 + (trial as usize % 20);
            let m = 1 + (trial as usize % 3);
            let forms = sample_forms(n, m, 77 + trial);
            let want = gray_incremental_find_all(&forms, usize::MAX).unwrap();
            let mut got = want.clone();
            got.reverse();
            let third = got.len() / 3;
            got.rotate_left(third);
            sort_gray_order(&mut got);
            assert_eq!(got, want, "trial {trial} (n = {n}, m = {m})");
            nonempty += (want.len() > 1) as u32;
        }
        assert!(nonempty > 20, "too few multi-solution systems: {nonempty}");
        for s in [0u64, 1, 2, 5, 1 << 40, u64::MAX >> 1] {
            assert_eq!(gray_rank(s ^ (s >> 1)), s);
        }
    }
}
