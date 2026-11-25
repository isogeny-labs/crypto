//! Hardware-counter (`RUSAGE_INFO_V4`: retired instructions, CPU cycles, IPC) and
//! paired interleaved A/B/A/B benchmark suite for `/Volumes/SSD990-2/crypto`.
//!
//! Run under P-core foreground scheduling on Apple Silicon:
//!   taskpolicy -a ./target/release/examples/hw_counter_ab_bench

use crypto_lib::ct_bignum::{Uint, U256};
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::endo_generic_point::{jacobian_to_biguint, scalar_mul_wnaf, TableMode};
use crypto_lib::ecc::isogeny_scalar_mul::{
    fixtures::{
        cm_dm11_deg3_256, cm_dm7_deg2_256, cm_dm8_deg2_256, horizontal_cm_crater_6curves,
        non_montgomery_256_bridge,
    },
    CraterDlpInstance, RcbWeierstrassLadder,
};
use crypto_lib::ecc::p256_field::P256FieldElement;
use crypto_lib::ecc::p256_point::P256ProjectivePoint;
use crypto_lib::ecc::secp256k1_point::ProjectivePoint as SecpProjectivePoint;
use num_bigint::{BigUint, RandBigInt};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::hint::black_box;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

#[derive(Clone, Copy, Debug, Default)]
struct HwSample {
    ns: f64,
    instructions: f64,
    cycles: f64,
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

    #[inline]
    pub fn read_self_counters() -> (u64, u64) {
        unsafe {
            let mut ri = core::mem::MaybeUninit::<RawRusageInfoV4>::zeroed();
            let rc = proc_pid_rusage(libc::getpid(), RUSAGE_INFO_V4, ri.as_mut_ptr());
            if rc == 0 {
                let ri = ri.assume_init();
                (ri.ri_instructions, ri.ri_cycles)
            } else {
                (0, 0)
            }
        }
    }

