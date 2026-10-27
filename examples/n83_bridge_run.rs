//! Native, fresh-process N83 K1107/K2304 paired Tier 1 producer.
//! Usage: n83_bridge_run <stratum 0..3> <fixture-dir> <IC> <rho> <output-dir>

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crypto_lib::hash::sha256;
use serde_json::{json, Map, Value};

const SOURCE_SHA256: &str = "8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829";
const POINTS_PER_STRATUM: usize = 8;
const PAIR_RHO_SEED_BASE: u64 = 2026111400;
const CALIBRATION_SEED: &str = "2026101601";
const IC_CAP: Duration = Duration::from_secs(900);
const RHO_CAP: Duration = Duration::from_secs(120);
const PRODUCER_CAP: Duration = Duration::from_secs(270 * 60);
const RSS_CAP_BYTES: u64 = 61 * 1024 * 1024 * 1024 / 4;
const K_VALUES: [usize; 2] = [1107, 2304];

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn fail(message: impl Into<String>) -> Box<dyn std::error::Error> {
    message.into().into()
}

fn sha(path: &Path) -> Result<String> {
    Ok(hex::encode(sha256(&fs::read(path)?)))
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn write_new(path: &Path, value: &Value) -> Result<()> {
    let mut stream = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut stream, value)?;
    stream.write_all(b"\n")?;
    stream.sync_all()?;
    Ok(())
}

