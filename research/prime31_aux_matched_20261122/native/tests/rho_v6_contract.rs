//! Small-field correctness and integration contracts only. These are not
//! Prime31 executions, calibration runs, or performance comparisons.

use prime31_aux_matched::cryptanalysis::ic_boundary::{
    rho_walk_v3, CountedGroup, GroupOps, NegationClasses, PrimeCurve, PrimePoint, RhoClasses, RhoV3,
};
use prime31_aux_matched::cryptanalysis::prime_fast::{rho_parallel, FastCurve, RhoConfig};
use prime31_aux_matched::replay::{Curve, Point};
use prime31_aux_matched::SOURCE_MANIFEST;
use serde_json::Value;

const R: u64 = 109;

fn fixture() -> (PrimeCurve, PrimePoint, Curve, Point) {
    let curve = PrimeCurve { p: 127, a: 1, b: 7 };
    let generator = PrimePoint::affine(1, 3);
    let replay = Curve { p: 127, a: 1, b: 7 };
    let base = Point::affine(1, 3);
    assert_eq!(replay.scalar(base, R).unwrap(), Point::IDENTITY);
    (curve, generator, replay, base)
}

#[test]
fn negation_classes_preserve_points_coefficients_and_automorphism_count() {
    let (curve, generator, _, _) = fixture();
    let classes = NegationClasses { r: R };
    assert_eq!(
        <NegationClasses as RhoClasses<PrimeCurve>>::automorphisms(&classes),
        2
    );
    assert_eq!(
        <NegationClasses as RhoClasses<PrimeCurve>>::negation_rule(&classes),
        Some(R)
    );
    for scalar in [1, 2, 37, 108] {
        let mut ops = GroupOps::default();
        let point = curve.mul(&mut ops, generator, scalar);
        let (canonical, multiplier) = classes.canon(&curve, &mut ops, point);
        let (negative_canonical, _) = classes.canon(&curve, &mut ops, curve.neg(point));
        assert_eq!(canonical, negative_canonical);
        assert!(multiplier == 1 || multiplier == R - 1);
        assert_eq!(curve.mul(&mut ops, point, multiplier), canonical);
    }
}

#[test]
fn production_counted_v6_recovers_and_independently_replays_smallfield_targets() {
    let (curve, generator, replay, base) = fixture();
    let config = RhoV3::v6(true);
    assert!(config.negation && config.g_only && config.dp_from_floor && config.ladder_verify);
    assert_eq!(config.weight, 2);
    for (secret, seed) in [(37, 2), (72, 7)] {
        let public = replay.scalar(base, secret).unwrap();
        let target = PrimePoint::affine(public.x, public.y);
        let result = rho_walk_v3(
            &curve,
            &NegationClasses { r: R },
            generator,
            target,
            R,
            seed,
            20_000,
            config,
        );
        assert!(result.verified);
        assert_eq!(result.automorphisms, 2);
        let recovered = result.recovered.expect("fixed toy fixture must recover");
        assert_eq!(recovered, secret);
        assert_eq!(replay.scalar(base, recovered).unwrap(), public);
        assert!(result.counters["setup_additions"] + result.counters["setup_doublings"] > 0);
        assert!(result.counters["verification_operations"] > 0);
    }
}

#[test]
fn counted_v6_zero_search_budget_stays_unverified() {
    let (curve, generator, replay, base) = fixture();
    let public = replay.scalar(base, 37).unwrap();
    let result = rho_walk_v3(
        &curve,
        &NegationClasses { r: R },
        generator,
        PrimePoint::affine(public.x, public.y),
        R,
        2,
        0,
        RhoV3::v6(true),
    );
    assert_eq!(result.automorphisms, 2);
    assert!(!result.verified);
    assert!(result.recovered.is_none());
    assert_eq!(result.steps, 0);
}

#[test]
fn prescribed_prime_fast_tuned_reference_recovers_same_smallfield_target() {
    let (_, _, replay, base) = fixture();
    let public = replay.scalar(base, 37).unwrap();
    let curve = FastCurve::new("p127-r109-correctness-only", 127, 1, 7, R);
    let generator = curve.point(base.x, base.y);
    let target = curve.point(public.x, public.y);
    let mut config = RhoConfig::tuned(R);
    assert_eq!(config.threads, 1);
    assert!(config.negation && config.look_ahead && config.stride_starts);
    config.seed = 2;
    config.max_steps = 20_000;
    let result =
        rho_parallel(&curve, generator, target, &config).expect("fixed toy fixture must recover");
    assert_eq!(result.threads, 1);
    assert_eq!(result.log, 37);
    assert_eq!(replay.scalar(base, result.log).unwrap(), public);
    assert!(result.ops.table_adds + result.ops.table_doubles > 0);
    assert!(result.ops.collision_checks > 0);
}

#[test]
fn both_references_remain_pinned_verbatim_production_ranges() {
    let manifest: Value = serde_json::from_str(SOURCE_MANIFEST).unwrap();
    let ranges = manifest["extracted_production_ranges"].as_array().unwrap();
    for (path, pin, required_scope) in [
        (
            "research/prime31_aux_matched_20261122/isogeny-choice/upstream/ic_boundary.rs",
            "1f94ec22dbf7f865fd309e4fb770478dd555afd3282808143526fa775b81726d",
            "complete production RhoV3",
        ),
        (
            "research/prime31_aux_matched_20261122/isogeny-choice/upstream/prime_fast.rs",
            "03397cbcd5146df5341eea15febe34912bfed346d25759343a33a7b123712731",
            "complete production prime_fast RhoConfig",
        ),
    ] {
        let file = ranges.iter().find(|r| r["path"] == path).unwrap();
        assert_eq!(file["whole_source_sha256"], pin);
        assert_eq!(file["pinned_whole_source_sha256"], pin);
        assert_eq!(file["inserted_bytes"], 0);
        assert_eq!(file["rewritten"], false);
        assert!(file["ranges"].as_array().unwrap().iter().any(|r| {
            r["scope"].as_str().unwrap().starts_with(required_scope)
                && r["rewritten"] == false
                && r["selected_bytes"].as_u64().unwrap() > 0
        }));
    }
}
