//! S2 probe: paired per-system cost of the lab solver (inherited F4, in-process instruction counts)
//! and of WDSat (config (a) `-x -b`, config (b) no XG no `-b`, per-process `time -l` Ir, plus the
//! R5 emitter's own in-process Ir), on the same random abscissae.  SAT = WDSat (a) returns a model.
//! usage: s2_split_probe <n> <a> <l> <systems> <seed> [lab=1] [wda=1] [wdb=1]
//! Env: WDSAT_BIN = directory of per-size binaries (`first/l<l>/wdsat_id<N>`).  Run unsandboxed.
#[path = "../src/bin/ecbench/wdsat_s4.rs"]
mod wdsat_s4;

use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::binary_semaev_s4::symmetrised_s4_eval_b;
use crypto_lib::cryptanalysis::koblitz_groebner::{FieldStructure, SolverEngine};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_standard_subspace_factor_base, groebner_decompose, point_key, points_with_x, KoblitzCurve,
};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

extern "C" {
    fn proc_pid_rusage(pid: i32, flavor: i32, buffer: *mut u64) -> i32;
    fn getpid() -> i32;
}
/// Instructions retired by this process so far (RUSAGE_INFO_V4 `ri_instructions`).
fn ri() -> u64 {
    let mut buf = [0u64; 64];
    let rc = unsafe { proc_pid_rusage(getpid(), 4, buf.as_mut_ptr()) };
    assert_eq!(rc, 0, "proc_pid_rusage failed");
    buf[31]
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: u32 = a[1].parse().unwrap();
    let ca: u8 = a[2].parse().unwrap();
    let l: u32 = a[3].parse().unwrap();
    let systems: usize = a[4].parse().unwrap();
    let seed: u64 = a[5].parse().unwrap();
    let flag = |k: &str| !a.iter().skip(6).any(|x| x == &format!("{k}=0"));
    let (do_lab, do_a, do_b) = (flag("lab"), flag("wda"), flag("wdb"));
    let bin = std::env::var("WDSAT_BIN").expect("WDSAT_BIN");
    let kc = KoblitzCurve::new(ca, n).expect("koblitz curve");
    let irr = kc.curve.irreducible.clone();
    let b = kc.curve.b.clone();
    let frob = build_standard_subspace_factor_base(&kc, l).expect("base");
    let index_of: HashMap<(BigUint, BigUint), usize> =
        frob.points.iter().enumerate().map(|(i, p)| (point_key(p), i)).collect();
    let st = FieldStructure::new(n, &irr);
    let eng = SolverEngine::InheritedF4 { max_degree: 3 };
    let el = |x: u64| F2mElement::from_biguint(&BigUint::from(x), n);
    let mut rng = StdRng::seed_from_u64(seed);
    let check = |roots: &Vec<[u64; 3]>, xr: &F2mElement| -> u64 {
        let mut bad = 0;
        for r in roots {
            let (p, q, s) = (el(r[0]), el(r[1]), el(r[2]));
            let e1 = p.add(&q).add(&s);
            let e2 = p.mul(&q, &irr).add(&p.mul(&s, &irr)).add(&q.mul(&s, &irr));
            let e3 = p.mul(&q, &irr).mul(&s, &irr);
            if !symmetrised_s4_eval_b(&e1, &e2, &e3, xr, &b, &irr).is_zero() {
                bad += 1;
            }
        }
        bad
    };
    let mut idx = -1i64; // idx -1 = untimed lab warm-up (template build)
    let mut done = 0usize;
    while done < systems {
        let xr_bits: u64 = rng.gen_range(1..(1u64 << n));
        let xr = el(xr_bits);
        let pts = points_with_x(&kc.curve, &xr);
        if pts.is_empty() {
            continue;
        }
        let target = pts[0].clone();
        if idx < 0 {
            if do_lab {
                let _ = groebner_decompose(&kc, &frob, &index_of, &st, &target, 3, eng, 8192);
            }
            idx = 0;
            continue;
        }
        let mut line = format!("{{\"n\":{n},\"a\":{ca},\"l\":{l},\"idx\":{idx},\"x_r\":{xr_bits}");
        if do_lab {
            let t0 = ri();
            let (res, stats) = groebner_decompose(&kc, &frob, &index_of, &st, &target, 3, eng, 8192);
            let t1 = ri();
            line += &format!(",\"lab_ir\":{},\"lab_found\":{},\"lab_exhausted\":{},\"lab_reductions\":{}", t1 - t0, res.is_some(), stats.exhausted, stats.reductions);
        }
        let e0 = ri();
        let em = wdsat_s4::emit(n, l, &irr, &b, &xr);
        let e1 = ri();
        line += &format!(",\"emit_ir\":{},\"vars\":{},\"eqs\":{}", e1 - e0, em.n_total, em.n_eqs);
        for (nm, on, tune) in [
            ("a", do_a, wdsat_s4::WdTune { sym: true, xg: true, all: false }),
            ("b", do_b, wdsat_s4::WdTune { sym: false, xg: false, all: false }),
        ] {
            if !on {
                continue;
            }
            match wdsat_s4::solve_s4(&bin, n, l, &irr, &b, &xr, &format!("s{idx}{nm}"), tune) {
                Ok(o) => {
                    let bad = check(&o.roots, &xr);
                    line += &format!(",\"wd{nm}_ir\":{},\"wd{nm}_sat\":{},\"wd{nm}_bad\":{},\"wd{nm}_inexact\":{},\"wd{nm}_refuted_emitter\":{}", o.ir, !o.roots.is_empty(), bad, o.inexact, o.refuted_by_emitter);
                }
                Err(e) => line += &format!(",\"wd{nm}_error\":{:?}", e),
            }
        }
        line += "}";
        println!("{line}");
        idx += 1;
        done += 1;
    }
}