    #[inline]
    pub fn read_pid_counters(pid: i32) -> Option<(u64, u64)> {
        unsafe {
            let mut ri = core::mem::MaybeUninit::<RawRusageInfoV4>::zeroed();
            let rc = proc_pid_rusage(pid, RUSAGE_INFO_V4, ri.as_mut_ptr());
            if rc == 0 {
                let ri = ri.assume_init();
                Some((ri.ri_instructions, ri.ri_cycles))
            } else {
                None
            }
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod macos_pmu {
    pub fn read_self_counters() -> (u64, u64) {
        (0, 0)
    }
    pub fn read_pid_counters(_pid: i32) -> Option<(u64, u64)> {
        None
    }
}

fn stats_median_iqr(mut vals: Vec<f64>) -> (f64, f64) {
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = vals.len();
    let med = vals[n / 2];
    let q1 = vals[n / 4];
    let q3 = vals[(3 * n) / 4];
    (med, q3 - q1)
}

fn measure_one<F: FnMut()>(iters: usize, f: &mut F) -> HwSample {
    let (i0, c0) = macos_pmu::read_self_counters();
    let t0 = Instant::now();
    for _ in 0..iters {
        f();
    }
    let ns = t0.elapsed().as_nanos() as f64 / (iters as f64);
    let (i1, c1) = macos_pmu::read_self_counters();
    HwSample {
        ns,
        instructions: (i1.saturating_sub(i0)) as f64 / (iters as f64),
        cycles: (c1.saturating_sub(c0)) as f64 / (iters as f64),
    }
}

fn print_summary(label: &str, samples: &[HwSample]) {
    let (med_ns, iqr_ns) = stats_median_iqr(samples.iter().map(|s| s.ns).collect());
    let (med_inst, _) = stats_median_iqr(samples.iter().map(|s| s.instructions).collect());
    let (med_cyc, _) = stats_median_iqr(samples.iter().map(|s| s.cycles).collect());
    let ipc = if med_cyc > 0.0 {
        med_inst / med_cyc
    } else {
        0.0
    };
    println!(
        "{label:<44} | {med_ns:>11.2} ns | IQR {iqr_ns:>8.2} ns | {med_inst:>11.1} inst | {med_cyc:>11.1} cyc | IPC {ipc:>4.2}"
    );
}

fn run_interleaved_ab<FA: FnMut(), FB: FnMut()>(
    title: &str,
    label_a: &str,
    label_b: &str,
    iters: usize,
    rounds: usize,
    mut fa: FA,
    mut fb: FB,
) {
    // Warmup both paths equally:
    for _ in 0..2 {
        fa();
        fb();
    }
    let mut sa = Vec::with_capacity(rounds);
    let mut sb = Vec::with_capacity(rounds);
    for r in 0..rounds {
        if r % 2 == 0 {
            sa.push(measure_one(iters, &mut fa));
            sb.push(measure_one(iters, &mut fb));
        } else {
            sb.push(measure_one(iters, &mut fb));
            sa.push(measure_one(iters, &mut fa));
        }
    }
    let (med_a, _) = stats_median_iqr(sa.iter().map(|s| s.ns).collect());
    let (med_b, _) = stats_median_iqr(sb.iter().map(|s| s.ns).collect());
    let (inst_a, _) = stats_median_iqr(sa.iter().map(|s| s.instructions).collect());
    let (inst_b, _) = stats_median_iqr(sb.iter().map(|s| s.instructions).collect());
    let speedup = med_a / med_b;
    let inst_ratio = if inst_b > 0.0 { inst_a / inst_b } else { 0.0 };
    println!("\n── {title} ({rounds} interleaved A/B rounds × {iters} ops) ──");
    print_summary(label_a, &sa);
    print_summary(label_b, &sb);
    println!(
        "   => SPEEDUP (A/B): {speedup:.3}x wall-time | {inst_ratio:.3}x fewer retired instructions ({:.1}% reduction)",
        (1.0 - 1.0 / inst_ratio.max(1e-9)) * 100.0
    );
}

fn bench_in_process_primitives_and_algebraic() {
    println!("==========================================================================================================");
    println!("1. CORE OPTIMIZED PRIMITIVE HARDWARE COUNTERS (Apple Silicon PMU via RUSAGE_INFO_V4)");
    println!("==========================================================================================================");

    let mut x = P256FieldElement::from_canonical(&Uint([0x1234_5678_9abc_def1, 3, 5, 7]));
    let y = P256FieldElement::from_canonical(&Uint([0xfedc_ba98_7654_3211, 11, 13, 17]));
    let rounds = 11;

    let mut s_mul = Vec::new();
    let mut s_sqr = Vec::new();
    let mut s_inv = Vec::new();
    for _ in 0..rounds {
        s_mul.push(measure_one(50_000, &mut || {
            x = black_box(x.mul(black_box(&y)));
        }));
        s_sqr.push(measure_one(50_000, &mut || {
            x = black_box(x.sqr());
        }));
        s_inv.push(measure_one(2_000, &mut || {
            x = black_box(x.inv());
        }));
    }
    print_summary("P256FieldElement::mul (4-limb CIOS)", &s_mul);
    print_summary("P256FieldElement::sqr (10-mul sym)", &s_sqr);
    print_summary("P256FieldElement::inv (255S+12M chain)", &s_inv);

    let p256_params = CurveParams::p256();
    let p256_g = P256ProjectivePoint::from_affine(
        &U256::from_biguint(&p256_params.gx),
        &U256::from_biguint(&p256_params.gy),
    );
    let secp_params = CurveParams::secp256k1();
    let secp_g = SecpProjectivePoint::from_affine(
        &U256::from_biguint(&secp_params.gx),
        &U256::from_biguint(&secp_params.gy),
    );
    let mut rng = StdRng::seed_from_u64(0x2026_1118);
    let p256_scalars: Vec<BigUint> = (0..16)
        .map(|_| rng.gen_biguint_below(&p256_params.n))
        .collect();
    let secp_scalars: Vec<BigUint> = (0..16)
        .map(|_| rng.gen_biguint_below(&secp_params.n))
        .collect();

    let mut s_p256_sm = Vec::new();
    let mut s_secp_sm = Vec::new();
    let mut idx = 0usize;
    for _ in 0..rounds {
        s_p256_sm.push(measure_one(64, &mut || {
            let k = &p256_scalars[idx & 15];
            idx = idx.wrapping_add(1);
            black_box(p256_g.scalar_mul_ct(black_box(k), 256));
        }));
        s_secp_sm.push(measure_one(64, &mut || {
            let k = &secp_scalars[idx & 15];
            idx = idx.wrapping_add(1);
            black_box(secp_g.scalar_mul_ct(black_box(k), 256));
        }));
    }
    print_summary("P256ProjectivePoint::scalar_mul_ct", &s_p256_sm);
    print_summary("SecpProjectivePoint::scalar_mul_ct", &s_secp_sm);

    println!("\n==========================================================================================================");
    println!("2. PAIRED INTERLEAVED A/B: LOW-DEGREE 2-ISOGENY MONTGOMERY BRIDGE ON NON-MONTGOMERY 256-BIT CURVE");
    println!("==========================================================================================================");
    let (bridge, g_br) = non_montgomery_256_bridge();
    assert!(!bridge.has_direct_montgomery_model());
    let rcb = RcbWeierstrassLadder::new(bridge.curve.clone(), bridge.scalar_bits);
    let prep_g = bridge.prepare_point(&g_br);
    let br_scalars: Vec<BigUint> = (0..32)
        .map(|_| rng.gen_biguint_below(&bridge.order))
        .collect();
    // Pre-verify bit-exact point equality across all 32 benchmark scalars before timing:
    for k in &br_scalars {
        let ref_pt = jacobian_to_biguint(
            &bridge.curve,
            &scalar_mul_wnaf(&bridge.curve, &g_br, k, 5, TableMode::Affine),
        );
        assert_eq!(
            jacobian_to_biguint(&bridge.curve, &rcb.scalar_mul_jacobian(&g_br, k)),
            ref_pt
        );
        assert_eq!(
            jacobian_to_biguint(&bridge.curve, &bridge.scalar_mul_jacobian(&g_br, k)),
            ref_pt
        );
        assert_eq!(
            jacobian_to_biguint(
                &bridge.curve,
                &bridge.scalar_mul_prepared_jacobian(&prep_g, &g_br, k)
            ),
            ref_pt
        );
    }

    let mut ia = 0usize;
    let mut ib = 0usize;
    run_interleaved_ab(
        "Constant-Time 256-bit Scalar Mult (Jacobian output, 0 inversions): Complete RCB vs 2-Isogeny Bridge (Unprepared)",
        "A: CT_RCB_Weierstrass_256 (0I)",
        "B: CT_2Iso_MontBridge_Unprepared (0I)",
        64,
        rounds,
        || {
            let k = &br_scalars[ia & 31];
            ia = ia.wrapping_add(1);
            black_box(rcb.scalar_mul_jacobian(black_box(&g_br), black_box(k)));
        },
        || {
            let k = &br_scalars[ib & 31];
            ib = ib.wrapping_add(1);
            black_box(bridge.scalar_mul_jacobian(black_box(&g_br), black_box(k)));
        },
    );

    run_interleaved_ab(
        "Constant-Time 256-bit Scalar Mult (Jacobian output, 0 inversions): Complete RCB vs 2-Isogeny Bridge (Prepared)",
        "A: CT_RCB_Weierstrass_256 (0I)",
        "B: CT_2Iso_MontBridge_Prepared (0I)",
        64,
        rounds,
        || {
            let k = &br_scalars[ia & 31];
            ia = ia.wrapping_add(1);
            black_box(rcb.scalar_mul_jacobian(black_box(&g_br), black_box(k)));
        },
        || {
            let k = &br_scalars[ib & 31];
            ib = ib.wrapping_add(1);
            black_box(bridge.scalar_mul_prepared_jacobian(
                black_box(&prep_g),
                black_box(&g_br),
                black_box(k),
            ));
        },
    );

    run_interleaved_ab(
        "Variable-Time wNAF(w=5, Jacobian) vs Constant-Time 2-Isogeny Montgomery Bridge (Unprepared, 0I)",
        "A: VT_Weierstrass_wNAF5_Jac (0I)",
        "B: CT_2Iso_MontBridge_Unprepared (0I)",
        64,
        rounds,
        || {
            let k = &br_scalars[ia & 31];
            ia = ia.wrapping_add(1);
            black_box(scalar_mul_wnaf(
                &bridge.curve,
                black_box(&g_br),
                black_box(k),
                5,
                TableMode::Jacobian,
            ));
        },
        || {
            let k = &br_scalars[ib & 31];
            ib = ib.wrapping_add(1);
            black_box(bridge.scalar_mul_jacobian(black_box(&g_br), black_box(k)));
        },
    );

    run_interleaved_ab(
        "Variable-Time wNAF(w=5, Affine table) vs Constant-Time 2-Isogeny Montgomery Bridge (Prepared, 0I)",
        "A: VT_Weierstrass_wNAF5_Aff",
        "B: CT_2Iso_MontBridge_Prepared (0I)",
        64,
        rounds,
        || {
            let k = &br_scalars[ia & 31];
            ia = ia.wrapping_add(1);
            black_box(scalar_mul_wnaf(
                &bridge.curve,
                black_box(&g_br),
                black_box(k),
                5,
                TableMode::Affine,
            ));
        },
        || {
            let k = &br_scalars[ib & 31];
            ib = ib.wrapping_add(1);
            black_box(bridge.scalar_mul_prepared_jacobian(
                black_box(&prep_g),
                black_box(&g_br),
                black_box(k),
            ));
        },
    );

    println!("\n==========================================================================================================");
    println!("3. PAIRED INTERLEAVED A/B: SMALL-DISCRIMINANT CM ACTION GLV-2 (D = -7, -8, -11) vs WEIERSTRASS wNAF(w=5)");
    println!("==========================================================================================================");

    for (name, (glv, g_cm)) in [
        ("CM D=-7 (deg-2 Velu psi = (1+sqrt(-7))/2, general a)", cm_dm7_deg2_256()),
        ("CM D=-8 (deg-2 Velu psi = sqrt(-2), a = -3)", cm_dm8_deg2_256()),
        ("CM D=-11 (deg-3 Velu psi = (1+sqrt(-11))/2, a = -3)", cm_dm11_deg3_256()),
    ] {
        let cm_scalars: Vec<BigUint> = (0..32)
            .map(|_| rng.gen_biguint_below(&glv.order))
            .collect();
        for k in &cm_scalars {
            assert_eq!(
                jacobian_to_biguint(
                    &glv.curve,
                    &scalar_mul_wnaf(&glv.curve, &g_cm, k, 5, TableMode::Jacobian)
                ),
                jacobian_to_biguint(
                    &glv.curve,
                    &glv.scalar_mul_jacobian(&g_cm, k, 4, TableMode::Jacobian)
                )
            );
        }
        let mut ca = 0usize;
        let mut cb = 0usize;
        run_interleaved_ab(
            &format!("{name} [Jacobian table, 0I]"),
            "A: Weierstrass_wNAF(w=5, Jac)",
            "B: CM_Action_GLV2(w=4, Jac)",
            64,
            rounds,
            || {
                let k = &cm_scalars[ca & 31];
                ca = ca.wrapping_add(1);
                black_box(scalar_mul_wnaf(
                    &glv.curve,
                    black_box(&g_cm),
                    black_box(k),
                    5,
                    TableMode::Jacobian,
                ));
            },
            || {
                let k = &cm_scalars[cb & 31];
                cb = cb.wrapping_add(1);
                black_box(glv.scalar_mul_jacobian(
                    black_box(&g_cm),
                    black_box(k),
                    4,
                    TableMode::Jacobian,
                ));
            },
        );
    }

    println!("\n==========================================================================================================");
    println!("3B. PAIRED INTERLEAVED A/B: HORIZONTAL Cl(O_D) VOLCANO CRATER MULTI-CURVE ECDLP (6 Non-Isomorphic Curves)");
    println!("==========================================================================================================");
    {
        use rand::Rng;
        let crater = horizontal_cm_crater_6curves();
        let mut vrng = StdRng::seed_from_u64(0x2026_1119);
        let mut instances = Vec::with_capacity(24);
        let mut secrets = Vec::with_capacity(24);
        for i in 0..24usize {
            let c_idx = i % 6;
            let base_mult = vrng.gen_range(2..crater.order);
            let secret = vrng.gen_range(1..crater.order);
            let base = crater
                .scalar_mul_node(c_idx, crater.nodes[c_idx].g, base_mult)
                .unwrap();
            let target = crater.scalar_mul_node(c_idx, base, secret).unwrap();
            instances.push(CraterDlpInstance {
                curve_idx: c_idx,
                base,
                target,
            });
            secrets.push(secret);
        }
        assert_eq!(crater.solve_independent_per_curve(&instances), secrets);
        assert_eq!(crater.solve_via_horizontal_cm_orbit(&instances), secrets);

        run_interleaved_ab(
            "Multi-Curve ECDLP (24 targets across 6 non-isomorphic CM crater curves, independent base points)",
            "A: Independent_PerCurve_BSGS (6 tables)",
            "B: Horizontal_CM_Action_SharedBSGS (1 table)",
            4,
            rounds,
            || {
                black_box(crater.solve_independent_per_curve(black_box(&instances)));
            },
            || {
                black_box(crater.solve_via_horizontal_cm_orbit(black_box(&instances)));
            },
        );
    }
}

#[cfg(target_os = "macos")]
fn run_cmd_with_child_pmu(bin: &str, args: &[&str]) -> (f64, u64, u64, String) {
    let t0 = Instant::now();
    let child = Command::new("/usr/sbin/taskpolicy")
        .arg("-a")
        .arg(bin)
        .args(args)
        .env("RAYON_NUM_THREADS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to spawn {bin}: {e}"));
    let pid = child.id() as i32;

    // Block until child exits WITHOUT reaping (`WNOWAIT`) so `proc_pid_rusage` can read
    // its exact kernel-accumulated `ri_instructions` and `ri_cycles`!
    unsafe {
        let mut info: libc::siginfo_t = core::mem::zeroed();
        libc::waitid(
            libc::P_PID,
            pid as libc::id_t,
            &mut info,
            libc::WEXITED | libc::WNOWAIT,
        );
    }
    let wall_ns = t0.elapsed().as_nanos() as f64;
    let (inst, cyc) = macos_pmu::read_pid_counters(pid).unwrap_or((0, 0));
    let output = child.wait_with_output().expect("wait_with_output");
    assert!(output.status.success(), "{bin} {:?} exited non-zero", args);
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    (wall_ns, inst, cyc, stdout)
}

#[cfg(target_os = "macos")]
fn extract_json_u64(line: &str, key: &str) -> Option<u64> {
    let pat = format!("\"{key}\":");
    let start = line.find(&pat)? + pat.len();
    let rest = line[start..].trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

#[cfg(target_os = "macos")]
fn extract_json_str(line: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\":\"");
    let start = line.find(&pat)? + pat.len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

#[cfg(target_os = "macos")]
fn bench_saved_baseline_vs_candidate_binaries() {
    let r0_pb = "/tmp/crypto_perfbench_baseline";
    let r1_pb = "/tmp/crypto_perfbench_r1";
    let r3_pb = "./target/release/examples/perfbench";
    let r0_vr = "/tmp/crypto_volcano_rho_baseline";
    let r1_vr = "/tmp/crypto_volcano_rho_r1";
    let r3_vr = "./target/release/volcano_rho";

    if !Path::new(r0_pb).exists() || !Path::new(r1_pb).exists() || !Path::new(r3_pb).exists() {
        println!("\n[skip binary A/B/C: one of {r0_pb}, {r1_pb}, {r3_pb} missing]");
        return;
    }

    println!("\n=======================================================================================================================================================");
    println!("4. 3-WAY INTERLEAVED REPOSITORY BENCHMARKS: Round 0 (Baseline) -> Round 1 -> Round 2/3 (Final)");
    println!("   Cyclic interleaved R0/R1/R3 execution on Apple Silicon P-cores + bit-exact fingerprint check + PMU retired instructions");
    println!("=======================================================================================================================================================");

    let kernels = [
        "field_ec/gf2_n53_mul_sqr_1m",
        "field_ec/gf2_n53_inv_16k",
        "field_ec/gf2_n53_batch_inv_4x64k",
        "field_ec/koblitz_n53_add_many_lazy_64x2k",
        "field_ec/koblitz_n53_fast_mul_2k",
        "field_ec/f2m_mul_sqr_m131_64k",
        "field_ec/f2m_mul_sqr_m163_64k",
        "field_ec/f2m_mul_sqr_m233_64k",
        "field_ec/f2m_inv_m163_2k",
        "field_ec/binary_scalar_mul_sect163k1_x8",
        "field_ec/binary_scalar_mul_sect233k1_x4",
        "field_ec/ct_scalar_mul_p256_x16",
        "field_ec/ct_scalar_mul_secp256k1_x16",
        "field_ec/point_scalar_mul_p256_x8",
        "field_ec/point_scalar_mul_secp256k1_x8",
        "field_ec/point_scalar_mul_ct_secp256k1_x1",
        "field_ec/ecdsa_verify_p256_x4",
        "field_ec/ecdsa_verify_secp256k1_x4",
        "pdp/subspace_oracle_n20_l7",
        "pdp/mitm_m3_n31",
        "dlp/koblitz_signed_rho_k0_n41",
        "relation/sparse_solve_r45_c4800_w3",
        "relation/block_wiedemann_r45_n768_w13",
    ];

    let rounds = 5usize;
    println!(
        "{:<42} | {:>10} | {:>10} | {:>10} | {:>8} | {:>8} | {:>9} | {:>9} | {:<16}",
        "Kernel", "R0 med_ns", "R1 med_ns", "R3 med_ns", "R1->R3", "R0->R3", "R0 M-inst", "R3 M-inst", "Fingerprint"
    );
    println!("{}", "-".repeat(150));

    for kernel in kernels {
        let args = [
            "run",
            "--exact",
            "--samples",
            "5",
            "--warmup-seconds",
            "0.02",
            "--filter",
            kernel,
        ];
        let mut s0_ns = Vec::with_capacity(rounds);
        let mut s1_ns = Vec::with_capacity(rounds);
        let mut s3_ns = Vec::with_capacity(rounds);
        let mut s0_in = Vec::with_capacity(rounds);
        let mut s3_in = Vec::with_capacity(rounds);
        let mut fp_verified = String::new();

        for r in 0..rounds {
            let run_bin = |bin: &str| {
                let (_, inst, _, out) = run_cmd_with_child_pmu(bin, &args);
                let line = out.lines().next().expect("perfbench json line");
                let med = extract_json_u64(line, "median_ns").expect("median_ns") as f64;
                let fp = extract_json_str(line, "fingerprint").expect("fingerprint");
                (med, inst as f64, fp)
            };

            let (v0, v1, v3) = match r % 3 {
                0 => (run_bin(r0_pb), run_bin(r1_pb), run_bin(r3_pb)),
                1 => {
                    let b = run_bin(r1_pb);
                    let c = run_bin(r3_pb);
                    let a = run_bin(r0_pb);
                    (a, b, c)
                }
                _ => {
                    let c = run_bin(r3_pb);
                    let a = run_bin(r0_pb);
                    let b = run_bin(r1_pb);
                    (a, b, c)
                }
            };
            assert_eq!(v0.2, v1.2, "FINGERPRINT DRIFT R0 vs R1 on {kernel}");
            assert_eq!(v0.2, v3.2, "FINGERPRINT DRIFT R0 vs R3 on {kernel}");
            fp_verified = v0.2;
            s0_ns.push(v0.0);
            s1_ns.push(v1.0);
            s3_ns.push(v3.0);
            s0_in.push(v0.1);
            s3_in.push(v3.1);
        }

        let (m0, _) = stats_median_iqr(s0_ns);
        let (m1, _) = stats_median_iqr(s1_ns);
        let (m3, _) = stats_median_iqr(s3_ns);
        let (i0, _) = stats_median_iqr(s0_in);
        let (i3, _) = stats_median_iqr(s3_in);
        println!(
            "{:<42} | {:>10.0} | {:>10.0} | {:>10.0} | {:>7.2}x | {:>7.2}x | {:>9.1} | {:>9.1} | {} OK",
            kernel,
            m0,
            m1,
            m3,
            m1 / m3,
            m0 / m3,
            i0 / 1e6,
            i3 / 1e6,
            fp_verified
        );
    }

    if Path::new(r0_vr).exists() && Path::new(r1_vr).exists() && Path::new(r3_vr).exists() {
        let vr_args = [
            "--bits",
            "32",
            "--depth",
            "2",
            "--ell",
            "3",
            "--trials",
            "20",
            "--per-level",
            "3",
            "--seed",
            "1",
            "--out",
            "/tmp/vr_ab_out",
        ];
        let mut s0_ns = Vec::with_capacity(rounds);
        let mut s1_ns = Vec::with_capacity(rounds);
        let mut s3_ns = Vec::with_capacity(rounds);
        let mut s0_in = Vec::with_capacity(rounds);
        let mut s1_in = Vec::with_capacity(rounds);
        let mut s3_in = Vec::with_capacity(rounds);
        let mut s0_cy = Vec::with_capacity(rounds);
        let mut s1_cy = Vec::with_capacity(rounds);
        let mut s3_cy = Vec::with_capacity(rounds);
        for r in 0..rounds {
            let (a, b, c) = match r % 3 {
                0 => (
                    run_cmd_with_child_pmu(r0_vr, &vr_args),
                    run_cmd_with_child_pmu(r1_vr, &vr_args),
                    run_cmd_with_child_pmu(r3_vr, &vr_args),
                ),
                1 => {
                    let b = run_cmd_with_child_pmu(r1_vr, &vr_args);
                    let c = run_cmd_with_child_pmu(r3_vr, &vr_args);
                    let a = run_cmd_with_child_pmu(r0_vr, &vr_args);
                    (a, b, c)
                }
                _ => {
                    let c = run_cmd_with_child_pmu(r3_vr, &vr_args);
                    let a = run_cmd_with_child_pmu(r0_vr, &vr_args);
                    let b = run_cmd_with_child_pmu(r1_vr, &vr_args);
                    (a, b, c)
                }
            };
            s0_ns.push(a.0);
            s1_ns.push(b.0);
            s3_ns.push(c.0);
            s0_in.push(a.1 as f64);
            s1_in.push(b.1 as f64);
            s3_in.push(c.1 as f64);
            s0_cy.push(a.2 as f64);
            s1_cy.push(b.2 as f64);
            s3_cy.push(c.2 as f64);
        }
        let (m0, q0) = stats_median_iqr(s0_ns);
        let (m1, q1) = stats_median_iqr(s1_ns);
        let (m3, q3) = stats_median_iqr(s3_ns);
        let (i0, _) = stats_median_iqr(s0_in);
        let (i1, _) = stats_median_iqr(s1_in);
        let (i3, _) = stats_median_iqr(s3_in);
        let (c0, _) = stats_median_iqr(s0_cy);
        let (c1, _) = stats_median_iqr(s1_cy);
        let (c3, _) = stats_median_iqr(s3_cy);
        println!("\n── Isogeny Volcano DP Pollard Rho (`volcano_rho` bits=32, depth=2, ell=3, 20 trials, 1 thread) ──");
        println!(
            "R0 (Baseline HEAD)       : median {:>8.2} ms (IQR {:>6.2} ms) | {:>8.2} M-inst | {:>8.2} M-cyc | IPC {:.2}",
            m0 / 1e6, q0 / 1e6, i0 / 1e6, c0 / 1e6, i0 / c0.max(1.0)
        );
        println!(
            "R1 (Shoup + u64 EGCD)    : median {:>8.2} ms (IQR {:>6.2} ms) | {:>8.2} M-inst | {:>8.2} M-cyc | IPC {:.2}",
            m1 / 1e6, q1 / 1e6, i1 / 1e6, c1 / 1e6, i1 / c1.max(1.0)
        );
        println!(
            "R3 (16-Lane Mont Batch)  : median {:>8.2} ms (IQR {:>6.2} ms) | {:>8.2} M-inst | {:>8.2} M-cyc | IPC {:.2}",
            m3 / 1e6, q3 / 1e6, i3 / 1e6, c3 / 1e6, i3 / c3.max(1.0)
        );
        println!(
            "   => Incremental (R1->R3): {:.3}x wall-time | {:.3}x fewer instructions | Cumulative (R0->R3): {:.3}x wall-time | {:.3}x fewer instructions!",
            m1 / m3, i1 / i3.max(1.0), m0 / m3, i0 / i3.max(1.0)
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("all");
    if mode == "all" || mode == "inproc" {
        bench_in_process_primitives_and_algebraic();
    }
    #[cfg(target_os = "macos")]
    if mode == "all" || mode == "binaries" {
        bench_saved_baseline_vs_candidate_binaries();
    }
}
