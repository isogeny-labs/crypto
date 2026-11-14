//! [M4GB](https://github.com/cr-marcstevens/m4gb) (Makarim–Stevens, ISSAC 2017)
//! as an external Gröbner-basis oracle over GF(2).
//!
//! M4GB is a GPL-3 program, so it is never linked: this module writes its text
//! input, runs the `solver_m4gb_n<MAXVARS>_gf2` binary as a separate process
//! (same pattern as [`crate::cryptanalysis::wdsat_oracle`]), and parses the
//! reduced degrevlex basis it prints back into [`F2BoolPoly`]s.  The binary is
//! found through the `M4GB_BINARY` environment variable, which may also name a directory of
//! binaries from which the narrowest one that fits is chosen.
//!
//! The binary adds the field equations `x² + x` itself.  Its output may
//! contain them (`Xi^2 + Xi`, which is zero in the Boolean ring and is
//! dropped) and, for variables above the input's count that it cannot name,
//! lines such as `^2 + `, which are skipped.

use crate::cryptanalysis::ic_framework::stages::BooleanSystem;
use crate::cryptanalysis::pq_groebner_f2::{bit, F2BoolMono, F2BoolPoly, Mask, MASK_BITS};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

/// Write `system` in M4GB's text input format over GF(2).
///
/// Zero polynomials are skipped; terms are square-free (`X0*X8`), the
/// constant term is `1`, variables are `X0 … X{n-1}`.
pub fn write_m4gb_input(system: &BooleanSystem) -> String {
    let mut out = format!("$fieldsize 2\n$vars {} X\n", system.n_vars);
    for poly in &system.equations {
        if poly.is_zero() {
            continue;
        }
        for (k, term) in poly.terms.iter().enumerate() {
            if k > 0 {
                out.push('+');
            }
            if term.mask == 0 {
                out.push('1');
                continue;
            }
            let mut first = true;
            for i in 0..MASK_BITS {
                if (term.mask >> i) & 1 == 1 {
                    if !first {
                        out.push('*');
                    }
                    first = false;
                    write!(out, "X{i}").unwrap();
                }
            }
        }
        out.push('\n');
    }
    out
}

/// Parse M4GB's output (a reduced basis, one polynomial per line) into
/// Boolean polynomials over `n_vars` unknowns.
///
/// Blank lines and lines with an unnamed variable (`^2 + `) are skipped,
/// `Xi^k` (`k ≥ 1`) reads as `Xi`, so field equations parse to zero and are
/// dropped.  A line `1` is kept: the basis is then the unit ideal.  A
/// variable index `≥ n_vars` is an error.
pub fn parse_m4gb_basis(text: &str, n_vars: usize) -> Result<Vec<F2BoolPoly>, String> {
    if n_vars > MASK_BITS {
        return Err(format!(
            "{n_vars} variables exceed the {MASK_BITS}-bit mask"
        ));
    }
    let mut basis = Vec::new();
    'lines: for (lineno, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut monos: Vec<F2BoolMono> = Vec::new();
        for token in line.split('+') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            if token == "1" {
                monos.push(F2BoolMono::one());
                continue;
            }
            let mut mask: Mask = 0;
            for factor in token.split('*') {
                let factor = factor.trim();
                let (name, power) = match factor.split_once('^') {
                    Some((n, p)) => (n.trim(), Some(p.trim())),
                    None => (factor, None),
                };
                if name.is_empty() {
                    // An unnamed variable above the input's count.
                    continue 'lines;
                }
                if let Some(p) = power {
                    match p.parse::<u32>() {
                        Ok(k) if k >= 1 => {}
                        _ => {
                            return Err(format!("line {}: bad exponent in `{factor}`", lineno + 1))
                        }
                    }
                }
                let idx = name
                    .strip_prefix('X')
                    .and_then(|s| s.parse::<usize>().ok())
                    .ok_or_else(|| format!("line {}: bad variable `{name}`", lineno + 1))?;
                if idx >= n_vars {
                    return Err(format!(
                        "line {}: variable X{idx} out of range (n_vars = {n_vars})",
                        lineno + 1
                    ));
                }
                mask |= bit(idx as u32);
            }
            monos.push(F2BoolMono::from_mask(mask));
        }
        let poly = F2BoolPoly::from_monos(monos, n_vars);
        if !poly.is_zero() {
            basis.push(poly);
        }
    }
    Ok(basis)
}

