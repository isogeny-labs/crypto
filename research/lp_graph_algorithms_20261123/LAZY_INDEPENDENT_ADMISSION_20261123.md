# Lazy admission of independent LP rows: frozen scoped protocol

Date: 2026-11-23 UTC. Lane: draft PR #4988. Status: **mathematical derivation and preregistration, not native tested**. [pcc:unverified]

## Motivation and exact mathematical claim

The original `ForestWitnessExtractor::add_row` constructs a source-bearing `BTreeMap` projection for every admitted row, including rows that **increase LP rank and produce no cancellation witness**. For a stream of two-LP relations this work includes the entire tree path of both endpoints even for a new bridge. The existing forest already knows component roots and which components have an anchor. We can decide a subset of independent admissions without path projection:

- A singleton `a x_u=0` is independent iff its connected equation-tree component is unanchored (over prime `F_ell`, `a!=0`).
- A two-LP row joining *different* components is independent iff at least one component is unanchored. When both components are already anchored the bridge is dependent and requires a source certificate.
- Empty rows and same-component pairs remain on the original coefficient-aware reduction path. A same-component cycle can be balanced or unbalanced, and must not be inferred independent from topology alone.

These facts follow from the frame-matroid dimension invariant: each unanchored tree component has one free root parameter, while each anchored component has none. A bridge removes one free parameter unless both are already zero; a first singleton fixes its component parameter. A rank-increasing row needs no original-source certificate at admission, because the exact *row coefficients and endpoint inverses* are still stored for subsequent tree traversal and witness recovery. The selected mode changes no source data, forest joins, cache invalidation, or downstream arithmetic. Assumption: valid canonical rows of arity <= 2 with nonzero coefficients in prime `F_ell`; odd-but-composite moduli are not covered by the rank theorem, although failed inversions must still be atomic.

## Precommitted comparison and falsification

**Control:** the existing standard `collect_witnesses` and `ForestWitnessExtractor::new`. **Candidate:** an opt-in `collect_witnesses_lazy` and constructor builder that skips full `project` only for those provably independent singleton/bridge admissions. The legacy mode remains unchanged for previous counters.

Inputs and acceptance:
1. Replay the unchanged 32-vertex / 31-tree-edge / 48-balanced-closure / 1-unbalanced-closure / reversed-edge fixture at each of `ell=3,7,101,1000003`. Check exactly equal emitted source witnesses and field rank. Candidate skips 31 projections; projected field path reductions should fall from **593 to 129**, avoiding the **464** independent-edge reductions; the parent traversal for dependent closers remains charged. These are predicted operation-counter assertions, **not observed timing**. Check every source witness with `witness_cancels_lp` and all projected small/RHS outputs with `materialize_complete_relations`.
2. Apply at least 150 fixed-seed random canonical source streams. Compare exact emitted witnesses, dense rank, independent rank and full replay; include dual-anchored bridges, mixed anchored/unanchored joins, singleton anchors, zero LP rows and rejected coefficients.
3. Count `independent_projection_skips` separately in the candidate, with backward-compatible serde defaults. Reject all malformed rows before changing metadata. Full-root crate qualification, authentic naturally collected Prime31 group-valid streams, matched pivot/MST and optimized-rho baseline remain **separate** gates.

The only resource authorization here is the existing source-bound GitHub Actions native test (Rust, release); no paid compute, timed benchmark, merge, auto-merge or leaderboard update. One writer on the existing draft PR scope, and one coordinator; no worker-spawning, local Rust toolchain, Conductor CLI or research-KB MCP is exposed in the current runtime. Repo KB wrapper is inaccessible without a checkout, so fallback consists of current `AGENTS.md`, `docs/ACTIVE.md`, session log, `docs/ic/graph-algorithms/{README,SOURCES}.md`, `large_prime_graph/witness.rs`, and cross-repo code search (crypto, cryptanalysis, crypto-autoresearcher). This does **not** establish absence of related work.

Prior art checked: Zaslavsky, *Biased Graphs I* (1989), https://people.math.binghamton.edu/zaslav/Tpapers/bg1.jctb1989.pdf; Diem, *Index calculus with double large prime variation* (2006), https://arxiv.org/abs/math/0606607. These establish the graph/rank model and double-large-prime context, not an ECDLP breakthrough. Existing internal source notes: `research/lp_graph_algorithms_20261123/FOREST_WITNESS_DESIGN_20261123.md`, `COMMON_SUFFIX_ELIMINATION_20261123.md`.

## Stop/go threshold

A native failing test or a changed witness is a correctness failure; fix or revert before describing the candidate as qualified. An exact-count improvement alone is component-level work reduction only; wall-clock speed and complete ECDLP cost remain unmeasured until the authenticated group-relation stream and matched full-cold protocol are available. [pcc:unverified]

## Implementation and honest failure history (2026-11-23 UTC)

