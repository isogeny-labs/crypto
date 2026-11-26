//! Matched production-oracle panel for prime-field point decomposition.
//!
//! This is an engineering benchmark, not a cryptanalytic or PCC claim.  It
//! runs one thread, rebuilds every oracle for every repeat, and validates all
//! witnesses outside the timed regions.  The default curves are the frozen
//! p31 exponent-control curve and the frozen ordinary p61 curve used by the
//! isogeny-class study.  Results produced on an ordinary shared host remain
//! `[pcc:unverified]`.
//!
//! Build with source provenance and pin the process externally, for example:
//!
//! ```text
//! CRYPTO_BUILD_GIT_COMMIT=$(git rev-parse HEAD) \
//! CRYPTO_BUILD_GIT_DIRTY=$(test -z "$(git status --porcelain)" && echo 0 || echo 1) \
//! CRYPTO_BUILD_RUSTFLAGS='-C target-cpu=native' \
//! CRYPTO_BUILD_RUSTC_VERSION="$(rustc --version)" \
//! RUSTFLAGS='-C target-cpu=native' cargo build --release \
//!   --example rootfind_production_panel
//! taskset -c 0 target/release/examples/rootfind_production_panel \
//!   --curves p31,p61 --size 64 --hits 32 --misses 32 --repeats 5 \
//!   --seed 59297
//! ```

use std::collections::{BTreeMap, HashSet};
use std::hint::black_box;
use std::time::Instant;

use crypto_lib::build_provenance;
use crypto_lib::cryptanalysis::ic_boundary::{
    fp_p31_subexp_mersenne_instance, prime_factor_base_cleared, CountedGroup, FactorBase, GroupOps,
    OracleCounters, PrimeCurve, PrimeInstance, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::RootfindSubtractOracle;
use crypto_lib::cryptanalysis::ic_framework::rootfind::RootfindMitmOracle;
use crypto_lib::cryptanalysis::ic_framework::rootfind_floor::RootfindFloorOracle;
use crypto_lib::cryptanalysis::ic_framework::rootfind_kmitm::RootfindKMitmOracle;
use crypto_lib::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};
use crypto_lib::hash::sha256::sha256;
use serde_json::{json, Value};

const M: u32 = 3;
const LANE_WIDTH: usize = 8;
const DEFAULT_SEED: u64 = 59_297;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Arm {
    Mitm,
    Subtract,
    SubtractAa,
    FloorScalar,
    FloorLanesScalar,
    FloorLanesPreferred,
    KmitmScalar,
    KmitmLanesPreferred,
}

impl Arm {
    const ALL: [Self; 8] = [
        Self::Mitm,
        Self::Subtract,
        Self::SubtractAa,
        Self::FloorScalar,
        Self::FloorLanesScalar,
        Self::FloorLanesPreferred,
        Self::KmitmScalar,
        Self::KmitmLanesPreferred,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Mitm => "rootfind-mitm",
            Self::Subtract => "rootfind-subtract",
            Self::SubtractAa => "rootfind-subtract-aa",
            Self::FloorScalar => "rootfind-floor-scalar",
            Self::FloorLanesScalar => "rootfind-floor-lanes-scalar",
            Self::FloorLanesPreferred => "rootfind-floor-lanes-preferred",
            Self::KmitmScalar => "rootfind-kmitm-scalar",
            Self::KmitmLanesPreferred => "rootfind-kmitm-lanes-preferred",
        }
    }
}

#[derive(Clone)]
struct Corpus {
    targets: Vec<PrimePoint>,
    planted_hit: Vec<bool>,
    hit_targets: Vec<PrimePoint>,
    reference_negative_targets: Vec<PrimePoint>,
    digest: String,
}

#[derive(Clone)]
struct ProbeRun {
    elapsed_ns: u64,
    outputs: Vec<Option<Vec<usize>>>,
    unit_adds: Vec<u64>,
    total_adds: u64,
    field_muls: u64,
    inversions: u64,
}

struct ArmRun {
    prepare_ns: u64,
    fresh_prepare_plus_single_reference_negative_ns: u64,
    setup_ops: GroupOps,
    first_pass_after_prepare: ProbeRun,
    warm_equal_mix: ProbeRun,
    warm_hit_only: ProbeRun,
    warm_reference_negative_only: ProbeRun,
    kernel: &'static str,
    ifma_used: bool,
    witness_digest: String,
    ledger_digest: String,
}

#[derive(Clone)]
struct Reference {
    outputs: Vec<Option<Vec<usize>>>,
    per_target_adds: Vec<u64>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |key: &str, default: &str| {
        args.iter()
            .position(|arg| arg == key)
            .and_then(|i| args.get(i + 1).cloned())
            .unwrap_or_else(|| default.to_owned())
    };
    let curves: Vec<String> = get("--curves", "p31,p61")
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    let size: usize = parse_arg(&get("--size", "64"), "--size");
    let hits: usize = parse_arg(&get("--hits", "32"), "--hits");
    let misses: usize = parse_arg(&get("--misses", "32"), "--misses");
    let repeats: usize = parse_arg(&get("--repeats", "5"), "--repeats");
    let seed: u64 = parse_arg(&get("--seed", &DEFAULT_SEED.to_string()), "--seed");
    assert!(size >= 4, "--size must be at least 4");
    assert!(
        hits > 0 && misses > 0,
        "both --hits and --misses must be positive"
    );
    assert_eq!(
        hits, misses,
        "the aggregate panel is an explicitly equal planted/reference-negative mix"
    );
    assert!(repeats > 0, "--repeats must be positive");

    let host = host_record();
    let mut curve_rows = Vec::new();
    for curve_id in curves {
        let inst = match curve_id.as_str() {
            "p31" | "p31-subexp" => fp_p31_subexp_mersenne_instance(),
            "p61" | "p61-i0025" => p61_instance(),
            other => panic!("unsupported --curves entry `{other}` (want p31,p61)"),
        };
        curve_rows.push(run_curve(
            &curve_id, &inst, size, hits, misses, repeats, seed,
        ));
    }

