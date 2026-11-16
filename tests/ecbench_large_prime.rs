use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use crypto_lib::cryptanalysis::ecbench::methods::{resolve, solve as ecbench_solve, MethodSpec};
use crypto_lib::cryptanalysis::ecbench::workload::{CurveSpec, Workload};
use crypto_lib::cryptanalysis::ecbench_large_prime::orbit::LambdaPowers;
use crypto_lib::cryptanalysis::ecbench_large_prime::{
    import_family, load_manifest, load_manifest_with_source, solve, solve_with_options,
    FilterPipelineReport, LargePrimeGraph, LargePrimeReport, SolveConfig, SolveOptions, TableFault,
    TrajectoryRequest,
};
use crypto_lib::cryptanalysis::ic_boundary::{koblitz_instance, BinaryInstance};
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;

#[path = "common/orbit_support.rs"]
mod orbit_support;
use orbit_support::{brute_force_counts, classify_merges, oracle_base, SplitMix};

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
/// (`jq -S -c 'del(.manifest) | del(.software) | .run.report.solve_wall_ns = 0'`).
///
/// The fields removed are exactly the ones that describe the environment and
/// not the report: the manifest *path*, the whole `software` object (crate
/// version, OS, architecture, the executable's own hash and, from compile-time
/// `CRYPTO_BUILD_GIT_COMMIT` / `GITHUB_SHA` / `CRYPTO_BUILD_GIT_DIRTY`, the
/// source commit, its source and the dirty flag) and the wall time.  An
/// earlier version of this test removed only `software.binary_blake3` and
/// recorded `git_commit: null`, so it failed on any build that had
/// `GITHUB_SHA` set (every GitHub Actions run) or `CRYPTO_BUILD_GIT_COMMIT`.
/// What is compared is everything else: the battery, the manifest identity and
/// the whole solver report.
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
    {
        let object = report.as_object_mut().unwrap();
        assert!(
            object.contains_key("software"),
            "the report is expected to carry a `software` object, which is dropped below"
        );
        object.remove("manifest");
        object.remove("software");
    }
    report["run"]["report"]["solve_wall_ns"] = serde_json::json!(0);
    let normalised = serde_json::to_string(&report).unwrap();
    assert_eq!(normalised, golden.trim_end());
    // None of the new fields appears when the quotient is off.
    for field in [
        "filter_pipeline",
        "large_prime_graph",
        "\"orbit\"",
        "\"stream\"",
        "post_collect",
        "trajectory",
    ] {
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
        LargePrimeGraph::FilterOrbitPostCollect,
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

// ---------------------------------------------------------------------------
// Tables with real merges and absorptions.
//
// The n = 11 fixture above has 5 small and 4 large columns, no merged large
// point and nothing to absorb, so the identity `large_points == orbits +
// merged + absorbed` holds there by construction and the absorb inversion
// `f_Q * f_S^-1` is never exercised.  The tests below use a Koblitz curve and
// parameters that provably produce both (the n = 17 `K_1` cell of the
// orbit-aware large-prime measurement, `research/orbit_aware_large_prime_20261011`),
// and compare the library's table with `orbit_support`'s independent
// brute-force recount.

/// `K_a` over `F_{2^n}`, the factor-base window and the collector parameters.
const MERGE_A: u8 = 1;
const MERGE_N: u32 = 17;
const MERGE_SMALL: u32 = 5;
const MERGE_ENV: u32 = 10;
const MERGE_SUMMANDS: u32 = 3;
const MERGE_CAP: u64 = 200;
const MERGE_SEED: u64 = 1;
/// Absorbed large point / multi-member orbit that the stream of `MERGE_SEED`
/// actually uses: found by running the clean arm and damaging one entry at a
/// time (19 absorbed points, 61 multi-member orbits); damaging these must be
/// noticed by the curve replay (and the audit), while most other entries are
/// not used by this short stream at all.
const USED_ABSORB_INDEX: usize = 3;
const USED_ORBIT_INDEX: usize = 38;

struct MergeCase {
    instance: BinaryInstance,
    target: FastPoint,
    planted: u64,
}

fn merge_case() -> MergeCase {
    let instance = koblitz_instance(MERGE_A, MERGE_N).expect("Koblitz instance");
    let mut rng = SplitMix(0x6f72_6269_745f_6c70 ^ u64::from(MERGE_N));
    let planted = 1 + rng.next() % (instance.r - 1);
    let target = instance.fast.mul_u64(instance.generator, planted);
    MergeCase {
        instance,
        target,
        planted,
    }
}

fn merge_config(graph: LargePrimeGraph) -> SolveConfig {
    SolveConfig {
        small_dimension: MERGE_SMALL,
        envelope_dimension: MERGE_ENV,
        summands: MERGE_SUMMANDS,
        max_large_primes: 2,
        max_trials: MERGE_CAP,
        max_states: 3_000_000,
        seed: MERGE_SEED,
        large_prime_graph: graph,
    }
}

fn run_merge_case(
    case: &MergeCase,
    graph: LargePrimeGraph,
    options: SolveOptions,
) -> LargePrimeReport {
    solve_with_options(&case.instance, case.target, merge_config(graph), options)
        .unwrap_or_else(|e| panic!("{graph:?}: {e}"))
}

fn pipeline(report: &LargePrimeReport) -> &FilterPipelineReport {
    report.filter_pipeline.as_ref().expect("filter pipeline")
}

/// Every complete relation passed both replays and every input row holds as a
/// group identity.
fn assert_all_replays_pass(report: &LargePrimeReport, label: &str) {
    let p = pipeline(report);
    assert!(p.complete_relations > 0, "{label}: no complete relation");
    assert_eq!(p.replays_verified, p.complete_relations, "{label}");
    assert_eq!(p.replays_failed, 0, "{label}");
    assert_eq!(p.replay_failed_linear, 0, "{label}");
    assert_eq!(p.replay_failed_curve, 0, "{label}");
    assert_eq!(p.partial_audit_failed, 0, "{label}");
}

#[test]
fn orbit_tables_with_real_merges_and_absorptions_match_an_independent_recount() {
    let case = merge_case();
    let oracle = oracle_base(&case.instance, MERGE_SMALL, MERGE_ENV);
    let counts = brute_force_counts(&case.instance, &oracle);
    // The point of the test: both mechanisms are present in the table.
    assert!(counts.merged > 0, "no merged large point: {counts:?}");
    assert!(counts.absorbed > 0, "no absorbable large point: {counts:?}");
    assert_eq!(
        counts.large_points,
        counts.orbits + counts.merged,
        "oracle bookkeeping"
    );

    // Plain arm: no quotient.
    let plain = run_merge_case(&case, LargePrimeGraph::Filter, SolveOptions::default());
    assert!(plain.factor_base.orbit.is_none());
    assert_eq!(plain.factor_base.large_columns, counts.large_points);
    assert_eq!(plain.factor_base.small_columns, oracle.small.len());

    // Orbit and post-collection arms: the same table.
    for graph in [
        LargePrimeGraph::FilterOrbit,
        LargePrimeGraph::FilterOrbitPostCollect,
    ] {
        let report = run_merge_case(&case, graph, SolveOptions::default());
        let orbit = report.factor_base.orbit.as_ref().unwrap();
        assert_eq!(orbit.large_points, counts.large_points, "{graph:?}");
        assert_eq!(orbit.large_orbits, counts.orbits, "{graph:?}");
        assert_eq!(orbit.merged_points, counts.merged, "{graph:?}");
        assert_eq!(orbit.absorbed_into_small, 0, "{graph:?}");
        assert_eq!(
            report.factor_base.large_columns, counts.orbits,
            "{graph:?}: the large columns are the orbits"
        );
        assert_all_replays_pass(&report, &format!("{graph:?}"));
    }

    // Absorb arm: large points whose orbit contains a small point become
    // small columns, and the remaining large points are re-grouped.
    let absorb = run_merge_case(
        &case,
        LargePrimeGraph::FilterOrbitAbsorb,
        SolveOptions::default(),
    );
    let orbit = absorb.factor_base.orbit.as_ref().unwrap();
    assert_eq!(orbit.large_points, counts.large_points);
    assert_eq!(orbit.absorbed_into_small, counts.absorbed);
    assert_eq!(orbit.large_orbits, counts.orbits_after_absorb);
    assert_eq!(orbit.merged_points, counts.merged_after_absorb);
    assert_eq!(
        orbit.large_points,
        orbit.large_orbits + orbit.merged_points + orbit.absorbed_into_small
    );
    assert_all_replays_pass(&absorb, "absorb");
    // The absorbed points are used: some partial relation that has large
    // primes in the orbit arm has none in the absorb arm.
    let orbit_arm = run_merge_case(&case, LargePrimeGraph::FilterOrbit, SolveOptions::default());
    assert_ne!(
        pipeline(&absorb).partials_by_class,
        pipeline(&orbit_arm).partials_by_class,
        "absorption changed no relation: the absorb arithmetic was not exercised"
    );
    assert!(pipeline(&absorb).partials_by_class[0] > pipeline(&orbit_arm).partials_by_class[0]);

    // Classification of the merges is a partition of them.
    let split = classify_merges(&case.instance, &oracle);
    assert_eq!(split.merged, counts.merged);
    assert_eq!(split.raw_frobenius + split.projected_only, split.merged);
}

#[test]
fn post_collection_arm_sees_the_plain_stream_and_the_quotient_adds_relations_on_it() {
    let case = merge_case();
    let digest = SolveOptions {
        stream_digest: true,
        ..Default::default()
    };
    let plain = run_merge_case(&case, LargePrimeGraph::Filter, digest.clone());
    let post = run_merge_case(
        &case,
        LargePrimeGraph::FilterOrbitPostCollect,
        digest.clone(),
    );
    let (p, q) = (pipeline(&plain), pipeline(&post));

    // Identical accepted decompositions, trial by trial.
    let (ps, qs) = (p.stream.as_ref().unwrap(), q.stream.as_ref().unwrap());
    assert!(ps.accepted > 0);
    assert_eq!(ps.digest, qs.digest);
    assert_eq!(ps.accepted, qs.accepted);
    assert_eq!(plain.counters.trials, post.counters.trials);
    assert_eq!(plain.counters.decompositions, post.counters.decompositions);
    assert_eq!(
        plain.counters.relation_histogram,
        post.counters.relation_histogram
    );
    assert_eq!(p.partials_by_class, q.partials_by_class);
    assert_eq!(plain.relation_samples.len(), post.relation_samples.len());
    for (a, b) in plain.relation_samples.iter().zip(&post.relation_samples) {
        assert_eq!(
            (a.trial, a.a, a.b, &a.raw_terms),
            (b.trial, b.a, b.b, &b.raw_terms)
        );
    }

    // The quotient bookkeeping is a partition of the same stream.
    let remap = q.post_collect.as_ref().unwrap();
    assert_eq!(
        remap.partials_by_orbit_class.iter().sum::<u64>(),
        qs.accepted
    );

    // On that identical stream the quotient can only add relations, and on
    // this cell it does (merged columns close cycles the plain graph cannot).
    assert!(q.complete_relations > p.complete_relations, "{q:?} {p:?}");
    assert!(q.independent_verified >= p.independent_verified);
    assert_all_replays_pass(&plain, "plain");
    assert_all_replays_pass(&post, "post-collection");

    // Orbit-aware collection (arm iii) accepts by orbit-column count, so it
    // is allowed to differ from the plain stream; it never accepts fewer.
    let orbit = run_merge_case(&case, LargePrimeGraph::FilterOrbit, digest);
    assert!(pipeline(&orbit).stream.as_ref().unwrap().accepted >= ps.accepted);
    assert!(plain.counters.decompositions <= orbit.counters.decompositions);
}

#[test]
fn rank_trajectory_reproduces_the_pipeline_and_is_ordered() {
    let case = merge_case();
    let options = SolveOptions {
        trajectory: Some(TrajectoryRequest {
            cutoffs: vec![MERGE_CAP / 2, MERGE_CAP],
        }),
        ..Default::default()
    };
    let report = run_merge_case(&case, LargePrimeGraph::FilterOrbitPostCollect, options);
    let p = pipeline(&report);
    let t = p.trajectory.as_ref().unwrap();
    assert!(t.consistent_with_pipeline);
    assert!(t.monotone);
    assert_eq!(t.stored_partials, report.counters.accepted_relations);
    assert_eq!(t.columns as usize, report.factor_base.small_columns + 1);
    // The prefix at the run's own cap is the run.
    let last = t.at_cutoffs.last().unwrap();
    assert_eq!(last.cutoff_trial, MERGE_CAP);
    assert_eq!(last.rank, p.rank);
    assert_eq!(last.independent, p.independent_verified);
    assert_eq!(last.complete, p.complete_relations);
    // Earlier prefixes do not exceed later ones.
    assert!(t.at_cutoffs[0].rank <= last.rank);
    // First hits are ordered by target rank and never exceed the run.
    let hits: Vec<_> = t
        .first_hits
        .iter()
        .filter(|h| h.target.starts_with("rank>="))
        .collect();
    assert!(hits.len() >= 2);
    let mut previous = 0u64;
    for hit in hits {
        if let (Some(trial), true) = (hit.trial, hit.rank <= p.rank) {
            assert!(trial >= previous, "{hit:?}");
            assert!(trial <= MERGE_CAP, "{hit:?}");
            previous = trial;
        } else {
            assert!(hit.rank > p.rank, "{hit:?}: reachable but not reported");
        }
    }
    // The default call reports none of it.
    let plain = solve(
        &case.instance,
        case.target,
        merge_config(LargePrimeGraph::FilterOrbitPostCollect),
    )
    .unwrap();
    assert!(pipeline(&plain).trajectory.is_none() && pipeline(&plain).stream.is_none());
    // Instrumentation does not change the result.
    assert_eq!(pipeline(&plain).rank, p.rank);
    assert_eq!(pipeline(&plain).complete_relations, p.complete_relations);
    assert_eq!(report.recovered.is_some(), plain.recovered.is_some());
}

/// A wrong absorb factor is invisible to the filter and to the linear replay
/// (they work with the numbers they are given) and is caught by the curve
/// replay and by the audit of the input rows.
#[test]
fn a_corrupted_absorb_factor_is_rejected_by_the_curve_replay() {
    let case = merge_case();
    let clean = run_merge_case(
        &case,
        LargePrimeGraph::FilterOrbitAbsorb,
        SolveOptions::default(),
    );
    assert_all_replays_pass(&clean, "clean absorb");
    assert!(
        clean
            .factor_base
            .orbit
            .as_ref()
            .unwrap()
            .absorbed_into_small
            > 0
    );

    for (label, index) in [
        ("every absorbed factor", None),
        ("one absorbed factor", Some(USED_ABSORB_INDEX)),
    ] {
        let broken = run_merge_case(
            &case,
            LargePrimeGraph::FilterOrbitAbsorb,
            SolveOptions {
                table_fault: Some(TableFault::AbsorbFactor { index }),
                ..Default::default()
            },
        );
        let p = pipeline(&broken);
        assert_eq!(
            p.replay_failed_linear, 0,
            "{label}: the linear replay cannot see a wrong coefficient"
        );
        assert!(
            p.partial_audit_failed > 0,
            "{label}: an input row with the damaged factor must fail its group identity"
        );
        assert!(
            p.replay_failed_curve > 0,
            "{label}: a relation built from it must fail the curve replay"
        );
        assert_eq!(p.replays_failed, p.replay_failed_curve, "{label}");
        assert_eq!(
            p.replays_verified + p.replays_failed,
            p.complete_relations,
            "{label}"
        );
        // Failed rows never reach the rank tracker, so what is recovered is
        // still right.
        if let Some(d) = broken.recovered {
            assert_eq!(d, case.planted, "{label}");
        }
        assert!(
            p.independent_verified + p.dependent_verified == p.replays_verified,
            "{label}"
        );
    }
}

/// The same for a merged orbit member with its Frobenius phase off by one.
#[test]
fn a_wrong_phase_on_a_merged_orbit_member_is_rejected_by_the_curve_replay() {
    let case = merge_case();
    for (label, index) in [
        ("the second member of every merged orbit", None),
        (
            "the second member of one merged orbit",
            Some(USED_ORBIT_INDEX),
        ),
    ] {
        let broken = run_merge_case(
            &case,
            LargePrimeGraph::FilterOrbitPostCollect,
            SolveOptions {
                table_fault: Some(TableFault::OrbitFactor { index }),
                ..Default::default()
            },
        );
        let p = pipeline(&broken);
        assert_eq!(p.replay_failed_linear, 0, "{label}");
        assert!(p.partial_audit_failed > 0, "{label}");
        assert!(p.replay_failed_curve > 0, "{label}");
        if let Some(d) = broken.recovered {
            assert_eq!(d, case.planted, "{label}");
        }
    }
    // A fault needs a table to damage.
    let err = solve_with_options(
        &case.instance,
        case.target,
        merge_config(LargePrimeGraph::Filter),
        SolveOptions {
            table_fault: Some(TableFault::OrbitFactor { index: None }),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert!(err.contains("orbit graph"), "{err}");
}

#[test]
fn orbit_quotient_refuses_subgroup_orders_of_two_to_the_63_and_up() {
    let mut instance = koblitz_instance(MERGE_A, MERGE_N).expect("Koblitz instance");
    assert!(LambdaPowers::new(&instance).is_ok());
    // Not a real group: only the guard on `r` is under test, and it comes
    // before any use of `r`.
    instance.r = (1u64 << 63) + 1;
    let err = LambdaPowers::new(&instance).unwrap_err();
    assert!(err.contains("2^63"), "{err}");
}