fn read(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn read_lines(path: &Path) -> Result<Vec<String>> {
    let text = fs::read_to_string(path)?;
    if !text.ends_with('\n') {
        return Err(fail(format!("{} lacks final newline", path.display())));
    }
    let lines: Vec<String> = text.lines().map(str::to_owned).collect();
    if lines.iter().any(|line| line.is_empty()) {
        return Err(fail("fixture has empty line"));
    }
    Ok(lines)
}

fn system_receipt() -> Value {
    let cgroup_member = fs::read_to_string("/proc/self/cgroup")
        .ok()
        .and_then(|contents| {
            contents.lines().find_map(|line| {
                line.strip_prefix("0::")
                    .filter(|member| member.starts_with('/'))
                    .map(str::to_owned)
            })
        });
    let cgroup_root = cgroup_member
        .as_ref()
        .map(|member| Path::new("/sys/fs/cgroup").join(member.trim_start_matches('/')));
    let cgroup_file = |name: &str| {
        cgroup_root
            .as_ref()
            .and_then(|root| fs::read_to_string(root.join(name)).ok())
    };
    let file = |path: &str| fs::read_to_string(path).ok();
    json!({
        "cgroup_member":cgroup_member,
        "cpu_stat":cgroup_file("cpu.stat"),
        "cpu_max":cgroup_file("cpu.max"),
        "memory_max":cgroup_file("memory.max"),
        "memory_events":cgroup_file("memory.events"),
        "proc_stat":file("/proc/stat"),
        "self_status":file("/proc/self/status"),
        "pressure_cpu":file("/proc/pressure/cpu"),
        "pressure_memory":file("/proc/pressure/memory"),
    })
}

fn current_rss(pid: u32) -> Option<u64> {
    let text = fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    let line = text.lines().find(|line| line.starts_with("VmRSS:"))?;
    line.split_whitespace()
        .nth(1)?
        .parse::<u64>()
        .ok()
        .map(|kib| kib * 1024)
}

fn visible_cpu_count(status: &str) -> Option<usize> {
    let list = status
        .lines()
        .find(|line| line.starts_with("Cpus_allowed_list:"))?
        .split_once(':')?
        .1
        .trim();
    let mut count = 0usize;
    for part in list.split(',') {
        if let Some((start, end)) = part.split_once('-') {
            let start: usize = start.parse().ok()?;
            let end: usize = end.parse().ok()?;
            if end < start {
                return None;
            }
            count += end - start + 1;
        } else {
            part.parse::<usize>().ok()?;
            count += 1;
        }
    }
    Some(count)
}

fn capacity_preflight(system: &Value) -> Result<Value> {
    let status = system["self_status"]
        .as_str()
        .ok_or_else(|| fail("CPU affinity absent"))?;
    let cpus = visible_cpu_count(status).ok_or_else(|| fail("CPU affinity invalid"))?;
    let memory_text = fs::read_to_string("/proc/meminfo")?;
    let mem_total_kib: u64 = memory_text
        .lines()
        .find(|line| line.starts_with("MemTotal:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .ok_or_else(|| fail("host memory absent"))?
        .parse()?;
    let cgroup_max = system["memory_max"].as_str().map(str::trim);
    let cgroup_telemetry = cgroup_max.is_some()
        && system["cpu_max"].as_str().is_some()
        && system["cpu_stat"].as_str().is_some()
        && system["memory_events"].as_str().is_some();
    let cgroup_bytes = cgroup_max.and_then(|value| value.parse::<u64>().ok());
    let required = RSS_CAP_BYTES + 256 * 1024 * 1024;
    let capacity_pass = cpus >= 4
        && mem_total_kib * 1024 >= required
        && cgroup_bytes.is_none_or(|value| value >= RSS_CAP_BYTES);
    let status = if !cgroup_telemetry {
        "REJECTED_CGROUP_TELEMETRY"
    } else if !capacity_pass {
        "REJECTED_HOST_CAPACITY"
    } else {
        "PASS_CAPACITY_PREFLIGHT"
    };
    Ok(json!({"status":status,
              "visible_cpus":cpus,"mem_total_kib":mem_total_kib,
              "cgroup_memory_max":cgroup_max,"rss_cap_bytes":RSS_CAP_BYTES}))
}

fn run_child(
    folder: &Path,
    program: &Path,
    args: &[String],
    environment: &[(String, String)],
    cap: Duration,
) -> Result<Value> {
    fs::create_dir_all(folder)?;
    let stdout = File::create(folder.join("stdout.jsonl"))?;
    let stderr = File::create(folder.join("stderr.txt"))?;
    let mut command = Command::new(program);
    command
        .args(args)
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));
    for (key, value) in environment {
        command.env(key, value);
    }
    let flags: Map<String, Value> = environment
        .iter()
        .map(|(key, value)| (key.clone(), json!(value)))
        .collect();
    let before = system_receipt();
    let started_unix_ms = now_ms();
    let started = Instant::now();
    let child = command.spawn()?;
    let pid = child.id();
    write_new(
        &folder.join("launch.json"),
        &json!({
            "program":program,"args":args,"environment_flags":flags,
            "cap_seconds":cap.as_secs(),"pid":pid,"started_unix_ms":started_unix_ms,
        }),
    )?;
    let mut max_sampled_rss = 0u64;
    let mut timed_out = false;
    let mut rss_exceeded = false;
    let mut rusage: libc::rusage = unsafe { std::mem::zeroed() };
    let exit_code = loop {
        let mut status = 0;
        let waited =
            unsafe { libc::wait4(pid as libc::pid_t, &mut status, libc::WNOHANG, &mut rusage) };
        if waited == pid as libc::pid_t {
            break if libc::WIFEXITED(status) {
                libc::WEXITSTATUS(status)
            } else if libc::WIFSIGNALED(status) {
                -libc::WTERMSIG(status)
            } else {
                -255
            };
        }
        if waited < 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::EINTR) {
                continue;
            }
            return Err(error.into());
        }
        if let Some(value) = current_rss(pid) {
            max_sampled_rss = max_sampled_rss.max(value);
            if value > RSS_CAP_BYTES {
                rss_exceeded = true;
            }
        }
        if started.elapsed() > cap {
            timed_out = true;
        }
        if rss_exceeded || timed_out {
            unsafe {
                libc::kill(pid as libc::pid_t, libc::SIGKILL);
            }
        }
        thread::sleep(Duration::from_millis(50));
    };
    let wall_ms = started.elapsed().as_secs_f64() * 1000.0;
    #[cfg(target_os = "linux")]
    let child_peak_rss = (rusage.ru_maxrss as u64).saturating_mul(1024);
    #[cfg(target_os = "macos")]
    let child_peak_rss = rusage.ru_maxrss as u64;
    let peak = max_sampled_rss.max(child_peak_rss);
    let status = if rss_exceeded || peak > RSS_CAP_BYTES {
        "RSS_CAP"
    } else if timed_out {
        "TIMEOUT"
    } else if exit_code == 0 {
        "PASS_PRODUCER"
    } else {
        "PRODUCER_FAILED"
    };
    let value = json!({
        "schema":"n83.native-bridge-child-usage/v1","status":status,
        "exit_code":exit_code,"wall_ms":wall_ms,"peak_rss_bytes":peak,
        "rss_cap_bytes":RSS_CAP_BYTES,"started_unix_ms":started_unix_ms,
        "finished_unix_ms":now_ms(),"system_before":before,
        "system_after":system_receipt(),
    });
    write_new(&folder.join("usage.json"), &value)?;
    Ok(value)
}