    let report = json!({
        "schema": "rootfind-production-panel/v1",
        "pcc": "unverified",
        "classification": "constant-factor engineering on public benchmark curves",
        "threading": "single-threaded; pin one CPU externally",
        "source": {
            "git_commit": build_provenance::git_commit(),
            "git_dirty": build_provenance::git_dirty(),
            "provenance_source": build_provenance::source(),
            "build_rustflags": option_env!("CRYPTO_BUILD_RUSTFLAGS").or(option_env!("RUSTFLAGS")),
            "runtime_rustflags": std::env::var("RUSTFLAGS").ok(),
            "build_rustc_version": option_env!("CRYPTO_BUILD_RUSTC_VERSION"),
            "build_rustc": option_env!("RUSTC"),
        },
        "seed_manifest": {
            "policy": "isogeny-labs/experiment-seeds/v1",
            "master_seed": seed,
            "corpus_stream": "splitmix64(master_seed xor curve.p xor 0x434f52505553)",
            "arm_order_stream": "splitmix64(master_seed xor curve.p xor repeat xor 0x4f52444552)",
            "paired_inputs": true,
        },
        "host": host,
        "parameters": {
            "m": M,
            "factor_base_abscissae_requested": size,
            "planted_hits": hits,
            "reference_negative_misses": misses,
            "repeats": repeats,
            "aggregate_corpus": "explicitly equal planted-hit/reference-negative mix",
            "timing_classification": "single-thread API throughput; preparation may populate caches",
        },
        "exhaustive_toy_completeness_control": "absent; this panel validates only its frozen corpus",
        "curves": curve_rows,
    });
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}

