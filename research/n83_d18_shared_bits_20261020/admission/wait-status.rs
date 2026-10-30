//! Linux event-driven admission notification. It performs no research computation.
use std::{fs, path::Path};

fn state(path: &Path) -> Result<bool, String> {
    let value = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(e) => return Err(e.to_string()),
    };
    let value = value.trim();
    if value == "AWAITING_COMMITTED_HOST_CARD" {
        return Ok(true);
    }
    if value.starts_with("FAILED") || value.starts_with("BLOCKED") {
        return Err(format!("admission stopped: {value}"));
    }
    Ok(false)
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), String> {
    use std::{
        ffi::CString,
        os::unix::ffi::OsStrExt,
        time::{Duration, Instant},
    };
    #[repr(C)]
    struct PollFd {
        fd: i32,
        events: i16,
        revents: i16,
    }
    unsafe extern "C" {
        fn inotify_init1(flags: i32) -> i32;
        fn inotify_add_watch(fd: i32, path: *const i8, mask: u32) -> i32;
        fn poll(fds: *mut PollFd, count: usize, milliseconds: i32) -> i32;
        fn read(fd: i32, buffer: *mut u8, bytes: usize) -> isize;
        fn close(fd: i32) -> i32;
    }
    struct Descriptor(i32);
    impl Drop for Descriptor {
        fn drop(&mut self) {
            unsafe {
                close(self.0);
            }
        }
    }
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        return Err("usage: wait-status STATUS_FILE TIMEOUT_SECONDS".into());
    }
    let path = Path::new(&args[1]);
    let seconds: u64 = args[2]
        .to_str()
        .ok_or("timeout encoding")?
        .parse()
        .map_err(|_| "timeout integer")?;
    if !(1..=14400).contains(&seconds) {
        return Err("timeout must be 1..14400 seconds".into());
    }
    let parent = CString::new(path.parent().ok_or("status parent")?.as_os_str().as_bytes())
        .map_err(|_| "parent NUL")?;
    let fd = unsafe { inotify_init1(0x80000) }; // IN_CLOEXEC
    if fd < 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    let fd = Descriptor(fd);
    // Watch before reading the state, so a completed write cannot be missed.
    if unsafe { inotify_add_watch(fd.0, parent.as_ptr(), 0x8 | 0x80) } < 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    let end = Instant::now() + Duration::from_secs(seconds);
    loop {
        if state(path)? {
            println!("AWAITING_COMMITTED_HOST_CARD");
            return Ok(());
        }
        let left = end.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err("admission notification timed out".into());
        }
        let mut pollfd = PollFd {
            fd: fd.0,
            events: 1,
            revents: 0,
        };
        let result = unsafe {
            poll(
                &mut pollfd,
                1,
                left.as_millis().max(1).min(i32::MAX as u128) as i32,
            )
        };
        if result < 0 {
            let e = std::io::Error::last_os_error();
            if e.kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            return Err(e.to_string());
        }
        if result > 0 {
            if pollfd.revents & 1 == 0 {
                return Err("notification descriptor failed".into());
            }
            let mut buffer = [0u8; 8192];
            if unsafe { read(fd.0, buffer.as_mut_ptr(), buffer.len()) } <= 0 {
                return Err(std::io::Error::last_os_error().to_string());
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("wait-status requires Linux inotify");
    std::process::exit(2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_explicit_ready_state_admits_and_failure_stops() {
        let p = std::env::temp_dir().join(format!("n83-admission-test-{}", std::process::id()));
        assert_eq!(state(&p), Ok(false));
        for s in ["WAITING_FOR_VALIDATION_LOCK", "PREPARING_HOST_CARD", ""] {
            fs::write(&p, s).unwrap();
            assert_eq!(state(&p), Ok(false));
        }
        fs::write(&p, "AWAITING_COMMITTED_HOST_CARD\n").unwrap();
        assert_eq!(state(&p), Ok(true));
        for s in ["FAILED exit=1", "BLOCKED_BENCHMARK_LOCK"] {
            fs::write(&p, s).unwrap();
            assert!(state(&p).is_err());
        }
        fs::remove_file(p).unwrap();
    }
}
