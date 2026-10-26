use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use crypto_lib::cryptanalysis::ecbench::methods::{resolve, solve as ecbench_solve, MethodSpec};
use crypto_lib::cryptanalysis::ecbench::workload::{CurveSpec, Workload};
use crypto_lib::cryptanalysis::ecbench_large_prime::{
    import_family, load_manifest, load_manifest_with_source, solve, LargePrimeGraph, SolveConfig,
};

fn fixture() -> crypto_lib::cryptanalysis::ecbench_large_prime::ImportedFamily {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ecbench_manifest_small.json");
    let manifest = load_manifest(&path).unwrap();
    import_family(&manifest.families[0]).unwrap().0
}

#[test]
fn corpus_source_identity_is_recorded_from_exact_bytes() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ecbench_manifest_small.json");
    let loaded = load_manifest_with_source(&path).unwrap();
    assert_eq!(loaded.bytes, 568);
    assert_eq!(
        loaded.sha256,
        "235f49a6d07a5f7afb18a37f574ebc7ee1ae0b69dc9d4a33cd0b8ff15556b5b7"
    );
    assert_eq!(loaded.manifest.families.len(), 1);
}

#[test]
fn ic_report_binds_the_exact_manifest_bytes() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ecbench_manifest_small.json");
    let output = Command::new(env!("CARGO_BIN_EXE_ic"))
        .args(["--json", "large-prime", "--manifest"])
        .arg(&path)
        .arg("--validate-only")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["status"], "checks_passed");
    assert_eq!(report["manifest_bytes"], 568);
    assert_eq!(
        report["manifest_sha256"],
        "235f49a6d07a5f7afb18a37f574ebc7ee1ae0b69dc9d4a33cd0b8ff15556b5b7"
    );
    assert_eq!(report["manifest_schema"], "ecbench.curve-corpus/v1");
}

#[test]
fn corpus_smallest_imports_and_solves_with_n_minus_one() {
    let family = fixture();
    let report = solve(
        &family.instance,
        family.target,
        SolveConfig {
            small_dimension: family.small_dimension,
            envelope_dimension: family.envelope_dimension,
            summands: family.instance.n - 1,
            max_large_primes: 2,
            max_trials: 20_000,
            max_states: 100_000,
            seed: 7,
            large_prime_graph: Default::default(),
        },
    )
    .unwrap();
    assert_eq!(report.recovered, Some(family.expected));
    assert!(report.verified_fast && report.verified_independent);
    assert!(report.verification_performed);
    assert!(report.counters.relation_histogram[2] > 0);
}

#[test]
fn exact_state_cap_refuses_without_downscaling() {
    let family = fixture();
    let err = solve(
        &family.instance,
        family.target,
        SolveConfig {
            small_dimension: family.small_dimension,
            envelope_dimension: family.envelope_dimension,
            summands: family.instance.n - 1,
            max_large_primes: 2,
            max_trials: 1,
            max_states: 10,
            seed: 7,
            large_prime_graph: Default::default(),
        },
    )
    .unwrap_err();
    assert!(err.contains("exact 10-summand MITM"));
}

#[test]
fn native_ecbench_runs_large_prime_and_matched_rho_on_one_workload() {
    let curve = CurveSpec::BinaryExplicit {
        n: 11,
        modulus: 2323,
        a: 1,
        b: 1,
        group_order: 1982,
        r: 991,
        gx: 1193,
        gy: 1007,
    };
    let instance = curve.build().unwrap();
    let workload = Workload::on(&curve, &instance, 20261004, 0, Default::default()).unwrap();
    let method = resolve(&MethodSpec {
        id: "ic.large_prime".into(),
        params: BTreeMap::from([
            ("small_dimension".into(), "3".into()),
            ("envelope_dimension".into(), "4".into()),
            ("summands".into(), "n-1".into()),
            ("large_primes".into(), "2".into()),
            ("max_trials".into(), "20000".into()),
            ("max_states".into(), "100000".into()),
        ]),
    })
    .unwrap();
    let ic = ecbench_solve(&method, &instance, &workload.curve, &workload.target, 7).unwrap();
    assert_eq!(ic.recovered, workload.planted);
    assert!(!ic.detail["verification_performed"].as_bool().unwrap());
    assert!(ic.unpriced.contains(&"row_ops_uncharged".to_string()));

    let rho = resolve(&MethodSpec {
        id: "rho.signed_frobenius".into(),
        params: BTreeMap::new(),
    })
    .unwrap();
    let generic = ecbench_solve(&rho, &instance, &workload.curve, &workload.target, 7).unwrap();
    assert_eq!(generic.recovered, workload.planted);
}