/// How to run the M4GB binary.
#[derive(Clone, Debug)]
pub struct M4gbOptions {
    pub binary: PathBuf,
    /// Passed as `--nrthreads`.
    pub threads: usize,
    pub timeout: Duration,
    /// Directory for the input and output files (default: the temp dir).
    pub work_dir: Option<PathBuf>,
    /// Keep the input and output files after the run.
    pub keep_files: bool,
}

impl M4gbOptions {
    /// `M4GB_BINARY` (required), `M4GB_THREADS` (default 1), `M4GB_WORK_DIR`.
    pub fn from_env() -> Option<Self> {
        let binary = std::env::var_os("M4GB_BINARY").filter(|v| !v.is_empty())?;
        let threads = std::env::var("M4GB_THREADS")
            .ok()
            .and_then(|v| v.trim().parse::<usize>().ok())
            .filter(|&t| t >= 1)
            .unwrap_or(1);
        let work_dir = std::env::var_os("M4GB_WORK_DIR")
            .filter(|v| !v.is_empty())
            .map(PathBuf::from);
        Some(Self {
            binary: PathBuf::from(binary),
            threads,
            timeout: Duration::from_secs(3600),
            work_dir,
            keep_files: false,
        })
    }
}

/// The `MAXVARS` of a `solver_m4gb_n<MAXVARS>_gf2` binary, from its file name.
pub fn max_vars_of_binary(path: &Path) -> Option<usize> {
    let name = path.file_name()?.to_str()?;
    let mut search = 0;
    while let Some(pos) = name[search..].find("_n") {
        let start = search + pos + 2;
        let digits: String = name[start..]
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        // `_gf2` must end the name: `_gf256` and `_gf2…` fields are not GF(2).
        if !digits.is_empty() && name[start + digits.len()..] == *"_gf2" {
            return digits.parse().ok();
        }
        search = start;
    }
    None
}

/// Pick the binary for a system of `n_vars` unknowns.
///
/// `path` is either one binary (used if its `_n<N>_gf2` name, when it has
/// one, allows `n_vars`) or a directory of `solver_m4gb_n<N>_gf2` binaries,
/// of which the one with the smallest `N >= n_vars` is chosen: a wide
/// binary needs far more memory than a narrow one on the same system.
pub fn select_binary(path: &Path, n_vars: usize) -> Option<PathBuf> {
    if path.is_dir() {
        let mut best: Option<(usize, PathBuf)> = None;
        for entry in fs::read_dir(path).ok()?.flatten() {
            let file = entry.path();
            if !file.is_file() {
                continue;
            }
            let is_solver = file
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("solver_m4gb_"));
            let Some(max) = max_vars_of_binary(&file).filter(|_| is_solver) else {
                continue;
            };
            if max >= n_vars && best.as_ref().is_none_or(|(m, _)| max < *m) {
                best = Some((max, file));
            }
        }
        best.map(|(_, file)| file)
    } else if max_vars_of_binary(path).is_none_or(|max| n_vars <= max) {
        Some(path.to_path_buf())
    } else {
        None
    }
}

/// One successful run.
#[derive(Clone, Debug)]
pub struct M4gbRun {
    pub basis: Vec<F2BoolPoly>,
    pub wall: Duration,
    pub cpu_seconds: Option<f64>,
    pub peak_mib: Option<u64>,
    /// Largest `lcmdegree=` seen in the log.
    pub max_lcm_degree: Option<u32>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone, Debug)]
pub enum M4gbError {
    Timeout(Duration),
    Spawn(String),
    Exit(String),
    Parse(String),
}

