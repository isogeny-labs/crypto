//! Running one command (or k pinned copies of it) and reading its operation
//! count, its timing and, with a PMU, its perf counters.
use crate::host::{conditions_begin, conditions_end};
use crate::score::OPS_SCHEMA;
use regex::Regex;
use serde_json::{json, Map, Value};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub struct Ops {
    pub ops: i64,
    pub kernel_seconds: Option<f64>,
    pub bytes_declared: Option<f64>,
    pub op_class: Option<String>,
    pub working_set_bytes: Option<i64>,
    pub source: String,
}

fn int_of(v: &Value) -> Option<i64> {
    v.as_i64().or_else(|| v.as_f64().map(|x| x as i64))
}

/// Operation count of one process: `optlevel.ops/v1` lines on stdout (summed),
/// else `--ops-regex` matches on stdout+stderr (summed), else `--ops`.
pub fn parse_ops(stdout: &str, stderr: &str, args_ops: Option<i64>, ops_regex: Option<&Regex>) -> Option<Ops> {
    let (mut ops, mut ksec, mut declared) = (0i64, 0.0f64, 0.0f64);
    let (mut cls, mut wset, mut found) = (None, None::<i64>, false);
    let (mut have_ksec, mut have_decl) = (true, true);
    for line in stdout.lines() {
        let line = line.trim();
        if !line.starts_with('{') || !line.contains(OPS_SCHEMA) {
            continue;
        }
        let Ok(d) = serde_json::from_str::<Value>(line) else { continue };
        if d.get("schema").and_then(Value::as_str) != Some(OPS_SCHEMA) {
            continue;
        }
        let Some(n) = d.get("ops").and_then(int_of) else { continue };
        found = true;
        ops += n;
        if let Some(c) = d.get("op_class").and_then(Value::as_str) {
            cls = Some(c.to_string());
        }
        if let Some(w) = d.get("working_set_bytes").and_then(int_of) {
            wset = Some(wset.unwrap_or(0).max(w));
        }
        match d.get("kernel_seconds").and_then(Value::as_f64) {
            Some(k) => ksec += k,
            None => have_ksec = false,
        }
        match d.get("bytes_declared").and_then(Value::as_f64) {
            Some(b) => declared += b,
            None => have_decl = false,
        }
    }
    if found {
        return Some(Ops { ops, kernel_seconds: have_ksec.then_some(ksec), bytes_declared: have_decl.then_some(declared),
                          op_class: cls, working_set_bytes: wset, source: "optlevel.ops/v1 line".into() });
    }
    if let Some(re) = ops_regex {
        let text = format!("{stdout}\n{stderr}");
        let vals: Vec<i64> = re.captures_iter(&text)
            .filter_map(|m| m.get(1).and_then(|g| g.as_str().replace([',', '_'], "").parse().ok())).collect();
        if !vals.is_empty() {
            return Some(Ops { ops: vals.iter().sum(), kernel_seconds: None, bytes_declared: None, op_class: None, working_set_bytes: None,
                              source: format!("--ops-regex '{}' ({} matches summed)", re.as_str(), vals.len()) });
        }
    }
    args_ops.map(|n| Ops { ops: n, kernel_seconds: None, bytes_declared: None, op_class: None, working_set_bytes: None, source: "--ops (constant)".into() })
}

/// `perf stat -x,` rows: value,unit,event,run-time,percent-running,...
/// Returns (counters, multiplexed events, events not counted).
pub fn parse_perf_csv(text: &str) -> (Map<String, Value>, Vec<String>, Vec<String>) {
    let (mut counters, mut mux, mut nc) = (Map::new(), Vec::new(), Vec::new());
    for line in text.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split(',').collect();
        if f.len() < 3 {
            continue;
        }
        let ev = f[2].strip_suffix(":u").unwrap_or(f[2]).to_string();
        if f[0].starts_with('<') {
            nc.push(ev);
            continue;
        }
        let Ok(v) = f[0].trim().parse::<f64>() else { continue };
        counters.insert(ev.clone(), json!(v));
        if f.len() > 4 && f[4].trim().parse::<f64>().map(|p| p < 100.0).unwrap_or(false) {
            mux.push(ev);
        }
    }
    (counters, mux, nc)
}

