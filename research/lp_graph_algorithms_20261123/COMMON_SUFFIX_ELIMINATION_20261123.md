# Shared-suffix cancellation for exact two-LP witness extraction

Date: 2026-11-23 UTC. Owner: existing graph-IC draft PR #4988. Status: **protocol and mathematical design before code/measurement; not validated**. [pcc:unverified]

## Problem, source revision, and hypothesis

At PR head `4d756ff176a956b7a5c1d0a571e87f4232448c97`, `ForestWitnessExtractor::project` separately reduces each endpoint of a two-large-prime row all the way to its directed equation-forest root. When endpoints share a deep ancestor, common tree edges are reduced twice, then canceled in an ordered source-coefficient map. This occurs even when the two endpoint contributions annihilate at their lowest common ancestor (LCA). The existing inversion cache already removes divisions on these edges; it does not remove their finite-field multiplies, ordered-map updates, or temporary source-map work.

Hypothesis: reduce each endpoint only to the LCA, combine the two surviving coefficients, then traverse the common suffix **once, only if its combined coefficient is nonzero**. Source-bearing witnesses, exact LP rank, refusal semantics and end-to-end reconstructed small rows must remain unchanged; component wall-clock improvement requires separate measurement. The routine still walks both parent chains to find the LCA, so no false asymptotic constant-time path claim.

## Algebraic invariant [pcc:unverified]

For an independent directed forest edge `e=(c,parent)` with `A_e x_c+B_e x_parent=0`, eliminating a term `g x_c` adds `-g A_e^{-1} row_e` to its original-source combination and leaves `-g A_e^{-1}B_e x_parent`. All coefficients and inverses are in the *subgroup-order field* `F_ell`, not the coordinate field `F_p`. The algorithm already checks invertibility at insertion.

For an input row `a x_u+b x_v` in one equation-tree component, let `w=LCA(u,v)`. Reduce only the edge-disjoint paths `u -> w` and `v -> w`, yielding `alpha x_w` and `beta x_w` plus original input/tree-row source coefficients. Combine `gamma=alpha+beta` modulo `ell`.

- If `gamma=0`, no edge above `w` is needed. The returned source combination is a witness without any common-suffix work.
- Otherwise, reduce `gamma x_w` along `w -> root` **once**. This is algebraically identical to the sum of two separate full-root reductions because all edge operations are linear.
- Rows with separate roots, one endpoint, and empty rows retain the original path.
- No change to forest admission, anchor normalization, rerooting, source replay, or source limits.

Write `h_u,h_v` for the endpoints' depths and `s` for the number of edges from `w` to root. The old arithmetic reductions cost `h_u+h_v`. The proposed cost is `h_u+h_v-2s+1_{gamma != 0}s`; source map work is output-sensitive and can still dominate. **Parent lookups** still cost `h_u+h_v`; expose them separately from field reductions. For `R` rows and maximum depth `D`, the conservative worst-case remains `O(RD log(R+2)+V log V+L)` ordered-map work, with `L` explicitly emitted source terms. No ECDLP exponent improvement follows.

