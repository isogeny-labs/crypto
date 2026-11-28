use crypto_lib::cryptanalysis::ic_boundary::{
    collect_and_solve_relations_touching_check_batched, BatchRelationsOracle, Calibration,
    CompletionRule, CountedGroup, FactorBase, GroupOps, IncrementalGauss, OracleCounters,
    RecoveryCheck, Relation, RestartPool, TargetSource,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::PrimeAbscissaBase;
use crypto_lib::cryptanalysis::ic_framework::rootfind_floor::RootfindFloorOracle;
use crypto_lib::cryptanalysis::ic_framework::rootfind_kmitm::RootfindKMitmOracle;
use crypto_lib::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params, Targets,
};
use crypto_lib::cryptanalysis::ic_framework::{
    run_pipeline, run_pipeline_with_progress, PipelineSpec, RunReport,
};

fn prime_ctx(
    inst: &crypto_lib::cryptanalysis::ic_boundary::PrimeInstance,
) -> (
    InstanceCtx<'_, crypto_lib::cryptanalysis::ic_boundary::PrimeCurve>,
    u64,
) {
    let generator = inst.generator_point();
    let planted = 30_011 % inst.r;
    let mut ops = GroupOps::default();
    let target = inst.curve.mul(&mut ops, generator, planted);
    (
        InstanceCtx {
            group: &inst.curve,
            generator,
            target,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: None,
        },
        planted,
    )
}

fn spec(oracle: &str, targets: Targets) -> PipelineSpec {
    let (name, oracle_params) = Params::parse_spec(oracle).unwrap();
    let mut spec = PipelineSpec {
        factor_base: "prime-abscissa".into(),
        oracle: name,
        oracle_params,
        targets,
        linalg: "incremental-gauss".into(),
        max_trials: 200_000,
        seed: 7,
        ..Default::default()
    };
    spec.factor_base_params.set("size", "24");
    spec
}

struct PanicBase;

impl FactorBaseBuilder<crypto_lib::cryptanalysis::ic_boundary::PrimeCurve> for PanicBase {
    fn name(&self) -> &str {
        "panic-base"
    }

    fn describe(&self, _params: &Params) -> String {
        "must not be built by an invalid batch configuration".into()
    }

