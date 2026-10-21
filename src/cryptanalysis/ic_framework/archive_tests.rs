use crate::cryptanalysis::ic_boundary::{
    koblitz_instance, roster_prime_instance, BinaryGroup, Calibration, CountedGroup, FactorBase,
    GroupOps, IncrementalGauss, RelationSolver, RowStatus,
};
use crate::cryptanalysis::ic_framework::{
    archive::{self, ArchiveInstance, BinaryArchiveBase, PrimeArchiveBase},
    plugins::{PrimeAbscissaBase, PrimeBatchMitmOracle, SubtractOracle},
    run_pipeline,
    stages::{FactorBaseBuilder, InstanceCtx, Params, Targets},
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
