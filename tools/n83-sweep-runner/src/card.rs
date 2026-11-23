//! Strict native frozen-command subset of the canonical card format.
//! Unsupported retries, environment overrides and runbooks are refused.
use super::{number, read, require, text, write, R};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    os::unix::process::CommandExt,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
pub(super) fn sha(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}
fn file_sha(p: &Path) -> R<String> {
    Ok(sha(&fs::read(p)?))
}
fn relative(s: &str) -> R<PathBuf> {
    let p = Path::new(s);
    require(
        !p.is_absolute()
            && !s.is_empty()
            && p.components()
                .all(|c| matches!(c, Component::Normal(_) | Component::CurDir)),
        "card path must be repository-relative without parent traversal",
    )?;
    Ok(p.to_owned())
}
fn scoped(c: &Value, s: &str) -> R<PathBuf> {
    let p = relative(s)?;
    require(
        c["write_scope"]
            .as_array()
            .ok_or("scope array")?
            .iter()
            .any(|v| {
                v.as_str()
                    .is_some_and(|v| relative(v).is_ok_and(|q| p.starts_with(q)))
            }),
        "card output outside scope",
    )?;
    Ok(p)
}
fn git(args: &[&str]) -> R<String> {
    let cwd = std::env::current_dir()?;
    let o = Command::new("git")
        .args(["-c", &format!("safe.directory={}", cwd.display())])
        .args(args)
        .output()?;
    require(o.status.success(), "card git failed")?;
    Ok(String::from_utf8(o.stdout)?.trim().to_owned())
}
fn bind(c: &Value) -> R<()> {
    require(
        text(c, "commit")? == git(&["rev-parse", "HEAD"])?,
        "card commit differs from HEAD",
    )?;
    for b in c["binds"].as_array().ok_or("binds array")? {
        let p = Path::new(text(b, "path")?);
        require(file_sha(p)? == text(b, "sha256")?, "bound SHA256 differs")?;
    }
    Ok(())
}
fn lint(c: &Value) -> R<()> {
    if let Some(p) = c.get("native_path") {
        require(
            p.as_str() == std::env::var("PATH").ok().as_deref(),
            "PATH differs from frozen native wrapper environment",
        )?;
    }
    for k in [
        "schema",
        "card_id",
        "experiment_id",
        "author_role",
        "executor_role",
        "kind",
        "bus_address",
        "background",
        "write_scope",
        "stop_when",
    ] {
        require(c.get(k).is_some(), "missing canonical card field")?;
    }
    require(
        c["schema"] == "crypto.autoresearch.mechanical-task-card.v1"
            && c["kind"] == "frozen_command"
            && c["author_role"] == "experiment-director"
            && c["executor_role"] == "executor-mechanical",
        "unsupported card identity/kind/role",
    )?;
    require(
        text(c, "card_id")?.starts_with("CARD-") && !text(c, "experiment_id")?.starts_with("EXP-"),
        "native subset requires CARD id and non-EXP diagnostic id",
    )?;
    require(
        c["background"].is_boolean() && c["stop_when"].as_array().is_some_and(|x| !x.is_empty()),
        "background/stop conditions missing",
    )?;
    for s in c["write_scope"].as_array().ok_or("scope array")? {
        relative(s.as_str().ok_or("scope string")?)?;
    }
    scoped(c, text(c, "receipt")?)?;
    if let Some(relay) = c.get("relay_stdout") {
        scoped(c, relay.as_str().ok_or("relay_stdout string")?)?;
    }
    bind(c)?;
    let cmds = c["commands"].as_array().ok_or("commands array")?;
    require(!cmds.is_empty(), "empty commands")?;
    for cmd in cmds {
        require(
            cmd.get("retry").is_none() && (cmd.get("cwd").is_none() || cmd["cwd"] == "."),
            "native subset refuses env/retry/non-root cwd",
        )?;
        if let Some(env) = cmd.get("env") {
            require(
                env.as_object().is_some_and(|m| {
                    m.iter().all(|(k, v)| {
                        ["N83_CHAIN_PHASE_TRACE", "RAYON_NUM_THREADS"].contains(&k.as_str())
                            && v == "1"
                    })
                }),
                "native subset refuses non-whitelisted environment",
            )?;
        }
        require(
            number(cmd, "timeout_seconds")? > 0,
            "invalid command timeout",
        )?;
        require(
            cmd["success_exit_codes"]
                .as_array()
                .is_some_and(|a| !a.is_empty() && a.iter().all(Value::is_i64)),
            "success codes invalid",
        )?;
        let argv = cmd["argv"].as_array().ok_or("argv array")?;
        require(!argv.is_empty(), "argv empty")?;
        for a in argv {
            let a = a.as_str().ok_or("argv string")?;
            require(
                !a.is_empty()
                    && !a.contains(['{', '}', '<', '>'])
                    && !["|", "||", "&&", ";", ">", ">>", "<", "&", "2>&1", "$("].contains(&a),
                "shell syntax or placeholder",
            )?;
            if Path::new(a).is_file() {
                require(
                    c["binds"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|b| b["path"] == a),
                    "argv file lacks SHA256 bind",
                )?;
            }
        }
        for stream in ["stdout", "stderr"] {
            scoped(c, text(cmd, stream)?)?;
        }
        for p in cmd["outputs"].as_array().ok_or("outputs array")? {
            scoped(c, p.as_str().ok_or("output string")?)?;
        }
    }
    Ok(())
}
fn tail(bytes: &[u8]) -> Vec<String> {
    let lines: Vec<_> = String::from_utf8_lossy(bytes)
        .lines()
        .map(str::to_owned)
        .collect();
    lines[lines.len().saturating_sub(20)..].to_vec()
}
fn report(c: &Value) -> R<String> {
    let raw = fs::read_to_string(text(c, "receipt")?)?;
    let r: Value = serde_json::from_str(&raw)?;
    require(r["card_id"] == c["card_id"], "receipt card mismatch")?;
    // The originally written receipt is the relay authority. Reparsing and
    // serializing floats can change their spelling, even for an exact relay.
    Ok(raw.strip_suffix('\n').unwrap_or(&raw).to_owned())
}
pub fn dispatch(mode: &str, path: &Path) -> R<()> {
    let original_bytes = fs::read(path)?;
    let original_sha = sha(&original_bytes);
    let c: Value = serde_json::from_slice(&original_bytes)?;
    if mode == "card-report" {
        println!("{}", report(&c)?);
        return Ok(());
    }
    lint(&c)?;
    if mode == "card-prompt" {
        let quote = |s: &str| format!("'{}'", s.replace('\'', "'\\''"));
        let mut argv = vec![
            "env".to_owned(),
            format!("PATH={}", std::env::var("PATH")?),
            std::env::current_exe()?
                .to_str()
                .ok_or("executable UTF8")?
                .to_owned(),
            "card-run".into(),
            path.to_str().ok_or("card UTF8")?.to_owned(),
        ];
        if c["root_executor"] == true {
            require(
                unsafe { libc::geteuid() } == 0,
                "root-executor card prompt must be validated as root",
            )?;
            argv.splice(0..0, ["/usr/bin/sudo".to_owned(), "-n".to_owned()]);
        }
        let command = argv.iter().map(|s| quote(s)).collect::<Vec<_>>().join(" ");
        if let Some(relay) = c.get("relay_stdout") {
            let relay = relay.as_str().ok_or("relay_stdout string")?;
            println!("Act as executor-mechanical. From /home/ec2-user/crypto-n83-resume execute exactly the shell command below using an elevated exec tool for Docker access. Do not modify the card, compute results, retry, or run Python. If the tool yields a running session, wait for that session. The frozen shell redirection saves actual stdout verbatim as the relay; do not regenerate it. Return the relay path and its SHA256 only. The command enforces frozen deadlines and writes the receipt. Read no unrelated files.\n\nset -o noclobber\n{} > {}\n\nCard SHA256: {}", command, quote(relay), original_sha);
        } else {
            println!("Act as executor-mechanical. From /home/ec2-user/crypto-n83-resume execute exactly the shell command below using an elevated exec tool for Docker access. Do not modify the card, compute results, retry, or run Python. Return final stdout verbatim, without prose or code fences. The repository native-only rule requires this native replacement for tools/task_card.py. If the tool yields a running session, wait for that session. The command enforces the frozen deadlines and writes the receipt. Read no unrelated files.\n\n{}\n\nCard SHA256: {}",command,original_sha);
        }
        return Ok(());
    }
    require(mode == "card-run", "unknown card mode")?;
    let receipt = scoped(&c, text(&c, "receipt")?)?;
    require(!receipt.exists(), "receipt already exists")?;
    for p in c["must_not_exist"].as_array().ok_or("fresh paths array")? {
        let p = scoped(&c, p.as_str().ok_or("fresh string")?)?;
        require(!p.exists(), "fresh card path exists")?;
    }
    let dirty_before = git(&["status", "--porcelain"])?;
    let mut results = Vec::new();
    let mut status = "completed".to_owned();
    for (index, cmd) in c["commands"].as_array().unwrap().iter().enumerate() {
        let attempt = (|| -> R<Value> {
            require(
                file_sha(path)? == original_sha,
                "card changed after initial load",
            )?;
            bind(&c)?;
            let out_path = scoped(&c, text(cmd, "stdout")?)?;
            let err_path = scoped(&c, text(cmd, "stderr")?)?;
            for p in [&out_path, &err_path] {
                fs::create_dir_all(p.parent().ok_or("stream parent")?)?;
            }
            let out = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&out_path)?;
            let err = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&err_path)?;
            let args: Vec<_> = cmd["argv"]
                .as_array()
                .unwrap()
                .iter()
                .map(|a| a.as_str().unwrap())
                .collect();
            let started = Instant::now();
            let mut launch = Command::new(args[0]);
            launch.args(&args[1..]);
            if let Some(env) = cmd.get("env") {
                for (k, v) in env.as_object().unwrap() {
                    launch.env(k, v.as_str().unwrap());
                }
            }
            let mut child = launch
                .stdout(Stdio::from(out))
                .stderr(Stdio::from(err))
                .process_group(0)
                .spawn()?;
            let mut timed_out = false;
            let exit = loop {
                if let Some(s) = child.try_wait()? {
                    break s;
                }
                if started.elapsed() >= Duration::from_secs(number(cmd, "timeout_seconds")?) {
                    timed_out = true;
                    let group = format!("-{}", child.id());
                    let _ = Command::new("kill").args(["-TERM", "--", &group]).status();
                    let stop = Instant::now();
                    let stopped = loop {
                        if let Some(s) = child.try_wait()? {
                            break s;
                        }
                        if stop.elapsed() > Duration::from_secs(2) {
                            let _ = Command::new("kill").args(["-KILL", "--", &group]).status();
                            break child.wait()?;
                        }
                        thread::sleep(Duration::from_millis(25));
                    };
                    break stopped;
                }
                thread::sleep(Duration::from_millis(25));
            };
            let stdout = fs::read(&out_path)?;
            let stderr = fs::read(&err_path)?;
            let mut outputs = serde_json::Map::new();
            for p in cmd["outputs"].as_array().unwrap() {
                let p = p.as_str().unwrap();
                outputs.insert(
                    p.to_owned(),
                    if Path::new(p).is_file() {
                        json!(file_sha(Path::new(p))?)
                    } else {
                        Value::Null
                    },
                );
            }
            let ok = !timed_out
                && cmd["success_exit_codes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|c| c.as_i64() == exit.code().map(i64::from))
                && outputs.values().all(|v| !v.is_null());
            let elapsed = started.elapsed().as_secs_f64();
            Ok(
                json!({"index":index+1,"id":cmd.get("id"),"cwd":".","argv":cmd["argv"],"exit_code":exit.code(),"attempts":1,"attempt_log":[{"exit_code":exit.code(),"timed_out":timed_out,"seconds":elapsed}],"timed_out":timed_out,"seconds":elapsed,"succeeded":ok,"stdout_sha256":sha(&stdout),"stderr_sha256":sha(&stderr),"stdout_tail":tail(&stdout),"stderr_tail":tail(&stderr),"outputs":outputs}),
            )
        })();
        let result=attempt.unwrap_or_else(|e|json!({"index":index+1,"id":cmd.get("id"),"argv":cmd["argv"],"exit_code":null,"succeeded":false,"execution_error":e.to_string(),"stdout_sha256":cmd["stdout"].as_str().and_then(|p|file_sha(Path::new(p)).ok()),"stderr_sha256":cmd["stderr"].as_str().and_then(|p|file_sha(Path::new(p)).ok()),"outputs":{}}));
        let ok = result["succeeded"] == true;
        results.push(result);
        if !ok {
            status = format!("stopped at command {}", index + 1);
            break;
        }
    }
    let card_changed = file_sha(path).ok().as_deref() != Some(original_sha.as_str());
    if card_changed {
        status = "stopped: card changed".into();
    }
    let r = json!({"schema":"crypto.autoresearch.task-card-receipt.v1","native_executor":"n83-sweep-runner strict frozen-command subset/v1","card_id":c["card_id"],"card_path":path,"card_sha256":original_sha,"card_changed":card_changed,"commit":git(&["rev-parse","HEAD"] )?,"status":status,"dirty_before":dirty_before,"dirty_after":git(&["status","--porcelain"] )?,"commands_total":c["commands"].as_array().unwrap().len(),"commands":results});
    fs::create_dir_all(receipt.parent().ok_or("receipt parent")?)?;
    write(receipt, &r)?;
    println!("{}", serde_json::to_string_pretty(&r)?);
    if status != "completed" {
        use std::io::Write;
        std::io::stdout().flush()?;
        std::process::exit(3)
    }
    Ok(())
}
pub fn check(card: &Path, relay: &Path) -> R<()> {
    let c = read(card)?;
    bind(&c)?;
    let r = read(text(&c, "receipt")?)?;
    require(
        r["card_sha256"] == file_sha(card)? && r["card_changed"] == false,
        "relay card custody differs",
    )?;
    require(
        r["commit"] == c["commit"]
            && r["commands_total"] == c["commands"].as_array().ok_or("card commands")?.len(),
        "receipt commit/count differs",
    )?;
    require(
        fs::read_to_string(relay)?.trim() == report(&c)?.trim(),
        "relay differs from native receipt rendering",
    )?;
    for cmd in r["commands"].as_array().ok_or("receipt commands")? {
        let index = number(cmd, "index")? as usize - 1;
        let original = &c["commands"][index];
        require(
            cmd["argv"] == original["argv"],
            "receipt command differs from frozen argv",
        )?;
        for (field, hash_key) in [("stdout", "stdout_sha256"), ("stderr", "stderr_sha256")] {
            if let Some(expected) = cmd[hash_key].as_str() {
                require(
                    file_sha(Path::new(text(original, field)?))? == expected,
                    "relay stream custody differs",
                )?;
            }
        }
        for (p, h) in cmd["outputs"].as_object().ok_or("outputs map")? {
            if let Some(expected) = h.as_str() {
                require(
                    file_sha(Path::new(p))? == expected,
                    "relay output digest differs",
                )?;
            }
        }
    }
    println!(
        "{}",
        json!({"relay_verbatim":true,"record_digests_verified":true,"status":r["status"]})
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scope_escape_rejected() {
        assert!(relative("../outside").is_err());
        assert!(relative("/tmp/outside").is_err());
        let c = json!({"write_scope":["evidence/exec"]});
        assert!(scoped(&c, "evidence/exec/ok.json").is_ok());
        assert!(scoped(&c, "evidence/execution/no.json").is_err());
    }
    #[test]
    fn canonical_sha256() {
        assert_eq!(
            sha(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
