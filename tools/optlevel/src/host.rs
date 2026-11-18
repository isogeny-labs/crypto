//! What this host is and what it can measure: CPU identity, the CPUs this
//! process may really use (affinity mask and cgroup CPU quota), caches, PMU
//! availability, and the load conditions around a run.
use crate::util::{py_dumps, read_trim, sha256_hex, which};
use regex::Regex;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

pub const ISA_FLAGS: [&str; 20] = [
    "sse4_2",
    "avx",
    "avx2",
    "fma",
    "bmi2",
    "adx",
    "popcnt",
    "pclmulqdq",
    "avx512f",
    "avx512dq",
    "avx512bw",
    "avx512vl",
    "avx512ifma",
    "avx512_vpopcntdq",
    "vpclmulqdq",
    "gfni",
    "neon",
    "asimd",
    "sve",
    "pmull",
];
/// Helper programs `tools/profile.sh` needs for its GPU subcommands.
pub const HELPER_TOOLS: [&str; 2] = ["gawk", "jq"];

/// CPUs in this process's affinity mask, ascending.
pub fn affinity() -> Vec<usize> {
    // SAFETY: cpu_set_t is plain data; sched_getaffinity writes at most its size.
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        if libc::sched_getaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &mut set) != 0 {
            return (0..online_cpus()).collect();
        }
        (0..libc::CPU_SETSIZE as usize)
            .filter(|&i| libc::CPU_ISSET(i, &set))
            .collect()
    }
}
pub fn online_cpus() -> usize {
    // SAFETY: sysconf has no preconditions.
    (unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) }).max(1) as usize
}

/// The CPU quota of a cgroup, in cores, with where it was read.
#[derive(Debug, Clone, PartialEq)]
pub struct Quota {
    pub cores: f64,
    pub source: String,
}

fn parse_v2(text: &str) -> Option<f64> {
    let mut it = text.split_whitespace();
    let (q, p) = (it.next()?, it.next().unwrap_or("100000"));
    if q == "max" {
        return None;
    }
    let (q, p): (f64, f64) = (q.parse().ok()?, p.parse().ok()?);
    (q > 0.0 && p > 0.0).then(|| q / p)
}

/// The tightest CPU quota that applies to this process: cgroup v2 `cpu.max`
/// and cgroup v1 `cpu.cfs_quota_us / cpu.cfs_period_us`, at the process's own
/// cgroup and every ancestor visible under the mount.  `root` is "/" on a real
/// system (tests pass a fixture tree).
pub fn cgroup_cpu_quota(root: &Path) -> Option<Quota> {
    let membership = std::fs::read_to_string(root.join("proc/self/cgroup")).unwrap_or_default();
    let base = root.join("sys/fs/cgroup");
    let mut best: Option<Quota> = None;
    let mut consider = |cores: Option<f64>, source: String| {
        if let Some(c) = cores {
            if best.as_ref().map(|b| c < b.cores).unwrap_or(true) {
                best = Some(Quota { cores: c, source });
            }
        }
    };
    // a cgroup path and its ancestors, deepest first, ending at the mount root
    let chain = |mount: &Path, rel: &str| -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        let mut cur = mount.join(rel.trim_start_matches('/'));
        loop {
            dirs.push(cur.clone());
            if cur == mount || !cur.starts_with(mount) || !cur.pop() {
                break;
            }
        }
        if !dirs.contains(&mount.to_path_buf()) {
            dirs.push(mount.to_path_buf());
        }
        dirs
    };
    let mut v2_rel = "/".to_string();
    let mut v1_rel = "/".to_string();
    for line in membership.lines() {
        let mut parts = line.splitn(3, ':');
        let (Some(_id), Some(ctrl), Some(path)) = (parts.next(), parts.next(), parts.next()) else {
            continue;
        };
        if ctrl.is_empty() {
            v2_rel = path.to_string();
        } else if ctrl.split(',').any(|c| c == "cpu") {
            v1_rel = path.to_string();
        }
    }
    for dir in chain(&base, &v2_rel) {
        if let Some(t) = read_trim(dir.join("cpu.max")) {
            consider(
                parse_v2(&t),
                format!(
                    "cgroup v2 cpu.max \"{t}\" ({})",
                    dir.strip_prefix(root).unwrap_or(&dir).display()
                ),
            );
        }
    }
    for mount in ["cpu,cpuacct", "cpu"] {
        let mount = base.join(mount);
        for dir in chain(&mount, &v1_rel) {
            let (Some(q), Some(p)) = (
                read_trim(dir.join("cpu.cfs_quota_us")),
                read_trim(dir.join("cpu.cfs_period_us")),
            ) else {
                continue;
            };
            let (Ok(qf), Ok(pf)) = (q.parse::<f64>(), p.parse::<f64>()) else {
                continue;
            };
            if qf > 0.0 && pf > 0.0 {
                consider(
                    Some(qf / pf),
                    format!(
                        "cgroup v1 cpu.cfs_quota_us {q} / cpu.cfs_period_us {p} ({})",
                        dir.strip_prefix(root).unwrap_or(&dir).display()
                    ),
                );
            }
        }
    }
    best
}

