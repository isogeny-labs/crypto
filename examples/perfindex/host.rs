//! What the host is and how busy it is: the facts that decide how far a wall
//! time can be trusted.

use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::process::Command;

pub fn read_trim(path: &str) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// First line of a command's output (stdout, else stderr); empty if it cannot run.
pub fn first_line(cmd: &str, args: &[&str]) -> String {
    match Command::new(cmd).args(args).output() {
        Ok(o) => {
            let text = if o.stdout.is_empty() {
                o.stderr
            } else {
                o.stdout
            };
            String::from_utf8_lossy(&text)
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string()
        }
        Err(_) => String::new(),
    }
}

pub fn clk_tck() -> u64 {
    // SAFETY: sysconf has no preconditions.
    let t = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
    if t > 0 {
        t as u64
    } else {
        100
    }
}

/// Scheduler ticks from `/proc/stat`.  `steal` is time the hypervisor ran
/// something else while this VM had work ready: invisible to the process's
/// own CPU accounting, and the usual cause of a slow sample whose CPU time
/// still equals its wall time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ticks {
    pub total: u64,
    pub steal: u64,
}

/// Sums the tick counters over `cpus` (all CPUs, from the aggregate line, when `None`).
pub fn parse_ticks(stat: &str, cpus: Option<&BTreeSet<usize>>) -> Option<Ticks> {
    let (mut total, mut steal) = (0u64, 0u64);
    let mut any = false;
    for line in stat.lines() {
        let mut f = line.split_whitespace();
        let Some(name) = f.next() else { continue };
        let Some(rest) = name.strip_prefix("cpu") else {
            continue;
        };
        let wanted = match (rest.is_empty(), cpus) {
            (true, None) => true,
            (false, Some(set)) => rest.parse::<usize>().is_ok_and(|i| set.contains(&i)),
            _ => false,
        };
        if !wanted {
            continue;
        }
        let v: Vec<u64> = f.filter_map(|x| x.parse().ok()).collect();
        if v.len() < 4 {
            continue;
        }
        total += v.iter().take(8).sum::<u64>(); // user nice system idle iowait irq softirq steal
        steal += v.get(7).copied().unwrap_or(0);
        any = true;
    }
    any.then_some(Ticks { total, steal })
}

pub fn cpu_ticks(cpus: Option<&BTreeSet<usize>>) -> Option<Ticks> {
    parse_ticks(&std::fs::read_to_string("/proc/stat").ok()?, cpus)
}

/// `"0-2,5"` as a set; `None` for an empty list.
pub fn parse_cpu_list(text: &str) -> Option<BTreeSet<usize>> {
    let mut set = BTreeSet::new();
    for part in text.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some((lo, hi)) = part.split_once('-') {
            let (lo, hi) = (
                lo.trim().parse::<usize>().ok()?,
                hi.trim().parse::<usize>().ok()?,
            );
            set.extend(lo..=hi);
        } else {
            set.insert(part.parse().ok()?);
        }
    }
    (!set.is_empty()).then_some(set)
}

fn loadavg() -> Option<Vec<f64>> {
    let s = std::fs::read_to_string("/proc/loadavg").ok()?;
    let v: Vec<f64> = s
        .split_whitespace()
        .take(3)
        .filter_map(|x| x.parse().ok())
        .collect();
    (v.len() == 3).then_some(v)
}

/// The 10-second PSI averages of `/proc/pressure/<kind>`.
fn pressure(kind: &str) -> Value {
    let Ok(text) = std::fs::read_to_string(format!("/proc/pressure/{kind}")) else {
        return Value::Null;
    };
    let mut out = serde_json::Map::new();
    for line in text.lines() {
        let mut f = line.split_whitespace();
        let Some(name) = f.next() else { continue };
        let mut m = serde_json::Map::new();
        for kv in f {
            if let Some((k, v)) = kv.split_once('=') {
                if let Ok(x) = v.parse::<f64>() {
                    m.insert(k.to_string(), json!(x));
                }
            }
        }
        out.insert(name.to_string(), Value::Object(m));
    }
    Value::Object(out)
}

/// A cheap snapshot of how busy the machine is right now.
pub fn conditions() -> Value {
    json!({
        "unix_time": crate::util::unix_time(),
        "loadavg": loadavg(),
        "psi_cpu": pressure("cpu"),
        "psi_memory": pressure("memory"),
    })
}

