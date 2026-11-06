//! The measurement core of `perfbench`: a registry of fixed-input kernels,
//! each returning a fingerprint of its output, timed one run at a time.
//!
//! A kernel is a [`Workload`] built by an untimed `setup`.  Each sample
//! calls [`Workload::prepare`] (untimed, for kernels that consume their
//! input, such as an in-place elimination) and then [`Workload::run`]
//! (timed).  `run` must do the same work every call and return a
//! fingerprint of what it computed; the harness refuses a kernel whose
//! fingerprint changes between samples, and `scripts/perf/perfindex.py`
//! refuses a comparison whose baseline and candidate fingerprints differ,
//! because a faster run that computes something else is not a speedup.
//!
//! Output is one JSON object per kernel on stdout, so the runner can
//! alternate baseline and candidate binaries kernel by kernel.

#![allow(dead_code)] // helpers are shared by kernels that may not use all of them

use std::hint::black_box;
use std::time::{Duration, Instant};

/// A timed unit of work with fixed inputs.
pub trait Workload {
    /// Untimed per-sample preparation (restore consumed inputs).
    fn prepare(&mut self) {}
    /// The timed work.  Returns a fingerprint of everything it computed.
    fn run(&mut self) -> u64;
}

/// A workload from a closure that needs no per-sample preparation.
pub struct Closure<F: FnMut() -> u64>(pub F);

impl<F: FnMut() -> u64> Workload for Closure<F> {
    fn run(&mut self) -> u64 {
        (self.0)()
    }
}

/// A workload that clones a pristine input before every sample (untimed)
/// and hands the copy to the timed body.
pub struct Fresh<T: Clone, F: FnMut(&mut T) -> u64> {
    pristine: T,
    work: Option<T>,
    body: F,
}

impl<T: Clone, F: FnMut(&mut T) -> u64> Fresh<T, F> {
    pub fn new(input: T, body: F) -> Self {
        Fresh {
            pristine: input,
            work: None,
            body,
        }
    }
}

impl<T: Clone, F: FnMut(&mut T) -> u64> Workload for Fresh<T, F> {
    fn prepare(&mut self) {
        self.work = Some(self.pristine.clone());
    }
    fn run(&mut self) -> u64 {
        let work = self.work.as_mut().expect("prepare runs before run");
        (self.body)(work)
    }
}

/// Which runs include a kernel.  `Quick` kernels form the default index;
/// `Full` adds the slower cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    Quick,
    Full,
}

/// One registered kernel.
pub struct Kernel {
    /// Stable identifier, `area/name`.  Never reuse an ID for different
    /// work: the index pairs baseline and candidate by ID.
    pub id: &'static str,
    /// The area weight group (see `docs/perf/PERFORMANCE_INDEX.md`).
    pub area: &'static str,
    /// What one run computes, in one line.
    pub desc: &'static str,
    pub tier: Tier,
    /// Builds the inputs (untimed) and returns the workload.
    pub setup: fn() -> Box<dyn Workload>,
}

/// FNV-1a over 64-bit words with a final avalanche: a fingerprint that
/// is stable across Rust versions and platforms (unlike `DefaultHasher`).
#[derive(Clone, Copy, Debug)]
pub struct Fp(u64);

impl Default for Fp {
    fn default() -> Self {
        Fp::new()
    }
}

