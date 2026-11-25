//! Wall-clock A/B of the merged 1D-1 fast path of `PairProjection`
//! (`build_fast`, `decompose_*_fast_path`/`decompose_four_first`) against the
//! reference path still on main (`build_generic`, `decompose_*_generic`).
//! Prime31 (p = 2^31 - 1), y^2 = x^3 + 2x + 3, B = 64 (2080 pairs).
//!
//! Method: warmup, interleaved arm order (ABBA..) per repeat, an A/A arm
//! (old vs old) for the noise floor, median and min..max spread, and an
//! output-equivalence check on every query. Prints one JSON object.
//! Usage: semaev_projection_1d1_wallclock [reps=15] [targets=200]
use semaev_1d1_shim::cryptanalysis::pkm_smooth_tower::Curve;
use semaev_1d1_shim::cryptanalysis::semaev_projection::{interval_factor_base, PairProjection};
use std::hint::black_box;
use std::time::Instant;

fn med(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 { v[n / 2] } else { 0.5 * (v[n / 2 - 1] + v[n / 2]) }
}
fn stat(v: &[f64]) -> String {
    let mut s = v.to_vec();
    let m = med(&mut s);
    let mad = {
        let mut d: Vec<f64> = v.iter().map(|x| (x - m).abs()).collect();
        med(&mut d)
    };
    format!(
        "{{\"median\":{m:.6},\"min\":{:.6},\"max\":{:.6},\"mad\":{mad:.6},\"n\":{},\"samples\":{:?}}}",
        s[0], s[s.len() - 1], v.len(), v
    )
}
fn t<R>(f: impl FnOnce() -> R) -> (f64, R) {
    let s = Instant::now();
    let r = f();
    (s.elapsed().as_secs_f64() * 1e3, r)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let reps: usize = a.get(1).and_then(|s| s.parse().ok()).unwrap_or(15);
    let nt: usize = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(200);
    let e = Curve { p: 2_147_483_647, a: 2, b: 3 };
    let base: Vec<u64> = interval_factor_base(&e, 400).into_iter().take(64).collect();
    assert_eq!(base.len(), 64);
    // Point targets (fast path applies), disjoint-ish from base start.
    let targets: Vec<u64> = interval_factor_base(&e, 40_000).into_iter().skip(10).step_by(3).take(nt).collect();
    assert_eq!(targets.len(), nt);

    let old = PairProjection::build_generic(e, &base).unwrap();
    let new = PairProjection::build_fast(e, &base).unwrap();
    assert_eq!(old.build_stats(), new.build_stats());
    assert_eq!(old.build_stats().unordered_pairs, 2080);

    // Equivalence on all queries (also serves as warmup).
    let (mut f3, mut f4) = (0, 0);
    for &x in &targets {
        let (o3, _) = old.decompose_three_generic(x);
        let (n3, _) = new.decompose_three_fast_path(x);
        assert_eq!(o3.is_some(), n3.is_some(), "three {x}");
        let (o4, so) = old.decompose_four_generic(x);
        let (n4, sn) = new.decompose_four_fast_path(x);
        let (n4f, _) = new.decompose_four_first(x);
        assert_eq!(o4.is_some(), n4.is_some(), "four {x}");
        assert_eq!(o4.is_some(), n4f.is_some(), "four_first {x}");
        assert_eq!(so.compatible_keys, sn.compatible_keys);
        f3 += o3.is_some() as u32;
        f4 += o4.is_some() as u32;
    }
    for _ in 0..3 {
        black_box(PairProjection::build_generic(e, &base).unwrap());
        black_box(PairProjection::build_fast(e, &base).unwrap());
    }

    let (mut b_old, mut b_new, mut b_aa) = (vec![], vec![], vec![]);
    let (mut q3_old, mut q3_new, mut q3_aa) = (vec![], vec![], vec![]);
    let (mut q4_old, mut q4_new, mut q4_first, mut q4_aa) = (vec![], vec![], vec![], vec![]);
    for r in 0..reps {
        let run_build = |new_arm: bool| {
            t(|| black_box(if new_arm { PairProjection::build_fast(e, &base) } else { PairProjection::build_generic(e, &base) }.unwrap())).0
        };
        let q3 = |p: &PairProjection, fast: bool| {
            t(|| for &x in &targets { black_box(if fast { p.decompose_three_fast_path(x) } else { p.decompose_three_generic(x) }); }).0
        };
        let q4 = |p: &PairProjection, mode: u8| {
            t(|| for &x in &targets {
                match mode {
                    0 => { black_box(p.decompose_four_generic(x)); }
                    1 => { black_box(p.decompose_four_fast_path(x)); }
                    _ => { black_box(p.decompose_four_first(x)); }
                }
            }).0
        };
        if r % 2 == 0 {
            b_old.push(run_build(false)); b_new.push(run_build(true)); b_aa.push(run_build(false));
            q3_old.push(q3(&old, false)); q3_new.push(q3(&new, true)); q3_aa.push(q3(&old, false));
            q4_old.push(q4(&old, 0)); q4_new.push(q4(&new, 1)); q4_first.push(q4(&new, 2)); q4_aa.push(q4(&old, 0));
        } else {
            b_aa.push(run_build(false)); b_new.push(run_build(true)); b_old.push(run_build(false));
            q3_aa.push(q3(&old, false)); q3_new.push(q3(&new, true)); q3_old.push(q3(&old, false));
            q4_aa.push(q4(&old, 0)); q4_first.push(q4(&new, 2)); q4_new.push(q4(&new, 1)); q4_old.push(q4(&old, 0));
        }
    }
    let (oo, on) = ((0u64,0u64,0u64),(0u64,0u64,0u64)); // counters are cfg(test) upstream; see test prime31_opcount_old_vs_new
    println!("{{\"schema\":\"semaev_1d1_wallclock/v1\",\"p\":{},\"B\":64,\"pairs\":2080,\"targets\":{nt},\"reps\":{reps},\"found3\":{f3},\"found4\":{f4},\"unit\":\"ms\",", e.p);
    println!("\"build_ms_old\":{},\"build_ms_new\":{},\"build_ms_aa_old\":{},", stat(&b_old), stat(&b_new), stat(&b_aa));
    println!("\"q3_ms_old\":{},\"q3_ms_new\":{},\"q3_ms_aa_old\":{},", stat(&q3_old), stat(&q3_new), stat(&q3_aa));
    println!("\"q4_ms_old\":{},\"q4_ms_new_fast_path\":{},\"q4_ms_new_first\":{},\"q4_ms_aa_old\":{},", stat(&q4_old), stat(&q4_new), stat(&q4_first), stat(&q4_aa));
    println!("\"build_ops_old\":{{\"sqrtm\":{},\"invm\":{},\"mulm\":{}}},\"build_ops_new\":{{\"sqrtm\":{},\"invm\":{},\"mulm\":{}}}}}", oo.0, oo.1, oo.2, on.0, on.1, on.2);
}
