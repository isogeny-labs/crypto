//! HNP recovery: default vartime verify vs `IC_HNP_CT=1` CT ladders.
//!
//! Public fixture: P-256, m=8, k_bits=192 (same regime as curve_audit Phase 1.5).
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf-r13 \
//!   /usr/sbin/taskpolicy -a cargo run --release --example hnp_verify_vartime_ab

use crypto_lib::cryptanalysis::hnp_ecdsa::{
    hnp_recover_key_with_reduction, BiasedSignature, HnpReduction,
};
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::keys::EccKeyPair;
use crypto_lib::ecc::point::Point;
use crypto_lib::utils::mod_inverse;
use num_bigint::{BigUint, RandBigInt};
use num_traits::Zero;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
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

fn make_instance() -> (CurveParams, BigUint, EccKeyPair, Vec<BiasedSignature>) {
    let curve = CurveParams::p256();
    let n = curve.n.clone();
    let mut d_rng = StdRng::seed_from_u64(0xC0FFEE);
    let d = d_rng.gen_biguint_below(&n);
    let kp = EccKeyPair::from_private(d.clone(), &curve);

    let k_bits = 192u32;
    let mut rng = StdRng::seed_from_u64(0xC0FFEE);
    let mut z_seed: u64 = 0xDEAD_BEEF;
    let mut sigs: Vec<BiasedSignature> = Vec::new();
    while sigs.len() < 8 {
        let bytes_count = k_bits.div_ceil(8) as usize;
        let mut buf = vec![0u8; bytes_count];
        rng.fill_bytes(&mut buf);
        let extra = (bytes_count as u32) * 8 - k_bits;
        if extra > 0 {
            buf[0] &= 0xff >> extra;
        }
        let k = BigUint::from_bytes_be(&buf);
        if k.is_zero() {
            continue;
        }
        let z = BigUint::from(z_seed) % &n;
        z_seed = z_seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let g = curve.generator();
        let a_fe = curve.a_fe();
        let kg = g.scalar_mul_vartime(&k, &a_fe);
        let x1 = match &kg {
            Point::Affine { x, .. } => x.value.clone(),
            Point::Infinity => continue,
        };
        let r = &x1 % &n;
        if r.is_zero() {
            continue;
        }
        let rd = (&r * &d) % &n;
        let z_plus_rd = (&z + &rd) % &n;
        let k_inv = match mod_inverse(&k, &n) {
            Some(v) => v,
            None => continue,
        };
        let s = (&k_inv * &z_plus_rd) % &n;
        if s.is_zero() {
            continue;
        }
        sigs.push(BiasedSignature { r, s, z, k_bits });
    }
    (curve, d, kp, sigs)
}

fn main() {
    let (curve, d, kp, sigs) = make_instance();

    unsafe {
        std::env::remove_var("IC_HNP_CT");
    }
    let d_vt = hnp_recover_key_with_reduction(&curve, &kp.public, &sigs, HnpReduction::Lll)
        .expect("vartime recover");
    assert_eq!(d_vt, d);
    unsafe {
        std::env::set_var("IC_HNP_CT", "1");
    }
    let d_ct = hnp_recover_key_with_reduction(&curve, &kp.public, &sigs, HnpReduction::Lll)
        .expect("ct recover");
    assert_eq!(d_ct, d);
    println!("correctness: ok (m={}, k_bits=192)", sigs.len());

    let mut wall_vt = Vec::new();
    let mut wall_ct = Vec::new();
    let mut insn_vt = Vec::new();
    let mut insn_ct = Vec::new();
    let reps = 6usize;

    for round in 0..5 {
        let order = if round % 2 == 0 {
            ["vt", "ct"]
        } else {
            ["ct", "vt"]
        };
        for name in order {
            unsafe {
                if name == "vt" {
                    std::env::remove_var("IC_HNP_CT");
                } else {
                    std::env::set_var("IC_HNP_CT", "1");
                }
            }
            let _ = black_box(hnp_recover_key_with_reduction(
                &curve,
                &kp.public,
                &sigs,
                HnpReduction::Lll,
            ));
            let (i0, _) = macos_pmu::read_insn_cycles();
            let t0 = Instant::now();
            for _ in 0..reps {
                let _ = black_box(hnp_recover_key_with_reduction(
                    &curve,
                    &kp.public,
                    &sigs,
                    HnpReduction::Lll,
                ));
            }
            let ns = t0.elapsed().as_nanos() as f64 / reps as f64;
            let (i1, _) = macos_pmu::read_insn_cycles();
            let insn = (i1.saturating_sub(i0)) as f64 / reps as f64;
            if name == "vt" {
                wall_vt.push(ns);
                insn_vt.push(insn);
            } else {
                wall_ct.push(ns);
                insn_ct.push(insn);
            }
        }
    }

    let wv = med(wall_vt);
    let wc = med(wall_ct);
    let iv = med(insn_vt);
    let ic = med(insn_ct);
    println!("hnp_recover_key_with_reduction LLL on P-256 (m=8, k_bits=192)");
    println!("  vartime (default) : med {wv:.0} ns/recover  ({iv:.0} insn)");
    println!("  CT (IC_HNP_CT=1)  : med {wc:.0} ns/recover  ({ic:.0} insn)");
    println!("  wall speedup CT/vt: {:.3}x", wc / wv);
    println!("  insn speedup CT/vt: {:.3}x", ic / iv);
}
