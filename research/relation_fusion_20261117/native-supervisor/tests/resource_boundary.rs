use relation_native_supervisor::{run, Limits};
use std::{
    fs, io,
    os::unix::process::ExitStatusExt,
    path::PathBuf,
    process::{Command, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
fn case(name: &str) -> PathBuf {
    let root = std::env::var_os("NATIVE_SUPERVISOR_QUAL_OUT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::temp_dir().join(format!(
                "native-supervisor-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ))
        });
    fs::create_dir_all(&root).unwrap();
    let case = root.join(name);
    fs::create_dir(&case).unwrap();
    case
}
fn command(case: &std::path::Path, mode: &str) -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_resource-probe"));
    c.arg(mode)
        .stdin(Stdio::null())
        .stdout(Stdio::from(
            fs::File::options()
                .create_new(true)
                .write(true)
                .open(case.join("stdout"))
                .unwrap(),
        ))
        .stderr(Stdio::from(
            fs::File::options()
                .create_new(true)
                .write(true)
                .open(case.join("stderr"))
                .unwrap(),
        ));
    c
}
#[test]
fn actual_os_affinity_memory_and_both_hard_limits_are_enforced() {
    let path = case("limits");
    let out = run(&mut command(&path, "limits"), Limits::default()).unwrap();
    assert!(out.status.success() && !out.timed_out);
    let b = fs::read_to_string(path.join("stdout")).unwrap();
    let v: Vec<_> = b.split_whitespace().collect();
    assert_eq!(
        &v[..6],
        &[
            "2147483648",
            "2147483648",
            "16777216",
            "16777216",
            "0",
            "true"
        ]
    );
    assert_eq!(v[6].parse::<u32>().unwrap(), out.child_pid);
}
#[test]
fn actual_os_file_ceiling_refuses_growth_and_failure_is_retained() {
    let path = case("file");
    let mut c = command(&path, "file");
    c.arg(path.join("too-large.bin"));
    let out = run(&mut c, Limits::default()).unwrap();
    assert!(!out.timed_out);
    assert_eq!(out.status.signal(), Some(libc::SIGXFSZ));
    assert_eq!(
        fs::metadata(path.join("too-large.bin")).unwrap().len(),
        16777216
    );
}
#[test]
fn short_deadline_kills_private_group_including_descendant() {
    let path = case("deadline");
    let start = Instant::now();
    let out = run(
        &mut command(&path, "descendant"),
        Limits::with_timeout(Duration::from_millis(100)).unwrap(),
    )
    .unwrap();
    assert!(out.timed_out);
    assert_eq!(out.status.signal(), Some(libc::SIGKILL));
    assert!(start.elapsed() < Duration::from_secs(3));
    let grandchild: u32 = fs::read_to_string(path.join("stdout"))
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    assert_ne!(grandchild, out.child_pid);
    let stop = Instant::now() + Duration::from_secs(2);
    loop {
        match fs::read_to_string(format!("/proc/{grandchild}/stat")) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => break,
            Ok(v) if v.rsplit_once(") ").unwrap().1.starts_with('Z') => break,
            Ok(_) => {
                assert!(Instant::now() < stop, "descendant remains live");
                std::thread::sleep(Duration::from_millis(2));
            }
            Err(e) => panic!("{e}"),
        }
    }
}
#[test]
fn inherited_ignored_sigchld_is_refused_before_launch() {
    let path = case("sigchld");
    let mut old = std::mem::MaybeUninit::<libc::sigaction>::uninit();
    assert_eq!(
        unsafe { libc::sigaction(libc::SIGCHLD, std::ptr::null(), old.as_mut_ptr()) },
        0
    );
    let old = unsafe { old.assume_init() };
    struct Restore(libc::sigaction);
    impl Drop for Restore {
        fn drop(&mut self) {
            unsafe {
                libc::sigaction(libc::SIGCHLD, &self.0, std::ptr::null_mut());
            }
        }
    }
    let _restore = Restore(old);
    let mut ignored = old;
    ignored.sa_sigaction = libc::SIG_IGN;
    assert_eq!(
        unsafe { libc::sigaction(libc::SIGCHLD, &ignored, std::ptr::null_mut()) },
        0
    );
    let err = run(&mut command(&path, "sleep"), Limits::default()).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    assert!(fs::read(path.join("stdout")).unwrap().is_empty());
}