The rank/circuit interpretation is constrained by gain-graph frame-matroid theory, not inferred from ordinary graph cycles alone. Literature: Zaslavsky, *Biased Graphs I* (JCTB 1989, https://people.math.binghamton.edu/zaslav/Tpapers/bg1.jctb1989.pdf); Diem, *Index calculus with double large prime variation* (https://arxiv.org/abs/math/0606607). Internal sources: `docs/ic/graph-algorithms/{README,SOURCES}.md`, `research/lp_graph_algorithms_20261123/{FOREST_WITNESS_DESIGN_20261123,ROOTED_WITNESS_PATHS_20261123,ANCHOR_REDUCTION_CACHE_20261123}.md`, and original `src/cryptanalysis/large_prime_graph/witness.rs`. These citations establish the model, not a novel complete-solver result.

## Preregistered bounded native test and falsification

Use only the existing Rust source module and `tests/large_prime_graph.rs`, through the existing `.github/workflows/graph-ic-gain-dsu-native.yml`. Build a 32-vertex forest: edges `(v,1),(v+1,1)` for `v=0..29`, then `(29,1),(31,1)`. This gives leaves 30,31 sharing a depth-29 suffix under root 0. Add 48 balanced closers `(30,1),(31,ell-1)`, then one unbalanced closer `(30,1),(31,1)`, for each `ell=3,7,101,1000003`. Parent reads through the balanced closers are `464+48*60=3344`, while arithmetic edge reductions are `464+48*2=560`. The unbalanced closer adds 60 parent reads but 31 reductions, pins the component, and increases LP rank from 31 to 32.

Acceptance: assert exact read/reduction counts; assert 48 source witnesses are valid, replay exactly to original small rows and RHS with existing `replay_complete_relation`, and compare LP rank with dense field elimination. Add controls for a nonzero common-suffix coefficient, distinct roots, endpoint reversal, and original invalid-row refusal. Counterexamples, compiler errors and missing CI must remain visible. Preserve the previous 26/26 source-exact native gate separately: it proves only the prechange source.

No wall-clock measurement is preregistered here; a future authentic source/group-verified Prime31 `p=2^31-1` 2LP stream must first be obtained with explicit `ell`, curve, seed, original group equations and source hashes. Compare old and new arms on identical streams including parent reads, field operations, source map work, exact small-projection rank, output/memory, refusal/fallback and complete target recovery. Do not launch paid compute, merge, or promote performance evidence. [pcc:unverified]

## Execution constraints

This runtime exposes the connected GitHub code interface, but no Rust toolchain, research-KB API, Conductor reservation executable, or distinct worker-spawning tool. Prior search of owner repositories and the existing graph notes did not establish an admissible natural Prime31 2LP fixture. The scoped code change is to the already-owned PR #4988; no new parallel worktree or claim of additional agents. A public source-bound Actions test is the permitted bounded native check; test pass requires an observed terminal run and original-source identity, not an inferred result.


## Source implementation and pending validation receipt (2026-11-23 UTC)

- Existing owner PR: https://github.com/isogeny-labs/crypto/pull/4988, kept **draft**.
- Source change: [`2bc8e8e`](https://github.com/isogeny-labs/crypto/commit/2bc8e8e0631fc998fda584797e005b218aae97fc), `src/cryptanalysis/large_prime_graph/witness.rs`. It factors one-component pairs at the LCA, charges `parent_path_reads` separately from actual modular `path_steps`, and leaves other row classes and forest updates unchanged. New `WitnessStats` field uses `#[serde(default)]` for backward-compatible old records.
- Regression: [`dc2e804`](https://github.com/isogeny-labs/crypto/commit/dc2e8047746d5de0082af81e6f7911ecfdc7d7cc), followed by canonical LP-index ordering and source-ID type corrections [`6870627`](https://github.com/isogeny-labs/crypto/commit/68706276e686378da6e2303bacf7e581826cdfb4), [`db70a61`](https://github.com/isogeny-labs/crypto/commit/db70a61ab57a158711fbc6aadbeedd0a9777ebf6). Tests use the exact production Rust module; no Python research implementation.
- [Original-source workflow run 45177691613](https://github.com/isogeny-labs/crypto/actions/runs/45177691613), created for the latest test head. **At this checkpoint the job was still in progress and is NOT a passing test receipt.** Check terminal run, job logs, source blob hashes, and fix any failures. Older 26/26 gate belongs to the previous branch head, not these changes.
- Implementation scope: one coordinator using GitHub's file-SHA update/commit contract. No separate agent workers available, no paid compute, no PR merge. Full production root compilation, authentic Prime31 relation/group replay and independent signed PCC remain unverified. [pcc:unverified]


## Qualified bounded native gate (2026-11-23 12:49 UTC)

**PASS, 27/27 original-source Rust release integration tests** on [GitHub Actions run 45177691613](https://github.com/isogeny-labs/crypto/actions/runs/45177691613), job `137703100873`, Rust 1.100.0. New `forest_witness_factors_shared_lca_suffix_once` passed across `F_3,F_7,F_101,F_1000003`; existing dense-rank, source-replay, anchor-cache, and pivot-projection regressions passed. This check is a source-bound harness, **not** unshimmed root-crate compilation or external independent PCC verification. [pcc:unverified]

CI checked out synthetic merge SHA `23c874c76125b6b892e6a7c09bda83391bb213ab`. Fetched those files and the current PR branch from GitHub and confirmed identical Git blob SHA for each relevant source: `witness.rs=fa0ca2eeb58932e8c7e4f4272d87eb4a41ef83b2`, `tests/large_prime_graph.rs=0fe4a342f1a4df0bf5f4cbfd85100a8eefdeeac8`, `large_prime_graph.rs=a79a800ac002e5a831529a5a2e0b5d5dc0bdcce9`, `large_prime_filter.rs=e111545df0d2f0684759a1fac7cb4c07d0af72e0`, `large_prime_filter_mst.rs=e747ae865b31e09b772769bf73b9279671d23346`. CI SHA-256: `witness.rs cc7708366f76281203051a398f1f28510c99998570dd68ba644c5566412c601c`, `tests/large_prime_graph.rs d12c696d8cf173f240d3e2fb6968d40fe56e58d92ddc9e7153b60545224285d5`.

The preregistered synthetic balances yielded **3344 full-parent reads and 560 finite-field tree reductions** before pinning; each of the 48 repeated balanced closers used two reductions rather than 60. The unbalanced closer required 60 full-parent reads and 31 reductions, and the final LP rank matched dense elimination at 32. These are operation-counter assertions exercised by the passing Rust test, not measured execution time or naturally collected Prime31 evidence. Full original-group replay, authentic factor-base independent rank, matched cold solver cost and independent PCC remain open.