    fn build(
        &self,
        _ctx: &InstanceCtx<'_, crypto_lib::cryptanalysis::ic_boundary::PrimeCurve>,
        _params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<FactorBase<crypto_lib::cryptanalysis::ic_boundary::PrimePoint>, String> {
        panic!("batch admission must run before factor-base construction")
    }
}

#[test]
fn versioned_batch_identities_reject_invalid_scheduling_before_setup() {
    let inst = crypto_lib::cryptanalysis::ic_boundary::roster_prime_instance(16).unwrap();
    let (ctx, planted) = prime_ctx(&inst);

    let mut floor = RootfindFloorOracle::new_batch_v1(3, &inst);
    let err = run_pipeline(
        &ctx,
        &spec("rootfind-floor-batch-v1:m=3", Targets::Walk),
        &PanicBase,
        &mut floor,
        planted,
        &Calibration::default(),
        None,
    )
    .unwrap_err();
    assert!(err.contains("requires a walk-prime-serial/batch target source"));

    let mut kmitm = RootfindKMitmOracle::new_prime_batch_v1(3, &inst);
    let err = run_pipeline(
        &ctx,
        &spec("rootfind-kmitm-batch-v1:m=3", Targets::Random),
        &PanicBase,
        &mut kmitm,
        planted,
        &Calibration::default(),
        None,
    )
    .unwrap_err();
    assert!(err.contains("requires a walk-prime-serial/batch target source"));

    let mut floor = RootfindFloorOracle::new_batch_v1(3, &inst);
    let err = run_pipeline_with_progress(
        &ctx,
        &spec("rootfind-floor-batch-v1:m=3", Targets::WalkPrimeBatch8),
        &PanicBase,
        &mut floor,
        planted,
        &Calibration::default(),
        None,
        &mut |_| panic!("invalid progress configuration must emit no event"),
    )
    .unwrap_err();
    assert!(err.contains("does not support the scalar per-query progress observer"));
}

fn assert_batch_parity(scalar: &RunReport, batch: &RunReport) {
    assert!(scalar.verified && batch.verified);
    assert_eq!(scalar.recovered, batch.recovered);
    assert_eq!(
        scalar.decomposition.relations_found,
        batch.decomposition.relations_found
    );
    assert_eq!(scalar.linear_algebra.rank, batch.linear_algebra.rank);

    let scalar_cost = &scalar.decomposition.cost;
    let batch_cost = &batch.decomposition.cost;
    assert!(!scalar_cost.native.contains_key("oracle_batch_calls"));
    assert_eq!(batch_cost.get("oracle_batch_preferred_width"), 8);
    assert!((1..=8).contains(&batch_cost.get("oracle_batch_max_targets")));
    assert!(batch_cost.get("oracle_batch_calls") > 0);
    assert!(batch_cost.get("oracle_batch_targets") >= batch_cost.get("oracle_batch_calls"));
    assert!(batch_cost.get("oracle_batch_targets") <= 8 * batch_cost.get("oracle_batch_calls"));

    let unused = batch_cost.get("oracle_batch_unused_results")
        + batch_cost.get("oracle_batch_unused_guarded_trials");
    assert_eq!(
        batch.decomposition.targets_tried,
        scalar.decomposition.targets_tried + unused,
        "only honestly charged terminal-batch work may extend the scalar run"
    );
    assert!(unused < 8);
}

#[test]
fn real_prime_floor_and_kmitm_batches_preserve_full_pipeline_answers() {
    let inst = crypto_lib::cryptanalysis::ic_boundary::roster_prime_instance(16).unwrap();
    let (ctx, planted) = prime_ctx(&inst);
    let base = PrimeAbscissaBase { instance: &inst };

    let lane_spec = spec("rootfind-floor:m=3", Targets::WalkPrimeBatch8);
    let mut scalar = RootfindFloorOracle::new(3, &inst);
    let scalar = run_pipeline(
        &ctx,
        &lane_spec,
        &base,
        &mut scalar,
        planted,
        &Calibration::default(),
        None,
    )
    .unwrap();
    let mut batch = RootfindFloorOracle::new_batch_v1(3, &inst);
    let batch = run_pipeline(
        &ctx,
        &spec("rootfind-floor-batch-v1:m=3", Targets::WalkPrimeBatch8),
        &base,
        &mut batch,
        planted,
        &Calibration::default(),
        None,
    )
    .unwrap();
    assert_batch_parity(&scalar, &batch);

    let lane_spec = spec("rootfind-kmitm:m=3", Targets::WalkPrimeBatch8);
    let mut scalar = RootfindKMitmOracle::new_prime(3, &inst);
    let scalar = run_pipeline(
        &ctx,
        &lane_spec,
        &base,
        &mut scalar,
        planted,
        &Calibration::default(),
        None,
    )
    .unwrap();
    let mut batch = RootfindKMitmOracle::new_prime_batch_v1(3, &inst);
    let batch = run_pipeline(
        &ctx,
        &spec("rootfind-kmitm-batch-v1:m=3", Targets::WalkPrimeBatch8),
        &base,
        &mut batch,
        planted,
        &Calibration::default(),
        None,
    )
    .unwrap();
    assert_batch_parity(&scalar, &batch);
}

#[test]
fn p31_kmitm_lanes_orient_the_folded_left_witness() {
    use crypto_lib::cryptanalysis::ic_boundary::{
        fp_p31_subexp_mersenne_instance, prime_factor_base_cleared, PairTable,
    };

    let inst = fp_p31_subexp_mersenne_instance();
    let (ctx, _) = prime_ctx(&inst);
    let mut base_ops = GroupOps::default();
    let fb = prime_factor_base_cleared(&inst, 24, &mut base_ops);
    let table = PairTable::build_negation_folded(&inst.curve, &fb);
    let mut keys = Vec::new();
    table.for_each_folded_key(|key| keys.push(key));
    keys.sort_unstable();

    // Make j=0's R-Q candidate hit a stored pair in the opposite
    // orientation. Excluding R+Q from the table makes this the first exact
    // map hit, so the old whole-witness sign recovery fails deterministically.
    let right = 0usize;
    let (target, expected) = keys
        .into_iter()
        .find_map(|key| {
            let (i, j) = table.lookup_folded(key)?;
            let ni = fb.neg_index[i];
            let nj = fb.neg_index[j];
            if [i, j, ni, nj].contains(&right) {
                return None;
            }
            let mut ops = GroupOps::default();
            let left = inst.curve.add(&mut ops, fb.points[i], fb.points[j]);
            let target = inst
                .curve
                .add(&mut ops, inst.curve.neg(left), fb.points[right]);
            if target.infinity || fb.x_index.contains_key(&target.x) {
                return None;
            }
            let plus = inst.curve.add(&mut ops, target, fb.points[right]);
            if plus.infinity || table.lookup_folded(plus.x + 1).is_some() {
                return None;
            }
            Some((target, vec![ni, nj, right]))
        })
        .expect("a deterministic opposite-orientation p31 witness");
    let targets = [target; 8];
    let (_, params) = Params::parse_spec("rootfind-kmitm:m=3").unwrap();

    let check = |oracle: &mut RootfindKMitmOracle<'_>| {
        let mut ops = GroupOps::default();
        let mut counters = OracleCounters::default();
        let mut out = vec![None; targets.len()];
        oracle.decompose_lanes(&ctx, &fb, &mut ops, &mut counters, &targets, &mut out);
        assert!(out
            .iter()
            .all(|witness| witness.as_ref() == Some(&expected)));
        for witness in out.iter().flatten() {
            let mut readd = fb.points[witness[0]];
            for &index in &witness[1..] {
                readd = inst.curve.add(&mut ops, readd, fb.points[index]);
            }
            assert_eq!(readd, target);
        }
        assert_eq!(oracle.mismatches, 0);
        assert_eq!(counters.frobfold_mismatches, 0);
    };

