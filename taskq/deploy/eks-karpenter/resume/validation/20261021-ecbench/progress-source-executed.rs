//! CRIU computational phase, never a benchmark epoch. The native harness always
//! receives `--cpus none` plus an L0 spec; normal L2 admission remains unchanged.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn computation_spec(bytes: &[u8]) -> Result<Value, String> {
    let mut spec: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    if spec["schema"] != "ecbench.spec/v1" || !spec["measurement"].is_object() {
        return Err("requires an ecbench.spec/v1 document with measurement settings".into());
    }
    spec["measurement"]["isolation_required"] = json!("L0");
    spec["label"] = json!("CRIU computation only: all timing invalid for benchmark admission");
    Ok(spec)
}

/// A workload-specific compatibility filter, not a security sandbox. CRIU 4.2.1
/// rejects perf_event descriptors. Ordinary benchmark executions never use this
/// launcher. The supervisor remains outside this filter to perform the dump.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn deny_perf() -> Result<(), String> {
    const LD_W_ABS: u16 = 0x20;
    const JMP_JEQ_K: u16 = 0x15;
    const RET_K: u16 = 0x06;
    let mut filter = [
        libc::sock_filter {
            code: LD_W_ABS,
            jt: 0,
            jf: 0,
            k: 4,
        }, // seccomp_data.arch
        libc::sock_filter {
            code: JMP_JEQ_K,
            jt: 1,
            jf: 0,
            k: 0xc000003e,
        },
        libc::sock_filter {
            code: RET_K,
            jt: 0,
            jf: 0,
            k: 0x80000000,
        }, // KILL_PROCESS
        libc::sock_filter {
            code: LD_W_ABS,
            jt: 0,
            jf: 0,
            k: 0,
        }, // syscall number
        libc::sock_filter {
            code: JMP_JEQ_K,
            jt: 0,
            jf: 1,
            k: libc::SYS_perf_event_open as u32,
        },
        libc::sock_filter {
            code: RET_K,
            jt: 0,
            jf: 0,
            k: 0x00050000 | libc::EPERM as u32,
        },
        libc::sock_filter {
            code: RET_K,
            jt: 0,
            jf: 0,
            k: 0x7fff0000,
        }, // ALLOW
    ];
    let program = libc::sock_fprog {
        len: filter.len() as u16,
        filter: filter.as_mut_ptr(),
    };
    // SAFETY: valid BPF program with checked architecture; the kernel copies it
    // synchronously. NNP ensures exec cannot gain new privilege.
    unsafe {
        if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0
            || libc::prctl(libc::PR_SET_SECCOMP, 2, &program) != 0
        {
            return Err(format!(
                "cannot install computation-only perf filter: {}",
                std::io::Error::last_os_error()
            ));
        }
    }
    Ok(())
}

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn deny_perf() -> Result<(), String> {
    Err("CRIU computation qualification currently requires Linux x86_64".into())
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err(
            "Usage: isolab-criu-progress /absolute/ecbench INPUT_SPEC OUTPUT_DIRECTORY".into(),
        );
    }
    if !Path::new(&args[1]).is_absolute() || !Path::new(&args[3]).is_absolute() {
        return Err(
            "executable and output directory must be absolute paths on the qualified runtime/PVC"
                .into(),
        );
    }
    // Run only after the trusted supervisor drops the payload's identity and
    // capabilities. Refuse an accidental privileged/root payload.
    let status = std::fs::read_to_string("/proc/self/status").map_err(|e| e.to_string())?;
    if unsafe { libc::geteuid() } != 1000
        || !status.lines().any(|l| l == "CapEff:\t0000000000000000")
    {
        return Err("payload must run as UID 1000 with no effective capabilities".into());
    }
    let source = std::fs::read(&args[2]).map_err(|e| e.to_string())?;
    let derived =
        serde_json::to_vec_pretty(&computation_spec(&source)?).map_err(|e| e.to_string())?;
    let spec_path = format!("{}.compute-spec.json", args[3]);
    let intent_path = format!("{}.recovery-intent.json", args[3]);
    let write_new = |path: &str, bytes: &[u8]| -> Result<(), String> {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|e| e.to_string())?;
        f.write_all(bytes)
            .and_then(|_| f.sync_all())
            .map_err(|e| e.to_string())
    };
    if Path::new(&args[3]).exists() {
        return Err("computation output already exists; do not restart a recovered command".into());
    }
    let executable = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let intent = json!({
        "schema":"isolab.criu.computation/v1", "benchmark_timing_valid":false,
        "original_spec_sha256":digest(&source), "computation_spec_sha256":digest(&derived),
        "ecbench_sha256":digest(&executable), "perf_event_policy":"deny",
        "isolation_required":"L0", "cpu_policy":"none",
        "fresh_benchmark_epoch_required":true,
        "recovery_policy":"CRIU+EBS; resume computation; new warmup, L2 and A/A before admitting timing"
    });
    write_new(&spec_path, &derived)?;
    write_new(
        &intent_path,
        &serde_json::to_vec_pretty(&intent).map_err(|e| e.to_string())?,
    )?;
    deny_perf()?;
    let e = Command::new(&args[1])
        .args([
            "run",
            "--spec",
            &spec_path,
            "--out",
            &args[3],
            "--cpus",
            "none",
            "--lock",
            &format!("{}.lock", args[3]),
        ])
        .exec();
    Err(format!("cannot exec the computation: {e}"))
}

fn main() {
    if let Err(error) = run() {
        eprintln!("isolab-criu-progress: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn computation_is_explicitly_l0_and_keeps_workload() {
        let original = json!({"schema":"ecbench.spec/v1", "measurement":{"isolation_required":"L2","rounds":5,"warmup":1}, "workloads":{"seed":73},"arms":[{"name":"reference"}]});
        let out = computation_spec(&serde_json::to_vec(&original).unwrap()).unwrap();
        assert_eq!(out["measurement"]["isolation_required"], "L0");
        assert_eq!(out["measurement"]["rounds"], 5);
        assert_eq!(out["measurement"]["warmup"], 1);
        assert_eq!(out["workloads"], original["workloads"]);
        assert_eq!(out["arms"], original["arms"]);
        assert_eq!(original["measurement"]["isolation_required"], "L2");
    }
    #[test]
    fn malformed_spec_is_rejected() {
        for input in [
            b"{}".as_slice(),
            b"null",
            b"{\"schema\":\"ecbench.spec/v1\",\"measurement\":null}",
        ] {
            assert!(computation_spec(input).is_err());
        }
    }
    #[test]
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    fn actual_filter_denies_perf_and_preserves_other_syscalls() {
        // Isolate the irreversible per-thread seccomp change from the test
        // runner. Child performs only stack/syscall operations before _exit.
        let pid = unsafe { libc::fork() };
        assert!(pid >= 0);
        if pid == 0 {
            if deny_perf().is_err() {
                unsafe { libc::_exit(2) }
            }
            let rc = unsafe {
                libc::syscall(
                    libc::SYS_perf_event_open,
                    std::ptr::null::<u8>(),
                    0,
                    -1,
                    -1,
                    0,
                )
            };
            let denied =
                rc == -1 && std::io::Error::last_os_error().raw_os_error() == Some(libc::EPERM);
            let alive = unsafe { libc::getpid() } > 0;
            unsafe { libc::_exit(if denied && alive { 0 } else { 3 }) }
        }
        let mut status = 0;
        assert_eq!(unsafe { libc::waitpid(pid, &mut status, 0) }, pid);
        assert_eq!(status, 0);
    }
}
