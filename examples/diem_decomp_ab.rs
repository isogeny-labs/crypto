//! Diem toy 2-decomposition: hash `O(|FB|)` vs brute `O(|FB|²)`.
//!
//! Public fixture: E / F_{251²}, FB = {P : x(P) ∈ F_251}.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf-r13 \
//!   /usr/sbin/taskpolicy -a cargo run --release --example diem_decomp_ab

use crypto_lib::cryptanalysis::diem_descent::{
    build_factor_base, find_2_decomposition, find_2_decomposition_brute, ECurveFpk, Fpk, Pt,
};
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
    pub fn read_insn_cycles() -> (u64, u64) {
        unsafe {
            let mut ri = core::mem::MaybeUninit::<RawRusageInfoV4>::zeroed();
            if proc_pid_rusage(libc::getpid(), RUSAGE_INFO_V4, ri.as_mut_ptr()) == 0 {
                let ri = ri.assume_init();
                (ri.ri_instructions, ri.ri_cycles)
            } else {
                (0, 0)
            }
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod macos_pmu {
    pub fn read_insn_cycles() -> (u64, u64) {
        (0, 0)
    }
}

fn f2512_curve() -> ECurveFpk {
    // F_251[θ]/(θ² − 2); a = 1+θ, b = 3 ∈ F_251.
    let p = 251u64;
    let k = 2u32;
    let irr = 2u64;
    let a = Fpk::from_coeffs(vec![1, 1], p, k);
    let b = Fpk::from_coeffs(vec![3, 0], p, k);
    ECurveFpk::new(a, b, p, k, irr)
}

fn planted_targets(curve: &ECurveFpk, fb: &[Pt], n: usize) -> Vec<Pt> {
    let mut out = Vec::with_capacity(n);
    let mut i = 0usize;
    while out.len() < n && !fb.is_empty() {
        let p1 = &fb[i % fb.len()];
        let p2 = &fb[(i * 3 + 1) % fb.len()];
        let s1 = if i % 2 == 0 { p1.clone() } else { p1.neg() };
        let s2 = if i % 3 == 0 { p2.clone() } else { p2.neg() };
        out.push(curve.add(&s1, &s2));
        i += 1;
    }
    out
}

fn main() {
    let curve = f2512_curve();
    println!("building FB over F_251^2 …");
    let t_fb = Instant::now();
    let fb = build_factor_base(&curve);
    println!(
        "FB size = {} (build {:.1}s)",
        fb.len(),
        t_fb.elapsed().as_secs_f64()
    );
    assert!(fb.len() >= 8, "need a usable factor base");

    let targets = planted_targets(&curve, &fb, 48);
    // Correctness: hash and brute agree on success/failure; both find a decomp.
    for (ti, r) in targets.iter().enumerate() {
        let h = find_2_decomposition(&curve, &fb, r);
        let b = find_2_decomposition_brute(&curve, &fb, r);
        assert!(h.is_some(), "hash miss on planted target {ti}");
        assert!(b.is_some(), "brute miss on planted target {ti}");
        let h = h.unwrap();
        let p1 = if h.signs.0 == -1 {
            fb[h.indices.0].neg()
        } else {
            fb[h.indices.0].clone()
        };
        let p2 = if h.signs.1 == -1 {
            fb[h.indices.1].neg()
        } else {
            fb[h.indices.1].clone()
        };
        assert_eq!(curve.add(&p1, &p2), *r, "hash decomp invalid at {ti}");
    }
    println!("correctness: ok ({} planted targets)", targets.len());

    let mut wall_hash = Vec::new();
    let mut wall_brute = Vec::new();
    let mut insn_hash = Vec::new();
    let mut insn_brute = Vec::new();
    let reps = 3usize;

    for r in 0..5 {
        let order = if r % 2 == 0 {
            ["hash", "brute"]
        } else {
            ["brute", "hash"]
        };
        for name in order {
            // Warmup
            for t in targets.iter().take(4) {
                let _ = black_box(if name == "hash" {
                    find_2_decomposition(&curve, &fb, t)
                } else {
                    find_2_decomposition_brute(&curve, &fb, t)
                });
            }
            let (i0, _) = macos_pmu::read_insn_cycles();
            let t0 = Instant::now();
            for _ in 0..reps {
                for t in &targets {
                    let _ = black_box(if name == "hash" {
                        find_2_decomposition(&curve, &fb, t)
                    } else {
                        find_2_decomposition_brute(&curve, &fb, t)
                    });
                }
            }
            let ns = t0.elapsed().as_nanos() as f64 / (reps * targets.len()) as f64;
            let (i1, _) = macos_pmu::read_insn_cycles();
            let insn = (i1.saturating_sub(i0)) as f64 / (reps * targets.len()) as f64;
            if name == "hash" {
                wall_hash.push(ns);
                insn_hash.push(insn);
            } else {
                wall_brute.push(ns);
                insn_brute.push(insn);
            }
        }
    }

    let wh = med(wall_hash);
    let wb = med(wall_brute);
    let ih = med(insn_hash);
    let ib = med(insn_brute);
    println!("diem find_2_decomposition on F_251^2 (FB={})", fb.len());
    println!("  hash (default) : med {wh:.0} ns/target  ({ih:.0} insn/target)");
    println!("  brute          : med {wb:.0} ns/target  ({ib:.0} insn/target)");
    println!("  wall speedup   : {:.3}x", wb / wh);
    println!("  insn speedup   : {:.3}x", ib / ih);
}
