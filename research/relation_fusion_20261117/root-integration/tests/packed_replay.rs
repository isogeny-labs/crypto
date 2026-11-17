//! Actual crypto_lib adapter, retaining independent replay and native checks.
use certified::{ReplayForm, ReplayKernels};
use certified_relations::workload;
use crypto_lib::cryptanalysis::f4_fp::{certified, Poly};
#[test]
fn actual_root_packed_replay_matches_basis_and_rejects_public_mutations() {
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
            for form in [ReplayForm::CompactSparse, ReplayForm::CompactDag] {
                for kernel in [
                    ReplayKernels::BothUpdatesFrontier,
                    ReplayKernels::FrontierPackedProducts,
                    ReplayKernels::FrontierPackedBatch,
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
                }
            }
        }
    }
}
#[test]
fn actual_root_packed_replay_refuses_unsupported_inputs() {
    assert!(certified::try_packed_replay_basis(
        &[],
        3,
        2147483647,
        ReplayForm::CompactSparse,
        ReplayKernels::FrontierPackedBatch
    )
    .is_err());
    assert!(certified::try_packed_replay_basis(
        &[vec![(vec![0, 0, 0], 1)]],
        3,
        101,
        ReplayForm::CompactDag,
        ReplayKernels::FrontierPackedBatch
    )
    .is_err());
}