fn run_curve(
    curve_id: &str,
    inst: &PrimeInstance,
    size: usize,
    hits: usize,
    misses: usize,
    repeats: usize,
    seed: u64,
) -> Value {
    validate_instance(inst);
    let ctx = instance_ctx(inst);
    let mut base_ops = GroupOps::default();
    let fb = prime_factor_base_cleared(inst, size, &mut base_ops);
    assert_eq!(
        fb.abscissae, size,
        "factor-base construction was incomplete"
    );
    let corpus = build_corpus(&ctx, &fb, inst, hits, misses, seed);
    let reference = build_reference(&ctx, &fb, inst, &corpus.targets);
    for (&is_hit, output) in corpus.planted_hit.iter().zip(&reference.outputs) {
        assert_eq!(
            output.is_some(),
            is_hit,
            "corpus class disagrees with reference"
        );
    }
    assert!(
        !corpus.planted_hit[0] && reference.outputs[0].is_none(),
        "the first target must be a recorded reference-negative"
    );

    eprintln!(
        "curve={curve_id} p_bits={} A={} corpus={} repeats={} [pcc:unverified]",
        64 - inst.curve.p.leading_zeros(),
        fb.abscissae,
        corpus.targets.len(),
        repeats,
    );

    let mut samples: BTreeMap<Arm, Vec<ArmRun>> = BTreeMap::new();
    let mut orders = Vec::with_capacity(repeats);
    let mut order_bytes = Vec::new();
    for repeat in 0..repeats {
        let mut order = Arm::ALL;
        let mut order_rng = SplitMix64::new(seed ^ inst.curve.p ^ repeat as u64 ^ 0x4f52_4445_52);
        shuffle(&mut order, &mut order_rng);
        orders.push(order.to_vec());
        order_bytes.extend_from_slice(&(repeat as u64).to_le_bytes());
        for arm in order {
            order_bytes.extend_from_slice(arm.name().as_bytes());
            order_bytes.push(0);
            let run = run_arm(arm, &ctx, &fb, inst, &corpus);
            validate_arm(arm, &run, &reference, &ctx, &fb, &corpus.targets);
            samples.entry(arm).or_default().push(run);
        }
    }

    let subtract = &samples[&Arm::Subtract];
    let mitm = &samples[&Arm::Mitm];
    let mut arm_rows = Vec::new();
    for arm in Arm::ALL {
        let runs = &samples[&arm];
        let prep: Vec<f64> = runs.iter().map(|r| r.prepare_ns as f64).collect();
        let first_pass: Vec<f64> = runs
            .iter()
            .map(|r| r.first_pass_after_prepare.elapsed_ns as f64 / corpus.targets.len() as f64)
            .collect();
        let warm_equal_mix: Vec<f64> = runs
            .iter()
            .map(|r| r.warm_equal_mix.elapsed_ns as f64 / corpus.targets.len() as f64)
            .collect();
        let warm_hits: Vec<f64> = runs
            .iter()
            .map(|r| r.warm_hit_only.elapsed_ns as f64 / corpus.hit_targets.len() as f64)
            .collect();
        let warm_reference_negative: Vec<f64> = runs
            .iter()
            .map(|r| {
                r.warm_reference_negative_only.elapsed_ns as f64
                    / corpus.reference_negative_targets.len() as f64
            })
            .collect();
        let setup_amortized_first_pass: Vec<f64> = runs
            .iter()
            .map(|r| {
                (r.prepare_ns + r.first_pass_after_prepare.elapsed_ns) as f64
                    / corpus.targets.len() as f64
            })
            .collect();
        let fresh_single: Vec<f64> = runs
            .iter()
            .map(|r| r.fresh_prepare_plus_single_reference_negative_ns as f64)
            .collect();
        let first = &runs[0];
        assert!(runs
            .iter()
            .all(|r| r.witness_digest == first.witness_digest));
        assert!(runs.iter().all(|r| r.ledger_digest == first.ledger_digest));
        assert!(runs.iter().all(|r| r.kernel == first.kernel));
        assert!(runs.iter().all(|r| r.ifma_used == first.ifma_used));
        let raw_samples: Vec<Value> = runs
            .iter()
            .enumerate()
            .map(|(repeat, run)| {
                json!({
                    "repeat": repeat,
                    "arm_order_position": orders[repeat].iter().position(|&a| a == arm).unwrap(),
                    "prepare_ns": run.prepare_ns,
                    "fresh_prepare_plus_single_reference_negative_ns": run.fresh_prepare_plus_single_reference_negative_ns,
                    "first_pass_after_prepare_ns": run.first_pass_after_prepare.elapsed_ns,
                    "setup_amortized_first_pass_ns": run.prepare_ns + run.first_pass_after_prepare.elapsed_ns,
                    "warm_equal_mix_ns": run.warm_equal_mix.elapsed_ns,
                    "warm_hit_only_ns": run.warm_hit_only.elapsed_ns,
                    "warm_reference_negative_only_ns": run.warm_reference_negative_only.elapsed_ns,
                    "equal_mix_charged_adds": run.first_pass_after_prepare.total_adds,
                    "hit_only_charged_adds": run.warm_hit_only.total_adds,
                    "reference_negative_only_charged_adds": run.warm_reference_negative_only.total_adds,
                })
            })
            .collect();
        arm_rows.push(json!({
            "arm": arm.name(),
            "kernel": first.kernel,
            "ifma_used": first.ifma_used,
            "timing_classification": "single-thread API throughput",
            "aggregate_stratum": "explicitly equal planted-hit/reference-negative mix",
            "prepare_ns": summary(&prep),
            "first_pass_after_prepare_ns_per_target": summary(&first_pass),
            "warm_equal_mix_ns_per_target": summary(&warm_equal_mix),
            "warm_hit_only_ns_per_target": summary(&warm_hits),
            "warm_reference_negative_only_ns_per_target": summary(&warm_reference_negative),
            "setup_amortized_first_pass_ns_per_target": summary(&setup_amortized_first_pass),
            "fresh_prepare_plus_single_reference_negative_ns": summary(&fresh_single),
            "fresh_single_contract": "fresh prepare plus scalar single-target oracle contract call; lane API throughput is reported separately",
            "paired_speedup_vs_subtract": paired_speedups(subtract, runs, corpus.targets.len()),
            "paired_speedup_vs_rootfind_mitm": paired_speedups(mitm, runs, corpus.targets.len()),
            "setup_charged_adds": first.setup_ops.adds,
            "equal_mix_probe_charged_adds": first.first_pass_after_prepare.total_adds,
            "equal_mix_probe_field_muls": first.first_pass_after_prepare.field_muls,
            "equal_mix_probe_inversions": first.first_pass_after_prepare.inversions,
            "hit_only_probe_charged_adds": first.warm_hit_only.total_adds,
            "reference_negative_only_probe_charged_adds": first.warm_reference_negative_only.total_adds,
            "post_hit_full_sweep_work_included": arm == Arm::FloorLanesPreferred && first.ifma_used,
            "witness_digest_sha256": first.witness_digest,
            "ledger_digest_sha256": first.ledger_digest,
            "raw_samples": raw_samples,
        }));
    }

    let aa = &samples[&Arm::SubtractAa];
    let aa_noise = paired_speedups(subtract, aa, corpus.targets.len());
    let floor_scalar_hit_ledgers: Vec<u64> = samples[&Arm::FloorScalar]
        .iter()
        .map(|run| run.warm_hit_only.total_adds)
        .collect();
    let floor_preferred_hit_ledgers: Vec<u64> = samples[&Arm::FloorLanesPreferred]
        .iter()
        .map(|run| run.warm_hit_only.total_adds)
        .collect();
    let ifma_post_hit_ledger = json!({
        "ifma_used": samples[&Arm::FloorLanesPreferred][0].ifma_used,
        "scalar_early_exit_charged_adds_raw": floor_scalar_hit_ledgers,
        "preferred_lane_full_sweep_charged_adds_raw": floor_preferred_hit_ledgers,
        "note": "IFMA resolves Bloom-pass candidates after the lockstep sweep, so hit batches include honest post-hit work",
    });
    let arm_orders: Vec<Vec<&str>> = orders
        .iter()
        .map(|order| order.iter().map(|arm| arm.name()).collect())
        .collect();
    let ifma_miss_ledger_control = ifma_all_reference_negative_ledger_control(
        &ctx,
        &fb,
        inst,
        &corpus.reference_negative_targets,
    );

    json!({
        "curve_id": curve_id,
        "instance_name": inst.name.clone(),
        "curve_kind": if inst.curve.p == 2_305_843_009_213_693_951u64 {
            "frozen ordinary p61 public benchmark (trace -704859453)"
        } else {
            "frozen ordinary p31 public benchmark"
        },
        "benchmark_role": if inst.curve.p == 2_305_843_009_213_693_951u64 {
            "full-width scalar control; p exceeds the AVX-512 IFMA backend limit"
        } else {
            "AVX-512 IFMA control when host support is present"
        },
        "p": inst.curve.p,
        "a": inst.curve.a,
        "b": inst.curve.b,
        "group_order": inst.group_order,
        "subgroup_order": inst.r,
        "cofactor": inst.cofactor,
        "generator": [inst.generator.0, inst.generator.1],
        "curve_digest_sha256": curve_digest(inst),
        "factor_base_abscissae": fb.abscissae,
        "factor_base_points": fb.points.len(),
        "factor_base_digest_sha256": factor_base_digest(&fb),
        "factor_base_setup_ops": base_ops,
        "corpus_targets": corpus.targets.len(),
        "planted_hit_targets": corpus.hit_targets.len(),
        "reference_negative_targets": corpus.reference_negative_targets.len(),
        "reference_negative_targets_exclude_factor_base_abscissae": true,
        "first_target_class": "reference-negative",
        "corpus_digest_sha256": corpus.digest,
        "arm_order_digest_sha256": hex::encode(sha256(&order_bytes)),
        "raw_arm_orders": arm_orders,
        "subtract_aa_noise_primary_ns_over_duplicate_ns": aa_noise,
        "ifma_all_reference_negative_ledger_control": ifma_miss_ledger_control,
        "ifma_post_hit_full_sweep_ledger": ifma_post_hit_ledger,
        "arms": arm_rows,
    })
}