impl std::fmt::Display for M4gbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            M4gbError::Timeout(d) => write!(f, "M4GB timed out after {} ms", d.as_millis()),
            M4gbError::Spawn(e) => write!(f, "M4GB spawn failed: {e}"),
            M4gbError::Exit(e) => write!(f, "M4GB failed: {e}"),
            M4gbError::Parse(e) => write!(f, "M4GB output parse failed: {e}"),
        }
    }
}

impl std::error::Error for M4gbError {}

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn drain<R: Read + Send + 'static>(mut r: R) -> std::thread::JoinHandle<String> {
    std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = r.read_to_end(&mut buf);
        String::from_utf8_lossy(&buf).into_owned()
    })
}

/// Value after `label` and a colon on the first line that has it.
fn log_value<'a>(log: &'a str, label: &str) -> Option<&'a str> {
    log.lines().find_map(|l| {
        let i = l.find(label)?;
        let rest = l[i + label.len()..].trim_start();
        rest.strip_prefix(':').map(str::trim)
    })
}

fn parse_log(stdout: &str, stderr: &str) -> (Option<f64>, Option<u64>, Option<u32>) {
    let both = format!("{stdout}\n{stderr}");
    let cpu = log_value(&both, "Total CPU time")
        .and_then(|v| v.split_whitespace().next())
        .and_then(|v| v.parse::<f64>().ok());
    let peak = log_value(&both, "Peak memory")
        .and_then(|v| v.split_whitespace().next())
        .and_then(|v| v.parse::<u64>().ok());
    let mut degree: Option<u32> = None;
    for line in both.lines() {
        if let Some(i) = line.find("lcmdegree=") {
            let digits: String = line[i + "lcmdegree=".len()..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if let Ok(d) = digits.parse::<u32>() {
                degree = Some(degree.map_or(d, |m| m.max(d)));
            }
        }
    }
    (cpu, peak, degree)
}

/// Whether stderr reports the unit ideal: M4GB's exception text starts
/// `contradiction:` (`I=(1)` over GF(2), `1=0` over larger fields).
fn is_contradiction(stderr: &str) -> bool {
    stderr.contains("contradiction:")
}

/// Run M4GB on `system`.  The deadline is the smaller of `options.timeout`
/// and `budget`.
pub fn run_m4gb(
    system: &BooleanSystem,
    options: &M4gbOptions,
    budget: Option<Duration>,
) -> Result<M4gbRun, M4gbError> {
    let limit = budget.map_or(options.timeout, |b| b.min(options.timeout));
    let dir = options.work_dir.clone().unwrap_or_else(std::env::temp_dir);
    fs::create_dir_all(&dir).map_err(|e| M4gbError::Spawn(format!("create work dir: {e}")))?;
    let id = format!(
        "m4gb-ic-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    );
    let input = dir.join(format!("{id}.in"));
    let output = dir.join(format!("{id}.out"));
    fs::write(&input, write_m4gb_input(system))
        .map_err(|e| M4gbError::Spawn(format!("write {}: {e}", input.display())))?;
    let cleanup = |keep: bool| {
        if !keep {
            let _ = fs::remove_file(&input);
            let _ = fs::remove_file(&output);
        }
    };

    let Some(binary) = select_binary(&options.binary, system.n_vars) else {
        cleanup(options.keep_files);
        return Err(M4gbError::Spawn(format!(
            "no M4GB binary under {} takes {} variables",
            options.binary.display(),
            system.n_vars
        )));
    };
    let started = Instant::now();
    let mut child = match Command::new(&binary)
        .arg("--solve")
        .arg("-i")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .arg("--loglevel")
        .arg("3")
        .arg("--nrthreads")
        .arg(options.threads.max(1).to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            cleanup(options.keep_files);
            return Err(M4gbError::Spawn(format!("{}: {e}", binary.display())));
        }
    };
    let out_h = child.stdout.take().map(drain);
    let err_h = child.stderr.take().map(drain);

    let deadline = started + limit;
    let status = loop {
        match child.try_wait() {
            Ok(Some(s)) => break s,
            Ok(None) if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                cleanup(options.keep_files);
                return Err(M4gbError::Timeout(limit));
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(2)),
            Err(e) => {
                let _ = child.kill();
                cleanup(options.keep_files);
                return Err(M4gbError::Exit(format!("wait: {e}")));
            }
        }
    };
    let wall = started.elapsed();
    let stdout = out_h.and_then(|h| h.join().ok()).unwrap_or_default();
    let stderr = err_h.and_then(|h| h.join().ok()).unwrap_or_default();
    let (cpu_seconds, peak_mib, max_lcm_degree) = parse_log(&stdout, &stderr);
    if !status.success() {
        cleanup(options.keep_files);
        if is_contradiction(&stderr) {
            // M4GB exits 1 without an output file when 1 is in the ideal:
            // a decided result, reported as the unit-ideal basis.
            return Ok(M4gbRun {
                basis: vec![F2BoolPoly::one(system.n_vars)],
                wall,
                cpu_seconds,
                peak_mib,
                max_lcm_degree,
                stdout,
                stderr,
            });
        }
        return Err(M4gbError::Exit(format!("exit status {status}")));
    }
    let text = fs::read_to_string(&output)
        .map_err(|e| M4gbError::Exit(format!("no output file {}: {e}", output.display())));
    cleanup(options.keep_files);
    let basis = parse_m4gb_basis(&text?, system.n_vars).map_err(M4gbError::Parse)?;
    Ok(M4gbRun {
        basis,
        wall,
        cpu_seconds,
        peak_mib,
        max_lcm_degree,
        stdout,
        stderr,
    })
}

