//! GHS public-fixture evidence: time the F_{2^6} end-to-end attack and
//! document why ECC scalar_mul is not an actionable hot kernel.
//!
//! Public fixtures (`ghs_attack_demo`, sect113r1 screen evidence) either
//! (a) run at toy F_{2^6} where descent/HCDLP dominate wall time, or
//! (b) are screen/census jobs without a tight ECC ladder loop.
//! `ECurve::scalar_mul` is already variable-time left-to-right binary;
//! inventing window/NAF micros on a cold path fails the audit bar.
//!
//!   TMPDIR=/Volumes/SSD990-2/tmp CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-perf-r13 \
//!   /usr/sbin/taskpolicy -a cargo run --release --example ghs_fixture_evidence

use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::ec_trapdoor::{DescentRow, FieldTower, TrapdoorCurve};
use crypto_lib::cryptanalysis::ghs_descent::{ECurve, Pt};
use crypto_lib::cryptanalysis::ghs_full_attack::{run_full_attack, AttackOptions};
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

fn toy_m1_instance() -> (TrapdoorCurve, ECurve, Pt, Pt) {
    // Same F_{2^6} m=1 scenario as examples/ghs_attack_demo.rs.
    let irr = IrreduciblePoly {
        degree: 6,
        low_terms: vec![0, 1],
    };
    let big_n = 6u32;
    let n = 2u32;
    let l = 3u32;
    let tower = FieldTower::new(big_n, n, l, irr.clone());
    let mut b_pick = None;
    for candidate in 2u64..(1u64 << 6) {
        let cand = F2mElement::from_biguint(&BigUint::from(candidate), big_n);
        if tower.is_in_subfield(&cand, l) && !tower.is_in_subfield(&cand, 1) {
            b_pick = Some(cand);
            break;
        }
    }
    let b = b_pick.expect("subfield curve");
    let a = F2mElement::one(big_n);
    let tc = TrapdoorCurve {
        big_n,
        n,
        l,
        big_irr: irr.clone(),
        a: a.clone(),
        b: b.clone(),
        sqrt_b: tower.sqrt(&b),
        row: DescentRow::default(),
        full_audit: None,
    };
    let curve = ECurve::new(big_n, irr, a, b);
    let mut p_opt = None;
    for xi in 1u64..(1u64 << 6) {
        let x = F2mElement::from_biguint(&BigUint::from(xi), big_n);
        for yi in 1u64..(1u64 << 6) {
            let y = F2mElement::from_biguint(&BigUint::from(yi), big_n);
            let cand = Pt::Aff {
                x: x.clone(),
                y: y.clone(),
            };
            if curve.is_on_curve(&cand) {
                p_opt = Some(cand);
                break;
            }
        }
        if p_opt.is_some() {
            break;
        }
    }
    let p = p_opt.expect("a point on E(K) exists");
    let mut q = Pt::Inf;
    for k in 2u32..50 {
        let cand_q = curve.scalar_mul(&p, &BigUint::from(k));
        if !matches!(cand_q, Pt::Inf) {
            q = cand_q;
            break;
        }
    }
    assert!(!matches!(q, Pt::Inf), "need non-identity Q");
    (tc, curve, p, q)
}

fn main() {
    let (tc, curve, p, q) = toy_m1_instance();
    let opts = AttackOptions {
        target_magic: 1,
        walk_max_visited: 32,
        walk_degrees: vec![2, 3],
        ecdlp_bound: BigUint::from(200u32),
    };

    let report = run_full_attack(&tc, &p, &q, &opts);
    println!(
        "ghs run_full_attack smoke: ecdlp_solved={} recovered={:?}",
        report.ecdlp_solved, report.recovered_scalar
    );
    println!(
        "inventory: public GHS fixtures are F_2^6 toys / screens; hot work is descent+HCDLP, not ECC ladder"
    );

    let k = BigUint::from(0x1234_5678u64);
    let mut smul_walls = Vec::new();
    let mut smul_insns = Vec::new();
    let smul_reps = 2000usize;
    for _ in 0..5 {
        let _ = black_box(curve.scalar_mul(&p, &k));
        let i0 = macos_pmu::read_insn();
        let t0 = Instant::now();
        for _ in 0..smul_reps {
            let _ = black_box(curve.scalar_mul(&p, &k));
        }
        let ns = t0.elapsed().as_nanos() as f64 / smul_reps as f64;
        let i1 = macos_pmu::read_insn();
        smul_walls.push(ns);
        smul_insns.push((i1.saturating_sub(i0)) as f64 / smul_reps as f64);
    }

    let mut atk_walls = Vec::new();
    let mut atk_insns = Vec::new();
    let atk_reps = 3usize;
    for _ in 0..5 {
        let _ = black_box(run_full_attack(&tc, &p, &q, &opts));
        let i0 = macos_pmu::read_insn();
        let t0 = Instant::now();
        for _ in 0..atk_reps {
            let _ = black_box(run_full_attack(&tc, &p, &q, &opts));
        }
        let ns = t0.elapsed().as_nanos() as f64 / atk_reps as f64;
        let i1 = macos_pmu::read_insn();
        atk_walls.push(ns);
        atk_insns.push((i1.saturating_sub(i0)) as f64 / atk_reps as f64);
    }

    let sw = med(smul_walls);
    let si = med(smul_insns);
    let aw = med(atk_walls);
    let ai = med(atk_insns);
    println!("GHS F_2^6 public fixture (wall + instructions)");
    println!("  ECurve::scalar_mul : med {sw:.0} ns/op   ({si:.0} insn)");
    println!("  run_full_attack    : med {aw:.0} ns/run  ({ai:.0} insn)");
    println!(
        "  scalar_mul share   : {:.2}% of full-attack wall",
        100.0 * sw / aw.max(1.0)
    );
    println!("verdict: REJECT GHS ECC-ladder micros (not hot on public fixtures)");
}