fn run_arm(
    arm: Arm,
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    inst: &PrimeInstance,
    corpus: &Corpus,
) -> ArmRun {
    let fresh_single =
        fresh_prepare_plus_single_reference_negative(arm, ctx, fb, inst, corpus.targets[0]);
    let targets = &corpus.targets;
    match arm {
        Arm::Mitm => {
            let start = Instant::now();
            let mut oracle = RootfindMitmOracle::<PrimePoint>::new(M);
            let mut setup_ops = GroupOps::default();
            oracle
                .prepare(ctx, fb, &params("rootfind-mitm:m=3"), &mut setup_ops)
                .unwrap();
            let prepare_ns = nanos(start);
            finish_scalar(
                prepare_ns,
                fresh_single,
                setup_ops,
                "group-law-scalar",
                &mut oracle,
                ctx,
                fb,
                corpus,
            )
        }
        Arm::Subtract | Arm::SubtractAa => {
            let start = Instant::now();
            let mut oracle = RootfindSubtractOracle::new(M, inst);
            let mut setup_ops = GroupOps::default();
            oracle
                .prepare(ctx, fb, &params("rootfind-subtract:m=3"), &mut setup_ops)
                .unwrap();
            let prepare_ns = nanos(start);
            let run = finish_scalar(
                prepare_ns,
                fresh_single,
                setup_ops,
                "subtract-scalar",
                &mut oracle,
                ctx,
                fb,
                corpus,
            );
            assert_eq!(oracle.mismatches, 0);
            run
        }
        Arm::FloorScalar => {
            let start = Instant::now();
            let mut oracle = RootfindFloorOracle::new(M, inst);
            let mut setup_ops = GroupOps::default();
            oracle
                .prepare(ctx, fb, &params("rootfind-floor:m=3"), &mut setup_ops)
                .unwrap();
            let prepare_ns = nanos(start);
            let run = finish_scalar(
                prepare_ns,
                fresh_single,
                setup_ops,
                "floor-scalar",
                &mut oracle,
                ctx,
                fb,
                corpus,
            );
            assert_eq!(oracle.mismatches, 0);
            run
        }
        Arm::FloorLanesScalar | Arm::FloorLanesPreferred => {
            let start = Instant::now();
            let mut oracle = RootfindFloorOracle::new(M, inst);
            let mut setup_ops = GroupOps::default();
            oracle
                .prepare(ctx, fb, &params("rootfind-floor:m=3"), &mut setup_ops)
                .unwrap();
            let ifma = oracle.ifma_prepared();
            let use_ifma = arm == Arm::FloorLanesPreferred && ifma;
            oracle.set_ifma_enabled(use_ifma);
            assert_eq!(oracle.ifma_enabled(), use_ifma);
            let prepare_ns = nanos(start);
            let first_pass_after_prepare = probe_floor_lanes(&mut oracle, ctx, fb, targets);
            let warm_equal_mix = probe_floor_lanes(&mut oracle, ctx, fb, targets);
            let warm_hit_only = probe_floor_lanes(&mut oracle, ctx, fb, &corpus.hit_targets);
            let warm_reference_negative_only =
                probe_floor_lanes(&mut oracle, ctx, fb, &corpus.reference_negative_targets);
            assert_eq!(oracle.mismatches, 0);
            let (kernel, ifma_used) = if use_ifma {
                ("avx512-ifma-floor-lanes", true)
            } else if arm == Arm::FloorLanesPreferred {
                ("floor-scalar-lanes-fallback", false)
            } else {
                ("floor-scalar-lanes", false)
            };
            finish_run(
                prepare_ns,
                fresh_single,
                setup_ops,
                kernel,
                ifma_used,
                first_pass_after_prepare,
                warm_equal_mix,
                warm_hit_only,
                warm_reference_negative_only,
            )
        }
        Arm::KmitmScalar => {
            let start = Instant::now();
            let mut oracle = RootfindKMitmOracle::new_prime(M, inst);
            let mut setup_ops = GroupOps::default();
            oracle
                .prepare(ctx, fb, &params("rootfind-kmitm:m=3"), &mut setup_ops)
                .unwrap();
            let prepare_ns = nanos(start);
            let run = finish_scalar(
                prepare_ns,
                fresh_single,
                setup_ops,
                "kmitm-scalar",
                &mut oracle,
                ctx,
                fb,
                corpus,
            );
            assert_eq!(oracle.mismatches, 0);
            run
        }
        Arm::KmitmLanesPreferred => {
            let start = Instant::now();
            let mut oracle = RootfindKMitmOracle::new_prime(M, inst);
            let mut setup_ops = GroupOps::default();
            oracle
                .prepare(ctx, fb, &params("rootfind-kmitm:m=3"), &mut setup_ops)
                .unwrap();
            let ifma = oracle.ifma_prepared();
            let avx2 = oracle.avx2_prepared();
            assert_eq!(
                oracle.right_k(),
                1,
                "m=3 KMITM lane backend requires right_k=1"
            );
            let (kernel, ifma_used) = if ifma {
                oracle.set_ifma_enabled(true);
                oracle.set_avx2_enabled(false);
                ("avx512-ifma-kmitm-lanes", true)
            } else if avx2 {
                oracle.set_ifma_enabled(false);
                oracle.set_avx2_enabled(true);
                ("avx2-kmitm-lanes", false)
            } else {
                oracle.set_ifma_enabled(false);
                oracle.set_avx2_enabled(false);
                ("kmitm-scalar-lanes-fallback", false)
            };
            assert_eq!(oracle.ifma_enabled(), ifma);
            assert_eq!(oracle.avx2_enabled(), !ifma && avx2);
            let prepare_ns = nanos(start);
            let first_pass_after_prepare = probe_kmitm_lanes(&mut oracle, ctx, fb, targets);
            let warm_equal_mix = probe_kmitm_lanes(&mut oracle, ctx, fb, targets);
            let warm_hit_only = probe_kmitm_lanes(&mut oracle, ctx, fb, &corpus.hit_targets);
            let warm_reference_negative_only =
                probe_kmitm_lanes(&mut oracle, ctx, fb, &corpus.reference_negative_targets);
            assert_eq!(oracle.mismatches, 0);
            finish_run(
                prepare_ns,
                fresh_single,
                setup_ops,
                kernel,
                ifma_used,
                first_pass_after_prepare,
                warm_equal_mix,
                warm_hit_only,
                warm_reference_negative_only,
            )
        }
    }
}

fn finish_scalar<O: DecompositionOracle<PrimeCurve>>(
    prepare_ns: u64,
    fresh_prepare_plus_single_reference_negative_ns: u64,
    setup_ops: GroupOps,
    kernel: &'static str,
    oracle: &mut O,
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    corpus: &Corpus,
) -> ArmRun {
    let first_pass_after_prepare = probe_scalar(oracle, ctx, fb, &corpus.targets);
    let warm_equal_mix = probe_scalar(oracle, ctx, fb, &corpus.targets);
    let warm_hit_only = probe_scalar(oracle, ctx, fb, &corpus.hit_targets);
    let warm_reference_negative_only =
        probe_scalar(oracle, ctx, fb, &corpus.reference_negative_targets);
    finish_run(
        prepare_ns,
        fresh_prepare_plus_single_reference_negative_ns,
        setup_ops,
        kernel,
        false,
        first_pass_after_prepare,
        warm_equal_mix,
        warm_hit_only,
        warm_reference_negative_only,
    )
}

