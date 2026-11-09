//! Target-free prefix of the chained decomposition systems, and what a
//! cached prefix echelon would leave of the inherited-F4 root.
//!
//! For `m ≥ 3` the chain `S₃(x₁,x₂,e₁), …, S₃(e_{m−2},x_m,x_R)` carries the
//! target only in its last link.  The Macaulay rows of the other links are
//! the same for every target.  Modes:
//!
//! - `shape`: for targets `1..=T`, how many generators agree with target 1
//!   (the target-free prefix), and the root Macaulay shape (rows, columns,
//!   rank) of the prefix, of the target link and of the whole system.
//! - `root`: per target, the from-scratch root (`from_system_with`) against
//!   the prefix-cached root (`from_system_with_prefix`): stage word XORs,
//!   wall time, and an exact row-space check.
//! - `stage`: whole decompositions (`groebner_decompose`, inherited F4) with
//!   the prefix cache off and on, interleaved per target: word XORs,
//!   CPU time, verdicts compared.

use crypto_lib::cryptanalysis::inherited_f4::{
    f4_root_elimination_thread, macaulay_shape, prefix_root_counts, set_prefix_root, ReducedBasis,
};
use crypto_lib::cryptanalysis::koblitz_groebner::{
    f4_word_ops_thread, FieldStructure, SolverEngine,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_standard_subspace_factor_base, decomposition_solver_equations, groebner_decompose,
    KoblitzCurve,
};
use crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolPoly;
use num_bigint::BigUint;
use std::time::Instant;