#[derive(Default, Clone)]
pub struct RunSpec<'a> {
    pub cmd: &'a [String],
    pub copies: usize,
    pub cpus: Option<Vec<usize>>,
    pub env: Vec<(String, String)>,
    pub cwd: Option<&'a str>,
    pub perf: Option<&'a str>,
    pub events: Option<&'a str>,
    pub ops: Option<i64>,
    pub ops_regex: Option<&'a Regex>,
    pub timeout: Option<f64>,
}

fn children_cpu_s() -> f64 {
    // SAFETY: rusage is plain data filled by getrusage.
    unsafe {
        let mut ru: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_CHILDREN, &mut ru);
        let tv = |t: libc::timeval| t.tv_sec as f64 + 1e-6 * t.tv_usec as f64;
        tv(ru.ru_utime) + tv(ru.ru_stime)
    }
}

fn scratch_dir() -> Result<PathBuf, String> {
    static SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let d = std::env::temp_dir().join(format!("optlevel-{}-{}", std::process::id(), SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)));
    std::fs::create_dir_all(&d).map_err(|e| format!("{}: {e}", d.display()))?;
    Ok(d)
}

/// Run `copies` concurrent instances of the command, copy i pinned to cpus[i]
/// when cpus is given.  Returns the summed operation count and the rate.
/// Output goes to files, so no pipe can fill and stall a copy.
pub fn run_once(spec: &RunSpec) -> Result<Value, String> {
    let copies = spec.copies.max(1);
    let tmp = scratch_dir()?;
    let begin = conditions_begin();
    let cpu0 = children_cpu_s();
    let t0 = Instant::now();
    let mut procs = Vec::new();
    for i in 0..copies {
        let mut argv: Vec<String> = spec.cmd.to_vec();
        let mut csv = None;
        if let (Some(perf), Some(events)) = (spec.perf, spec.events) {
            let path = tmp.join(format!("perf{i}.csv"));
            let mut pre: Vec<String> = [perf, "stat", "-x,", "--no-big-num", "-e", events, "-o"].iter().map(|s| s.to_string()).collect();
            pre.push(path.to_string_lossy().into_owned());
            pre.push("--".into());
            pre.extend(argv);
            argv = pre;
            csv = Some(path);
        }
        let (out, err) = (tmp.join(format!("out{i}")), tmp.join(format!("err{i}")));
        let mk = |p: &PathBuf| std::fs::File::create(p).map_err(|e| format!("{}: {e}", p.display()));
        let mut c = Command::new(&argv[0]);
        c.args(&argv[1..]).stdin(Stdio::null()).stdout(mk(&out)?).stderr(mk(&err)?);
        for (k, v) in &spec.env {
            c.env(k, v);
        }
        if let Some(d) = spec.cwd {
            c.current_dir(d);
        }
        if let Some(cpu) = spec.cpus.as_ref().map(|c| c[i]) {
            // SAFETY: the closure runs in the forked child before exec and only
            // calls sched_setaffinity, which is async-signal-safe.
            unsafe {
                c.pre_exec(move || {
                    let mut set: libc::cpu_set_t = std::mem::zeroed();
                    libc::CPU_SET(cpu, &mut set);
                    libc::sched_setaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &set);
                    Ok(())
                });
            }
        }
        match c.spawn() {
            Ok(child) => procs.push((child, out, err, csv)),
            Err(e) => {
                for (mut ch, ..) in procs {
                    let _ = ch.kill();
                    let _ = ch.wait();
                }
                let _ = std::fs::remove_dir_all(&tmp);
                return Err(format!("cannot run {}: {e}", argv[0]));
            }
        }
    }
    // a watchdog kills the copies at the deadline; waiting itself never polls
    let (done_tx, done_rx) = std::sync::mpsc::channel::<()>();
    let watchdog = spec.timeout.map(|secs| {
        let pids: Vec<i32> = procs.iter().map(|p| p.0.id() as i32).collect();
        std::thread::spawn(move || {
            if done_rx.recv_timeout(Duration::from_secs_f64(secs.max(0.0))).is_err() {
                for pid in pids {
                    // SAFETY: the children are not reaped until the main thread sends on the channel.
                    unsafe { libc::kill(pid, libc::SIGKILL) };
                }
            }
        })
    });
    let mut outs = Vec::new();
    for (mut child, out, err, csv) in procs {
        let status = child.wait().map_err(|e| format!("wait: {e}"))?;
        let code = status.code().or_else(|| status.signal().map(|s| -s)).unwrap_or(-1);
        outs.push((code, out, err, csv));
    }
    let wall = t0.elapsed().as_secs_f64();
    let _ = done_tx.send(());
    if let Some(w) = watchdog {
        let _ = w.join();
    }
    let own_cpu = children_cpu_s() - cpu0;
    let mut res = Map::new();
    res.insert("copies".into(), json!(copies));
    res.insert("pinned_cpus".into(), json!(spec.cpus));
    res.insert("wall_s".into(), json!(wall));
    res.insert("cpu_s".into(), json!(own_cpu));
    res.insert("exit_status".into(), json!(outs.iter().map(|o| o.0).collect::<Vec<_>>()));
    res.insert("conditions".into(), conditions_end(&begin, own_cpu));
    let (mut rate, mut total_ops, mut declared) = (0.0f64, 0i64, 0.0f64);
    let (mut ksec, mut src, mut cls, mut decl_ok) = (Vec::new(), None, None, true);
    let text = |p: &PathBuf| String::from_utf8_lossy(&std::fs::read(p).unwrap_or_default()).into_owned();
    for (_, out, err, _) in &outs {
        let (so, se) = (text(out), text(err));
        let Some(o) = parse_ops(&so, &se, spec.ops, spec.ops_regex) else {
            res.insert("ops".into(), Value::Null);
            let tail: String = se.chars().rev().take(400).collect::<Vec<_>>().into_iter().rev().collect();
            res.insert("stderr_tail".into(), json!(tail));
            let _ = std::fs::remove_dir_all(&tmp);
            return Ok(Value::Object(res));
        };
        total_ops += o.ops;
        src = Some(o.source);
        cls = o.op_class.or(cls);
        if let Some(w) = o.working_set_bytes {
            res.insert("working_set_bytes".into(), json!(w));
        }
        let secs = o.kernel_seconds.filter(|k| *k != 0.0).unwrap_or(wall);
        ksec.push(o.kernel_seconds);
        rate += o.ops as f64 / secs;
        match o.bytes_declared {
            Some(b) => declared += b,
            None => decl_ok = false,
        }
    }
    res.insert("ops".into(), json!(total_ops));
    res.insert("ops_per_s".into(), json!(rate));
    res.insert("ops_source".into(), json!(src));
    res.insert("op_class".into(), json!(cls));
    let kmax = if ksec.iter().all(Option::is_some) { ksec.iter().flatten().copied().reduce(f64::max) } else { None };
    res.insert("kernel_seconds".into(), json!(kmax));
    res.insert("bytes_declared".into(), if decl_ok { json!(declared) } else { Value::Null });
    if spec.perf.is_some() && spec.events.is_some() {
        let (mut c, mut mux, mut nc) = (Map::new(), Vec::new(), Vec::new());
        for (.., csv) in &outs {
            if let Some(t) = csv.as_ref().and_then(|p| std::fs::read_to_string(p).ok()) {
                let (ci, m, n) = parse_perf_csv(&t);
                for (k, v) in ci {
                    let prev = c.get(&k).and_then(Value::as_f64).unwrap_or(0.0);
                    c.insert(k, json!(prev + v.as_f64().unwrap_or(0.0)));
                }
                mux.extend(m);
                nc.extend(n);
            }
        }
        mux.sort();
        mux.dedup();
        nc.sort();
        nc.dedup();
        res.insert("counters".into(), Value::Object(c));
        res.insert("multiplexed".into(), json!(mux));
        res.insert("not_counted".into(), json!(nc));
    }
    let _ = std::fs::remove_dir_all(&tmp);
    Ok(Value::Object(res))
}