fn finish_run(
    prepare_ns: u64,
    fresh_prepare_plus_single_reference_negative_ns: u64,
    setup_ops: GroupOps,
    kernel: &'static str,
    ifma_used: bool,
    first_pass_after_prepare: ProbeRun,
    warm_equal_mix: ProbeRun,
    warm_hit_only: ProbeRun,
    warm_reference_negative_only: ProbeRun,
) -> ArmRun {
    assert_eq!(
        first_pass_after_prepare.outputs, warm_equal_mix.outputs,
        "first/second equal-mix verdict or witness drift"
    );
    assert_eq!(
        first_pass_after_prepare.unit_adds, warm_equal_mix.unit_adds,
        "first/second equal-mix ledger drift"
    );
    let witness_digest = witness_digest(&first_pass_after_prepare.outputs);
    let ledger_digest = u64_digest(&first_pass_after_prepare.unit_adds);
    ArmRun {
        prepare_ns,
        fresh_prepare_plus_single_reference_negative_ns,
        setup_ops,
        first_pass_after_prepare,
        warm_equal_mix,
        warm_hit_only,
        warm_reference_negative_only,
        kernel,
        ifma_used,
        witness_digest,
        ledger_digest,
    }
}

fn fresh_prepare_plus_single_reference_negative(
    arm: Arm,
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    inst: &PrimeInstance,
    target: PrimePoint,
) -> u64 {
    match arm {
        Arm::Mitm => fresh_single(
            RootfindMitmOracle::<PrimePoint>::new(M),
            "rootfind-mitm:m=3",
            ctx,
            fb,
            target,
        ),
        Arm::Subtract | Arm::SubtractAa => fresh_single(
            RootfindSubtractOracle::new(M, inst),
            "rootfind-subtract:m=3",
            ctx,
            fb,
            target,
        ),
        Arm::FloorScalar | Arm::FloorLanesScalar | Arm::FloorLanesPreferred => fresh_single(
            RootfindFloorOracle::new(M, inst),
            "rootfind-floor:m=3",
            ctx,
            fb,
            target,
        ),
        Arm::KmitmScalar | Arm::KmitmLanesPreferred => fresh_single(
            RootfindKMitmOracle::new_prime(M, inst),
            "rootfind-kmitm:m=3",
            ctx,
            fb,
            target,
        ),
    }
}

fn fresh_single<O: DecompositionOracle<PrimeCurve>>(
    mut oracle: O,
    spec: &str,
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    target: PrimePoint,
) -> u64 {
    let start = Instant::now();
    oracle
        .prepare(ctx, fb, &params(spec), &mut GroupOps::default())
        .unwrap();
    let output = oracle.decompose(
        ctx,
        fb,
        &mut GroupOps::default(),
        &mut OracleCounters::default(),
        target,
    );
    let elapsed = nanos(start);
    assert!(
        output.is_none(),
        "fresh single-query target was not reference-negative"
    );
    elapsed
}

fn probe_scalar<O: DecompositionOracle<PrimeCurve>>(
    oracle: &mut O,
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    targets: &[PrimePoint],
) -> ProbeRun {
    let start = Instant::now();
    let mut outputs = Vec::with_capacity(targets.len());
    let mut unit_adds = Vec::with_capacity(targets.len());
    let mut total = GroupOps::default();
    let mut inversions = 0u64;
    for &target in targets {
        let mut ops = GroupOps::default();
        let mut counters = OracleCounters::default();
        outputs.push(oracle.decompose(ctx, fb, &mut ops, &mut counters, target));
        unit_adds.push(ops.adds);
        total.merge(ops);
        inversions += counters.inversions;
    }
    let elapsed_ns = nanos(start);
    black_box(&outputs);
    ProbeRun {
        elapsed_ns,
        outputs,
        unit_adds,
        total_adds: total.adds,
        field_muls: total.field_ops.muls + total.field_ops.sqrs,
        inversions,
    }
}

fn probe_floor_lanes(
    oracle: &mut RootfindFloorOracle<'_>,
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    targets: &[PrimePoint],
) -> ProbeRun {
    let start = Instant::now();
    let mut outputs = Vec::with_capacity(targets.len());
    let mut unit_adds = Vec::with_capacity(targets.len().div_ceil(LANE_WIDTH));
    let mut total = GroupOps::default();
    let mut inversions = 0u64;
    let mut out = vec![None; LANE_WIDTH];
    for chunk in targets.chunks(LANE_WIDTH) {
        let mut ops = GroupOps::default();
        let mut counters = OracleCounters::default();
        oracle.decompose_lanes(
            ctx,
            fb,
            &mut ops,
            &mut counters,
            chunk,
            &mut out[..chunk.len()],
        );
        unit_adds.push(ops.adds);
        total.merge(ops);
        inversions += counters.inversions;
        outputs.extend(out[..chunk.len()].iter_mut().map(Option::take));
    }
    let elapsed_ns = nanos(start);
    black_box(&outputs);
    ProbeRun {
        elapsed_ns,
        outputs,
        unit_adds,
        total_adds: total.adds,
        field_muls: total.field_ops.muls + total.field_ops.sqrs,
        inversions,
    }
}

fn probe_kmitm_lanes(
    oracle: &mut RootfindKMitmOracle<'_>,
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    targets: &[PrimePoint],
) -> ProbeRun {
    let start = Instant::now();
    let mut outputs = Vec::with_capacity(targets.len());
    let mut unit_adds = Vec::with_capacity(targets.len().div_ceil(LANE_WIDTH));
    let mut total = GroupOps::default();
    let mut inversions = 0u64;
    let mut out = vec![None; LANE_WIDTH];
    for chunk in targets.chunks(LANE_WIDTH) {
        let mut ops = GroupOps::default();
        let mut counters = OracleCounters::default();
        oracle.decompose_lanes(
            ctx,
            fb,
            &mut ops,
            &mut counters,
            chunk,
            &mut out[..chunk.len()],
        );
        unit_adds.push(ops.adds);
        total.merge(ops);
        inversions += counters.inversions;
        outputs.extend(out[..chunk.len()].iter_mut().map(Option::take));
    }
    let elapsed_ns = nanos(start);
    black_box(&outputs);
    ProbeRun {
        elapsed_ns,
        outputs,
        unit_adds,
        total_adds: total.adds,
        field_muls: total.field_ops.muls + total.field_ops.sqrs,
        inversions,
    }
}