fn cpuinfo() -> (String, Vec<String>) {
    let text = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let mut model = String::new();
    let mut flags = Vec::new();
    let keep = |f: &str| {
        matches!(
            f,
            "popcnt"
                | "pclmulqdq"
                | "vpclmulqdq"
                | "gfni"
                | "bmi1"
                | "bmi2"
                | "adx"
                | "avx"
                | "avx2"
        ) || (f.starts_with("avx512"))
    };
    for line in text.lines() {
        if let Some(v) = line.strip_prefix("model name") {
            if model.is_empty() {
                model = v.trim_start_matches([' ', '\t', ':']).trim().to_string();
            }
        } else if let Some(v) = line.strip_prefix("flags") {
            if flags.is_empty() {
                flags = v
                    .trim_start_matches([' ', '\t', ':'])
                    .split_whitespace()
                    .filter(|f| keep(f))
                    .map(str::to_string)
                    .collect();
            }
        }
    }
    (model, flags)
}

fn mem_total() -> String {
    std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|t| {
            t.lines()
                .find(|l| l.starts_with("MemTotal"))
                .map(|l| l.split_once(':').map_or("", |x| x.1).trim().to_string())
        })
        .unwrap_or_default()
}

fn os_release() -> String {
    format!(
        "{} {}",
        first_line("uname", &["-s"]),
        first_line("uname", &["-r"])
    )
}

/// The host.  `extended` adds the knobs that decide how noisy a timing is
/// (governor, frequency, SMT, ASLR, THP, the process's CPU mask).
pub fn host_manifest(extended: bool) -> Value {
    let (cpu, flags) = cpuinfo();
    let mut m = json!({
        "cpu": cpu,
        "cpu_flags": flags,
        "logical_cores": std::thread::available_parallelism().map(|n| n.get()).ok(),
        "memory": mem_total(),
        "os": os_release(),
        "arch": std::env::consts::ARCH,
        "rustc": first_line("rustc", &["--version"]),
        "loadavg_start": loadavg(),
    });
    if extended {
        let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
        let allowed = status
            .lines()
            .find_map(|l| l.strip_prefix("Cpus_allowed_list:"))
            .map(|s| s.trim().to_string());
        let o = m.as_object_mut().unwrap();
        o.insert("cpus_allowed".into(), json!(allowed));
        o.insert(
            "smt".into(),
            json!(read_trim("/sys/devices/system/cpu/smt/active")),
        );
        o.insert(
            "governor".into(),
            json!(read_trim(
                "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor"
            )),
        );
        o.insert(
            "cpu0_khz".into(),
            json!(
                read_trim("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")
                    .and_then(|s| s.parse::<u64>().ok())
            ),
        );
        o.insert(
            "aslr".into(),
            json!(read_trim("/proc/sys/kernel/randomize_va_space")),
        );
        o.insert(
            "thp".into(),
            json!(read_trim("/sys/kernel/mm/transparent_hugepage/enabled")),
        );
        let vg = first_line("valgrind", &["--version"]);
        o.insert("valgrind".into(), json!((!vg.is_empty()).then_some(vg)));
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    const STAT: &str = "cpu  100 0 50 800 10 0 5 20 0 0\n\
                        cpu0 40 0 20 400 5 0 2 8 0 0\n\
                        cpu1 60 0 30 400 5 0 3 12 0 0\n\
                        intr 1 2 3\n";

    #[test]
    fn ticks_sum_the_first_eight_columns_and_pick_steal() {
        let all = parse_ticks(STAT, None).unwrap();
        assert_eq!(
            all,
            Ticks {
                total: 100 + 50 + 800 + 10 + 5 + 20,
                steal: 20
            }
        );
        let c1: BTreeSet<usize> = [1].into();
        assert_eq!(
            parse_ticks(STAT, Some(&c1)).unwrap(),
            Ticks {
                total: 60 + 30 + 400 + 5 + 3 + 12,
                steal: 12
            }
        );
        let both: BTreeSet<usize> = [0, 1].into();
        assert_eq!(parse_ticks(STAT, Some(&both)).unwrap().steal, 20);
        let none: BTreeSet<usize> = [7].into();
        assert_eq!(parse_ticks(STAT, Some(&none)), None);
    }

    #[test]
    fn cpu_lists() {
        assert_eq!(parse_cpu_list("0-2,5"), Some([0, 1, 2, 5].into()));
        assert_eq!(parse_cpu_list("3"), Some([3].into()));
        assert_eq!(parse_cpu_list(""), None);
        assert_eq!(parse_cpu_list("x"), None);
    }

    #[test]
    fn the_short_manifest_keeps_the_history_keys() {
        // perfhistory.py stores this object in committed snapshots.
        let m = host_manifest(false);
        let mut keys: Vec<&String> = m.as_object().unwrap().keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                "arch",
                "cpu",
                "cpu_flags",
                "loadavg_start",
                "logical_cores",
                "memory",
                "os",
                "rustc"
            ]
        );
        assert!(host_manifest(true).get("governor").is_some());
    }

    #[test]
    fn live_ticks_are_consistent() {
        if let Some(t) = cpu_ticks(None) {
            assert!(t.total > 0 && t.steal <= t.total);
        }
        assert!(clk_tck() > 0);
    }
}