fn ic_environment(folder: &Path) -> Vec<(String, String)> {
    let flags = [
        ("KIC_PACKED_ROOT_TABLE", "1"),
        ("KIC_PACKED_ROOT_TABLE_DENSE", "0"),
        ("KIC_PACKED_ROOT_TABLE_LOAD75", "0"),
        ("KIC_STATE28", "0"),
        ("KIC_STATE26", "0"),
        ("KIC_ROOT_ONLY22", "0"),
        ("KIC_ROOT_ONLY21", "0"),
        ("KIC_ROOT_ONLY11", "1"),
        ("KIC_INDEXED_ROOT5", "1"),
        ("KIC_INDEXED_ROOT4", "0"),
        ("KIC_ROOT_BLOOM", "0"),
        ("KIC_SYMMETRIC_S3", "1"),
        ("KIC_INDEXED5_LOAD75", "1"),
        ("KIC_INDEXED5_FIXED30", "0"),
        ("KIC_INDEXED4_FIXED31", "0"),
        ("KIC_SPARSE_CANONICAL", "1"),
        ("KIC_RANK_THREADS", "4"),
        ("KIC_TARGET_THREADS", "2"),
        ("KIC_TARGET_CANCEL", "1"),
        ("KIC_INDEXED5_ID30", "0"),
    ];
    let mut result: Vec<(String, String)> = flags
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
    result.push((
        "KIC_DUMP_BASE".into(),
        folder.join("base_dump.jsonl").display().to_string(),
    ));
    result
}