fn validate_arm(
    arm: Arm,
    run: &ArmRun,
    reference: &Reference,
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    targets: &[PrimePoint],
) {
    for probe in [&run.first_pass_after_prepare, &run.warm_equal_mix] {
        assert_eq!(probe.outputs.len(), targets.len());
        for ((output, expected), &target) in
            probe.outputs.iter().zip(&reference.outputs).zip(targets)
        {
            assert_eq!(
                output.is_some(),
                expected.is_some(),
                "{} verdict drift",
                arm.name()
            );
            if let Some(witness) = output {
                validate_witness(ctx, fb, target, witness);
            }
        }
        match arm {
            Arm::Subtract | Arm::SubtractAa | Arm::FloorScalar => {
                assert_eq!(
                    probe.outputs,
                    reference.outputs,
                    "{} witness drift",
                    arm.name()
                );
                assert_eq!(
                    probe.unit_adds,
                    reference.per_target_adds,
                    "{} per-target charged-add drift",
                    arm.name()
                );
            }
            Arm::FloorLanesScalar => {
                assert_eq!(
                    probe.outputs,
                    reference.outputs,
                    "{} witness drift",
                    arm.name()
                );
                assert_eq!(
                    probe.unit_adds,
                    chunk_sums(&reference.per_target_adds),
                    "{} batch charged-add drift",
                    arm.name()
                );
            }
            Arm::FloorLanesPreferred => {
                assert_eq!(
                    probe.outputs,
                    reference.outputs,
                    "{} witness drift",
                    arm.name()
                );
                // Without IFMA this is the scalar-lane implementation and
                // must carry the same chunk ledger.  The IFMA implementation
                // resolves Bloom-pass candidates after its full lockstep
                // sweep, so planted-hit lanes honestly carry post-hit work;
                // its ledger is reported but cannot equal early-exit scalar.
                if !run.ifma_used {
                    assert_eq!(
                        probe.unit_adds,
                        chunk_sums(&reference.per_target_adds),
                        "{} fallback batch charged-add drift",
                        arm.name()
                    );
                }
            }
            _ => {}
        }
    }
    let hit_targets: Vec<_> = targets
        .iter()
        .zip(&reference.outputs)
        .filter_map(|(&target, output)| output.as_ref().map(|_| target))
        .collect();
    assert_eq!(run.warm_hit_only.outputs.len(), hit_targets.len());
    for (output, &target) in run.warm_hit_only.outputs.iter().zip(&hit_targets) {
        let witness = output
            .as_ref()
            .unwrap_or_else(|| panic!("{} rejected a planted-hit stratum target", arm.name()));
        validate_witness(ctx, fb, target, witness);
    }
    assert!(
        run.warm_reference_negative_only
            .outputs
            .iter()
            .all(Option::is_none),
        "{} accepted a reference-negative stratum target",
        arm.name()
    );
}

fn build_corpus(
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    inst: &PrimeInstance,
    hits: usize,
    misses: usize,
    seed: u64,
) -> Corpus {
    let mut rng = SplitMix64::new(seed ^ inst.curve.p ^ 0x43_4f_52_50_55_53);
    let mut seen = HashSet::with_capacity(hits + misses);
    let mut hit_rows = Vec::with_capacity(hits);
    while hit_rows.len() < hits {
        let mut idx = [0usize; 3];
        for pos in 0..idx.len() {
            loop {
                let candidate = (rng.next() as usize) % fb.points.len();
                if !idx[..pos]
                    .iter()
                    .any(|&prior| fb.col_of[prior] == fb.col_of[candidate])
                {
                    idx[pos] = candidate;
                    break;
                }
            }
        }
        let mut ops = GroupOps::default();
        let mut target = ctx.group.identity();
        for i in idx {
            target = ctx.group.add(&mut ops, target, fb.points[i]);
        }
        if !target.infinity && seen.insert(point_key(target)) {
            hit_rows.push((target, true));
        }
    }

    let mut classifier = RootfindSubtractOracle::new(M, inst);
    classifier
        .prepare(
            ctx,
            fb,
            &params("rootfind-subtract:m=3"),
            &mut GroupOps::default(),
        )
        .unwrap();
    let mut attempts = 0usize;
    let mut reference_negative_rows = Vec::with_capacity(misses);
    while reference_negative_rows.len() < misses {
        attempts += 1;
        assert!(
            attempts <= misses.saturating_mul(100_000).saturating_add(10_000),
            "could not construct enough reference-negative targets"
        );
        let scalar = 1 + rng.next() % (inst.r - 1);
        let mut mul_ops = GroupOps::default();
        let target = ctx.group.mul(&mut mul_ops, inst.generator_point(), scalar);
        let mut ops = GroupOps::default();
        let mut counters = OracleCounters::default();
        if classifier
            .decompose(ctx, fb, &mut ops, &mut counters, target)
            .is_none()
            && !fb.x_index.contains_key(&target.x)
            && seen.insert(point_key(target))
        {
            reference_negative_rows.push((target, false));
        }
    }
    assert_eq!(classifier.mismatches, 0);
    shuffle(&mut hit_rows, &mut rng);
    shuffle(&mut reference_negative_rows, &mut rng);
    let first_reference_negative = reference_negative_rows.remove(0);
    let mut remainder = Vec::with_capacity(hits + misses - 1);
    remainder.extend(hit_rows);
    remainder.extend(reference_negative_rows);
    shuffle(&mut remainder, &mut rng);
    let mut rows = Vec::with_capacity(hits + misses);
    rows.push(first_reference_negative);
    rows.extend(remainder);
    assert_eq!(seen.len(), rows.len(), "corpus targets are not unique");
    let targets: Vec<_> = rows.iter().map(|row| row.0).collect();
    let planted_hit: Vec<_> = rows.iter().map(|row| row.1).collect();
    let hit_targets = rows
        .iter()
        .filter_map(|row| row.1.then_some(row.0))
        .collect();
    let reference_negative_targets = rows
        .iter()
        .filter_map(|row| (!row.1).then_some(row.0))
        .collect();
    let digest = corpus_digest(&targets, &planted_hit);
    Corpus {
        targets,
        planted_hit,
        hit_targets,
        reference_negative_targets,
        digest,
    }
}

