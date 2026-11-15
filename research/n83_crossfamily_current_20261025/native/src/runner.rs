//! Bounded Linux producer for the frozen two-stratum N83 schedule.
use super::{hash, Arm, Result, ARMS, FILES, SOURCE_SHA256};
use crate::provider::Bound;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const RSS_CAP: u64 = 61 * 1024 * 1024 * 1024 / 4;
const PRODUCER_CAP: u64 = 300 * 60;
const RHO_CAP: u64 = 120;
const IC_CAP: u64 = 1200;
const RHO_SEED: &str = "2026101601";
const RHO_SEED_BASE: u64 = 2_026_122_600;

fn read_optional(path: impl AsRef<Path>) -> Option<String> {
    fs::read_to_string(path).ok()
}
fn utc() -> String {
    let output = Command::new("date")
        .arg("-u")
        .arg("+%Y-%m-%dT%H:%M:%SZ")
        .output();
    output
        .ok()
        .filter(|x| x.status.success())
        .and_then(|x| String::from_utf8(x.stdout).ok())
        .map(|x| x.trim().to_owned())
        .unwrap_or_else(|| "UNKNOWN_UTC".to_owned())
}
fn write_new(path: &Path, value: &Value) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}
fn file_hash(path: &Path) -> Result<String> {
    Ok(hash(&fs::read(path)?))
}
fn cgroup_root() -> PathBuf {
    if let Some(lines) = read_optional("/proc/self/cgroup") {
        for line in lines.lines() {
            if let Some(relative) = line.strip_prefix("0::") {
                let path = Path::new("/sys/fs/cgroup").join(relative.trim_start_matches('/'));
                if path.exists() {
                    return path;
                }
            }
        }
    }
    PathBuf::from("/sys/fs/cgroup")
}
fn cgroup_snapshot(root: &Path) -> Value {
    let keys = [
        "memory.current",
        "memory.peak",
        "memory.max",
        "memory.events",
        "cpu.stat",
        "cpu.max",
        "cpu.pressure",
        "cpuset.cpus.effective",
    ];
    let mut data = serde_json::Map::new();
    for key in keys {
        data.insert(
            key.to_owned(),
            read_optional(root.join(key)).map_or(Value::Null, Value::String),
        );
    }
    Value::Object(data)
}
pub fn affinity() -> Result<Vec<usize>> {
    let mut set: libc::cpu_set_t = unsafe { std::mem::zeroed() };
    if unsafe { libc::sched_getaffinity(0, std::mem::size_of_val(&set), &mut set) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok((0..libc::CPU_SETSIZE as usize)
        .filter(|index| unsafe { libc::CPU_ISSET(*index, &set) })
        .collect())
}
fn memory_total_kib() -> Option<u64> {
    read_optional("/proc/meminfo")?.lines().find_map(|line| {
        line.strip_prefix("MemTotal:")?
            .split_whitespace()
            .next()?
            .parse()
            .ok()
    })
}
fn disk_free(path: &Path) -> Result<u64> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    let c_path = CString::new(path.as_os_str().as_bytes())?;
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(c_path.as_ptr(), &mut stat) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(stat.f_bavail.saturating_mul(stat.f_frsize))
}
fn cpu_telemetry(cpus: &[usize]) -> Value {
    let stat = read_optional("/proc/stat").unwrap_or_default();
    let aggregate = stat
        .lines()
        .find(|x| x.starts_with("cpu "))
        .unwrap_or_default();
    let jiffies: Vec<u64> = aggregate
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse().ok())
        .collect();
    let interrupts = stat
        .lines()
        .find(|x| x.starts_with("intr "))
        .and_then(|x| x.split_whitespace().nth(1))
        .and_then(|x| x.parse::<u64>().ok());
    let frequencies: BTreeMap<String, Option<String>> = cpus
        .iter()
        .map(|id| {
            (
                id.to_string(),
                read_optional(format!(
                    "/sys/devices/system/cpu/cpu{id}/cpufreq/scaling_cur_freq"
                )),
            )
        })
        .collect();
    json!({"recorded_utc":utc(),"cpu_jiffies":jiffies,"steal_jiffies":jiffies.get(7),"interrupts_total":interrupts,"frequency_khz_by_visible_cpu":frequencies,"cpu_pressure":read_optional("/proc/pressure/cpu")})
}
fn topology(cpus: &[usize]) -> Value {
    let lscpu = Command::new("lscpu").arg("-J").output();
    let (lscpu_json, lscpu_exit) = match lscpu {
        Ok(output) => (
            output
                .status
                .success()
                .then(|| String::from_utf8_lossy(&output.stdout).into_owned()),
            output.status.code(),
        ),
        Err(_) => (None, None),
    };
    let mut visible = serde_json::Map::new();
    for id in cpus {
        let mut cpu = serde_json::Map::new();
        for key in ["core_id", "physical_package_id", "thread_siblings_list"] {
            cpu.insert(
                key.to_owned(),
                read_optional(format!("/sys/devices/system/cpu/cpu{id}/topology/{key}"))
                    .map_or(Value::Null, Value::String),
            );
        }
        visible.insert(id.to_string(), Value::Object(cpu));
    }
    json!({"lscpu_json":lscpu_json,"lscpu_exit_status":lscpu_exit,"online_cpus":read_optional("/sys/devices/system/cpu/online"),"online_numa_nodes":read_optional("/sys/devices/system/node/online"),"cgroup_effective_cpus":read_optional("/sys/fs/cgroup/cpuset.cpus.effective"),"visible_cpu_topology":visible})
}
fn host(root: &Path, group: &Path, stratum: usize, bound: &Bound) -> Result<Value> {
    let cpus = affinity()?;
    let before = cgroup_snapshot(group);
    let id = bound.allocation["instance_id"]
        .as_str()
        .ok_or("bound instance ID absent")?;
    let allocation = &bound.allocation;
    Ok(
        json!({"schema":"n83.crossfamily-current-host/v2","provider":"aws_ec2_spot","instance_id":format!("{id}-crossfamily-current-stratum{stratum}"),"parent_instance_id":id,"stratum":stratum,"visible_affinity":cpus,"mem_total_kib":memory_total_kib(),"disk_free_bytes":disk_free(root)?,"cgroup_path":group,"cgroup_before":before,"topology":topology(&cpus),"cpu_telemetry_before":cpu_telemetry(&cpus),"recorded_utc":utc(),"tier":"cloud_instance_tier1","instance_type":allocation["instance_type"],"availability_zone":allocation["availability_zone"],"region":allocation["region"],"account_id":allocation["account_id"],"image_id":allocation["image_id"],"instance_identity":bound.identity,"git_sha":allocation["source_head_sha"],"source_archive_sha256":bound.archive_sha256,"source_manifest_sha256":bound.manifest_sha256,"aws_allocation_sha256":bound.allocation_sha256}),
    )
}
fn sampled_rss(pid: i32) -> Option<u64> {
    read_optional(format!("/proc/{pid}/status"))?
        .lines()
        .find_map(|line| {
            line.strip_prefix("VmRSS:")?
                .split_whitespace()
                .next()?
                .parse::<u64>()
                .ok()
        })
        .map(|kib| kib.saturating_mul(1024))
}
fn process(
    root: &Path,
    folder: &Path,
    command: &[String],
    environment: &BTreeMap<String, String>,
    cap: u64,
    group: &Path,
    cpus: &[usize],
) -> Result<Value> {
    fs::create_dir(folder)?;
    let flags: BTreeMap<_, _> = environment
        .iter()
        .filter(|(key, _)| key.starts_with("KIC_"))
        .collect();
    write_new(
        &folder.join("launch.json"),
        &json!({"command":command,"environment_flags":flags,"cap_seconds":cap,"rss_cap_bytes":RSS_CAP,"started_utc":utc()}),
    )?;
    let before = cgroup_snapshot(group);
    let cpu_before = cpu_telemetry(cpus);
    let stdout = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(folder.join("stdout.jsonl"))?;
    let stderr = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(folder.join("stderr.log"))?;
    let mut call = Command::new(&command[0]);
    call.args(&command[1..])
        .current_dir(root)
        .env_clear()
        .envs(environment)
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));
    unsafe {
        call.pre_exec(|| {
            if libc::setpgid(0, 0) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let child = call.spawn()?;
    let pid = child.id() as i32;
    let started = Instant::now();
    let mut sampled_peak = 0u64;
    let mut stop: Option<&str> = None;
    let mut raw_status = 0i32;
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    loop {
        let got = unsafe { libc::wait4(pid, &mut raw_status, libc::WNOHANG, &mut usage) };
        if got == pid {
            break;
        }
        if got < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        if let Some(rss) = sampled_rss(pid) {
            sampled_peak = sampled_peak.max(rss);
            if rss > RSS_CAP {
                stop = Some("STOPPED_RESOURCE_GUARD");
            }
        }
        if started.elapsed() >= Duration::from_secs(cap) {
            stop = Some("TIMEOUT");
        }
        if stop.is_some() {
            unsafe {
                libc::kill(-pid, libc::SIGKILL);
            }
            if unsafe { libc::wait4(pid, &mut raw_status, 0, &mut usage) } != pid {
                return Err("failed to reap guarded child".into());
            }
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    let code = std::process::ExitStatus::from_raw(raw_status)
        .code()
        .unwrap_or(-1);
    let wall = started.elapsed().as_secs_f64();
    let user = usage.ru_utime.tv_sec as f64 + usage.ru_utime.tv_usec as f64 / 1e6;
    let system = usage.ru_stime.tv_sec as f64 + usage.ru_stime.tv_usec as f64 / 1e6;
    let result = json!({"exit_status":code,"stop_reason":stop,"wall_seconds":wall,"user_cpu_seconds":user,"system_cpu_seconds":system,"peak_rss_bytes":(usage.ru_maxrss as u64).saturating_mul(1024),"sampled_peak_rss_bytes":sampled_peak,"involuntary_context_switches":usage.ru_nivcsw,"major_faults":usage.ru_majflt,"finished_utc":utc(),"cgroup_before":before,"cgroup_after":cgroup_snapshot(group),"status":stop.unwrap_or(if code == 0 { "PASS_PRODUCER" } else { "FAILED" })});
    write_new(&folder.join("usage.json"), &result)?;
    fs::write(folder.join("exit_status.txt"), format!("{code}\n"))?;
    write_new(
        &folder.join("host_telemetry.json"),
        &json!({"before":cpu_before,"after":cpu_telemetry(cpus)}),
    )?;
    Ok(result)
}
fn arm(name: &str) -> Result<(usize, Arm)> {
    ARMS.iter()
        .copied()
        .enumerate()
        .find(|(_, arm)| arm.name == name)
        .ok_or_else(|| "unknown scheduled arm".into())
}
fn environment(root: &Path, folder: &Path, arm: Arm) -> Result<BTreeMap<String, String>> {
    let receipt = match arm.layout {
        "root11_wide3" => "receipt_K3450_wide3.json",
        "root4_standard" => "receipt_K3500_root4.json",
        _ => return Err("unknown layout".into()),
    };
    let value: Value = serde_json::from_slice(&fs::read(
        root.join("research/n83_k3072_current_source_20261017")
            .join(receipt),
    )?)?;
    let flags = value["environment_flags"]
        .as_object()
        .ok_or("layout flags absent")?;
    let mut env = BTreeMap::new();
    env.insert(
        "PATH".into(),
        std::env::var("PATH").unwrap_or_else(|_| "/usr/bin:/bin".into()),
    );
    for (key, value) in flags {
        env.insert(
            key.to_owned(),
            value.as_str().ok_or("nontext layout flag")?.to_owned(),
        );
    }
    for (key, value) in [
        ("KIC_TARGET_THREADS", "2"),
        ("KIC_INDEXED5_ID30", "0"),
        ("KIC_TARGET_CANCEL", "1"),
        ("KIC_INDEX_ROOT_BATCH", "1"),
    ] {
        env.insert(key.into(), value.into());
    }
    env.insert(
        "KIC_DUMP_BASE".into(),
        folder
            .join("base_dump.jsonl")
            .to_string_lossy()
            .into_owned(),
    );
    Ok(env)
}
pub struct RunInputs<'a> {
    pub root: &'a Path,
    pub stratum: usize,
    pub ic: &'a Path,
    pub rho: &'a Path,
    pub output: &'a Path,
}
pub fn run(inputs: RunInputs<'_>, bound: Bound, choice: Value, events: Value) -> Result<()> {
    let RunInputs {
        root,
        stratum,
        ic,
        rho,
        output,
    } = inputs;
    if output.exists() {
        return Err("output directory already exists".into());
    }
    if !ic.is_file() || !rho.is_file() {
        return Err("producer binary missing".into());
    }
    fs::create_dir(output)?;
    let group = cgroup_root();
    let host = host(root, &group, stratum, &bound)?;
    let cpus = affinity()?;
    write_new(&output.join("host.json"), &host)?;
    let mut bound_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output.join("aws_allocation.json"))?;
    bound_file.write_all(&bound.allocation_bytes)?;
    bound_file.sync_all()?;
    let memory = host["mem_total_kib"]
        .as_u64()
        .unwrap_or(0)
        .saturating_mul(1024);
    let limit = host["cgroup_before"]["memory.max"]
        .as_str()
        .and_then(|x| x.trim().parse::<u64>().ok());
    if memory < RSS_CAP + 256 * 1024 * 1024
        || host["disk_free_bytes"].as_u64().unwrap_or(0) < 3 * 1024 * 1024 * 1024
        || cpus.len() < 4
        || limit.is_some_and(|x| x < RSS_CAP)
    {
        write_new(
            &output.join("preflight.json"),
            &json!({"status":"REJECTED_HOST_CAPACITY","host":host}),
        )?;
        return Err("host capacity gate failed".into());
    }
    write_new(
        &output.join("preflight.json"),
        &json!({"status":"PASS_CAPACITY_PREFLIGHT","rss_cap_bytes":RSS_CAP}),
    )?;
    let binaries = output.join("executed_bins");
    fs::create_dir(&binaries)?;
    let ic_copy = binaries.join("koblitz_orbit_dlp_fast");
    let rho_copy = binaries.join("n83_rho_optimization_eval");
    fs::copy(ic, &ic_copy)?;
    fs::copy(rho, &rho_copy)?;
    let input = root.join("research/n83_crossfamily_current_20261025");
    let targets = fs::read_to_string(input.join("targets.jsonl"))?;
    let lines: Vec<_> = targets
        .split_inclusive('\n')
        .filter(|x| !x.is_empty())
        .collect();
    if lines.len() != 16 || lines.iter().any(|x| !x.ends_with('\n')) {
        return Err("sealed target stream differs".into());
    }
    let mut hashes = serde_json::Map::new();
    for (name, _) in FILES {
        hashes.insert(name.into(), Value::String(file_hash(&input.join(name))?));
    }
    let event_rows = events.as_array().ok_or("schedule absent")?;
    write_new(
        &output.join("source.json"),
        &json!({"schema":"n83.crossfamily-current-source/v2","source_sha256":SOURCE_SHA256,"binary_sha256":file_hash(&ic_copy)?,"rho_binary_sha256":file_hash(&rho_copy)?,"runner_sha256":file_hash(&root.join("research/n83_crossfamily_current_20261025/native/src/runner.rs"))?,"targets_sha256":FILES[0].1,"answers_sha256":FILES[1].1,"fixture_manifest_sha256":file_hash(&input.join("fixture_manifest.json"))?,"stratum":stratum,"choice":choice,"events":events,"hashes":hashes,"rho_seed_rule":"2026122600+arm_count*point_index+fixed_candidate_index","calibration_seed":RHO_SEED,"calibration_repetitions_per_side":3,"global_producer_cap_seconds":PRODUCER_CAP,"ic_cap_seconds":IC_CAP,"rho_cap_seconds":RHO_CAP,"aws_allocation_sha256":bound.allocation_sha256,"source_archive_sha256":bound.archive_sha256,"source_manifest_sha256":bound.manifest_sha256,"source_head_sha":bound.allocation["source_head_sha"]}),
    )?;
    let started = Instant::now();
    for event in event_rows {
        let index = event["point_index"]
            .as_u64()
            .ok_or("scheduled point absent")? as usize;
        let candidate = event["candidate"]
            .as_str()
            .ok_or("scheduled candidate absent")?;
        let name = event["name"].as_str().ok_or("scheduled name absent")?;
        let kind = event["kind"].as_str().ok_or("scheduled kind absent")?;
        let (fixed, selected) = arm(candidate)?;
        let point = output.join(format!("points/p{index:02}"));
        fs::create_dir_all(&point)?;
        let target = point.join("target.jsonl");
        if target.exists() {
            if fs::read_to_string(&target)? != lines[index] {
                return Err("per-point target differs".into());
            }
        } else {
            fs::write(&target, lines[index])?;
        }
        let folder = point.join("runs").join(name);
        fs::create_dir_all(folder.parent().ok_or("run parent absent")?)?;
        let cap = if kind == "ic" { IC_CAP } else { RHO_CAP };
        if started.elapsed().as_secs_f64() + cap as f64 + 30.0 > PRODUCER_CAP as f64 {
            write_new(
                &output.join("producer_stop.json"),
                &json!({"status":"GLOBAL_CAP_BEFORE_NEXT_ARM","elapsed_seconds":started.elapsed().as_secs_f64(),"next_event":event,"global_producer_cap_seconds":PRODUCER_CAP}),
            )?;
            break;
        }
        let (command, env) = if kind == "ic" {
            (
                vec![
                    ic_copy.to_string_lossy().into_owned(),
                    format!("construct:83:1:{}", selected.k),
                    target.to_string_lossy().into_owned(),
                    "29".into(),
                    folder
                        .join("targets_result.jsonl")
                        .to_string_lossy()
                        .into_owned(),
                ],
                environment(root, &folder, selected)?,
            )
        } else {
            let (rho_target, seed) = if kind == "calibration" {
                (
                    root.join("research/n83_cold_factor_base_20261016/targets.jsonl"),
                    RHO_SEED.parse::<u64>()?,
                )
            } else {
                (
                    target.clone(),
                    RHO_SEED_BASE + 4 * index as u64 + fixed as u64,
                )
            };
            let mut env = BTreeMap::new();
            env.insert(
                "PATH".into(),
                std::env::var("PATH").unwrap_or_else(|_| "/usr/bin:/bin".into()),
            );
            (
                vec![
                    rho_copy.to_string_lossy().into_owned(),
                    rho_target.to_string_lossy().into_owned(),
                    seed.to_string(),
                    root.join("research/n83_dense_root_20261013/rho_config.json")
                        .to_string_lossy()
                        .into_owned(),
                    folder.join("receipt.json").to_string_lossy().into_owned(),
                ],
                env,
            )
        };
        println!("starting p{index:02}/{name} at {}", utc());
        let usage = process(root, &folder, &command, &env, cap, &group, &cpus)?;
        println!(
            "p{index:02}/{name}: {} after {:.3}s",
            usage["status"],
            usage["wall_seconds"].as_f64().unwrap_or(0.0)
        );
    }
    write_new(
        &output.join("host_telemetry_after.json"),
        &cpu_telemetry(&cpus),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn controlled_child(name: &str, command: &[&str], cap: u64) -> Result<Value> {
        let base = std::env::temp_dir().join(format!("n83-native-{name}-{}", std::process::id()));
        if base.exists() {
            fs::remove_dir_all(&base)?;
        }
        fs::create_dir(&base)?;
        let folder = base.join("child");
        let args = command
            .iter()
            .map(|arg| (*arg).to_owned())
            .collect::<Vec<_>>();
        let env = BTreeMap::from([("PATH".to_owned(), "/usr/bin:/bin".to_owned())]);
        let result = process(
            Path::new("/"),
            &folder,
            &args,
            &env,
            cap,
            &cgroup_root(),
            &affinity()?,
        )?;
        assert_eq!(
            fs::read_to_string(folder.join("exit_status.txt"))?
                .trim()
                .parse::<i32>()?,
            result["exit_status"].as_i64().unwrap() as i32
        );
        assert_eq!(
            serde_json::from_slice::<Value>(&fs::read(folder.join("usage.json"))?)?,
            result
        );
        fs::remove_dir_all(base)?;
        Ok(result)
    }

    #[test]
    fn successful_child_has_a_reaped_receipt() -> Result<()> {
        let result = controlled_child("pass", &["/bin/true"], 2)?;
        assert_eq!(result["status"], "PASS_PRODUCER");
        assert_eq!(result["exit_status"], 0);
        Ok(())
    }

    #[test]
    fn timed_out_child_is_killed_and_reaped() -> Result<()> {
        let result = controlled_child("timeout", &["/bin/sleep", "3"], 1)?;
        assert_eq!(result["status"], "TIMEOUT");
        assert_eq!(result["stop_reason"], "TIMEOUT");
        assert_ne!(result["exit_status"], 0);
        Ok(())
    }
}