/// CPUs a pinned-copies run can use at once: the affinity mask, cut to the
/// CPU quota (rounded down, at least 1).  `declared` is OPTLEVEL_CPUS, for
/// containers whose allotment no kernel interface shows.
pub fn usable_cpus(
    affinity: usize,
    quota: Option<&Quota>,
    declared: Option<usize>,
) -> (usize, String) {
    let mut n = affinity.max(1);
    let mut why = format!("affinity mask ({affinity} CPUs)");
    if let Some(q) = quota {
        let cap = ((q.cores + 1e-9).floor() as usize).max(1);
        if cap < n {
            n = cap;
            why = format!(
                "{} = {:.2} cores, below the affinity mask ({affinity} CPUs)",
                q.source, q.cores
            );
        }
    }
    if let Some(d) = declared.filter(|d| *d >= 1 && *d < n) {
        n = d;
        why = format!("OPTLEVEL_CPUS={d} (declared by the operator)");
    }
    (n, why)
}

fn declared_cpus() -> Option<usize> {
    std::env::var("OPTLEVEL_CPUS").ok()?.trim().parse().ok()
}

/// The number of CPUs calibrate, measure and controls scale to.
pub fn usable_cpu_count() -> usize {
    usable_cpus(
        affinity().len(),
        cgroup_cpu_quota(Path::new("/")).as_ref(),
        declared_cpus(),
    )
    .0
}

pub fn cache_sizes() -> Value {
    let mut out = serde_json::Map::new();
    let re = Regex::new(r"^(\d+)([KMG]?)").unwrap();
    let mut dirs: Vec<PathBuf> = std::fs::read_dir("/sys/devices/system/cpu/cpu0/cache")
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().starts_with("index"))
                .unwrap_or(false)
        })
        .collect();
    dirs.sort();
    for idx in dirs {
        let (lvl, typ, size) = (
            read_trim(idx.join("level")),
            read_trim(idx.join("type")),
            read_trim(idx.join("size")).unwrap_or_default(),
        );
        if let (Some(lvl), Some(m)) = (lvl.filter(|l| !l.is_empty()), re.captures(&size)) {
            let mult: u64 = match &m[2] {
                "K" => 1 << 10,
                "M" => 1 << 20,
                "G" => 1 << 30,
                _ => 1,
            };
            let suffix = match typ.as_deref() {
                Some("Data") => "d",
                Some("Instruction") => "i",
                _ => "",
            };
            out.insert(
                format!("L{lvl}{suffix}"),
                json!(m[1].parse::<u64>().unwrap_or(0) * mult),
            );
        }
    }
    Value::Object(out)
}

fn uname() -> (String, String) {
    // SAFETY: utsname is plain data filled by uname; the fields are NUL-terminated.
    unsafe {
        let mut u: libc::utsname = std::mem::zeroed();
        if libc::uname(&mut u) != 0 {
            return ("unknown".into(), "unknown".into());
        }
        let s = |p: &[libc::c_char]| {
            std::ffi::CStr::from_ptr(p.as_ptr())
                .to_string_lossy()
                .into_owned()
        };
        (s(&u.machine), s(&u.release))
    }
}

fn sorted_dir(path: &str) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(path)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    v.sort();
    v
}

