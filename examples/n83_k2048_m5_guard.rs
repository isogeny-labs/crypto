//! Native bounded host guard for the paired N83 K2048 SAT-model panel.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const IMAGE: &str = "sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31";
const BINARY_SHA256: &str = "5123ef012623a0f425f93a91979a918f7e2df369c1309352919a47e9eef52f06";
const CORPUS_SHA256: &str = "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd";
const U_SHA256: &str = "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a";
const D_SHA256: &str = "2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7";
const MEMORY_BYTES: u64 = 4 * 1024 * 1024 * 1024;
const MAX_SECONDS: u64 = 300;

fn must(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn command(program: &str, args: &[&str]) -> Result<Output> {
    Ok(Command::new(program).args(args).output()?)
}

fn success(program: &str, args: &[&str]) -> Result<Output> {
    let output = command(program, args)?;
    must(output.status.success(), "subprocess failed")?;
    Ok(output)
}

fn text(output: Output) -> Result<String> {
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn epoch_ms() -> Result<u128> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())
}

fn wait_bounded(mut child: Child, deadline: Instant) -> Result<(bool, Output)> {
    loop {
        if child.try_wait()?.is_some() {
            return Ok((false, child.wait_with_output()?));
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            return Ok((true, child.wait_with_output()?));
        }
        thread::sleep(Duration::from_millis(200));
    }
}

fn write_new(path: &Path, value: &Value) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!("{}", serde_json::to_string(value)?);
    Ok(())
}