- The opt-in source change is [a49f3b3](https://github.com/isogeny-labs/crypto/commit/a49f3b3e8838f02419f31b9635056d045ed613a6): the legacy constructor is preserved, the new opt-in builder and `collect_witnesses_lazy` use exact source-bearing fallback on rank-dependent rows, and the serde-default counter `independent_projection_skips` tracks avoided full path expansions.
- Exact-source tests were added at [f51f59f](https://github.com/isogeny-labs/crypto/commit/f51f59fa9885336ff03a5cb2512d097e0efebe61). The first native run [45188058565](https://github.com/isogeny-labs/crypto/actions/runs/45188058565), job 137738758247, **failed 28 passed / 1 failed**. The long-chain exact-witness/step regression passed. In the randomized test, the `inverse_calls` equality assertion failed (legacy 28, candidate 27), *after* rank, dense-rank and exact source-witness equality checks had already passed in that trial. This is a faulty expectation in the test: independent bridges into an anchored component can trigger an unnecessary anchor-cache miss in the legacy arm, then invalidate that cache on join. The lazy arm can correctly avoid that inversion. It is not evidence of changed algebraic outputs.
- The corrected assertion and accumulated inversion-savings regression are committed at [848e1af](https://github.com/isogeny-labs/crypto/commit/848e1af41f314a1c4f50a8e0cfa2cc6db2cc8f05). [Native run 45188494358](https://github.com/isogeny-labs/crypto/actions/runs/45188494358) must be checked to terminal status; do not describe it as passing until then. The producer failure and original logs remain retained.
- The deep fixture predicts legacy 593 versus lazy 129 arithmetic parent-edge reductions, and 31 avoided independent source-map projections, with identical exact output witnesses. New randomized cases can additionally avoid anchor-cache inversions. **All counts are bounded synthetic instrumented tests, not Prime31 wall time or a complete cold ECDLP advantage.**

The research-KB MCP and repo checkout/Conductor binaries are not available in this invocation, and the ordinary Prime31 curve's natural 2LP relation stream remains unlocated. For the actual prime-field target, use coordinate prime `p=2147483647`, curve `y²=x³+847514846x+2087444724`, subgroup-order relation prime `ell=2147446831` and generator `G=(625037715,2100627251)` as reported in `research/prime31_ic_20261115/native/M3_RANK_EARLY.md`, **subject to independent source and group verification before measurement**. The synthetic regression fields (`ell=3,7,101,1000003`) do not stand in for that target.

## Source-exact native qualification (2026-11-23 13:55 UTC)

**PASS — 29/29 release Rust integration tests**, zero failures, on [Actions run 45188494358](https://github.com/isogeny-labs/crypto/actions/runs/45188494358), job `137740283414`, Rust `1.100.0`. The CI synthetic merge checkout was `12cb44c138`, based on implementation-and-test head `848e1af`; after completion, GitHub blob-SHA comparisons against the current draft branch matched for all five source/test modules:
- `src/cryptanalysis/large_prime_graph.rs`: `a79a800ac002e5a831529a5a2e0b5d5dc0bdcce9`
- `src/cryptanalysis/large_prime_graph/witness.rs`: `c6caf51dbcea0436294d662ac48c70df151b7202`
- `src/cryptanalysis/large_prime_filter.rs`: `e111545df0d2f0684759a1fac7cb4c07d0af72e0`
- `src/cryptanalysis/large_prime_filter_mst.rs`: `e747ae865b31e09b772769bf73b9279671d23346`
- `tests/large_prime_graph.rs`: `2de8535ba43211d79fc78c0ae3b2a8fc2111cc5a`

CI SHA-256 original witness: `80dfe96e62779b661ca5a28a593ac91c65b23ec022d90f9a996e48d92b304fdc`; tests: `32f4895cab9a91da117635a0dab439e65769cdc9fd4bb2a9f750ff0b05a5a727`. The new deep independent-chain fixture and randomized 150-stream exact-rank/witness/replay test both passed. The fixture verified **31 skipped source projections**, **593 to 129 field tree reductions** (464 avoided), same LP rank, same 49 exact cancellation witnesses, and identical replayed small rows and RHS across `F3,F7,F101,F1000003`. The 150 deterministic randomized streams verified exact equality of emitted witness vectors, original replayed small/RHS outputs, and dense LP rank, and detected fewer anchor-cache inversions in the opt-in arm; these are **not measured wall times**. [pcc:unverified]

**Limits:** The native test was a scoped original-source harness, not an unshimmed root-crate `cargo test --release --lib`, externally signed PCC verification, natural group-verified Prime31 source, or matched cold ECDLP runtime comparison. No paid compute or merge, and no leaderboard promotion. The failed 28/29 test expectation from the earlier run is preserved above; the corrected 29/29 run does not erase it. The next execution should prioritize a genuine `p=2147483647` 2LP collector trace with independently replayed group equations, then use that frozen stream for lazy versus legacy, pivot and MST comparison with measured useful small-rank, materialized fill and isolated total cold cost.
