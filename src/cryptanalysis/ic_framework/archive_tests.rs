use crate::cryptanalysis::ic_boundary::{
    koblitz_instance, random_binary_instance, roster_prime_instance, BinaryGroup, Calibration,
    CountedGroup, FactorBase, GroupOps, IncrementalGauss, RelationSolver, RowStatus,
};
use crate::cryptanalysis::ic_framework::{
    archive::{self, ArchiveInstance, BinaryArchiveBase, PrimeArchiveBase},
    plugins::{
        FrobeniusMitmOracle, KoblitzSymmetrisedBase, PrimeAbscissaBase, PrimeBatchMitmOracle,
        SubtractOracle, SymmetrisedOracle,
    },
    run_pipeline,
    stages::{DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params, Targets},
    useful_rank::{self, AuditedMatrix},
    PipelineSpec,
};
use factor_base::{
    recipe::{Config, Recipe, Shape},
    store,
};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
static SEQ: AtomicU64 = AtomicU64::new(0);

#[test]
fn archive_symmetry_options_preserve_points_and_replay_column_maps() {
    use crate::cryptanalysis::glv_invariant_base::{generate_cm_instance, CmFamily};
    for (family, selected) in [(CmFamily::J0, "cubic"), (CmFamily::J1728, "quartic")] {
        let i = generate_cm_instance(family, 10, 19, 8).unwrap();
        let cache = Cache::new();
        let id = store::build(
            &Config {
                curve: archive::prime_spec(&i),
                recipe: Recipe::PrimeProgression {
                    start: "0".into(),
                    step: "1".into(),
                    count: i.curve.p as usize,
                },
                max_candidates: i.curve.p as usize,
                shard_points: 64,
            },
            &cache.0,
        )
        .unwrap();
        let g = i.generator_point();
        let ctx = InstanceCtx {
            group: &i.curve,
            generator: g,
            target: g,
            r: i.r,
            cofactor: i.cofactor,
            group_order: i.group_order,
            name: i.name.clone(),
            field_degree: None,
        };
        let builder = PrimeArchiveBase { instance: &i };
        let mut ops = GroupOps::default();
        let mut params = cache.params(&id);
        params.set("symmetry", "none");
        let plain = builder.build(&ctx, &params, &mut ops).unwrap();
        assert_eq!(plain.columns, plain.points.len());
        params.set("symmetry", "negation");
        let negation = builder.build(&ctx, &params, &mut ops).unwrap();
        params.set("symmetry", selected);
        let folded = builder.build(&ctx, &params, &mut ops).unwrap();
        assert_eq!(plain.points, folded.points);
        assert_eq!(negation.points, folded.points);
        assert!(folded.columns < negation.columns);
        let audit = AuditedMatrix::new(
            &ctx,
            &folded,
            Box::new(IncrementalGauss::new(folded.columns + 1, i.r)),
        )
        .unwrap();
        useful_rank::replay(&ctx, &folded, &audit.audit).unwrap();
        params.set("symmetry", "frobenius");
        assert!(builder
            .build(&ctx, &params, &mut ops)
            .err()
            .unwrap()
            .contains("unavailable"));
        params.set("symmetry", "negaton");
        assert!(builder
            .build(&ctx, &params, &mut ops)
            .err()
            .unwrap()
            .contains("unsupported archive symmetry"));
    }
}