impl Fp {
    pub const fn new() -> Self {
        Fp(0xcbf2_9ce4_8422_2325)
    }
    #[inline]
    pub fn u64(mut self, x: u64) -> Self {
        for b in x.to_le_bytes() {
            self.0 ^= b as u64;
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
        self
    }
    pub fn u128(self, x: u128) -> Self {
        self.u64(x as u64).u64((x >> 64) as u64)
    }
    pub fn usize(self, x: usize) -> Self {
        self.u64(x as u64)
    }
    pub fn bool(self, x: bool) -> Self {
        self.u64(x as u64)
    }
    pub fn words(mut self, xs: &[u64]) -> Self {
        self = self.usize(xs.len());
        for &x in xs {
            self = self.u64(x);
        }
        self
    }
    pub fn bytes(mut self, xs: &[u8]) -> Self {
        self = self.usize(xs.len());
        for &b in xs {
            self.0 ^= b as u64;
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
        self
    }
    pub fn str(self, s: &str) -> Self {
        self.bytes(s.as_bytes())
    }
    pub fn finish(self) -> u64 {
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

/// Carries the workload into `ThreadPool::install`, which demands `Send`.
struct OnThisThread<'a>(&'a mut Box<dyn Workload>);

impl<'a> OnThisThread<'a> {
    // A method, so the closure captures the whole wrapper and not the
    // non-`Send` field alone (edition 2021 disjoint capture).
    fn into_inner(self) -> &'a mut Box<dyn Workload> {
        self.0
    }
}

// SAFETY: the pool is built with `use_current_thread`, so `install` called
// from this thread runs its closure inline on this thread; the workload
// never leaves the thread that owns it.
unsafe impl Send for OnThisThread<'_> {}

/// The region `perfindex.py instr` counts.  Callgrind runs with
/// `--instr-atstart=no`, and the region switches instrumentation on and
/// off around the run with client requests.  Instrumentation is global,
/// so work the kernel hands to other threads — a rayon pool, or a
/// private pool an entry point builds for itself — is counted too, which
/// a per-thread `--toggle-collect` on this function would miss.
#[inline(never)]
pub fn perfbench_measured_region(w: &mut dyn Workload) -> u64 {
    callgrind_request(CALLGRIND_START_INSTRUMENTATION);
    let fp = black_box(w.run());
    callgrind_request(CALLGRIND_STOP_INSTRUMENTATION);
    fp
}

/// `VG_USERREQ_TOOL_BASE('C', 'T') + 4` and `+ 5` (`valgrind/callgrind.h`).
const CALLGRIND_START_INSTRUMENTATION: u64 = (b'C' as u64) << 24 | (b'T' as u64) << 16 | 4;
const CALLGRIND_STOP_INSTRUMENTATION: u64 = CALLGRIND_START_INSTRUMENTATION + 1;

/// A valgrind client request: the magic preamble of `valgrind.h`, which
/// valgrind recognises and a real CPU executes as four rotations of `rdi`
/// by 128 bits in all and a no-op exchange — nothing, outside valgrind.
#[inline(always)]
fn callgrind_request(request: u64) {
    #[cfg(target_arch = "x86_64")]
    {
        let args = [request, 0, 0, 0, 0, 0u64];
        // SAFETY: the sequence leaves every register as it found it
        // (`rdi` is rotated by a multiple of 64, `xchg rbx, rbx` is a
        // no-op); valgrind reads `args` through `rax` and writes `rdx`.
        unsafe {
            std::arch::asm!(
                "rol rdi, 3",
                "rol rdi, 13",
                "rol rdi, 61",
                "rol rdi, 51",
                "xchg rbx, rbx",
                in("rax") args.as_ptr(),
                inout("rdx") 0u64 => _,
                inout("rdi") 0u64 => _,
                options(nostack),
            );
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = request;
}

/// What the process itself reports about the cost of a run: CPU time, peak
/// resident set, context switches and page faults.  Wall time alone cannot
/// say whether a slow sample was slow work or a descheduled process, and a
/// comparison that cannot say is not evidence (AGENTS.md section 10).  The
/// two calls are declared here, not taken from the `libc` crate, because the
/// harness is overlaid on older revisions whose manifests may not list it;
/// `std` links the symbols on every unix target.
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod acct {
    #[repr(C)]
    struct Timespec {
        sec: i64,
        nsec: i64,
    }

    #[repr(C)]
    struct Rusage {
        utime: [i64; 2],
        stime: [i64; 2],
        maxrss: i64,
        ixrss: i64,
        idrss: i64,
        isrss: i64,
        minflt: i64,
        majflt: i64,
        nswap: i64,
        inblock: i64,
        oublock: i64,
        msgsnd: i64,
        msgrcv: i64,
        nsignals: i64,
        nvcsw: i64,
        nivcsw: i64,
    }

    extern "C" {
        fn clock_gettime(clock: i32, ts: *mut Timespec) -> i32;
        fn getrusage(who: i32, usage: *mut Rusage) -> i32;
    }

    #[cfg(target_os = "linux")]
    const CLOCK_PROCESS_CPUTIME_ID: i32 = 2;
    #[cfg(target_os = "macos")]
    const CLOCK_PROCESS_CPUTIME_ID: i32 = 12;
    const RUSAGE_SELF: i32 = 0;

    #[derive(Clone, Copy, Default)]
    pub struct Usage {
        pub max_rss_kb: u64,
        pub vol_csw: u64,
        pub invol_csw: u64,
        pub minflt: u64,
        pub majflt: u64,
    }

    /// CPU time the whole process (every thread) has used, in nanoseconds.
    pub fn process_cpu_ns() -> Option<u64> {
        let mut ts = Timespec { sec: 0, nsec: 0 };
        // SAFETY: `ts` is a valid, writable `timespec` for the call.
        let rc = unsafe { clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut ts) };
        (rc == 0).then(|| ts.sec as u64 * 1_000_000_000 + ts.nsec as u64)
    }

    pub fn usage() -> Option<Usage> {
        let mut r = Rusage {
            utime: [0; 2],
            stime: [0; 2],
            maxrss: 0,
            ixrss: 0,
            idrss: 0,
            isrss: 0,
            minflt: 0,
            majflt: 0,
            nswap: 0,
            inblock: 0,
            oublock: 0,
            msgsnd: 0,
            msgrcv: 0,
            nsignals: 0,
            nvcsw: 0,
            nivcsw: 0,
        };
        // SAFETY: `r` matches the C `struct rusage` of 64-bit Linux and macOS
        // field for field, and is valid and writable for the call.
        let rc = unsafe { getrusage(RUSAGE_SELF, &mut r) };
        if rc != 0 {
            return None;
        }
        // Linux reports kilobytes, macOS bytes.
        #[cfg(target_os = "macos")]
        let max_rss_kb = r.maxrss as u64 / 1024;
        #[cfg(target_os = "linux")]
        let max_rss_kb = r.maxrss as u64;
        Some(Usage {
            max_rss_kb,
            vol_csw: r.nvcsw as u64,
            invol_csw: r.nivcsw as u64,
            minflt: r.minflt as u64,
            majflt: r.majflt as u64,
        })
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
mod acct {
    #[derive(Clone, Copy, Default)]
    pub struct Usage {
        pub max_rss_kb: u64,
        pub vol_csw: u64,
        pub invol_csw: u64,
        pub minflt: u64,
        pub majflt: u64,
    }
    pub fn process_cpu_ns() -> Option<u64> {
        None
    }
    pub fn usage() -> Option<Usage> {
        None
    }
}

/// JSON for an optional number.
fn json_opt(x: Option<u64>) -> String {
    x.map_or_else(|| "null".to_string(), |v| v.to_string())
}

/// Bumped when a field of the output row changes meaning.  Version 2 added
/// the cost-accounting fields; the first five fields keep their meaning.
const ROW_SCHEMA: u32 = 2;

struct Options {
    filters: Vec<String>,
    exact: bool,
    full: bool,
    samples: usize,
    min_samples: usize,
    max_seconds: f64,
    warmup_seconds: f64,
    instr: bool,
}

fn parse_options(args: &[String]) -> Options {
    let mut o = Options {
        filters: Vec::new(),
        exact: false,
        full: false,
        samples: 11,
        min_samples: 3,
        max_seconds: 3.0,
        warmup_seconds: 0.2,
        instr: false,
    };
    let mut i = 0;
    let value = |i: &mut usize| -> String {
        *i += 1;
        args.get(*i)
            .cloned()
            .unwrap_or_else(|| panic!("missing value after {}", args[*i - 1]))
    };
    while i < args.len() {
        match args[i].as_str() {
            "--filter" => o
                .filters
                .extend(value(&mut i).split(',').map(str::to_string)),
            "--exact" => o.exact = true,
            "--full" => o.full = true,
            "--samples" => o.samples = value(&mut i).parse().expect("--samples N"),
            "--min-samples" => o.min_samples = value(&mut i).parse().expect("--min-samples N"),
            "--max-seconds" => o.max_seconds = value(&mut i).parse().expect("--max-seconds S"),
            "--warmup-seconds" => {
                o.warmup_seconds = value(&mut i).parse().expect("--warmup-seconds S")
            }
            "--instr" => o.instr = true,
            other => panic!("unknown option {other}"),
        }
        i += 1;
    }
    o
}

fn selected<'a>(kernels: &'a [Kernel], o: &Options) -> Vec<&'a Kernel> {
    kernels
        .iter()
        .filter(|k| o.full || k.tier == Tier::Quick || !o.filters.is_empty())
        .filter(|k| {
            o.filters.is_empty()
                || o.filters.iter().any(|f| {
                    if o.exact {
                        k.id == f
                    } else {
                        k.id.contains(f.as_str())
                    }
                })
        })
        .collect()
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn median(xs: &mut [u64]) -> u64 {
    xs.sort_unstable();
    let n = xs.len();
    if n % 2 == 1 {
        xs[n / 2]
    } else {
        (xs[n / 2 - 1] + xs[n / 2]) / 2
    }
}

fn measure(k: &Kernel, o: &Options) {
    let setup_start = Instant::now();
    let mut w = (k.setup)();
    let setup_ns = setup_start.elapsed().as_nanos() as u64;
    let rss_after_setup = acct::usage().map(|u| u.max_rss_kb);

    if o.instr {
        // One warm run outside the region, one measured run inside it, both
        // on a one-thread rayon pool whose only worker is this thread: work a
        // kernel hands to rayon then runs inline under
        // `perfbench_measured_region`, where callgrind counts it, instead of
        // on a pool worker whose stack never enters the region.
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .use_current_thread()
            .build()
            .expect("one-thread rayon pool");
        let job = OnThisThread(&mut w);
        let (fp0, fp) = pool.install(move || {
            let w = job.into_inner();
            w.prepare();
            let fp0 = black_box(w.run());
            w.prepare();
            (fp0, perfbench_measured_region(w.as_mut()))
        });
        drop(pool);
        assert_eq!(fp0, fp, "{}: fingerprint changed between runs", k.id);
        println!(
            "{{\"id\":{},\"area\":{},\"fingerprint\":\"{:016x}\",\"setup_ns\":{},\"schema\":{}}}",
            json_str(k.id),
            json_str(k.area),
            fp,
            setup_ns,
            ROW_SCHEMA
        );
        return;
    }

    // Warm-up: at least one run, until `warmup_seconds` has passed.
    let warm_start = Instant::now();
    w.prepare();
    let fp = black_box(w.run());
    let mut warmup_runs = 1u32;
    while warm_start.elapsed().as_secs_f64() < o.warmup_seconds {
        w.prepare();
        let again = black_box(w.run());
        assert_eq!(fp, again, "{}: fingerprint changed between runs", k.id);
        warmup_runs += 1;
    }

    let budget = Duration::from_secs_f64(o.max_seconds);
    let start = Instant::now();
    let mut samples = Vec::with_capacity(o.samples);
    let mut cpu_samples: Vec<u64> = Vec::with_capacity(o.samples);
    let usage_before = acct::usage();
    while samples.len() < o.samples && (samples.len() < o.min_samples || start.elapsed() < budget) {
        w.prepare();
        let cpu0 = acct::process_cpu_ns();
        let t = Instant::now();
        let got = black_box(w.run());
        let ns = t.elapsed().as_nanos() as u64;
        let cpu1 = acct::process_cpu_ns();
        assert_eq!(fp, got, "{}: fingerprint changed between runs", k.id);
        samples.push(ns);
        if let (Some(a), Some(b)) = (cpu0, cpu1) {
            cpu_samples.push(b.saturating_sub(a));
        }
    }
    let usage_after = acct::usage();
    // The time budget, not the requested count, ended the sampling.
    let budget_hit = samples.len() < o.samples;
    let list = samples
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",");
    let min = *samples.iter().min().unwrap();
    let med = median(&mut samples.clone());
    let cpu_list = cpu_samples
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",");
    let med_cpu = if cpu_samples.is_empty() {
        None
    } else {
        Some(median(&mut cpu_samples.clone()))
    };
    // Counters are differences over the timed samples only; the peak
    // resident set is the process's high-water mark (setup included) and
    // the figure after setup is printed beside it.
    let delta = |f: fn(&acct::Usage) -> u64| -> Option<u64> {
        Some(f(&usage_after?).saturating_sub(f(&usage_before?)))
    };
    println!(
        "{{\"id\":{},\"area\":{},\"fingerprint\":\"{:016x}\",\"setup_ns\":{},\"median_ns\":{},\"min_ns\":{},\"samples_ns\":[{}],\
\"schema\":{},\"threads\":{},\"samples\":{},\"budget_hit\":{},\"warmup_runs\":{},\
\"cpu_ns\":[{}],\"median_cpu_ns\":{},\"maxrss_setup_kb\":{},\"maxrss_kb\":{},\
\"vol_csw\":{},\"invol_csw\":{},\"minflt\":{},\"majflt\":{}}}",
        json_str(k.id),
        json_str(k.area),
        fp,
        setup_ns,
        med,
        min,
        list,
        ROW_SCHEMA,
        rayon::current_num_threads(),
        samples.len(),
        budget_hit,
        warmup_runs,
        cpu_list,
        json_opt(med_cpu),
        json_opt(rss_after_setup),
        json_opt(usage_after.map(|u| u.max_rss_kb)),
        json_opt(delta(|u| u.vol_csw)),
        json_opt(delta(|u| u.invol_csw)),
        json_opt(delta(|u| u.minflt)),
        json_opt(delta(|u| u.majflt)),
    );
}

const USAGE: &str = "\
perfbench — fixed-input kernels for the performance index

USAGE:
  perfbench list [--full]
  perfbench run  [--filter A,B] [--exact] [--full] [--samples N] [--min-samples N]
                 [--max-seconds S] [--warmup-seconds S] [--instr]

Kernels run on the rayon pool as configured by RAYON_NUM_THREADS; the index
runner sets it explicitly.  Compare builds with scripts/perf/perfindex.py.";

pub fn main(kernels: Vec<Kernel>) {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut ids = std::collections::HashSet::new();
    for k in &kernels {
        assert!(ids.insert(k.id), "duplicate kernel id {}", k.id);
        assert!(
            k.id.starts_with(k.area) && k.id[k.area.len()..].starts_with('/'),
            "kernel id {} must start with its area {}/",
            k.id,
            k.area
        );
    }
    match args.first().map(String::as_str) {
        Some("list") => {
            let o = parse_options(&args[1..]);
            for k in selected(&kernels, &o) {
                println!(
                    "{{\"id\":{},\"area\":{},\"tier\":\"{:?}\",\"desc\":{}}}",
                    json_str(k.id),
                    json_str(k.area),
                    k.tier,
                    json_str(k.desc)
                );
            }
        }
        Some("run") => {
            let o = parse_options(&args[1..]);
            let chosen = selected(&kernels, &o);
            if chosen.is_empty() {
                eprintln!("no kernel matches");
                std::process::exit(2);
            }
            for k in chosen {
                measure(k, &o);
            }
        }
        _ => {
            eprintln!("{USAGE}");
            std::process::exit(2);
        }
    }
}
