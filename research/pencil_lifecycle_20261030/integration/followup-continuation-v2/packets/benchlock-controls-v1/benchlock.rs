/// An exclusive `flock`, owned by the acquiring process until dropped.
/// An inherited descriptor is not an ownership transfer to a forked child.
pub struct BenchLock {
    _file: std::fs::File,
    #[cfg(unix)]
    owner_pid: libc::pid_t,
    pub path: String,
}

impl BenchLock {
    pub fn acquire(path: &str, wait: bool) -> Result<Self, String> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(path)
            .map_err(|e| format!("cannot open lock {path}: {e}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            // Non-blocking tries, so a queued session stays interruptible.
            loop {
                // SAFETY: a valid descriptor owned by `file`.
                if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
                    break;
                }
                let error = std::io::Error::last_os_error();
                if error.kind() != std::io::ErrorKind::WouldBlock {
                    return Err(format!("cannot lock {path}: {error}"));
                }
                if !wait {
                    return Err(format!(
                        "another benchmark holds {path}; pass --wait to queue behind it: {error}"
                    ));
                }
                if let Some(sig) = crate::cryptanalysis::ecbench::signals::interrupted() {
                    return Err(format!(
                        "interrupted by signal {sig} while waiting for {path}"
                    ));
                }
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
        }
        Ok(Self {
            _file: file,
            #[cfg(unix)]
            // SAFETY: getpid has no preconditions.
            owner_pid: unsafe { libc::getpid() },
            path: path.to_string(),
        })
    }
}


impl Drop for BenchLock {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            // A child dropping its inherited guard must not unlock the owner.
            // SAFETY: getpid has no preconditions; _file still owns its fd.
            if unsafe { libc::getpid() } == self.owner_pid {
                // Release the shared open-file-description lock even if an
                // unrelated concurrent child has not yet exec'd/closed its fd.
                // Drop cannot report errors; File closure remains the fallback.
                let _ = unsafe { libc::flock(self._file.as_raw_fd(), libc::LOCK_UN) };
            }
        }
    }
}

