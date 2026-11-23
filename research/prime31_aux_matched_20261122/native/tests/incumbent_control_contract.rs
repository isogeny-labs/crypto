//! Bounded public-target correctness contracts for the maintained complete IC
//! adapter. These small fixtures are not Prime31 performance evidence.

use prime31_aux_matched::collector::PublicTarget;
use prime31_aux_matched::cryptanalysis::prime_fast::{self, FastCurve};
use prime31_aux_matched::incumbent_control::{
    solve_fast_cold, FastIcConfig, FastIcStatus, MAX_FAST_IC_BASE, MAX_FAST_IC_EXTRA_RELATIONS,
    MAX_FAST_IC_TARGET_WALK,
};
use prime31_aux_matched::replay::{Curve, Point};
use prime31_aux_matched::SOURCE_MANIFEST;

fn public_target() -> PublicTarget {
    let curve = Curve { p: 19, a: 0, b: 2 };
    let generator = Point::affine(4, 3);
    PublicTarget {
        p: curve.p,
        a: curve.a,
        b: curve.b,
        order: 13,
        generator,
        target: curve.scalar(generator, 7).unwrap(),
    }
}

fn bounded_config() -> FastIcConfig {
    FastIcConfig {
        fb_size: Some(1),
        extra_relations: 0,
        max_target_walk: 64,
        seed: 4188,
    }
}

#[test]
fn complete_cold_adapter_recovers_a_public_target_and_replays_independently() {
    let target = public_target();
    let config = bounded_config();
    let result = solve_fast_cold(&target, &config).unwrap();
    assert_eq!(result.status, FastIcStatus::Recovered);
    assert_eq!(result.recovered_scalar, Some(7));
    assert!(result.independent_replay);
    let report = result.native_report.as_ref().unwrap();
    assert_eq!(report.factor_base_size, 1);
    assert_eq!(report.determined_logs, 1);
    assert!((1..=config.max_target_walk).contains(&report.target_points_walked));
    assert!(result.native_operation_counters.is_none());
    assert_eq!(result.accounting["cold_targets"], 1);
    assert_eq!(result.accounting["shared_setup_reuse"], false);
    assert_eq!(result.accounting["target_walk_cap_bounds_setup"], false);
    assert_eq!(result.accounting["setup_round_guard"], 50_000_000u64);
    assert_eq!(result.accounting["complete_operation_ledger"], false);
    // Internal diagnostics are not checked for a timing ratio or speed claim.
}

#[test]
fn adapter_preserves_maintained_solver_answer_and_non_timing_report() {
    let target = public_target();
    let config = bounded_config();
    let curve = FastCurve::new(
        "direct-incumbent-contract",
        target.p,
        target.a,
        target.b,
        target.order,
    );
    let (direct_scalar, direct) = prime_fast::ic_solve_fast(
        &curve,
        curve.point(target.generator.x, target.generator.y),
        curve.point(target.target.x, target.target.y),
        config.fb_size,
        config.extra_relations,
        config.max_target_walk,
        config.seed,
    )
    .expect("bounded maintained fixture recovers");
    let adapted = solve_fast_cold(&target, &config).unwrap();
    assert_eq!(adapted.recovered_scalar, Some(direct_scalar));
    let report = adapted.native_report.unwrap();
    assert_eq!(report.factor_base_size, direct.factor_base_size);
    assert_eq!(report.determined_logs, direct.determined_logs);
    assert_eq!(report.target_points_walked, direct.target_points_walked);
}

#[test]
fn invalid_dimensions_are_rejected_before_native_setup() {
    let target = public_target();
    let valid = bounded_config();
    let invalid = [
        FastIcConfig {
            fb_size: Some(0),
            ..valid.clone()
        },
        FastIcConfig {
            fb_size: Some(MAX_FAST_IC_BASE + 1),
            ..valid.clone()
        },
        FastIcConfig {
            fb_size: Some(usize::MAX),
            ..valid.clone()
        },
        FastIcConfig {
            extra_relations: MAX_FAST_IC_EXTRA_RELATIONS + 1,
            ..valid.clone()
        },
        FastIcConfig {
            extra_relations: usize::MAX,
            ..valid.clone()
        },
        FastIcConfig {
            max_target_walk: 0,
            ..valid.clone()
        },
        FastIcConfig {
            max_target_walk: MAX_FAST_IC_TARGET_WALK + 1,
            ..valid.clone()
        },
    ];
    for config in invalid {
        assert!(config.validate_limits(&target).is_err());
        assert!(solve_fast_cold(&target, &config).is_err());
    }
    let mut bad = target.clone();
    bad.p = 1u64 << 32;
    assert!(solve_fast_cold(&bad, &valid).is_err());
    let mut bad = target.clone();
    bad.order = 2;
    assert!(solve_fast_cold(&bad, &valid).is_err());
    let mut bad = target;
    bad.target = Point::affine(bad.p, 0);
    assert!(solve_fast_cold(&bad, &valid).is_err());
}

#[test]
fn production_manifest_selects_the_whole_unmodified_ic_pipeline() {
    let manifest: serde_json::Value = serde_json::from_str(SOURCE_MANIFEST).unwrap();
    let source = manifest["extracted_production_ranges"]
        .as_array()
        .unwrap()
        .iter()
        .find(|value| value["generated_name"] == "prime_fast.rs")
        .expect("prime_fast production closure");
    let range = source["ranges"]
        .as_array()
        .unwrap()
        .iter()
        .find(|value| {
            value["start_anchor"]
                .as_str()
                .unwrap()
                .starts_with("// ── Index calculus: factor base")
        })
        .expect("complete maintained IC range is included");
    assert_eq!(source["rewritten"], false);
    assert_eq!(source["inserted_bytes"], 0);
    assert!(range["selected_bytes"].as_u64().unwrap() > 0);
    assert!(range["end_anchor"]
        .as_str()
        .unwrap()
        .starts_with("// ── Shared factor-base subproduct tree"));
    assert_eq!(manifest["prime_fast_complete_ic_included"], true);
}
