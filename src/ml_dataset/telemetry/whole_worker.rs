//! Linux process-lifetime instruction observations for a gated dataset worker.
//!
//! The gate is already exec'd and blocked before the counter is attached. The
//! event begins on the next exec, which is the worker, and inherits later tasks.
//! This module does not assert that every descendant has terminated or that an
//! ECDLP scalar was replayed; those are independent label-admission gates.
#![allow(dead_code)] // Staged collector: supervisor integration is a separate owned change.

use std::io::{self, Write};
use std::os::fd::{FromRawFd, OwnedFd};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, ExitStatus};

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_INSTRUCTIONS: u64 = 1;
const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 2;
const PERF_FORMAT_ID: u64 = 4;
const PERF_ATTR_DISABLED: u64 = 1;
const PERF_ATTR_INHERIT: u64 = 1 << 1;
const PERF_ATTR_EXCLUDE_KERNEL: u64 = 1 << 5;
const PERF_ATTR_EXCLUDE_HV: u64 = 1 << 6;
const PERF_ATTR_ENABLE_ON_EXEC: u64 = 1 << 12;
const PERF_EVENT_IOC_ID: libc::c_ulong = 0x8008_2407;

#[repr(C)]
#[derive(Clone, Copy)]
struct PerfEventAttr {
    type_: u32,
    size: u32,
    config: u64,
    sample_period: u64,
    sample_type: u64,
    read_format: u64,
    flags: u64,
    wakeup_events: u32,
    bp_type: u32,
    config1: u64,
    config2: u64,
    branch_sample_type: u64,
    sample_regs_user: u64,
    sample_stack_user: u32,
    clockid: i32,
    sample_regs_intr: u64,
    aux_watermark: u32,
    sample_max_stack: u16,
    __reserved_2: u16,
}

fn instructions_attr() -> PerfEventAttr {
    PerfEventAttr {
        type_: PERF_TYPE_HARDWARE,
        size: std::mem::size_of::<PerfEventAttr>() as u32,
        config: PERF_COUNT_HW_INSTRUCTIONS,
        sample_period: 0,
        sample_type: 0,
        read_format: PERF_FORMAT_TOTAL_TIME_ENABLED
            | PERF_FORMAT_TOTAL_TIME_RUNNING
            | PERF_FORMAT_ID,
        flags: PERF_ATTR_DISABLED
            | PERF_ATTR_INHERIT
            | PERF_ATTR_EXCLUDE_KERNEL
            | PERF_ATTR_EXCLUDE_HV
            | PERF_ATTR_ENABLE_ON_EXEC,
        wakeup_events: 0,
        bp_type: 0,
        config1: 0,
        config2: 0,
        branch_sample_type: 0,
        sample_regs_user: 0,
        sample_stack_user: 0,
        clockid: 0,
        sample_regs_intr: 0,
        aux_watermark: 0,
        sample_max_stack: 0,
        __reserved_2: 0,
    }
}

pub(crate) struct GatedWorker {
    child: Child,
    release_pipe: Option<OwnedFd>,
}