    // Portable shared-batch-inversion path: this keeps the regression live on
    // hosts without either vector backend.
    let mut scalar_lanes = RootfindKMitmOracle::new_prime(3, &inst);
    scalar_lanes
        .prepare(&ctx, &fb, &params, &mut GroupOps::default())
        .unwrap();
    scalar_lanes.set_ifma_enabled(false);
    scalar_lanes.set_avx2_enabled(false);
    check(&mut scalar_lanes);

    // Reproduce the backend that exposed the p31 bug when this host supports
    // it. The portable assertion above remains mandatory everywhere.
    let mut ifma_lanes = RootfindKMitmOracle::new_prime(3, &inst);
    ifma_lanes
        .prepare(&ctx, &fb, &params, &mut GroupOps::default())
        .unwrap();
    if ifma_lanes.ifma_prepared() {
        ifma_lanes.set_ifma_enabled(true);
        ifma_lanes.set_avx2_enabled(false);
        check(&mut ifma_lanes);
    }
}

#[derive(Clone, Copy)]
struct LaneCyclic(u64);

impl CountedGroup for LaneCyclic {
    type Elt = u64;

    fn identity(&self) -> Self::Elt {
        0
    }

    fn is_identity(&self, point: &Self::Elt) -> bool {
        *point == 0
    }

    fn add(&self, ops: &mut GroupOps, left: Self::Elt, right: Self::Elt) -> Self::Elt {
        ops.adds += 1;
        (left + right) % self.0
    }

    fn double(&self, ops: &mut GroupOps, point: Self::Elt) -> Self::Elt {
        ops.doubles += 1;
        (2 * point) % self.0
    }

    fn neg(&self, point: Self::Elt) -> Self::Elt {
        (self.0 - point) % self.0
    }

    fn key(&self, point: &Self::Elt) -> u64 {
        *point
    }

    fn native_lane_walk_adder(
        &self,
        _batched: bool,
    ) -> Option<
        Box<
            dyn FnMut(&mut GroupOps, &[(usize, Self::Elt, Self::Elt, usize)], &mut Vec<Self::Elt>)
                + '_,
        >,
    > {
        let modulus = self.0;
        Some(Box::new(move |ops, requests, out| {
            out.clear();
            out.reserve(requests.len());
            for &(_, point, addend, _) in requests {
                ops.adds += 1;
                out.push((point + addend) % modulus);
            }
        }))
    }

    fn supports_native_lane_walk(&self) -> bool {
        true
    }
}

