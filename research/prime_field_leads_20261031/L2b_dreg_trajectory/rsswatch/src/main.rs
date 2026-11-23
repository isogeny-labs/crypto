//! rsswatch <pid>: prints the resident size in KiB of a process; exit 1 if unreadable.
//! macOS: proc_pidinfo, PROC_PIDTASKINFO. Linux: VmRSS in /proc/<pid>/status
//! (Linux branch added 2026-11-01 for the rerun; the macOS branch is unchanged).
//! Used only as the stop-rule memory meter.
#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Default)]
struct TaskInfo { virt: u64, res: u64, tu: u64, ts: u64, tu2: u64, ts2: u64, pol: i32, faults: i32, pageins: i32, cow: i32, msgs_s: i32, msgs_r: i32, sysc_m: i32, sysc_u: i32, csw: i32, threads: i32, running: i32, prio: i32 }
#[cfg(target_os = "macos")]
extern "C" { fn proc_pidinfo(pid: i32, flavor: i32, arg: u64, buf: *mut u8, size: i32) -> i32; }
#[cfg(target_os = "macos")]
fn rss_kib(pid: i32) -> Option<u64> {
    let mut ti = TaskInfo::default();
    let n = unsafe { proc_pidinfo(pid, 4, 0, &mut ti as *mut _ as *mut u8, std::mem::size_of::<TaskInfo>() as i32) };
    if n <= 0 { None } else { Some(ti.res / 1024) }
}
#[cfg(target_os = "linux")]
fn rss_kib(pid: i32) -> Option<u64> {
    let s = std::fs::read_to_string(format!("/proc/{}/status", pid)).ok()?;
    let line = s.lines().find(|l| l.starts_with("VmRSS:"))?;
    line.split_whitespace().nth(1)?.parse().ok()
}
fn main() {
    let pid: i32 = std::env::args().nth(1).unwrap().parse().unwrap();
    match rss_kib(pid) { Some(k) => println!("{}", k), None => std::process::exit(1) }
}
