//! Linux-only, single-owner child supervision for the fixed research resource envelope.
//! No worker arithmetic or proof check changes. This is not performance evidence.
#![cfg(target_os = "linux")]
use std::{
    io,
    mem::MaybeUninit,
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    os::unix::process::CommandExt,
    process::{Child, Command, ExitStatus},
    time::{Duration, Instant},
};
#[derive(Clone, Copy)]
pub struct Limits {
    timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
        }
    }
}
impl Limits {
    /// Tests may tighten the deadline; callers cannot enlarge the fixed envelope.
    pub fn with_timeout(timeout: Duration) -> io::Result<Self> {
        if timeout.is_zero() || timeout > Duration::from_secs(30) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "deadline must be positive and at most30s",
            ));
        }
        Ok(Self { timeout })
    }
}
#[derive(Debug)]
pub struct Outcome {
    pub status: ExitStatus,
    pub timed_out: bool,
    pub child_pid: u32,
}
fn admission() -> io::Result<()> {
    // No unowned SIGCHLD reaper: pidfd_open is acquired before our sole wait.
    // This library is used by one collector; callers must not concurrently reap its children.
    let mut disposition = MaybeUninit::<libc::sigaction>::uninit();
    // SAFETY: the output pointer has the ABI layout supplied by libc; no handler is installed.
    if unsafe { libc::sigaction(libc::SIGCHLD, std::ptr::null(), disposition.as_mut_ptr()) } < 0 {
        return Err(io::Error::last_os_error());
    }
    let disposition = unsafe { disposition.assume_init() };
    if disposition.sa_sigaction != libc::SIG_DFL || disposition.sa_flags & libc::SA_NOCLDWAIT != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "supervisor requires default SIGCHLD and sole child ownership",
        ));
    }
    let mut cpus = MaybeUninit::<libc::cpu_set_t>::zeroed();
    if unsafe {
        libc::sched_getaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), cpus.as_mut_ptr())
    } < 0
    {
        return Err(io::Error::last_os_error());
    }
    let cpus = unsafe { cpus.assume_init() };
    for i in 0..libc::CPU_SETSIZE as usize {
        if unsafe { libc::CPU_ISSET(i, &cpus) } != (i == 0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "supervisor must inherit exactlyCPU0",
            ));
        }
    }
    Ok(())
}
struct ChildOwner {
    child: Child,
    reaped: bool,
}
impl ChildOwner {
    fn kill_and_wait(&mut self) -> io::Result<ExitStatus> {
        let pid =
            i32::try_from(self.child.id()).map_err(|_| io::Error::other("child pid range"))?;
        // SAFETY: std::Command created a private process group with this still-owned,
        // unreaped leader. The SIGCHLD admission and sole-owner contract prevent reuse.
        if unsafe { libc::kill(-pid, libc::SIGKILL) } < 0 {
            let error = io::Error::last_os_error();
            if error.raw_os_error() != Some(libc::ESRCH) {
                return Err(error);
            }
        }
        let status = self.child.wait()?;
        self.reaped = true;
        Ok(status)
    }
}
impl Drop for ChildOwner {
    fn drop(&mut self) {
        if !self.reaped {
            let _ = self.kill_and_wait();
        }
    }
}
/// Owns and reaps one child. No other thread or signal handler may reap this child.
/// Timeout kills the whole private process group; every error also cleans up.
/// Native pidfd support is mandatory; an unsupported host is explicitly refused.
/// A frozen single-process worker is killed if this supervising process dies.
/// Workers must not execute privileged files, change credentials or disable the
/// guard. This research helper is not a confinement mechanism for arbitrary code.
pub fn run(command: &mut Command, limits: Limits) -> io::Result<Outcome> {
    admission()?;
    command.process_group(0);
    command
        .env("RAYON_NUM_THREADS", "1")
        .env("OMP_NUM_THREADS", "1")
        .env("OPENBLAS_NUM_THREADS", "1")
        .env("RUST_TEST_THREADS", "1");
    let parent_pid = std::process::id() as libc::pid_t;
    // SAFETY: after fork this hook uses only Linux resource syscalls and OS-error
    // construction; no allocation, logging, locks, formatting or panics occur.
    unsafe {
        command.pre_exec(move || {
            // Frozen workers are normal unprivileged executables that do not
            // change credentials. Linux preserves this signal across their exec.
            if libc::prctl(
                libc::PR_SET_PDEATHSIG,
                libc::SIGKILL as libc::c_ulong,
                0 as libc::c_ulong,
                0 as libc::c_ulong,
                0 as libc::c_ulong,
            ) < 0
            {
                return Err(io::Error::last_os_error());
            }
            // A parent that died before prctl would not generate a later signal.
            // Reject that fork-to-registration race without allocating after fork.
            if libc::getppid() != parent_pid {
                return Err(io::Error::from_raw_os_error(libc::ECHILD));
            }

            let memory = libc::rlimit {
                rlim_cur: 2147483648,
                rlim_max: 2147483648,
            };
            let file = libc::rlimit {
                rlim_cur: 16777216,
                rlim_max: 16777216,
            };
            if libc::setrlimit(libc::RLIMIT_AS, &memory) < 0
                || libc::setrlimit(libc::RLIMIT_FSIZE, &file) < 0
            {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let start = Instant::now();
    let deadline = start
        .checked_add(limits.timeout)
        .ok_or_else(|| io::Error::other("deadline overflow"))?;
    let mut owner = ChildOwner {
        child: command.spawn()?,
        reaped: false,
    };
    let pid = owner.child.id();
    // SAFETY: pid is our unreaped child, flags0, and the returned descriptor is owned.
    let fd = unsafe { libc::syscall(libc::SYS_pidfd_open, pid as libc::pid_t, 0u32) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    let fd = unsafe {
        OwnedFd::from_raw_fd(i32::try_from(fd).map_err(|_| io::Error::other("pidfd range"))?)
    };
    let mut pollfd = libc::pollfd {
        fd: fd.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Ok(Outcome {
                status: owner.kill_and_wait()?,
                timed_out: true,
                child_pid: pid,
            });
        }
        // Round up to a millisecond; EINTR always recomputes the original deadline.
        let milliseconds = i32::try_from(remaining.as_millis() + 1)
            .map_err(|_| io::Error::other("poll timeout range"))?;
        let ready = unsafe { libc::poll(&mut pollfd, 1, milliseconds) };
        if ready < 0 {
            let e = io::Error::last_os_error();
            if e.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(e);
        }
        if ready == 0 {
            continue;
        }
        if pollfd.revents & libc::POLLIN == 0 {
            return Err(io::Error::other("unexpected pidfd poll event"));
        }
        let status = owner.child.wait()?;
        owner.reaped = true;
        return Ok(Outcome {
            status,
            timed_out: false,
            child_pid: pid,
        });
    }
}