/// A long-lived `solver --serve` process.
struct Server {
    child: Child,
    stdin: ChildStdin,
    lines: mpsc::Receiver<String>,
}

static SERVERS: OnceLock<Mutex<HashMap<PathBuf, Server>>> = OnceLock::new();

fn servers() -> std::sync::MutexGuard<'static, HashMap<PathBuf, Server>> {
    SERVERS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn spawn_server(binary: &Path, threads: usize, limit: Duration) -> Result<Server, M4gbError> {
    let mut child = Command::new(binary)
        .arg("--serve")
        .arg("--loglevel")
        .arg("1")
        .arg("--nrthreads")
        .arg(threads.max(1).to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| M4gbError::Spawn(format!("{}: {e}", binary.display())))?;
    let (Some(stdin), Some(stdout), Some(stderr)) =
        (child.stdin.take(), child.stdout.take(), child.stderr.take())
    else {
        let _ = child.kill();
        let _ = child.wait();
        return Err(M4gbError::Spawn("server pipes unavailable".into()));
    };
    // Drain stderr so the server can never block on it.
    std::thread::spawn(move || {
        let mut r = stderr;
        let mut buf = [0u8; 4096];
        while matches!(r.read(&mut buf), Ok(n) if n > 0) {}
    });
    let (tx, lines) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    let deadline = Instant::now() + limit;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        match lines.recv_timeout(remaining) {
            Ok(l) if l.trim() == "READY" => break,
            Ok(_) => {}
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(M4gbError::Spawn(format!(
                    "{}: no READY from --serve within {} ms",
                    binary.display(),
                    limit.as_millis()
                )));
            }
        }
    }
    Ok(Server {
        child,
        stdin,
        lines,
    })
}

