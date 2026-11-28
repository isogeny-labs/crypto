//! Current-root matched checker integration; operation diagnostics, never timings.
use certified_relations::workload;
use crypto_lib::cryptanalysis::f4_fp::{
    certified::{self, NativeCertification, ReplayForm, ReplayKernels},
    Poly,
};
use std::{fs, io::Write, path::Path};
fn artifact(root: &Path, name: &str, bytes: &[u8]) {
    assert!(bytes.len() <= 16 * 1024 * 1024);
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(root.join(name))
        .unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();
}
#[test]
fn pair_graph_preserves_every_wire_basis_and_public_receipt_obligation() {
    let output = std::env::var_os("RELATION_PAIRGRAPH_OUT").map(std::path::PathBuf::from);
    if let Some(root) = &output {
        fs::create_dir(root).unwrap();
    }
    let mut outcomes = vec![];
    let mut discharged_without_division = 0;
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
            for (form, name) in [
                (ReplayForm::CompactSparse, "compact-sparse"),
                (ReplayForm::CompactDag, "compact-dag"),
                (ReplayForm::PackedSparse, "packed-sparse"),
                (ReplayForm::PackedDag, "packed-dag"),
                (ReplayForm::FlatSparse, "flat-sparse"),
                (ReplayForm::FlatDag, "flat-dag"),
            ] {
                for (kernel, kernel_name) in [
                    (ReplayKernels::BothUpdatesFrontier, "frontier103"),
                    (ReplayKernels::FrontierPackedProducts, "products231"),
                    (ReplayKernels::FrontierPackedBatch, "batch487"),
                ] {
                    let baseline =
                        certified::try_packed_replay_basis(&input, n, p, form, kernel).unwrap();
                    let mut graph = certified::try_packed_replay_basis_with_native_check(
                        &input,
                        n,
                        p,
                        form,
                        kernel,
                        NativeCertification::PairGraph,
                    )
                    .unwrap();
                    assert_eq!(baseline.basis, graph.basis);
                    assert_eq!(baseline.original_json, graph.original_json);
                    assert_eq!(baseline.witness_json, graph.witness_json);
                    assert_eq!(
                        serde_json::to_value(&baseline.replay_stats).unwrap(),
                        serde_json::to_value(&graph.replay_stats).unwrap()
                    );
                    assert_eq!(baseline.observed_solving_degree, None);
                    assert_eq!(graph.observed_solving_degree, None);
                    assert!(baseline.native_checks.pair_graph.is_none());
                    let receipt = graph.native_checks.pair_graph.as_ref().unwrap();
                    assert_eq!(receipt.resource_refusals, 0);
                    assert_eq!(
                        receipt.product_edges + receipt.chain_edges + receipt.direct_edges,
                        graph.native_checks.certification_pairs
                    );
                    assert_eq!(
                        graph.native_checks.certification_pairs,
                        baseline.native_checks.certification_pairs
                    );
                    discharged_without_division += receipt.product_edges + receipt.chain_edges;
                    assert!(certified::verify_packed_replay_basis(&baseline, &input));
                    assert!(certified::verify_packed_replay_basis(&graph, &input));
                    graph.basis[0][0].1 = (graph.basis[0][0].1 + 1) % p;
                    assert!(!certified::verify_packed_replay_basis(&graph, &input));
                    graph.basis[0][0].1 = (graph.basis[0][0].1 + p - 1) % p;
                    graph.replay_stats.cofactor_terms += 1;
                    assert!(!certified::verify_packed_replay_basis(&graph, &input));
                    graph.replay_stats.cofactor_terms -= 1;
                    graph.native_checks.certification_pairs += 1;
                    assert!(!certified::verify_packed_replay_basis(&graph, &input));
                    graph.native_checks.certification_pairs -= 1;
                    graph.native_checks.pair_graph.as_mut().unwrap().chain_edges += 1;
                    assert!(!certified::verify_packed_replay_basis(&graph, &input));
                    graph.native_checks.pair_graph.as_mut().unwrap().chain_edges -= 1;
                    graph.observed_solving_degree = Some(0);
                    assert!(!certified::verify_packed_replay_basis(&graph, &input));
                    graph.observed_solving_degree = None;
                    let byte = graph.original_json[0];
                    graph.original_json[0] = b'!';
                    assert!(!certified::verify_packed_replay_basis(&graph, &input));
                    graph.original_json[0] = byte;
                    let byte = graph.witness_json[0];
                    graph.witness_json[0] = b'!';
                    assert!(!certified::verify_packed_replay_basis(&graph, &input));
                    graph.witness_json[0] = byte;
                    let mut changed = input.clone();
                    changed[0][0].1 = (changed[0][0].1 + 1) % p;
                    assert!(!certified::verify_packed_replay_basis(&graph, &changed));
                    assert!(certified::verify_packed_replay_basis(&graph, &input));
                    let outcome = serde_json::json!({"prime":p,"n":n,"seed":7,"graph":"LAST-star","form":name,"kernel":kernel_name,"exhaustive":baseline.native_checks,"pair_graph":graph.native_checks,"same_original_proof_basis_and_replay_stats":true,"mutations_rejected":8,"observed_solving_degree":null,"timing_claim":false,"independent_signed_verdict":false});
                    if let Some(root) = &output {
                        let case = root.join(format!("p{p}-n{n}-{name}-{kernel_name}"));
                        fs::create_dir(&case).unwrap();
                        artifact(&case, "original.json", &graph.original_json);
                        artifact(&case, "proof.json", &graph.witness_json);
                        artifact(
                            &case,
                            "basis.json",
                            &serde_json::to_vec(&graph.basis).unwrap(),
                        );
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
    assert_eq!(outcomes.len(), 108);
    assert!(discharged_without_division > 0);
    if let Some(root) = &output {
        artifact(root,"matrix.json",&serde_json::to_vec_pretty(&serde_json::json!({"schema":"relation-pairgraph-correctness/v1","paired_cases":108,"constructions":216,"mutations_rejected":864,"cases":outcomes,"timing_claim":false,"15x_admitted":false})).unwrap());
    }
}
#[test]
fn pair_graph_preserves_unsupported_input_refusals() {
    for kind in [
        NativeCertification::Exhaustive,
        NativeCertification::PairGraph,
    ] {
        assert!(certified::try_packed_replay_basis_with_native_check(
            &[],
            3,
            2147483647,
            ReplayForm::FlatSparse,
            ReplayKernels::FrontierPackedBatch,
            kind
        )
        .is_err());
        assert!(certified::try_packed_replay_basis_with_native_check(
            &[vec![(vec![0, 0, 0], 1)]],
            3,
            101,
            ReplayForm::FlatDag,
            ReplayKernels::FrontierPackedBatch,
            kind
        )
        .is_err());
    }
}
