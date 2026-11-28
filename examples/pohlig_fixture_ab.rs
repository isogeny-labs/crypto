//! Pohlig–Hellman public-fixture evidence (no CT residual to A/B).
//!
//! Production path in `pohlig_hellman_curve` / `BabySteps` already uses
//! `scalar_mul_vartime` / `add_vartime` exclusively.  This harness times the
//! pinned smooth-order curve from `ec_vartime_walks_pinned` under
//! taskpolicy -a with wall + RUSAGE_INFO_V4 instructions — dual evidence that
//! the algorithm is publicly fixtureable and already on the vartime frontier.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf-r13 \
//!   /usr/sbin/taskpolicy -a cargo run --release --example pohlig_fixture_ab

use crypto_lib::cryptanalysis::pohlig_hellman::pohlig_hellman_curve;
use crypto_lib::ecc::curve::CurveParams;
use num_bigint::BigUint;
use std::hint::black_box;
use std::time::Instant;

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

#[cfg(target_os = "macos")]
mod macos_pmu {
    #[repr(C)]
    pub struct RawRusageInfoV4 {
        pub ri_uuid: [u8; 16],
        pub ri_user_time: u64,
        pub ri_system_time: u64,
        pub ri_pkg_idle_wkups: u64,
        pub ri_interrupt_wkups: u64,
        pub ri_pageins: u64,
        pub ri_wired_size: u64,
        pub ri_resident_size: u64,
        pub ri_phys_footprint: u64,
        pub ri_proc_start_abstime: u64,
        pub ri_proc_exit_abstime: u64,
        pub ri_child_user_time: u64,
        pub ri_child_system_time: u64,
        pub ri_child_pkg_idle_wkups: u64,
        pub ri_child_interrupt_wkups: u64,
        pub ri_child_pageins: u64,
        pub ri_child_elapsed_abstime: u64,
        pub ri_diskio_bytesread: u64,
        pub ri_diskio_byteswritten: u64,
        pub ri_cpu_time_qos_default: u64,
        pub ri_cpu_time_qos_maintenance: u64,
        pub ri_cpu_time_qos_background: u64,
        pub ri_cpu_time_qos_utility: u64,
        pub ri_cpu_time_qos_legacy: u64,
        pub ri_cpu_time_qos_user_initiated: u64,
        pub ri_cpu_time_qos_user_interactive: u64,
        pub ri_billed_system_time: u64,
        pub ri_serviced_system_time: u64,
        pub ri_logical_writes: u64,
        pub ri_lifetime_max_phys_footprint: u64,
        pub ri_instructions: u64,
        pub ri_cycles: u64,
        pub ri_billed_energy: u64,
        pub ri_serviced_energy: u64,
        pub ri_interval_max_phys_footprint: u64,
        pub ri_runnable_time: u64,
    }
    const RUSAGE_INFO_V4: libc::c_int = 4;
    extern "C" {
        fn proc_pid_rusage(
            pid: libc::c_int,
            flavor: libc::c_int,
            buffer: *mut RawRusageInfoV4,
        ) -> libc::c_int;
    }
    pub fn read_insn() -> u64 {
        unsafe {
            let mut ri = core::mem::MaybeUninit::<RawRusageInfoV4>::zeroed();
            if proc_pid_rusage(libc::getpid(), RUSAGE_INFO_V4, ri.as_mut_ptr()) == 0 {
                ri.assume_init().ri_instructions
            } else {
                0
            }
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod macos_pmu {
    pub fn read_insn() -> u64 {
        0
    }
}

/// `y² = x³ + 4` over a 47-bit prime; order `3 · 19⁴ · 151 · 181 · 13171`.
fn ph_curve() -> CurveParams {
    CurveParams {
        name: "pinned-j0-smooth47",
        p: BigUint::from(140_737_529_461_027u64),
        a: BigUint::from(0u32),
        b: BigUint::from(4u32),
        gx: BigUint::from(16_535_943_672_630u64),
        gy: BigUint::from(38_148_592_391_465u64),
        n: BigUint::from(140_737_531_856_763u64),
        h: 1,
    }
}

fn main() {
    let curve = ph_curve();
    let g = curve.generator();
    let a = curve.a_fe();
    let d = BigUint::from(12_345u64);
    let q = g.scalar_mul_vartime(&d, &a);
    let bound = 1u64 << 17;

    let report = pohlig_hellman_curve(&curve, &g, &q, &curve.n, bound);
    assert_eq!(report.recovered_d.as_ref(), Some(&d), "bit-exact recover");
    println!(
        "correctness: ok (factors={:?}, steps={})",
        report.factors, report.total_steps
    );
    println!("inventory: production PH path is already vartime (no IC_*_CT residual)");

    let mut walls = Vec::new();
    let mut insns = Vec::new();
    let reps = 8usize;
    for _ in 0..7 {
        let _ = black_box(pohlig_hellman_curve(&curve, &g, &q, &curve.n, bound));
        let i0 = macos_pmu::read_insn();
        let t0 = Instant::now();
        for _ in 0..reps {
            let r = black_box(pohlig_hellman_curve(&curve, &g, &q, &curve.n, bound));
            assert_eq!(r.recovered_d.as_ref(), Some(&d));
        }
        let ns = t0.elapsed().as_nanos() as f64 / reps as f64;
        let i1 = macos_pmu::read_insn();
        walls.push(ns);
        insns.push((i1.saturating_sub(i0)) as f64 / reps as f64);
    }
    println!("pohlig_hellman_curve on pinned-j0-smooth47 (bound=2^17)");
    println!("  wall med : {:.0} ns/solve", med(walls));
    println!("  insn med : {:.0} insn/solve", med(insns));
    println!("verdict: REJECT further PH CT→vartime (already shipped); no new structural angle");
}