fn flag(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn cpu_ns() -> u128 {
    // SAFETY: `getrusage` writes into the zeroed struct it is handed.
    let usage = unsafe {
        let mut usage: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_SELF, &mut usage);
        usage
    };
    let tv = |t: libc::timeval| t.tv_sec as u128 * 1_000_000_000 + t.tv_usec as u128 * 1_000;
    tv(usage.ru_utime) + tv(usage.ru_stime)
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let a: u8 = flag("--a").and_then(|v| v.parse().ok()).unwrap_or(0);
    let n: u32 = flag("--n").and_then(|v| v.parse().ok()).unwrap_or(17);
    let dim: u32 = flag("--dim").and_then(|v| v.parse().ok()).unwrap_or(6);
    let m: usize = flag("--m").and_then(|v| v.parse().ok()).unwrap_or(3);
    let targets: u64 = flag("--targets").and_then(|v| v.parse().ok()).unwrap_or(8);
    let first: u64 = flag("--first").and_then(|v| v.parse().ok()).unwrap_or(1);
    let budget: usize = flag("--budget")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20_000);
    let degree: u32 = flag("--degree").and_then(|v| v.parse().ok()).unwrap_or(3);
    let support_local = flag("--multipliers").as_deref() != Some("occurring");
    let kc = KoblitzCurve::new(a, n).expect("curve has a usable subgroup");
    let fb = build_standard_subspace_factor_base(&kc, dim).expect("factor base");
    let index_of = fb.index_map();
    let st = FieldStructure::new(kc.n, &kc.curve.irreducible);
    let engine = SolverEngine::InheritedF4 { max_degree: 3 };
    let target = |k: u64| kc.mul(kc.generator(), &BigUint::from(k));
    let system = |k: u64| -> (Vec<F2BoolPoly>, usize) {
        decomposition_solver_equations(&kc, &fb, &st, &target(k), m, engine)
            .expect("a supported target")
    };
    let cell = format!("K{a}/2^{n} dim {dim} m {m}");
    let ks: Vec<u64> = (first..first + targets).collect();
    match mode.as_str() {
        "shape" => {
            let (s1, n_vars) = system(ks[0]);
            for &k in &ks {
                let (s, _) = system(k);
                let same = s.iter().zip(&s1).take_while(|(x, y)| x == y).count();
                let linear = s
                    .iter()
                    .filter(|p| p.terms.iter().all(|t| t.mask.count_ones() <= 1))
                    .count();
                let prefix = n as usize * (m - 2);
                let full = macaulay_shape(&s, n_vars, degree, support_local);
                let pre = macaulay_shape(&s[..prefix], n_vars, degree, support_local);
                let link = macaulay_shape(&s[prefix..], n_vars, degree, support_local);
                println!(
                    "{{\"cell\":\"{cell}\",\"k\":{k},\"n_vars\":{n_vars},\"generators\":{},\"same_as_first\":{same},\"prefix\":{prefix},\"linear\":{linear},\"full\":{full:?},\"prefix_shape\":{pre:?},\"link_shape\":{link:?}}}",
                    s.len()
                );
            }
        }
        "root" => {
            let prefix = n as usize * (m - 2);
            for &k in &ks {
                let (s, n_vars) = system(k);
                let e0 = f4_root_elimination_thread();
                let t0 = Instant::now();
                let (scratch, c0) =
                    ReducedBasis::from_system_with(&s, n_vars, degree, support_local)
                        .expect("fits");
                let t_scratch = t0.elapsed().as_secs_f64() * 1e3;
                let e_scratch = f4_root_elimination_thread() - e0;
                let (cached_before, _) = prefix_root_counts();
                let t1 = Instant::now();
                let (cached, c1) = ReducedBasis::from_system_with_prefix(
                    &s,
                    prefix,
                    n_vars,
                    degree,
                    support_local,
                )
                .expect("fits");
                let t_cached = t1.elapsed().as_secs_f64() * 1e3;
                let (cached_after, once) = prefix_root_counts();
                assert!(
                    cached.same_row_space(&scratch),
                    "{cell} k={k}: prefix root differs from the from-scratch root"
                );
                println!(
                    "{{\"cell\":\"{cell}\",\"k\":{k},\"rank\":{},\"columns\":{},\"scratch_xors\":{},\"scratch_root_elim\":{e_scratch},\"prefix_xors\":{},\"prefix_once_total\":{once},\"prefix_cache_built\":{},\"scratch_ms\":{t_scratch:.3},\"prefix_ms\":{t_cached:.3},\"same_row_space\":true}}",
                    scratch.rank(),
                    scratch.columns(),
                    c0.word_ops(),
                    c1.word_ops(),
                    cached_after != cached_before,
                );
            }
        }
        "stage" => {
            let index_of = &index_of;
            let mut tot = [(0u64, 0u128); 2];
            for &k in &ks {
                let t = target(k);
                let mut verdicts = Vec::new();
                // Alternate arm order per target.
                let order: [bool; 2] = if k % 2 == 0 {
                    [false, true]
                } else {
                    [true, false]
                };
                for &on in &order {
                    set_prefix_root(on);
                    let x0 = f4_word_ops_thread();
                    let c0 = cpu_ns();
                    let (found, stats) =
                        groebner_decompose(&kc, &fb, index_of, &st, &t, m, engine, budget);
                    let cpu = cpu_ns() - c0;
                    let x = f4_word_ops_thread() - x0;
                    let arm = usize::from(on);
                    tot[arm].0 += x;
                    tot[arm].1 += cpu;
                    println!(
                        "{{\"cell\":\"{cell}\",\"k\":{k},\"arm\":\"{}\",\"word_xors\":{x},\"cpu_ms\":{:.3},\"found\":{},\"splits\":{},\"reductions\":{},\"exhausted\":{}}}",
                        if on { "prefix" } else { "scratch" },
                        cpu as f64 / 1e6,
                        found.is_some(),
                        stats.splits,
                        stats.reductions,
                        stats.exhausted,
                    );
                    verdicts.push((
                        found.map(|mut v| {
                            v.sort_unstable();
                            v
                        }),
                        stats.splits,
                        stats.reductions,
                        stats.exhausted,
                    ));
                }
                assert_eq!(verdicts[0], verdicts[1], "{cell} k={k}: verdicts differ");
            }
            set_prefix_root(false);
            let (_, once) = prefix_root_counts();
            println!(
                "{{\"cell\":\"{cell}\",\"summary\":true,\"targets\":{},\"scratch_word_xors\":{},\"prefix_word_xors\":{},\"prefix_once_total\":{once},\"ratio_incl_once\":{:.4},\"ratio_excl_once\":{:.4},\"scratch_cpu_ms\":{:.1},\"prefix_cpu_ms\":{:.1}}}",
                ks.len(),
                tot[0].0,
                tot[1].0,
                tot[0].0 as f64 / tot[1].0 as f64,
                tot[0].0 as f64 / (tot[1].0 - once) as f64,
                tot[0].1 as f64 / 1e6,
                tot[1].1 as f64 / 1e6,
            );
        }
        other => panic!("unknown mode {other:?}; use shape, root or stage"),
    }
}