impl GatedWorker {
    /// Spawn an exec'd shell gate. The worker cannot start until `release`.
    pub(crate) fn spawn(program: &str, args: &[&str]) -> io::Result<Self> {
        let mut pipe = [-1; 2];
        // SAFETY: pipe points to two valid integers; the returned fds are owned below.
        if unsafe { libc::pipe2(pipe.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: successful pipe2 returned two distinct owned file descriptors.
        let read_pipe = unsafe { OwnedFd::from_raw_fd(pipe[0]) };
        let write_pipe = unsafe { OwnedFd::from_raw_fd(pipe[1]) };
        let mut command = Command::new("/bin/sh");
        command
            .arg("-c")
            .arg("IFS= read -r gate <&3 || exit 125; exec \"$@\"");
        command.arg("dataset-worker-gate").arg(program).args(args);
        use std::os::fd::AsRawFd;
        let read_fd = read_pipe.as_raw_fd();
        // SAFETY: only async-signal-safe syscalls run in the post-fork child.
        unsafe {
            command.pre_exec(move || {
                if libc::dup2(read_fd, 3) < 0 {
                    return Err(io::Error::last_os_error());
                }
                if libc::fcntl(3, libc::F_SETFD, 0) < 0 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let child = command.spawn()?;
        drop(read_pipe);
        Ok(Self {
            child,
            release_pipe: Some(write_pipe),
        })
    }

    pub(crate) fn pid(&self) -> u32 {
        self.child.id()
    }

    /// Releasing the pipe starts the worker's exec. An unreleased gate exits 125 on drop.
    pub(crate) fn release(&mut self) -> io::Result<()> {
        let pipe = self
            .release_pipe
            .take()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "gate already released"))?;
        let mut file = std::fs::File::from(pipe);
        file.write_all(b"go\n")
    }

    pub(crate) fn wait(&mut self) -> io::Result<ExitStatus> {
        self.child.wait()
    }
}

impl Drop for GatedWorker {
    fn drop(&mut self) {
        // Closing the pipe makes a still-blocked gate exit without executing the worker.
        if self.release_pipe.take().is_some() {
            let _ = self.child.wait();
        }
    }
}

pub(crate) struct Collector {
    fd: OwnedFd,
    pub(crate) gate_pid: u32,
    pub(crate) event_id: u64,
    pub(crate) attr_size: u32,
    pub(crate) attr_flags: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Observation {
    pub(crate) raw_instructions: u64,
    pub(crate) time_enabled_ns: u64,
    pub(crate) time_running_ns: u64,
    pub(crate) event_id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Admission {
    pub(crate) worker_succeeded: bool,
    pub(crate) scalar_replayed: bool,
    pub(crate) descendants_terminated_and_accounted: bool,
    pub(crate) platform_admitted: bool,
}

impl Collector {
    pub(crate) fn attach(gate_pid: u32) -> io::Result<Self> {
        if gate_pid == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "zero gate PID"));
        }
        let attr = instructions_attr();
        // SAFETY: attr has the kernel perf_event_attr layout and a valid size.
        let fd = unsafe {
            libc::syscall(
                libc::SYS_perf_event_open,
                &attr,
                gate_pid as libc::pid_t,
                -1,
                -1,
                0,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: a nonnegative perf_event_open result is an owned descriptor.
        let fd = unsafe { OwnedFd::from_raw_fd(fd as i32) };
        use std::os::fd::AsRawFd;
        let mut event_id: u64 = 0;
        // SAFETY: the ioctl writes one u64 to the provided valid pointer.
        if unsafe { libc::ioctl(fd.as_raw_fd(), PERF_EVENT_IOC_ID, &mut event_id) } < 0 {
            return Err(io::Error::last_os_error());
        }
        if event_id == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "zero perf event ID",
            ));
        }
        Ok(Self {
            fd,
            gate_pid,
            event_id,
            attr_size: attr.size,
            attr_flags: attr.flags,
        })
    }

    /// Read after the worker and every admitted descendant have terminated.
    pub(crate) fn read(&self) -> io::Result<Observation> {
        use std::os::fd::AsRawFd;
        let mut words = [0u64; 4];
        // SAFETY: words is a writable 32-byte buffer owned by this call.
        let got = unsafe { libc::read(self.fd.as_raw_fd(), words.as_mut_ptr().cast(), 32) };
        if got < 0 {
            return Err(io::Error::last_os_error());
        }
        if got != 32 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "short perf counter read",
            ));
        }
        if words[3] != self.event_id {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "perf event ID mismatch",
            ));
        }
        Ok(Observation {
            raw_instructions: words[0],
            time_enabled_ns: words[1],
            time_running_ns: words[2],
            event_id: words[3],
        })
    }
}

/// A raw observation is a training label only with independent execution proof.
pub(crate) fn eligible_instructions(o: Observation, expected_id: u64, a: Admission) -> Option<u64> {
    (o.event_id == expected_id
        && o.time_enabled_ns > 0
        && o.time_running_ns == o.time_enabled_ns
        && a.worker_succeeded
        && a.scalar_replayed
        && a.descendants_terminated_and_accounted
        && a.platform_admitted)
        .then_some(o.raw_instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attr_and_admission_are_exact() {
        let attr = instructions_attr();
        assert_eq!(attr.size, 112);
        assert_eq!(attr.read_format, 7);
        assert_eq!(attr.flags, 0x1063);
        let observed = Observation {
            raw_instructions: 42,
            time_enabled_ns: 100,
            time_running_ns: 100,
            event_id: 7,
        };
        let admitted = Admission {
            worker_succeeded: true,
            scalar_replayed: true,
            descendants_terminated_and_accounted: true,
            platform_admitted: true,
        };
        assert_eq!(eligible_instructions(observed, 7, admitted), Some(42));
        assert_eq!(
            eligible_instructions(
                Observation {
                    time_running_ns: 99,
                    ..observed
                },
                7,
                admitted
            ),
            None
        );
        assert_eq!(eligible_instructions(observed, 8, admitted), None);
        assert_eq!(
            eligible_instructions(
                observed,
                7,
                Admission {
                    descendants_terminated_and_accounted: false,
                    ..admitted
                }
            ),
            None
        );
    }

    #[test]
    fn gate_eof_does_not_execute_worker() {
        let path =
            std::env::temp_dir().join(format!("dataset-gate-no-exec-{}", std::process::id()));
        let path_str = path.to_str().unwrap();
        let mut gate = GatedWorker::spawn("/usr/bin/touch", &[path_str]).unwrap();
        gate.release_pipe.take();
        let status = gate.wait().unwrap();
        assert_eq!(status.code(), Some(125));
        assert!(!path.exists());
    }

    #[test]
    fn gate_executes_worker_only_after_release() {
        let path =
            std::env::temp_dir().join(format!("dataset-gate-release-{}", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let mut gate = GatedWorker::spawn("/usr/bin/touch", &[path.to_str().unwrap()]).unwrap();
        assert!(!path.exists());
        gate.release().unwrap();
        assert!(gate.wait().unwrap().success());
        assert!(path.exists());
        std::fs::remove_file(path).unwrap();
    }
}
