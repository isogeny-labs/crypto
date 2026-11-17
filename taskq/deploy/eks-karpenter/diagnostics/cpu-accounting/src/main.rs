//! Bounded diagnostic of /proc/stat versus wait4 on an exclusive EKS CPU.
//! This does not grade or change any benchmark observation.
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn monotonic() -> u64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: ts is writable and CLOCK_MONOTONIC is valid on Linux.
    assert_eq!(
        unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) },
        0
    );
    ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
}

fn affinity() -> Vec<usize> {
    // SAFETY: initialized cpu_set_t, with its exact size passed to the kernel.
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        assert_eq!(
            libc::sched_getaffinity(0, std::mem::size_of_val(&set), &mut set),
            0
        );
        (0..libc::CPU_SETSIZE as usize)
            .filter(|&cpu| libc::CPU_ISSET(cpu, &set))
            .collect()
    }
}

fn pin(cpu: usize) {
    assert!(cpu < libc::CPU_SETSIZE as usize);
    // SAFETY: cpu is in range and the initialized set is passed at its exact size.
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_SET(cpu, &mut set);
        assert_eq!(
            libc::sched_setaffinity(0, std::mem::size_of_val(&set), &set),
            0
        );
    }
    assert_eq!(affinity(), vec![cpu]);
}

fn ticks(cpu: usize) -> Vec<u64> {
    let stat = fs::read_to_string("/proc/stat").unwrap();
    let name = format!("cpu{cpu}");
    let line = stat
        .lines()
        .find(|line| line.split_whitespace().next() == Some(&name))
        .unwrap();
    let values: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    assert!(values.len() >= 8);
    values
}

fn child(cpu: usize, faults: bool) {
    pin(cpu);
    let mut pages = vec![0u8; if faults { 20 * 1024 * 1024 } else { 0 }];
    for offset in (0..pages.len()).step_by(4096) {
        pages[offset] = 1;
    }
    std::hint::black_box(&pages);
    let end = monotonic() + 20_000_000;
    let mut value = 1u64;
    while monotonic() < end {
        for _ in 0..4096 {
            value = value.wrapping_mul(6364136223846793005).wrapping_add(1);
        }
        std::hint::black_box(value);
    }
    std::hint::black_box(pages);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("child") {
        child(args[2].parse().unwrap(), args[3] == "faults");
        return;
    }
    assert_eq!(
        args.len(),
        2,
        "Usage: isolab-cpu-accounting OUTPUT-DIRECTORY"
    );
    let out = Path::new(&args[1]);
    fs::create_dir(out).unwrap();
    let cpus = affinity();
    assert!(cpus.len() >= 2);
    let observer = cpus[0];
    let run = *cpus.last().unwrap();
    assert_ne!(run, 0);
    let read = |p: &str| fs::read_to_string(p).unwrap_or_else(|e| format!("unavailable: {e}"));
    // SAFETY: sysconf has no preconditions.
    let hz = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
    assert!(hz > 0);
    let metadata = json!({"schema":"isolab.cpu-accounting-probe/v1","samples":2000,"child_spin_ns":20000000,
        "allowed_cpus":cpus,"observer_cpu":observer,"run_cpu":run,"hz":hz,
        "cmdline":read("/proc/cmdline"),"kernel":read("/proc/version"),
        "nohz_full":read("/sys/devices/system/cpu/nohz_full"),
        "workqueue_cpumask":read("/sys/devices/virtual/workqueue/cpumask"),
        "cpu_siblings":read(&format!("/sys/devices/system/cpu/cpu{run}/topology/thread_siblings_list")),
        "process_status":read("/proc/self/status"),"interrupts_before":read("/proc/interrupts"),
        "stat_columns":["user","nice","system","idle","iowait","irq","softirq","steal","guest","guest_nice"]});
    fs::write(
        out.join("metadata.json"),
        serde_json::to_vec_pretty(&metadata).unwrap(),
    )
    .unwrap();
    pin(observer);
    let mut records = BufWriter::new(fs::File::create(out.join("records.jsonl")).unwrap());
    let mut histograms: BTreeMap<String, BTreeMap<i64, usize>> = BTreeMap::new();
    let mut flagged = Vec::new();
    for seq in 0..2000 {
        let mode = if seq % 2 == 0 { "spin" } else { "faults" };
        let before_start = monotonic();
        let before = ticks(run);
        let before_end = monotonic();
        let mut process = Command::new(std::env::current_exe().unwrap())
            .args(["child", &run.to_string(), mode])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let mut status = 0;
        // SAFETY: zeroed output structure; wait4 waits for our own child.
        let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
        loop {
            let got = unsafe { libc::wait4(process.id() as i32, &mut status, 0, &mut usage) };
            if got == process.id() as i32 {
                break;
            }
            assert_eq!(
                std::io::Error::last_os_error().kind(),
                std::io::ErrorKind::Interrupted
            );
        }
        // The child has already been reaped; do not use its high-level wait handle.
        let _ = &mut process;
        assert!(libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0);
        let after_start = monotonic();
        let after = ticks(run);
        let after_end = monotonic();
        let ns = |t: libc::timeval| t.tv_sec as u64 * 1_000_000_000 + t.tv_usec as u64 * 1000;
        let child_ns = ns(usage.ru_utime) + ns(usage.ru_stime);
        let delta: Vec<i64> = after
            .iter()
            .zip(&before)
            .map(|(a, b)| *a as i64 - *b as i64)
            .collect();
        let busy: i64 = [0, 1, 2, 5, 6].iter().map(|&i| delta[i]).sum();
        let foreign = (busy as f64 - child_ns as f64 * hz as f64 / 1e9).round() as i64;
        *histograms
            .entry(mode.into())
            .or_default()
            .entry(foreign)
            .or_default() += 1;
        let record = json!({"seq":seq,"mode":mode,"before_start_ns":before_start,"before_end_ns":before_end,
            "after_start_ns":after_start,"after_end_ns":after_end,"before_ticks":before,"after_ticks":after,
            "delta_ticks":delta,"busy_ticks":busy,"foreign_busy_ticks":foreign,"child_user_ns":ns(usage.ru_utime),
            "child_system_ns":ns(usage.ru_stime),"minor_faults":usage.ru_minflt,"involuntary_switches":usage.ru_nivcsw});
        serde_json::to_writer(&mut records, &record).unwrap();
        writeln!(records).unwrap();
        if foreign > 2 {
            flagged.push(record);
        }
    }
    records.flush().unwrap();
    fs::write(out.join("summary.json"), serde_json::to_vec_pretty(&json!({"samples":2000,
        "histograms":histograms,"above_two_ticks":flagged,"benchmark_admission":"not a benchmark; diagnostic only"})).unwrap()).unwrap();
    fs::write(out.join("interrupts-after.txt"), read("/proc/interrupts")).unwrap();
}
