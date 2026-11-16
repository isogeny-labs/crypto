//! Native host guard for the frozen primary N83 D18 K2048 exporter.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Output};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const IMAGE: &str = "sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31";
const SOURCE_MOUNT: &str = "/Volumes/SSD990/crypto";
const SOURCE_HEAD: &str = "a1e6b778dcee8f4ec72f359df7ce3728b302b78a";
const BINARY_SHA256: &str = "a0f9b7fe67ffea4816d077b8017e3ed4512cc36fd04689e3eae1257975831a5d";
const MEMORY_BYTES: u64 = 4 * 1024 * 1024 * 1024;
const MAX_SECONDS: u64 = 900;

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

fn run(mode: &str, binary: &Path, output: &Path, source: &Path, alternate: &Path) -> Result<Value> {
    must(
        mode == "export" || mode == "replay",
        "mode outside frozen protocol",
    )?;
    let binary = binary.canonicalize()?;
    let output = output.canonicalize()?;
    let source = source.canonicalize()?;
    let alternate = alternate.canonicalize()?;
    must(
        source.starts_with(SOURCE_MOUNT),
        "source checkout outside pinned mount",
    )?;
    must(
        !output.starts_with(&source),
        "output must be outside source checkout",
    )?;
    let base = output.join("base");
    if mode == "export" {
        must(!base.exists(), "export base already exists")?;
    } else {
        must(
            base.join("manifest.json").is_file(),
            "replay manifest absent",
        )?;
        must(
            !base.join("replay.json").exists(),
            "replay output already exists",
        )?;
    }
    let receipt = output.join(format!("{mode}-native-guard.json"));
    must(!receipt.exists(), "guard receipt already exists")?;
    let source_str = source.to_str().ok_or("source path encoding")?;
    let head = text(success("git", &["-C", source_str, "rev-parse", "HEAD"])?)?;
    must(
        head == SOURCE_HEAD,
        "source HEAD differs from frozen exporter",
    )?;
    let dirty = text(success(
        "git",
        &["-C", source_str, "status", "--porcelain"],
    )?)?;
    must(dirty.is_empty(), "source checkout is dirty")?;
    let actual_binary_hash = sha256(&fs::read(&binary)?);
    must(
        actual_binary_hash == BINARY_SHA256,
        "binary SHA-256 differs",
    )?;
    let image = text(success(
        "docker",
        &["image", "inspect", IMAGE, "--format", "{{.Id}}"],
    )?)?;
    must(image == IMAGE, "Linux image ID differs")?;
    let name = format!(
        "n83-d18-k2048-native-{mode}-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
    );
    let mounts = [
        format!("type=bind,src={SOURCE_MOUNT},dst={SOURCE_MOUNT},readonly"),
        format!(
            "type=bind,src={},dst={},readonly",
            alternate.display(),
            alternate.display()
        ),
        format!("type=bind,src={},dst=/runner,readonly", binary.display()),
        format!("type=bind,src={},dst=/out", output.display()),
    ];
    let shell = format!(
        "timeout {MAX_SECONDS} /runner \"$1\" /out/base; status=$?; cat /sys/fs/cgroup/memory.peak > /out/$1-native-memory.peak; exit \"$status\""
    );
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
        "--workdir",
        source_str,
        "--entrypoint",
        "sh",
        IMAGE,
        "-c",
        &shell,
        "sh",
        mode,
    ]);
    let started = Instant::now();
    let container_id = text(success("docker", &args)?)?;
    let waiter = Command::new("docker")
        .args(["wait", &name])
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    let (timed_out, waited) =
        wait_bounded(waiter, started + Duration::from_secs(MAX_SECONDS + 30))?;
    if timed_out {
        let _ = command("docker", &["kill", &name]);
    }
    let logs = command("docker", &["logs", &name])?;
    fs::write(
        output.join(format!("{mode}-native.stdout.log")),
        logs.stdout,
    )?;
    fs::write(
        output.join(format!("{mode}-native.stderr.log")),
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
    let peak = fs::read_to_string(output.join(format!("{mode}-native-memory.peak")))
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok());
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
        && peak.is_some_and(|value| value <= MEMORY_BYTES);
    Ok(json!({
        "schema":"n83.d18-k2048-native-guard/v1",
        "mode":mode,"status":if valid {"PASS"} else {"FAILED_OR_CENSORED"},
        "source_head_sha":head,"binary_sha256":actual_binary_hash,
        "image_id":IMAGE,"container_id":container_id,
        "outer_elapsed_seconds":started.elapsed().as_secs_f64(),
        "cpu_limit":1,"memory_limit_bytes":MEMORY_BYTES,"network":"none",
        "timeout_seconds":MAX_SECONDS,"outer_watchdog_fired":timed_out,
        "exit_code":exit_code,"oom_killed":state["State"]["OOMKilled"],
        "actual_memory_limit_bytes":state["HostConfig"]["Memory"],
        "actual_memory_swap_bytes":state["HostConfig"]["MemorySwap"],
        "actual_nano_cpus":state["HostConfig"]["NanoCpus"],
        "actual_network_mode":state["HostConfig"]["NetworkMode"],
        "actual_read_only_rootfs":state["HostConfig"]["ReadonlyRootfs"],
        "cgroup_memory_peak_bytes":peak,
        "verifier_source_blake3":blake3::hash(include_bytes!("n83_d18_k2048_guard.rs")).to_hex().to_string(),
    }))
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, mode, binary, output, source, alternate] => run(
            mode,
            Path::new(binary),
            Path::new(output),
            Path::new(source),
            Path::new(alternate),
        )
        .and_then(|value| {
            write_new(
                &Path::new(output).join(format!("{mode}-native-guard.json")),
                &value,
            )
        }),
        _ => Err("usage: guard MODE BINARY OUTPUT SOURCE ALTERNATE_GIT_OBJECTS".into()),
    };
    if let Err(error) = result {
        eprintln!("n83 K2048 native guard: {error}");
        std::process::exit(1);
    }
}
