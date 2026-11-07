//! Read-only cgroup-v2 evidence for a gated dataset worker.
//!
//! This is a diagnostic, not a proof of containment: a worker with permission
//! to write another cgroup's `cgroup.procs` can migrate an escaped descendant.
//! Label admission needs an isolated, non-writable cgroup and a trusted setup
//! record in addition to the observations here.

use serde_json::{json, Value};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path, PathBuf};

#[derive(Debug)]
pub(crate) struct Start {
    path: PathBuf,
    relative: String,
    inode: u64,
    shared_with_supervisor: bool,
    worker_pid: u32,
}

fn unified_path(contents: &str) -> Result<&str, String> {
    let rows: Vec<_> = contents
        .lines()
        .filter(|row| row.starts_with("0::"))
        .collect();
    if rows.len() != 1 {
        return Err("expected exactly one unified cgroup-v2 membership".into());
    }
    let path = rows[0].strip_prefix("0::").unwrap();
    if !path.starts_with('/') || path.contains('\0') || path.contains('\n') {
        return Err("invalid unified cgroup path".into());
    }
    let relative = Path::new(path)
        .strip_prefix("/")
        .map_err(|e| e.to_string())?;
    if relative
        .components()
        .any(|part| !matches!(part, Component::Normal(_)))
        && !relative.as_os_str().is_empty()
    {
        return Err("noncanonical unified cgroup path".into());
    }
    Ok(path)
}

fn members(contents: &str) -> Result<Vec<u32>, String> {
    contents
        .lines()
        .map(|line| {
            line.parse::<u32>()
                .ok()
                .filter(|pid| *pid > 0)
                .ok_or_else(|| "invalid cgroup.procs PID".to_string())
        })
        .collect()
}

fn populated(contents: &str) -> Result<bool, String> {
    let rows: Vec<_> = contents
        .lines()
        .filter_map(|line| line.strip_prefix("populated "))
        .collect();
    match rows.as_slice() {
        ["0"] => Ok(false),
        ["1"] => Ok(true),
        _ => Err("missing or invalid cgroup.events populated field".into()),
    }
}

impl Start {
    pub(crate) fn capture(worker_pid: u32) -> Result<Self, String> {
        if worker_pid == 0 {
            return Err("zero worker PID".into());
        }
        let worker =
            fs::read_to_string(format!("/proc/{worker_pid}/cgroup")).map_err(|e| e.to_string())?;
        let supervisor = fs::read_to_string("/proc/self/cgroup").map_err(|e| e.to_string())?;
        let relative = unified_path(&worker)?.to_string();
        let shared_with_supervisor = relative == unified_path(&supervisor)?;
        let path = Path::new("/sys/fs/cgroup").join(relative.trim_start_matches('/'));
        let inode = fs::metadata(&path).map_err(|e| e.to_string())?.ino();
        let pids =
            members(&fs::read_to_string(path.join("cgroup.procs")).map_err(|e| e.to_string())?)?;
        if !pids.contains(&worker_pid) {
            return Err("gate PID absent from its cgroup before release".into());
        }
        Ok(Self {
            path,
            relative,
            inode,
            shared_with_supervisor,
            worker_pid,
        })
    }

    pub(crate) fn finish(self) -> Value {
        let result = (|| -> Result<(bool, usize), String> {
            if fs::metadata(&self.path).map_err(|e| e.to_string())?.ino() != self.inode {
                return Err("cgroup inode changed during worker lifetime".into());
            }
            let populated = populated(
                &fs::read_to_string(self.path.join("cgroup.events")).map_err(|e| e.to_string())?,
            )?;
            let pids = members(
                &fs::read_to_string(self.path.join("cgroup.procs")).map_err(|e| e.to_string())?,
            )?;
            Ok((populated, pids.len()))
        })();
        match result {
            Ok((is_populated, direct_members)) => json!({
                "status": if self.shared_with_supervisor { "shared_cgroup" }
                    else if is_populated || direct_members != 0 { "members_remain" }
                    else { "empty_observed_no_migration_proof" },
                "worker_pid":self.worker_pid,"relative_path":self.relative,
                "cgroup_inode":self.inode,"shared_with_supervisor":self.shared_with_supervisor,
                "populated_after_exit":is_populated,"direct_members_after_exit":direct_members,
                "no_migration_proven":false,
            }),
            Err(error) => json!({"status":"observation_failed", "worker_pid":self.worker_pid,
                "relative_path":self.relative,"cgroup_inode":self.inode,
                "shared_with_supervisor":self.shared_with_supervisor,
                "populated_after_exit":null,"direct_members_after_exit":null,
                "no_migration_proven":false,"error":error}),
        }
    }
}