pub fn host_info() -> Value {
    let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let model = Regex::new(r"model name\s*:\s*(.*)")
        .unwrap()
        .captures(&cpuinfo)
        .map(|m| m[1].trim().to_string());
    let mut flagset: Vec<String> = Regex::new(r"(?m)^(?:flags|Features)\s*:\s*(.*)")
        .unwrap()
        .captures(&cpuinfo)
        .map(|m| m[1].split_whitespace().map(String::from).collect())
        .unwrap_or_default();
    flagset.sort();
    let caches = cache_sizes();
    let cpus = affinity();
    let mem = Regex::new(r"MemTotal:\s*(\d+)")
        .unwrap()
        .captures(&std::fs::read_to_string("/proc/meminfo").unwrap_or_default())
        .and_then(|m| m[1].parse::<u64>().ok())
        .map(|kb| kb * 1024);
    let node_re = Regex::new(r"^node\d+$").unwrap();
    let nodes = sorted_dir("/sys/devices/system/node")
        .iter()
        .filter(|n| node_re.is_match(n))
        .count();
    let (arch, kernel) = uname();
    let hypervisor = flagset.iter().any(|f| f == "hypervisor");
    let logical = online_cpus();
    let cpu_model = model.unwrap_or_else(|| "unknown".into());
    let quota = cgroup_cpu_quota(Path::new("/"));
    let (usable, usable_why) = usable_cpus(cpus.len(), quota.as_ref(), declared_cpus());
    // The fingerprint hashes the same text as the tool's first version, so a
    // calibration taken by either version matches this host in both.
    let key = py_dumps(&json!([
        cpu_model, arch, logical, flagset, caches, mem, hypervisor
    ]));
    json!({
        "cpu_model": cpu_model, "arch": arch, "kernel": kernel,
        "logical_cpus": logical, "allowed_cpus": cpus,
        "affinity_cpus": cpus.len(),
        "cpu_quota_cores": quota.as_ref().map(|q| q.cores),
        "cpu_quota_source": quota.as_ref().map(|q| q.source.clone()),
        "usable_cpus": usable, "usable_cpus_basis": usable_why,
        "isa": flagset.iter().filter(|f| ISA_FLAGS.contains(&f.as_str())).collect::<Vec<_>>(),
        "hypervisor": hypervisor, "caches_bytes": caches, "mem_total_bytes": mem,
        "smt_active": read_trim("/sys/devices/system/cpu/smt/active"),
        "thread_siblings_cpu0": read_trim("/sys/devices/system/cpu/cpu0/topology/thread_siblings_list"),
        "numa_nodes": if nodes > 0 { json!(nodes) } else { Value::Null },
        "cpufreq_governor": read_trim("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor"),
        "turbo_disabled": read_trim("/sys/devices/system/cpu/intel_pstate/no_turbo"),
        "cpufreq_boost": read_trim("/sys/devices/system/cpu/cpufreq/boost"),
        "event_sources": sorted_dir("/sys/bus/event_source/devices"),
        "perf_event_paranoid": read_trim("/proc/sys/kernel/perf_event_paranoid"),
        "cgroup_cpu_max": read_trim("/sys/fs/cgroup/cpu.max"),
        "fingerprint": sha256_hex(key.as_bytes())[..16].to_string(),
    })
}

/// Which helper programs are missing, with what needs them.
pub fn helper_tools() -> Value {
    let found: serde_json::Map<String, Value> = HELPER_TOOLS
        .iter()
        .map(|t| (t.to_string(), json!(which(t))))
        .collect();
    let missing: Vec<&str> = HELPER_TOOLS
        .iter()
        .copied()
        .filter(|t| found[*t].is_null())
        .collect();
    json!({"found": found, "missing": missing,
           "needed_by": "tools/profile.sh (its ncu and nsys subcommands exit 2 without gawk and jq)"})
}

// ---- PMU ---------------------------------------------------------------------
pub fn find_perf() -> Option<String> {
    let mut cands: Vec<String> = Vec::new();
    cands.extend(std::env::var("PERF").ok());
    cands.extend(which("perf"));
    let mut glob = |dir: &str, prefix: &str| {
        let mut v: Vec<String> = sorted_dir(dir)
            .into_iter()
            .filter(|n| n.starts_with(prefix))
            .map(|n| format!("{dir}/{n}/perf"))
            .collect();
        v.reverse();
        cands.extend(v);
    };
    glob("/usr/lib", "linux-tools");
    glob("/usr/lib/linux-tools", "");
    cands.into_iter().find(|p| {
        Command::new(p)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    })
}

