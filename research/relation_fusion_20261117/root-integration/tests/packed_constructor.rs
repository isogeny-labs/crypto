//! Actual crypto_lib adapter, retaining independent replay and native checks.
use certified::{ReplayForm, ReplayKernels};
use certified_relations::workload;
use crypto_lib::cryptanalysis::f4_fp::{certified, Poly};
use std::io::Write;
fn digest(bytes: &[u8]) -> String {
    crypto_lib::hash::sha256::sha256(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn artifact(root: &std::path::Path, name: &str, bytes: &[u8]) {
    assert!(bytes.len() <= 16 * 1024 * 1024, "artifact byte cap");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(name))
        .unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();
}
#[test]
fn actual_root_packed_constructor_matches_basis_and_rejects_public_mutations() {
    // Optional durable qualification evidence; never overwrite an old run.
    let output = std::env::var_os("RELATION_ROOT_INTEGRATION_OUT").map(std::path::PathBuf::from);
    if let Some(root) = &output {
        std::fs::create_dir(root).unwrap();
    }
    let mut outcomes = Vec::new();
    for p in [2147483647, 9007199254740881] {
        for n in [3, 4, 6] {
            let edges: Vec<_> = (0..n - 1).map(|i| (i, n - 1)).collect();
            let system = workload::generate_graph(p, n, 7, &edges).unwrap().system;
            let input: Vec<Poly> = system
                .decode()
                .unwrap()
                .into_iter()
                .map(|f| {
                    f.into_iter()
                        .map(|(m, c)| (m.into_iter().map(|e| e as u32).collect(), c))
                        .collect()
                })
                .collect();
            let mut old = certified::try_compact_quotient_basis(&input, n, p)
                .unwrap()
                .basis;
            old.sort();
            for (form, form_name) in [
                (ReplayForm::PackedSparse, "packed-sparse"),
                (ReplayForm::PackedDag, "packed-dag"),
            ] {
                for (kernel, kernel_name) in [
                    (ReplayKernels::BothUpdatesFrontier, "frontier103"),
                    (ReplayKernels::FrontierPackedProducts, "products231"),
                    (ReplayKernels::FrontierPackedBatch, "batch487"),
                ] {
                    let mut report =
                        certified::try_packed_replay_basis(&input, n, p, form, kernel).unwrap();
                    let mut actual = report.basis.clone();
                    actual.sort();
                    assert_eq!(actual, old);
                    assert!(certified::verify_packed_replay_basis(&report, &input));
                    assert!(report.observed_solving_degree.is_none());
                    assert!(report.native_checks.certification_pairs > 0);
                    report.basis[0][0].1 = (report.basis[0][0].1 + 1) % p;
                    assert!(!certified::verify_packed_replay_basis(&report, &input));
                    report.basis[0][0].1 = (report.basis[0][0].1 + p - 1) % p;
                    report.replay_stats.cofactor_terms += 1;
                    assert!(!certified::verify_packed_replay_basis(&report, &input));
                    report.replay_stats.cofactor_terms -= 1;
                    report.native_checks.certification_pairs += 1;
                    assert!(!certified::verify_packed_replay_basis(&report, &input));
                    report.native_checks.certification_pairs -= 1;
                    report.observed_solving_degree = Some(0);
                    assert!(!certified::verify_packed_replay_basis(&report, &input));
                    report.observed_solving_degree = None;
                    let first = report.original_json[0];
                    report.original_json[0] = b'!';
                    assert!(!certified::verify_packed_replay_basis(&report, &input));
                    report.original_json[0] = first;
                    let first = report.witness_json[0];
                    report.witness_json[0] = b'!';
                    assert!(!certified::verify_packed_replay_basis(&report, &input));
                    report.witness_json[0] = first;
                    let mut changed = input.clone();
                    changed[0][0].1 = (changed[0][0].1 + 1) % p;
                    assert!(!certified::verify_packed_replay_basis(&report, &changed));
                    assert!(certified::verify_packed_replay_basis(&report, &input));
                    let basis = serde_json::to_vec(&report.basis).unwrap();
                    let control = serde_json::to_vec(&old).unwrap();
                    let outcome = serde_json::json!({
                        "schema":"relation-root-integration-case/v1", "prime":p, "n":n, "seed":7, "graph":"LAST-star",
                        "form":form_name, "kernel":kernel_name,
                        "original_sha256":digest(&report.original_json), "proof_sha256":digest(&report.witness_json),
                        "basis_sha256":digest(&basis), "sorted_basis_sha256":digest(&serde_json::to_vec(&actual).unwrap()),
                        "control_basis_sha256":digest(&control), "replay_stats":&report.replay_stats, "native_checks":&report.native_checks,
                        "observed_solving_degree":report.observed_solving_degree,
                        "control_basis_matches":true, "final_replay_accepts":true,
                        "mutations_rejected":["basis_coefficient","replay_stats","native_checks","observed_degree","original_wire","proof_wire","changed_input"],
                        "standing":"unsigned producer test evidence; not execution attestation", "timing_claim":false,
                    });
                    if let Some(root) = &output {
                        let case = root.join(format!("p{p}-n{n}-{form_name}-{kernel_name}"));
                        std::fs::create_dir(&case).unwrap();
                        artifact(&case, "original.json", &report.original_json);
                        artifact(&case, "proof.json", &report.witness_json);
                        artifact(&case, "basis.json", &basis);
                        artifact(&case, "control-basis.json", &control);
                        artifact(
                            &case,
                            "outcome.json",
                            &serde_json::to_vec_pretty(&outcome).unwrap(),
                        );
                    }
                    println!("{outcome}");
                    outcomes.push(outcome);
                }
            }
        }
    }
    assert_eq!(outcomes.len(), 36);
    if let Some(root) = &output {
        artifact(
            root,
            "matrix.json",
            &serde_json::to_vec_pretty(&serde_json::json!({
                "schema":"relation-root-integration-matrix/v1", "cases":outcomes, "case_count":36,
                "timing_claim":false, "full_library_unit_suite_qualified":false,
            }))
            .unwrap(),
        );
    }
}
#[test]
fn actual_root_packed_constructor_refuses_unsupported_inputs() {
    assert!(certified::try_packed_replay_basis(
        &[],
        3,
        2147483647,
        ReplayForm::PackedSparse,
        ReplayKernels::FrontierPackedBatch
    )
    .is_err());
    assert!(certified::try_packed_replay_basis(
        &[vec![(vec![0, 0, 0], 1)]],
        3,
        101,
        ReplayForm::PackedDag,
        ReplayKernels::FrontierPackedBatch
    )
    .is_err());
}