#[test]
fn generic_binary_archives_support_prime_and_composite_extension_degrees() {
    for degree in [6, 7] {
        let i = random_binary_instance(degree, 73, 8).unwrap();
        let cache = Cache::new();
        let id = store::build(
            &Config {
                curve: archive::binary_spec(&i),
                recipe: Recipe::BinarySubspace {
                    shape: Shape::Coordinate,
                    dimension: degree as usize,
                    seed: 1,
                    offsets: vec!["0".into()],
                },
                max_candidates: 1 << degree,
                shard_points: 32,
            },
            &cache.0,
        )
        .unwrap();
        let ArchiveInstance::Binary(loaded) = archive::instance(&cache.0, &id).unwrap() else {
            panic!()
        };
        assert_eq!(archive::binary_spec(&loaded), archive::binary_spec(&i));
        let group = BinaryGroup(&loaded.fast);
        let ctx = InstanceCtx {
            group: &group,
            generator: loaded.generator,
            target: loaded.generator,
            r: loaded.r,
            cofactor: loaded.cofactor,
            group_order: loaded.group_order,
            name: loaded.name.clone(),
            field_degree: Some(degree),
        };
        let builder = BinaryArchiveBase { instance: &loaded };
        let mut params = cache.params(&id);
        let mut ops = GroupOps::default();
        let fb = builder.build(&ctx, &params, &mut ops).unwrap();
        let audit = AuditedMatrix::new(
            &ctx,
            &fb,
            Box::new(IncrementalGauss::new(fb.columns + 1, loaded.r)),
        )
        .unwrap();
        useful_rank::replay(&ctx, &fb, &audit.audit).unwrap();
        assert!(loaded.koblitz.is_none());
        params.set("symmetry", "frobenius");
        assert!(builder
            .build(&ctx, &params, &mut ops)
            .err()
            .unwrap()
            .contains("certified Koblitz"));
    }
}

#[test]
fn archived_frobenius_orbits_support_one_and_two_large_prime_relations() {
    let mut i = koblitz_instance(0, 19).unwrap();
    // Preserve a generator different from the catalog representative.
    i.generator = BinaryGroup(&i.fast).mul(&mut GroupOps::default(), i.generator, 3);
    let cache = Cache::new();
    let id = store::build(
        &Config {
            curve: archive::binary_spec(&i),
            recipe: Recipe::FrobeniusOrbits { seeds: 2, seed: 7 },
            max_candidates: 4096,
            shard_points: 32,
        },
        &cache.0,
    )
    .unwrap();
    let ArchiveInstance::Binary(mut loaded) = archive::instance(&cache.0, &id).unwrap() else {
        panic!()
    };
    assert!(loaded.koblitz.is_some());
    assert_eq!(loaded.generator, i.generator);
    assert_eq!(
        loaded.fast.lower(loaded.generator),
        *loaded.koblitz.as_ref().unwrap().generator()
    );
    let mut params = cache.params(&id);
    params.set("symmetry", "frobenius");
    {
        let group = BinaryGroup(&loaded.fast);
        let mut ops = GroupOps::default();
        let planted = 31337 % loaded.r;
        let ctx = InstanceCtx {
            group: &group,
            generator: loaded.generator,
            target: group.mul(&mut ops, loaded.generator, planted),
            r: loaded.r,
            cofactor: loaded.cofactor,
            group_order: loaded.group_order,
            name: loaded.name.clone(),
            field_degree: Some(loaded.n),
        };
        let builder = BinaryArchiveBase { instance: &loaded };
        let fb = builder.build(&ctx, &params, &mut ops).unwrap();
        let (_, archived_points) = store::load(&cache.0, &id).unwrap();
        assert_eq!(fb.points.len(), archived_points.len());
        assert_eq!(fb.columns, 2);
        for large_primes in [0, 1, 2] {
            let spec = PipelineSpec { factor_base: "archive".into(), factor_base_params: params.clone(),
                oracle: "mitm-frobenius-counted".into(),
                oracle_params: Params::parse_spec(&format!("mitm-frobenius-counted:m=3,large_primes={large_primes},lp_domain_factor=3,lp_raw_x_cap=4096")).unwrap().1,
                targets: Targets::Random, linalg: "incremental-gauss-full-rank-checked".into(),
                max_trials: 20000, seed: 5, ..Default::default() };
            let report = run_pipeline(
                &ctx,
                &spec,
                &builder,
                &mut FrobeniusMitmOracle::new_counted(3, &loaded),
                planted,
                &Calibration::default(),
                None,
            )
            .unwrap();
            assert!(report.verified, "large_primes={large_primes}: {report:?}");
            let audit = report.useful_rank.as_ref().unwrap();
            assert_eq!(audit.invalid_rows, 0);
            useful_rank::replay(&ctx, &fb, audit).unwrap();
            assert_eq!(report.decomposition.cost.get("lp_verification_failures"), 0);
            if large_primes != 0 {
                assert!(report.decomposition.cost.get("lp_partials_fed") > 0);
            }
            if large_primes == 1 {
                assert_eq!(report.decomposition.cost.get("lp_partials_two_large"), 0);
            } else if large_primes == 2 {
                assert!(report.decomposition.cost.get("lp_partials_two_large") > 0);
            }
        }
    }
    loaded.koblitz.as_mut().unwrap().lambda += 1u32;
    let group = BinaryGroup(&loaded.fast);
    let ctx = InstanceCtx {
        group: &group,
        generator: loaded.generator,
        target: loaded.generator,
        r: loaded.r,
        cofactor: loaded.cofactor,
        group_order: loaded.group_order,
        name: loaded.name.clone(),
        field_degree: Some(loaded.n),
    };
    assert!(BinaryArchiveBase { instance: &loaded }
        .build(&ctx, &params, &mut GroupOps::default())
        .err()
        .unwrap()
        .contains("coefficient replay"));
}

