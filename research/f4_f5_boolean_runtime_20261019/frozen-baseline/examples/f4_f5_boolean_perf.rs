//! Matched Boolean matrix, complete-root and decomposition comparisons.
//! See research/f4_f5_boolean_perf_20261018/PROTOCOL.md for accounting.
use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::{
    ic_boundary::{BinaryGroup, FactorBase, GroupOps, OracleCounters},
    ic_framework::{
        rootfind::RootfindMitmOracle,
        stages::{DecompositionOracle, InstanceCtx, Params},
    },
    koblitz_fast::FastCurve,
    koblitz_groebner::{
        build_decomposition_system, f4_profile, f4_profile_reset, matrix_f4_f2_counted,
        solve_boolean_system, FieldStructure, SolveOptions, SolverEngine,
    },
    koblitz_index_calculus::{
        build_frobenius_factor_base, groebner_decompose, groebner_decompose_f6_ic,
        groebner_decompose_f6_ic_pair_index, KoblitzCurve, PairSumTable,
    },
    matrix_f5_f2::{canonical_row_space_fingerprint, matrix_f5_f2_timed},
    mq_fes::{gray_incremental_find_all, moebius_find_all, QuadraticForm},
    mq_monica::monica_find_all,
    pq_f4_f2::{groebner_basis_f4, solutions_from_reduced_basis},
    pq_groebner_f2::F2BoolPoly,
};
use num_bigint::BigUint;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use std::time::Instant;
#[path = "../tools/generalized-rootfinder/src/solver.rs"]
#[allow(dead_code)]
mod generalized;

fn digest(polys: &[F2BoolPoly]) -> String {
    let mut h = blake3::Hasher::new();
    for p in polys {
        h.update(&(p.n_vars as u64).to_le_bytes());
        h.update(&(p.terms.len() as u64).to_le_bytes());
        for t in &p.terms {
            h.update(&t.mask.to_le_bytes());
        }
    }
    h.finalize().to_hex().to_string()
}

fn flag(name: &str, default: &str) -> String {
    let args: Vec<_> = std::env::args().collect();
    args.iter()
        .position(|s| s == name)
        .map(|i| args[i + 1].clone())
        .unwrap_or_else(|| default.into())
}

// Valgrind's amd64 client-request ABI. Native execution leaves the default
// result unchanged; Callgrind handles all threads (--separate-threads=no).
#[cfg(target_arch = "x86_64")]
fn callgrind(request: usize, argument: usize) {
    let args = [request, argument, 0, 0, 0, 0];
    let mut result = 0usize;
    unsafe {
        std::arch::asm!(
            "rol rdi, 3", "rol rdi, 13", "rol rdi, 61", "rol rdi, 51", "xchg rbx, rbx",
            in("rax") args.as_ptr(), inout("rdx") result, lateout("rdi") _, options(nostack),
        );
    }
    std::hint::black_box(result);
}
#[cfg(not(target_arch = "x86_64"))]
fn callgrind(_: usize, _: usize) {
    panic!("profiling ABI requires x86_64");
}
fn profile_start() {
    if flag("--profile", "0") == "1" {
        callgrind(0x4354_0001, 0);
        callgrind(0x4354_0004, 0);
    }
}
fn profile_stop(label: &Value) {
    if flag("--profile", "0") == "1" {
        let label = std::ffi::CString::new(label.to_string()).unwrap();
        callgrind(0x4354_0003, label.as_ptr() as usize);
        callgrind(0x4354_0005, 0);
    }
}

