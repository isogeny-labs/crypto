//! Fail-closed worker census using macOS kernel argument and cwd records.
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

fn arguments(bytes: &[u8]) -> Result<Vec<&[u8]>, String> {
    let count = bytes.get(..4).ok_or("missing kernel argument count")?;
    let count = i32::from_ne_bytes(count.try_into().unwrap());
    if !(1..=262_144).contains(&count) {
        return Err("invalid kernel argument count".into());
    }
    let rest = &bytes[4..];
    let executable_end = rest
        .iter()
        .position(|b| *b == 0)
        .ok_or("missing kernel executable terminator")?;
    let mut cursor = executable_end + 1;
    while rest.get(cursor) == Some(&0) {
        cursor += 1;
    }
    let mut args = Vec::new();
    for _ in 0..count {
        let remaining = rest.get(cursor..).ok_or("truncated kernel arguments")?;
        let len = remaining
            .iter()
            .position(|b| *b == 0)
            .ok_or("unterminated kernel argument")?;
        args.push(&remaining[..len]);
        cursor += len + 1;
    }
    Ok(args)
}

// proc_pidinfo does not expose BSD info for zombies on macOS. ps obtains
// the kernel process state and does not decode or format argument strings here.
fn is_exiting_or_zombie(pid: u32) -> Result<bool, String> {
    // Darwin proc_info.h: PROC_FLAG_INEXIT = 4. The flag confirms that
    // a process has begun exit(), before its BSD status necessarily becomes Z.
    const PROC_FLAG_INEXIT: u32 = 4;
    let mut info: libc::proc_bsdshortinfo = unsafe { std::mem::zeroed() };
    let size = std::mem::size_of_val(&info);
    let got = unsafe {
        libc::proc_pidinfo(
            pid as i32,
            libc::PROC_PIDT_SHORTBSDINFO,
            0,
            (&mut info as *mut libc::proc_bsdshortinfo).cast(),
            size as i32,
        )
    };
    if got == size as i32
        && info.pbsi_pid == pid
        && (info.pbsi_status == libc::SZOMB || info.pbsi_flags & PROC_FLAG_INEXIT != 0)
    {
        return Ok(true);
    }
    is_zombie(pid)
}

fn is_zombie(pid: u32) -> Result<bool, String> {
    let output = std::process::Command::new("/bin/ps")
        .args(["-p", &pid.to_string(), "-o", "stat="])
        .output()
        .map_err(|e| format!("cannot inspect process state: {e}"))?;
    if output.status.success() {
        let state = std::str::from_utf8(&output.stdout).map_err(|_| "invalid process state")?;
        return Ok(state.trim_start().starts_with('Z'));
    }
    if unsafe { libc::kill(pid as i32, 0) } == -1
        && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
    {
        return Ok(true);
    }
    Err(format!("cannot inspect process {pid} state"))
}

