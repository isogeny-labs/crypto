# Ordinary Prime31 capture source and workload gap

## Dated continuation: M31 B677 capture, 2026-11-24 UTC

The bounded search below is a historical result. A different retained source was subsequently found in PR #5097 at `577d6c2c1c4a527604d965e6fb62c44df7902f36`: a ten-query, 645-row generic exhaustive-pair capture on this exact M31 curve. Native run 45309117216 at `3ba271b1f7100bdfd06eafbac8af283ec9d55094` replayed every equation and passed the 613 rows with at most two LP terms through the original admission and graph consumers. The source uses a 915,304-entry signed pair table, so the old h<=32 workload bound below does not describe this capture. [pcc:unverified]

The [new frozen receipt](evidence/native-m31-capture-20261124/README.md), [protocol](M31_CAPTURE_REPLAY_20261124.md) and [report](reports/run-20261124T0200Z/REPORT.md) preserve this distinction. Unsigned data registration: PCC-20261124-fcc0a3. The generic/algebraic track distinction and original collector custody/cold-cost gaps remain open. No new collection or timing comparison was performed.

## Historical bounded source search

Status: source inspection and a derived uniform-query bound, not an executed collection. [pcc:unverified]

## Scope and source identity

The fixed admission curve has p=2147483647, a=847514846, b=2087444724, subgroup order ell=2147446831 and G=(625037715,2100627251). Keep public target Q and exact collector/attempt identities with every stream.

A bounded search across crypto, crypto-autoresearcher and cryptanalysis found a reusable producer, not a qualifying fixed-curve capture:

- [Generic large-prime producer](https://github.com/isogeny-labs/crypto/blob/5403e9ab890e9b9dc92fa67c661ab7df0ea3c860/src/ml_dataset/relations_generic.rs), especially Limits::new, pool, pairs, make_row, query_parameters and TranscriptV2.
- [Dispatch](https://github.com/isogeny-labs/crypto/blob/5403e9ab890e9b9dc92fa67c661ab7df0ea3c860/src/ml_dataset/relations.rs).
- [Earlier source map](https://github.com/isogeny-labs/crypto/blob/5403e9ab890e9b9dc92fa67c661ab7df0ea3c860/research/ml_dataset_large_prime_trace_20261028/SOURCE_MAP.md).
- The merged #5029 lp_graph_collect hook uses binary/Koblitz curves; its output is not an ordinary Prime31 capture.

This producer uses a residual pair table and therefore belongs to the **generic-decomposition** track. It does not supply the separately owned nonenumerative algebraic relation extractor. No new collector, source cap change or collection run is included here.

## Exact adapter mapping

The v2 transcript schema is ml-ecdlp-generic-large-prime-transcript/v2. Before conversion, replay its existing native verifier and require the exact fixed public curve/G/Q and source binding.

| Admission field | Existing transcript source |
|---|---|
| target | public_instance.target |
| small_points | Enumerated small.columns with canonical oriented points |
| large_points | Enumerated residual.domain.columns with canonical oriented points |
| row id | Accepted attempt.sequence |
| row a | attempt.coef_a |
| row b | attempt.coef_b |
| small terms | attempt.row.small, after checking and removing its existing target column |
| large terms | attempt.row.large |

The existing target-column index is small.columns.len(), carrying -b modulo ell. Require that coefficient to agree with the public b before removing it. The admission adapter then writes its own protected u32::MAX column. Leaving both columns in the input would count the unknown target twice.

Preserve the original complete transcript, every rejected attempt, source/config bindings, pair-table setup, native filter certificates and counters as a hashed custody sidecar. An admitted JSONL file verifies group equations; it does not authenticate a collection process by itself.

## Derived workload obstruction

Limits::new caps public queries at 4096 and residual columns at 32. With h <= 32 negation-orbit columns, there are 2h signed nonidentity points and at most (2h)(2h+1)/2 - h = 2h^2 <= 2048 nonidentity unordered pair entries. The h opposite-sign pairs sum to the identity and are excluded from the lookup; duplicate sums only reduce its distinct support.

In the ideal uniform-query model corresponding to independently uniform a in F_ell (and independently selected b/small indices), aG+bQ minus the selected small summands is uniform on the prime-order group. Thus a lookup support of size M accepts with probability M/ell, and

    E[accepted rows] <= 4096 * 2048 / 2147446831
                     = 8388608 / 2147446831
                     ~= 0.003906317.

This is a derived expectation under the stated randomness model, not a measured result or a deterministic guarantee for a fixed PRNG seed. It is already an upper bound before rejecting duplicate or rank-useless rows. With the same support size, even an expectation of 256 raw rows would require at least ell/8 ~= 268.4 million uniform queries; this is a scale estimate, not a proposed or authorized run.

The current capped producer therefore is not a credible way to supply the planned 256/1024/4096-row checkpoints simply by executing it unchanged. This is a workload/capture gap, not a mathematical nonexistence claim about Prime31 relations.

## Next discriminating work

1. Reuse the existing transcript and native replay rather than inventing another collector. Implement a narrow conversion with exact target-column handling and immutable custody when an eligible transcript exists.
2. Obtain an explicit capture protocol from the owning collector/extractor lane whose support size, expected/raw/useful yield, memory and query budget can furnish the checkpoints. Preserve the generic/algebraic track labels; do not silently raise these dataset-generation caps.
3. Only after source and experiment admission, run the existing bounded identical-stream comparison with inclusive materialization, source replay, exact rank, peak memory and full cold-recovery accounting. Record incomplete rank and refusal without substituting a planted control.

No natural capture, timing, RSS or cold-solving outcome was produced in this continuation. The native qualification's known-answer fixtures are solely controls for admission and wire compatibility.
