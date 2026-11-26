//! Bounded, two-phase native process protocol; every observed status is scoped.
use crate::{inputs, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    io::{Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};
use torus_spectrum_compiler::ir::Program;
const BACKEND_SHA: &str = "754bc9abb49bcd2ae095bf07a9ba2f68759bf0f232e71f601ac642f802a59fc8";
pub struct Limits {
    pub max_nodes: usize,
    pub backend_ms: u64,
    pub query_bytes: usize,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_nodes: 200_000,
            backend_ms: 5000,
            query_bytes: 2 << 20,
            stdout_bytes: 1 << 20,
            stderr_bytes: 64 << 10,
        }
    }
}
impl Limits {
    pub fn validate(&self) -> Result<()> {
        if !(16..=200_000).contains(&self.max_nodes)
            || !(1..=5000).contains(&self.backend_ms)
            || !(256..=2 << 20).contains(&self.query_bytes)
            || !(64..=1 << 20).contains(&self.stdout_bytes)
            || !(64..=64 << 10).contains(&self.stderr_bytes)
        {
            return Err("backend limit admission".into());
        }
        Ok(())
    }
}
struct Limited {
    data: Vec<u8>,
    cap: usize,
}
impl Write for Limited {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        if b.len() > self.cap.saturating_sub(self.data.len()) {
            return Err(std::io::Error::other("query byte cap"));
        }
        self.data.extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
pub struct Run {
    pub status: Option<String>,
    pub model: String,
    pub stdout: String,
    pub stderr: String,
    pub refusal: Option<String>,
    pub exit_code: Option<i32>,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
    pub query_sha256: String,
    pub query: String,
    pub get_value_request: Option<String>,
    pub get_value_sent: bool,
}
impl Run {
    pub fn json(&self) -> Value {
        json!({"status":self.status,"stdout":self.stdout,"stderr":self.stderr,"refusal":self.refusal,"exit_code":self.exit_code,"stdout_truncated":self.stdout_truncated,"stderr_truncated":self.stderr_truncated,"query_sha256":self.query_sha256,"query":self.query,"get_value_request":self.get_value_request,"get_value_sent":self.get_value_sent,"backend_sha256":BACKEND_SHA,"scope":"bounded native transcript, no performance or absence certificate"})
    }
}
fn kill(child: &Arc<Mutex<Child>>) {
    if let Ok(mut c) = child.lock() {
        let _ = c.kill();
    }
}
fn capture(
    mut source: impl Read,
    cap: usize,
    child: Arc<Mutex<Child>>,
    first: Option<mpsc::Sender<String>>,
) -> (Vec<u8>, bool) {
    let mut data = Vec::new();
    let mut first = first;
    let mut chunk = [0u8; 4096];
    loop {
        match source.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => {
                let keep = n.min(cap.saturating_sub(data.len()));
                data.extend_from_slice(&chunk[..keep]);
                if let Some(tx) = first.as_ref() {
                    if let Some(end) = data.iter().position(|x| *x == b'\n') {
                        let _ = tx.send(String::from_utf8_lossy(&data[..end]).to_string());
                        first = None;
                    }
                }
                if keep < n {
                    kill(&child);
                    return (data, true);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => {
                kill(&child);
                return (data, true);
            }
        }
    }
    (data, false)
}
#[cfg(target_os = "linux")]
fn resource_cap(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    // setrlimit is async-signal-safe; no allocator or locking in the child hook.
    unsafe {
        command.pre_exec(|| {
            let r = libc::rlimit {
                rlim_cur: 2 << 30,
                rlim_max: 2 << 30,
            };
            if libc::setrlimit(libc::RLIMIT_AS, &r) != 0 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }
}
pub fn run(exe: &Path, p: &Program, limits: &Limits) -> Result<Run> {
    limits.validate()?;
    p.validate()?;
    if p.gates.len() > limits.max_nodes {
        return Err("program node admission".into());
    }
    if !cfg!(target_os = "linux") {
        return Err("backend address-space cap unsupported on this host".into());
    }
    let mut binary = std::fs::File::open(exe).map_err(|e| e.to_string())?;
    let mut h = Sha256::new();
    let mut block = [0u8; 65536];
    loop {
        let n = binary.read(&mut block).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        h.update(&block[..n]);
    }
    if format!("{:x}", h.finalize()) != BACKEND_SHA {
        return Err("backend executable identity mismatch".into());
    }
    let mut query = Limited {
        data: vec![],
        cap: limits.query_bytes,
    };
    p.smt(&mut query)?;
    let expected = inputs(p)?;
    if expected.is_empty() {
        return Err("backend adapter requires semantic inputs".into());
    }
    let get_value = format!(
        "(get-value ({}))\n(exit)\n",
        expected
            .iter()
            .map(|(w, _)| format!("w{w}"))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let query_sha256 = format!("{:x}", Sha256::digest(&query.data));
    let mut command = Command::new(exe);
    command
        .args([
            "--lang=smt2",
            "--ff-solver=split",
            "--ff-bitsum",
            &format!("--tlimit={}", limits.backend_ms),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(target_os = "linux")]
    resource_cap(&mut command);
    let mut child = command.spawn().map_err(|e| e.to_string())?;
    let mut stdin = child.stdin.take().ok_or("missing backend stdin")?;
    let stdout = child.stdout.take().ok_or("missing backend stdout")?;
    let stderr = child.stderr.take().ok_or("missing backend stderr")?;
    let child = Arc::new(Mutex::new(child));
    let (tx, rx) = mpsc::channel();
    let c = child.clone();
    let cap = limits.stdout_bytes;
    let stdout_thread = thread::spawn(move || capture(stdout, cap, c, Some(tx)));
    let c = child.clone();
    let cap = limits.stderr_bytes;
    let stderr_thread = thread::spawn(move || capture(stderr, cap, c, None));
    let stop = Arc::new(AtomicBool::new(false));
    let expired = Arc::new(AtomicBool::new(false));
    let (s, t, c) = (stop.clone(), expired.clone(), child.clone());
    let deadline = Duration::from_millis(limits.backend_ms + 1000);
    let watchdog = thread::spawn(move || {
        let start = Instant::now();
        while !s.load(Ordering::SeqCst) {
            if start.elapsed() >= deadline {
                t.store(true, Ordering::SeqCst);
                kill(&c);
                break;
            }
            thread::sleep(Duration::from_millis(5));
        }
    });
    let mut refusal = stdin
        .write_all(&query.data)
        .and_then(|_| stdin.flush())
        .err()
        .map(|e| format!("query transport: {e}"));
    let first = if refusal.is_none() {
        rx.recv_timeout(deadline).ok()
    } else {
        None
    };
    let status = first
        .as_deref()
        .filter(|s| matches!(*s, "sat" | "unsat" | "unknown"))
        .map(str::to_owned);
    let mut sent = false;
    if refusal.is_none() {
        let follow = if status.as_deref() == Some("sat") {
            sent = true;
            get_value.as_str()
        } else {
            "(exit)\n"
        };
        if let Err(e) = stdin
            .write_all(follow.as_bytes())
            .and_then(|_| stdin.flush())
        {
            refusal = Some(format!("model transport: {e}"));
        }
    }
    drop(stdin);
    let exit = loop {
        let state = child
            .lock()
            .map_err(|_| "child mutex poisoned")?
            .try_wait()
            .map_err(|e| e.to_string())?;
        if let Some(s) = state {
            break s;
        }
        thread::sleep(Duration::from_millis(5));
    };
    stop.store(true, Ordering::SeqCst);
    watchdog.join().map_err(|_| "watchdog thread failed")?;
    let (out, outcap) = stdout_thread.join().map_err(|_| "stdout reader failed")?;
    let (err, errcap) = stderr_thread.join().map_err(|_| "stderr reader failed")?;
    let stdout = String::from_utf8(out).map_err(|_| "non-UTF8 backend stdout")?;
    let stderr = String::from_utf8(err).map_err(|_| "non-UTF8 backend stderr")?;
    if expired.load(Ordering::SeqCst) {
        refusal = Some("parent watchdog limit".into());
    }
    if outcap || errcap {
        refusal = Some("backend output cap".into());
    }
    if !exit.success() && refusal.is_none() {
        refusal = Some(format!("backend exit {exit}"));
    }
    if status.is_none() && refusal.is_none() {
        refusal = Some("missing or unsupported backend status".into());
    }
    let model = stdout
        .split_once('\n')
        .map(|(_, rest)| rest.to_owned())
        .unwrap_or_default();
    Ok(Run {
        status,
        model,
        stdout,
        stderr,
        refusal,
        exit_code: exit.code(),
        stdout_truncated: outcap,
        stderr_truncated: errcap,
        query_sha256,
        query: String::from_utf8(query.data).map_err(|_| "non-UTF8 query")?,
        get_value_request: sent.then_some(get_value),
        get_value_sent: sent,
    })
}
