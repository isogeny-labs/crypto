#[path = "../hec_anchor_relations_20261114/anchor.rs"]
mod anchor;
#[path = "../hec_confluent_20261116/confluent.rs"]
mod confluent;
#[path = "../hec_anchor_relations_20261114/dense.rs"]
mod dense;
mod solver;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use solver::{solve, Kernel, Report};
fn compare(a: &Report, b: &Report) {
    assert_eq!(
        (
            a.k,
            a.status,
            a.visited,
            a.profiles,
            a.admitted,
            a.rank,
            a.descent,
            a.source_support,
            a.target_support
        ),
        (
            b.k,
            b.status,
            b.visited,
            b.profiles,
            b.admitted,
            b.rank,
            b.descent,
            b.source_support,
            b.target_support
        )
    );
    assert_eq!(a.rows, b.rows, "exact_consumed_original_equations");
}
fn main() {
    let (mut pairs, mut calls, mut verified) = (0, 0, 0);
    for (g, p, h, selected) in [
        (2, 251, 8, vec![4, 1, 1]),
        (3, 211, 9, vec![2, 2, 1, 1, 1]),
        (4, 61, 10, vec![2, 2, 2, 2]),
    ] {
        let inst = hec_native_ic::instance::pick_instance(p, g).unwrap();
        for (index, expected) in [inst.ell / 7 + 19, inst.ell / 11 + 29]
            .into_iter()
            .enumerate()
        {
            let target = inst.curve.mul(&inst.d1, expected);
            for (label, d, parts) in [
                ("selected", 2, selected.clone()),
                ("distinct_d2", 2, vec![1; g + 4]),
                ("distinct_d0", 0, vec![1; g + 1]),
            ] {
                let seed = 2026111607 + index as u64;
                let a = solve(&inst, &target, h, d, &parts, seed, Kernel::Gaussian, true);
                let b = solve(&inst, &target, h, d, &parts, seed, Kernel::Direct, true);
                let ap = solve(&inst, &target, h, d, &parts, seed, Kernel::Gaussian, false);
                let bp = solve(&inst, &target, h, d, &parts, seed, Kernel::Direct, false);
                compare(&a, &b);
                compare(&a, &ap);
                compare(&b, &bp);
                if let Some(k) = a.k {
                    assert_eq!(k, expected);
                    verified += 4;
                }
                calls += 4;
                pairs += 1;
                println!("PAIR g={g} p={p} c={} ell={} h={h} target_index={index} family={label} d={d} parts={parts:?} status={} k={:?} visited={} profiles={} admitted={} rank={} descent_steps={} source_support={} target_support={} gaussian_prepared={} direct_prepared={} direct_unused={} cache_muls={} cache_doubles={} profile_adds={} gray_adds={} equations_exact=true gate_production_exact=true source_verified={} expected_checked_outside_API=true",inst.c,inst.ell,a.status,a.k,a.visited,a.profiles,a.admitted,a.rank,a.descent,a.source_support,a.target_support,a.prepared,b.prepared,b.prepared-b.visited,b.cache_muls,b.cache_doubles,b.profile_adds,b.gray_adds,a.k.is_some());
            }
        }
    }
    assert_eq!((pairs, calls), (18, 72));
    println!("TERMINAL paired_inputs={pairs} solve_calls={calls} verified_calls={verified} retained_noncompletions={} timing_claim=false",calls-verified);
}
