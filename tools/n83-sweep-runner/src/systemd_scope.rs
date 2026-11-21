//! Synchronize only this Docker scope before releasing the native worker.
use super::{hash, number, read, require, text, write, R};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub const BARRIER: &str =
    "while [ ! -f /out/RESOURCE_READY ]; do /bin/sleep 0.02; done; exec \"$@\"";

pub fn unit(id: &str) -> R<String> {
    require(
        id.len() == 64
            && id
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "invalid owned Docker scope ID",
    )?;
    Ok(format!("docker-{id}.scope"))
}
fn property(name: &str, key: &str) -> R<String> {
    command(&["show", name, "--property", key, "--value"])
}
fn command(args: &[&str]) -> R<String> {
    let out = Command::new("/usr/bin/timeout")
        .args(["--kill-after=2s", "15s", "/usr/bin/systemctl"])
        .args(args)
        .output()?;
    require(
        out.status.success(),
        &format!(
            "owned-scope systemctl failed: status={:?}, stderr={}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        ),
    )?;
    Ok(String::from_utf8(out.stdout)?.trim().to_owned())
}
pub fn dropin(name: &str) -> R<PathBuf> {
    let id = name
        .strip_prefix("docker-")
        .and_then(|v| v.strip_suffix(".scope"))
        .ok_or("invalid scope")?;
    require(unit(id)? == name, "invalid scope name")?;
    Ok(Path::new("/run/systemd/transient").join(format!("{name}.d/50-MemorySwapMax.conf")))
}
fn kernel(path: &Path, memory: u64, cpus: &str) -> R<Value> {
    let get =
        |key: &str| -> R<String> { Ok(fs::read_to_string(path.join(key))?.trim().to_owned()) };
    let max = get("memory.max")?;
    let swap = get("memory.swap.max")?;
    let cpu = get("cpu.max")?;
    let affinity = get("cpuset.cpus")?;
    require(
        max == memory.to_string(),
        "kernel hard memory limit differs",
    )?;
    if !cpus.is_empty() {
        require(affinity == cpus, "kernel cpuset differs")?;
    }
    let parts: Vec<_> = cpu.split_whitespace().collect();
    require(
        parts.len() == 2
            && parts[0].parse::<u64>()? == parts[1].parse::<u64>()?
            && parts[1].parse::<u64>()? > 0,
        "kernel CPU quota differs from one CPU",
    )?;
    Ok(json!({"memory_max":max,"memory_swap_max":swap,"cpu_max":cpu,"cpuset_cpus":affinity}))
}
pub fn prepare_and_release(dir: &Path, live: &Value, memory: u64) -> R<String> {
    require(
        !dir.join("RESOURCE_READY").exists() && !dir.join("systemd-swap-sync.json").exists(),
        "startup synchronization already recorded or worker released; refusing reconfiguration",
    )?;
    require(
        unsafe { libc::geteuid() } == 0,
        "scope synchronization requires root",
    )?;
    require(
        [4u64 << 30, 8u64 << 30].contains(&memory),
        "unsupported memory envelope",
    )?;
    let name = unit(text(live, "id")?)?;
    let pid = number(&live["state"], "Pid")?;
    let member = super::unified_cgroup_path(&fs::read_to_string(format!("/proc/{pid}/cgroup"))?)?;
    let expected = Path::new("/sys/fs/cgroup/system.slice").join(&name);
    require(
        member == expected,
        "live Docker PID does not belong to owned scope",
    )?;
    let cg = property(&name, "ControlGroup")?;
    require(
        cg == format!("/system.slice/{name}"),
        "systemd control group differs",
    )?;
    let cpus = live["host"]["CpusetCpus"].as_str().unwrap_or("");
    let before = kernel(&member, memory, cpus)?;
    let before_property = property(&name, "MemorySwapMax")?;
    command(&["set-property", "--runtime", &name, "MemorySwapMax=0"])?;
    let after_property = property(&name, "MemorySwapMax")?;
    let after = kernel(&member, memory, cpus)?;
    require(
        after_property == "0" && after["memory_swap_max"] == "0",
        "scope swap synchronization failed",
    )?;
    let file = dropin(&name)?;
    let contents = fs::read_to_string(&file)?;
    require(
        contents.lines().any(|line| line == "MemorySwapMax=0"),
        "missing persistent runtime swap property",
    )?;
    write(
        dir.join("systemd-swap-sync.json"),
        &json!({"schema":"n83.systemd-swap-sync/v1",
        "unit":name,"docker_id":live["id"],"pid":pid,"control_group":cg,
        "before_property":before_property,"after_property":after_property,
        "kernel_before":before,"kernel_after":after,"dropin_path":file,
        "dropin_contents":contents,"memory_limit_bytes":memory,"worker_held_until_sync":true}),
    )?;
    write(
        dir.join("RESOURCE_READY"),
        &json!({"schema":"n83.resource-ready/v1","worker_may_start":true,
        "systemd_swap_sync_blake3":hash(dir.join("systemd-swap-sync.json"))?}),
    )?;
    Ok(name)
}
pub fn cleanup(name: &str) -> R<()> {
    let file = dropin(name)?;
    match fs::read_to_string(&file) {
        Ok(value) => {
            require(
                value.lines().any(|line| line == "MemorySwapMax=0"),
                "unexpected owned swap drop-in contents",
            )?;
            match fs::remove_file(&file) {
                Ok(()) => (),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                Err(e) => return Err(e.into()),
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => return Err(e.into()),
    }
    require(!file.exists(), "owned runtime swap property remains")
}
pub fn audit(dir: &Path, cfg: &Value, outer: &Value) -> R<()> {
    require(
        cfg["systemd_swap_sync"] == true,
        "missing startup synchronization policy",
    )?;
    require(
        outer["systemd_runtime_cleanup_verified"] == true,
        "runtime property cleanup unverified",
    )?;
    require(
        hash(dir.join("systemd-swap-sync.json"))? == text(outer, "systemd_swap_sync_blake3")?
            && hash(dir.join("RESOURCE_READY"))? == text(outer, "resource_ready_blake3")?,
        "startup barrier custody mismatch",
    )?;
    let sync = read(dir.join("systemd-swap-sync.json"))?;
    let ready = read(dir.join("RESOURCE_READY"))?;
    let live = read(dir.join("inspect-live.json"))?;
    require(
        sync["schema"] == "n83.systemd-swap-sync/v1"
            && sync["worker_held_until_sync"] == true
            && sync["unit"] == unit(text(&live, "id")?)?
            && sync["pid"] == live["state"]["Pid"]
            && sync["after_property"] == "0"
            && sync["kernel_after"]["memory_swap_max"] == "0"
            && sync["memory_limit_bytes"] == super::memory_limit(cfg)?
            && sync["kernel_after"]["memory_max"] == super::memory_limit(cfg)?.to_string(),
        "startup resource policy mismatch",
    )?;
    require(
        ready["schema"] == "n83.resource-ready/v1"
            && ready["worker_may_start"] == true
            && ready["systemd_swap_sync_blake3"] == hash(dir.join("systemd-swap-sync.json"))?,
        "worker release marker mismatch",
    )
}

/// Non-cryptographic readiness probe using the exact startup barrier.
pub fn smoke(dir: &Path) -> R<()> {
    fs::create_dir(dir)?;
    let dir = fs::canonicalize(dir)?;
    let name = format!("n83-scope-smoke-{}", std::process::id());
    let mount = format!("type=bind,src={},dst=/out", dir.display());
    let mut scope = None;
    let result = (|| -> R<()> {
        super::docker(&[
            "run",
            "--detach",
            "--pull=never",
            "--name",
            &name,
            "--network=none",
            "--memory=8589934592",
            "--memory-swap=8589934592",
            "--cpus=1",
            "--cpuset-cpus=14",
            "--pids-limit=64",
            "--read-only",
            "--cap-drop=ALL",
            "--security-opt=no-new-privileges",
            "--mount",
            &mount,
            "sha256:541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63",
            "/bin/sh",
            "-c",
            BARRIER,
            "n83-resource-barrier",
            "/usr/bin/timeout",
            "--signal=KILL",
            "5s",
            "/bin/sh",
            "-c",
            "/bin/sleep 2; /bin/echo N83_WORKER_RELEASED",
        ])?;
        let live = super::inspect(&name)?;
        write(dir.join("inspect-held.json"), &live)?;
        scope = Some(unit(text(&live, "id")?)?);
        let logs = super::docker(&["logs", &name])?;
        require(
            logs.stdout.is_empty()
                && logs.stderr.is_empty()
                && live["state"]["Running"] == true
                && !dir.join("RESOURCE_READY").exists(),
            "probe worker escaped startup barrier",
        )?;
        prepare_and_release(&dir, &live, 8u64 << 30)?;
        let scope = scope.as_deref().ok_or("missing smoke scope")?;
        command(&["set-property", "--runtime", scope, "CPUWeight=100"])?;
        let path = Path::new("/sys/fs/cgroup/system.slice").join(scope);
        let reapplied = kernel(&path, 8u64 << 30, "14")?;
        require(
            reapplied["memory_swap_max"] == "0" && property(scope, "MemorySwapMax")? == "0",
            "synced scope lost swap limit on resource reapply",
        )?;
        write(dir.join("reapplied-resources.json"), &reapplied)?;
        let waited = super::docker(&["wait", &name])?;
        require(
            String::from_utf8(waited.stdout)?.trim() == "0",
            "probe worker failed",
        )?;
        let logs = super::docker(&["logs", &name])?;
        require(
            logs.stdout == b"N83_WORKER_RELEASED\n" && logs.stderr.is_empty(),
            "probe release output differs",
        )?;
        fs::write(dir.join("worker.stdout"), logs.stdout)?;
        super::docker(&["rm", &name])?;
        cleanup(scope)?;
        write(
            dir.join("result.json"),
            &json!({"status":"PASS_STARTUP_BARRIER_SCOPE_SYNC_REAPPLY_AND_CLEANUP",
            "worker_held_before_release":true,"resource_reapply_keeps_swap_zero":true,"unit":scope,
            "scope":"non-cryptographic readiness only; no SAT result or performance claim"}),
        )
    })();
    if result.is_err() {
        let _ = super::docker(&["rm", "--force", &name]);
        if let Some(scope) = scope {
            let _ = cleanup(&scope);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scope_names_cannot_select_other_units_or_paths() {
        let id = "a".repeat(64);
        let name = unit(&id).unwrap();
        assert_eq!(name, format!("docker-{id}.scope"));
        assert!(dropin(&name).unwrap().starts_with("/run/systemd/transient"));
        for bad in [
            "",
            "docker.service",
            "../../peer.scope",
            "ABCD",
            &"a".repeat(63),
        ] {
            assert!(unit(bad).is_err());
            assert!(dropin(bad).is_err());
        }
    }
    #[test]
    fn released_worker_refuses_reconfiguration_before_live_metadata_or_commands() {
        let dir = std::env::temp_dir().join(format!(
            "n83-released-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("RESOURCE_READY"), b"already released").unwrap();
        let error = prepare_and_release(&dir, &Value::Null, 8u64 << 30)
            .unwrap_err()
            .to_string();
        assert!(error.contains("refusing reconfiguration"));
        assert_eq!(
            fs::read(dir.join("RESOURCE_READY")).unwrap(),
            b"already released"
        );
        assert!(!dir.join("systemd-swap-sync.json").exists());
        fs::remove_file(dir.join("RESOURCE_READY")).unwrap();
        fs::remove_dir(dir).unwrap();
    }
}