#[test]
fn imported_koblitz_instance_supports_the_native_torsion_coordinate_control() {
    let i = koblitz_instance(1, 17).unwrap();
    let cache = Cache::new();
    let id = store::build(
        &Config {
            curve: archive::binary_spec(&i),
            recipe: Recipe::FrobeniusOrbits { seeds: 1, seed: 7 },
            max_candidates: 4096,
            shard_points: 32,
        },
        &cache.0,
    )
    .unwrap();
    let ArchiveInstance::Binary(loaded) = archive::instance(&cache.0, &id).unwrap() else {
        panic!()
    };
    let group = BinaryGroup(&loaded.fast);
    let ctx = InstanceCtx {
        group: &group,
        generator: loaded.generator,
        target: loaded.generator,
        r: loaded.r,
        cofactor: loaded.cofactor,
        group_order: loaded.group_order,
        name: loaded.name.clone(),
        field_degree: Some(loaded.n),
    };
    let params = Params::parse_spec("koblitz-symmetrised:divisor=0;1,projected=1")
        .unwrap()
        .1;
    let mut ops = GroupOps::default();
    let fb = KoblitzSymmetrisedBase { instance: &loaded }
        .build(&ctx, &params, &mut ops)
        .unwrap();
    let audit = AuditedMatrix::new(
        &ctx,
        &fb,
        Box::new(IncrementalGauss::new(fb.columns + 1, loaded.r)),
    )
    .unwrap();
    useful_rank::replay(&ctx, &fb, &audit.audit).unwrap();
    assert!(audit.audit.cofactor_kernel_points > 0);
    let mut wrong_count = audit.audit.clone();
    wrong_count.cofactor_kernel_points += 1;
    assert!(useful_rank::replay(&ctx, &fb, &wrong_count).is_err());
    SymmetrisedOracle::new(2, &loaded)
        .prepare(&ctx, &fb, &params, &mut ops)
        .unwrap();
    let mut false_zero = fb;
    let nonzero = false_zero.coef_of.iter().position(|&c| c != 0).unwrap();
    false_zero.coef_of[nonzero] = 0;
    assert!(AuditedMatrix::new(
        &ctx,
        &false_zero,
        Box::new(IncrementalGauss::new(false_zero.columns + 1, loaded.r))
    )
    .err()
    .unwrap()
    .contains("nonzero cofactor projection"));
}

