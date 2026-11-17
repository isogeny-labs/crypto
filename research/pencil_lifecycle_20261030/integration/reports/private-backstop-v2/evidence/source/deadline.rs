//! Absolute software cutoff; never evidence of physical worker deallocation.
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::unix::{fs::OpenOptionsExt, process::CommandExt},
    path::Path,
    process::{Command, Output, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
pub fn now() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|v| v.as_secs())
        .map_err(err)
}
fn utc_ns() -> Result<u128, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|v| v.as_nanos())
        .map_err(err)
}
fn monotonic_ns() -> Result<u128, String> {
    let mut value = std::mem::MaybeUninit::<libc::timespec>::uninit();
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, value.as_mut_ptr()) } != 0 {
        return Err(err(std::io::Error::last_os_error()));
    }
    let value = unsafe { value.assume_init() };
    let sec = u128::try_from(value.tv_sec).map_err(err)?;
    let ns = u128::try_from(value.tv_nsec).map_err(err)?;
    if ns >= 1_000_000_000 {
        return Err("invalid monotonic sample".into());
    }
    sec.checked_mul(1_000_000_000)
        .and_then(|v| v.checked_add(ns))
        .ok_or_else(|| "monotonic sample overflow".into())
}
/// One admission anchor, copied unchanged to every command and watchdog.
/// Monotonic cutoff is never recalculated after a UTC adjustment.
#[derive(Clone, Copy, Debug)]
pub struct Deadline {
    pub utc_seconds: u64,
    monotonic: Option<(u128, u128)>,
}
impl Deadline {
    pub fn utc_only(utc_seconds: u64) -> Self {
        Self {
            utc_seconds,
            monotonic: None,
        }
    }
    fn from_samples(
        utc_seconds: u64,
        sampled_utc_ns: u128,
        sampled_monotonic_ns: u128,
    ) -> Result<Self, String> {
        let utc_ns = u128::from(utc_seconds) * 1_000_000_000;
        let remaining = utc_ns
            .checked_sub(sampled_utc_ns)
            .filter(|v| *v > 0)
            .ok_or("cutoff expired at admission")?;
        let end = sampled_monotonic_ns
            .checked_add(remaining)
            .ok_or("monotonic deadline overflow")?;
        Ok(Self {
            utc_seconds,
            monotonic: Some((sampled_monotonic_ns, end)),
        })
    }
    pub fn capture(utc_seconds: u64) -> Result<Self, String> {
        // Sample monotonic FIRST: delay between samples shortens the allowed
        // interval rather than extending it beyond the original UTC budget.
        let mono = monotonic_ns()?;
        Self::from_samples(utc_seconds, utc_ns()?, mono)
    }
    fn check_samples(self, utc_now_ns: u128, mono_now_ns: Option<u128>) -> Result<(), String> {
        if utc_now_ns >= u128::from(self.utc_seconds) * 1_000_000_000 {
            return Err("absolute UTC computation cutoff reached".into());
        }
        if let Some((begin, end)) = self.monotonic {
            let now = mono_now_ns.ok_or("monotonic observation missing")?;
            if now < begin || now >= end {
                return Err("monotonic computation cutoff reached or clock invalid".into());
            }
        }
        Ok(())
    }
}
pub fn check(cutoff: Deadline) -> Result<(), String> {
    cutoff.check_samples(
        utc_ns()?,
        if cutoff.monotonic.is_some() {
            Some(monotonic_ns()?)
        } else {
            None
        },
    )
}
/// Source-only defence against a synchronous filesystem/CRIU operation bypassing
/// polling. Caller must be trusted root PID1 in the reviewed non-host, unshared
/// Pod PID namespace. This assertion is controller provenance, not attestation.
/// Never call from tests or a host process.
pub fn install_namespace_backstop(cutoff: Deadline, assertion: &str) -> Result<(), String> {
    if assertion != "controller-owned-private-pid-namespace/v1"
        || unsafe { libc::getpid() } != 1
        || unsafe { libc::geteuid() } != 0
    {
        return Err("private PID namespace supervisor guard missing".into());
    }
    check(cutoff)?;
    thread::Builder::new()
        .name("absolute-cutoff".into())
        .spawn(move || {
            while check(cutoff).is_ok() {
                thread::sleep(Duration::from_millis(10));
            }
            // Linux kill(-1) excludes the caller and PID1. At cutoff stop all other
            // processes in this Pod namespace, including detached CRIU descendants.
            unsafe {
                libc::kill(-1, libc::SIGKILL);
            }
            // Existing reserved shutdown tail only. Failure to publish/reap remains
            // partial; this backstop intentionally performs no blocking filesystem IO.
            thread::sleep(Duration::from_millis(100));
            unsafe {
                libc::_exit(125);
            }
        })
        .map_err(err)?;
    Ok(())
}
fn retained(path: &Path) -> Result<File, String> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(err)
}
fn read_bounded(path: &Path) -> Result<Vec<u8>, String> {
    let mut v = Vec::new();
    File::open(path)
        .map_err(err)?
        .take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut v)
        .map_err(err)?;
    if v.len() > 16 * 1024 * 1024 {
        return Err("command output retained but exceeds return cap".into());
    }
    Ok(v)
}
/// Every command uses file capture rather than pipes, so inherited descendants
/// cannot hold a pipe reader indefinitely. Exact captures survive failure.
#[allow(
    clippy::zombie_processes,
    reason = "try_wait reaps; bounded cleanup may retain explicitly unobserved status"
)]
pub fn output(
    cmd: &mut Command,
    cutoff: Deadline,
    stdout: &Path,
    stderr: &Path,
    already_group_leader: bool,
) -> Result<Output, String> {
    check(cutoff)?;
    let out = retained(stdout)?;
    let error = retained(stderr)?;
    cmd.stdout(Stdio::from(out)).stderr(Stdio::from(error));
    if !already_group_leader {
        unsafe {
            cmd.pre_exec(|| {
                if libc::setsid() < 0 {
                    Err(std::io::Error::last_os_error())
                } else {
                    Ok(())
                }
            });
        }
    }
    check(cutoff)?;
    let mut child = cmd.spawn().map_err(err)?;
    let pid = i32::try_from(child.id()).map_err(err)?;
    let mut killed = None;
    let mut wait_error = None;
    let status = loop {
        match child.try_wait() {
            Ok(Some(s)) => break Some(s),
            Ok(None) => {}
            Err(e) => {
                wait_error = Some(e.to_string());
                break None;
            }
        }
        if killed.is_none() && check(cutoff).is_err() {
            unsafe {
                libc::kill(-pid, libc::SIGKILL);
            }
            killed = Some(Instant::now());
        }
        if killed.is_some_and(|v| v.elapsed() >= Duration::from_millis(100)) {
            break None;
        }
        thread::sleep(Duration::from_millis(1));
    };
    if status.is_none() {
        unsafe {
            libc::kill(-pid, libc::SIGKILL);
        }
    }
    let receipt = serde_json::json!({"schema":"isolab.absolute-command-observation/v1","cutoff_unix_seconds":cutoff.utc_seconds,"pid":pid,"reap_observed":status.is_some(),"exit_code":status.as_ref().and_then(|s|s.code()),"success":status.as_ref().is_some_and(|s|s.success()),"cutoff_kill_requested":killed.is_some(),"wait_error":wait_error,"total_active_worker_ns":null,"accepted_external_closure":null});
    let receipt_path = stderr.with_extension("command-receipt.json");
    let publication = (|| -> Result<(), String> {
        let mut f = retained(&receipt_path)?;
        f.write_all(&serde_json::to_vec_pretty(&receipt).map_err(err)?)
            .map_err(err)?;
        f.sync_all().map_err(err)?;
        File::open(receipt_path.parent().ok_or("receipt parent")?)
            .map_err(err)?
            .sync_all()
            .map_err(err)
    })();
    if killed.is_some() || status.is_none() {
        return Err(format!(
            "command cutoff or unobserved reap; receipt publication: {publication:?}"
        ));
    }
    publication?;
    let status = status.ok_or("command status missing")?;
    Ok(Output {
        status,
        stdout: read_bounded(stdout)?,
        stderr: read_bounded(stderr)?,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, os::unix::process::ExitStatusExt};
    #[test]
    fn bounded_fake_commands_preserve_failure_signal_and_cutoff() {
        let dir = std::env::temp_dir().join(format!("owned-deadline-{}", std::process::id()));
        fs::create_dir(&dir).unwrap();
        for (name, script, exit, signal) in [
            ("failure", "printf kept; exit 7", Some(7), None),
            ("signal", "kill -TERM $$", None, Some(15)),
        ] {
            let mut cmd = Command::new("/bin/sh");
            cmd.args(["-c", script]);
            let result = output(
                &mut cmd,
                Deadline::capture(now().unwrap() + 5).unwrap(),
                &dir.join(format!("{name}.out")),
                &dir.join(format!("{name}.err")),
                false,
            )
            .unwrap();
            assert_eq!(result.status.code(), exit);
            assert_eq!(result.status.signal(), signal);
        }
        let mut cmd = Command::new("/bin/sh");
        cmd.args(["-c", "printf before; sleep 10"]);
        assert!(output(
            &mut cmd,
            Deadline::capture(now().unwrap() + 1).unwrap(),
            &dir.join("sleep.out"),
            &dir.join("sleep.err"),
            false
        )
        .is_err());
        assert_eq!(fs::read(dir.join("sleep.out")).unwrap(), b"before");
        let receipt: serde_json::Value =
            serde_json::from_slice(&fs::read(dir.join("sleep.command-receipt.json")).unwrap())
                .unwrap();
        assert_eq!(receipt["cutoff_kill_requested"], true);
        assert_eq!(receipt["reap_observed"], true);
        assert!(receipt["total_active_worker_ns"].is_null());
        let mut expired = Command::new("/bin/sh");
        expired.args(["-c", "exit 0"]);
        assert!(output(
            &mut expired,
            Deadline::utc_only(now().unwrap()),
            &dir.join("never.out"),
            &dir.join("never.err"),
            false
        )
        .is_err());
        assert!(!dir.join("never.out").exists());
        fs::remove_dir_all(dir).unwrap();
    }
}

#[cfg(test)]
mod monotonic_tests {
    use super::*;
    #[test]
    fn backwards_utc_never_extends_shared_admission_anchor() {
        let d = Deadline::from_samples(110, 100_000_000_000, 20_000_000_000).unwrap();
        assert!(d
            .check_samples(101_000_000_000, Some(21_000_000_000))
            .is_ok());
        // A backwards UTC step is allowed only while the ORIGINAL monotonic
        // allowance remains. The same copied context is shared by all users.
        let watchdog = d;
        let command = d;
        assert!(command
            .check_samples(50_000_000_000, Some(29_999_999_999))
            .is_ok());
        assert!(watchdog
            .check_samples(50_000_000_000, Some(30_000_000_000))
            .is_err());
        assert!(command
            .check_samples(50_000_000_000, Some(30_000_000_000))
            .is_err());
        assert!(d
            .check_samples(110_000_000_000, Some(21_000_000_000))
            .is_err());
        assert!(d
            .check_samples(99_000_000_000, Some(19_000_000_000))
            .is_err());
        assert!(d.check_samples(101_000_000_000, None).is_err());
        assert!(Deadline::from_samples(100, 100_000_000_000, 20).is_err());
        assert!(Deadline::from_samples(101, 100_000_000_000, u128::MAX).is_err());
    }
    #[test]
    fn fractional_admission_does_not_round_up_the_remaining_budget() {
        let d = Deadline::from_samples(101, 100_900_000_000, 7_000_000_000).unwrap();
        assert!(d
            .check_samples(100_950_000_000, Some(7_099_999_999))
            .is_ok());
        assert!(d
            .check_samples(80_000_000_000, Some(7_100_000_000))
            .is_err());
    }
}
