# Root-normalized anchor reuse for coefficient-aware LP witness extraction

Date: 2026-11-23 UTC. Owner: [graph-IC draft PR #4988](https://github.com/isogeny-labs/crypto/pull/4988). Status: **Rust source and a new regression submitted; new native qualification pending**. [pcc:unverified]

## Mechanism and exact invariant

The accepted two-large-prime matrix is over the **prime subgroup field** `F_ell`, distinct from the elliptic-curve coordinate field `F_p`. An independent equation forest expresses each vertex as a field multiple of its component root, with an exact combination of original source rows. When an additional independent equation pins the component, the stored anchor row `a` has a reduction

`a + sum_i beta_i * tree_row_i = alpha * x_root`, with `alpha != 0`.

A subsequent row reducing to `gamma * x_root` is canceled by adding
`-(gamma / alpha) * (a + sum_i beta_i * tree_row_i)`.
The pair `(alpha^{-1}, {beta_i})` is invariant **while the directed equation forest and component root are unchanged**, so recomputing the anchor tree path and modular inverse for every dependent row is unnecessary.

The new `ForestWitnessExtractor` caches the root-normalized anchor source map and inverse on first use. It borrows the map on subsequent uses rather than cloning it. A successful independent bridge invalidates both affected root slots; pinning a new root also invalidates that slot. All pending cache writes, counters, row metadata and forest changes are committed **after** coefficient computation succeeds. A rejected row leaves the observable graph/witness counters and prior cache unchanged. Output is still reconstructed from original rows, with the legacy source replay as a separate check.

This applies only to the canonical, invertible-coefficient, arity-at-most-two field case. A non-prime odd modulus accepted by the legacy constructor is **not** a field assumption; existing nonunit inverse rejection is retained.

## Accounting, not a whole-solver performance claim

Let `U` be accepted nonempty rows, `A` the total number of *uses* of an anchored root while projecting new rows, and `C` the number of anchor-cache misses. The previous accepted-stream modular inverse count was `U + A`; the new expected count is `U + C`. Each cached use avoids one root-anchor path projection, one inverse and one cloned source map. The input-row inversions and its own root projections remain unchanged. Each forest join may invalidate an anchor cache, so `C <= A` and `C` is bounded by the number of cache epochs actually used. Since there are at most `V-1` forest joins and at most `V` newly introduced anchors, `C = O(V)` over an insertion-only stream (no removals), while `A` can grow with the number of dependent rows. This argument counts successful rows only; rejected-row diagnostics remain separately chargeable.

A cache contains a sparse map of original tree-edge coefficients; the additional cache storage is proportional to the sum of those cached supports, not merely `O(V)` in all cases. Worst-case quadratic storage in `V` is possible for many long anchored components, and output expansion may still dominate. The `Cow::Borrowed` branch avoids copying the cached map per use; applying it still costs at least its support size via field multiplications and ordered-map insertions. An anchor with no subsequent use saves nothing. This is a component-level optimization, **not** a new ECDLP exponent or measured wall-time improvement. [pcc:unverified]

## Registered bounded correctness test

New source-exact native Rust test `forest_witness_reuses_root_anchor_and_invalidates_after_join` runs over `ell = 3,7,101,1000003`:

- Two tree edges plus one unbalanced closing row pin a three-vertex component; 48 repeated closing equations must emit identical exact LP cancellation certificates.
- Before a join, expect 1 anchor-cache miss, 47 cache hits and **52 inversions** in total, against 99 for the previous uncached algorithm (one input-row inverse in both arms).
- Joining an unpinned fourth vertex hits the warm cache once but invalidates it; the next dependent singleton forces one recomputation. Expect 2 misses, 48 hits and **55 total inversions**.
- Check all source coefficients against the independently built `LpIndex`, dense `F_ell` rank, legacy `replay_complete_relation` and atomic refusal after warming the cache.
- Existing 450 seeded dense-rank checks, 160 independent source-replay cases and synthetic pivot comparisons stay in the same source-exact test workflow. They do not establish authentic Prime31 relation admission or full-crate compilation.

Falsify this change on any mismatch of exact coefficients, rank, source IDs, RHS, nonunit refusal, or if native tests do not compile. Maintain failures as failures; never infer a runtime speedup from an operation-counter reduction. The test uses only the existing repository source modules and GitHub-hosted Rust; no paid compute.

## Evidence, literature and ownership

- [Merged graph algorithm catalog #4917](https://github.com/isogeny-labs/crypto/pull/4917), especially the coefficient-aware rank/output accounting, and `docs/ic/graph-algorithms/{README,SOURCES}.md`.
- [Merged gain-forest integration #4905](https://github.com/isogeny-labs/crypto/pull/4905), separate pipeline owner.
- [Source-bound forest/anchor design](./FOREST_WITNESS_DESIGN_20261123.md), [rooted path proof](./ROOTED_WITNESS_PATHS_20261123.md), [edge cache](./EDGE_INVERSE_CACHE_20261123.md), [paired inverse](./PAIRED_INVERSE_20261123.md).
- The preceding [source-exact 25-test workflow 45158286356](https://github.com/isogeny-labs/crypto/actions/runs/45158286356) **passed all 25 tests** at source `1e332f06` (old paired-inversion implementation), in Rust 1.100.0. This receipt does **not** qualify the new anchor-cache change.
- **Sources checked (KB):** dedicated research-KB MCP and repository wrapper unavailable in the connector-only environment. Scoped primary fallback: current GitHub modules and notes above, plus code-search attempts in `crypto`, `cryptanalysis`, `crypto-autoresearcher` for a natural Prime31 two-LP source stream; no eligible stream established. This is a limited retrieval result, not an absence proof.

## Required next execution

1. Inspect the new native source-exact workflow at the final code/test head; read job logs, correct any failures and retain previous failed receipts.
2. Only after passing, freeze a source/group-verified, naturally collected `p=2^31-1` two-LP stream with the actual subgroup order `ell`, generator, curve, target, per-row source witnesses and independent field checks. Do not substitute the synthetic 4-prime test.
3. On identical normalized input, compare old rooted-parent `21418acd`, prior cached-edge `0dc8dc8`, paired-inverse `1e332f06`, new anchor cache, pivot and MST; separately charge setup, rejected rows, anchor-map work, inverse/multiply counts, source expansion/fill, output rank, memory and phase wall time under matched resources.
4. Verify all exact sources and final target logarithms `[x]G=Q`, then compare full cold solver costs against the strongest eligible index-calculus control and optimized negation-map rho. Require independent PCC admission before promoting any performance result.

No merge, auto-merge, EKS/paid compute, new leaderboard row, or external validation is authorized or claimed here.