/// Like [`run_m4gb`], but the request goes to a long-lived `--serve` process
/// (one per selected binary, shared by the whole process), so process start,
/// field tables and the thread pool are paid once.  Requests to one server
/// are serialised.  `cpu_seconds`, `peak_mib` and `max_lcm_degree` are `None`.
pub fn run_m4gb_persistent(
    system: &BooleanSystem,
    options: &M4gbOptions,
    budget: Option<Duration>,
) -> Result<M4gbRun, M4gbError> {
    let limit = budget.map_or(options.timeout, |b| b.min(options.timeout));
    let dir = options.work_dir.clone().unwrap_or_else(std::env::temp_dir);
    fs::create_dir_all(&dir).map_err(|e| M4gbError::Spawn(format!("create work dir: {e}")))?;
    let id = format!(
        "m4gb-ic-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    );
    let input = dir.join(format!("{id}.in"));
    let output = dir.join(format!("{id}.out"));
    fs::write(&input, write_m4gb_input(system))
        .map_err(|e| M4gbError::Spawn(format!("write {}: {e}", input.display())))?;
    let cleanup = |keep: bool| {
        if !keep {
            let _ = fs::remove_file(&input);
            let _ = fs::remove_file(&output);
        }
    };
    let Some(binary) = select_binary(&options.binary, system.n_vars) else {
        cleanup(options.keep_files);
        return Err(M4gbError::Spawn(format!(
            "no M4GB binary under {} takes {} variables",
            options.binary.display(),
            system.n_vars
        )));
    };

    let mut pool = servers();
    if !pool.contains_key(&binary) {
        match spawn_server(&binary, options.threads, limit) {
            Ok(server) => {
                pool.insert(binary.clone(), server);
            }
            Err(e) => {
                cleanup(options.keep_files);
                return Err(e);
            }
        }
    }
    let server = pool.get_mut(&binary).expect("server just inserted");

    let started = Instant::now();
    let request = format!("SOLVE {} {}\n", input.display(), output.display());
    if let Err(e) = server
        .stdin
        .write_all(request.as_bytes())
        .and_then(|_| server.stdin.flush())
    {
        if let Some(mut dead) = pool.remove(&binary) {
            let _ = dead.child.kill();
            let _ = dead.child.wait();
        }
        cleanup(options.keep_files);
        return Err(M4gbError::Exit(format!("m4gb server write failed: {e}")));
    }
    let deadline = started + limit;
    let reply = loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        match server.lines.recv_timeout(remaining) {
            Ok(line) => {
                let line = line.trim_end();
                if line == "OK" || line == "UNSAT" || line.starts_with("ERR ") || line == "ERR" {
                    break line.to_string();
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if let Some(mut dead) = pool.remove(&binary) {
                    let _ = dead.child.kill();
                    let _ = dead.child.wait();
                }
                cleanup(options.keep_files);
                return Err(M4gbError::Timeout(limit));
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                if let Some(mut dead) = pool.remove(&binary) {
                    let _ = dead.child.kill();
                    let _ = dead.child.wait();
                }
                cleanup(options.keep_files);
                return Err(M4gbError::Exit("m4gb server exited".into()));
            }
        }
    };
    let wall = started.elapsed();
    drop(pool);
    let run = |basis| M4gbRun {
        basis,
        wall,
        cpu_seconds: None,
        peak_mib: None,
        max_lcm_degree: None,
        stdout: String::new(),
        stderr: String::new(),
    };
    if reply == "UNSAT" {
        cleanup(options.keep_files);
        return Ok(run(vec![F2BoolPoly::one(system.n_vars)]));
    }
    if reply.starts_with("ERR") {
        cleanup(options.keep_files);
        return Err(M4gbError::Exit(
            reply.strip_prefix("ERR").unwrap_or("").trim().to_string(),
        ));
    }
    let text = fs::read_to_string(&output)
        .map_err(|e| M4gbError::Exit(format!("no output file {}: {e}", output.display())));
    cleanup(options.keep_files);
    let basis = parse_m4gb_basis(&text?, system.n_vars).map_err(M4gbError::Parse)?;
    Ok(run(basis))
}

/// Ask every pooled server to `QUIT`, wait for it, and clear the pool.
pub fn shutdown_m4gb_servers() {
    shutdown_where(|_| true);
}

fn shutdown_where(pick: impl Fn(&Path) -> bool) {
    let drained: Vec<Server> = {
        let mut pool = servers();
        let keys: Vec<PathBuf> = pool.keys().filter(|k| pick(k)).cloned().collect();
        keys.into_iter().filter_map(|k| pool.remove(&k)).collect()
    };
    for mut s in drained {
        let _ = s.stdin.write_all(b"QUIT\n");
        let _ = s.stdin.flush();
        drop(s.stdin);
        let _ = s.child.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "X14\nX9 + 1\nX8 + X10 + 1\nX7 + X15\nX2 + X10 + 1\n^2 + \n^2 + \nX15^2 + X15\nX10^2 + X10\n";

    fn m(mask: Mask) -> F2BoolMono {
        F2BoolMono::from_mask(mask)
    }

    #[test]
    fn write_then_parse_round_trips() {
        let sys = BooleanSystem {
            equations: vec![
                F2BoolPoly::from_monos(vec![m(0b101), m(0b10), m(0)], 3),
                F2BoolPoly::zero(3),
                F2BoolPoly::from_monos(vec![m(0b110), m(0b1)], 3),
                F2BoolPoly::one(3),
            ],
            n_vars: 3,
        };
        let text = write_m4gb_input(&sys);
        assert!(text.starts_with("$fieldsize 2\n$vars 3 X\n"));
        assert!(text.contains("X0*X2+X1+1\n"));
        assert!(!text.contains('^'));
        // Skip the two header lines; the body is a valid basis-syntax text.
        let body: String = text.lines().skip(2).collect::<Vec<_>>().join("\n");
        let parsed = parse_m4gb_basis(&body, 3).unwrap();
        let expected: Vec<F2BoolPoly> = sys
            .equations
            .iter()
            .filter(|p| !p.is_zero())
            .cloned()
            .collect();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn sample_output_drops_quirk_lines() {
        let basis = parse_m4gb_basis(SAMPLE, 16).unwrap();
        // Nine lines: two unnamed-variable lines and two field equations
        // (zero in the Boolean ring) are dropped.
        assert_eq!(basis.len(), 5);
        assert!(basis.iter().all(|p| !p.is_zero()));
        assert_eq!(basis[0], F2BoolPoly::from_monos(vec![m(1 << 14)], 16));
        assert_eq!(
            basis[2],
            F2BoolPoly::from_monos(vec![m(1 << 8), m(1 << 10), m(0)], 16)
        );
    }

    #[test]
    fn unit_ideal_and_range_errors() {
        let basis = parse_m4gb_basis("1\n", 4).unwrap();
        assert_eq!(basis, vec![F2BoolPoly::one(4)]);
        assert!(parse_m4gb_basis("X4 + 1\n", 4).is_err());
        assert!(parse_m4gb_basis("Y1\n", 4).is_err());
    }

    #[test]
    fn binary_name_gives_max_vars() {
        assert_eq!(
            max_vars_of_binary(Path::new("/x/y/solver_m4gb_n128_gf2")),
            Some(128)
        );
        assert_eq!(max_vars_of_binary(Path::new("solver_m4gb_n20_gf31")), None);
        assert_eq!(max_vars_of_binary(Path::new("m4gb")), None);
        assert_eq!(max_vars_of_binary(Path::new("solver_m4gb_n16_gf256")), None);
    }

    #[test]
    fn contradiction_is_detected() {
        assert!(is_contradiction(
            "Caught exception:\ncontradiction: I=(1)\n"
        ));
        assert!(is_contradiction("Caught exception:\ncontradiction: 1=0\n"));
        assert!(!is_contradiction("Caught exception:\nout of memory\n"));
    }

    #[test]
    fn select_binary_prefers_the_narrowest_fit() {
        let dir = std::env::temp_dir().join(format!("m4gb-select-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        for n in [
            "solver_m4gb_n128_gf2",
            "solver_m4gb_n40_gf2",
            "solver_m4gb_n64_gf2",
            "solver_m4gb_n20_gf31",
            "notes.txt",
        ] {
            fs::write(dir.join(n), b"").unwrap();
        }
        let name = |n: usize| {
            select_binary(&dir, n).map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
        };
        assert_eq!(name(5).as_deref(), Some("solver_m4gb_n40_gf2"));
        assert_eq!(name(40).as_deref(), Some("solver_m4gb_n40_gf2"));
        assert_eq!(name(41).as_deref(), Some("solver_m4gb_n64_gf2"));
        assert_eq!(name(100).as_deref(), Some("solver_m4gb_n128_gf2"));
        assert_eq!(name(129), None);
        // A single file is used as is while it is wide enough.
        let one = dir.join("solver_m4gb_n40_gf2");
        assert_eq!(select_binary(&one, 40), Some(one.clone()));
        assert_eq!(select_binary(&one, 41), None);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn log_lines_are_read() {
        let log = "[m4gb]  P lcmdegree=3\n[m4gb]  P lcmdegree=5\n[m4gb]  P lcmdegree=4\nTotal CPU time : 0.98 s\nPeak memory    : 188 MiB\n";
        let (cpu, peak, deg) = parse_log(log, "");
        assert_eq!(cpu, Some(0.98));
        assert_eq!(peak, Some(188));
        assert_eq!(deg, Some(5));
    }

    #[cfg(unix)]
    mod persistent {
        use super::*;
        use std::os::unix::fs::PermissionsExt;

        fn stub(tag: &str, body: impl Fn(&Path) -> String) -> (PathBuf, M4gbOptions) {
            let dir = std::env::temp_dir().join(format!("m4gb-stub-{}-{tag}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            let bin = dir.join("solver_m4gb_n16_gf2");
            fs::write(dir.join("sample.txt"), SAMPLE).unwrap();
            fs::write(&bin, format!("#!/bin/sh\n{}", body(&dir))).unwrap();
            fs::set_permissions(&bin, fs::Permissions::from_mode(0o755)).unwrap();
            let options = M4gbOptions {
                binary: bin.clone(),
                threads: 1,
                timeout: Duration::from_secs(20),
                work_dir: Some(dir.clone()),
                keep_files: false,
            };
            (dir, options)
        }

        fn tiny() -> BooleanSystem {
            BooleanSystem {
                equations: vec![F2BoolPoly::from_monos(vec![m(0b1), m(0)], 3)],
                n_vars: 3,
            }
        }

        fn sample_stub(dir: &Path) -> String {
            format!(
                "echo READY\nwhile read cmd in out; do\n  case \"$cmd\" in\n    SOLVE) echo $$ >> {d}/count\n      case \"$in\" in\n        *unsat*) echo UNSAT ;;\n        *) cp {d}/sample.txt \"$out\"; echo noise; echo OK ;;\n      esac ;;\n    QUIT) exit 0 ;;\n  esac\ndone\n",
                d = dir.display()
            )
        }

        #[test]
        fn persistent_reuses_one_server() {
            let (dir, options) = stub("reuse", sample_stub);
            let expected = parse_m4gb_basis(SAMPLE, 16).unwrap();
            let mut sys = tiny();
            sys.n_vars = 16;
            for _ in 0..2 {
                let run = run_m4gb_persistent(&sys, &options, None).unwrap();
                assert_eq!(run.basis, expected);
                assert!(run.cpu_seconds.is_none() && run.peak_mib.is_none());
            }
            assert!(servers().contains_key(&options.binary));
            let count = fs::read_to_string(dir.join("count")).unwrap();
            let pids: Vec<&str> = count.lines().collect();
            assert_eq!(pids.len(), 2);
            assert_eq!(pids[0], pids[1], "one server process served both requests");
            shutdown_where(|k| k == options.binary);
            assert!(!servers().contains_key(&options.binary));
            let _ = fs::remove_dir_all(&dir);
        }

        #[test]
        fn persistent_unsat_is_the_unit_ideal() {
            let (dir, options) = stub("unsat", sample_stub);
            let run = run_m4gb_persistent(&tiny(), &options, None).unwrap();
            assert_eq!(run.basis, vec![F2BoolPoly::one(3)]);
            shutdown_where(|k| k == options.binary);
            let _ = fs::remove_dir_all(&dir);
        }

        #[test]
        fn persistent_timeout_removes_the_server() {
            let (dir, options) = stub("timeout", |_| {
                "echo READY\nread cmd in out\nexec sleep 30\n".to_string()
            });
            let err = run_m4gb_persistent(&tiny(), &options, Some(Duration::from_millis(200)))
                .unwrap_err();
            assert!(matches!(err, M4gbError::Timeout(_)), "{err:?}");
            assert!(!servers().contains_key(&options.binary));
            shutdown_where(|k| k == options.binary);
            let _ = fs::remove_dir_all(&dir);
        }
    }
}