fn build_reference(
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    inst: &PrimeInstance,
    targets: &[PrimePoint],
) -> Reference {
    let mut oracle = RootfindSubtractOracle::new(M, inst);
    oracle
        .prepare(
            ctx,
            fb,
            &params("rootfind-subtract:m=3"),
            &mut GroupOps::default(),
        )
        .unwrap();
    let run = probe_scalar(&mut oracle, ctx, fb, targets);
    assert_eq!(oracle.mismatches, 0);
    for (&target, output) in targets.iter().zip(&run.outputs) {
        if let Some(witness) = output {
            validate_witness(ctx, fb, target, witness);
        }
    }
    Reference {
        outputs: run.outputs,
        per_target_adds: run.unit_adds,
    }
}

fn validate_witness(
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    target: PrimePoint,
    witness: &[usize],
) {
    assert_eq!(witness.len(), M as usize, "wrong witness arity");
    let mut ops = GroupOps::default();
    let mut sum = ctx.group.identity();
    for &index in witness {
        assert!(index < fb.points.len(), "witness index out of range");
        sum = ctx.group.add(&mut ops, sum, fb.points[index]);
    }
    assert_eq!(sum, target, "witness does not re-add to its target");
}

fn validate_instance(inst: &PrimeInstance) {
    assert_eq!(
        inst.group_order,
        inst.r.checked_mul(inst.cofactor).unwrap(),
        "group order is not r*h"
    );
    let generator = inst.generator_point();
    assert!(inst.curve.is_on_curve(generator));
    let mut ops = GroupOps::default();
    assert_eq!(
        inst.curve.mul(&mut ops, generator, inst.r),
        PrimePoint::INFINITY,
        "generator is not in the declared subgroup"
    );
}

fn instance_ctx(inst: &PrimeInstance) -> InstanceCtx<'_, PrimeCurve> {
    InstanceCtx {
        group: &inst.curve,
        generator: inst.generator_point(),
        target: inst.generator_point(),
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: None,
    }
}

fn p61_instance() -> PrimeInstance {
    PrimeInstance {
        name: "p61-i0025".to_owned(),
        curve: PrimeCurve {
            p: 2_305_843_009_213_693_951,
            a: 1_055,
            b: 16_361,
        },
        group_order: 2_305_843_009_918_553_405,
        r: 6_883_113_462_443_443,
        cofactor: 335,
        generator: (843_688_558_021_621_073, 105_039_246_913_253_664),
        representation: None,
    }
}

fn params(spec: &str) -> Params {
    Params::parse_spec(spec)
        .unwrap_or_else(|error| panic!("invalid built-in oracle spec `{spec}`: {error}"))
        .1
}

fn parse_arg<T: std::str::FromStr>(value: &str, name: &str) -> T {
    value
        .parse()
        .unwrap_or_else(|_| panic!("invalid value `{value}` for {name}"))
}

fn nanos(start: Instant) -> u64 {
    start.elapsed().as_nanos().try_into().unwrap_or(u64::MAX)
}

fn chunk_sums(values: &[u64]) -> Vec<u64> {
    values
        .chunks(LANE_WIDTH)
        .map(|chunk| chunk.iter().sum())
        .collect()
}

fn summary(values: &[f64]) -> Value {
    let center = median(values.to_vec());
    json!({
        "median": center,
        "mad": mad(values, center),
        "raw": values,
    })
}

fn paired_speedups(baseline: &[ArmRun], candidate: &[ArmRun], targets: usize) -> Value {
    assert_eq!(baseline.len(), candidate.len());
    let first_pass = paired_ratio_values(baseline, candidate, |run| {
        run.first_pass_after_prepare.elapsed_ns as f64 / targets as f64
    });
    let warm_equal_mix = paired_ratio_values(baseline, candidate, |run| {
        run.warm_equal_mix.elapsed_ns as f64 / targets as f64
    });
    let setup_amortized = paired_ratio_values(baseline, candidate, |run| {
        (run.prepare_ns + run.first_pass_after_prepare.elapsed_ns) as f64 / targets as f64
    });
    let fresh_single = paired_ratio_values(baseline, candidate, |run| {
        run.fresh_prepare_plus_single_reference_negative_ns as f64
    });
    let hit_only = paired_ratio_values(baseline, candidate, |run| {
        run.warm_hit_only.elapsed_ns as f64
    });
    let reference_negative_only = paired_ratio_values(baseline, candidate, |run| {
        run.warm_reference_negative_only.elapsed_ns as f64
    });
    json!({
        "definition": "within-repeat baseline_ns / candidate_ns; values above 1 favor candidate",
        "first_pass_after_prepare": summary(&first_pass),
        "warm_equal_mix": summary(&warm_equal_mix),
        "setup_amortized_first_pass": summary(&setup_amortized),
        "fresh_prepare_plus_single_reference_negative": summary(&fresh_single),
        "warm_hit_only": summary(&hit_only),
        "warm_reference_negative_only": summary(&reference_negative_only),
    })
}

fn paired_ratio_values<F>(baseline: &[ArmRun], candidate: &[ArmRun], metric: F) -> Vec<f64>
where
    F: Fn(&ArmRun) -> f64,
{
    baseline
        .iter()
        .zip(candidate)
        .map(|(base, arm)| metric(base) / metric(arm))
        .collect()
}