fn execute(stratum: usize, fixture: &Path, ic: &Path, rho: &Path, output: &Path) -> Result<()> {
    if stratum >= 4 {
        return Err(fail("stratum must be 0..3"));
    }
    let protocol = read(&fixture.join("protocol.json"))?;
    let targets_path = fixture.join("targets.jsonl");
    let answers_path = fixture.join("answers.jsonl");
    let workloads_path = fixture.join("workloads.json");
    if protocol["schema"] != "n83.native-bridge-protocol/v1"
        || protocol["source_sha256"] != SOURCE_SHA256
        || protocol["targets_sha256"] != sha(&targets_path)?
        || protocol["answers_sha256"] != sha(&answers_path)?
        || protocol["workloads_sha256"] != sha(&workloads_path)?
        || sha(Path::new("examples/koblitz_orbit_dlp_fast.rs"))? != SOURCE_SHA256
    {
        return Err(fail("frozen protocol, source or fixtures differ"));
    }
    let targets = read_lines(&targets_path)?;
    let answers = read_lines(&answers_path)?;
    if targets.len() != 32 || answers.len() != 32 {
        return Err(fail("32 one-point fixtures required"));
    }
    let calibration = Path::new("research/n83_cold_factor_base_20261016/targets.jsonl");
    let rho_config = Path::new("research/n83_dense_root_20261013/rho_config.json");
    if protocol["calibration_target_sha256"] != sha(calibration)?
        || protocol["rho_config_sha256"] != sha(rho_config)?
    {
        return Err(fail("calibration target or rho configuration differs"));
    }
    fs::create_dir(output)?;
    let bins = output.join("executed_bins");
    fs::create_dir(&bins)?;
    let ic_copy = bins.join("koblitz_orbit_dlp_fast");
    let rho_copy = bins.join("n83_rho_optimization_eval");
    fs::copy(ic, &ic_copy)?;
    fs::copy(rho, &rho_copy)?;
    let git = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    if !git.status.success() {
        return Err(fail("git revision unavailable"));
    }
    let lscpu = Command::new("lscpu").arg("-J").output()?;
    if !lscpu.status.success() {
        return Err(fail("cloud CPU topology unavailable"));
    }
    let host = json!({
        "schema":"n83.native-bridge-host/v1","stratum":stratum,
        "git_sha":String::from_utf8(git.stdout)?.trim(),
        "runner_name":std::env::var("RUNNER_NAME").ok(),
        "github_run_id":std::env::var("GITHUB_RUN_ID").ok(),
        "github_run_attempt":std::env::var("GITHUB_RUN_ATTEMPT").ok(),
        "lscpu_json":String::from_utf8(lscpu.stdout)?,
        "system_before":system_receipt(),"recorded_unix_ms":now_ms(),
    });
    write_new(&output.join("host.json"), &host)?;
    let preflight = capacity_preflight(&host["system_before"])?;
    write_new(&output.join("preflight.json"), &preflight)?;
    if preflight["status"] != "PASS_CAPACITY_PREFLIGHT" {
        return Err(fail("host capacity preflight rejected"));
    }
    write_new(
        &output.join("source.json"),
        &json!({
            "schema":"n83.native-bridge-source/v1","source_sha256":SOURCE_SHA256,
            "ic_binary_sha256":sha(&ic_copy)?,"rho_binary_sha256":sha(&rho_copy)?,
            "fixture_sha256":protocol["targets_sha256"],"stratum":stratum,
            "ordered_k":K_VALUES,"rho_seed_base":PAIR_RHO_SEED_BASE,
        }),
    )?;
    let started = Instant::now();
    let mut events = Vec::new();
    for index in stratum * POINTS_PER_STRATUM..(stratum + 1) * POINTS_PER_STRATUM {
        let point_folder = output.join("points").join(format!("p{index:02}"));
        fs::create_dir_all(&point_folder)?;
        let point_path = point_folder.join("target.jsonl");
        fs::write(&point_path, format!("{}\n", targets[index]))?;
        let answer: Value = serde_json::from_str(&answers[index])?;
        if answer["fixture_index"] != index
            || answer["point"] != serde_json::from_str::<Value>(&targets[index])?
        {
            return Err(fail(format!("point {index} fixture sidecar differs")));
        }
        for order in 0..2 {
            let fixed = (index + order) % 2;
            let k = K_VALUES[fixed];
            let candidate = format!("k{k}root11");
            let kinds = [
                "cal_before_0",
                "cal_before_1",
                "cal_before_2",
                if (index + fixed) % 2 == 0 {
                    "ic"
                } else {
                    "rho"
                },
                if (index + fixed) % 2 == 0 {
                    "rho"
                } else {
                    "ic"
                },
                "cal_after_0",
                "cal_after_1",
                "cal_after_2",
            ];
            for kind in kinds {
                if started.elapsed() + IC_CAP + Duration::from_secs(30) > PRODUCER_CAP {
                    write_new(
                        &output.join("producer_stop.json"),
                        &json!({
                            "status":"GLOBAL_CAP_BEFORE_NEXT_EVENT","point_index":index,
                            "candidate":candidate,"kind":kind,
                        }),
                    )?;
                    write_new(&output.join("events.json"), &json!(events))?;
                    return Ok(());
                }
                let name = format!("{kind}_{candidate}");
                let folder = point_folder.join("runs").join(&name);
                let (program, args, env, cap) = if kind == "ic" {
                    (
                        ic_copy.as_path(),
                        vec![
                            format!("construct:83:1:{k}"),
                            point_path.display().to_string(),
                            "29".into(),
                            folder.join("targets_result.jsonl").display().to_string(),
                        ],
                        ic_environment(&folder),
                        IC_CAP,
                    )
                } else {
                    let is_cal = kind.starts_with("cal_");
                    let target = if is_cal {
                        calibration
                    } else {
                        point_path.as_path()
                    };
                    let seed = if is_cal {
                        CALIBRATION_SEED.to_string()
                    } else {
                        (PAIR_RHO_SEED_BASE + 2 * index as u64 + fixed as u64).to_string()
                    };
                    (
                        rho_copy.as_path(),
                        vec![
                            target.display().to_string(),
                            seed,
                            rho_config.display().to_string(),
                            folder.join("receipt.json").display().to_string(),
                        ],
                        Vec::new(),
                        RHO_CAP,
                    )
                };
                let usage = run_child(&folder, program, &args, &env, cap)?;
                events.push(json!({"point_index":index,"k":k,"kind":kind,
                                   "name":name,"usage":usage["status"]}));
                println!(
                    "p{index:02} {name} {} {:.3} ms",
                    usage["status"],
                    usage["wall_ms"].as_f64().unwrap_or(0.0)
                );
            }
        }
    }
    write_new(&output.join("events.json"), &json!(events))?;
    write_new(
        &output.join("host_after.json"),
        &json!({
            "system_after":system_receipt(),"recorded_unix_ms":now_ms(),
        }),
    )?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 6 {
        return Err(fail(
            "usage: n83_bridge_run <stratum> <fixture-dir> <IC> <rho> <output-dir>",
        ));
    }
    execute(
        args[1].parse()?,
        Path::new(&args[2]),
        Path::new(&args[3]),
        Path::new(&args[4]),
        Path::new(&args[5]),
    )
}