fn run(
    arm: &str,
    fixture: u8,
    mode: &str,
    binary: &Path,
    object: &Path,
    corpus: &Path,
    output: &Path,
) -> Result<Value> {
    must(arm == "U" || arm == "D", "arm outside protocol")?;
    must(fixture <= 1, "fixture outside protocol")?;
    must(
        mode == "preflight" || mode == "search",
        "mode outside protocol",
    )?;
    let binary = binary.canonicalize()?;
    let object = object.canonicalize()?;
    let corpus = corpus.canonicalize()?;
    let output = output.canonicalize()?;
    let cell = format!("{}-{fixture}-{mode}", arm.to_ascii_lowercase());
    let worker_path = output.join(format!("{cell}-native.json"));
    let guard_path = output.join(format!("{cell}-native-guard.json"));
    must(
        !worker_path.exists() && !guard_path.exists(),
        "cell output already exists",
    )?;
    let binary_hash = sha256(&fs::read(&binary)?);
    let object_hash = sha256(&fs::read(&object)?);
    let corpus_hash = sha256(&fs::read(&corpus)?);
    must(binary_hash == BINARY_SHA256, "binary SHA-256 differs")?;
    must(
        object_hash == if arm == "U" { U_SHA256 } else { D_SHA256 },
        "object SHA-256 differs",
    )?;
    must(corpus_hash == CORPUS_SHA256, "corpus SHA-256 differs")?;
    let image = text(success(
        "docker",
        &["image", "inspect", IMAGE, "--format", "{{.Id}}"],
    )?)?;
    must(image == IMAGE, "Linux image ID differs")?;
    let name = format!("n83-k2048-m5-native-{cell}-{}", epoch_ms()?);
    let mounts = [
        format!("type=bind,src={},dst=/runner,readonly", binary.display()),
        format!("type=bind,src={},dst=/object,readonly", object.display()),
        format!("type=bind,src={},dst=/corpus,readonly", corpus.display()),
        format!("type=bind,src={},dst=/out", output.display()),
    ];
    let shell = "timeout 300 /runner \"$1\" /object /corpus \"$2\" \"$3\" \"/out/$4-native.json\"; status=$?; cat /sys/fs/cgroup/memory.peak > \"/out/$4-native-memory.peak\"; exit \"$status\"";
    let fixture_text = fixture.to_string();
    let mut args = vec![
        "run",
        "--detach",
        "--name",
        &name,
        "--cpus=1",
        "--memory=4g",
        "--memory-swap=4g",
        "--pids-limit=128",
        "--network=none",
        "--read-only",
        "--security-opt=no-new-privileges",
        "--tmpfs",
        "/tmp:rw,nosuid,nodev,size=64m",
    ];
    for mount in &mounts {
        args.extend(["--mount", mount.as_str()]);
    }
    args.extend([
        "--entrypoint",
        "sh",
        IMAGE,
        "-c",
        shell,
        "sh",
        arm,
        &fixture_text,
        mode,
        &cell,
    ]);
    let started_epoch_ms = epoch_ms()?;
    let started = Instant::now();
    let container_id = text(success("docker", &args)?)?;
    let waiter = Command::new("docker")
        .args(["wait", &name])
        .stdout(Stdio::piped())
        .spawn()?;
    let (timed_out, waited) =
        wait_bounded(waiter, started + Duration::from_secs(MAX_SECONDS + 30))?;
    if timed_out {
        let _ = command("docker", &["kill", &name]);
    }
    let logs = command("docker", &["logs", &name])?;
    fs::write(
        output.join(format!("{cell}-native.stdout.log")),
        logs.stdout,
    )?;
    fs::write(
        output.join(format!("{cell}-native.stderr.log")),
        logs.stderr,
    )?;
    let inspected = command("docker", &["inspect", &name])?;
    let state: Value = if inspected.status.success() {
        serde_json::from_slice::<Vec<Value>>(&inspected.stdout)?
            .into_iter()
            .next()
            .ok_or("container inspect empty")?
    } else {
        Value::Null
    };
    let peak = fs::read_to_string(output.join(format!("{cell}-native-memory.peak")))
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok());
    let exit_code = if waited.status.success() {
        String::from_utf8(waited.stdout)?.trim().parse::<i32>().ok()
    } else {
        None
    };
    let _ = command("docker", &["rm", "-f", &name]);
    let valid = exit_code == Some(0)
        && !timed_out
        && state["State"]["OOMKilled"] == false
        && state["HostConfig"]["Memory"] == MEMORY_BYTES
        && state["HostConfig"]["MemorySwap"] == MEMORY_BYTES
        && state["HostConfig"]["NanoCpus"] == 1_000_000_000_u64
        && state["HostConfig"]["NetworkMode"] == "none"
        && state["HostConfig"]["ReadonlyRootfs"] == true
        && peak.is_some_and(|value| value <= MEMORY_BYTES)
        && worker_path.is_file();
    Ok(json!({
        "schema":"n83.k2048-m5-native-guard/v1",
        "cell":cell,"status":if valid {"PASS"} else {"FAILED_OR_CENSORED"},
        "started_epoch_ms":started_epoch_ms,"ended_epoch_ms":epoch_ms()?,
        "binary_sha256":binary_hash,"object_sha256":object_hash,"corpus_sha256":corpus_hash,
        "image_id":IMAGE,"container_id":container_id,
        "outer_seconds":started.elapsed().as_secs_f64(),
        "cpu_limit":1,"memory_limit_bytes":MEMORY_BYTES,"network":"none",
        "timeout_seconds":MAX_SECONDS,"outer_watchdog_fired":timed_out,
        "exit_code":exit_code,"oom_killed":state["State"]["OOMKilled"],
        "actual_memory_limit_bytes":state["HostConfig"]["Memory"],
        "actual_memory_swap_bytes":state["HostConfig"]["MemorySwap"],
        "actual_nano_cpus":state["HostConfig"]["NanoCpus"],
        "actual_network_mode":state["HostConfig"]["NetworkMode"],
        "actual_read_only_rootfs":state["HostConfig"]["ReadonlyRootfs"],
        "cgroup_memory_peak_bytes":peak,"worker_receipt_exists":worker_path.is_file(),
        "guard_source_blake3":blake3::hash(include_bytes!("n83_k2048_m5_guard.rs")).to_hex().to_string(),
    }))
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, arm, fixture, mode, binary, object, corpus, output] => fixture
            .parse::<u8>()
            .map_err(|_| "invalid fixture".into())
            .and_then(|fixture| {
                run(
                    arm,
                    fixture,
                    mode,
                    Path::new(binary),
                    Path::new(object),
                    Path::new(corpus),
                    Path::new(output),
                )
                .and_then(|value| {
                    write_new(
                        &Path::new(output).join(format!(
                            "{}-{fixture}-{mode}-native-guard.json",
                            arm.to_ascii_lowercase()
                        )),
                        &value,
                    )
                })
            }),
        _ => Err("usage: guard ARM FIXTURE MODE BINARY OBJECT CORPUS OUTPUT".into()),
    };
    if let Err(error) = result {
        eprintln!("n83 K2048 m5 native guard: {error}");
        std::process::exit(1);
    }
}
