//! Hardware-counter version of the K conversion
//! (research/k_conversion_20261018/PROTOCOL_AMENDMENT_A1_perf.md).
//!
//! The Callgrind study (PROTOCOL.md) toggled collection on exactly two
//! intervals: `ReducedBasis::from_system_closed_full` (the dense closure) and
//! `StrongRhoG::solve` (the rho walk).  Degree 4 at N >= 34 is too slow under
//! Callgrind, so this opens user-space `instructions` and `cpu-cycles`
//! counters with `perf_event_open` and wraps the same two intervals.  The
//! algebra, the draws and the seeds are unchanged; this file only drives them.
//!
//! ```sh
//! k_conversion_perf probe
//! k_conversion_perf closure FILE DRAW D     # committed draw, closure at degree D
//! k_conversion_perf rho n61|n83 SEED
//! ```
//!
//! One JSON line on stdout.  A host that cannot count (no PMU, or
//! `perf_event_paranoid` > 2) prints `"status":"unsupported"` with the errno
//! and exits 3: a counter is never estimated.  `KPERF_SOFTWARE=1` swaps the
//! hardware events for the kernel's task-clock and page-faults and labels the
//! line `"selftest":true`; it tests the plumbing and is never a measurement.
use std::hint::black_box;
use std::io;
use std::time::Instant;

use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::inherited_f4::ReducedBasis;
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{find_irreducible_sparse, KoblitzCurve};
use crypto_lib::cryptanalysis::koblitz_strong_rho::{
    JumpG, RawPointG, StrongRho, StrongRhoCharges, StrongRhoParams, WideStrongRho,
};
use crypto_lib::cryptanalysis::koblitz_wide::WideKoblitz;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::Value;

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_TYPE_SOFTWARE: u32 = 1;
const HW_CPU_CYCLES: u64 = 0;
const HW_INSTRUCTIONS: u64 = 1;
const SW_TASK_CLOCK: u64 = 1;
const SW_PAGE_FAULTS: u64 = 2;
/// `read_format`: TOTAL_TIME_ENABLED | TOTAL_TIME_RUNNING.
#[cfg(target_os = "linux")]
const READ_FORMAT: u64 = 0b11;
/// `attr.flags` bits: disabled, inherit, exclude_kernel, exclude_hv.
#[cfg(target_os = "linux")]
const FLAGS: u64 = 1 | 1 << 1 | 1 << 5 | 1 << 6;
const IOC_ENABLE: libc::c_ulong = 0x2400;
const IOC_DISABLE: libc::c_ulong = 0x2401;
const IOC_RESET: libc::c_ulong = 0x2403;

/// `struct perf_event_attr` at `PERF_ATTR_SIZE_VER7` (128 bytes).  The libc
/// crate does not expose it, so the layout is spelled out and its size
/// checked below.
#[repr(C)]
#[derive(Default)]
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
    reserved_2: u16,
    aux_sample_size: u32,
    reserved_3: u32,
    sig_data: u64,
}
const _: () = assert!(std::mem::size_of::<PerfEventAttr>() == 128);

/// One counter reading: value and the kernel's enabled / running times.
#[derive(Clone, Copy, Default)]
struct Reading {
    value: u64,
    enabled: u64,
    running: u64,
}

struct Counters {
    fds: [libc::c_int; 2],
    selftest: bool,
}

#[cfg(target_os = "linux")]
fn open_event(type_: u32, config: u64) -> Result<libc::c_int, i32> {
    let attr = PerfEventAttr {
        type_,
        size: 128,
        config,
        read_format: READ_FORMAT,
        flags: FLAGS,
        ..PerfEventAttr::default()
    };
    // pid 0, cpu -1: the calling thread on any CPU; inherit follows spawned threads.
    let fd = unsafe {
        libc::syscall(
            libc::SYS_perf_event_open,
            &attr as *const PerfEventAttr,
            0 as libc::pid_t,
            -1 as libc::c_int,
            -1 as libc::c_int,
            0 as libc::c_ulong,
        )
    };
    if fd < 0 {
        Err(io::Error::last_os_error().raw_os_error().unwrap_or(0))
    } else {
        Ok(fd as libc::c_int)
    }
}

#[cfg(not(target_os = "linux"))]
fn open_event(_type: u32, _config: u64) -> Result<libc::c_int, i32> {
    Err(libc::ENOSYS)
}

impl Counters {
    /// Instructions and cycles (or, with `KPERF_SOFTWARE=1`, task-clock and
    /// page-faults in their place).
    fn open() -> Result<Self, i32> {
        let selftest = std::env::var("KPERF_SOFTWARE").is_ok_and(|v| v == "1");
        let (t, a, b) = if selftest {
            (PERF_TYPE_SOFTWARE, SW_TASK_CLOCK, SW_PAGE_FAULTS)
        } else {
            (PERF_TYPE_HARDWARE, HW_INSTRUCTIONS, HW_CPU_CYCLES)
        };
        let first = open_event(t, a)?;
        let second = match open_event(t, b) {
            Ok(fd) => fd,
            Err(e) => {
                unsafe { libc::close(first) };
                return Err(e);
            }
        };
        Ok(Counters {
            fds: [first, second],
            selftest,
        })
    }

