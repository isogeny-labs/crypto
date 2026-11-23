//! OS contract probes; never a cryptographic performance workload.
use std::{
    io::{self, Write},
    process::{Command, Stdio},
    time::Duration,
};
fn main() -> io::Result<()> {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("limits") => {
            let mut memory = std::mem::MaybeUninit::<libc::rlimit>::uninit();
            let mut file = std::mem::MaybeUninit::<libc::rlimit>::uninit();
            if unsafe { libc::getrlimit(libc::RLIMIT_AS, memory.as_mut_ptr()) } < 0
                || unsafe { libc::getrlimit(libc::RLIMIT_FSIZE, file.as_mut_ptr()) } < 0
            {
                return Err(io::Error::last_os_error());
            }
            let memory = unsafe { memory.assume_init() };
            let file = unsafe { file.assume_init() };
            let affinity = std::fs::read_to_string("/proc/self/status")?;
            let affinity = affinity
                .lines()
                .find(|l| l.starts_with("Cpus_allowed_list:"))
                .ok_or_else(|| io::Error::other("affinity"))?
                .split_whitespace()
                .last()
                .unwrap();
            let mut allocation = Vec::<u8>::new();
            let refused = allocation.try_reserve_exact(2147483649).is_err();
            println!(
                "{} {} {} {} {} {} {}",
                memory.rlim_cur,
                memory.rlim_max,
                file.rlim_cur,
                file.rlim_max,
                affinity,
                refused,
                unsafe { libc::getpgrp() }
            );
        }
        Some("file") => {
            let mut f = std::fs::File::options()
                .create_new(true)
                .write(true)
                .open(&a[2])?;
            let b = [0u8; 65536];
            for _ in 0..257 {
                f.write_all(&b)?;
            }
            return Err(io::Error::other("file cap was not enforced"));
        }
        Some("descendant") => {
            let child = Command::new(std::env::current_exe()?)
                .arg("sleep")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
            println!("{}", child.id());
            io::stdout().flush()?;
            std::thread::sleep(Duration::from_secs(10));
        }
        Some("sleep-pid") => {
            println!("{}", std::process::id());
            io::stdout().flush()?;
            std::thread::sleep(Duration::from_secs(10));
        }
        Some("supervise") => {
            let directory = std::path::Path::new(&a[2]);
            let mut c = Command::new(std::env::current_exe()?);
            c.arg("sleep-pid")
                .stdin(Stdio::null())
                .stdout(Stdio::from(
                    std::fs::File::options()
                        .write(true)
                        .create_new(true)
                        .open(directory.join("child.stdout"))?,
                ))
                .stderr(Stdio::from(
                    std::fs::File::options()
                        .write(true)
                        .create_new(true)
                        .open(directory.join("child.stderr"))?,
                ));
            let result = relation_native_supervisor::run(
                &mut c,
                relation_native_supervisor::Limits::default(),
            )?;
            if !result.status.success() {
                return Err(io::Error::other("supervised child failed"));
            }
        }
        Some("sleep") => std::thread::sleep(Duration::from_secs(10)),
        _ => return Err(io::Error::other("probe mode")),
    }
    Ok(())
}