fn ifma_all_reference_negative_ledger_control(
    ctx: &InstanceCtx<'_, PrimeCurve>,
    fb: &FactorBase<PrimePoint>,
    inst: &PrimeInstance,
    targets: &[PrimePoint],
) -> Value {
    let mut scalar = RootfindFloorOracle::new(M, inst);
    scalar
        .prepare(
            ctx,
            fb,
            &params("rootfind-floor:m=3"),
            &mut GroupOps::default(),
        )
        .unwrap();
    scalar.set_ifma_enabled(false);
    let scalar_run = probe_scalar(&mut scalar, ctx, fb, targets);
    assert!(scalar_run.outputs.iter().all(Option::is_none));

    let mut lanes = RootfindFloorOracle::new(M, inst);
    lanes
        .prepare(
            ctx,
            fb,
            &params("rootfind-floor:m=3"),
            &mut GroupOps::default(),
        )
        .unwrap();
    let prepared = lanes.ifma_prepared();
    lanes.set_ifma_enabled(prepared);
    assert_eq!(lanes.ifma_enabled(), prepared);
    let lane_run = probe_floor_lanes(&mut lanes, ctx, fb, targets);
    assert!(lane_run.outputs.iter().all(Option::is_none));
    if prepared {
        assert_eq!(
            lane_run.unit_adds,
            chunk_sums(&scalar_run.unit_adds),
            "IFMA all-reference-negative charged-add ledger differs from scalar"
        );
    }
    json!({
        "status": if prepared { "asserted" } else { "not-applicable-no-ifma-backend" },
        "ifma_used": prepared,
        "targets": targets.len(),
        "all_targets_reference_negative": true,
        "scalar_charged_adds": scalar_run.total_adds,
        "preferred_lane_charged_adds": lane_run.total_adds,
        "scalar_chunk_ledger_digest_sha256": u64_digest(&chunk_sums(&scalar_run.unit_adds)),
        "preferred_lane_ledger_digest_sha256": u64_digest(&lane_run.unit_adds),
    })
}

fn median(mut values: Vec<f64>) -> f64 {
    assert!(!values.is_empty());
    values.sort_by(f64::total_cmp);
    let mid = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[mid - 1] + values[mid]) / 2.0
    } else {
        values[mid]
    }
}

fn mad(values: &[f64], center: f64) -> f64 {
    median(values.iter().map(|value| (value - center).abs()).collect())
}

fn witness_digest(outputs: &[Option<Vec<usize>>]) -> String {
    let mut bytes = Vec::new();
    for output in outputs {
        match output {
            None => bytes.push(0),
            Some(witness) => {
                bytes.push(1);
                bytes.extend_from_slice(&(witness.len() as u64).to_le_bytes());
                for &index in witness {
                    bytes.extend_from_slice(&(index as u64).to_le_bytes());
                }
            }
        }
    }
    hex::encode(sha256(&bytes))
}

fn u64_digest(values: &[u64]) -> String {
    let mut bytes = Vec::with_capacity(values.len() * 8);
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    hex::encode(sha256(&bytes))
}

fn corpus_digest(targets: &[PrimePoint], planted_hit: &[bool]) -> String {
    let mut bytes = Vec::with_capacity(targets.len() * 18);
    for (&target, &hit) in targets.iter().zip(planted_hit) {
        bytes.extend_from_slice(&target.x.to_le_bytes());
        bytes.extend_from_slice(&target.y.to_le_bytes());
        bytes.push(u8::from(target.infinity));
        bytes.push(u8::from(hit));
    }
    hex::encode(sha256(&bytes))
}

fn curve_digest(inst: &PrimeInstance) -> String {
    let mut bytes = Vec::new();
    for value in [
        inst.curve.p,
        inst.curve.a,
        inst.curve.b,
        inst.group_order,
        inst.r,
        inst.cofactor,
        inst.generator.0,
        inst.generator.1,
    ] {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    hex::encode(sha256(&bytes))
}

fn factor_base_digest(fb: &FactorBase<PrimePoint>) -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&(fb.points.len() as u64).to_le_bytes());
    bytes.extend_from_slice(&(fb.columns as u64).to_le_bytes());
    for (index, point) in fb.points.iter().enumerate() {
        bytes.extend_from_slice(&point.x.to_le_bytes());
        bytes.extend_from_slice(&point.y.to_le_bytes());
        bytes.push(u8::from(point.infinity));
        bytes.extend_from_slice(&(fb.neg_index[index] as u64).to_le_bytes());
        bytes.extend_from_slice(&(fb.col_of[index] as u64).to_le_bytes());
        bytes.extend_from_slice(&fb.coef_of[index].to_le_bytes());
    }
    for x in &fb.abscissa_list {
        bytes.extend_from_slice(&x.to_le_bytes());
    }
    hex::encode(sha256(&bytes))
}

fn point_key(point: PrimePoint) -> (u64, u64, bool) {
    (point.x, point.y, point.infinity)
}

fn host_record() -> Value {
    let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let model = cpuinfo
        .lines()
        .find_map(|line| line.strip_prefix("model name\t: "))
        .unwrap_or("unknown");
    let flags = cpuinfo
        .lines()
        .find_map(|line| line.strip_prefix("flags\t\t: "))
        .unwrap_or("unknown");
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let affinity_list = status
        .lines()
        .find_map(|line| line.strip_prefix("Cpus_allowed_list:\t"))
        .unwrap_or("unknown");
    let affinity_mask = status
        .lines()
        .find_map(|line| line.strip_prefix("Cpus_allowed:\t"))
        .unwrap_or("unknown");
    let logical_cpus = std::thread::available_parallelism().map_or(1, |count| count.get());
    let identity = format!(
        "{}|{}|{}|{}|{}",
        std::env::consts::ARCH,
        model,
        flags,
        logical_cpus,
        affinity_list
    );
    json!({
        "arch": std::env::consts::ARCH,
        "os": std::env::consts::OS,
        "cpu_model": model,
        "cpu_flags": flags,
        "cpu_flags_sha256": hex::encode(sha256(flags.as_bytes())),
        "logical_cpus_visible": logical_cpus,
        "cpus_allowed_list": affinity_list,
        "cpus_allowed_mask": affinity_mask,
        "identity_sha256": hex::encode(sha256(identity.as_bytes())),
        "virtualized_or_isolated": "not inferred; caller must record runner isolation",
    })
}

fn shuffle<T>(values: &mut [T], rng: &mut SplitMix64) {
    for i in (1..values.len()).rev() {
        values.swap(i, (rng.next() as usize) % (i + 1));
    }
}

struct SplitMix64(u64);

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}
