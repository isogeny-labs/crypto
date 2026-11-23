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
1. Replay the unchanged 32-vertex / 31-tree-edge / 48-balanced-closure / 1-unbalanced-closure / reversed-edge fixture at each of `ell=3,7,101,1000003`. Check exactly equal emitted source witnesses and field rank. Candidate skips 31 projections; projected field path reductions should fall from **591 to 127**, avoiding the **464** independent-edge reductions; the parent traversal for dependent closers remains charged. These are predicted operation-counter assertions, **not observed timing**. Check every source witness with `witness_cancels_lp` and all projected small/RHS outputs with `materialize_complete_relations`.
2. Apply at least 150 fixed-seed random canonical source streams. Compare exact emitted witnesses, dense rank, independent rank and full replay; include dual-anchored bridges, mixed anchored/unanchored joins, singleton anchors, zero LP rows and rejected coefficients.
3. Count `independent_projection_skips` separately in the candidate, with backward-compatible serde defaults. Reject all malformed rows before changing metadata. Full-root crate qualification, authentic naturally collected Prime31 group-valid streams, matched pivot/MST and optimized-rho baseline remain **separate** gates.

The only resource authorization here is the existing source-bound GitHub Actions native test (Rust, release); no paid compute, timed benchmark, merge, auto-merge or leaderboard update. One writer on the existing draft PR scope, and one coordinator; no worker-spawning, local Rust toolchain, Conductor CLI or research-KB MCP is exposed in the current runtime. Repo KB wrapper is inaccessible without a checkout, so fallback consists of current `AGENTS.md`, `docs/ACTIVE.md`, session log, `docs/ic/graph-algorithms/{README,SOURCES}.md`, `large_prime_graph/witness.rs`, and cross-repo code search (crypto, cryptanalysis, crypto-autoresearcher). This does **not** establish absence of related work.

Prior art checked: Zaslavsky, *Biased Graphs I* (1989), https://people.math.binghamton.edu/zaslav/Tpapers/bg1.jctb1989.pdf; Diem, *Index calculus with double large prime variation* (2006), https://arxiv.org/abs/math/0606607. These establish the graph/rank model and double-large-prime context, not an ECDLP breakthrough. Existing internal source notes: `research/lp_graph_algorithms_20261123/FOREST_WITNESS_DESIGN_20261123.md`, `COMMON_SUFFIX_ELIMINATION_20261123.md`.

## Stop/go threshold

A native failing test or a changed witness is a correctness failure; fix or revert before describing the candidate as qualified. An exact-count improvement alone is component-level work reduction only; wall-clock speed and complete ECDLP cost remain unmeasured until the authenticated group-relation stream and matched full-cold protocol are available. [pcc:unverified]