pub(crate) fn unavailable(worker_pid: u32, error: String) -> Value {
    json!({"status":"capture_failed","worker_pid":worker_pid,
        "no_migration_proven":false,"error":error})
}

pub(crate) fn verify_diagnostic(value: &Value, worker_pid: u64) -> Result<(), String> {
    if value["worker_pid"].as_u64() != Some(worker_pid) || value["no_migration_proven"] != false {
        return Err("cgroup diagnostic identity or scope invalid".into());
    }
    let status = value["status"]
        .as_str()
        .ok_or("cgroup diagnostic lacks status")?;
    match status {
        "capture_failed" => {
            if value["error"].as_str().is_none_or(str::is_empty) {
                return Err("cgroup capture failure lacks error".into());
            }
        }
        "observation_failed"
        | "shared_cgroup"
        | "members_remain"
        | "empty_observed_no_migration_proof" => {
            if value["relative_path"].as_str().is_none()
                || value["cgroup_inode"].as_u64().is_none_or(|n| n == 0)
            {
                return Err("cgroup diagnostic lacks capture identity".into());
            }
            if status == "observation_failed" {
                if value["error"].as_str().is_none_or(str::is_empty) {
                    return Err("cgroup observation failure lacks error".into());
                }
            } else {
                let shared = value["shared_with_supervisor"]
                    .as_bool()
                    .ok_or("missing shared flag")?;
                let populated = value["populated_after_exit"]
                    .as_bool()
                    .ok_or("missing populated flag")?;
                let members = value["direct_members_after_exit"]
                    .as_u64()
                    .ok_or("missing member count")?;
                let expected = if shared {
                    "shared_cgroup"
                } else if populated || members != 0 {
                    "members_remain"
                } else {
                    "empty_observed_no_migration_proof"
                };
                if status != expected {
                    return Err("inconsistent cgroup diagnostic status".into());
                }
            }
        }
        _ => return Err("unknown cgroup diagnostic status".into()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_only_canonical_unified_membership() {
        assert_eq!(unified_path("0::/a/b\n").unwrap(), "/a/b");
        assert!(unified_path("0::/a/../b\n").is_err());
        assert!(unified_path("0::/a\n0::/b\n").is_err());
        assert!(unified_path("1:name=/a\n").is_err());
        assert_eq!(members("1\n23\n").unwrap(), vec![1, 23]);
        assert!(members("0\n").is_err());
        assert!(populated("populated 0\nfrozen 0\n").is_ok_and(|x| !x));
        assert!(populated("populated 0\npopulated 1\n").is_err());
    }

    #[test]
    fn diagnostic_never_admits_migration_proof() {
        let v = json!({"status":"empty_observed_no_migration_proof","worker_pid":42,
            "relative_path":"/worker","cgroup_inode":17,"shared_with_supervisor":false,
            "populated_after_exit":false,"direct_members_after_exit":0,"no_migration_proven":false});
        verify_diagnostic(&v, 42).unwrap();
        let mut forged = v.clone();
        forged["no_migration_proven"] = json!(true);
        assert!(verify_diagnostic(&forged, 42).is_err());
        forged = v.clone();
        forged["direct_members_after_exit"] = json!(1);
        assert!(verify_diagnostic(&forged, 42).is_err());
        assert!(verify_diagnostic(&v, 43).is_err());
    }

    #[test]
    fn finish_distinguishes_empty_shared_and_live_subtrees() {
        let path = std::env::temp_dir().join(format!(
            "dataset-cgroup-diagnostic-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        fs::create_dir(&path).unwrap();
        fs::write(path.join("cgroup.events"), "populated 0\n").unwrap();
        fs::write(path.join("cgroup.procs"), "").unwrap();
        let inode = fs::metadata(&path).unwrap().ino();
        let start = |shared_with_supervisor| Start {
            path: path.clone(),
            relative: "/test".into(),
            inode,
            shared_with_supervisor,
            worker_pid: 42,
        };
        let empty = start(false).finish();
        assert_eq!(empty["status"], "empty_observed_no_migration_proof");
        verify_diagnostic(&empty, 42).unwrap();
        assert_eq!(start(true).finish()["status"], "shared_cgroup");
        fs::write(path.join("cgroup.events"), "populated 1\n").unwrap();
        assert_eq!(start(false).finish()["status"], "members_remain");
        fs::write(path.join("cgroup.events"), "populated 0\n").unwrap();
        fs::write(path.join("cgroup.procs"), "123\n").unwrap();
        assert_eq!(start(false).finish()["status"], "members_remain");
        fs::remove_dir_all(path).unwrap();
    }
}