/// The same test as tools/perf_ipc.sh: can this process count cycles:u and
/// instructions:u?  Returns the perf path, or the reason it is unavailable.
pub fn pmu_probe() -> Result<String, String> {
    let sources = sorted_dir("/sys/bus/event_source/devices");
    let perf = find_perf().ok_or_else(|| {
        format!(
            "no working perf binary (event sources: {})",
            if sources.is_empty() {
                "none".into()
            } else {
                sources.join(" ")
            }
        )
    })?;
    let r = Command::new(&perf)
        .args(["stat", "-x,", "-e", "cycles:u,instructions:u", "--", "true"])
        .output()
        .map_err(|e| format!("perf probe failed: {e}"))?;
    classify_perf_probe(&String::from_utf8_lossy(&r.stderr), r.status.code())?;
    Ok(perf)
}
pub fn classify_perf_probe(stderr: &str, code: Option<i32>) -> Result<(), String> {
    if Regex::new(r"<not (supported|counted)>")
        .unwrap()
        .is_match(stderr)
    {
        return Err("perf cannot count cycles:u/instructions:u here (<not supported>): no hardware PMU is exposed".into());
    }
    if Regex::new(r"(?i)permission|access to performance monitoring")
        .unwrap()
        .is_match(stderr)
    {
        return Err(format!(
            "perf_event_paranoid={} forbids counting",
            read_trim("/proc/sys/kernel/perf_event_paranoid").unwrap_or_else(|| "?".into())
        ));
    }
    match code {
        Some(0) => Ok(()),
        c => Err(format!(
            "perf probe exit {}",
            c.map(|c| c.to_string()).unwrap_or_else(|| "signal".into())
        )),
    }
}

// ---- conditions around a run ---------------------------------------------------
pub struct ProcStat {
    pub busy_s: f64,
    pub steal_s: f64,
}
pub fn proc_stat() -> ProcStat {
    let text = std::fs::read_to_string("/proc/stat").unwrap_or_default();
    let mut v: Vec<f64> = text
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse().ok())
        .collect();
    v.resize(v.len().max(8), 0.0);
    // SAFETY: sysconf has no preconditions.
    let hz = (unsafe { libc::sysconf(libc::_SC_CLK_TCK) }).max(1) as f64;
    ProcStat {
        busy_s: (v[0] + v[1] + v[2] + v[5] + v[6]) / hz,
        steal_s: v[7] / hz,
    }
}
pub fn throttle_count() -> Option<u64> {
    let mut tot = None;
    for cpu in sorted_dir("/sys/devices/system/cpu") {
        if !cpu.starts_with("cpu")
            || !cpu[3..]
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
        {
            continue;
        }
        let dir = format!("/sys/devices/system/cpu/{cpu}/thermal_throttle");
        for f in sorted_dir(&dir)
            .into_iter()
            .filter(|f| f.ends_with("_throttle_count"))
        {
            if let Some(v) = read_trim(format!("{dir}/{f}")).and_then(|v| v.parse::<u64>().ok()) {
                tot = Some(tot.unwrap_or(0) + v);
            }
        }
    }
    tot
}
pub fn loadavg1() -> f64 {
    read_trim("/proc/loadavg")
        .and_then(|s| s.split_whitespace().next().and_then(|x| x.parse().ok()))
        .unwrap_or(0.0)
}

pub struct Begin {
    t: std::time::Instant,
    stat: ProcStat,
    throttle: Option<u64>,
    loadavg: f64,
    psi_cpu: Option<String>,
}
pub fn conditions_begin() -> Begin {
    Begin {
        t: std::time::Instant::now(),
        stat: proc_stat(),
        throttle: throttle_count(),
        loadavg: loadavg1(),
        psi_cpu: read_trim("/proc/pressure/cpu"),
    }
}
pub fn conditions_end(begin: &Begin, own_cpu_s: f64) -> Value {
    let wall = begin.t.elapsed().as_secs_f64().max(1e-9);
    let st = proc_stat();
    let other = ((st.busy_s - begin.stat.busy_s) - own_cpu_s).max(0.0) / wall;
    let psi = begin.psi_cpu.as_deref().and_then(|p| {
        Regex::new(r"some avg10=([\d.]+)")
            .unwrap()
            .captures(p)
            .and_then(|m| m[1].parse::<f64>().ok())
    });
    let thr = match (throttle_count(), begin.throttle) {
        (Some(a), Some(b)) => json!(a as i64 - b as i64),
        _ => Value::Null,
    };
    json!({"loadavg_1m_before": begin.loadavg, "loadavg_1m_after": loadavg1(), "psi_cpu_some_avg10_before": psi,
           "other_cpu_cores": other, // CPU-seconds per second used by everything else on the host
           "steal_cores": (st.steal_s - begin.stat.steal_s) / wall, "thermal_throttle_events": thr})
}