fn main() {
    let n: u32 = flag("--n", "23").parse().unwrap();
    let seed: u64 = flag("--seed", "230").parse().unwrap();
    let repeat: usize = flag("--repeats", "1").parse().unwrap();
    let arm = flag("--arm", "matrix-f4");
    let degree: u32 = flag("--degree", "4").parse().unwrap();
    let target_count: usize = flag("--targets", "1").parse().unwrap();
    let kc = KoblitzCurve::new(1, n).unwrap();
    let fb = build_frobenius_factor_base(&kc, 0).unwrap();
    let st = FieldStructure::new(n, &kc.curve.irreducible);
    let index = fb.index_map();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut targets = Vec::new();
    while targets.len() < target_count {
        let t = (0..2).fold(BinaryPoint::Infinity, |acc, _| {
            kc.add(&acc, &fb.points[rng.gen_range(0..fb.points.len())])
        });
        if matches!(t, BinaryPoint::Affine { .. }) {
            targets.push(t);
        }
    }
    if flag("--random", "0") == "1" {
        targets = (0..target_count)
            .map(|i| {
                kc.mul(
                    kc.generator(),
                    &BigUint::from(1 + (i as u64 * 2_654_435_761) % 1_000_003),
                )
            })
            .collect();
    }
    let fast = FastCurve::new(&kc.curve).unwrap();
    let group = BinaryGroup(&fast);
    let count = fb.points.len();
    let native_fb = FactorBase::from_column_map(
        "same explicit signed base".into(),
        fb.points.iter().map(|p| fast.lift(p)).collect(),
        (0..count).collect(),
        vec![1; count],
        count,
        |p| p.pack(),
        |p| fast.neg(*p).pack(),
        |p| p.x,
    )
    .unwrap();
    let ctx = InstanceCtx {
        group: &group,
        generator: fast.lift(kc.generator()),
        target: fast.lift(&targets[0]),
        r: kc.subgroup_order.to_u64_digits()[0],
        cofactor: kc.cofactor.to_u64_digits()[0],
        group_order: kc.group_order.to_u64_digits()[0],
        name: format!("K1 n{n}"),
        field_degree: Some(n),
    };
    let mut oracle = RootfindMitmOracle::new(2);
    let mut setup_ops = GroupOps::default();
    let setup = Instant::now();
    if arm.starts_with("rootfind-") {
        profile_start();
    }
    let table = (arm == "rootfind-table").then(|| PairSumTable::build(&kc, &fb).unwrap());
    if arm == "rootfind-mitm" {
        oracle
            .prepare(&ctx, &native_fb, &Params::default(), &mut setup_ops)
            .unwrap();
    }
    if arm.starts_with("rootfind-") {
        profile_stop(&json!(["setup", arm, n, seed]));
    }
    let pair_setup_ns = setup.elapsed().as_nanos();
    for (target_id, target) in targets.iter().enumerate() {
        let BinaryPoint::Affine { x, .. } = target else {
            panic!("infinite target")
        };
        let sys = build_decomposition_system(&fb.subspace_basis, x, &kc.curve.b, 2, &st).unwrap();
        let input = digest(&sys.equations);
        let generalized_problem = json!({"field":{"kind":"prime","characteristic":2},
            "variables":(0..sys.n_vars).map(|i|format!("x{i}")).collect::<Vec<_>>(),
            "equations":sys.equations.iter().map(|p|p.terms.iter().map(|t|json!({"coefficient":1,
                "powers":(0..sys.n_vars).map(|i|(t.mask>>i)&1).collect::<Vec<_>>()})).collect::<Vec<_>>()).collect::<Vec<_>>(),
            "L":{"coefficients":[0,1,1]},"method":"auto"});
        for r in 0..repeat {
            f4_profile_reset();
            let mut row = json!({"arm":arm,"n":n,"seed":seed,"target":target_id,
                "target_x":format!("{x:?}"),"n_vars":sys.n_vars,"repeat":r,"degree":degree,
                "input_digest":input,"random":flag("--random","0")=="1"});
            row["table_write_once"] =
                json!(std::env::var("KIC_GF2_WRITE_TABLE_ONCE").as_deref() == Ok("1"));
            if arm.starts_with("matrix-") {
                let started = Instant::now();
                profile_start();
                let (rows, ops, report) = if arm == "matrix-f4" {
                    let (rows, ops) =
                        matrix_f4_f2_counted(&sys.equations, sys.n_vars, degree).unwrap();
                    (rows, ops, json!(f4_profile()))
                } else {
                    let (rows, report, t) =
                        matrix_f5_f2_timed(&sys.equations, sys.n_vars, degree).unwrap();
                    let ops = report.word_ops();
                    (rows, ops, json!({"counts":report,"timings":t}))
                };
                profile_stop(&json!([arm, n, seed, target_id, r]));
                row["elapsed_ns"] = json!(started.elapsed().as_nanos());
                row["word_ops"] = json!(ops);
                row["profile"] = report;
                row["output_digest"] = json!(digest(&rows));
                row["row_space"] = json!(canonical_row_space_fingerprint(&rows).unwrap());
                row["rank"] = json!(rows.len());
            } else if arm.starts_with("decomp-") || arm.starts_with("rootfind-") {
                let started = Instant::now();
                profile_start();
                let (hit, stats) = match arm.as_str() {
                    "decomp-f4" => groebner_decompose(
                        &kc,
                        &fb,
                        &index,
                        &st,
                        target,
                        2,
                        SolverEngine::MatrixF4 { max_degree: 3 },
                        20_000,
                    ),
                    "decomp-f5" => groebner_decompose(
                        &kc,
                        &fb,
                        &index,
                        &st,
                        target,
                        2,
                        SolverEngine::MatrixF5 { max_degree: 3 },
                        20_000,
                    ),
                    "decomp-f6" => groebner_decompose_f6_ic(
                        &kc,
                        &fb,
                        &index,
                        &st,
                        target,
                        2,
                        SolverEngine::default(),
                        20_000,
                    ),
                    "decomp-f6-pairs" => groebner_decompose_f6_ic_pair_index(
                        &kc,
                        &fb,
                        &index,
                        &st,
                        target,
                        2,
                        SolverEngine::default(),
                        20_000,
                    ),
                    "decomp-generalized" => {
                        let fresh =
                            build_decomposition_system(&fb.subspace_basis, x, &kc.curve.b, 2, &st)
                                .unwrap();
                        let problem = json!({"field":{"kind":"prime","characteristic":2},
                            "variables":(0..fresh.n_vars).map(|i|format!("x{i}")).collect::<Vec<_>>(),
                            "equations":fresh.equations.iter().map(|p|p.terms.iter().map(|t|json!({"coefficient":1,
                                "powers":(0..fresh.n_vars).map(|i|(t.mask>>i)&1).collect::<Vec<_>>()})).collect::<Vec<_>>()).collect::<Vec<_>>(),
                            "L":{"coefficients":[0,1,1]}});
                        let result = generalized::solve_json(
                            &problem,
                            &generalized::Limits {
                                max_work: 1u64 << fresh.n_vars,
                                max_solutions: usize::MAX,
                                ..Default::default()
                            },
                        )
                        .unwrap();
                        assert_eq!(result["status"], "complete");
                        let mut found = None;
                        'roots: for root in result["solutions"].as_array().unwrap() {
                            let bits = root.as_array().unwrap().iter().enumerate().fold(
                                0u64,
                                |mask, (i, v)| if v == "0x1" { mask | (1u64 << i) } else { mask },
                            );
                            let xs = (0..2)
                                .map(|i| fresh.summand_x(&fb.subspace_basis, bits, i, n))
                                .collect::<Vec<_>>();
                            let a = xs[0].raw_bits().first().copied().unwrap_or(0);
                            let b = xs[1].raw_bits().first().copied().unwrap_or(0);
                            if let (Some(left), Some(right)) =
                                (native_fb.x_index.get(&a), native_fb.x_index.get(&b))
                            {
                                for &i in left {
                                    for &j in right {
                                        if kc.add(&fb.points[i], &fb.points[j]) == *target {
                                            found = Some(vec![i, j]);
                                            break 'roots;
                                        }
                                    }
                                }
                            }
                        }
                        row["certificate"] = result["certificate"].clone();
                        (found, Default::default())
                    }
                    "rootfind-table" => (
                        table.as_ref().unwrap().decompose(&kc, &fb, target, 2),
                        Default::default(),
                    ),
                    "rootfind-mitm" => {
                        let mut ops = GroupOps::default();
                        let mut counters = OracleCounters::default();
                        let hit = oracle.decompose(
                            &ctx,
                            &native_fb,
                            &mut ops,
                            &mut counters,
                            fast.lift(target),
                        );
                        row["group_ops"] = json!(ops);
                        row["lookups"] = json!(counters.lookups);
                        row["setup_group_ops"] = json!(setup_ops);
                        row["setup_counts"] = json!(<RootfindMitmOracle<_> as DecompositionOracle<
                                BinaryGroup<'_>,
                            >>::setup_native(&oracle));
                        (hit, Default::default())
                    }
                    _ => panic!("unknown arm"),
                };
                profile_stop(&json!([arm, n, seed, target_id, r]));
                row["elapsed_ns"] = json!(started.elapsed().as_nanos());
                assert!(
                    !stats.exhausted && !stats.unsupported,
                    "incomplete decomposition"
                );
                if let Some(ids) = &hit {
                    assert_eq!(
                        &ids.iter()
                            .fold(BinaryPoint::Infinity, |acc, &i| kc.add(&acc, &fb.points[i])),
                        target
                    );
                }
                row["hit"] = json!(hit.is_some());
                row["indices"] = json!(hit);
                row["stats"] = json!(stats);
                row["profile"] = json!(f4_profile());
                row["pair_setup_ns"] = json!(pair_setup_ns);
            } else {
                let started = Instant::now();
                profile_start();
                let roots = match arm.as_str() {
                    "generalized-native" => {
                        let limits = generalized::Limits {
                            max_work: 1u64 << sys.n_vars,
                            max_solutions: usize::MAX,
                            ..Default::default()
                        };
                        let result =
                            generalized::solve_json(&generalized_problem, &limits).unwrap();
                        assert_eq!(result["status"], "complete");
                        row["certificate"] = result["certificate"].clone();
                        row["generalized_method"] = result["method"].clone();
                        result["solutions"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|x| {
                                x.as_array().unwrap().iter().enumerate().fold(
                                    0u64,
                                    |bits, (i, v)| {
                                        assert!(v == "0x0" || v == "0x1");
                                        if v == "0x1" {
                                            bits | (1u64 << i)
                                        } else {
                                            bits
                                        }
                                    },
                                )
                            })
                            .collect()
                    }
                    "solve-f4" | "solve-f5" | "solve-inherited" => {
                        let engine = match arm.as_str() {
                            "solve-f4" => SolverEngine::MatrixF4 { max_degree: 3 },
                            "solve-f5" => SolverEngine::MatrixF5 { max_degree: 3 },
                            _ => SolverEngine::default(),
                        };
                        let (roots, stats) = solve_boolean_system(
                            &sys.equations,
                            sys.n_vars,
                            &SolveOptions {
                                engine,
                                max_solutions: usize::MAX,
                                node_budget: 20_000,
                                ..Default::default()
                            },
                        );
                        assert!(
                            !stats.exhausted && !stats.unsupported,
                            "incomplete root solve"
                        );
                        row["stats"] = json!(stats);
                        roots
                    }
                    "generic-gb" => {
                        let (gb, stats) = groebner_basis_f4(
                            sys.equations.clone(),
                            sys.n_vars,
                            Some(std::time::Duration::from_secs(50)),
                        );
                        assert!(
                            !stats.timed_out && !stats.oversize,
                            "generic basis budget exhausted"
                        );
                        row["steps"] = json!(stats.steps);
                        solutions_from_reduced_basis(&gb, sys.n_vars, sys.n_vars, &mut 0)
                            .expect("reduced basis")
                    }
                    "roots-moebius" | "roots-gray" | "roots-monica" => {
                        let forms: Vec<_> = sys
                            .equations
                            .iter()
                            .map(|p| QuadraticForm::from_poly(p).unwrap())
                            .collect();
                        match arm.as_str() {
                            "roots-moebius" => moebius_find_all(&forms, usize::MAX).unwrap(),
                            "roots-gray" => gray_incremental_find_all(&forms, usize::MAX).unwrap(),
                            _ => monica_find_all(&forms, usize::MAX).unwrap(),
                        }
                    }
                    _ => panic!("unknown arm {arm}"),
                };
                profile_stop(&json!([arm, n, seed, target_id, r]));
                row["elapsed_ns"] = json!(started.elapsed().as_nanos());
                row["profile"] = json!(f4_profile());
                let mut roots = roots;
                roots.sort_unstable();
                assert!(roots
                    .iter()
                    .all(|&x| sys.equations.iter().all(|p| p.eval(x) == 0)));
                let forms: Vec<_> = sys
                    .equations
                    .iter()
                    .map(|p| QuadraticForm::from_poly(p).unwrap())
                    .collect();
                let mut reference = gray_incremental_find_all(&forms, usize::MAX).unwrap();
                reference.sort_unstable();
                assert_eq!(roots, reference, "complete root sets disagree");
                row["roots"] = json!(roots);
            }
            row["verified"] = json!(true);
            println!("{row}");
        }
    }
}
