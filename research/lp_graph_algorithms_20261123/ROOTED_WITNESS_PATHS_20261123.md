# Graph-IC: direct-parent paths for exact two-LP witnesses

Date: 2026-11-23. Owner lane: [PR #4988](https://github.com/isogeny-labs/crypto/pull/4988). Evidence status: **development implementation; native CI pending at this checkpoint**. [pcc:unverified]

## Problem and control

The reference forest/anchor witness extractor in `large_prime_graph/witness.rs` previously ran a fresh BFS over the independent equation forest whenever reducing a row endpoint or reprojecting an anchor. It allocated `prev` with one slot per LP column per query, and scanned adjacency entries even when the unique forest path was short. The exact source coefficients and field inversions, not only graph connectivity, must survive any optimization.

The existing unchanged pivot and MST large-prime filters remain cost and correctness controls; this change affects only the *reference witness-extractor path lookup*, not relation collection or downstream sparse LA. Authored 450 dense-rank, 160 reconstruction and 96 same-stream projection tests remain relevant controls. Do not compare a graph metadata counter to an end-to-end runtime ratio.

## Implementation: union by size plus oriented equation-tree paths

The extractor now stores two independent parent structures:

- Component-disjoint-set parent/size: used solely for component roots and union-by-size, as before.
- `tree_parent[v]` and `tree_parent_edge[v]`: a directed path of *independent LP equation edges* from each vertex to the current component root.

Upon admitting an independent two-LP bridge between distinct components, choose the larger component as the permanent root. Reverse the existing `tree_parent` path from the attachment endpoint of the *smaller* component to its old root. Attach that endpoint beneath the bridge endpoint in the larger component. A projection then walks the exact `tree_parent` path, reading each edge's original row and applying the same modular coefficient elimination as before. Anchors and source maps are unchanged.

Invariants (for exact field `F_ell`): each directed parent edge is an admitted independent two-LP row; each non-root vertex has one such edge; the unique path reaches the same root as the component DSU. Reversing the parent path only changes orientation, not the underlying undirected equation forest. Attaching a smaller component adds the one new accepted bridge and retains the original row identity. Thus row-space rank, source-supported witness coefficients, and replay semantics are unchanged **if the implementation satisfies the invariants**; independent original-source tests are required to validate this reasoning. All input and modular inversion failure checks precede the mutation.

## Bound and accounting

Let `V` be LP columns, `R` input rows, `D` maximum oriented equation-tree depth, `H` total parent hops traversed in all projections, `F` total parent edges reversed on joins, and `L` materialized source terms. Direct parent lookups take `O(H)` pointer steps and `O(D)` temporary path storage, instead of BFS scanning a component and allocating `O(V)` predecessor slots on each lookup. Each vertex is reversed only when its entire component is the smaller one, so its component at least doubles: `F=O(V log V)` over the whole forest construction. Thus the dynamic graph path-maintenance cost is `O(H + V log V)`, plus existing map operations and inversions, with `H <= O(RD)` under the bounded two-endpoint/one-anchor-projection structure. Worst-case `D=O(V)`, so this does **not** prove a better ECDLP exponent, nor does it remove explicit source combination costs.

The existing legacy `adjacency_scans` counter is retained for old JSON compatibility and is zero in this arm; `path_steps` counts actual elimination hops, and the new `reroot_steps` counts pointer reversals. `reroot_steps` has a Serde default for backward-compatible reports. Total cost must also count map updates, inversions, factor-base output support, replay, memory, and rejected candidates.

## Falsifiable correctness/control gate

A new Rust test `forest_witness_parent_paths_survive_small_component_reroots` constructs two 2-vertex equation trees and joins them through their *non-root* endpoints, forcing one path reversal. Over `F_3,F_7,F_101,F_1000003`, it checks the exact rank, three dependent-row positions (balanced cycle, anchored singleton, zero LP row), cancellation of all LP columns, materialized source/payload constraints, and the path counters. The already authored randomized dense-rank, replay, source-cap, and pivot-comparison tests must still pass. This is a synthetic correctness gate, not an authentic Prime31 throughput result.

A next authorized measurement should replay exactly the same natural two-LP relation stream through the previous BFS extractor (frozen revision before `6d1f19e`) and this parent-path extractor (commit `4a11cd93926b65418d68575f74809a849347c9ce`, unless superseded), checking equal full `(small,rhs,sources)` outputs before recording runtime, peak RSS, path-length distributions and row-refusal rates. Include the original pivot/MST arms, downstream useful rank and full cold solve controls. Do not launch competing builds during timing.

## Sources and limits

- Current [graph catalog](https://github.com/isogeny-labs/crypto/blob/main/docs/ic/graph-algorithms/README.md), merged [#4917](https://github.com/isogeny-labs/crypto/pull/4917), existing [forest witness design](FOREST_WITNESS_DESIGN_20261123.md), and overlapping [#4968](https://github.com/isogeny-labs/crypto/pull/4968) inform ownership and controls.
- Classic union-by-size doubling analysis, e.g. [Sedgewick/Wayne-style explanation](https://algorithm.tamnd.com/union-by-size/). The `O(V log V)` bound here applies to *reversed vertices*, not the depth of the actual undirected equation forest.
- Prior [cryptanalysis k=3 graph results](https://github.com/isogeny-labs/cryptanalysis/blob/f9c7b49e770212c24e29ca5c87e202e2aa6e8493/docs/ic/gaudry-gain-forest-harvester-20261122/RESULTS.md) describe a different field/graph family; no performance transfer is assumed.
- Sources checked (KB): dedicated research-KB and conductor functions were not present in this runtime. The wrapper was unavailable without a checkout. Used the merged catalog, exact live GitHub Rust source and tests, the sibling research receipt and referenced primary algorithm analysis instead. No novelty or completeness claim.
- Runtime: no local `rustc`, `cargo`, `gh` or `conductor`; GitHub's source-exact Actions workflow is the bounded native gate. A passing wrapper does not establish a root-crate build or independent PCC.


## Executed source-exact native gate — 2026-11-23 08:47 UTC

[GitHub Actions run **45141218801**](https://github.com/isogeny-labs/crypto/actions/runs/45141218801), job 137579407581, **SUCCESS**, original-source Rust integration harness **23/23 passed**, zero failures, release profile, Rust 1.100.0. The job used pull-request merge checkout `f3a5540cbb` of source head `4a11cd93926b65418d68575f74809a849347c9ce` into base `816f9c3a942efed569049618c056d12a0c56d693`. Source SHA256: `src/cryptanalysis/large_prime_graph/witness.rs=a4e88bbc9adcbaab694fd0ffb5e1b2e19dc8bbf84e96d76071c7da02bdbf3f35`; `tests/large_prime_graph.rs=a498e261785aaaf710a6a36e5f311a0ee2f8a42ac2619ac3418898692d4af9d7`. The new `forest_witness_parent_paths_survive_small_component_reroots` and all existing rank/replay and pivot-projection tests passed. The later doc/session-log commits changed neither production module nor test blob.

This receipt verifies compiled source-exact *module-level* correctness on synthetic inputs and independent Rust helper replay; it does not verify the unshimmed root crate, a full authentic Prime31 collector stream, same-stream comparative running times, a PCC-signed proof or complete solver performance. [pcc:unverified]