#[test]
fn batch_collector_preserves_query_order_across_duplicate_guards_and_trial_cap() {
    let group = LaneCyclic(17);
    let fb = FactorBase::empty("empty collector test base".into());

    let mut scalar_seen = Vec::new();
    let mut scalar_matrix = IncrementalGauss::new(1, 17);
    let scalar = collect_and_solve_relations_touching_check_batched(
        &group,
        1,
        5,
        17,
        1,
        &fb,
        20261126,
        2048,
        TargetSource::WalkPrimeLanes {
            lanes: 8,
            batched: false,
        },
        RestartPool::Lazy,
        &mut scalar_matrix,
        CompletionRule::TargetPinned,
        |_, _, point, a, b| {
            scalar_seen.push((point, a, b));
            Vec::new()
        },
        |_| {},
        false,
        RecoveryCheck::DoubleAndAdd,
        None,
        1,
        None,
    );

    let mut batch_seen = Vec::new();
    let mut batch_fn = |_: &mut GroupOps,
                        _: &mut OracleCounters,
                        points: &[u64],
                        coefficients: &[(u64, u64)],
                        _: &mut [Vec<Relation>]| {
        batch_seen.extend(
            points
                .iter()
                .copied()
                .zip(coefficients.iter().copied())
                .map(|(point, (a, b))| (point, a, b)),
        );
    };
    let batch_callback: &mut BatchRelationsOracle<'_, u64> = &mut batch_fn;
    let mut batch_matrix = IncrementalGauss::new(1, 17);
    let batch = collect_and_solve_relations_touching_check_batched(
        &group,
        1,
        5,
        17,
        1,
        &fb,
        20261126,
        2048,
        TargetSource::WalkPrimeLanes {
            lanes: 8,
            batched: false,
        },
        RestartPool::Lazy,
        &mut batch_matrix,
        CompletionRule::TargetPinned,
        |_, _, _, _, _| Vec::new(),
        |_| {},
        false,
        RecoveryCheck::DoubleAndAdd,
        None,
        8,
        Some(batch_callback),
    );

    assert_eq!(scalar_seen, batch_seen);
    assert_eq!(scalar.trials, batch.trials);
    assert!(batch.relations.get("repeated_targets_skipped") > 0);
    assert_eq!(batch.relations.get("oracle_batch_preferred_width"), 8);
    assert!(batch.relations.get("oracle_batch_max_targets") <= 8);

    let mut partial_seen = 0u64;
    let mut partial_fn = |_: &mut GroupOps,
                          _: &mut OracleCounters,
                          points: &[u64],
                          _: &[(u64, u64)],
                          _: &mut [Vec<Relation>]| {
        partial_seen += points.len() as u64;
    };
    let partial_callback: &mut BatchRelationsOracle<'_, u64> = &mut partial_fn;
    let mut partial_matrix = IncrementalGauss::new(1, 17);
    let partial = collect_and_solve_relations_touching_check_batched(
        &group,
        1,
        5,
        17,
        1,
        &fb,
        9,
        3,
        TargetSource::WalkPrimeLanes {
            lanes: 8,
            batched: false,
        },
        RestartPool::Lazy,
        &mut partial_matrix,
        CompletionRule::TargetPinned,
        |_, _, _, _, _| Vec::new(),
        |_| {},
        false,
        RecoveryCheck::DoubleAndAdd,
        None,
        8,
        Some(partial_callback),
    );
    assert_eq!(partial.trials, 3);
    assert_eq!(partial_seen, 3);
    assert_eq!(partial.relations.get("oracle_batch_max_targets"), 3);
}

#[test]
fn early_stop_prices_pending_results_and_duplicate_guard_sentinel() {
    let group = LaneCyclic(17);
    let fb = FactorBase::empty("empty collector test base".into());
    let mut witnessed = None;

    for seed in 0..1024 {
        let mut batch_fn = |_: &mut GroupOps,
                            _: &mut OracleCounters,
                            _points: &[u64],
                            _coefficients: &[(u64, u64)],
                            out: &mut [Vec<Relation>]| {
            for relations in out {
                relations.push(Relation::Row {
                    columns: Vec::new(),
                    a: 12,
                    b: 1,
                });
            }
        };
        let batch_callback: &mut BatchRelationsOracle<'_, u64> = &mut batch_fn;
        let mut matrix = IncrementalGauss::new(1, 17);
        let outcome = collect_and_solve_relations_touching_check_batched(
            &group,
            1,
            5,
            17,
            1,
            &fb,
            seed,
            128,
            TargetSource::WalkPrimeLanes {
                lanes: 8,
                batched: false,
            },
            RestartPool::Lazy,
            &mut matrix,
            CompletionRule::TargetPinned,
            |_, _, _, _, _| Vec::new(),
            |_| {},
            false,
            RecoveryCheck::DoubleAndAdd,
            None,
            8,
            Some(batch_callback),
        );
        if outcome.relations.get("oracle_batch_unused_guarded_trials") > 0 {
            witnessed = Some(outcome);
            break;
        }
    }

    let outcome = witnessed.expect("a fixed small-group seed must leave a guard sentinel pending");
    assert!(outcome.verified);
    assert_eq!(outcome.recovered, Some(5));
    let unused_results = outcome.relations.get("oracle_batch_unused_results");
    let unused_guards = outcome.relations.get("oracle_batch_unused_guarded_trials");
    assert_eq!(unused_guards, 1);
    assert_eq!(outcome.trials, 1 + unused_results + unused_guards);
}