pub(super) fn assert_no_workers(root: &Path) -> Result<(), String> {
    let mut argmax: libc::c_int = 0;
    let mut size = std::mem::size_of_val(&argmax);
    let mut query = [libc::CTL_KERN, libc::KERN_ARGMAX];
    let status = unsafe {
        libc::sysctl(
            query.as_mut_ptr(),
            2,
            (&mut argmax as *mut libc::c_int).cast(),
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };
    if status != 0
        || size != std::mem::size_of_val(&argmax)
        || !(4..=2 * 1024 * 1024).contains(&argmax)
    {
        return Err("cannot obtain bounded kernel argument capacity".into());
    }
    // Enumerate only numeric identities; argv must come from the kernel, not
    // ps's formatted command string, which loses quoting and empty arguments.
    let output = std::process::Command::new("/bin/ps")
        .args(["-axo", "uid=,pid="])
        .output()
        .map_err(|e| format!("cannot enumerate workers: {e}"))?;
    if !output.status.success() {
        return Err("cannot enumerate workers".into());
    }
    let rows = std::str::from_utf8(&output.stdout).map_err(|_| "invalid process census")?;
    for row in rows.lines() {
        let fields = row
            .split_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "invalid process identity")?;
        if fields.len() != 2 {
            return Err("invalid process census row".into());
        }
        let (uid, pid) = (fields[0], fields[1]);
        if uid != unsafe { libc::geteuid() } || pid == std::process::id() {
            continue;
        }
        let mut mib = [
            libc::CTL_KERN,
            libc::KERN_PROCARGS2,
            pid.try_into().map_err(|_| "invalid process id")?,
        ];
        let mut bytes = vec![0u8; argmax as usize];
        let mut length = bytes.len();
        let mut result = -1;
        const PROCARGS_ATTEMPTS: usize = 16;
        for attempt in 0..PROCARGS_ATTEMPTS {
            length = bytes.len();
            result = unsafe {
                libc::sysctl(
                    mib.as_mut_ptr(),
                    3,
                    bytes.as_mut_ptr().cast(),
                    &mut length,
                    std::ptr::null_mut(),
                    0,
                )
            };
            if result == 0 {
                break;
            }
            let error = std::io::Error::last_os_error();
            if matches!(error.raw_os_error(), Some(libc::ESRCH | libc::ENOENT)) {
                length = 0;
                result = 0;
                break;
            }
            if error.raw_os_error() == Some(libc::EINVAL) {
                if is_exiting_or_zombie(pid)? {
                    length = 0;
                    result = 0;
                    break;
                }
                if attempt + 1 < PROCARGS_ATTEMPTS {
                    // Kernel argv can be unavailable while exec/exit advances.
                    // Give a short-lived process time to become inspectable or
                    // reach a confirmed exit state, even under scheduler load.
                    // An unreadable live process still fails closed at the limit.
                    std::thread::sleep(std::time::Duration::from_millis(2));
                    continue;
                }
            }
            return Err(format!("cannot inspect process {pid}: {error}"));
        }
        if result != 0 {
            return Err(format!(
                "cannot inspect process {pid} after bounded retries"
            ));
        }
        bytes.truncate(length);
        // A process that has exited can have no retained argv.
        if bytes.is_empty() {
            continue;
        }
        let args = arguments(&bytes).map_err(|e| format!("cannot inspect process {pid}: {e}"))?;
        if !args
            .windows(2)
            .any(|p| p == [b"dataset".as_slice(), b"worker".as_slice()])
        {
            continue;
        }
        for pair in args.windows(2).filter(|p| p[0] == b"--out") {
            let path = Path::new(OsStr::from_bytes(pair[1]));
            let resolved = if path.is_absolute() {
                path.to_path_buf()
            } else {
                let mut info: libc::proc_vnodepathinfo = unsafe { std::mem::zeroed() };
                let size = std::mem::size_of_val(&info);
                let got = unsafe {
                    libc::proc_pidinfo(
                        pid as i32,
                        libc::PROC_PIDVNODEPATHINFO,
                        0,
                        (&mut info as *mut libc::proc_vnodepathinfo).cast(),
                        size as i32,
                    )
                };
                if got != size as i32 {
                    return Err(format!("cannot inspect worker {pid} cwd"));
                }
                let raw: Vec<u8> = info
                    .pvi_cdir
                    .vip_path
                    .iter()
                    .flatten()
                    .map(|c| *c as u8)
                    .take_while(|c| *c != 0)
                    .collect();
                let cwd = Path::new(OsStr::from_bytes(&raw));
                if !cwd.is_absolute() {
                    return Err(format!("invalid worker {pid} cwd"));
                }
                cwd.join(path)
            };
            let resolved = resolved
                .canonicalize()
                .map_err(|e| format!("cannot resolve worker {pid} output: {e}"))?;
            if resolved.starts_with(root) {
                return Err(format!(
                    "live worker {pid} still owns output under {}; reconcile only after it exits",
                    root.display()
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kernel_arguments_preserve_empty_values_and_ignore_environment() {
        let mut bytes = 5i32.to_ne_bytes().to_vec();
        bytes.extend_from_slice(b"/bin/tool\0\0tool\0dataset\0\0worker\0--out\0ENV=worker\0");
        let args = arguments(&bytes).unwrap();
        assert_eq!(
            args,
            [b"tool".as_slice(), b"dataset", b"", b"worker", b"--out"]
        );
        assert!(!args
            .windows(2)
            .any(|p| p == [b"dataset".as_slice(), b"worker".as_slice()]));
        assert!(arguments(&bytes[..bytes.len() - 20]).is_err());
        assert!(arguments(&0i32.to_ne_bytes()).is_err());
    }
    #[test]
    fn census_accepts_confirmed_zombie_without_reaping_it() {
        let mut child = std::process::Command::new("/usr/bin/true").spawn().unwrap();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
        let zombie = loop {
            if is_zombie(child.id()).unwrap() {
                break true;
            }
            if std::time::Instant::now() >= deadline {
                break false;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        };
        let result = assert_no_workers(&std::env::temp_dir().canonicalize().unwrap());
        child.wait().unwrap();
        assert!(zombie, "child never reached kernel zombie state");
        result.unwrap();
    }
    #[test]
    fn short_lived_process_churn_does_not_block_an_idle_output() {
        let root = std::env::temp_dir().join(format!("worker-churn-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        for _ in 0..32 {
            let mut child = std::process::Command::new("/usr/bin/true").spawn().unwrap();
            let result = assert_no_workers(&root.canonicalize().unwrap());
            child.wait().unwrap();
            result.unwrap();
        }
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn census_refuses_live_relative_worker_and_accepts_after_exit() {
        let root = std::env::temp_dir().join(format!("worker-census-{}", std::process::id()));
        std::fs::create_dir_all(root.join("out")).unwrap();
        let mut child = std::process::Command::new("/bin/sh")
            .args(["-c", "read line", "dataset", "worker", "--out", "out"])
            .current_dir(&root)
            .stdin(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let result = assert_no_workers(&root.canonicalize().unwrap());
        child.kill().unwrap();
        child.wait().unwrap();
        let error = result.unwrap_err();
        assert!(error.contains("still owns output"), "{error}");
        assert_no_workers(&root.canonicalize().unwrap()).unwrap();
        std::fs::remove_dir_all(root).unwrap();
    }
}