    fn start(&self) {
        for &fd in &self.fds {
            unsafe {
                libc::ioctl(fd, IOC_RESET, 0);
                libc::ioctl(fd, IOC_ENABLE, 0);
            }
        }
    }

    fn stop(&self) -> [Reading; 2] {
        for &fd in &self.fds {
            unsafe { libc::ioctl(fd, IOC_DISABLE, 0) };
        }
        let mut out = [Reading::default(); 2];
        for (r, &fd) in out.iter_mut().zip(&self.fds) {
            let mut buf = [0u64; 3];
            let got = unsafe { libc::read(fd, buf.as_mut_ptr().cast(), 24) };
            assert_eq!(got, 24, "short counter read");
            *r = Reading {
                value: buf[0],
                enabled: buf[1],
                running: buf[2],
            };
        }
        out
    }
}

/// True when the kernel descheduled a counter: running time more than 0.1%
/// short of enabled time (inherited threads add a few microseconds of skew).
fn is_multiplexed(r: &[Reading; 2]) -> bool {
    r.iter()
        .any(|x| (x.running as f64) < 0.999 * x.enabled as f64)
}

/// JSON fragment shared by every mode: the two counters, their times, and
/// whether the kernel multiplexed them.
fn counter_json(c: &Counters, r: &[Reading; 2], secs: f64) -> String {
    let multiplexed = is_multiplexed(r);
    format!(
        r#""selftest":{},"instructions":{},"cycles":{},"time_enabled_ns":{},"time_running_ns":{},"multiplexed":{multiplexed},"secs":{secs:.3},"vmhwm_kb":{}"#,
        c.selftest,
        r[0].value,
        r[1].value,
        r[0].enabled.max(r[1].enabled),
        r[0].running.min(r[1].running),
        vmhwm_kb()
    )
}

fn vmhwm_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1)?.parse().ok())
        })
        .unwrap_or(0)
}

fn paranoid() -> String {
    std::fs::read_to_string("/proc/sys/perf_event_paranoid")
        .or_else(|_| std::fs::read_to_string("/proc/sys/kernel/perf_event_paranoid"))
        .map_or("unknown".into(), |s| s.trim().to_string())
}

fn unsupported(mode: &str, errno: i32) -> ! {
    println!(
        r#"{{"status":"unsupported","mode":"{mode}","errno":{errno},"error":"{}","perf_event_paranoid":"{}"}}"#,
        io::Error::from_raw_os_error(errno),
        paranoid()
    );
    std::process::exit(3);
}

fn open_or_exit(mode: &str) -> Counters {
    Counters::open().unwrap_or_else(|e| unsupported(mode, e))
}

#[inline(never)]
fn k_closure(
    sys: &[crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolPoly],
    n_vars: usize,
    d: u32,
) -> Option<(
    ReducedBasis,
    crypto_lib::cryptanalysis::inherited_f4::InheritCost,
)> {
    ReducedBasis::from_system_closed_full(black_box(sys), n_vars, d, 1_000_000)
}

#[inline(never)]
fn k_rho_solve_n61(
    rho: &StrongRho,
    q: RawPointG<u64>,
    jumps: &[JumpG<u64, u64>],
    seed: u64,
    params: &StrongRhoParams,
    charges: StrongRhoCharges,
) -> Option<(u64, u64)> {
    let mut start = StdRng::seed_from_u64(seed);
    rho.solve(black_box(q), jumps, &mut start, params, charges)
        .map(|o| (o.scalar, o.walk_steps))
}

#[inline(never)]
fn k_rho_solve_n83(
    rho: &WideStrongRho,
    q: RawPointG<u128>,
    jumps: &[JumpG<u128, u128>],
    seed: u64,
    params: &StrongRhoParams,
    charges: StrongRhoCharges,
) -> bool {
    let mut start = StdRng::seed_from_u64(seed);
    rho.solve(black_box(q), jumps, &mut start, params, charges)
        .is_none()
}

fn probe() {
    let c = open_or_exit("probe");
    c.start();
    let mut acc = 1u64;
    for i in 0..50_000_000u64 {
        acc = black_box(acc.wrapping_mul(6364136223846793005).wrapping_add(i));
    }
    black_box(acc);
    let r = c.stop();
    println!(
        r#"{{"status":"counters_available","mode":"probe","selftest":{},"loop_iterations":50000000,"instructions":{},"cycles":{},"multiplexed":{},"perf_event_paranoid":"{}"}}"#,
        c.selftest,
        r[0].value,
        r[1].value,
        is_multiplexed(&r),
        paranoid()
    );
}

