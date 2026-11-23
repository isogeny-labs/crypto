//! Tip remasure: `rho.negation` vs `rho.negation_v5` (pow2+cap64 path).
//!
//! Authoritative parity gate (workspace `docs/notes/rho-parity-20261120/`):
//! candidate / `rho.negation` ≤ 1.0× on end-to-end wall (and S), with exact
//! recovered log. This example adds Darwin `ri_instructions` under
//! `taskpolicy -a` for the open #4892 tip.
//!
//! ```text
//! TMPDIR=/Volumes/SSD990-2/tmp \
//! CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-rho-parity \
//! /usr/sbin/taskpolicy -a cargo run --release --example rho_parity_ab_v5 -- \
//!   --bits 20,24 --runs 32 --jsonl .scratch-rho-parity-ab-v5.jsonl
//! ```

use crypto_lib::cryptanalysis::ic_boundary::{
    find_prime_order_curve, rho_v3_jumps_for, rho_walk_v3, rho_walk_with, CountedGroup, GroupOps,
    NegationClasses, RhoV3, RhoWalk,
};
use std::env;
use std::fs::OpenOptions;
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;

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
    pub fn read() -> (u64, u64) {
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
    pub fn read() -> (u64, u64) {
        (0, 0)
    }
}

fn med_f64(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.iter().sum::<f64>() / v.len() as f64
}

fn parse_bits(s: &str) -> Vec<u32> {
    s.split(',')
        .filter_map(|x| x.trim().parse().ok())
        .collect()
}