/// The golden file is the normalised `ic --json large-prime` output of the
/// base commit `bbd330a9d`, taken before the orbit quotient existed
/// (`jq -S -c 'del(.manifest) | del(.software.binary_blake3) | .run.report.solve_wall_ns = 0'`;
/// the manifest path, the executable's own hash and the wall time are the only
/// fields that legitimately differ between runs).
#[test]
fn orbit_mode_off_reproduces_the_recorded_fixture_report_byte_for_byte() {
    let manifest =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ecbench_manifest_small.json");
    let golden = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/ecbench_large_prime_n11_default.golden.json"),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ic"))
        .args(["--json", "large-prime", "--manifest"])
        .arg(&manifest)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    report.as_object_mut().unwrap().remove("manifest");
    report["software"]
        .as_object_mut()
        .unwrap()
        .remove("binary_blake3");
    report["run"]["report"]["solve_wall_ns"] = serde_json::json!(0);
    let normalised = serde_json::to_string(&report).unwrap();
    assert_eq!(normalised, golden.trim_end());
    // None of the new fields appears when the quotient is off.
    for field in ["filter_pipeline", "large_prime_graph", "\"orbit\""] {
        assert!(!normalised.contains(field), "unexpected `{field}`");
    }
}

fn filter_config(
    family: &crypto_lib::cryptanalysis::ecbench_large_prime::ImportedFamily,
    graph: LargePrimeGraph,
    max_large_primes: u8,
) -> SolveConfig {
    SolveConfig {
        small_dimension: family.small_dimension,
        envelope_dimension: family.envelope_dimension,
        summands: family.instance.n - 1,
        max_large_primes,
        max_trials: 400,
        max_states: 100_000,
        seed: 7,
        large_prime_graph: graph,
    }
}

#[test]
fn filter_and_orbit_arms_recover_the_scalar_and_every_replay_verifies() {
    let family = fixture();
    for graph in [
        LargePrimeGraph::Filter,
        LargePrimeGraph::FilterOrbit,
        LargePrimeGraph::FilterOrbitAbsorb,
    ] {
        let report = solve(
            &family.instance,
            family.target,
            filter_config(&family, graph, 2),
        )
        .unwrap();
        assert_eq!(report.recovered, Some(family.expected), "{graph:?}");
        assert!(
            report.verified_fast && report.verified_independent,
            "{graph:?}"
        );
        let pipeline = report.filter_pipeline.as_ref().unwrap();
        assert!(pipeline.complete_relations > 0, "{graph:?}");
        assert_eq!(
            pipeline.replays_verified, pipeline.complete_relations,
            "{graph:?}"
        );
        assert_eq!(pipeline.replays_failed, 0, "{graph:?}");
        assert_eq!(
            pipeline.replay_failed_linear + pipeline.replay_failed_curve,
            0
        );
        assert_eq!(pipeline.partial_audit_failed, 0, "{graph:?}");
        assert_eq!(
            pipeline.independent_verified + pipeline.dependent_verified,
            pipeline.replays_verified
        );
        assert_eq!(pipeline.rank, report.counters.matrix_rank);
        let orbit = report.factor_base.orbit.as_ref();
        assert_eq!(
            orbit.is_some(),
            graph != LargePrimeGraph::Filter,
            "{graph:?}"
        );
        if let Some(orbit) = orbit {
            assert_eq!(
                orbit.large_points,
                orbit.large_orbits + orbit.merged_points + orbit.absorbed_into_small
            );
            assert!(orbit.absorb_into_small || orbit.absorbed_into_small == 0);
        }
    }
}

#[test]
fn with_no_large_primes_the_filter_arm_only_sees_full_relations() {
    let family = fixture();
    let report = solve(
        &family.instance,
        family.target,
        filter_config(&family, LargePrimeGraph::Filter, 0),
    )
    .unwrap();
    let pipeline = report.filter_pipeline.as_ref().unwrap();
    assert_eq!(
        pipeline.partials_by_class[1] + pipeline.partials_by_class[2],
        0
    );
    assert_eq!(pipeline.replays_failed, 0);
    assert_eq!(pipeline.partial_audit_failed, 0);
}