/// The committed unsatisfiable draw `draw` of `file`, as dreg_closure rebuilds it.
fn closure(file: &str, draw: u64, d: u32) {
    let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("{file}: {e}"));
    let v: Value = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Value>(l).expect("json"))
        .find(|v| {
            v["outcome"]["kind"] != "satisfiable"
                && v.get("control").is_none()
                && v["draw"].as_u64() == Some(draw)
        })
        .unwrap_or_else(|| panic!("{file}: no unsatisfiable draw {draw}"));
    let m = v.get("m").and_then(Value::as_u64).unwrap_or(3) as usize;
    let n = v["n"].as_u64().expect("n") as u32;
    let ell = v["ell"].as_u64().expect("ell") as usize;
    let irr = find_irreducible_sparse(n).expect("field");
    let st = FieldStructure::new(n, &irr);
    let b = F2mElement::one(n);
    let basis: Vec<F2mElement> = v["v_basis"]
        .as_array()
        .expect("v_basis")
        .iter()
        .map(|w| F2mElement::from_biguint(&BigUint::from(w.as_u64().unwrap()), n))
        .collect();
    let x_r = F2mElement::from_biguint(&BigUint::from(v["x_r"].as_u64().unwrap()), n);
    assert_eq!(basis.len(), ell);
    let sys = build_decomposition_system(&basis, &x_r, &b, m, &st).expect("system");
    assert_eq!(
        sys.n_vars as u64,
        v["n_vars"].as_u64().unwrap(),
        "{file}: rebuilt shape"
    );
    // Counters wrap only the closure: parsing, the system build and the
    // plain XL basis stay outside, as in the Callgrind toggle.
    let counters = open_or_exit("closure");
    let started = Instant::now();
    counters.start();
    let got = k_closure(&sys.equations, sys.n_vars, d);
    let readings = counters.stop();
    let secs = started.elapsed().as_secs_f64();
    let (closed, cost) = got.expect("closure built");
    println!(
        r#"{{"status":"ok","mode":"closure","cell":"{}","file":"{file}","draw":{draw},"degree":{d},"n_vars":{},"columns":{},"closure_word_ops":{},"closure_products":{},"refuted":{},"rank":{},{}}}"#,
        v["cell"].as_str().unwrap_or(""),
        sys.n_vars,
        closed.columns(),
        cost.word_ops(),
        cost.closure_products,
        closed.is_refuted(),
        closed.rank(),
        counter_json(&counters, &readings, secs)
    );
}

fn rho(which: &str, seed: u64) {
    match which {
        "n61" => {
            let kc = KoblitzCurve::new(0, 61).expect("Koblitz a=0 n=61");
            let rho = StrongRho::new(&kc);
            let g = rho.generator();
            let secret = 0x0123_4567_89ab_cdefu64 % rho.modulus();
            let want = secret ^ (seed.wrapping_mul(0x9e37_79b9) % rho.modulus());
            let q = rho.scalar_mul(g, want);
            let params = StrongRhoParams::default();
            let mut charges = StrongRhoCharges::default();
            let jumps = rho.jumps(seed, &mut charges);
            let counters = open_or_exit("rho");
            let started = Instant::now();
            counters.start();
            let out = k_rho_solve_n61(&rho, q, &jumps, seed, &params, charges);
            let readings = counters.stop();
            let secs = started.elapsed().as_secs_f64();
            let (found, steps) = out.expect("n=61 solve reaches no cap");
            println!(
                r#"{{"status":"ok","mode":"rho","n":61,"seed":{seed},"walk_steps":{steps},"verified":{},{}}}"#,
                found == want,
                counter_json(&counters, &readings, secs)
            );
        }
        "n83" => {
            let irr = IrreduciblePoly {
                degree: 83,
                low_terms: vec![0, 1, 2, 45],
            };
            let kc = WideKoblitz::new(0, &irr).expect("E_0 over GF(2^83)");
            let rho = kc.strong_rho();
            let g = rho.generator();
            let q = rho.scalar_mul(g, 0x5eed_0083);
            let params = StrongRhoParams {
                step_cap_factor: 0,
                ..StrongRhoParams::default()
            };
            let mut charges = StrongRhoCharges::default();
            let jumps = rho.jumps(seed, &mut charges);
            let counters = open_or_exit("rho");
            let started = Instant::now();
            counters.start();
            let capped = k_rho_solve_n83(&rho, q, &jumps, seed, &params, charges);
            let readings = counters.stop();
            let secs = started.elapsed().as_secs_f64();
            println!(
                r#"{{"status":"ok","mode":"rho","n":83,"seed":{seed},"walk_steps":1000000,"capped":{capped},"note":"step_cap_factor 0 leaves the cap at its floor of 10^6",{}}}"#,
                counter_json(&counters, &readings, secs)
            );
        }
        other => panic!("unknown rho field {other}"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("probe") => probe(),
        Some("closure") if args.len() == 5 => closure(
            &args[2],
            args[3].parse().expect("DRAW"),
            args[4].parse().expect("D"),
        ),
        Some("rho") if args.len() >= 3 => rho(
            &args[2],
            args.get(3).map_or(1, |s| s.parse().expect("SEED")),
        ),
        _ => {
            eprintln!("usage: k_conversion_perf probe | closure FILE DRAW D | rho n61|n83 SEED");
            std::process::exit(2);
        }
    }
}