fn arg_val(args: &[String], key: &str) -> Option<String> {
    args.windows(2)
        .find(|w| w[0] == key)
        .map(|w| w[1].clone())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let bits = arg_val(&args, "--bits")
        .map(|s| parse_bits(&s))
        .unwrap_or_else(|| vec![20, 24]);
    let runs: u64 = arg_val(&args, "--runs")
        .and_then(|s| s.parse().ok())
        .unwrap_or(32);
    let jsonl_path = arg_val(&args, "--jsonl");
    let mut jsonl = jsonl_path.as_ref().map(|p| {
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(p)
            .unwrap_or_else(|e| panic!("open jsonl {p}: {e}"))
    });

    let curve_seeds: Vec<u64> = arg_val(&args, "--curve-seeds")
        .map(|s| {
            s.split(',')
                .filter_map(|x| x.trim().parse().ok())
                .collect()
        })
        .unwrap_or_else(|| vec![59297, 727, 1059, 1981, 3141, 4217, 5557, 6781]);

    println!(
        "rho_parity_ab_v5 tip remasure | bits={bits:?} runs={runs} curves={} | arms=rho.negation vs rho.negation_v5",
        curve_seeds.len()
    );
    println!(
        "order=ABBA (neg,v5,v5,neg); scores use library wall_ns/gae; Ir via RUSAGE around same call; exact [d]G=Q"
    );

    for b in bits {
        let mut pool_wall_ratio = Vec::new();
        let mut pool_s_ratio = Vec::new();
        let mut pool_ir_ratio = Vec::new();
        let mut pool_ns_ratio = Vec::new();
        let mut ok_neg = 0u32;
        let mut ok_v5 = 0u32;
        let mut total = 0u32;
        let mut k_seen = 0usize;

        for &cseed in &curve_seeds {
            let inst = find_prime_order_curve(b, cseed);
            let g = inst.curve.clone();
            let gen = inst.generator_point();
            let r = inst.r;
            let neg = NegationClasses { r };
            let cap = 64 * (r as f64).sqrt() as u64 + 4096;
            k_seen = rho_v3_jumps_for(r, 2, 2);

            for i in 0..runs {
                total += 1;
                let d = 1 + (i.wrapping_mul(0x9e3779b97f4a7c15) ^ cseed ^ 0xC0FFEE) % (r - 1);
                let mut o = GroupOps::default();
                let q = g.mul(&mut o, gen, d);
                let seed = 0xA11CEu64
                    .wrapping_add(i.wrapping_mul(0x9E3779B97F4A7C15))
                    .wrapping_add(cseed);

                let arms = [0u8, 1, 1, 0];
                let mut sample_neg_wall = Vec::new();
                let mut sample_v5_wall = Vec::new();
                let mut sample_neg_ir = Vec::new();
                let mut sample_v5_ir = Vec::new();
                let mut sample_neg_gae = Vec::new();
                let mut sample_v5_gae = Vec::new();
                let mut last_s_neg = 0.0;
                let mut last_s_v5 = 0.0;
                let mut last_ok_neg = false;
                let mut last_ok_v5 = false;

                for &arm in &arms {
                    let (ir0, _) = macos_pmu::read();
                    let _outer = Instant::now();
                    let res = if arm == 0 {
                        rho_walk_with(&g, &neg, gen, q, r, seed, cap, RhoWalk::negation())
                    } else {
                        rho_walk_v3(&g, &neg, gen, q, r, seed, cap, RhoV3::v5(true))
                    };
                    let (ir1, _) = macos_pmu::read();
                    let ir = ir1.saturating_sub(ir0) as f64;
                    black_box(_outer);
                    black_box(&res);

                    let mut replay = GroupOps::default();
                    let recovered = res.recovered;
                    let exact = recovered == Some(d)
                        && recovered
                            .map(|log| g.mul(&mut replay, gen, log) == q)
                            .unwrap_or(false);

                    if arm == 0 {
                        sample_neg_wall.push(res.wall_ns as f64);
                        sample_neg_ir.push(ir);
                        sample_neg_gae.push(res.gae);
                        last_s_neg = res.s;
                        last_ok_neg = exact;
                    } else {
                        sample_v5_wall.push(res.wall_ns as f64);
                        sample_v5_ir.push(ir);
                        sample_v5_gae.push(res.gae);
                        last_s_v5 = res.s;
                        last_ok_v5 = exact;
                    }
                }

                let wn = mean(&sample_neg_wall);
                let wv = mean(&sample_v5_wall);
                let inn = mean(&sample_neg_ir);
                let inv = mean(&sample_v5_ir);
                let gn = mean(&sample_neg_gae).max(1e-12);
                let gv = mean(&sample_v5_gae).max(1e-12);
                pool_wall_ratio.push(wv / wn);
                pool_s_ratio.push(last_s_v5 / last_s_neg.max(1e-12));
                if inn > 0.0 {
                    pool_ir_ratio.push(inv / inn);
                }
                pool_ns_ratio.push((wv / gv) / (wn / gn));
                ok_neg += last_ok_neg as u32;
                ok_v5 += last_ok_v5 as u32;
            }
        }

        let wall_med = med_f64(pool_wall_ratio.clone());
        let s_med = med_f64(pool_s_ratio.clone());
        let ir_med = if pool_ir_ratio.is_empty() {
            f64::NAN
        } else {
            med_f64(pool_ir_ratio.clone())
        };
        let ns_med = med_f64(pool_ns_ratio.clone());
        let wall_mean = mean(&pool_wall_ratio);
        let s_mean = mean(&pool_s_ratio);
        let parity_wall = wall_med <= 1.0;
        let parity_s = s_med <= 1.0;

        println!(
            "bits={b} curves={} n={total} k_v5={k_seen} | ok_neg={ok_neg}/{total} ok_v5={ok_v5}/{total}",
            curve_seeds.len()
        );
        println!(
            "  S:     med_v5/neg={s_med:.4} mean={s_mean:.4}  parity_s={parity_s}"
        );
        println!(
            "  wall:  med_v5/neg={wall_med:.4} mean={wall_mean:.4}  parity_wall={parity_wall}  (lib wall_ns)"
        );
        println!(
            "  ns/GAE med_v5/neg={ns_med:.4}  instr med_v5/neg={ir_med:.4}"
        );

        if let Some(f) = jsonl.as_mut() {
            let line = format!(
                "{{\"bits\":{b},\"curves\":{},\"n\":{total},\"k_v5\":{k_seen},\"ok_neg\":{ok_neg},\"ok_v5\":{ok_v5},\
                 \"S_med_ratio\":{s_med},\"S_mean_ratio\":{s_mean},\
                 \"wall_med_ratio\":{wall_med},\"wall_mean_ratio\":{wall_mean},\
                 \"ns_gae_med_ratio\":{ns_med},\"ir_med_ratio\":{ir_med},\
                 \"parity_s\":{parity_s},\"parity_wall\":{parity_wall}}}\n",
                curve_seeds.len()
            );
            let _ = f.write_all(line.as_bytes());
        }
    }
}