#[test]
fn shorter_subfield_action_does_not_claim_a_full_signed_frobenius_fold() {
    let mut i = koblitz_instance(1, 9).unwrap();
    assert_eq!(i.group_order, 518);
    i.r = 7;
    i.cofactor = 74;
    let group = BinaryGroup(&i.fast);
    let mut ops = GroupOps::default();
    i.generator = (1..512)
        .find_map(|x| {
            i.points_with_x(x)
                .into_iter()
                .map(|p| group.mul(&mut ops, p, 74))
                .find(|p| !p.infinity)
        })
        .unwrap();
    assert!(group.mul(&mut ops, i.generator, 7).infinity);
    let cache = Cache::new();
    let id = store::build(
        &Config {
            curve: archive::binary_spec(&i),
            recipe: Recipe::FrobeniusOrbits { seeds: 1, seed: 7 },
            max_candidates: 512,
            shard_points: 32,
        },
        &cache.0,
    )
    .unwrap();
    let ArchiveInstance::Binary(loaded) = archive::instance(&cache.0, &id).unwrap() else {
        panic!()
    };
    assert_eq!(loaded.r, 7);
    assert!(loaded.koblitz.is_none());
}
struct Cache(PathBuf);
impl Cache {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "ic-archive-{}-{}",
            std::process::id(),
            SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn params(&self, id: &str) -> Params {
        let mut p = Params::default();
        p.set("store", self.0.to_str().unwrap()).set("manifest", id);
        p
    }
}
impl Drop for Cache {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn prime_archive_preserves_points_and_rejects_identity_and_hash_mismatch() {
    let i = roster_prime_instance(16).unwrap();
    let c = Config {
        curve: archive::prime_spec(&i),
        recipe: Recipe::PrimeProgression {
            start: "0".into(),
            step: "1".into(),
            count: 256,
        },
        max_candidates: 256,
        shard_points: 7,
    };
    let cache = Cache::new();
    let id = store::build(&c, &cache.0).unwrap();
    let ArchiveInstance::Prime(loaded) = archive::instance(&cache.0, &id).unwrap() else {
        panic!()
    };
    assert_eq!(archive::prime_spec(&i), archive::prime_spec(&loaded));
    let g = i.generator_point();
    let mut ops = GroupOps::default();
    let q = i.curve.mul(&mut ops, g, 7);
    let mut ctx = InstanceCtx {
        group: &i.curve,
        generator: g,
        target: q,
        r: i.r,
        cofactor: i.cofactor,
        group_order: i.group_order,
        name: i.name.clone(),
        field_degree: None,
    };
    let builder = PrimeArchiveBase { instance: &i };
    let params = cache.params(&id);
    let fb = builder.build(&ctx, &params, &mut ops).unwrap();
    let (m, points) = store::load(&cache.0, &id).unwrap();
    assert_eq!(fb.points.len(), points.len());
    for (a, b) in fb.points.iter().zip(points) {
        assert_eq!(format!("0x{:x}", a.x), b.spec().x);
        assert_eq!(format!("0x{:x}", a.y), b.spec().y);
    }
    assert_eq!(fb.points.len(), 2 * fb.columns);
    assert!(fb.cost.get("archive_bytes_read") > 0);
    // A fresh cache populated with the exact immutable objects imports identically.
    let fetched = Cache::new();
    let manifest_bytes = std::fs::read(store::manifest_path(&cache.0, &id).unwrap()).unwrap();
    store::write_once(
        &store::manifest_path(&fetched.0, &id).unwrap(),
        &manifest_bytes,
    )
    .unwrap();
    for object in &m.objects {
        let bytes = std::fs::read(store::object_path(&cache.0, &object.sha256).unwrap()).unwrap();
        store::write_once(
            &store::object_path(&fetched.0, &object.sha256).unwrap(),
            &bytes,
        )
        .unwrap();
    }
    let imported = builder.build(&ctx, &fetched.params(&id), &mut ops).unwrap();
    assert_eq!(fb.points, imported.points);
    assert_eq!(fb.col_of, imported.col_of);
    assert_eq!(fb.coef_of, imported.coef_of);
    let mut oversized = m.clone();
    oversized.point_count = 8193;
    let bytes = store::canonical(&oversized).unwrap();
    let oversized_id = store::digest(&bytes);
    store::write_once(
        &store::manifest_path(&cache.0, &oversized_id).unwrap(),
        &bytes,
    )
    .unwrap();
    assert!(builder
        .build(&ctx, &cache.params(&oversized_id), &mut ops)
        .err()
        .unwrap()
        .contains("archive_ic_capacity"));
    ctx.generator = q;
    assert!(builder
        .build(&ctx, &params, &mut ops)
        .err()
        .unwrap()
        .contains("generator"));
    ctx.generator = g;
    let mut wrong = i.curve.clone();
    wrong.a = (wrong.a + 1) % wrong.p;
    ctx.group = &wrong;
    assert!(builder
        .build(&ctx, &params, &mut ops)
        .err()
        .unwrap()
        .contains("curve mismatch"));
    ctx.group = &i.curve;
    std::fs::write(
        store::object_path(&cache.0, &m.objects[0].sha256).unwrap(),
        b"corrupted",
    )
    .unwrap();
    assert!(builder.build(&ctx, &params, &mut ops).is_err());
}

#[test]
fn binary_archive_runs_and_replays_useful_rank() {
    let i = koblitz_instance(1, 11).unwrap();
    let c = Config {
        curve: archive::binary_spec(&i),
        recipe: Recipe::BinarySubspace {
            shape: Shape::Coordinate,
            dimension: 6,
            seed: 12,
            offsets: vec!["0".into()],
        },
        max_candidates: 256,
        shard_points: 7,
    };
    let cache = Cache::new();
    let id = store::build(&c, &cache.0).unwrap();
    let ArchiveInstance::Binary(loaded) = archive::instance(&cache.0, &id).unwrap() else {
        panic!()
    };
    assert_eq!(archive::binary_spec(&i), archive::binary_spec(&loaded));
    let group = BinaryGroup(&loaded.fast);
    let mut ops = GroupOps::default();
    let planted = 37;
    let ctx = InstanceCtx {
        group: &group,
        generator: loaded.generator,
        target: group.mul(&mut ops, loaded.generator, planted),
        r: loaded.r,
        cofactor: loaded.cofactor,
        group_order: loaded.group_order,
        name: loaded.name.clone(),
        field_degree: Some(loaded.n),
    };
    let ArchiveInstance::Binary(mut wrong) = archive::instance(&cache.0, &id).unwrap() else {
        panic!()
    };
    wrong.fast.a ^= 1;
    let wrong_group = BinaryGroup(&wrong.fast);
    let wrong_ctx = InstanceCtx {
        group: &wrong_group,
        generator: ctx.generator,
        target: ctx.target,
        r: ctx.r,
        cofactor: ctx.cofactor,
        group_order: ctx.group_order,
        name: ctx.name.clone(),
        field_degree: ctx.field_degree,
    };
    let wrong_builder = BinaryArchiveBase { instance: &wrong };
    assert!(wrong_builder
        .build(&wrong_ctx, &cache.params(&id), &mut ops)
        .err()
        .unwrap()
        .contains("native binary curve mismatch"));
    let builder = BinaryArchiveBase { instance: &loaded };
    let mut incomplete_orbits = cache.params(&id);
    incomplete_orbits.set("symmetry", "frobenius");
    assert!(builder
        .build(&ctx, &incomplete_orbits, &mut ops)
        .err()
        .unwrap()
        .contains("closed under signed Frobenius"));
    let spec = PipelineSpec {
        factor_base: "archive".into(),
        factor_base_params: cache.params(&id),
        oracle: "subtract".into(),
        targets: Targets::Random,
        linalg: "incremental-gauss".into(),
        max_trials: 10000,
        seed: 31,
        ..Default::default()
    };
    let report = run_pipeline(
        &ctx,
        &spec,
        &builder,
        &mut SubtractOracle,
        planted,
        &Calibration::default(),
        Some(1.0),
    )
    .unwrap();
    assert!(report.verified, "{report:?}");
    let audit = report.useful_rank.as_ref().unwrap();
    assert!(audit.independent_rows > 0);
    assert_eq!(
        audit.verified_rows,
        audit.independent_rows + audit.dependent_rows
    );
    assert_eq!(audit.rank as u64, report.linear_algebra.rank);
    assert!(report.s_over_rho.is_none());
    assert!(report.cost_qualification.is_some());
    let fb = builder
        .build(&ctx, &spec.factor_base_params, &mut ops)
        .unwrap();
    useful_rank::replay(&ctx, &fb, audit).unwrap();
    let mut tampered = audit.clone();
    tampered.receipts[0].rhs = (tampered.receipts[0].rhs + 1) % ctx.r;
    assert!(useful_rank::replay(&ctx, &fb, &tampered).is_err());
}

#[test]
fn audit_separates_scaled_duplicates_dependencies_and_false_equations() {
    let i = roster_prime_instance(16).unwrap();
    let g = i.generator_point();
    let mut ops = GroupOps::default();
    let twice = i.curve.mul(&mut ops, g, 2);
    let q = i.curve.mul(&mut ops, g, 5);
    let fb = FactorBase::from_column_map(
        "audit fixture".into(),
        vec![g, i.curve.neg(g), twice, i.curve.neg(twice)],
        vec![0, 0, 1, 1],
        vec![1, i.r - 1, 1, i.r - 1],
        2,
        |p| i.curve.key(p),
        |p| i.curve.key(&i.curve.neg(*p)),
        |p| p.x,
    )
    .unwrap();
    // A non-unit cofactor catches accidentally checking raw P instead of [h]P.
    let ctx = InstanceCtx {
        group: &i.curve,
        generator: g,
        target: q,
        r: i.r,
        cofactor: 4,
        group_order: i.r * 4,
        name: i.name,
        field_degree: None,
    };
    let mut matrix =
        AuditedMatrix::new(&ctx, &fb, Box::new(IncrementalGauss::new(3, i.r))).unwrap();
    assert_eq!(matrix.add_row(vec![1, 0, 0], 4), RowStatus::Independent);
    assert_eq!(matrix.add_row(vec![2, 0, 0], 8), RowStatus::Dependent);
    assert_eq!(matrix.add_row(vec![0, 1, 0], 8), RowStatus::Independent);
    assert_eq!(matrix.add_row(vec![1, 1, 0], 12), RowStatus::Dependent);
    assert_eq!(matrix.add_row(vec![0, 0, 1], 5), RowStatus::Independent);
    assert_eq!(matrix.audit.independent_rows, 3);
    assert_eq!(matrix.audit.dependent_rows, 2);
    assert_eq!(matrix.audit.duplicate_rows, 1);
    assert_eq!(matrix.audit.base_rank, 2);
    useful_rank::replay(&ctx, &fb, &matrix.audit).unwrap();
    let complete = matrix.audit.clone();
    let mut truncated = matrix.audit.clone();
    truncated.transcript_complete = false;
    assert!(useful_rank::replay(&ctx, &fb, &truncated).is_err());
    let mut counters = matrix.audit.clone();
    counters.duplicate_rows += 1;
    assert!(useful_rank::replay(&ctx, &fb, &counters).is_err());
    assert_eq!(matrix.add_row(vec![1, 0, 0], 5), RowStatus::Inconsistent);
    assert_eq!(matrix.audit.invalid_rows, 1);
    assert_eq!(matrix.rank(), 3);
    assert!(useful_rank::replay(&ctx, &fb, &matrix.audit).is_err());
    // Fill the bounded transcript using true zero equations: retained receipts
    // stop, while group checks and rank accounting continue for the entire tail.
    for _ in 0..50_000 {
        matrix.add_row(vec![0, 0, 0], 0);
    }
    assert!(!matrix.audit.transcript_complete);
    assert!(matrix.audit.duplicate_checks_omitted > 0);
    assert_eq!(matrix.audit.verified_rows, 50_005);
    assert_eq!(matrix.audit.independent_rows, 3);
    assert_eq!(matrix.audit.rank, 3);
    assert!(matrix.audit.receipts.len() < matrix.audit.rows_seen as usize);
    let mut oversized = fb;
    oversized.columns = usize::MAX;
    assert!(useful_rank::replay(&ctx, &oversized, &complete)
        .unwrap_err()
        .contains("1..4096 columns"));
}

#[test]
fn wide_archive_is_rejected_without_truncation() {
    // Frozen wide manifests need no shards to reach the explicit native-width gate.
    let root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools/factor-base/validation/archive");
    let mut wide = 0;
    for entry in std::fs::read_dir(root.join("manifests")).unwrap() {
        let path = entry.unwrap().path();
        let id = path.file_stem().unwrap().to_str().unwrap();
        let m: store::Manifest = store::read_json(&path).unwrap();
        if m.config.curve.subgroup_order.len() > 18 {
            assert!(archive::instance(&root, id)
                .err()
                .unwrap()
                .contains("unsupported_archive_width"));
            wide += 1;
        }
    }
    assert!(wide >= 2);
}

#[test]
fn comparison_protocol_hash_and_holdout_are_frozen() {
    let bytes = include_bytes!("../../../docs/ic/archive-comparison-v1.json");
    let receipt = include_str!("../../../docs/ic/archive-comparison-v1.json.sha256");
    assert_eq!(
        store::digest(bytes),
        receipt.split_whitespace().next().unwrap()
    );
    let protocol: serde_json::Value = serde_json::from_slice(bytes).unwrap();
    let discovery = protocol["discovery"]["workload_seeds"].as_array().unwrap();
    let confirmation = protocol["confirmation"]["workload_seeds"]
        .as_array()
        .unwrap();
    assert!(!confirmation.is_empty());
    assert!(confirmation.iter().all(|seed| !discovery.contains(seed)));
    assert!(protocol["total_runtime_winner"].is_null());
    let v2_bytes = include_bytes!("../../../docs/ic/archive-technique-comparison-v2.json");
    let v2_receipt = include_str!("../../../docs/ic/archive-technique-comparison-v2.json.sha256");
    assert_eq!(
        store::digest(v2_bytes),
        v2_receipt.split_whitespace().next().unwrap()
    );
    let v2: serde_json::Value = serde_json::from_slice(v2_bytes).unwrap();
    assert_eq!(
        v2["base_selection_protocol"]["sha256"],
        store::digest(bytes)
    );
    assert!(v2["total_runtime_winner"].is_null());
    assert_eq!(
        v2["pipeline"]["linalg"],
        "incremental-gauss-full-rank-checked"
    );
}

#[test]
fn audit_flag_is_not_forwarded_to_strict_oracle_parameters() {
    let i = roster_prime_instance(16).unwrap();
    let g = i.generator_point();
    let mut ops = GroupOps::default();
    let ctx = InstanceCtx {
        group: &i.curve,
        generator: g,
        target: i.curve.mul(&mut ops, g, 17),
        r: i.r,
        cofactor: i.cofactor,
        group_order: i.group_order,
        name: i.name.clone(),
        field_degree: None,
    };
    let mut spec = PipelineSpec {
        factor_base: "prime-abscissa".into(),
        oracle: "mitm-prime-batch".into(),
        targets: Targets::Random,
        max_trials: 0,
        ..Default::default()
    };
    spec.factor_base_params.set("size", "4");
    spec.oracle_params.set("audit_rank", "true");
    let report = run_pipeline(
        &ctx,
        &spec,
        &PrimeAbscissaBase { instance: &i },
        &mut PrimeBatchMitmOracle::new(),
        17,
        &Calibration::default(),
        None,
    )
    .unwrap();
    assert_eq!(report.useful_rank.unwrap().rows_seen, 0);
    assert!(report.cost_qualification.is_some());
    assert!(!report.verified);
}
