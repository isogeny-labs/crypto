//! Correctness-only native integration fixture, not an isolated timing result.
//! `cargo run --release --example koblitz_halftrace`
//! The measured A/B protocol is research/koblitz_halftrace_20261030/spec.json.
use crypto_lib::cryptanalysis::koblitz_fast::with_cached_point_lifts;
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_frobenius_factor_base, koblitz_index_calculus_dlp_with_factor_base, KoblitzCurve,
    KoblitzIcOptions,
};
use num_bigint::BigUint;

fn check() {
    let kc = KoblitzCurve::new(0, 9).expect("public synthetic K_0 test curve");
    let old = with_cached_point_lifts(false, || build_frobenius_factor_base(&kc, 0)).unwrap();
    let new = with_cached_point_lifts(true, || build_frobenius_factor_base(&kc, 0)).unwrap();
    assert_eq!(old.points, new.points);
    assert_eq!(old.orbit_of, new.orbit_of);
    assert_eq!(old.signed_orbit_of, new.signed_orbit_of);
    for m in [2, 3] {
        for secret in [3u64, 7, 13] {
            let secret = BigUint::from(secret) % &kc.subgroup_order;
            let q = kc.mul(kc.generator(), &secret);
            let options = KoblitzIcOptions {
                m,
                seed: 20261030,
                allow_direct_relation: false,
                ..Default::default()
            };
            let a = koblitz_index_calculus_dlp_with_factor_base(&kc, &q, &old, &options).unwrap();
            let b = koblitz_index_calculus_dlp_with_factor_base(&kc, &q, &new, &options).unwrap();
            assert_eq!(a.log.as_ref(), Some(&secret));
            assert_eq!(b.log, a.log);
            assert_eq!(b.trials, a.trials);
            assert_eq!(b.independent_relations, a.independent_relations);
            assert_eq!(kc.mul(kc.generator(), b.log.as_ref().unwrap()), q);
        }
    }
}

fn main() {
    check();
    println!("native halftrace integration: ordered bases, orbit maps and six two/three-summand DLP cases verified; no timing claim");
}

#[test]
fn native_end_to_end_halftrace_parity() {
    check();
}

#[test]
fn native_ecbench_recipe_recovers_the_same_public_point() {
    use crypto_lib::cryptanalysis::ecbench::methods::{resolve, solve, MethodSpec};
    use crypto_lib::cryptanalysis::ecbench::workload::{CurveSpec, Instance};
    let spec = CurveSpec::Koblitz { a: 0, n: 9 };
    let instance = spec.build().unwrap();
    let facts = instance.facts(&spec);
    let Instance::Binary(binary) = &instance else {
        unreachable!()
    };
    let target = binary.fast.mul_u64(binary.generator, 53);
    let coordinates = [format!("0x{:x}", target.x), format!("0x{:x}", target.y)];
    let run = |tables: bool| {
        let base = if tables {
            "koblitz-orbit:divisor=2,halftrace_tables=1"
        } else {
            "koblitz-orbit:divisor=2"
        };
        let method = resolve(&MethodSpec {
            id: "ic.pipeline".into(),
            params: [
                ("factor_base".into(), base.into()),
                ("oracle".into(), "mitm-frobenius-counted:m=3".into()),
                (
                    "linalg".into(),
                    "incremental-gauss-full-rank-checked".into(),
                ),
            ]
            .into_iter()
            .collect(),
        })
        .unwrap();
        solve(&method, &instance, &facts, &coordinates, 20261030).unwrap()
    };
    let (old, new) = (run(false), run(true));
    assert_eq!(old.recovered, Some(u128::from(53 % binary.r)));
    assert_eq!(new.recovered, old.recovered);
    assert_eq!(
        new.factor_base.as_ref().unwrap().points_sha256,
        old.factor_base.as_ref().unwrap().points_sha256
    );
    for name in ["targets_tried", "relations_found", "matrix_rank"] {
        assert_eq!(new.counters[name], old.counters[name], "{name}");
    }
}
