//! rsswatch <pid>: prints the resident size in KiB of a process (macOS proc_pidinfo,
//! PROC_PIDTASKINFO); exit 1 if unreadable. Used only as the 16 GiB stop-rule meter.
#[repr(C)]
#[derive(Default)]
struct TaskInfo { virt: u64, res: u64, tu: u64, ts: u64, tu2: u64, ts2: u64, pol: i32, faults: i32, pageins: i32, cow: i32, msgs_s: i32, msgs_r: i32, sysc_m: i32, sysc_u: i32, csw: i32, threads: i32, running: i32, prio: i32 }
extern "C" { fn proc_pidinfo(pid: i32, flavor: i32, arg: u64, buf: *mut u8, size: i32) -> i32; }
fn main() {
    let pid: i32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut ti = TaskInfo::default();
    let n = unsafe { proc_pidinfo(pid, 4, 0, &mut ti as *mut _ as *mut u8, std::mem::size_of::<TaskInfo>() as i32) };
    if n <= 0 { std::process::exit(1); }
    println!("{}", ti.res / 1024);
}
